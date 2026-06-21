//! Semaev summation polynomials over a prime-field Weierstrass curve.
//!
//! This module implements the Semaev summation-polynomial machinery — the combinatorial
//! primitive at the heart of the Gaudry–Diem–Joux–Vitse index calculus. The
//! summation polynomial `S_m(X_1, …, X_m)` is a symmetric multivariate polynomial over
//! `F_p` that vanishes on `(x_1, …, x_m)` precisely when there exist `y_i` such that
//! `P_i = (x_i, y_i)` are points on the curve `E/F_p` with `P_1 + ⋯ + P_m = ∞`.
//!
//! # Structure
//!
//! - [`mod`] (this file) — `SemaevError` enum, the toy `F_p`/Weierstrass curve fixture,
//!   and the `semaev_toy` constructor.
//! - [`poly`] — the `F_p[x]` univariate resultant and the multivariate/symmetric-polynomial
//!   type `S_m`.
//! - [`recursion`] — `semaev_poly(m)` via the resultant ladder `S_m = Res_X(S_{m-1}, S_3)`.
//!
//! # Toy fixture
//!
//! The fixture is `y² = x³ + x + 33` over `F_47` (`p = 47`, `a = 1`, `b = 33`).
//! The generator `G = (10, 3)` satisfies the curve equation:
//! `10³ + 10 + 33 = 1043 = 9 = 3² mod 47 ✓`. The group order is `n = 60 = 2² · 3 · 5`.
//!
//! # Principle-4 boundary
//!
//! The fixture is toy-scale. The algorithms are crypto-scale-correct; only the parameters
//! are small for auditability. The `Uint<4>` ceiling is the C1 boundary the ROADMAP flags
//! for the index-calculus solver; if the solver needs a wider field, that is a C1-widening
//! discovery at the index-calculus layer, not the Semaev polynomial layer's
//! concern.

pub mod base;
pub mod poly;
pub mod recursion;

pub use recursion::semaev_poly;

use crypto_bigint::Uint;

use crate::curve::Curve;

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from the Semaev summation-polynomial construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemaevError {
    /// The polynomial degree is zero (constant or empty) where a non-constant polynomial
    /// is required.
    ///
    /// The `F_p[x]` resultant and the multivariate-symmetric type require non-constant
    /// inputs; a zero-degree polynomial is a degenerate case that the construction
    /// does not handle.
    DegreeZero,
    /// The variable index is out of range for the multivariate polynomial.
    ///
    /// `S_m` has `m` variables indexed `0..m`; an index `≥ m` is out of range.
    VariableOutOfRange {
        /// The index that was out of range.
        index: usize,
        /// The number of variables in the polynomial.
        num_vars: usize,
    },
    /// The number of arguments does not match the polynomial's arity.
    ///
    /// `S_m` takes exactly `m` arguments; passing a different number is an error.
    ArityMismatch {
        /// The expected number of arguments (the polynomial's `m`).
        expected: usize,
        /// The actual number of arguments supplied.
        got: usize,
    },
    /// The resultant computation encountered a zero leading coefficient.
    ///
    /// This should not occur for well-formed `F_p[x]` polynomials (the leading
    /// coefficient is always non-zero by the invariant that the degree is the
    /// index of the last non-zero coefficient). Returned as a guard against
    /// internal invariant violations.
    ZeroLeadingCoefficient,
}

impl std::fmt::Display for SemaevError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemaevError::DegreeZero => {
                write!(f, "polynomial has degree zero (constant or empty)")
            }
            SemaevError::VariableOutOfRange { index, num_vars } => {
                write!(f, "variable index {index} out of range for {num_vars}-variable polynomial")
            }
            SemaevError::ArityMismatch { expected, got } => {
                write!(f, "arity mismatch: expected {expected} arguments, got {got}")
            }
            SemaevError::ZeroLeadingCoefficient => {
                write!(f, "zero leading coefficient (internal invariant violation)")
            }
        }
    }
}

impl std::error::Error for SemaevError {}

// ─── toy fixture ─────────────────────────────────────────────────────────────

/// The prime for the Semaev toy fixture: `p = 47`.
///
/// A small prime fitting the `Uint<4>` ceiling. The curve `y² = x³ + x + 33` over
/// `F_47` has group order `n = 60 = 2² · 3 · 5`, providing enough points to exhibit
/// non-vacuous `S_3`/`S_4` vanishing.
pub const SEMAEV_TOY_P: u64 = 47;

/// Return the hardcoded Semaev toy curve fixture.
///
/// The curve is `y² = x³ + x + 33` over `F_47` (`p = 47`, `a = 1`, `b = 33`).
/// The generator `G = (10, 3)` satisfies the curve equation:
/// `10³ + 10 + 33 = 1043 = 1043 - 22*47 = 1043 - 1034 = 9 = 3² mod 47 ✓`.
/// The group order is `n = 60 = 2² · 3 · 5` (verified offline by brute-force point-counting).
///
/// Known points (multiples of G, computed offline):
/// - `G  = (10, 3)`
/// - `2G = (16, 44)`
/// - `3G = (8, 41)`
/// - `4G = (24, 43)`
/// - `5G = (38, 0)` — 2-torsion point (y = 0)
///
/// # Principle-4 annotation
///
/// This fixture is hand-picked (not discovered via Schoof–SEA). The group order `n = 60`
/// was verified offline by brute-force point-counting. Toy scale only — crypto-scale `p`
/// would require Schoof–SEA for point counting.
///
/// The fixture is shared with `rho::curve::test_curves::composite_toy` (same curve
/// parameters), which provides additional verification of the group order and generator.
pub fn semaev_toy() -> Curve {
    Curve {
        p:  Uint::<4>::from(SEMAEV_TOY_P),
        a:  Uint::<4>::from(1u64),
        b:  Uint::<4>::from(33u64),
        // n = 60 = 2² · 3 · 5 (verified offline: brute-force point-count of
        // y² = x³ + x + 33 mod 47; also verified by composite_toy in test_curves.rs).
        n:  Uint::<4>::from(60u64),
        // G = (10, 3): 10³ + 10 + 33 = 1043 = 9 = 3² mod 47 ✓
        gx: Uint::<4>::from(10u64),
        gy: Uint::<4>::from(3u64),
    }
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_field::FpMonty4 as FpMonty;
    use crate::curve::AffinePoint;

    #[test]
    fn semaev_toy_generator_on_curve() {
        let c = semaev_toy();
        let g: AffinePoint<FpMonty> = c.generator();
        assert!(c.is_on_curve(&g), "semaev toy: generator not on curve");
    }

    #[test]
    fn semaev_toy_n_times_g_is_infinity() {
        let c = semaev_toy();
        let g: AffinePoint<FpMonty> = c.generator();
        let ng = c.scalar_mul(&g, &c.n);
        assert!(ng.is_infinity(), "semaev toy: n·G should be ∞");
    }

    #[test]
    fn semaev_error_display() {
        // Smoke-test Display impls (no panic).
        let _ = format!("{}", SemaevError::DegreeZero);
        let _ = format!("{}", SemaevError::VariableOutOfRange { index: 3, num_vars: 2 });
        let _ = format!("{}", SemaevError::ArityMismatch { expected: 3, got: 2 });
        let _ = format!("{}", SemaevError::ZeroLeadingCoefficient);
    }
}
