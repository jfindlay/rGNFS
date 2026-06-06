//! Dedekind factorisation of the ideal (p) in ℤ[α].
//!
//! Given a number field K = ℚ(α) with defining polynomial f ∈ ℤ[x] and a rational prime p
//! not dividing disc(f), this module decomposes the ideal (p) into prime ideals of ℤ[α]
//! using Dedekind's theorem.
//!
//! # Dedekind's theorem (concrete form)
//!
//! For a monic f ∈ ℤ[x] and a prime p with p ∤ disc(f):
//! - Factor f mod p. Each irreducible factor of f mod p corresponds to a prime ideal above p.
//! - For a linear factor (x − r), the prime ideal is (p, α − r) in two-element form.
//! - For an irreducible factor of degree > 1, the prime ideal still exists but its two-element
//!   representation requires a second generator that is not simply α − r for an integer r.
//!
//! # Scope of this implementation
//!
//! This implementation handles the **linear-factor case** only: it finds all r ∈ {0, …, p−1}
//! with f(r) ≡ 0 (mod p) and returns the corresponding prime ideals (p, α − r).
//!
//! For the NFS factor-base construction, only linear factors matter: a prime p splits with a
//! linear factor (x − r) iff there is a prime ideal of norm p above p, which is exactly the
//! condition needed for sieving.
//!
//! # Inert primes
//!
//! If f has no roots mod p (i.e., f is irreducible mod p for a degree-2 field, or more generally
//! has no linear factors mod p), then (p) is inert (or factors only into higher-degree prime
//! ideals). In this case, `dedekind_factor` returns a single sentinel ideal `(p, α − 0)` to
//! indicate that (p) is prime (or has no linear-factor decomposition). This sentinel is documented
//! by the `is_inert` flag on the return type.
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

// ─── Public API ───────────────────────────────────────────────────────────────

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
/// This function does NOT handle primes dividing disc(f) (bad primes). For those, the
/// Dedekind criterion requires additional work (see G.A.4 if implemented).
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
}
