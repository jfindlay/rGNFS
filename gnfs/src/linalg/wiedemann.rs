//! Coppersmith's block Wiedemann algorithm for finding the left nullspace of a GF(2) matrix.
//!
//! Implements the scalar Wiedemann variant (one (x, y) pair at a time) at demonstration
//! fidelity. The Berlekamp-Massey step is real — it finds the minimal polynomial of the
//! Krylov sequence over GF(2). The kernel extraction from the generator polynomial is also
//! real.
//!
//! # Algorithm overview
//!
//! The algorithm finds vectors `w` such that `w^T A = 0` (the left nullspace of `A`),
//! equivalently the nullspace of `A^T`. It works with the symmetric matrix `B = A * A^T`
//! (m × m), whose nullspace equals the left nullspace of `A`.
//!
//! For a random pair (x, y) with x, y ∈ GF(2)^m:
//! 1. Compute the Krylov sequence `s_i = x^T * B^i * y` for i = 0, ..., 2m + 10.
//! 2. Run Berlekamp-Massey on `{s_i}` to find the minimal polynomial `f(z)`.
//! 3. Compute `w = f(B) * y = Σ_{k=0}^{d} f_k * B^k * y` via Horner's method.
//! 4. If `A^T * w = 0` and `w ≠ 0`, then `w` is a left nullspace vector.
//!
//! # Principle-4 annotation
//!
//! Block Wiedemann's payoff is distributed/parallel: the Krylov sequence `{x^T A^i y}` can
//! be computed in parallel across multiple (x, y) pairs, with no global synchronisation per
//! step (unlike block Lanczos, which requires a global inner product at each step). At toy
//! scale, this parallelism is invisible — Lanczos is simpler and just as fast. This
//! implementation uses a single (x, y) pair (the scalar Wiedemann variant) at demonstration
//! fidelity; the block variant would use `BLOCK_WIDTH` pairs simultaneously.
//!
//! The Berlekamp-Massey step is real (not a stub) — it finds the minimal polynomial of the
//! Krylov sequence over GF(2). The kernel extraction from the generator polynomial is also
//! real.

use super::blockvec::BlockVec;
use super::kernel::KernelVector;
use super::operator::MatrixOperator;

// ─── block_wiedemann ──────────────────────────────────────────────────────────

/// Run Coppersmith's block Wiedemann algorithm to find the left nullspace of A.
///
/// Returns a (possibly empty) list of kernel vectors — vectors `x` such that `x^T A = 0`.
/// The algorithm is randomized (via `rng_seed`); different seeds may find different kernel
/// vectors.
///
/// # Algorithm
///
/// Uses the scalar Wiedemann variant: for each of several random (x, y) pairs, computes the
/// Krylov sequence `s_i = x^T * B^i * y` (where `B = A * A^T`), runs Berlekamp-Massey to
/// find the minimal polynomial `f(z)`, then evaluates `w = f(B) * y`. If `A^T * w = 0` and
/// `w ≠ 0`, then `w` is a left nullspace vector.
///
/// # Principle-4 annotation
///
/// Block Wiedemann's payoff is distributed/parallel: the Krylov sequence `{x^T A^i y}` can
/// be computed in parallel across multiple (x, y) pairs, with no global synchronisation per
/// step (unlike block Lanczos, which requires a global inner product at each step). At toy
/// scale, this parallelism is invisible — Lanczos is simpler and just as fast. This
/// implementation uses a single (x, y) pair (the scalar Wiedemann variant) at demonstration
/// fidelity; the block variant would use `BLOCK_WIDTH` pairs simultaneously.
///
/// The Berlekamp-Massey step is real (not a stub) — it finds the minimal polynomial of the
/// Krylov sequence over GF(2). The kernel extraction from the generator polynomial is also
/// real.
#[must_use]
pub fn block_wiedemann(op: &MatrixOperator<'_>, rng_seed: u64) -> Vec<KernelVector> {
    let m = op.num_rows();

    // Degenerate case: empty matrix.
    if m == 0 || op.num_cols() == 0 {
        return Vec::new();
    }

    // We find the left nullspace of A (vectors w: A^T w = 0) by working with
    // B = A * A^T (m × m symmetric). B * v = apply(apply_transpose(v)).

    // Number of (x, y) attempts. More attempts → more kernel vectors found.
    // 4 attempts is sufficient for toy-scale matrices; at NFS scale the block variant
    // would use BLOCK_WIDTH pairs in parallel.
    let num_attempts = 4;

    let mut results: Vec<KernelVector> = Vec::new();
    let mut rng_state = rng_seed;

    for attempt in 0..num_attempts {
        // Advance the RNG state for each attempt so we get independent (x, y) pairs.
        rng_state = lcg_next(
            rng_state.wrapping_add((attempt as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)),
        );

        let x = random_gf2_vec(m, rng_state);
        rng_state = lcg_next(rng_state);
        let y = random_gf2_vec(m, rng_state);
        rng_state = lcg_next(rng_state);

        if let Some(kv) = wiedemann_attempt(op, m, &x, &y) {
            // Avoid duplicates.
            if !results.iter().any(|r| r.row_indices == kv.row_indices) {
                results.push(kv);
            }
        }
    }

    results
}

// ─── wiedemann_attempt ────────────────────────────────────────────────────────

/// Run one scalar Wiedemann attempt with the given (x, y) pair.
///
/// Returns `Some(kv)` if a valid kernel vector is found, `None` otherwise.
fn wiedemann_attempt(
    op: &MatrixOperator<'_>,
    m: usize,
    x: &[bool],
    y: &[bool],
) -> Option<KernelVector> {
    // Sequence length: 2*m + 10 is sufficient for BM to find the minimal polynomial.
    let seq_len = 2 * m + 10;

    // Build the Krylov sequence s_i = x^T * B^i * y for i = 0..seq_len.
    // We iterate: v_0 = y, v_{i+1} = B * v_i = apply(apply_transpose(v_i)).
    // s_i = x^T * v_i = XOR of v_i[j] for j where x[j] = true (a single GF(2) bit).
    let sequence = krylov_sequence(op, m, x, y, seq_len);

    // Run Berlekamp-Massey to find the minimal polynomial f(z) of the sequence.
    let f = berlekamp_massey(&sequence);

    // The minimal polynomial must have degree >= 1 and f[0] = 1 (constant term).
    // If f = [1] (degree 0), the sequence is all-zero — no useful kernel vector.
    if f.len() <= 1 {
        return None;
    }

    // Compute w = f(B) * y via Horner's method.
    // f(B) * y = f_0 * y + f_1 * B * y + ... + f_d * B^d * y.
    // Horner: w = f_d * y; for k = d-1 down to 0: w = B * w XOR (f_k * y).
    let w_bv = eval_poly_on_krylov(op, m, y, &f);

    // Check if w is nonzero.
    if w_bv.data.iter().all(|&word| word == 0) {
        return None;
    }

    // Check if A^T * w = 0 (i.e., w is in the left nullspace of A).
    let at_w = op.apply_transpose(&w_bv);
    if !at_w.data.iter().all(|&word| word == 0) {
        return None;
    }

    // Extract the kernel vector from column 0 of w_bv.
    let col = w_bv.column(0);
    let kv = KernelVector::from_mask(&col);
    if kv.is_empty() {
        return None;
    }

    Some(kv)
}

// ─── krylov_sequence ─────────────────────────────────────────────────────────

/// Compute the scalar Krylov sequence `s_i = x^T * B^i * y` for i = 0..seq_len.
///
/// `B = A * A^T`. Starting from `v_0 = y`, iterates `v_{i+1} = B * v_i`.
/// At each step, `s_i = x^T * v_i` is the GF(2) inner product of `x` with `v_i`.
///
/// Returns a `Vec<bool>` of length `seq_len`.
fn krylov_sequence(
    op: &MatrixOperator<'_>,
    m: usize,
    x: &[bool],
    y: &[bool],
    seq_len: usize,
) -> Vec<bool> {
    // Pack y into a BlockVec (column 0 only).
    let y_bv = bool_vec_to_blockvec(y, m);

    let mut v = y_bv;
    let mut sequence = Vec::with_capacity(seq_len);

    for _ in 0..seq_len {
        // s_i = x^T * v = XOR of v[j] for j where x[j] = true.
        let s = inner_product_scalar(x, &v);
        sequence.push(s);

        // v = B * v = apply(apply_transpose(v)).
        let at_v = op.apply_transpose(&v);
        v = op.apply(&at_v);
    }

    sequence
}

// ─── eval_poly_on_krylov ─────────────────────────────────────────────────────

/// Evaluate `f(B) * y` via Horner's method over GF(2).
///
/// `f` is the minimal polynomial as a `Vec<bool>` where `f[k]` is the coefficient of `z^k`.
/// `f[0]` is always `true` (the constant term is 1 in GF(2)).
///
/// Horner's method: `w = f_d * y`; for `k = d-1` down to `0`: `w = B * w XOR (f_k * y)`.
///
/// Returns a `BlockVec` with 1 column (column 0 = the result vector).
fn eval_poly_on_krylov(op: &MatrixOperator<'_>, m: usize, y: &[bool], f: &[bool]) -> BlockVec {
    let d = f.len() - 1; // degree of f
    let y_bv = bool_vec_to_blockvec(y, m);

    // Start with w = f_d * y (f_d is always true since f is the minimal polynomial).
    // In GF(2), f_d * y = y if f_d = 1, else 0.
    let mut w = if f[d] { y_bv.clone() } else { BlockVec::zeros(m) };

    // Horner: for k = d-1 down to 0: w = B * w XOR (f_k * y).
    for k in (0..d).rev() {
        // w = B * w.
        let at_w = op.apply_transpose(&w);
        w = op.apply(&at_w);

        // w = w XOR (f_k * y).
        if f[k] {
            w.xor_assign(&y_bv);
        }
    }

    w
}

// ─── berlekamp_massey ────────────────────────────────────────────────────────

/// Berlekamp-Massey algorithm over GF(2).
///
/// Given a sequence `s[0..N]` over GF(2) (as `Vec<bool>`), finds the shortest LFSR
/// (linear feedback shift register) that generates it. Returns the minimal polynomial
/// `f(z) = 1 + f_1*z + ... + f_d*z^d` as a `Vec<bool>` where `f[k]` is the coefficient
/// of `z^k`.
///
/// In GF(2), all arithmetic is mod 2 (XOR for addition, AND for multiplication).
///
/// # Algorithm
///
/// Standard Berlekamp-Massey for GF(2) sequences. The output polynomial `C` satisfies:
/// `sum_{i=0}^{L} C[i] * s[n-i] = 0` for all `n >= L`, where `L` is the LFSR length.
///
/// # Returns
///
/// The minimal polynomial as a `Vec<bool>` with `result[0] = true` (constant term = 1).
/// If the sequence is all-zero, returns `[true]` (the trivial polynomial of degree 0).
#[must_use]
pub fn berlekamp_massey(s: &[bool]) -> Vec<bool> {
    let n = s.len();
    if n == 0 {
        return vec![true];
    }

    // C: current LFSR polynomial (C[0] = 1 always).
    let mut c = vec![true]; // C = 1
    // B: previous LFSR polynomial (before last length change).
    let mut b = vec![true]; // B = 1
    // L: current LFSR length.
    let mut l: usize = 0;
    // m: steps since last length change.
    let mut m: usize = 1;

    for n_idx in 0..n {
        // Compute discrepancy d = s[n_idx] XOR sum_{i=1}^{L} C[i] * s[n_idx - i].
        let mut d = s[n_idx];
        for i in 1..=l {
            if n_idx >= i && c.len() > i && c[i] {
                d ^= s[n_idx - i];
            }
        }

        if !d {
            // Discrepancy is 0: no update needed, just advance m.
            m += 1;
        } else if 2 * l <= n_idx {
            // Length must increase: T = C, C = C XOR z^m * B, L = n+1-L, B = T, m = 1.
            let t = c.clone();
            // C = C XOR (z^m * B): shift B by m positions and XOR into C.
            xor_shifted(&mut c, &b, m);
            l = n_idx + 1 - l;
            b = t;
            m = 1;
        } else {
            // Length stays the same: C = C XOR z^m * B.
            xor_shifted(&mut c, &b, m);
            m += 1;
        }
    }

    c
}

// ─── xor_shifted ─────────────────────────────────────────────────────────────

/// XOR `dst` with `src` shifted left by `shift` positions (i.e., multiply by z^shift).
///
/// `dst[k] ^= src[k - shift]` for `k >= shift`. Extends `dst` if necessary.
fn xor_shifted(dst: &mut Vec<bool>, src: &[bool], shift: usize) {
    let needed = src.len() + shift;
    if dst.len() < needed {
        dst.resize(needed, false);
    }
    for (i, &bit) in src.iter().enumerate() {
        if bit {
            dst[i + shift] ^= true;
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Compute the GF(2) inner product of a dense bool vector `x` with column 0 of a `BlockVec`.
///
/// Returns `true` iff the parity of `{v[j] : x[j] = true}` is odd.
fn inner_product_scalar(x: &[bool], v: &BlockVec) -> bool {
    debug_assert_eq!(x.len(), v.num_rows, "inner_product_scalar: dimension mismatch");
    let mut result = false;
    for (j, &xj) in x.iter().enumerate() {
        if xj {
            // Column 0 of v at row j: bit 0 of v.data[j].
            result ^= (v.data[j] & 1) == 1;
        }
    }
    result
}

/// Pack a `Vec<bool>` into a `BlockVec` with 1 column (column 0).
fn bool_vec_to_blockvec(v: &[bool], m: usize) -> BlockVec {
    debug_assert_eq!(v.len(), m, "bool_vec_to_blockvec: length mismatch");
    let mut bv = BlockVec::zeros(m);
    for (i, &bit) in v.iter().enumerate() {
        if bit {
            bv.data[i] = 1u64; // set bit 0 (column 0)
        }
    }
    bv
}

/// Generate a random GF(2) vector of length `m` using a simple LCG.
///
/// Uses a linear congruential generator (LCG) with Knuth's constants for portability
/// and reproducibility. The seed determines the output deterministically.
fn random_gf2_vec(m: usize, seed: u64) -> Vec<bool> {
    let mut state = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut result = Vec::with_capacity(m);
    let mut word = 0u64;
    let mut bits_left = 0usize;
    for _ in 0..m {
        if bits_left == 0 {
            state = lcg_next(state);
            word = state;
            bits_left = 64;
        }
        result.push((word & 1) == 1);
        word >>= 1;
        bits_left -= 1;
    }
    result
}

/// One step of the LCG (Knuth's constants).
#[inline]
fn lcg_next(state: u64) -> u64 {
    state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407)
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── berlekamp_massey unit tests ───────────────────────────────────────────

    /// BM on an all-zero sequence returns the trivial polynomial [1].
    #[test]
    fn bm_all_zero_sequence() {
        let s = vec![false; 10];
        let f = berlekamp_massey(&s);
        assert_eq!(f, vec![true], "all-zero sequence should give trivial polynomial");
    }

    /// BM on the Fibonacci sequence mod 2: 0,1,1,0,1,1,0,1,1,...
    ///
    /// The Fibonacci recurrence mod 2 is s_n = s_{n-1} XOR s_{n-2}, so the minimal
    /// polynomial is f(z) = 1 + z + z^2 (degree 2, coefficients [1, 1, 1]).
    ///
    /// Sequence: 0, 1, 1, 0, 1, 1, 0, 1, 1, 0 (period 3 starting from index 1).
    /// Actually the Fibonacci sequence mod 2 starting from (0, 1):
    ///   s_0=0, s_1=1, s_2=1, s_3=0, s_4=1, s_5=1, s_6=0, ...
    /// The LFSR recurrence: s_n = s_{n-1} XOR s_{n-2}.
    /// Minimal polynomial: f(z) = 1 + z + z^2.
    #[test]
    fn bm_fibonacci_mod2() {
        // Fibonacci sequence mod 2: 0, 1, 1, 0, 1, 1, 0, 1, 1, 0
        let s: Vec<bool> = vec![false, true, true, false, true, true, false, true, true, false];
        let f = berlekamp_massey(&s);
        // Expected minimal polynomial: 1 + z + z^2, i.e., [true, true, true].
        assert_eq!(f.len(), 3, "Fibonacci mod 2 should have degree-2 minimal polynomial");
        assert!(f[0], "f[0] (constant term) must be 1");
        assert!(f[1], "f[1] must be 1 for Fibonacci mod 2");
        assert!(f[2], "f[2] must be 1 for Fibonacci mod 2");
    }

    /// BM on a period-1 sequence (all ones): s_n = 1 for all n.
    ///
    /// The recurrence is s_n = s_{n-1}, so the minimal polynomial is f(z) = 1 + z.
    #[test]
    fn bm_all_ones_sequence() {
        let s = vec![true; 8];
        let f = berlekamp_massey(&s);
        // Minimal polynomial: 1 + z, i.e., [true, true].
        assert_eq!(f.len(), 2, "all-ones sequence should have degree-1 minimal polynomial");
        assert!(f[0], "f[0] must be 1");
        assert!(f[1], "f[1] must be 1");
    }

    /// BM on a single-element sequence [1]: minimal polynomial is 1 + z.
    #[test]
    fn bm_single_one() {
        let s = vec![true];
        let f = berlekamp_massey(&s);
        assert_eq!(f.len(), 2, "single [1] should give degree-1 polynomial");
        assert!(f[0]);
        assert!(f[1]);
    }

    /// BM on a single-element sequence [0]: minimal polynomial is [1] (trivial).
    #[test]
    fn bm_single_zero() {
        let s = vec![false];
        let f = berlekamp_massey(&s);
        assert_eq!(f, vec![true], "single [0] should give trivial polynomial");
    }

    // ── xor_shifted unit tests ────────────────────────────────────────────────

    /// xor_shifted with shift=0 is just XOR.
    #[test]
    fn xor_shifted_zero_shift() {
        let mut dst = vec![true, false, true];
        let src = vec![true, true, false];
        xor_shifted(&mut dst, &src, 0);
        // [1,0,1] XOR [1,1,0] = [0,1,1]
        assert_eq!(dst, vec![false, true, true]);
    }

    /// xor_shifted with shift=2 shifts src by 2 before XOR.
    #[test]
    fn xor_shifted_nonzero_shift() {
        let mut dst = vec![true, false, true, false, false];
        let src = vec![true, true];
        xor_shifted(&mut dst, &src, 2);
        // dst[2] ^= src[0] = true, dst[3] ^= src[1] = true
        // [1,0,1,0,0] XOR [0,0,1,1,0] = [1,0,0,1,0]
        assert_eq!(dst, vec![true, false, false, true, false]);
    }

    // ── random_gf2_vec unit tests ─────────────────────────────────────────────

    /// random_gf2_vec is deterministic.
    #[test]
    fn random_gf2_vec_deterministic() {
        let v1 = random_gf2_vec(20, 42);
        let v2 = random_gf2_vec(20, 42);
        assert_eq!(v1, v2, "random_gf2_vec should be deterministic");
    }

    /// random_gf2_vec with different seeds gives different results.
    #[test]
    fn random_gf2_vec_different_seeds() {
        let v1 = random_gf2_vec(20, 42);
        let v2 = random_gf2_vec(20, 43);
        assert_ne!(v1, v2, "different seeds should give different vectors");
    }
}
