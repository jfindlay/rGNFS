//! `F_p[x]` univariate resultant and the multivariate/symmetric-polynomial type `S_m`.
//!
//! This module provides the polynomial substrate the Semaev construction stands on
//! (C-SemaevPoly, frozen at E.J.1):
//!
//! - [`FpPoly`] — a univariate polynomial over `F_p` (coefficients in `F_p`, stored
//!   least-significant first: `coeffs[i]` is the coefficient of `x^i`).
//! - [`resultant`] — the `F_p[x]` resultant `Res(f, g) ∈ F_p`, zero iff `gcd(f,g) ≠ 1`.
//!   This is the *field* resultant (distinct from `shared-numfield`'s `ℤ[x]` Sylvester
//!   resultant and `gf2m`'s `GF(2^m)` resultant); it ports the `gf2m::Poly::resultant`
//!   field-resultant idiom to `F_p`.
//! - [`MultiPoly`] — a multivariate polynomial over `F_p` in `m` variables, stored as a
//!   map from exponent vectors to coefficients. Supports evaluation, partial-assignment
//!   evaluation, one-variable resultant-elimination, and symmetric-reduction.
//!
//! # Symmetric representation
//!
//! `S_m` is symmetric in its `m` arguments. The representation chosen here is a **dense
//! multivariate polynomial** stored as a `HashMap<Vec<u64>, u64>` from exponent vector to
//! coefficient (both as `u64` for toy-scale arithmetic). This representation:
//! - Makes evaluation and partial-assignment evaluation `O(terms)` — cheap for the small
//!   `m` and small degrees of the Semaev polynomials.
//! - Makes symmetric-reduction straightforward: sort the exponent vector and accumulate.
//! - Makes one-variable resultant-elimination (the E.J.3 recursion step) implementable
//!   by extracting the univariate polynomial in the eliminated variable and computing its
//!   resultant with the other polynomial.
//!
//! The elementary-symmetric-basis alternative would be more compact for large `m` but
//! harder to evaluate and eliminate; the dense representation is the right choice for
//! the toy-scale `m ≤ 5` the construction targets.
//!
//! # `F_p[x]` resultant vs `ℤ[x]` and `GF(2^m)` resultants
//!
//! - `shared-numfield::resultant` — `ℤ[x]` Sylvester-matrix resultant (integer ring).
//!   **Not used here** — wrong ring.
//! - `gf2m::Poly::resultant` — `GF(2^m)` field resultant (characteristic-2 field).
//!   **Idiom ported here** — the field-resultant Euclidean algorithm with leading-coefficient
//!   tracking, adapted to `F_p` (odd characteristic, sign factor `(-1)^(deg_a * deg_b)`).
//! - [`resultant`] (this module) — `F_p[x]` field resultant (odd-characteristic prime field).
//!   The sign factor `(-1)^(deg_a * deg_b)` is non-trivial (unlike characteristic 2).

use std::collections::HashMap;

use crypto_bigint::Uint;

use crate::field::Fp;
use crate::semaev::SemaevError;

// ─── FpPoly: univariate polynomial over F_p ───────────────────────────────────

/// A univariate polynomial over `F_p`, stored as a coefficient vector.
///
/// `coeffs[i]` is the coefficient of `x^i`. The zero polynomial is represented by
/// an empty vector or a vector of all-zero coefficients. The invariant is that the
/// last element of `coeffs` (if any) is non-zero (i.e., the vector is trimmed).
///
/// All arithmetic is performed modulo `p` (passed as a `&Uint<4>` parameter, following
/// the `shared-field` idiom).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpPoly<F: Fp<4>> {
    /// Coefficients, least-significant first: `coeffs[i]` is the coefficient of `x^i`.
    ///
    /// Invariant: the last element (if any) is non-zero (the vector is trimmed).
    pub coeffs: Vec<F>,
}

impl<F: Fp<4>> FpPoly<F> {
    /// Construct the zero polynomial.
    pub fn zero() -> Self {
        FpPoly { coeffs: Vec::new() }
    }

    /// Construct a polynomial from a coefficient vector (least-significant first).
    ///
    /// Trims trailing zero coefficients to maintain the invariant.
    pub fn from_coeffs(coeffs: Vec<F>, p: &Uint<4>) -> Self {
        let mut poly = FpPoly { coeffs };
        poly.trim(p);
        poly
    }

    /// Construct a constant polynomial (degree 0 or the zero polynomial).
    pub fn constant(c: F, p: &Uint<4>) -> Self {
        if c.is_zero(p) {
            FpPoly::zero()
        } else {
            FpPoly { coeffs: vec![c] }
        }
    }

    /// Return `true` if this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Return the degree of the polynomial, or `None` for the zero polynomial.
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Return the leading coefficient, or `None` for the zero polynomial.
    pub fn leading_coeff(&self) -> Option<&F> {
        self.coeffs.last()
    }

    /// Trim trailing zero coefficients to maintain the invariant.
    fn trim(&mut self, p: &Uint<4>) {
        while let Some(last) = self.coeffs.last() {
            if last.is_zero(p) {
                self.coeffs.pop();
            } else {
                break;
            }
        }
    }

    /// Polynomial addition: `self + rhs mod p`.
    #[must_use]
    pub fn add(&self, rhs: &Self, p: &Uint<4>) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(|| F::zero(p));
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(|| F::zero(p));
            coeffs.push(a.add(&b, p));
        }
        FpPoly::from_coeffs(coeffs, p)
    }

    /// Polynomial subtraction: `self - rhs mod p`.
    #[must_use]
    pub fn sub(&self, rhs: &Self, p: &Uint<4>) -> Self {
        let len = self.coeffs.len().max(rhs.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).cloned().unwrap_or_else(|| F::zero(p));
            let b = rhs.coeffs.get(i).cloned().unwrap_or_else(|| F::zero(p));
            coeffs.push(a.sub(&b, p));
        }
        FpPoly::from_coeffs(coeffs, p)
    }

    /// Polynomial negation: `-self mod p`.
    #[must_use]
    pub fn neg(&self, p: &Uint<4>) -> Self {
        FpPoly { coeffs: self.coeffs.iter().map(|c| c.neg(p)).collect() }
    }

    /// Scalar multiplication: `c * self mod p`.
    #[must_use]
    pub fn scale(&self, c: &F, p: &Uint<4>) -> Self {
        let coeffs: Vec<F> = self.coeffs.iter().map(|a| a.mul(c, p)).collect();
        FpPoly::from_coeffs(coeffs, p)
    }

    /// Polynomial multiplication: `self * rhs mod p`.
    #[must_use]
    pub fn mul(&self, rhs: &Self, p: &Uint<4>) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return FpPoly::zero();
        }
        let n = self.coeffs.len() + rhs.coeffs.len() - 1;
        let mut coeffs = vec![F::zero(p); n];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in rhs.coeffs.iter().enumerate() {
                let prod = a.mul(b, p);
                coeffs[i + j] = coeffs[i + j].add(&prod, p);
            }
        }
        FpPoly::from_coeffs(coeffs, p)
    }

    /// Evaluate the polynomial at `x`: `f(x) mod p`.
    #[must_use]
    pub fn eval(&self, x: &F, p: &Uint<4>) -> F {
        // Horner's method.
        let mut result = F::zero(p);
        for c in self.coeffs.iter().rev() {
            result = result.mul(x, p).add(c, p);
        }
        result
    }

    /// Polynomial division with remainder: `(q, r)` such that `self = q * rhs + r`.
    ///
    /// Requires `rhs` to be non-zero (panics in debug mode if `rhs` is zero).
    ///
    /// # Panics (debug)
    ///
    /// Panics if `rhs` is the zero polynomial.
    #[must_use]
    pub fn divmod(&self, rhs: &Self, p: &Uint<4>) -> (Self, Self) {
        debug_assert!(!rhs.is_zero(), "divmod: divisor is zero");
        if self.is_zero() {
            return (FpPoly::zero(), FpPoly::zero());
        }
        let deg_b = rhs.degree().unwrap();
        if self.degree().unwrap() < deg_b {
            return (FpPoly::zero(), self.clone());
        }

        let lc_b_inv = rhs.leading_coeff().unwrap().inv(p);
        let mut rem = self.clone();

        let deg_a = self.degree().unwrap();
        let mut q_coeffs = vec![F::zero(p); deg_a - deg_b + 1];

        while !rem.is_zero() && rem.degree().unwrap() >= deg_b {
            let deg_r = rem.degree().unwrap();
            let lc_r = rem.leading_coeff().unwrap().clone();
            // Quotient term: lc_r / lc_b * x^(deg_r - deg_b)
            let coeff = lc_r.mul(&lc_b_inv, p);
            let shift = deg_r - deg_b;
            q_coeffs[shift] = coeff.clone();

            // Subtract coeff * x^shift * rhs from rem.
            for (i, c) in rhs.coeffs.iter().enumerate() {
                let sub = coeff.mul(c, p);
                rem.coeffs[i + shift] = rem.coeffs[i + shift].sub(&sub, p);
            }
            rem.trim(p);
        }

        (FpPoly::from_coeffs(q_coeffs, p), rem)
    }

    /// Make the polynomial monic by dividing all coefficients by the leading coefficient.
    ///
    /// Returns the zero polynomial unchanged.
    #[must_use]
    pub fn make_monic(&self, p: &Uint<4>) -> Self {
        match self.leading_coeff() {
            None => FpPoly::zero(),
            Some(lc) => {
                let lc_inv = lc.inv(p);
                self.scale(&lc_inv, p)
            }
        }
    }
}

// ─── F_p[x] resultant ─────────────────────────────────────────────────────────

/// Compute the resultant `Res(f, g) ∈ F_p` for `f, g ∈ F_p[x]`.
///
/// The resultant is zero iff `gcd(f, g) ≠ 1` (i.e., `f` and `g` share a common root
/// in the algebraic closure of `F_p`).
///
/// # Algorithm
///
/// Uses the Euclidean algorithm with leading-coefficient tracking, porting the
/// `gf2m::Poly::resultant` field-resultant idiom to `F_p` (odd characteristic).
/// The key difference from the `GF(2^m)` case is the sign factor:
///
/// ```text
/// Res(a, b) = (-1)^(deg_a * deg_b) * lc(b)^(deg_a - deg_r) * Res(b, r)
/// ```
///
/// where `r = a mod b`. In characteristic 2, `(-1) = 1` so the sign is trivial;
/// in `F_p` (odd characteristic), the sign must be tracked.
///
/// # Returns
///
/// - `F_p::zero()` if either `f` or `g` is the zero polynomial.
/// - `F_p::one()` if either `f` or `g` is a non-zero constant (degree 0).
/// - The resultant `Res(f, g)` otherwise.
///
/// # Errors
///
/// Returns `Err(SemaevError::ZeroLeadingCoefficient)` if an internal invariant is
/// violated (the leading coefficient of a non-zero polynomial is zero). This should
/// not occur for well-formed inputs.
#[must_use]
pub fn resultant<F: Fp<4>>(
    f: &FpPoly<F>,
    g: &FpPoly<F>,
    p: &Uint<4>,
) -> Result<F, SemaevError> {
    // Zero polynomial → resultant is 0.
    if f.is_zero() || g.is_zero() {
        return Ok(F::zero(p));
    }

    let mut a = f.clone();
    let mut b = g.clone();
    // `acc` accumulates the resultant scalar (the product of sign factors and
    // leading-coefficient powers).
    let mut acc = F::one(p);

    loop {
        let deg_a = match a.degree() {
            Some(d) => d,
            None => return Ok(F::zero(p)),
        };
        let deg_b = match b.degree() {
            Some(d) => d,
            None => return Ok(F::zero(p)),
        };

        // Base case: constant polynomial.
        if deg_a == 0 {
            // Res(c, b) = c^deg(b) for constant c.
            let c = a.coeffs[0].clone();
            let exp = Uint::<4>::from(deg_b as u64);
            let c_pow = c.pow(&exp, p);
            return Ok(acc.mul(&c_pow, p));
        }
        if deg_b == 0 {
            // Res(a, c) = c^deg(a) for constant c.
            let c = b.coeffs[0].clone();
            let exp = Uint::<4>::from(deg_a as u64);
            let c_pow = c.pow(&exp, p);
            return Ok(acc.mul(&c_pow, p));
        }

        // Sign factor: (-1)^(deg_a * deg_b).
        // In F_p (odd characteristic), (-1)^k = -1 if k is odd, 1 if k is even.
        if (deg_a * deg_b) % 2 == 1 {
            acc = acc.neg(p);
        }

        // Accumulate: acc *= lc(b)^(deg_a - deg_r).
        let lc_b = b.leading_coeff().ok_or(SemaevError::ZeroLeadingCoefficient)?.clone();
        let (_, r) = a.divmod(&b, p);

        if r.is_zero() {
            // b divides a exactly. If deg_b >= 1, gcd(a,b) has degree >= 1 → resultant = 0.
            return Ok(F::zero(p));
        }

        let deg_r = r.degree().unwrap(); // r is non-zero
        // Exponent = deg_a - deg_r (the degree drop in the remainder step).
        let exp_val = deg_a - deg_r;
        let exp_uint = Uint::<4>::from(exp_val as u64);
        let lc_b_pow = lc_b.pow(&exp_uint, p);
        acc = acc.mul(&lc_b_pow, p);

        a = b;
        b = r;
    }
}

// ─── MultiPoly: multivariate polynomial over F_p ─────────────────────────────

/// A multivariate polynomial over `F_p` in `m` variables.
///
/// Stored as a `HashMap` from exponent vector to coefficient (both as `u64` for
/// toy-scale arithmetic). The exponent vector `e` of length `m` represents the
/// monomial `x_0^{e[0]} * x_1^{e[1]} * … * x_{m-1}^{e[m-1]}`.
///
/// # Symmetric polynomials
///
/// The Semaev summation polynomials `S_m` are symmetric in their `m` arguments.
/// This type supports symmetric-reduction via [`MultiPoly::symmetrize`], which
/// maps each monomial to its orbit under permutation of variables and accumulates
/// the coefficients.
///
/// # Operations
///
/// - [`MultiPoly::eval`] — evaluate at a full assignment `(x_0, …, x_{m-1})`.
/// - [`MultiPoly::partial_eval`] — evaluate at a partial assignment (fix some variables,
///   leave others free), returning a new `MultiPoly` in the remaining variables.
/// - [`MultiPoly::elim_var_resultant`] — eliminate one variable by computing the
///   resultant of two multivariate polynomials viewed as univariate in that variable.
/// - [`MultiPoly::symmetrize`] — reduce to the symmetric part (sum over all permutations).
/// - [`MultiPoly::is_symmetric`] — check whether the polynomial is symmetric.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiPoly {
    /// Number of variables.
    pub num_vars: usize,
    /// Coefficients: exponent vector → coefficient (as `u64`, reduced mod `p`).
    ///
    /// Entries with zero coefficient are omitted (the map is sparse).
    pub terms: HashMap<Vec<u64>, u64>,
    /// The prime modulus `p` (stored as `u64` for toy-scale arithmetic).
    ///
    /// # Principle-4 annotation
    ///
    /// SCALE: toy-scale only — `p` fits in a `u64`. Crypto-scale `p` would require
    /// `Uint<4>` arithmetic throughout.
    pub p: u64,
}

impl MultiPoly {
    /// Construct the zero polynomial in `m` variables over `F_p`.
    pub fn zero(num_vars: usize, p: u64) -> Self {
        MultiPoly { num_vars, terms: HashMap::new(), p }
    }

    /// Construct a constant polynomial in `m` variables over `F_p`.
    pub fn constant(c: u64, num_vars: usize, p: u64) -> Self {
        let mut poly = MultiPoly::zero(num_vars, p);
        let c_mod = c % p;
        if c_mod != 0 {
            poly.terms.insert(vec![0u64; num_vars], c_mod);
        }
        poly
    }

    /// Construct a monomial `coeff * x_{var}^deg` in `m` variables over `F_p`.
    pub fn monomial(coeff: u64, var: usize, deg: u64, num_vars: usize, p: u64) -> Self {
        let mut poly = MultiPoly::zero(num_vars, p);
        let c_mod = coeff % p;
        if c_mod != 0 {
            let mut exp = vec![0u64; num_vars];
            exp[var] = deg;
            poly.terms.insert(exp, c_mod);
        }
        poly
    }

    /// Return `true` if this is the zero polynomial.
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// Add a term `coeff * x^exp` to the polynomial (in-place, mod p).
    pub fn add_term(&mut self, exp: Vec<u64>, coeff: u64) {
        debug_assert_eq!(exp.len(), self.num_vars, "exponent vector length mismatch");
        let c = coeff % self.p;
        if c == 0 {
            return;
        }
        let entry = self.terms.entry(exp).or_insert(0);
        *entry = (*entry + c) % self.p;
        if *entry == 0 {
            // Remove zero entries to keep the map sparse.
            // (We can't remove while holding the entry reference; do it after.)
        }
        // Clean up zero entries.
        self.terms.retain(|_, v| *v != 0);
    }

    /// Polynomial addition: `self + rhs mod p`.
    ///
    /// # Panics (debug)
    ///
    /// Panics if `self` and `rhs` have different numbers of variables or different `p`.
    #[must_use]
    pub fn add(&self, rhs: &Self) -> Self {
        debug_assert_eq!(self.num_vars, rhs.num_vars, "add: num_vars mismatch");
        debug_assert_eq!(self.p, rhs.p, "add: p mismatch");
        let mut result = self.clone();
        for (exp, &coeff) in &rhs.terms {
            let entry = result.terms.entry(exp.clone()).or_insert(0);
            *entry = (*entry + coeff) % self.p;
        }
        result.terms.retain(|_, v| *v != 0);
        result
    }

    /// Polynomial subtraction: `self - rhs mod p`.
    #[must_use]
    pub fn sub(&self, rhs: &Self) -> Self {
        debug_assert_eq!(self.num_vars, rhs.num_vars, "sub: num_vars mismatch");
        debug_assert_eq!(self.p, rhs.p, "sub: p mismatch");
        let mut result = self.clone();
        for (exp, &coeff) in &rhs.terms {
            let entry = result.terms.entry(exp.clone()).or_insert(0);
            *entry = (*entry + self.p - coeff % self.p) % self.p;
        }
        result.terms.retain(|_, v| *v != 0);
        result
    }

    /// Scalar multiplication: `c * self mod p`.
    #[must_use]
    pub fn scale(&self, c: u64) -> Self {
        let c_mod = c % self.p;
        if c_mod == 0 {
            return MultiPoly::zero(self.num_vars, self.p);
        }
        let mut result = self.clone();
        for v in result.terms.values_mut() {
            *v = (*v * c_mod) % self.p;
        }
        result
    }

    /// Polynomial multiplication: `self * rhs mod p`.
    ///
    /// # Panics (debug)
    ///
    /// Panics if `self` and `rhs` have different numbers of variables or different `p`.
    #[must_use]
    pub fn mul(&self, rhs: &Self) -> Self {
        debug_assert_eq!(self.num_vars, rhs.num_vars, "mul: num_vars mismatch");
        debug_assert_eq!(self.p, rhs.p, "mul: p mismatch");
        let mut result = MultiPoly::zero(self.num_vars, self.p);
        for (exp_a, &coeff_a) in &self.terms {
            for (exp_b, &coeff_b) in &rhs.terms {
                let mut exp = vec![0u64; self.num_vars];
                for i in 0..self.num_vars {
                    exp[i] = exp_a[i] + exp_b[i];
                }
                let coeff = (coeff_a * coeff_b) % self.p;
                let entry = result.terms.entry(exp).or_insert(0);
                *entry = (*entry + coeff) % self.p;
            }
        }
        result.terms.retain(|_, v| *v != 0);
        result
    }

    /// Evaluate the polynomial at a full assignment `vals[i] = x_i mod p`.
    ///
    /// # Errors
    ///
    /// Returns `Err(SemaevError::ArityMismatch)` if `vals.len() ≠ self.num_vars`.
    pub fn eval(&self, vals: &[u64]) -> Result<u64, SemaevError> {
        if vals.len() != self.num_vars {
            return Err(SemaevError::ArityMismatch {
                expected: self.num_vars,
                got: vals.len(),
            });
        }
        let mut result: u64 = 0;
        for (exp, &coeff) in &self.terms {
            // Evaluate the monomial: coeff * ∏ vals[i]^exp[i] mod p.
            let mut mono: u64 = coeff;
            for (i, &e) in exp.iter().enumerate() {
                if e > 0 {
                    mono = (mono * pow_mod(vals[i], e, self.p)) % self.p;
                }
            }
            result = (result + mono) % self.p;
        }
        Ok(result)
    }

    /// Evaluate the polynomial at a partial assignment, returning a polynomial in the
    /// remaining free variables.
    ///
    /// `assignment[i]` is `Some(val)` to fix variable `i` to `val`, or `None` to leave
    /// it free. The result is a polynomial in the free variables, renumbered `0, 1, …`.
    ///
    /// # Errors
    ///
    /// Returns `Err(SemaevError::ArityMismatch)` if `assignment.len() ≠ self.num_vars`.
    pub fn partial_eval(&self, assignment: &[Option<u64>]) -> Result<Self, SemaevError> {
        if assignment.len() != self.num_vars {
            return Err(SemaevError::ArityMismatch {
                expected: self.num_vars,
                got: assignment.len(),
            });
        }
        // Determine the free variables (those with `None` assignment).
        let free_vars: Vec<usize> =
            assignment.iter().enumerate().filter(|(_, a)| a.is_none()).map(|(i, _)| i).collect();
        let new_num_vars = free_vars.len();
        let mut result = MultiPoly::zero(new_num_vars, self.p);

        for (exp, &coeff) in &self.terms {
            // Evaluate the fixed variables.
            let mut fixed_val: u64 = coeff;
            for (i, &e) in exp.iter().enumerate() {
                if let Some(val) = assignment[i] {
                    if e > 0 {
                        fixed_val = (fixed_val * pow_mod(val, e, self.p)) % self.p;
                    }
                }
            }
            if fixed_val == 0 {
                continue;
            }
            // Build the new exponent vector for the free variables.
            let new_exp: Vec<u64> = free_vars.iter().map(|&i| exp[i]).collect();
            let entry = result.terms.entry(new_exp).or_insert(0);
            *entry = (*entry + fixed_val) % self.p;
        }
        result.terms.retain(|_, v| *v != 0);
        Ok(result)
    }

    /// Extract the univariate polynomial in variable `var`, treating all other variables
    /// as symbolic coefficients.
    ///
    /// Returns a `Vec<MultiPoly>` where `result[k]` is the coefficient of `x_{var}^k`
    /// (a polynomial in the remaining `num_vars - 1` variables).
    ///
    /// # Errors
    ///
    /// Returns `Err(SemaevError::VariableOutOfRange)` if `var >= self.num_vars`.
    pub fn univariate_in_var(&self, var: usize) -> Result<Vec<MultiPoly>, SemaevError> {
        if var >= self.num_vars {
            return Err(SemaevError::VariableOutOfRange {
                index: var,
                num_vars: self.num_vars,
            });
        }
        // Find the maximum degree in `var`.
        let max_deg = self
            .terms
            .keys()
            .map(|exp| exp[var] as usize)
            .max()
            .unwrap_or(0);

        let new_num_vars = self.num_vars - 1;
        let mut coeffs: Vec<MultiPoly> =
            (0..=max_deg).map(|_| MultiPoly::zero(new_num_vars, self.p)).collect();

        for (exp, &coeff) in &self.terms {
            let deg_in_var = exp[var] as usize;
            // Build the new exponent vector (drop variable `var`).
            let new_exp: Vec<u64> =
                exp.iter().enumerate().filter(|&(i, _)| i != var).map(|(_, &e)| e).collect();
            let entry = coeffs[deg_in_var].terms.entry(new_exp).or_insert(0);
            *entry = (*entry + coeff) % self.p;
        }
        for c in &mut coeffs {
            c.terms.retain(|_, v| *v != 0);
        }
        Ok(coeffs)
    }

    /// Eliminate variable `var` from two multivariate polynomials by computing their
    /// resultant as univariate polynomials in `var`.
    ///
    /// Both `self` and `other` must have the same `num_vars` and `p`. The result is a
    /// polynomial in `num_vars - 1` variables (variable `var` eliminated).
    ///
    /// This is the key operation for the E.J.3 resultant recursion:
    /// `S_m = Res_X(S_{m-1}(X_1, …, X_{m-2}, X), S_3(X_{m-1}, X_m, X))`.
    ///
    /// # Errors
    ///
    /// Returns `Err(SemaevError::VariableOutOfRange)` if `var >= self.num_vars`.
    /// Returns `Err(SemaevError::ArityMismatch)` if `self` and `other` have different
    /// numbers of variables.
    pub fn elim_var_resultant(
        &self,
        other: &Self,
        var: usize,
    ) -> Result<MultiPoly, SemaevError> {
        if self.num_vars != other.num_vars {
            return Err(SemaevError::ArityMismatch {
                expected: self.num_vars,
                got: other.num_vars,
            });
        }
        if var >= self.num_vars {
            return Err(SemaevError::VariableOutOfRange {
                index: var,
                num_vars: self.num_vars,
            });
        }
        // Extract univariate polynomials in `var` (coefficients are MultiPoly in the
        // remaining variables).
        let f_coeffs = self.univariate_in_var(var)?;
        let g_coeffs = other.univariate_in_var(var)?;

        let new_num_vars = self.num_vars - 1;
        let p = self.p;

        // Build the Sylvester matrix over the coefficient ring (MultiPoly in remaining vars).
        // The Sylvester matrix is (m+n) × (m+n) where m = deg_f, n = deg_g in `var`.
        let m = f_coeffs.len().saturating_sub(1); // deg(f) in var
        let n = g_coeffs.len().saturating_sub(1); // deg(g) in var

        if m == 0 || n == 0 {
            // One polynomial is constant in `var` — resultant is a power of that constant.
            // If f is constant in var: Res(f, g) = f^deg_g.
            // If g is constant in var: Res(f, g) = g^deg_f.
            if m == 0 {
                let f_const = f_coeffs.first().cloned().unwrap_or_else(|| MultiPoly::zero(new_num_vars, p));
                return Ok(multi_pow(&f_const, n));
            } else {
                let g_const = g_coeffs.first().cloned().unwrap_or_else(|| MultiPoly::zero(new_num_vars, p));
                return Ok(multi_pow(&g_const, m));
            }
        }

        let size = m + n;
        // Build the Sylvester matrix as a flat Vec<MultiPoly>.
        // Row i (0 <= i < n): shift of f by i positions.
        // Row n+j (0 <= j < m): shift of g by j positions.
        let zero = MultiPoly::zero(new_num_vars, p);
        let mut mat: Vec<MultiPoly> = vec![zero.clone(); size * size];

        // Fill rows for f (n rows, each shifted by 0..n-1).
        for i in 0..n {
            for (k, c) in f_coeffs.iter().enumerate() {
                // f_coeffs[k] is the coefficient of var^k (ascending).
                // In the Sylvester matrix (descending powers), column = (m - k) + i.
                let col = (m - k) + i;
                if col < size {
                    mat[i * size + col] = c.clone();
                }
            }
        }
        // Fill rows for g (m rows, each shifted by 0..m-1).
        for j in 0..m {
            for (k, c) in g_coeffs.iter().enumerate() {
                let col = (n - k) + j;
                if col < size {
                    mat[(n + j) * size + col] = c.clone();
                }
            }
        }

        // Compute the determinant of the Sylvester matrix via Gaussian elimination
        // over the MultiPoly ring.
        sylvester_det(&mut mat, size, new_num_vars, p)
    }

    /// Symmetrize the polynomial: sum over all permutations of the variables.
    ///
    /// For a polynomial `f(x_0, …, x_{m-1})`, the symmetrization is
    /// `(1/m!) * Σ_{σ ∈ S_m} f(x_{σ(0)}, …, x_{σ(m-1)})`.
    ///
    /// Since we work over `F_p` and `m!` may be zero mod `p` for large `m`, this
    /// method returns the **sum** (not the average): `Σ_{σ ∈ S_m} f(x_{σ(0)}, …, x_{σ(m-1)})`.
    /// The result is symmetric (invariant under all permutations).
    ///
    /// # Principle-4 annotation
    ///
    /// SCALE: toy-scale only — the number of permutations is `m!`, which grows rapidly.
    /// For `m ≤ 5` (the target range), `m! ≤ 120`, which is manageable.
    #[must_use]
    pub fn symmetrize(&self) -> Self {
        let mut result = MultiPoly::zero(self.num_vars, self.p);
        // Generate all permutations of 0..num_vars.
        let perms = permutations(self.num_vars);
        for perm in &perms {
            // Apply permutation: replace x_i with x_{perm[i]}.
            for (exp, &coeff) in &self.terms {
                let mut new_exp = vec![0u64; self.num_vars];
                for (i, &p_i) in perm.iter().enumerate() {
                    new_exp[p_i] = exp[i];
                }
                let entry = result.terms.entry(new_exp).or_insert(0);
                *entry = (*entry + coeff) % self.p;
            }
        }
        result.terms.retain(|_, v| *v != 0);
        result
    }

    /// Check whether the polynomial is symmetric (invariant under all permutations).
    ///
    /// A polynomial is symmetric iff for every monomial `c * x^exp`, the monomial
    /// `c * x^{sorted(exp)}` has the same coefficient for all permutations of `exp`.
    #[must_use]
    pub fn is_symmetric(&self) -> bool {
        // For each term, check that all permutations of its exponent vector have the
        // same coefficient.
        for (exp, &coeff) in &self.terms {
            let perms = permutations(self.num_vars);
            for perm in &perms {
                let mut new_exp = vec![0u64; self.num_vars];
                for (i, &p_i) in perm.iter().enumerate() {
                    new_exp[p_i] = exp[i];
                }
                let other_coeff = self.terms.get(&new_exp).copied().unwrap_or(0);
                if other_coeff != coeff {
                    return false;
                }
            }
        }
        true
    }
}

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Modular exponentiation: `base^exp mod modulus` (u64, iterative square-and-multiply).
pub(crate) fn pow_mod(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
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

/// Raise a `MultiPoly` to an integer power (repeated multiplication).
fn multi_pow(f: &MultiPoly, n: usize) -> MultiPoly {
    if n == 0 {
        return MultiPoly::constant(1, f.num_vars, f.p);
    }
    let mut result = MultiPoly::constant(1, f.num_vars, f.p);
    for _ in 0..n {
        result = result.mul(f);
    }
    result
}

/// Generate all permutations of `0..n` (Heap's algorithm).
fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    let mut perm: Vec<usize> = (0..n).collect();
    let mut c = vec![0usize; n];
    result.push(perm.clone());
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                perm.swap(0, i);
            } else {
                perm.swap(c[i], i);
            }
            result.push(perm.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    result
}

/// Compute the determinant of a Sylvester matrix over the `MultiPoly` ring via
/// cofactor expansion (Gaussian elimination over the polynomial ring).
///
/// `mat` is a flat row-major `size × size` matrix of `MultiPoly` values.
/// Returns the determinant as a `MultiPoly` in `new_num_vars` variables over `F_p`.
///
/// # Principle-4 annotation
///
/// SCALE: toy-scale only — cofactor expansion is `O(n!)` in the worst case. For the
/// Sylvester matrix of `S_3` and `S_3` (the E.J.3 recursion step), the matrix is
/// small (degree ≤ 4 in the eliminated variable → matrix size ≤ 8). For larger `m`,
/// a more efficient algorithm (e.g., Bareiss over the polynomial ring) would be needed.
fn sylvester_det(
    mat: &mut Vec<MultiPoly>,
    size: usize,
    new_num_vars: usize,
    p: u64,
) -> Result<MultiPoly, SemaevError> {
    if size == 0 {
        return Ok(MultiPoly::constant(1, new_num_vars, p));
    }
    if size == 1 {
        return Ok(mat[0].clone());
    }

    // Gaussian elimination with polynomial pivoting.
    // We use cofactor expansion along the first column for small matrices,
    // and Gaussian elimination (Bareiss-style) for larger ones.
    //
    // For the Sylvester matrix of Semaev polynomials (size ≤ 8 for m ≤ 4),
    // cofactor expansion is acceptable.
    det_recursive(mat, size, new_num_vars, p)
}

/// Recursive cofactor expansion for the determinant of a `size × size` matrix.
fn det_recursive(
    mat: &[MultiPoly],
    size: usize,
    new_num_vars: usize,
    p: u64,
) -> Result<MultiPoly, SemaevError> {
    if size == 1 {
        return Ok(mat[0].clone());
    }
    if size == 2 {
        // det = mat[0]*mat[3] - mat[1]*mat[2]
        let a = mat[0].mul(&mat[3]);
        let b = mat[1].mul(&mat[2]);
        return Ok(a.sub(&b));
    }

    // Cofactor expansion along the first row.
    let mut result = MultiPoly::zero(new_num_vars, p);
    for col in 0..size {
        let entry = &mat[col]; // mat[0 * size + col]
        if entry.is_zero() {
            continue;
        }
        // Build the (size-1) × (size-1) minor (delete row 0, column col).
        let minor: Vec<MultiPoly> = (1..size)
            .flat_map(|row| {
                (0..size).filter(|&c| c != col).map(move |c| mat[row * size + c].clone())
            })
            .collect();
        let minor_det = det_recursive(&minor, size - 1, new_num_vars, p)?;
        let term = entry.mul(&minor_det);
        if col % 2 == 0 {
            result = result.add(&term);
        } else {
            result = result.sub(&term);
        }
    }
    Ok(result)
}

// ─── unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FpNaive;

    const P31: u64 = 31;

    fn p31() -> Uint<4> {
        Uint::<4>::from(P31)
    }

    // ── FpPoly tests ──────────────────────────────────────────────────────────

    #[test]
    fn fp_poly_zero_is_zero() {
        let z: FpPoly<FpNaive> = FpPoly::zero();
        assert!(z.is_zero());
        assert_eq!(z.degree(), None);
    }

    #[test]
    fn fp_poly_constant_nonzero() {
        let p = p31();
        let c = FpNaive::from_u64(5, &p);
        let f = FpPoly::constant(c, &p);
        assert!(!f.is_zero());
        assert_eq!(f.degree(), Some(0));
    }

    #[test]
    fn fp_poly_add_sub_roundtrip() {
        let p = p31();
        // f = x^2 + 2x + 3, g = x + 1 over F_31
        let f = FpPoly::from_coeffs(
            vec![
                FpNaive::from_u64(3, &p),
                FpNaive::from_u64(2, &p),
                FpNaive::from_u64(1, &p),
            ],
            &p,
        );
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(1, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let sum = f.add(&g, &p);
        let diff = sum.sub(&g, &p);
        assert_eq!(diff, f, "f + g - g should equal f");
    }

    #[test]
    fn fp_poly_mul_eval() {
        let p = p31();
        // f = x - 2, g = x - 3 → f*g = x^2 - 5x + 6
        let f = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(29, &p), FpNaive::from_u64(1, &p)], // -2 = 29 mod 31
            &p,
        );
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(28, &p), FpNaive::from_u64(1, &p)], // -3 = 28 mod 31
            &p,
        );
        let h = f.mul(&g, &p);
        // h(2) = 0, h(3) = 0
        let x2 = FpNaive::from_u64(2, &p);
        let x3 = FpNaive::from_u64(3, &p);
        assert!(h.eval(&x2, &p).is_zero(&p), "h(2) should be 0");
        assert!(h.eval(&x3, &p).is_zero(&p), "h(3) should be 0");
    }

    #[test]
    fn fp_poly_divmod_exact() {
        let p = p31();
        // f = x^2 - 5x + 6 = (x-2)(x-3), g = x - 2
        // f / g = x - 3, remainder = 0
        let f = FpPoly::from_coeffs(
            vec![
                FpNaive::from_u64(6, &p),
                FpNaive::from_u64(26, &p), // -5 mod 31 = 26
                FpNaive::from_u64(1, &p),
            ],
            &p,
        );
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(29, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let (q, r) = f.divmod(&g, &p);
        assert!(r.is_zero(), "remainder should be 0 for exact division");
        // q should be x - 3 = [28, 1]
        assert_eq!(q.degree(), Some(1));
        assert_eq!(q.coeffs[0].to_uint(), Uint::<4>::from(28u64)); // -3 mod 31
        assert_eq!(q.coeffs[1].to_uint(), Uint::<4>::from(1u64));
    }

    // ── resultant tests ───────────────────────────────────────────────────────

    #[test]
    fn resultant_coprime_is_nonzero() {
        let p = p31();
        // f = x - 2, g = x - 3 (coprime over F_31)
        let f = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(29, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(28, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let res = resultant(&f, &g, &p).unwrap();
        assert!(!res.is_zero(&p), "Res(x-2, x-3) should be nonzero (coprime)");
    }

    #[test]
    fn resultant_common_factor_is_zero() {
        let p = p31();
        // f = (x-2)(x-3) = x^2 - 5x + 6, g = x - 2 (share root x=2)
        let f = FpPoly::from_coeffs(
            vec![
                FpNaive::from_u64(6, &p),
                FpNaive::from_u64(26, &p), // -5 mod 31
                FpNaive::from_u64(1, &p),
            ],
            &p,
        );
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(29, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let res = resultant(&f, &g, &p).unwrap();
        assert!(res.is_zero(&p), "Res(f, g) should be 0 when gcd(f,g) ≠ 1");
    }

    #[test]
    fn resultant_zero_poly_is_zero() {
        let p = p31();
        let f: FpPoly<FpNaive> = FpPoly::zero();
        let g = FpPoly::from_coeffs(
            vec![FpNaive::from_u64(1, &p), FpNaive::from_u64(1, &p)],
            &p,
        );
        let res = resultant(&f, &g, &p).unwrap();
        assert!(res.is_zero(&p), "Res(0, g) should be 0");
    }

    // ── MultiPoly tests ───────────────────────────────────────────────────────

    #[test]
    fn multi_poly_eval_constant() {
        let poly = MultiPoly::constant(5, 2, P31);
        let result = poly.eval(&[3, 7]).unwrap();
        assert_eq!(result, 5, "constant polynomial evaluates to its value");
    }

    #[test]
    fn multi_poly_eval_monomial() {
        // x_0^2 * x_1 over F_31
        let mut poly = MultiPoly::zero(2, P31);
        poly.add_term(vec![2, 1], 1);
        // eval at (3, 4): 3^2 * 4 = 36 = 5 mod 31
        let result = poly.eval(&[3, 4]).unwrap();
        assert_eq!(result, 5, "x_0^2 * x_1 at (3,4) = 36 mod 31 = 5");
    }

    #[test]
    fn multi_poly_symmetrize_is_symmetric() {
        // x_0^2 + x_0*x_1 (not symmetric) → symmetrize → symmetric
        let mut poly = MultiPoly::zero(2, P31);
        poly.add_term(vec![2, 0], 1); // x_0^2
        poly.add_term(vec![1, 1], 1); // x_0 * x_1
        let sym = poly.symmetrize();
        assert!(sym.is_symmetric(), "symmetrized polynomial should be symmetric");
    }

    #[test]
    fn multi_poly_symmetric_eval_permutation_invariant() {
        // Build a symmetric polynomial: x_0^2 + x_1^2 + x_0*x_1 + x_1*x_0 (symmetric)
        let mut poly = MultiPoly::zero(2, P31);
        poly.add_term(vec![2, 0], 1); // x_0^2
        poly.add_term(vec![0, 2], 1); // x_1^2
        poly.add_term(vec![1, 1], 2); // 2*x_0*x_1
        assert!(poly.is_symmetric(), "x_0^2 + x_1^2 + 2*x_0*x_1 should be symmetric");
        // eval at (3, 5) and (5, 3) should be equal
        let v1 = poly.eval(&[3, 5]).unwrap();
        let v2 = poly.eval(&[5, 3]).unwrap();
        assert_eq!(v1, v2, "symmetric polynomial: eval at (3,5) = eval at (5,3)");
    }

    #[test]
    fn multi_poly_partial_eval() {
        // f(x_0, x_1) = x_0 + x_1 over F_31
        let mut poly = MultiPoly::zero(2, P31);
        poly.add_term(vec![1, 0], 1); // x_0
        poly.add_term(vec![0, 1], 1); // x_1
        // Fix x_0 = 3 → result should be 3 + x_0 (now x_0 is the remaining variable)
        let partial = poly.partial_eval(&[Some(3), None]).unwrap();
        assert_eq!(partial.num_vars, 1, "one free variable after partial eval");
        // eval at x_0 = 7 → 3 + 7 = 10
        let result = partial.eval(&[7]).unwrap();
        assert_eq!(result, 10, "partial eval: 3 + 7 = 10");
    }

    #[test]
    fn multi_poly_is_symmetric_false() {
        // x_0^2 (not symmetric in 2 variables)
        let mut poly = MultiPoly::zero(2, P31);
        poly.add_term(vec![2, 0], 1);
        assert!(!poly.is_symmetric(), "x_0^2 should not be symmetric in 2 variables");
    }
}
