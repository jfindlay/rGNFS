//! Exact integer square root for `BigInt`.
//!
//! `isqrt(n)` returns `Some(x)` iff `n` is a perfect square `x²`, else `None`.

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// Compute the exact integer square root of `n`.
///
/// Returns `Some(x)` iff `n == x * x` for some non-negative integer `x`, else `None`.
///
/// - `isqrt(0) == Some(0)`.
/// - `isqrt(n) == None` for any negative `n`.
/// - Uses Newton's method to find the integer square root, then verifies exactness.
pub fn isqrt(n: &BigInt) -> Option<BigInt> {
    // Negative numbers have no real square root.
    if n.is_negative() {
        return None;
    }
    // Base case: √0 = 0.
    if n.is_zero() {
        return Some(BigInt::zero());
    }
    // Base case: √1 = 1.
    if n.is_one() {
        return Some(BigInt::one());
    }

    // Newton's method: start with an initial estimate, then iterate x ← (x + n/x) / 2.
    // The iteration converges to floor(√n). We stop when x² ≤ n < (x+1)².
    //
    // Initial estimate: use the bit length to get a power-of-two upper bound.
    // If n has b bits, then 2^((b+1)/2) > √n.
    let bits = n.bits(); // number of bits in |n|
    let init_shift = (bits + 1) / 2;
    let mut x = BigInt::one() << init_shift;

    loop {
        // Newton step: x_new = (x + n/x) / 2
        let x_new = (&x + n / &x) >> 1;
        if x_new >= x {
            // Converged: x is floor(√n).
            break;
        }
        x = x_new;
    }

    // Verify exactness.
    if &x * &x == *n {
        Some(x)
    } else {
        None
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    #[test]
    fn isqrt_zero() {
        assert_eq!(isqrt(&bi(0)), Some(bi(0)));
    }

    #[test]
    fn isqrt_one() {
        assert_eq!(isqrt(&bi(1)), Some(bi(1)));
    }

    #[test]
    fn isqrt_perfect_squares() {
        for k in 0i64..=20 {
            assert_eq!(isqrt(&bi(k * k)), Some(bi(k)), "isqrt({k}²) should be Some({k})");
        }
    }

    #[test]
    fn isqrt_non_squares() {
        for n in [2i64, 3, 5, 6, 7, 8, 10, 15, 24, 26] {
            assert_eq!(isqrt(&bi(n)), None, "isqrt({n}) should be None");
        }
    }

    #[test]
    fn isqrt_negative() {
        assert_eq!(isqrt(&bi(-1)), None);
        assert_eq!(isqrt(&bi(-4)), None);
    }

    #[test]
    fn isqrt_large_perfect_square() {
        // 1_000_000² = 10^12
        let n = BigInt::from(1_000_000i64) * BigInt::from(1_000_000i64);
        assert_eq!(isqrt(&n), Some(BigInt::from(1_000_000i64)));
    }
}
