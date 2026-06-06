//! Polynomial types over ℤ and ℚ.
//!
//! `IntPoly` — polynomial over ℤ with `BigInt` coefficients.
//! `RatPoly` — polynomial over ℚ with `BigRational` coefficients.
//!
//! Coefficients are stored least-significant first: `coeffs[i]` is the coefficient of `x^i`.
//! Invariant: trailing zeros are always trimmed.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

// ─── helpers ─────────────────────────────────────────────────────────────────

fn trim_bigint(v: &mut Vec<BigInt>) {
    while v.last().map_or(false, |c| c.is_zero()) {
        v.pop();
    }
}

fn trim_bigrational(v: &mut Vec<BigRational>) {
    while v.last().map_or(false, |c| c.is_zero()) {
        v.pop();
    }
}

// ─── IntPoly ─────────────────────────────────────────────────────────────────

/// Polynomial over ℤ with `BigInt` coefficients.
///
/// Coefficients stored least-significant first: `coeffs[i]` is the coefficient of `x^i`.
/// Invariant: trailing zeros are trimmed; the zero polynomial has `coeffs == []`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntPoly {
    pub coeffs: Vec<BigInt>,
}

impl IntPoly {
    /// The zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    /// The constant polynomial 1.
    pub fn one() -> Self {
        Self { coeffs: vec![BigInt::one()] }
    }

    /// Construct from a coefficient vector, trimming trailing zeros.
    pub fn from_coeffs(mut coeffs: Vec<BigInt>) -> Self {
        trim_bigint(&mut coeffs);
        Self { coeffs }
    }

    /// Degree of the polynomial, or `None` for the zero polynomial.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() { None } else { Some(self.coeffs.len() - 1) }
    }

    /// Leading coefficient, or `None` for the zero polynomial.
    pub fn leading_coeff(&self) -> Option<&BigInt> {
        self.coeffs.last()
    }

    /// Evaluate the polynomial at `x` using Horner's method.
    pub fn eval(&self, x: &BigInt) -> BigInt {
        let mut result = BigInt::zero();
        for c in self.coeffs.iter().rev() {
            result = result * x + c;
        }
        result
    }

    /// True iff the polynomial is monic (leading coefficient is 1).
    pub fn is_monic(&self) -> bool {
        self.leading_coeff().map_or(false, |c| c.is_one())
    }

    /// Add two polynomials.
    pub fn add(&self, rhs: &Self) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(BigInt::zero);
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(BigInt::zero);
            coeffs.push(a + b);
        }
        Self::from_coeffs(coeffs)
    }

    /// Subtract `rhs` from `self`.
    pub fn sub(&self, rhs: &Self) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(BigInt::zero);
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(BigInt::zero);
            coeffs.push(a - b);
        }
        Self::from_coeffs(coeffs)
    }

    /// Negate the polynomial.
    pub fn neg(&self) -> Self {
        Self::from_coeffs(self.coeffs.iter().map(|c| -c).collect())
    }

    /// Multiply two polynomials.
    pub fn mul(&self, rhs: &Self) -> Self {
        if self.coeffs.is_empty() || rhs.coeffs.is_empty() {
            return Self::zero();
        }
        let n = self.coeffs.len() + rhs.coeffs.len() - 1;
        let mut coeffs = vec![BigInt::zero(); n];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }
        Self::from_coeffs(coeffs)
    }

    /// Multiply all coefficients by the scalar `c`.
    pub fn scale(&self, c: &BigInt) -> Self {
        Self::from_coeffs(self.coeffs.iter().map(|a| a * c).collect())
    }

    /// Embed into ℚ[x] by converting each coefficient to a rational.
    pub fn to_rat_poly(&self) -> RatPoly {
        RatPoly::from_coeffs(
            self.coeffs.iter().map(|c| BigRational::from(c.clone())).collect(),
        )
    }
}

// ─── RatPoly ─────────────────────────────────────────────────────────────────

/// Polynomial over ℚ with `BigRational` coefficients.
///
/// Same storage convention as `IntPoly`: `coeffs[i]` is the coefficient of `x^i`.
/// Invariant: trailing zeros are trimmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatPoly {
    pub coeffs: Vec<BigRational>,
}

impl RatPoly {
    /// The zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: vec![] }
    }

    /// The constant polynomial 1.
    pub fn one() -> Self {
        Self { coeffs: vec![BigRational::one()] }
    }

    /// Construct from a coefficient vector, trimming trailing zeros.
    pub fn from_coeffs(mut coeffs: Vec<BigRational>) -> Self {
        trim_bigrational(&mut coeffs);
        Self { coeffs }
    }

    /// Degree of the polynomial, or `None` for the zero polynomial.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() { None } else { Some(self.coeffs.len() - 1) }
    }

    /// Leading coefficient, or `None` for the zero polynomial.
    pub fn leading_coeff(&self) -> Option<&BigRational> {
        self.coeffs.last()
    }

    /// Evaluate the polynomial at `x` using Horner's method.
    pub fn eval(&self, x: &BigRational) -> BigRational {
        let mut result = BigRational::zero();
        for c in self.coeffs.iter().rev() {
            result = result * x + c;
        }
        result
    }

    /// True iff the polynomial is monic (leading coefficient is 1).
    pub fn is_monic(&self) -> bool {
        self.leading_coeff().map_or(false, |c| c.is_one())
    }

    /// Add two polynomials.
    pub fn add(&self, rhs: &Self) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(BigRational::zero);
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(BigRational::zero);
            coeffs.push(a + b);
        }
        Self::from_coeffs(coeffs)
    }

    /// Subtract `rhs` from `self`.
    pub fn sub(&self, rhs: &Self) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(BigRational::zero);
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(BigRational::zero);
            coeffs.push(a - b);
        }
        Self::from_coeffs(coeffs)
    }

    /// Negate the polynomial.
    pub fn neg(&self) -> Self {
        Self::from_coeffs(self.coeffs.iter().map(|c| -c).collect())
    }

    /// Multiply two polynomials.
    pub fn mul(&self, rhs: &Self) -> Self {
        if self.coeffs.is_empty() || rhs.coeffs.is_empty() {
            return Self::zero();
        }
        let n = self.coeffs.len() + rhs.coeffs.len() - 1;
        let mut coeffs = vec![BigRational::zero(); n];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }
        Self::from_coeffs(coeffs)
    }

    /// Multiply all coefficients by the scalar `c`.
    pub fn scale(&self, c: &BigRational) -> Self {
        Self::from_coeffs(self.coeffs.iter().map(|a| a * c).collect())
    }

    /// Polynomial long division over ℚ: returns `(quotient, remainder)` such that
    /// `self = quotient * divisor + remainder` and `deg(remainder) < deg(divisor)`.
    ///
    /// Panics if `divisor` is the zero polynomial.
    pub fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        let divisor_deg = divisor.degree().expect("division by zero polynomial");
        let lc_inv = {
            let lc = divisor.leading_coeff().unwrap();
            BigRational::new(lc.denom().clone(), lc.numer().clone())
        };

        let mut remainder = self.clone();
        let mut quotient_coeffs = vec![];

        loop {
            let rem_deg = match remainder.degree() {
                None => break,
                Some(d) => d,
            };
            if rem_deg < divisor_deg {
                break;
            }
            // Leading term of remainder / leading term of divisor
            let shift = rem_deg - divisor_deg;
            let factor = remainder.leading_coeff().unwrap().clone() * &lc_inv;

            // Ensure quotient_coeffs is large enough
            if quotient_coeffs.len() <= shift {
                quotient_coeffs.resize(shift + 1, BigRational::zero());
            }
            quotient_coeffs[shift] = factor.clone();

            // Subtract factor * x^shift * divisor from remainder
            for (i, c) in divisor.coeffs.iter().enumerate() {
                let idx = i + shift;
                if idx < remainder.coeffs.len() {
                    remainder.coeffs[idx] -= &factor * c;
                } else {
                    // This shouldn't happen since we're subtracting a term of the same degree
                    // but handle defensively
                    while remainder.coeffs.len() <= idx {
                        remainder.coeffs.push(BigRational::zero());
                    }
                    remainder.coeffs[idx] -= &factor * c;
                }
            }
            trim_bigrational(&mut remainder.coeffs);
        }

        (Self::from_coeffs(quotient_coeffs), remainder)
    }

    /// Compute `self mod divisor` (the remainder of polynomial division).
    ///
    /// Panics if `divisor` is the zero polynomial.
    pub fn rem(&self, divisor: &Self) -> Self {
        self.div_rem(divisor).1
    }
}

// ─── unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    fn br(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    fn bri(n: i64) -> BigRational {
        BigRational::from(BigInt::from(n))
    }

    // ── IntPoly ──

    #[test]
    fn int_poly_zero_degree() {
        assert_eq!(IntPoly::zero().degree(), None);
        assert_eq!(IntPoly::one().degree(), Some(0));
    }

    #[test]
    fn int_poly_from_coeffs_trims() {
        // [1, 0, 0] should trim to [1]
        let p = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(0)]);
        assert_eq!(p.degree(), Some(0));
        assert_eq!(p.coeffs.len(), 1);
    }

    #[test]
    fn int_poly_add() {
        // (1 + 2x) + (3 + 4x + 5x²) = 4 + 6x + 5x²
        let a = IntPoly::from_coeffs(vec![bi(1), bi(2)]);
        let b = IntPoly::from_coeffs(vec![bi(3), bi(4), bi(5)]);
        let c = a.add(&b);
        assert_eq!(c.coeffs, vec![bi(4), bi(6), bi(5)]);
    }

    #[test]
    fn int_poly_sub() {
        // (3 + 4x + 5x²) - (1 + 2x) = 2 + 2x + 5x²
        let a = IntPoly::from_coeffs(vec![bi(3), bi(4), bi(5)]);
        let b = IntPoly::from_coeffs(vec![bi(1), bi(2)]);
        let c = a.sub(&b);
        assert_eq!(c.coeffs, vec![bi(2), bi(2), bi(5)]);
    }

    #[test]
    fn int_poly_mul() {
        // (1 + x)(1 - x) = 1 - x²
        let a = IntPoly::from_coeffs(vec![bi(1), bi(1)]);
        let b = IntPoly::from_coeffs(vec![bi(1), bi(-1)]);
        let c = a.mul(&b);
        assert_eq!(c.coeffs, vec![bi(1), bi(0), bi(-1)]);
        // After trim: [1, 0, -1] — the middle zero is NOT trailing, so it stays
        assert_eq!(c.degree(), Some(2));
    }

    #[test]
    fn int_poly_eval() {
        // f(x) = 1 + 2x + 3x², f(2) = 1 + 4 + 12 = 17
        let f = IntPoly::from_coeffs(vec![bi(1), bi(2), bi(3)]);
        assert_eq!(f.eval(&bi(2)), bi(17));
    }

    #[test]
    fn int_poly_is_monic() {
        let f = IntPoly::from_coeffs(vec![bi(-1), bi(0), bi(1)]);
        assert!(f.is_monic());
        let g = IntPoly::from_coeffs(vec![bi(1), bi(2)]);
        assert!(!g.is_monic());
    }

    #[test]
    fn int_poly_to_rat_poly() {
        let f = IntPoly::from_coeffs(vec![bi(1), bi(2)]);
        let g = f.to_rat_poly();
        assert_eq!(g.coeffs, vec![bri(1), bri(2)]);
    }

    // ── RatPoly ──

    #[test]
    fn rat_poly_div_rem_exact() {
        // (x² - 1) / (x - 1) = x + 1, remainder 0
        let dividend = RatPoly::from_coeffs(vec![bri(-1), bri(0), bri(1)]);
        let divisor = RatPoly::from_coeffs(vec![bri(-1), bri(1)]);
        let (q, r) = dividend.div_rem(&divisor);
        assert_eq!(q.coeffs, vec![bri(1), bri(1)]);
        assert_eq!(r, RatPoly::zero());
    }

    #[test]
    fn rat_poly_div_rem_with_remainder() {
        // x² / (x + 1) = x - 1, remainder 1
        // x² = (x - 1)(x + 1) + 1
        let dividend = RatPoly::from_coeffs(vec![bri(0), bri(0), bri(1)]);
        let divisor = RatPoly::from_coeffs(vec![bri(1), bri(1)]);
        let (q, r) = dividend.div_rem(&divisor);
        // q = x - 1, r = 1
        assert_eq!(q.coeffs, vec![bri(-1), bri(1)]);
        assert_eq!(r.coeffs, vec![bri(1)]);
    }

    #[test]
    fn rat_poly_div_rem_rational_coeffs() {
        // (1/2 x² + x) / (x + 2) = 1/2 x, remainder 0
        // (1/2 x)(x + 2) = 1/2 x² + x ✓
        let dividend = RatPoly::from_coeffs(vec![bri(0), bri(1), br(1, 2)]);
        let divisor = RatPoly::from_coeffs(vec![bri(2), bri(1)]);
        let (q, r) = dividend.div_rem(&divisor);
        assert_eq!(q.coeffs, vec![bri(0), br(1, 2)]);
        assert_eq!(r, RatPoly::zero());
    }

    #[test]
    fn rat_poly_mul_then_rem() {
        // (x² + 1) mod (x² - 2) = 3  (since x² ≡ 2, so x² + 1 ≡ 3)
        let p = RatPoly::from_coeffs(vec![bri(1), bri(0), bri(1)]);
        let f = RatPoly::from_coeffs(vec![bri(-2), bri(0), bri(1)]);
        let r = p.rem(&f);
        assert_eq!(r.coeffs, vec![bri(3)]);
    }
}
