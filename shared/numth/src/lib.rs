//! Number-theoretic primitives for the rGNFS project.
//!
//! This crate provides three families of primitives consumed by NFS sieving
//! (`gnfs::sieve`), NFS-DL relation collection (`gnfs::dl`), and ECDLP index calculus
//! (`rho::index_calculus`):
//!
//! - **Primality testing** ([`prime`]): Miller–Rabin with deterministic witness
//!   sets for ``n < 3,317,044,064,679,887,385,961,981``, falling back to a
//!   12-round probabilistic test for larger ``Uint<4>`` values.
//!
//! - **B-smoothness detection** ([`smooth`]): trial-division factorisation over
//!   a prime factor base, returning a [`SmoothWitness`] that carries the
//!   complete factorisation and an unfactored cofactor.  Designed for all three
//!   downstream consumers (see the module-level note in [`smooth`]).
//!
//! - **Elliptic Curve Method** ([`ecm`]): Lenstra's ECM for integer
//!   factorization using Montgomery-form curves (Suyama parameterization).
//!   Used as a sub-step inside NFS large-prime variations (`gnfs::sieve`) and as a
//!   fallback for factoring composite group orders in Pohlig–Hellman (`rho::ssa`).
//!
//! # Re-exports
//!
//! The most commonly used items are re-exported at the crate root:
//!
//! ```rust
//! use shared_numth::{
//!     is_prime, miller_rabin,
//!     SmoothWitness, trial_smooth, factor_base_up_to,
//!     EcmResult, ecm_one_curve, ecm_factor,
//! };
//! ```

pub mod ecm;
pub mod prime;
pub mod smooth;

pub use ecm::{ecm_factor, ecm_one_curve, EcmResult};
pub use prime::{is_prime, miller_rabin};
pub use smooth::{factor_base_up_to, trial_smooth, SmoothWitness};
