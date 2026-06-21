//! Special-q descent recursion for NFS-DL individual-logarithm computation.
//!
//! This module implements the per-node descent step and the full descent loop:
//!
//! - [`descend_node`] — find a rewriting relation for a single medium prime, producing a
//!   [`DescentNode`] with child nodes for the smaller primes.
//! - [`run_descent`] — drive the full descent: pop the largest frontier prime, descend it,
//!   push children back, repeat until all primes are factor-base leaves.
//! - [`DescentSieveConfig`] — sieve parameters for the descent (a_bound, b_bound).
//!
//! # Special-q sieve reuse seam
//!
//! The special-q sieve (`special_q_sieve`) was built for relation *collection*: it scans a
//! range `[q_min, q_max]` of algebraic factor-base primes and produces relations where each
//! special prime `q` appears in the algebraic exponent vector.
//!
//! **Seam discovery**: `special_q_sieve` only processes primes that are already in the
//! algebraic factor base (`fb.algebraic_ideals`). A medium prime `q` above the factor-base
//! bound `b_alg` is not in the factor base, so it cannot be used as a special prime directly.
//!
//! **Resolution**: For each descent step, we build a temporary `FactorBase` with
//! `b_alg ≥ q` (so that `q` is included as an algebraic ideal), then call
//! `special_q_sieve` with `q_min = q_max = q`. This is an additive adaptation — it does not
//! modify `special_q.rs` or `factor_base.rs`. The temporary factor base is local to the
//! descent step and is discarded after the sieve run.
//!
//! At demonstration fidelity, rebuilding the factor base per descent step is acceptable.
//! At NFS scale, the factor base would be pre-built to cover the medium-prime range.
//!
//! # Termination
//!
//! The descent terminates because each step strictly reduces the largest prime in the
//! frontier (the `DescentFrontier` ordering invariant). `descend_node` enforces this:
//! if the sieve produces a relation whose largest prime is ≥ the target prime, it returns
//! `Err(DescentStepError::NoStrictReduction { target })` rather than accepting the relation.
//!
//! # Descent substrate contract
//!
//! This module implements the body of `descend_node` (signature frozen at initial
//! implementation) and adds `run_descent` (the full descent loop). Both are re-exported
//! from `gnfs::dl::descent`.

use crate::{
    dl::{
        DLRelation,
        descent::{
            node::{DescentFrontier, DescentNode, DescentTarget},
            solve::{DescentStepError, SolveDlError},
        },
    },
    polyselect::PolyPair,
    sieve::{FactorBase, SpecialQConfig, special_q_sieve},
};
use crate::dl::VirtualLogTable;

// ─── DescentSieveConfig ───────────────────────────────────────────────────────

/// Sieve parameters for the descent step.
///
/// Controls the sieve region used when searching for a rewriting relation for each
/// medium prime. Larger bounds increase the probability of finding a relation but
/// also increase the cost per descent step.
///
/// # Principle-2 annotation (demonstration fidelity)
///
/// At NFS scale, the sieve bounds are calibrated to the medium-prime bound and the
/// descent depth. At toy scale, small bounds (a_bound = 10, b_bound = 5) suffice.
#[derive(Debug, Clone)]
pub struct DescentSieveConfig {
    /// Half-width of the sieve region: `a` ranges over `−A..=A`.
    pub a_bound: u64,
    /// Height of the sieve region: `b` ranges over `1..=B`.
    pub b_bound: u64,
    /// Scale factor for the smoothness threshold (default: 0.5 for descent — looser than
    /// relation collection to maximise the chance of finding a rewriting relation).
    pub threshold_scale: f64,
}

impl DescentSieveConfig {
    /// Construct a `DescentSieveConfig` with the given bounds and default threshold (0.5).
    ///
    /// :param a_bound: Half-width of the sieve region.
    /// :param b_bound: Height of the sieve region.
    /// :returns: A new `DescentSieveConfig`.
    pub fn new(a_bound: u64, b_bound: u64) -> Self {
        Self { a_bound, b_bound, threshold_scale: 0.5 }
    }

    /// Construct a `DescentSieveConfig` with an explicit threshold scale.
    ///
    /// :param a_bound: Half-width of the sieve region.
    /// :param b_bound: Height of the sieve region.
    /// :param threshold_scale: Scale factor for the smoothness threshold (0.0–1.0).
    /// :returns: A new `DescentSieveConfig`.
    pub fn with_threshold(a_bound: u64, b_bound: u64, threshold_scale: f64) -> Self {
        Self { a_bound, b_bound, threshold_scale }
    }
}

// ─── descend_node ─────────────────────────────────────────────────────────────

/// Descend a single frontier node: find a relation rewriting `target` as smaller primes.
///
/// Runs a special-q sieve rooted at `target.prime()` to find a relation in which `target`
/// appears alongside strictly smaller primes. The relation rewrites `log(target)` as a
/// combination of the smaller primes' logs.
///
/// # Special-q sieve reuse
///
/// To drive the special-q sieve for a specific target prime `q`, this function builds a
/// temporary `FactorBase` with `b_alg ≥ q` (so `q` is included as an algebraic ideal),
/// then calls `special_q_sieve` with `q_min = q_max = q`. See the module-level doc for
/// the full seam analysis.
///
/// # Leaf detection
///
/// For each prime in the chosen relation, `fb.rational_index(p)` and
/// `fb.algebraic_index(p, r)` are used to check if the prime is in the original factor
/// base. Leaf nodes get `known_log = Some(vtable.rational_logs[idx])` (or algebraic
/// equivalent); interior nodes get `known_log = None`.
///
/// # Arguments
///
/// - `target`: The prime/ideal to descend.
/// - `poly`: The NFS polynomial pair (for the sieve).
/// - `fb`: The original factor base (for leaf detection and log lookup).
/// - `vtable`: The virtual-log table (for leaf log lookup).
/// - `sieve_cfg`: Sieve region and threshold parameters.
///
/// # Errors
///
/// - [`DescentStepError::NoRelationFound`] if the sieve finds no relation for `target`.
/// - [`DescentStepError::NoStrictReduction`] if all found relations fail the strict-reduction
///   invariant (largest child prime ≥ target prime).
pub fn descend_node<F: Clone>(
    target: DescentTarget,
    poly: &PolyPair,
    fb: &FactorBase,
    vtable: &VirtualLogTable<F>,
    sieve_cfg: &DescentSieveConfig,
) -> Result<DescentNode<F>, DescentStepError> {
    let q = target.prime();

    // Build a temporary factor base with b_alg ≥ q so that q is included as an algebraic
    // ideal. We use the same b_rat as the original factor base.
    //
    // Principle-2 annotation: at NFS scale, the descent factor base would be pre-built to
    // cover the medium-prime range. At demonstration fidelity, rebuilding per step is fine.
    let temp_fb = FactorBase::new(&poly.f, fb.b_rat, q);

    // Run the special-q sieve with q_min = q_max = q (fixed target prime).
    // The sieve will only process the ideal (q, r_q) for each root r_q of f mod q.
    let sq_config = SpecialQConfig::with_threshold(
        sieve_cfg.a_bound,
        sieve_cfg.b_bound,
        q,
        q,
        sieve_cfg.threshold_scale,
    );

    let results = special_q_sieve(poly, &temp_fb, &sq_config);

    // Collect all relations from all (q, r_q) runs.
    // Each relation has q in its algebraic exponent vector (guaranteed by the sieve).
    let all_relations: Vec<_> = results
        .iter()
        .flat_map(|r| r.relations.iter().map(move |rel| (r.r_q, rel)))
        .collect();

    if all_relations.is_empty() {
        return Err(DescentStepError::NoRelationFound { target });
    }

    // Find a relation that strictly reduces: all primes in the relation are < q.
    // We search through all found relations and pick the first one that satisfies the
    // strict-reduction invariant.
    let chosen = find_strictly_reducing_relation(&all_relations, q, &temp_fb);

    let (_r_q, relation) = match chosen {
        Some(r) => r,
        None => return Err(DescentStepError::NoStrictReduction { target }),
    };

    // Build child nodes from the relation's prime factors.
    // For each prime in the relation (rational and algebraic sides), create a child node:
    // - If the prime is in the original factor base: leaf node with known_log.
    // - Otherwise: interior node with known_log = None (to be descended further).
    // The target prime q is excluded from children (it is the prime being rewritten).
    let children = build_children(relation, &target, &temp_fb, fb, vtable);

    // Wrap the relation as a DLRelation (no Schirokauer columns at this stage —
    // descent uses the raw factoring relation, not the augmented DL relation).
    let dl_relation = DLRelation::new(relation.clone(), vec![]);

    Ok(DescentNode {
        target,
        rewriting_relation: Some(dl_relation),
        children,
        known_log: None,
    })
}

// ─── run_descent ──────────────────────────────────────────────────────────────

/// Drive the full descent: pop the largest frontier prime, descend it, push children back.
///
/// Iterates until the frontier is empty (all primes are factor-base leaves) or a descent
/// step fails (returning `Err(SolveDlError::DescentFailed { stuck_prime })`).
///
/// # Termination
///
/// The descent terminates because each step strictly reduces the largest prime in the
/// frontier. `descend_node` enforces the strict-reduction invariant; if it fails,
/// `run_descent` surfaces `SolveDlError::DescentFailed` rather than looping.
///
/// # Arguments
///
/// - `frontier`: The initial frontier of medium primes (from `init_descent_frontier`).
/// - `poly`: The NFS polynomial pair (for the sieve).
/// - `fb`: The factor base (for leaf detection and log lookup).
/// - `vtable`: The virtual-log table (for leaf log lookup).
/// - `sieve_cfg`: Sieve region and threshold parameters.
///
/// # Returns
///
/// `Ok(completed_nodes)` — the fully-descended nodes (all leaves have `known_log = Some`).
///
/// # Errors
///
/// - [`SolveDlError::DescentFailed`] if a frontier prime cannot be descended.
pub fn run_descent<F: Clone>(
    mut frontier: DescentFrontier<F>,
    poly: &PolyPair,
    fb: &FactorBase,
    vtable: &VirtualLogTable<F>,
    sieve_cfg: &DescentSieveConfig,
) -> Result<Vec<DescentNode<F>>, SolveDlError> {
    let mut completed: Vec<DescentNode<F>> = Vec::new();

    while let Some((target, _node)) = frontier.pop_largest() {
        let q = target.prime();

        // Check if this prime is already a factor-base leaf.
        let leaf_log = lookup_leaf_log(&target, fb, vtable);

        if let Some(log) = leaf_log {
            // This prime is in the factor base: it's a leaf node.
            let leaf_node = DescentNode {
                target,
                rewriting_relation: None,
                children: vec![],
                known_log: Some(log),
            };
            completed.push(leaf_node);
            continue;
        }

        // Not a leaf: descend this prime.
        let descended = descend_node(target.clone(), poly, fb, vtable, sieve_cfg)
            .map_err(|_| SolveDlError::DescentFailed { stuck_prime: q })?;

        // Push non-leaf children back onto the frontier for further descent.
        for child in &descended.children {
            if child.known_log.is_none() {
                // Interior child: push onto frontier for further descent.
                let child_target = child.target.clone();
                let child_node = DescentNode {
                    target: child_target.clone(),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: None,
                };
                frontier.push(child_target, child_node);
            }
        }

        completed.push(descended);
    }

    Ok(completed)
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Find the first relation in `all_relations` that strictly reduces the target prime `q`.
///
/// A relation strictly reduces `q` if every prime in the relation (rational and algebraic
/// sides) is strictly less than `q`. Returns `(r_q, relation)` for the first such relation,
/// or `None` if no such relation exists.
fn find_strictly_reducing_relation<'a>(
    all_relations: &[(u64, &'a crate::sieve::Relation)],
    q: u64,
    temp_fb: &FactorBase,
) -> Option<(u64, &'a crate::sieve::Relation)> {
    for &(r_q, rel) in all_relations {
        if relation_strictly_reduces(rel, q, temp_fb) {
            return Some((r_q, rel));
        }
    }
    None
}

/// Check if all primes in `rel` are strictly less than `q`.
///
/// Extracts the actual prime values from the rational and algebraic exponent vectors
/// using `temp_fb`, then checks that all are < `q`.
fn relation_strictly_reduces(
    rel: &crate::sieve::Relation,
    q: u64,
    temp_fb: &FactorBase,
) -> bool {
    // Check rational side: all rational primes in the relation must be < q.
    for (idx, _exp) in rel.rational_exponents.iter() {
        if idx >= temp_fb.rational_primes.len() {
            return false;
        }
        let p = temp_fb.rational_primes[idx];
        if p >= q {
            return false;
        }
    }

    // Check algebraic side: all algebraic primes in the relation must be < q.
    // The special-q sieve guarantees q appears in the algebraic exponent vector,
    // but we want all *other* primes to be < q. The prime q itself is expected
    // to appear (it's the special prime), so we allow it — but all others must be < q.
    for (idx, _exp) in rel.algebraic_exponents.iter() {
        if idx >= temp_fb.algebraic_ideals.len() {
            return false;
        }
        let p = temp_fb.algebraic_ideals[idx].p;
        // Allow q itself (the special prime); all others must be strictly smaller.
        if p > q {
            return false;
        }
    }

    true
}

/// Build child nodes from a relation's prime factors.
///
/// For each prime in the relation (rational and algebraic sides), creates a child node:
/// - If the prime is in the original factor base: leaf node with `known_log = Some(log)`.
/// - Otherwise: interior node with `known_log = None` (to be descended further).
///
/// The target prime `q` (the special prime) is excluded from the children — it is the
/// prime being rewritten, not a child. The special-q sieve guarantees `q` appears in the
/// algebraic exponent vector; we skip it here so it does not re-enter the frontier.
fn build_children<F: Clone>(
    rel: &crate::sieve::Relation,
    target: &DescentTarget,
    temp_fb: &FactorBase,
    fb: &FactorBase,
    vtable: &VirtualLogTable<F>,
) -> Vec<DescentNode<F>> {
    let q = target.prime();
    let mut children: Vec<DescentNode<F>> = Vec::new();

    // Rational side children.
    for (idx, exp) in rel.rational_exponents.iter() {
        if idx >= temp_fb.rational_primes.len() {
            continue;
        }
        let p = temp_fb.rational_primes[idx];

        // Add one child per unit of exponent (multiplicity).
        for _ in 0..exp {
            let child_target = DescentTarget::Rational(p);
            let known_log = fb.rational_index(p).map(|fb_idx| vtable.rational_logs[fb_idx].clone());
            children.push(DescentNode {
                target: child_target,
                rewriting_relation: None,
                children: vec![],
                known_log,
            });
        }
    }

    // Algebraic side children: skip the special prime q (the target being rewritten).
    // The sieve guarantees q appears in the algebraic exponent vector with exponent ≥ 1;
    // we exclude it here so it does not re-enter the frontier (which would loop).
    for (idx, exp) in rel.algebraic_exponents.iter() {
        if idx >= temp_fb.algebraic_ideals.len() {
            continue;
        }
        let ap = &temp_fb.algebraic_ideals[idx];
        let p = ap.p;
        let r = ap.r;

        // Skip the special prime q (the target being rewritten).
        if p == q {
            continue;
        }

        // Add one child per unit of exponent (multiplicity).
        for _ in 0..exp {
            let child_target = DescentTarget::Algebraic { p, r };
            let known_log =
                fb.algebraic_index(p, r).map(|fb_idx| vtable.algebraic_logs[fb_idx].clone());
            children.push(DescentNode {
                target: child_target,
                rewriting_relation: None,
                children: vec![],
                known_log,
            });
        }
    }

    children
}

/// Look up the virtual log for a target if it is a factor-base leaf.
///
/// Returns `Some(log)` if the target's prime is in the original factor base, `None` otherwise.
fn lookup_leaf_log<F: Clone>(
    target: &DescentTarget,
    fb: &FactorBase,
    vtable: &VirtualLogTable<F>,
) -> Option<F> {
    match target {
        DescentTarget::Rational(p) => {
            fb.rational_index(*p).map(|idx| vtable.rational_logs[idx].clone())
        }
        DescentTarget::Algebraic { p, r } => {
            fb.algebraic_index(*p, *r).map(|idx| vtable.algebraic_logs[idx].clone())
        }
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sieve::FactorBase;
    use num_bigint::BigInt;
    use shared_numfield::IntPoly;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    /// f(x) = x³ − x − 1.
    fn f_cubic() -> IntPoly {
        IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
    }

    #[test]
    fn descent_sieve_config_new() {
        let cfg = DescentSieveConfig::new(10, 5);
        assert_eq!(cfg.a_bound, 10);
        assert_eq!(cfg.b_bound, 5);
        assert_eq!(cfg.threshold_scale, 0.5);
    }

    #[test]
    fn descent_sieve_config_with_threshold() {
        let cfg = DescentSieveConfig::with_threshold(20, 10, 0.3);
        assert_eq!(cfg.a_bound, 20);
        assert_eq!(cfg.b_bound, 10);
        assert_eq!(cfg.threshold_scale, 0.3);
    }

    #[test]
    fn relation_strictly_reduces_basic() {
        // Build a minimal factor base with b_alg = 7 (includes primes 2, 3, 5, 7).
        let f = f_cubic();
        let temp_fb = FactorBase::new(&f, 10, 7);

        // A relation with only rational primes 2 and 3 (both < 7) strictly reduces q=7.
        use crate::sieve::{ExponentVector, Relation};
        let mut rat_exp = ExponentVector::new();
        // index 0 = prime 2, index 1 = prime 3 (in rational_primes sorted ascending).
        rat_exp.entries.push((0, 1)); // 2^1
        rat_exp.entries.push((1, 1)); // 3^1
        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: rat_exp,
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };
        assert!(relation_strictly_reduces(&rel, 7, &temp_fb));
    }

    #[test]
    fn relation_strictly_reduces_fails_for_large_prime() {
        // A relation with rational prime 11 (> 7) does NOT strictly reduce q=7.
        let f = f_cubic();
        let temp_fb = FactorBase::new(&f, 11, 7);

        use crate::sieve::{ExponentVector, Relation};
        let mut rat_exp = ExponentVector::new();
        // Find the index of prime 11 in temp_fb.rational_primes.
        let idx_11 = temp_fb.rational_index(11).expect("11 should be in rational base");
        rat_exp.entries.push((idx_11, 1)); // 11^1
        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: rat_exp,
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };
        assert!(!relation_strictly_reduces(&rel, 7, &temp_fb));
    }
}
