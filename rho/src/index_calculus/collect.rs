//! Relation collection for the index-calculus ECDLP solver.
//!
//! This module implements the relation-collection loop: generate random multiples
//! `R = a·G + b·Q`, attempt to decompose each over the factor base via
//! [`crate::index_calculus::decompose::decompose`], and record successful decompositions
//! as `Relation` rows. The loop continues until the system is over-determined
//! (≥ `strategy.fb_size() + 1` relations).
//!
//! # Algorithm
//!
//! For `a = 1, 2, …` and `b = 0, 1, …` (systematic enumeration):
//! 1. Compute `R = a·G + b·Q` using the frozen `Curve::scalar_mul` and
//!    `Curve::add_jacobian`.
//! 2. Skip if `R = ∞` (decomposition is undefined for the point at infinity).
//! 3. Call `decompose(R, strategy)`. On `Some(fb_points)`, extract the factor-base
//!    indices and construct a `Relation` via `Relation::from_decomposition`.
//! 4. Stop when `relations.len() >= strategy.fb_size() + 1`.
//!
//! # Principle-4 boundary
//!
//! The toy fixture has `n = 60`, `|FB| = 6`, `m = 2`. The collection loop is
//! O(n²) in the worst case (all (a, b) pairs mod n). Crypto-scale collection
//! requires probabilistic sampling and a Gröbner-basis decomposition oracle.

use crypto_bigint::Uint;

use shared_field::FpNaive4 as FpNaive;

use crate::curve::{AffinePoint, JacobianPoint};
use crate::index_calculus::decompose::decompose;
use crate::index_calculus::strategy::{IndexCalcStrategy, Relation};
use crate::index_calculus::IndexCalcError;

/// Collect an over-determined system of index-calculus relations.
///
/// Generates multiples `R = a·G + b·Q` (via the frozen `Curve::scalar_mul`), calls
/// `decompose(R, strategy)`, and on success records a `Relation` row. Loops until the
/// system is over-determined (≥ `strategy.fb_size() + 1` relations).
///
/// Returns a `Vec<Relation>` with at least `strategy.fb_size() + 1` entries.
///
/// # Errors
///
/// Returns `IndexCalcError::UnderdeterminedSystem` if the loop exhausts all `(a, b)`
/// pairs up to the search limit without finding enough relations.
pub fn collect_relations(
    g: AffinePoint<FpNaive>,
    q: AffinePoint<FpNaive>,
    strategy: &IndexCalcStrategy,
) -> Result<Vec<Relation>, IndexCalcError> {
    let curve = &strategy.curve;
    let p = &curve.p;

    // The group order n (toy-scale: fits in u64).
    let n_u64 = curve.n.as_words()[0];

    // Target: at least fb_size + 1 relations for an over-determined system.
    let target = strategy.fb_size() + 1;
    let mut relations: Vec<Relation> = Vec::with_capacity(target + 4);

    // Systematic enumeration: a ∈ [1, n), b ∈ [0, n).
    // Taking a, b mod n is correct since the group has order n.
    // a = 0 is excluded: R = b·Q gives no information about log_G(Q) when b = 0 too.
    'outer: for a in 1..n_u64 {
        for b in 0..n_u64 {
            // Compute R = a·G + b·Q.
            let ag = curve.scalar_mul(&g, &Uint::<4>::from(a));
            let bq = curve.scalar_mul(&q, &Uint::<4>::from(b));

            // Skip if either component is ∞ (degenerate — b = 0 gives R = a·G, still valid).
            // Only skip if the final sum is ∞ (decomposition undefined for ∞).
            let r = if bq.is_infinity() {
                ag
            } else if ag.is_infinity() {
                bq
            } else {
                let jag = JacobianPoint::from_affine(&ag, p);
                let jbq = JacobianPoint::from_affine(&bq, p);
                curve.add_jacobian(&jag, &jbq).to_affine(p)
            };

            if r.is_infinity() {
                continue;
            }

            // Attempt decomposition over the factor base.
            if let Some(fb_points) = decompose(r, strategy) {
                // Extract factor-base indices from the decomposition.
                let fb_indices: Vec<usize> = fb_points.iter().map(|fp| fp.index).collect();

                let relation = Relation::from_decomposition(a, b, &fb_indices, &strategy.ell);
                relations.push(relation);

                if relations.len() >= target {
                    break 'outer;
                }
            }
        }
    }

    if relations.len() < target {
        return Err(IndexCalcError::UnderdeterminedSystem {
            found: relations.len(),
            needed: target,
        });
    }

    Ok(relations)
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::index_calculus::strategy::IndexCalcStrategy;
    use crate::semaev::semaev_toy;

    #[test]
    fn collect_returns_enough_relations() {
        let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
        let curve = semaev_toy();
        let g: AffinePoint<FpNaive> = curve.generator();
        // Q = 7·G (arbitrary non-trivial multiple).
        let q = curve.scalar_mul(&g, &Uint::<4>::from(7u64));

        let relations = collect_relations(g, q, &strategy)
            .expect("collect_relations should succeed for the toy fixture");

        assert!(
            relations.len() >= strategy.fb_size() + 1,
            "expected at least {} relations, got {}",
            strategy.fb_size() + 1,
            relations.len()
        );
    }
}
