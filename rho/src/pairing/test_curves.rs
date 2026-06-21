//! Pairing-friendly toy fixture for the bilinear-pairing KATs.
//!
//! # Fixture: `pairing_toy`
//!
//! A toy curve with **embedding degree `k = 2`** with respect to the torsion
//! prime `ℓ = 3`, over the base field `F_47`.
//!
//! ## Curve
//!
//! `E: y² = x³ + x + 33  mod  47`
//!
//! This is the same curve as `rho::curve::test_curves::composite_toy` (group
//! order `#E(F_47) = 60 = 2² · 3 · 5`), reused here because `3 | 60` gives
//! the 3-torsion we need.
//!
//! ## Embedding degree minimality
//!
//! For `ℓ = 3` and `p = 47`:
//! - `ℓ | p² − 1 = 2208 = 3 · 736`  ✓
//! - `ℓ ∤ p − 1 = 46`               ✓  (46 / 3 is not an integer)
//!
//! Therefore the embedding degree of `E` with respect to `ℓ = 3` is **exactly
//! `k = 2`**.  The pairing lands in `F_{47^2}*`, not a smaller field.
//!
//! ## Extension field
//!
//! `F_{47^2} = F_47[u] / (u² + 1)`.  The modulus `u² + 1` is irreducible over
//! `F_47` because `47 ≡ 3 (mod 4)`, so `−1` is a quadratic non-residue mod 47
//! (second supplement to quadratic reciprocity), meaning `u² + 1` has no root
//! in `F_47`.
//!
//! ## Torsion points
//!
//! | Symbol | Coordinates | Field |
//! |--------|-------------|-------|
//! | `P` | `(8, 6)` | `E(F_47)[3]` |
//! | `Q` | `((4, 15), (22, 34))` | `E(F_{47^2})[3] \ E(F_47)` |
//!
//! Both satisfy `3·P = ∞` and `3·Q = ∞`.  `P` and `Q` are **linearly
//! independent**: `Q ∉ ⟨P⟩ = {∞, P, 2P}`.
//!
//! ## Weil vs Tate: argument order
//!
//! The **Weil pairing** `w_ℓ(P, Q)` is non-degenerate for any linearly
//! independent pair — the ratio `f_{ℓ,P}(Q) / f_{ℓ,Q}(P)` cancels the
//! "bad" part regardless of which point is in `E(F_p)`.
//!
//! The **Tate pairing** `t_ℓ(A, B)` is non-degenerate when `B ∉ ℓ·E(F_{p^k})`.
//! For this fixture:
//! - `P ∈ E(F_p)[ℓ]` is NOT in `ℓ·E(F_{p^k})` (it is in the eigenvalue-1
//!   eigenspace of Frobenius on `E[ℓ]`, which is the Z/3 factor of the 3-Sylow
//!   `Z/3 × Z/9` of `E(F_{47^2})`; this factor has no preimage under `[3]`).
//! - `Q ∈ E(F_{p^k})[ℓ] \ E(F_p)` IS in `ℓ·E(F_{p^k})` (it is in the
//!   eigenvalue-(-1) eigenspace, which is the image of `[3]` on the Z/9 factor).
//!
//! Therefore: `t_ℓ(P, Q) = 1` (trivial) but `t_ℓ(Q, P) ≠ 1` (non-degenerate).
//! The Tate KATs use `t_ℓ(Q, P)` — Q as the first (Miller-base) argument and
//! P as the second (evaluation) argument.
//!
//! ## Full 3-torsion structure
//!
//! `E[3]` over `F_{47^2}` has exactly 9 points (`Z/3 × Z/3`), confirming the
//! embedding degree is exactly 2 (the full torsion group first appears over
//! `F_{p^k}`).
//!
//! ## Parameters computed offline
//!
//! All coordinates were computed by brute-force enumeration over `F_47` and
//! `F_{47^2}` and verified by:
//! - `3·P = ∞` (base-field scalar multiplication)
//! - `3·Q = ∞` (extension-field scalar multiplication via `PairingPoint`)
//! - `Q ≠ P`, `Q ≠ 2P`, `Q ≠ ∞` (independence)
//! - `is_on_curve` for both `P` and `Q`

use crypto_bigint::Uint;
use shared_field::{Fp, FpNaive};

use crate::curve::Curve;
use crate::pairing::ecext::PairingPoint;
use crate::pairing::fpext::{FpExt, IrreducibleModulus};

// ── Fixture constants ─────────────────────────────────────────────────────────

/// Torsion prime for the pairing-friendly fixture.
///
/// `ℓ = 3` divides `p² − 1 = 2208` but not `p − 1 = 46`, so the embedding
/// degree with respect to `ℓ` is exactly `k = 2`.
pub const PAIRING_TOY_ELL: u64 = 3;

/// Embedding degree for the pairing-friendly fixture.
pub const PAIRING_TOY_K: u32 = 2;

/// Base-field prime for the pairing-friendly fixture.
pub const PAIRING_TOY_P: u64 = 47;

// ── Fixture constructor ───────────────────────────────────────────────────────

/// Return the pairing-friendly toy fixture.
///
/// Returns `(curve, modulus, ℓ, P, Q)` where:
/// - `curve` — `y² = x³ + x + 33 mod 47` (same as `composite_toy`).
/// - `modulus` — irreducible `u² + 1` over `F_47` (the `F_{47^2}` modulus).
/// - `ℓ = 3` — the torsion prime; embedding degree `k = 2` w.r.t. `ℓ`.
/// - `P = (8, 6)` — a base-field 3-torsion point in `E(F_47)[3]`, lifted to
///   `E(F_{47^2})` as `((8, 0), (6, 0))`.
/// - `Q = ((4, 15), (22, 34))` — a linearly-independent 3-torsion point in
///   `E(F_{47^2})[3] \ E(F_47)`.
///
/// # Embedding degree minimality
///
/// `ℓ | p² − 1 = 2208` and `ℓ ∤ p − 1 = 46`, so `k = 2` is minimal.
///
/// # Independence
///
/// `⟨P⟩ = {∞, P, 2P}` and `Q ∉ ⟨P⟩`, so `P` and `Q` are linearly independent
/// generators of `E[3] ≅ Z/3 × Z/3`.
pub fn pairing_toy() -> (
    Curve,
    IrreducibleModulus<FpNaive<4>>,
    u64,
    PairingPoint<FpNaive<4>>,
    PairingPoint<FpNaive<4>>,
) {
    let p = Uint::<4>::from(PAIRING_TOY_P);

    // Curve: y² = x³ + x + 33 mod 47
    let curve = Curve {
        p,
        a: Uint::<4>::from(1u64),
        b: Uint::<4>::from(33u64),
        n: Uint::<4>::from(60u64), // #E(F_47) = 60 = 2² · 3 · 5
        gx: Uint::<4>::from(10u64),
        gy: Uint::<4>::from(3u64),
    };

    // Irreducible modulus: u² + 1 over F_47.
    // Coefficients [1, 0, 1] represent 1 + 0·u + 1·u² (monic, degree 2).
    let modulus = IrreducibleModulus::new(
        vec![
            FpNaive::<4>::from_u64(1, &p), // constant: 1
            FpNaive::<4>::from_u64(0, &p), // u¹: 0
            FpNaive::<4>::from_u64(1, &p), // u²: 1 (monic leading coeff)
        ],
        &p,
    );

    // P = (8, 6) ∈ E(F_47)[3], lifted to E(F_{47^2}) as ((8,0), (6,0)).
    let p_point = PairingPoint::new(
        FpExt { coeffs: vec![FpNaive::<4>::from_u64(8, &p), FpNaive::<4>::from_u64(0, &p)] },
        FpExt { coeffs: vec![FpNaive::<4>::from_u64(6, &p), FpNaive::<4>::from_u64(0, &p)] },
    );

    // Q = ((4,15), (22,34)) ∈ E(F_{47^2})[3] \ E(F_47).
    let q_point = PairingPoint::new(
        FpExt { coeffs: vec![FpNaive::<4>::from_u64(4, &p), FpNaive::<4>::from_u64(15, &p)] },
        FpExt { coeffs: vec![FpNaive::<4>::from_u64(22, &p), FpNaive::<4>::from_u64(34, &p)] },
    );

    (curve, modulus, PAIRING_TOY_ELL, p_point, q_point)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;

    // ── Embedding degree minimality ───────────────────────────────────────────

    /// Assert `ℓ | p² − 1` (ℓ divides the p²-1 cyclotomic factor).
    #[test]
    fn embedding_degree_ell_divides_p2_minus_1() {
        let p = PAIRING_TOY_P;
        let ell = PAIRING_TOY_ELL;
        let p2_minus_1 = p * p - 1;
        assert_eq!(
            p2_minus_1 % ell,
            0,
            "ℓ = {ell} must divide p² − 1 = {p2_minus_1}"
        );
    }

    /// Assert `ℓ ∤ p − 1` (minimality: embedding degree is exactly k=2, not k=1).
    #[test]
    fn embedding_degree_ell_does_not_divide_p_minus_1() {
        let p = PAIRING_TOY_P;
        let ell = PAIRING_TOY_ELL;
        let p_minus_1 = p - 1;
        assert_ne!(
            p_minus_1 % ell,
            0,
            "ℓ = {ell} must NOT divide p − 1 = {p_minus_1} (embedding degree must be exactly k=2)"
        );
    }

    // ── Torsion: 3·P = ∞ ─────────────────────────────────────────────────────

    /// `3·P = ∞` — P is a 3-torsion point.
    #[test]
    fn p_is_3_torsion() {
        let (_, modulus, ell, p_point, _) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let ell_p = p_point.scalar_mul(ell, &a, &modulus, &p);
        assert!(ell_p.is_infinity(), "3·P should be ∞ (P is a 3-torsion point)");
    }

    /// `3·Q = ∞` — Q is a 3-torsion point.
    #[test]
    fn q_is_3_torsion() {
        let (_, modulus, ell, _, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let ell_q = q_point.scalar_mul(ell, &a, &modulus, &p);
        assert!(ell_q.is_infinity(), "3·Q should be ∞ (Q is a 3-torsion point)");
    }

    // ── On-curve checks ───────────────────────────────────────────────────────

    /// P is on the curve over `F_{47^2}`.
    #[test]
    fn p_is_on_curve() {
        let (_, modulus, _, p_point, _) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let b = FpExt::from_base(FpNaive::<4>::from_u64(33, &p), 2, &p);
        assert!(
            p_point.is_on_curve(&a, &b, &modulus, &p),
            "P should be on the curve"
        );
    }

    /// Q is on the curve over `F_{47^2}`.
    #[test]
    fn q_is_on_curve() {
        let (_, modulus, _, _, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let b = FpExt::from_base(FpNaive::<4>::from_u64(33, &p), 2, &p);
        assert!(
            q_point.is_on_curve(&a, &b, &modulus, &p),
            "Q should be on the curve"
        );
    }

    // ── Independence: Q ∉ ⟨P⟩ ────────────────────────────────────────────────

    /// `Q ≠ P` (Q is not P itself).
    #[test]
    fn q_not_equal_to_p() {
        let (_, _, _, p_point, q_point) = pairing_toy();
        assert_ne!(q_point, p_point, "Q should not equal P");
    }

    /// `Q ≠ 2P` (Q is not the second non-trivial element of ⟨P⟩).
    #[test]
    fn q_not_equal_to_2p() {
        let (_, modulus, _, p_point, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let two_p = p_point.scalar_mul(2, &a, &modulus, &p);
        assert_ne!(q_point, two_p, "Q should not equal 2P");
    }

    /// `Q ≠ ∞` (Q is not the identity).
    #[test]
    fn q_not_infinity() {
        let (_, _, _, _, q_point) = pairing_toy();
        assert!(!q_point.is_infinity(), "Q should not be the point at infinity");
    }

    // ── Q is not in E(F_47) ───────────────────────────────────────────────────

    /// Q has a non-zero imaginary component in its x-coordinate, confirming it
    /// is not in `E(F_47)` (which would require `x₁ = 0`).
    #[test]
    fn q_not_in_base_field() {
        let (_, _, _, _, q_point) = pairing_toy();
        match &q_point {
            PairingPoint::Infinity => panic!("Q should not be infinity"),
            PairingPoint::Finite { x, .. } => {
                let p = Uint::<4>::from(PAIRING_TOY_P);
                // x₁ ≠ 0 means Q is not in E(F_47).
                assert!(
                    !x.coeffs[1].is_zero(&p),
                    "Q.x should have non-zero imaginary part (Q ∉ E(F_47))"
                );
            }
        }
    }

    // ── Group law on torsion points ───────────────────────────────────────────

    /// `P + Q` is on the curve.
    #[test]
    fn p_plus_q_is_on_curve() {
        let (_, modulus, _, p_point, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let b = FpExt::from_base(FpNaive::<4>::from_u64(33, &p), 2, &p);
        let pq = p_point.add(&q_point, &a, &modulus, &p);
        assert!(pq.is_on_curve(&a, &b, &modulus, &p), "P + Q should be on the curve");
    }

    /// `P + Q` is also a 3-torsion point: `3·(P + Q) = ∞`.
    #[test]
    fn p_plus_q_is_3_torsion() {
        let (_, modulus, ell, p_point, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let pq = p_point.add(&q_point, &a, &modulus, &p);
        let three_pq = pq.scalar_mul(ell, &a, &modulus, &p);
        assert!(three_pq.is_infinity(), "3·(P + Q) should be ∞");
    }

    // ── Known scalar-mul values ───────────────────────────────────────────────

    /// `2P = ((8,0), (41,0))` — known value from offline computation.
    #[test]
    fn two_p_known_value() {
        let (_, modulus, _, p_point, _) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let two_p = p_point.scalar_mul(2, &a, &modulus, &p);
        match &two_p {
            PairingPoint::Infinity => panic!("2P should not be ∞"),
            PairingPoint::Finite { x, y } => {
                assert_eq!(x.to_uint_vec(), vec![Uint::<4>::from(8u64), Uint::<4>::ZERO]);
                assert_eq!(y.to_uint_vec(), vec![Uint::<4>::from(41u64), Uint::<4>::ZERO]);
            }
        }
    }

    /// `2Q = ((4,15), (25,13))` — known value from offline computation.
    #[test]
    fn two_q_known_value() {
        let (_, modulus, _, _, q_point) = pairing_toy();
        let p = Uint::<4>::from(PAIRING_TOY_P);
        let a = FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p);
        let two_q = q_point.scalar_mul(2, &a, &modulus, &p);
        match &two_q {
            PairingPoint::Infinity => panic!("2Q should not be ∞"),
            PairingPoint::Finite { x, y } => {
                assert_eq!(x.to_uint_vec(), vec![Uint::<4>::from(4u64), Uint::<4>::from(15u64)]);
                assert_eq!(y.to_uint_vec(), vec![Uint::<4>::from(25u64), Uint::<4>::from(13u64)]);
            }
        }
    }
}
