//! Individual-logarithm descent substrate for NFS-DL.
//!
//! This module is the entry point for the D.C descent sub-track. It provides:
//!
//! - [`node`] — descent-tree data structures: [`DescentTarget`], [`DescentNode`],
//!   [`DescentFrontier`] (C-Descent contract, frozen D.C.1).
//! - [`solve`] — the C2 `solve_dl` interface and initialization-smoothing:
//!   [`solve_dl`], [`init_descent_frontier`], [`descend_node`], and the error types
//!   [`SolveDlError`], [`InitSmoothingError`], [`DescentStepError`].
//!
//! # Contract C-Descent (frozen D.C.1)
//!
//! The descent substrate is sub-track-internal: consumed by D.C.2 (special-q recursion) and
//! D.C.3 (log assembly), but not exposed outside Track D. The types are re-exported here for
//! convenience; callers within Track D import from `gnfs::dl::descent`.
//!
//! # Contract C2 (shape frozen D.C.1, finalized D.C.3)
//!
//! `solve_dl` is the cross-track interface consumed by E.C (the MOV bridge). Its signature and
//! the `SolveDlError` shape are frozen at D.C.1. The error taxonomy may be extended additively
//! at D.C.3 once the full pipeline is integrated.

pub mod node;
pub mod solve;

pub use node::{DescentFrontier, DescentNode, DescentTarget};
pub use solve::{
    DescentStepError, InitSmoothingError, SolveDlError, descend_node, init_descent_frontier,
    solve_dl,
};
