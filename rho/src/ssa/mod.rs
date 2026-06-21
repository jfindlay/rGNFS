//! Smart–Satoh–Araki p-adic attack on anomalous elliptic curves.
//!
//! This module implements the SSA reduction, which solves the ECDLP in polynomial time on
//! anomalous curves (curves with trace of Frobenius = 1, i.e. `#E(F_p) = p`). It consumes
//! the frozen p-adic substrate from `shared-padic` (C-Padic, C-Hensel, C-PadicLog).
//!
//! # Structure
//!
//! - [`mod`] (this file) — `SsaError` enum, the hardcoded anomalous toy curve fixture, and
//!   `verify_anomalous` (O(p) point-count via Legendre symbol).
//! - [`lift`] — affine F_p point → Z_p lift via Hensel for the SSA reduction.
//!
//! # Anomalous curve fixture
//!
//! The fixture is `y² = x³ + 5` over `F_7` (`p = 7`, `a = 0`, `b = 5`). This curve has
//! exactly 7 points (including the point at infinity), so `#E(F_7) = 7 = p` and the trace
//! of Frobenius is 1. The base point `G = (3, 2)` has `y ≠ 0`, ensuring the Hensel y-solve
//! has a simple root (`f'(y₀) = 2·y₀ = 4 ≢ 0 mod 7`).
//!
//! # Principle-4 boundary
//!
//! The fixture is hand-picked (not discovered via Schoof–SEA). The O(p) point-count verify
//! is a fixture check, not a general point-counting algorithm. Toy precision only.

pub mod formal_log;
pub mod lift;
pub mod reduce;

pub use reduce::ssa_solve;

use crypto_bigint::Uint;
use num_bigint::BigInt;
use shared_padic::HenselError;
use shared_padic::ZpError;

use shared_field::Fp;

use crate::curve::Curve;

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from the SSA reduction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SsaError {
    /// The curve is not anomalous: `#E(F_p) ≠ p` (trace of Frobenius ≠ 1).
    ///
    /// The SSA reduction exists only for anomalous curves. Calling `ssa_solve` on a
    /// non-anomalous curve would compute a meaningless value; this error is the
    /// precondition guard.
    NotAnomalous,
    /// Hensel lifting failed (e.g. non-simple root — 2-torsion base point with y = 0).
    LiftFailed(HenselError),
    /// A Z/p^k arithmetic error from the p-adic layer.
    Padic(ZpError),
}

impl std::fmt::Display for SsaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SsaError::NotAnomalous => write!(f, "curve is not anomalous: #E(F_p) ≠ p"),
            SsaError::LiftFailed(e) => write!(f, "Hensel lift failed: {e}"),
            SsaError::Padic(e) => write!(f, "p-adic arithmetic error: {e}"),
        }
    }
}

impl std::error::Error for SsaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SsaError::LiftFailed(e) => Some(e),
            SsaError::Padic(e) => Some(e),
            SsaError::NotAnomalous => None,
        }
    }
}

impl From<HenselError> for SsaError {
    fn from(e: HenselError) -> Self {
        SsaError::LiftFailed(e)
    }
}

impl From<ZpError> for SsaError {
    fn from(e: ZpError) -> Self {
        SsaError::Padic(e)
    }
}

// ─── anomalous toy curve fixture ─────────────────────────────────────────────

/// The prime for the anomalous toy fixture: p = 7.
pub const ANOMALOUS_TOY_P: u64 = 7;

/// Return the hardcoded anomalous toy curve fixture.
///
/// The curve is `y² = x³ + 5` over `F_7` (`p = 7`, `a = 0`, `b = 5`). It has exactly
/// `#E(F_7) = 7 = p` points (trace of Frobenius = 1). The base point `G = (3, 2)` has
/// `y ≠ 0`, so the Hensel y-solve has a simple root.
///
/// # Principle-4 annotation
///
/// This fixture is hand-picked, not discovered via Schoof–SEA. The O(p) `verify_anomalous`
/// helper confirms the order; it is a fixture check, not a general point-counting algorithm.
pub fn anomalous_toy() -> Curve {
    Curve {
        p: Uint::<4>::from(ANOMALOUS_TOY_P),
        a: Uint::<4>::from(0u64),
        b: Uint::<4>::from(5u64),
        // n = p = 7 (the group order equals the field characteristic for anomalous curves).
        n: Uint::<4>::from(ANOMALOUS_TOY_P),
        // Base point G = (3, 2): 3³ + 5 = 32 ≡ 4 mod 7, 2² = 4 ✓
        gx: Uint::<4>::from(3u64),
        gy: Uint::<4>::from(2u64),
    }
}

// ─── anomalous verification ───────────────────────────────────────────────────

/// Check whether `curve` is anomalous: `#E(F_p) = p` (trace of Frobenius = 1).
///
/// Counts points via the Legendre symbol (O(p) — a fixture check, not Schoof–SEA). Returns
/// `true` iff the point count equals `p`.
///
/// # Principle-4 boundary
///
/// This is an O(p) brute-force count. It is correct only for small toy primes; at crypto
/// scale (p ~ 2^256), Schoof–SEA would be required. The fixture is hand-picked; this
/// function verifies the hand-picked value, not discovers anomalous curves.
pub fn verify_anomalous<F: Fp<4>>(curve: &Curve) -> bool {
    // Extract p as u64.
    // SCALE: toy-scale only — crypto-scale p would need full Uint<4>→BigInt conversion.
    let p_words = curve.p.as_words();
    debug_assert!(
        p_words[1] == 0 && p_words[2] == 0 && p_words[3] == 0,
        "verify_anomalous: p >= 2^64 is not supported at toy scale (principle-4 boundary)"
    );
    let p = p_words[0];

    // Extract a and b as u64.
    let a = curve.a.as_words()[0];
    let b = curve.b.as_words()[0];

    // Count points: #E = 1 (infinity) + Σ_{x=0}^{p-1} (1 + Legendre(x³+ax+b | p))
    let mut count: u64 = 1; // point at infinity
    for x in 0..p {
        let rhs = (x.wrapping_pow(3).wrapping_add(a.wrapping_mul(x)).wrapping_add(b)) % p;
        let leg = legendre_symbol(rhs, p);
        // Legendre symbol: -1 (not QR), 0 (rhs=0), +1 (QR).
        // Number of y solutions: 0, 1, or 2.
        count = count.wrapping_add((1i64 + leg) as u64);
    }

    count == p
}

/// Compute the Legendre symbol (n | p) for odd prime p.
///
/// Returns 0 if p | n, 1 if n is a quadratic residue mod p, -1 otherwise.
/// Uses Euler's criterion: (n | p) = n^((p-1)/2) mod p.
fn legendre_symbol(n: u64, p: u64) -> i64 {
    if n % p == 0 {
        return 0;
    }
    let result = pow_mod(n, (p - 1) / 2, p);
    if result == 1 { 1 } else { -1 }
}

/// Modular exponentiation: base^exp mod modulus (u64, iterative square-and-multiply).
fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

// ─── BigInt helpers (used by lift.rs) ────────────────────────────────────────

/// Extract the toy-scale u64 value from a `Uint<4>` field element.
///
/// # Panics (debug)
///
/// Panics in debug mode if the upper limbs are non-zero (p ≥ 2^64).
///
/// # Principle-4 annotation
///
/// SCALE: toy-scale only — crypto-scale p would need full Uint<4>→BigInt conversion.
pub(crate) fn uint4_to_u64(u: &Uint<4>) -> u64 {
    let words = u.as_words();
    debug_assert!(
        words[1] == 0 && words[2] == 0 && words[3] == 0,
        "uint4_to_u64: Uint<4> has non-zero upper limbs — p >= 2^64 not supported at toy scale \
         (principle-4 boundary)"
    );
    words[0]
}

/// Convert a `Uint<4>` field element to a `BigInt`.
///
/// # Principle-4 annotation
///
/// SCALE: toy-scale only — crypto-scale p would need the full limb vector.
pub(crate) fn uint4_to_bigint(u: &Uint<4>) -> BigInt {
    BigInt::from(uint4_to_u64(u))
}
