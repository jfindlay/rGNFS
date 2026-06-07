//! Blocked GF(2) vector representation for block Lanczos and Wiedemann solvers.
//!
//! A `BlockVec` packs `BLOCK_WIDTH` GF(2) vectors of length `num_rows` into a single
//! `Vec<u64>`. The layout is "row of words": `data[i]` is a `u64` whose bit `j` is the
//! `j`-th vector's value at row `i`. This makes iterating over rows contiguous in memory
//! and keeps the 64 vectors interleaved bit-by-bit within each word.

// ─── BLOCK_WIDTH ─────────────────────────────────────────────────────────────

/// Block width: 64 vectors packed into machine words.
///
/// Principle-4 annotation: at toy scale a single word suffices and the blocking overhead
/// is invisible; at NFS scale the word-wide block is the inner loop's cache-friendly unit.
/// The width is the scale knob — D.B may widen to 128 or parameterise over block width.
pub const BLOCK_WIDTH: usize = 64;

// ─── BlockVec ─────────────────────────────────────────────────────────────────

/// A block of `BLOCK_WIDTH` GF(2) vectors, each of length `num_rows`.
///
/// Representation: `data[i]` is a `u64` whose bit `j` (0 ≤ j < 64) is the `j`-th vector's
/// value at row `i`. This is the "row of words" layout: iterating over rows is contiguous,
/// and the 64 vectors are interleaved bit-by-bit within each word.
///
/// # D.B generalisation note
///
/// For F_ℓ (ℓ > 2), the natural generalisation is `data: Vec<[Scalar; BLOCK_WIDTH]>` where
/// `Scalar` is the field element type. The GF(2) specialisation packs 64 scalars into one
/// `u64`. D.B may introduce a `BlockVec<S>` generic or a parallel `BlockVecFl` type; the
/// *interface* (inner products, apply, apply_transpose) is the stable seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockVec {
    /// Packed row data: `data[row]` bit `j` = vector `j`'s value at `row`.
    pub data: Vec<u64>,
    /// Number of rows (the vector dimension, i.e. matrix height for A·V).
    pub num_rows: usize,
}

impl BlockVec {
    /// Construct a zero block vector of the given dimension.
    #[must_use]
    pub fn zeros(num_rows: usize) -> Self {
        Self { data: vec![0u64; num_rows], num_rows }
    }

    /// Construct from a dense `num_rows × BLOCK_WIDTH` bool matrix (column-major: `cols[j][i]`).
    ///
    /// Used for test construction; solvers use `zeros` + `set_bit`.
    ///
    /// # Panics
    ///
    /// Panics if `cols.len() > BLOCK_WIDTH` or if any column has a different length than
    /// the first column.
    #[must_use]
    pub fn from_columns(cols: &[Vec<bool>]) -> Self {
        assert!(cols.len() <= BLOCK_WIDTH, "from_columns: cols.len() must be <= BLOCK_WIDTH");
        let num_rows = cols.first().map_or(0, |c| c.len());
        for col in cols {
            assert_eq!(col.len(), num_rows, "from_columns: all columns must have the same length");
        }
        let mut data = vec![0u64; num_rows];
        for (j, col) in cols.iter().enumerate() {
            for (i, &bit) in col.iter().enumerate() {
                if bit {
                    data[i] |= 1u64 << j;
                }
            }
        }
        Self { data, num_rows }
    }

    /// Get bit `(row, col)` where `col < BLOCK_WIDTH`.
    ///
    /// # Panics
    ///
    /// Panics if `row >= self.num_rows` or `col >= BLOCK_WIDTH`.
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> bool {
        assert!(row < self.num_rows, "BlockVec::get: row {row} out of bounds (num_rows={})", self.num_rows);
        assert!(col < BLOCK_WIDTH, "BlockVec::get: col {col} out of bounds (BLOCK_WIDTH={BLOCK_WIDTH})");
        (self.data[row] >> col) & 1 == 1
    }

    /// Set bit `(row, col)` to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `row >= self.num_rows` or `col >= BLOCK_WIDTH`.
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        assert!(row < self.num_rows, "BlockVec::set: row {row} out of bounds (num_rows={})", self.num_rows);
        assert!(col < BLOCK_WIDTH, "BlockVec::set: col {col} out of bounds (BLOCK_WIDTH={BLOCK_WIDTH})");
        if value {
            self.data[row] |= 1u64 << col;
        } else {
            self.data[row] &= !(1u64 << col);
        }
    }

    /// XOR `self` with `other` in place (component-wise GF(2) addition).
    ///
    /// # Panics
    ///
    /// Panics if `self.num_rows != other.num_rows`.
    pub fn xor_assign(&mut self, other: &BlockVec) {
        assert_eq!(
            self.num_rows, other.num_rows,
            "xor_assign: dimension mismatch ({} vs {})",
            self.num_rows, other.num_rows
        );
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a ^= b;
        }
    }

    /// Compute the `BLOCK_WIDTH × BLOCK_WIDTH` GF(2) inner-product matrix `self^T · other`.
    ///
    /// Returns a `[u64; BLOCK_WIDTH]` where `result[i]` bit `j` = `⟨self.col(i), other.col(j)⟩`
    /// over GF(2) (i.e., parity of the AND of the two columns).
    ///
    /// This is the core primitive for block Lanczos's A-orthogonality check and for
    /// Wiedemann's Krylov-sequence inner products.
    ///
    /// # Panics
    ///
    /// Panics if `self.num_rows != other.num_rows`.
    #[must_use]
    pub fn inner_product_matrix(&self, other: &BlockVec) -> [u64; BLOCK_WIDTH] {
        assert_eq!(
            self.num_rows, other.num_rows,
            "inner_product_matrix: dimension mismatch ({} vs {})",
            self.num_rows, other.num_rows
        );
        // result[i] bit j = parity of (self.col(i) AND other.col(j)).
        // Equivalently: for each row r, if self.data[r] has bit i set and other.data[r] has
        // bit j set, then result[i] ^= (1 << j).
        //
        // Efficient approach: for each row r, let s = self.data[r] and o = other.data[r].
        // For each bit i set in s, result[i] ^= o.
        let mut result = [0u64; BLOCK_WIDTH];
        for r in 0..self.num_rows {
            let s = self.data[r];
            let o = other.data[r];
            if s == 0 || o == 0 {
                continue;
            }
            // Iterate over set bits of s.
            let mut bits = s;
            while bits != 0 {
                let i = bits.trailing_zeros() as usize;
                result[i] ^= o;
                bits &= bits - 1; // clear lowest set bit
            }
        }
        result
    }

    /// Extract column `j` as a dense `Vec<bool>` (for debugging / KAT).
    ///
    /// # Panics
    ///
    /// Panics if `j >= BLOCK_WIDTH`.
    #[must_use]
    pub fn column(&self, j: usize) -> Vec<bool> {
        assert!(j < BLOCK_WIDTH, "BlockVec::column: j {j} out of bounds (BLOCK_WIDTH={BLOCK_WIDTH})");
        (0..self.num_rows).map(|i| (self.data[i] >> j) & 1 == 1).collect()
    }
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_is_all_zero() {
        let v = BlockVec::zeros(5);
        assert_eq!(v.num_rows, 5);
        assert!(v.data.iter().all(|&w| w == 0));
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut v = BlockVec::zeros(4);
        v.set(0, 0, true);
        v.set(1, 3, true);
        v.set(3, 63, true);
        assert!(v.get(0, 0));
        assert!(!v.get(0, 1));
        assert!(v.get(1, 3));
        assert!(v.get(3, 63));
        assert!(!v.get(2, 0));
    }

    #[test]
    fn from_columns_roundtrip() {
        let col0 = vec![true, false, true];
        let col1 = vec![false, true, true];
        let v = BlockVec::from_columns(&[col0.clone(), col1.clone()]);
        assert_eq!(v.column(0), col0);
        assert_eq!(v.column(1), col1);
        // Column 2 should be all false.
        assert!(v.column(2).iter().all(|&b| !b));
    }

    #[test]
    fn xor_assign_correctness() {
        let col0 = vec![true, false, true];
        let col1 = vec![false, true, true];
        let mut a = BlockVec::from_columns(&[col0]);
        let b = BlockVec::from_columns(&[col1]);
        a.xor_assign(&b);
        // col0 XOR col1 = [true^false, false^true, true^true] = [true, true, false]
        assert_eq!(a.column(0), vec![true, true, false]);
    }

    #[test]
    fn inner_product_matrix_identity() {
        // Two identical vectors: inner product of col i with col i = parity of col i.
        // col0 = [1, 1, 0] → parity = 0; col1 = [1, 0, 1] → parity = 0.
        let col0 = vec![true, true, false];
        let col1 = vec![true, false, true];
        let v = BlockVec::from_columns(&[col0, col1]);
        let ip = v.inner_product_matrix(&v);
        // result[0] bit 0 = parity(col0 AND col0) = parity([1,1,0]) = 0.
        // result[0] bit 1 = parity(col0 AND col1) = parity([1,0,0]) = 1.
        // result[1] bit 0 = parity(col1 AND col0) = parity([1,0,0]) = 1.
        // result[1] bit 1 = parity(col1 AND col1) = parity([1,0,1]) = 0.
        assert_eq!(ip[0] & 1, 0, "result[0] bit 0 should be 0");
        assert_eq!((ip[0] >> 1) & 1, 1, "result[0] bit 1 should be 1");
        assert_eq!(ip[1] & 1, 1, "result[1] bit 0 should be 1");
        assert_eq!((ip[1] >> 1) & 1, 0, "result[1] bit 1 should be 0");
    }
}
