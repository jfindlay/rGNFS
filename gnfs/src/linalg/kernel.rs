//! Nullspace vector representation: the linear algebra → square root seam.
//!
//! `KernelVector` represents a vector in the left nullspace of the filtered matrix as a
//! sorted, deduplicated list of row indices. The square root step expands this to original
//! relation indices via the provenance map.

use crate::filter::SparseMatrix;

// ─── KernelVector ─────────────────────────────────────────────────────────────

/// A vector in the left nullspace of the matrix: a subset of rows whose GF(2) sum is zero.
///
/// Representation: a sorted, deduplicated `Vec<usize>` of **filtered-matrix row indices**
/// (indices into `SparseMatrix::rows`). The square root step expands this to original
/// relation indices by collecting `matrix.rows[i].provenance` for each `i` in `row_indices`
/// and taking the symmetric difference (XOR union).
///
/// # Why row indices, not a bit-mask
///
/// - The square root step needs row indices to look up provenance; a bit-mask would require a scan.
/// - Kernel vectors are sparse (typically a small fraction of rows); a bit-mask wastes space.
/// - Solvers (Lanczos, Wiedemann) internally work with bit-packed block vectors, but they
///   convert to `KernelVector` on output — the conversion is O(rows) and happens once per
///   kernel vector, not in the inner loop.
///
/// # F_ℓ generalisation note
///
/// For F_ℓ, a kernel vector is still a subset of rows (those with nonzero coefficient in
/// the nullspace vector). The representation is identical; the F_ℓ extension may add a
/// `coefficients` field (`Vec<Scalar>`) for the non-GF(2) case, but the row-index spine
/// is stable.
///
/// # Invariants
///
/// - `row_indices` is sorted and deduplicated.
/// - Each index is < `matrix.rows.len()` (the filtered matrix, not the original relations).
/// - The GF(2) sum of the selected rows is the zero vector (verified by `verify`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelVector {
    /// Sorted, deduplicated row indices into the filtered matrix.
    pub row_indices: Vec<usize>,
}

impl KernelVector {
    /// Construct from a sorted, deduplicated list of row indices.
    ///
    /// # Panics
    ///
    /// Panics if `row_indices` is not sorted or contains duplicates.
    #[must_use]
    pub fn new(row_indices: Vec<usize>) -> Self {
        assert!(
            row_indices.windows(2).all(|w| w[0] < w[1]),
            "KernelVector::new: row_indices must be sorted and deduplicated"
        );
        Self { row_indices }
    }

    /// Construct from a bit-mask over rows (used by solvers internally).
    ///
    /// `mask[i]` is true iff row `i` is in the kernel vector.
    #[must_use]
    pub fn from_mask(mask: &[bool]) -> Self {
        let row_indices = mask.iter().enumerate().filter_map(|(i, &b)| if b { Some(i) } else { None }).collect();
        Self { row_indices }
    }

    /// Verify that this is a valid left-nullspace vector of the given matrix.
    ///
    /// Returns `true` iff the GF(2) sum of `matrix.rows[i].cols` for `i` in `row_indices`
    /// is the empty set (all columns cancel).
    #[must_use]
    pub fn verify(&self, matrix: &SparseMatrix) -> bool {
        // XOR all selected rows' column sets; the result should be empty.
        // Use a bit-set over columns for efficiency.
        let num_cols = matrix.num_cols;
        // Use a Vec<u64> as a bit-set.
        let words = (num_cols + 63) / 64;
        let mut bits = vec![0u64; words];
        for &i in &self.row_indices {
            if i >= matrix.rows.len() {
                return false;
            }
            for &c in &matrix.rows[i].cols {
                bits[c / 64] ^= 1u64 << (c % 64);
            }
        }
        bits.iter().all(|&w| w == 0)
    }

    /// Expand through the provenance map to original relation indices.
    ///
    /// Returns the symmetric difference (XOR union) of `matrix.rows[i].provenance` for
    /// each `i` in `row_indices`. This is the set of original relations whose product
    /// yields a congruence of squares.
    ///
    /// The result is sorted and deduplicated.
    #[must_use]
    pub fn expand_provenance(&self, matrix: &SparseMatrix) -> Vec<usize> {
        // Collect all provenance indices and compute the symmetric difference.
        // Since each provenance list is sorted and deduplicated, we XOR them pairwise.
        let mut result: Vec<usize> = Vec::new();
        for &i in &self.row_indices {
            let prov = &matrix.rows[i].provenance;
            // Symmetric difference of result and prov (both sorted).
            let mut merged = Vec::with_capacity(result.len() + prov.len());
            let mut ri = 0;
            let mut pi = 0;
            while ri < result.len() && pi < prov.len() {
                match result[ri].cmp(&prov[pi]) {
                    std::cmp::Ordering::Less => {
                        merged.push(result[ri]);
                        ri += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        merged.push(prov[pi]);
                        pi += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        // Symmetric difference: both present → cancel.
                        ri += 1;
                        pi += 1;
                    }
                }
            }
            merged.extend_from_slice(&result[ri..]);
            merged.extend_from_slice(&prov[pi..]);
            result = merged;
        }
        result
    }

    /// Number of selected rows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.row_indices.len()
    }

    /// True if no rows are selected (the trivial kernel vector).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.row_indices.is_empty()
    }
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::{MatrixRow, SparseMatrix};

    fn make_matrix(rows: Vec<(Vec<usize>, Vec<usize>)>, num_cols: usize) -> SparseMatrix {
        let col_weights = vec![0u32; num_cols];
        SparseMatrix {
            rows: rows
                .into_iter()
                .map(|(cols, prov)| MatrixRow { cols, provenance: prov })
                .collect(),
            num_cols,
            obstruction_col_start: num_cols,
            obstruction_count: 0,
            col_weights,
        }
    }

    #[test]
    fn new_panics_on_unsorted() {
        let result = std::panic::catch_unwind(|| KernelVector::new(vec![2, 1]));
        assert!(result.is_err(), "new should panic on unsorted input");
    }

    #[test]
    fn new_panics_on_duplicate() {
        let result = std::panic::catch_unwind(|| KernelVector::new(vec![1, 1]));
        assert!(result.is_err(), "new should panic on duplicate input");
    }

    #[test]
    fn from_mask_roundtrip() {
        let mask = vec![true, false, true, false, true];
        let kv = KernelVector::from_mask(&mask);
        assert_eq!(kv.row_indices, vec![0, 2, 4]);
    }

    #[test]
    fn verify_valid_nullspace() {
        // Matrix: row 0 = {0, 1}, row 1 = {1, 2}, row 2 = {0, 2}.
        // XOR of all three: {0,1} XOR {1,2} XOR {0,2} = {} (all cancel).
        let m = make_matrix(
            vec![(vec![0, 1], vec![0]), (vec![1, 2], vec![1]), (vec![0, 2], vec![2])],
            3,
        );
        let kv = KernelVector::new(vec![0, 1, 2]);
        assert!(kv.verify(&m), "XOR of all three rows should be zero");
    }

    #[test]
    fn verify_invalid_nullspace() {
        // Matrix: row 0 = {0, 1}, row 1 = {1, 2}.
        // XOR = {0, 2} ≠ {}.
        let m = make_matrix(
            vec![(vec![0, 1], vec![0]), (vec![1, 2], vec![1])],
            3,
        );
        let kv = KernelVector::new(vec![0, 1]);
        assert!(!kv.verify(&m), "XOR of rows 0 and 1 should not be zero");
    }

    #[test]
    fn expand_provenance_simple() {
        // Row 0 provenance = [0], row 1 provenance = [1].
        // Symmetric difference = [0, 1].
        let m = make_matrix(
            vec![(vec![0, 1], vec![0]), (vec![1, 2], vec![1])],
            3,
        );
        let kv = KernelVector::new(vec![0, 1]);
        let expanded = kv.expand_provenance(&m);
        assert_eq!(expanded, vec![0, 1]);
    }

    #[test]
    fn expand_provenance_cancellation() {
        // Row 0 provenance = [0, 1], row 1 provenance = [1, 2].
        // Symmetric difference = [0, 2] (1 cancels).
        let m = make_matrix(
            vec![(vec![0], vec![0, 1]), (vec![0], vec![1, 2])],
            1,
        );
        let kv = KernelVector::new(vec![0, 1]);
        let expanded = kv.expand_provenance(&m);
        assert_eq!(expanded, vec![0, 2]);
    }
}
