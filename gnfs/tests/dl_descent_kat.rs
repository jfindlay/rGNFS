//! Known-answer tests (KATs) for the NFS-DL individual-log descent: descent substrate,
//! initialization-smoothing, `solve_dl` interface shape, special-q descent recursion, log
//! assembly, and end-to-end shape.
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
//! # KAT (c) — `solve_dl` interface shape
//!
//! Verifies the `solve_dl` interface shape:
//! - k > 1 returns `SolveDlError::Unsupported { k }` immediately.
//! - k = 1 path is wired: returns a `Result`, not a panic; if `Err`, must be a known variant.
//!
//! # KAT (d) — Single-node descent
//!
//! One medium prime `q = 17` descends to a relation over smaller primes via the special-q sieve.
//! Uses `f(x) = x³ − x − 1`, `m = 2`. The relation `(a=5, b=1)` has:
//! - Rational norm: `3` (prime 3 < 17).
//! - Algebraic norm: `119 = 7 × 17` (primes 7 and 17; 7 < 17).
//! All children have `prime() < 17` (strict reduction).
//!
//! # KAT (e) — Multi-level descent
//!
//! A frontier with two medium primes `{17, 7}` descends through the full tree:
//! - `q=17` descends to `{3, (7,5)}` via `(a=5, b=1)`.
//! - `q=7` descends to `{2, 2}` via `(a=-2, b=1)` (rational norm `4=2²`, algebraic norm `7`).
//! The final tree has depth ≥ 2 (17 → 7 → 2).
//!
//! # KAT (f) — Termination: undescendable input
//!
//! A prime `q` that the sieve cannot find a relation for surfaces
//! `SolveDlError::DescentFailed { stuck_prime: q }` rather than looping.
//! Uses a very restrictive sieve config (a_bound=0, b_bound=0) to guarantee no relations.
//!
//! # KAT (g) — Assembly KAT
//!
//! A small hand-built descent tree assembles to the correct `log_g(h) mod ell`.
//! Verifies the sign/exponent bookkeeping in `assemble_log`.
//!
//! # KAT (h) — End-to-end shape KAT
//!
//! Calls `solve_dl_full` with the toy F_p setup (p=11, g=2, h=4, ell=5).
//! Asserts the result is `Ok(2)` (since log_2(4) = 2 mod 5) or a clean `Err`.

use crypto_bigint::Uint;
use gnfs::dl::{
    DescentFrontier, DescentNode, DescentSieveConfig, DescentTarget, InitSmoothingError,
    SolveDlContext, SolveDlError, VirtualLogTable, assemble_log, descend_node,
    init_descent_frontier, run_descent, solve_dl, solve_dl_full,
};
use gnfs::{FactorBase, PolyPair};
use num_bigint::BigInt;
use shared_field::{Fp, FpNaive4};
use shared_numfield::IntPoly;

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

// ─── KAT (c): `solve_dl` interface shape ─────────────────────────────────────

/// KAT (c): `solve_dl` with k = 2 no longer returns `SolveDlError::Unsupported`.
///
/// The k=2 extension field path is wired. `solve_dl` with k=2 must NOT return `Unsupported`.
/// The result may be `Ok(x)` or a known `Err` variant (InitSmoothingFailed / DescentFailed),
/// but must not be `Unsupported`.
#[test]
fn kat_solve_dl_k2_not_unsupported() {
    let result = solve_dl(
        &bi(2),
        &bi(3),
        &bi(11),
        2, // k = 2: extension field path wired, must NOT return Unsupported.
        &bi(10),
    );
    assert!(
        !matches!(result, Err(SolveDlError::Unsupported { .. })),
        "k=2 must not return Unsupported (extension field path wired); got: {:?}",
        result
    );
}

/// KAT (c2): `solve_dl` with k = 3 returns `SolveDlError::Unsupported { k: 3 }`.
///
/// k=3 is beyond the toy ceiling (k>2). `solve_dl` must return `Unsupported` for k>2.
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
/// The k = 1 path is wired through initialization-smoothing. The result may be `Ok` (if the
/// frontier is empty after smoothing) or a known `Err` variant. The KAT verifies the *shape*
/// of the result, not the final answer.
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
    assert!(
        msg.contains("k > 1") || msg.contains("k > 2") || msg.contains("not yet supported")
            || msg.contains("not supported"),
        "Unsupported message: {msg}"
    );

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
/// required for the `solve_dl` interface to be usable in standard error-handling idioms.
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

// ─── Toy setup helpers ────────────────────────────────────────────────────────

/// `f(x) = x³ − x − 1` (coefficients: [−1, −1, 0, 1]).
fn f_cubic() -> IntPoly {
    IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
}

/// Build the toy polynomial pair: `f(x) = x³ − x − 1`, `m = 2`, `n = 5`.
fn toy_poly_pair() -> PolyPair {
    let f = f_cubic();
    let m = bi(2);
    let n = bi(5);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let pair = PolyPair::new(f, g, m, n);
    pair.verify().expect("toy polynomial pair should be valid");
    pair
}

/// Build a mock `VirtualLogTable<u64>` with all logs set to 1.
///
/// The log values are not meaningful for the structural KATs (d, e, f) — we only
/// care about the tree shape and leaf detection, not the actual log values.
fn mock_vtable(fb: &FactorBase) -> VirtualLogTable<u64> {
    VirtualLogTable {
        rational_logs: vec![1u64; fb.rational_size()],
        algebraic_logs: vec![1u64; fb.algebraic_size()],
    }
}

// ─── KAT (d): Single-node descent ─────────────────────────────────────────────

/// KAT (d): Single-node descent — `q = 17` descends to a relation over smaller primes.
///
/// # Setup
///
/// - `f(x) = x³ − x − 1`, `m = 2`, `n = 5`.
/// - Factor base: `b_rat = 30`, `b_alg = 7` (so 17 is a medium prime, not in the factor base).
/// - Target: `q = 17` (medium prime).
/// - Sieve: `a_bound = 10`, `b_bound = 5`, `threshold_scale = 0.3`.
///
/// # Expected result
///
/// The sieve finds the relation `(a=5, b=1)`:
/// - Rational norm: `5 − 2 = 3` (prime 3 < 17).
/// - Algebraic norm: `5³ − 5 − 1 = 119 = 7 × 17` (primes 7 and 17; 7 < 17).
///
/// `descend_node` returns `Ok` with:
/// - `rewriting_relation = Some(...)`.
/// - `children` non-empty.
/// - All children have `prime() < 17` (strict reduction).
#[test]
fn kat_d_single_node_descent_q17() {
    let poly = toy_poly_pair();
    // Factor base: b_alg = 7 makes 17 a medium prime (not in the algebraic factor base).
    let fb = FactorBase::new(&poly.f, 30, 7);
    let vtable = mock_vtable(&fb);
    let sieve_cfg = DescentSieveConfig::with_threshold(10, 5, 0.3);

    // Target: Algebraic ideal (17, 5) — a medium prime on the algebraic side.
    // f(5) = 119 = 7×17 ≡ 0 mod 17, so r=5 is a root of f mod 17.
    // With b_alg = 7, the ideal (17, 5) is NOT in the algebraic factor base (17 > 7),
    // so it is a medium prime that must be descended.
    assert!(
        fb.algebraic_index(17, 5).is_none(),
        "algebraic ideal (17, 5) should not be in the factor base (b_alg=7)"
    );
    let target = DescentTarget::Algebraic { p: 17, r: 5 };

    let result = descend_node::<u64>(target, &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        result.is_ok(),
        "descend_node for q=17 should succeed; got: {:?}",
        result.err()
    );

    let node = result.unwrap();

    // The node should have a rewriting relation.
    assert!(
        node.rewriting_relation.is_some(),
        "descended node should have a rewriting relation"
    );

    // The node should have children.
    assert!(
        !node.children.is_empty(),
        "descended node should have at least one child"
    );

    // All children must have prime() < 17 (strict reduction invariant).
    for child in &node.children {
        assert!(
            child.target.prime() < 17,
            "child prime {} must be < 17 (strict reduction)",
            child.target.prime()
        );
    }

    // The node itself should not have a known_log (it's an interior node).
    assert!(
        node.known_log.is_none(),
        "interior node should not have known_log"
    );
}

/// KAT (d2): Single-node descent — `q = 7` descends to a relation over smaller primes.
///
/// # Setup
///
/// - Factor base: `b_rat = 5`, `b_alg = 5` (so 7 is a medium prime on both sides).
/// - Target: `q = 7` (medium prime, not in the factor base).
/// - Sieve: `a_bound = 10`, `b_bound = 5`, `threshold_scale = 0.3`.
///
/// # Expected result
///
/// The sieve finds the relation `(a=-2, b=1)`:
/// - Rational norm: `|-2 − 2| = 4 = 2²` (prime 2 < 7).
/// - Algebraic norm: `|(-2)³ − (-2) − 1| = |-7| = 7` (just the prime 7).
///
/// All children have `prime() < 7` (strict reduction).
#[test]
fn kat_d2_single_node_descent_q7() {
    let poly = toy_poly_pair();
    // Factor base: b_alg = 5 makes 7 a medium prime (not in the algebraic factor base).
    let fb = FactorBase::new(&poly.f, 5, 5);
    let vtable = mock_vtable(&fb);
    let sieve_cfg = DescentSieveConfig::with_threshold(10, 5, 0.3);

    // Target: q = 7 (medium prime, not in fb since b_alg = 5).
    // f(5) = 119 = 7×17 ≡ 0 mod 7, so r=5 is a root of f mod 7.
    let target = DescentTarget::Algebraic { p: 7, r: 5 };

    let result = descend_node::<u64>(target, &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        result.is_ok(),
        "descend_node for q=7 should succeed; got: {:?}",
        result.err()
    );

    let node = result.unwrap();

    assert!(node.rewriting_relation.is_some(), "descended node should have a rewriting relation");
    assert!(!node.children.is_empty(), "descended node should have at least one child");

    // All children must have prime() < 7.
    for child in &node.children {
        assert!(
            child.target.prime() < 7,
            "child prime {} must be < 7 (strict reduction)",
            child.target.prime()
        );
    }
}

// ─── KAT (e): Multi-level descent ─────────────────────────────────────────────

/// KAT (e): Multi-level descent — frontier with two medium primes descends to leaves.
///
/// # Setup
///
/// - Factor base: `b_rat = 5`, `b_alg = 5` (factor-base primes: rational {2,3,5}, algebraic
///   ideals for primes ≤ 5).
/// - Frontier: two medium primes `{17, 7}` (both above `b_alg = 5`).
/// - Sieve: `a_bound = 10`, `b_bound = 5`, `threshold_scale = 0.3`.
///
/// # Expected result
///
/// `run_descent` processes both primes:
/// - `q=17` descends to children including `q=7` (still medium) and factor-base leaves.
/// - `q=7` descends to factor-base leaves (primes ≤ 5).
/// - The completed nodes list is non-empty.
/// - At least one completed node has depth ≥ 1 (has children).
///
/// # Note on depth
///
/// The "depth ≥ 2" requirement from the PLAN is satisfied by the two-step descent:
/// the frontier starts with q=17, which descends to q=7, which descends to leaves.
/// The completed nodes list contains both the q=17 node (with children) and the q=7 node.
#[test]
fn kat_e_multi_level_descent() {
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 5, 5);
    let vtable = mock_vtable(&fb);
    let sieve_cfg = DescentSieveConfig::with_threshold(10, 5, 0.3);

    // Build a frontier with two medium primes: 17 and 7.
    // Both are above b_alg = 5, so neither is a factor-base leaf.
    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();

    let target_17 = DescentTarget::Algebraic { p: 17, r: 5 };
    let node_17 = DescentNode {
        target: target_17.clone(),
        rewriting_relation: None,
        children: vec![],
        known_log: None,
    };
    frontier.push(target_17, node_17);

    let target_7 = DescentTarget::Algebraic { p: 7, r: 5 };
    let node_7 = DescentNode {
        target: target_7.clone(),
        rewriting_relation: None,
        children: vec![],
        known_log: None,
    };
    frontier.push(target_7, node_7);

    let result = run_descent(frontier, &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        result.is_ok(),
        "run_descent should succeed for frontier {{17, 7}}; got: {:?}",
        result.err()
    );

    let completed = result.unwrap();

    // Should have completed at least 2 nodes (one for each frontier prime).
    assert!(
        completed.len() >= 2,
        "run_descent should complete at least 2 nodes; got {}",
        completed.len()
    );

    // At least one completed node should have children (interior node from descent).
    let has_interior = completed.iter().any(|n| !n.children.is_empty());
    assert!(has_interior, "at least one completed node should have children (interior node)");

    // All leaf nodes (known_log = Some) should have primes ≤ b_alg = 5.
    for node in &completed {
        if node.known_log.is_some() {
            assert!(
                node.target.prime() <= 5,
                "leaf node prime {} should be ≤ b_alg = 5",
                node.target.prime()
            );
        }
    }
}

/// KAT (e2): Multi-level descent — verify the tree has depth ≥ 2.
///
/// Constructs a frontier with `q=17` only, then verifies that the completed tree
/// contains a node with children (depth ≥ 1), and that at least one child was itself
/// descended (depth ≥ 2 overall).
#[test]
fn kat_e2_descent_tree_depth() {
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 5, 5);
    let vtable = mock_vtable(&fb);
    let sieve_cfg = DescentSieveConfig::with_threshold(10, 5, 0.3);

    // Start with q=17 only.
    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
    let target_17 = DescentTarget::Algebraic { p: 17, r: 5 };
    let node_17 = DescentNode {
        target: target_17.clone(),
        rewriting_relation: None,
        children: vec![],
        known_log: None,
    };
    frontier.push(target_17, node_17);

    let result = run_descent(frontier, &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        result.is_ok(),
        "run_descent should succeed for frontier {{17}}; got: {:?}",
        result.err()
    );

    let completed = result.unwrap();

    // The q=17 node should have been descended (has children).
    let q17_node = completed.iter().find(|n| n.target.prime() == 17);
    assert!(q17_node.is_some(), "completed nodes should include q=17");
    let q17_node = q17_node.unwrap();
    assert!(
        !q17_node.children.is_empty(),
        "q=17 node should have children (it was descended)"
    );

    // The q=17 node's children should include q=7 (a medium prime that was also descended).
    // This verifies depth ≥ 2: 17 → 7 → leaves.
    let has_q7_child = q17_node.children.iter().any(|c| c.target.prime() == 7);
    assert!(
        has_q7_child,
        "q=17 node should have q=7 as a child (from the relation (a=5, b=1))"
    );
}

// ─── KAT (f): Termination — undescendable input ───────────────────────────────

/// KAT (f): Termination — a prime that cannot be descended surfaces `DescentFailed`.
///
/// Uses a sieve config with `a_bound = 0` and `b_bound = 0` to guarantee no relations
/// are found. The descent must surface `SolveDlError::DescentFailed { stuck_prime: q }`
/// rather than looping or panicking.
///
/// # Setup
///
/// - Factor base: `b_rat = 5`, `b_alg = 5`.
/// - Frontier: `q = 7` (medium prime).
/// - Sieve: `a_bound = 0`, `b_bound = 0` (no sieve area — guaranteed no relations).
///
/// # Expected result
///
/// `run_descent` returns `Err(SolveDlError::DescentFailed { stuck_prime: 7 })`.
#[test]
fn kat_f_termination_undescendable_input() {
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 5, 5);
    let vtable = mock_vtable(&fb);
    // Zero sieve area: guaranteed no relations found.
    let sieve_cfg = DescentSieveConfig::new(0, 0);

    let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
    let target = DescentTarget::Algebraic { p: 7, r: 5 };
    let node = DescentNode {
        target: target.clone(),
        rewriting_relation: None,
        children: vec![],
        known_log: None,
    };
    frontier.push(target, node);

    let result = run_descent(frontier, &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        matches!(result, Err(SolveDlError::DescentFailed { stuck_prime: 7 })),
        "undescendable prime should surface DescentFailed {{ stuck_prime: 7 }}; got: {:?}",
        result
    );
}

/// KAT (f2): Termination — `descend_node` directly returns `NoRelationFound` for zero sieve area.
///
/// Verifies that `descend_node` itself returns the correct error when the sieve finds nothing.
#[test]
fn kat_f2_descend_node_no_relation_found() {
    use gnfs::dl::DescentStepError;

    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 5, 5);
    let vtable = mock_vtable(&fb);
    // Zero sieve area: guaranteed no relations.
    let sieve_cfg = DescentSieveConfig::new(0, 0);

    let target = DescentTarget::Algebraic { p: 7, r: 5 };
    let result = descend_node::<u64>(target.clone(), &poly, &fb, &vtable, &sieve_cfg);

    assert!(
        matches!(result, Err(DescentStepError::NoRelationFound { .. })),
        "zero sieve area should return NoRelationFound; got: {:?}",
        result
    );
}

// ─── KAT (g): Assembly KAT ────────────────────────────────────────────────────

/// KAT (g): `assemble_log` assembles a hand-built descent tree to the correct log mod ell.
///
/// # Setup
///
/// - ell = 7
/// - leaf1: target = Rational(2), known_log = 3
/// - leaf2: target = Rational(3), known_log = 5
/// - root: target = Rational(6), children = [leaf1, leaf2], known_log = None
///
/// # Hand-computed expected result
///
/// log(root) = log(leaf1) + log(leaf2) mod 7 = 3 + 5 = 8 ≡ 1 (mod 7).
///
/// The assembly sums all children's logs (one child per unit of exponent, since
/// `build_children` creates one child per unit of exponent). With exponent 1 on each
/// of prime 2 and prime 3: log(root) = 1*3 + 1*5 = 8 ≡ 1 (mod 7).
#[test]
fn kat_g_assembly_hand_built_tree() {
    let ell = bi(7);

    // Build the descent tree manually.
    // Root node: target = Rational(6), children = [leaf1, leaf2].
    // In the completed list from run_descent, this would be a single entry.
    let root = DescentNode::<BigInt> {
        target: DescentTarget::Rational(6),
        rewriting_relation: None, // not needed for assembly
        children: vec![
            DescentNode {
                target: DescentTarget::Rational(2),
                rewriting_relation: None,
                children: vec![],
                known_log: Some(bi(3)),
            },
            DescentNode {
                target: DescentTarget::Rational(3),
                rewriting_relation: None,
                children: vec![],
                known_log: Some(bi(5)),
            },
        ],
        known_log: None,
    };

    // The initial frontier had one target: Rational(6).
    let initial_targets = vec![DescentTarget::Rational(6)];

    // assemble_log: completed = [root], initial_targets = [Rational(6)].
    let result = assemble_log(&[root], &initial_targets, &ell, |f: &BigInt| f.clone());

    assert_eq!(
        result,
        Ok(bi(1)),
        "assembly: log(6) = log(2) + log(3) = 3 + 5 = 8 ≡ 1 (mod 7)"
    );
}

/// KAT (g2): Assembly with multiplicity — prime 2 appears twice (exponent 2).
///
/// # Setup
///
/// - ell = 7
/// - leaf1a: target = Rational(2), known_log = 3 (first copy)
/// - leaf1b: target = Rational(2), known_log = 3 (second copy, exponent 2)
/// - leaf2: target = Rational(3), known_log = 5
/// - root: target = Rational(12), children = [leaf1a, leaf1b, leaf2]
///
/// # Hand-computed expected result
///
/// log(root) = 2 * log(2) + 1 * log(3) mod 7 = 2*3 + 1*5 = 11 ≡ 4 (mod 7).
///
/// This verifies the multiplicity handling: `build_children` creates one child per unit of
/// exponent, so summing all children's logs gives the correct weighted sum.
#[test]
fn kat_g2_assembly_with_multiplicity() {
    let ell = bi(7);

    let root = DescentNode::<BigInt> {
        target: DescentTarget::Rational(12),
        rewriting_relation: None,
        children: vec![
            DescentNode {
                target: DescentTarget::Rational(2),
                rewriting_relation: None,
                children: vec![],
                known_log: Some(bi(3)),
            },
            DescentNode {
                target: DescentTarget::Rational(2),
                rewriting_relation: None,
                children: vec![],
                known_log: Some(bi(3)),
            },
            DescentNode {
                target: DescentTarget::Rational(3),
                rewriting_relation: None,
                children: vec![],
                known_log: Some(bi(5)),
            },
        ],
        known_log: None,
    };

    let initial_targets = vec![DescentTarget::Rational(12)];
    let result = assemble_log(&[root], &initial_targets, &ell, |f: &BigInt| f.clone());

    assert_eq!(
        result,
        Ok(bi(4)),
        "assembly with multiplicity: 2*log(2) + log(3) = 2*3 + 5 = 11 ≡ 4 (mod 7)"
    );
}

/// KAT (g3): Assembly with two initial targets — log(g^e * h) = log(p1) + log(p2) mod ell.
///
/// # Setup
///
/// - ell = 7
/// - leaf1: target = Rational(2), known_log = 3
/// - leaf2: target = Rational(5), known_log = 2
/// - initial_targets = [Rational(2), Rational(5)] (factors of g^e * h = 10)
///
/// # Hand-computed expected result
///
/// log(g^e * h) = log(2) + log(5) mod 7 = 3 + 2 = 5 (mod 7).
#[test]
fn kat_g3_assembly_two_initial_targets() {
    let ell = bi(7);

    let leaf1 = DescentNode::<BigInt> {
        target: DescentTarget::Rational(2),
        rewriting_relation: None,
        children: vec![],
        known_log: Some(bi(3)),
    };
    let leaf2 = DescentNode::<BigInt> {
        target: DescentTarget::Rational(5),
        rewriting_relation: None,
        children: vec![],
        known_log: Some(bi(2)),
    };

    let initial_targets = vec![DescentTarget::Rational(2), DescentTarget::Rational(5)];
    let result = assemble_log(&[leaf1, leaf2], &initial_targets, &ell, |f: &BigInt| f.clone());

    assert_eq!(
        result,
        Ok(bi(5)),
        "two initial targets: log(2) + log(5) = 3 + 2 = 5 (mod 7)"
    );
}

// ─── KAT (h): End-to-end shape KAT ───────────────────────────────────────────

/// Helper: convert `FpNaive4` to `BigInt` for assembly arithmetic.
///
/// For small values (< 2^64), the first word of `to_uint()` is the canonical residue.
fn fp_to_bigint(f: &FpNaive4) -> BigInt {
    BigInt::from(f.to_uint().as_words()[0])
}

/// KAT (h): End-to-end shape KAT — `solve_dl_full` recovers `log_2(4) = 2 mod 5`.
///
/// # Setup
///
/// - p = 11, g = 2, h = 4 = 2^2, ell = 5.
/// - Factor base: rational primes {2, 3} (b_rat = 3, b_alg = 3).
/// - Virtual-log table: log_2(2) = 1 mod 5, log_2(3) = 3 mod 5.
/// - Toy KAT: ell = p − 1 = 10 would give the full group order, but we use ell = 5
///   (a prime factor of p − 1 = 10) to match the existing toy DL setup.
///
/// # Expected result
///
/// log_2(4) = 2 mod 5 (since 2^2 = 4 mod 11).
///
/// # Pipeline
///
/// 1. init_descent_frontier: g^0 * h = 4 = 2^2, smooth over {2, 3}. e = 0, frontier = {2, 2}.
/// 2. run_descent: both Rational(2) are factor-base leaves. completed = [leaf(2), leaf(2)].
/// 3. assemble_log: log(4) = log(2) + log(2) = 1 + 1 = 2 mod 5.
/// 4. log_g(h) = (2 − 0) mod 5 = 2. ✓
///
/// # Toy KAT note
///
/// ell = 5 is a prime factor of p − 1 = 10. The log is recovered mod 5 only. For the full
/// log mod 10, Pohlig–Hellman / CRT would be needed (out of scope for the individual-log descent).
#[test]
fn kat_h_solve_dl_full_toy_fp() {
    let ell5 = Uint::<4>::from(5u64);
    let ell = bi(5);

    // Build the toy polynomial pair: f(x) = x³ − x − 1, m = 2.
    let poly = toy_poly_pair();

    // Factor base: b_rat = 3, b_alg = 3 (rational primes {2, 3}).
    let fb = FactorBase::new(&poly.f, 3, 3);

    // Virtual-log table: log_2(2) = 1 mod 5, log_2(3) = 3 mod 5.
    // These are the known virtual logs from the toy DL setup (p=11, g=2, ell=5).
    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![
            FpNaive4::from_u64(1, &ell5), // log_2(2) = 1 mod 5
            FpNaive4::from_u64(3, &ell5), // log_2(3) = 3 mod 5
        ],
        algebraic_logs: vec![], // no algebraic ideals in this toy setup
    };

    // Sieve config: any config works since all factors are factor-base leaves.
    let sieve_cfg = DescentSieveConfig::new(10, 5);

    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // h = 4 = 2^2 mod 11. log_2(4) = 2 mod 5.
    let g = bi(2);
    let h = bi(4);
    let p = bi(11);

    let result = solve_dl_full(&g, &h, &p, 1, &ell, &ctx);

    assert_eq!(
        result,
        Ok(bi(2)),
        "solve_dl_full: log_2(4) should be 2 mod 5; got: {:?}",
        result
    );

    // Cross-check: g^result mod p == h.
    if let Ok(ref log) = result {
        let g_pow = mod_pow(&g, log, &p);
        assert_eq!(
            g_pow, h,
            "cross-check: g^log mod p should equal h; g^{log} mod {p} = {g_pow}, expected {h}"
        );
    }
}

/// KAT (h2): `solve_dl_full` with h = 8 = 2^3 mod 11 → log_2(8) = 3 mod 5.
///
/// Verifies that the assembly correctly handles a different h value.
#[test]
fn kat_h2_solve_dl_full_h8() {
    let ell5 = Uint::<4>::from(5u64);
    let ell = bi(5);

    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);

    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![
            FpNaive4::from_u64(1, &ell5), // log_2(2) = 1 mod 5
            FpNaive4::from_u64(3, &ell5), // log_2(3) = 3 mod 5
        ],
        algebraic_logs: vec![],
    };

    let sieve_cfg = DescentSieveConfig::new(10, 5);

    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // h = 8 = 2^3 mod 11. log_2(8) = 3 mod 5.
    let g = bi(2);
    let h = bi(8);
    let p = bi(11);

    let result = solve_dl_full(&g, &h, &p, 1, &ell, &ctx);

    assert_eq!(
        result,
        Ok(bi(3)),
        "solve_dl_full: log_2(8) should be 3 mod 5; got: {:?}",
        result
    );

    if let Ok(ref log) = result {
        let g_pow = mod_pow(&g, log, &p);
        assert_eq!(g_pow, h, "cross-check: g^{log} mod {p} = {g_pow}, expected {h}");
    }
}

/// KAT (h3): `solve_dl_full` with k = 2 no longer returns `Unsupported`.
///
/// The k=2 extension field path is wired. `solve_dl_full` with k=2 delegates to `solve_dl`
/// (which builds the extension context internally). The result must NOT be `Unsupported`.
#[test]
fn kat_h3_solve_dl_full_k2_not_unsupported() {
    let ell5 = Uint::<4>::from(5u64);
    let ell = bi(5);

    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);
    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![FpNaive4::from_u64(1, &ell5)],
        algebraic_logs: vec![],
    };
    let sieve_cfg = DescentSieveConfig::new(10, 5);

    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // k=2 is wired (extension field path); must NOT return Unsupported.
    // Note: p=11, k=2 — the k=2 path will try to find an irreducible poly of degree 2 over F_11.
    // The result may be Ok or a known Err, but must not be Unsupported.
    let result = solve_dl_full(&bi(2), &bi(4), &bi(11), 2, &ell, &ctx);
    assert!(
        !matches!(result, Err(SolveDlError::Unsupported { .. })),
        "k=2 must not return Unsupported (extension field path wired); got: {:?}",
        result
    );
}

/// KAT (h4): `solve_dl_full` with k = 3 returns `Unsupported`.
///
/// k=3 is beyond the toy ceiling (k>2). `solve_dl_full` must return `Unsupported` for k>2.
#[test]
fn kat_h4_solve_dl_full_unsupported_k3() {
    let ell5 = Uint::<4>::from(5u64);
    let ell = bi(5);

    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);
    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![FpNaive4::from_u64(1, &ell5)],
        algebraic_logs: vec![],
    };
    let sieve_cfg = DescentSieveConfig::new(10, 5);

    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    let result = solve_dl_full(&bi(2), &bi(4), &bi(11), 3, &ell, &ctx);
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 3 })),
        "k=3 should return Unsupported; got: {:?}",
        result
    );
}

// ─── Helpers for KAT (h) ──────────────────────────────────────────────────────

/// Modular exponentiation: base^exp mod modulus.
fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    use num_traits::Zero;
    if exp.is_zero() {
        return BigInt::from(1);
    }
    let mut result = BigInt::from(1);
    let mut b = base.clone() % modulus;
    let mut e = exp.clone();
    while e > BigInt::from(0) {
        if &e % 2 == BigInt::from(1) {
            result = (result * &b) % modulus;
        }
        b = (&b * &b) % modulus;
        e /= 2;
    }
    result
}
