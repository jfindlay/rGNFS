//! NFS discrete-logarithm (NFS-DL) substrate: Schirokauer map and DL relation format.
//!
//! This module is the entry point for the NFS-DL bridge sub-track (Track D). It provides:
//!
//! - [`schirokauer`] — the Schirokauer map λ: K* → (ℤ/ℓ)^r (virtual-log coordinates).
//! - [`DLRelation`] — the DL relation format: the factoring [`Relation`] augmented with
//!   Schirokauer columns (C-DLRelation contract).
//!
//! # Two-number-field setup
//!
//! NFS-DL uses the same polynomial pair (f, g) as NFS-factoring (the existing [`PolyPair`]).
//! The DL target is log_g(h) in F_p. No `DLPolyPair` is needed — `PolyPair` as-is suffices
//! for D.A.1's scope. The Schirokauer map is applied to elements of the algebraic number
//! field K = ℚ[α]/(f(α)).
//!
//! # Contract C-DLRelation (frozen D.A.1)
//!
//! The DL relation reuses the factoring [`Relation`] (u32 exponent vectors, DL-ready by design)
//! augmented with Schirokauer columns. The augmentation is a wrapper, not a re-typed relation.
//! Re-narrowing C-Relation would be a destructive reshard (per `sieve/mod.rs` doc).
//!
//! # Contract C-Schirokauer (frozen D.A.1)
//!
//! See [`schirokauer`] module for the map interface. The r > 1 multi-coordinate shape is
//! carried even when toy instances use r = 1 (required for D.C descent and E.C solver).

pub mod schirokauer;
pub mod relation;

pub use schirokauer::{schirokauer as compute_schirokauer, PrimeIdeal, SchirokauerError};
pub use relation::{augment_relation, collect_dl_relations, DLMatrix};

use num_bigint::BigInt;

use crate::sieve::Relation;

// ─── DLRelation ───────────────────────────────────────────────────────────────

/// A DL relation: a factoring [`Relation`] augmented with Schirokauer virtual-log columns.
///
/// # Contract C-DLRelation (frozen D.A.1)
///
/// This is the unit D.A.2 produces and D.B consumes. The shape is:
/// - `relation`: the factoring [`Relation`] (u32 exponent vectors, rational + algebraic sides).
///   Reused directly — not re-typed. The integer exponents are read mod ℓ for GF(ℓ) linear
///   algebra (D.B), just as they are read mod 2 for GF(2) linear algebra (G.E).
/// - `schirokauer_cols`: the virtual-log coordinates from the Schirokauer map, one `BigInt`
///   per prime ideal in the Schirokauer ideal set. These are the extra columns that make the
///   DL linear system solvable over F_ℓ.
///
/// # Augmentation design
///
/// The factoring `Relation` is DL-ready by design (see `sieve/mod.rs` C-Relation contract).
/// `DLRelation` wraps it with the Schirokauer columns rather than re-typing the relation.
/// This preserves the C-Relation contract and avoids a destructive reshard.
///
/// # Usage
///
/// D.A.2 constructs `DLRelation` values by:
/// 1. Collecting a smooth relation (a, b) via the sieve (reusing `line_sieve` / `special_q_sieve`).
/// 2. Evaluating the Schirokauer map on the algebraic element a + b·α.
/// 3. Wrapping the result: `DLRelation { relation, schirokauer_cols }`.
///
/// D.B assembles the DL matrix from a collection of `DLRelation` values.
#[derive(Debug, Clone)]
pub struct DLRelation {
    /// The factoring relation (u32 exponent vectors, rational + algebraic sides).
    ///
    /// Reused directly from the NFS-factoring pipeline. Integer exponents are read
    /// mod ℓ for GF(ℓ) linear algebra (D.B).
    pub relation: Relation,

    /// Schirokauer virtual-log columns: one `BigInt` per prime ideal in the Schirokauer set.
    ///
    /// Each entry is λ_i(β) ∈ ℤ/ℓ, the ℓ-adic virtual-log coordinate for the i-th ideal.
    /// The length equals the number of ideals passed to [`compute_schirokauer`].
    pub schirokauer_cols: Vec<BigInt>,
}

impl DLRelation {
    /// Construct a `DLRelation` from a factoring relation and Schirokauer columns.
    ///
    /// :param relation: The factoring relation (from the sieve).
    /// :param schirokauer_cols: Virtual-log coordinates from [`compute_schirokauer`].
    pub fn new(relation: Relation, schirokauer_cols: Vec<BigInt>) -> Self {
        Self { relation, schirokauer_cols }
    }

    /// The number of Schirokauer columns (r, the virtual-log dimension).
    pub fn schirokauer_rank(&self) -> usize {
        self.schirokauer_cols.len()
    }
}
