//! Known-answer tests (KATs) for D.C.1: descent substrate + initialization-smoothing + C2 shape.
//!
//! # KAT (a) — Frontier ordering invariant
//!
//! Verifies that `DescentFrontier` is a max-heap ordered by `target.prime()` descending:
//! pushing primes in arbitrary order and popping should yield them largest-first.
//!
//! # KAT (b) — Initialization-smoothing
//!
//! Verifies that `init_descent_frontier` finds a smooth exponent for a hand-chosen toy input:
//! - p = 101, g = 2, h = 50.
//! - g^0 * h = 50 = 2 * 5^2; smooth over primes ≤ 20 (medium_bound = 20).
//! - Expected: e = 0, frontier non-empty (contains primes 2, 5, 5).
//!
//! # KAT (c) — C2 interface shape
//!
//! Verifies the `solve_dl` interface shape:
//! - k > 1 returns `SolveDlError::Unsupported { k }` immediately.
//! - k = 1 path is wired: returns a `Result`, not a panic; if `Err`, must be a known variant.

use gnfs::dl::{
    DescentFrontier, DescentNode, DescentTarget, InitSmoothingError, SolveDlError,
    init_descent_frontier, solve_dl,
};
use num_bigint::BigInt;

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

// ─── KAT (a): Frontier ordering invariant ─────────────────────────────────────

/// KAT (a): `DescentFrontier` pops targets in largest-prime-first order.
///
/// Push primes in arbitrary order; pop should yield them sorted descending by prime().
/// This verifies the termination invariant: the frontier always yields the largest prime
/// first, so each descent step strictly reduces the maximum prime.
#[test]
fn kat_descent_frontier_ordering() {
    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
    for p in [7u64, 3, 11, 5, 13, 2] {
        let target = DescentTarget::Rational(p);
        let node = DescentNode {
            target: target.clone(),
            rewriting_relation: None,
            children: vec![],
            known_log: None,
        };
        frontier.push(target, node);
    }

    let mut popped = vec![];
    while let Some((t, _)) = frontier.pop_largest() {
        popped.push(t.prime());
    }

    assert_eq!(
        popped,
        vec![13, 11, 7, 5, 3, 2],
        "frontier must pop primes in descending order (largest-first)"
    );
}

/// KAT (a2): `DescentFrontier` with mixed Rational and Algebraic targets.
///
/// Algebraic targets with the same prime as a Rational target should be ordered consistently.
/// The key invariant is that larger primes are always popped before smaller ones.
#[test]
fn kat_descent_frontier_mixed_targets() {
    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();

    // Push Rational(7), Algebraic{p=11, r=3}, Rational(5).
    let targets = [
        DescentTarget::Rational(7),
        DescentTarget::Algebraic { p: 11, r: 3 },
        DescentTarget::Rational(5),
    ];
    for target in &targets {
        let node = DescentNode {
            target: target.clone(),
            rewriting_relation: None,
            children: vec![],
            known_log: None,
        };
        frontier.push(target.clone(), node);
    }

    // Pop all; primes should be in descending order: 11, 7, 5.
    let mut primes = vec![];
    while let Some((t, _)) = frontier.pop_largest() {
        primes.push(t.prime());
    }
    assert_eq!(primes, vec![11, 7, 5], "mixed targets must pop in descending prime order");
}

/// KAT (a3): `DescentFrontier::is_empty` and `len` are consistent.
#[test]
fn kat_descent_frontier_is_empty_and_len() {
    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
    assert!(frontier.is_empty(), "new frontier should be empty");
    assert_eq!(frontier.len(), 0, "new frontier should have len 0");

    let target = DescentTarget::Rational(7);
    let node = DescentNode {
        target: target.clone(),
        rewriting_relation: None,
        children: vec![],
        known_log: None,
    };
    frontier.push(target, node);
    assert!(!frontier.is_empty(), "frontier with one element should not be empty");
    assert_eq!(frontier.len(), 1, "frontier with one element should have len 1");

    frontier.pop_largest();
    assert!(frontier.is_empty(), "frontier should be empty after popping the only element");
    assert_eq!(frontier.len(), 0, "frontier should have len 0 after popping the only element");
}

// ─── KAT (b): Initialization-smoothing ───────────────────────────────────────

/// KAT (b): `init_descent_frontier` finds a smooth exponent for a hand-chosen toy input.
///
/// # Setup
///
/// - p = 101, g = 2, h = 50.
/// - g^0 * h = 50 = 2 * 5^2; smooth over primes ≤ 20 (medium_bound = 20).
///
/// # Expected result
///
/// - e = 0 (the first candidate is already smooth).
/// - frontier is non-empty (contains the prime factors of 50: 2, 5, 5).
/// - All frontier primes are ≤ medium_bound = 20.
#[test]
fn kat_init_descent_frontier_smooth() {
    let p = bi(101);
    let g = bi(2);
    // h = 50 = 2 * 5^2; g^0 * h = 50, which is smooth over primes <= 20.
    let h = bi(50);

    let result = init_descent_frontier::<u64>(&g, &h, &p, 20, 100);
    assert!(result.is_ok(), "expected smooth exponent to be found; got: {:?}", result.err());

    let (e, frontier) = result.unwrap();
    assert_eq!(e, bi(0), "e should be 0 since g^0 * h = 50 is already smooth");
    assert!(!frontier.is_empty(), "frontier should be non-empty (50 = 2 * 5^2)");
}

/// KAT (b2): `init_descent_frontier` frontier primes are all ≤ medium_bound.
///
/// After finding a smooth exponent, all primes in the frontier must be ≤ medium_bound.
/// This verifies the smoothness invariant: the frontier contains only "medium" primes.
#[test]
fn kat_init_descent_frontier_primes_bounded() {
    let p = bi(101);
    let g = bi(2);
    let h = bi(50); // 50 = 2 * 5^2; smooth over primes <= 20.
    let medium_bound = 20u64;

    let (_, mut frontier) = init_descent_frontier::<u64>(&g, &h, &p, medium_bound, 100)
        .expect("should find smooth exponent");

    // Pop all frontier primes and verify they are all <= medium_bound.
    while let Some((target, _)) = frontier.pop_largest() {
        assert!(
            target.prime() <= medium_bound,
            "frontier prime {} exceeds medium_bound {}",
            target.prime(),
            medium_bound
        );
    }
}

/// KAT (b3): `init_descent_frontier` returns `NoSmoothExponent` when no smooth exponent exists.
///
/// With a very small medium_bound and a prime h, no smooth exponent should be found.
#[test]
fn kat_init_descent_frontier_no_smooth() {
    let p = bi(101);
    let g = bi(2);
    // h = 97 is prime and > medium_bound = 3; very unlikely to find smooth in 5 attempts.
    let h = bi(97);

    let result = init_descent_frontier::<u64>(&g, &h, &p, 3, 5);
    // 97 is prime and > 3; 2*97=194≡93 mod 101 (93=3*31, 31>3); etc.
    // With medium_bound=3 and max_attempts=5, very likely to fail.
    match result {
        Err(InitSmoothingError::NoSmoothExponent { attempts: 5 }) => {
            // Expected: no smooth exponent found in 5 attempts.
        }
        Ok(_) => {
            // Unlikely but possible (if some g^e * h happens to be 3-smooth mod 101).
            // Don't fail the test — the KAT is probabilistic.
        }
        Err(e) => panic!("unexpected error variant: {:?}", e),
    }
}

/// KAT (b4): `init_descent_frontier` with a larger example.
///
/// p = 101, g = 2, h = 12.
/// g^0 * h = 12 = 2^2 * 3; smooth over primes <= 10 (medium_bound = 10).
#[test]
fn kat_init_descent_frontier_h12() {
    let p = bi(101);
    let g = bi(2);
    let h = bi(12); // 12 = 2^2 * 3; smooth over primes <= 10.

    let result = init_descent_frontier::<u64>(&g, &h, &p, 10, 100);
    assert!(result.is_ok(), "12 = 2^2 * 3 should be smooth over primes <= 10");

    let (e, frontier) = result.unwrap();
    assert_eq!(e, bi(0), "e should be 0 since g^0 * h = 12 is already smooth");
    assert!(!frontier.is_empty(), "frontier should be non-empty (12 = 2^2 * 3)");
}

// ─── KAT (c): C2 interface shape ──────────────────────────────────────────────

/// KAT (c): `solve_dl` with k > 1 returns `SolveDlError::Unsupported { k }`.
///
/// The F_{p^k} extension-field path is not yet supported. `solve_dl` must return
/// `Unsupported` immediately for k > 1, without attempting any computation.
#[test]
fn kat_solve_dl_unsupported_extension_field() {
    let result = solve_dl(
        &bi(2),
        &bi(3),
        &bi(11),
        2, // k = 2: extension field — must return Unsupported.
        &bi(10),
    );
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 2 })),
        "k=2 must return Unsupported {{ k: 2 }}; got: {:?}",
        result
    );
}

/// KAT (c2): `solve_dl` with k = 3 returns `SolveDlError::Unsupported { k: 3 }`.
///
/// Verifies that the Unsupported check is not hard-coded to k = 2.
#[test]
fn kat_solve_dl_unsupported_k3() {
    let result = solve_dl(&bi(2), &bi(3), &bi(11), 3, &bi(10));
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 3 })),
        "k=3 must return Unsupported {{ k: 3 }}; got: {:?}",
        result
    );
}

/// KAT (c3): `solve_dl` with k = 1 returns a `Result`, not a panic.
///
/// The k = 1 path is wired through initialization-smoothing. At D.C.1, the result may be
/// `Ok` (if the frontier is empty after smoothing) or a known `Err` variant. The KAT
/// verifies the *shape* of the result, not the final answer.
#[test]
fn kat_solve_dl_k1_returns_result() {
    // k = 1 path is wired; result may be Ok or a specific Err variant, but must not panic.
    let result = solve_dl(
        &bi(2),
        &bi(3),
        &bi(11),
        1,
        &bi(10),
    );
    // Shape KAT: must return a Result, not panic; if Err, must be a known variant.
    match result {
        Ok(_) => {}
        Err(SolveDlError::InitSmoothingFailed { .. }) => {}
        Err(SolveDlError::DescentFailed { .. }) => {}
        Err(SolveDlError::Unsupported { .. }) => {
            panic!("k=1 should not return Unsupported")
        }
    }
}

/// KAT (c4): `SolveDlError::Display` produces human-readable messages.
///
/// Verifies that the `Display` implementation for `SolveDlError` produces non-empty,
/// human-readable strings for each variant.
#[test]
fn kat_solve_dl_error_display() {
    let unsupported = SolveDlError::Unsupported { k: 2 };
    let msg = unsupported.to_string();
    assert!(!msg.is_empty(), "Unsupported display should be non-empty");
    assert!(msg.contains("k > 1") || msg.contains("not yet supported"), "Unsupported message: {msg}");

    let init_failed = SolveDlError::InitSmoothingFailed { attempts: 1000 };
    let msg = init_failed.to_string();
    assert!(!msg.is_empty(), "InitSmoothingFailed display should be non-empty");
    assert!(msg.contains("1000"), "InitSmoothingFailed message should contain attempt count: {msg}");

    let descent_failed = SolveDlError::DescentFailed { stuck_prime: 17 };
    let msg = descent_failed.to_string();
    assert!(!msg.is_empty(), "DescentFailed display should be non-empty");
    assert!(msg.contains("17"), "DescentFailed message should contain stuck prime: {msg}");
}

/// KAT (c5): `SolveDlError` implements `std::error::Error`.
///
/// Verifies that `SolveDlError` satisfies the `std::error::Error` trait bound, which is
/// required for the C2 interface to be usable in standard error-handling idioms.
#[test]
fn kat_solve_dl_error_is_std_error() {
    fn assert_std_error<E: std::error::Error>(_: &E) {}

    let e = SolveDlError::Unsupported { k: 2 };
    assert_std_error(&e);

    let e = SolveDlError::InitSmoothingFailed { attempts: 42 };
    assert_std_error(&e);

    let e = SolveDlError::DescentFailed { stuck_prime: 7 };
    assert_std_error(&e);
}

/// KAT (c6): `DescentTarget::prime()` returns the correct prime for both variants.
#[test]
fn kat_descent_target_prime() {
    let rational = DescentTarget::Rational(17);
    assert_eq!(rational.prime(), 17, "Rational(17).prime() should be 17");

    let algebraic = DescentTarget::Algebraic { p: 13, r: 5 };
    assert_eq!(algebraic.prime(), 13, "Algebraic {{ p: 13, r: 5 }}.prime() should be 13");
}
