//! Quadratic-character columns for the GF(2) matrix.
//!
//! Quadratic-character (QC) columns ensure that the algebraic square root exists in K.
//! For each QC prime `q` (a prime that splits completely in K = ℚ[x]/(f)), the QC column
//! records the Legendre-symbol parity of the algebraic norm at `q`. A dependency in the
//! nullspace is a valid congruence of squares only if all QC columns sum to zero (even
//! parity), which ensures the algebraic square root is well-defined.
//!
//! # Column layout
//!
//! The sign column occupies `obstruction_col_start` (index 0 of the obstruction block).
//! QC columns occupy `obstruction_col_start + 1` through `obstruction_col_start + num_qc`.
//! This requires `fb.obstruction_count = 1 + num_qc`.
//!
//! # Legendre symbol
//!
//! For a prime `q` and integer `a`, the Legendre symbol `(a/q)` is:
//! - 0 if `q | a`
//! - 1 if `a` is a quadratic residue mod `q`
//! - −1 if `a` is a quadratic non-residue mod `q`
//!
//! Computed via Euler's criterion: `(a/q) ≡ a^((q-1)/2) (mod q)`.
//! The QC parity is 1 iff the symbol is −1 (i.e., `a^((q-1)/2) ≡ q-1 (mod q)`).

use num_bigint::BigInt;
use num_traits::Zero;

use crate::filter::SparseMatrix;
use crate::sieve::{FactorBase, Relation};
use crate::polyselect::PolyPair;
use shared_numfield::IntPoly;

// ─── DEFAULT_NUM_QC ───────────────────────────────────────────────────────────

/// Default number of quadratic-character columns (demonstration fidelity).
///
/// Principle-4 annotation: at NFS scale this is typically 20–50; at toy scale 10 suffices.
pub const DEFAULT_NUM_QC: usize = 10;

// ─── populate_qc_columns ─────────────────────────────────────────────────────

/// Populate quadratic-character columns in a matrix.
///
/// For each row `i`, for each QC prime `qc_primes[k]`, sets column
/// `matrix.obstruction_col_start + 1 + k` to the Legendre-symbol parity of the
/// algebraic norm of `relations[provenance[0]]` at `qc_primes[k]`.
///
/// Preconditions:
/// - `matrix` was built with `fb.obstruction_count = 1 + qc_primes.len()`.
/// - `qc_primes` are primes > `fb.b_alg` that split completely in K.
/// - Each row's provenance is non-empty (true for any matrix from `build_matrix`).
///
/// # Legendre-symbol computation
///
/// For a relation with algebraic norm `N_alg(a, b)` and a QC prime `q`, the QC parity is
/// `(N_alg / q) mod 2` where `(· / q)` is the Legendre symbol. If `N_alg ≡ 0 (mod q)`,
/// the symbol is 0 (even parity). The parity is 1 iff the Legendre symbol is −1.
///
/// For merged rows (provenance has multiple original relations), the QC parity is the
/// XOR (GF(2) sum) of the individual relations' parities — consistent with how merged
/// rows' factor-base columns are the XOR of the originals.
pub fn populate_qc_columns(
    matrix: &mut SparseMatrix,
    relations: &[Relation],
    fb: &FactorBase,
    poly: &PolyPair,
    qc_primes: &[u64],
) {
    let num_qc = qc_primes.len();
    debug_assert_eq!(
        fb.obstruction_count,
        1 + num_qc,
        "populate_qc_columns: fb.obstruction_count must be 1 + qc_primes.len()"
    );

    for row in matrix.rows.iter_mut() {
        // Compute the QC parity for each QC prime by XOR-ing over all provenance relations.
        for (k, &q) in qc_primes.iter().enumerate() {
            let col = matrix.obstruction_col_start + 1 + k;
            // XOR the parities of all provenance relations.
            let mut parity = false;
            for &rel_idx in &row.provenance {
                let rel = &relations[rel_idx];
                let norm = algebraic_norm_mod_q(&rel.a, &rel.b, &poly.f, q);
                parity ^= legendre_parity(norm, q);
            }
            // Update the column set: insert or remove the column.
            if parity {
                // Insert col into row.cols (maintaining sorted order).
                match row.cols.binary_search(&col) {
                    Ok(_) => {} // already present (shouldn't happen for a fresh matrix)
                    Err(pos) => {
                        row.cols.insert(pos, col);
                        matrix.col_weights[col] += 1;
                    }
                }
            }
            // If parity is false, the column is already absent (zero); nothing to do.
        }
    }
}

// ─── select_qc_primes ────────────────────────────────────────────────────────

/// Select `num_qc` auxiliary primes for quadratic-character columns.
///
/// Returns the first `num_qc` primes `q > b_alg` such that `f(x)` has `deg(f)` distinct
/// roots mod `q` (i.e., `q` splits completely in K = ℚ[x]/(f)).
pub fn select_qc_primes(f: &IntPoly, b_alg: u64, num_qc: usize) -> Vec<u64> {
    let deg = match f.degree() {
        None => return Vec::new(),
        Some(0) => return Vec::new(),
        Some(d) => d,
    };

    let mut result = Vec::with_capacity(num_qc);
    let mut candidate = b_alg + 1;

    while result.len() < num_qc {
        if is_prime(candidate) && splits_completely(f, candidate, deg) {
            result.push(candidate);
        }
        candidate += 1;
    }

    result
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Compute `N_alg(a, b) mod q` as a `u64` in `[0, q)`.
///
/// Uses the homogeneous form: `Σ f.coeffs[i] · a^i · b^{d-i} mod q`.
/// All arithmetic is done mod `q` to avoid large intermediate values.
fn algebraic_norm_mod_q(a: &BigInt, b: &BigInt, f: &IntPoly, q: u64) -> u64 {
    let d = match f.degree() {
        None => return 0,
        Some(d) => d,
    };

    let q_big = BigInt::from(q);
    let a_mod = mod_reduce_u64(a, q);
    let b_mod = mod_reduce_u64(b, q);

    // Compute Σ f.coeffs[i] · a^i · b^{d-i} mod q using direct summation.
    let mut result = 0u64;
    let mut a_pow = 1u64; // a^i mod q
    // Precompute b^{d-i} for i from 0 to d: b^d, b^{d-1}, ..., b^0.
    // We compute b_pow_d_minus_i = b^{d-i} mod q.
    // Start with b^d and divide by b each step — but division mod q requires modular inverse.
    // Instead, precompute all b powers: b_pows[j] = b^j mod q.
    let mut b_pows = vec![0u64; d + 1];
    b_pows[0] = 1;
    for j in 1..=d {
        b_pows[j] = mul_mod(b_pows[j - 1], b_mod, q);
    }

    for i in 0..=d {
        let coeff_big = f.coeffs.get(i).cloned().unwrap_or_else(BigInt::zero);
        if !coeff_big.is_zero() {
            let coeff_mod = mod_reduce_u64_bigint(&coeff_big, q);
            let b_pow = b_pows[d - i];
            let term = mul_mod(mul_mod(coeff_mod, a_pow, q), b_pow, q);
            result = add_mod(result, term, q);
        }
        if i < d {
            a_pow = mul_mod(a_pow, a_mod, q);
        }
    }

    // Suppress unused variable warning for q_big.
    let _ = q_big;
    result
}

/// Compute the Legendre-symbol parity: true iff `(norm / q) = -1`.
///
/// Uses Euler's criterion: `norm^((q-1)/2) mod q`.
/// - If result is 0: symbol is 0 (parity = false).
/// - If result is 1: symbol is 1 (parity = false).
/// - If result is q-1: symbol is -1 (parity = true).
fn legendre_parity(norm: u64, q: u64) -> bool {
    if norm == 0 {
        return false; // q | norm → symbol is 0
    }
    let exp = (q - 1) / 2;
    let result = pow_mod(norm, exp, q);
    result == q - 1
}

/// Check if `q` is prime using trial division.
fn is_prime(q: u64) -> bool {
    if q < 2 {
        return false;
    }
    if q == 2 {
        return true;
    }
    if q % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= q {
        if q % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Check if `f` splits completely mod `q`: has exactly `deg(f)` distinct roots.
///
/// A prime `q` splits completely in K = ℚ[x]/(f) iff `f mod q` has `deg(f)` distinct
/// linear factors. We check this by counting distinct roots of `f` in `[0, q)`.
fn splits_completely(f: &IntPoly, q: u64, deg: usize) -> bool {
    let mut root_count = 0usize;
    for r in 0..q {
        let val = eval_poly_mod(f, r, q);
        if val == 0 {
            root_count += 1;
        }
    }
    root_count == deg
}

/// Evaluate `f(r) mod q` for `r` in `[0, q)`.
fn eval_poly_mod(f: &IntPoly, r: u64, q: u64) -> u64 {
    // Horner's method mod q.
    let mut result = 0u64;
    for c in f.coeffs.iter().rev() {
        result = mul_mod(result, r, q);
        let c_mod = mod_reduce_u64_bigint(c, q);
        result = add_mod(result, c_mod, q);
    }
    result
}

/// Reduce a `BigInt` to `[0, q)` as a `u64`.
fn mod_reduce_u64(a: &BigInt, q: u64) -> u64 {
    let q_big = BigInt::from(q);
    let r = a % &q_big;
    let r = if r < BigInt::zero() { r + &q_big } else { r };
    // Convert to u64: the result is in [0, q), which fits in u64.
    use num_traits::ToPrimitive;
    r.to_u64().unwrap_or(0)
}

/// Reduce a `BigInt` coefficient to `[0, q)` as a `u64`.
fn mod_reduce_u64_bigint(a: &BigInt, q: u64) -> u64 {
    mod_reduce_u64(a, q)
}

/// Multiply `a * b mod q` without overflow using u128.
fn mul_mod(a: u64, b: u64, q: u64) -> u64 {
    ((a as u128 * b as u128) % q as u128) as u64
}

/// Add `a + b mod q`.
fn add_mod(a: u64, b: u64, q: u64) -> u64 {
    let s = a + b;
    if s >= q { s - q } else { s }
}

/// Compute `base^exp mod q` using fast exponentiation.
fn pow_mod(mut base: u64, mut exp: u64, q: u64) -> u64 {
    if q == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= q;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, q);
        }
        base = mul_mod(base, base, q);
        exp >>= 1;
    }
    result
}

// ─── Unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    /// f(x) = x³ − x − 1.
    fn f_cubic() -> IntPoly {
        IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
    }

    #[test]
    fn legendre_parity_known_values() {
        // (2/7): 2^3 = 8 ≡ 1 (mod 7) → symbol = 1 → parity = false.
        assert!(!legendre_parity(2, 7));
        // (3/7): 3^3 = 27 ≡ 6 ≡ -1 (mod 7) → symbol = -1 → parity = true.
        assert!(legendre_parity(3, 7));
        // (0/7): symbol = 0 → parity = false.
        assert!(!legendre_parity(0, 7));
    }

    #[test]
    fn pow_mod_correctness() {
        assert_eq!(pow_mod(2, 10, 1000), 24); // 2^10 = 1024, 1024 mod 1000 = 24
        assert_eq!(pow_mod(3, 0, 7), 1);
        assert_eq!(pow_mod(0, 5, 7), 0);
    }

    #[test]
    fn is_prime_small() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(9));
        assert!(is_prime(97));
    }

    #[test]
    fn splits_completely_cubic() {
        // f(x) = x³ − x − 1. Check a few primes.
        let f = f_cubic();
        // q = 23: f splits completely mod 23 iff it has 3 distinct roots.
        // f(x) = x³ - x - 1 mod 23: check by brute force.
        let roots_23: Vec<u64> = (0..23).filter(|&r| eval_poly_mod(&f, r, 23) == 0).collect();
        let sc_23 = splits_completely(&f, 23, 3);
        assert_eq!(sc_23, roots_23.len() == 3);
    }

    #[test]
    fn select_qc_primes_returns_correct_count() {
        let f = f_cubic();
        let primes = select_qc_primes(&f, 13, 3);
        assert_eq!(primes.len(), 3, "should return exactly 3 primes");
        // All returned primes should be > 13.
        for &p in &primes {
            assert!(p > 13, "all QC primes should be > b_alg");
        }
        // All returned primes should split completely.
        for &p in &primes {
            assert!(splits_completely(&f, p, 3), "prime {p} should split completely");
        }
    }

    #[test]
    fn algebraic_norm_mod_q_matches_direct() {
        // N_alg(2, 1) for f(x) = x³ - x - 1 = 8 - 2 - 1 = 5.
        let f = f_cubic();
        let norm = algebraic_norm_mod_q(&bi(2), &bi(1), &f, 7);
        assert_eq!(norm, 5 % 7, "N_alg(2,1) mod 7 should be 5");
    }
}
