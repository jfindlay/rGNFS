//! Sparse matrix as a linear operator over GF(2) for block solvers.
//!
//! `MatrixOperator` wraps a `SparseMatrix` and provides `apply` (A·V) and
//! `apply_transpose` (Aᵀ·V) for block vectors. Both solvers (Lanczos, Wiedemann)
//! consume this interface exclusively; they never read `SparseMatrix` fields directly.

use crate::filter::SparseMatrix;
use super::blockvec::{BlockVec, BLOCK_WIDTH};

// ─── MatrixOperator ───────────────────────────────────────────────────────────

/// A view of a `SparseMatrix` as a linear operator over GF(2).
///
/// Provides `apply` (A·V) and `apply_transpose` (Aᵀ·V) for block vectors. Both solvers
/// (Lanczos, Wiedemann) consume this interface exclusively; they never read `SparseMatrix`
/// fields directly. This is the frozen seam.
///
/// # Transpose strategy (design decision, frozen)
///
/// `apply_transpose` computes Aᵀ·V **on-the-fly** by iterating over rows and scattering
/// contributions, rather than pre-building a CSC (column-major) companion.
///
/// Principle-4 annotation: the on-the-fly transpose is the correct algorithm at all scales;
/// the *cache-blocking* that makes it fast at NFS scale is the engineering optimisation
/// out of scope (scoping principle 3).
///
/// # F_ℓ generalisation note
///
/// For F_ℓ, the operator needs the same shape but with scalar multiplication. The natural
/// generalisation is a trait `LinearOperator<V>` with `apply(&self, v: &V) -> V` and
/// `apply_transpose(&self, v: &V) -> V`. The GF(2) linear algebra step implements the
/// concrete version; the F_ℓ extension may introduce the trait and have `MatrixOperator`
/// implement it.
pub struct MatrixOperator<'a> {
    matrix: &'a SparseMatrix,
}

impl<'a> MatrixOperator<'a> {
    /// Construct an operator view of the given sparse matrix.
    #[must_use]
    pub fn new(matrix: &'a SparseMatrix) -> Self {
        Self { matrix }
    }

    /// Number of rows (matrix height).
    #[must_use]
    pub fn num_rows(&self) -> usize {
        self.matrix.rows.len()
    }

    /// Number of columns (matrix width).
    #[must_use]
    pub fn num_cols(&self) -> usize {
        self.matrix.num_cols
    }

    /// Compute A·V: multiply the matrix by a block vector.
    ///
    /// Input `v` has dimension `num_cols`; output has dimension `num_rows`.
    /// Each output row is the GF(2) dot product of the matrix row with each of the
    /// `BLOCK_WIDTH` input vectors.
    ///
    /// # Panics
    ///
    /// Panics if `v.num_rows != self.num_cols()`.
    #[must_use]
    pub fn apply(&self, v: &BlockVec) -> BlockVec {
        assert_eq!(
            v.num_rows, self.num_cols(),
            "apply: v.num_rows ({}) must equal num_cols ({})",
            v.num_rows, self.num_cols()
        );
        let _ = BLOCK_WIDTH; // ensure the constant is used
        let mut result = BlockVec::zeros(self.num_rows());
        for (i, row) in self.matrix.rows.iter().enumerate() {
            // result.data[i] = XOR of v.data[c] for each c in row.cols.
            // Each v.data[c] is a u64 whose bit j is vector j's value at column c.
            // The GF(2) dot product of row with vector j is the parity of {v[c][j] : c in row.cols}.
            // XOR-ing all v.data[c] together gives a u64 whose bit j is that parity.
            let mut word = 0u64;
            for &c in &row.cols {
                word ^= v.data[c];
            }
            result.data[i] = word;
        }
        result
    }

    /// Compute Aᵀ·V: multiply the transpose by a block vector.
    ///
    /// Input `v` has dimension `num_rows`; output has dimension `num_cols`.
    /// Computed on-the-fly by scattering: for each matrix row `i`, for each column `c`
    /// in that row, XOR `v.data[i]` into `result.data[c]`.
    ///
    /// # Panics
    ///
    /// Panics if `v.num_rows != self.num_rows()`.
    #[must_use]
    pub fn apply_transpose(&self, v: &BlockVec) -> BlockVec {
        assert_eq!(
            v.num_rows, self.num_rows(),
            "apply_transpose: v.num_rows ({}) must equal num_rows ({})",
            v.num_rows, self.num_rows()
        );
        let mut result = BlockVec::zeros(self.num_cols());
        for (i, row) in self.matrix.rows.iter().enumerate() {
            let vi = v.data[i];
            if vi == 0 {
                continue;
            }
            for &c in &row.cols {
                result.data[c] ^= vi;
            }
        }
        result
    }
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filter::{MatrixRow, SparseMatrix};

    /// Build a small 3×4 matrix:
    ///
    /// ```text
    ///     col: 0 1 2 3
    /// row 0:   1 0 1 0
    /// row 1:   0 1 0 1
    /// row 2:   1 1 0 0
    /// ```
    fn small_matrix() -> SparseMatrix {
        SparseMatrix {
            rows: vec![
                MatrixRow { cols: vec![0, 2], provenance: vec![0] },
                MatrixRow { cols: vec![1, 3], provenance: vec![1] },
                MatrixRow { cols: vec![0, 1], provenance: vec![2] },
            ],
            num_cols: 4,
            obstruction_col_start: 4,
            obstruction_count: 0,
            col_weights: vec![2, 2, 1, 1],
        }
    }

    #[test]
    fn apply_zero_vector() {
        let m = small_matrix();
        let op = MatrixOperator::new(&m);
        let v = BlockVec::zeros(4);
        let result = op.apply(&v);
        assert_eq!(result, BlockVec::zeros(3));
    }

    #[test]
    fn apply_transpose_zero_vector() {
        let m = small_matrix();
        let op = MatrixOperator::new(&m);
        let v = BlockVec::zeros(3);
        let result = op.apply_transpose(&v);
        assert_eq!(result, BlockVec::zeros(4));
    }

    #[test]
    fn apply_single_column_vector() {
        // Set vector 0 to the standard basis e_0 (only row 0 is 1).
        let m = small_matrix();
        let op = MatrixOperator::new(&m);
        let mut v = BlockVec::zeros(4);
        v.set(0, 0, true); // vector 0, row 0 = 1
        let result = op.apply(&v);
        // A·e_0 = column 0 of A = [1, 0, 1]^T.
        assert!(result.get(0, 0), "row 0 should be 1 (col 0 is in row 0)");
        assert!(!result.get(1, 0), "row 1 should be 0 (col 0 not in row 1)");
        assert!(result.get(2, 0), "row 2 should be 1 (col 0 is in row 2)");
    }

    #[test]
    fn apply_transpose_single_column_vector() {
        // Set vector 0 to e_0 (only row 0 is 1).
        let m = small_matrix();
        let op = MatrixOperator::new(&m);
        let mut v = BlockVec::zeros(3);
        v.set(0, 0, true); // vector 0, row 0 = 1
        let result = op.apply_transpose(&v);
        // Aᵀ·e_0 = row 0 of A = [1, 0, 1, 0]^T.
        assert!(result.get(0, 0), "col 0 should be 1");
        assert!(!result.get(1, 0), "col 1 should be 0");
        assert!(result.get(2, 0), "col 2 should be 1");
        assert!(!result.get(3, 0), "col 3 should be 0");
    }
}
