//! Terminal assembly step and end-to-end factor driver for GNFS.
//!
//! This module implements the final step of the GNFS square-root stage:
//!
//! 1. **`factor_from_congruence`**: given X (rational sqrt) and Y (algebraic sqrt), compute
//!    gcd(X − Y, N). Returns a non-trivial factor if X ≢ ±Y (mod N), or `None` if trivial.
//!
//! 2. **`factor`**: the top-level "factor N" driver. Iterates over kernel vectors, calls
//!    `rational_sqrt` and `algebraic_sqrt` for each, calls `factor_from_congruence`, and
//!    returns the first non-trivial factor found.
//!
//! # Background
//!
//! The GNFS congruence-of-squares identity guarantees X² ≡ Y² (mod N) for each kernel vector.
//! This means N | (X − Y)(X + Y). If X ≢ ±Y (mod N), then gcd(X − Y, N) is a non-trivial
//! factor of N. If X ≡ Y (mod N) or X ≡ −Y (mod N), the kernel vector yields only the trivial
//! factorization; the driver advances to the next kernel vector.
//!
//! # Retry loop
//!
//! The trivial-gcd outcome (X ≡ ±Y mod N) is *expected* for some kernel vectors — it is not a
//! bug. The driver loops over all kernel vectors and returns the first non-trivial factor. If all
//! vectors yield trivial gcds, `factor` returns `None`.

use num_bigint::BigInt;
use num_traits::{One, Signed};
use shared_bigint::gcd;

use crate::filter::SparseMatrix;
use crate::linalg::KernelVector;
use crate::polyselect::PolyPair;
use crate::sieve::Relation;
use crate::sqrt::{algebraic_sqrt, rational_sqrt};

// ─── factor_from_congruence ───────────────────────────────────────────────────

/// Attempt to extract a non-trivial factor of N from the congruence X² ≡ Y² (mod N).
///
/// Computes gcd(X − Y, N). If the result is a non-trivial factor (1 < g < N), returns
/// `Some(g)`. Also checks gcd(X + Y, N) as a fallback for the X ≡ −Y (mod N) case.
/// Returns `None` if both gcds are trivial (g = 1 or g = N).
///
/// # Parameters
///
/// - `x`: The rational square root X (mod N), satisfying X² ≡ ∏(a_i − b_i·m) (mod N).
/// - `y`: The algebraic square root Y (mod N), satisfying Y² ≡ Norm(∏(a_i − b_i·α)) (mod N).
/// - `n`: The number to factor.
///
/// # Returns
///
/// `Some(factor)` if a non-trivial factor is found (1 < factor < N), `None` if trivial.
pub fn factor_from_congruence(x: &BigInt, y: &BigInt, n: &BigInt) -> Option<BigInt> {
    // Primary check: gcd(X − Y, N).
    let diff = x - y;
    let g = gcd(&diff.abs(), n);
    if is_nontrivial(&g, n) {
        return Some(g);
    }

    // Fallback check: gcd(X + Y, N). Catches the X ≡ −Y (mod N) case where X − Y ≡ 0 (mod N)
    // but X + Y ≢ 0 (mod N), or vice versa.
    let sum = x + y;
    let g2 = gcd(&sum.abs(), n);
    if is_nontrivial(&g2, n) {
        return Some(g2);
    }

    None
}

/// Return true iff `g` is a non-trivial factor of `n` (1 < g < n).
fn is_nontrivial(g: &BigInt, n: &BigInt) -> bool {
    g > &BigInt::one() && g < n
}

// ─── factor ───────────────────────────────────────────────────────────────────

/// Factor N using the GNFS square-root stage.
///
/// Iterates over `kernel_vectors`. For each kernel vector:
/// 1. Calls `rational_sqrt` to get X.
/// 2. Calls `algebraic_sqrt` to get Y.
/// 3. Calls `factor_from_congruence(&x, &y, &poly.n)`.
/// 4. Returns the first non-trivial factor found.
///
/// Returns `None` if all kernel vectors yield trivial gcds (X ≡ ±Y mod N for every vector).
///
/// # Parameters
///
/// - `poly`: The polynomial pair (provides N via `poly.n`).
/// - `matrix`: The filtered sparse GF(2) matrix (carries the provenance map).
/// - `relations`: The original relation list (indexed by the provenance map).
/// - `kernel_vectors`: The nullspace vectors from the linear algebra step.
///
/// # Returns
///
/// `Some(factor)` — the first non-trivial factor of N found across all kernel vectors.
/// `None` — all kernel vectors yielded trivial gcds; factorization failed at this stage.
pub fn factor(
    poly: &PolyPair,
    matrix: &SparseMatrix,
    relations: &[Relation],
    kernel_vectors: &[KernelVector],
) -> Option<BigInt> {
    for kv in kernel_vectors {
        let x = rational_sqrt(kv, matrix, relations, poly);
        let y = algebraic_sqrt(kv, matrix, relations, poly);
        if let Some(f) = factor_from_congruence(&x, &y, &poly.n) {
            return Some(f);
        }
    }
    None
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    // ── factor_from_congruence ──

    #[test]
    fn nontrivial_factor_via_diff() {
        // N = 35, X = 6, Y = 1. X − Y = 5. gcd(5, 35) = 5. Non-trivial. ✓
        let n = bi(35);
        let x = bi(6);
        let y = bi(1);
        let f = factor_from_congruence(&x, &y, &n).expect("should find factor 5");
        assert_eq!(f, bi(5));
    }

    #[test]
    fn trivial_gcd_x_eq_y() {
        // N = 35, X = 6, Y = 6. X − Y = 0. gcd(0, 35) = 35 (trivial). X + Y = 12. gcd(12, 35) = 1.
        let n = bi(35);
        let x = bi(6);
        let y = bi(6);
        assert!(factor_from_congruence(&x, &y, &n).is_none(), "X = Y should give trivial gcd");
    }

    #[test]
    fn trivial_gcd_x_eq_neg_y_mod_n() {
        // N = 35, X = 6, Y = 29. X + Y = 35 ≡ 0 (mod 35). gcd(35, 35) = 35 (trivial).
        // X − Y = −23. gcd(23, 35) = 1 (trivial).
        let n = bi(35);
        let x = bi(6);
        let y = bi(29); // 6 + 29 = 35 ≡ 0 (mod 35)
        assert!(
            factor_from_congruence(&x, &y, &n).is_none(),
            "X ≡ −Y (mod N) should give trivial gcd"
        );
    }

    #[test]
    fn nontrivial_factor_via_sum() {
        // N = 35, X = 5, Y = 30. X − Y = −25. gcd(25, 35) = 5. Non-trivial. ✓
        // (Also: X + Y = 35. gcd(35, 35) = 35 (trivial). But diff gives 5 first.)
        let n = bi(35);
        let x = bi(5);
        let y = bi(30);
        let f = factor_from_congruence(&x, &y, &n).expect("should find factor 5");
        assert!(f == bi(5) || f == bi(7), "factor should be 5 or 7, got {f}");
    }

    #[test]
    fn factor_n_equals_1_is_trivial() {
        // Edge case: N = 1. Any gcd with 1 is 1, which is not > 1.
        let n = bi(1);
        let x = bi(0);
        let y = bi(0);
        assert!(factor_from_congruence(&x, &y, &n).is_none());
    }
}
