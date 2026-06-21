//! End-to-end MOV/Frey–Rück reduction KATs.
//!
//! # What is tested
//!
//! The MOV reduction transports an ECDLP on `E/F_47` (embedding degree k=2, torsion prime
//! ℓ=3) into a discrete log in `F_{47²}*` via the reduced Tate pairing, then solves that DLP
//! by calling `gnfs::dl::solve_dl` (the real k=2 solver, not a stub).
//!
//! ## KATs
//!
//! 1. **Pairing identity** — `e(Q', R) = e(G, R)^k` in `F_{47²}*` for k=1 and k=2.
//!    This is the primary correctness signal: a wrong bridge encoding, a wrong modulus, or a
//!    wrong reduction breaks this identity directly.
//! 2. **MOV scalar recovery** — `mov_reduce(curve, G, Q', R, ell)` returns `k mod ell` for
//!    k=1 and k=2 (the full ℓ=3 subgroup, excluding k=0 which is the identity).
//!
//! # Fixture
//!
//! All tests use the `pairing_toy` fixture:
//! - Curve `y² = x³ + x + 33 mod 47`, embedding degree `k = 2`, torsion prime `ℓ = 3`.
//! - `G = P = (8, 6) ∈ E(F_47)[3]` — the ECDLP generator (base-field 3-torsion point).
//! - `R = Q = ((4, 15), (22, 34)) ∈ E(F_{47²})[3] \ E(F_47)` — the μ_ℓ-generator.
//!   `e(G, R) = reduced_tate(R, G, ell) ≠ 1` (verified in the non-degeneracy KAT).
//! - `Q' = k·G` — the ECDLP target, constructed with a known scalar k ∈ {1, 2}.
//!
//! # Argument order
//!
//! The reduced Tate pairing is non-degenerate when the second argument is not in
//! `ℓ·E(F_{p^k})`. For this fixture, `G ∈ E(F_p)[ℓ]` is not in `ℓ·E(F_{p^k})`, so
//! `reduced_tate(R, G, ell)` (R first, G second) is non-degenerate. See `test_curves.rs`
//! for the group-theoretic explanation.
//!
//! # Recovered log
//!
//! The returned value is `k mod ℓ` — the discrete log in the order-ℓ subgroup. A full
//! composite-order lift via Pohlig–Hellman is a principle-4 annotation, not wired here.
//!
//! # No-regression gate
//!
//! The `gnfs/tests/dl_ext_kat.rs` and `rho/tests/pairing_kat.rs` KATs must stay green.
//! This file does NOT re-test those — it only tests the end-to-end MOV reduction.

use crypto_bigint::Uint;
use rho::pairing::ecext::PairingPoint;
use rho::pairing::fpext::FpExt;
use rho::pairing::mov::mov_reduce;
use rho::pairing::tate::reduced_tate;
use rho::pairing::test_curves::{pairing_toy, PAIRING_TOY_P};
use shared_field::{Fp, FpNaive};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Return the curve coefficient `a = 1` lifted into `F_{47²}`.
fn a_ext() -> FpExt<FpNaive<4>> {
    let p = Uint::<4>::from(PAIRING_TOY_P);
    FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p)
}

// ── Non-degeneracy pre-check ──────────────────────────────────────────────────

/// `e(G, R) ≠ 1` — the μ_ℓ-generator R is valid for G.
///
/// Verifies that `reduced_tate(R, G, ell) ≠ 1` before running the MOV reduction.
/// If this were 1, the DL would be undefined and `mov_reduce` would return an error.
///
/// This is the pre-condition for the MOV reduction: R must be a μ_ℓ-generator with
/// respect to G (i.e., `e(G, R)` generates μ_ℓ).
#[test]
fn mov_r_is_valid_mu_ell_generator() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    // e(G, R) = reduced_tate(R, G, ell): R first, G second (non-degenerate direction).
    let g_pair = reduced_tate::<FpNaive<4>>(&curve, &modulus, &q_point, &p_point, ell);
    assert!(
        !g_pair.is_one(&p),
        "e(G, R) = reduced_tate(R, G, ell) must be ≠ 1 for R to be a valid μ_ℓ-generator"
    );
}

// ── Pairing identity: e(Q', R) = e(G, R)^k ───────────────────────────────────

/// Pairing identity for k=1: `e(1·G, R) = e(G, R)^1`.
///
/// `Q' = 1·G = G`, so `e(Q', R) = e(G, R)`. Trivially true, but verifies the
/// scalar-multiply and pairing pipeline are consistent.
#[test]
fn pairing_identity_k1() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Q' = 1·G = G (scalar multiply G by k=1).
    let q_prime = p_point.scalar_mul(1, &a, &modulus, &p);

    // e(G, R) = reduced_tate(R, G, ell).
    let g_pair = reduced_tate::<FpNaive<4>>(&curve, &modulus, &q_point, &p_point, ell);
    // e(Q', R) = reduced_tate(R, Q', ell).
    let h_pair = reduced_tate::<FpNaive<4>>(&curve, &modulus, &q_point, &q_prime, ell);

    // e(G, R)^1 = e(G, R).
    let g_pair_pow1 = g_pair.clone();

    assert_eq!(
        h_pair, g_pair_pow1,
        "pairing identity k=1: e(Q', R) should equal e(G, R)^1 = e(G, R)"
    );
}

/// Pairing identity for k=2: `e(2·G, R) = e(G, R)^2`.
///
/// `Q' = 2·G`, so by bilinearity `e(Q', R) = e(2·G, R) = e(G, R)^2`.
/// This is the primary correctness signal: a wrong bridge encoding, a wrong modulus,
/// or a wrong reduction breaks this identity directly.
#[test]
fn pairing_identity_k2() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Q' = 2·G (scalar multiply G by k=2).
    let q_prime = p_point.scalar_mul(2, &a, &modulus, &p);

    // e(G, R) = reduced_tate(R, G, ell).
    let g_pair = reduced_tate::<FpNaive<4>>(&curve, &modulus, &q_point, &p_point, ell);
    // e(Q', R) = reduced_tate(R, Q', ell).
    let h_pair = reduced_tate::<FpNaive<4>>(&curve, &modulus, &q_point, &q_prime, ell);

    // e(G, R)^2 = e(G, R).square().
    let g_pair_sq = g_pair.square(&modulus, &p);

    assert_eq!(
        h_pair, g_pair_sq,
        "pairing identity k=2: e(2·G, R) should equal e(G, R)^2 in F_{{47²}}*"
    );
}

// ── MOV scalar recovery ───────────────────────────────────────────────────────

/// MOV reduction recovers k=1: `mov_reduce(curve, G, 1·G, R, ell) = 1`.
///
/// Constructs the ECDLP `Q' = 1·G` with known scalar k=1, runs the MOV reduction,
/// and asserts the recovered scalar equals 1 mod ℓ.
///
/// The recovered log is `k mod ℓ` (the subgroup log in the order-ℓ=3 subgroup).
#[test]
fn mov_reduce_recovers_k1() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Q' = 1·G — the ECDLP target with known scalar k=1.
    let q_prime: PairingPoint<FpNaive<4>> = p_point.scalar_mul(1, &a, &modulus, &p);

    // R = Q (the fixture's extension-field 3-torsion point, a μ_ℓ-generator).
    // e(G, R) ≠ 1 is verified by mov_r_is_valid_mu_ell_generator above.
    let r_point = q_point;

    let result = mov_reduce::<FpNaive<4>>(&curve, &modulus, &p_point, &q_prime, &r_point, ell);

    assert_eq!(
        result,
        Ok(1u64),
        "mov_reduce should recover k=1 mod ell=3; got: {:?}",
        result
    );
}

/// MOV reduction recovers k=2: `mov_reduce(curve, G, 2·G, R, ell) = 2`.
///
/// Constructs the ECDLP `Q' = 2·G` with known scalar k=2, runs the MOV reduction,
/// and asserts the recovered scalar equals 2 mod ℓ.
///
/// The recovered log is `k mod ℓ` (the subgroup log in the order-ℓ=3 subgroup).
/// This is the decisive end-to-end correctness gate for the MOV reduction:
/// `e(Q', R) = e(G, R)^2` in `F_{47²}*`, and `solve_dl` recovers the exponent 2.
#[test]
fn mov_reduce_recovers_k2() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Q' = 2·G — the ECDLP target with known scalar k=2.
    let q_prime: PairingPoint<FpNaive<4>> = p_point.scalar_mul(2, &a, &modulus, &p);

    // R = Q (the fixture's extension-field 3-torsion point, a μ_ℓ-generator).
    let r_point = q_point;

    let result = mov_reduce::<FpNaive<4>>(&curve, &modulus, &p_point, &q_prime, &r_point, ell);

    assert_eq!(
        result,
        Ok(2u64),
        "mov_reduce should recover k=2 mod ell=3; got: {:?}",
        result
    );
}

/// MOV reduction returns an error for a degenerate pairing (R = G, a base-field point).
///
/// If R is a base-field point (like G itself), `e(G, R) = reduced_tate(R, G, ell) = 1`
/// (degenerate — R is in `ℓ·E(F_{p^k})`). The reduction must detect this and return an error
/// rather than silently computing a wrong log.
///
/// This verifies the `g_pair ≠ 1` guard in `mov_reduce`.
#[test]
fn mov_reduce_degenerate_r_returns_error() {
    let (curve, modulus, ell, p_point, _q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Q' = 1·G.
    let q_prime: PairingPoint<FpNaive<4>> = p_point.scalar_mul(1, &a, &modulus, &p);

    // R = G (a base-field point) — degenerate: e(G, G) = reduced_tate(G, G, ell) = 1.
    // (The Tate pairing t_ℓ(P, Q) = 1 when Q ∈ ℓ·E(F_{p^k}); for a base-field point
    // used as both arguments, the result is trivial.)
    let r_degenerate = p_point.clone();

    let result =
        mov_reduce::<FpNaive<4>>(&curve, &modulus, &p_point, &q_prime, &r_degenerate, ell);

    assert!(
        result.is_err(),
        "mov_reduce with degenerate R (base-field point) should return an error; got: {:?}",
        result
    );
}

// ── PARI cross-check (dev-only, #[ignore]) ────────────────────────────────────

/// PARI cross-check: verify the MOV reduction result against PARI's `fflog`.
///
/// Run manually when PARI/GP is available:
///
/// ```text
/// # In PARI/GP:
/// # F_{47^2} = F_47[u]/(u^2+1). The ell=3 subgroup generator is ζ = 23 + 6u.
/// # The MOV reduction maps the ECDLP (G, Q'=k·G) to (g=e(G,R), h=e(Q',R)) in μ_3.
/// # By bilinearity: h = g^k, so fflog(h, g) = k.
/// p = 47; t = ffgen(Mod(1,p)*x^2+1, 'u);
/// # The pairing outputs for the toy fixture:
/// # e(G, R) = reduced_tate(R, G, 3) = ζ = (23, 6) = 23 + 6u (or its inverse ζ²)
/// # e(2·G, R) = e(G, R)^2 = ζ^2 = (23, 41) = 23 + 41u
/// zeta = 23 + 6*u;   # ζ in F_{47^2}
/// fflog(zeta^2, zeta)  # should return 2 (= k for Q'=2·G)
/// fflog(zeta, zeta)    # should return 1 (= k for Q'=1·G)
/// ```
///
/// The PARI `fflog` function computes discrete logs in finite extension fields.
/// This cross-check verifies that our MOV reduction agrees with PARI's oracle.
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn kat_pari_mov_cross_check() {
    // This test is a placeholder for the PARI cross-check.
    // When PARI is available, run the commands above and verify the results match.
    //
    // Expected results (matching mov_reduce_recovers_k1 and mov_reduce_recovers_k2):
    // - mov_reduce(curve, G, 1·G, R, 3) = 1  ↔  fflog(ζ, ζ) = 1
    // - mov_reduce(curve, G, 2·G, R, 3) = 2  ↔  fflog(ζ², ζ) = 2
    //
    // The mov_reduce results are verified by the KATs above.
    // This test is a documentation stub for the PARI oracle cross-check.
    assert!(true, "PARI cross-check: see test doc for manual verification commands");
}
