//! Known-answer tests for the hyperelliptic curve and Mumford divisor representation.
//!
//! Coverage:
//! - Point-on-curve: `is_on_curve` holds for known affine points.
//! - Zero divisor: `[1, 0]` is valid and reports `is_zero()`.
//! - Mumford reduced-divisor invariant: `u` monic, `deg v < deg u ≤ g`,
//!   `u | (f − v·h − v²)` for divisors built from one and two points.
//! - Divisor-from-points round-trip: build `[u,v]` from points, recover the
//!   points as roots of `u` with `y = v(xᵢ)`.
//! - Genus: `g = ⌊(deg f − 1)/2⌋ = 2` for the toy curve.
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
use rho::hyperelliptic::{eval_poly, HyperellipticCurve, MumfordDivisor};
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
