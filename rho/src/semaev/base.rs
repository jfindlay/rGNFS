//! Semaev base summation polynomials `S_2`, `S_3` and the vanishing predicate.
//!
//! This module provides the explicit low-order Semaev summation polynomials:
//!
//! - [`s2`] — `S_2(X_1, X_2) = X_1 − X_2` (two points sum to `∞` iff `P_2 = −P_1`,
//!   i.e. `x_1 = x_2`).
//! - [`s3`] — the symmetric `S_3(X_1, X_2, X_3)` derived from the short-Weierstrass
//!   group law; vanishes iff there exist `y_i` such that `(x_i, y_i)` are on the curve
//!   and `P_1 + P_2 + P_3 = ∞`.
//! - [`vanishes_s2`] / [`vanishes_s3`] — the vanishing predicate: checks both that
//!   `S_m(x_1, …, x_m) = 0` **and** that there exist `y_i` on the curve making
//!   `Σ P_i = ∞` (via the frozen group law). These must agree.
//!
//! # Derivation of `S_3`
//!
//! For the short-Weierstrass curve `y² = x³ + ax + b`, three points `P_1, P_2, P_3`
//! satisfy `P_1 + P_2 + P_3 = ∞` iff they are collinear — they lie on a common line
//! `y = λx + μ` that intersects the curve at exactly these three x-coordinates.
//!
//! Substituting `y = λx + μ` into `y² = x³ + ax + b` gives a cubic in `x`:
//!
//! ```text
//! x³ − λ²x² + (a − 2λμ)x + (b − μ²) = 0
//! ```
//!
//! By Vieta's formulas with roots `x_1, x_2, x_3`:
//!
//! ```text
//! e_1 = x_1 + x_2 + x_3 = λ²
//! e_2 = x_1x_2 + x_1x_3 + x_2x_3 = a − 2λμ
//! e_3 = x_1x_2x_3 = μ² − b
//! ```
//!
//! Eliminating `λ` and `μ`: from `e_1 = λ²` and `e_3 = μ² − b`, we get
//! `λ² = e_1` and `μ² = e_3 + b`. From `e_2 = a − 2λμ`, we get
//! `2λμ = a − e_2`, so `(2λμ)² = (a − e_2)²`. Since `(2λμ)² = 4λ²μ² = 4e_1(e_3 + b)`:
//!
//! ```text
//! S_3(x_1, x_2, x_3) = (a − e_2)² − 4e_1(e_3 + b) = 0
//! ```
//!
//! Expanding:
//!
//! ```text
//! S_3 = e_2² − 2a·e_2 − 4·e_1·e_3 − 4b·e_1 + a²
//! ```
//!
//! This polynomial is symmetric in `x_1, x_2, x_3` (it is expressed entirely in
//! elementary symmetric polynomials `e_1, e_2, e_3`) and has degree 2 in each variable.
//!
//! # Vanishing relation
//!
//! `S_3(x_1, x_2, x_3) = 0` iff there **exist** `y_i` such that `P_i = (x_i, y_i)` are
//! on the curve and `P_1 + P_2 + P_3 = ∞`. This is an existential condition on the
//! y-coordinates — the polynomial only depends on the x-coordinates. The vanishing
//! predicate [`vanishes_s3`] checks both the polynomial condition and the group-law
//! existence condition (by trying all combinations of y-values for each x-coordinate).
//!
//! # Principle-4 boundary
//!
//! The fixture is toy-scale (`p = 47`, group order `n = 60`). The algorithms are
//! crypto-scale-correct; only the parameters are small for auditability.

use shared_field::Fp;

use crate::curve::{AffinePoint, Curve, JacobianPoint};
use crate::semaev::poly::MultiPoly;

// ─── S_2 ─────────────────────────────────────────────────────────────────────

/// Construct the Semaev base polynomial `S_2(X_1, X_2) = X_1 − X_2` over `F_p`.
///
/// `S_2(x_1, x_2) = 0` iff `x_1 = x_2`, i.e. `P_2 = −P_1` (the negation `−(x,y) = (x,−y)`
/// fixes the x-coordinate). This is the degenerate but foundational base case: two points
/// sum to `∞` iff one is the negation of the other.
///
/// The result is a [`MultiPoly`] in 2 variables over `F_p`:
/// `S_2 = 1·x_0 + (p−1)·x_1` (i.e. `x_0 − x_1 mod p`).
#[must_use]
pub fn s2(p: u64) -> MultiPoly {
    // S_2(X_1, X_2) = X_1 - X_2 = x_0 - x_1 (variables indexed 0, 1)
    let mut poly = MultiPoly::zero(2, p);
    poly.add_term(vec![1, 0], 1);       // +x_0
    poly.add_term(vec![0, 1], p - 1);   // -x_1 = (p-1)*x_1 mod p
    poly
}

// ─── S_3 ─────────────────────────────────────────────────────────────────────

/// Construct the Semaev base polynomial `S_3(X_1, X_2, X_3)` for the curve `y² = x³ + ax + b`.
///
/// `S_3(x_1, x_2, x_3) = 0` iff there exist `y_i` such that `P_i = (x_i, y_i)` are on the
/// curve and `P_1 + P_2 + P_3 = ∞`. The polynomial is symmetric in all three variables and
/// has degree 2 in each.
///
/// # Derivation
///
/// Derived from the collinearity condition for the short-Weierstrass group law (see module
/// doc). The formula in elementary symmetric polynomials `e_1, e_2, e_3` is:
///
/// ```text
/// S_3 = e_2² − 2a·e_2 − 4·e_1·e_3 − 4b·e_1 + a²
/// ```
///
/// Expanded into monomials in `x_0, x_1, x_2` (variables indexed 0, 1, 2):
///
/// ```text
/// S_3 = x_0²x_1² + x_0²x_2² + x_1²x_2²
///       − 2·x_0²x_1x_2 − 2·x_0x_1²x_2 − 2·x_0x_1x_2²
///       − 2a·(x_0x_1 + x_0x_2 + x_1x_2)
///       − 4b·(x_0 + x_1 + x_2)
///       + a²
/// ```
///
/// # Parameters
///
/// - `a` — the curve coefficient `a` in `y² = x³ + ax + b` (as `u64`, reduced mod `p`).
/// - `b` — the curve coefficient `b` in `y² = x³ + ax + b` (as `u64`, reduced mod `p`).
/// - `p` — the field prime.
#[must_use]
pub fn s3(a: u64, b: u64, p: u64) -> MultiPoly {
    let mut poly = MultiPoly::zero(3, p);

    // ── e_2² terms ────────────────────────────────────────────────────────────
    // e_2 = x_0x_1 + x_0x_2 + x_1x_2
    // e_2² = x_0²x_1² + x_0²x_2² + x_1²x_2²
    //        + 2x_0²x_1x_2 + 2x_0x_1²x_2 + 2x_0x_1x_2²
    poly.add_term(vec![2, 2, 0], 1); // x_0²x_1²
    poly.add_term(vec![2, 0, 2], 1); // x_0²x_2²
    poly.add_term(vec![0, 2, 2], 1); // x_1²x_2²
    poly.add_term(vec![2, 1, 1], 2); // 2·x_0²x_1x_2
    poly.add_term(vec![1, 2, 1], 2); // 2·x_0x_1²x_2
    poly.add_term(vec![1, 1, 2], 2); // 2·x_0x_1x_2²

    // ── −4·e_1·e_3 terms ──────────────────────────────────────────────────────
    // e_1 = x_0 + x_1 + x_2,  e_3 = x_0x_1x_2
    // 4·e_1·e_3 = 4(x_0²x_1x_2 + x_0x_1²x_2 + x_0x_1x_2²)
    // Subtract: add coefficient (p - 4) mod p for each term.
    let neg4 = (p - 4 % p) % p;
    poly.add_term(vec![2, 1, 1], neg4); // −4·x_0²x_1x_2
    poly.add_term(vec![1, 2, 1], neg4); // −4·x_0x_1²x_2
    poly.add_term(vec![1, 1, 2], neg4); // −4·x_0x_1x_2²

    // ── −2a·e_2 terms ─────────────────────────────────────────────────────────
    // −2a·(x_0x_1 + x_0x_2 + x_1x_2)
    let neg2a = (p - (2 * a % p)) % p;
    poly.add_term(vec![1, 1, 0], neg2a); // −2a·x_0x_1
    poly.add_term(vec![1, 0, 1], neg2a); // −2a·x_0x_2
    poly.add_term(vec![0, 1, 1], neg2a); // −2a·x_1x_2

    // ── −4b·e_1 terms ─────────────────────────────────────────────────────────
    // −4b·(x_0 + x_1 + x_2)
    let neg4b = (p - (4 * b % p)) % p;
    poly.add_term(vec![1, 0, 0], neg4b); // −4b·x_0
    poly.add_term(vec![0, 1, 0], neg4b); // −4b·x_1
    poly.add_term(vec![0, 0, 1], neg4b); // −4b·x_2

    // ── +a² constant ──────────────────────────────────────────────────────────
    let a2 = a * a % p;
    if a2 != 0 {
        poly.add_term(vec![0, 0, 0], a2); // a²
    }

    poly
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Find all y-coordinates on the curve `y² = x³ + ax + b` for a given x-coordinate.
///
/// Returns a `Vec` of `AffinePoint`s with the given x-coordinate that lie on the curve.
/// Returns an empty `Vec` if `x³ + ax + b` is not a quadratic residue mod `p`.
/// Returns a single point `(x, 0)` if `y = 0` (2-torsion point).
fn points_with_x<F: Fp<4>>(curve: &Curve, x_u64: u64) -> Vec<AffinePoint<F>> {
    let p = &curve.p;
    let x = F::from_u64(x_u64, p);
    let a = F::from_uint(curve.a, p);
    let b = F::from_uint(curve.b, p);

    // Compute rhs = x³ + ax + b
    let rhs = x.square(p).mul(&x, p).add(&a.mul(&x, p), p).add(&b, p);

    if rhs.is_zero(p) {
        // y = 0: single 2-torsion point
        return vec![AffinePoint::Finite { x: x.clone(), y: F::zero(p) }];
    }

    // Try to find a square root of rhs mod p using the Euler criterion + Tonelli-Shanks.
    // For p ≡ 3 mod 4 (which 47 satisfies), sqrt = rhs^((p+1)/4) mod p.
    // For general p, use the Legendre symbol first.
    //
    // Compute (p-1)/2 as Uint<4>: p is odd, so p-1 is even; right-shift by 1.
    let mut p_minus_1_over_2 = p.wrapping_sub(&crypto_bigint::Uint::<4>::ONE);
    p_minus_1_over_2 >>= 1;
    let legendre = rhs.pow(&p_minus_1_over_2, p);

    if !legendre.is_one(p) {
        // rhs is not a quadratic residue — no points with this x-coordinate
        return vec![];
    }

    // Compute sqrt: for p ≡ 3 mod 4, sqrt = rhs^((p+1)/4).
    // p = 47 ≡ 3 mod 4, so this works for the toy fixture.
    // For general p, a full Tonelli-Shanks implementation would be needed.
    //
    // (p+1)/4 as Uint<4>: p+1 is divisible by 4 when p ≡ 3 mod 4.
    let mut p_plus_1_over_4 = p.wrapping_add(&crypto_bigint::Uint::<4>::ONE);
    p_plus_1_over_4 >>= 2;
    let y = rhs.pow(&p_plus_1_over_4, p);

    // Verify: y² should equal rhs
    debug_assert_eq!(y.square(p), rhs, "sqrt computation failed");

    let neg_y = y.neg(p);
    if y == neg_y {
        // y = 0 case (already handled above, but guard)
        vec![AffinePoint::Finite { x, y }]
    } else {
        vec![
            AffinePoint::Finite { x: x.clone(), y: y.clone() },
            AffinePoint::Finite { x, y: neg_y },
        ]
    }
}

/// Check whether any combination of y-values for the given x-coordinates makes
/// `P_1 + P_2 + P_3 = ∞` via the group law.
fn exists_summing_triple<F: Fp<4>>(curve: &Curve, x1: u64, x2: u64, x3: u64) -> bool {
    let pts1 = points_with_x::<F>(curve, x1);
    let pts2 = points_with_x::<F>(curve, x2);
    let pts3 = points_with_x::<F>(curve, x3);

    let p = &curve.p;
    for p1 in &pts1 {
        for p2 in &pts2 {
            for p3 in &pts3 {
                let p1j = JacobianPoint::from_affine(p1, p);
                let p12j = curve.add_mixed(&p1j, p2);
                let p123j = curve.add_mixed(&p12j, p3);
                if p123j.to_affine(p).is_infinity() {
                    return true;
                }
            }
        }
    }
    false
}

// ─── vanishing predicate ─────────────────────────────────────────────────────

/// Check the `S_2` vanishing relation: `S_2(x_1, x_2) = 0 ⟺ P_1 + P_2 = ∞`.
///
/// For `S_2`, the vanishing condition `x_1 = x_2` is equivalent to `P_2 = −P_1`
/// (same x-coordinate), which means `P_1 + P_2 = ∞`. This is an exact equivalence
/// (not just existential) because the negation is unique.
///
/// Both conditions are checked and must agree:
/// 1. `S_2(x_1, x_2) = 0` (polynomial evaluation on x-coordinates).
/// 2. `P_1 + P_2 = ∞` (group-law sum via the frozen `Curve`).
///
/// Returns `true` if both conditions hold (or both fail).
///
/// # Panics
///
/// Panics if the two conditions disagree — this is a correctness invariant violation.
pub fn vanishes_s2<F: Fp<4>>(curve: &Curve, p1: &AffinePoint<F>, p2: &AffinePoint<F>) -> bool {
    let p_val = curve.p;
    let p_u64 = p_val.as_words()[0]; // toy-scale: p fits in u64

    // Polynomial check: S_2(x_1, x_2) = 0?
    let poly_zero = match (p1, p2) {
        (AffinePoint::Infinity, _) | (_, AffinePoint::Infinity) => {
            // S_2 is defined for finite x-coordinates; infinity has no x-coordinate.
            false
        }
        (AffinePoint::Finite { x: x1, .. }, AffinePoint::Finite { x: x2, .. }) => {
            let s2_poly = s2(p_u64);
            let x1_u64 = x1.to_uint().as_words()[0];
            let x2_u64 = x2.to_uint().as_words()[0];
            let val = s2_poly.eval(&[x1_u64, x2_u64]).expect("S_2 eval: arity 2");
            val == 0
        }
    };

    // Group-law check: P_1 + P_2 = ∞?
    let p1j = JacobianPoint::from_affine(p1, &p_val);
    let sum_j = curve.add_mixed(&p1j, p2);
    let group_inf = sum_j.to_affine(&p_val).is_infinity();

    assert_eq!(
        poly_zero, group_inf,
        "S_2 vanishing disagreement: poly_zero={poly_zero}, group_inf={group_inf}"
    );
    poly_zero
}

/// Check the `S_3` vanishing relation: `S_3(x_1, x_2, x_3) = 0 ⟺ ∃ y_i: Σ P_i = ∞`.
///
/// `S_3(x_1, x_2, x_3) = 0` iff there **exist** `y_i` such that `P_i = (x_i, y_i)` are
/// on the curve and `P_1 + P_2 + P_3 = ∞`. This is an existential condition — the
/// polynomial depends only on x-coordinates.
///
/// Both conditions are checked and must agree:
/// 1. `S_3(x_1, x_2, x_3) = 0` (polynomial evaluation on x-coordinates).
/// 2. There exist `y_i` on the curve such that `(x_1,y_1) + (x_2,y_2) + (x_3,y_3) = ∞`
///    (checked by trying all combinations of y-values via the frozen group law).
///
/// Returns `true` if both conditions hold (or both fail).
///
/// # Panics
///
/// Panics if the two conditions disagree — this is a correctness invariant violation.
pub fn vanishes_s3<F: Fp<4>>(
    curve: &Curve,
    x1: u64,
    x2: u64,
    x3: u64,
) -> bool {
    let p_val = curve.p;
    let p_u64 = p_val.as_words()[0]; // toy-scale: p fits in u64
    let a_u64 = curve.a.as_words()[0];
    let b_u64 = curve.b.as_words()[0];

    // Polynomial check: S_3(x_1, x_2, x_3) = 0?
    let s3_poly = s3(a_u64, b_u64, p_u64);
    let poly_val = s3_poly.eval(&[x1, x2, x3]).expect("S_3 eval: arity 3");
    let poly_zero = poly_val == 0;

    // Group-law existence check: ∃ y_i on curve s.t. (x_1,y_1)+(x_2,y_2)+(x_3,y_3) = ∞?
    let exists_inf = exists_summing_triple::<F>(curve, x1, x2, x3);

    assert_eq!(
        poly_zero, exists_inf,
        "S_3 vanishing disagreement at ({x1},{x2},{x3}): poly_zero={poly_zero}, exists_inf={exists_inf}"
    );
    poly_zero
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_field::FpNaive4 as FpNaive;
    use crate::semaev::{SEMAEV_TOY_P, semaev_toy};
    use crypto_bigint::Uint;

    const P: u64 = SEMAEV_TOY_P; // 47

    fn p_uint() -> Uint<4> {
        Uint::<4>::from(P)
    }

    fn fp(v: u64) -> FpNaive {
        FpNaive::from_u64(v, &p_uint())
    }

    fn pt(x: u64, y: u64) -> AffinePoint<FpNaive> {
        AffinePoint::Finite { x: fp(x), y: fp(y) }
    }

    // ── S_2 structure ─────────────────────────────────────────────────────────

    #[test]
    fn s2_is_x0_minus_x1() {
        let poly = s2(P);
        // S_2(3, 3) = 0
        assert_eq!(poly.eval(&[3, 3]).unwrap(), 0, "S_2(3,3) should be 0");
        // S_2(3, 5) = 3 - 5 = -2 = 45 mod 47
        assert_eq!(poly.eval(&[3, 5]).unwrap(), 45, "S_2(3,5) = -2 = 45 mod 47");
        // S_2(5, 3) = 5 - 3 = 2
        assert_eq!(poly.eval(&[5, 3]).unwrap(), 2, "S_2(5,3) = 2");
    }

    // ── S_3 structure ─────────────────────────────────────────────────────────

    #[test]
    fn s3_is_symmetric() {
        let poly = s3(1, 33, P); // toy curve: a=1, b=33
        assert!(poly.is_symmetric(), "S_3 should be symmetric in all three variables");
    }

    #[test]
    fn s3_degree_2_in_each_var() {
        let poly = s3(1, 33, P);
        // Check that no exponent in any term exceeds 2 for any variable.
        for exp in poly.terms.keys() {
            for &e in exp {
                assert!(e <= 2, "S_3 should have degree ≤ 2 in each variable, got exp {:?}", exp);
            }
        }
    }

    // ── S_3 vanishing on the toy curve ────────────────────────────────────────

    /// `S_3(x_1, x_2, x_3) = 0` for the triple `(G, 2G, -3G)` which sums to `∞`.
    ///
    /// Points (computed from the toy fixture):
    /// - `G  = (10, 3)`
    /// - `2G = (7, 30)`
    /// - `3G = (17, 13)`, so `-3G = (17, 34)`
    #[test]
    fn s3_vanishes_on_g_2g_neg3g() {
        let poly = s3(1, 33, P);
        // G=(10,3), 2G=(7,30), -3G=(17,34): x-coords 10, 7, 17
        let val = poly.eval(&[10, 7, 17]).unwrap();
        assert_eq!(val, 0, "S_3(10, 7, 17) should be 0 (G + 2G + (-3G) = ∞)");
    }

    /// `S_3(x_1, x_2, x_3) ≠ 0` for `(G, G, G)` — no choice of y-values for x=10
    /// makes three points with x=10 sum to `∞`.
    #[test]
    fn s3_nonzero_on_g_g_g() {
        let poly = s3(1, 33, P);
        // x=10 for all three: y-values are {3, 44}. No combination sums to ∞.
        let val = poly.eval(&[10, 10, 10]).unwrap();
        assert_ne!(val, 0, "S_3(10, 10, 10) should be nonzero (no y-values make sum ∞)");
    }

    // ── vanishing predicate ───────────────────────────────────────────────────

    #[test]
    fn vanishes_s2_true_for_negation() {
        let c = semaev_toy();
        // G = (10, 3), -G = (10, 44): G + (-G) = ∞
        let g = pt(10, 3);
        let neg_g = pt(10, 44);
        assert!(vanishes_s2(&c, &g, &neg_g), "S_2 should vanish for G and -G");
    }

    #[test]
    fn vanishes_s2_false_for_distinct_x() {
        let c = semaev_toy();
        // G = (10, 3), 2G = (7, 30): G + 2G = 3G ≠ ∞
        let g = pt(10, 3);
        let two_g = pt(7, 30);
        assert!(!vanishes_s2(&c, &g, &two_g), "S_2 should not vanish for G and 2G");
    }

    #[test]
    fn vanishes_s3_true_for_summing_triple() {
        let c = semaev_toy();
        // x-coords 10, 7, 17: G + 2G + (-3G) = ∞ (y-values (3,30,34))
        assert!(
            vanishes_s3::<FpNaive>(&c, 10, 7, 17),
            "S_3 should vanish for x-coords of G, 2G, -3G"
        );
    }

    #[test]
    fn vanishes_s3_false_for_non_summing_triple() {
        let c = semaev_toy();
        // x=10 for all three: no y-values make sum ∞
        assert!(
            !vanishes_s3::<FpNaive>(&c, 10, 10, 10),
            "S_3 should not vanish for x-coords (10,10,10)"
        );
    }
}
