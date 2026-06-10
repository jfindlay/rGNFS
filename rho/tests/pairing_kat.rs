//! Known-answer tests for the Weil pairing (E.B.3).
//!
//! # What is tested
//!
//! 1. **Non-degeneracy** — `w(P, Q) ≠ 1` for the independent torsion pair `(P, Q)`.
//! 2. **Left bilinearity** — `w(aP, Q) = w(P, Q)^a` for `a = 2`.
//! 3. **Right bilinearity** — `w(P, bQ) = w(P, Q)^b` for `b = 2`.
//! 4. **Full bilinearity** — `w(aP, bQ) = w(P, Q)^{ab}` for `a = 2, b = 2`.
//! 5. **Alternation** — `w(P, P) = 1` (antisymmetry of the Weil pairing).
//!
//! # Fixture
//!
//! All tests use the `pairing_toy` fixture:
//! - Curve `y² = x³ + x + 33 mod 47`, embedding degree `k = 2`, torsion prime `ℓ = 3`.
//! - `P = (8, 6) ∈ E(F_47)[3]` (lifted to `E(F_{47^2})`).
//! - `Q = ((4, 15), (22, 34)) ∈ E(F_{47^2})[3] \ E(F_47)`.
//!
//! # Bilinearity as the primary correctness signal
//!
//! An off-by-one in the Miller loop bit iteration, a wrong line/vertical
//! function, or a wrong sign convention breaks bilinearity directly.  These
//! KATs are therefore the decisive correctness gate for the Miller + Weil
//! implementation.

use crypto_bigint::Uint;
use rho::pairing::fpext::FpExt;
use rho::pairing::test_curves::{pairing_toy, PAIRING_TOY_P};
use rho::pairing::weil::weil_pairing;
use shared_field::{Fp, FpNaive};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Return the curve coefficient `a = 1` lifted into `F_{47^2}`.
fn a_ext() -> FpExt<FpNaive<4>> {
    let p = Uint::<4>::from(PAIRING_TOY_P);
    FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p)
}

// ── Non-degeneracy ────────────────────────────────────────────────────────────

/// `w(P, Q) ≠ 1` — the Weil pairing is non-degenerate for independent P, Q.
///
/// A degenerate pairing (= 1) would mean P and Q are linearly dependent, or
/// the Miller loop has a bug that collapses the result.
#[test]
fn weil_non_degeneracy() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let w = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
    assert!(
        !w.is_one(&p),
        "weil_pairing(P, Q, ℓ) should be non-trivial (≠ 1) for independent P, Q"
    );
}

// ── Alternation ───────────────────────────────────────────────────────────────

/// `w(P, P) = 1` — the Weil pairing is alternating (antisymmetric).
///
/// Direct evaluation of `w(P, P)` is degenerate because `P` is in the support
/// of the divisor of `f_{ℓ,P}` (the tangent line at `P` vanishes at `P`).
/// We instead verify the alternation property indirectly via bilinearity:
///
/// ```text
/// w(P, P) = w(P, P+Q) / w(P, Q)
/// ```
///
/// since `w(P, P+Q) = w(P, P) · w(P, Q)` by right bilinearity.  The points
/// `P+Q` and `Q` are both different from `P` and `-P`, so the Miller loop is
/// non-degenerate.
///
/// This is equivalent to the antisymmetry property `w(P, Q) = w(Q, P)^{-1}`,
/// which implies `w(P, P) = w(P, P)^{-1}`, hence `w(P, P)^2 = 1`.  Since
/// `ℓ = 3` is an odd prime, `μ_3` contains no element of order 2, so
/// `w(P, P) = 1`.
#[test]
fn weil_alternation() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // Compute P + Q (a 3-torsion point, different from P and -P).
    let p_plus_q = p_point.add(&q_point, &a, &modulus, &p);

    // w(P, Q)
    let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
    // w(P, P+Q)
    let w_p_ppq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &p_plus_q, ell);

    // w(P, P) = w(P, P+Q) / w(P, Q)
    let w_pp = w_p_ppq.mul(&w_pq.inv(&modulus, &p), &modulus, &p);

    assert!(
        w_pp.is_one(&p),
        "weil_pairing(P, P, ℓ) should be 1 (alternation / antisymmetry); \
         computed indirectly as w(P,P+Q)/w(P,Q)"
    );
}

// ── Left bilinearity ──────────────────────────────────────────────────────────

/// `w(2P, Q) = w(P, Q)^2` — left bilinearity with `a = 2`.
///
/// This is the primary correctness signal for the Miller loop: an off-by-one
/// in the bit iteration or a wrong line function breaks this identity.
#[test]
fn weil_bilinearity_left() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // w(P, Q)
    let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
    // w(P, Q)^2
    let w_pq_sq = w_pq.square(&modulus, &p);

    // 2P via scalar_mul
    let two_p = p_point.scalar_mul(2, &a, &modulus, &p);
    // w(2P, Q)
    let w_2p_q = weil_pairing::<FpNaive<4>>(&curve, &modulus, &two_p, &q_point, ell);

    assert_eq!(
        w_2p_q, w_pq_sq,
        "left bilinearity: w(2P, Q) should equal w(P, Q)^2"
    );
}

// ── Right bilinearity ─────────────────────────────────────────────────────────

/// `w(P, 2Q) = w(P, Q)^2` — right bilinearity with `b = 2`.
#[test]
fn weil_bilinearity_right() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // w(P, Q)
    let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
    // w(P, Q)^2
    let w_pq_sq = w_pq.square(&modulus, &p);

    // 2Q via scalar_mul
    let two_q = q_point.scalar_mul(2, &a, &modulus, &p);
    // w(P, 2Q)
    let w_p_2q = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &two_q, ell);

    assert_eq!(
        w_p_2q, w_pq_sq,
        "right bilinearity: w(P, 2Q) should equal w(P, Q)^2"
    );
}

// ── Full bilinearity ──────────────────────────────────────────────────────────

/// `w(2P, 2Q) = w(P, Q)^4` — full bilinearity with `a = 2, b = 2`.
///
/// Combines left and right bilinearity: `w(aP, bQ) = w(P, Q)^{ab}`.
/// For `a = b = 2`: `w(2P, 2Q) = w(P, Q)^4`.
#[test]
fn weil_bilinearity_full() {
    let (curve, modulus, ell, p_point, q_point) = pairing_toy();
    let p = curve.p;
    let a = a_ext();

    // w(P, Q)
    let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
    // w(P, Q)^4 = (w(P, Q)^2)^2
    let w_pq_sq = w_pq.square(&modulus, &p);
    let w_pq_4 = w_pq_sq.square(&modulus, &p);

    // 2P and 2Q
    let two_p = p_point.scalar_mul(2, &a, &modulus, &p);
    let two_q = q_point.scalar_mul(2, &a, &modulus, &p);
    // w(2P, 2Q)
    let w_2p_2q = weil_pairing::<FpNaive<4>>(&curve, &modulus, &two_p, &two_q, ell);

    assert_eq!(
        w_2p_2q, w_pq_4,
        "full bilinearity: w(2P, 2Q) should equal w(P, Q)^4"
    );
}
