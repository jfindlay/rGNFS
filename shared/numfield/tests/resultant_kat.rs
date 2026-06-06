//! Known-answer tests (KATs) for resultant and subresultant GCD — session G.A.2.
//!
//! Contract C-Res: verifies `resultant`, `subresultant_gcd`, and `IntPoly::pseudo_div_rem`.

use num_bigint::BigInt;
use shared_numfield::{IntPoly, resultant, subresultant_gcd};

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn ip(coeffs: Vec<i64>) -> IntPoly {
    IntPoly::from_coeffs(coeffs.into_iter().map(BigInt::from).collect())
}

// ─── pseudo_div_rem KATs ──────────────────────────────────────────────────────

#[test]
fn pseudo_div_rem_monic_divisor_exact() {
    // (x² − 1) ÷ (x − 1) = (x + 1) remainder 0.
    // lc(g) = 1, so pseudo-div equals ordinary div.
    let f = ip(vec![-1, 0, 1]); // x² − 1
    let g = ip(vec![-1, 1]); // x − 1
    let (q, r) = f.pseudo_div_rem(&g);
    assert_eq!(r, IntPoly::zero(), "remainder should be 0");
    assert_eq!(q, ip(vec![1, 1]), "quotient should be x + 1");
}

#[test]
fn pseudo_div_rem_non_monic_divisor() {
    // f = 2x² + 3x + 1, g = 2x + 1.
    // lc(g) = 2, e = 2, multiplier = 4.
    // 4f = 8x² + 12x + 4 = (4x + 4)(2x + 1) + 0.
    let f = ip(vec![1, 3, 2]); // 2x² + 3x + 1
    let g = ip(vec![1, 2]); // 2x + 1
    let (q, r) = f.pseudo_div_rem(&g);
    assert_eq!(r, IntPoly::zero(), "remainder should be 0");
    assert_eq!(q, ip(vec![4, 4]), "quotient should be 4x + 4");
}

#[test]
fn pseudo_div_rem_with_nonzero_remainder() {
    // f = x², g = x + 1.
    // x² = (x − 1)(x + 1) + 1.
    let f = ip(vec![0, 0, 1]); // x²
    let g = ip(vec![1, 1]); // x + 1
    let (q, r) = f.pseudo_div_rem(&g);
    assert_eq!(q, ip(vec![-1, 1]), "quotient should be x − 1");
    assert_eq!(r, ip(vec![1]), "remainder should be 1");
}

#[test]
fn pseudo_div_rem_degree_less_than_divisor() {
    // deg(f) < deg(g) → quotient = 0, remainder = f.
    let f = ip(vec![3, 1]); // x + 3
    let g = ip(vec![-1, 0, 1]); // x² − 1
    let (q, r) = f.pseudo_div_rem(&g);
    assert_eq!(q, IntPoly::zero());
    assert_eq!(r, f);
}

// ─── Resultant KAT 1: Res(x²−1, x−1) = 0 ────────────────────────────────────
//
// f = x² − 1 and g = x − 1 share the root x = 1, so their resultant is 0.

#[test]
fn kat_resultant_1_shared_root() {
    let f = ip(vec![-1, 0, 1]); // x² − 1
    let g = ip(vec![-1, 1]); // x − 1
    let r = resultant(&f, &g);
    assert_eq!(r, bi(0), "Res(x²−1, x−1) should be 0 (shared root x=1)");
}

// ─── Resultant KAT 2: Res(x²−2, x²−3) = 1 ───────────────────────────────────
//
// For two monic quadratics f = x² + a, g = x² + b: Res(f, g) = (a − b)².
// Here a = −2, b = −3: Res = (−2 − (−3))² = 1² = 1.

#[test]
fn kat_resultant_2_coprime_quadratics() {
    let f = ip(vec![-2, 0, 1]); // x² − 2
    let g = ip(vec![-3, 0, 1]); // x² − 3
    let r = resultant(&f, &g);
    assert_eq!(r, bi(1), "Res(x²−2, x²−3) should be 1");
}

// ─── Resultant KAT 3: norm consistency ───────────────────────────────────────
//
// For f = x² − 2 (minimal polynomial of √2) and g = x + 1:
//   Res_x(f, x+1) = f(−1) = (−1)² − 2 = −1.
// This cross-checks G.A.1a: Norm_{ℚ(√2)/ℚ}(1 + α) = Res(f, x+1) = −1.

#[test]
fn kat_resultant_3_norm_consistency() {
    let f = ip(vec![-2, 0, 1]); // x² − 2  (minimal poly of √2)
    let g = ip(vec![1, 1]); // x + 1  (represents element 1+α evaluated at α)
    let r = resultant(&f, &g);
    assert_eq!(r, bi(-1), "Res(x²−2, x+1) should be −1 (= Norm(1+√2))");
}

// ─── GCD KAT 1: gcd(x²−1, x−1) ∝ x−1 ───────────────────────────────────────
//
// x² − 1 = (x+1)(x−1), so gcd(x²−1, x−1) = x − 1 (up to sign/content).

#[test]
fn kat_gcd_1_shared_factor() {
    let f = ip(vec![-1, 0, 1]); // x² − 1
    let g = ip(vec![-1, 1]); // x − 1
    let d = subresultant_gcd(&f, &g);
    // The result should be a primitive polynomial proportional to x − 1.
    // Primitive part of (x − 1) is (x − 1) itself (leading coeff positive).
    assert_eq!(d.degree(), Some(1), "gcd should have degree 1");
    // Verify d divides both f and g by checking pseudo-remainder is zero.
    let (_, r_f) = f.pseudo_div_rem(&d);
    let (_, r_g) = g.pseudo_div_rem(&d);
    assert_eq!(r_f, IntPoly::zero(), "d should divide x²−1");
    assert_eq!(r_g, IntPoly::zero(), "d should divide x−1");
}

// ─── GCD KAT 2: gcd(x²−2, x²−3) is a constant ───────────────────────────────
//
// x²−2 and x²−3 are coprime over ℤ (their resultant is 1 ≠ 0), so their
// primitive GCD is a constant (degree 0).

#[test]
fn kat_gcd_2_coprime_polynomials() {
    let f = ip(vec![-2, 0, 1]); // x² − 2
    let g = ip(vec![-3, 0, 1]); // x² − 3
    let d = subresultant_gcd(&f, &g);
    assert_eq!(d.degree(), Some(0), "gcd of coprime polynomials should be a constant");
}
