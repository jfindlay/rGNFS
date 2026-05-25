//! Integer factorization via Pollard rho.
//!
//! Optimization layers (per the plan):
//! 1. Floyd's cycle detection — baseline.
//! 2. Brent's cycle detection — ~24% fewer f-evaluations.
//! 3. Montgomery batched GCD — accumulate products, single GCD per batch.
//! 4. Multi-c parallel restart — `rayon` over a set of `c` values.
//!
//! All layers share the pseudorandom function `f(x) = x² + c mod N`.
//! Public entry point: [`rho::factor`] (parallel, batched); individual variants
//! are also exported for benchmarking.

pub mod rho;

pub use rho::{brent, brent_batched, factor, floyd};
