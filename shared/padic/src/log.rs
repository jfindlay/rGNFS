//! p-adic / formal-group logarithm series.
//!
//! [`padic_log`] computes the formal-group logarithm
//!
//! ```text
//! log(z) = log(1 + x) = x − x²/2 + x³/3 − x⁴/4 + …
//! ```
//!
//! where `z = 1 + x` and `x = z − 1` must satisfy `v_p(x) ≥ 1` (i.e. `z ≡ 1 mod p`). This is
//! the convergence condition: the series converges p-adically on the kernel of reduction.
//!
//! # Convergence guard
//!
//! The series diverges p-adically when `v_p(x) = 0`. [`padic_log`] returns
//! [`PadicLogError::ConvergenceViolation`] if `v_p(z − 1) < 1`. This is the log's analogue of
//! the unit-inversion guard in [`crate::zp::Zp::inv`]: both defend against silent wrong answers.
//!
//! # Precision and the p-divisible denominator subtlety
//!
//! The term `x^n / n` has p-adic valuation `n·v_p(x) − v_p(n)`. With `v_p(x) ≥ 1`, this is
//! `≥ n − v_p(n)`. The series is truncated at the first `n` where `n − v_p(n) ≥ k`: that term
//! (and all subsequent terms) contribute nothing mod `p^k`.
//!
//! **The subtle correctness point:** when `p | n` (e.g. the term `x^p / p`), the denominator
//! lowers the p-adic valuation of the term. Naively inverting `n` mod `p^k` would fail because
//! `gcd(n, p^k) > 1`. The implementation handles this by factoring out the p-part of `n`:
//! write `n = p^a · m` with `gcd(m, p) = 1`. Then `x^n / n = (x^n / p^a) · m⁻¹ mod p^k`.
//! Since `v_p(x^n) = n · v_p(x) ≥ n > a` (for `v_p(x) ≥ 1` and `n ≥ 1`), the division
//! `x^n / p^a` is exact integer division (no rounding), and `m` is a unit so `m⁻¹ mod p^k`
//! exists.
//!
//! # Homomorphism
//!
//! The p-adic logarithm is a group homomorphism from the kernel of reduction (the multiplicative
//! group `1 + p·Z_p`) to the additive group `p·Z_p`:
//!
//! ```text
//! log(a · b) = log(a) + log(b)   (mod p^k)
//! ```
//!
//! This is the property that makes the Smart–Satoh–Araki reduction work: the ECDLP `Q = k·G`
//! becomes additive division after applying the log. (E.E supplies the elliptic-curve
//! specialisation; this module ships the general series.)
//!
//! # Scope (principle-4 boundary)
//!
//! Toy precision only: `k` is small. The elliptic formal-group parametrisation (needed by E.E's
//! SSA reduction) is out of scope here — E.E specialises this general series to the elliptic
//! formal group.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use thiserror::Error;

use crate::zp::{Valuation, Zp, ZpError};

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from the p-adic logarithm.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PadicLogError {
    /// The convergence condition `v_p(z − 1) ≥ 1` is violated.
    ///
    /// The formal-group series `log(1 + x)` converges p-adically only when `x ≡ 0 mod p`.
    /// Passing `z` with `z ≢ 1 mod p` would return a precision-limited garbage value; this
    /// error is the convergence guard that prevents that silent failure.
    #[error(
        "convergence violation: v_p(z − 1) = {valuation} < 1; \
         log(z) requires z ≡ 1 (mod p={p})"
    )]
    ConvergenceViolation {
        /// The p-adic valuation of `z − 1`.
        valuation: u64,
        /// The prime.
        p: BigInt,
    },
    /// A Z/p^k arithmetic error propagated from the underlying [`Zp`] layer.
    #[error("Z/p^k arithmetic error: {0}")]
    Zp(#[from] ZpError),
}

// ─── p-adic logarithm ────────────────────────────────────────────────────────

/// Compute the p-adic logarithm `log(z)` of `z` in Z/p^k.
///
/// Evaluates the formal-group series `log(1 + x) = x − x²/2 + x³/3 − …` where `x = z − 1`,
/// truncated at the precision `k` of `z`.
///
/// # Convergence requirement
///
/// `z` must satisfy `z ≡ 1 (mod p)`, i.e. `v_p(z − 1) ≥ 1`. This is the kernel of reduction:
/// the series converges p-adically only on this subgroup.
///
/// # Returns
///
/// The p-adic logarithm as a `Zp` element at the same precision as `z`.
///
/// # Errors
///
/// - [`PadicLogError::ConvergenceViolation`] if `v_p(z − 1) < 1`.
/// - [`PadicLogError::Zp`] for arithmetic errors from the underlying Z/p^k layer.
pub fn padic_log(z: &Zp) -> Result<Zp, PadicLogError> {
    let p = z.prime().clone();
    let k = z.precision();
    let modulus = pow_bigint(&p, k);

    // x = z − 1 (as a BigInt, before reducing mod p^k).
    // We compute x in [0, p^k) by taking z.residue() − 1 and normalising.
    let x: BigInt = (z.residue() - BigInt::one()).mod_floor(&modulus);

    // Convergence guard: v_p(x) must be ≥ 1.
    // This is the analogue of the unit-inversion guard in Zp::inv.
    let vx = padic_valuation_bigint(&x, &p);
    match vx {
        Valuation::Infinity => {
            // x = 0 means z = 1; log(1) = 0. This is valid (v_p(0) = ∞ ≥ 1).
        }
        Valuation::Finite(v) if v == 0 => {
            return Err(PadicLogError::ConvergenceViolation { valuation: 0, p });
        }
        Valuation::Finite(_) => {
            // v ≥ 1: convergence condition satisfied.
        }
    }

    // Special case: z = 1 (x = 0) → log(1) = 0.
    if x.is_zero() {
        return Ok(Zp::new(&BigInt::zero(), &p, k)?);
    }

    // Compute the series sum = Σ_{n=1}^{N} (−1)^{n+1} · x^n / n  mod p^k
    // using BigInt arithmetic throughout, reducing mod p^k only at the end.
    //
    // Stopping criterion: the term x^n/n has p-adic valuation ≥ n − v_p(n) (with v_p(x) ≥ 1).
    // We stop at the first n where n − v_p(n) ≥ k, since that term contributes 0 mod p^k.
    //
    // See module-level doc for the p-divisible denominator handling.
    let mut sum = BigInt::zero();
    let mut x_pow = x.clone(); // x^n, updated each iteration (starts at x^1)

    for n in 1u64.. {
        let n_big = BigInt::from(n);

        // Stopping criterion: n − v_p(n) ≥ k means this term and all subsequent ones are 0 mod p^k.
        let vpn = padic_valuation_u64(n, &p);
        let effective_valuation = n.saturating_sub(vpn); // n − v_p(n) ≥ 0
        if effective_valuation >= k as u64 {
            break;
        }

        // Compute x^n / n mod p^k.
        //
        // Write n = p^a · m with gcd(m, p) = 1.
        // Then x^n / n = (x^n / p^a) · m⁻¹ mod p^k.
        //
        // Since v_p(x^n) = n · v_p(x) ≥ n > a (because v_p(n) = a ≤ log_p(n) < n for n ≥ 1),
        // the division x^n / p^a is exact integer division.
        //
        // SUBTLE CORRECTNESS POINT: we cannot invert n directly mod p^k when p | n (gcd(n, p^k) > 1).
        // Factoring out the p-part of n is mandatory for correctness.
        let (a, m) = factor_out_p(&n_big, &p); // n = p^a · m
        let p_pow_a = pow_bigint(&p, a as u32);

        // x^n / p^a: exact integer division (guaranteed by v_p(x^n) ≥ n > a).
        let x_pow_div_pa = &x_pow / &p_pow_a;

        // m⁻¹ mod p^k: m is a unit (gcd(m, p) = 1), so this always succeeds.
        let m_inv = mod_inverse_bigint(&m, &modulus);

        // term = (x^n / p^a) · m⁻¹ mod p^k
        let term = (&x_pow_div_pa * &m_inv).mod_floor(&modulus);

        // Sign: (−1)^{n+1} — positive for odd n, negative for even n.
        if n % 2 == 1 {
            sum = (&sum + &term).mod_floor(&modulus);
        } else {
            sum = (&sum - &term).mod_floor(&modulus);
        }

        // Advance x^n → x^{n+1} = x^n · x, reduced mod p^k to keep values bounded.
        x_pow = (&x_pow * &x).mod_floor(&modulus);
    }

    Ok(Zp::new(&sum, &p, k)?)
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Compute `p^k` as a `BigInt`.
fn pow_bigint(p: &BigInt, k: u32) -> BigInt {
    num_traits::pow(p.clone(), k as usize)
}

/// Compute the p-adic valuation of a `BigInt` `x`.
///
/// Returns [`Valuation::Infinity`] for x = 0.
fn padic_valuation_bigint(x: &BigInt, p: &BigInt) -> Valuation {
    if x.is_zero() {
        return Valuation::Infinity;
    }
    let mut x = x.abs();
    let mut e: u64 = 0;
    while (&x % p).is_zero() {
        x /= p;
        e += 1;
    }
    Valuation::Finite(e)
}

/// Compute the p-adic valuation of a `u64` `n` (n ≥ 1).
///
/// Returns 0 if `gcd(n, p) = 1`.
fn padic_valuation_u64(n: u64, p: &BigInt) -> u64 {
    padic_valuation_bigint(&BigInt::from(n), p).into_finite_or(0)
}

/// Factor out all powers of `p` from `n`: returns `(a, m)` where `n = p^a · m`, `gcd(m, p) = 1`.
fn factor_out_p(n: &BigInt, p: &BigInt) -> (u64, BigInt) {
    let mut m = n.clone();
    let mut a: u64 = 0;
    while (&m % p).is_zero() {
        m /= p;
        a += 1;
    }
    (a, m)
}

/// Compute the modular inverse of `a` mod `m` using the extended Euclidean algorithm.
///
/// Precondition: `gcd(a, m) = 1`. Panics if not satisfied.
fn mod_inverse_bigint(a: &BigInt, m: &BigInt) -> BigInt {
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

// ─── Valuation helper ────────────────────────────────────────────────────────

/// Extension trait to extract the finite value from a [`Valuation`], with a fallback.
trait ValuationExt {
    fn into_finite_or(self, default: u64) -> u64;
}

impl ValuationExt for Valuation {
    fn into_finite_or(self, default: u64) -> u64 {
        match self {
            Valuation::Finite(v) => v,
            Valuation::Infinity => default,
        }
    }
}
