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
//! - [`sieve`] — sieving substrate: two-sided factor bases, rational/algebraic norms, the
//!   norm bridge to `Uint<4>`, and the `Relation` type (C-Relation contract).
//! - [`filter`] — filtering substrate: sparse GF(2) matrix construction, relation-provenance
//!   map, and singleton removal (C-Matrix contract). Entry points: [`filter::build_matrix`],
//!   [`filter::remove_singletons`].
//! - [`sqrt`] — square root stage: rational square root from a kernel vector (G.F.2);
//!   algebraic square root via Couveignes (G.F.3, stub); assembly (G.F.4, stub).
//!   Entry point: [`sqrt::rational_sqrt`].
//!
//! # Pedagogical intent
//!
//! This is a reference library: correctness and clarity take precedence over performance.
//! Each module is annotated with the mathematical background and the science↔engineering
//! tradeoffs that appear at toy vs. cryptographic scale.

pub mod polyselect;
pub mod sieve;
pub mod filter;
pub mod linalg;
pub mod sqrt;

pub use polyselect::{
    select_base_m, select_base_m_with_m, optimal_degree,
    BaseMGenerator, PolyGenerator, PolyPair, PolyPairError,
    score,
    root_sieve, rotate, RootSieveConfig, RootSieveGenerator,
    coppersmith_polys, coppersmith_best, CoppersmithConfig, CoppersmithGenerator,
};

pub use sieve::{
    AlgebraicPrime, FactorBase,
    algebraic_norm, norm_sign, norm_to_uint, rational_norm, NormBridgeError,
    ExponentVector, Relation, RelationError,
    line_sieve, LineSieveConfig,
    special_q_sieve, SpecialQConfig, SpecialQResult,
    lattice_sieve, LatticeBasis, LatticeSieveConfig, LatticeSieveResult,
};

pub use filter::{
    MatrixRow, SparseMatrix, EXCESS_FLOOR,
    build_matrix, remove_singletons,
    prune_cliques, merge_columns,
};

pub use sqrt::{algebraic_sqrt, rational_sqrt};
