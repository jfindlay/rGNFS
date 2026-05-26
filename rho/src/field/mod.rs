//! Field arithmetic abstraction.
//!
//! Re-exports the ``Fp`` trait and concrete implementations from the
//! ``shared-field`` crate, fixing the limb count to ``L = 4`` (256-bit) for
//! backward compatibility with the rest of ``rho``.
//!
//! All ``F: Fp`` bounds in ``rho`` are written as ``F: Fp<4>`` to fix the
//! limb count.  The type aliases ``FpNaive`` and ``FpMonty`` resolve to
//! ``shared_field::FpNaive<4>`` and ``shared_field::FpMonty<4>`` respectively.

// Re-export the generic Fp trait.  All code in rho that imports Fp from here
// and uses it as a bound must write F: Fp<4>.
pub use shared_field::Fp;

/// Type alias: schoolbook 256-bit field element (4 × 64-bit limbs).
///
/// Fixes the const generic ``L = 4`` for all code in ``rho`` that operates on
/// 256-bit primes (secp256k1, P-256, etc.).
pub type FpNaive = shared_field::FpNaive<4>;

/// Type alias: Montgomery-form 256-bit field element (4 × 64-bit limbs).
///
/// Fixes the const generic ``L = 4`` for all code in ``rho`` that operates on
/// 256-bit primes (secp256k1, P-256, etc.).
pub type FpMonty = shared_field::FpMonty<4>;
