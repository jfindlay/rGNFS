//! F_{p^k} extension-target type and residue map for NFS-DL.
//!
//! # Contract C-ExtTarget (frozen D.E.1)
//!
//! This module defines the solver-target representation for the k>1 NFS-DL path and the
//! residue map that bridges F_{p^k} (the char-p extension field where the DL target lives)
//! with the residue field of the degree-k prime ideal in K = ℚ[α]/(f) (the char-0 number
//! field the sieve algebra lives in).
//!
//! ## Extension-target type
//!
//! [`ExtTarget`] represents an element of F_{p^k}* as a coefficient vector
//! `[c_0, c_1, …, c_{k-1}]` over F_p, together with the field parameters `p`, `k`, and
//! the irreducible modulus `m(u) = m_0 + m_1·u + … + u^k` (stored as a `Vec<BigInt>` of
//! length `k+1`). This is the representation the k>1 `solve_dl` path reads.
//!
//! The choice of `Vec<BigInt>` coefficients (rather than coupling to `rho`'s `FpExt<F>`)
//! keeps `gnfs` free of a `rho` dependency. The `from_coeffs` constructor accepts the
//! `Vec<BigInt>` form that `FpExt::to_uint_vec` already produces (after `Uint<4> → BigInt`
//! conversion), over-specified for E.C even though D.E's own KATs build targets directly.
//!
//! ## Residue map
//!
//! [`ExtResidueMap`] carries the parameters of the degree-k prime ideal above p in K:
//! the number field `f`, the prime `p`, and the irreducible factor of `f mod p` of degree k
//! (the inert prime condition). It provides:
//!
//! - [`ExtResidueMap::target_to_nf`]: lift an `ExtTarget` coefficient vector to a
//!   `NumberFieldElement` in K (embed c_0 + c_1·α + … + c_{k-1}·α^{k-1} as a char-0 element).
//! - [`ExtResidueMap::nf_to_target`]: reduce a `NumberFieldElement` mod the degree-k prime
//!   ideal, recovering the F_{p^k} coefficient vector.
//!
//! ## Inert-prime condition
//!
//! The degree-k prime ideal's residue field must be **exactly F_{p^k}** — the prime must be
//! inert (f irreducible mod p of degree k), not split. A split prime gives a smaller residue
//! field and the lift is vacuous. [`ExtResidueMap::new`] asserts this condition.
//!
//! ## Rigidity guard
//!
//! The number field stays char-0. The residue map is the **only** place F_{p^k} (char p)
//! meets the sieve algebra. Extension-field arithmetic (multiplication mod m(u)) is
//! implemented here for the KAT cross-checks only; it must not leak into the factor-base or
//! relation-collection algebra.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use shared_numfield::{NumberField, NumberFieldElement, RatPoly};

// ─── ExtTarget ────────────────────────────────────────────────────────────────

/// An element of F_{p^k}* represented as a coefficient vector for the NFS-DL solver.
///
/// Stores `[c_0, c_1, …, c_{k-1}]` with `c_i ∈ [0, p)`, representing
/// `c_0 + c_1·u + … + c_{k-1}·u^{k-1}` in F_{p^k} = F_p[u]/(m(u))`.
///
/// # Contract C-ExtTarget (frozen D.E.1)
///
/// This is the type the k>1 `solve_dl` path reads. Consumed by D.E.2 (factor base),
/// D.E.3 (descent), and E.C (MOV bridge). The `from_coeffs` constructor is over-specified
/// for E.C: it accepts the `Vec<BigInt>` form `FpExt::to_uint_vec` produces.
///
/// # Invariants
///
/// - `coeffs.len() == k`.
/// - Each `c_i ∈ [0, p)`.
/// - `modulus.len() == k + 1` (monic irreducible of degree k).
/// - `modulus[k] == 1` (monic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtTarget {
    /// Coefficient vector `[c_0, …, c_{k-1}]` in F_p.
    pub coeffs: Vec<BigInt>,
    /// The prime base of the field.
    pub p: BigInt,
    /// The extension degree k.
    pub k: usize,
    /// The irreducible modulus `m(u) = m_0 + m_1·u + … + u^k` as a `Vec<BigInt>` of length k+1.
    ///
    /// Stored least-significant first: `modulus[i]` is the coefficient of `u^i`.
    /// The leading coefficient `modulus[k]` must be 1 (monic).
    pub modulus: Vec<BigInt>,
}

impl ExtTarget {
    /// Construct an `ExtTarget` from a coefficient vector, prime, and irreducible modulus.
    ///
    /// This is the primary constructor, over-specified for E.C: it accepts the `Vec<BigInt>`
    /// coefficient form that `FpExt::to_uint_vec` already produces (after `Uint<4> → BigInt`
    /// conversion). E.C produces targets from pairing outputs via this path.
    ///
    /// # Arguments
    ///
    /// - `coeffs`: Coefficient vector `[c_0, …, c_{k-1}]` in F_p (each in `[0, p)`).
    /// - `p`: The prime base.
    /// - `modulus`: The irreducible modulus `m(u)` as a `Vec<BigInt>` of length `k+1`,
    ///   least-significant first, with `modulus[k] = 1` (monic).
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `coeffs` is empty.
    /// - `modulus.len() != coeffs.len() + 1`.
    /// - `modulus[k] != 1` (not monic).
    /// - Any coefficient is negative or `>= p`.
    pub fn from_coeffs(coeffs: Vec<BigInt>, p: BigInt, modulus: Vec<BigInt>) -> Self {
        let k = coeffs.len();
        assert!(k > 0, "ExtTarget: coefficient vector must be non-empty (k >= 1)");
        assert!(
            modulus.len() == k + 1,
            "ExtTarget: modulus must have length k+1 = {}; got {}",
            k + 1,
            modulus.len()
        );
        assert!(
            modulus[k] == BigInt::one(),
            "ExtTarget: modulus must be monic (modulus[k] = 1); got {}",
            modulus[k]
        );
        for (i, c) in coeffs.iter().enumerate() {
            assert!(
                !c.is_negative() && c < &p,
                "ExtTarget: coefficient[{i}] = {c} is out of range [0, {p})"
            );
        }
        Self { coeffs, p, k, modulus }
    }

    /// Return the coefficient at index `i` (the coefficient of `u^i`).
    ///
    /// # Panics
    ///
    /// Panics if `i >= k`.
    pub fn coeff(&self, i: usize) -> &BigInt {
        &self.coeffs[i]
    }

    /// Return `true` if this element is the additive identity (all coefficients zero).
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_zero())
    }

    /// Return `true` if this element is the multiplicative identity (c_0 = 1, rest zero).
    pub fn is_one(&self) -> bool {
        self.coeffs[0] == BigInt::one() && self.coeffs[1..].iter().all(|c| c.is_zero())
    }

    /// Add two extension-field elements: coefficient-wise addition mod p.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `rhs` have different `k`, `p`, or `modulus`.
    pub fn add(&self, rhs: &Self) -> Self {
        self.assert_compatible(rhs);
        let coeffs = self
            .coeffs
            .iter()
            .zip(rhs.coeffs.iter())
            .map(|(a, b)| mod_reduce(&(a + b), &self.p))
            .collect();
        Self { coeffs, p: self.p.clone(), k: self.k, modulus: self.modulus.clone() }
    }

    /// Multiply two extension-field elements: polynomial multiplication mod m(u) and mod p.
    ///
    /// Schoolbook O(k^2) multiplication followed by reduction mod the irreducible modulus.
    ///
    /// # Panics
    ///
    /// Panics if `self` and `rhs` have different `k`, `p`, or `modulus`.
    pub fn mul(&self, rhs: &Self) -> Self {
        self.assert_compatible(rhs);
        let k = self.k;
        let p = &self.p;

        // Schoolbook polynomial multiplication: product has degree 2k-2.
        let mut product: Vec<BigInt> = vec![BigInt::zero(); 2 * k - 1];
        for (i, ai) in self.coeffs.iter().enumerate() {
            for (j, bj) in rhs.coeffs.iter().enumerate() {
                product[i + j] = mod_reduce(&(&product[i + j] + ai * bj), p);
            }
        }

        // Reduce mod m(u): for each coefficient of degree >= k, substitute
        // u^k ≡ -m_0 - m_1·u - … - m_{k-1}·u^{k-1} (since m is monic of degree k).
        let deg = product.len();
        for d in (k..deg).rev() {
            if product[d].is_zero() {
                continue;
            }
            let lead = product[d].clone();
            product[d] = BigInt::zero();
            for j in 0..k {
                let mj = &self.modulus[j];
                // Subtract lead * mj from product[d - k + j].
                let sub = mod_reduce(&(&lead * mj), p);
                product[d - k + j] = mod_reduce(&(&product[d - k + j] - &sub), p);
            }
        }

        let coeffs = product[..k].to_vec();
        Self { coeffs, p: self.p.clone(), k: self.k, modulus: self.modulus.clone() }
    }

    /// Compute the Frobenius endomorphism: `x ↦ x^p`.
    ///
    /// In characteristic p, `(∑ c_i · u^i)^p = ∑ c_i^p · u^{i·p} = ∑ c_i · u^{i·p}`
    /// (since `c_i ∈ F_p` and `c_i^p = c_i` by Fermat). Each `u^{i·p}` is then reduced
    /// mod m(u) by repeated squaring.
    ///
    /// This is the p-power Frobenius, NOT the identity `x^{p^k}`.
    pub fn frobenius(&self) -> Self {
        let k = self.k;
        let p = &self.p;

        // Build the zero element.
        let mut result = Self {
            coeffs: vec![BigInt::zero(); k],
            p: p.clone(),
            k,
            modulus: self.modulus.clone(),
        };

        for (i, ci) in self.coeffs.iter().enumerate() {
            if ci.is_zero() {
                continue;
            }
            // Compute u^{i*p} mod m(u) by repeated squaring.
            // u^{i*p} = (u^i)^p.
            let ui = monomial(i, k, p, &self.modulus);
            let ui_p = pow_ext(&ui, p, p, &self.modulus);
            // Scale by c_i (which is in F_p, so just multiply each coefficient by c_i).
            let scaled_coeffs = scale_ext(&ui_p, ci, p);
            let scaled = Self {
                coeffs: scaled_coeffs,
                p: p.clone(),
                k,
                modulus: self.modulus.clone(),
            };
            result = result.add(&scaled);
        }

        result
    }

    /// Exponentiate by a `BigInt` scalar: `self^exp mod m(u)`.
    ///
    /// Square-and-multiply (right-to-left binary method).
    pub fn pow(&self, exp: &BigInt) -> Self {
        let k = self.k;
        let p = &self.p;
        let mut result = Self {
            coeffs: {
                let mut c = vec![BigInt::zero(); k];
                c[0] = BigInt::one();
                c
            },
            p: p.clone(),
            k,
            modulus: self.modulus.clone(),
        };
        let mut base = self.clone();
        let mut e = exp.clone();
        while !e.is_zero() {
            if (&e % 2u32) == BigInt::one() {
                result = result.mul(&base);
            }
            base = base.mul(&base.clone());
            e >>= 1;
        }
        result
    }

    /// Assert that `self` and `rhs` are compatible (same k, p, modulus).
    fn assert_compatible(&self, rhs: &Self) {
        assert_eq!(self.k, rhs.k, "ExtTarget: k mismatch ({} vs {})", self.k, rhs.k);
        assert_eq!(self.p, rhs.p, "ExtTarget: p mismatch ({} vs {})", self.p, rhs.p);
        assert_eq!(
            self.modulus, rhs.modulus,
            "ExtTarget: modulus mismatch"
        );
    }
}

// ─── ExtResidueMap ────────────────────────────────────────────────────────────

/// The residue map F_{p^k} ↔ residue field of the degree-k prime ideal in K = ℚ[α]/(f).
///
/// Carries the parameters of the degree-k prime ideal above p in K:
/// - The number field `field` (K = ℚ[α]/(f)).
/// - The prime `p`.
/// - The irreducible modulus `m(u)` (the irreducible factor of f mod p of degree k).
///
/// # Inert-prime condition
///
/// The degree-k prime ideal's residue field must be **exactly F_{p^k}** — the prime must
/// be inert (f irreducible mod p of degree k), not split. [`ExtResidueMap::new`] asserts
/// this by verifying that f mod p has no roots in F_p (for k=2: f mod p is irreducible
/// of degree 2 iff it has no roots in F_p).
///
/// # Residue-map semantics
///
/// For an inert prime p in K = ℚ[α]/(f) where f is irreducible mod p of degree k,
/// the residue field is F_p[α]/(f mod p) ≅ F_{p^k}. The identification is:
///
/// - **F_{p^k} → K**: given coefficients `[c_0, …, c_{k-1}]` in F_p, the corresponding
///   element in K is `c_0 + c_1·α + … + c_{k-1}·α^{k-1}` (a char-0 polynomial in α).
/// - **K → F_{p^k}**: given a `NumberFieldElement` β = Σ (a_i/b_i)·α^i, evaluate at
///   α mod p (i.e., reduce each rational coefficient mod p) to get a coefficient vector
///   in F_p. This is `NumberFieldElement::reduce_mod_ideal` applied to each basis element.
///
/// # Rigidity guard
///
/// The number field stays char-0. The residue map is the **only** place F_{p^k} (char p)
/// meets the sieve algebra. Do not use `ExtResidueMap` to introduce extension-field
/// arithmetic into the factor-base or relation-collection algebra.
#[derive(Debug)]
pub struct ExtResidueMap {
    /// The number field K = ℚ[α]/(f).
    pub field: NumberField,
    /// The prime p.
    pub p: BigInt,
    /// The extension degree k.
    pub k: usize,
    /// The irreducible modulus m(u) = f mod p (as a `Vec<BigInt>` of length k+1).
    ///
    /// This is the reduction of f mod p, stored least-significant first.
    /// For the pairing_toy fixture (p=47, k=2, f = u^2+1): `[1, 0, 1]`.
    pub modulus: Vec<BigInt>,
}

impl ExtResidueMap {
    /// Construct an `ExtResidueMap` for the degree-k prime ideal above p in K = ℚ[α]/(f).
    ///
    /// # Arguments
    ///
    /// - `field`: The number field K = ℚ[α]/(f). The defining polynomial f must be monic
    ///   and irreducible mod p of degree k (the inert-prime condition).
    /// - `p`: The prime above which the degree-k ideal lies.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `p <= 0`.
    /// - `field.degree() < 1`.
    /// - f mod p has a root in F_p (split prime — residue field would be smaller than F_{p^k}).
    ///   This is the inert-prime assertion: the residue field must be exactly F_{p^k}.
    ///
    /// # Note on the inert-prime check
    ///
    /// For k=2, irreducibility of f mod p is equivalent to having no roots in F_p (since a
    /// degree-2 polynomial is irreducible iff it has no roots). For k>2, the check is
    /// conservative: we verify no roots in F_p, which is necessary but not sufficient for
    /// irreducibility. A full irreducibility check for k>2 would require factoring f mod p.
    /// At toy scale (k=2), the conservative check is exact.
    pub fn new(field: NumberField, p: BigInt) -> Self {
        assert!(p.is_positive(), "ExtResidueMap: p must be positive, got {p}");
        let k = field.degree();
        assert!(k >= 1, "ExtResidueMap: field degree must be >= 1");

        // Assert the inert-prime condition: f mod p must have no roots in F_p.
        // This ensures the residue field is exactly F_{p^k}, not a smaller field.
        let f = &field.f;
        let p_usize = p.to_u64_digits().1[0] as usize; // safe for toy-scale p
        for r in 0..p_usize {
            let r_big = BigInt::from(r);
            let val = f.eval(&r_big);
            let rem = mod_reduce(&val, &p);
            assert!(
                !rem.is_zero(),
                "ExtResidueMap: f has a root r={r} mod p={p} — prime is split, not inert; \
                 residue field is smaller than F_{{p^{k}}}. Use an inert prime."
            );
        }

        // Build the modulus: f mod p, stored as Vec<BigInt> of length k+1.
        let modulus: Vec<BigInt> = f.coeffs.iter().map(|c| mod_reduce(c, &p)).collect();
        assert_eq!(modulus.len(), k + 1, "modulus length mismatch");
        assert_eq!(modulus[k], BigInt::one(), "f must be monic (leading coeff = 1 mod p)");

        Self { field, p, k, modulus }
    }

    /// Lift an `ExtTarget` coefficient vector to a `NumberFieldElement` in K.
    ///
    /// Given `[c_0, …, c_{k-1}]` in F_p, returns the element
    /// `c_0 + c_1·α + … + c_{k-1}·α^{k-1}` in K = ℚ[α]/(f) as a char-0 element.
    ///
    /// This is the F_{p^k} → K direction of the residue map.
    ///
    /// # Panics
    ///
    /// Panics if `target.k != self.k` or `target.p != self.p`.
    pub fn target_to_nf<'a>(&'a self, target: &ExtTarget) -> NumberFieldElement<'a> {
        assert_eq!(
            target.k, self.k,
            "ExtResidueMap::target_to_nf: k mismatch ({} vs {})",
            target.k, self.k
        );
        assert_eq!(
            target.p, self.p,
            "ExtResidueMap::target_to_nf: p mismatch ({} vs {})",
            target.p, self.p
        );

        // Build the polynomial c_0 + c_1·x + … + c_{k-1}·x^{k-1} over ℚ.
        // Each c_i is in [0, p), lifted to ℚ as a rational integer.
        let rat_coeffs: Vec<BigRational> =
            target.coeffs.iter().map(|c| BigRational::from(c.clone())).collect();

        let poly = RatPoly::from_coeffs(rat_coeffs);
        // Reduce mod f to enforce the NumberFieldElement invariant.
        let f_rat = self.field.f.to_rat_poly();
        let reduced = poly.rem(&f_rat);
        NumberFieldElement { field: &self.field, poly: reduced }
    }

    /// Reduce a `NumberFieldElement` mod the degree-k prime ideal, recovering the F_{p^k}
    /// coefficient vector.
    ///
    /// Given β = Σ (a_i/b_i)·α^i in K, evaluates each basis coefficient mod p to produce
    /// the coefficient vector `[c_0, …, c_{k-1}]` in F_p.
    ///
    /// This is the K → F_{p^k} direction of the residue map.
    ///
    /// # Panics
    ///
    /// Panics if any coefficient denominator is divisible by p (bad prime for this element).
    pub fn nf_to_target(&self, elt: &NumberFieldElement<'_>) -> ExtTarget {
        // The element β = Σ (a_i/b_i)·α^i has degree < k (by the NumberFieldElement invariant).
        // We read off the coefficients mod p: c_i = (a_i · b_i^{-1}) mod p.
        let k = self.k;
        let p = &self.p;

        let mut coeffs = vec![BigInt::zero(); k];
        for (i, coeff) in elt.poly.coeffs.iter().enumerate() {
            if i >= k {
                // Should not happen if the element is properly reduced mod f.
                break;
            }
            let denom = coeff.denom();
            assert!(
                !(denom % p).is_zero(),
                "ExtResidueMap::nf_to_target: bad prime — coefficient denominator {} \
                 is divisible by p={}; reduction is undefined",
                denom,
                p
            );
            let denom_inv = mod_inverse_bigint(denom, p);
            let numer = coeff.numer();
            coeffs[i] = mod_reduce(&(numer * &denom_inv), p);
        }

        ExtTarget::from_coeffs(coeffs, p.clone(), self.modulus.clone())
    }

    /// Construct an `ExtTarget` with the given coefficient vector, using this map's parameters.
    ///
    /// Convenience constructor: equivalent to `ExtTarget::from_coeffs(coeffs, self.p, self.modulus)`.
    pub fn make_target(&self, coeffs: Vec<BigInt>) -> ExtTarget {
        ExtTarget::from_coeffs(coeffs, self.p.clone(), self.modulus.clone())
    }
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Reduce `a` into the canonical range `[0, m)` for `m > 0`.
fn mod_reduce(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r.is_negative() { r + m } else { r }
}

/// Compute the modular inverse of `a` modulo `m` over ℤ.
///
/// Returns `x` such that `a * x ≡ 1 (mod m)`, reduced into `[0, m)`.
///
/// # Panics
///
/// Panics if `gcd(a, m) != 1`.
fn mod_inverse_bigint(a: &BigInt, m: &BigInt) -> BigInt {
    let (gcd, x, _) = extended_gcd_int(a, m);
    assert!(
        gcd.is_one(),
        "mod_inverse_bigint: {} is not invertible mod {} (gcd = {})",
        a,
        m,
        gcd
    );
    mod_reduce(&x, m)
}

/// Extended Euclidean algorithm over ℤ.
///
/// Returns `(gcd, s, t)` such that `s * a + t * b = gcd` and `gcd >= 0`.
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

/// Build the monomial `u^i` as a coefficient vector of length `k`.
fn monomial(i: usize, k: usize, p: &BigInt, modulus: &[BigInt]) -> Vec<BigInt> {
    let mut coeffs = vec![BigInt::zero(); k];
    if i < k {
        coeffs[i] = BigInt::one();
    } else {
        // u^i for i >= k: reduce mod m(u) by repeated application.
        // Build u^k = -m_0 - m_1·u - … - m_{k-1}·u^{k-1} and multiply.
        // For simplicity, use pow_ext on u^1.
        let u = {
            let mut c = vec![BigInt::zero(); k];
            if k > 1 {
                c[1] = BigInt::one();
            } else {
                // k=1: u ≡ -m_0 mod m(u) = -m_0 mod p (since m(u) = u + m_0)
                c[0] = mod_reduce(&(-&modulus[0]), p);
            }
            c
        };
        let ui = pow_coeffs(&u, i as u64, p, modulus, k);
        return ui;
    }
    coeffs
}

/// Exponentiate a coefficient vector by a `u64` scalar mod m(u) and mod p.
fn pow_coeffs(base: &[BigInt], exp: u64, p: &BigInt, modulus: &[BigInt], k: usize) -> Vec<BigInt> {
    let mut result = {
        let mut c = vec![BigInt::zero(); k];
        c[0] = BigInt::one();
        c
    };
    let mut b = base.to_vec();
    let mut e = exp;
    while e != 0 {
        if e & 1 == 1 {
            result = mul_coeffs(&result, &b, p, modulus, k);
        }
        b = mul_coeffs(&b, &b.clone(), p, modulus, k);
        e >>= 1;
    }
    result
}

/// Exponentiate a coefficient vector by a `BigInt` scalar mod m(u) and mod p.
fn pow_ext(base: &[BigInt], exp: &BigInt, p: &BigInt, modulus: &[BigInt]) -> Vec<BigInt> {
    let k = base.len();
    let mut result = {
        let mut c = vec![BigInt::zero(); k];
        c[0] = BigInt::one();
        c
    };
    let mut b = base.to_vec();
    let mut e = exp.clone();
    while !e.is_zero() {
        if (&e % 2u32) == BigInt::one() {
            result = mul_coeffs(&result, &b, p, modulus, k);
        }
        b = mul_coeffs(&b, &b.clone(), p, modulus, k);
        e >>= 1;
    }
    result
}

/// Multiply two coefficient vectors mod m(u) and mod p.
fn mul_coeffs(a: &[BigInt], b: &[BigInt], p: &BigInt, modulus: &[BigInt], k: usize) -> Vec<BigInt> {
    let mut product: Vec<BigInt> = vec![BigInt::zero(); 2 * k - 1];
    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            product[i + j] = mod_reduce(&(&product[i + j] + ai * bj), p);
        }
    }
    // Reduce mod m(u).
    let deg = product.len();
    for d in (k..deg).rev() {
        if product[d].is_zero() {
            continue;
        }
        let lead = product[d].clone();
        product[d] = BigInt::zero();
        for j in 0..k {
            let mj = &modulus[j];
            let sub = mod_reduce(&(&lead * mj), p);
            product[d - k + j] = mod_reduce(&(&product[d - k + j] - &sub), p);
        }
    }
    product[..k].to_vec()
}

/// Scale a coefficient vector by a scalar in F_p.
fn scale_ext(a: &[BigInt], scalar: &BigInt, p: &BigInt) -> Vec<BigInt> {
    a.iter().map(|c| mod_reduce(&(c * scalar), p)).collect()
}

// ─── MOV bridge helper ────────────────────────────────────────────────────────

/// Encode a pairing output (as coefficient vector) into the base-p `BigInt` that `solve_dl`
/// consumes.
///
/// This is the gnfs-side half of the E.C MOV bridge (contract C-MovBridge). It accepts the
/// `Vec<BigInt>` coefficient form that `FpExt::to_uint_vec` produces (after `Uint<4> → BigInt`
/// conversion), asserts that the supplied modulus matches what `find_irreducible_degree2(p)`
/// returns (the modulus-consistency guard), builds an [`ExtTarget`], and encodes it as a
/// base-p `BigInt` via [`crate::dl::ext::descent::ext_target_to_bigint`].
///
/// # Modulus-consistency guard
///
/// The guard is mandatory: without it, a prime `p` where `find_irreducible_degree2` picks a
/// different irreducible than the pairing's modulus would compute a DL in a different F_{p²}
/// and return a wrong discrete log with no error.
///
/// # Arguments
///
/// - `coeffs`: Coefficient vector `[c_0, c_1]` in F_p (each in `[0, p)`), from the pairing
///   output.
/// - `p`: The prime base as `BigInt`.
/// - `expected_modulus`: The irreducible modulus the pairing used, as `&[BigInt]` of length 3
///   (for k=2): `[m_0, m_1, 1]` least-significant first.
///
/// # Panics
///
/// Panics if:
/// - `find_irreducible_degree2(p)` fails (no irreducible polynomial found — should not happen
///   for valid primes p > 2).
/// - `expected_modulus` differs from what `find_irreducible_degree2(p)` returns (modulus
///   mismatch: the pairing and the DL solver would be working in different F_{p²}).
/// - Any coefficient is negative or `>= p` (forwarded from `ExtTarget::from_coeffs`).
pub fn fpext_coeffs_to_dl_target(
    coeffs: Vec<BigInt>,
    p: &BigInt,
    expected_modulus: &[BigInt],
) -> BigInt {
    // Modulus-consistency guard: the pairing's irreducible modulus must match the one
    // find_irreducible_degree2 would choose for this p. Without this check, a p where
    // find_irreducible_degree2 picks a different irreducible than the pairing's modulus
    // computes a DL in a different F_{p²} and returns a wrong discrete log with no error.
    let canonical_modulus = crate::dl::ext::descent::find_irreducible_degree2(p)
        .expect("fpext_coeffs_to_dl_target: find_irreducible_degree2 failed for the given p");
    assert!(
        expected_modulus == canonical_modulus.as_slice(),
        "fpext_coeffs_to_dl_target: modulus mismatch — pairing modulus {:?} differs from \
         find_irreducible_degree2(p={p}) = {:?}; the pairing and the DL solver would operate \
         in different F_{{p²}} fields",
        expected_modulus,
        canonical_modulus,
    );

    let target = ExtTarget::from_coeffs(coeffs, p.clone(), canonical_modulus);
    crate::dl::ext::descent::ext_target_to_bigint(&target)
}

// ─── Unit tests (KATs) ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_numfield::IntPoly;

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
    //
    // The number field K = ℚ[α]/(α^2 + 1) has f = x^2 + 1.
    // p = 47 is inert in K: f mod 47 = x^2 + 1 is irreducible over F_47.
    // The residue field of the prime ideal above 47 in K is F_{47^2}.  ✓

    const P47: u64 = 47;

    fn p47() -> BigInt {
        BigInt::from(P47)
    }

    /// The irreducible modulus m(u) = u^2 + 1 as Vec<BigInt>: [1, 0, 1].
    fn modulus_k2() -> Vec<BigInt> {
        vec![BigInt::from(1u64), BigInt::from(0u64), BigInt::from(1u64)]
    }

    /// Construct an ExtTarget in F_{47^2} from two u64 coefficients (c0, c1).
    fn fp2_target(c0: u64, c1: u64) -> ExtTarget {
        ExtTarget::from_coeffs(
            vec![BigInt::from(c0), BigInt::from(c1)],
            p47(),
            modulus_k2(),
        )
    }

    /// The number field K = ℚ[α]/(α^2 + 1).
    fn field_k2() -> NumberField {
        // f = x^2 + 1: coeffs [1, 0, 1] (least-significant first).
        NumberField::new(IntPoly::from_coeffs(vec![
            BigInt::from(1i64),
            BigInt::from(0i64),
            BigInt::from(1i64),
        ]))
    }

    /// Construct the ExtResidueMap for K = ℚ[α]/(α^2+1), p = 47.
    fn residue_map_k2() -> ExtResidueMap {
        ExtResidueMap::new(field_k2(), p47())
    }

    // ── KAT: ExtTarget construction ───────────────────────────────────────────

    /// KAT: `ExtTarget::from_coeffs` constructs a well-formed k=2 target.
    ///
    /// Verifies the encoding of a k=2 target at the pairing_toy parameters (p=47, u^2+1).
    #[test]
    fn kat_ext_target_k2_well_formed() {
        let t = fp2_target(13, 29);
        assert_eq!(t.k, 2, "k should be 2");
        assert_eq!(t.p, p47(), "p should be 47");
        assert_eq!(t.coeffs[0], BigInt::from(13u64), "c_0 should be 13");
        assert_eq!(t.coeffs[1], BigInt::from(29u64), "c_1 should be 29");
        assert_eq!(t.modulus, modulus_k2(), "modulus should be [1, 0, 1]");
    }

    /// KAT: `ExtTarget::from_coeffs` accepts the Vec<BigInt> form FpExt::to_uint_vec produces.
    ///
    /// E.C produces targets from pairing outputs via FpExt::to_uint_vec → Vec<Uint<4>> →
    /// Vec<BigInt> → ExtTarget::from_coeffs. This KAT verifies the constructor accepts that form.
    #[test]
    fn kat_ext_target_from_uint_vec_form() {
        // Simulate the FpExt::to_uint_vec → Vec<BigInt> conversion.
        // FpExt::to_uint_vec returns [Uint<4>::from(13), Uint<4>::from(29)].
        // After conversion: [BigInt::from(13), BigInt::from(29)].
        let coeffs_from_pairing = vec![BigInt::from(13u64), BigInt::from(29u64)];
        let t = ExtTarget::from_coeffs(coeffs_from_pairing, p47(), modulus_k2());
        assert_eq!(t.coeffs[0], BigInt::from(13u64));
        assert_eq!(t.coeffs[1], BigInt::from(29u64));
    }

    // ── KAT: Residue-map round-trip ───────────────────────────────────────────

    /// KAT: residue-map round-trip — coefficient vector → ExtTarget → nf_to_target → identity.
    ///
    /// Verifies that `nf_to_target(target_to_nf(t)) == t` for a sample element.
    /// This is the primary correctness KAT for the residue map.
    #[test]
    fn kat_residue_map_round_trip() {
        let map = residue_map_k2();

        let cases = [(0u64, 1u64), (1, 0), (13, 29), (3, 7), (46, 46), (0, 0)];
        for (c0, c1) in cases {
            let original = fp2_target(c0, c1);
            let nf_elt = map.target_to_nf(&original);
            let recovered = map.nf_to_target(&nf_elt);
            assert_eq!(
                recovered, original,
                "round-trip failed for ({c0}, {c1}): got {:?}",
                recovered.coeffs
            );
        }
    }

    // ── KAT: Residue-map homomorphism (addition) ──────────────────────────────

    /// KAT: the residue map respects addition.
    ///
    /// Verifies that `nf_to_target(target_to_nf(a) + target_to_nf(b)) == a.add(b)`.
    /// Cross-checks against the ExtTarget addition (which mirrors FpExt::add).
    #[test]
    fn kat_residue_map_homomorphism_add() {
        let map = residue_map_k2();

        let a = fp2_target(3, 7);
        let b = fp2_target(11, 5);

        // Compute a + b in F_{47^2} via ExtTarget.
        let sum_ext = a.add(&b);

        // Compute a + b via the number field: lift both, add in K, reduce back.
        let a_nf = map.target_to_nf(&a);
        let b_nf = map.target_to_nf(&b);
        let sum_nf = a_nf.add(&b_nf);
        let sum_via_map = map.nf_to_target(&sum_nf);

        assert_eq!(
            sum_via_map, sum_ext,
            "residue map does not respect addition: \
             map(a+b) = {:?}, a.add(b) = {:?}",
            sum_via_map.coeffs, sum_ext.coeffs
        );
    }

    // ── KAT: Residue-map homomorphism (multiplication) ────────────────────────

    /// KAT: the residue map respects multiplication.
    ///
    /// Verifies that `nf_to_target(target_to_nf(a) * target_to_nf(b)) == a.mul(b)`.
    /// Cross-checks against the ExtTarget multiplication (which mirrors FpExt::mul).
    ///
    /// This is the load-bearing homomorphism check: the residue map must respect the
    /// multiplicative structure of F_{p^k} for the NFS-DL solver to be correct.
    #[test]
    fn kat_residue_map_homomorphism_mul() {
        let map = residue_map_k2();

        let cases = [
            (3u64, 7u64, 11u64, 5u64),
            (1, 0, 0, 1),
            (13, 29, 3, 7),
            (46, 46, 2, 3),
        ];
        for (a0, a1, b0, b1) in cases {
            let a = fp2_target(a0, a1);
            let b = fp2_target(b0, b1);

            // Compute a * b in F_{47^2} via ExtTarget.
            let prod_ext = a.mul(&b);

            // Compute a * b via the number field: lift both, multiply in K, reduce back.
            let a_nf = map.target_to_nf(&a);
            let b_nf = map.target_to_nf(&b);
            let prod_nf = a_nf.mul(&b_nf);
            let prod_via_map = map.nf_to_target(&prod_nf);

            assert_eq!(
                prod_via_map, prod_ext,
                "residue map does not respect multiplication for ({a0},{a1}) * ({b0},{b1}): \
                 map(a*b) = {:?}, a.mul(b) = {:?}",
                prod_via_map.coeffs, prod_ext.coeffs
            );
        }
    }

    // ── KAT: Residue-map homomorphism (Frobenius) ─────────────────────────────

    /// KAT: the residue map respects the Frobenius endomorphism x ↦ x^p.
    ///
    /// Verifies that `nf_to_target(target_to_nf(a)^p) == a.frobenius()`.
    /// Cross-checks against the ExtTarget Frobenius (which mirrors FpExt::frobenius).
    ///
    /// In F_{47^2} = F_47[u]/(u^2+1), the Frobenius sends (c_0 + c_1·u) to
    /// (c_0 + c_1·u)^47 = c_0 + c_1·u^47. Since u^2 = -1, u^47 = u^{2*23+1} = (u^2)^23 · u
    /// = (-1)^23 · u = -u. So Frob(c_0 + c_1·u) = c_0 - c_1·u = (c_0, -c_1 mod 47).
    #[test]
    fn kat_residue_map_homomorphism_frobenius() {
        let map = residue_map_k2();

        let cases = [(3u64, 7u64), (11, 5), (0, 1), (1, 0), (13, 29)];
        for (c0, c1) in cases {
            let a = fp2_target(c0, c1);

            // Compute Frobenius via ExtTarget.
            let frob_ext = a.frobenius();

            // Compute Frobenius via the number field: lift a, raise to p-th power in K,
            // reduce back.
            let a_nf = map.target_to_nf(&a);
            let p_u64 = P47;
            let a_pow_p = a_nf.pow(p_u64);
            let frob_via_map = map.nf_to_target(&a_pow_p);

            assert_eq!(
                frob_via_map, frob_ext,
                "residue map does not respect Frobenius for ({c0}, {c1}): \
                 map(a^p) = {:?}, a.frobenius() = {:?}",
                frob_via_map.coeffs, frob_ext.coeffs
            );
        }
    }

    // ── KAT: Frobenius^k = identity ───────────────────────────────────────────

    /// KAT: applying Frobenius k=2 times is the identity on F_{47^2}.
    ///
    /// Verifies that `a.frobenius().frobenius() == a` for all test cases.
    /// This is a fundamental property of the Frobenius endomorphism.
    #[test]
    fn kat_frobenius_k_times_is_identity() {
        let cases = [(3u64, 7u64), (11, 5), (0, 1), (1, 0), (13, 29), (46, 46)];
        for (c0, c1) in cases {
            let a = fp2_target(c0, c1);
            let frob2 = a.frobenius().frobenius();
            assert_eq!(
                frob2, a,
                "Frobenius^2 should be identity for ({c0}, {c1}): got {:?}",
                frob2.coeffs
            );
        }
    }

    // ── KAT: u^2 = -1 in F_{47^2} ────────────────────────────────────────────

    /// KAT: in F_{47^2} = F_47[u]/(u^2+1), u^2 = -1 = 46 mod 47.
    ///
    /// Verifies the modular reduction: (0, 1)^2 = (46, 0).
    #[test]
    fn kat_u_squared_is_neg_one() {
        let u = fp2_target(0, 1);
        let u2 = u.mul(&u.clone());
        // -1 in F_47 = 46.
        let neg_one = fp2_target(46, 0);
        assert_eq!(u2, neg_one, "u^2 should be -1 = (46, 0) in F_{{47^2}}");
    }

    // ── KAT: Inert-prime assertion ────────────────────────────────────────────

    /// KAT: `ExtResidueMap::new` asserts the inert-prime condition.
    ///
    /// Verifies that constructing a residue map with a split prime panics.
    /// For f = x^2 + 1 and p = 5: f mod 5 = x^2 + 1. Roots: 2^2+1=5≡0 and 3^2+1=10≡0.
    /// So p=5 splits in ℚ[α]/(α^2+1) — the residue field is F_5, not F_{25}.
    #[test]
    fn kat_inert_prime_assertion_panics_on_split() {
        // f = x^2 + 1, p = 5: f(2) = 5 ≡ 0 mod 5 — split prime.
        let field = NumberField::new(IntPoly::from_coeffs(vec![
            BigInt::from(1i64),
            BigInt::from(0i64),
            BigInt::from(1i64),
        ]));
        let result = std::panic::catch_unwind(|| {
            ExtResidueMap::new(field, BigInt::from(5i64));
        });
        assert!(
            result.is_err(),
            "ExtResidueMap::new should panic for a split prime (p=5 splits in ℚ[α]/(α^2+1))"
        );
    }

    /// KAT: `ExtResidueMap::new` succeeds for the inert prime p=47 in ℚ[α]/(α^2+1).
    ///
    /// Verifies that the residue degree is exactly k=2 (the prime is inert).
    #[test]
    fn kat_inert_prime_assertion_succeeds_for_p47() {
        // Should not panic.
        let map = residue_map_k2();
        assert_eq!(map.k, 2, "residue degree should be k=2 for inert prime p=47");
        assert_eq!(map.p, p47(), "prime should be 47");
        assert_eq!(map.modulus, modulus_k2(), "modulus should be [1, 0, 1]");
    }

    // ── KAT: make_target convenience constructor ──────────────────────────────

    /// KAT: `ExtResidueMap::make_target` produces the same result as `ExtTarget::from_coeffs`.
    #[test]
    fn kat_make_target_matches_from_coeffs() {
        let map = residue_map_k2();
        let coeffs = vec![BigInt::from(13u64), BigInt::from(29u64)];
        let t1 = map.make_target(coeffs.clone());
        let t2 = ExtTarget::from_coeffs(coeffs, p47(), modulus_k2());
        assert_eq!(t1, t2, "make_target should match from_coeffs");
    }

    // ── KAT: Frobenius formula for F_{47^2} ───────────────────────────────────

    /// KAT: Frobenius(c_0 + c_1·u) = c_0 - c_1·u in F_{47^2}.
    ///
    /// In F_{47^2} = F_47[u]/(u^2+1), u^47 = -u (since u^2 = -1 and 47 is odd).
    /// So Frob(c_0 + c_1·u) = c_0 + c_1·u^47 = c_0 - c_1·u.
    /// In coefficient form: (c_0, c_1) ↦ (c_0, -c_1 mod 47) = (c_0, 47 - c_1) for c_1 > 0.
    #[test]
    fn kat_frobenius_formula_f47_2() {
        let cases = [
            (3u64, 7u64, 3u64, 40u64),  // (3, 7) → (3, 47-7) = (3, 40)
            (11, 5, 11, 42),             // (11, 5) → (11, 47-5) = (11, 42)
            (0, 1, 0, 46),               // (0, 1) → (0, 47-1) = (0, 46)
            (1, 0, 1, 0),                // (1, 0) → (1, 0) (base-field element fixed by Frob)
            (13, 29, 13, 18),            // (13, 29) → (13, 47-29) = (13, 18)
        ];
        for (c0, c1, expected_c0, expected_c1) in cases {
            let a = fp2_target(c0, c1);
            let frob = a.frobenius();
            assert_eq!(
                frob.coeffs[0],
                BigInt::from(expected_c0),
                "Frobenius c_0 mismatch for ({c0}, {c1})"
            );
            assert_eq!(
                frob.coeffs[1],
                BigInt::from(expected_c1),
                "Frobenius c_1 mismatch for ({c0}, {c1})"
            );
        }
    }

    // ── KAT: Multiplicative identity ──────────────────────────────────────────

    /// KAT: `a.mul(one) == a` (multiplicative identity).
    #[test]
    fn kat_mul_one_identity() {
        let one = fp2_target(1, 0);
        let a = fp2_target(13, 29);
        assert_eq!(a.mul(&one), a, "a * 1 should be a");
    }

    /// KAT: `a.add(zero) == a` (additive identity).
    #[test]
    fn kat_add_zero_identity() {
        let zero = fp2_target(0, 0);
        let a = fp2_target(13, 29);
        assert_eq!(a.add(&zero), a, "a + 0 should be a");
    }

    // ── KAT: Commutativity of multiplication ──────────────────────────────────

    /// KAT: `a.mul(b) == b.mul(a)` (commutativity).
    #[test]
    fn kat_mul_commutative() {
        let a = fp2_target(3, 7);
        let b = fp2_target(11, 5);
        assert_eq!(a.mul(&b), b.mul(&a), "multiplication should be commutative");
    }

    // ── KAT: Distributivity ───────────────────────────────────────────────────

    /// KAT: `a.mul(b.add(c)) == a.mul(b).add(a.mul(c))` (distributivity).
    #[test]
    fn kat_distributivity() {
        let a = fp2_target(3, 7);
        let b = fp2_target(11, 5);
        let c = fp2_target(2, 41);
        let lhs = a.mul(&b.add(&c));
        let rhs = a.mul(&b).add(&a.mul(&c));
        assert_eq!(lhs, rhs, "distributivity failed");
    }

    // ── KAT: is_zero / is_one ─────────────────────────────────────────────────

    /// KAT: `is_zero` and `is_one` are correct.
    #[test]
    fn kat_is_zero_is_one() {
        assert!(fp2_target(0, 0).is_zero(), "(0,0) should be zero");
        assert!(!fp2_target(1, 0).is_zero(), "(1,0) should not be zero");
        assert!(fp2_target(1, 0).is_one(), "(1,0) should be one");
        assert!(!fp2_target(0, 1).is_one(), "(0,1) should not be one");
        assert!(!fp2_target(0, 0).is_one(), "(0,0) should not be one");
    }
}
