//! Known-answer tests for `Ideal`: two-element primary representation, norm, and multiplication.
//!
//! All tests use f = x² − 2 (defining ℚ(√2)) as the ambient number field.

use num_bigint::BigInt;
use shared_numfield::{Ideal, IntPoly, NumberField};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// f = x² − 2 (defines ℚ(√2))
fn field_sqrt2() -> NumberField {
    // coeffs: [-2, 0, 1] → -2 + 0·x + 1·x²
    NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
}

// ─── Norm KAT ─────────────────────────────────────────────────────────────────

/// The ideal I = (5, α − 2) in ℚ(√2) has norm 5.
///
/// This is a prime ideal above 5 with residue degree 1 (the standard NFS case).
#[test]
fn norm_kat_prime_ideal() {
    let k = field_sqrt2();
    let i = Ideal::new(&k, bi(5), bi(2));
    assert_eq!(i.norm(), bi(5), "N((5, α−2)) should be 5");
}

// ─── Norm multiplicativity KAT ────────────────────────────────────────────────

/// Norm(I·J) = Norm(I) · Norm(J) for I = (5, α−2) and J = (7, α−3) in ℚ(√2).
///
/// Concretely: Norm(I·J) = 35 = 5 · 7.
#[test]
fn norm_multiplicativity_kat() {
    let k = field_sqrt2();
    let i = Ideal::new(&k, bi(5), bi(2));
    let j = Ideal::new(&k, bi(7), bi(3));
    let ij = i.mul(&j);

    let norm_i = i.norm();
    let norm_j = j.norm();
    let norm_ij = ij.norm();

    assert_eq!(norm_ij, bi(35), "N(I·J) should be 35");
    assert_eq!(norm_ij, norm_i * norm_j, "N(I·J) should equal N(I)·N(J)");
}

// ─── CRT consistency KAT ──────────────────────────────────────────────────────

/// The product ideal I·J = (35, α − r) where r ≡ 2 (mod 5) and r ≡ 3 (mod 7).
///
/// By CRT: r = 17 is the unique solution in [0, 35) satisfying both congruences.
/// Verify: 17 mod 5 = 2 ✓, 17 mod 7 = 3 ✓.
#[test]
fn crt_consistency_kat() {
    let k = field_sqrt2();
    let i = Ideal::new(&k, bi(5), bi(2));
    let j = Ideal::new(&k, bi(7), bi(3));
    let ij = i.mul(&j);

    // Product prime should be 35
    assert_eq!(ij.p, bi(35), "product ideal should have p = 35");

    // r must satisfy both congruences
    let r = &ij.r;
    assert_eq!(r % bi(5), bi(2), "r should be ≡ 2 (mod 5), got r = {r}");
    assert_eq!(r % bi(7), bi(3), "r should be ≡ 3 (mod 7), got r = {r}");

    // The unique solution in [0, 35) is 17
    assert_eq!(*r, bi(17), "CRT solution in [0,35) should be 17");
}

// ─── Panic test ───────────────────────────────────────────────────────────────

/// `Ideal::new` must panic when p ≤ 0.
#[test]
fn new_panics_on_nonpositive_p() {
    let k = field_sqrt2();

    let result_zero = std::panic::catch_unwind(|| {
        Ideal::new(&k, bi(0), bi(1));
    });
    assert!(result_zero.is_err(), "Ideal::new should panic when p = 0");

    let result_neg = std::panic::catch_unwind(|| {
        Ideal::new(&k, bi(-5), bi(1));
    });
    assert!(result_neg.is_err(), "Ideal::new should panic when p < 0");
}
