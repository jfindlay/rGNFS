//! Square root stage for GNFS: rational and algebraic square roots, and final assembly.
//!
//! This module is the entry point for the `gnfs::sqrt` sub-crate. It provides:
//!
//! - [`rational`] — rational square root: given a `KernelVector`, recover the relation index
//!   set S via `expand_provenance`, form the product ∏(a − bm) over ℤ, extract its integer
//!   square root X via `isqrt`, and reduce X mod N. Entry point: [`rational::rational_sqrt`].
//! - `algebraic` (G.F.3, not yet implemented) — algebraic square root via Couveignes' CRT
//!   algorithm in ℤ[α].
//! - `assembly` (G.F.4, not yet implemented) — final assembly: combine X and Y, compute
//!   gcd(X − Y, N) to extract a non-trivial factor.
//!
//! # Background
//!
//! The square root stage is the terminal stage of the GNFS factoring pipeline. A `KernelVector`
//! from the linear algebra step (G.E) encodes a subset S of relations whose combined rational
//! and algebraic norms are each perfect squares. This module recovers the original (a, b) pairs
//! through the C-Matrix provenance map and computes:
//!
//! 1. **Rational square root X**: X² ≡ ∏_{i ∈ S}(a_i − b_i·m) (mod N).
//! 2. **Algebraic square root Y** (G.F.3): Y² ≡ Norm(∏_{i ∈ S}(a_i − b_i·α)) (mod N).
//! 3. **Assembly** (G.F.4): gcd(X − Y, N) is a non-trivial factor of N when X ≢ ±Y (mod N).
//!
//! # C-LinAlg seam
//!
//! `KernelVector::expand_provenance` is the G.E → G.F seam, over-specified at G.E.1 precisely
//! for this consumer. G.F.2 is its first real client.

pub mod rational;

pub use rational::rational_sqrt;
