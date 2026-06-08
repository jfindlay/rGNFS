//! Known-answer tests (KATs) for `NumberFieldElement::reduce_mod_ideal`.
//!
//! Field: ℚ(√2), defined by f = x² − 2.
//! Prime ideal: (p, α − r) = (7, α − 3).
//!
//! Verification that (7, α − 3) is a valid prime ideal: f(3) = 9 − 2 = 7 ≡ 0 (mod 7). ✓
//!
//! Hand-computed residues (all mod 7):
//!
//! | Element β          | poly coeffs     | Residue β(3) mod 7 |
//! |--------------------|-----------------|---------------------|
//! | α                  | [0, 1]          | 3                   |
//! | 5 (constant)       | [5]             | 5                   |
//! | 1 + α              | [1, 1]          | 1 + 3 = 4           |
//! | 1 − α              | [1, −1]         | 1 − 3 = −2 ≡ 5     |
//! | −1 (constant)      | [−1]            | −1 ≡ 6              |
//! | 1/2 (rational)     | [1/2]           | 2⁻¹ ≡ 4            |
//! | α/2                | [0, 1/2]        | 3 · 4 = 12 ≡ 5     |
//!
//! Product check: reduce(1+α) · reduce(1−α) = 4 · 5 = 20 ≡ 6 (mod 7) = reduce(−1). ✓

use num_bigint::BigInt;
use num_rational::BigRational;
use shared_numfield::{IntPoly, NumberField, NumberFieldElement};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn bri(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

fn br(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// Build the number field ℚ(√2), defined by f = x² − 2.
fn field_sqrt2() -> NumberField {
    // coeffs least-significant first: [−2, 0, 1] → x² − 2
    NumberField::new(IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]))
}

// ─── KAT 1: reduce(α) = r mod p ──────────────────────────────────────────────

/// Reducing α mod (7, α − 3) gives 3.
///
/// α is represented as the polynomial x, so evaluating at x = 3 gives 3.
#[test]
fn kat_reduce_alpha_gives_r() {
    let k = field_sqrt2();
    let alpha = k.alpha();
    let p = bi(7);
    let r = bi(3);
    let residue = alpha.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(3), "reduce(α) mod (7, α−3) should be 3");
}

// ─── KAT 2: reduce(constant) = constant mod p ────────────────────────────────

/// Reducing the rational constant 5 mod (7, α − 3) gives 5.
///
/// A constant element has poly = [5], so evaluation at any r gives 5.
#[test]
fn kat_reduce_constant_gives_constant_mod_p() {
    let k = field_sqrt2();
    let five = k.from_int(bi(5));
    let p = bi(7);
    let r = bi(3);
    let residue = five.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(5), "reduce(5) mod (7, α−3) should be 5");
}

// ─── KAT 3: reduce(1 + α) = (1 + r) mod p ───────────────────────────────────

/// Reducing 1 + α mod (7, α − 3) gives 4.
///
/// poly = [1, 1]; evaluation at r=3: 1 + 1·3 = 4.
#[test]
fn kat_reduce_1_plus_alpha() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    let alpha = k.alpha();
    let beta = one.add(&alpha); // 1 + α
    let p = bi(7);
    let r = bi(3);
    let residue = beta.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(4), "reduce(1+α) mod (7, α−3) should be 4");
}

// ─── KAT 4: reduce(1 − α) = (1 − r) mod p ───────────────────────────────────

/// Reducing 1 − α mod (7, α − 3) gives 5.
///
/// poly = [1, −1]; evaluation at r=3: 1 + (−1)·3 = −2 ≡ 5 (mod 7).
#[test]
fn kat_reduce_1_minus_alpha() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    let alpha = k.alpha();
    let beta = one.sub(&alpha); // 1 − α
    let p = bi(7);
    let r = bi(3);
    let residue = beta.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(5), "reduce(1−α) mod (7, α−3) should be 5");
}

// ─── KAT 5: product consistency ──────────────────────────────────────────────

/// reduce((1+α)(1−α)) = reduce(1+α) · reduce(1−α) mod p.
///
/// (1+α)(1−α) = 1 − α² = 1 − 2 = −1 in ℚ(√2).
/// reduce(−1) = −1 mod 7 = 6.
/// reduce(1+α) · reduce(1−α) = 4 · 5 = 20 ≡ 6 (mod 7). ✓
#[test]
fn kat_reduce_product_matches_product_of_residues() {
    let k = field_sqrt2();
    let one = k.from_int(bi(1));
    let alpha = k.alpha();
    let lhs = one.add(&alpha); // 1 + α
    let rhs = one.sub(&alpha); // 1 − α
    let product = lhs.mul(&rhs); // (1+α)(1−α) = −1 in K

    let p = bi(7);
    let r = bi(3);

    // Reduce the product directly.
    let residue_product = product.reduce_mod_ideal(&p, &r);
    // Reduce each factor and multiply mod p.
    let res_lhs = lhs.reduce_mod_ideal(&p, &r);
    let res_rhs = rhs.reduce_mod_ideal(&p, &r);
    let expected = (res_lhs * res_rhs) % &p;

    assert_eq!(
        residue_product, expected,
        "reduce((1+α)(1−α)) should equal reduce(1+α)·reduce(1−α) mod 7"
    );
    // Also verify the concrete value: −1 mod 7 = 6.
    assert_eq!(residue_product, bi(6), "reduce(−1) mod 7 should be 6");
}

// ─── KAT 6: rational element with denominator ────────────────────────────────

/// Reducing the rational 1/2 mod (7, α − 3) gives 4.
///
/// 2⁻¹ mod 7 = 4 (since 2·4 = 8 ≡ 1 mod 7).
#[test]
fn kat_reduce_rational_with_denominator() {
    let k = field_sqrt2();
    // Build element 1/2 as a constant rational.
    let half = k.from_rational(br(1, 2));
    let p = bi(7);
    let r = bi(3);
    let residue = half.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(4), "reduce(1/2) mod (7, α−3) should be 4 (= 2⁻¹ mod 7)");
}

// ─── KAT 7: element with rational coefficient involving α ────────────────────

/// Reducing α/2 mod (7, α − 3) gives 5.
///
/// poly = [0, 1/2]; evaluation at r=3: 0 + (1/2)·3 = 3/2 = 3·(2⁻¹) = 3·4 = 12 ≡ 5 (mod 7).
#[test]
fn kat_reduce_alpha_over_2() {
    let k = field_sqrt2();
    // Build element α/2 directly from a RatPoly.
    let alpha_over_2 = NumberFieldElement {
        field: &k,
        poly: shared_numfield::RatPoly::from_coeffs(vec![bri(0), br(1, 2)]),
    };
    let p = bi(7);
    let r = bi(3);
    let residue = alpha_over_2.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(5), "reduce(α/2) mod (7, α−3) should be 5");
}

// ─── KAT 8: zero element ─────────────────────────────────────────────────────

/// Reducing the zero element gives 0.
#[test]
fn kat_reduce_zero() {
    let k = field_sqrt2();
    let zero = k.from_int(bi(0));
    let p = bi(7);
    let r = bi(3);
    let residue = zero.reduce_mod_ideal(&p, &r);
    assert_eq!(residue, bi(0), "reduce(0) mod (7, α−3) should be 0");
}

// ─── KAT 9: result is always in [0, p) ───────────────────────────────────────

/// All residues are in [0, p).
#[test]
fn kat_reduce_result_in_range() {
    let k = field_sqrt2();
    let p = bi(7);
    let r = bi(3);
    let elements: Vec<NumberFieldElement<'_>> = vec![
        k.from_int(bi(0)),
        k.from_int(bi(1)),
        k.from_int(bi(-1)),
        k.from_int(bi(6)),
        k.from_int(bi(7)),
        k.from_int(bi(14)),
        k.alpha(),
    ];
    for elem in &elements {
        let res = elem.reduce_mod_ideal(&p, &r);
        assert!(
            res >= bi(0) && res < p,
            "residue {res} is not in [0, 7) for element {:?}",
            elem
        );
    }
}
