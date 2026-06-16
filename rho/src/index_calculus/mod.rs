//! Index-calculus ECDLP solver over a prime-field Weierstrass curve.
//!
//! This module implements the Gaudry–Diem–Joux–Vitse index-calculus algorithm over
//! `E(F_p)`: the "solve" step in the project's transfer/structure/solve triad (E.H
//! transfers via GHS descent, E.J builds the Semaev structure, **E.K solves** via
//! index calculus). It consumes the frozen `rho::semaev` Semaev polynomial surface
//! (C-Semaev) and the frozen `rho::curve::Curve`/`AffinePoint` group law.
//!
//! # Structure
//!
//! - [`mod`] (this file) — `IndexCalcError` enum, module skeleton.
//! - [`strategy`] — `FbPoint`, `IndexCalcStrategy`, `Relation` (C-IndexCalcStrategy,
//!   C-EKRelation, frozen at E.K.1).
//! - [`solve`] — `index_calculus_dlp` — the full pipeline assembler (C-IndexCalc,
//!   frozen at E.K.5 ◆).
//!
//! # Toy fixture
//!
//! The fixture is the `semaev_toy()` curve: `y² = x³ + x + 33` over `F_47` (`p = 47`,
//! `a = 1`, `b = 33`). The generator `G = (10, 3)` has group order `n = 60 = 2²·3·5`.
//! The prime-order subgroup uses `ℓ = 5` (the largest prime factor of `n`); the
//! ℓ-order subgroup generator is `G_ℓ = (n/ℓ)·G = 12·G`.
//!
//! # Principle-4 boundary
//!
//! E.K demonstrates the index-calculus *mechanism* over `E(F_p)` at toy scale. Over
//! `E(F_p)` index calculus is **not** faster than Pollard-rho — the asymptotic speed-up
//! needs the extension-field structure of `E(F_{p^n})` (the genuine Gaudry–Diem setting,
//! a deferred re-shard). The toy `F_p`/`m`/`ℓ` are a principle-4 boundary: mechanism-
//! correct, asymptotic win NOT observable.

pub mod collect;
pub mod decompose;
pub mod linalg;
pub mod solve;
pub mod strategy;

pub use collect::collect_relations;
pub use decompose::decompose;
pub use linalg::{build_ek_matrix, solve_ek_linalg};
pub use solve::index_calculus_dlp;
pub use strategy::{FbPoint, IndexCalcStrategy, Relation, TOY_ELL, TOY_FB_SIZE, TOY_M};

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from the index-calculus ECDLP solver.
///
/// Mirrors the attack-module idiom (`rho::ssa::SsaError`, `rho::ghs::GhsError`,
/// `rho::semaev::SemaevError`): a small `Debug + Clone + PartialEq + Eq` enum with
/// `Display` + `std::error::Error` impls. E.K.2–E.K.5 extend additively as their
/// steps need additional variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexCalcError {
    /// The chosen subgroup modulus ℓ does not divide the group order n, or is not prime.
    ///
    /// The index-calculus linear algebra requires the relation exponents to live in a
    /// field `Z/ℓℤ`; this requires ℓ to be prime and to divide n. A composite or
    /// non-dividing ℓ produces a matrix over a ring, not a field, and the block-Lanczos
    /// engine does not apply.
    InvalidSubgroup {
        /// The subgroup modulus that was rejected.
        ell: u64,
        /// The group order n.
        n: u64,
    },
    /// Factor-base enumeration could not find `requested` QR points (curve too small).
    ///
    /// The toy curve `F_47` has ~30 affine x-coordinates with a QR; `FB_SIZE = 6` is
    /// well within range. This error guards against pathological curves or over-large
    /// `fb_size` requests.
    FactorBaseTooSmall {
        /// The number of factor-base points requested.
        requested: usize,
        /// The number of QR points actually found before exhausting x ∈ [0, p).
        found: usize,
    },
    /// A Semaev or curve operation surfaced an arity or variable error.
    ///
    /// Wraps `SemaevError` for propagation from the point-decomposition step (E.K.2+).
    Semaev(crate::semaev::SemaevError),
    /// The collection loop exhausted all `(a, b)` pairs without finding enough relations.
    ///
    /// Raised by `collect_relations` when the factor base is too sparse relative to the
    /// curve, or the search limit is too low. The `found` field gives the number of
    /// relations collected before giving up; `needed` is `fb_size + 1`.
    UnderdeterminedSystem {
        /// The number of relations found before exhausting the search space.
        found: usize,
        /// The minimum number of relations required (`fb_size + 1`).
        needed: usize,
    },
    /// The Z/ℓℤ linear system has no non-trivial kernel vector.
    ///
    /// Raised by `solve_ek_linalg` when the block Wiedemann / Lanczos solver finds no
    /// kernel vector. This should not occur for an over-determined relation system
    /// (guaranteed by `collect_relations`), but guards against degenerate inputs.
    NoKernel,
    /// DLP recovery from the augmented linear system failed.
    ///
    /// Raised by `index_calculus_dlp` when the Gaussian elimination over `F_ℓ` cannot
    /// recover a unique `log_G(Q) mod ℓ` from the collected relations. This can occur
    /// if all relations have `b_i ≡ 0 mod ℓ` (no information about `Q`'s log) or if
    /// the augmented system is degenerate.
    RecoveryFailed,
}

impl std::fmt::Display for IndexCalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexCalcError::InvalidSubgroup { ell, n } => {
                write!(f, "invalid subgroup: ℓ = {ell} does not divide n = {n}, or is not prime")
            }
            IndexCalcError::FactorBaseTooSmall { requested, found } => {
                write!(
                    f,
                    "factor base too small: requested {requested} QR points, found only {found}"
                )
            }
            IndexCalcError::Semaev(e) => write!(f, "Semaev error: {e}"),
            IndexCalcError::UnderdeterminedSystem { found, needed } => {
                write!(
                    f,
                    "underdetermined system: found {found} relations, need at least {needed}"
                )
            }
            IndexCalcError::NoKernel => {
                write!(f, "no kernel: the Z/ℓℤ linear system has no non-trivial kernel vector")
            }
            IndexCalcError::RecoveryFailed => {
                write!(
                    f,
                    "recovery failed: Gaussian elimination over F_ℓ could not recover \
                     a unique log_G(Q) mod ℓ from the collected relations"
                )
            }
        }
    }
}

impl std::error::Error for IndexCalcError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IndexCalcError::Semaev(e) => Some(e),
            IndexCalcError::InvalidSubgroup { .. }
            | IndexCalcError::FactorBaseTooSmall { .. }
            | IndexCalcError::UnderdeterminedSystem { .. }
            | IndexCalcError::NoKernel
            | IndexCalcError::RecoveryFailed => None,
        }
    }
}

impl From<crate::semaev::SemaevError> for IndexCalcError {
    fn from(e: crate::semaev::SemaevError) -> Self {
        IndexCalcError::Semaev(e)
    }
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_smoke() {
        // Smoke-test Display impls (no panic).
        let _ = format!("{}", IndexCalcError::InvalidSubgroup { ell: 5, n: 60 });
        let _ = format!("{}", IndexCalcError::FactorBaseTooSmall { requested: 6, found: 3 });
        let _ = format!(
            "{}",
            IndexCalcError::Semaev(crate::semaev::SemaevError::DegreeZero)
        );
        let _ = format!("{}", IndexCalcError::NoKernel);
        let _ = format!("{}", IndexCalcError::RecoveryFailed);
    }

    #[test]
    fn toy_strategy_builds() {
        let s = IndexCalcStrategy::toy().expect("toy strategy should build");
        assert_eq!(s.fb_size(), TOY_FB_SIZE);
        assert_eq!(s.m, TOY_M);
        assert_eq!(s.ell, crypto_bigint::Uint::<4>::from(TOY_ELL));
    }
}
