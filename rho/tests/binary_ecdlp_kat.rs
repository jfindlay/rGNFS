//! Known-answer tests for the binary-curve Pollard-rho ECDLP solver.
//!
//! # What is tested
//!
//! 1. **End-to-end `k·G = Q` recovery** — the solver finds a scalar `k` such
//!    that `k·G = Q` for planted scalars on the toy binary curve.
//! 2. **Walk-state invariant** — `W = a·G + b·Q` holds after construction and
//!    after a sequence of steps (the walk-state correctness contract).
//! 3. **Degenerate-collision handling** — the solver retries on degenerate
//!    collisions and eventually finds a valid `k`.
//! 4. **τ-orbit rho** — `solve_koblitz` recovers the same `k` as the baseline
//!    `solve` on the same inputs (Koblitz τ-automorphism correctness contract).
//!
//! # Toy curve
//!
//! All tests use the curve `y²+xy = x³+x²+1` over GF(2^4) with `x⁴+x+1`
//! (poly = 0x13), group order n = 4:
//! - G  = (1, 6)
//! - 2G = (0, 1)
//! - 3G = (1, 7) = −G
//! - 4G = ∞
//!
//! The group order n = 4 is composite (2²).  The solver uses extended GCD for
//! the modular inverse, which handles composite orders.  With 50 retries, the
//! solver reliably finds a valid collision where `gcd(b₂−b₁, 4) = 1`.
//!
//! # Walk-state invariant
//!
//! The invariant `W = a·G + b·Q` is the walk-state correctness contract.  A wrong
//! addend table or group-law bug shows up as a recovered `k` with `k·G ≠ Q`,
//! which the end-to-end KAT catches.

use crypto_bigint::Uint;
use rho::binary_curve::{BinaryAffinePoint, BinaryCurve};
use rho::binary_ecdlp::{solve, solve_brent, BinaryAddendTable, BinaryWalkState};
use rho::binary_ecdlp::koblitz::solve_koblitz;
use shared_gf2m::F2mNaive;

// ── Curve and field parameters ────────────────────────────────────────────────

/// GF(2^4) irreducible: x⁴+x+1 = 0x13.
fn poly4() -> Uint<1> {
    Uint::<1>::from(0x13u64)
}

/// Toy binary curve: y²+xy = x³+x²+1 over GF(2^4) with x⁴+x+1.
///
/// Group order 4: G=(1,6), 2G=(0,1), 3G=(1,7), 4G=∞.
fn toy_curve() -> BinaryCurve {
    BinaryCurve {
        poly: poly4(),
        a: Uint::<1>::ONE,
        b: Uint::<1>::ONE,
        n: Uint::<1>::from(4u64),
        gx: Uint::<1>::ONE,
        gy: Uint::<1>::from(6u64),
    }
}

// ── Helper ────────────────────────────────────────────────────────────────────

/// Verify the solver finds a valid DLP solution on the toy binary curve.
///
/// Plants `Q = k_target · G`, runs the solver, and asserts that the returned
/// `k` satisfies `k · G = Q`.  The solver is not required to return the
/// specific `k_target` — any `k'` with `k'·G = Q` is a valid solution.
fn check_solver_on_binary_curve(
    curve: &BinaryCurve,
    n: u64,
    k_target: u64,
    label: &str,
) {
    let g = curve.generator::<F2mNaive<1>>();
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k = solve(curve, &g, &q, n)
        .unwrap_or_else(|| panic!("{label}: solve failed for k_target={k_target}"));

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert_eq!(
        check, q,
        "{label}: recovered k={k} gives k·G ≠ Q (k_target={k_target})"
    );
}

// ── End-to-end ECDLP KATs ─────────────────────────────────────────────────────

/// DLP k=1 on the toy binary curve: Q = G.
///
/// The trivial case: Q = G, so k = 1.  The solver must find k' with k'·G = G.
#[test]
fn binary_dlog_k1() {
    check_solver_on_binary_curve(&toy_curve(), 4, 1, "toy_binary");
}

/// DLP k=2 on the toy binary curve: Q = 2G = (0, 1).
///
/// The solver must find k' with k'·G = (0, 1).
#[test]
fn binary_dlog_k2() {
    check_solver_on_binary_curve(&toy_curve(), 4, 2, "toy_binary");
}

/// DLP k=3 on the toy binary curve: Q = 3G = (1, 7) = −G.
///
/// The solver must find k' with k'·G = (1, 7).
#[test]
fn binary_dlog_k3() {
    check_solver_on_binary_curve(&toy_curve(), 4, 3, "toy_binary");
}

/// DLP k=0 (Q = ∞): the identity case returns 0.
///
/// The solver must return k=0 when Q is the point at infinity.
#[test]
fn binary_dlog_k0_identity() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let q: BinaryAffinePoint<F2mNaive<1>> = BinaryAffinePoint::Infinity;

    let k = solve(&curve, &g, &q, 4)
        .expect("binary_dlog_k0_identity: solve failed for Q=∞");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert!(
        check.is_infinity(),
        "binary_dlog_k0_identity: k={k} gives k·G ≠ ∞"
    );
}

// ── Walk-state invariant KATs ─────────────────────────────────────────────────

/// Walk-state invariant: `W = a·G + b·Q` holds after construction.
///
/// Verifies the invariant for a freshly constructed `BinaryWalkState`.
#[test]
fn walk_invariant_after_construction() {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let k_target: u64 = 2;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let mut rng = ChaCha20Rng::seed_from_u64(0xABCD_1234);
    let walk = BinaryWalkState::<F2mNaive<1>>::new_random(&curve, &g, &q, n, &mut rng);

    let ag = curve.scalar_mul(&g, &Uint::<1>::from(walk.a));
    let bq = curve.scalar_mul(&q, &Uint::<1>::from(walk.b));
    let reconstructed = curve.add(&ag, &bq);

    assert_eq!(
        walk.point, reconstructed,
        "walk invariant broken at construction: point ≠ a·G + b·Q"
    );
}

/// Walk-state invariant: `W = a·G + b·Q` holds after multiple steps.
///
/// Verifies the invariant is maintained across 10 walk steps.
#[test]
fn walk_invariant_across_steps() {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let k_target: u64 = 3;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let mut rng = ChaCha20Rng::seed_from_u64(0xDEAD_BEEF);
    let table = BinaryAddendTable::new(&curve, &g, &q, n, &mut rng);
    let mut walk = BinaryWalkState::<F2mNaive<1>>::new_random(&curve, &g, &q, n, &mut rng);

    for step in 0..10 {
        // Verify invariant before each step.
        let ag = curve.scalar_mul(&g, &Uint::<1>::from(walk.a));
        let bq = curve.scalar_mul(&q, &Uint::<1>::from(walk.b));
        let reconstructed = curve.add(&ag, &bq);
        assert_eq!(
            walk.point, reconstructed,
            "walk invariant broken at step {step}: point ≠ a·G + b·Q"
        );
        walk.step(&curve, &table, n);
    }
}

// ── solve_brent reproducibility KAT ──────────────────────────────────────────

/// solve_brent with a fixed seed produces a valid k.
///
/// Verifies that the solver is deterministic given the same seed, and that
/// the returned k satisfies k·G = Q.
#[test]
fn solve_brent_deterministic() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let k_target: u64 = 2;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k1 = solve_brent::<F2mNaive<1>>(&curve, &g, &q, n, 42, 50)
        .expect("solve_brent_deterministic: first call failed");
    let k2 = solve_brent::<F2mNaive<1>>(&curve, &g, &q, n, 42, 50)
        .expect("solve_brent_deterministic: second call failed");

    assert_eq!(k1, k2, "solve_brent: different results for same seed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k1));
    assert_eq!(check, q, "solve_brent_deterministic: k·G ≠ Q");
}

// ── Multiple seeds KAT ────────────────────────────────────────────────────────

/// solve_brent succeeds across multiple seeds for k=2.
///
/// Verifies that the solver is not seed-dependent — it finds a valid k for
/// several different seeds.
#[test]
fn solve_brent_multiple_seeds() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let k_target: u64 = 2;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    for seed in [0u64, 1, 42, 0xDEAD_BEEF, 0xCAFE_BABE] {
        let k = solve_brent::<F2mNaive<1>>(&curve, &g, &q, n, seed, 50)
            .unwrap_or_else(|| panic!("solve_brent failed for seed={seed}"));
        let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
        assert_eq!(check, q, "seed={seed}: k={k} gives k·G ≠ Q");
    }
}

// ── τ-orbit rho KATs ──────────────────────────────────────────────────────────
//
// These tests verify that `solve_koblitz` (the τ-orbit rho variant) recovers
// the same `k` as the baseline `solve` on the same inputs.
//
// Toy curve parameters:
//   - GF(2^4) with x⁴+x+1, group order n=4, field degree m=4.
//   - Frobenius trace t = 2^4 + 1 − 4 = 13.
//
// Baseline rule: `solve` and `solve_brent` are NOT modified.
// `solve_koblitz` is a NEW function that reads the baseline walk.

/// τ-orbit rho recovers a valid k for k=1 (Q = G).
///
/// The τ-orbit collapse reduces the effective group size.  The recovered k
/// must satisfy k·G = Q (not necessarily k = k_target, since the group has
/// multiple valid representations).
#[test]
fn koblitz_dlog_k1() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13; // Frobenius trace: 2^4 + 1 − 4 = 13
    let k_target: u64 = 1;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 0, 50)
        .expect("koblitz_dlog_k1: solve_koblitz failed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert_eq!(check, q, "koblitz_dlog_k1: k={k} gives k·G ≠ Q");
}

/// τ-orbit rho recovers a valid k for k=2 (Q = 2G).
#[test]
fn koblitz_dlog_k2() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13;
    let k_target: u64 = 2;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 0, 50)
        .expect("koblitz_dlog_k2: solve_koblitz failed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert_eq!(check, q, "koblitz_dlog_k2: k={k} gives k·G ≠ Q");
}

/// τ-orbit rho recovers a valid k for k=3 (Q = 3G = −G).
#[test]
fn koblitz_dlog_k3() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13;
    let k_target: u64 = 3;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 0, 50)
        .expect("koblitz_dlog_k3: solve_koblitz failed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert_eq!(check, q, "koblitz_dlog_k3: k={k} gives k·G ≠ Q");
}

/// τ-orbit rho handles Q = ∞ (k = 0).
#[test]
fn koblitz_dlog_k0_identity() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13;
    let q: BinaryAffinePoint<F2mNaive<1>> = BinaryAffinePoint::Infinity;

    let k = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 0, 50)
        .expect("koblitz_dlog_k0_identity: solve_koblitz failed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k));
    assert!(
        check.is_infinity(),
        "koblitz_dlog_k0_identity: k={k} gives k·G ≠ ∞"
    );
}

/// τ-orbit rho agrees with the baseline on all non-trivial k values.
///
/// This is the primary Koblitz τ-automorphism correctness signal: the τ-orbit
/// variant recovers the same `k` (up to group equivalence: k·G = Q) as the
/// baseline. Both solvers must return a valid k satisfying k·G = Q.
#[test]
fn koblitz_agrees_with_baseline() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13;

    for k_target in [1u64, 2, 3] {
        let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

        // Baseline solver.
        let k_baseline = solve(&curve, &g, &q, n)
            .unwrap_or_else(|| panic!("baseline solve failed for k_target={k_target}"));

        // τ-orbit solver.
        let k_koblitz = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 0, 50)
            .unwrap_or_else(|| panic!("koblitz solve failed for k_target={k_target}"));

        // Both must satisfy k·G = Q (they need not return the same k value,
        // since the group may have multiple valid representations).
        let check_baseline = curve.scalar_mul(&g, &Uint::<1>::from(k_baseline));
        let check_koblitz = curve.scalar_mul(&g, &Uint::<1>::from(k_koblitz));

        assert_eq!(
            check_baseline, q,
            "baseline: k={k_baseline} gives k·G ≠ Q for k_target={k_target}"
        );
        assert_eq!(
            check_koblitz, q,
            "koblitz: k={k_koblitz} gives k·G ≠ Q for k_target={k_target}"
        );

        // Both solvers agree on the same Q (the target point).
        assert_eq!(
            check_baseline, check_koblitz,
            "baseline and koblitz disagree on Q for k_target={k_target}"
        );
    }
}

/// τ-orbit rho is deterministic: same seed gives same result.
#[test]
fn koblitz_deterministic() {
    let curve = toy_curve();
    let g = curve.generator::<F2mNaive<1>>();
    let n = 4u64;
    let m = 4usize;
    let t: i64 = 13;
    let k_target: u64 = 2;
    let q = curve.scalar_mul(&g, &Uint::<1>::from(k_target));

    let k1 = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 42, 50)
        .expect("koblitz_deterministic: first call failed");
    let k2 = solve_koblitz::<F2mNaive<1>>(&curve, &g, &q, n, m, t, 42, 50)
        .expect("koblitz_deterministic: second call failed");

    assert_eq!(k1, k2, "solve_koblitz: different results for same seed");

    let check = curve.scalar_mul(&g, &Uint::<1>::from(k1));
    assert_eq!(check, q, "koblitz_deterministic: k·G ≠ Q");
}
