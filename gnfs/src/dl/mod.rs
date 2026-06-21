//! NFS discrete-logarithm (NFS-DL) substrate: Schirokauer map, DL relation format,
//! F_ℓ linear-algebra substrate, individual-logarithm descent, and F_{p^k} extension
//! substrate.
//!
//! This module is the entry point for the NFS-DL substrate. It provides:
//!
//! - [`schirokauer`] — the Schirokauer map λ: K* → (ℤ/ℓ)^r (virtual-log coordinates).
//! - [`DLRelation`] — the DL relation format: the factoring [`Relation`] augmented with
//!   Schirokauer columns (DL relation contract).
//! - [`linalg`] — F_ℓ linear-algebra substrate: block vectors, sparse matrix, operator,
//!   block Lanczos solver, scalar Wiedemann solver, and virtual-log recovery
//!   (F_ℓ linear-algebra contract, frozen at initial implementation, extended with block Wiedemann).
//! - [`descent`] — individual-logarithm descent substrate: descent-tree node/frontier types
//!   (descent substrate contract) and the cross-track `solve_dl` interface
//!   (shape frozen at initial implementation, finalized at integration).
//! - [`ext`] — F_{p^k} extension-field substrate: extension-target type and residue map
//!   (extension-target contract, frozen at initial implementation).
//!
//! # Two-number-field setup
//!
//! NFS-DL uses the same polynomial pair (f, g) as NFS-factoring (the existing [`PolyPair`]).
//! The DL target is log_g(h) in F_p. No `DLPolyPair` is needed — `PolyPair` as-is suffices
//! for the Schirokauer map scope. The Schirokauer map is applied to elements of the algebraic
//! number field K = ℚ[α]/(f(α)).
//!
//! # DL relation contract
//!
//! The DL relation reuses the factoring [`Relation`] (u32 exponent vectors, DL-ready by design)
//! augmented with Schirokauer columns. The augmentation is a wrapper, not a re-typed relation.
//! Re-narrowing the relation type would break the sieve contract (per `sieve/mod.rs` doc).
//!
//! # Schirokauer map contract
//!
//! See [`schirokauer`] module for the map interface. The r > 1 multi-coordinate shape is
//! carried even when toy instances use r = 1 (required for individual-log descent and the
//! MOV bridge).
//!
//! # F_ℓ linear-algebra contract
//!
//! See [`linalg`] module for the F_ℓ block-solver substrate interface. The block Wiedemann
//! extension adds `block_wiedemann_fl` (scalar Wiedemann over F_ℓ) and `recover_virtual_logs`
//! (virtual-log table extraction). Individual-log descent consumes this interface directly.
//!
//! # Descent substrate contract + `solve_dl` interface
//!
//! See [`descent`] module for the descent substrate and the cross-track `solve_dl` interface.
//! The descent substrate is internal to `gnfs::dl`; `solve_dl` is consumed by the MOV bridge.
//!
//! # Extension-target contract
//!
//! See [`ext::target`] module for the F_{p^k} extension-target type and residue map.
//! The extension-target contract is consumed by the extension factor base, the k>1 descent
//! + solver, and the MOV bridge.

pub mod schirokauer;
pub mod relation;
pub mod linalg;
pub mod descent;
pub mod ext;

pub use schirokauer::{schirokauer as compute_schirokauer, PrimeIdeal, SchirokauerError};
pub use relation::{augment_relation, collect_dl_relations, DLMatrix};
pub use linalg::{
    FL_BLOCK_WIDTH,
    FlBlockVec,
    FlSparseMatrix,
    FlSparseRow,
    FlMatrixOperator,
    FlSolution,
    VirtualLogTable,
    bigint_to_fp,
    build_fl_matrix,
    recover_virtual_logs,
    block_lanczos_fl,
    block_wiedemann_fl,
};
pub use descent::{
    DescentFrontier,
    DescentNode,
    DescentTarget,
    DescentStepError,
    DescentSieveConfig,
    InitSmoothingError,
    SolveDlContext,
    SolveDlError,
    assemble_log,
    descend_node,
    init_descent_frontier,
    run_descent,
    solve_dl,
    solve_dl_full,
};
pub use ext::target::{ExtResidueMap, ExtTarget};

use num_bigint::BigInt;

use crate::sieve::Relation;

// ─── DLRelation ───────────────────────────────────────────────────────────────

/// A DL relation: a factoring [`Relation`] augmented with Schirokauer virtual-log columns.
///
/// # DL relation contract
///
/// This is the unit DL relation collection produces and the F_ℓ linear-algebra step consumes.
/// The shape is:
/// - `relation`: the factoring [`Relation`] (u32 exponent vectors, rational + algebraic sides).
///   Reused directly — not re-typed. The integer exponents are read mod ℓ for GF(ℓ) linear
///   algebra, just as they are read mod 2 for GF(2) linear algebra (the factoring path).
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
/// DL relation collection constructs `DLRelation` values by:
/// 1. Collecting a smooth relation (a, b) via the sieve (reusing `line_sieve` / `special_q_sieve`).
/// 2. Evaluating the Schirokauer map on the algebraic element a + b·α.
/// 3. Wrapping the result: `DLRelation { relation, schirokauer_cols }`.
///
/// The F_ℓ linear-algebra step assembles the DL matrix from a collection of `DLRelation` values.
#[derive(Debug, Clone)]
pub struct DLRelation {
    /// The factoring relation (u32 exponent vectors, rational + algebraic sides).
    ///
    /// Reused directly from the NFS-factoring pipeline. Integer exponents are read
    /// mod ℓ for GF(ℓ) linear algebra.
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
