//! Linear algebra substrate for GNFS: blocked GF(2) vectors, matrix operator, kernel
//! vectors, quadratic-character columns, and the block Lanczos and Wiedemann nullspace
//! solvers.
//!
//! This module is the entry point for the `gnfs::linalg` sub-crate. It provides:
//!
//! - [`blockvec`] — `BlockVec` and `BLOCK_WIDTH`: blocked GF(2) vector representation
//!   for block Lanczos and Wiedemann solvers.
//! - [`operator`] — `MatrixOperator`: sparse matrix as a linear operator over GF(2).
//! - [`kernel`] — `KernelVector`: nullspace vector representation (the linear algebra → square root seam).
//! - [`qc`] — `populate_qc_columns`, `select_qc_primes`, `DEFAULT_NUM_QC`: quadratic-
//!   character column construction.
//! - [`lanczos`] — `block_lanczos`: Montgomery's block Lanczos nullspace solver.
//! - [`wiedemann`] — `block_wiedemann`: Coppersmith's block Wiedemann nullspace solver.
//!
//! # Background
//!
//! The linear algebra step takes the filtered sparse GF(2) matrix from the filtering step
//! and finds vectors in its left nullspace. Each nullspace vector corresponds to a set of
//! relations whose product is a perfect square on both sides — the raw material for the
//! congruence of squares in the square root step.
//!
//! # Linear algebra contract
//!
//! The types and functions in this module implement the GF(2) linear-algebra substrate.
//! The block Lanczos solver, Wiedemann solver, square root step, and the F_ℓ extension
//! consume this interface directly.

pub mod blockvec;
pub mod operator;
pub mod lanczos;
pub mod wiedemann;
mod kernel;
mod qc;

pub use blockvec::{BlockVec, BLOCK_WIDTH};
pub use operator::MatrixOperator;
pub use kernel::KernelVector;
pub use qc::{populate_qc_columns, select_qc_primes, DEFAULT_NUM_QC};
pub use lanczos::block_lanczos;
pub use wiedemann::block_wiedemann;
