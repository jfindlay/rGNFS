//! F_ℓ linear-algebra substrate for NFS-DL: block vectors, sparse matrix, operator, and solvers.
//!
//! This module is the entry point for the `gnfs::dl::linalg` sub-module. It provides:
//!
//! - [`blockvec_fl`] — `FlBlockVec`, `FlSparseMatrix`, `FlSparseRow`, `FlMatrixOperator`,
//!   `FlSolution`, `FL_BLOCK_WIDTH`, `bigint_to_fp`, `build_fl_matrix`,
//!   `VirtualLogTable`, `recover_virtual_logs`.
//! - [`lanczos_fl`] — `block_lanczos_fl`: block Lanczos over F_ℓ.
//! - [`wiedemann_fl`] — `block_wiedemann_fl`: scalar Wiedemann over F_ℓ (D.B.2).
//!
//! # Parallel-module design (C-LinAlgFl contract)
//!
//! This module is **parallel** to `gnfs::linalg` (the frozen GF(2) substrate). No shared
//! trait is introduced; the GF(2) types remain frozen and untouched. The duplication is
//! intentional: F_ℓ scalars are ~256-bit field elements, not bits, so the GF(2) bit-packing
//! is inapplicable. See the C-LinAlgFl contract in `docs/PLAN.md`.
//!
//! # Contract C-LinAlgFl (frozen D.B.1; extended D.B.2)
//!
//! The types and functions in this module implement the C-LinAlgFl contract frozen at D.B.1
//! and extended at D.B.2 (block Wiedemann + virtual-log recovery). D.C (individual-log
//! descent) consumes this interface directly.

pub mod blockvec_fl;
pub mod lanczos_fl;
pub mod wiedemann_fl;

pub use blockvec_fl::{
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
};
pub use lanczos_fl::block_lanczos_fl;
pub use wiedemann_fl::block_wiedemann_fl;
