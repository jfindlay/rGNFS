//! Known-answer tests (KATs) for Z/p^k arithmetic in `shared-padic`.
//!
//! All KATs use p = 7, k = 4 (modulus = 7^4 = 2401) unless otherwise noted.
//!
//! KAT coverage:
//! - `add` / `sub` / `mul` against hand-computed residues.
//! - `inv` correct on a unit (v_7(3) = 0 → 3 * 1601 ≡ 1 mod 2401).
//! - `inv` errors loudly on a non-unit (v_7(7) = 1 → `ZpError::NonUnit`).
//! - `inv` errors loudly on zero (v_7(0) = ∞ → `ZpError::NonUnit`).
//! - `valuation` correct on sample elements (v_7(3) = 0, v_7(7) = 1, v_7(49) = 2, v_7(0) = ∞).
//! - Precision `lift` and `truncate` between precisions.
//! - Precision mixing: arithmetic on elements with different k truncates to the minimum.

use shared_padic::{Zp, ZpError};

// ─── helpers ──────────────────────────────────────────────────────────────────

/// Construct a Z/7^k element from an i64 value.
fn z7(value: i64, k: u32) -> Zp {
    Zp::from_i64(value, 7, k).expect("valid Zp construction")
}

// ─── add / sub / mul KATs ─────────────────────────────────────────────────────

/// add: 100 + 200 = 300 mod 7^4 = 300.
#[test]
fn kat_add_basic() {
    let a = z7(100, 4);
    let b = z7(200, 4);
    let c = a.add(&b).expect("add should succeed");
    assert_eq!(*c.residue(), num_bigint::BigInt::from(300), "100 + 200 mod 2401 = 300");
}

/// add wraps: 2300 + 200 = 2500 mod 2401 = 99.
#[test]
fn kat_add_wrap() {
    let a = z7(2300, 4);
    let b = z7(200, 4);
    let c = a.add(&b).expect("add should succeed");
    // 2300 + 200 = 2500; 2500 mod 2401 = 99
    assert_eq!(*c.residue(), num_bigint::BigInt::from(99), "2300 + 200 mod 2401 = 99");
}

/// sub: 300 - 100 = 200 mod 7^4 = 200.
#[test]
fn kat_sub_basic() {
    let a = z7(300, 4);
    let b = z7(100, 4);
    let c = a.sub(&b).expect("sub should succeed");
    assert_eq!(*c.residue(), num_bigint::BigInt::from(200), "300 - 100 mod 2401 = 200");
}

/// sub wraps: 50 - 100 = -50 ≡ 2351 mod 2401.
#[test]
fn kat_sub_wrap() {
    let a = z7(50, 4);
    let b = z7(100, 4);
    let c = a.sub(&b).expect("sub should succeed");
    // -50 mod 2401 = 2401 - 50 = 2351
    assert_eq!(*c.residue(), num_bigint::BigInt::from(2351), "50 - 100 mod 2401 = 2351");
}

/// mul: 3 * 5 = 15 mod 7^4 = 15.
#[test]
fn kat_mul_basic() {
    let a = z7(3, 4);
    let b = z7(5, 4);
    let c = a.mul(&b).expect("mul should succeed");
    assert_eq!(*c.residue(), num_bigint::BigInt::from(15), "3 * 5 mod 2401 = 15");
}

/// mul wraps: 100 * 200 = 20000 mod 2401.
///
/// 20000 / 2401 = 8 remainder 792 (8 * 2401 = 19208; 20000 - 19208 = 792).
#[test]
fn kat_mul_wrap() {
    let a = z7(100, 4);
    let b = z7(200, 4);
    let c = a.mul(&b).expect("mul should succeed");
    // 100 * 200 = 20000; 20000 mod 2401 = 792
    assert_eq!(*c.residue(), num_bigint::BigInt::from(792), "100 * 200 mod 2401 = 792");
}

// ─── unit inversion KATs ──────────────────────────────────────────────────────

/// inv(3) mod 7^4: v_7(3) = 0, so 3 is a unit.
///
/// Hand-computed: 3 * 1601 = 4803 = 2 * 2401 + 1 ≡ 1 mod 2401.
#[test]
fn kat_inv_unit() {
    let a = z7(3, 4);
    let inv_a = a.inv().expect("3 is a unit mod 7^4, inv must succeed");
    assert_eq!(*inv_a.residue(), num_bigint::BigInt::from(1601), "inv(3) mod 2401 = 1601");
    // Verify: 3 * 1601 ≡ 1 mod 2401.
    let product = a.mul(&inv_a).expect("mul should succeed");
    assert_eq!(*product.residue(), num_bigint::BigInt::from(1), "3 * inv(3) ≡ 1 mod 2401");
}

/// inv(1) mod 7^4: the multiplicative identity inverts to itself.
#[test]
fn kat_inv_one() {
    let a = z7(1, 4);
    let inv_a = a.inv().expect("1 is a unit mod 7^4");
    assert_eq!(*inv_a.residue(), num_bigint::BigInt::from(1), "inv(1) = 1");
}

/// inv(2400) mod 7^4: 2400 ≡ -1 mod 2401, so inv(-1) = -1 ≡ 2400.
#[test]
fn kat_inv_minus_one() {
    let a = z7(2400, 4); // -1 mod 2401
    let inv_a = a.inv().expect("-1 is a unit mod 7^4");
    assert_eq!(*inv_a.residue(), num_bigint::BigInt::from(2400), "inv(-1) = -1 ≡ 2400 mod 2401");
}

// ─── non-unit inversion guard (the prime-power non-field check) ───────────────

/// inv(7) mod 7^4 must error: v_7(7) = 1 > 0, so 7 is not a unit.
///
/// This is the load-bearing non-field guard: Z/p^k is not a field, and silently returning a
/// wrong inverse would corrupt every Hensel lift and p-adic log that follows.
#[test]
fn kat_inv_nonunit_p() {
    let a = z7(7, 4);
    let result = a.inv();
    assert!(
        matches!(result, Err(ZpError::NonUnit { valuation: 1, .. })),
        "inv(7) must return NonUnit with valuation 1, got: {result:?}"
    );
}

/// inv(49) mod 7^4 must error: v_7(49) = v_7(7^2) = 2 > 0.
#[test]
fn kat_inv_nonunit_p2() {
    let a = z7(49, 4);
    let result = a.inv();
    assert!(
        matches!(result, Err(ZpError::NonUnit { valuation: 2, .. })),
        "inv(49) must return NonUnit with valuation 2, got: {result:?}"
    );
}

/// inv(0) mod 7^4 must error: v_7(0) = ∞.
#[test]
fn kat_inv_zero() {
    let a = z7(0, 4);
    let result = a.inv();
    assert!(
        matches!(result, Err(ZpError::NonUnit { .. })),
        "inv(0) must return NonUnit (v_7(0) = ∞), got: {result:?}"
    );
}

// ─── valuation KATs ───────────────────────────────────────────────────────────

/// v_7(3) = 0: 3 is coprime to 7.
#[test]
fn kat_valuation_unit() {
    use shared_padic::zp::Valuation;
    let a = z7(3, 4);
    assert_eq!(a.valuation(), Valuation::Finite(0), "v_7(3) = 0");
}

/// v_7(7) = 1: 7 = 7^1.
#[test]
fn kat_valuation_p() {
    use shared_padic::zp::Valuation;
    let a = z7(7, 4);
    assert_eq!(a.valuation(), Valuation::Finite(1), "v_7(7) = 1");
}

/// v_7(49) = 2: 49 = 7^2.
#[test]
fn kat_valuation_p2() {
    use shared_padic::zp::Valuation;
    let a = z7(49, 4);
    assert_eq!(a.valuation(), Valuation::Finite(2), "v_7(49) = 2");
}

/// v_7(343) = 3: 343 = 7^3.
#[test]
fn kat_valuation_p3() {
    use shared_padic::zp::Valuation;
    let a = z7(343, 4);
    assert_eq!(a.valuation(), Valuation::Finite(3), "v_7(343) = 3");
}

/// v_7(0) = ∞ (the zero element is divisible by every power of p).
#[test]
fn kat_valuation_zero() {
    use shared_padic::zp::Valuation;
    let a = z7(0, 4);
    assert_eq!(a.valuation(), Valuation::Infinity, "v_7(0) = ∞");
}

/// v_7(14) = 1: 14 = 2 * 7.
#[test]
fn kat_valuation_composite() {
    use shared_padic::zp::Valuation;
    let a = z7(14, 4);
    assert_eq!(a.valuation(), Valuation::Finite(1), "v_7(14) = 1");
}

// ─── precision lift / truncate KATs ──────────────────────────────────────────

/// lift: 3 mod 7^2 lifted to k=4 has the same residue (3 < 7^2 < 7^4).
#[test]
fn kat_lift_basic() {
    let a = z7(3, 2); // 3 mod 49
    let lifted = a.lift(4).expect("lift from k=2 to k=4 should succeed");
    assert_eq!(lifted.precision(), 4, "lifted precision = 4");
    assert_eq!(*lifted.residue(), num_bigint::BigInt::from(3), "residue unchanged after lift");
}

/// truncate: 100 mod 7^4 truncated to k=2 reduces mod 49.
///
/// 100 mod 49 = 100 - 2*49 = 2.
#[test]
fn kat_truncate_basic() {
    let a = z7(100, 4); // 100 mod 2401
    let trunc = a.truncate(2).expect("truncate from k=4 to k=2 should succeed");
    assert_eq!(trunc.precision(), 2, "truncated precision = 2");
    // 100 mod 49 = 2
    assert_eq!(*trunc.residue(), num_bigint::BigInt::from(2), "100 mod 49 = 2");
}

/// truncate to k=1 reduces mod 7.
///
/// 100 mod 7 = 2 (100 = 14*7 + 2).
#[test]
fn kat_truncate_to_k1() {
    let a = z7(100, 4);
    let trunc = a.truncate(1).expect("truncate to k=1 should succeed");
    assert_eq!(trunc.precision(), 1, "truncated precision = 1");
    // 100 mod 7 = 2
    assert_eq!(*trunc.residue(), num_bigint::BigInt::from(2), "100 mod 7 = 2");
}

/// lift then truncate is identity on the residue.
#[test]
fn kat_lift_then_truncate() {
    let a = z7(5, 2);
    let lifted = a.lift(4).expect("lift should succeed");
    let back = lifted.truncate(2).expect("truncate should succeed");
    assert_eq!(*back.residue(), *a.residue(), "lift then truncate is identity on residue");
    assert_eq!(back.precision(), a.precision(), "precision restored");
}

/// lift with k2 < k errors.
#[test]
fn kat_lift_invalid() {
    let a = z7(3, 4);
    let result = a.lift(2);
    assert!(result.is_err(), "lift to lower precision must error");
}

/// truncate with k2 > k errors.
#[test]
fn kat_truncate_invalid() {
    let a = z7(3, 2);
    let result = a.truncate(4);
    assert!(result.is_err(), "truncate to higher precision must error");
}

// ─── precision mixing KATs ────────────────────────────────────────────────────

/// add with different precisions truncates to the minimum.
///
/// a = 100 mod 7^4 = 100; b = 50 mod 7^2 = 50.
/// min precision = 2; 100 mod 49 = 2; 2 + 50 = 52 mod 49 = 3.
#[test]
fn kat_add_mixed_precision() {
    let a = z7(100, 4); // 100 mod 2401
    let b = z7(50, 2);  // 50 mod 49
    let c = a.add(&b).expect("add with mixed precision should succeed");
    assert_eq!(c.precision(), 2, "result precision = min(4, 2) = 2");
    // 100 mod 49 = 2; 2 + 50 = 52; 52 mod 49 = 3
    assert_eq!(*c.residue(), num_bigint::BigInt::from(3), "mixed-precision add: 100+50 mod 49 = 3");
}

/// mul with different precisions truncates to the minimum.
///
/// a = 10 mod 7^4; b = 5 mod 7^2 = 5.
/// min precision = 2; 10 mod 49 = 10; 10 * 5 = 50 mod 49 = 1.
#[test]
fn kat_mul_mixed_precision() {
    let a = z7(10, 4);
    let b = z7(5, 2);
    let c = a.mul(&b).expect("mul with mixed precision should succeed");
    assert_eq!(c.precision(), 2, "result precision = min(4, 2) = 2");
    // 10 * 5 = 50; 50 mod 49 = 1
    assert_eq!(*c.residue(), num_bigint::BigInt::from(1), "mixed-precision mul: 10*5 mod 49 = 1");
}

// ─── prime mismatch guard ─────────────────────────────────────────────────────

/// add with different primes must error.
#[test]
fn kat_add_prime_mismatch() {
    let a = Zp::from_i64(3, 7, 4).expect("valid");
    let b = Zp::from_i64(3, 5, 4).expect("valid");
    let result = a.add(&b);
    assert!(
        matches!(result, Err(ZpError::PrimeMismatch { .. })),
        "add with different primes must return PrimeMismatch, got: {result:?}"
    );
}

// ─── construction guard ───────────────────────────────────────────────────────

/// Construction with p < 2 must error.
#[test]
fn kat_new_invalid_p() {
    let result = Zp::from_i64(1, 1, 4);
    assert!(result.is_err(), "p=1 must be rejected");
}

/// Construction with k = 0 must error.
#[test]
fn kat_new_invalid_k() {
    let result = Zp::from_i64(1, 7, 0);
    assert!(result.is_err(), "k=0 must be rejected");
}
