//! F_{p^k} extension-field substrate for NFS-DL: target representation, residue map,
//! extension factor base, and extension relation collection.
//!
//! This module provides the D.E sub-track's extension-field substrate — the types and maps
//! that lift NFS-DL from F_p to F_{p^k}. It is consumed by D.E.3 (descent + solver) and
//! E.C (the MOV bridge).
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
//! # Contract C-ExtFactorBase (frozen D.E.2)
//!
//! See [`factorbase`] for the extension factor base and [`relation`] for the extension
//! relation collection. The contract exposes:
//! - [`factorbase::ExtFactorBase`] — the factor base augmented with the degree-k prime ideal
//!   whose residue field is exactly F_{p^k} (inert/degree-k, not split).
//! - [`relation::ExtDLRelation`] — a DL relation augmented for the extension setting.
//! - [`relation::augment_ext_relation`] — augments a smooth relation with Schirokauer columns
//!   (r>1, exercising C-Schirokauer) and the degree-k prime exponent.
//! - [`relation::collect_ext_dl_relations`] — collects and augments a batch of relations.
//!
//! # Rigidity guard
//!
//! The number field K = ℚ[α]/(f) stays char-0. The residue map is the **only** place where
//! F_{p^k} (char p) meets the sieve algebra. Extension-field arithmetic must not leak into
//! the relation-collection coefficient field.

pub mod target;
pub mod factorbase;
pub mod relation;
