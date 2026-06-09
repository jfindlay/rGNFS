//! F_ℓ block vector, sparse matrix, matrix operator, and matrix-build for the DL linear system.
//!
//! This module provides the F_ℓ analogues of the GF(2) types in `gnfs::linalg`:
//! - [`FlBlockVec`] — a block of [`FL_BLOCK_WIDTH`] F_ℓ vectors.
//! - [`FlSparseMatrix`] / [`FlSparseRow`] — sparse F_ℓ matrix in CSR-like format.
//! - [`FlMatrixOperator`] — the matrix as a linear operator (apply / apply_transpose).
//! - [`FlSolution`] — solver return type carrying the full coefficient vector.
//! - [`build_fl_matrix`] — build an F_ℓ matrix from a [`DLMatrix`].
//! - [`bigint_to_fp`] — convert a [`BigInt`] to an F_ℓ element.
//!
//! # Parallel-module design
//!
//! This module is **parallel** to `gnfs::linalg` (the GF(2) substrate). No shared trait is
//! introduced; the GF(2) types remain frozen and untouched. The duplication is intentional:
//! F_ℓ scalars are ~256-bit field elements, not bits, so the GF(2) bit-packing is inapplicable.
//!
//! # ℓ-threading
//!
//! The modulus `ell: &Uint<L>` is passed to each arithmetic method rather than stored in the
//! struct. This matches the `Fp` trait's pattern and avoids lifetime complexity.

use crypto_bigint::Uint;
use num_bigint::{BigInt, Sign};
use shared_field::Fp;

use crate::dl::relation::DLMatrix;

// ─── FL_BLOCK_WIDTH ───────────────────────────────────────────────────────────

/// Block width for F_ℓ block vectors: 32 field elements per block.
///
/// Smaller than GF(2)'s BLOCK_WIDTH=64 because field elements are larger than bits.
/// 32 balances memory footprint against blocking benefit. Principle-4 annotation:
/// at toy scale the blocking overhead is invisible; at NFS scale this is the
/// cache-friendly unit for the inner loop.
pub const FL_BLOCK_WIDTH: usize = 32;

// ─── FlBlockVec ───────────────────────────────────────────────────────────────

/// A block of FL_BLOCK_WIDTH F_ℓ vectors, each of length `num_rows`.
///
/// Representation: `data[row]` is an array `[F; FL_BLOCK_WIDTH]` where `data[row][j]`
/// is the j-th vector's value at `row`. This is the F_ℓ analogue of GF(2)'s bit-packed
/// BlockVec — the layout is identical (row-major with vectors interleaved), but each
/// scalar is a field element rather than a bit.
///
/// Generic over `F: Fp<L>` and `L` (limb count). The modulus ℓ is passed to each
/// arithmetic method (matching the Fp trait's pattern), not stored in the struct.
#[derive(Debug, Clone)]
pub struct FlBlockVec<F: Fp<L>, const L: usize> {
    /// Row data: `data[row][j]` = vector j's value at row.
    pub data: Vec<[F; FL_BLOCK_WIDTH]>,
    /// Number of rows (vector dimension).
    pub num_rows: usize,
}

impl<F: Fp<L>, const L: usize> FlBlockVec<F, L> {
    /// Construct a zero block vector of the given dimension.
    pub fn zeros(num_rows: usize, ell: &Uint<L>) -> Self {
        let zero_row: [F; FL_BLOCK_WIDTH] = std::array::from_fn(|_| F::zero(ell));
        Self { data: vec![zero_row; num_rows], num_rows }
    }

    /// Get element (row, col) where col < FL_BLOCK_WIDTH.
    ///
    /// # Panics
    ///
    /// Panics if `row >= self.num_rows` or `col >= FL_BLOCK_WIDTH`.
    pub fn get(&self, row: usize, col: usize) -> &F {
        assert!(row < self.num_rows, "FlBlockVec::get: row {row} out of bounds (num_rows={})", self.num_rows);
        assert!(col < FL_BLOCK_WIDTH, "FlBlockVec::get: col {col} out of bounds (FL_BLOCK_WIDTH={FL_BLOCK_WIDTH})");
        &self.data[row][col]
    }

    /// Set element (row, col) to value.
    ///
    /// # Panics
    ///
    /// Panics if `row >= self.num_rows` or `col >= FL_BLOCK_WIDTH`.
    pub fn set(&mut self, row: usize, col: usize, value: F) {
        assert!(row < self.num_rows, "FlBlockVec::set: row {row} out of bounds (num_rows={})", self.num_rows);
        assert!(col < FL_BLOCK_WIDTH, "FlBlockVec::set: col {col} out of bounds (FL_BLOCK_WIDTH={FL_BLOCK_WIDTH})");
        self.data[row][col] = value;
    }

    /// Component-wise F_ℓ addition: self += other.
    ///
    /// # Panics
    ///
    /// Panics if `self.num_rows != other.num_rows`.
    pub fn add_assign(&mut self, other: &Self, ell: &Uint<L>) {
        assert_eq!(
            self.num_rows, other.num_rows,
            "FlBlockVec::add_assign: dimension mismatch ({} vs {})",
            self.num_rows, other.num_rows
        );
        for r in 0..self.num_rows {
            for j in 0..FL_BLOCK_WIDTH {
                let sum = self.data[r][j].add(&other.data[r][j], ell);
                self.data[r][j] = sum;
            }
        }
    }

    /// Compute the FL_BLOCK_WIDTH × FL_BLOCK_WIDTH inner-product matrix self^T · other.
    ///
    /// Returns `result[i][j] = ⟨self.col(i), other.col(j)⟩` over F_ℓ (sum of products).
    /// This is the core primitive for block Lanczos's orthogonality check.
    ///
    /// # Panics
    ///
    /// Panics if `self.num_rows != other.num_rows`.
    pub fn inner_product_matrix(
        &self,
        other: &Self,
        ell: &Uint<L>,
    ) -> [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] {
        assert_eq!(
            self.num_rows, other.num_rows,
            "inner_product_matrix: dimension mismatch ({} vs {})",
            self.num_rows, other.num_rows
        );
        // result[i][j] = sum over r of self.data[r][i] * other.data[r][j]
        let mut result: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
            std::array::from_fn(|_| std::array::from_fn(|_| F::zero(ell)));
        for r in 0..self.num_rows {
            for i in 0..FL_BLOCK_WIDTH {
                let si = &self.data[r][i];
                if si.is_zero(ell) {
                    continue;
                }
                for j in 0..FL_BLOCK_WIDTH {
                    let prod = si.mul(&other.data[r][j], ell);
                    let new_val = result[i][j].add(&prod, ell);
                    result[i][j] = new_val;
                }
            }
        }
        result
    }

    /// Extract column j as a dense Vec<F> (for solution extraction / KAT).
    ///
    /// # Panics
    ///
    /// Panics if `j >= FL_BLOCK_WIDTH`.
    pub fn column(&self, j: usize) -> Vec<F> {
        assert!(j < FL_BLOCK_WIDTH, "FlBlockVec::column: j {j} out of bounds (FL_BLOCK_WIDTH={FL_BLOCK_WIDTH})");
        (0..self.num_rows).map(|r| self.data[r][j].clone()).collect()
    }

    /// Construct from dense columns (for test construction).
    ///
    /// # Panics
    ///
    /// Panics if `cols.len() > FL_BLOCK_WIDTH` or if columns have inconsistent lengths.
    pub fn from_columns(cols: &[Vec<F>], ell: &Uint<L>) -> Self {
        assert!(
            cols.len() <= FL_BLOCK_WIDTH,
            "from_columns: cols.len() ({}) must be <= FL_BLOCK_WIDTH ({})",
            cols.len(),
            FL_BLOCK_WIDTH
        );
        let num_rows = cols.first().map_or(0, |c| c.len());
        for col in cols {
            assert_eq!(col.len(), num_rows, "from_columns: all columns must have the same length");
        }
        let mut result = Self::zeros(num_rows, ell);
        for (j, col) in cols.iter().enumerate() {
            for (r, val) in col.iter().enumerate() {
                result.data[r][j] = val.clone();
            }
        }
        result
    }
}

// ─── FlSparseRow / FlSparseMatrix ─────────────────────────────────────────────

/// A sparse row in the F_ℓ matrix: (column_index, value) pairs.
#[derive(Debug, Clone)]
pub struct FlSparseRow<F> {
    /// Sparse entries: (column index, F_ℓ value). Sorted by column index.
    pub entries: Vec<(usize, F)>,
}

/// Sparse F_ℓ matrix in CSR-like format.
///
/// Built from DLMatrix by reducing exponent columns mod ℓ and converting
/// Schirokauer BigInt columns to F_ℓ elements.
#[derive(Debug, Clone)]
pub struct FlSparseMatrix<F> {
    /// Sparse rows.
    pub rows: Vec<FlSparseRow<F>>,
    /// Number of columns.
    pub num_cols: usize,
}

// ─── FlMatrixOperator ─────────────────────────────────────────────────────────

/// A view of an FlSparseMatrix as a linear operator over F_ℓ.
///
/// Provides apply (A·V) and apply_transpose (Aᵀ·V) for block vectors.
/// Mirrors the GF(2) MatrixOperator interface.
///
/// The const parameter `L` (limb count) is carried in the struct so that the impl
/// can constrain it. This is required by Rust's const-generic rules: a const parameter
/// in an impl must appear in the self type or the trait being implemented.
pub struct FlMatrixOperator<'a, F, const L: usize> {
    matrix: &'a FlSparseMatrix<F>,
}

impl<'a, F: Fp<L>, const L: usize> FlMatrixOperator<'a, F, L> {
    /// Construct an operator view of the given sparse matrix.
    pub fn new(matrix: &'a FlSparseMatrix<F>) -> Self {
        Self { matrix }
    }

    /// Number of rows (matrix height).
    pub fn num_rows(&self) -> usize {
        self.matrix.rows.len()
    }

    /// Number of columns (matrix width).
    pub fn num_cols(&self) -> usize {
        self.matrix.num_cols
    }

    /// Compute A·V: multiply the matrix by a block vector.
    ///
    /// Input v has dimension num_cols; output has dimension num_rows.
    ///
    /// # Panics
    ///
    /// Panics if `v.num_rows != self.num_cols()`.
    pub fn apply(&self, v: &FlBlockVec<F, L>, ell: &Uint<L>) -> FlBlockVec<F, L> {
        assert_eq!(
            v.num_rows,
            self.num_cols(),
            "apply: v.num_rows ({}) must equal num_cols ({})",
            v.num_rows,
            self.num_cols()
        );
        let mut result: FlBlockVec<F, L> = FlBlockVec::zeros(self.num_rows(), ell);
        for (i, row) in self.matrix.rows.iter().enumerate() {
            for &(c, ref val) in &row.entries {
                // result[i][j] += val * v[c][j] for all j
                for j in 0..FL_BLOCK_WIDTH {
                    let prod = val.mul(&v.data[c][j], ell);
                    let new_val = result.data[i][j].add(&prod, ell);
                    result.data[i][j] = new_val;
                }
            }
        }
        result
    }

    /// Compute Aᵀ·V: multiply the transpose by a block vector.
    ///
    /// Input v has dimension num_rows; output has dimension num_cols.
    ///
    /// # Panics
    ///
    /// Panics if `v.num_rows != self.num_rows()`.
    pub fn apply_transpose(&self, v: &FlBlockVec<F, L>, ell: &Uint<L>) -> FlBlockVec<F, L> {
        assert_eq!(
            v.num_rows,
            self.num_rows(),
            "apply_transpose: v.num_rows ({}) must equal num_rows ({})",
            v.num_rows,
            self.num_rows()
        );
        let mut result: FlBlockVec<F, L> = FlBlockVec::zeros(self.num_cols(), ell);
        for (i, row) in self.matrix.rows.iter().enumerate() {
            for &(c, ref val) in &row.entries {
                // result[c][j] += val * v[i][j] for all j
                for j in 0..FL_BLOCK_WIDTH {
                    let prod = val.mul(&v.data[i][j], ell);
                    let new_val = result.data[c][j].add(&prod, ell);
                    result.data[c][j] = new_val;
                }
            }
        }
        result
    }
}

// ─── FlSolution ───────────────────────────────────────────────────────────────

/// Solution from the F_ℓ block solver: a kernel vector over F_ℓ.
///
/// For NFS-DL, the kernel of the augmented relation matrix over F_ℓ gives the
/// virtual logarithms of the factor-base elements. The solution vector's entries
/// (indexed by column = factor-base element) are the virtual logs mod ℓ.
///
/// # Fields
/// - `coefficients`: the solution vector, length = num_cols of the matrix.
///   Entry i is the virtual log of factor-base element i (or Schirokauer correction).
/// - `is_kernel`: true if this is a kernel vector (A·x = 0), false if particular solution.
#[derive(Debug, Clone)]
pub struct FlSolution<F> {
    /// Solution vector: coefficients[i] = virtual log of column i.
    pub coefficients: Vec<F>,
    /// True if this is a kernel vector (homogeneous solution).
    pub is_kernel: bool,
}

// ─── bigint_to_fp ─────────────────────────────────────────────────────────────

/// Convert a BigInt (assumed to be in [0, ℓ) or reducible mod ℓ) to an Fp element.
///
/// The conversion path: BigInt → bytes → Uint<L> → Fp::from_uint.
/// Handles negative BigInt values by reducing mod ℓ (adding ℓ if negative).
///
/// # Panics
///
/// Panics if the BigInt's absolute value exceeds L*64 bits (cannot fit in Uint<L>).
/// For toy ℓ (≤256 bits with L=4), this is not a constraint.
pub fn bigint_to_fp<F: Fp<L>, const L: usize>(bi: &BigInt, ell: &Uint<L>) -> F {
    // Handle negative values by adding ℓ.
    // The Schirokauer map may produce values in (-ℓ, ℓ); adding ℓ once suffices.
    let canonical = if bi.sign() == Sign::Minus {
        // bi is negative: add ℓ to bring into [0, ℓ).
        // Convert ℓ to BigInt, add, then convert back.
        let ell_bi = uint_to_bigint(ell);
        bi + &ell_bi
    } else {
        bi.clone()
    };

    // Convert the canonical (non-negative) BigInt to Uint<L>.
    // Use to_bytes_le() which returns little-endian bytes.
    let (sign, bytes_le) = canonical.to_bytes_le();
    assert!(sign != Sign::Minus, "bigint_to_fp: value is still negative after adding ℓ");

    // Pad or truncate to exactly L*8 bytes (L limbs × 8 bytes/limb).
    let limb_bytes = L * 8;
    assert!(
        bytes_le.len() <= limb_bytes,
        "bigint_to_fp: BigInt has {} bytes but Uint<{}> can hold at most {} bytes",
        bytes_le.len(),
        L,
        limb_bytes
    );

    // Build Uint<L> from little-endian bytes.
    // crypto_bigint::Uint uses little-endian limbs internally.
    let mut words = [0u64; L];
    for (i, chunk) in bytes_le.chunks(8).enumerate() {
        let mut word_bytes = [0u8; 8];
        word_bytes[..chunk.len()].copy_from_slice(chunk);
        words[i] = u64::from_le_bytes(word_bytes);
    }
    let uint_val = Uint::<L>::from_words(words);

    F::from_uint(uint_val, ell)
}

/// Convert a Uint<L> to a BigInt (helper for bigint_to_fp).
fn uint_to_bigint<const L: usize>(u: &Uint<L>) -> BigInt {
    // Uint<L> stores limbs in little-endian order (words()[0] is least significant).
    let words = u.as_words();
    let mut bytes = Vec::with_capacity(L * 8);
    for &w in words {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    // Remove trailing zero bytes (they are the most significant).
    while bytes.last() == Some(&0) {
        bytes.pop();
    }
    if bytes.is_empty() {
        BigInt::from(0u64)
    } else {
        BigInt::from_bytes_le(Sign::Plus, &bytes)
    }
}

// ─── build_fl_matrix ──────────────────────────────────────────────────────────

/// Build an F_ℓ sparse matrix from a DLMatrix.
///
/// Column layout: rational exponents | algebraic exponents | Schirokauer columns
/// (matching DLMatrix::num_cols).
///
/// - Rational/algebraic exponents (u32) are reduced mod ℓ via from_u64.
/// - Schirokauer columns (BigInt, already in ℤ/ℓ) are converted via bigint_to_fp.
///
/// # Type parameters
/// - F: the Fp implementation (FpNaive4 or FpMonty4)
/// - L: limb count (4 for 256-bit)
pub fn build_fl_matrix<F: Fp<L>, const L: usize>(
    dl_matrix: &DLMatrix,
    ell: &Uint<L>,
) -> FlSparseMatrix<F> {
    let num_cols = dl_matrix.num_cols();
    let zero = F::zero(ell);

    let rows = dl_matrix
        .relations
        .iter()
        .map(|dl_rel| {
            let rel = &dl_rel.relation;
            let mut entries: Vec<(usize, F)> = Vec::new();

            // Rational exponent columns: indices 0..rational_size.
            // ExponentVector::iter() returns owned (usize, u32) pairs.
            for (prime_idx, exp) in rel.rational_exponents.iter() {
                let col = prime_idx;
                if col < dl_matrix.rational_size {
                    let val = F::from_u64(exp as u64, ell);
                    if !val.is_zero(ell) {
                        entries.push((col, val));
                    }
                }
            }

            // Algebraic exponent columns: indices rational_size..rational_size+algebraic_size.
            for (ideal_idx, exp) in rel.algebraic_exponents.iter() {
                let col = dl_matrix.rational_size + ideal_idx;
                if col < dl_matrix.rational_size + dl_matrix.algebraic_size {
                    let val = F::from_u64(exp as u64, ell);
                    if !val.is_zero(ell) {
                        entries.push((col, val));
                    }
                }
            }

            // Schirokauer columns: indices rational_size+algebraic_size..num_cols.
            let schirokauer_start = dl_matrix.rational_size + dl_matrix.algebraic_size;
            for (k, bi_val) in dl_rel.schirokauer_cols.iter().enumerate() {
                let col = schirokauer_start + k;
                let val: F = bigint_to_fp(bi_val, ell);
                if !val.is_zero(ell) {
                    entries.push((col, val));
                }
            }

            // Sort entries by column index (CSR invariant).
            entries.sort_by_key(|&(c, _)| c);

            // Deduplicate: if the same column appears twice (shouldn't happen in a well-formed
            // DLMatrix, but be defensive), sum the values.
            let entries = deduplicate_entries(entries, ell, &zero);

            FlSparseRow { entries }
        })
        .collect();

    FlSparseMatrix { rows, num_cols }
}

/// Deduplicate sorted (col, val) entries by summing values for the same column.
fn deduplicate_entries<F: Fp<L>, const L: usize>(
    entries: Vec<(usize, F)>,
    ell: &Uint<L>,
    zero: &F,
) -> Vec<(usize, F)> {
    if entries.is_empty() {
        return entries;
    }
    let mut result: Vec<(usize, F)> = Vec::with_capacity(entries.len());
    for (col, val) in entries {
        if let Some(last) = result.last_mut() {
            if last.0 == col {
                last.1 = last.1.add(&val, ell);
                continue;
            }
        }
        result.push((col, val));
    }
    // Remove zero entries (from cancellation).
    result.retain(|(_, v)| !v.is_zero(ell));
    let _ = zero; // suppress unused warning
    result
}

// ─── VirtualLogTable / recover_virtual_logs ───────────────────────────────────

/// The virtual-log table extracted from an F_ℓ solver solution.
///
/// Maps factor-base element index → log_g(element) mod ℓ, as recovered from the
/// kernel vector of the DL relation matrix. This is the table D.C's individual-log
/// descent consumes.
///
/// # Column layout (matching DLMatrix / build_fl_matrix)
///
/// The DL relation matrix has columns:
/// - `0..num_rational`: rational factor-base primes.
/// - `num_rational..num_rational+num_algebraic`: algebraic factor-base ideals.
/// - `num_rational+num_algebraic..`: Schirokauer correction columns.
///
/// The `rational_logs` and `algebraic_logs` vectors carry the virtual logs for the
/// first two groups. The Schirokauer columns are not stored here (they are correction
/// terms, not logs of factor-base elements).
#[derive(Debug, Clone)]
pub struct VirtualLogTable<F> {
    /// Virtual logs of rational factor-base primes: `rational_logs[i] = log_g(p_i) mod ℓ`.
    pub rational_logs: Vec<F>,
    /// Virtual logs of algebraic factor-base ideals: `algebraic_logs[i] = log_g(φ_i) mod ℓ`.
    pub algebraic_logs: Vec<F>,
}

/// Extract the virtual-log table from an F_ℓ solver solution.
///
/// The `FlSolution.coefficients` vector (length = num_cols of the F_ℓ matrix) contains
/// the virtual logarithms:
/// - Columns `0..num_rational`: virtual logs of rational factor-base primes.
/// - Columns `num_rational..num_rational+num_algebraic`: virtual logs of algebraic ideals.
/// - Columns `num_rational+num_algebraic..`: Schirokauer correction columns (not extracted).
///
/// # Arguments
///
/// - `solution`: The kernel vector from `block_lanczos_fl` or `block_wiedemann_fl`.
/// - `num_rational`: Number of rational factor-base primes (rational exponent columns).
/// - `num_algebraic`: Number of algebraic factor-base ideals (algebraic exponent columns).
///
/// # Panics
///
/// Panics if `solution.coefficients.len() < num_rational + num_algebraic`.
pub fn recover_virtual_logs<F: Clone>(
    solution: &FlSolution<F>,
    num_rational: usize,
    num_algebraic: usize,
) -> VirtualLogTable<F> {
    let total = num_rational + num_algebraic;
    assert!(
        solution.coefficients.len() >= total,
        "recover_virtual_logs: solution has {} coefficients but need at least {} \
         (num_rational={} + num_algebraic={})",
        solution.coefficients.len(),
        total,
        num_rational,
        num_algebraic,
    );

    let rational_logs = solution.coefficients[..num_rational].to_vec();
    let algebraic_logs = solution.coefficients[num_rational..num_rational + num_algebraic].to_vec();

    VirtualLogTable { rational_logs, algebraic_logs }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use shared_field::FpNaive4;

    fn ell7() -> Uint<4> {
        Uint::<4>::from(7u64)
    }

    fn ell13() -> Uint<4> {
        Uint::<4>::from(13u64)
    }

    #[test]
    fn zeros_is_all_zero() {
        let ell = ell7();
        let v = FlBlockVec::<FpNaive4, 4>::zeros(5, &ell);
        assert_eq!(v.num_rows, 5);
        for r in 0..5 {
            for j in 0..FL_BLOCK_WIDTH {
                assert!(v.data[r][j].is_zero(&ell));
            }
        }
    }

    #[test]
    fn set_and_get_roundtrip() {
        let ell = ell7();
        let mut v = FlBlockVec::<FpNaive4, 4>::zeros(4, &ell);
        let val3 = FpNaive4::from_u64(3, &ell);
        let val5 = FpNaive4::from_u64(5, &ell);
        v.set(0, 0, val3.clone());
        v.set(1, 3, val5.clone());
        assert_eq!(v.get(0, 0), &val3);
        assert_eq!(v.get(1, 3), &val5);
        assert!(v.get(0, 1).is_zero(&ell));
    }

    #[test]
    fn add_assign_correctness() {
        let ell = ell7();
        let mut a = FlBlockVec::<FpNaive4, 4>::zeros(2, &ell);
        let mut b = FlBlockVec::<FpNaive4, 4>::zeros(2, &ell);
        a.set(0, 0, FpNaive4::from_u64(3, &ell));
        b.set(0, 0, FpNaive4::from_u64(5, &ell));
        a.add_assign(&b, &ell);
        // 3 + 5 = 8 ≡ 1 (mod 7)
        assert_eq!(a.get(0, 0).to_uint(), Uint::<4>::from(1u64));
    }

    #[test]
    fn inner_product_matrix_small() {
        // 2-row block, ℓ = 7.
        // col0 = [2, 3], col1 = [4, 1].
        // IP[0][0] = 2*2 + 3*3 = 4 + 9 = 13 ≡ 6 (mod 7)
        // IP[0][1] = 2*4 + 3*1 = 8 + 3 = 11 ≡ 4 (mod 7)
        // IP[1][0] = 4*2 + 1*3 = 8 + 3 = 11 ≡ 4 (mod 7)
        // IP[1][1] = 4*4 + 1*1 = 16 + 1 = 17 ≡ 3 (mod 7)
        let ell = ell7();
        let col0 = vec![FpNaive4::from_u64(2, &ell), FpNaive4::from_u64(3, &ell)];
        let col1 = vec![FpNaive4::from_u64(4, &ell), FpNaive4::from_u64(1, &ell)];
        let v = FlBlockVec::<FpNaive4, 4>::from_columns(&[col0, col1], &ell);
        let ip = v.inner_product_matrix(&v, &ell);
        assert_eq!(ip[0][0].to_uint(), Uint::<4>::from(6u64), "IP[0][0] should be 6");
        assert_eq!(ip[0][1].to_uint(), Uint::<4>::from(4u64), "IP[0][1] should be 4");
        assert_eq!(ip[1][0].to_uint(), Uint::<4>::from(4u64), "IP[1][0] should be 4");
        assert_eq!(ip[1][1].to_uint(), Uint::<4>::from(3u64), "IP[1][1] should be 3");
    }

    #[test]
    fn bigint_to_fp_positive() {
        let ell = ell13();
        let bi = BigInt::from(20i64); // 20 mod 13 = 7
        let fp: FpNaive4 = bigint_to_fp(&bi, &ell);
        assert_eq!(fp.to_uint(), Uint::<4>::from(7u64));
    }

    #[test]
    fn bigint_to_fp_negative() {
        let ell = ell13();
        let bi = BigInt::from(-3i64); // -3 + 13 = 10
        let fp: FpNaive4 = bigint_to_fp(&bi, &ell);
        assert_eq!(fp.to_uint(), Uint::<4>::from(10u64));
    }

    #[test]
    fn bigint_to_fp_zero() {
        let ell = ell7();
        let bi = BigInt::from(0i64);
        let fp: FpNaive4 = bigint_to_fp(&bi, &ell);
        assert!(fp.is_zero(&ell));
    }

    #[test]
    fn from_columns_roundtrip() {
        let ell = ell7();
        let col0 = vec![FpNaive4::from_u64(1, &ell), FpNaive4::from_u64(2, &ell)];
        let col1 = vec![FpNaive4::from_u64(3, &ell), FpNaive4::from_u64(4, &ell)];
        let v = FlBlockVec::<FpNaive4, 4>::from_columns(&[col0.clone(), col1.clone()], &ell);
        assert_eq!(v.column(0), col0);
        assert_eq!(v.column(1), col1);
        // Column 2 should be all zero.
        assert!(v.column(2).iter().all(|x| x.is_zero(&ell)));
    }
}
