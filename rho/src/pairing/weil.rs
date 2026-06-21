//! Weil pairing `w_ℓ(P, Q)`.
//!
//! The Weil pairing is a bilinear, non-degenerate, alternating map
//!
//! ```text
//! w_ℓ : E[ℓ] × E[ℓ] → μ_ℓ ⊂ F_{p^k}*
//! ```
//!
//! computed as the ratio of two Miller evaluations:
//!
//! ```text
//! w_ℓ(P, Q) = (−1)^ℓ · f_{ℓ,P}(Q) / f_{ℓ,Q}(P)
//! ```
//!
//! where `f_{ℓ,P}` is the rational function computed by Miller's algorithm
//! (see [`crate::pairing::miller`]).
//!
//! # Weil vs Tate
//!
//! The **Weil pairing** uses the ratio form above and requires **two** Miller
//! evaluations.  It lands in `μ_ℓ ⊂ F_{p^k}*` directly — **no final
//! exponentiation** is needed.
//!
//! The **Tate pairing** ([`crate::pairing::tate`]) uses a single Miller
//! evaluation followed by a final exponentiation `^{(p^k − 1)/ℓ}` to land
//! in `μ_ℓ`.  The final exponentiation is the Tate-specific step; it is
//! absent here.
//!
//! # Sign convention
//!
//! The `(−1)^ℓ` factor arises from the standard normalisation of the Weil
//! pairing.  For odd `ℓ` (as in the toy fixture `ℓ = 3`), this is `−1` in
//! `F_{p^k}` (the additive inverse of 1).  For even `ℓ` it is `+1`.

use shared_field::Fp;

use crate::curve::Curve;
use crate::pairing::ecext::PairingPoint;
use crate::pairing::fpext::{FpExt, IrreducibleModulus};
use crate::pairing::miller::miller_loop;

// ── Public entry point ────────────────────────────────────────────────────────

/// Compute the Weil pairing `w_ℓ(P, Q)`.
///
/// Returns `(−1)^ℓ · f_{ℓ,P}(Q) / f_{ℓ,Q}(P)` as an element of `F_{p^k}*`.
///
/// # Parameters
///
/// - `curve` — the short-Weierstrass curve `y² = x³ + ax + b`.
/// - `modulus` — the irreducible polynomial defining `F_{p^k}`.
/// - `p_point` — the first pairing argument `P ∈ E[ℓ]`.
/// - `q_point` — the second pairing argument `Q ∈ E[ℓ]`, linearly independent
///   of `P` for a non-degenerate result.
/// - `ell` — the torsion order; must be ≥ 2.
///
/// # Panics
///
/// Panics if `ell < 2`.
pub fn weil_pairing<F: Fp<4>>(
    curve: &Curve,
    modulus: &IrreducibleModulus<F>,
    p_point: &PairingPoint<F>,
    q_point: &PairingPoint<F>,
    ell: u64,
) -> FpExt<F> {
    assert!(ell >= 2, "weil_pairing: ell must be >= 2");

    let p = &curve.p;

    // f_{ℓ,P}(Q) — Miller loop with P as the base point, evaluated at Q.
    let f_p = miller_loop(curve, modulus, p_point, q_point, ell);

    // f_{ℓ,Q}(P) — Miller loop with Q as the base point, evaluated at P.
    let f_q = miller_loop(curve, modulus, q_point, p_point, ell);

    // w_ℓ(P, Q) = (−1)^ℓ · f_{ℓ,P}(Q) / f_{ℓ,Q}(P)
    let ratio = f_p.mul(&f_q.inv(modulus, p), modulus, p);

    // Apply the (−1)^ℓ sign factor.
    // For odd ℓ: multiply by −1 (the additive inverse of 1 in F_{p^k}).
    // For even ℓ: multiply by +1 (no-op).
    if ell % 2 == 1 {
        // −1 in F_{p^k} is the negation of the multiplicative identity.
        let k = modulus.degree();
        let neg_one = FpExt::one(k, p).neg(p);
        ratio.mul(&neg_one, modulus, p)
    } else {
        ratio
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_field::FpNaive;

    use crate::pairing::test_curves::pairing_toy;

    /// `weil_pairing(P, Q, ℓ) ≠ 1` — the pairing is non-degenerate.
    #[test]
    fn weil_non_degenerate() {
        let (curve, modulus, ell, p_point, q_point) = pairing_toy();
        let p = curve.p;
        let w = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
        assert!(
            !w.is_one(&p),
            "weil_pairing(P, Q, ℓ) should be non-trivial (≠ 1) for independent P, Q"
        );
    }

    /// `w(P, P) = 1` — alternation (antisymmetry), verified indirectly.
    ///
    /// Direct evaluation of `w(P, P)` is degenerate (P is in the support of
    /// the divisor of `f_{ℓ,P}`).  We verify via bilinearity:
    /// `w(P, P) = w(P, P+Q) / w(P, Q)`.
    #[test]
    fn weil_alternation() {
        let (curve, modulus, ell, p_point, q_point) = pairing_toy();
        let p = curve.p;
        let k = modulus.degree();
        let a_ext = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), k, &p);

        let p_plus_q = p_point.add(&q_point, &a_ext, &modulus, &p);
        let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
        let w_p_ppq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &p_plus_q, ell);
        let w_pp = w_p_ppq.mul(&w_pq.inv(&modulus, &p), &modulus, &p);
        assert!(
            w_pp.is_one(&p),
            "w(P, P) should be 1 (alternation); computed as w(P,P+Q)/w(P,Q)"
        );
    }

    /// Left bilinearity: `w(2P, Q) = w(P, Q)^2`.
    #[test]
    fn weil_bilinearity_left() {
        let (curve, modulus, ell, p_point, q_point) = pairing_toy();
        let p = curve.p;
        let k = modulus.degree();

        let a_ext = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), k, &p);

        // w(P, Q)
        let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
        // w(P, Q)^2
        let w_pq_sq = w_pq.square(&modulus, &p);

        // 2P
        let two_p = p_point.scalar_mul(2, &a_ext, &modulus, &p);
        // w(2P, Q)
        let w_2p_q = weil_pairing::<FpNaive<4>>(&curve, &modulus, &two_p, &q_point, ell);

        assert_eq!(
            w_2p_q, w_pq_sq,
            "left bilinearity: w(2P, Q) should equal w(P, Q)^2"
        );
    }

    /// Right bilinearity: `w(P, 2Q) = w(P, Q)^2`.
    #[test]
    fn weil_bilinearity_right() {
        let (curve, modulus, ell, p_point, q_point) = pairing_toy();
        let p = curve.p;
        let k = modulus.degree();

        let a_ext = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), k, &p);

        // w(P, Q)
        let w_pq = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &q_point, ell);
        // w(P, Q)^2
        let w_pq_sq = w_pq.square(&modulus, &p);

        // 2Q
        let two_q = q_point.scalar_mul(2, &a_ext, &modulus, &p);
        // w(P, 2Q)
        let w_p_2q = weil_pairing::<FpNaive<4>>(&curve, &modulus, &p_point, &two_q, ell);

        assert_eq!(
            w_p_2q, w_pq_sq,
            "right bilinearity: w(P, 2Q) should equal w(P, Q)^2"
        );
    }
}
