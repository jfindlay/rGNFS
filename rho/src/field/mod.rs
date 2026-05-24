//! Field arithmetic abstraction.
//!
//! Defines the [`Fp`] trait that both implementations must satisfy, plus the
//! concrete [`FpNaive`] and [`FpMonty`] types.

pub mod monty;
pub mod naive;

pub use monty::FpMonty;
pub use naive::FpNaive;

use crypto_bigint::Uint;

/// Prime-field arithmetic over GF(p).
///
/// All values are implicitly reduced mod p. Implementations are allowed to use
/// internal representations (e.g., Montgomery form) as long as [`to_uint`]
/// returns the canonical residue in `[0, p)`.
///
/// The const generic `L` is the limb count of `Uint<L>` and must be chosen so
/// that `L * 64 >= bit-width(p)`.
pub trait Fp: Clone + PartialEq + Eq + std::fmt::Debug + Send + Sync + 'static {
    /// Number of 64-bit limbs in the underlying `Uint`.
    const LIMBS: usize;

    /// Additive identity.
    fn zero(p: &Uint<4>) -> Self;

    /// Multiplicative identity.
    fn one(p: &Uint<4>) -> Self;

    /// Construct from a small `u64` value, reducing mod p.
    fn from_u64(v: u64, p: &Uint<4>) -> Self;

    /// Construct from an arbitrary `Uint<4>`, reducing mod p.
    fn from_uint(v: Uint<4>, p: &Uint<4>) -> Self;

    /// Return the canonical residue in `[0, p)`.
    fn to_uint(&self) -> Uint<4>;

    /// Modular addition.
    fn add(&self, rhs: &Self, p: &Uint<4>) -> Self;

    /// Modular subtraction.
    fn sub(&self, rhs: &Self, p: &Uint<4>) -> Self;

    /// Modular negation.
    fn neg(&self, p: &Uint<4>) -> Self;

    /// Modular multiplication.
    fn mul(&self, rhs: &Self, p: &Uint<4>) -> Self;

    /// Modular squaring (`x * x`). May be faster than `mul(x, x)`.
    fn square(&self, p: &Uint<4>) -> Self;

    /// Modular exponentiation: `self^exp mod p`.
    fn pow(&self, exp: &Uint<4>, p: &Uint<4>) -> Self;

    /// Modular inverse via Fermat: `self^(p-2) mod p`.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero (no inverse exists).
    fn inv(&self, p: &Uint<4>) -> Self;

    /// Return `true` if this element is zero.
    fn is_zero(&self, _p: &Uint<4>) -> bool {
        self.to_uint() == Uint::<4>::ZERO
    }
}
