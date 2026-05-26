//! Multi-precision helpers for prime-field arithmetic.
//!
//! - [`batch_inv`]: Montgomery's batched inversion trick, generic over ``Fp<L>``.
//! - [`mp`]: multi-precision helpers beyond ``crypto-bigint`` if needed.

pub mod batch_inv;
pub mod mp;

pub use batch_inv::batch_invert;
