//! Known-answer tests for Pollard rho factorization.
//!
//! Each test factors a known semiprime (product of two primes) and verifies
//! that all variants (Floyd, Brent, brent_batched, and the parallel `factor`)
//! return a correct non-trivial factor.  The check is: `1 < d < n` and
//! `n % d == 0`.
//!
//! Semiprimes span 30–80 bits to cover the plan's full pedagogical range.
//! For each semiprime the factors are listed but not asserted — rho may return
//! either factor; both are correct.

use rho::factor::{brent, brent_batched, factor, floyd};

/// Assert that `d` is a non-trivial factor of `n`.
fn check(d: Option<u128>, n: u128, label: &str) {
    let d = d.unwrap_or_else(|| panic!("{label}: returned None for n={n}"));
    assert!(d > 1, "{label}: factor {d} ≤ 1 for n={n}");
    assert!(d < n, "{label}: factor {d} ≥ n={n}");
    assert_eq!(n % d, 0, "{label}: {d} does not divide {n}");
}

/// Known semiprimes: `(n, p, q)` where `n = p * q` and both `p`, `q` are prime.
/// All `n < 2^80` per the supported range of `mulmod`.
const SEMIPRIMES: &[(u128, u128, u128)] = &[
    // Small sanity cases (easy for any variant)
    (15, 3, 5),
    (77, 7, 11),
    (221, 13, 17),
    (3_599, 59, 61),
    // ~20 bit
    (1_000_003 * 1_000_033, 1_000_003, 1_000_033),
    // ~30 bit
    (32_452_843 * 32_452_867, 32_452_843, 32_452_867),
    // ~40 bit
    (549_755_813_881 * 2, 549_755_813_881, 2), // even; trivial for factor()
    // ~40 bit proper semiprime
    (1_000_000_007 * 1_000_000_009, 1_000_000_007, 1_000_000_009),
    // ~50 bit (two ~25-bit primes)
    (33_554_467 * 33_554_473, 33_554_467, 33_554_473),
    // ~60 bit (two ~30-bit primes)
    (1_073_741_827 * 1_073_741_789, 1_073_741_827, 1_073_741_789),
    // ~64 bit (two ~32-bit primes): x < n < 2^64 so x*x < 2^128 — still needs carrying_mul.
    (4_294_967_291 * 4_294_967_311, 4_294_967_291, 4_294_967_311),
];

#[test]
fn floyd_known_semiprimes() {
    for &(n, _, _) in SEMIPRIMES {
        // Skip the even-number entry: floyd may loop forever on it (rho works on
        // odd composites; the even fast-path lives in factor()).
        if n % 2 == 0 {
            continue;
        }
        let d = floyd(n, 1, 2);
        check(d, n, &format!("floyd n={n}"));
    }
}

#[test]
fn brent_known_semiprimes() {
    for &(n, _, _) in SEMIPRIMES {
        if n % 2 == 0 {
            continue;
        }
        let d = brent(n, 1, 2);
        check(d, n, &format!("brent n={n}"));
    }
}

#[test]
fn brent_batched_known_semiprimes() {
    for &(n, _, _) in SEMIPRIMES {
        if n % 2 == 0 {
            continue;
        }
        let d = brent_batched(n, 1, 2, 128);
        check(d, n, &format!("brent_batched n={n}"));
    }
}

#[test]
fn factor_parallel_known_semiprimes() {
    for &(n, _, _) in SEMIPRIMES {
        let d = factor(n, 8, 128);
        check(d, n, &format!("factor n={n}"));
    }
}

#[test]
fn factor_even_fast_path() {
    // factor() has a dedicated even-number fast path.
    let d = factor(2 * 1_000_000_007, 4, 128);
    let n = 2 * 1_000_000_007;
    check(d, n, "even n");
    assert_eq!(d.unwrap(), 2);
}

#[test]
fn brent_batched_various_batch_sizes() {
    // Verify that different batch sizes all produce correct results.
    let n: u128 = 1_073_741_827 * 1_073_741_789;
    for &bs in &[1usize, 16, 64, 128, 256] {
        let d = brent_batched(n, 1, 2, bs);
        check(d, n, &format!("brent_batched batch_size={bs}"));
    }
}
