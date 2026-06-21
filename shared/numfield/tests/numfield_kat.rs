//! Known-answer tests (KATs) for the `shared-numfield` crate.
//!
//! KATs for number-field arithmetic over ℤ[α] (C-NF):
//!
//! 1. ℤ[α] arithmetic for f = x² − 2: (1+α)(1−α) = −1 in ℚ(√2).
//! 2. Norm for β = 1+α in ℚ(√2): Norm(1+α) = −1.
//! 3. Cubic KAT: f = x³ − x − 1 (discriminant −23):
//!    - Norm(α) = 1
//!    - Norm(α − 1) = −1

use num_bigint::BigInt;
use num_rational::BigRational;
use shared_numfield::{IntPoly, NumberField};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn bri(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

/// Build the number field ℚ(√2), defined by f = x² − 2.
fn field_sqrt2() -> NumberField {
    // coeffs least-significant first: [-2, 0, 1] → x² + 0·x − 2
    NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
}

/// Build the cubic number field defined by f = x³ − x − 1.
fn field_cubic() -> NumberField {
    // coeffs: [-1, -1, 0, 1] → x³ + 0·x² − x − 1
    NumberField::new(IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)]))
}

// ─── KAT 1: (1+α)(1−α) = −1 in ℚ(√2) ───────────────────────────────────────

#[test]
fn kat1_product_1_plus_alpha_times_1_minus_alpha() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    let alpha = k.alpha();

    // 1 + α
    let lhs = one.add(&alpha);
    // 1 − α
    let rhs = one.sub(&alpha);
    // (1 + α)(1 − α) = 1 − α² = 1 − 2 = −1
    let product = lhs.mul(&rhs);

    let expected = k.from_int(bi(-1));
    assert_eq!(
        product, expected,
        "KAT1 failed: (1+α)(1−α) should equal −1 in ℚ(√2)"
    );
}

// ─── KAT 2: Norm(1+α) = −1 in ℚ(√2) ─────────────────────────────────────────

#[test]
fn kat2_norm_1_plus_alpha_in_sqrt2() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    let alpha = k.alpha();

    // β = 1 + α
    let beta = one.add(&alpha);
    // Norm(1 + √2) = (1 + √2)(1 − √2) = 1 − 2 = −1
    let norm = beta.norm();

    assert_eq!(norm, bri(-1), "KAT2 failed: Norm(1+α) should equal −1 in ℚ(√2)");
}

// ─── KAT 3a: Norm(α) = 1 in ℚ(α) for f = x³ − x − 1 ────────────────────────

#[test]
fn kat3a_norm_alpha_cubic() {
    let k = field_cubic();
    let alpha = k.alpha();

    // For monic f = x³ − x − 1, Norm(α) = (−1)^d · f(0) / lc(f) = (−1)^3 · (−1) = 1
    // (since f(0) = −1 and f is monic)
    let norm = alpha.norm();

    assert_eq!(norm, bri(1), "KAT3a failed: Norm(α) should equal 1 for f = x³ − x − 1");
}

// ─── KAT 3b: Norm(α − 1) = 1 in ℚ(α) for f = x³ − x − 1 ───────────────────
//
// Note: the correct formula is Norm(α − c) = (−1)^d · f(c) for monic f of degree d.
// For d = 3 and c = 1: Norm(α − 1) = (−1)^3 · f(1) = −1 · (1 − 1 − 1) = −1 · (−1) = 1.
// Equivalently: the minimal polynomial of β = α − 1 is f(y+1) = y³ + 3y² + 2y − 1,
// whose constant term is −1, so Norm(β) = (−1)^3 · (−1) = 1.

#[test]
fn kat3b_norm_alpha_minus_1_cubic() {
    let k = field_cubic();
    let alpha = k.alpha();
    let one = k.from_int(bi(1));

    // β = α − 1
    let beta = alpha.sub(&one);
    // Norm(α − 1) = (−1)^3 · f(1) = −(−1) = 1.
    // (The PLAN's session detail stated f(1) = −1 directly, omitting the (−1)^d sign.)
    let norm = beta.norm();

    assert_eq!(
        norm,
        bri(1),
        "KAT3b failed: Norm(α−1) should equal 1 for f = x³ − x − 1"
    );
}

// ─── Additional sanity checks ─────────────────────────────────────────────────

#[test]
fn alpha_squared_equals_2_in_sqrt2() {
    let k = field_sqrt2();
    let alpha = k.alpha();
    // α² ≡ 2 mod (x² − 2)
    let alpha_sq = alpha.square();
    assert_eq!(alpha_sq, k.from_int(bi(2)));
}

#[test]
fn inv_round_trip_in_sqrt2() {
    let k = field_sqrt2();
    let alpha = k.alpha();
    let one = k.from_int(bi(1));
    let beta = one.add(&alpha); // 1 + α
    let beta_inv = beta.inv();
    let product = beta.mul(&beta_inv);
    assert!(product.is_one(), "β · β⁻¹ should be 1, got {:?}", product);
}

#[test]
fn pow_4_equals_4_in_sqrt2() {
    let k = field_sqrt2();
    let alpha = k.alpha();
    // α^4 = (α²)² = 2² = 4
    let a4 = alpha.pow(4);
    assert_eq!(a4, k.from_int(bi(4)));
}

#[test]
fn trace_of_alpha_is_zero_in_sqrt2() {
    let k = field_sqrt2();
    let alpha = k.alpha();
    // Tr(√2) = √2 + (−√2) = 0
    let tr = alpha.trace();
    assert_eq!(tr, bri(0));
}

#[test]
fn trace_of_one_is_degree_in_sqrt2() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    // Tr(1) = [K:ℚ] = 2
    let tr = one.trace();
    assert_eq!(tr, bri(2));
}

#[test]
fn norm_of_rational_is_power_of_degree() {
    let k = field_sqrt2();
    // Norm(3) = 3^2 = 9 (for a rational element r, Norm(r) = r^d)
    let three = k.from_int(bi(3));
    let norm = three.norm();
    assert_eq!(norm, bri(9));
}

#[test]
fn cubic_alpha_cubed_equals_alpha_plus_1() {
    let k = field_cubic();
    let alpha = k.alpha();
    let one = k.from_int(bi(1));
    // f = x³ − x − 1 → α³ = α + 1
    let alpha_cubed = alpha.pow(3);
    let expected = alpha.add(&one);
    assert_eq!(alpha_cubed, expected, "α³ should equal α + 1 in ℚ(α) for f = x³ − x − 1");
}
