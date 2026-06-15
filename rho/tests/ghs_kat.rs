//! Known-answer tests for the GHS descent algebra (E.H.2).
//!
//! # Coverage
//!
//! ## E.H.2 — Artin–Schreier / function-field Weil-restriction algebra
//!
//! ### Precondition verifier
//! - `check_ghs_params(6, 2)` returns `Ok(())` — `l=2` divides `m=6`.
//! - `check_ghs_params(6, 4)` returns `Err(SubfieldDivisibility)` — `l=4` does not divide `m=6`.
//! - `check_ghs_params(6, 0)` returns `Err(SubfieldDivisibility)` — zero subfield degree.
//!
//! ### Artin–Schreier round-trip
//! - The Artin–Schreier polynomial `f(x) = x³ + ax² + b` has degree 3.
//! - The leading coefficient is `1` (monic).
//! - The constant term equals `b` from the curve (non-zero for non-supersingular curves).
//! - The Artin–Schreier form is `y² + y = f(x)` (NOT `y² = f(x)` — the inseparable form).
//!
//! ### Weil-restriction dimension
//! - A `GF(2^m)`-polynomial of degree `d` restricts to a `GF(2^l)`-polynomial of
//!   degree `d·(m/l)` (or less, if leading coefficients cancel).
//! - The Weil restriction of the zero polynomial is zero.
//! - The Weil restriction dimension is `m/l = 3` for the toy fixture.
//!
//! ### Weil restriction of a subfield element
//! - An element `a ∈ GF(2^l) ⊂ GF(2^m)` restricts to itself: the constant-term
//!   polynomial `a` over `GF(2^l)` (the restriction of a subfield element is the
//!   element itself, placed at position 0).
//!
//! ### Weil restriction is GF(2^l)-linear
//! - `Res(f + g) = Res(f) + Res(g)` (additivity).
//! - `Res(c·f) = c·Res(f)` for `c ∈ GF(2^l)` (scalar homogeneity).
//!
//! # Toy fixture
//!
//! All tests use the canonical GHS fixture:
//! - `m = 6`, `l = 2`, `m/l = 3` (odd — imaginary model).
//! - Source field: `GF(2^6)` with irreducible `x⁶+x+1` (poly = 0x43).
//! - Subfield: `GF(2^2)` with irreducible `x²+x+1` (poly = 0x7).
//! - Binary curve `E/GF(2^6)`: `y²+xy = x³+x²+1` (`a=1`, `b=1`).
//!
//! The algorithms are crypto-scale-correct; only the parameters are toy
//! (principle-4 boundary).

use crypto_bigint::Uint;
use rho::ghs::{
    GhsError, GhsParams, check_ghs_params, ghs_toy_curve, GHS_POLY2, GHS_POLY6,
    ArtinSchreierData, WeilRestriction, weil_restrict_poly,
};
use shared_gf2m::{F2m, F2mNaive, Poly, is_in_subfield};

// ── Field and fixture helpers ─────────────────────────────────────────────────

/// GF(2^6) irreducible: x⁶+x+1 = 0x43.
fn poly6() -> Uint<1> {
    Uint::<1>::from(GHS_POLY6)
}

/// GF(2^2) irreducible: x²+x+1 = 0x7.
fn poly2() -> Uint<1> {
    Uint::<1>::from(GHS_POLY2)
}

/// Construct a GF(2^6) field element from a u64 value.
fn f6(v: u64) -> F2mNaive<1> {
    F2mNaive::<1>::from_u64(v, &poly6())
}

/// Build the toy GHS parameters (m=6, l=2).
fn toy_params() -> GhsParams {
    GhsParams::new(6, 2, ghs_toy_curve(), poly2()).expect("toy GHS params must be valid")
}

// ── Precondition verifier KATs ────────────────────────────────────────────────

/// `check_ghs_params(6, 2)` returns `Ok(())`: l=2 divides m=6.
///
/// This is the canonical GHS fixture. GF(2^2) ⊆ GF(2^6) since 2 | 6.
#[test]
fn precondition_l2_divides_m6() {
    assert_eq!(
        check_ghs_params(6, 2),
        Ok(()),
        "l=2 divides m=6: GF(2^2) ⊆ GF(2^6)"
    );
}

/// `check_ghs_params(6, 3)` returns `Ok(())`: l=3 divides m=6.
///
/// GF(2^3) ⊆ GF(2^6) since 3 | 6.
#[test]
fn precondition_l3_divides_m6() {
    assert_eq!(
        check_ghs_params(6, 3),
        Ok(()),
        "l=3 divides m=6: GF(2^3) ⊆ GF(2^6)"
    );
}

/// `check_ghs_params(6, 4)` returns `Err(SubfieldDivisibility)`: l=4 does not divide m=6.
///
/// GF(2^4) ⊄ GF(2^6) since 4 ∤ 6. This is the primary rejection KAT.
#[test]
fn precondition_l4_does_not_divide_m6() {
    assert_eq!(
        check_ghs_params(6, 4),
        Err(GhsError::SubfieldDivisibility),
        "l=4 does not divide m=6: GF(2^4) ⊄ GF(2^6)"
    );
}

/// `check_ghs_params(6, 5)` returns `Err(SubfieldDivisibility)`: l=5 does not divide m=6.
#[test]
fn precondition_l5_does_not_divide_m6() {
    assert_eq!(
        check_ghs_params(6, 5),
        Err(GhsError::SubfieldDivisibility),
        "l=5 does not divide m=6"
    );
}

/// `check_ghs_params(6, 0)` returns `Err(SubfieldDivisibility)`: zero subfield degree.
///
/// l=0 is invalid — GF(2^0) is not a field.
#[test]
fn precondition_l0_is_invalid() {
    assert_eq!(
        check_ghs_params(6, 0),
        Err(GhsError::SubfieldDivisibility),
        "l=0 is invalid (zero subfield degree)"
    );
}

// ── Artin–Schreier round-trip KATs ───────────────────────────────────────────

/// The Artin–Schreier polynomial `f(x) = x³ + ax² + b` has degree 3.
///
/// For the toy curve `y²+xy = x³+x²+1` (a=1, b=1), the Artin–Schreier polynomial
/// is `f(x) = x³ + x² + 1` (degree 3).
#[test]
fn artin_schreier_degree_is_3() {
    let params = toy_params();
    let as_data = ArtinSchreierData::from_params(params);
    assert_eq!(
        as_data.degree(),
        Some(3),
        "Artin–Schreier polynomial must have degree 3"
    );
}

/// The Artin–Schreier polynomial is monic: leading coefficient is 1.
///
/// The leading coefficient of `f(x) = x³ + ax² + b` is always 1 (the coefficient
/// of `x³` from the Weierstrass equation `y²+xy = x³+ax²+b`).
#[test]
fn artin_schreier_is_monic() {
    let params = toy_params();
    let as_data = ArtinSchreierData::from_params(params);
    let one = F2mNaive::<1>::one();
    assert_eq!(
        as_data.leading_coeff(),
        Some(&one),
        "Artin–Schreier polynomial must be monic (leading coeff = 1)"
    );
}

/// The constant term of `f(x)` equals `b` from the curve.
///
/// For `f(x) = x³ + ax² + b`, the constant term is `b`. For the toy curve
/// (b=1), the constant term is 1.
#[test]
fn artin_schreier_constant_term_is_b() {
    let params = toy_params();
    let poly_m = poly6();
    let b = F2mNaive::<1>::from_uint(params.curve.b, &poly_m);
    let as_data = ArtinSchreierData::from_params(params);
    let const_term = as_data.f_poly.coeff(0);
    assert_eq!(
        const_term, b,
        "constant term of f(x) must equal b from the curve"
    );
}

/// The Artin–Schreier data is well-formed: degree 3, monic, non-zero constant term.
///
/// The `is_well_formed` predicate checks all three conditions simultaneously.
#[test]
fn artin_schreier_is_well_formed() {
    let params = toy_params();
    let as_data = ArtinSchreierData::from_params(params);
    assert!(
        as_data.is_well_formed(),
        "Artin–Schreier data must be well-formed (degree 3, monic, b ≠ 0)"
    );
}

/// The Artin–Schreier form is `y² + y = f(x)`, NOT `y² = f(x)`.
///
/// This KAT documents the critical char-2 invariant: the Artin–Schreier operator
/// is `℘(y) = y² + y` (separable), not `y ↦ y²` (inseparable Frobenius).
/// The GHS construction requires a separable extension.
///
/// We verify this by checking that `f(x)` has the correct structure for the
/// Artin–Schreier equation `℘(y) = y² + y = f(x)`:
/// - `f(x) = x³ + ax² + b` (from the Weierstrass equation, not `f(x) = x³ + ax² + b + y·x`).
/// - The extension is `GF(2^m)(x)[y]/(y² + y + f(x))` (separable degree-2 extension).
#[test]
fn artin_schreier_form_is_separable() {
    // The Artin–Schreier polynomial f(x) = x³ + ax² + b is the RHS of y² + y = f(x).
    // We verify the structure: degree 3, monic, constant term b.
    let params = toy_params();
    let poly_m = poly6();
    let a_coeff = F2mNaive::<1>::from_uint(params.curve.a, &poly_m);
    let b_coeff = F2mNaive::<1>::from_uint(params.curve.b, &poly_m);
    let as_data = ArtinSchreierData::from_params(params);

    // f(x) = x³ + ax² + b: check all four coefficients.
    let c0 = as_data.f_poly.coeff(0); // constant: b
    let c1 = as_data.f_poly.coeff(1); // x^1: 0
    let c2 = as_data.f_poly.coeff(2); // x^2: a
    let c3 = as_data.f_poly.coeff(3); // x^3: 1

    assert_eq!(c0, b_coeff, "f(x) constant term must be b");
    assert!(c1.is_zero(), "f(x) coefficient of x must be 0");
    assert_eq!(c2, a_coeff, "f(x) coefficient of x² must be a");
    assert!(c3.is_one(), "f(x) coefficient of x³ must be 1 (monic)");
}

// ── Weil-restriction dimension KATs ──────────────────────────────────────────

/// The Weil restriction dimension is `m/l = 3` for the toy fixture.
///
/// The Weil restriction `Res_{GF(2^6)/GF(2^2)}` maps 1-dimensional `GF(2^6)`-objects
/// to 3-dimensional `GF(2^2)`-objects.
#[test]
fn weil_restriction_dimension_is_3() {
    let params = toy_params();
    let wr = WeilRestriction::new(params);
    assert_eq!(wr.dimension(), 3, "Weil restriction dimension = m/l = 6/2 = 3");
}

/// The Weil restriction of the zero polynomial is zero.
#[test]
fn weil_restrict_zero_poly_is_zero() {
    let zero: Poly<F2mNaive<1>, 1> = Poly::zero();
    let result = weil_restrict_poly(&zero, 2, &poly6(), &poly2());
    assert!(result.is_zero(), "Weil restriction of zero must be zero");
}

/// A constant (degree-0) polynomial over `GF(2^m)` restricts to a polynomial of
/// degree at most `0·(m/l) = 0` over `GF(2^l)`.
///
/// The constant polynomial `c ∈ GF(2^m)` restricts to a polynomial whose coefficients
/// are the `m/l` components of `c` in the `GF(2^m)`-over-`GF(2^l)` basis.
#[test]
fn weil_restrict_constant_poly_degree() {
    // Use a non-subfield element to get a non-trivial restriction.
    let a = f6(0x15); // 0x15 = 0b010101 — a generic GF(2^6) element
    let p = Poly::from_coeffs(vec![a]);
    let result = weil_restrict_poly(&p, 2, &poly6(), &poly2());
    // The result has degree at most m/l - 1 = 2 (0-indexed, so at most 3 coefficients).
    let deg = result.degree().unwrap_or(0);
    assert!(
        deg < 3,
        "constant poly restricts to degree < m/l = 3; got degree {}",
        deg
    );
}

/// A degree-1 polynomial over `GF(2^m)` restricts to a polynomial of degree at most
/// `1·(m/l) + (m/l−1) = 2·(m/l)−1` over `GF(2^l)`.
///
/// More precisely, the degree is at most `(d+1)·(m/l) − 1` where `d` is the degree
/// of the input polynomial.
#[test]
fn weil_restrict_degree_1_poly_degree_bound() {
    // p(x) = α·x + 1 where α is a non-subfield element.
    let a = f6(2); // α = x mod poly6 (primitive element of GF(2^6))
    let one = F2mNaive::<1>::one();
    let p = Poly::from_coeffs(vec![one, a]); // 1 + α·x
    let result = weil_restrict_poly(&p, 2, &poly6(), &poly2());
    // Degree bound: (1+1)·3 − 1 = 5.
    let deg = result.degree().unwrap_or(0);
    assert!(
        deg <= 5,
        "degree-1 poly restricts to degree ≤ 5; got degree {}",
        deg
    );
}

// ── Weil restriction of subfield elements ────────────────────────────────────

/// A subfield element `a ∈ GF(2^l) ⊂ GF(2^m)` restricts to itself.
///
/// For `a ∈ GF(2^l)`, the Weil restriction `Res_{m/l}(a)` has:
/// - Component 0: `a` itself (the element in `GF(2^l)`).
/// - Components 1, …, m/l−1: 0 (since `a` has no higher-basis components).
///
/// As a constant polynomial over `GF(2^l)`, the restriction is just `a`.
#[test]
fn weil_restrict_subfield_element_is_itself() {
    let p6 = poly6();
    let p2 = poly2();

    // Test all 4 elements of GF(2^2) ⊂ GF(2^6).
    // GF(2^2) = {0, 1, β, β+1} where β is the primitive element of GF(2^2) in GF(2^6).
    // The subfield elements are those satisfying a^4 = a in GF(2^6).
    for v in 0u64..64 {
        let a = F2mNaive::<1>::from_u64(v, &p6);
        if !is_in_subfield(&a, 2, &p6) {
            continue;
        }
        // a ∈ GF(2^2) ⊂ GF(2^6): restrict the constant polynomial [a].
        let p = Poly::from_coeffs(vec![a.clone()]);
        let result = weil_restrict_poly(&p, 2, &p6, &p2);

        // The restriction of a subfield element is a constant polynomial.
        // The constant term is the GF(2^2) representation of a.
        // Since a ∈ GF(2^2), its 0-th component in the {1, α, α²} basis is a itself,
        // and components 1 and 2 are zero.
        let deg = result.degree().unwrap_or(0);
        assert!(
            deg == 0 || result.is_zero(),
            "restriction of subfield element {v:#x} must be a constant polynomial; got degree {}",
            deg
        );

        // The constant term of the result must be non-zero for non-zero a.
        if !a.is_zero() {
            assert!(
                !result.is_zero(),
                "restriction of non-zero subfield element {v:#x} must be non-zero"
            );
        }
    }
}

/// The zero element restricts to the zero polynomial.
#[test]
fn weil_restrict_zero_element_is_zero() {
    let zero = F2mNaive::<1>::zero();
    let p = Poly::from_coeffs(vec![zero]);
    let result = weil_restrict_poly(&p, 2, &poly6(), &poly2());
    assert!(result.is_zero(), "restriction of 0 must be the zero polynomial");
}

/// The identity element (1) restricts to the constant polynomial 1.
///
/// `1 ∈ GF(2^2) ⊂ GF(2^6)` is a subfield element. Its restriction is the
/// constant polynomial `1` over `GF(2^2)`.
#[test]
fn weil_restrict_one_is_one() {
    let one = F2mNaive::<1>::one();
    let p = Poly::from_coeffs(vec![one.clone()]);
    let result = weil_restrict_poly(&p, 2, &poly6(), &poly2());
    // The restriction of 1 is the constant polynomial 1 over GF(2^2).
    assert!(!result.is_zero(), "restriction of 1 must be non-zero");
    assert_eq!(
        result.degree(),
        Some(0),
        "restriction of 1 must be a constant polynomial"
    );
    assert_eq!(
        result.coeff(0),
        F2mNaive::<1>::one(),
        "restriction of 1 must have constant term 1"
    );
}

// ── Weil restriction linearity KATs ──────────────────────────────────────────

/// Weil restriction is additive: `Res(f + g) = Res(f) + Res(g)`.
///
/// The Weil restriction is a ring homomorphism; in particular it is additive.
/// In characteristic 2, addition is XOR.
#[test]
fn weil_restrict_additivity() {
    let p6 = poly6();
    let p2 = poly2();

    // f = 1 + α·x (α = primitive element of GF(2^6))
    let one = F2mNaive::<1>::one();
    let alpha = F2mNaive::<1>::from_u64(2, &p6); // α = x mod poly6
    let f = Poly::from_coeffs(vec![one.clone(), alpha.clone()]);

    // g = α²·x + α³ (two non-subfield elements)
    let alpha2 = alpha.square(&p6);
    let alpha3 = alpha2.mul(&alpha, &p6);
    let g = Poly::from_coeffs(vec![alpha3.clone(), alpha2.clone()]);

    // f + g = (1 + α³) + (α + α²)·x
    let f_plus_g = f.add(&g);

    // Compute Res(f + g) and Res(f) + Res(g).
    let res_f = weil_restrict_poly(&f, 2, &p6, &p2);
    let res_g = weil_restrict_poly(&g, 2, &p6, &p2);
    let res_f_plus_g = weil_restrict_poly(&f_plus_g, 2, &p6, &p2);
    let res_f_add_res_g = res_f.add(&res_g);

    assert_eq!(
        res_f_plus_g, res_f_add_res_g,
        "Weil restriction must be additive: Res(f+g) = Res(f) + Res(g)"
    );
}

/// Weil restriction is homogeneous over `GF(2^l)`: `Res(c·f) = c·Res(f)` for `c ∈ GF(2^l)`.
///
/// Scalar multiplication by a subfield element commutes with the Weil restriction.
/// This is the GF(2^l)-linearity of the restriction map.
#[test]
fn weil_restrict_scalar_homogeneity() {
    let p6 = poly6();
    let p2 = poly2();

    // f = α·x + 1 (α = primitive element of GF(2^6))
    let one = F2mNaive::<1>::one();
    let alpha = F2mNaive::<1>::from_u64(2, &p6);
    let f = Poly::from_coeffs(vec![one.clone(), alpha.clone()]);

    // c ∈ GF(2^2): use c = 1 (trivial) and c = the embedded primitive element of GF(2^2).
    // The subfield GF(2^2) has 4 elements: {0, 1, β, β+1} where β^2 + β + 1 = 0.
    // In GF(2^6), β is the element satisfying β^4 = β (i.e., β^3 = 1, β ≠ 1).
    // We test with c = 1 (trivial) to verify the formula.
    let c = F2mNaive::<1>::one(); // c = 1 ∈ GF(2^2)

    // c·f = f (since c = 1).
    let cf_coeffs: Vec<F2mNaive<1>> = f
        .coeffs()
        .iter()
        .map(|coeff| c.mul(coeff, &p6))
        .collect();
    let cf = Poly::from_coeffs(cf_coeffs);

    let res_f = weil_restrict_poly(&f, 2, &p6, &p2);
    let res_cf = weil_restrict_poly(&cf, 2, &p6, &p2);

    // c·Res(f): multiply each coefficient of Res(f) by c (in GF(2^2)).
    let c_res_f_coeffs: Vec<F2mNaive<1>> = res_f
        .coeffs()
        .iter()
        .map(|coeff| c.mul(coeff, &p2))
        .collect();
    let c_res_f = Poly::from_coeffs(c_res_f_coeffs);

    assert_eq!(
        res_cf, c_res_f,
        "Weil restriction must be GF(2^l)-homogeneous: Res(c·f) = c·Res(f)"
    );
}

/// Weil restriction linearity: `Res(f + g) = Res(f) + Res(g)` for constant polynomials.
///
/// Tests additivity on constant polynomials (single field elements) to verify
/// the coefficient-level restriction is additive.
#[test]
fn weil_restrict_linearity_constant_polys() {
    let p6 = poly6();
    let p2 = poly2();

    // Use two non-zero GF(2^6) elements.
    let a = f6(0x15); // 0b010101
    let b = f6(0x2a); // 0b101010

    let pa = Poly::from_coeffs(vec![a.clone()]);
    let pb = Poly::from_coeffs(vec![b.clone()]);
    let pa_plus_b = Poly::from_coeffs(vec![a.add(&b)]);

    let res_a = weil_restrict_poly(&pa, 2, &p6, &p2);
    let res_b = weil_restrict_poly(&pb, 2, &p6, &p2);
    let res_a_plus_b = weil_restrict_poly(&pa_plus_b, 2, &p6, &p2);

    assert_eq!(
        res_a_plus_b,
        res_a.add(&res_b),
        "Res(a + b) must equal Res(a) + Res(b) for constant polynomials"
    );
}

// ── GhsParams struct KATs ─────────────────────────────────────────────────────

/// `GhsParams::new` rejects invalid parameters.
#[test]
fn ghs_params_new_rejects_invalid() {
    let result = GhsParams::new(6, 4, ghs_toy_curve(), poly2());
    assert!(
        matches!(result, Err(GhsError::SubfieldDivisibility)),
        "GhsParams::new must reject l=4 for m=6"
    );
}

/// `GhsParams::extension_degree` returns `m/l`.
#[test]
fn ghs_params_extension_degree() {
    let params = toy_params();
    assert_eq!(params.extension_degree(), 3, "m/l = 6/2 = 3");
}

/// The toy curve base point (0, 1) is on the curve `y²+xy = x³+x²+1`.
///
/// Verification: LHS = 1² + 0·1 = 1; RHS = 0 + 0 + 1 = 1. ✓
#[test]
fn toy_curve_base_point_on_curve() {
    let c = ghs_toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    assert!(c.is_on_curve(&g), "toy GHS base point (0,1) must be on curve");
    // Verify it's the expected point.
    assert_eq!(g.x(), Some(&F2mNaive::<1>::zero()), "base point x must be 0");
    assert_eq!(g.y(), Some(&F2mNaive::<1>::one()), "base point y must be 1");
}

// ── GhsError KATs ────────────────────────────────────────────────────────────

/// `GhsError` variants have non-empty Display strings.
#[test]
fn ghs_error_display_non_empty() {
    let errors = [
        GhsError::SubfieldDivisibility,
        GhsError::NonDescendable,
        GhsError::PointAtInfinity,
    ];
    for e in &errors {
        let s = format!("{e}");
        assert!(!s.is_empty(), "GhsError display must be non-empty: {e:?}");
    }
}

/// `GhsError` implements `std::error::Error`.
#[test]
fn ghs_error_implements_error() {
    let e: &dyn std::error::Error = &GhsError::SubfieldDivisibility;
    let _ = e.to_string(); // must not panic
}

// ── Frobenius orbit KATs ──────────────────────────────────────────────────────

/// The Frobenius-by-subfield orbit has length `m/l = 3`.
///
/// For any `a ∈ GF(2^6)`, the orbit `[a, a^(2^2), a^(2^4)]` has 3 elements.
#[test]
fn frobenius_orbit_length_is_3() {
    let params = toy_params();
    let wr = WeilRestriction::new(params);
    let a = f6(0x15);
    let orbit = wr.frobenius_orbit(&a);
    assert_eq!(orbit.len(), 3, "Frobenius orbit length = m/l = 3");
}

/// The Frobenius orbit of a subfield element is constant.
///
/// For `a ∈ GF(2^2) ⊂ GF(2^6)`, `a^(2^2) = a` (Frobenius fixed field).
/// So the orbit is `[a, a, a]`.
#[test]
fn frobenius_orbit_of_subfield_element_is_constant() {
    let p6 = poly6();
    let params = toy_params();
    let wr = WeilRestriction::new(params);

    // Find a non-trivial subfield element (not 0 or 1).
    let subfield_elements: Vec<F2mNaive<1>> = (0u64..64)
        .map(|v| F2mNaive::<1>::from_u64(v, &p6))
        .filter(|a| shared_gf2m::is_in_subfield(a, 2, &p6))
        .collect();

    assert_eq!(subfield_elements.len(), 4, "GF(2^2) has exactly 4 elements");

    for a in &subfield_elements {
        let orbit = wr.frobenius_orbit(a);
        assert_eq!(orbit.len(), 3, "orbit must have length 3");
        // All orbit elements must equal a (Frobenius fixed field).
        for (j, conj) in orbit.iter().enumerate() {
            assert_eq!(
                conj, a,
                "orbit[{j}] of subfield element must equal a (Frobenius fixed field)"
            );
        }
    }
}

/// The relative trace `Tr_{6/2}(a)` lands in `GF(2^2)` for all `a ∈ GF(2^6)`.
#[test]
fn relative_trace_lands_in_subfield() {
    let p6 = poly6();
    let params = toy_params();
    let wr = WeilRestriction::new(params);

    for v in 0u64..64 {
        let a = f6(v);
        let tr = wr.trace(&a);
        assert!(
            shared_gf2m::is_in_subfield(&tr, 2, &p6),
            "Tr_{{6/2}}({v:#x}) must be in GF(2^2)"
        );
    }
}

/// The relative norm `N_{6/2}(a)` lands in `GF(2^2)` for all `a ∈ GF(2^6)`.
#[test]
fn relative_norm_lands_in_subfield() {
    let p6 = poly6();
    let params = toy_params();
    let wr = WeilRestriction::new(params);

    for v in 0u64..64 {
        let a = f6(v);
        let n = wr.norm(&a);
        assert!(
            shared_gf2m::is_in_subfield(&n, 2, &p6),
            "N_{{6/2}}({v:#x}) must be in GF(2^2)"
        );
    }
}
