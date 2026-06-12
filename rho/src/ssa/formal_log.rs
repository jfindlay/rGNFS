//! Elliptic formal-group logarithm specialising C-PadicLog for the SSA reduction.
//!
//! Given a kernel-of-reduction point in standard projective coordinates `(X, Y, Z)` over Z/p^k,
//! this module computes the elliptic formal-group logarithm `ψ(P̃)` via the local parameter
//! `t = −X/Y` (the formal group parameter in standard projective coordinates).
//!
//! # Mathematical background
//!
//! For a point in the kernel of reduction on a short Weierstrass curve, the formal group
//! parameter is `t = −x/y` where `(x, y)` are affine coordinates. In standard projective
//! coordinates `(X:Y:Z)` with affine `(x, y) = (X/Z, Y/Z)`, this becomes `t = −X/Y`
//! (the Z factors cancel).
//!
//! The elliptic formal-group logarithm is then `ψ(t) = log(1 + t)` evaluated via C-PadicLog.
//! For a kernel-of-reduction point, `v_p(t) ≥ 1`, satisfying the convergence guard.
//!
//! # Projective coordinate representation
//!
//! After the multiply-by-p step in `reduce.rs`, the result is in standard projective coordinates
//! `(X, Y, Z)` where `v_p(Z) ≥ 1` (the point reduces to infinity mod p). The formal group
//! parameter `t = −X/Y` has `v_p(t) ≥ 1` because `v_p(X) ≥ v_p(Y) + 1` for kernel points.
//!
//! # Principle-4 boundary
//!
//! Toy precision only. The series is evaluated at the precision of the input `Zp` elements.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use shared_padic::{Zp, ZpError, padic_log};

use crate::ssa::SsaError;

// ─── elliptic formal-group log ────────────────────────────────────────────────

/// Compute the elliptic formal-group logarithm from standard projective coordinates.
///
/// Given a kernel-of-reduction point `(X, Y, Z)` in standard projective coordinates over Z/p^k
/// (where affine `(x, y) = (X/Z, Y/Z)`), computes `ψ(P̃) = log(1 + t)` where `t = −X/Y`.
///
/// The formal group parameter `t = −X/Y` has `v_p(t) ≥ 1` for kernel-of-reduction points,
/// satisfying C-PadicLog's convergence guard.
///
/// # Arguments
///
/// - `x_proj` — the X projective coordinate (in Z/p^k).
/// - `y_proj` — the Y projective coordinate (in Z/p^k).
/// - `k` — the p-adic precision.
///
/// # Returns
///
/// `ψ(P̃)` as a `Zp` element at precision `k − v_p(t)`.
///
/// # Errors
///
/// - [`SsaError::Padic`] if the p-adic log fails (convergence violation — indicates the
///   caller did not multiply by p first, so the point is not in the kernel of reduction).
///
/// # Principle-4 annotation
///
/// SCALE: toy precision only. The series is truncated at the input precision `k`.
pub fn elliptic_log_proj(x_proj: &Zp, y_proj: &Zp) -> Result<Zp, SsaError> {
    let p = x_proj.prime().clone();
    let k = x_proj.precision().min(y_proj.precision());

    let x_res = x_proj.residue().clone();
    let y_res = y_proj.residue().clone();

    // Compute t = −X/Y in Z_p.
    //
    // X and Y are integers mod p^k. For a kernel-of-reduction point (after multiply-by-p),
    // v_p(X) ≥ v_p(Y) + 1, so t = −X/Y has v_p(t) ≥ 1.
    //
    // Factor out p-parts: X = p^vX * X', Y = p^vY * Y' (units X', Y').
    // t = −p^(vX−vY) * X' / Y'.
    //
    // Precision of t: k_t = k − (vX − vY).
    let vx = padic_valuation_bigint(&x_res, &p);
    let vy = padic_valuation_bigint(&y_res, &p);

    // vt = vX − vY. For kernel-of-reduction points, vt ≥ 1.
    let vt = vx as i64 - vy as i64;
    if vt < 1 {
        // The point is not in the kernel of reduction. This should not happen if the
        // caller multiplied by p first.
        return Err(SsaError::Padic(ZpError::InvalidParams { p, k: 0 }));
    }
    let vt = vt as u32;

    // Precision of t: k_t = k − vt.
    let k_t = k.saturating_sub(vt);
    if k_t < 1 {
        return Err(SsaError::Padic(ZpError::InvalidParams { p, k: k_t }));
    }
    let pk_t = pow_bigint(&p, k_t);

    // Factor out p-parts.
    let p_pow_vx = pow_bigint(&p, vx);
    let p_pow_vy = pow_bigint(&p, vy);
    let x_unit = &x_res / &p_pow_vx; // X' (unit part of X)
    let y_unit = &y_res / &p_pow_vy; // Y' (unit part of Y)

    // t = −p^vt * X' / Y' mod p^k_t.
    let p_pow_vt = pow_bigint(&p, vt);
    let y_unit_inv = mod_inverse_bigint(&y_unit, &pk_t);
    let t_residue = (-(&p_pow_vt * &x_unit * &y_unit_inv)).mod_floor(&pk_t);

    // Construct t as a Zp element at precision k_t.
    let t = Zp::new(&t_residue, &p, k_t)?;

    // Construct z = 1 + t for the padic_log call.
    // v_p(z − 1) = v_p(t) = vt ≥ 1. ✓ (convergence condition satisfied)
    let one = Zp::new(&BigInt::one(), &p, k_t)?;
    let z = one.add(&t)?;

    // Apply the p-adic logarithm: ψ(P̃) = log(1 + t) = log(z).
    let log_z = padic_log(&z).map_err(|e| match e {
        shared_padic::PadicLogError::ConvergenceViolation { .. } => {
            // Should not happen: we checked vt ≥ 1 above.
            SsaError::Padic(ZpError::InvalidParams { p: p.clone(), k: k_t })
        }
        shared_padic::PadicLogError::Zp(e) => SsaError::Padic(e),
    })?;

    Ok(log_z)
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Compute the p-adic valuation of a `BigInt` `x` (number of times p divides x).
///
/// Returns 0 for x = 0 (callers handle the zero case separately).
pub(crate) fn padic_valuation_bigint(x: &BigInt, p: &BigInt) -> u32 {
    if x.is_zero() {
        return 0;
    }
    let mut x = x.abs();
    let mut e: u32 = 0;
    while (&x % p).is_zero() {
        x /= p;
        e += 1;
    }
    e
}

/// Compute `p^k` as a `BigInt`.
pub(crate) fn pow_bigint(p: &BigInt, k: u32) -> BigInt {
    num_traits::pow(p.clone(), k as usize)
}

/// Compute the modular inverse of `a` mod `m` using the extended Euclidean algorithm.
///
/// Precondition: `gcd(a, m) = 1`. Panics if not satisfied.
pub(crate) fn mod_inverse_bigint(a: &BigInt, m: &BigInt) -> BigInt {
    let (mut old_r, mut r) = (a.clone(), m.clone());
    let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());

    while !r.is_zero() {
        let q = &old_r / &r;
        let new_r = &old_r - &q * &r;
        let new_s = &old_s - &q * &s;
        old_r = r;
        r = new_r;
        old_s = s;
        s = new_s;
    }

    assert!(
        old_r.is_one(),
        "mod_inverse_bigint called on non-unit: gcd({a}, {m}) = {old_r}"
    );

    old_s.mod_floor(m)
}
