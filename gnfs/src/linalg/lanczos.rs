//! Montgomery's block Lanczos algorithm for finding the left nullspace of a GF(2) matrix.
//!
//! Implements the block Lanczos iteration from Montgomery (1995), "A block Lanczos algorithm
//! for finding dependencies over GF(2)". The algorithm finds vectors `x` such that `x^T A = 0`
//! (the left nullspace of `A`), equivalently the nullspace of `A^T`.
//!
//! # Algorithm overview
//!
//! The algorithm works with the symmetric matrix `B = A * A^T` (m × m), whose nullspace equals
//! the nullspace of `A^T` (the left nullspace of `A`). It builds a Krylov basis for `B` using
//! block vectors of width `BLOCK_WIDTH = 64`, handling the GF(2) self-orthogonality problem
//! (where a nonzero vector can satisfy `v^T B v = 0`) via column winnowing.
//!
//! At each step:
//! 1. Compute `W = B * V` (the Krylov product).
//! 2. Compute `S = V^T * W` (the BLOCK_WIDTH × BLOCK_WIDTH inner product matrix).
//! 3. Find "active" columns of `S` (those forming a linearly independent set) via GF(2)
//!    Gaussian elimination.
//! 4. Check "inactive" columns of `V` for nullspace membership.
//! 5. Advance `V` using the three-term recurrence restricted to active columns.
//!
//! # Principle-4 annotation
//!
//! The block width (BLOCK_WIDTH = 64) is the scale knob. At toy scale, a single block suffices
//! and the blocking overhead is invisible. At NFS scale, the word-wide block is the inner loop's
//! cache-friendly unit. The algorithm is correct at all scales; the speedup from blocking is a
//! scale optimisation.

use super::blockvec::{BlockVec, BLOCK_WIDTH};
use super::kernel::KernelVector;
use super::operator::MatrixOperator;

// ─── block_lanczos ────────────────────────────────────────────────────────────

/// Run Montgomery's block Lanczos algorithm to find the left nullspace of A.
///
/// Returns a (possibly empty) list of kernel vectors — vectors `x` such that `x^T A = 0`.
/// The algorithm is randomized (via `rng_seed`); different seeds may find different kernel
/// vectors.
///
/// # Self-orthogonality handling
///
/// Over GF(2), a nonzero vector `v` can satisfy `v^T B v = 0` (self-orthogonal under
/// `B = A * A^T`). The block Lanczos winnowing handles this: at each step, only "active"
/// columns (those where the block inner product matrix `S = V^T B V` is invertible) advance
/// the iteration. Inactive columns are checked for nullspace membership and collected as
/// kernel candidates.
///
/// # Principle-4 annotation
///
/// The block width (BLOCK_WIDTH = 64) is the scale knob. At toy scale a single block suffices
/// and the blocking overhead is invisible. At NFS scale, the word-wide block is the inner
/// loop's cache-friendly unit. The algorithm is correct at all scales; the speedup from
/// blocking is a scale optimisation.
#[must_use]
pub fn block_lanczos(op: &MatrixOperator<'_>, rng_seed: u64) -> Vec<KernelVector> {
    let m = op.num_rows();

    // Degenerate case: empty matrix.
    if m == 0 || op.num_cols() == 0 {
        return Vec::new();
    }

    // We find the left nullspace of A (vectors x: A^T x = 0) by running block Lanczos on
    // B = A * A^T (m × m symmetric). B * v = apply(apply_transpose(v)).

    let mut v_cur = random_block_vec(m, rng_seed);
    let mut v_prev = BlockVec::zeros(m);

    // s_prev_inv: the inverse of S_prev restricted to its pivot columns, stored as a
    // BLOCK_WIDTH × BLOCK_WIDTH GF(2) matrix. Initialized to zero (no previous step).
    let mut s_prev_inv = [0u64; BLOCK_WIDTH];

    // active_mask_prev: bitmask of which columns were active in the previous step.
    let mut active_mask_prev = 0u64;

    let mut results: Vec<KernelVector> = Vec::new();

    // Max iterations: 2 * ceil(m / BLOCK_WIDTH) + BLOCK_WIDTH + 10 is a safe upper bound
    // for the Krylov dimension. Over GF(2), the Krylov sequence has length at most m.
    let max_iter = 2 * m.div_ceil(BLOCK_WIDTH) + BLOCK_WIDTH + 10;

    for _iter in 0..max_iter {
        // w_cur = B * v_cur = A * (A^T * v_cur).
        let at_v = op.apply_transpose(&v_cur);
        let w_cur = op.apply(&at_v);

        // s = v_cur^T * w_cur (BLOCK_WIDTH × BLOCK_WIDTH inner product matrix).
        // s[i] bit j = <v_cur.col(i), w_cur.col(j)> over GF(2).
        let s = v_cur.inner_product_matrix(&w_cur);

        // Find active columns via GF(2) Gaussian elimination on s.
        // Returns (pivot_cols_mask, s_inv) where s_inv is the inverse of s restricted to
        // pivot rows/cols, and pivot_cols_mask is a u64 bitmask of active column indices.
        let (active_mask, s_inv) = gf2_block_pivot(s);

        // Collect kernel candidates from inactive columns.
        // An inactive column j of v_cur is a candidate if A^T * v_cur.col(j) = 0.
        // Since BLOCK_WIDTH = 64, all 64 bits of inactive_mask are meaningful.
        let inactive_mask = !active_mask;
        let mut bits = inactive_mask;
        while bits != 0 {
            let j = bits.trailing_zeros() as usize;
            bits &= bits - 1;

            let col = v_cur.column(j);
            // Check if col is nontrivial.
            if col.iter().any(|&b| b) {
                // Check if A^T * col = 0 (i.e., col is in the nullspace of A^T).
                let col_bv = BlockVec::from_columns(&[col.clone()]);
                let at_col = op.apply_transpose(&col_bv);
                let is_zero = at_col.data.iter().all(|&x| x == 0);
                if is_zero {
                    let kv = KernelVector::from_mask(&col);
                    if !kv.is_empty() {
                        results.push(kv);
                    }
                }
            }
        }

        // If no active columns, the iteration has converged.
        if active_mask == 0 {
            break;
        }

        // Compute v_next using the three-term recurrence (restricted to active columns):
        //
        //   v_next = w_cur - v_cur * alpha - v_prev * beta
        //
        // where:
        //   alpha = s_inv * (v_cur^T * B * w_cur)  [restricted to active columns]
        //   beta  = s_prev_inv * (v_prev^T * w_cur) [restricted to active columns]
        //
        // B * w_cur = apply(apply_transpose(w_cur)).
        // v_cur^T * (B * w_cur) = v_cur.inner_product_matrix(B * w_cur).
        // v_prev^T * w_cur = v_prev.inner_product_matrix(w_cur).
        //
        // The matrix multiplications (s_inv * M) are GF(2) matrix products over BLOCK_WIDTH.

        // Compute B * w_cur for the alpha term.
        let at_w = op.apply_transpose(&w_cur);
        let bw_cur = op.apply(&at_w);

        // alpha_raw = v_cur^T * B * w_cur (BLOCK_WIDTH × BLOCK_WIDTH).
        let alpha_raw = v_cur.inner_product_matrix(&bw_cur);

        // alpha = s_inv * alpha_raw (restricted to active columns).
        // Only rows of s_inv corresponding to active columns contribute.
        let alpha = gf2_matmul_block(s_inv, alpha_raw, active_mask);

        // beta_raw = v_prev^T * w_cur (BLOCK_WIDTH × BLOCK_WIDTH).
        let beta_raw = v_prev.inner_product_matrix(&w_cur);

        // beta = s_prev_inv * beta_raw (restricted to previously active columns).
        let beta = gf2_matmul_block(s_prev_inv, beta_raw, active_mask_prev);

        // v_next = w_cur - v_cur * alpha - v_prev * beta (over GF(2), - = +).
        // v_cur * alpha: for each row r of v_cur, v_cur.data[r] is a u64 (bit j = col j).
        // (v_cur * alpha)[r] = XOR over j of (v_cur.data[r] bit j) * alpha[j].
        // Similarly for v_prev * beta.
        let mut v_next = w_cur.clone();
        block_vec_sub_matmul(&mut v_next, &v_cur, alpha);
        block_vec_sub_matmul(&mut v_next, &v_prev, beta);

        // Zero out inactive columns of v_next (only active columns advance).
        zero_inactive_columns(&mut v_next, active_mask);

        // Advance the iteration state.
        v_prev = v_cur;
        s_prev_inv = s_inv;
        active_mask_prev = active_mask;
        v_cur = v_next;
    }

    // Deduplicate results: remove duplicate kernel vectors.
    results.dedup_by(|a, b| a.row_indices == b.row_indices);
    results
}

// ─── gf2_block_pivot ─────────────────────────────────────────────────────────

/// GF(2) Gaussian elimination on a BLOCK_WIDTH × BLOCK_WIDTH matrix.
///
/// The matrix is represented as `[u64; BLOCK_WIDTH]` where `s[i]` is row `i`
/// (bit `j` of `s[i]` is the (i, j) entry).
///
/// Returns `(pivot_mask, s_inv)` where:
/// - `pivot_mask`: a `u64` bitmask of pivot column indices (the "active" columns).
/// - `s_inv`: the inverse of the submatrix formed by pivot rows/cols, embedded in
///   BLOCK_WIDTH × BLOCK_WIDTH (non-pivot rows/cols are zero). Used in the recurrence.
///
/// # Algorithm
///
/// Standard GF(2) Gaussian elimination with partial pivoting. For each column `j` from
/// 0 to BLOCK_WIDTH-1, find a row with bit `j` set (the pivot), swap it to position `j`,
/// then XOR it into all other rows that have bit `j` set (full elimination, not just
/// forward). This produces the reduced row echelon form and simultaneously builds the
/// inverse via the augmented matrix technique.
#[must_use]
fn gf2_block_pivot(s: [u64; BLOCK_WIDTH]) -> (u64, [u64; BLOCK_WIDTH]) {
    // Work with an augmented matrix [s | I] to compute the inverse simultaneously.
    // aug[i] = (s_row[i], identity_row[i]).
    let mut s_work = s;
    let mut inv = [0u64; BLOCK_WIDTH];
    // Initialize inv as the identity matrix.
    for i in 0..BLOCK_WIDTH {
        inv[i] = 1u64 << i;
    }

    let mut pivot_mask = 0u64;
    // pivot_row[j] = the row index that was used as pivot for column j (or BLOCK_WIDTH if none).
    let mut pivot_row = [BLOCK_WIDTH; BLOCK_WIDTH];

    for col in 0..BLOCK_WIDTH {
        // Find a row at or below `col` with bit `col` set.
        // We search all rows (not just below col) since we do full elimination.
        // But to maintain a consistent pivot structure, search from `col` downward.
        let mut found = BLOCK_WIDTH;
        for row in col..BLOCK_WIDTH {
            if (s_work[row] >> col) & 1 == 1 {
                found = row;
                break;
            }
        }

        if found == BLOCK_WIDTH {
            // No pivot in this column — it's a dependent column (inactive).
            continue;
        }

        // Swap rows `col` and `found`.
        s_work.swap(col, found);
        inv.swap(col, found);

        pivot_mask |= 1u64 << col;
        pivot_row[col] = col;

        // Eliminate bit `col` from all other rows (full reduced row echelon form).
        for row in 0..BLOCK_WIDTH {
            if row != col && (s_work[row] >> col) & 1 == 1 {
                s_work[row] ^= s_work[col];
                inv[row] ^= inv[col];
            }
        }
    }

    // After full elimination, `s_work` is the reduced row echelon form.
    // `inv` restricted to pivot rows/cols is the inverse of the pivot submatrix.
    // For non-pivot rows, inv[row] is the corresponding row of the identity (unused).

    // Build s_inv: for each pivot column `col`, the inverse row is `inv[col]`.
    // Non-pivot rows of s_inv are zero (they don't participate in the recurrence).
    let mut s_inv = [0u64; BLOCK_WIDTH];
    for col in 0..BLOCK_WIDTH {
        if (pivot_mask >> col) & 1 == 1 {
            // pivot_row[col] = col after the swap, so inv[col] is the inverse row.
            s_inv[col] = inv[col];
        }
    }

    (pivot_mask, s_inv)
}

// ─── gf2_matmul_block ────────────────────────────────────────────────────────

/// Multiply two BLOCK_WIDTH × BLOCK_WIDTH GF(2) matrices, restricted to `active_mask` rows.
///
/// `a[i]` and `b[i]` are row `i` of the respective matrices (bit `j` = column `j`).
/// Returns `c` where `c[i] = (a * b)[i]` for rows `i` in `active_mask`, and `c[i] = 0`
/// otherwise.
///
/// # Algorithm
///
/// For each row `i` in `active_mask`: `c[i] = XOR of b[j]` for each bit `j` set in `a[i]`.
/// This is the standard GF(2) matrix multiplication using the "row of a selects rows of b"
/// identity.
#[must_use]
fn gf2_matmul_block(a: [u64; BLOCK_WIDTH], b: [u64; BLOCK_WIDTH], active_mask: u64) -> [u64; BLOCK_WIDTH] {
    let mut c = [0u64; BLOCK_WIDTH];
    let mut mask = active_mask;
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        // c[i] = XOR of b[j] for each bit j set in a[i].
        let mut row_a = a[i];
        while row_a != 0 {
            let j = row_a.trailing_zeros() as usize;
            row_a &= row_a - 1;
            c[i] ^= b[j];
        }
    }
    c
}

// ─── block_vec_sub_matmul ────────────────────────────────────────────────────

/// Compute `dst -= src * mat` over GF(2) (in-place XOR).
///
/// `src` is an m × BLOCK_WIDTH block vector; `mat` is a BLOCK_WIDTH × BLOCK_WIDTH GF(2)
/// matrix (row `i` = `mat[i]`, bit `j` = column `j`). The product `src * mat` is an
/// m × BLOCK_WIDTH block vector where row `r` is `XOR of mat[j]` for each bit `j` set in
/// `src.data[r]`.
///
/// Over GF(2), subtraction is XOR, so `dst -= src * mat` is `dst ^= src * mat`.
fn block_vec_sub_matmul(dst: &mut BlockVec, src: &BlockVec, mat: [u64; BLOCK_WIDTH]) {
    debug_assert_eq!(dst.num_rows, src.num_rows, "block_vec_sub_matmul: dimension mismatch");
    for r in 0..src.num_rows {
        let word = src.data[r];
        if word == 0 {
            continue;
        }
        // product_row = XOR of mat[j] for each bit j set in word.
        let mut product_row = 0u64;
        let mut bits = word;
        while bits != 0 {
            let j = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            product_row ^= mat[j];
        }
        dst.data[r] ^= product_row;
    }
}

// ─── zero_inactive_columns ───────────────────────────────────────────────────

/// Zero out all columns of `v` that are NOT in `active_mask`.
///
/// For each row `r`, clears all bits in `v.data[r]` that correspond to inactive columns.
/// This restricts the block vector to only the active subspace.
fn zero_inactive_columns(v: &mut BlockVec, active_mask: u64) {
    for word in v.data.iter_mut() {
        *word &= active_mask;
    }
}

// ─── random_block_vec ────────────────────────────────────────────────────────

/// Generate a random block vector of dimension `num_rows` using a simple LCG.
///
/// Uses a linear congruential generator (LCG) with Knuth's constants for portability
/// and reproducibility. The seed determines the output deterministically.
///
/// # Panics
///
/// Does not panic.
#[must_use]
fn random_block_vec(num_rows: usize, seed: u64) -> BlockVec {
    let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15); // golden-ratio mix
    let mut data = Vec::with_capacity(num_rows);
    for _ in 0..num_rows {
        // LCG: x_{n+1} = a * x_n + c (mod 2^64), Knuth's constants.
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        data.push(state);
    }
    BlockVec { data, num_rows }
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gf2_block_pivot_identity() {
        // The identity matrix should have all 64 columns as pivots, and its inverse is itself.
        let mut s = [0u64; BLOCK_WIDTH];
        for i in 0..BLOCK_WIDTH {
            s[i] = 1u64 << i;
        }
        let (mask, inv) = gf2_block_pivot(s);
        assert_eq!(mask, u64::MAX, "all columns should be pivots for the identity");
        // Verify inv * s = I (over GF(2)).
        for i in 0..BLOCK_WIDTH {
            let mut row = 0u64;
            let mut bits = inv[i];
            while bits != 0 {
                let j = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                row ^= s[j];
            }
            assert_eq!(row, 1u64 << i, "inv * I should be I at row {i}");
        }
    }

    #[test]
    fn gf2_block_pivot_zero_matrix() {
        // The zero matrix has no pivots.
        let s = [0u64; BLOCK_WIDTH];
        let (mask, _inv) = gf2_block_pivot(s);
        assert_eq!(mask, 0, "zero matrix should have no pivots");
    }

    #[test]
    fn gf2_block_pivot_rank_one() {
        // A rank-1 matrix: only row 0 is nonzero, with bit 0 set.
        let mut s = [0u64; BLOCK_WIDTH];
        s[0] = 1u64; // only (0,0) is set
        let (mask, _inv) = gf2_block_pivot(s);
        assert_eq!(mask, 1u64, "rank-1 matrix should have exactly 1 pivot (col 0)");
    }

    #[test]
    fn random_block_vec_deterministic() {
        let v1 = random_block_vec(10, 42);
        let v2 = random_block_vec(10, 42);
        assert_eq!(v1.data, v2.data, "random_block_vec should be deterministic");
    }

    #[test]
    fn random_block_vec_different_seeds() {
        let v1 = random_block_vec(10, 42);
        let v2 = random_block_vec(10, 43);
        assert_ne!(v1.data, v2.data, "different seeds should produce different vectors");
    }

    #[test]
    fn block_vec_sub_matmul_zero_mat() {
        // Subtracting zero matrix should leave dst unchanged.
        let src = random_block_vec(5, 1);
        let mut dst = random_block_vec(5, 2);
        let dst_orig = dst.clone();
        block_vec_sub_matmul(&mut dst, &src, [0u64; BLOCK_WIDTH]);
        assert_eq!(dst, dst_orig, "subtracting zero matrix should leave dst unchanged");
    }

    #[test]
    fn zero_inactive_columns_all_active() {
        let mut v = random_block_vec(5, 7);
        let orig = v.clone();
        zero_inactive_columns(&mut v, u64::MAX);
        assert_eq!(v, orig, "all-active mask should leave v unchanged");
    }

    #[test]
    fn zero_inactive_columns_none_active() {
        let mut v = random_block_vec(5, 7);
        zero_inactive_columns(&mut v, 0);
        assert!(v.data.iter().all(|&w| w == 0), "no-active mask should zero all columns");
    }
}
