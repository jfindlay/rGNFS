//! Shared utilities.
//!
//! - [`batch_inv`]: Montgomery's batched inversion trick (Phase 7).
//! - [`mp`]: multi-precision helpers beyond `crypto-bigint` if needed.

pub mod batch_inv;
pub mod mp;
