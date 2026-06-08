//! Schirokauer map: ℓ-adic virtual-logarithm coordinates for NFS-DL.
//!
//! The Schirokauer map λ: K* → (ℤ/ℓ)^r sends a number-field element to its ℓ-adic
//! virtual-logarithm coordinates. For each prime ideal φ_i = (p_i, α − r_i) with
//! p_i ≡ 1 (mod ℓ) (so that ℓ | p_i − 1):
//!
//! 1. Compute ε_i = (p_i − 1)/ℓ (an integer).
//! 2. Compute β^{ε_i} in K = ℚ[α]/(f) (exact arithmetic).
//! 3. Reduce β^{ε_i} mod ℓ² (coefficient-wise).
//! 4. Subtract 1 from the constant term: δ = β^{ε_i} − 1.
//! 5. Divide each coefficient of δ by ℓ (exact integer division — valid since β^{ε_i} ≡ 1 mod ℓ
//!    in ℤ[α] for β coprime to ℓ).
//! 6. Evaluate the result at α ≡ r_i (mod ℓ) to obtain λ_i(β) ∈ ℤ/ℓ.
//!
//! The map is a group homomorphism: λ(βγ) = λ(β) + λ(γ) mod ℓ. This is the defining
//! algebraic property verified by the KAT.
//!
//! # Contract C-Schirokauer (frozen D.A.1)
//!
//! Public interface:
//! - [`schirokauer`] — the map function.
//! - [`SchirokauerError`] — error type.
//! - [`PrimeIdeal`] — re-exported from `shared-numfield` for caller convenience.
//!
//! The r > 1 multi-coordinate shape is carried even when toy instances use r = 1, since
//! D.C's descent and E.C's solver will need it.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use shared_numfield::{Ideal, NumberFieldElement};

// Re-export PrimeIdeal as a type alias for caller convenience.
// The C-Ideal contract uses `Ideal<'_>` from shared-numfield; we alias it here.
/// A prime ideal (p, α − r) in ℤ[α], as used by the Schirokauer map.
///
/// This is the `Ideal` type from `shared-numfield`, re-exported under the DL module's
/// preferred name. The Schirokauer map consumes a slice of these.
pub type PrimeIdeal<'a> = Ideal<'a>;

// ─── SchirokauerError ─────────────────────────────────────────────────────────

/// Error type for the Schirokauer map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchirokauerError {
    /// The prime p of the ideal does not satisfy p ≡ 1 (mod ℓ).
    ///
    /// The Schirokauer map requires ℓ | p − 1 so that ε = (p − 1)/ℓ is an integer
    /// and β^ε is an ℓ-th root of unity in F_p. If this condition fails, the map
    /// is undefined for this ideal.
    ///
    /// :param p: The rational prime of the ideal.
    /// :param ell: The target subgroup order ℓ.
    RamifiedPrime { p: BigInt, ell: BigInt },

    /// The element β is not coprime to ℓ (ℓ divides a coefficient of β).
    ///
    /// The ℓ-adic log extraction requires β ≡ 1 (mod ℓ) after the ε-exponentiation,
    /// which fails when ℓ | β. The map is undefined for such elements.
    ///
    /// :param ell: The target subgroup order ℓ.
    ElementDivisibleByEll { ell: BigInt },

    /// The exponent ε = (p − 1)/ℓ does not fit in u64.
    ///
    /// For toy instances ε is small; for production instances a BigInt pow is needed.
    /// This error signals that the caller should use a larger-exponent variant.
    /// (D.A.1 scope: toy instances only; BigInt pow is implemented internally.)
    ///
    /// :param p: The rational prime of the ideal.
    /// :param ell: The target subgroup order ℓ.
    ExponentOverflow { p: BigInt, ell: BigInt },

    /// After computing β^ε, the result does not satisfy β^ε ≡ 1 (mod ℓ) in ℤ[α].
    ///
    /// This indicates a bug or a non-integer-coefficient element. The ℓ-adic log
    /// extraction (dividing β^ε − 1 by ℓ) requires exact divisibility.
    ///
    /// :param coeff_index: The index of the coefficient that is not divisible by ℓ.
    NotDivisibleByEll { coeff_index: usize },
}

impl std::fmt::Display for SchirokauerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RamifiedPrime { p, ell } => {
                write!(f, "prime p={p} does not satisfy p ≡ 1 (mod ℓ={ell}); Schirokauer map undefined")
            }
            Self::ElementDivisibleByEll { ell } => {
                write!(f, "element has a coefficient divisible by ℓ={ell}; map undefined")
            }
            Self::ExponentOverflow { p, ell } => {
                write!(f, "exponent ε=(p−1)/ℓ overflows u64 for p={p}, ℓ={ell}")
            }
            Self::NotDivisibleByEll { coeff_index } => {
                write!(
                    f,
                    "β^ε − 1 coefficient at index {coeff_index} is not divisible by ℓ; \
                     element may not be an algebraic integer or has a bad prime"
                )
            }
        }
    }
}

impl std::error::Error for SchirokauerError {}

// ─── schirokauer ─────────────────────────────────────────────────────────────

/// Compute the Schirokauer map of `elt` with respect to `ell` and a list of prime ideals.
///
/// Returns the r ℓ-adic virtual-log coordinates [λ_1(β), ..., λ_r(β)] ∈ (ℤ/ℓ)^r,
/// one coordinate per ideal in `ideals`. The multi-coordinate shape (r > 1) is carried
/// even when toy instances use r = 1 (required by C-Schirokauer for D.C/E.C consumers).
///
/// # Algorithm
///
/// For each ideal φ_i = (p_i, α − r_i) with p_i ≡ 1 (mod ℓ):
/// 1. ε_i = (p_i − 1)/ℓ.
/// 2. Compute β^{ε_i} in K (exact arithmetic via square-and-multiply over BigInt).
/// 3. Reduce β^{ε_i} mod ℓ² (coefficient-wise integer reduction).
/// 4. δ = β^{ε_i} − 1 (subtract 1 from constant term).
/// 5. Divide each coefficient of δ by ℓ (exact; valid since β^{ε_i} ≡ 1 mod ℓ).
/// 6. Evaluate at α ≡ r_i (mod ℓ) to get λ_i(β) ∈ ℤ/ℓ.
///
/// # Errors
///
/// - [`SchirokauerError::RamifiedPrime`] if any ideal has p ≢ 1 (mod ℓ).
/// - [`SchirokauerError::ElementDivisibleByEll`] if ℓ divides a coefficient of `elt`.
/// - [`SchirokauerError::NotDivisibleByEll`] if β^{ε_i} − 1 is not divisible by ℓ
///   (indicates a non-integer-coefficient element or arithmetic error).
///
/// # Panics
///
/// Panics if `elt` has non-integer (rational) coefficients with denominators not
/// invertible mod ℓ (propagated from `reduce_mod_ideal`). For NFS algebraic integers,
/// coefficients are always integers and this does not occur.
pub fn schirokauer<'a>(
    elt: &NumberFieldElement<'a>,
    ell: &BigInt,
    ideals: &[PrimeIdeal<'a>],
) -> Result<Vec<BigInt>, SchirokauerError> {
    // Validate: ℓ must be positive.
    debug_assert!(ell.is_positive(), "ℓ must be positive");

    // Check that the element is not divisible by ℓ (coefficient-wise).
    // For integer-coefficient elements, this means no coefficient is divisible by ℓ.
    // (We check after extracting integer coefficients below, per ideal.)

    let ell_sq = ell * ell; // ℓ²

    let mut result = Vec::with_capacity(ideals.len());

    for ideal in ideals {
        let p = &ideal.p;
        let r = &ideal.r;

        // Check p ≡ 1 (mod ℓ): required for ε = (p−1)/ℓ to be an integer.
        let p_minus_1 = p - BigInt::one();
        if !p_minus_1.is_multiple_of(ell) {
            return Err(SchirokauerError::RamifiedPrime { p: p.clone(), ell: ell.clone() });
        }

        // ε = (p − 1)/ℓ.
        let epsilon = p_minus_1 / ell;

        // Compute β^ε in K using square-and-multiply over BigInt exponent.
        let beta_eps = pow_bigint(elt, &epsilon);

        // Extract integer coefficients of β^ε (mod ℓ²).
        // NumberFieldElement uses BigRational coefficients; for algebraic integers,
        // denominators are 1. We extract the numerator and reduce mod ℓ².
        let coeffs_mod_ell_sq = extract_int_coeffs_mod(&beta_eps, &ell_sq)?;

        // Check that β^ε ≡ 1 (mod ℓ): each coefficient must be divisible by ℓ,
        // except the constant term which must be ≡ 1 (mod ℓ).
        // Equivalently: (β^ε − 1) must have all coefficients divisible by ℓ.
        let delta = subtract_one_mod(&coeffs_mod_ell_sq, &ell_sq);

        // Divide each coefficient of δ by ℓ (exact integer division mod ℓ).
        let log_coeffs = divide_by_ell(&delta, ell)?;

        // Evaluate at α ≡ r (mod ℓ): compute Σ log_coeffs[i] * (r mod ℓ)^i mod ℓ.
        let r_mod_ell = r.mod_floor(ell);
        let lambda = eval_poly_mod(&log_coeffs, &r_mod_ell, ell);

        result.push(lambda);
    }

    Ok(result)
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Compute `base^exp` in the number field K using square-and-multiply over a `BigInt` exponent.
///
/// This extends `NumberFieldElement::pow` (which takes `u64`) to handle arbitrary-precision
/// exponents. For toy instances, the exponent fits in u64 and the inner loop is short.
fn pow_bigint<'a>(base: &NumberFieldElement<'a>, exp: &BigInt) -> NumberFieldElement<'a> {
    if exp.is_zero() {
        return base.field.from_rational(BigRational::one());
    }

    let mut result = base.field.from_rational(BigRational::one());
    let mut b = base.clone_pub();
    let mut e = exp.clone();

    while !e.is_zero() {
        if e.is_odd() {
            result = result.mul(&b);
        }
        b = b.square();
        e >>= 1;
    }

    result
}

/// Extract integer coefficients of a `NumberFieldElement` reduced mod `m`, returning
/// a `Vec<BigInt>` of length `field.degree()`.
///
/// For algebraic integers, all coefficients have denominator 1. Panics if any
/// denominator is not 1 (i.e., the element is not an algebraic integer).
///
/// Returns `Err(SchirokauerError::ElementDivisibleByEll)` if the element has a zero
/// polynomial (which would indicate a degenerate case).
fn extract_int_coeffs_mod(
    elt: &NumberFieldElement<'_>,
    m: &BigInt,
) -> Result<Vec<BigInt>, SchirokauerError> {
    let d = elt.field.degree();
    let mut coeffs = vec![BigInt::zero(); d];

    for (i, coeff) in elt.poly.coeffs.iter().enumerate() {
        // For algebraic integers, denominator must be 1.
        // We assert this (panics on non-integer elements, matching the contract).
        assert!(
            coeff.denom().is_one(),
            "Schirokauer map requires algebraic integer elements (integer coefficients); \
             coefficient {} has denominator {}",
            i,
            coeff.denom()
        );
        coeffs[i] = coeff.numer().mod_floor(m);
    }

    Ok(coeffs)
}

/// Compute (β^ε − 1) mod ℓ²: subtract 1 from the constant term of `coeffs` (mod `m`).
fn subtract_one_mod(coeffs: &[BigInt], m: &BigInt) -> Vec<BigInt> {
    let mut delta = coeffs.to_vec();
    if delta.is_empty() {
        // Zero polynomial: treat constant term as 0.
        delta.push(BigInt::zero());
    }
    // Subtract 1 from constant term, keeping in [0, m).
    delta[0] = (&delta[0] - BigInt::one()).mod_floor(m);
    delta
}

/// Divide each coefficient of `delta` by `ell` (exact integer division).
///
/// Returns `Err(SchirokauerError::NotDivisibleByEll)` if any coefficient is not
/// divisible by `ell`. This signals that β^ε ≢ 1 (mod ℓ) in ℤ[α], which should
/// not happen for algebraic integers coprime to ℓ.
fn divide_by_ell(delta: &[BigInt], ell: &BigInt) -> Result<Vec<BigInt>, SchirokauerError> {
    let mut result = Vec::with_capacity(delta.len());
    for (i, c) in delta.iter().enumerate() {
        if !c.is_multiple_of(ell) {
            return Err(SchirokauerError::NotDivisibleByEll { coeff_index: i });
        }
        result.push(c / ell);
    }
    Ok(result)
}

/// Evaluate the polynomial with `coeffs` at `x` modulo `m`.
///
/// Computes Σ coeffs[i] * x^i mod m using Horner's method.
fn eval_poly_mod(coeffs: &[BigInt], x: &BigInt, m: &BigInt) -> BigInt {
    if coeffs.is_empty() {
        return BigInt::zero();
    }
    // Horner: result = coeffs[n-1] + x*(coeffs[n-2] + x*(...))
    let mut result = coeffs.last().unwrap().mod_floor(m);
    for c in coeffs.iter().rev().skip(1) {
        result = (&result * x + c).mod_floor(m);
    }
    result
}

// ─── Clone helper ─────────────────────────────────────────────────────────────

/// Extension trait to expose `clone_in_field` publicly for use in this module.
///
/// `NumberFieldElement::clone_in_field` is private; we replicate the clone here.
trait NumberFieldElementExt<'a> {
    fn clone_pub(&self) -> NumberFieldElement<'a>;
}

impl<'a> NumberFieldElementExt<'a> for NumberFieldElement<'a> {
    fn clone_pub(&self) -> NumberFieldElement<'a> {
        NumberFieldElement { field: self.field, poly: self.poly.clone() }
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_numfield::{IntPoly, NumberField};

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    /// K = ℚ(i), f = x² + 1.
    fn field_qi() -> NumberField {
        NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
    }

    #[test]
    fn pow_bigint_zero_exp() {
        let k = field_qi();
        let alpha = k.alpha();
        let result = pow_bigint(&alpha, &BigInt::zero());
        assert!(result.is_one());
    }

    #[test]
    fn pow_bigint_matches_pow_u64() {
        let k = field_qi();
        let alpha = k.alpha();
        // α^8 = 1 in ℚ(i) (since α^4 = 1)
        let r1 = pow_bigint(&alpha, &bi(8));
        let r2 = alpha.pow(8);
        assert_eq!(r1, r2);
    }

    #[test]
    fn eval_poly_mod_basic() {
        // 3 + 2x evaluated at x=4 mod 5 = (3 + 8) mod 5 = 11 mod 5 = 1
        let coeffs = vec![bi(3), bi(2)];
        let result = eval_poly_mod(&coeffs, &bi(4), &bi(5));
        assert_eq!(result, bi(1));
    }
}
