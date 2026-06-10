//! F_{p^k} extension-field substrate for NFS-DL: target representation and residue map.
//!
//! This module provides the D.E sub-track's extension-field substrate — the types and maps
//! that lift NFS-DL from F_p to F_{p^k}. It is consumed by D.E.2 (extension factor base),
//! D.E.3 (k>1 descent + `solve_dl` wiring), and E.C (the MOV bridge).
//!
//! # Contract C-ExtTarget (frozen D.E.1)
//!
//! See [`target`] for the extension-target type and residue map. The contract exposes:
//! - [`target::ExtTarget`] — the representation of `g, h ∈ F_{p^k}*` the k>1 solver reads.
//! - [`target::ExtResidueMap`] — the residue map F_{p^k} ↔ residue field of the degree-k
//!   prime ideal in K = ℚ[α]/(f).
//! - [`target::ExtTarget::from_coeffs`] — constructor accepting the `Vec<BigInt>` coefficient
//!   form that `FpExt::to_uint_vec` already produces (over-specified for E.C).
//!
//! # Rigidity guard
//!
//! The number field K = ℚ[α]/(f) stays char-0. The residue map is the **only** place where
//! F_{p^k} (char p) meets the sieve algebra. Extension-field arithmetic must not leak into
//! the relation-collection coefficient field.

pub mod target;
