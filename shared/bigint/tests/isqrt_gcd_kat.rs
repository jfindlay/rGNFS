//! Known-answer tests (KATs) for `isqrt` and `gcd` in `shared-bigint`.
//!
//! `isqrt` KATs:
//! - `isqrt(k²) = Some(k)` for several k.
//! - `isqrt(k² + 1) = None` for several k.
//! - `isqrt(0) = Some(0)`.
//! - `isqrt(negative) = None`.
//!
//! `gcd` KATs:
//! - Known gcds for small pairs.
//! - `gcd(0, n) = |n|`.
//! - `gcd(n, 0) = |n|`.
//! - `gcd(0, 0) = 0`.
//! - Non-negative result for mixed-sign inputs.

use num_bigint::BigInt;
use shared_bigint::{gcd, isqrt};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

// ─── isqrt KATs ───────────────────────────────────────────────────────────────

/// isqrt(0) = Some(0).
#[test]
fn kat_isqrt_zero() {
    assert_eq!(isqrt(&bi(0)), Some(bi(0)));
}

/// isqrt(k²) = Some(k) for k = 1, 2, 3, 4, 5, 9, 12, 25, 100.
#[test]
fn kat_isqrt_perfect_squares() {
    let cases: &[(i64, i64)] = &[
        (1, 1),
        (4, 2),
        (9, 3),
        (16, 4),
        (25, 5),
        (81, 9),
        (144, 12),
        (625, 25),
        (10_000, 100),
    ];
    for &(n, expected_root) in cases {
        assert_eq!(
            isqrt(&bi(n)),
            Some(bi(expected_root)),
            "isqrt({n}) should be Some({expected_root})"
        );
    }
}

/// isqrt(k² + 1) = None for several k.
#[test]
fn kat_isqrt_non_squares() {
    // k² + 1 is never a perfect square for k ≥ 1 (since (k+1)² = k² + 2k + 1 > k² + 1 for k ≥ 1).
    let cases: &[i64] = &[2, 3, 5, 7, 8, 10, 15, 17, 24, 26, 99, 101];
    for &n in cases {
        assert_eq!(isqrt(&bi(n)), None, "isqrt({n}) should be None");
    }
}

/// isqrt(negative) = None.
#[test]
fn kat_isqrt_negative() {
    for n in [-1i64, -4, -9, -100, -1_000_000] {
        assert_eq!(isqrt(&bi(n)), None, "isqrt({n}) should be None");
    }
}

/// isqrt works for large perfect squares.
#[test]
fn kat_isqrt_large_perfect_square() {
    // 999_999² = 999_998_000_001
    let root = BigInt::from(999_999i64);
    let n = &root * &root;
    assert_eq!(isqrt(&n), Some(root.clone()), "isqrt(999_999²) should be Some(999_999)");

    // 1_000_000² = 10^12
    let root2 = BigInt::from(1_000_000i64);
    let n2 = &root2 * &root2;
    assert_eq!(isqrt(&n2), Some(root2.clone()), "isqrt(1_000_000²) should be Some(1_000_000)");
}

/// isqrt(k² + 1) = None for large k (one-off from a perfect square).
#[test]
fn kat_isqrt_large_non_square() {
    let root = BigInt::from(1_000_000i64);
    let n = &root * &root + BigInt::from(1);
    assert_eq!(isqrt(&n), None, "isqrt(1_000_000² + 1) should be None");
}

// ─── gcd KATs ─────────────────────────────────────────────────────────────────

/// gcd(0, n) = |n|.
#[test]
fn kat_gcd_zero_left() {
    assert_eq!(gcd(&bi(0), &bi(12)), bi(12));
    assert_eq!(gcd(&bi(0), &bi(-7)), bi(7));
    assert_eq!(gcd(&bi(0), &bi(0)), bi(0));
}

/// gcd(n, 0) = |n|.
#[test]
fn kat_gcd_zero_right() {
    assert_eq!(gcd(&bi(15), &bi(0)), bi(15));
    assert_eq!(gcd(&bi(-9), &bi(0)), bi(9));
}

/// gcd matches known values for small positive pairs.
#[test]
fn kat_gcd_known_values() {
    let cases: &[(i64, i64, i64)] = &[
        (12, 8, 4),
        (35, 14, 7),
        (100, 75, 25),
        (17, 13, 1),   // coprime
        (6, 6, 6),     // equal
        (1, 100, 1),   // 1 is coprime to everything
        (48, 36, 12),
        (1001, 77, 77), // 1001 = 7·11·13, 77 = 7·11
    ];
    for &(a, b, expected) in cases {
        assert_eq!(gcd(&bi(a), &bi(b)), bi(expected), "gcd({a}, {b}) should be {expected}");
    }
}

/// gcd returns a non-negative result for mixed-sign inputs.
#[test]
fn kat_gcd_mixed_sign() {
    // gcd(−12, 8) = 4
    assert_eq!(gcd(&bi(-12), &bi(8)), bi(4));
    // gcd(12, −8) = 4
    assert_eq!(gcd(&bi(12), &bi(-8)), bi(4));
    // gcd(−12, −8) = 4
    assert_eq!(gcd(&bi(-12), &bi(-8)), bi(4));
    // gcd(−35, −14) = 7
    assert_eq!(gcd(&bi(-35), &bi(-14)), bi(7));
}

/// gcd result is always non-negative.
#[test]
fn kat_gcd_always_nonneg() {
    let pairs: &[(i64, i64)] = &[
        (0, 0),
        (1, -1),
        (-5, 3),
        (-100, -75),
        (17, -17),
    ];
    for &(a, b) in pairs {
        let g = gcd(&bi(a), &bi(b));
        assert!(g >= bi(0), "gcd({a}, {b}) = {g} should be non-negative");
    }
}
