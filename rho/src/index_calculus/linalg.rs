//! Z/ℓZ linear algebra for the index-calculus ECDLP solver.
//!
//! This module provides the relation→`FlSparseMatrix` adapter and the Z/ℓZ solve wrapper
//! that form the linear-algebra step of the index-calculus pipeline. It reuses the
//! `gnfs::dl::linalg` engine (block Lanczos / Wiedemann over F_ℓ) with an index-calculus-
//! specific adapter in place of the NFS-bound `build_fl_matrix(DLMatrix)`.
//!
//! # Adapter design
//!
//! Each `Relation.exponents` is already in the `Vec<(usize, FpNaive)>` shape that maps
//! directly to `FlSparseRow { entries: Vec<(usize, F)> }` — the adapter is a near-identity
//! copy (no re-encoding). This shape was chosen specifically so the linear-algebra step
//! would not need a scalar adapter.
//!
//! # Solver choice
//!
//! `solve_ek_linalg` uses `block_wiedemann_fl` (Gaussian elimination at toy scale) as the
//! primary solver. `block_lanczos_fl` is also available and produces equivalent results.
//! At toy scale (6×7 matrix over F_5) both are fast; Wiedemann's Gaussian-elimination
//! fallback is more reliable over small fields (avoids BM probabilistic failure modes).
//!
//! # Principle-4 boundary
//!
//! The toy fixture is F_47, n = 60, ℓ = 5, m = 2, FB_SIZE = 6. The algorithms are
//! mechanism-correct; the asymptotic index-calculus win (which needs E(F_{p^n})) is not
//! observable at this scale — a deferred re-shard.

use gnfs::dl::linalg::{FlMatrixOperator, FlSparseMatrix, FlSparseRow, block_wiedemann_fl};
use shared_field::FpNaive4 as FpNaive;
use crate::index_calculus::strategy::{IndexCalcStrategy, Relation};
use crate::index_calculus::IndexCalcError;

// ─── build_ek_matrix ─────────────────────────────────────────────────────────

/// Build the relation matrix for the index-calculus linear algebra step.
///
/// Each relation becomes a sparse row over F_ℓ: the exponent vector (factor-base index,
/// exp mod ℓ) pairs map directly to `FlSparseRow` entries. The matrix has
/// `relations.len()` rows and `strategy.fb_size()` columns.
///
/// The adapter is a near-identity copy: `Relation.exponents` is already in the
/// `Vec<(usize, FpNaive)>` shape that `FlSparseRow` expects (sorted by index, no zeros,
/// no duplicates — invariants enforced by `Relation::from_decomposition`).
pub fn build_ek_matrix(relations: &[Relation], strategy: &IndexCalcStrategy) -> FlSparseMatrix<FpNaive> {
    let num_cols = strategy.fb_size();

    let rows = relations
        .iter()
        .map(|rel| {
            // Near-identity adapter: clone the (index, FpNaive) pairs directly.
            // The Relation invariants (sorted, no zeros, no duplicates) match the
            // FlSparseRow CSR invariant — no re-encoding needed.
            let entries: Vec<(usize, FpNaive)> = rel.exponents.clone();
            FlSparseRow { entries }
        })
        .collect();

    FlSparseMatrix { rows, num_cols }
}

// ─── solve_ek_linalg ─────────────────────────────────────────────────────────

/// Solve the Z/ℓZ relation system: find a kernel vector of the relation matrix.
///
/// Returns the kernel vector (a `Vec<FpNaive>` of length `fb_size`) if found, or an
/// error. Reuses the frozen `gnfs::dl::linalg` `block_wiedemann_fl` engine (Gaussian
/// elimination at toy scale).
///
/// The kernel vector `v` satisfies `M·v = 0` over F_ℓ (where M is the relation matrix
/// built by `build_ek_matrix`): for each relation row, the dot product of the exponent
/// vector with `v` is 0 mod ℓ.
///
/// # Errors
///
/// Returns `IndexCalcError::NoKernel` if the solver finds no non-trivial kernel vector.
/// This can happen if the relation system is under-determined (fewer relations than
/// factor-base points + 1), but `collect_relations` guarantees over-determination.
pub fn solve_ek_linalg(
    relations: &[Relation],
    strategy: &IndexCalcStrategy,
) -> Result<Vec<FpNaive>, IndexCalcError> {
    let matrix = build_ek_matrix(relations, strategy);
    let ell = &strategy.ell;

    let op = FlMatrixOperator::<FpNaive, 4>::new(&matrix);

    // Use block_wiedemann_fl (Gaussian elimination at toy scale): more reliable than
    // block_lanczos_fl over small fields (F_5) where BM probabilistic failure modes
    // can occur. The rng_seed is unused by the Gaussian-elimination implementation.
    let solutions = block_wiedemann_fl(&op, ell, 42);

    // Return the first kernel vector found, or NoKernel if none.
    solutions
        .into_iter()
        .next()
        .map(|sol| sol.coefficients)
        .ok_or(IndexCalcError::NoKernel)
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::index_calculus::strategy::IndexCalcStrategy;

    fn ell() -> Uint<4> {
        Uint::<4>::from(crate::index_calculus::strategy::TOY_ELL)
    }

    #[test]
    fn build_ek_matrix_dimensions() {
        let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
        // Construct two minimal relations.
        let r0 = Relation::from_decomposition(1, 0, &[0, 1], &ell());
        let r1 = Relation::from_decomposition(2, 0, &[1, 2], &ell());
        let relations = vec![r0, r1];

        let matrix = build_ek_matrix(&relations, &strategy);

        assert_eq!(matrix.rows.len(), 2, "matrix should have 2 rows");
        assert_eq!(
            matrix.num_cols,
            strategy.fb_size(),
            "matrix should have fb_size columns"
        );
    }

    #[test]
    fn build_ek_matrix_row_matches_relation() {
        let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
        // Relation with indices [0, 2]: exponents [(0, 1), (2, 1)].
        let rel = Relation::from_decomposition(1, 0, &[0, 2], &ell());
        let matrix = build_ek_matrix(&[rel.clone()], &strategy);

        let row = &matrix.rows[0];
        assert_eq!(row.entries.len(), rel.exponents.len(), "row entry count should match");
        for ((ri, rv), (ei, ev)) in row.entries.iter().zip(rel.exponents.iter()) {
            assert_eq!(ri, ei, "column index should match");
            assert_eq!(rv, ev, "value should match");
        }
    }

    #[test]
    fn build_ek_matrix_empty_relations() {
        let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
        let matrix = build_ek_matrix(&[], &strategy);
        assert_eq!(matrix.rows.len(), 0, "empty relations → 0 rows");
        assert_eq!(matrix.num_cols, strategy.fb_size(), "num_cols still fb_size");
    }
}
