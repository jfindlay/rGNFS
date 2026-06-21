//! Generic Weierstrass curve over GF(p) for baseline ECDLP experiments.
//!
//! The curve is `y² = x³ − 3x + 1` over the 63-bit prime
//! `p = 2^63 − 25 = 9_223_372_036_854_775_783`.
//!
//! # Parameters
//!
//! - **p** = `9_223_372_036_854_775_783`  (63-bit prime)
//! - **a** = `p − 3`  (i.e. −3 mod p, same as NIST-style curves)
//! - **b** = `1`
//! - **G** = `(3, 821_487_384_573_098_969)`
//!
//! The group order n is not used internally (it is not required for
//! curve-arithmetic and r-adding walk work), but several reference multiples of G are recorded
//! in the KATs for correctness checking.
//!
//! # Cross-check note
//!
//! `k256` / `p256` operate on different primes, so we cannot cross-check
//! this curve against those crates directly.  The KATs instead cross-check
//! the group law against an independent Python/affine-coordinate reference
//! implementation (recorded values are hard-coded in the test module).

use crypto_bigint::Uint;
use crate::curve::Curve;

/// Return the generic 63-bit Weierstrass test curve.
///
/// `y² = x³ − 3x + 1 mod p`,  `p = 2^63 − 25`.
pub fn generic_curve() -> Curve {
    let p = Uint::<4>::from(9_223_372_036_854_775_783u64);
    // a = p - 3 ≡ -3 (mod p)
    let a = Uint::<4>::from(9_223_372_036_854_775_780u64);
    let b = Uint::<4>::ONE;
    // Generator G = (3, 821_487_384_573_098_969)
    let gx = Uint::<4>::from(3u64);
    let gy = Uint::<4>::from(821_487_384_573_098_969u64);
    // Group order n is unknown at this stage; use 0 as a sentinel.
    // (Factoring/ECDLP code receives n from the caller, not from this struct.)
    let n = Uint::<4>::ZERO;
    Curve { p, a, b, n, gx, gy }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use shared_field::{Fp, FpMonty4 as FpMonty};

    use crate::curve::{AffinePoint, JacobianPoint};

    // Reference points computed by independent Python affine arithmetic:
    //   y² = x³ − 3x + 1  mod  9_223_372_036_854_775_783
    //
    //   2G = (3_398_084_434_630_706_869, 1_269_927_104_498_887_686)
    //   3G = (6_439_912_711_603_677_474, 4_727_936_577_064_732_748)
    //   4G = (2_537_216_284_137_677_713, 4_719_361_313_946_664_957)
    //   5G = (5_154_529_326_275_311_306, 8_346_922_522_928_899_966)

    fn ref_points() -> [(u64, u64); 5] {
        [
            (3, 821_487_384_573_098_969),                      // 1G
            (3_398_084_434_630_706_869, 1_269_927_104_498_887_686), // 2G
            (6_439_912_711_603_677_474, 4_727_936_577_064_732_748), // 3G
            (2_537_216_284_137_677_713, 4_719_361_313_946_664_957), // 4G
            (5_154_529_326_275_311_306, 8_346_922_522_928_899_966), // 5G
        ]
    }

    #[test]
    fn generator_on_curve() {
        let c = generic_curve();
        let g: AffinePoint<FpMonty> = c.generator();
        assert!(c.is_on_curve(&g), "generator not on curve");
    }

    #[test]
    fn scalar_mul_small_matches_reference() {
        let c = generic_curve();
        let g: AffinePoint<FpMonty> = c.generator();
        let refs = ref_points();

        for (k, &(ref_x, ref_y)) in refs.iter().enumerate() {
            let k_val = (k + 1) as u64;
            let result = c.scalar_mul(&g, &Uint::<4>::from(k_val));
            match &result {
                AffinePoint::Infinity => panic!("{k_val}·G should not be infinity"),
                AffinePoint::Finite { x, y } => {
                    assert_eq!(
                        x.to_uint(),
                        Uint::<4>::from(ref_x),
                        "{k_val}·G x-coordinate mismatch"
                    );
                    assert_eq!(
                        y.to_uint(),
                        Uint::<4>::from(ref_y),
                        "{k_val}·G y-coordinate mismatch"
                    );
                }
            }
            assert!(c.is_on_curve(&result), "{k_val}·G not on curve");
        }
    }

    #[test]
    fn add_then_scalar_mul_consistent() {
        // 2G via double must equal 2G via scalar_mul.
        let c = generic_curve();
        let p = &c.p;
        let g: AffinePoint<FpMonty> = c.generator();
        let gj = JacobianPoint::from_affine(&g, p);

        let two_g_double = c.double_jacobian(&gj).to_affine(p);
        let two_g_scalar = c.scalar_mul(&g, &Uint::<4>::from(2u64));
        assert_eq!(two_g_double, two_g_scalar, "double ≠ 2·G via scalar_mul");
    }

    #[test]
    fn negation_consistency() {
        // G + (−G) = ∞
        let c = generic_curve();
        let p = &c.p;
        let g: AffinePoint<FpMonty> = c.generator();
        let neg_g = c.negate(&g);
        let gj = JacobianPoint::from_affine(&g, p);
        let neg_gj = JacobianPoint::from_affine(&neg_g, p);
        let sum = c.add_jacobian(&gj, &neg_gj).to_affine(p);
        assert!(sum.is_infinity(), "G + (−G) should be ∞");
    }
}
