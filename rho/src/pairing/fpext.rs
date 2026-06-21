//! Extension-field arithmetic: `F_{p^k} = F_p[u] / (m(u))`.
//!
//! [`FpExt`] represents an element of the degree-`k` extension of a prime field
//! `F_p`, realised as the polynomial quotient ring `F_p[u]/(m(u))` for an
//! irreducible modulus `m` of degree `k`.  Elements are coefficient vectors
//! `[c_0, c_1, …, c_{k-1}]` with `c_i ∈ F_p`, representing `c_0 + c_1·u + … +
//! c_{k-1}·u^{k-1}`.
//!
//! # Construction choice: direct degree-k quotient
//!
//! A **direct quotient** `F_p[u]/(m)` is used rather than a tower construction
//! (`F_{p^2}` then `F_{(p^2)^{k/2}}`).  For toy embedding degrees `k ≤ 6` at
//! demonstration fidelity the direct quotient is simpler, more general, and the
//! performance difference is irrelevant.
//!
//! # Inversion
//!
//! `inv` uses the **extended Euclidean algorithm over `F_p[u]`** (polynomial GCD),
//! not Fermat's little theorem `a^{p^k - 2}`.  The Fermat route requires `p^k - 2`
//! multiplications in `F_{p^k}`, which is prohibitively expensive even for small
//! `k`.  The extended-Euclid route runs in `O(k^2)` base-field operations.
//!
//! # Frobenius endomorphism
//!
//! [`FpExt::frobenius`] computes `x ↦ x^p` (the **p-power Frobenius**), NOT
//! `x^{p^k}` (which is the identity).  This is the load-bearing map for the
//! MOV reduction: the Frobenius generates the Galois group `Gal(F_{p^k}/F_p)`,
//! and applying it `k` times returns the identity.

use crypto_bigint::Uint;
use shared_field::Fp;

// ── Core type ─────────────────────────────────────────────────────────────────

/// An element of `F_{p^k} = F_p[u]/(m(u))`.
///
/// Stored as a coefficient vector `coeffs[i]` representing `∑ coeffs[i]·u^i`.
/// The modulus `p` and irreducible polynomial `modulus` are carried by reference
/// in every operation (matching the `Fp` trait's convention for the base field).
#[derive(Clone, Debug)]
pub struct FpExt<F> {
    /// Coefficients `[c_0, c_1, …, c_{k-1}]` in `F_p`.
    pub(crate) coeffs: Vec<F>,
}

impl<F: Clone + PartialEq> PartialEq for FpExt<F> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

impl<F: Clone + PartialEq + Eq> Eq for FpExt<F> {}

// ── Irreducible modulus ────────────────────────────────────────────────────────

/// An irreducible polynomial `m(u) = ∑ m_i · u^i` over `F_p`.
///
/// Stored as a coefficient vector of length `k + 1` (degree-`k` polynomial).
/// The leading coefficient `m[k]` must be 1 (monic).
#[derive(Clone, Debug)]
pub struct IrreducibleModulus<F> {
    /// Coefficients `[m_0, m_1, …, m_k]`; `m[k] = 1` (monic).
    pub(crate) coeffs: Vec<F>,
}

impl<F: Fp<4>> IrreducibleModulus<F> {
    /// Construct a monic irreducible modulus from its coefficient vector.
    ///
    /// `coeffs` must have length `k + 1` with `coeffs[k] = 1` (monic).
    ///
    /// # Panics
    ///
    /// Panics if `coeffs` is empty or the leading coefficient is not 1.
    pub fn new(coeffs: Vec<F>, p: &Uint<4>) -> Self {
        assert!(!coeffs.is_empty(), "modulus must be non-empty");
        assert!(
            coeffs.last().unwrap().is_one(p),
            "modulus must be monic (leading coefficient = 1)"
        );
        Self { coeffs }
    }

    /// Degree of the modulus polynomial.
    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }
}

// ── FpExt operations ──────────────────────────────────────────────────────────

impl<F: Fp<4>> FpExt<F> {
    /// Additive identity in `F_{p^k}`.
    pub fn zero(k: usize, p: &Uint<4>) -> Self {
        Self { coeffs: (0..k).map(|_| F::zero(p)).collect() }
    }

    /// Multiplicative identity in `F_{p^k}`.
    pub fn one(k: usize, p: &Uint<4>) -> Self {
        let mut coeffs: Vec<F> = (0..k).map(|_| F::zero(p)).collect();
        coeffs[0] = F::one(p);
        Self { coeffs }
    }

    /// Embed a base-field element `a ∈ F_p` as the constant `a ∈ F_{p^k}`.
    ///
    /// The constant embedding `F_p ↪ F_{p^k}` sets `c_0 = a`, `c_i = 0` for
    /// `i > 0`.  This lifts curve coefficients `a, b ∈ F_p` into the extension.
    pub fn from_base(a: F, k: usize, p: &Uint<4>) -> Self {
        let mut coeffs: Vec<F> = (0..k).map(|_| F::zero(p)).collect();
        coeffs[0] = a;
        Self { coeffs }
    }

    /// Degree of the extension (`k`).
    pub fn degree(&self) -> usize {
        self.coeffs.len()
    }

    /// Return `true` if this element is zero.
    pub fn is_zero(&self, p: &Uint<4>) -> bool {
        self.coeffs.iter().all(|c| c.is_zero(p))
    }

    /// Return `true` if this element is the multiplicative identity.
    pub fn is_one(&self, p: &Uint<4>) -> bool {
        let mut iter = self.coeffs.iter();
        let first = iter.next().map_or(false, |c| c.is_one(p));
        first && iter.all(|c| c.is_zero(p))
    }

    /// Coefficient-wise addition: `self + rhs mod p`.
    pub fn add(&self, rhs: &Self, p: &Uint<4>) -> Self {
        let k = self.coeffs.len();
        debug_assert_eq!(k, rhs.coeffs.len(), "degree mismatch in FpExt::add");
        Self {
            coeffs: self.coeffs.iter().zip(rhs.coeffs.iter()).map(|(a, b)| a.add(b, p)).collect(),
        }
    }

    /// Coefficient-wise subtraction: `self - rhs mod p`.
    pub fn sub(&self, rhs: &Self, p: &Uint<4>) -> Self {
        let k = self.coeffs.len();
        debug_assert_eq!(k, rhs.coeffs.len(), "degree mismatch in FpExt::sub");
        Self {
            coeffs: self.coeffs.iter().zip(rhs.coeffs.iter()).map(|(a, b)| a.sub(b, p)).collect(),
        }
    }

    /// Coefficient-wise negation: `-self mod p`.
    pub fn neg(&self, p: &Uint<4>) -> Self {
        Self { coeffs: self.coeffs.iter().map(|c| c.neg(p)).collect() }
    }

    /// Polynomial multiplication with reduction mod `m(u)`.
    ///
    /// Schoolbook `O(k^2)` multiplication followed by reduction mod the
    /// irreducible modulus.
    pub fn mul(&self, rhs: &Self, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        let k = self.coeffs.len();
        debug_assert_eq!(k, rhs.coeffs.len(), "degree mismatch in FpExt::mul");
        debug_assert_eq!(k, modulus.degree(), "modulus degree mismatch in FpExt::mul");

        // Schoolbook polynomial multiplication: product has degree 2k-2.
        let mut product: Vec<F> = (0..2 * k - 1).map(|_| F::zero(p)).collect();
        for (i, ai) in self.coeffs.iter().enumerate() {
            for (j, bj) in rhs.coeffs.iter().enumerate() {
                let term = ai.mul(bj, p);
                product[i + j] = product[i + j].add(&term, p);
            }
        }

        // Reduce mod m(u): for each coefficient of degree >= k, substitute
        // u^k ≡ -m_0 - m_1·u - … - m_{k-1}·u^{k-1} (since m is monic of degree k).
        poly_reduce(product, modulus, k, p)
    }

    /// Squaring: `self^2 mod m(u)`.
    pub fn square(&self, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        self.mul(self, modulus, p)
    }

    /// Exponentiation by a `u64` scalar: `self^exp mod m(u)`.
    ///
    /// Square-and-multiply (right-to-left binary method).
    pub fn pow_u64(&self, exp: u64, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        let k = self.coeffs.len();
        let mut result = Self::one(k, p);
        let mut base = self.clone();
        let mut e = exp;
        while e != 0 {
            if e & 1 == 1 {
                result = result.mul(&base, modulus, p);
            }
            base = base.square(modulus, p);
            e >>= 1;
        }
        result
    }

    /// Exponentiation by a `Uint<4>` scalar: `self^exp mod m(u)`.
    ///
    /// Square-and-multiply (right-to-left binary method).
    pub fn pow(&self, exp: &Uint<4>, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        let k = self.coeffs.len();
        let mut result = Self::one(k, p);
        let mut base = self.clone();
        let mut e = *exp;
        while e != Uint::<4>::ZERO {
            if bool::from(e.bit(0)) {
                result = result.mul(&base, modulus, p);
            }
            base = base.square(modulus, p);
            e >>= 1;
        }
        result
    }

    /// Multiplicative inverse via the extended Euclidean algorithm over `F_p[u]`.
    ///
    /// Uses polynomial extended GCD: finds `s(u)` such that
    /// `self(u) · s(u) ≡ 1 (mod m(u))`.  This runs in `O(k^2)` base-field
    /// operations, far cheaper than Fermat's `a^{p^k - 2}` which would require
    /// `O(k · log(p^k))` extension-field multiplications.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero (no inverse exists).
    pub fn inv(&self, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        assert!(!self.is_zero(p), "attempted inversion of zero in FpExt");
        let k = self.coeffs.len();

        // Extended Euclidean algorithm over F_p[u].
        // We maintain: old_r * self ≡ old_s (mod m)
        //              r     * self ≡ s     (mod m)
        // and iterate until r = 0, at which point old_r = gcd = 1 (since m is
        // irreducible and self ≠ 0), and old_s is the inverse.

        // Represent polynomials as Vec<F> with index = degree.
        // old_r = self, r = modulus (without leading 1 — we work with full reps).
        let mut old_r: Vec<F> = self.coeffs.clone();
        let mut r: Vec<F> = modulus.coeffs.clone();

        let mut old_s: Vec<F> = vec![F::one(p)]; // 1
        let mut s: Vec<F> = vec![F::zero(p)]; // 0

        while !poly_is_zero(&r, p) {
            let (q, rem) = poly_div_rem(&old_r, &r, p);
            let new_s = poly_sub(&old_s, &poly_mul_poly(&q, &s, p), p);

            old_r = r;
            r = rem;
            old_s = s;
            s = new_s;
        }

        // old_r should be a non-zero constant (the GCD = 1 up to a scalar).
        // Normalise: divide old_s by old_r[0].
        let gcd_scalar = old_r[0].clone();
        let gcd_inv = gcd_scalar.inv(p);
        let result_coeffs: Vec<F> = old_s.iter().map(|c| c.mul(&gcd_inv, p)).collect();

        // Pad or truncate to degree k.
        let mut coeffs: Vec<F> = (0..k).map(|_| F::zero(p)).collect();
        for (i, c) in result_coeffs.iter().enumerate() {
            if i < k {
                coeffs[i] = c.clone();
            }
        }
        Self { coeffs }
    }

    /// Frobenius endomorphism: `x ↦ x^p`.
    ///
    /// Computes the `p`-power Frobenius map, which generates the Galois group
    /// `Gal(F_{p^k}/F_p)`.  Applying it `k` times returns the identity.
    ///
    /// This is the load-bearing map for the MOV reduction.  Note: this is
    /// `x^p`, NOT `x^{p^k}` (which is the identity by Fermat in `F_{p^k}*`).
    pub fn frobenius(&self, modulus: &IrreducibleModulus<F>, p: &Uint<4>) -> Self {
        // x^p = (∑ c_i · u^i)^p = ∑ c_i^p · u^{i·p}  (freshman's dream in char p)
        // Since c_i ∈ F_p, c_i^p = c_i (Fermat in F_p).
        // So x^p = ∑ c_i · u^{i·p}, and we reduce each u^{i·p} mod m(u).
        let k = self.coeffs.len();
        let mut result = Self::zero(k, p);

        for (i, ci) in self.coeffs.iter().enumerate() {
            if ci.is_zero(p) {
                continue;
            }
            // Compute u^{i*p} mod m(u) by repeated squaring.
            // u^{i*p} = (u^i)^p.
            // Build u^i first, then raise to p.
            let ui = monomial(i, k, p);
            let ui_p = ui.pow(p, modulus, p);
            // Multiply by c_i (which is in F_p, so just scale each coefficient).
            let scaled = scale(&ui_p, ci, p);
            result = result.add(&scaled, p);
        }

        result
    }

    /// Canonicalise to a vector of `Uint<4>` residues.
    ///
    /// Returns `[c_0.to_uint(), c_1.to_uint(), …, c_{k-1}.to_uint()]`.
    /// This is the bridge to `solve_dl`'s `BigInt` encoding (the MOV bridge convention).
    pub fn to_uint_vec(&self) -> Vec<Uint<4>> {
        self.coeffs.iter().map(|c| c.to_uint()).collect()
    }
}

// ── Polynomial helpers (private) ──────────────────────────────────────────────

/// Build the monomial `u^i` as an `FpExt` element of degree `k`.
fn monomial<F: Fp<4>>(i: usize, k: usize, p: &Uint<4>) -> FpExt<F> {
    let mut coeffs: Vec<F> = (0..k).map(|_| F::zero(p)).collect();
    if i < k {
        coeffs[i] = F::one(p);
    } else {
        // u^i for i >= k: need to reduce mod m — but we only call this for i < k
        // in frobenius (since we iterate over coefficients 0..k).
        // For safety, just return zero if out of range.
    }
    FpExt { coeffs }
}

/// Scale an `FpExt` element by a base-field scalar.
fn scale<F: Fp<4>>(a: &FpExt<F>, scalar: &F, p: &Uint<4>) -> FpExt<F> {
    FpExt { coeffs: a.coeffs.iter().map(|c| c.mul(scalar, p)).collect() }
}

/// Reduce a polynomial of degree `2k-2` modulo a monic irreducible of degree `k`.
///
/// Replaces `u^k, u^{k+1}, …` using `u^k ≡ -m_0 - m_1·u - … - m_{k-1}·u^{k-1}`.
fn poly_reduce<F: Fp<4>>(
    mut poly: Vec<F>,
    modulus: &IrreducibleModulus<F>,
    k: usize,
    p: &Uint<4>,
) -> FpExt<F> {
    // Work from the highest degree down to k.
    let deg = poly.len(); // 2k-1 for a product of two degree-(k-1) polys
    for d in (k..deg).rev() {
        if poly[d].is_zero(p) {
            continue;
        }
        // Coefficient of u^d: substitute u^d = u^{d-k} · u^k
        //   = u^{d-k} · (-m_0 - m_1·u - … - m_{k-1}·u^{k-1})
        // i.e. subtract poly[d] * m_j from poly[d-k+j] for j in 0..k.
        let lead = poly[d].clone();
        poly[d] = F::zero(p);
        for j in 0..k {
            let mj = modulus.coeffs[j].clone();
            let term = lead.mul(&mj, p);
            let target = d - k + j;
            poly[target] = poly[target].sub(&term, p);
        }
    }
    FpExt { coeffs: poly[..k].to_vec() }
}

/// Return `true` if the polynomial (as a `Vec<F>`) is identically zero.
fn poly_is_zero<F: Fp<4>>(poly: &[F], p: &Uint<4>) -> bool {
    poly.iter().all(|c| c.is_zero(p))
}

/// Degree of a polynomial (index of the highest non-zero coefficient).
///
/// Returns `None` for the zero polynomial.
fn poly_degree<F: Fp<4>>(poly: &[F], p: &Uint<4>) -> Option<usize> {
    poly.iter().enumerate().rev().find(|(_, c)| !c.is_zero(p)).map(|(i, _)| i)
}

/// Polynomial division with remainder over `F_p[u]`.
///
/// Returns `(quotient, remainder)` such that `a = quotient * b + remainder`
/// and `deg(remainder) < deg(b)`.
///
/// # Panics
///
/// Panics if `b` is the zero polynomial.
fn poly_div_rem<F: Fp<4>>(a: &[F], b: &[F], p: &Uint<4>) -> (Vec<F>, Vec<F>) {
    let deg_b = poly_degree(b, p).expect("division by zero polynomial");
    let lead_b_inv = b[deg_b].inv(p);

    let mut rem: Vec<F> = a.to_vec();
    let mut quot: Vec<F> = vec![F::zero(p); a.len()];

    loop {
        let deg_r = match poly_degree(&rem, p) {
            Some(d) => d,
            None => break, // remainder is zero
        };
        if deg_r < deg_b {
            break;
        }
        // Leading term of rem / leading term of b.
        let coeff = rem[deg_r].mul(&lead_b_inv, p);
        let shift = deg_r - deg_b;
        quot[shift] = quot[shift].add(&coeff, p);
        // Subtract coeff * u^shift * b from rem.
        for (j, bj) in b.iter().enumerate() {
            let term = coeff.mul(bj, p);
            rem[shift + j] = rem[shift + j].sub(&term, p);
        }
    }

    // Trim trailing zeros from quotient and remainder.
    let quot = trim_zeros(quot, p);
    let rem = trim_zeros(rem, p);

    (quot, rem)
}

/// Multiply two polynomials over `F_p` (schoolbook, no reduction).
fn poly_mul_poly<F: Fp<4>>(a: &[F], b: &[F], p: &Uint<4>) -> Vec<F> {
    if a.is_empty() || b.is_empty() {
        return vec![F::zero(p)];
    }
    let mut result: Vec<F> = vec![F::zero(p); a.len() + b.len() - 1];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            let term = ai.mul(bj, p);
            result[i + j] = result[i + j].add(&term, p);
        }
    }
    result
}

/// Subtract two polynomials over `F_p`.
fn poly_sub<F: Fp<4>>(a: &[F], b: &[F], p: &Uint<4>) -> Vec<F> {
    let len = a.len().max(b.len());
    let mut result: Vec<F> = vec![F::zero(p); len];
    for (i, ai) in a.iter().enumerate() {
        result[i] = result[i].add(ai, p);
    }
    for (i, bi) in b.iter().enumerate() {
        result[i] = result[i].sub(bi, p);
    }
    trim_zeros(result, p)
}

/// Remove trailing zero coefficients from a polynomial.
fn trim_zeros<F: Fp<4>>(mut poly: Vec<F>, p: &Uint<4>) -> Vec<F> {
    while poly.len() > 1 && poly.last().map_or(false, |c| c.is_zero(p)) {
        poly.pop();
    }
    poly
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
pub mod tests {
    use super::*;
    use shared_field::FpNaive;

    // ── Fixture: F_{47^2} = F_47[u]/(u^2 + 1) ────────────────────────────────
    //
    // Prime p = 47.  Embedding degree k = 2 w.r.t. torsion prime ℓ = 3:
    //   - ℓ | p^2 - 1 = 2208 = 3 · 736  ✓
    //   - ℓ ∤ p - 1 = 46                ✓  (46 / 3 is not an integer)
    //
    // Irreducible modulus m(u) = u^2 + 1 over F_47:
    //   - m has no root in F_47 iff -1 is a QNR mod 47.
    //   - 47 ≡ 3 (mod 4), so -1 is a QNR mod 47 by the second supplement to QR.
    //   - Therefore u^2 + 1 is irreducible over F_47.  ✓

    /// Base prime for the k=2 fixture.
    pub const P47: u64 = 47;

    /// Return the modulus `p = 47` as `Uint<4>`.
    pub fn p47() -> Uint<4> {
        Uint::<4>::from(P47)
    }

    /// Return the irreducible modulus `m(u) = u^2 + 1` over `F_47`.
    ///
    /// Coefficients: `[1, 0, 1]` representing `1 + 0·u + 1·u^2`.
    pub fn modulus_k2() -> IrreducibleModulus<FpNaive<4>> {
        let p = p47();
        IrreducibleModulus::new(
            vec![
                FpNaive::<4>::from_u64(1, &p), // constant: 1
                FpNaive::<4>::from_u64(0, &p), // u^1: 0
                FpNaive::<4>::from_u64(1, &p), // u^2: 1  (monic leading coeff)
            ],
            &p,
        )
    }

    /// Construct an element of `F_{47^2}` from two `u64` coefficients `(c0, c1)`.
    pub fn fp2(c0: u64, c1: u64) -> FpExt<FpNaive<4>> {
        let p = p47();
        FpExt {
            coeffs: vec![FpNaive::<4>::from_u64(c0, &p), FpNaive::<4>::from_u64(c1, &p)],
        }
    }

    // ── Irreducibility check ──────────────────────────────────────────────────

    /// Assert that `m(u) = u^2 + 1` has no root in `F_47`.
    ///
    /// A degree-2 polynomial is irreducible over `F_p` iff it has no root in `F_p`.
    #[test]
    fn modulus_k2_is_irreducible() {
        let p = p47();
        // Evaluate m(x) = x^2 + 1 for every x in F_47; none should be zero.
        for x in 0..P47 {
            let x_fp = FpNaive::<4>::from_u64(x, &p);
            let x2 = x_fp.square(&p);
            let one = FpNaive::<4>::one(&p);
            let val = x2.add(&one, &p);
            assert!(
                !val.is_zero(&p),
                "m({x}) = 0 mod 47 — modulus is NOT irreducible!"
            );
        }
    }

    // ── Field axioms ──────────────────────────────────────────────────────────

    /// `a + 0 = a` (additive identity).
    #[test]
    fn add_zero_identity() {
        let p = p47();
        let a = fp2(13, 29);
        let zero = FpExt::zero(2, &p);
        assert_eq!(a.add(&zero, &p), a);
    }

    /// `a · 1 = a` (multiplicative identity).
    #[test]
    fn mul_one_identity() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(13, 29);
        let one = FpExt::one(2, &p);
        assert_eq!(a.mul(&one, &m, &p), a);
    }

    /// `a + (-a) = 0` (additive inverse).
    #[test]
    fn add_neg_is_zero() {
        let p = p47();
        let a = fp2(13, 29);
        let neg_a = a.neg(&p);
        let sum = a.add(&neg_a, &p);
        assert!(sum.is_zero(&p), "a + (-a) should be zero");
    }

    /// `a · a^{-1} = 1` (multiplicative inverse).
    #[test]
    fn mul_inv_is_one() {
        let p = p47();
        let m = modulus_k2();
        // Test several non-zero elements.
        let cases = [(1, 0), (0, 1), (3, 5), (13, 29), (46, 46), (7, 0), (0, 23)];
        for (c0, c1) in cases {
            let a = fp2(c0, c1);
            let ai = a.inv(&m, &p);
            let prod = a.mul(&ai, &m, &p);
            assert!(
                prod.is_one(&p),
                "a·a^{{-1}} ≠ 1 for a = ({c0}, {c1}): got {:?}",
                prod.to_uint_vec()
            );
        }
    }

    /// Distributivity: `a · (b + c) = a·b + a·c`.
    #[test]
    fn distributivity() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(3, 7);
        let b = fp2(11, 5);
        let c = fp2(2, 41);
        let lhs = a.mul(&b.add(&c, &p), &m, &p);
        let rhs = a.mul(&b, &m, &p).add(&a.mul(&c, &m, &p), &p);
        assert_eq!(lhs, rhs, "distributivity failed");
    }

    /// Commutativity of multiplication: `a · b = b · a`.
    #[test]
    fn mul_commutative() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(3, 7);
        let b = fp2(11, 5);
        assert_eq!(a.mul(&b, &m, &p), b.mul(&a, &m, &p));
    }

    /// Associativity of multiplication: `(a · b) · c = a · (b · c)`.
    #[test]
    fn mul_associative() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(3, 7);
        let b = fp2(11, 5);
        let c = fp2(2, 41);
        let lhs = a.mul(&b, &m, &p).mul(&c, &m, &p);
        let rhs = a.mul(&b.mul(&c, &m, &p), &m, &p);
        assert_eq!(lhs, rhs, "mul associativity failed");
    }

    // ── Frobenius / freshman's dream ──────────────────────────────────────────

    /// Freshman's dream: `(a + b)^p = a^p + b^p` in characteristic `p`.
    ///
    /// This holds because the binomial coefficients `C(p, i)` for `0 < i < p`
    /// are divisible by `p`, so all cross terms vanish.
    #[test]
    fn freshmen_dream() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(3, 7);
        let b = fp2(11, 5);
        let lhs = a.add(&b, &p).pow(&p, &m, &p);
        let rhs = a.pow(&p, &m, &p).add(&b.pow(&p, &m, &p), &p);
        assert_eq!(lhs, rhs, "freshman's dream (a+b)^p = a^p + b^p failed");
    }

    /// `frobenius(a) = a^p`.
    #[test]
    fn frobenius_matches_pow_p() {
        let p = p47();
        let m = modulus_k2();
        let cases = [(3, 7), (11, 5), (0, 1), (1, 0), (13, 29)];
        for (c0, c1) in cases {
            let a = fp2(c0, c1);
            let frob = a.frobenius(&m, &p);
            let pow_p = a.pow(&p, &m, &p);
            assert_eq!(
                frob, pow_p,
                "frobenius ≠ a^p for a = ({c0}, {c1})"
            );
        }
    }

    /// `frobenius` applied `k` times is the identity on `F_{p^k}`.
    ///
    /// For `k = 2`: `frob(frob(a)) = a`.
    #[test]
    fn frobenius_k_times_is_identity() {
        let p = p47();
        let m = modulus_k2();
        let cases = [(3, 7), (11, 5), (0, 1), (1, 0), (13, 29), (46, 46)];
        for (c0, c1) in cases {
            let a = fp2(c0, c1);
            // Apply Frobenius k=2 times.
            let frob2 = a.frobenius(&m, &p).frobenius(&m, &p);
            assert_eq!(
                frob2, a,
                "frob^k ≠ identity for a = ({c0}, {c1})"
            );
        }
    }

    // ── from_base embedding ───────────────────────────────────────────────────

    /// `from_base(a) · from_base(b) = from_base(a·b)` (embedding is a ring hom).
    #[test]
    fn from_base_ring_hom() {
        let p = p47();
        let m = modulus_k2();
        let a_base = FpNaive::<4>::from_u64(7, &p);
        let b_base = FpNaive::<4>::from_u64(11, &p);
        let ab_base = a_base.mul(&b_base, &p);

        let a_ext = FpExt::from_base(a_base, 2, &p);
        let b_ext = FpExt::from_base(b_base, 2, &p);
        let ab_ext = FpExt::from_base(ab_base, 2, &p);

        assert_eq!(a_ext.mul(&b_ext, &m, &p), ab_ext, "from_base is not a ring hom");
    }

    // ── to_uint_vec ───────────────────────────────────────────────────────────

    /// `to_uint_vec` returns canonical residues.
    #[test]
    fn to_uint_vec_canonical() {
        let _p = p47();
        let a = fp2(13, 29);
        let v = a.to_uint_vec();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], Uint::<4>::from(13u64));
        assert_eq!(v[1], Uint::<4>::from(29u64));
    }

    // ── pow consistency ───────────────────────────────────────────────────────

    /// `pow_u64(n)` matches `pow(Uint::from(n))`.
    #[test]
    fn pow_u64_matches_pow_uint() {
        let p = p47();
        let m = modulus_k2();
        let a = fp2(3, 7);
        for n in [0u64, 1, 2, 5, 10, 47] {
            let r1 = a.pow_u64(n, &m, &p);
            let r2 = a.pow(&Uint::<4>::from(n), &m, &p);
            assert_eq!(r1, r2, "pow_u64 ≠ pow for n={n}");
        }
    }

    // ── u^2 = -1 in F_{47^2} ─────────────────────────────────────────────────

    /// In `F_{47^2} = F_47[u]/(u^2+1)`, we have `u^2 = -1`.
    #[test]
    fn u_squared_is_neg_one() {
        let p = p47();
        let m = modulus_k2();
        // u = (0, 1) in coefficient form.
        let u = fp2(0, 1);
        let u2 = u.square(&m, &p);
        // -1 in F_47 = 46.
        let neg_one = fp2(46, 0);
        assert_eq!(u2, neg_one, "u^2 should be -1 in F_{{47^2}}");
    }
}
