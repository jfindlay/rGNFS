//! F_ℓ linear-algebra substrate for NFS-DL: block vectors, sparse matrix, operator, and solvers.
//!
//! This module is the entry point for the `gnfs::dl::linalg` sub-module. It provides:
//!
//! - [`blockvec_fl`] — `FlBlockVec`, `FlSparseMatrix`, `FlSparseRow`, `FlMatrixOperator`,
//!   `FlSolution`, `FL_BLOCK_WIDTH`, `bigint_to_fp`, `build_fl_matrix`,
//!   `VirtualLogTable`, `recover_virtual_logs`.
//! - [`lanczos_fl`] — `block_lanczos_fl`: block Lanczos over F_ℓ.
//! - [`wiedemann_fl`] — `block_wiedemann_fl`: scalar Wiedemann over F_ℓ.
//!
//! # Parallel-module design
//!
//! This module is **parallel** to `gnfs::linalg` (the GF(2) substrate). No shared
//! trait is introduced; the GF(2) types remain untouched. The duplication is
//! intentional: F_ℓ scalars are ~256-bit field elements, not bits, so the GF(2) bit-packing
//! is inapplicable.
//!
//! # F_ℓ linear-algebra contract
//!
//! The types and functions in this module implement the F_ℓ linear-algebra substrate.
//! The block Wiedemann extension adds `block_wiedemann_fl` and `recover_virtual_logs`
//! (virtual-log table extraction). Individual-log descent consumes this interface directly.

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
