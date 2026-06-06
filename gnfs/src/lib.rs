//! General Number Field Sieve (GNFS): polynomial selection, sieving, and factorization.
//!
//! This crate implements the GNFS pipeline for integer factorization and discrete logarithm
//! computation. It builds on the number-field substrate in `shared-numfield` (polynomials,
//! number fields, resultants, Dedekind factorisation) and the number-theory utilities in
//! `shared-numth` (primality, smoothness).
//!
//! # Structure
//!
//! - [`polyselect`] — polynomial selection: base-m expansion, Murphy-E scoring, root sieve,
//!   Coppersmith multi-poly. Entry point: [`polyselect::select_base_m`].
//!
//! # Pedagogical intent
//!
//! This is a reference library: correctness and clarity take precedence over performance.
//! Each module is annotated with the mathematical background and the science↔engineering
//! tradeoffs that appear at toy vs. cryptographic scale.

pub mod polyselect;

pub use polyselect::{
    select_base_m, select_base_m_with_m, optimal_degree,
    BaseMGenerator, PolyGenerator, PolyPair, PolyPairError,
    score,
    root_sieve, rotate, RootSieveConfig, RootSieveGenerator,
};
