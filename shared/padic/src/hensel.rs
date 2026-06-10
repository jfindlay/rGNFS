//! Hensel lifting: Newton root-lifting over Z_p.
//!
//! Given a polynomial `f ∈ ℤ[x]`, a simple root `r_0` of `f` mod `p` (with `f'(r_0) ≢ 0 mod p`),
//! and a target precision `k`, [`hensel_lift`] lifts `r_0` to the unique root of `f` mod `p^k`
//! via Newton's method.
//!
//! # Newton iteration
//!
//! The iteration is `r ← r − f(r) · f'(r)^{-1}` over Z/p^k, doubling precision each step:
//! if `r_n` is a root mod `p^{2^n}`, then `r_{n+1}` is a root mod `p^{2^{n+1}}`. This
//! quadratic convergence means `⌈log₂(k)⌉` steps suffice to reach precision `k`.
//!
//! # Uniqueness precondition
//!
//! The lift is unique if and only if the root is **simple**: `f'(r_0) ≢ 0 mod p`. A non-simple
//! root (where `f'(r_0) ≡ 0 mod p`) may have zero, one, or multiple lifts; this case is out of
//! scope (principle-4 boundary) and returns [`HenselError::NonSimpleRoot`].
//!
//! # Scope (principle-4 boundary)
//!
//! Simple roots only. Non-simple-root lifting (the general Hensel lemma with slower convergence)
//! is out of scope for this toy-precision substrate.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Zero;
use shared_numfield::poly::IntPoly;
use thiserror::Error;

use crate::zp::{Valuation, Zp, ZpError};

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from Hensel lifting.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HenselError {
    /// The root is not simple: `f'(r_0) ≡ 0 mod p`. The lift is not unique; only simple roots
    /// are supported (principle-4 boundary).
    #[error("non-simple root: f'({r0}) ≡ 0 (mod {p}); Hensel lift requires f'(r0) ≢ 0 mod p")]
    NonSimpleRoot {
        /// The candidate root.
        r0: BigInt,
        /// The prime.
        p: BigInt,
    },
    /// The candidate is not actually a root of `f` mod `p`.
    #[error("r0={r0} is not a root of f mod p={p}: f(r0) ≡ {residue} (mod {p})")]
    NotARoot {
        /// The candidate.
        r0: BigInt,
        /// The prime.
        p: BigInt,
        /// The residue f(r0) mod p.
        residue: BigInt,
    },
    /// A Z/p^k arithmetic error propagated from the underlying [`Zp`] layer.
    #[error("Z/p^k arithmetic error: {0}")]
    Zp(#[from] ZpError),
}

// ─── Hensel lift ─────────────────────────────────────────────────────────────

/// Lift a simple root of `f` mod `p` to the unique root mod `p^k` via Newton's method.
///
/// # Arguments
///
/// - `f` — the polynomial in ℤ[x]
/// - `r0` — a root of `f` mod `p` (i.e. `f(r0) ≡ 0 mod p`)
/// - `p` — the prime base
/// - `k` — the target precision (the result is a root mod `p^k`)
///
/// # Returns
///
/// The unique root `r` of `f` mod `p^k` with `r ≡ r0 (mod p)`, as a `BigInt` in `[0, p^k)`.
///
/// # Errors
///
/// - [`HenselError::NotARoot`] if `f(r0) ≢ 0 mod p`.
/// - [`HenselError::NonSimpleRoot`] if `f'(r0) ≡ 0 mod p` (the uniqueness precondition fails).
/// - [`HenselError::Zp`] for arithmetic errors from the underlying Z/p^k layer.
pub fn hensel_lift(f: &IntPoly, r0: &BigInt, p: &BigInt, k: u32) -> Result<BigInt, HenselError> {
    let df = f.derivative();

    // Verify r0 is actually a root mod p.
    let fr0_mod_p = f.eval_mod(r0, p);
    if !fr0_mod_p.is_zero() {
        return Err(HenselError::NotARoot {
            r0: r0.clone(),
            p: p.clone(),
            residue: fr0_mod_p,
        });
    }

    // Verify the root is simple: f'(r0) ≢ 0 mod p.
    let dfr0_mod_p = df.eval_mod(r0, p);
    if dfr0_mod_p.is_zero() {
        return Err(HenselError::NonSimpleRoot { r0: r0.clone(), p: p.clone() });
    }

    // k = 1 is already done.
    if k == 1 {
        return Ok(r0.mod_floor(p));
    }

    // Newton iteration, doubling precision each step.
    // Invariant: after step n, `r` is a root of f mod p^{current_k}.
    let mut r = r0.mod_floor(p); // start at precision 1
    let mut current_k: u32 = 1;

    while current_k < k {
        // Next precision: double, but cap at the target k.
        let next_k = (2 * current_k).min(k);

        // Evaluate f(r) and f'(r) mod p^{next_k}.
        // We use eval_mod with modulus p^{next_k} to keep values bounded.
        let modulus = pow_bigint(p, next_k);
        let fr = f.eval_mod(&r, &modulus);
        let dfr = df.eval_mod(&r, &modulus);

        // f'(r) must be a unit mod p (it is, since f'(r0) ≢ 0 mod p and r ≡ r0 mod p).
        // Invert f'(r) mod p^{next_k} via the Zp layer.
        let dfr_zp = Zp::new(&dfr, p, next_k)?;
        let dfr_is_nonunit = match dfr_zp.valuation() {
            Valuation::Infinity => true,
            Valuation::Finite(v) => v > 0,
        };
        if dfr_is_nonunit {
            // This should not happen for a simple root, but guard defensively.
            return Err(HenselError::NonSimpleRoot { r0: r0.clone(), p: p.clone() });
        }
        let dfr_inv = dfr_zp.inv()?;

        // Newton step: r ← r − f(r) · f'(r)^{-1} mod p^{next_k}.
        let fr_zp = Zp::new(&fr, p, next_k)?;
        let correction = fr_zp.mul(&dfr_inv)?;
        let r_zp = Zp::new(&r, p, next_k)?;
        let r_new_zp = r_zp.sub(&correction)?;

        r = r_new_zp.residue().clone();
        current_k = next_k;
    }

    Ok(r)
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Compute `p^k` as a `BigInt`.
fn pow_bigint(p: &BigInt, k: u32) -> BigInt {
    num_traits::pow(p.clone(), k as usize)
}
