//! Known-answer tests (KATs) for base-m polynomial selection.
//!
//! Three KATs:
//!
//! 1. **Base-m round-trip** (toy N): for `N = 1009 * 1013`, `d = 3`, the generated `f`
//!    satisfies `f(m) == N` exactly and `g(m) == 0`, and `PolyPair::verify()` returns `Ok(())`.
//!
//! 2. **RSA-100 base-m (deterministic)**: base-m expansion of RSA-100 at `d = 5` is
//!    deterministic — the same `(N, m, d)` always yields the same `f`. Verified by running
//!    twice and checking equality, and by confirming `f(m) == N` and `verify()` holds.
//!
//! 3. **optimal_degree**: returns 3 or 4 for toy N (60–100 bit) and 5 for RSA-100 (330 bit).

use gnfs::{optimal_degree, select_base_m, select_base_m_with_m};
use num_bigint::BigInt;
use std::str::FromStr;

// RSA-100 = 1522605027922533360535618378132637429718068114961380688657908494580122963258952897654000350692006139
// This is a 330-bit number (100 decimal digits).
const RSA_100_STR: &str =
    "1522605027922533360535618378132637429718068114961380688657908494580122963258952897654000350692006139";

// ─── KAT 1: Base-m round-trip (toy N) ───────────────────────────────────────

/// KAT 1: Base-m round-trip for toy N = 1009 * 1013 = 1022117.
///
/// Verifies:
/// - `f(m) == N` exactly (base-m is exact by construction).
/// - `g(m) == 0`.
/// - `PolyPair::verify()` returns `Ok(())`.
/// - The polynomial has the expected degree 3.
#[test]
fn kat1_toy_base_m_round_trip() {
    let n = BigInt::from(1009u64 * 1013u64); // = 1022117
    let degree = 3;

    let pair = select_base_m(&n, degree);

    // f(m) must equal N exactly (not just mod N — base-m gives f(m) = N).
    assert_eq!(
        pair.f.eval(&pair.m),
        n,
        "f(m) should equal N exactly for base-m expansion"
    );

    // g(m) = m - m = 0.
    assert_eq!(
        pair.g.eval(&pair.m),
        BigInt::from(0i32),
        "g(m) should be zero"
    );

    // All invariants hold.
    pair.verify().expect("PolyPair::verify() should return Ok(()) for a valid base-m pair");

    // Degree is as requested.
    assert_eq!(pair.degree, degree, "degree field should match requested degree");
    assert_eq!(
        pair.f.degree(),
        Some(degree),
        "f.degree() should match requested degree"
    );

    // Skew and factor_base_bounds are None at construction.
    assert!(pair.skew.is_none(), "skew should be None at construction");
    assert!(pair.factor_base_bounds.is_none(), "factor_base_bounds should be None at construction");
}

// ─── KAT 2: RSA-100 base-m (deterministic) ──────────────────────────────────

/// KAT 2: RSA-100 base-m expansion at degree 5 is deterministic.
///
/// The PLAN specifies: "compute m = floor(N^(1/6)) and verify that f(m) == N and verify()
/// holds — the determinism check is that the same (N, m, d) always gives the same f."
///
/// This test:
/// - Runs `select_base_m` twice and checks the results are identical.
/// - Verifies `f(m) == N` exactly.
/// - Verifies `PolyPair::verify()` returns `Ok(())`.
/// - Verifies the polynomial has degree 5.
///
/// Additionally tests `select_base_m_with_m` with the same `m` to confirm the explicit-m
/// variant produces the same result.
#[test]
fn kat2_rsa100_base_m_deterministic() {
    let n = BigInt::from_str(RSA_100_STR).expect("RSA-100 should parse");
    let degree = 5;

    // Run twice — must be identical (deterministic).
    let pair1 = select_base_m(&n, degree);
    let pair2 = select_base_m(&n, degree);

    assert_eq!(
        pair1.f.coeffs, pair2.f.coeffs,
        "base-m expansion must be deterministic: same (N, d) → same f"
    );
    assert_eq!(pair1.m, pair2.m, "base-m expansion must be deterministic: same (N, d) → same m");

    // f(m) == N exactly.
    assert_eq!(
        pair1.f.eval(&pair1.m),
        n,
        "f(m) should equal N exactly for RSA-100 base-m expansion"
    );

    // g(m) == 0.
    assert_eq!(
        pair1.g.eval(&pair1.m),
        BigInt::from(0i32),
        "g(m) should be zero"
    );

    // All invariants hold.
    pair1.verify().expect("PolyPair::verify() should return Ok(()) for RSA-100 base-m pair");

    // Degree is 5.
    assert_eq!(pair1.degree, degree);
    assert_eq!(pair1.f.degree(), Some(degree));

    // select_base_m_with_m with the same m gives the same result.
    let pair3 = select_base_m_with_m(&n, &pair1.m, degree);
    assert_eq!(
        pair1.f.coeffs, pair3.f.coeffs,
        "select_base_m_with_m with the same m should give the same f"
    );
    pair3.verify().expect("select_base_m_with_m result should verify");
}

// ─── KAT 3: optimal_degree ──────────────────────────────────────────────────

/// KAT 3: `optimal_degree` returns 3–4 for toy N and 5 for RSA-100.
///
/// The heuristic `d ≈ (3 ln N / ln ln N)^{1/3}` clamped to [3, 6]:
/// - For 60-bit N: d ≈ 3.
/// - For 100-bit N: d ≈ 3–4.
/// - For RSA-100 (330-bit): d ≈ 5.
#[test]
fn kat3_optimal_degree() {
    // Toy N: 1009 * 1013 ≈ 2^20 (20-bit) — clamp floor gives 3.
    let n_toy = BigInt::from(1009u64 * 1013u64);
    let d_toy = optimal_degree(&n_toy);
    assert!(
        d_toy == 3 || d_toy == 4,
        "optimal_degree for toy N (~20 bit) should be 3 or 4, got {d_toy}"
    );

    // 60-bit N.
    let n_60 = BigInt::from(1u64) << 60;
    let d_60 = optimal_degree(&n_60);
    assert!(
        d_60 == 3 || d_60 == 4,
        "optimal_degree for 60-bit N should be 3 or 4, got {d_60}"
    );

    // 100-bit N.
    let n_100 = BigInt::from(1u64) << 100;
    let d_100 = optimal_degree(&n_100);
    assert!(
        d_100 == 3 || d_100 == 4,
        "optimal_degree for 100-bit N should be 3 or 4, got {d_100}"
    );

    // RSA-100 (330-bit, 100 decimal digits).
    let n_rsa100 = BigInt::from_str(RSA_100_STR).expect("RSA-100 should parse");
    let d_rsa100 = optimal_degree(&n_rsa100);
    assert_eq!(
        d_rsa100, 5,
        "optimal_degree for RSA-100 (330-bit) should be 5, got {d_rsa100}"
    );
}

// ─── Additional: monic_f and number_field ────────────────────────────────────

/// Verify that `monic_f()` produces a monic polynomial and `number_field()` constructs
/// a valid `NumberField` from the base-m polynomial pair.
#[test]
fn monic_f_and_number_field_from_base_m() {
    let n = BigInt::from(1009u64 * 1013u64);
    let pair = select_base_m(&n, 3);

    let mf = pair.monic_f();
    assert!(mf.is_monic(), "monic_f() should return a monic polynomial");
    assert_eq!(mf.degree(), Some(3), "monic_f() should have the same degree as f");

    // number_field() should not panic (it calls NumberField::new which requires monic f).
    let _kf = pair.number_field();
}
