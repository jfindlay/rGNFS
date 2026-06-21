//! Affine F_p point → Z_p lift via Hensel (C-AnomalousLift, E.E.1).
//!
//! Given an affine point `P = (x₀, y₀) ∈ E(F_p)` and a target precision `k`, this module
//! lifts `P` to a pair `(x̃, ỹ) ∈ Z/p^k × Z/p^k` satisfying `ỹ² ≡ x̃³ + ax̃ + b (mod p^k)`.
//!
//! # Lift strategy
//!
//! The x-coordinate is lifted exactly: `x̃ = Zp::new(x₀, p, k)`. The y-coordinate is
//! Hensel-solved from the curve equation: build `g(y) = y² − c` where `c = x̃³ + ax̃ + b`
//! (computed over `BigInt` mod `p^k`), then call `hensel_lift(g, y₀, p, k)` with `y₀` as
//! the simple root mod `p`.
//!
//! # Simple-root precondition
//!
//! The Hensel y-solve requires `g'(y₀) = 2·y₀ ≢ 0 mod p`, i.e. `y₀ ≠ 0` and `p ≠ 2`.
//! A 2-torsion base point (`y₀ = 0`) or `p = 2` is a degenerate case; `hensel_lift` returns
//! `HenselError::NonSimpleRoot` in that case, which propagates as `SsaError::LiftFailed`.
//!
//! # Lift-correctness invariant (C-AnomalousLift)
//!
//! The returned `(x̃, ỹ)` satisfies `ỹ² ≡ x̃³ + ax̃ + b (mod p^k)`.
//!
//! # Principle-4 boundary
//!
//! Toy-scale only: `Uint<4>` coordinates at toy `p` fit one `u64` limb. The limb extraction
//! `as_words()[0]` is correct only at toy scale. Crypto-scale `p` would need the full
//! `Uint<4>→BigInt` path.

use num_bigint::BigInt;
use shared_numfield::poly::IntPoly;
use shared_padic::{Zp, hensel_lift};

use shared_field::Fp;

use crate::curve::{AffinePoint, Curve};
use crate::ssa::{SsaError, uint4_to_bigint};

// ─── point lift ──────────────────────────────────────────────────────────────

/// Lift an affine F_p point to Z_p coordinates at precision `k`.
///
/// Given `point = (x₀, y₀) ∈ E(F_p)` and the curve `y² = x³ + ax + b`, returns
/// `(x̃, ỹ)` in `Z/p^k × Z/p^k` satisfying `ỹ² ≡ x̃³ + ax̃ + b (mod p^k)`.
///
/// # Arguments
///
/// - `point` — an affine point on `curve` over `F_p` (must be finite, not the identity).
/// - `curve` — the short Weierstrass curve `y² = x³ + ax + b`.
/// - `k` — the target p-adic precision.
///
/// # Returns
///
/// `(x̃, ỹ)` as a `(Zp, Zp)` pair at precision `k`.
///
/// # Errors
///
/// - [`SsaError::LiftFailed`] if the point is the identity (infinity) or a 2-torsion point
///   (`y₀ = 0`), or if `p = 2` (non-simple root — Hensel lift not applicable).
/// - [`SsaError::Padic`] for Z/p^k arithmetic errors.
///
/// # Principle-4 annotation
///
/// SCALE: toy-scale only — `Uint<4>` coordinates at toy `p` fit one `u64` limb; the
/// `as_words()[0]` extraction is correct only when `p < 2^64`. Crypto-scale `p` would
/// need the full `Uint<4>→BigInt` conversion.
pub fn lift_point<F: Fp<4>>(
    point: &AffinePoint<F>,
    curve: &Curve,
    k: u32,
) -> Result<(Zp, Zp), SsaError> {
    // Extract (x₀, y₀) from the affine point. The identity cannot be lifted.
    let (x0_uint, y0_uint) = match point {
        AffinePoint::Infinity => {
            return Err(SsaError::LiftFailed(shared_padic::HenselError::NotARoot {
                r0: BigInt::from(0i64),
                p: uint4_to_bigint(&curve.p),
                residue: BigInt::from(0i64),
            }));
        }
        AffinePoint::Finite { x, y } => (x.to_uint(), y.to_uint()),
    };

    // SCALE: toy-scale only — crypto-scale p would need full Uint<4>→BigInt conversion.
    let x0 = uint4_to_bigint(&x0_uint);
    let y0 = uint4_to_bigint(&y0_uint);
    let p_big = uint4_to_bigint(&curve.p);
    let a_big = uint4_to_bigint(&curve.a);
    let b_big = uint4_to_bigint(&curve.b);

    // Step 1: lift x exactly — x̃ = Zp::new(x₀, p, k).
    // The x-coordinate needs no Hensel iteration: it lifts to itself.
    let x_tilde = Zp::new(&x0, &p_big, k)?;

    // Step 2: compute c = x₀³ + a·x₀ + b over BigInt (the RHS of the curve equation).
    // This is the constant term for the Hensel polynomial g(y) = y² − c.
    // We compute it over BigInt (not mod p^k) so that hensel_lift can reduce internally.
    let c = &x0 * &x0 * &x0 + &a_big * &x0 + &b_big;

    // Step 3: build g(y) = y² − c as an IntPoly.
    // Coefficients: [−c, 0, 1] (constant, linear, quadratic).
    // g(y) = 1·y² + 0·y + (−c)
    let g = IntPoly::from_coeffs(vec![-c.clone(), BigInt::from(0i64), BigInt::from(1i64)]);

    // Step 4: Hensel-solve g(y) = 0 for ỹ, starting from y₀ as the root mod p.
    // g'(y) = 2y; g'(y₀) = 2·y₀. The simple-root condition requires y₀ ≠ 0 and p ≠ 2.
    // If y₀ = 0 (2-torsion point), hensel_lift returns HenselError::NonSimpleRoot.
    let y_tilde_big = hensel_lift(&g, &y0, &p_big, k)?;
    let y_tilde = Zp::new(&y_tilde_big, &p_big, k)?;

    Ok((x_tilde, y_tilde))
}

// ─── lift-correctness check ───────────────────────────────────────────────────

/// Verify the lift-correctness invariant: `ỹ² ≡ x̃³ + ax̃ + b (mod p^k)`.
///
/// Returns `true` iff the lifted point satisfies the curve equation mod `p^k`.
/// This is the C-AnomalousLift correctness defense.
#[allow(dead_code)] // consumed by E.E.2 and KATs
pub fn check_lift_on_curve(x_tilde: &Zp, y_tilde: &Zp, curve: &Curve) -> Result<bool, SsaError> {
    let p_big = uint4_to_bigint(&curve.p);
    let a_big = uint4_to_bigint(&curve.a);
    let b_big = uint4_to_bigint(&curve.b);
    let k = x_tilde.precision().min(y_tilde.precision());

    // lhs = ỹ²
    let y_sq = y_tilde.mul(y_tilde)?;

    // rhs = x̃³ + a·x̃ + b
    let x_sq = x_tilde.mul(x_tilde)?;
    let x_cu = x_sq.mul(x_tilde)?;
    let a_zp = Zp::new(&a_big, &p_big, k)?;
    let b_zp = Zp::new(&b_big, &p_big, k)?;
    let ax = a_zp.mul(x_tilde)?;
    let rhs = x_cu.add(&ax)?.add(&b_zp)?;

    Ok(y_sq.residue() == rhs.residue())
}
