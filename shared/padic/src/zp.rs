//! Z/p^k arithmetic: the prime-power-modulus ring.
//!
//! [`Zp`] represents an element of Z/p^k — a residue in [0, p^k) together with the prime `p`
//! and precision `k`. This is the arithmetic substrate for the p-adic tower: Hensel lifting
//! (E.D.2) and the p-adic logarithm (E.D.3) both iterate over this type.
//!
//! # Non-field guard
//!
//! Z/p^k is **not** a field for k > 1. Only elements with p-adic valuation 0 (units, coprime to
//! p) are invertible. [`Zp::inv`] returns [`ZpError::NonUnit`] for any element with v_p > 0
//! rather than silently returning a wrong value. This is the load-bearing distinction from
//! `shared/field`'s `Fp<L>`, which uses Fermat's little theorem (valid only for prime moduli).
//!
//! # Precision mixing
//!
//! When two `Zp` elements with different precisions are combined, the result is truncated to the
//! minimum precision (standard p-adic convention: you cannot gain information by combining
//! lower-precision data).
//!
//! # Scope
//!
//! Toy precision only (principle-4 boundary). Crypto-scale precision towers are out of scope.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use thiserror::Error;

// ─── error type ──────────────────────────────────────────────────────────────

/// Errors from Z/p^k arithmetic operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ZpError {
    /// Inversion attempted on a non-unit (v_p > 0). Z/p^k is not a field.
    #[error("inversion of non-unit: v_p({residue}) = {valuation} > 0 (mod p^{k})")]
    NonUnit {
        /// The residue that was not a unit.
        residue: BigInt,
        /// The p-adic valuation of the residue.
        valuation: u64,
        /// The precision of the element.
        k: u32,
    },
    /// Precision mismatch: the two elements carry different primes.
    #[error("prime mismatch: lhs p={lhs_p}, rhs p={rhs_p}")]
    PrimeMismatch {
        /// Prime of the left-hand operand.
        lhs_p: BigInt,
        /// Prime of the right-hand operand.
        rhs_p: BigInt,
    },
    /// Invalid construction: p must be ≥ 2 and k must be ≥ 1.
    #[error("invalid Zp parameters: p={p}, k={k}")]
    InvalidParams {
        /// The prime (or attempted prime).
        p: BigInt,
        /// The precision.
        k: u32,
    },
}

// ─── valuation sentinel ───────────────────────────────────────────────────────

/// The p-adic valuation of an element, with a sentinel for zero.
///
/// v_p(0) is conventionally +∞; we represent it as [`Valuation::Infinity`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Valuation {
    /// Finite valuation: v_p(x) = e for some e ≥ 0.
    Finite(u64),
    /// v_p(0) = +∞ (the zero element is divisible by every power of p).
    Infinity,
}

// ─── Zp ──────────────────────────────────────────────────────────────────────

/// An element of Z/p^k: a residue in [0, p^k) with explicit prime `p` and precision `k`.
///
/// The prime `p` is stored on the element (not threaded per-call) for ergonomics in a precision
/// tower: Hensel iteration constructs many elements at varying precisions, and carrying `p`
/// avoids a per-call parameter.
///
/// # Invariants
///
/// - `residue` is in [0, p^k).
/// - `p` ≥ 2.
/// - `k` ≥ 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zp {
    /// The residue in [0, p^k).
    residue: BigInt,
    /// The prime base.
    p: BigInt,
    /// The precision: the modulus is p^k.
    k: u32,
}

impl Zp {
    // ─── constructors ─────────────────────────────────────────────────────────

    /// Construct a `Zp` element from an arbitrary integer, reducing mod p^k.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::InvalidParams`] if `p < 2` or `k < 1`.
    pub fn new(value: &BigInt, p: &BigInt, k: u32) -> Result<Self, ZpError> {
        if p < &BigInt::from(2u32) || k < 1 {
            return Err(ZpError::InvalidParams { p: p.clone(), k });
        }
        let modulus = modulus(p, k);
        let residue = value.mod_floor(&modulus);
        Ok(Self { residue, p: p.clone(), k })
    }

    /// Construct from a small `i64` value, reducing mod p^k.
    ///
    /// Convenience wrapper around [`Zp::new`] for test and toy use.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::InvalidParams`] if `p < 2` or `k < 1`.
    pub fn from_i64(value: i64, p: i64, k: u32) -> Result<Self, ZpError> {
        Self::new(&BigInt::from(value), &BigInt::from(p), k)
    }

    // ─── accessors ────────────────────────────────────────────────────────────

    /// The residue in [0, p^k).
    #[must_use]
    pub fn residue(&self) -> &BigInt {
        &self.residue
    }

    /// The prime base `p`.
    #[must_use]
    pub fn prime(&self) -> &BigInt {
        &self.p
    }

    /// The precision `k` (the modulus is p^k).
    #[must_use]
    pub fn precision(&self) -> u32 {
        self.k
    }

    /// The p-adic valuation of this element.
    ///
    /// Returns [`Valuation::Infinity`] for the zero element, [`Valuation::Finite(e)`] otherwise,
    /// where e is the largest integer such that p^e divides the residue.
    #[must_use]
    pub fn valuation(&self) -> Valuation {
        padic_valuation(&self.residue, &self.p)
    }

    // ─── arithmetic ───────────────────────────────────────────────────────────

    /// Add two elements, truncating to the minimum precision.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::PrimeMismatch`] if the two elements have different primes.
    pub fn add(&self, rhs: &Self) -> Result<Self, ZpError> {
        let (lhs, rhs) = align_precision(self, rhs)?;
        let modulus = modulus(&lhs.p, lhs.k);
        let residue = (&lhs.residue + &rhs.residue).mod_floor(&modulus);
        Ok(Self { residue, p: lhs.p, k: lhs.k })
    }

    /// Subtract two elements, truncating to the minimum precision.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::PrimeMismatch`] if the two elements have different primes.
    pub fn sub(&self, rhs: &Self) -> Result<Self, ZpError> {
        let (lhs, rhs) = align_precision(self, rhs)?;
        let modulus = modulus(&lhs.p, lhs.k);
        let residue = (&lhs.residue - &rhs.residue).mod_floor(&modulus);
        Ok(Self { residue, p: lhs.p, k: lhs.k })
    }

    /// Multiply two elements, truncating to the minimum precision.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::PrimeMismatch`] if the two elements have different primes.
    pub fn mul(&self, rhs: &Self) -> Result<Self, ZpError> {
        let (lhs, rhs) = align_precision(self, rhs)?;
        let modulus = modulus(&lhs.p, lhs.k);
        let residue = (&lhs.residue * &rhs.residue).mod_floor(&modulus);
        Ok(Self { residue, p: lhs.p, k: lhs.k })
    }

    /// Invert this element mod p^k.
    ///
    /// Inversion is defined only for **units**: elements with p-adic valuation 0 (i.e., coprime
    /// to p). Z/p^k is not a field; non-units have no inverse.
    ///
    /// Uses the extended Euclidean algorithm to compute the modular inverse.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::NonUnit`] if `v_p(self) > 0` (the element is divisible by p).
    pub fn inv(&self) -> Result<Self, ZpError> {
        // Guard: only units invert. This is the load-bearing non-field check.
        match self.valuation() {
            Valuation::Infinity => {
                return Err(ZpError::NonUnit {
                    residue: self.residue.clone(),
                    valuation: u64::MAX, // sentinel for ∞
                    k: self.k,
                });
            }
            Valuation::Finite(v) if v > 0 => {
                return Err(ZpError::NonUnit {
                    residue: self.residue.clone(),
                    valuation: v,
                    k: self.k,
                });
            }
            Valuation::Finite(_) => {}
        }

        let modulus = modulus(&self.p, self.k);
        // Extended Euclidean: find x such that residue * x ≡ 1 (mod p^k).
        // Since gcd(residue, p^k) = 1 (unit), the inverse exists.
        let inv_residue = mod_inverse(&self.residue, &modulus);
        Ok(Self { residue: inv_residue, p: self.p.clone(), k: self.k })
    }

    // ─── precision operations ─────────────────────────────────────────────────

    /// Lift this element to a higher precision k2 > k.
    ///
    /// The residue is unchanged (it is already a valid representative mod p^k2 since it is in
    /// [0, p^k) ⊂ [0, p^k2)). The precision is simply increased.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::InvalidParams`] if `k2 < self.k` (use [`Zp::truncate`] to lower
    /// precision).
    pub fn lift(&self, k2: u32) -> Result<Self, ZpError> {
        if k2 < self.k {
            return Err(ZpError::InvalidParams { p: self.p.clone(), k: k2 });
        }
        // The residue is already in [0, p^k) ⊂ [0, p^k2); no reduction needed.
        Ok(Self { residue: self.residue.clone(), p: self.p.clone(), k: k2 })
    }

    /// Truncate this element to a lower precision k2 ≤ k.
    ///
    /// Reduces the residue mod p^k2.
    ///
    /// # Errors
    ///
    /// Returns [`ZpError::InvalidParams`] if `k2 > self.k` (use [`Zp::lift`] to raise precision)
    /// or `k2 < 1`.
    pub fn truncate(&self, k2: u32) -> Result<Self, ZpError> {
        if k2 > self.k || k2 < 1 {
            return Err(ZpError::InvalidParams { p: self.p.clone(), k: k2 });
        }
        let modulus = modulus(&self.p, k2);
        let residue = self.residue.mod_floor(&modulus);
        Ok(Self { residue, p: self.p.clone(), k: k2 })
    }
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Compute p^k as a `BigInt`.
fn modulus(p: &BigInt, k: u32) -> BigInt {
    num_traits::pow(p.clone(), k as usize)
}

/// Compute the p-adic valuation of `x`: the largest e ≥ 0 such that p^e | x.
///
/// Returns [`Valuation::Infinity`] for x = 0.
fn padic_valuation(x: &BigInt, p: &BigInt) -> Valuation {
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

/// Align two `Zp` elements to the minimum precision, returning clones at that precision.
///
/// # Errors
///
/// Returns [`ZpError::PrimeMismatch`] if the primes differ.
fn align_precision(lhs: &Zp, rhs: &Zp) -> Result<(Zp, Zp), ZpError> {
    if lhs.p != rhs.p {
        return Err(ZpError::PrimeMismatch { lhs_p: lhs.p.clone(), rhs_p: rhs.p.clone() });
    }
    let k = lhs.k.min(rhs.k);
    let modulus = modulus(&lhs.p, k);
    let lhs_r = lhs.residue.mod_floor(&modulus);
    let rhs_r = rhs.residue.mod_floor(&modulus);
    Ok((
        Zp { residue: lhs_r, p: lhs.p.clone(), k },
        Zp { residue: rhs_r, p: rhs.p.clone(), k },
    ))
}

/// Compute the modular inverse of `a` mod `m` using the extended Euclidean algorithm.
///
/// Precondition: gcd(a, m) = 1 (caller must ensure this — `inv` checks the unit condition).
///
/// # Panics
///
/// Panics if `gcd(a, m) ≠ 1` (should never happen when called from `inv` after the unit check).
fn mod_inverse(a: &BigInt, m: &BigInt) -> BigInt {
    // Extended Euclidean algorithm: find (x, y) such that a*x + m*y = gcd(a, m) = 1.
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

    // old_r is now gcd(a, m); old_s is the Bézout coefficient for a.
    assert!(old_r.is_one(), "mod_inverse called on non-unit: gcd({a}, {m}) = {old_r}");

    // Normalise to [0, m).
    old_s.mod_floor(m)
}
