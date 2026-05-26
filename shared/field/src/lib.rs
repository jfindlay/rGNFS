//! Prime-field arithmetic over GF(p), generic over the limb count.
//!
//! This crate exposes the [`Fp`] trait and two concrete implementations:
//! [`FpNaive`] (schoolbook arithmetic) and [`FpMonty`] (Montgomery form via
//! ``crypto-bigint``'s ``DynResidue``).
//!
//! # Design: const-generic-on-trait approach
//!
//! The trait is parameterised as ``Fp<const L: usize>`` where ``L`` is the
//! number of 64-bit limbs in the underlying ``Uint<L>``.  This is stable Rust
//! (edition 2024) and avoids the nightly-only ``generic_const_exprs`` feature
//! that would be required by the alternative approach of an associated constant
//! ``const LIMBS: usize`` with a ``type Uint = Uint<{Self::LIMBS}>`` associated
//! type.
//!
//! Callers that want the 256-bit (4-limb) field used throughout ``rho`` can
//! write ``F: Fp<4>`` or use the type aliases ``FpNaive4`` / ``FpMonty4``
//! exported from this crate.

pub mod monty;
pub mod naive;

pub use monty::FpMonty;
pub use naive::FpNaive;

/// Type alias: schoolbook 256-bit field element (4 × 64-bit limbs).
pub type FpNaive4 = FpNaive<4>;

/// Type alias: Montgomery-form 256-bit field element (4 × 64-bit limbs).
pub type FpMonty4 = FpMonty<4>;

use crypto_bigint::Uint;

/// Prime-field arithmetic over GF(p), generic over the limb count ``L``.
///
/// All values are implicitly reduced mod p.  Implementations are allowed to use
/// internal representations (e.g., Montgomery form) as long as [`to_uint`]
/// returns the canonical residue in ``[0, p)``.
///
/// # Const-generic design
///
/// The trait is parameterised as ``Fp<const L: usize>`` where ``L`` is the
/// number of 64-bit limbs in ``Uint<L>``.  Choose ``L`` so that
/// ``L * 64 >= bit-width(p)``.  For example, ``L = 4`` covers 256-bit primes
/// (secp256k1, P-256, etc.).
///
/// # Deferred methods
///
/// ``legendre(p) -> i8`` and ``sqrt(p) -> Option<Self>`` (Tonelli–Shanks) are
/// not included here because they require knowing the prime is prime and are
/// non-trivial to implement generically without additional trait bounds.  They
/// will be added in a later session once the ``shared::numth`` crate provides
/// the necessary primality infrastructure.
pub trait Fp<const L: usize>: Clone + PartialEq + Eq + std::fmt::Debug + Send + Sync + 'static {
    /// Additive identity: 0 mod p.
    fn zero(p: &Uint<L>) -> Self;

    /// Multiplicative identity: 1 mod p.
    fn one(p: &Uint<L>) -> Self;

    /// Construct from a small ``u64`` value, reducing mod p.
    fn from_u64(v: u64, p: &Uint<L>) -> Self;

    /// Construct from an arbitrary ``Uint<L>``, reducing mod p.
    fn from_uint(v: Uint<L>, p: &Uint<L>) -> Self;

    /// Return the canonical residue in ``[0, p)``.
    fn to_uint(&self) -> Uint<L>;

    /// Modular addition: ``self + rhs mod p``.
    fn add(&self, rhs: &Self, p: &Uint<L>) -> Self;

    /// Modular subtraction: ``self - rhs mod p``.
    fn sub(&self, rhs: &Self, p: &Uint<L>) -> Self;

    /// Modular negation: ``-self mod p``.
    fn neg(&self, p: &Uint<L>) -> Self;

    /// Modular multiplication: ``self * rhs mod p``.
    fn mul(&self, rhs: &Self, p: &Uint<L>) -> Self;

    /// Modular squaring: ``self^2 mod p``.
    ///
    /// May be faster than ``mul(self, self)`` for implementations that exploit
    /// the squaring structure (e.g., Karatsuba squaring).
    fn square(&self, p: &Uint<L>) -> Self;

    /// Modular exponentiation: ``self^exp mod p``.
    fn pow(&self, exp: &Uint<L>, p: &Uint<L>) -> Self;

    /// Modular inverse via Fermat's little theorem: ``self^(p-2) mod p``.
    ///
    /// # Panics
    ///
    /// Panics if ``self`` is zero (no inverse exists).
    fn inv(&self, p: &Uint<L>) -> Self;

    /// Return ``true`` if this element is zero.
    ///
    /// Default implementation compares ``to_uint()`` to ``Uint::<L>::ZERO``.
    /// Implementations may override for efficiency.
    fn is_zero(&self, _p: &Uint<L>) -> bool {
        self.to_uint() == Uint::<L>::ZERO
    }

    /// Return ``true`` if this element is the multiplicative identity (1 mod p).
    ///
    /// Default implementation compares ``to_uint()`` to ``Uint::<L>::ONE``.
    fn is_one(&self, _p: &Uint<L>) -> bool {
        self.to_uint() == Uint::<L>::ONE
    }

    /// Modular doubling: ``2 * self mod p``.
    ///
    /// Default implementation delegates to ``add(self, self, p)``.
    /// Implementations may override with a faster shift-based path.
    fn double(&self, p: &Uint<L>) -> Self {
        self.add(self, p)
    }
}
