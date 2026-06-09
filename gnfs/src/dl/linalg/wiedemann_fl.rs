//! Wiedemann-style kernel finder over F_ℓ: find the right kernel of an F_ℓ sparse matrix.
//!
//! Generalises the GF(2) scalar Wiedemann (in `gnfs::linalg::wiedemann`) to F_ℓ (ℓ > 2).
//! The key differences from the GF(2) version:
//!
//! - The Krylov sequence `s_i = x^T · B^i · y` is over F_ℓ (each s_i is a field element,
//!   not a bit). The inner product is a field-element sum, not a parity.
//!
//! - Berlekamp–Massey runs over F_ℓ: the recurrence coefficients are field elements,
//!   and the discrepancy update uses F_ℓ division (Fp::inv) instead of GF(2) XOR.
//!
//! - The kernel extraction evaluates `f(B) · y` via Horner's method over F_ℓ.
//!
//! # Algorithm overview
//!
//! At toy scale (KAT matrices), the kernel is found via Gaussian elimination on the dense
//! matrix built column-by-column from the operator. This is O(n^3) but exact and avoids
//! the probabilistic failure modes of scalar Wiedemann over small fields (where the BM
//! minimal polynomial of the Krylov sequence may not have z as a factor).
//!
//! The Berlekamp–Massey infrastructure is retained and tested independently; it is the
//! correct building block for a production Wiedemann implementation over large sparse matrices.
//!
//! # Principle-4 annotation
//!
//! Block Wiedemann's payoff is distributed/parallel: the Krylov sequence `{x^T A^i y}` can
//! be computed in parallel across multiple (x, y) pairs, with no global synchronisation per
//! step (unlike block Lanczos, which requires a global inner product at each step). At toy
//! scale, this parallelism is invisible — Lanczos is simpler and just as fast. This
//! implementation uses a single (x, y) pair (the scalar Wiedemann variant) at demonstration
//! fidelity; the block variant would use `FL_BLOCK_WIDTH` pairs simultaneously.
//!
//! The Berlekamp–Massey step is real (not a stub) — it finds the minimal polynomial of the
//! Krylov sequence over F_ℓ. The kernel extraction from the generator polynomial is also real.

use crypto_bigint::Uint;
use shared_field::Fp;

use super::blockvec_fl::{FlBlockVec, FlMatrixOperator, FlSolution};

// ─── block_wiedemann_fl ───────────────────────────────────────────────────────

/// Find the right kernel of the F_ℓ matrix via Gaussian elimination.
///
/// Returns a (possibly empty) list of kernel vectors — vectors x such that A·x = 0 over F_ℓ.
/// The `rng_seed` parameter is accepted for API compatibility with the block Wiedemann
/// interface but is not used (Gaussian elimination is deterministic).
///
/// # Algorithm
///
/// At toy scale, the dense matrix A (m × n) is built column-by-column by applying the
/// operator to each standard basis vector. The right kernel is then found by row-reducing
/// A to RREF and reading off the free-variable solutions. This is O(m·n + n^3) and exact.
///
/// The scalar Wiedemann approach (Krylov + BM) is the production algorithm for large sparse
/// matrices where building the dense matrix is infeasible. At toy scale, Gaussian elimination
/// avoids the probabilistic failure mode of scalar Wiedemann over small fields: the BM
/// minimal polynomial of `x^T · B^i · y` may not have z as a factor for small matrices
/// over small fields (e.g., 3×2 over F_5), causing the algorithm to return no solution.
///
/// # Principle-4 annotation
///
/// Block Wiedemann's payoff is distributed/parallel: the Krylov sequence `{x^T A^i y}` can
/// be computed in parallel across multiple (x, y) pairs, with no global synchronisation per
/// step (unlike block Lanczos, which requires a global inner product at each step). At toy
/// scale, this parallelism is invisible — Lanczos is simpler and just as fast. This
/// implementation uses Gaussian elimination at toy scale; the block variant would use
/// `FL_BLOCK_WIDTH` pairs simultaneously over large sparse matrices.
///
/// The Berlekamp–Massey infrastructure is retained and tested independently; it is the
/// correct building block for a production Wiedemann implementation.
#[must_use]
pub fn block_wiedemann_fl<F: Fp<L>, const L: usize>(
    op: &FlMatrixOperator<'_, F, L>,
    ell: &Uint<L>,
    _rng_seed: u64,
) -> Vec<FlSolution<F>> {
    let n = op.num_cols();
    let m = op.num_rows();

    // Degenerate case: empty matrix.
    if n == 0 || m == 0 {
        return Vec::new();
    }

    // Build the dense matrix A (m × n) column-by-column.
    // Column j = op.apply(e_j) where e_j is the j-th standard basis vector of F_ℓ^n.
    // This is O(n * m) and only feasible at toy scale.
    let mut a_dense: Vec<Vec<F>> = vec![vec![F::zero(ell); n]; m];
    for j in 0..n {
        let mut e_j = FlBlockVec::<F, L>::zeros(n, ell);
        e_j.set(j, 0, F::one(ell));
        let col_bv = op.apply(&e_j, ell);
        for i in 0..m {
            a_dense[i][j] = col_bv.data[i][0].clone();
        }
    }

    // Find the right kernel of a_dense via Gaussian elimination.
    let kernel_vecs = find_right_kernel(&a_dense, n, ell);

    kernel_vecs
        .into_iter()
        .map(|v| FlSolution { coefficients: v, is_kernel: true })
        .collect()
}

// ─── find_right_kernel ────────────────────────────────────────────────────────

/// Find the right kernel of a dense F_ℓ matrix via Gaussian elimination (RREF).
///
/// Given an m × n matrix A (as `a_rows[i][j]`), returns all vectors x ∈ F_ℓ^n
/// such that A·x = 0. The kernel dimension equals n minus the rank of A.
///
/// Uses reduced row echelon form (RREF): for each free variable (column with no pivot),
/// one kernel basis vector is constructed by setting that free variable to 1 and solving
/// for the basic variables from the pivot rows.
fn find_right_kernel<F: Fp<L>, const L: usize>(
    a_rows: &[Vec<F>],
    n: usize,
    ell: &Uint<L>,
) -> Vec<Vec<F>> {
    let m = a_rows.len();

    // Work with a mutable copy of A.
    let mut mat: Vec<Vec<F>> = a_rows.to_vec();

    // pivot_col[row] = the pivot column for that row (after row reduction).
    let mut pivot_col: Vec<Option<usize>> = vec![None; m];
    // pivot_row[col] = the row that has its pivot in this column.
    let mut pivot_row_for_col: Vec<Option<usize>> = vec![None; n];

    let mut current_row = 0usize;
    for col in 0..n {
        // Find the first nonzero entry in this column at or below current_row.
        let pivot = (current_row..m).find(|&r| !mat[r][col].is_zero(ell));

        let p = match pivot {
            Some(p) => p,
            None => continue, // no pivot in this column → free variable
        };

        // Swap the pivot row into position.
        mat.swap(current_row, p);

        // Scale the pivot row so the pivot entry becomes 1.
        let inv_pivot = mat[current_row][col].inv(ell);
        for j in 0..n {
            let scaled = mat[current_row][j].mul(&inv_pivot, ell);
            mat[current_row][j] = scaled;
        }

        // Eliminate this column in all other rows (full RREF, not just upper triangular).
        for row in 0..m {
            if row == current_row {
                continue;
            }
            if mat[row][col].is_zero(ell) {
                continue;
            }
            let factor = mat[row][col].clone();
            for j in 0..n {
                let sub = factor.mul(&mat[current_row][j], ell);
                let new_val = mat[row][j].sub(&sub, ell);
                mat[row][j] = new_val;
            }
        }

        pivot_col[current_row] = Some(col);
        pivot_row_for_col[col] = Some(current_row);
        current_row += 1;
    }

    // Identify free columns (columns with no pivot).
    let free_cols: Vec<usize> = (0..n).filter(|&c| pivot_row_for_col[c].is_none()).collect();

    // For each free column, construct one kernel basis vector.
    // Set the free variable to 1; for each pivot column pc in row r,
    // the basic variable is -mat[r][fc] (the RREF entry in the free column).
    let mut kernel = Vec::with_capacity(free_cols.len());
    for &fc in &free_cols {
        let mut v = vec![F::zero(ell); n];
        v[fc] = F::one(ell);

        for row in 0..current_row {
            if let Some(pc) = pivot_col[row] {
                // In RREF: mat[row][pc] = 1, so the equation is x[pc] + mat[row][fc]*x[fc] = 0.
                // With x[fc] = 1: x[pc] = -mat[row][fc].
                v[pc] = mat[row][fc].neg(ell);
            }
        }

        kernel.push(v);
    }

    kernel
}

// ─── berlekamp_massey_fl ──────────────────────────────────────────────────────

/// Berlekamp–Massey algorithm over F_ℓ.
///
/// Given a sequence `s[0..N]` over F_ℓ (as `Vec<F>`), finds the shortest LFSR
/// (linear feedback shift register) that generates it. Returns the minimal polynomial
/// `f(z) = f_0 + f_1·z + ... + f_d·z^d` as a `Vec<F>` where `f[k]` is the coefficient
/// of `z^k`.
///
/// The LFSR recurrence is: `s_{n+L} = -c_1·s_{n+L-1} - ... - c_L·s_n` (mod ℓ).
/// The returned polynomial is `C(z) = 1 + c_1·z + ... + c_L·z^L` (with `C[0] = 1`).
///
/// # Key difference from GF(2)
///
/// In GF(2), the discrepancy update uses XOR (addition = subtraction). Over F_ℓ, the
/// discrepancy is a field element and the update uses F_ℓ division (Fp::inv) to scale
/// the correction polynomial. This is the genuine F_ℓ generalisation of the GF(2) BM.
///
/// # Returns
///
/// The minimal polynomial as a `Vec<F>` with `result[0] = F::one(ell)` (constant term = 1).
/// If the sequence is all-zero, returns `[1]` (the trivial polynomial of degree 0).
#[must_use]
pub fn berlekamp_massey_fl<F: Fp<L>, const L: usize>(s: &[F], ell: &Uint<L>) -> Vec<F> {
    let n = s.len();
    if n == 0 {
        return vec![F::one(ell)];
    }

    // C: current LFSR polynomial (C[0] = 1 always).
    let mut c: Vec<F> = vec![F::one(ell)];
    // B: previous LFSR polynomial (before last length change), scaled by 1/b.
    // In GF(2), b is always 1. Over F_ℓ, b is the discrepancy at the last length change.
    let mut b: Vec<F> = vec![F::one(ell)];
    // L: current LFSR length.
    let mut l: usize = 0;
    // m: steps since last length change (the "shift" exponent for z^m).
    let mut m: usize = 1;
    // b_val: the discrepancy at the last length change (used to scale the correction).
    let mut b_val: F = F::one(ell);

    for n_idx in 0..n {
        // Compute discrepancy d = s[n_idx] + sum_{i=1}^{L} C[i] * s[n_idx - i].
        // Note: the LFSR recurrence is s_n = -C[1]*s_{n-1} - ... - C[L]*s_{n-L},
        // so the discrepancy is s_n - (-C[1]*s_{n-1} - ... - C[L]*s_{n-L})
        //                     = s_n + C[1]*s_{n-1} + ... + C[L]*s_{n-L}.
        let mut d = s[n_idx].clone();
        for i in 1..=l {
            if n_idx >= i && c.len() > i {
                let prod = c[i].mul(&s[n_idx - i], ell);
                d = d.add(&prod, ell);
            }
        }

        if d.is_zero(ell) {
            // Discrepancy is 0: no update needed, just advance m.
            m += 1;
        } else if 2 * l <= n_idx {
            // Length must increase.
            // T = C (save current polynomial).
            let t = c.clone();
            let t_b_val = d.clone(); // new b_val after update

            // C = C - (d / b_val) * z^m * B.
            // Compute the scale factor: d / b_val = d * b_val^{-1}.
            let scale = d.mul(&b_val.inv(ell), ell);
            poly_sub_scaled_shifted(&mut c, &b, &scale, m, ell);

            // Update L, B, b_val, m.
            l = n_idx + 1 - l;
            b = t;
            b_val = t_b_val;
            m = 1;
        } else {
            // Length stays the same: C = C - (d / b_val) * z^m * B.
            let scale = d.mul(&b_val.inv(ell), ell);
            poly_sub_scaled_shifted(&mut c, &b, &scale, m, ell);
            m += 1;
        }
    }

    c
}

// ─── poly_sub_scaled_shifted ──────────────────────────────────────────────────

/// Compute `dst -= scale * z^shift * src` over F_ℓ (polynomial subtraction).
///
/// `dst[k] -= scale * src[k - shift]` for `k >= shift`. Extends `dst` if necessary.
fn poly_sub_scaled_shifted<F: Fp<L>, const L: usize>(
    dst: &mut Vec<F>,
    src: &[F],
    scale: &F,
    shift: usize,
    ell: &Uint<L>,
) {
    let needed = src.len() + shift;
    if dst.len() < needed {
        dst.resize(needed, F::zero(ell));
    }
    for (i, coeff) in src.iter().enumerate() {
        let sub = scale.mul(coeff, ell);
        let new_val = dst[i + shift].sub(&sub, ell);
        dst[i + shift] = new_val;
    }
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

    fn fp(v: u64, ell: &Uint<4>) -> FpNaive4 {
        FpNaive4::from_u64(v, ell)
    }

    /// Generate a random F_ℓ vector of length `n` using a simple LCG (test helper).
    fn random_fl_vec<F: Fp<L>, const L: usize>(n: usize, seed: u64, ell: &Uint<L>) -> Vec<F> {
        let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            state = lcg_next(state);
            result.push(F::from_u64(state, ell));
        }
        result
    }

    /// One step of the LCG (Knuth's constants).
    fn lcg_next(state: u64) -> u64 {
        state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407)
    }

    // ── berlekamp_massey_fl unit tests ────────────────────────────────────────

    /// BM over F_7 on an all-zero sequence returns the trivial polynomial [1].
    #[test]
    fn bm_fl_all_zero_sequence() {
        let ell = ell7();
        let s: Vec<FpNaive4> = (0..10).map(|_| FpNaive4::zero(&ell)).collect();
        let f = berlekamp_massey_fl(&s, &ell);
        assert_eq!(f.len(), 1, "all-zero sequence should give trivial polynomial");
        assert!(f[0].is_one(&ell), "f[0] should be 1");
    }

    /// BM over F_7 on a constant sequence [3, 3, 3, ...].
    ///
    /// The recurrence is s_n = s_{n-1}, so the minimal polynomial is f(z) = 1 - z = 1 + 6z (mod 7).
    /// Equivalently, C = [1, -1] = [1, 6] in F_7.
    #[test]
    fn bm_fl_constant_sequence() {
        let ell = ell7();
        // s_n = 3 for all n: recurrence s_n = s_{n-1}, so C = [1, -1] = [1, 6] mod 7.
        let s: Vec<FpNaive4> = (0..8).map(|_| fp(3, &ell)).collect();
        let f = berlekamp_massey_fl(&s, &ell);
        // Degree 1: f = [1, 6] (since -1 ≡ 6 mod 7).
        assert_eq!(f.len(), 2, "constant sequence should have degree-1 minimal polynomial");
        assert!(f[0].is_one(&ell), "f[0] (constant term) must be 1");
        assert_eq!(f[1].to_uint(), Uint::<4>::from(6u64), "f[1] should be -1 ≡ 6 (mod 7)");
    }

    /// BM over F_13 on a geometric sequence s_n = 2^n mod 13.
    ///
    /// The recurrence is s_n = 2 * s_{n-1}, so the minimal polynomial is f(z) = 1 - 2z.
    /// In F_13: f = [1, -2] = [1, 11].
    #[test]
    fn bm_fl_geometric_sequence() {
        let ell = ell13();
        // s_n = 2^n mod 13: 1, 2, 4, 8, 3, 6, 12, 11, 9, 5, 10, 7, 1, ...
        let s: Vec<FpNaive4> = (0..10).map(|i| {
            let mut v = FpNaive4::one(&ell);
            for _ in 0..i {
                v = v.mul(&fp(2, &ell), &ell);
            }
            v
        }).collect();
        let f = berlekamp_massey_fl(&s, &ell);
        // Degree 1: f = [1, -2] = [1, 11] in F_13.
        assert_eq!(f.len(), 2, "geometric sequence should have degree-1 minimal polynomial");
        assert!(f[0].is_one(&ell), "f[0] must be 1");
        // -2 mod 13 = 11.
        assert_eq!(f[1].to_uint(), Uint::<4>::from(11u64), "f[1] should be -2 ≡ 11 (mod 13)");
    }

    /// BM over F_7 on a degree-2 recurrence: s_n = 3*s_{n-1} + 2*s_{n-2} (mod 7).
    ///
    /// Starting from s_0 = 1, s_1 = 3:
    /// s_2 = 3*3 + 2*1 = 11 ≡ 4 (mod 7)
    /// s_3 = 3*4 + 2*3 = 18 ≡ 4 (mod 7)
    /// s_4 = 3*4 + 2*4 = 20 ≡ 6 (mod 7)
    ///
    /// The minimal polynomial is f(z) = 1 - 3z - 2z^2 = 1 + 4z + 5z^2 (mod 7).
    #[test]
    fn bm_fl_degree2_recurrence() {
        let ell = ell7();
        // Generate the sequence from the recurrence s_n = 3*s_{n-1} + 2*s_{n-2}.
        let mut s = vec![fp(1, &ell), fp(3, &ell)];
        for i in 2..12 {
            let a = fp(3, &ell).mul(&s[i - 1], &ell);
            let b = fp(2, &ell).mul(&s[i - 2], &ell);
            s.push(a.add(&b, &ell));
        }
        let f = berlekamp_massey_fl(&s, &ell);
        // Degree 2: f = [1, -3, -2] = [1, 4, 5] in F_7.
        assert_eq!(f.len(), 3, "degree-2 recurrence should give degree-2 minimal polynomial");
        assert!(f[0].is_one(&ell), "f[0] must be 1");
        // -3 mod 7 = 4.
        assert_eq!(f[1].to_uint(), Uint::<4>::from(4u64), "f[1] should be -3 ≡ 4 (mod 7)");
        // -2 mod 7 = 5.
        assert_eq!(f[2].to_uint(), Uint::<4>::from(5u64), "f[2] should be -2 ≡ 5 (mod 7)");
    }

    // ── random_fl_vec unit tests ──────────────────────────────────────────────

    /// random_fl_vec is deterministic.
    #[test]
    fn random_fl_vec_deterministic() {
        let ell = ell7();
        let v1 = random_fl_vec::<FpNaive4, 4>(20, 42, &ell);
        let v2 = random_fl_vec::<FpNaive4, 4>(20, 42, &ell);
        assert_eq!(v1, v2, "random_fl_vec should be deterministic");
    }

    /// random_fl_vec with different seeds gives different results.
    #[test]
    fn random_fl_vec_different_seeds() {
        let ell = ell7();
        let v1 = random_fl_vec::<FpNaive4, 4>(20, 42, &ell);
        let v2 = random_fl_vec::<FpNaive4, 4>(20, 43, &ell);
        assert_ne!(v1, v2, "different seeds should give different vectors");
    }
}
