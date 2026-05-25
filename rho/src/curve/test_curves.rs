//! Small prime-order curves for unit tests and Phase 4 ECDLP KATs.
//!
//! Both curves have ~20-bit prime group order, which makes Pollard rho terminate
//! in ~1000 expected steps — fast enough for debug-mode tests.
//!
//! Parameters were computed offline via point-counting (brute-force enumeration of
//! `y² = x³ + ax + b mod p` for each `x`) and verified by checking that `n·G = ∞`.
//!
//! # Curve A — `tiny_a`
//!
//! `y² = x³ − 3x + 3  mod  1_048_517`
//!
//! | Symbol | Value |
//! |--------|-------|
//! | p | `1_048_517` (20-bit prime) |
//! | a | `1_048_514` (= −3 mod p) |
//! | b | `3` |
//! | n | `1_048_051` (20-bit prime) |
//! | G | `(1, 1)` |
//!
//! # Curve B — `tiny_b`
//!
//! `y² = x³ − 3x + 16  mod  1_048_583`
//!
//! | Symbol | Value |
//! |--------|-------|
//! | p | `1_048_583` (20-bit prime) |
//! | a | `1_048_580` (= −3 mod p) |
//! | b | `16` |
//! | n | `1_048_387` (20-bit prime) |
//! | G | `(0, 4)` |

use crypto_bigint::Uint;
use crate::curve::Curve;

// ── Curve A ───────────────────────────────────────────────────────────────────

/// Return the 20-bit test curve A.
///
/// `y² = x³ − 3x + 3 mod 1_048_517`.  Prime group order `n = 1_048_051`.
pub fn tiny_a() -> Curve {
    let p = Uint::<4>::from(1_048_517u64);
    Curve {
        p,
        a: Uint::<4>::from(1_048_514u64), // -3 mod p
        b: Uint::<4>::from(3u64),
        n: Uint::<4>::from(1_048_051u64),
        gx: Uint::<4>::from(1u64),
        gy: Uint::<4>::from(1u64),
    }
}

/// Group order of `tiny_a`.
pub const TINY_A_N: u64 = 1_048_051;

// ── Curve B ───────────────────────────────────────────────────────────────────

/// Return the 20-bit test curve B.
///
/// `y² = x³ − 3x + 16 mod 1_048_583`.  Prime group order `n = 1_048_387`.
pub fn tiny_b() -> Curve {
    let p = Uint::<4>::from(1_048_583u64);
    Curve {
        p,
        a: Uint::<4>::from(1_048_580u64), // -3 mod p
        b: Uint::<4>::from(16u64),
        n: Uint::<4>::from(1_048_387u64),
        gx: Uint::<4>::from(0u64),
        gy: Uint::<4>::from(4u64),
    }
}

/// Group order of `tiny_b`.
pub const TINY_B_N: u64 = 1_048_387;

// ── Tiny GLV curve ────────────────────────────────────────────────────────────

/// Return the tiny GLV-capable test curve.
///
/// `y² = x³ + 7 mod 1051`.  Prime group order `n = 1093`.
///
/// This is a secp256k1-style curve (`a = 0`, `b = 7`) over a small prime field
/// that admits the GLV endomorphism `φ(x, y) = (β·x mod p, y)` with
/// `φ(P) = λ·P` for all curve points P.
///
/// # Parameters
///
/// | Symbol | Value | Notes |
/// |--------|-------|-------|
/// | p | `1051` | prime, `p ≡ 3 (mod 4)`, `p ≡ 1 (mod 3)` |
/// | b | `7` | secp256k1-style |
/// | n | `1093` | prime group order, `n ≡ 1 (mod 3)` |
/// | G | `(3, 666)` | base point |
/// | β | `870` | cube root of unity mod p |
/// | λ | `151` | GLV scalar (`λ² + λ + 1 ≡ 0 mod n`) |
pub fn tiny_glv() -> Curve {
    Curve {
        p: Uint::<4>::from(1051u64),
        a: Uint::<4>::ZERO,
        b: Uint::<4>::from(7u64),
        n: Uint::<4>::from(1093u64),
        gx: Uint::<4>::from(3u64),
        gy: Uint::<4>::from(666u64),
    }
}

/// Group order of `tiny_glv`.
pub const TINY_GLV_N: u64 = 1093;
/// Cube root of unity mod p for `tiny_glv`.
pub const TINY_GLV_BETA: u64 = 870;
/// GLV eigenvalue for `tiny_glv`.
pub const TINY_GLV_LAMBDA: u64 = 151;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::curve::AffinePoint;
    use crate::field::{Fp, FpMonty};

    #[test]
    fn tiny_a_generator_on_curve() {
        let c = tiny_a();
        let g: AffinePoint<FpMonty> = c.generator();
        assert!(c.is_on_curve(&g), "tiny_a: generator not on curve");
    }

    #[test]
    fn tiny_a_n_times_g_is_infinity() {
        let c = tiny_a();
        let g: AffinePoint<FpMonty> = c.generator();
        let ng = c.scalar_mul(&g, &c.n);
        assert!(ng.is_infinity(), "tiny_a: n·G should be ∞");
    }

    /// Known DLP values computed by the Python reference implementation.
    ///
    /// | k | Q.x | Q.y |
    /// |---|-----|-----|
    /// | 7 | 1_026_105 | 636_225 |
    /// | 100 | 659_291 | 755_487 |
    /// | 1000 | 925_418 | 411_028 |
    #[test]
    fn tiny_a_scalar_mul_reference() {
        let c = tiny_a();
        let g: AffinePoint<FpMonty> = c.generator();
        let cases: &[(u64, u64, u64)] = &[
            (7,    1_026_105, 636_225),
            (100,  659_291,   755_487),
            (1000, 925_418,   411_028),
        ];
        for &(k, ref_x, ref_y) in cases {
            let result = c.scalar_mul(&g, &Uint::<4>::from(k));
            match &result {
                AffinePoint::Infinity => panic!("tiny_a: {k}·G is ∞"),
                AffinePoint::Finite { x, y } => {
                    assert_eq!(x.to_uint(), Uint::<4>::from(ref_x), "tiny_a: {k}·G x mismatch");
                    assert_eq!(y.to_uint(), Uint::<4>::from(ref_y), "tiny_a: {k}·G y mismatch");
                }
            }
        }
    }

    #[test]
    fn tiny_b_generator_on_curve() {
        let c = tiny_b();
        let g: AffinePoint<FpMonty> = c.generator();
        assert!(c.is_on_curve(&g), "tiny_b: generator not on curve");
    }

    #[test]
    fn tiny_b_n_times_g_is_infinity() {
        let c = tiny_b();
        let g: AffinePoint<FpMonty> = c.generator();
        let ng = c.scalar_mul(&g, &c.n);
        assert!(ng.is_infinity(), "tiny_b: n·G should be ∞");
    }

    /// Known DLP values computed by the Python reference implementation.
    ///
    /// | k | Q.x | Q.y |
    /// |---|-----|-----|
    /// | 7 | 284_547 | 163_192 |
    /// | 42 | 132_859 | 318_692 |
    /// | 99991 | 654_745 | 751_943 |
    #[test]
    fn tiny_b_scalar_mul_reference() {
        let c = tiny_b();
        let g: AffinePoint<FpMonty> = c.generator();
        let cases: &[(u64, u64, u64)] = &[
            (7,     284_547, 163_192),
            (42,    132_859, 318_692),
            (99991, 654_745, 751_943),
        ];
        for &(k, ref_x, ref_y) in cases {
            let result = c.scalar_mul(&g, &Uint::<4>::from(k));
            match &result {
                AffinePoint::Infinity => panic!("tiny_b: {k}·G is ∞"),
                AffinePoint::Finite { x, y } => {
                    assert_eq!(x.to_uint(), Uint::<4>::from(ref_x), "tiny_b: {k}·G x mismatch");
                    assert_eq!(y.to_uint(), Uint::<4>::from(ref_y), "tiny_b: {k}·G y mismatch");
                }
            }
        }
    }

    /// tiny_glv generator is on the curve.
    #[test]
    fn tiny_glv_generator_on_curve() {
        let c = tiny_glv();
        let g: AffinePoint<FpMonty> = c.generator();
        assert!(c.is_on_curve(&g), "tiny_glv: generator not on curve");
    }

    /// n·G = ∞ for tiny_glv.
    #[test]
    fn tiny_glv_n_times_g_is_infinity() {
        let c = tiny_glv();
        let g: AffinePoint<FpMonty> = c.generator();
        let ng = c.scalar_mul(&g, &c.n);
        assert!(ng.is_infinity(), "tiny_glv: n·G should be ∞");
    }

    /// φ(G) = λ·G for tiny_glv (verifies the GLV constants are correct).
    #[test]
    fn tiny_glv_endomorphism_matches_lambda() {
        use crate::ecdlp::glv::glv_phi;
        let c = tiny_glv();
        let g: AffinePoint<FpMonty> = c.generator();
        let phi_g = glv_phi(&g, &c.p, TINY_GLV_BETA);
        let lam_g = c.scalar_mul(&g, &Uint::<4>::from(TINY_GLV_LAMBDA));
        assert_eq!(phi_g, lam_g, "tiny_glv: φ(G) ≠ λ·G");
    }
}
