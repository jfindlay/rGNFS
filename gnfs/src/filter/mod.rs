//! Filter substrate for GNFS: sparse GF(2) matrix construction, singleton removal,
//! clique/excess pruning, and column merging.
//!
//! This module is the entry point for the ``gnfs::filter`` sub-crate. It provides:
//!
//! - [`matrix`] — the sparse GF(2) matrix type (``SparseMatrix``, ``MatrixRow``) and
//!   the ``EXCESS_FLOOR`` constant.
//! - [`singleton`] — singleton removal to fixpoint (``remove_singletons``).
//! - [`merge`] — clique/excess pruning (``prune_cliques``) and column merging
//!   (``merge_columns``) over the singleton-removed matrix (clique pruning and merging step).
//! - [`build_matrix`] — constructs the initial ``SparseMatrix`` from a relation corpus.
//!
//! # Background
//!
//! The filtering step takes the relation corpus from sieving and reduces it to a
//! well-overdetermined sparse GF(2) matrix suitable for linear algebra. The two main
//! operations are:
//!
//! 1. **Singleton removal**: primes/ideals appearing in only one surviving relation cannot
//!    contribute to a GF(2) dependency; remove their rows and iterate to a fixpoint.
//!
//! 2. **Clique pruning and merging**: reduce matrix weight while preserving excess >=
//!    ``EXCESS_FLOOR`` (pruning), then eliminate low-weight columns by XOR-merging the
//!    rows that contain them (merging).
//!
//! # Column layout
//!
//! Total columns = ``FactorBase::matrix_width()`` = rational_size + algebraic_size + obstruction_count.
//!
//! - ``[0, rational_size)``: rational factor-base columns (GF(2) parities of rational exponents).
//! - ``[rational_size, rational_size + algebraic_size)``: algebraic columns (GF(2) parities of
//!   algebraic exponents).
//! - ``[rational_size + algebraic_size, matrix_width)``: obstruction columns. The sign bit
//!   (``rational_sign``) occupies the first obstruction column; quadratic-character columns
//!   follow and are filled by the linear algebra step (carried as zeros here).
//!
//! # Filtering contract
//!
//! The types and functions in this module implement the sparse GF(2) matrix substrate.
//! The clique pruning and merging step, the linear algebra step, and the square root step
//! consume this interface directly.

pub mod matrix;
pub mod merge;
pub mod singleton;

pub use matrix::{MatrixRow, SparseMatrix, EXCESS_FLOOR};
pub use merge::{merge_columns, prune_cliques};
pub use singleton::remove_singletons;

use crate::sieve::{FactorBase, Relation};

// ─── build_matrix ─────────────────────────────────────────────────────────────

/// Build the initial sparse GF(2) matrix from a relation corpus and factor base.
///
/// Each relation in ``relations`` contributes one row. Column layout:
///
/// - Rational columns ``[0, fb.rational_size())``: GF(2) parities of rational exponents.
/// - Algebraic columns ``[fb.rational_size(), fb.rational_size() + fb.algebraic_size())``:
///   GF(2) parities of algebraic exponents.
/// - Obstruction columns ``[fb.rational_size() + fb.algebraic_size(), fb.matrix_width())``:
///   sign bit at ``fb.rational_size() + fb.algebraic_size()`` (from ``relation.rational_sign``);
///   remaining obstruction columns (quadratic characters) set to 0 — the linear algebra step
///   fills them.
///
/// Note: ``Relation::rational_row_gf2`` places the sign at local index 0 of its return
/// value; ``build_matrix`` re-maps it to the global obstruction column index.
///
/// Provenance for row ``i`` is ``[i]`` (the original relation index).
///
/// :param relations: The relation corpus from sieving.
/// :param fb: The factor base (for size and column layout).
/// :returns: The initial sparse GF(2) matrix.
pub fn build_matrix(relations: &[Relation], fb: &FactorBase) -> SparseMatrix {
    let num_cols = fb.matrix_width();
    let rat_size = fb.rational_size();
    let alg_size = fb.algebraic_size();
    let obstruction_col_start = rat_size + alg_size;
    let obstruction_count = fb.obstruction_count;

    let mut col_weights = vec![0u32; num_cols];
    let mut rows = Vec::with_capacity(relations.len());

    for (i, relation) in relations.iter().enumerate() {
        // rational_row_gf2 returns [sign_bit, rat_col_0, rat_col_1, ...].
        // Local index 0 = sign bit → global column obstruction_col_start.
        // Local index 1+k = rational column k → global column k.
        let rat_row = relation.rational_row_gf2(fb);

        // algebraic_row_gf2 returns [alg_col_0, ..., alg_col_{alg_size-1}, obstruction_zeros...].
        // Local index k (k < alg_size) → global column rat_size + k.
        // Trailing obstruction zeros are already zero; we don't need to read them.
        let alg_row = relation.algebraic_row_gf2(fb);

        let mut cols: Vec<usize> = Vec::new();

        // Rational columns: local indices 1..=rat_size → global 0..rat_size.
        for k in 0..rat_size {
            if rat_row[1 + k] {
                cols.push(k);
            }
        }

        // Algebraic columns: local indices 0..alg_size → global rat_size..rat_size+alg_size.
        for k in 0..alg_size {
            if alg_row[k] {
                cols.push(rat_size + k);
            }
        }

        // Sign bit (obstruction column 0): local rat_row[0] → global obstruction_col_start.
        if rat_row[0] {
            cols.push(obstruction_col_start);
        }

        // Remaining obstruction columns (quadratic chars): carried as zeros, nothing to push.
        // (obstruction_col_start + 1 .. num_cols are all zero for the filtering step.)

        // cols is already sorted because we pushed in ascending global-column order:
        // rational (0..rat_size) < algebraic (rat_size..obstruction_col_start) < obstruction.
        debug_assert!(cols.windows(2).all(|w| w[0] < w[1]), "cols must be sorted and unique");

        // Update column weights.
        for &c in &cols {
            col_weights[c] += 1;
        }

        rows.push(MatrixRow { cols, provenance: vec![i] });
    }

    SparseMatrix { rows, num_cols, obstruction_col_start, obstruction_count, col_weights }
}
