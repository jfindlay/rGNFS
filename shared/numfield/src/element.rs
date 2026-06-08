//! Number field K = ℚ(α) and its elements.
//!
//! `NumberField` wraps a monic irreducible polynomial `f ∈ ℤ[x]` that defines the field
//! extension K = ℚ[x]/(f(x)).
//!
//! `NumberFieldElement<'a>` is an element of K represented as a polynomial in α of degree
//! strictly less than deg(f), with rational coefficients. Multiplication eagerly reduces mod f.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

use crate::poly::{IntPoly, RatPoly};

// ─── NumberField ─────────────────────────────────────────────────────────────

/// A number field K = ℚ(α) where α is a root of the monic irreducible polynomial f ∈ ℤ[x].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberField {
    /// The defining polynomial f(x). Must be monic and have degree ≥ 1.
    pub f: IntPoly,
}

impl NumberField {
    /// Construct a number field from a monic irreducible polynomial.
    ///
    /// Panics if `f` is not monic or has degree < 1. Does NOT verify irreducibility.
    pub fn new(f: IntPoly) -> Self {
        assert!(f.is_monic(), "defining polynomial must be monic");
        assert!(f.degree().unwrap_or(0) >= 1, "defining polynomial must have degree ≥ 1");
        Self { f }
    }

    /// Extension degree [K : ℚ] = deg(f).
    pub fn degree(&self) -> usize {
        self.f.degree().unwrap()
    }

    /// The primitive element α (the polynomial x mod f).
    pub fn alpha(&self) -> NumberFieldElement<'_> {
        // α is represented as the polynomial x, i.e., coeffs = [0, 1]
        let coeffs = vec![BigRational::zero(), BigRational::one()];
        NumberFieldElement { field: self, poly: RatPoly::from_coeffs(coeffs) }
    }

    /// Embed a rational number r ∈ ℚ into K as the constant element r.
    pub fn from_rational(&self, r: BigRational) -> NumberFieldElement<'_> {
        let poly = if r.is_zero() {
            RatPoly::zero()
        } else {
            RatPoly::from_coeffs(vec![r])
        };
        NumberFieldElement { field: self, poly }
    }

    /// Embed an integer n ∈ ℤ into K as the constant element n.
    pub fn from_int(&self, n: BigInt) -> NumberFieldElement<'_> {
        self.from_rational(BigRational::from(n))
    }
}

// ─── NumberFieldElement ───────────────────────────────────────────────────────

/// An element of K = ℚ(α), represented as a polynomial in α of degree < [K : ℚ].
///
/// Invariant: `self.poly.degree() < self.field.degree()` always holds after construction
/// and after any arithmetic operation.
pub struct NumberFieldElement<'a> {
    /// Reference to the ambient number field.
    pub field: &'a NumberField,
    /// The element as a polynomial in α. Degree < field.degree().
    pub poly: RatPoly,
}

impl<'a> NumberFieldElement<'a> {
    /// Reduce `poly` mod `f` and wrap in a `NumberFieldElement`.
    fn reduced(field: &'a NumberField, poly: RatPoly) -> Self {
        let f_rat = field.f.to_rat_poly();
        let reduced = poly.rem(&f_rat);
        Self { field, poly: reduced }
    }

    /// Add two elements. Both must belong to the same field.
    pub fn add(&self, rhs: &Self) -> Self {
        debug_assert!(
            std::ptr::eq(self.field, rhs.field),
            "elements must belong to the same field"
        );
        // Addition cannot increase degree beyond deg(f)-1, so no reduction needed.
        Self { field: self.field, poly: self.poly.add(&rhs.poly) }
    }

    /// Subtract `rhs` from `self`.
    pub fn sub(&self, rhs: &Self) -> Self {
        debug_assert!(std::ptr::eq(self.field, rhs.field));
        Self { field: self.field, poly: self.poly.sub(&rhs.poly) }
    }

    /// Negate the element.
    pub fn neg(&self) -> Self {
        Self { field: self.field, poly: self.poly.neg() }
    }

    /// Multiply two elements, reducing mod f eagerly.
    pub fn mul(&self, rhs: &Self) -> Self {
        debug_assert!(std::ptr::eq(self.field, rhs.field));
        let product = self.poly.mul(&rhs.poly);
        Self::reduced(self.field, product)
    }

    /// Square the element, reducing mod f eagerly.
    pub fn square(&self) -> Self {
        self.mul(self)
    }

    /// Raise the element to the power `exp` via square-and-multiply.
    pub fn pow(&self, exp: u64) -> Self {
        if exp == 0 {
            return self.field.from_rational(BigRational::one());
        }
        let mut result = self.field.from_rational(BigRational::one());
        let mut base = self.clone_in_field();
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.square();
            e >>= 1;
        }
        result
    }

    /// Multiplicative inverse via the extended Euclidean algorithm in ℚ[x].
    ///
    /// Since f is irreducible and self ≠ 0, gcd(self.poly, f) = 1 and the Bezout
    /// coefficient gives the inverse. Panics if self is zero.
    pub fn inv(&self) -> Self {
        assert!(!self.is_zero(), "cannot invert the zero element");
        let f_rat = self.field.f.to_rat_poly();
        // Extended Euclidean: find s, t such that s * self.poly + t * f_rat = 1
        let (_, s, _) = extended_gcd_rat(&self.poly, &f_rat);
        // s is the inverse of self.poly mod f_rat; reduce mod f to enforce invariant
        Self::reduced(self.field, s)
    }

    /// Field norm N_{K/ℚ}(β) = Res_x(f, g) where g = self.poly.
    ///
    /// Computed via the Sylvester matrix determinant. For monic f of degree d and
    /// g of degree < d, the resultant equals the determinant of the d×d matrix
    /// obtained from the companion-matrix construction.
    pub fn norm(&self) -> BigRational {
        sylvester_resultant(&self.field.f, &self.poly)
    }

    /// Field trace Tr_{K/ℚ}(β) = sum of conjugates of β.
    ///
    /// Computed via the companion matrix of f: build C (companion of f), evaluate
    /// g(C) as a matrix polynomial, and return the trace (sum of diagonal entries).
    pub fn trace(&self) -> BigRational {
        let d = self.field.degree();
        let c = companion_matrix(&self.field.f);
        // Evaluate self.poly at the companion matrix
        let g_of_c = eval_poly_at_matrix(&self.poly, &c, d);
        // Trace = sum of diagonal
        let mut tr = BigRational::zero();
        for i in 0..d {
            tr += &g_of_c[i][i];
        }
        tr
    }

    /// True iff the element is zero.
    pub fn is_zero(&self) -> bool {
        self.poly.coeffs.is_empty() || self.poly.coeffs.iter().all(|c| c.is_zero())
    }

    /// True iff the element is the multiplicative identity 1.
    pub fn is_one(&self) -> bool {
        match self.poly.degree() {
            None => false,
            Some(0) => self.poly.coeffs[0].is_one(),
            _ => false,
        }
    }

    /// True iff the element is rational (degree 0 or zero polynomial).
    pub fn is_rational(&self) -> bool {
        self.poly.degree().map_or(true, |d| d == 0)
    }

    /// Reduce this element modulo the prime ideal `(p, α − r)`, returning the 𝔽_p residue.
    ///
    /// Evaluates the element's `RatPoly` representation at α ≡ r (mod p), clearing
    /// denominators via modular inversion. The result is a `BigInt` in [0, p).
    ///
    /// This is the bridge from the number-field world into 𝔽_p: given an element
    /// β = Σ (aᵢ/bᵢ) αⁱ, returns Σ (aᵢ · bᵢ⁻¹ · rⁱ) mod p.
    ///
    /// # Panics
    ///
    /// Panics if any coefficient denominator is divisible by p. This signals a "bad prime"
    /// for this element: the reduction is undefined because the denominator is not invertible
    /// mod p. Callers must ensure p does not divide any denominator before calling this method.
    pub fn reduce_mod_ideal(&self, p: &BigInt, r: &BigInt) -> BigInt {
        // Accumulate the result as a BigInt in [0, p).
        let mut acc = BigInt::zero();
        // r_pow tracks r^i mod p, starting at r^0 = 1.
        let mut r_pow = BigInt::one();

        for coeff in &self.poly.coeffs {
            // coeff = numer / denom; check that p ∤ denom (bad-prime contract).
            let denom = coeff.denom();
            if denom.is_multiple_of(p) {
                panic!(
                    "reduce_mod_ideal: bad prime — coefficient denominator {} is divisible by p={}; \
                     reduction is undefined for this prime ideal",
                    denom, p
                );
            }

            // Compute denom⁻¹ mod p via the extended Euclidean algorithm.
            let denom_inv = mod_inverse_bigint(denom, p);

            // term = (numer * denom_inv * r_pow) mod p
            let numer = coeff.numer();
            let term = (numer * &denom_inv * &r_pow).mod_floor(p);
            // Accumulate, keeping in [0, p).
            acc = (acc + term).mod_floor(p);

            // Advance r_pow: r^{i+1} = r^i * r mod p.
            r_pow = (&r_pow * r).mod_floor(p);
        }

        acc
    }

    /// Clone this element, keeping the same field reference.
    fn clone_in_field(&self) -> Self {
        Self { field: self.field, poly: self.poly.clone() }
    }
}

impl PartialEq for NumberFieldElement<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Invariant: both polys are already reduced mod f, so coefficient-wise equality suffices.
        self.poly == other.poly
    }
}

impl Eq for NumberFieldElement<'_> {}

impl std::fmt::Debug for NumberFieldElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberFieldElement({:?})", self.poly)
    }
}

// ─── Modular arithmetic helpers ───────────────────────────────────────────────

/// Compute the modular inverse of `a` modulo `m` over ℤ.
///
/// Returns `x` such that `a * x ≡ 1 (mod m)`, reduced into [0, m).
/// Panics if gcd(a, m) ≠ 1 (i.e., `a` is not invertible mod `m`).
fn mod_inverse_bigint(a: &BigInt, m: &BigInt) -> BigInt {
    let (gcd, x, _) = extended_gcd_int(a, m);
    assert!(
        gcd.is_one(),
        "mod_inverse_bigint: {} is not invertible mod {} (gcd = {})",
        a,
        m,
        gcd
    );
    x.mod_floor(m)
}

/// Extended Euclidean algorithm over ℤ.
///
/// Returns `(gcd, s, t)` such that `s * a + t * b = gcd` and `gcd ≥ 0`.
fn extended_gcd_int(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();
    let mut old_t = BigInt::zero();
    let mut t = BigInt::one();

    while !r.is_zero() {
        let q = &old_r / &r;
        let rem = &old_r - &q * &r;
        old_r = r;
        r = rem;
        let new_s = old_s - &q * &s;
        old_s = s;
        s = new_s;
        let new_t = old_t - &q * &t;
        old_t = t;
        t = new_t;
    }

    if old_r.is_negative() {
        (-old_r, -old_s, -old_t)
    } else {
        (old_r, old_s, old_t)
    }
}

// ─── Extended GCD over ℚ[x] ──────────────────────────────────────────────────

/// Extended Euclidean algorithm in ℚ[x].
///
/// Returns `(gcd, s, t)` such that `s * a + t * b = gcd`.
/// The gcd is normalised to be monic (or zero).
fn extended_gcd_rat(a: &RatPoly, b: &RatPoly) -> (RatPoly, RatPoly, RatPoly) {
    // Standard iterative extended Euclidean
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_s = RatPoly::one();
    let mut s = RatPoly::zero();
    let mut old_t = RatPoly::zero();
    let mut t = RatPoly::one();

    while !r.coeffs.is_empty() {
        let (q, rem) = old_r.div_rem(&r);
        old_r = r.clone();
        r = rem;
        let new_s = old_s.sub(&q.mul(&s));
        old_s = s.clone();
        s = new_s;
        let new_t = old_t.sub(&q.mul(&t));
        old_t = t.clone();
        t = new_t;
    }

    // Normalise gcd to monic
    if let Some(lc) = old_r.leading_coeff().cloned() {
        let lc_inv = BigRational::new(lc.denom().clone(), lc.numer().clone());
        old_r = old_r.scale(&lc_inv);
        old_s = old_s.scale(&lc_inv);
        old_t = old_t.scale(&lc_inv);
    }

    (old_r, old_s, old_t)
}

// ─── Sylvester resultant ─────────────────────────────────────────────────────

/// Compute Res_x(f, g) via the Sylvester matrix determinant.
///
/// For monic f of degree d and g of degree < d, the Sylvester matrix is (2d-1)×(2d-1)
/// in general. However, since deg(g) < deg(f) = d, we can use the companion-matrix
/// approach: Res(f, g) = (-1)^(d * deg(g)) * leading_coeff(f)^deg(g) * product of g(α_i)
/// where α_i are the roots of f. Equivalently, for monic f, Res(f, g) = det(g(C)) where
/// C is the companion matrix of f.
///
/// We implement the full Sylvester matrix determinant for correctness.
fn sylvester_resultant(f: &IntPoly, g: &RatPoly) -> BigRational {
    let d = f.degree().unwrap(); // deg(f)
    let e = g.degree().unwrap_or(0); // deg(g), 0 if g is constant or zero

    // Special case: g is the zero polynomial → resultant is 0
    if g.coeffs.is_empty() {
        return BigRational::zero();
    }

    // The Sylvester matrix of f (degree d) and g (degree e) is (d+e) × (d+e).
    // Row layout: first e rows are shifts of f (by x^0, x^1, ..., x^{e-1}),
    //             next d rows are shifts of g (by x^0, x^1, ..., x^{d-1}).
    let n = d + e;
    let mut mat: Vec<Vec<BigRational>> = vec![vec![BigRational::zero(); n]; n];

    // f has coefficients f.coeffs[0..=d], stored least-significant first.
    // In the Sylvester matrix, f's coefficients appear in descending order (highest first).
    // Row i (0-indexed, i < e): f shifted by x^{e-1-i}, so column j gets f[d - (j - (e-1-i))]
    // = f[d - j + e - 1 - i].
    // Rows 0..e: f shifted by x^i for i in 0..e (row i has shift i).
    // Standard Sylvester convention: f-rows have shifts 0, 1, ..., e-1 in order.
    for i in 0..e {
        let shift = i;
        // f coefficients in descending order: f.coeffs[d], f.coeffs[d-1], ..., f.coeffs[0]
        // placed at columns shift, shift+1, ..., shift+d
        for k in 0..=d {
            let col = shift + k;
            // f.coeffs[d-k] is the coefficient of x^{d-k}, which in descending order is index k
            mat[i][col] = BigRational::from(f.coeffs[d - k].clone());
        }
    }

    // Rows e..e+d: g shifted by x^i for i in 0..d (row e+i has shift i).
    // Standard Sylvester convention: g-rows have shifts 0, 1, ..., d-1 in order.
    for i in 0..d {
        let shift = i;
        // g coefficients in descending order: g.coeffs[e], ..., g.coeffs[0]
        // placed at columns shift, shift+1, ..., shift+e
        for k in 0..=e {
            let col = shift + k;
            mat[e + i][col] = g.coeffs[e - k].clone();
        }
    }

    det_rational(&mut mat, n)
}

// ─── Matrix determinant over ℚ ───────────────────────────────────────────────

/// Compute the determinant of an n×n matrix over ℚ via Gaussian elimination.
///
/// Modifies `mat` in place (it is consumed). Uses partial pivoting for numerical
/// stability (though over ℚ this is exact arithmetic).
fn det_rational(mat: &mut Vec<Vec<BigRational>>, n: usize) -> BigRational {
    let mut sign = BigRational::one();

    for col in 0..n {
        // Find pivot: first non-zero entry in column col at or below row col
        let pivot_row = (col..n).find(|&r| !mat[r][col].is_zero());
        let pivot_row = match pivot_row {
            None => return BigRational::zero(), // singular matrix
            Some(r) => r,
        };

        if pivot_row != col {
            mat.swap(col, pivot_row);
            sign = -sign.clone();
        }

        let pivot = mat[col][col].clone();
        let pivot_inv = BigRational::new(pivot.denom().clone(), pivot.numer().clone());

        for row in (col + 1)..n {
            if mat[row][col].is_zero() {
                continue;
            }
            let factor = mat[row][col].clone() * &pivot_inv;
            for j in col..n {
                let sub = factor.clone() * mat[col][j].clone();
                mat[row][j] -= sub;
            }
        }
    }

    // Determinant = sign * product of diagonal entries
    let mut det = sign;
    for i in 0..n {
        det *= mat[i][i].clone();
    }
    det
}

// ─── Companion matrix ─────────────────────────────────────────────────────────

/// Build the d×d companion matrix of a monic polynomial f of degree d.
///
/// The companion matrix C has:
/// - C[i][d-1] = -f.coeffs[i] / f.coeffs[d]  for i = 0..d-1  (last column)
/// - C[i][i-1] = 1  for i = 1..d-1  (subdiagonal)
/// - All other entries 0.
///
/// Since f is monic, f.coeffs[d] = 1, so the last column is just -f.coeffs[i].
fn companion_matrix(f: &IntPoly) -> Vec<Vec<BigRational>> {
    let d = f.degree().unwrap();
    let mut c = vec![vec![BigRational::zero(); d]; d];

    // Subdiagonal: C[i][i-1] = 1 for i = 1..d
    for i in 1..d {
        c[i][i - 1] = BigRational::one();
    }

    // Last column: C[i][d-1] = -f.coeffs[i] (since f is monic)
    for i in 0..d {
        c[i][d - 1] = BigRational::from(-f.coeffs[i].clone());
    }

    c
}

/// Multiply two d×d matrices over ℚ.
fn mat_mul(a: &[Vec<BigRational>], b: &[Vec<BigRational>], d: usize) -> Vec<Vec<BigRational>> {
    let mut result = vec![vec![BigRational::zero(); d]; d];
    for i in 0..d {
        for k in 0..d {
            if a[i][k].is_zero() {
                continue;
            }
            for j in 0..d {
                result[i][j] += a[i][k].clone() * b[k][j].clone();
            }
        }
    }
    result
}


/// Scale a d×d matrix by a scalar.
fn mat_scale(a: &[Vec<BigRational>], s: &BigRational, d: usize) -> Vec<Vec<BigRational>> {
    let mut result = vec![vec![BigRational::zero(); d]; d];
    for i in 0..d {
        for j in 0..d {
            result[i][j] = a[i][j].clone() * s;
        }
    }
    result
}

/// Identity matrix of size d.
fn mat_identity(d: usize) -> Vec<Vec<BigRational>> {
    let mut m = vec![vec![BigRational::zero(); d]; d];
    for i in 0..d {
        m[i][i] = BigRational::one();
    }
    m
}

/// Evaluate a polynomial g at a d×d matrix M: g(M) = sum_i g.coeffs[i] * M^i.
fn eval_poly_at_matrix(g: &RatPoly, m: &[Vec<BigRational>], d: usize) -> Vec<Vec<BigRational>> {
    if g.coeffs.is_empty() {
        return vec![vec![BigRational::zero(); d]; d];
    }

    // Horner's method: g(M) = (...((g_n * M + g_{n-1}) * M + g_{n-2}) * M + ...) + g_0
    let mut result = mat_scale(&mat_identity(d), g.coeffs.last().unwrap(), d);
    for c in g.coeffs.iter().rev().skip(1) {
        result = mat_mul(&result, m, d);
        // Add c * I
        for i in 0..d {
            result[i][i] += c;
        }
    }
    result
}

// ─── unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    fn bri(n: i64) -> BigRational {
        BigRational::from(BigInt::from(n))
    }

    /// f = x² - 2 (defines ℚ(√2))
    fn field_sqrt2() -> NumberField {
        // coeffs: [-2, 0, 1] → -2 + 0·x + 1·x²
        NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
    }

    #[test]
    fn alpha_is_x() {
        let k = field_sqrt2();
        let a = k.alpha();
        assert_eq!(a.poly.coeffs, vec![bri(0), bri(1)]);
    }

    #[test]
    fn add_sub_elements() {
        let k = field_sqrt2();
        let one = k.from_int(bi(1));
        let alpha = k.alpha();
        // 1 + α
        let sum = one.add(&alpha);
        assert_eq!(sum.poly.coeffs, vec![bri(1), bri(1)]);
        // (1 + α) - α = 1
        let diff = sum.sub(&alpha);
        assert_eq!(diff, one);
    }

    #[test]
    fn mul_reduces_mod_f() {
        let k = field_sqrt2();
        let alpha = k.alpha();
        // α² = 2 (since f = x² - 2, so α² ≡ 2 mod f)
        let alpha_sq = alpha.square();
        assert_eq!(alpha_sq, k.from_int(bi(2)));
    }

    #[test]
    fn inv_of_alpha() {
        let k = field_sqrt2();
        let alpha = k.alpha();
        // α⁻¹ in ℚ(√2): α · α⁻¹ = 1
        let inv = alpha.inv();
        let product = alpha.mul(&inv);
        assert!(product.is_one(), "α · α⁻¹ should be 1, got {:?}", product);
    }

    #[test]
    fn pow_consistency() {
        let k = field_sqrt2();
        let alpha = k.alpha();
        // α^4 = (α²)² = 2² = 4
        let a4 = alpha.pow(4);
        assert_eq!(a4, k.from_int(bi(4)));
    }

    #[test]
    fn norm_of_rational() {
        let k = field_sqrt2();
        // Norm(3) = 3^d = 3² = 9
        let three = k.from_int(bi(3));
        let n = three.norm();
        assert_eq!(n, bri(9));
    }

    #[test]
    fn trace_of_alpha() {
        let k = field_sqrt2();
        let alpha = k.alpha();
        // Tr(α) in ℚ(√2) = √2 + (-√2) = 0
        let tr = alpha.trace();
        assert_eq!(tr, bri(0));
    }

    #[test]
    fn trace_of_one() {
        let k = field_sqrt2();
        let one = k.from_int(bi(1));
        // Tr(1) = d = 2
        let tr = one.trace();
        assert_eq!(tr, bri(2));
    }
}
