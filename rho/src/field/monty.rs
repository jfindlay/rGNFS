//! Montgomery-form field arithmetic over GF(p).
//!
//! `FpMonty` wraps `crypto_bigint::modular::runtime_mod::DynResidue` to get
//! the Montgomery multiplication speedup.  The internal representation is
//! `aR mod p` where `R = 2^(64·L)`.  Conversion to/from canonical form is
//! transparent via [`Fp::from_uint`] / [`Fp::to_uint`].
//!
//! Phase 1 will add the field benchmark that quantifies the speedup over
//! [`FpNaive`].

use crypto_bigint::{
    modular::runtime_mod::{DynResidue, DynResidueParams},
    Uint,
};
use super::Fp;

/// Montgomery-form field element backed by `crypto-bigint`'s `DynResidue`.
///
/// The `DynResidue` carries its `DynResidueParams` internally, which encodes
/// the modulus and the Montgomery constant.  We copy the params as a `Copy`
/// field so we can reconstruct elements after operations that consume `self`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpMonty {
    /// Inner `DynResidue` (Montgomery form `v * R mod p`).
    inner: DynResidue<4>,
    /// Cached params — re-used on every operation.
    params: DynResidueParams<4>,
}

impl FpMonty {
    /// Build `DynResidueParams` from a modulus `p` (must be odd).
    fn params_from(p: &Uint<4>) -> DynResidueParams<4> {
        DynResidueParams::<4>::new(p)
    }

    /// Wrap a `DynResidue` together with its params.
    fn wrap(inner: DynResidue<4>, params: DynResidueParams<4>) -> Self {
        Self { inner, params }
    }
}

impl Fp for FpMonty {
    const LIMBS: usize = 4;

    fn zero(p: &Uint<4>) -> Self {
        let params = Self::params_from(p);
        let inner = DynResidue::zero(params);
        Self::wrap(inner, params)
    }

    fn one(p: &Uint<4>) -> Self {
        let params = Self::params_from(p);
        let inner = DynResidue::one(params);
        Self::wrap(inner, params)
    }

    fn from_u64(v: u64, p: &Uint<4>) -> Self {
        let params = Self::params_from(p);
        let inner = DynResidue::new(&Uint::<4>::from(v), params);
        Self::wrap(inner, params)
    }

    fn from_uint(v: Uint<4>, p: &Uint<4>) -> Self {
        let params = Self::params_from(p);
        let inner = DynResidue::new(&v, params);
        Self::wrap(inner, params)
    }

    fn to_uint(&self) -> Uint<4> {
        self.inner.retrieve()
    }

    fn add(&self, rhs: &Self, _p: &Uint<4>) -> Self {
        let result = self.inner.add(&rhs.inner);
        Self::wrap(result, self.params)
    }

    fn sub(&self, rhs: &Self, _p: &Uint<4>) -> Self {
        let result = self.inner.sub(&rhs.inner);
        Self::wrap(result, self.params)
    }

    fn neg(&self, _p: &Uint<4>) -> Self {
        let result = self.inner.neg();
        Self::wrap(result, self.params)
    }

    fn mul(&self, rhs: &Self, _p: &Uint<4>) -> Self {
        let result = self.inner.mul(&rhs.inner);
        Self::wrap(result, self.params)
    }

    fn square(&self, _p: &Uint<4>) -> Self {
        let result = self.inner.square();
        Self::wrap(result, self.params)
    }

    fn pow(&self, exp: &Uint<4>, _p: &Uint<4>) -> Self {
        let result = self.inner.pow(exp);
        Self::wrap(result, self.params)
    }

    fn inv(&self, p: &Uint<4>) -> Self {
        assert!(!self.is_zero(p), "attempted inversion of zero");
        let exp = p.wrapping_sub(&Uint::<4>::from(2u64));
        self.pow(&exp, p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p13() -> Uint<4> {
        Uint::<4>::from(13u64)
    }

    fn secp_p() -> Uint<4> {
        Uint::<4>::from_be_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        )
    }

    #[test]
    fn add_small() {
        let p = p13();
        let a = FpMonty::from_u64(7, &p);
        let b = FpMonty::from_u64(9, &p);
        assert_eq!(a.add(&b, &p).to_uint(), Uint::<4>::from(3u64));
    }

    #[test]
    fn sub_wrap() {
        let p = p13();
        let a = FpMonty::from_u64(3, &p);
        let b = FpMonty::from_u64(7, &p);
        assert_eq!(a.sub(&b, &p).to_uint(), Uint::<4>::from(9u64));
    }

    #[test]
    fn mul_small() {
        let p = p13();
        let a = FpMonty::from_u64(5, &p);
        let b = FpMonty::from_u64(6, &p);
        assert_eq!(a.mul(&b, &p).to_uint(), Uint::<4>::from(4u64));
    }

    #[test]
    fn inv_small() {
        let p = p13();
        let a = FpMonty::from_u64(5, &p);
        let ai = a.inv(&p);
        let one = a.mul(&ai, &p);
        assert_eq!(one, FpMonty::one(&p));
    }

    #[test]
    fn neg_zero_is_zero() {
        let p = p13();
        let z = FpMonty::zero(&p);
        assert_eq!(z.neg(&p), FpMonty::zero(&p));
    }

    #[test]
    fn add_neg_is_zero() {
        let p = secp_p();
        let a = FpMonty::from_u64(0xDEAD_BEEF, &p);
        let neg_a = a.neg(&p);
        let sum = a.add(&neg_a, &p);
        assert_eq!(sum, FpMonty::zero(&p));
    }

    #[test]
    fn mul_secp_prime() {
        let p = secp_p();
        let two = FpMonty::from_u64(2, &p);
        let half = (p.wrapping_add(&Uint::<4>::ONE)) >> 1;
        let half_fp = FpMonty::from_uint(half, &p);
        let one = two.mul(&half_fp, &p);
        assert_eq!(one, FpMonty::one(&p));
    }

    #[test]
    fn matches_naive() {
        // Cross-check FpMonty against FpNaive for a set of operations on the secp256k1 prime.
        use crate::field::naive::FpNaive;
        use crate::field::Fp;

        let p = secp_p();
        let vals: &[u64] = &[0, 1, 2, 12345, 0xDEAD_BEEF, 0xFFFF_FFFF_FFFF];

        for &a_u in vals {
            for &b_u in vals {
                let an = FpNaive::from_u64(a_u, &p);
                let bn = FpNaive::from_u64(b_u, &p);
                let am = FpMonty::from_u64(a_u, &p);
                let bm = FpMonty::from_u64(b_u, &p);

                assert_eq!(
                    an.add(&bn, &p).to_uint(),
                    am.add(&bm, &p).to_uint(),
                    "add mismatch a={a_u} b={b_u}"
                );
                assert_eq!(
                    an.sub(&bn, &p).to_uint(),
                    am.sub(&bm, &p).to_uint(),
                    "sub mismatch a={a_u} b={b_u}"
                );
                assert_eq!(
                    an.mul(&bn, &p).to_uint(),
                    am.mul(&bm, &p).to_uint(),
                    "mul mismatch a={a_u} b={b_u}"
                );
                if b_u != 0 {
                    assert_eq!(
                        an.mul(&bn.inv(&p), &p).to_uint(),
                        am.mul(&bm.inv(&p), &p).to_uint(),
                        "inv-mul mismatch a={a_u} b={b_u}"
                    );
                }
            }
        }
    }
}
