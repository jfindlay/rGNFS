//! Individual-logarithm descent substrate for NFS-DL.
//!
//! This module is the entry point for the NFS-DL individual-log descent. It provides:
//!
//! - [`node`] — descent-tree data structures: [`DescentTarget`], [`DescentNode`],
//!   [`DescentFrontier`] (descent substrate contract).
//! - [`solve`] — the `solve_dl` interface and initialization-smoothing:
//!   [`solve_dl`], [`init_descent_frontier`], and the error types
//!   [`SolveDlError`], [`InitSmoothingError`], [`DescentStepError`].
//! - [`recurse`] — the special-q descent recursion: [`descend_node`], [`run_descent`],
//!   and [`DescentSieveConfig`].
//!
//! # Descent substrate contract
//!
//! The descent substrate is internal to `gnfs::dl`: consumed by the special-q recursion and
//! log assembly, but not exposed outside `gnfs::dl`. The types are re-exported here for
//! convenience; callers within `gnfs::dl` import from `gnfs::dl::descent`.
//!
//! # `solve_dl` interface
//!
//! `solve_dl` is the cross-track interface consumed by the MOV bridge. Its signature and
//! the `SolveDlError` shape are frozen. The error taxonomy was finalized once the full
//! pipeline was integrated.

pub mod node;
pub mod recurse;
pub mod solve;

pub use node::{DescentFrontier, DescentNode, DescentTarget};
pub use recurse::{DescentSieveConfig, descend_node, run_descent};
pub use solve::{
    DescentStepError,
    InitSmoothingError,
    SolveDlContext,
    SolveDlError,
    assemble_log,
    init_descent_frontier,
    solve_dl,
    solve_dl_full,
};
