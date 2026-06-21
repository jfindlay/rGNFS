//! Known-answer tests for Shor's ECDLP algorithm (S.C.2 ◆).
//!
//! # What is tested
//!
//! 1. **Order/group-law KATs** — classical verification that the toy curve has the expected
//!    structure: `r·G = ∞`, `k·G = Q` for the chosen discrete-log instances.
//!
//! 2. **2D-lattice extraction KATs** — a known measured pair `(a', b')` recovers the correct
//!    `k` via `b'·k ≡ −a' (mod r)`. Deterministic classical KAT, no quantum circuit needed.
//!
//! 3. **End-to-end ECDLP** — `solve_ecdlp(G, Q=k·G)` recovers `k` for a fixture of `k` values
//!    on the toy curve, with a fixed measurement seed so the run is reproducible. The assertion
//!    checks `recovered_k · G == Q` (relationship-preservation).
//!
//! 4. **rho cross-check** (`#[ignore]`-gated) — `rho::ecdlp::solve_brent` on the equivalent
//!    `rho::Curve` returns a `k'` satisfying `k'·G = Q` for the same instances.
//!
//! # Toy curve parameters (C-PointAdd freeze)
//!
//! ```text
//! p = 7,  a = 0,  b = 3,  y² = x³ + 3 mod 7
//! G = (1, 2),  r = 13 (prime group order)
//! ```
//!
//! Scalar-multiple table (from `shor::curve` module documentation):
//!
//! ```text
//! 0·G = ∞,      1·G = (1,2),  2·G = (6,3),  3·G = (2,2),
//! 4·G = (4,5),  5·G = (3,3),  6·G = (5,3),  7·G = (5,4),
//! 8·G = (3,4),  9·G = (4,2),  10·G = (2,5), 11·G = (6,4),
//! 12·G = (1,5), 13·G = ∞
//! ```
//!
//! # Qubit budget
//!
//! The two-register ECDLP circuit uses 17 qubits:
//! - 4 (a-register) + 4 (b-register) + 3 (x) + 3 (y) + 3 (λ scratch) = 17
//! - Well within the ~25-qubit simulator ceiling (principle 4).

use shor::curve::{self, G, Point, R};
use shor::ecdlp::{
    extract_k_from_measurement, run_period_finding_circuit, solve_ecdlp, EXP_BITS, TOTAL_QUBITS,
};

// ── KAT group 1: order/group-law KATs (classical verification) ────────────────
//
// These KATs verify that the toy curve has the expected structure.
// They are purely classical — no quantum circuit is run.

/// r·G = ∞: the group order is 13.
#[test]
fn order_r_times_g_is_infinity() {
    let result = curve::scalar_mul(R, G);
    assert!(result.is_infinity(), "r·G should be ∞ (group order r={R})");
}

/// (r+1)·G = G: scalar multiplication wraps correctly.
#[test]
fn order_r_plus_1_times_g_is_g() {
    let result = curve::scalar_mul(R + 1, G);
    assert_eq!(result, G, "(r+1)·G should equal G");
}

/// k·G = Q for the known scalar-multiple table.
///
/// Verifies the discrete-log instances used in the end-to-end KATs are well-formed.
#[test]
fn group_law_scalar_multiples() {
    // Published scalar-multiple table from the curve module documentation.
    let expected: &[(u64, Point)] = &[
        (0, Point::Infinity),
        (1, Point::Affine { x: 1, y: 2 }),
        (2, Point::Affine { x: 6, y: 3 }),
        (3, Point::Affine { x: 2, y: 2 }),
        (4, Point::Affine { x: 4, y: 5 }),
        (5, Point::Affine { x: 3, y: 3 }),
        (6, Point::Affine { x: 5, y: 3 }),
        (7, Point::Affine { x: 5, y: 4 }),
        (8, Point::Affine { x: 3, y: 4 }),
        (9, Point::Affine { x: 4, y: 2 }),
        (10, Point::Affine { x: 2, y: 5 }),
        (11, Point::Affine { x: 6, y: 4 }),
        (12, Point::Affine { x: 1, y: 5 }),
        (13, Point::Infinity),
    ];
    for &(k, expected_pt) in expected {
        let got = curve::scalar_mul(k, G);
        assert_eq!(
            got, expected_pt,
            "{k}·G = {got:?}, expected {expected_pt:?}"
        );
    }
}

/// k·G = Q for the specific instances used in the end-to-end KATs.
///
/// Verifies that the discrete-log instances are well-formed before running the quantum circuit.
#[test]
fn ecdlp_instances_well_formed() {
    // The end-to-end KATs use k = 3, 5, 7, 9.
    for k in [3u64, 5, 7, 9] {
        let q = curve::scalar_mul(k, G);
        assert!(!q.is_infinity(), "k={k}: k·G should not be ∞ (k < r={R})");
        assert!(curve::on_curve(q), "k={k}: k·G should be on the curve");
        // Verify the inverse: scalar_mul(k, G) = Q, and Q is not G (k ≠ 1).
        assert_ne!(q, G, "k={k}: k·G should not equal G (k ≠ 1)");
    }
}

/// Qubit budget: 17 qubits, within the ~25-qubit ceiling.
#[test]
fn qubit_budget_within_ceiling() {
    assert_eq!(EXP_BITS, 4, "EXP_BITS should be 4 (⌈log₂ 13⌉ = 4)");
    assert_eq!(TOTAL_QUBITS, 17, "TOTAL_QUBITS should be 17");
    assert!(
        TOTAL_QUBITS <= 25,
        "TOTAL_QUBITS={TOTAL_QUBITS} exceeds the ~25-qubit ceiling"
    );
}

// ── KAT group 2: 2D-lattice extraction KATs (classical, deterministic) ────────
//
// Given a known measured pair (a', b'), verify that extract_k_from_measurement recovers
// the correct k via b'·k ≡ −a' (mod r). These are purely classical KATs.

/// extract_k: b'=0 → None (uninformative measurement).
#[test]
fn lattice_extract_b_zero_returns_none() {
    // b' = 0 mod r: the measurement is uninformative.
    let result = extract_k_from_measurement(5, 0, R);
    assert_eq!(result, None, "b'=0 should return None (uninformative)");
}

/// extract_k: b'=0 mod r → None even if b' ≠ 0 as a raw value.
#[test]
fn lattice_extract_b_zero_mod_r_returns_none() {
    // b' = r = 13: b' mod r = 0, so uninformative.
    let result = extract_k_from_measurement(5, R, R);
    assert_eq!(result, None, "b'=r should return None (b' mod r = 0)");
}

/// extract_k: known (a', b') pairs recover the correct k.
///
/// For k=3: we need b'·3 ≡ −a' (mod 13). Choose b'=1, then a' = -3 mod 13 = 10.
/// Check: 1·3 ≡ 3 ≡ −10 (mod 13). ✓
#[test]
fn lattice_extract_k3_from_known_pair() {
    // b'=1, a'=10: b'·k ≡ −a' (mod 13) → 1·k ≡ −10 ≡ 3 (mod 13) → k=3.
    let k = extract_k_from_measurement(10, 1, R);
    assert_eq!(k, Some(3), "extract_k(a'=10, b'=1, r=13) should be Some(3)");
    // Verify: k·G = 3·G = (2,2).
    if let Some(k_val) = k {
        let q = curve::scalar_mul(k_val, G);
        let expected_q = curve::scalar_mul(3, G);
        assert_eq!(q, expected_q, "recovered k={k_val}: k·G should equal 3·G");
    }
}

/// extract_k: known (a', b') pairs recover the correct k for k=7.
///
/// For k=7: choose b'=2, then a' = -2·7 mod 13 = -14 mod 13 = -1 mod 13 = 12.
/// Check: 2·7 = 14 ≡ 1 ≡ −12 (mod 13). ✓
#[test]
fn lattice_extract_k7_from_known_pair() {
    // b'=2, a'=12: b'·k ≡ −a' (mod 13) → 2·k ≡ −12 ≡ 1 (mod 13) → k = 1·(2^{-1}) mod 13.
    // 2^{-1} mod 13 = 7 (since 2·7 = 14 ≡ 1 mod 13). So k = 7.
    let k = extract_k_from_measurement(12, 2, R);
    assert_eq!(k, Some(7), "extract_k(a'=12, b'=2, r=13) should be Some(7)");
    if let Some(k_val) = k {
        let q = curve::scalar_mul(k_val, G);
        let expected_q = curve::scalar_mul(7, G);
        assert_eq!(q, expected_q, "recovered k={k_val}: k·G should equal 7·G");
    }
}

/// extract_k: a'=0, b'=1 → k=0.
///
/// b'·k ≡ −a' (mod r) → 1·k ≡ 0 (mod 13) → k=0.
/// (k=0 corresponds to Q=∞, which is handled as a trivial case in solve_ecdlp.)
#[test]
fn lattice_extract_k0_from_zero_a() {
    let k = extract_k_from_measurement(0, 1, R);
    assert_eq!(k, Some(0), "extract_k(a'=0, b'=1, r=13) should be Some(0)");
}

/// extract_k: all non-zero b' values recover a valid k in [0, r).
///
/// For each b' in 1..r, extract_k should return Some(k) with k in [0, r).
#[test]
fn lattice_extract_all_b_nonzero() {
    for b_prime in 1..R {
        for a_prime in 0..R {
            let result = extract_k_from_measurement(a_prime, b_prime, R);
            assert!(result.is_some(), "b'={b_prime}, a'={a_prime}: should return Some(k)");
            let k = result.unwrap();
            assert!(k < R, "b'={b_prime}, a'={a_prime}: k={k} should be < r={R}");
            // Verify the relation: b'·k ≡ -a' (mod r).
            let lhs = (b_prime * k) % R;
            let rhs = if a_prime == 0 { 0 } else { R - (a_prime % R) };
            assert_eq!(
                lhs, rhs,
                "b'={b_prime}, a'={a_prime}, k={k}: b'·k={lhs} ≢ -a'={rhs} (mod {R})"
            );
        }
    }
}

// ── KAT group 3: end-to-end ECDLP (quantum circuit + classical post-processing) ──
//
// These KATs run the full Shor ECDLP algorithm end-to-end with fixed seeds.
// Seeds are chosen so that the circuit measurement yields an informative (a', b') pair
// that recovers the correct k on the first or early retry.
//
// The assertion checks recovered_k · G == Q (relationship-preservation), not recovered_k == k,
// since the quantum algorithm may return any k' satisfying k'·G = Q (which equals k mod r).

/// Helper: assert that solve_ecdlp(G, Q=k·G, seed) recovers a k' with k'·G = Q.
fn assert_solve_ecdlp(k: u64, seed: u64, label: &str) {
    let q = curve::scalar_mul(k, G);
    let result = solve_ecdlp(G, q, seed);
    assert!(
        result.is_some(),
        "{label}: solve_ecdlp(G, {k}·G, seed={seed}) returned None"
    );
    let recovered_k = result.unwrap();
    let kg = curve::scalar_mul(recovered_k, G);
    assert_eq!(
        kg, q,
        "{label}: recovered_k={recovered_k}, but recovered_k·G={kg:?} ≠ Q={q:?}"
    );
}

/// solve_ecdlp: trivial case Q=∞ → k=0.
#[test]
fn ecdlp_trivial_q_infinity() {
    let result = solve_ecdlp(G, Point::Infinity, 0);
    assert_eq!(result, Some(0), "solve_ecdlp(G, ∞, 0) should return Some(0)");
}

/// solve_ecdlp: trivial case Q=G → k=1.
#[test]
fn ecdlp_trivial_q_equals_g() {
    let result = solve_ecdlp(G, G, 0);
    assert_eq!(result, Some(1), "solve_ecdlp(G, G, 0) should return Some(1)");
}

/// solve_ecdlp: k=3, Q=3·G=(2,2). Seed 0.
///
/// Verifies that the two-register circuit recovers k=3 (or any k' with k'·G = 3·G).
#[test]
fn ecdlp_k3_seed0() {
    assert_solve_ecdlp(3, 0, "ecdlp_k3_seed0");
}

/// solve_ecdlp: k=5, Q=5·G=(3,3). Seed 0.
#[test]
fn ecdlp_k5_seed0() {
    assert_solve_ecdlp(5, 0, "ecdlp_k5_seed0");
}

/// solve_ecdlp: k=7, Q=7·G=(5,4). Seed 0.
#[test]
fn ecdlp_k7_seed0() {
    assert_solve_ecdlp(7, 0, "ecdlp_k7_seed0");
}

/// solve_ecdlp: k=9, Q=9·G=(4,2). Seed 0.
#[test]
fn ecdlp_k9_seed0() {
    assert_solve_ecdlp(9, 0, "ecdlp_k9_seed0");
}

/// solve_ecdlp: k=12, Q=12·G=(1,5). Seed 0.
///
/// k=12 is the largest non-trivial scalar (r-1=12), testing the boundary.
#[test]
fn ecdlp_k12_seed0() {
    assert_solve_ecdlp(12, 0, "ecdlp_k12_seed0");
}

/// solve_ecdlp: all non-trivial k values (2..=12) recover k'·G = Q.
///
/// Runs solve_ecdlp for every non-trivial k on the toy curve with seed=0.
/// Verifies relationship-preservation: recovered_k · G == Q.
///
/// Marked `#[ignore]` because it runs 11 full quantum circuit instances (~165s in debug
/// mode). The targeted fixture tests (k=3, k=5, k=7, k=9, k=12) cover the same ground.
#[test]
#[ignore]
fn ecdlp_all_k_values_seed0() {
    for k in 2..R {
        assert_solve_ecdlp(k, 0, &format!("ecdlp_k{k}_seed0"));
    }
}

/// solve_ecdlp is reproducible: same seed → same result.
///
/// Marked `#[ignore]` because it runs 6 full quantum circuit instances (~90s in debug mode).
/// Reproducibility is verified by the deterministic seeded RNG in `measure_all_seeded`.
#[test]
#[ignore]
fn ecdlp_reproducible() {
    for k in [3u64, 7, 9] {
        let q = curve::scalar_mul(k, G);
        let r1 = solve_ecdlp(G, q, 0);
        let r2 = solve_ecdlp(G, q, 0);
        assert_eq!(r1, r2, "solve_ecdlp(G, {k}·G, seed=0) is not reproducible");
    }
}

/// run_period_finding_circuit: the measured (a', b') satisfies b'·k ≡ −a' (mod r) for
/// informative measurements (b' ≠ 0 mod r).
///
/// This test runs the circuit for k=3 with seed=0 and verifies the lattice relation holds
/// for the measured pair (if informative).
#[test]
fn period_finding_lattice_relation() {
    let k = 3u64;
    let q = curve::scalar_mul(k, G);
    let (a_prime, b_prime) = run_period_finding_circuit(G, q, 0);
    // If b' ≠ 0 mod r, verify the lattice relation.
    if b_prime % R != 0 {
        let lhs = (b_prime % R * k) % R;
        let rhs = if a_prime % R == 0 { 0 } else { R - (a_prime % R) };
        assert_eq!(
            lhs, rhs,
            "Lattice relation failed: b'·k={lhs} ≢ -a'={rhs} (mod {R}), \
             a'={a_prime}, b'={b_prime}, k={k}"
        );
    }
    // If b' = 0 mod r, the measurement is uninformative — no assertion needed.
}

// ── KAT group 4: rho cross-check (#[ignore]-gated) ────────────────────────────
//
// These KATs use rho::ecdlp::solve_brent as an oracle to cross-check the Shor solver.
// They are #[ignore]-gated so they do not run in the normal test suite (the rho solver
// is a dev-dependency, and the cross-check is not part of the green path).
//
// The rho::Curve struct has fields: p, a, b, n, gx, gy (all Uint<4>).
// The toy curve parameters: p=7, a=0, b=3, n=13, gx=1, gy=2.

/// rho cross-check: solve_brent recovers k'·G = Q for k=3 on the toy curve.
///
/// Builds a rho::Curve from the toy curve parameters and calls solve_brent.
/// The returned k' must satisfy k'·G = Q (relationship-preservation).
///
/// This test is #[ignore]-gated: it requires the rho dev-dependency and is not
/// part of the green path (the Shor solver is oracle-free).
#[test]
#[ignore]
fn rho_crosscheck_k3() {
    use crypto_bigint::Uint;
    use rho::curve::{AffinePoint, Curve};
    use rho::ecdlp::solve_brent;
    use shared_field::FpMonty4 as FpMonty;

    // Build the rho::Curve from the toy curve parameters.
    let toy_curve = Curve {
        p: Uint::<4>::from(7u64),
        a: Uint::<4>::from(0u64),
        b: Uint::<4>::from(3u64),
        n: Uint::<4>::from(13u64),
        gx: Uint::<4>::from(1u64),
        gy: Uint::<4>::from(2u64),
    };
    let g_rho: AffinePoint<FpMonty> = toy_curve.generator();

    let k_target = 3u64;
    let q_rho = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k_target));

    let k = solve_brent(&toy_curve, &g_rho, &q_rho, 13, 0, 20)
        .expect("rho::solve_brent failed for k=3 on toy curve");

    // Verify relationship-preservation: k'·G = Q.
    let check = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k));
    assert_eq!(check, q_rho, "rho cross-check: k'·G ≠ Q (k'={k}, expected k={k_target})");
}

/// rho cross-check: solve_brent recovers k'·G = Q for k=7 on the toy curve.
#[test]
#[ignore]
fn rho_crosscheck_k7() {
    use crypto_bigint::Uint;
    use rho::curve::{AffinePoint, Curve};
    use rho::ecdlp::solve_brent;
    use shared_field::FpMonty4 as FpMonty;

    let toy_curve = Curve {
        p: Uint::<4>::from(7u64),
        a: Uint::<4>::from(0u64),
        b: Uint::<4>::from(3u64),
        n: Uint::<4>::from(13u64),
        gx: Uint::<4>::from(1u64),
        gy: Uint::<4>::from(2u64),
    };
    let g_rho: AffinePoint<FpMonty> = toy_curve.generator();

    let k_target = 7u64;
    let q_rho = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k_target));

    let k = solve_brent(&toy_curve, &g_rho, &q_rho, 13, 0, 20)
        .expect("rho::solve_brent failed for k=7 on toy curve");

    let check = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k));
    assert_eq!(check, q_rho, "rho cross-check: k'·G ≠ Q (k'={k}, expected k={k_target})");
}

/// rho cross-check: solve_brent and solve_ecdlp agree on k'·G = Q for k=5.
///
/// Both solvers should return a k' satisfying k'·G = Q = 5·G.
/// The specific k' values may differ (any valid discrete log is acceptable).
#[test]
#[ignore]
fn rho_crosscheck_agrees_with_shor_k5() {
    use crypto_bigint::Uint;
    use rho::curve::{AffinePoint, Curve};
    use rho::ecdlp::solve_brent;
    use shared_field::FpMonty4 as FpMonty;

    let toy_curve = Curve {
        p: Uint::<4>::from(7u64),
        a: Uint::<4>::from(0u64),
        b: Uint::<4>::from(3u64),
        n: Uint::<4>::from(13u64),
        gx: Uint::<4>::from(1u64),
        gy: Uint::<4>::from(2u64),
    };
    let g_rho: AffinePoint<FpMonty> = toy_curve.generator();

    let k_target = 5u64;
    let q_rho = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k_target));

    // rho solver.
    let k_rho = solve_brent(&toy_curve, &g_rho, &q_rho, 13, 0, 20)
        .expect("rho::solve_brent failed for k=5");
    let check_rho = toy_curve.scalar_mul(&g_rho, &Uint::<4>::from(k_rho));
    assert_eq!(check_rho, q_rho, "rho: k'·G ≠ Q (k'={k_rho})");

    // Shor solver.
    let q_shor = curve::scalar_mul(k_target, G);
    let k_shor = solve_ecdlp(G, q_shor, 0).expect("solve_ecdlp failed for k=5");
    let check_shor = curve::scalar_mul(k_shor, G);
    assert_eq!(check_shor, q_shor, "shor: k'·G ≠ Q (k'={k_shor})");

    // Both agree on the point Q (relationship-preservation).
    // (The specific k' values may differ since any valid discrete log is acceptable.)
    eprintln!("rho k'={k_rho}, shor k'={k_shor} — both satisfy k'·G = Q = {k_target}·G");
}
