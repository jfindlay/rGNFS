//! Known-answer tests for `dedekind_factor`: Dedekind factorisation of (p) in ℤ[α].
//!
//! Tests cover inert primes, split primes, cubic fields, and norm-product verification.
//!
//! # Inert-prime convention
//!
//! When f has no roots mod p (all irreducible factors of f mod p have degree > 1), the ideal (p)
//! is inert (or factors only into higher-degree prime ideals). `dedekind_factor` returns a single
//! sentinel ideal `Ideal { p, r: 0 }` in this case. The sentinel represents the convention that
//! (p, α − 0) = (p, α) generates the same ideal as (p) when f is irreducible mod p.

use num_bigint::BigInt;
use shared_numfield::{dedekind_factor, IntPoly, NumberField};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// f = x² − 2 (defines ℚ(√2))
fn field_sqrt2() -> NumberField {
    // coeffs: [-2, 0, 1] → -2 + 0·x + 1·x²
    NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
}

/// f = x³ − x − 1 (discriminant −23, a totally real cubic)
fn field_cubic() -> NumberField {
    // coeffs: [-1, -1, 0, 1] → -1 − x + 0·x² + x³
    NumberField::new(IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)]))
}

// ─── KAT 1 — Inert prime ──────────────────────────────────────────────────────

/// f = x² − 2, p = 3: 2 is a quadratic non-residue mod 3 (Legendre(2,3) = −1),
/// so f is irreducible mod 3 and (3) is inert in ℤ[√2].
///
/// Verification: f(0) = −2 ≡ 1, f(1) = −1 ≡ 2, f(2) = 2 ≡ 2 (mod 3) — no roots.
///
/// `dedekind_factor` returns the sentinel ideal (3, α − 0) indicating (3) is inert.
#[test]
fn kat1_inert_prime() {
    let k = field_sqrt2();
    let ideals = dedekind_factor(&k, &bi(3));

    // Inert: single sentinel ideal
    assert_eq!(
        ideals.len(),
        1,
        "inert prime p=3 should return one sentinel ideal, got {} ideals",
        ideals.len()
    );
    assert_eq!(ideals[0].p, bi(3), "sentinel ideal should have p = 3");
    assert_eq!(ideals[0].r, bi(0), "sentinel ideal for inert prime should have r = 0");
}

// ─── KAT 2 — Split prime ──────────────────────────────────────────────────────

/// f = x² − 2, p = 7: 2 is a quadratic residue mod 7 (3² = 9 ≡ 2, 4² = 16 ≡ 2 mod 7),
/// so f splits as (x − 3)(x − 4) mod 7 and (7) splits into two prime ideals in ℤ[√2].
///
/// `dedekind_factor` returns two ideals: (7, α − 3) and (7, α − 4).
#[test]
fn kat2_split_prime() {
    let k = field_sqrt2();
    let ideals = dedekind_factor(&k, &bi(7));

    assert_eq!(
        ideals.len(),
        2,
        "split prime p=7 should return two ideals, got {} ideals",
        ideals.len()
    );

    // Both ideals should have p = 7
    for ideal in &ideals {
        assert_eq!(ideal.p, bi(7), "each ideal above 7 should have p = 7");
    }

    // The r values should be 3 and 4 (in some order)
    let mut rs: Vec<BigInt> = ideals.iter().map(|i| i.r.clone()).collect();
    rs.sort();
    assert_eq!(rs, vec![bi(3), bi(4)], "roots of x²−2 mod 7 should be 3 and 4");
}

// ─── KAT 3 — Cubic field, partial split ───────────────────────────────────────

/// f = x³ − x − 1, p = 5: f factors as (x − 2)(irreducible quadratic) mod 5.
///
/// Verification: f(2) = 8 − 2 − 1 = 5 ≡ 0 (mod 5). The other factor x² + 2x + 3 has
/// discriminant 4 − 12 = −8 ≡ 2 (mod 5), and Legendre(2, 5) = −1, so it is irreducible.
///
/// `dedekind_factor` returns at least one ideal with r = 2 (the linear factor).
/// The irreducible quadratic factor also contributes a prime ideal above 5, but its
/// two-element representation is not handled by this implementation (linear-factor scope only).
#[test]
fn kat3_cubic_partial_split() {
    let k = field_cubic();
    let ideals = dedekind_factor(&k, &bi(5));

    // At least one ideal with r = 2 (from the linear factor x − 2)
    let has_r2 = ideals.iter().any(|i| i.r == bi(2));
    assert!(
        has_r2,
        "dedekind_factor for f=x³−x−1, p=5 should include an ideal with r=2 (root of f mod 5)"
    );

    // All returned ideals should have p = 5
    for ideal in &ideals {
        assert_eq!(ideal.p, bi(5), "each ideal above 5 should have p = 5");
    }

    // Verify r=2 is indeed a root: f(2) = 8 − 2 − 1 = 5 ≡ 0 (mod 5)
    let f = &k.f;
    let val = f.eval(&bi(2));
    assert_eq!(&val % bi(5), bi(0), "f(2) should be ≡ 0 (mod 5)");
}

// ─── KAT 4 — Norm product ─────────────────────────────────────────────────────

/// For f = x² − 2, p = 7 (totally split), the product of norms of the returned ideals
/// equals p^deg(f) = 7² = 49.
///
/// Each prime ideal (7, α − r) has norm 7 (residue degree 1). The product is 7 · 7 = 49.
/// This verifies the fundamental identity: ∏ N(𝔭ᵢ)^eᵢ = p^[K:ℚ].
#[test]
fn kat4_norm_product() {
    let k = field_sqrt2();
    let ideals = dedekind_factor(&k, &bi(7));

    // For a totally split prime in a degree-2 field, we get exactly 2 ideals.
    assert_eq!(ideals.len(), 2, "p=7 should split into 2 prime ideals in ℚ(√2)");

    // Product of norms = p^d = 7^2 = 49
    let norm_product: BigInt = ideals.iter().map(|i| i.norm()).product();
    let expected = bi(7).pow(2u32);
    assert_eq!(
        norm_product, expected,
        "product of norms should equal p^d = 7^2 = 49, got {norm_product}"
    );
}

// ─── Panic test ───────────────────────────────────────────────────────────────

/// `dedekind_factor` must panic when p ≤ 0.
#[test]
fn panics_on_nonpositive_p() {
    let k = field_sqrt2();

    let result_zero = std::panic::catch_unwind(|| {
        dedekind_factor(&k, &bi(0));
    });
    assert!(result_zero.is_err(), "dedekind_factor should panic when p = 0");

    let result_neg = std::panic::catch_unwind(|| {
        dedekind_factor(&k, &bi(-7));
    });
    assert!(result_neg.is_err(), "dedekind_factor should panic when p < 0");
}
