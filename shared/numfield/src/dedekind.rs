//! Dedekind factorisation of the ideal (p) in ℤ[α], with bad-prime handling.
//!
//! Given a number field K = ℚ(α) with defining polynomial f ∈ ℤ[x] and a rational prime p,
//! this module decomposes the ideal (p) into prime ideals of ℤ[α] using Dedekind's theorem
//! and the Dedekind criterion (index criterion) for bad primes.
//!
//! # Dedekind's theorem (concrete form)
//!
//! For a monic f ∈ ℤ[x] and a prime p with p ∤ disc(f):
//! - Factor f mod p. Each irreducible factor of f mod p corresponds to a prime ideal above p.
//! - For a linear factor (x − r), the prime ideal is (p, α − r) in two-element form.
//! - For an irreducible factor of degree > 1, the prime ideal still exists but its two-element
//!   representation requires a second generator that is not simply α − r for an integer r.
//!
//! # Bad primes and the Dedekind criterion
//!
//! A prime p is **bad** (or ramified in the discriminant) if p | disc(f). For bad primes,
//! Dedekind's theorem does not directly apply: the ring ℤ[α] may not be the full ring of
//! integers at p, meaning (p) may not factor correctly via f mod p alone.
//!
//! The **Dedekind criterion** (index criterion) tests whether ℤ[α] is the full ring of integers
//! at p:
//! 1. Let g = squarefree part of f mod p (i.e., g = f / gcd(f, f') mod p).
//! 2. Let h = f / g mod p.
//! 3. Let t = (g·h − f) / p mod p (an integer polynomial since p | (g·h − f)).
//! 4. Compute T = gcd(g, gcd(h, t)) mod p.
//! 5. If T = 1 (constant), then p ∤ [ℤ_K : ℤ[α]] and Dedekind's theorem applies normally.
//! 6. If T ≠ 1, then p | [ℤ_K : ℤ[α]] and the full ring of integers is larger than ℤ[α].
//!
//! # Scope of this implementation
//!
//! This implementation handles the **linear-factor case** for both good and bad primes:
//! it finds all r ∈ {0, …, p−1} with f(r) ≡ 0 (mod p) and returns the corresponding prime
//! ideals (p, α − r). For bad primes where the Dedekind criterion detects index divisibility
//! (T ≠ 1), the result is flagged and a caveat is documented: higher-degree factors of f mod p
//! may yield additional prime ideals not representable in two-element form without the full
//! Round 2 / HNF-basis algorithm.
//!
//! # Inert primes
//!
//! If f has no roots mod p (all factors of f mod p have degree > 1), then (p) is inert (or
//! factors only into higher-degree prime ideals). In this case, `dedekind_factor` returns a
//! single sentinel ideal `(p, α − 0)` to indicate that (p) is prime (or has no linear-factor
//! decomposition). This sentinel is documented by the `is_inert` flag on the return type.
//!
//! **Convention**: an empty `Vec` would be ambiguous (no ideals vs. inert). Instead, the caller
//! can check whether the returned `Vec` has length 1 with `r == 0` and the field has no roots
//! mod p — but for simplicity, the function returns the sentinel `Ideal { p, r: 0 }` for inert
//! primes. Callers that only care about linear-factor ideals should filter by checking
//! `f.eval(&r) % p == 0` for each returned ideal's `r`.
//!
//! # Panics
//!
//! Panics if `p ≤ 0`.

use num_bigint::BigInt;
use num_traits::{Signed, Zero};

use crate::element::NumberField;
use crate::ideal::Ideal;
use crate::poly::IntPoly;
use crate::resultant::{resultant, subresultant_gcd};

// ─── Public types ─────────────────────────────────────────────────────────────

/// Result of `dedekind_factor_extended`: prime ideals above p with bad-prime metadata.
pub struct DedekindResult<'a> {
    /// Prime ideals above p in two-element form (p, α − r).
    ///
    /// For good primes and bad primes where T = 1 (Dedekind criterion passes), this is
    /// the complete factorisation of (p) restricted to linear factors of f mod p.
    ///
    /// For bad primes where T ≠ 1 (`index_divisible = true`), this list covers only the
    /// linear factors of f mod p. Higher-degree irreducible factors of f mod p may yield
    /// additional prime ideals above p that are not representable in two-element form
    /// without the full Round 2 / HNF-basis algorithm (out of scope for this implementation).
    pub ideals: Vec<Ideal<'a>>,

    /// True if p | disc(f), i.e., p is a bad prime for f.
    ///
    /// Bad primes are exactly those where Dedekind's theorem does not apply directly.
    /// The Dedekind criterion (`index_divisible`) gives the finer test.
    pub is_bad_prime: bool,

    /// True if the Dedekind criterion detected that p | [ℤ_K : ℤ[α]].
    ///
    /// When true, ℤ[α] is not the full ring of integers at p, and `ideals` may be
    /// incomplete: higher-degree factors of f mod p contribute additional prime ideals
    /// not captured here. The full factorisation requires the Round 2 algorithm (HNF
    /// basis computation), which is out of scope for this implementation.
    ///
    /// Always false when `is_bad_prime` is false (good primes pass the criterion
    /// automatically since p ∤ disc(f) implies p ∤ [ℤ_K : ℤ[α]]).
    pub index_divisible: bool,
}

// ─── Public API ───────────────────────────────────────────────────────────────

/// Compute the discriminant of a monic polynomial f ∈ ℤ[x].
///
/// For monic f of degree d, disc(f) = (−1)^(d(d−1)/2) · Res(f, f') where f' is the
/// formal derivative. Since f is monic, lc(f) = 1 and no division by lc(f) is needed.
///
/// Returns 0 for polynomials of degree < 1 (constant or zero).
pub fn discriminant(f: &IntPoly) -> BigInt {
    let d = match f.degree() {
        None | Some(0) => return BigInt::zero(),
        Some(d) => d,
    };

    let f_prime = formal_derivative(f);
    let res = resultant(f, &f_prime);

    // Sign factor: (−1)^(d(d−1)/2)
    // d(d−1)/2 is odd iff d ≡ 2 or 3 (mod 4).
    let sign_exp = d * (d - 1) / 2;
    if sign_exp % 2 == 0 { res } else { -res }
}

/// Test whether p divides disc(f), i.e., whether p is a bad prime for f.
///
/// Returns true iff p | disc(f). For good primes (p ∤ disc(f)), Dedekind's theorem
/// applies directly. For bad primes, use `dedekind_factor_extended` which applies the
/// Dedekind criterion to detect whether ℤ[α] is the full ring of integers at p.
pub fn is_bad_prime(f: &IntPoly, p: &BigInt) -> bool {
    let disc = discriminant(f);
    if disc.is_zero() {
        // disc = 0 means f is not squarefree; every prime divides 0.
        return true;
    }
    (&disc % p).is_zero()
}

/// Factorise the ideal (p) in ℤ[α] using Dedekind's theorem.
///
/// Given a number field K = ℚ(α) with defining polynomial f ∈ ℤ[x] and a rational prime p
/// not dividing disc(f), returns the prime ideals above p that correspond to linear factors
/// of f mod p.
///
/// Each returned ideal is in two-element form `(p, α − r)` where r ∈ {0, …, p−1} satisfies
/// f(r) ≡ 0 (mod p).
///
/// # Inert primes
///
/// If f has no roots mod p (all factors of f mod p have degree > 1), the function returns a
/// single sentinel ideal `Ideal { p, r: 0 }`. This represents the convention that (p) is
/// inert (prime) in ℤ[α] with respect to linear factors. The sentinel ideal `(p, α − 0) = (p, α)`
/// generates the same ideal as (p) when f is irreducible mod p.
///
/// # Panics
///
/// Panics if `p ≤ 0`.
///
/// # Note
///
/// This function does NOT handle primes dividing disc(f) (bad primes). For those, use
/// `dedekind_factor_extended` which applies the Dedekind criterion.
pub fn dedekind_factor<'a>(field: &'a NumberField, p: &BigInt) -> Vec<Ideal<'a>> {
    assert!(p.is_positive(), "prime p must be positive, got {p}");

    let f = &field.f;
    let p_usize = p_to_usize(p);

    // Find all roots r ∈ {0, …, p−1} with f(r) ≡ 0 (mod p).
    let mut roots: Vec<BigInt> = Vec::new();
    for r in 0..p_usize {
        let r_big = BigInt::from(r);
        let val = f.eval(&r_big);
        // val mod p, reduced to [0, p)
        let rem = mod_reduce(&val, p);
        if rem.is_zero() {
            roots.push(r_big);
        }
    }

    if roots.is_empty() {
        // Inert prime: no linear factors. Return the sentinel ideal (p, α − 0).
        // The ideal (p, α) = (p, α − 0) generates the same ideal as (p) when f is
        // irreducible mod p, representing the inert prime above p.
        vec![Ideal::new(field, p.clone(), BigInt::zero())]
    } else {
        roots.into_iter().map(|r| Ideal::new(field, p.clone(), r)).collect()
    }
}

/// Factorise the ideal (p) in ℤ[α], handling both good and bad primes.
///
/// For good primes (p ∤ disc(f)) or bad primes where the Dedekind criterion gives T = 1:
/// equivalent to `dedekind_factor` — returns the prime ideals from linear factors of f mod p.
///
/// For bad primes where T ≠ 1 (p | [ℤ_K : ℤ[α]]): returns prime ideals from linear factors
/// of f mod p, with `index_divisible = true` to document that higher-degree factors may yield
/// additional ideals not representable in two-element form without the full Round 2 algorithm.
///
/// The Dedekind criterion is always computed for bad primes; for good primes it is skipped
/// (p ∤ disc(f) implies p ∤ [ℤ_K : ℤ[α]] automatically).
///
/// # Panics
///
/// Panics if `p ≤ 0`.
pub fn dedekind_factor_extended<'a>(field: &'a NumberField, p: &BigInt) -> DedekindResult<'a> {
    assert!(p.is_positive(), "prime p must be positive, got {p}");

    let f = &field.f;
    let bad = is_bad_prime(f, p);

    // Apply the Dedekind criterion only for bad primes.
    // For good primes, p ∤ disc(f) guarantees p ∤ [ℤ_K : ℤ[α]], so T = 1 automatically.
    let index_divisible = if bad { dedekind_criterion(f, p) } else { false };

    // Find linear factors (roots) of f mod p — same logic as dedekind_factor.
    let p_usize = p_to_usize(p);
    let mut roots: Vec<BigInt> = Vec::new();
    for r in 0..p_usize {
        let r_big = BigInt::from(r);
        let val = f.eval(&r_big);
        let rem = mod_reduce(&val, p);
        if rem.is_zero() {
            roots.push(r_big);
        }
    }

    let ideals = if roots.is_empty() {
        vec![Ideal::new(field, p.clone(), BigInt::zero())]
    } else {
        roots.into_iter().map(|r| Ideal::new(field, p.clone(), r)).collect()
    };

    DedekindResult { ideals, is_bad_prime: bad, index_divisible }
}

// ─── Dedekind criterion ───────────────────────────────────────────────────────

/// Apply the Dedekind criterion to test whether p | [ℤ_K : ℤ[α]].
///
/// Returns true if p divides the index (T ≠ 1 mod p), false if T = 1 (Dedekind's theorem
/// applies normally). This is the index criterion / Dedekind's index test.
///
/// Algorithm:
/// 1. g = squarefree part of f mod p = f / gcd(f, f') mod p.
/// 2. h = f / g mod p.
/// 3. t = (g·h − f) / p mod p.
/// 4. T = gcd(g, gcd(h, t)) mod p.
/// 5. Return T ≠ 1 (i.e., deg(T) > 0 or T's constant is not ≡ 1 mod p).
fn dedekind_criterion(f: &IntPoly, p: &BigInt) -> bool {
    // Step 1: compute g = squarefree part of f mod p.
    let f_mod = poly_mod(f, p);
    let f_prime = formal_derivative(f);
    let f_prime_mod = poly_mod(&f_prime, p);

    // gcd(f mod p, f' mod p) in 𝔽_p[x] — subresultant_gcd gives a primitive representative.
    let gcd_ff_prime = subresultant_gcd(&f_mod, &f_prime_mod);
    let gcd_ff_prime = poly_mod(&gcd_ff_prime, p);

    // g = f / gcd(f, f') mod p — exact division in 𝔽_p[x].
    // Use pseudo_div_rem on the ℤ-polynomials (coefficients in [0,p)) and reduce mod p.
    let g = poly_exact_div_mod(&f_mod, &gcd_ff_prime, p);

    // Step 2: h = f / g mod p.
    let h = poly_exact_div_mod(&f_mod, &g, p);

    // Step 3: t = (g·h − f) / p mod p.
    // g·h − f should be divisible by p coefficient-wise.
    // Compute g·h − f over ℤ (not mod p yet) to extract the factor of p.
    // We need the actual integer polynomial g·h − f (with coefficients in ℤ, not reduced mod p).
    // Since g and h have coefficients in [0,p), their product has coefficients in [0, p²·deg).
    // f has coefficients in [0,p) (f_mod). So g·h − f_mod has coefficients that are multiples of p.
    let gh_int = g.mul(&h);
    let diff = gh_int.sub(&f_mod);
    // Each coefficient of diff should be divisible by p.
    let t_coeffs: Vec<BigInt> = diff.coeffs.iter().map(|c| {
        let (q, r) = (c / p, c % p);
        // r should be zero; if not, something went wrong. We proceed defensively.
        debug_assert!(r.is_zero(), "Dedekind criterion: coefficient not divisible by p");
        q
    }).collect();
    let t_raw = IntPoly::from_coeffs(t_coeffs);
    let t = poly_mod(&t_raw, p);

    // Step 4: T = gcd(g, gcd(h, t)) mod p.
    let gcd_ht = poly_mod(&subresultant_gcd(&h, &t), p);
    let big_t = poly_mod(&subresultant_gcd(&g, &gcd_ht), p);

    // Step 5: T ≠ 1 iff deg(T) > 0 or T's constant coefficient ≢ 1 (mod p).
    // A constant polynomial with value c ≡ 1 (mod p) means T = 1 in 𝔽_p[x].
    !poly_is_one_mod(&big_t, p)
}

// ─── Polynomial helpers ────────────────────────────────────────────────────────

/// Compute the formal derivative of f: if f = ∑ aᵢ xⁱ, then f' = ∑ i·aᵢ xⁱ⁻¹.
fn formal_derivative(f: &IntPoly) -> IntPoly {
    if f.coeffs.len() <= 1 {
        return IntPoly::zero();
    }
    let coeffs: Vec<BigInt> = f.coeffs[1..]
        .iter()
        .enumerate()
        .map(|(i, c)| BigInt::from(i as i64 + 1) * c)
        .collect();
    IntPoly::from_coeffs(coeffs)
}

/// Reduce all coefficients of f modulo p into [0, p).
fn poly_mod(f: &IntPoly, p: &BigInt) -> IntPoly {
    let coeffs: Vec<BigInt> = f.coeffs.iter().map(|c| mod_reduce(c, p)).collect();
    IntPoly::from_coeffs(coeffs)
}

/// Exact division of f by g in 𝔽_p[x]: returns q such that f = q·g in 𝔽_p[x].
///
/// Uses `pseudo_div_rem` on the ℤ-polynomials (with coefficients in [0,p)) and reduces
/// the quotient mod p. The remainder is discarded (caller must ensure g | f in 𝔽_p[x]).
///
/// This works because pseudo_div_rem computes lc(g)^e · f = q · g + r over ℤ. When g is
/// monic mod p (leading coefficient 1), the pseudo-multiplier is 1 and the division is exact.
/// When g is not monic, the quotient is scaled by lc(g)^e; reducing mod p gives the correct
/// quotient in 𝔽_p[x] since lc(g) is a unit in 𝔽_p.
fn poly_exact_div_mod(f: &IntPoly, g: &IntPoly, p: &BigInt) -> IntPoly {
    if g.degree().is_none() {
        // g = 0: return f unchanged (degenerate case; caller should not pass zero divisor).
        return f.clone();
    }
    if f.degree().is_none() {
        return IntPoly::zero();
    }
    // If deg(f) < deg(g), quotient is 0.
    if f.degree().unwrap() < g.degree().unwrap() {
        return IntPoly::zero();
    }

    let (q, _r) = f.pseudo_div_rem(g);
    poly_mod(&q, p)
}

/// Test whether a polynomial T is the unit 1 in 𝔽_p[x].
///
/// T = 1 in 𝔽_p[x] iff T has degree 0 and its constant coefficient ≡ 1 (mod p),
/// or T is the zero polynomial (which represents 0, not 1 — so zero returns false).
fn poly_is_one_mod(t: &IntPoly, p: &BigInt) -> bool {
    match t.degree() {
        None => false, // zero polynomial is not 1
        Some(0) => {
            // Constant polynomial: check if coefficient ≡ 1 (mod p).
            let c = mod_reduce(&t.coeffs[0], p);
            c == BigInt::from(1u32)
        }
        Some(_) => false, // degree > 0: not a unit
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Reduce `a` into the canonical range [0, m) for m > 0.
fn mod_reduce(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r.is_negative() { r + m } else { r }
}

/// Convert a `BigInt` prime to `usize` for iteration.
///
/// Panics if p does not fit in usize (primes this large are not supported by the
/// brute-force root-finding loop).
fn p_to_usize(p: &BigInt) -> usize {
    use num_traits::ToPrimitive;
    p.to_usize().expect("prime p must fit in usize for root-finding iteration")
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly::IntPoly;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    /// f = x² − 2 (defines ℚ(√2))
    fn field_sqrt2() -> NumberField {
        NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
    }

    #[test]
    fn inert_prime_returns_sentinel() {
        // p = 3, f = x² − 2: 2 is a QNR mod 3, so f is irreducible mod 3.
        // f(0) = -2 ≡ 1, f(1) = -1 ≡ 2, f(2) = 2 ≡ 2 mod 3 — no roots.
        let k = field_sqrt2();
        let ideals = dedekind_factor(&k, &bi(3));
        assert_eq!(ideals.len(), 1, "inert prime should return one sentinel ideal");
        assert_eq!(ideals[0].p, bi(3));
        assert_eq!(ideals[0].r, bi(0), "sentinel ideal has r = 0");
    }

    #[test]
    fn split_prime_returns_two_ideals() {
        // p = 7, f = x² − 2: 3² = 9 ≡ 2 and 4² = 16 ≡ 2 mod 7, so roots are 3 and 4.
        let k = field_sqrt2();
        let ideals = dedekind_factor(&k, &bi(7));
        assert_eq!(ideals.len(), 2, "split prime should return two ideals");
        let mut rs: Vec<i64> = ideals.iter().map(|i| i.r.to_string().parse().unwrap()).collect();
        rs.sort();
        assert_eq!(rs, vec![3i64, 4], "roots of x²−2 mod 7 should be 3 and 4");
    }

    #[test]
    fn panics_on_zero_p() {
        let k = field_sqrt2();
        let result = std::panic::catch_unwind(|| {
            dedekind_factor(&k, &bi(0));
        });
        assert!(result.is_err(), "dedekind_factor should panic when p = 0");
    }

    #[test]
    fn formal_derivative_quadratic() {
        // f = x² − 2 → f' = 2x
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        let fp = formal_derivative(&f);
        assert_eq!(fp.coeffs, vec![bi(0), bi(2)]);
    }

    #[test]
    fn formal_derivative_cubic() {
        // f = x³ − x − 1 → f' = 3x² − 1
        let f = IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)]);
        let fp = formal_derivative(&f);
        assert_eq!(fp.coeffs, vec![bi(-1), bi(0), bi(3)]);
    }

    #[test]
    fn discriminant_quadratic() {
        // disc(x² − 2) = 8
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        assert_eq!(discriminant(&f), bi(8));
    }

    #[test]
    fn discriminant_cubic() {
        // disc(x³ − x − 1) = −23
        let f = IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)]);
        assert_eq!(discriminant(&f), bi(-23));
    }

    #[test]
    fn is_bad_prime_sqrt2() {
        // disc(x² − 2) = 8; 2 | 8 (bad), 3 ∤ 8 (good)
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        assert!(is_bad_prime(&f, &bi(2)), "2 should be a bad prime for x²−2");
        assert!(!is_bad_prime(&f, &bi(3)), "3 should not be a bad prime for x²−2");
    }

    #[test]
    fn dedekind_criterion_good_prime() {
        // p = 7, f = x² − 2: good prime, criterion should give T = 1 (false).
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        assert!(!dedekind_criterion(&f, &bi(7)), "p=7 is a good prime; criterion should be false");
    }

    #[test]
    fn dedekind_criterion_bad_prime_sqrt2() {
        // p = 2, f = x² − 2: bad prime. f mod 2 = x², f' mod 2 = 0.
        // gcd(x², 0) = x², g = x²/x² = 1, h = x²/1 = x².
        // g·h − f = x² − x² = 0 over ℤ (since f_mod = x²). t = 0.
        // T = gcd(1, gcd(x², 0)) = gcd(1, x²) = 1.
        // So criterion returns false (T = 1) for x² − 2 at p = 2.
        // This is correct: even though 2 | disc, the Dedekind criterion at p=2 for x²−2
        // gives T=1 because ℤ[√2] IS the full ring of integers (it's a PID).
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        // For x²−2 at p=2: ℤ[√2] is the maximal order, so index_divisible should be false.
        assert!(!dedekind_criterion(&f, &bi(2)));
    }
}
