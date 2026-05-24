//! Schoolbook modular arithmetic over GF(p).
//!
//! `FpNaive` stores a canonical residue `v ∈ [0, p)` as a `Uint<4>` and
//! performs all operations with straightforward Barrett/add-then-reduce logic.
//! It is the pedagogical baseline: correct, slow, easy to audit.

use crypto_bigint::{Uint, NonZero, CheckedAdd};
use super::Fp;

/// Schoolbook field element: canonical residue stored as a `Uint<4>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpNaive {
    /// Canonical residue in `[0, p)`.
    v: Uint<4>,
}

impl FpNaive {
    /// Wrap a pre-reduced value (must satisfy `v < p`).
    #[inline]
    fn wrap(v: Uint<4>) -> Self {
        Self { v }
    }

    /// Reduce `v mod p` for a value known to be less than `2p`.
    #[inline]
    fn reduce_once(v: Uint<4>, p: &Uint<4>) -> Uint<4> {
        if v >= *p { v.wrapping_sub(p) } else { v }
    }
}

impl Fp for FpNaive {
    const LIMBS: usize = 4;

    fn zero(_p: &Uint<4>) -> Self {
        Self::wrap(Uint::<4>::ZERO)
    }

    fn one(_p: &Uint<4>) -> Self {
        Self::wrap(Uint::<4>::ONE)
    }

    fn from_u64(v: u64, p: &Uint<4>) -> Self {
        let u = Uint::<4>::from(v);
        let nz = NonZero::new(*p).expect("modulus must be non-zero");
        Self::wrap(u.rem(&nz))
    }

    fn from_uint(v: Uint<4>, p: &Uint<4>) -> Self {
        let nz = NonZero::new(*p).expect("modulus must be non-zero");
        Self::wrap(v.rem(&nz))
    }

    fn to_uint(&self) -> Uint<4> {
        self.v
    }

    fn add(&self, rhs: &Self, p: &Uint<4>) -> Self {
        // Sum fits in 5 limbs at most; use wrapping add and subtract p once if needed.
        let s = self.v.checked_add(&rhs.v);
        let s = match s.into_option() {
            Some(s) => s,
            // Overflow: sum >= 2^256, which exceeds any realistic p, so result < p after sub.
            None => self.v.wrapping_add(&rhs.v),
        };
        Self::wrap(Self::reduce_once(s, p))
    }

    fn sub(&self, rhs: &Self, p: &Uint<4>) -> Self {
        if self.v >= rhs.v {
            Self::wrap(self.v.wrapping_sub(&rhs.v))
        } else {
            // self < rhs: result = self + p - rhs
            Self::wrap(p.wrapping_sub(&rhs.v).wrapping_add(&self.v))
        }
    }

    fn neg(&self, p: &Uint<4>) -> Self {
        if self.v == Uint::<4>::ZERO {
            Self::wrap(Uint::<4>::ZERO)
        } else {
            Self::wrap(p.wrapping_sub(&self.v))
        }
    }

    fn mul(&self, rhs: &Self, p: &Uint<4>) -> Self {
        // Widen to 8 limbs for the full product, then reduce.
        // mul_wide returns (lo, hi): value = hi * 2^256 + lo.
        let (lo, hi) = self.v.mul_wide(&rhs.v);
        // Uint::from((lo_half, hi_half)): tuple is (L_limbs, H_limbs) → hi * 2^(L*64) + lo.
        let wide = Uint::<8>::from((lo, hi));
        // Embed p into Uint<8> as a low-half value: (lo=p, hi=ZERO) → value = p.
        let p8 = Uint::<8>::from((*p, Uint::<4>::ZERO));
        let nz_p8 = NonZero::new(p8).expect("modulus must be non-zero");
        let rem8: Uint<8> = wide.rem(&nz_p8);
        // rem8 < p < 2^256, so its high 4 limbs are zero; extract the low 4.
        let limbs8 = rem8.as_words();
        let lo4 = Uint::<4>::from_words([limbs8[0], limbs8[1], limbs8[2], limbs8[3]]);
        Self::wrap(lo4)
    }

    fn square(&self, p: &Uint<4>) -> Self {
        self.mul(self, p)
    }

    fn pow(&self, exp: &Uint<4>, p: &Uint<4>) -> Self {
        // Square-and-multiply (left-to-right).
        let mut result = Self::one(p);
        let mut base = self.clone();
        let mut e = *exp;
        while e != Uint::<4>::ZERO {
            if e.bit(0).into() {
                result = result.mul(&base, p);
            }
            base = base.square(p);
            e >>= 1;
        }
        result
    }

    fn inv(&self, p: &Uint<4>) -> Self {
        assert!(!self.is_zero(p), "attempted inversion of zero");
        // Fermat: a^(p-2) mod p for prime p.
        let exp = p.wrapping_sub(&Uint::<4>::from(2u64));
        self.pow(&exp, p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// secp256k1 prime: p = 2^256 - 2^32 - 977.
    fn secp_p() -> Uint<4> {
        Uint::<4>::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
    }

    /// Small prime for cheap tests.
    fn p13() -> Uint<4> {
        Uint::<4>::from(13u64)
    }

    #[test]
    fn add_small() {
        let p = p13();
        let a = FpNaive::from_u64(7, &p);
        let b = FpNaive::from_u64(9, &p);
        // 7 + 9 = 16 ≡ 3 (mod 13)
        assert_eq!(a.add(&b, &p).to_uint(), Uint::<4>::from(3u64));
    }

    #[test]
    fn sub_wrap() {
        let p = p13();
        let a = FpNaive::from_u64(3, &p);
        let b = FpNaive::from_u64(7, &p);
        // 3 - 7 ≡ -4 ≡ 9 (mod 13)
        assert_eq!(a.sub(&b, &p).to_uint(), Uint::<4>::from(9u64));
    }

    #[test]
    fn mul_small() {
        let p = p13();
        let a = FpNaive::from_u64(5, &p);
        let b = FpNaive::from_u64(6, &p);
        // 5 * 6 = 30 ≡ 4 (mod 13)
        assert_eq!(a.mul(&b, &p).to_uint(), Uint::<4>::from(4u64));
    }

    #[test]
    fn inv_small() {
        let p = p13();
        let a = FpNaive::from_u64(5, &p);
        let ai = a.inv(&p);
        let one = a.mul(&ai, &p);
        assert_eq!(one, FpNaive::one(&p));
    }

    #[test]
    fn neg_zero_is_zero() {
        let p = p13();
        let z = FpNaive::zero(&p);
        assert_eq!(z.neg(&p), FpNaive::zero(&p));
    }

    #[test]
    fn add_neg_is_zero() {
        let p = secp_p();
        let a = FpNaive::from_u64(0xDEAD_BEEF, &p);
        let neg_a = a.neg(&p);
        let sum = a.add(&neg_a, &p);
        assert_eq!(sum, FpNaive::zero(&p));
    }

    #[test]
    fn mul_secp_prime() {
        // 2 * ((p+1)/2) = p+1 ≡ 1 (mod p) when p is odd.
        let p = secp_p();
        let two = FpNaive::from_u64(2, &p);
        // (p+1)/2: p is odd so this is exact.
        let half = (p.wrapping_add(&Uint::<4>::ONE)) >> 1;
        let half_fp = FpNaive::from_uint(half, &p);
        let one = two.mul(&half_fp, &p);
        assert_eq!(one, FpNaive::one(&p));
    }

    #[test]
    fn pow_fermat() {
        // Fermat's little theorem: a^p ≡ a (mod p) for prime p.
        let p = p13();
        let a = FpNaive::from_u64(7, &p);
        let ap = a.pow(&p, &p);
        assert_eq!(ap, a);
    }

    #[test]
    fn from_uint_reduces() {
        let p = p13();
        // p + 5 should reduce to 5.
        let big = p.wrapping_add(&Uint::<4>::from(5u64));
        let a = FpNaive::from_uint(big, &p);
        assert_eq!(a.to_uint(), Uint::<4>::from(5u64));
    }
}
