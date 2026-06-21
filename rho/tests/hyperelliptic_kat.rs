//! Known-answer tests for the hyperelliptic curve, Mumford divisor representation,
//! and Cantor's Jacobian group law.
//!
//! # Coverage
//!
//! ## E.I.2 — curve and Mumford representation
//! - Point-on-curve: `is_on_curve` holds for known affine points.
//! - Zero divisor: `[1, 0]` is valid and reports `is_zero()`.
//! - Mumford reduced-divisor invariant: `u` monic, `deg v < deg u ≤ g`,
//!   `u | (f − v·h − v²)` for divisors built from one and two points.
//! - Divisor-from-points round-trip: build `[u,v]` from points, recover the
//!   points as roots of `u` with `y = v(xᵢ)`.
//! - Genus: `g = ⌊(deg f − 1)/2⌋ = 2` for the toy curve.
//!
//! ## Cantor group law (Jacobian group law for hyperelliptic curves)
//! - Identity axiom: `D + 0 = D` and `0 + D = D`.
//! - Negation: `D + (−D) = 0` with `−D = [u, (h+v) mod u]`.
//! - Associativity: `(D₁+D₂)+D₃ = D₁+(D₂+D₃)` on a sample.
//! - Cantor consistency: `2D` via doubling equals `D + D` via compose.
//! - Divisor-order law: `n·D = 0` for a divisor of known order `n`.
//! - Reduced-divisor invariant: every result of Cantor add is a valid
//!   reduced divisor (`deg u ≤ g`, Mumford invariant holds).
//! - Optional PARI cross-check (`#[ignore]`-gated).
//!
//! # Toy curve
//!
//! All tests use the genus-2 curve `y² + x·y = x⁵ + x³ + 1` over GF(2^4)
//! with irreducible `x⁴+x+1` (poly = 0x13).
//!
//! Known affine points (enumerated by exhaustive search):
//! ```text
//! (0,1), (1,6), (1,7), (2,8), (2,10), (3,12), (3,15),
//! (4,8), (4,12), (5,10), (5,15), (6,1), (6,7), (7,1), (7,6),
//! (9,5), (9,12), (11,3), (11,8), (13,2), (13,15), (14,4), (14,10)
//! ```
//! Total: 23 affine points.
//!
//! The algorithms are arbitrary-genus-correct; only the parameters are toy
//! (principle-4 boundary).

use crypto_bigint::Uint;
use rho::hyperelliptic::{cantor, eval_poly, HyperellipticCurve, MumfordDivisor};
use shared_gf2m::{F2m, F2mNaive, Poly};

// ── Curve and field parameters ────────────────────────────────────────────────

/// GF(2^4) irreducible: x⁴+x+1 = 0x13.
fn poly4() -> Uint<1> {
    Uint::<1>::from(0x13u64)
}

/// Toy genus-2 curve: `y² + x·y = x⁵ + x³ + 1` over GF(2^4) with `x⁴+x+1`.
///
/// - `h(x) = x`  (h_coeffs = [0, 1])
/// - `f(x) = x⁵ + x³ + 1`  (f_coeffs = [1, 0, 0, 1, 0, 1])
/// - `deg f = 5`, genus `g = (5−1)/2 = 2`.
fn toy_curve() -> HyperellipticCurve<1> {
    HyperellipticCurve::new(
        poly4(),
        vec![Uint::<1>::ZERO, Uint::<1>::ONE], // h = x
        vec![
            Uint::<1>::ONE,  // x^0: 1
            Uint::<1>::ZERO, // x^1: 0
            Uint::<1>::ZERO, // x^2: 0
            Uint::<1>::ONE,  // x^3: 1
            Uint::<1>::ZERO, // x^4: 0
            Uint::<1>::ONE,  // x^5: 1
        ],
    )
}

/// Construct a GF(2^4) field element from a u64 value.
fn f4(v: u64) -> F2mNaive<1> {
    F2mNaive::<1>::from_u64(v, &poly4())
}

// ── Genus KAT ─────────────────────────────────────────────────────────────────

/// Genus is 2 for `deg f = 5`: `g = ⌊(5−1)/2⌋ = 2`.
#[test]
fn genus_is_two() {
    let c = toy_curve();
    assert_eq!(c.genus(), 2, "genus should be 2 for deg f = 5");
}

// ── Point-on-curve KATs ───────────────────────────────────────────────────────

/// Point (0, 1) is on the curve.
///
/// LHS: 1² + 0·1 = 1 + 0 = 1.
/// RHS: f(0) = 0⁵ + 0³ + 1 = 1. ✓
#[test]
fn point_0_1_is_on_curve() {
    let c = toy_curve();
    assert!(c.is_on_curve(&f4(0), &f4(1)), "(0,1) should be on curve");
}

/// Point (1, 6) is on the curve.
///
/// LHS: 6² + 1·6 = 7 + 6 = 1 (in GF(2^4)).
/// RHS: f(1) = 1 + 1 + 1 = 1. ✓
#[test]
fn point_1_6_is_on_curve() {
    let c = toy_curve();
    assert!(c.is_on_curve(&f4(1), &f4(6)), "(1,6) should be on curve");
}

/// Point (1, 7) is on the curve.
///
/// LHS: 7² + 1·7 = 6 + 7 = 1 (in GF(2^4)).
/// RHS: f(1) = 1. ✓
/// Note: (1,6) and (1,7) are the two y-values for x=1 (conjugate pair).
#[test]
fn point_1_7_is_on_curve() {
    let c = toy_curve();
    assert!(c.is_on_curve(&f4(1), &f4(7)), "(1,7) should be on curve");
}

/// Point (2, 8) is on the curve.
#[test]
fn point_2_8_is_on_curve() {
    let c = toy_curve();
    assert!(c.is_on_curve(&f4(2), &f4(8)), "(2,8) should be on curve");
}

/// Point (3, 12) is on the curve.
#[test]
fn point_3_12_is_on_curve() {
    let c = toy_curve();
    assert!(c.is_on_curve(&f4(3), &f4(12)), "(3,12) should be on curve");
}

/// A non-point (0, 0) is not on the curve.
///
/// LHS: 0² + 0·0 = 0.
/// RHS: f(0) = 1 ≠ 0. ✗
#[test]
fn non_point_0_0_not_on_curve() {
    let c = toy_curve();
    assert!(!c.is_on_curve(&f4(0), &f4(0)), "(0,0) should not be on curve");
}

/// All 23 known affine points are on the curve.
///
/// Exhaustive check against the enumerated point list.
#[test]
fn all_known_points_are_on_curve() {
    let c = toy_curve();
    let known_points: &[(u64, u64)] = &[
        (0, 1),
        (1, 6),
        (1, 7),
        (2, 8),
        (2, 10),
        (3, 12),
        (3, 15),
        (4, 8),
        (4, 12),
        (5, 10),
        (5, 15),
        (6, 1),
        (6, 7),
        (7, 1),
        (7, 6),
        (9, 5),
        (9, 12),
        (11, 3),
        (11, 8),
        (13, 2),
        (13, 15),
        (14, 4),
        (14, 10),
    ];
    for &(x, y) in known_points {
        assert!(
            c.is_on_curve(&f4(x), &f4(y)),
            "({},{}) should be on curve",
            x,
            y
        );
    }
}

/// Non-points are not on the curve.
///
/// Spot-checks a few (x, y) pairs that are not on the curve.
#[test]
fn non_points_not_on_curve() {
    let c = toy_curve();
    // (0, 0): f(0) = 1, but 0² + 0·0 = 0 ≠ 1.
    assert!(!c.is_on_curve(&f4(0), &f4(0)));
    // (1, 0): f(1) = 1, but 0² + 1·0 = 0 ≠ 1.
    assert!(!c.is_on_curve(&f4(1), &f4(0)));
    // (2, 0): not on curve.
    assert!(!c.is_on_curve(&f4(2), &f4(0)));
}

// ── Zero divisor KATs ─────────────────────────────────────────────────────────

/// The zero divisor `[1, 0]` is valid and reports `is_zero()`.
///
/// The zero divisor is the group identity.  It satisfies all Mumford invariants:
/// - `u = 1` is monic, `deg u = 0 ≤ g = 2`.
/// - `v = 0`, `deg v` is undefined (zero polynomial), vacuously `< deg u`.
/// - `u | (f − v·h − v²)`: `1 | anything` trivially.
#[test]
fn zero_divisor_is_valid() {
    let c = toy_curve();
    let zero: MumfordDivisor<F2mNaive<1>, 1> = c.zero_divisor();
    assert!(c.is_valid(&zero), "zero divisor [1, 0] must be valid");
    assert!(zero.is_zero(), "zero divisor must report is_zero()");
}

/// The zero divisor has `u = 1` (degree 0) and `v = 0`.
#[test]
fn zero_divisor_structure() {
    let c = toy_curve();
    let zero: MumfordDivisor<F2mNaive<1>, 1> = c.zero_divisor();
    assert_eq!(zero.u, Poly::one(), "zero divisor u must be 1");
    assert!(zero.v.is_zero(), "zero divisor v must be 0");
    assert_eq!(zero.u.degree(), Some(0), "zero divisor u has degree 0");
}

// ── Mumford invariant KATs ────────────────────────────────────────────────────

/// Divisor from one point (2, 8): degree-1 divisor.
///
/// `u = x + 2` (monic, degree 1 ≤ g = 2).
/// `v = 8` (constant, degree 0 < 1 = deg u).
/// Invariant: `u | (f − v·h − v²)`.
#[test]
fn divisor_from_one_point_is_valid() {
    let c = toy_curve();
    let p = vec![(f4(2), f4(8))];
    let div = c.divisor_from_points::<F2mNaive<1>>(&p);

    // u must be monic of degree 1.
    assert_eq!(div.u.degree(), Some(1), "deg u should be 1 for one point");
    assert_eq!(
        div.u.leading_coeff().unwrap(),
        &f4(1),
        "u must be monic"
    );

    // v must have degree < 1 (i.e., degree 0 or zero).
    let deg_v = div.v.degree().unwrap_or(0);
    assert!(deg_v < 1, "deg v must be < deg u = 1");

    // v(2) = 8: the interpolant passes through the point.
    let v_at_x = div.eval_v(&f4(2), &poly4());
    assert_eq!(v_at_x, f4(8), "v(2) should equal y = 8");

    // Full validity check (includes u | f − v·h − v²).
    assert!(c.is_valid(&div), "divisor from one point must be valid");
}

/// Divisor from two points (2, 8) and (3, 12): degree-2 divisor.
///
/// `u = (x+2)(x+3)` (monic, degree 2 = g).
/// `v` = Lagrange interpolant with `v(2) = 8`, `v(3) = 12` (degree ≤ 1).
/// Invariant: `u | (f − v·h − v²)`.
#[test]
fn divisor_from_two_points_is_valid() {
    let c = toy_curve();
    let p1 = (f4(2), f4(8));
    let p2 = (f4(3), f4(12));
    let div = c.divisor_from_points::<F2mNaive<1>>(&[p1, p2]);

    // u must be monic of degree 2 (= g).
    assert_eq!(div.u.degree(), Some(2), "deg u should be 2 for two points");
    assert_eq!(
        div.u.leading_coeff().unwrap(),
        &f4(1),
        "u must be monic"
    );

    // deg v < 2.
    let deg_v = div.v.degree().unwrap_or(0);
    assert!(deg_v < 2, "deg v must be < deg u = 2");

    // v(2) = 8 and v(3) = 12.
    let v_at_2 = div.eval_v(&f4(2), &poly4());
    let v_at_3 = div.eval_v(&f4(3), &poly4());
    assert_eq!(v_at_2, f4(8), "v(2) should equal y1 = 8");
    assert_eq!(v_at_3, f4(12), "v(3) should equal y2 = 12");

    // Full validity check.
    assert!(c.is_valid(&div), "divisor from two points must be valid");
}

/// Mumford invariant holds explicitly: `(f − v·h − v²) mod u = 0`.
///
/// Verifies the curve-compatibility condition directly (not just via `is_valid`).
#[test]
fn mumford_invariant_explicit() {
    let c = toy_curve();
    let p1 = (f4(2), f4(8));
    let p2 = (f4(3), f4(12));
    let div = c.divisor_from_points::<F2mNaive<1>>(&[p1, p2]);

    let poly = poly4();
    let f_poly = c.f::<F2mNaive<1>>();
    let h_poly = c.h::<F2mNaive<1>>();

    // Compute f − v·h − v²  (in char 2: sub = add).
    let v_sq = div.v.mul(&div.v, &poly);
    let vh = div.v.mul(&h_poly, &poly);
    let rhs = f_poly.add(&vh).add(&v_sq);

    // Remainder must be zero.
    let (_, rem) = rhs.divmod(&div.u, &poly);
    assert!(rem.is_zero(), "u must divide f − v·h − v² (Mumford invariant)");
}

/// Degree bounds: `deg u ≤ g` and `deg v < deg u`.
///
/// Checks both degree bounds for the two-point divisor.
#[test]
fn mumford_degree_bounds() {
    let c = toy_curve();
    let g = c.genus();
    let p1 = (f4(2), f4(8));
    let p2 = (f4(3), f4(12));
    let div = c.divisor_from_points::<F2mNaive<1>>(&[p1, p2]);

    let deg_u = div.u.degree().expect("u must be nonzero");
    assert!(deg_u <= g, "deg u = {} must be ≤ g = {}", deg_u, g);

    if !div.v.is_zero() {
        let deg_v = div.v.degree().expect("v is nonzero");
        assert!(deg_v < deg_u, "deg v = {} must be < deg u = {}", deg_v, deg_u);
    }
}

// ── Divisor-from-points round-trip KATs ──────────────────────────────────────

/// Round-trip for a one-point divisor: build `[u,v]` from (2, 8), recover the point.
///
/// The root of `u` is `x = 2`, and `v(2) = 8 = y`.
#[test]
fn round_trip_one_point() {
    let c = toy_curve();
    let x = f4(2);
    let y = f4(8);
    let div = c.divisor_from_points::<F2mNaive<1>>(&[(x.clone(), y.clone())]);

    // u(2) = 0: x = 2 is a root of u.
    let u_at_x = div.eval_u(&x, &poly4());
    assert!(u_at_x.is_zero(), "u(2) must be 0 (x=2 is a root of u)");

    // v(2) = 8: the y-coordinate is recovered.
    let v_at_x = div.eval_v(&x, &poly4());
    assert_eq!(v_at_x, y, "v(2) must equal y = 8");
}

/// Round-trip for a two-point divisor: build `[u,v]` from (2,8) and (3,12),
/// recover both points.
///
/// The roots of `u` are `x = 2` and `x = 3`; `v(xᵢ) = yᵢ`.
#[test]
fn round_trip_two_points() {
    let c = toy_curve();
    let p1 = (f4(2), f4(8));
    let p2 = (f4(3), f4(12));
    let div = c.divisor_from_points::<F2mNaive<1>>(&[p1.clone(), p2.clone()]);

    // u(2) = 0 and u(3) = 0.
    let u_at_x1 = div.eval_u(&p1.0, &poly4());
    let u_at_x2 = div.eval_u(&p2.0, &poly4());
    assert!(u_at_x1.is_zero(), "u(2) must be 0");
    assert!(u_at_x2.is_zero(), "u(3) must be 0");

    // v(2) = 8 and v(3) = 12.
    let y1_recovered = div.eval_v(&p1.0, &poly4());
    let y2_recovered = div.eval_v(&p2.0, &poly4());
    assert_eq!(y1_recovered, p1.1, "v(2) must equal y1 = 8");
    assert_eq!(y2_recovered, p2.1, "v(3) must equal y2 = 12");
}

/// Round-trip for a two-point divisor using different points: (1, 6) and (7, 1).
///
/// Verifies the round-trip works for a second pair of points.
#[test]
fn round_trip_two_points_alt() {
    let c = toy_curve();
    let p1 = (f4(1), f4(6));
    let p2 = (f4(7), f4(1));
    assert!(c.is_on_curve(&p1.0, &p1.1), "p1 must be on curve");
    assert!(c.is_on_curve(&p2.0, &p2.1), "p2 must be on curve");

    let div = c.divisor_from_points::<F2mNaive<1>>(&[p1.clone(), p2.clone()]);

    // Validity.
    assert!(c.is_valid(&div), "divisor must be valid");

    // Round-trip.
    let u_at_x1 = div.eval_u(&p1.0, &poly4());
    let u_at_x2 = div.eval_u(&p2.0, &poly4());
    assert!(u_at_x1.is_zero(), "u(1) must be 0");
    assert!(u_at_x2.is_zero(), "u(7) must be 0");

    let y1_recovered = div.eval_v(&p1.0, &poly4());
    let y2_recovered = div.eval_v(&p2.0, &poly4());
    assert_eq!(y1_recovered, p1.1, "v(1) must equal y1 = 6");
    assert_eq!(y2_recovered, p2.1, "v(7) must equal y2 = 1");
}

// ── Polynomial evaluation KAT ─────────────────────────────────────────────────

/// `eval_poly` correctly evaluates `f(x) = x⁵ + x³ + 1` at known points.
///
/// - `f(0) = 1` (constant term).
/// - `f(1) = 1 + 1 + 1 = 1` (in GF(2^4): 1 XOR 1 XOR 1 = 1).
#[test]
fn eval_poly_known_values() {
    let c = toy_curve();
    let f_poly = c.f::<F2mNaive<1>>();

    // f(0) = constant term = 1.
    let f_at_0 = eval_poly(&f_poly, &f4(0), &poly4());
    assert_eq!(f_at_0, f4(1), "f(0) should be 1");

    // f(1) = 1 + 1 + 1 = 1 in GF(2^4) (XOR: 1^1^1 = 1).
    let f_at_1 = eval_poly(&f_poly, &f4(1), &poly4());
    assert_eq!(f_at_1, f4(1), "f(1) should be 1");
}

// ── Conjugate pair KAT ────────────────────────────────────────────────────────

/// For each x with two y-values, the two y-values satisfy `y₁ + y₂ = h(x)`.
///
/// In char 2, the two roots of `y² + h(x)·y − f(x) = 0` sum to `h(x)` (by
/// Vieta's formulas in char 2: `y₁ + y₂ = −h(x) = h(x)`).
#[test]
fn conjugate_pair_sum_equals_h() {
    let c = toy_curve();
    let poly = poly4();
    let h_poly = c.h::<F2mNaive<1>>();

    // Conjugate pairs: (1,6)/(1,7), (2,8)/(2,10), (3,12)/(3,15).
    let pairs: &[(u64, u64, u64)] = &[
        (1, 6, 7),
        (2, 8, 10),
        (3, 12, 15),
    ];

    for &(x_val, y1_val, y2_val) in pairs {
        let x = f4(x_val);
        let y1 = f4(y1_val);
        let y2 = f4(y2_val);

        // Both points are on the curve.
        assert!(c.is_on_curve(&x, &y1), "({},{}) must be on curve", x_val, y1_val);
        assert!(c.is_on_curve(&x, &y2), "({},{}) must be on curve", x_val, y2_val);

        // y₁ + y₂ = h(x) in char 2.
        let hx = eval_poly(&h_poly, &x, &poly);
        let y_sum = y1.add(&y2);
        assert_eq!(y_sum, hx, "y1+y2 should equal h(x) for x={}", x_val);
    }
}

// ── Cantor group-law KATs ─────────────────────────────────────────────────────

/// Helper: build the primary test divisor D₁ = [(x+2)(x+3), v] from (2,8) and (3,12).
fn cantor_d1() -> MumfordDivisor<F2mNaive<1>, 1> {
    let c = toy_curve();
    c.divisor_from_points::<F2mNaive<1>>(&[(f4(2), f4(8)), (f4(3), f4(12))])
}

/// Helper: build a second test divisor D₂ from (1,6) and (7,1).
fn cantor_d2() -> MumfordDivisor<F2mNaive<1>, 1> {
    let c = toy_curve();
    c.divisor_from_points::<F2mNaive<1>>(&[(f4(1), f4(6)), (f4(7), f4(1))])
}

/// Helper: build a third test divisor D₃ from (4,8) and (5,10).
fn cantor_d3() -> MumfordDivisor<F2mNaive<1>, 1> {
    let c = toy_curve();
    c.divisor_from_points::<F2mNaive<1>>(&[(f4(4), f4(8)), (f4(5), f4(10))])
}

/// Identity axiom (left): `0 + D = D`.
///
/// Adding the zero divisor on the left must return `D` unchanged.
#[test]
fn cantor_identity_left() {
    let c = toy_curve();
    let poly = poly4();
    let zero: MumfordDivisor<F2mNaive<1>, 1> = c.zero_divisor();
    let d = cantor_d1();
    let result = cantor::add(&c, &zero, &d, &poly);
    assert_eq!(result, d, "0 + D must equal D (left identity)");
}

/// Identity axiom (right): `D + 0 = D`.
///
/// Adding the zero divisor on the right must return `D` unchanged.
#[test]
fn cantor_identity_right() {
    let c = toy_curve();
    let poly = poly4();
    let zero: MumfordDivisor<F2mNaive<1>, 1> = c.zero_divisor();
    let d = cantor_d1();
    let result = cantor::add(&c, &d, &zero, &poly);
    assert_eq!(result, d, "D + 0 must equal D (right identity)");
}

/// Negation: `D + (−D) = 0`.
///
/// The negation is `−D = [u, (h+v) mod u]`, NOT `[u, −v]`.
/// In char 2, `−v = v`, but the hyperelliptic involution sends
/// `(x, y) → (x, y+h(x))`, so the divisor negation reflects `v → h+v`.
/// This KAT is the loud signal for the char-2 negation trap.
#[test]
fn cantor_negate_then_add_is_zero() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let neg_d = cantor::negate(&c, &d, &poly);
    let result = cantor::add(&c, &d, &neg_d, &poly);
    assert!(result.is_zero(), "D + (−D) must be the identity [1,0]");
}

/// Negation of the identity is the identity: `−0 = 0`.
#[test]
fn cantor_negate_zero_is_zero() {
    let c = toy_curve();
    let poly = poly4();
    let zero: MumfordDivisor<F2mNaive<1>, 1> = c.zero_divisor();
    let neg_zero = cantor::negate(&c, &zero, &poly);
    assert!(neg_zero.is_zero(), "−0 must be 0");
}

/// Negation is an involution: `−(−D) = D`.
#[test]
fn cantor_double_negate_is_identity() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let neg_d = cantor::negate(&c, &d, &poly);
    let neg_neg_d = cantor::negate(&c, &neg_d, &poly);
    assert_eq!(neg_neg_d, d, "−(−D) must equal D");
}

/// Associativity: `(D₁+D₂)+D₃ = D₁+(D₂+D₃)`.
///
/// Checks the group law is associative on a concrete sample of three
/// distinct degree-2 divisors.
#[test]
fn cantor_associativity() {
    let c = toy_curve();
    let poly = poly4();
    let d1 = cantor_d1();
    let d2 = cantor_d2();
    let d3 = cantor_d3();

    let lhs = cantor::add(&c, &cantor::add(&c, &d1, &d2, &poly), &d3, &poly);
    let rhs = cantor::add(&c, &d1, &cantor::add(&c, &d2, &d3, &poly), &poly);
    assert_eq!(lhs, rhs, "(D₁+D₂)+D₃ must equal D₁+(D₂+D₃)");
}

/// Cantor consistency: `2D` via doubling equals `D + D` via compose.
///
/// `scalar_mul(D, 2)` must equal `add(D, D)`.  This guards against a
/// compose-without-reduce fracture (an unreduced divisor is not a group element).
#[test]
fn cantor_double_consistency() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let double_add = cantor::add(&c, &d, &d, &poly);
    let double_scalar = cantor::scalar_mul(&c, &d, 2, &poly);
    assert_eq!(double_add, double_scalar, "D+D must equal 2·D via scalar_mul");
}

/// Every result of Cantor add is a valid reduced divisor.
///
/// Checks `deg u ≤ g` and the Mumford invariant `u | (f − v·h − v²)` for
/// several sums.
#[test]
fn cantor_results_are_valid_reduced_divisors() {
    let c = toy_curve();
    let poly = poly4();
    let d1 = cantor_d1();
    let d2 = cantor_d2();
    let d3 = cantor_d3();

    let sums = [
        cantor::add(&c, &d1, &d2, &poly),
        cantor::add(&c, &d1, &d3, &poly),
        cantor::add(&c, &d2, &d3, &poly),
        cantor::add(&c, &d1, &d1, &poly), // doubling
        cantor::add(&c, &d2, &d2, &poly),
    ];

    for (i, sum) in sums.iter().enumerate() {
        assert!(
            c.is_valid(sum),
            "sum[{}] must be a valid reduced divisor",
            i
        );
    }
}

/// Divisor-order law: `n·D = 0` for a divisor of known order `n`.
///
/// The order of `D₁` in the Jacobian is found by iterating `k·D₁` until
/// the identity is reached (brute-force, feasible for the toy GF(2^4) group
/// whose order is at most ~300).  The test then verifies `n·D₁ = 0`.
///
/// This is a self-contained KAT: no PARI required.
#[test]
fn cantor_divisor_order_law() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();

    // Find the order of D by iterating D, 2D, 3D, ... until we reach 0.
    // The Jacobian of a genus-2 curve over GF(2^4) has order at most ~300,
    // so this terminates quickly.
    let mut current = d.clone();
    let mut order: u64 = 1;
    loop {
        if current.is_zero() {
            break;
        }
        current = cantor::add(&c, &current, &d, &poly);
        order += 1;
        assert!(order <= 1000, "order search exceeded 1000 — something is wrong");
    }

    // Verify: n·D = 0.
    let result = cantor::scalar_mul(&c, &d, order, &poly);
    assert!(
        result.is_zero(),
        "{}·D must be the identity (divisor order law); order = {}",
        order,
        order
    );
}

/// Scalar multiplication: `k·D` for small scalars matches iterated addition.
///
/// Verifies `3·D = D + D + D` and `4·D = D + D + D + D`.
#[test]
fn cantor_scalar_mul_matches_iterated_add() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();

    // 3·D = D + D + D.
    let three_d_add = cantor::add(&c, &cantor::add(&c, &d, &d, &poly), &d, &poly);
    let three_d_scalar = cantor::scalar_mul(&c, &d, 3, &poly);
    assert_eq!(three_d_add, three_d_scalar, "3·D via add must equal 3·D via scalar_mul");

    // 4·D = D + D + D + D.
    let four_d_add = cantor::add(&c, &three_d_add, &d, &poly);
    let four_d_scalar = cantor::scalar_mul(&c, &d, 4, &poly);
    assert_eq!(four_d_add, four_d_scalar, "4·D via add must equal 4·D via scalar_mul");
}

/// Scalar multiplication by 0 returns the identity.
#[test]
fn cantor_scalar_mul_zero_is_identity() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let result = cantor::scalar_mul(&c, &d, 0, &poly);
    assert!(result.is_zero(), "0·D must be the identity");
}

/// Scalar multiplication by 1 returns D unchanged.
#[test]
fn cantor_scalar_mul_one_is_d() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let result = cantor::scalar_mul(&c, &d, 1, &poly);
    assert_eq!(result, d, "1·D must equal D");
}

/// Commutativity: `D₁ + D₂ = D₂ + D₁`.
///
/// The Jacobian is an abelian group; addition is commutative.
#[test]
fn cantor_commutativity() {
    let c = toy_curve();
    let poly = poly4();
    let d1 = cantor_d1();
    let d2 = cantor_d2();
    let lhs = cantor::add(&c, &d1, &d2, &poly);
    let rhs = cantor::add(&c, &d2, &d1, &poly);
    assert_eq!(lhs, rhs, "D₁+D₂ must equal D₂+D₁ (commutativity)");
}

/// Negation structure: `−D = [u, (h+v) mod u]`.
///
/// Verifies the negation formula directly: the `u` component is unchanged,
/// and the `v` component is `(h+v) mod u`.
#[test]
fn cantor_negate_formula() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let neg_d = cantor::negate(&c, &d, &poly);

    // u is unchanged.
    assert_eq!(neg_d.u, d.u, "negate must preserve u");

    // v_neg = (h + v) mod u.
    let h = c.h::<F2mNaive<1>>();
    let hv = h.add(&d.v);
    let (_, expected_v) = hv.divmod(&d.u, &poly);
    assert_eq!(neg_d.v, expected_v, "negate: v must be (h+v) mod u");
}

/// The negation of D is a valid reduced divisor.
#[test]
fn cantor_negate_is_valid() {
    let c = toy_curve();
    let poly = poly4();
    let d = cantor_d1();
    let neg_d = cantor::negate(&c, &d, &poly);
    assert!(c.is_valid(&neg_d), "−D must be a valid reduced divisor");
}

/// Optional PARI cross-check: verify the Jacobian group order via `hyperellcharpoly`.
///
/// This test is gated with `#[ignore]` and requires PARI/GP to be installed.
/// Run manually with:
/// ```text
/// cargo test -p rho cantor_pari_cross_check -- --ignored
/// ```
///
/// Expected PARI session:
/// ```text
/// ? K = GF(2^4, 'a, a^4+a+1);
/// ? C = hyperelliptic(Pol([1,0,0,1,0,1]*Mod(1,2)), Pol([0,1]*Mod(1,2)));
/// ? hyperellcharpoly(C)   \\ L-polynomial
/// ```
/// The group order `#Jac(C/GF(2^4))` equals `L(1)` where `L` is the L-polynomial.
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn cantor_pari_cross_check() {
    // Placeholder: when PARI is available, verify that the order found by
    // `cantor_divisor_order_law` divides the Jacobian group order returned by
    // `hyperellcharpoly`.
    //
    // The toy curve `y² + xy = x⁵ + x³ + 1` over GF(2^4) with x⁴+x+1.
    // PARI command:
    //   K = GF(2^4, 'a, a^4+a+1);
    //   C = hyperelliptic(Pol([1,0,0,1,0,1]*Mod(1,2)), Pol([0,1]*Mod(1,2)));
    //   hyperellcharpoly(C)
    unimplemented!("run manually with PARI installed");
}
