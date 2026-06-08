//! Block Lanczos over F_ℓ: find the kernel of an F_ℓ sparse matrix.
//!
//! Generalises Montgomery's GF(2) block Lanczos to F_ℓ (ℓ > 2). The key differences
//! from the GF(2) version:
//!
//! - The inner-product matrix S = V^T·B·V is a FL_BLOCK_WIDTH × FL_BLOCK_WIDTH matrix
//!   over F_ℓ (not a bit-packed u64 array). Inversion uses Gaussian elimination with
//!   Fermat's little theorem for field inversion.
//!
//! - Self-orthogonality: over F_ℓ, a nonzero vector v can satisfy v^T·B·v = 0 even when
//!   v is not in the kernel of A. The algorithm handles this via Gaussian elimination with
//!   explicit F_ℓ pivoting: columns where S is singular are "inactive" and checked for
//!   kernel membership separately.
//!
//! - The three-term recurrence uses F_ℓ matrix multiplication (not GF(2) XOR).
//!
//! # Algorithm overview
//!
//! We find the right kernel of A (vectors x: A·x = 0) by running block Lanczos on
//! B = A^T·A (n × n symmetric), whose kernel equals the kernel of A.
//!
//! At each step:
//! 1. Compute W = B·V = A^T·(A·V).
//! 2. Compute S = V^T·W (FL_BLOCK_WIDTH × FL_BLOCK_WIDTH inner-product matrix over F_ℓ).
//! 3. Find "active" columns of S via F_ℓ Gaussian elimination (pivot on nonzero entries).
//! 4. Check "inactive" columns of V for kernel membership (A·v = 0).
//! 5. Advance V using the three-term recurrence restricted to active columns.

use crypto_bigint::Uint;
use shared_field::Fp;

use super::blockvec_fl::{FL_BLOCK_WIDTH, FlBlockVec, FlMatrixOperator, FlSolution};

// ─── block_lanczos_fl ─────────────────────────────────────────────────────────

/// Block Lanczos over F_ℓ: find the kernel of the matrix.
///
/// Returns a (possibly empty) list of kernel vectors — vectors x such that A·x = 0 over F_ℓ.
/// The algorithm is randomized (via rng_seed); different seeds may find different vectors.
///
/// # Self-orthogonality handling (F_ℓ care)
///
/// Over F_ℓ (ℓ > 2), the inner-product matrix S = V^T·B·V can be singular even when
/// V has full column rank (unlike GF(2) where singularity implies linear dependence).
/// The F_ℓ block Lanczos handles this via Gaussian elimination with explicit F_ℓ
/// inversion (Fermat inv), pivoting on nonzero entries rather than GF(2) parity.
///
/// # Arguments
/// - op: the F_ℓ matrix operator
/// - ell: the prime modulus (as Uint<L>)
/// - rng_seed: seed for random initial vector
pub fn block_lanczos_fl<F: Fp<L>, const L: usize>(
    op: &FlMatrixOperator<'_, F, L>,
    ell: &Uint<L>,
    rng_seed: u64,
) -> Vec<FlSolution<F>> {
    let n = op.num_cols();

    // Degenerate case: empty matrix.
    if n == 0 || op.num_rows() == 0 {
        return Vec::new();
    }

    // We find the right kernel of A (vectors x: A·x = 0) by running block Lanczos on
    // B = A^T·A (n × n symmetric). B·v = apply_transpose(apply(v)).

    let mut v_cur = random_fl_block_vec(n, rng_seed, ell);
    let mut v_prev = FlBlockVec::<F, L>::zeros(n, ell);

    // s_prev_inv: the inverse of S_prev restricted to its pivot columns.
    // Stored as a FL_BLOCK_WIDTH × FL_BLOCK_WIDTH matrix over F_ℓ.
    let mut s_prev_inv: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| F::zero(ell)));

    // active_cols_prev: which columns were active in the previous step.
    let mut active_cols_prev: [bool; FL_BLOCK_WIDTH] = [false; FL_BLOCK_WIDTH];

    let mut results: Vec<FlSolution<F>> = Vec::new();

    // Max iterations: 2 * ceil(n / FL_BLOCK_WIDTH) + FL_BLOCK_WIDTH + 10.
    let max_iter = 2 * n.div_ceil(FL_BLOCK_WIDTH) + FL_BLOCK_WIDTH + 10;

    for _iter in 0..max_iter {
        // w_cur = B·v_cur = A^T·(A·v_cur).
        let av = op.apply(&v_cur, ell);
        let w_cur = op.apply_transpose(&av, ell);

        // s = v_cur^T·w_cur (FL_BLOCK_WIDTH × FL_BLOCK_WIDTH inner-product matrix).
        let s = v_cur.inner_product_matrix(&w_cur, ell);

        // Find active columns via F_ℓ Gaussian elimination on s.
        // Returns (active_cols, s_inv) where s_inv is the inverse of s restricted to
        // pivot rows/cols, and active_cols[j] is true iff column j is a pivot.
        let (active_cols, s_inv) = fl_block_pivot(s, ell);

        // Collect kernel candidates from inactive columns.
        // An inactive column j of v_cur is a candidate if A·v_cur.col(j) = 0.
        let inactive_iter = (0..FL_BLOCK_WIDTH).filter(|&j| !active_cols[j]);
        for j in inactive_iter {
            let col = v_cur.column(j);
            // Check if col is nontrivial (not all zero).
            if col.iter().any(|x| !x.is_zero(ell)) {
                // Check if A·col = 0 (i.e., col is in the kernel of A).
                let col_bv = FlBlockVec::<F, L>::from_columns(&[col.clone()], ell);
                let a_col = op.apply(&col_bv, ell);
                let is_zero = a_col.data.iter().all(|row| row[0].is_zero(ell));
                if is_zero {
                    let sol = FlSolution { coefficients: col, is_kernel: true };
                    results.push(sol);
                }
            }
        }

        // If no active columns, the iteration has converged.
        let any_active = active_cols.iter().any(|&b| b);
        if !any_active {
            break;
        }

        // Compute v_next using the three-term recurrence (restricted to active columns):
        //
        //   v_next = w_cur - v_cur·alpha - v_prev·beta
        //
        // where:
        //   alpha = s_inv · (v_cur^T·B·w_cur)  [restricted to active columns]
        //   beta  = s_prev_inv · (v_prev^T·w_cur) [restricted to active columns]
        //
        // B·w_cur = A^T·(A·w_cur).
        // v_cur^T·(B·w_cur) = v_cur.inner_product_matrix(B·w_cur).
        // v_prev^T·w_cur = v_prev.inner_product_matrix(w_cur).

        // Compute B·w_cur for the alpha term.
        let aw = op.apply(&w_cur, ell);
        let bw_cur = op.apply_transpose(&aw, ell);

        // alpha_raw = v_cur^T·B·w_cur (FL_BLOCK_WIDTH × FL_BLOCK_WIDTH).
        let alpha_raw = v_cur.inner_product_matrix(&bw_cur, ell);

        // alpha = s_inv · alpha_raw (restricted to active columns).
        let alpha = fl_matmul_block(&s_inv, &alpha_raw, &active_cols, ell);

        // beta_raw = v_prev^T·w_cur (FL_BLOCK_WIDTH × FL_BLOCK_WIDTH).
        let beta_raw = v_prev.inner_product_matrix(&w_cur, ell);

        // beta = s_prev_inv · beta_raw (restricted to previously active columns).
        let beta = fl_matmul_block(&s_prev_inv, &beta_raw, &active_cols_prev, ell);

        // v_next = w_cur - v_cur·alpha - v_prev·beta (over F_ℓ).
        let mut v_next = w_cur.clone();
        fl_block_vec_sub_matmul(&mut v_next, &v_cur, &alpha, ell);
        fl_block_vec_sub_matmul(&mut v_next, &v_prev, &beta, ell);

        // Zero out inactive columns of v_next (only active columns advance).
        zero_inactive_fl_columns(&mut v_next, &active_cols, ell);

        // Advance the iteration state.
        v_prev = v_cur;
        s_prev_inv = s_inv;
        active_cols_prev = active_cols;
        v_cur = v_next;
    }

    // Deduplicate results: remove solutions with identical coefficient vectors.
    results.dedup_by(|a, b| {
        a.coefficients.iter().zip(b.coefficients.iter()).all(|(x, y)| x == y)
    });
    results
}

// ─── fl_block_pivot ───────────────────────────────────────────────────────────

/// F_ℓ Gaussian elimination on a FL_BLOCK_WIDTH × FL_BLOCK_WIDTH matrix.
///
/// The matrix is represented as `[[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH]` where `s[i][j]`
/// is the (i, j) entry.
///
/// Returns `(active_cols, s_inv)` where:
/// - `active_cols[j]`: true iff column j is a pivot column.
/// - `s_inv`: the inverse of the submatrix formed by pivot rows/cols, embedded in
///   FL_BLOCK_WIDTH × FL_BLOCK_WIDTH (non-pivot rows/cols are zero).
///
/// # Algorithm
///
/// Standard F_ℓ Gaussian elimination with partial pivoting. For each column j from
/// 0 to FL_BLOCK_WIDTH-1, find a row with a nonzero entry in column j, swap it to
/// position j, scale it to make the pivot 1 (via Fermat inv), then eliminate column j
/// from all other rows. This produces the reduced row echelon form and simultaneously
/// builds the inverse via the augmented matrix technique.
fn fl_block_pivot<F: Fp<L>, const L: usize>(
    s: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH],
    ell: &Uint<L>,
) -> ([bool; FL_BLOCK_WIDTH], [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH]) {
    // Work with an augmented matrix [s | I] to compute the inverse simultaneously.
    let mut s_work = s;
    let mut inv: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
        std::array::from_fn(|i| std::array::from_fn(|j| {
            if i == j { F::one(ell) } else { F::zero(ell) }
        }));

    let mut active_cols = [false; FL_BLOCK_WIDTH];

    for col in 0..FL_BLOCK_WIDTH {
        // Find a row at or below `col` with a nonzero entry in column `col`.
        let mut found = FL_BLOCK_WIDTH;
        for row in col..FL_BLOCK_WIDTH {
            if !s_work[row][col].is_zero(ell) {
                found = row;
                break;
            }
        }

        if found == FL_BLOCK_WIDTH {
            // No pivot in this column — it's a dependent column (inactive).
            continue;
        }

        // Swap rows `col` and `found`.
        s_work.swap(col, found);
        inv.swap(col, found);

        active_cols[col] = true;

        // Scale row `col` so that s_work[col][col] = 1.
        let pivot_inv = s_work[col][col].inv(ell);
        for k in 0..FL_BLOCK_WIDTH {
            let new_sk = s_work[col][k].mul(&pivot_inv, ell);
            s_work[col][k] = new_sk;
            let new_ik = inv[col][k].mul(&pivot_inv, ell);
            inv[col][k] = new_ik;
        }

        // Eliminate column `col` from all other rows (full reduced row echelon form).
        for row in 0..FL_BLOCK_WIDTH {
            if row != col && !s_work[row][col].is_zero(ell) {
                let factor = s_work[row][col].clone();
                for k in 0..FL_BLOCK_WIDTH {
                    let sub = factor.mul(&s_work[col][k], ell);
                    let new_val = s_work[row][k].sub(&sub, ell);
                    s_work[row][k] = new_val;
                    let sub_inv = factor.mul(&inv[col][k], ell);
                    let new_inv = inv[row][k].sub(&sub_inv, ell);
                    inv[row][k] = new_inv;
                }
            }
        }
    }

    // Build s_inv: for each pivot column `col`, the inverse row is `inv[col]`.
    // Non-pivot rows of s_inv are zero (they don't participate in the recurrence).
    let mut s_inv: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| F::zero(ell)));
    for col in 0..FL_BLOCK_WIDTH {
        if active_cols[col] {
            s_inv[col] = inv[col].clone();
        }
    }

    (active_cols, s_inv)
}

// ─── fl_matmul_block ──────────────────────────────────────────────────────────

/// Multiply two FL_BLOCK_WIDTH × FL_BLOCK_WIDTH F_ℓ matrices, restricted to `active_cols` rows.
///
/// Returns `c` where `c[i] = (a * b)[i]` for rows `i` in `active_cols`, and `c[i] = 0`
/// otherwise.
fn fl_matmul_block<F: Fp<L>, const L: usize>(
    a: &[[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH],
    b: &[[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH],
    active_cols: &[bool; FL_BLOCK_WIDTH],
    ell: &Uint<L>,
) -> [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] {
    let mut c: [[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
        std::array::from_fn(|_| std::array::from_fn(|_| F::zero(ell)));
    for i in 0..FL_BLOCK_WIDTH {
        if !active_cols[i] {
            continue;
        }
        for k in 0..FL_BLOCK_WIDTH {
            if a[i][k].is_zero(ell) {
                continue;
            }
            for j in 0..FL_BLOCK_WIDTH {
                let prod = a[i][k].mul(&b[k][j], ell);
                let new_val = c[i][j].add(&prod, ell);
                c[i][j] = new_val;
            }
        }
    }
    c
}

// ─── fl_block_vec_sub_matmul ──────────────────────────────────────────────────

/// Compute `dst -= src · mat` over F_ℓ (in-place subtraction).
///
/// `src` is an n × FL_BLOCK_WIDTH block vector; `mat` is a FL_BLOCK_WIDTH × FL_BLOCK_WIDTH
/// F_ℓ matrix. The product `src · mat` is an n × FL_BLOCK_WIDTH block vector where
/// row `r` entry `j` is `sum_k src[r][k] * mat[k][j]`.
fn fl_block_vec_sub_matmul<F: Fp<L>, const L: usize>(
    dst: &mut FlBlockVec<F, L>,
    src: &FlBlockVec<F, L>,
    mat: &[[F; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH],
    ell: &Uint<L>,
) {
    debug_assert_eq!(dst.num_rows, src.num_rows, "fl_block_vec_sub_matmul: dimension mismatch");
    for r in 0..src.num_rows {
        for k in 0..FL_BLOCK_WIDTH {
            if src.data[r][k].is_zero(ell) {
                continue;
            }
            for j in 0..FL_BLOCK_WIDTH {
                let prod = src.data[r][k].mul(&mat[k][j], ell);
                let new_val = dst.data[r][j].sub(&prod, ell);
                dst.data[r][j] = new_val;
            }
        }
    }
}

// ─── zero_inactive_fl_columns ─────────────────────────────────────────────────

/// Zero out all columns of `v` that are NOT in `active_cols`.
fn zero_inactive_fl_columns<F: Fp<L>, const L: usize>(
    v: &mut FlBlockVec<F, L>,
    active_cols: &[bool; FL_BLOCK_WIDTH],
    ell: &Uint<L>,
) {
    let zero = F::zero(ell);
    for r in 0..v.num_rows {
        for j in 0..FL_BLOCK_WIDTH {
            if !active_cols[j] {
                v.data[r][j] = zero.clone();
            }
        }
    }
}

// ─── random_fl_block_vec ──────────────────────────────────────────────────────

/// Generate a random F_ℓ block vector of dimension `num_rows` using a simple LCG.
///
/// Uses a linear congruential generator (LCG) with Knuth's constants for portability
/// and reproducibility. The seed determines the output deterministically.
fn random_fl_block_vec<F: Fp<L>, const L: usize>(
    num_rows: usize,
    seed: u64,
    ell: &Uint<L>,
) -> FlBlockVec<F, L> {
    let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15); // golden-ratio mix
    let mut result = FlBlockVec::<F, L>::zeros(num_rows, ell);
    for r in 0..num_rows {
        for j in 0..FL_BLOCK_WIDTH {
            // LCG: x_{n+1} = a * x_n + c (mod 2^64), Knuth's constants.
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            // Use the LCG output as a u64 and reduce mod ℓ via from_u64.
            result.data[r][j] = F::from_u64(state, ell);
        }
    }
    result
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

    #[test]
    fn fl_block_pivot_identity() {
        // The identity matrix should have all FL_BLOCK_WIDTH columns as pivots.
        let ell = ell7();
        let s: [[FpNaive4; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
            std::array::from_fn(|i| std::array::from_fn(|j| {
                if i == j { FpNaive4::one(&ell) } else { FpNaive4::zero(&ell) }
            }));
        let (active, inv) = fl_block_pivot(s, &ell);
        assert!(active.iter().all(|&b| b), "all columns should be pivots for the identity");
        // Verify inv is the identity.
        for i in 0..FL_BLOCK_WIDTH {
            for j in 0..FL_BLOCK_WIDTH {
                let expected = if i == j { FpNaive4::one(&ell) } else { FpNaive4::zero(&ell) };
                assert_eq!(inv[i][j], expected, "inv[{i}][{j}] should be identity entry");
            }
        }
    }

    #[test]
    fn fl_block_pivot_zero_matrix() {
        let ell = ell7();
        let s: [[FpNaive4; FL_BLOCK_WIDTH]; FL_BLOCK_WIDTH] =
            std::array::from_fn(|_| std::array::from_fn(|_| FpNaive4::zero(&ell)));
        let (active, _inv) = fl_block_pivot(s, &ell);
        assert!(active.iter().all(|&b| !b), "zero matrix should have no pivots");
    }

    #[test]
    fn random_fl_block_vec_deterministic() {
        let ell = ell7();
        let v1 = random_fl_block_vec::<FpNaive4, 4>(10, 42, &ell);
        let v2 = random_fl_block_vec::<FpNaive4, 4>(10, 42, &ell);
        for r in 0..10 {
            for j in 0..FL_BLOCK_WIDTH {
                assert_eq!(v1.data[r][j], v2.data[r][j], "random_fl_block_vec should be deterministic");
            }
        }
    }
}
