//! Free polynomial ring GF(2^m)[x] over a GF(2^m) field element type.
//!
//! [`Poly<F, L>`] is a polynomial with coefficients in GF(2^m), stored as a
//! `Vec<F>` where index `i` holds the coefficient of `x^i`.  The representation
//! is *normalized*: trailing zero coefficients are always dropped so that
//! `degree()` is exact and the zero polynomial is the empty vector.
//!
//! # Per-call field-modulus idiom
//!
//! Every operation that requires field arithmetic (i.e. anything beyond `add`,
//! which is pure XOR) takes `poly: &Uint<L>` — the GF(2^m) irreducible — as an
//! explicit parameter.  This mirrors the `F2m` trait's own per-call convention
//! and the `BinaryCurve` / `HyperellipticCurve` pattern: the field modulus is
//! never stored on the struct.
//!
//! # Characteristic-2 invariants
//!
//! - **`add` is XOR**: coefficient-wise field `add` (= XOR) with no modulus
//!   needed.
//! - **Formal derivative kills even-degree terms**: `d/dx(xⁿ) = n·xⁿ⁻¹` where
//!   `n` reduces mod 2 in characteristic 2.  So `(x²)' = 0`, `(x⁴)' = 0`,
//!   `(x³)' = x²`.  This is load-bearing for square-free / resultant work.
//! - **`divmod` inverts the leading coefficient**: GF(2^m) is a field, so
//!   division is total for any nonzero divisor, but the divisor's leading
//!   coefficient must be inverted via `F2m::inv`.
//!
//! # Const-generic design
//!
//! `Poly<F, L>` carries the limb count `L` as a const generic, mirroring
//! `F2mNaive<L>`.  This is required because `F2m<L>` is parameterised by `L`
//! and the per-call `poly: &Uint<L>` parameter must be typed.
//!
//! # Toy field sizes
//!
//! KATs use GF(2^4) with `x⁴+x+1` (poly = 0x13), mirroring the binary-curve
//! KATs.  The algorithms are not toy — they are correct for arbitrary `m` and
//! arbitrary polynomial degree — but the parameters are small for auditability
//! (principle-4 boundary).

use crypto_bigint::Uint;

use crate::F2m;

// ── Type definition ───────────────────────────────────────────────────────────

/// A polynomial over GF(2^m) stored as a coefficient vector.
///
/// `coeffs[i]` is the coefficient of `x^i`.  The vector is always normalized:
/// trailing zero coefficients are dropped.  The zero polynomial is represented
/// as an empty vector.
///
/// `L` is the number of 64-bit limbs in the underlying `Uint<L>` (the same `L`
/// as in `F2m<L>`).  The GF(2^m) irreducible polynomial is NOT stored here —
/// it is passed per-call as `poly: &Uint<L>`, mirroring the `F2m` trait
/// convention.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly<F, const L: usize> {
    /// Coefficient vector: `coeffs[i]` = coefficient of `x^i`.
    ///
    /// Invariant: no trailing zeros (i.e. `coeffs.last()` is either absent or
    /// non-zero).
    coeffs: Vec<F>,
}

// ── Constructors ──────────────────────────────────────────────────────────────

impl<F: F2m<L>, const L: usize> Poly<F, L> {
    /// Construct the zero polynomial.
    #[must_use]
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// Construct the polynomial `1` (multiplicative identity).
    #[must_use]
    pub fn one() -> Self {
        Self { coeffs: vec![F::one()] }
    }

    /// Construct from a coefficient vector, normalizing trailing zeros.
    ///
    /// `coeffs[i]` is the coefficient of `x^i`.
    #[must_use]
    pub fn from_coeffs(coeffs: Vec<F>) -> Self {
        let mut p = Self { coeffs };
        p.normalize();
        p
    }

    /// Construct the monomial `c · xⁿ`.
    #[must_use]
    pub fn monomial(n: usize, c: F) -> Self {
        if c.is_zero() {
            return Self::zero();
        }
        let mut coeffs = vec![F::zero(); n + 1];
        coeffs[n] = c;
        Self { coeffs }
    }

    /// Return a reference to the coefficient vector.
    #[must_use]
    pub fn coeffs(&self) -> &[F] {
        &self.coeffs
    }

    /// Drop trailing zero coefficients to maintain the normalization invariant.
    fn normalize(&mut self) {
        while self.coeffs.last().map_or(false, |c| c.is_zero()) {
            self.coeffs.pop();
        }
    }
}

// ── Degree and leading coefficient ───────────────────────────────────────────

impl<F: F2m<L>, const L: usize> Poly<F, L> {
    /// Return `true` if this is the zero polynomial.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Return the degree of the polynomial, or `None` for the zero polynomial.
    ///
    /// The degree of the zero polynomial is undefined; callers that need a
    /// sentinel can use `degree_or` or match on `None`.
    #[must_use]
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() { None } else { Some(self.coeffs.len() - 1) }
    }

    /// Return the degree, or `default` if the polynomial is zero.
    #[must_use]
    pub fn degree_or(&self, default: usize) -> usize {
        self.degree().unwrap_or(default)
    }

    /// Return the leading coefficient (coefficient of the highest-degree term).
    ///
    /// Returns `None` for the zero polynomial.
    #[must_use]
    pub fn leading_coeff(&self) -> Option<&F> {
        self.coeffs.last()
    }

    /// Return the coefficient of `x^i`, or zero if `i` is out of range.
    #[must_use]
    pub fn coeff(&self, i: usize) -> F {
        self.coeffs.get(i).cloned().unwrap_or_else(F::zero)
    }
}

// ── Ring operations ───────────────────────────────────────────────────────────

impl<F: F2m<L>, const L: usize> Poly<F, L> {
    /// Add two polynomials: coefficient-wise field addition (= XOR in char 2).
    ///
    /// No field modulus needed — XOR of two field elements is always in-field.
    #[must_use]
    pub fn add(&self, rhs: &Self) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeff(i);
            let b = rhs.coeff(i);
            coeffs.push(a.add(&b));
        }
        let mut result = Self { coeffs };
        result.normalize();
        result
    }

    /// Subtract two polynomials.
    ///
    /// In characteristic 2, subtraction equals addition (XOR).  This method
    /// exists for API symmetry; it delegates to `add`.
    #[must_use]
    pub fn sub(&self, rhs: &Self) -> Self {
        // sub == add in characteristic 2.
        self.add(rhs)
    }

    /// Multiply two polynomials: schoolbook convolution with field `mul`.
    ///
    /// `poly` is the GF(2^m) irreducible, needed for field coefficient
    /// multiplication.
    #[must_use]
    pub fn mul(&self, rhs: &Self, poly: &Uint<L>) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let n = self.coeffs.len() + rhs.coeffs.len() - 1;
        let mut coeffs = vec![F::zero(); n];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                let prod = a.mul(b, poly);
                coeffs[i + j] = coeffs[i + j].add(&prod);
            }
        }
        let mut result = Self { coeffs };
        result.normalize();
        result
    }

    /// Polynomial long division: divide `self` by `rhs`, returning `(quotient, remainder)`.
    ///
    /// Requires `rhs ≠ 0`.  The result satisfies `self = quotient * rhs + remainder`
    /// with `deg(remainder) < deg(rhs)` (or `remainder = 0`).
    ///
    /// The divisor's leading coefficient is inverted via `F2m::inv` — GF(2^m) is
    /// a field, so division is total for any nonzero divisor.
    ///
    /// `poly` is the GF(2^m) irreducible.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is the zero polynomial.
    #[must_use]
    pub fn divmod(&self, rhs: &Self, poly: &Uint<L>) -> (Self, Self) {
        assert!(!rhs.is_zero(), "Poly::divmod: divisor is zero");

        let deg_b = rhs.degree().expect("rhs is non-zero");

        // Invert the leading coefficient of the divisor once.
        let lc_b_inv = rhs.leading_coeff().unwrap().inv(poly);

        let mut remainder = self.clone();
        // Quotient coefficients indexed by degree.
        let mut quot_coeffs: Vec<F> = Vec::new();

        loop {
            let deg_r = match remainder.degree() {
                Some(d) => d,
                None => break, // remainder is zero
            };
            if deg_r < deg_b {
                break;
            }
            let shift = deg_r - deg_b;

            // Leading term of quotient: lc(remainder) * lc(rhs)⁻¹
            let lc_r = remainder.leading_coeff().unwrap().clone();
            let q_coeff = lc_r.mul(&lc_b_inv, poly);

            // Ensure quot_coeffs is large enough.
            if quot_coeffs.len() <= shift {
                quot_coeffs.resize(shift + 1, F::zero());
            }
            quot_coeffs[shift] = q_coeff.clone();

            // Subtract q_coeff * x^shift * rhs from remainder.
            for (i, b) in rhs.coeffs.iter().enumerate() {
                let term = q_coeff.mul(b, poly);
                let idx = i + shift;
                let cur = remainder.coeff(idx);
                let new_val = cur.add(&term);
                if idx < remainder.coeffs.len() {
                    remainder.coeffs[idx] = new_val;
                } else {
                    remainder.coeffs.resize(idx + 1, F::zero());
                    remainder.coeffs[idx] = new_val;
                }
            }
            remainder.normalize();
        }

        let mut quotient = Self { coeffs: quot_coeffs };
        quotient.normalize();
        (quotient, remainder)
    }

    /// Scale all coefficients by a field element.
    ///
    /// Returns `scalar * self`.  `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn scale(&self, scalar: &F, poly: &Uint<L>) -> Self {
        if scalar.is_zero() {
            return Self::zero();
        }
        let coeffs = self.coeffs.iter().map(|c| c.mul(scalar, poly)).collect();
        let mut result = Self { coeffs };
        result.normalize();
        result
    }

    /// Make the polynomial monic: scale so the leading coefficient is 1.
    ///
    /// Returns the zero polynomial unchanged.  `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn monic(&self, poly: &Uint<L>) -> Self {
        match self.leading_coeff() {
            None => Self::zero(), // zero polynomial
            Some(lc) => {
                if lc.is_one() {
                    self.clone()
                } else {
                    let lc_inv = lc.inv(poly);
                    self.scale(&lc_inv, poly)
                }
            }
        }
    }

    /// Formal derivative in characteristic 2.
    ///
    /// `d/dx(Σ aᵢxⁱ) = Σ i·aᵢxⁱ⁻¹` where `i·aᵢ` is `aᵢ` for odd `i` and `0`
    /// for even `i` (since `i mod 2 = 0` in characteristic 2 kills even-degree
    /// terms).
    ///
    /// **Char-2 trap**: `(x²)' = 0`, `(x⁴)' = 0`, `(x³)' = x²`.  A derivative
    /// ported from integer-coefficient polynomials will get this wrong.
    ///
    /// No field modulus needed — the derivative only selects and shifts
    /// coefficients; it does not multiply field elements.
    #[must_use]
    pub fn derivative(&self) -> Self {
        if self.coeffs.len() <= 1 {
            // Constant or zero polynomial: derivative is zero.
            return Self::zero();
        }
        // Coefficient of x^(i-1) in the derivative is i * coeffs[i].
        // In char 2, i * c = c if i is odd, 0 if i is even.
        // So we collect coeffs[i] for odd i, placed at position i-1.
        let mut coeffs: Vec<F> = Vec::with_capacity(self.coeffs.len() - 1);
        for (i, c) in self.coeffs.iter().enumerate().skip(1) {
            // i is the degree of the term; the derivative term lands at i-1.
            // In char 2: coefficient is c if i is odd, zero if i is even.
            let deriv_coeff = if i % 2 == 1 { c.clone() } else { F::zero() };
            coeffs.push(deriv_coeff);
        }
        let mut result = Self { coeffs };
        result.normalize();
        result
    }

    // ── GCD and extended GCD ──────────────────────────────────────────────────

    /// Polynomial GCD over GF(2^m)[x] via the Euclidean algorithm.
    ///
    /// Returns a monic GCD (or the zero polynomial if both inputs are zero).
    /// `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn gcd(a: &Self, b: &Self, poly: &Uint<L>) -> Self {
        let (g, _, _) = Self::xgcd(a, b, poly);
        g
    }

    /// Extended Euclidean algorithm over GF(2^m)[x].
    ///
    /// Returns `(g, s, t)` such that `s * a + t * b = g` where `g` is the monic
    /// GCD of `a` and `b`.
    ///
    /// If both `a` and `b` are zero, returns `(0, 0, 0)`.
    ///
    /// `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn xgcd(a: &Self, b: &Self, poly: &Uint<L>) -> (Self, Self, Self) {
        // Standard extended Euclidean algorithm over a Euclidean domain.
        // Maintain: r0 = a, r1 = b; s0 = 1, s1 = 0; t0 = 0, t1 = 1.
        // Invariant: s_i * a + t_i * b = r_i.
        if a.is_zero() && b.is_zero() {
            return (Self::zero(), Self::zero(), Self::zero());
        }

        let mut r0 = a.clone();
        let mut r1 = b.clone();
        let mut s0 = Self::one();
        let mut s1 = Self::zero();
        let mut t0 = Self::zero();
        let mut t1 = Self::one();

        while !r1.is_zero() {
            let (q, r) = r0.divmod(&r1, poly);

            // (r0, r1) ← (r1, r0 mod r1)
            r0 = r1;
            r1 = r;

            // (s0, s1) ← (s1, s0 - q * s1)  [sub == add in char 2]
            let qs = q.mul(&s1, poly);
            let new_s = s0.add(&qs);
            s0 = s1;
            s1 = new_s;

            // (t0, t1) ← (t1, t0 - q * t1)
            let qt = q.mul(&t1, poly);
            let new_t = t0.add(&qt);
            t0 = t1;
            t1 = new_t;
        }

        // r0 is the GCD (possibly non-monic); normalize to monic.
        if r0.is_zero() {
            return (Self::zero(), Self::zero(), Self::zero());
        }
        let lc = r0.leading_coeff().unwrap().clone();
        if lc.is_one() {
            return (r0, s0, t0);
        }
        // Scale g, s, t by lc⁻¹ so that g is monic.
        let lc_inv = lc.inv(poly);
        let g = r0.scale(&lc_inv, poly);
        let s = s0.scale(&lc_inv, poly);
        let t = t0.scale(&lc_inv, poly);
        (g, s, t)
    }

    // ── Resultant ─────────────────────────────────────────────────────────────

    /// Resultant of two polynomials over GF(2^m)[x].
    ///
    /// The resultant `res(a, b)` is a field element in GF(2^m).  It is zero iff
    /// `a` and `b` share a common root (equivalently, iff `gcd(a, b)` is
    /// non-trivial).
    ///
    /// Computed via the Euclidean algorithm with leading-coefficient tracking.
    /// In a field, pseudo-remainder sequences simplify to exact division.
    ///
    /// Returns `F::one()` if either polynomial is a nonzero constant (degree 0).
    /// Returns `F::zero()` if either polynomial is zero.
    ///
    /// `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn resultant(a: &Self, b: &Self, poly: &Uint<L>) -> F {
        // Resultant via the Euclidean algorithm (field case).
        //
        // Uses the identity (in char 2, sign factor = 1):
        //   res(a, b) = lc(b)^(deg_a - deg_r) * res(b, r)
        // where r = a mod b.
        if a.is_zero() || b.is_zero() {
            return F::zero();
        }

        let mut a = a.clone();
        let mut b = b.clone();
        let mut acc = F::one(); // accumulates the resultant scalar

        loop {
            let deg_a = match a.degree() {
                Some(d) => d,
                None => return F::zero(),
            };
            let deg_b = match b.degree() {
                Some(d) => d,
                None => return F::zero(),
            };

            if deg_a == 0 {
                // res(c, b) = c^deg(b) for constant c.
                let c = a.coeffs[0].clone();
                let exp_uint = Uint::<L>::from(deg_b as u64);
                let c_pow = c.pow(&exp_uint, poly);
                return acc.mul(&c_pow, poly);
            }

            if deg_b == 0 {
                // res(a, c) = c^deg(a) for constant c.
                let c = b.coeffs[0].clone();
                let exp_uint = Uint::<L>::from(deg_a as u64);
                let c_pow = c.pow(&exp_uint, poly);
                return acc.mul(&c_pow, poly);
            }

            // In char 2, the sign factor (-1)^(deg_a * deg_b) = 1.
            // Accumulate: acc *= lc(b)^(deg_a - deg_r).
            let lc_b = b.leading_coeff().unwrap().clone();
            let (_, r) = a.divmod(&b, poly);

            if r.is_zero() {
                // b divides a exactly.  If deg_b >= 1, then gcd(a, b) has degree
                // >= 1, so the resultant is 0.
                return F::zero();
            }

            let deg_r = r.degree().unwrap(); // r is non-zero
            // Exponent = deg_a - deg_r.
            let exp_val = deg_a - deg_r;
            let exp_uint = Uint::<L>::from(exp_val as u64);
            let lc_b_pow = lc_b.pow(&exp_uint, poly);
            acc = acc.mul(&lc_b_pow, poly);

            a = b;
            b = r;
        }
    }

    // ── Polynomial modular inverse ────────────────────────────────────────────

    /// Compute the modular inverse of `self` modulo `modulus` in GF(2^m)[x].
    ///
    /// Returns `Some(s)` such that `s * self ≡ 1 (mod modulus)` if
    /// `gcd(self, modulus) = 1`, or `None` if the GCD is non-trivial (no
    /// inverse exists).
    ///
    /// `poly` is the GF(2^m) irreducible.
    #[must_use]
    pub fn mod_inverse(&self, modulus: &Self, poly: &Uint<L>) -> Option<Self> {
        let (g, s, _t) = Self::xgcd(self, modulus, poly);
        if g.degree() == Some(0) && g.coeffs[0].is_one() {
            // gcd = 1: s is the inverse.
            // Reduce s mod modulus to get the canonical representative.
            let (_, s_reduced) = s.divmod(modulus, poly);
            Some(s_reduced)
        } else {
            None
        }
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naive::F2mNaive;

    type F = F2mNaive<1>;
    type P = Poly<F, 1>;

    fn fp() -> Uint<1> {
        Uint::<1>::from(0x13u64) // GF(2^4): x⁴+x+1
    }

    fn f(v: u64) -> F {
        F::from_u64(v, &fp())
    }

    fn poly(coeffs: &[u64]) -> P {
        P::from_coeffs(coeffs.iter().map(|&v| f(v)).collect())
    }

    // ── Normalization ─────────────────────────────────────────────────────────

    #[test]
    fn trailing_zeros_dropped() {
        // [1, 0, 0] should normalize to [1] (degree 0).
        let p = P::from_coeffs(vec![f(1), f(0), f(0)]);
        assert_eq!(p.degree(), Some(0));
        assert_eq!(p.coeffs.len(), 1);
    }

    #[test]
    fn zero_poly_is_empty() {
        let p = P::from_coeffs(vec![f(0), f(0)]);
        assert!(p.is_zero());
        assert_eq!(p.degree(), None);
    }

    // ── Degree ────────────────────────────────────────────────────────────────

    #[test]
    fn degree_of_constant() {
        assert_eq!(poly(&[3]).degree(), Some(0));
    }

    #[test]
    fn degree_of_linear() {
        assert_eq!(poly(&[0, 1]).degree(), Some(1));
    }

    // ── Add ───────────────────────────────────────────────────────────────────

    #[test]
    fn add_is_xor_coeffwise() {
        // (x + 1) + (x + 1) = 0  (char-2: a + a = 0)
        let a = poly(&[1, 1]);
        assert_eq!(a.add(&a), P::zero());
    }

    #[test]
    fn add_different_degrees() {
        // (x² + 1) + (x + 1) = x² + x
        let a = poly(&[1, 0, 1]); // x² + 1
        let b = poly(&[1, 1]); // x + 1
        let c = a.add(&b);
        assert_eq!(c, poly(&[0, 1, 1])); // x² + x
    }

    // ── Mul ───────────────────────────────────────────────────────────────────

    #[test]
    fn mul_by_one() {
        let a = poly(&[1, 1, 1]); // x² + x + 1
        let one = P::one();
        assert_eq!(a.mul(&one, &fp()), a);
    }

    #[test]
    fn mul_by_zero() {
        let a = poly(&[1, 1, 1]);
        assert_eq!(a.mul(&P::zero(), &fp()), P::zero());
    }

    #[test]
    fn mul_linear_linear() {
        // (x + 1) * (x + 1) = x² + 1  (in char 2: (x+1)² = x²+1)
        let a = poly(&[1, 1]);
        let prod = a.mul(&a, &fp());
        // x² + 2x + 1 = x² + 1 in char 2 (2x = 0)
        assert_eq!(prod, poly(&[1, 0, 1]));
    }

    // ── Divmod ────────────────────────────────────────────────────────────────

    #[test]
    fn divmod_exact() {
        // (x² + 1) / (x + 1) in GF(2^4)[x].
        // (x+1)*(x+1) = x²+1, so quotient = x+1, remainder = 0.
        let a = poly(&[1, 0, 1]); // x² + 1
        let b = poly(&[1, 1]); // x + 1
        let (q, r) = a.divmod(&b, &fp());
        assert_eq!(r, P::zero());
        // Verify: q * b = a.
        assert_eq!(q.mul(&b, &fp()), a);
    }

    #[test]
    fn divmod_with_remainder() {
        // x² / (x + 1): verify round-trip a = q*b + r.
        let a = poly(&[0, 0, 1]); // x²
        let b = poly(&[1, 1]); // x + 1
        let (q, r) = a.divmod(&b, &fp());
        // Verify round-trip: q * b + r = a.
        let reconstructed = q.mul(&b, &fp()).add(&r);
        assert_eq!(reconstructed, a);
        assert!(r.degree().map_or(true, |d| d < b.degree().unwrap()));
    }

    #[test]
    fn divmod_degree_zero_divisor() {
        // Dividing by a nonzero constant: quotient = a * lc⁻¹, remainder = 0.
        let a = poly(&[1, 1, 1]); // x² + x + 1
        let b = poly(&[2]); // constant 2 in GF(2^4)
        let (q, r) = a.divmod(&b, &fp());
        assert_eq!(r, P::zero());
        assert_eq!(q.mul(&b, &fp()), a);
    }

    // ── Monic ─────────────────────────────────────────────────────────────────

    #[test]
    fn monic_already_monic() {
        let a = poly(&[1, 0, 1]); // x² + 1, leading coeff = 1
        assert_eq!(a.monic(&fp()), a);
    }

    #[test]
    fn monic_scales_correctly() {
        // Polynomial with leading coeff 2: monic should scale by inv(2).
        let a = poly(&[1, 0, 2]); // 2x² + 1
        let m = a.monic(&fp());
        assert_eq!(m.leading_coeff().unwrap(), &f(1));
        // Verify: m * 2 = a (up to normalization).
        assert_eq!(m.scale(&f(2), &fp()), a);
    }

    // ── Derivative ────────────────────────────────────────────────────────────

    #[test]
    fn derivative_of_constant_is_zero() {
        assert_eq!(poly(&[5]).derivative(), P::zero());
    }

    #[test]
    fn derivative_of_x_is_one() {
        // (x)' = 1
        assert_eq!(poly(&[0, 1]).derivative(), P::one());
    }

    #[test]
    fn derivative_x_squared_is_zero() {
        // (x²)' = 0  — the char-2 trap: even-degree terms vanish.
        assert_eq!(poly(&[0, 0, 1]).derivative(), P::zero());
    }

    #[test]
    fn derivative_x_cubed_is_x_squared() {
        // (x³)' = x²  — odd degree survives.
        assert_eq!(poly(&[0, 0, 0, 1]).derivative(), poly(&[0, 0, 1]));
    }

    #[test]
    fn derivative_x4_is_zero() {
        // (x⁴)' = 0  — even degree.
        assert_eq!(poly(&[0, 0, 0, 0, 1]).derivative(), P::zero());
    }

    #[test]
    fn derivative_mixed() {
        // (x³ + x² + x + 1)' = x² + 1  (odd terms: x³→x², x→1; even terms: x²→0, 1→0)
        let a = poly(&[1, 1, 1, 1]); // x³ + x² + x + 1
        let da = a.derivative();
        // x² + 1 = coeffs [1, 0, 1]
        assert_eq!(da, poly(&[1, 0, 1]));
    }

    // ── GCD ───────────────────────────────────────────────────────────────────

    #[test]
    fn gcd_coprime_is_one() {
        // x and x+1 are coprime in GF(2^4)[x].
        let a = poly(&[0, 1]); // x
        let b = poly(&[1, 1]); // x + 1
        let g = P::gcd(&a, &b, &fp());
        assert_eq!(g, P::one());
    }

    #[test]
    fn gcd_with_common_factor() {
        // gcd(x*(x+1), (x+1)*(x+1)) = x+1.
        let xp1 = poly(&[1, 1]); // x + 1
        let x = poly(&[0, 1]); // x
        let a = x.mul(&xp1, &fp()); // x*(x+1) = x²+x
        let b = xp1.mul(&xp1, &fp()); // (x+1)² = x²+1
        let g = P::gcd(&a, &b, &fp());
        // g should be monic and equal x+1.
        assert_eq!(g, xp1);
    }

    #[test]
    fn gcd_self_is_monic_self() {
        let a = poly(&[1, 1, 1]); // x² + x + 1
        let g = P::gcd(&a, &a, &fp());
        assert_eq!(g, a.monic(&fp()));
    }

    // ── XGCD ─────────────────────────────────────────────────────────────────

    #[test]
    fn xgcd_bezout_identity() {
        // s*a + t*b = gcd(a, b).
        let a = poly(&[1, 0, 1]); // x² + 1
        let b = poly(&[1, 1]); // x + 1
        let (g, s, t) = P::xgcd(&a, &b, &fp());
        let lhs = s.mul(&a, &fp()).add(&t.mul(&b, &fp()));
        assert_eq!(lhs, g, "Bézout identity s*a + t*b = gcd failed");
    }

    #[test]
    fn xgcd_coprime_gcd_is_one() {
        let a = poly(&[0, 1]); // x
        let b = poly(&[1, 1]); // x + 1
        let (g, s, t) = P::xgcd(&a, &b, &fp());
        assert_eq!(g, P::one());
        let lhs = s.mul(&a, &fp()).add(&t.mul(&b, &fp()));
        assert_eq!(lhs, P::one());
    }

    // ── Resultant ─────────────────────────────────────────────────────────────

    #[test]
    fn resultant_coprime_is_nonzero() {
        // gcd(x, x+1) = 1, so res(x, x+1) ≠ 0.
        let a = poly(&[0, 1]); // x
        let b = poly(&[1, 1]); // x + 1
        let r = P::resultant(&a, &b, &fp());
        assert!(!r.is_zero(), "res(x, x+1) should be nonzero");
    }

    #[test]
    fn resultant_common_factor_is_zero() {
        // gcd(x*(x+1), (x+1)²) = x+1 ≠ 1, so resultant = 0.
        let xp1 = poly(&[1, 1]);
        let x = poly(&[0, 1]);
        let a = x.mul(&xp1, &fp()); // x*(x+1)
        let b = xp1.mul(&xp1, &fp()); // (x+1)²
        let r = P::resultant(&a, &b, &fp());
        assert!(r.is_zero(), "res(a,b) should be 0 when gcd is nontrivial");
    }

    // ── Modular inverse ───────────────────────────────────────────────────────

    #[test]
    fn mod_inverse_exists() {
        // x and x²+x+1 are coprime (x²+x+1 is irreducible over GF(2)).
        let a = poly(&[0, 1]); // x
        let m = poly(&[1, 1, 1]); // x² + x + 1
        let inv = a.mod_inverse(&m, &fp());
        assert!(inv.is_some(), "mod_inverse should exist for coprime polynomials");
        let inv = inv.unwrap();
        // Verify: inv * a ≡ 1 (mod m).
        let prod = inv.mul(&a, &fp());
        let (_, r) = prod.divmod(&m, &fp());
        assert_eq!(r, P::one(), "inv * a mod m should be 1");
    }

    #[test]
    fn mod_inverse_none_for_common_factor() {
        // gcd(x+1, x²+1) = x+1 ≠ 1, so no inverse.
        // x²+1 = (x+1)² in char 2.
        let a = poly(&[1, 1]); // x + 1
        let m = poly(&[1, 0, 1]); // x² + 1 = (x+1)²
        let inv = a.mod_inverse(&m, &fp());
        assert!(inv.is_none(), "mod_inverse should not exist when gcd is nontrivial");
    }
}
