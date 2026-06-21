//! Semaev summation polynomials `S_m` via the resultant recursion.
//!
//! This module provides the resultant-ladder construction of the Semaev summation
//! polynomials:
//!
//! - [`semaev_poly`] — compute `S_m(X_1, …, X_m)` for any `m ≥ 2` via the recursion
//!   `S_m = Res_X(S_{m-1}(X_1, …, X_{m-2}, X), S_3(X_{m-1}, X_m, X))`.
//!
//! # Recursion
//!
//! The Semaev summation polynomial `S_m` satisfies the elimination identity:
//!
//! ```text
//! S_m(X_1, …, X_m) = Res_X(S_{m-1}(X_1, …, X_{m-2}, X), S_3(X_{m-1}, X_m, X))
//! ```
//!
//! This eliminates the shared variable `X` between `S_{m-1}` (with its last argument
//! replaced by `X`) and `S_3` (with arguments `X_{m-1}`, `X_m`, `X`). The base cases
//!   are `S_2` and `S_3` from [`crate::semaev::base`].
//!
//! # Variable embedding
//!
//! The two polynomials being combined live in different variable spaces. Before computing
//! the resultant, both are embedded into a common `(m+1)`-variable space:
//!
//! - `S_{m-1}(X_1, …, X_{m-2}, X)`: variables `0..m-3` map to `X_1..X_{m-2}`;
//!   the last variable (index `m-2`) maps to `X` (index `m` in the combined space).
//! - `S_3(X_{m-1}, X_m, X)`: variable 0 maps to `X_{m-1}` (index `m-2`); variable 1
//!   maps to `X_m` (index `m-1`); variable 2 maps to `X` (index `m`).
//!
//! Eliminating variable `m` (the shared `X`) yields `S_m` in `m` variables.
//!
//! # Vanishing invariant
//!
//! `S_m(x_1, …, x_m) = 0 ⟺ ∃ y_i: P_i = (x_i, y_i) ∈ E ∧ Σ P_i = ∞`.
//! The recursion agrees with the direct construction; `S_m` is symmetric.
//!
//! # Principle-4 boundary
//!
//! The construction targets `m ≤ 5` (toy-scale). The Sylvester-matrix determinant in
//! [`MultiPoly::elim_var_resultant`] uses cofactor expansion (`O(n!)`) — acceptable for
//! the small matrix sizes arising from `S_3`'s degree-2 structure. Crypto-scale `m`
//! would require a Bareiss-style algorithm over the polynomial ring.

use crate::semaev::base::{s2, s3};
use crate::semaev::poly::MultiPoly;
use crate::semaev::SemaevError;

// ─── variable embedding ───────────────────────────────────────────────────────

/// Embed a `k`-variable polynomial into a `new_num_vars`-variable space.
///
/// Variable `i` of `poly` maps to variable `var_map[i]` in the new space.
/// Variables not in the image of `var_map` are treated as absent (exponent 0).
///
/// # Panics (debug)
///
/// Panics if `var_map.len() != poly.num_vars` or any `var_map[i] >= new_num_vars`.
fn embed_poly(poly: &MultiPoly, var_map: &[usize], new_num_vars: usize) -> MultiPoly {
    debug_assert_eq!(
        var_map.len(),
        poly.num_vars,
        "embed_poly: var_map length must equal poly.num_vars"
    );
    debug_assert!(
        var_map.iter().all(|&v| v < new_num_vars),
        "embed_poly: all var_map entries must be < new_num_vars"
    );

    let mut result = MultiPoly::zero(new_num_vars, poly.p);
    for (exp, &coeff) in &poly.terms {
        let mut new_exp = vec![0u64; new_num_vars];
        for (i, &e) in exp.iter().enumerate() {
            new_exp[var_map[i]] = e;
        }
        let entry = result.terms.entry(new_exp).or_insert(0);
        *entry = (*entry + coeff) % poly.p;
    }
    result.terms.retain(|_, v| *v != 0);
    result
}

// ─── semaev_poly ─────────────────────────────────────────────────────────────

/// Compute the Semaev summation polynomial `S_m` for the curve `y² = x³ + ax + b`.
///
/// Returns a [`MultiPoly`] in `m` variables over `F_p` such that
/// `S_m(x_1, …, x_m) = 0 ⟺ ∃ y_i: P_i = (x_i, y_i) ∈ E ∧ Σ P_i = ∞`.
///
/// # Algorithm
///
/// - `m = 2`: returns `S_2 = X_1 − X_2` (from [`crate::semaev::base::s2`]).
/// - `m = 3`: returns `S_3(X_1, X_2, X_3)` (from [`crate::semaev::base::s3`]).
/// - `m ≥ 4`: computes `S_m = Res_X(S_{m-1}(X_1, …, X_{m-2}, X), S_3(X_{m-1}, X_m, X))`
///   by embedding both polynomials into a common `(m+1)`-variable space and eliminating
///   the shared variable `X` (index `m`).
///
/// # Parameters
///
/// - `m` — the number of summands (must be `≥ 2`).
/// - `a` — the curve coefficient `a` in `y² = x³ + ax + b` (as `u64`, reduced mod `p`).
/// - `b` — the curve coefficient `b` in `y² = x³ + ax + b` (as `u64`, reduced mod `p`).
/// - `p` — the field prime.
///
/// # Errors
///
/// Returns `Err(SemaevError::DegreeZero)` if `m < 2`.
/// Propagates `Err(SemaevError::*)` from the resultant computation if an internal
/// invariant is violated.
pub fn semaev_poly(m: usize, a: u64, b: u64, p: u64) -> Result<MultiPoly, SemaevError> {
    match m {
        0 | 1 => Err(SemaevError::DegreeZero),
        2 => Ok(s2(p)),
        3 => Ok(s3(a, b, p)),
        _ => {
            // Recursive case: S_m = Res_X(S_{m-1}(X_1,...,X_{m-2},X), S_3(X_{m-1},X_m,X))
            //
            // Combined variable space: m+1 variables (indices 0..=m).
            //   - Indices 0..m-1 are X_1..X_m (the output variables of S_m).
            //   - Index m is X (the elimination variable).
            //
            // S_{m-1} has m-1 variables. Embed into m+1 variables:
            //   - Variables 0..m-3 of S_{m-1} → indices 0..m-3 of combined space.
            //   - Variable m-2 of S_{m-1} (last, becomes X) → index m of combined space.
            //   var_map = [0, 1, ..., m-3, m]
            //
            // S_3 has 3 variables. Embed into m+1 variables:
            //   - Variable 0 of S_3 (X_{m-1}) → index m-2 of combined space.
            //   - Variable 1 of S_3 (X_m)     → index m-1 of combined space.
            //   - Variable 2 of S_3 (X)        → index m of combined space.
            //   var_map = [m-2, m-1, m]

            let s_prev = semaev_poly(m - 1, a, b, p)?;
            let s3_poly = s3(a, b, p);

            let combined_num_vars = m + 1;

            // var_map for S_{m-1}: [0, 1, ..., m-3, m]
            let mut prev_var_map: Vec<usize> = (0..m - 2).collect(); // 0..m-3 inclusive
            prev_var_map.push(m); // last variable → elimination index m

            // var_map for S_3: [m-2, m-1, m]
            let s3_var_map = vec![m - 2, m - 1, m];

            let f = embed_poly(&s_prev, &prev_var_map, combined_num_vars);
            let g = embed_poly(&s3_poly, &s3_var_map, combined_num_vars);

            // Eliminate variable m (the shared X) to get S_m in m variables.
            f.elim_var_resultant(&g, m)
        }
    }
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semaev::{SEMAEV_TOY_P, semaev_toy};
    use crate::semaev::base::s3 as base_s3;

    const P: u64 = SEMAEV_TOY_P; // 47
    const A: u64 = 1;
    const B: u64 = 33;

    // ── base cases ────────────────────────────────────────────────────────────

    #[test]
    fn semaev_poly_m2_matches_s2() {
        let via_recursion = semaev_poly(2, A, B, P).unwrap();
        let direct = s2(P);
        assert_eq!(
            via_recursion, direct,
            "semaev_poly(2) should match s2 directly"
        );
    }

    #[test]
    fn semaev_poly_m3_matches_s3() {
        let via_recursion = semaev_poly(3, A, B, P).unwrap();
        let direct = base_s3(A, B, P);
        assert_eq!(
            via_recursion, direct,
            "semaev_poly(3) should match s3 directly"
        );
    }

    #[test]
    fn semaev_poly_m1_is_error() {
        assert!(
            semaev_poly(1, A, B, P).is_err(),
            "semaev_poly(1) should return an error"
        );
    }

    // ── S_4 structure ─────────────────────────────────────────────────────────

    #[test]
    fn semaev_poly_m4_has_4_vars() {
        let s4 = semaev_poly(4, A, B, P).unwrap();
        assert_eq!(s4.num_vars, 4, "S_4 should have 4 variables");
    }

    #[test]
    fn semaev_poly_m4_is_symmetric() {
        let s4 = semaev_poly(4, A, B, P).unwrap();
        assert!(s4.is_symmetric(), "S_4 should be symmetric in all 4 variables");
    }

    #[test]
    fn semaev_poly_m4_is_nonzero() {
        let s4 = semaev_poly(4, A, B, P).unwrap();
        assert!(!s4.is_zero(), "S_4 should be a non-zero polynomial");
    }

    // ── S_4 vanishing ─────────────────────────────────────────────────────────

    /// `S_4(x_1, x_2, x_3, x_4) = 0` for x-coords of `G, 2G, 3G, −6G`.
    ///
    /// `G + 2G + 3G + (−6G) = 6G + (−6G) = ∞`. Since `n = 60`, `−6G = 54G`.
    /// Known points: `G=(10,3)`, `2G=(7,30)`, `3G=(17,13)`.
    /// We need `−6G`: `6G = G + 2G + 3G`. Compute via the group law.
    /// From the toy fixture: `6G` has x-coordinate computed offline.
    ///
    /// Alternatively, use `G + 2G + (−3G) + ∞` — but that's only 3 points.
    /// Use `G + (−G) + 2G + (−2G) = ∞`: x-coords 10, 10, 7, 7.
    #[test]
    fn s4_vanishes_for_g_neg_g_2g_neg_2g() {
        let s4 = semaev_toy();
        let _ = s4; // fixture used for context; x-coords are the signal
        let poly = semaev_poly(4, A, B, P).unwrap();
        // G=(10,3), -G=(10,44), 2G=(7,30), -2G=(7,17): G + (-G) + 2G + (-2G) = ∞
        let val = poly.eval(&[10, 10, 7, 7]).unwrap();
        assert_eq!(val, 0, "S_4(10,10,7,7) should be 0: G+(-G)+2G+(-2G) = ∞");
    }

    /// `S_4(x_1, x_2, x_3, x_4) = 0` for x-coords of `G, 2G, 3G, −6G`.
    ///
    /// `1 + 2 + 3 - 6 = 0 mod 60`, so `G + 2G + 3G + (-6G) = ∞`.
    /// Need x-coord of `6G`. From the group law on the toy curve, `6G` can be computed.
    /// `6G = 5G + G`. `5G = (32, 36)` (from the test file header).
    /// `6G = (32,36) + (10,3)`: slope = (3-36)/(10-32) = (-33)/(-22) = 33/22 mod 47.
    /// `22^{-1} mod 47`: 22*15 = 330 = 7*47 + 1 → 22^{-1} = 15.
    /// slope = 33*15 = 495 = 10*47 + 25 → slope = 25.
    /// x_6G = 25^2 - 32 - 10 = 625 - 42 = 583 = 583 - 12*47 = 583 - 564 = 19.
    /// y_6G = 25*(32 - 19) - 36 = 25*13 - 36 = 325 - 36 = 289 = 289 - 6*47 = 289 - 282 = 7.
    /// So `6G = (19, 7)`, `-6G = (19, 40)`.
    #[test]
    fn s4_vanishes_for_g_2g_3g_neg6g() {
        let poly = semaev_poly(4, A, B, P).unwrap();
        // G=(10,3), 2G=(7,30), 3G=(17,13), -6G=(19,40): G+2G+3G+(-6G) = 6G+(-6G) = ∞
        // x-coords: 10, 7, 17, 19
        let val = poly.eval(&[10, 7, 17, 19]).unwrap();
        assert_eq!(val, 0, "S_4(10,7,17,19) should be 0: G+2G+3G+(-6G) = ∞");
    }

    /// `S_4(x_1, x_2, x_3, x_4) ≠ 0` for x-coords of `G, 2G, 4G, 8G`.
    ///
    /// For all 16 sign combinations `ε_1·1 + ε_2·2 + ε_3·4 + ε_4·8` (ε_i ∈ {±1}),
    /// the result is never `0 mod 60` — verified by exhaustive check. Hence no y-value
    /// combination makes the sum `∞`, so `S_4 ≠ 0` at these x-coordinates.
    ///
    /// `8G = (25, 28)` (computed from `2*(4G) = 2*(23,12)` via the doubling formula).
    #[test]
    fn s4_nonzero_for_g_2g_4g_8g() {
        let poly = semaev_poly(4, A, B, P).unwrap();
        // x-coords: G=10, 2G=7, 4G=23, 8G=25
        let val = poly.eval(&[10, 7, 23, 25]).unwrap();
        assert_ne!(val, 0, "S_4(10,7,23,25) should be nonzero: no y-values make G+2G+4G+8G = ∞");
    }
}
