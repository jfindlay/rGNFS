//! Singleton removal for the sparse GF(2) matrix.
//!
//! A column of Hamming weight ≤ 1 (a prime/ideal appearing in at most one surviving
//! relation) cannot be part of any GF(2) dependency. Removing the row that contains
//! such a column may reduce other columns to weight 1, creating new singletons. This
//! module iterates the removal to a fixpoint.
//!
//! # Obstruction columns
//!
//! Obstruction columns (index >= ``matrix.obstruction_col_start``) are structural and
//! are never treated as singletons regardless of their weight. Only non-obstruction
//! columns participate in singleton detection.
//!
//! # Ordered removal
//!
//! ``SparseMatrix::remove_row`` uses ``Vec::remove`` (ordered, not swap-remove) to
//! preserve row-index stability during the fixpoint loop. This is required because the
//! fixpoint loop re-scans column weights after each removal round.

use crate::filter::matrix::SparseMatrix;

/// Remove singleton columns to a fixpoint, returning the reduced matrix.
///
/// A column of Hamming weight ≤ 1 (a prime/ideal appearing in at most one surviving
/// relation) cannot be part of any GF(2) dependency. The row containing it is removed,
/// which may reduce other columns to weight 1, creating new singletons. This iterates
/// until no weight-≤1 column remains among the non-obstruction columns.
///
/// Obstruction columns (>= ``matrix.obstruction_col_start``) are exempt: they are
/// structural and are never treated as singletons regardless of their weight.
///
/// Provenance is preserved unchanged: singleton removal drops rows, never merges them,
/// so each surviving row's provenance is its original singleton set.
///
/// Weight-0 columns (no rows contain them) are skipped — they don't cause row removal
/// and are left in place (the column space is not compacted).
///
/// :param matrix: The matrix to reduce (consumed).
/// :returns: The singleton-reduced matrix.
pub fn remove_singletons(mut matrix: SparseMatrix) -> SparseMatrix {
    loop {
        // Collect indices of rows to remove in this pass.
        // A row is removed if it contains a non-obstruction column of weight exactly 1.
        // We use a set to avoid removing the same row twice in one pass (a row could
        // contain multiple singleton columns).
        let mut rows_to_remove: Vec<usize> = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for col in 0..matrix.obstruction_col_start {
            if matrix.col_weights[col] == 1 {
                // Find the unique row containing this column.
                // Linear scan is acceptable at toy scale; at cryptographic scale a
                // column-to-row inverted index would be maintained.
                for (row_idx, row) in matrix.rows.iter().enumerate() {
                    if row.cols.binary_search(&col).is_ok() {
                        if seen.insert(row_idx) {
                            rows_to_remove.push(row_idx);
                        }
                        break;
                    }
                }
            }
        }

        if rows_to_remove.is_empty() {
            // Fixpoint reached: no singleton columns remain.
            break;
        }

        // Remove in descending order so that earlier indices remain valid after each removal.
        rows_to_remove.sort_unstable();
        rows_to_remove.dedup();
        for &row_idx in rows_to_remove.iter().rev() {
            matrix.remove_row(row_idx);
        }
    }

    matrix
}
