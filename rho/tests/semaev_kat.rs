//! Known-answer tests (KATs) for the Semaev module (E.J.1 + E.J.2).
//!
//! # Fixture
//!
//! All tests use the Semaev toy curve `y² = x³ + x + 33` over `F_47` (`p = 47`, `a = 1`,
//! `b = 33`). The generator `G = (10, 3)` satisfies the curve equation:
//! `10³ + 10 + 33 = 1043 = 9 = 3² mod 47 ✓`. The group order is `n = 60 = 2² · 3 · 5`
//! (verified by `n·G = ∞`).
//!
//! The `F_p[x]` resultant KATs use `F_47` arithmetic (the same prime as the toy fixture).
//! The multivariate polynomial KATs use `F_47` arithmetic.
//!
//! Known multiples of `G` (computed from the group law):
//! - `G  = (10, 3)`,  `−G  = (10, 44)`
//! - `2G = (7, 30)`,  `−2G = (7, 17)`
//! - `3G = (17, 13)`, `−3G = (17, 34)`
//! - `4G = (23, 12)`, `−4G = (23, 35)`
//! - `5G = (32, 36)`, `−5G = (32, 11)`
//!
//! # KAT coverage (E.J.1 — `F_p[x]` resultant)
//!
//! 1. **Resultant zero-iff-common-factor**: `Res(f, g) = 0 ⟺ gcd(f, g) ≠ 1`.
//!    - `Res(x-2, x-3) ≠ 0` (coprime linear factors).
//!    - `Res((x-2)(x-3), x-2) = 0` (share root x=2).
//!    - `Res((x-2)(x-3), (x-3)(x-5)) = 0` (share root x=3).
//! 2. **Resultant symmetry**: `Res(f, g) = (-1)^(deg_f * deg_g) * Res(g, f)`.
//!    - Verified for linear × linear (sign = -1).
//!    - Verified for linear × quadratic (even product → sign = +1).
//!    - Verified for quadratic × quadratic (even product → sign = +1).
//! 3. **Resultant of coprime quadratics is nonzero**: `Res(x²+1, x²+2) ≠ 0` over `F_47`
//!    (both irreducible over `F_47` — verified: -1 and -2 are QNR mod 47 since 47 ≡ 3 mod 4).
//! 4. **Resultant of zero polynomial is zero**: `Res(0, g) = 0`.
//! 5. **Resultant value spot-check**: `Res(x-2, x-3) = 1` over `F_47`.
//!
//! # KAT coverage (E.J.1 — multivariate/symmetric round-trip)
//!
//! 6. **Symmetric polynomial round-trip**: a symmetric polynomial round-trips through the
//!    `MultiPoly` representation (build → evaluate → same value).
//! 7. **Evaluation at permutation is invariant**: for a symmetric polynomial `f`,
//!    `f(a, b, c) = f(b, a, c) = f(c, b, a)` etc.
//! 8. **Partial evaluation**: fixing one variable of a 3-variable polynomial gives a
//!    2-variable polynomial with the correct evaluation.
//! 9. **`is_symmetric` correctly identifies symmetric polynomials**.
//! 10. **`symmetrize` produces a symmetric polynomial**.
//!
//! # KAT coverage (E.J.2 — `S_2`, `S_3` base cases + vanishing relation)
//!
//! 11. **`S_2` vanishing**: `S_2(x_1, x_2) = 0 ⟺ P_2 = −P_1`.
//!     - `S_2(10, 10) = 0` and `G + (−G) = ∞` ✓.
//!     - `S_2(10, 7) ≠ 0` and `G + 2G ≠ ∞` ✓.
//! 12. **`S_3` vanishing**: `S_3(x_1, x_2, x_3) = 0` for triples with `P_1+P_2+P_3 = ∞`.
//!     - `S_3(10, 7, 17) = 0`: x-coords of `G, 2G, −3G` (sum `∞`).
//!     - `S_3(10, 7, 23) ≠ 0`: x-coords of `G, 2G, 4G` — no y-values make sum `∞`.
//!     - `S_3(10, 10, 10) ≠ 0`: no y-values for x=10 make sum `∞`.
//!     - `S_3(10, 7, 32) ≠ 0`: x-coords of `G, 2G, 5G` — no y-values make sum `∞`.
//! 13. **`S_3` symmetry**: `S_3` is invariant under all permutations of its arguments.
//! 14. **`S_3` degree**: degree ≤ 2 in each variable.
//!
//! # Principle-4 boundary
//!
//! The fixture is toy-scale (`p = 47`, group order `n = 60`). The algorithms are
//! crypto-scale-correct; only the parameters are small for auditability.

use crypto_bigint::Uint;
use rho::field::{Fp, FpNaive};
use rho::semaev::poly::{FpPoly, MultiPoly, resultant};
use rho::semaev::base::{s3, vanishes_s2, vanishes_s3};
use rho::semaev::{SEMAEV_TOY_P, SemaevError, semaev_toy};
use rho::curve::AffinePoint;

// ─── helpers ──────────────────────────────────────────────────────────────────

/// The prime for the Semaev toy fixture: `p = 47`.
const P: u64 = SEMAEV_TOY_P; // 47

/// Return the `Uint<4>` modulus for the toy fixture.
fn p() -> Uint<4> {
    Uint::<4>::from(P)
}

/// Construct an `FpNaive` element from a `u64`.
fn fp(v: u64) -> FpNaive {
    FpNaive::from_u64(v, &p())
}

/// Construct a linear polynomial `x - root` over `F_47`.
///
/// `x - root = x + (p - root)` since `-root mod p = p - root`.
fn linear(root: u64) -> FpPoly<FpNaive> {
    let p_val = p();
    FpPoly::from_coeffs(
        vec![
            FpNaive::from_u64((P - root % P) % P, &p_val), // constant: -root mod p
            FpNaive::from_u64(1, &p_val),                   // leading: 1
        ],
        &p_val,
    )
}

/// Construct the product `(x - r1)(x - r2)` over `F_47`.
fn quadratic_with_roots(r1: u64, r2: u64) -> FpPoly<FpNaive> {
    let p_val = p();
    linear(r1).mul(&linear(r2), &p_val)
}

// ─── fixture verification ─────────────────────────────────────────────────────

/// Verify the toy curve fixture: generator on curve and n·G = ∞.
#[test]
fn semaev_toy_fixture_valid() {
    let c = semaev_toy();
    let g: AffinePoint<FpNaive> = c.generator();
    assert!(c.is_on_curve(&g), "semaev toy: generator not on curve");
    let ng = c.scalar_mul(&g, &c.n);
    assert!(ng.is_infinity(), "semaev toy: n·G should be ∞");
}

// ─── KAT 1: resultant zero-iff-common-factor ─────────────────────────────────

/// `Res(x-2, x-3) ≠ 0` — coprime linear factors over `F_47`.
#[test]
fn resultant_coprime_linears_nonzero() {
    let p_val = p();
    let f = linear(2);
    let g = linear(3);
    let res = resultant(&f, &g, &p_val).unwrap();
    assert!(
        !res.is_zero(&p_val),
        "Res(x-2, x-3) should be nonzero (coprime)"
    );
}

/// `Res((x-2)(x-3), x-2) = 0` — share root x=2.
#[test]
fn resultant_common_root_is_zero() {
    let p_val = p();
    let f = quadratic_with_roots(2, 3); // (x-2)(x-3)
    let g = linear(2);                  // x-2
    let res = resultant(&f, &g, &p_val).unwrap();
    assert!(
        res.is_zero(&p_val),
        "Res((x-2)(x-3), x-2) should be 0 (share root x=2)"
    );
}

/// `Res((x-2)(x-3), (x-3)(x-5)) = 0` — share root x=3.
#[test]
fn resultant_shared_root_quadratics_is_zero() {
    let p_val = p();
    let f = quadratic_with_roots(2, 3); // (x-2)(x-3)
    let g = quadratic_with_roots(3, 5); // (x-3)(x-5)
    let res = resultant(&f, &g, &p_val).unwrap();
    assert!(
        res.is_zero(&p_val),
        "Res((x-2)(x-3), (x-3)(x-5)) should be 0 (share root x=3)"
    );
}

/// `Res(0, g) = 0` — zero polynomial.
#[test]
fn resultant_zero_poly_is_zero() {
    let p_val = p();
    let f: FpPoly<FpNaive> = FpPoly::zero();
    let g = linear(5);
    let res = resultant(&f, &g, &p_val).unwrap();
    assert!(res.is_zero(&p_val), "Res(0, g) should be 0");
}

// ─── KAT 2: resultant symmetry ────────────────────────────────────────────────

/// `Res(f, g) = (-1)^(deg_f * deg_g) * Res(g, f)` — linear × linear (sign = -1).
///
/// For `f = x - 2`, `g = x - 3`: `deg_f * deg_g = 1 * 1 = 1` (odd) → `Res(f,g) = -Res(g,f)`.
/// `Res(x-a, x-b) = b - a` (the resultant of two monic linears is the difference of roots).
/// So `Res(x-2, x-3) = 3 - 2 = 1` and `Res(x-3, x-2) = 2 - 3 = -1 = 46 mod 47`.
#[test]
fn resultant_symmetry_linear_linear() {
    let p_val = p();
    let f = linear(2); // x - 2
    let g = linear(3); // x - 3
    let res_fg = resultant(&f, &g, &p_val).unwrap();
    let res_gf = resultant(&g, &f, &p_val).unwrap();
    // deg_f * deg_g = 1 * 1 = 1 (odd) → sign = -1 → Res(f,g) = -Res(g,f)
    let neg_res_gf = res_gf.neg(&p_val);
    assert_eq!(
        res_fg, neg_res_gf,
        "Res(f,g) should equal -Res(g,f) for linear × linear"
    );
}

/// `Res(f, g) = (-1)^(deg_f * deg_g) * Res(g, f)` — quadratic × quadratic (sign = +1).
///
/// For `f = (x-2)(x-3)`, `g = (x-5)(x-7)` (coprime): `deg_f * deg_g = 4` (even) →
/// `Res(f,g) = Res(g,f)`.
#[test]
fn resultant_symmetry_quadratic_quadratic() {
    let p_val = p();
    let f = quadratic_with_roots(2, 3); // (x-2)(x-3)
    let g = quadratic_with_roots(5, 7); // (x-5)(x-7)
    let res_fg = resultant(&f, &g, &p_val).unwrap();
    let res_gf = resultant(&g, &f, &p_val).unwrap();
    // deg_f * deg_g = 2 * 2 = 4 (even) → sign = +1 → Res(f,g) = Res(g,f)
    assert_eq!(
        res_fg, res_gf,
        "Res(f,g) should equal Res(g,f) for quadratic × quadratic"
    );
}

/// `Res(f, g) = (-1)^(deg_f * deg_g) * Res(g, f)` — linear × quadratic (even product).
///
/// `deg_f * deg_g = 1 * 2 = 2` (even) → `Res(f,g) = Res(g,f)`.
#[test]
fn resultant_symmetry_linear_quadratic() {
    let p_val = p();
    let f = linear(2);                  // x - 2
    let g = quadratic_with_roots(5, 7); // (x-5)(x-7)
    let res_fg = resultant(&f, &g, &p_val).unwrap();
    let res_gf = resultant(&g, &f, &p_val).unwrap();
    // deg_f * deg_g = 1 * 2 = 2 (even) → sign = +1 → Res(f,g) = Res(g,f)
    assert_eq!(
        res_fg, res_gf,
        "Res(f,g) should equal Res(g,f) for linear × quadratic (even product)"
    );
}

// ─── KAT 3: resultant of coprime quadratics is nonzero ───────────────────────

/// `Res(x²+1, x²+2) ≠ 0` over `F_47` — both irreducible (no roots mod 47).
///
/// Since `47 ≡ 3 mod 4`, `-1` is a QNR mod 47 (Euler's criterion: `(-1)^23 = -1 mod 47`).
/// Similarly, `-2` is a QNR mod 47 (verified: `(-2)^23 = -(2^23) = -1 mod 47` since
/// `2^23 = 1 mod 47` — the order of 2 mod 47 divides 23, and indeed `2^23 mod 47 = 1`).
/// Both polynomials are irreducible over `F_47`, hence coprime.
#[test]
fn resultant_coprime_irreducible_quadratics_nonzero() {
    let p_val = p();
    // x^2 + 1 — irreducible over F_47 since -1 is QNR mod 47
    let f = FpPoly::from_coeffs(
        vec![fp(1), fp(0), fp(1)], // 1 + 0*x + 1*x^2
        &p_val,
    );
    // x^2 + 2 — irreducible over F_47 since -2 is QNR mod 47
    let g = FpPoly::from_coeffs(
        vec![fp(2), fp(0), fp(1)], // 2 + 0*x + 1*x^2
        &p_val,
    );
    let res = resultant(&f, &g, &p_val).unwrap();
    assert!(
        !res.is_zero(&p_val),
        "Res(x²+1, x²+2) should be nonzero (coprime irreducible quadratics over F_47)"
    );
}

// ─── KAT 4: resultant value spot-check ───────────────────────────────────────

/// `Res(x-2, x-3) = -1 = 46 mod 47` over `F_47`.
///
/// The resultant of two monic linears `x - a` and `x - b` is `g(a) = a - b` (the value of
/// `g = x - b` at the root `a` of `f = x - a`). Here `Res(x-2, x-3) = g(2) = 2 - 3 = -1 = 46`.
///
/// Equivalently: `Res(f, g) = ∏_{f(α)=0} g(α) = g(2) = 2 - 3 = -1 mod 47`.
#[test]
fn resultant_linear_value_spot_check() {
    let p_val = p();
    let f = linear(2); // x - 2
    let g = linear(3); // x - 3
    let res = resultant(&f, &g, &p_val).unwrap();
    // Res(x-2, x-3) = g(2) = 2 - 3 = -1 = 46 mod 47
    assert_eq!(
        res.to_uint(),
        Uint::<4>::from(46u64),
        "Res(x-2, x-3) should be -1 = 46 mod 47"
    );
}

// ─── KAT 5: multivariate symmetric round-trip ────────────────────────────────

/// A symmetric polynomial round-trips through the `MultiPoly` representation.
///
/// Build `f(x_0, x_1) = x_0^2 + x_1^2 + 2*x_0*x_1 = (x_0 + x_1)^2`, evaluate at
/// `(3, 5)` and `(5, 3)` — both should give `(3+5)^2 = 64 = 64 - 47 = 17 mod 47`.
#[test]
fn multi_poly_symmetric_round_trip() {
    // f(x_0, x_1) = x_0^2 + x_1^2 + 2*x_0*x_1 = (x_0 + x_1)^2
    let mut f = MultiPoly::zero(2, P);
    f.add_term(vec![2, 0], 1); // x_0^2
    f.add_term(vec![0, 2], 1); // x_1^2
    f.add_term(vec![1, 1], 2); // 2*x_0*x_1

    assert!(f.is_symmetric(), "f should be symmetric");

    // (3 + 5)^2 = 64 = 64 - 47 = 17 mod 47
    let v1 = f.eval(&[3, 5]).unwrap();
    let v2 = f.eval(&[5, 3]).unwrap();
    assert_eq!(v1, 17, "f(3,5) = (3+5)^2 mod 47 = 17");
    assert_eq!(v2, 17, "f(5,3) = (5+3)^2 mod 47 = 17");
    assert_eq!(v1, v2, "symmetric polynomial: f(3,5) = f(5,3)");
}

// ─── KAT 6: evaluation at permutation is invariant ───────────────────────────

/// For a symmetric 3-variable polynomial, evaluation at any permutation of arguments
/// gives the same result.
///
/// Build `f(x_0, x_1, x_2) = x_0 + x_1 + x_2` (elementary symmetric polynomial e_1).
/// Evaluate at all permutations of `(2, 5, 11)` — all should give `2+5+11 = 18`.
#[test]
fn multi_poly_symmetric_eval_permutation_invariant_3var() {
    // f = x_0 + x_1 + x_2 (symmetric)
    let mut f = MultiPoly::zero(3, P);
    f.add_term(vec![1, 0, 0], 1); // x_0
    f.add_term(vec![0, 1, 0], 1); // x_1
    f.add_term(vec![0, 0, 1], 1); // x_2

    assert!(f.is_symmetric(), "x_0 + x_1 + x_2 should be symmetric");

    // 2 + 5 + 11 = 18 mod 47
    let expected = 18u64;
    let perms = [
        [2u64, 5, 11],
        [2, 11, 5],
        [5, 2, 11],
        [5, 11, 2],
        [11, 2, 5],
        [11, 5, 2],
    ];
    for perm in &perms {
        let v = f.eval(perm).unwrap();
        assert_eq!(
            v, expected,
            "symmetric f: eval at {:?} should be {expected}",
            perm
        );
    }
}

/// For a symmetric 3-variable polynomial `x_0*x_1 + x_1*x_2 + x_0*x_2` (e_2),
/// evaluation at all permutations of `(2, 3, 5)` gives the same result.
///
/// `e_2(2,3,5) = 2*3 + 3*5 + 2*5 = 6 + 15 + 10 = 31 mod 47`.
#[test]
fn multi_poly_e2_symmetric_eval_permutation_invariant() {
    // e_2(x_0, x_1, x_2) = x_0*x_1 + x_1*x_2 + x_0*x_2
    let mut f = MultiPoly::zero(3, P);
    f.add_term(vec![1, 1, 0], 1); // x_0*x_1
    f.add_term(vec![0, 1, 1], 1); // x_1*x_2
    f.add_term(vec![1, 0, 1], 1); // x_0*x_2

    assert!(f.is_symmetric(), "e_2 should be symmetric");

    // e_2(2,3,5) = 2*3 + 3*5 + 2*5 = 6 + 15 + 10 = 31 mod 47
    let expected = f.eval(&[2, 3, 5]).unwrap();
    assert_eq!(expected, 31, "e_2(2,3,5) = 31 mod 47");
    let perms = [
        [2u64, 3, 5],
        [2, 5, 3],
        [3, 2, 5],
        [3, 5, 2],
        [5, 2, 3],
        [5, 3, 2],
    ];
    for perm in &perms {
        let v = f.eval(perm).unwrap();
        assert_eq!(
            v, expected,
            "e_2: eval at {:?} should equal eval at (2,3,5)",
            perm
        );
    }
}

// ─── KAT 7: partial evaluation ────────────────────────────────────────────────

/// Fixing one variable of a 3-variable polynomial gives a 2-variable polynomial.
///
/// `f(x_0, x_1, x_2) = x_0 + x_1 + x_2`. Fix `x_0 = 7` → `g(x_1, x_2) = 7 + x_1 + x_2`.
/// Evaluate `g(3, 5)` → `7 + 3 + 5 = 15`.
#[test]
fn multi_poly_partial_eval_3var() {
    let mut f = MultiPoly::zero(3, P);
    f.add_term(vec![1, 0, 0], 1); // x_0
    f.add_term(vec![0, 1, 0], 1); // x_1
    f.add_term(vec![0, 0, 1], 1); // x_2

    // Fix x_0 = 7
    let g = f.partial_eval(&[Some(7), None, None]).unwrap();
    assert_eq!(g.num_vars, 2, "one free variable after fixing x_0");

    // g(3, 5) = 7 + 3 + 5 = 15
    let result = g.eval(&[3, 5]).unwrap();
    assert_eq!(result, 15, "partial eval: 7 + 3 + 5 = 15");
}

/// Fixing two variables of a 3-variable polynomial gives a 1-variable polynomial.
///
/// `f(x_0, x_1, x_2) = x_0*x_1 + x_1*x_2 + x_0*x_2`.
/// Fix `x_0 = 2`, `x_1 = 3` → `g(x_2) = 2*3 + 3*x_2 + 2*x_2 = 6 + 5*x_2`.
/// `g(5) = 6 + 25 = 31 mod 47`. `g(1) = 6 + 5 = 11`.
#[test]
fn multi_poly_partial_eval_two_fixed() {
    // f(x_0, x_1, x_2) = x_0*x_1 + x_1*x_2 + x_0*x_2
    let mut f = MultiPoly::zero(3, P);
    f.add_term(vec![1, 1, 0], 1); // x_0*x_1
    f.add_term(vec![0, 1, 1], 1); // x_1*x_2
    f.add_term(vec![1, 0, 1], 1); // x_0*x_2

    // Fix x_0 = 2, x_1 = 3 → g(x_2) = 2*3 + 3*x_2 + 2*x_2 = 6 + 5*x_2
    let g = f.partial_eval(&[Some(2), Some(3), None]).unwrap();
    assert_eq!(g.num_vars, 1, "one free variable after fixing x_0 and x_1");

    // g(5) = 6 + 5*5 = 6 + 25 = 31 mod 47
    let result = g.eval(&[5]).unwrap();
    assert_eq!(result, 31, "partial eval: 6 + 5*5 = 31 mod 47");

    // g(1) = 6 + 5*1 = 11
    let result2 = g.eval(&[1]).unwrap();
    assert_eq!(result2, 11, "partial eval: 6 + 5*1 = 11");
}

// ─── KAT 8: is_symmetric correctly identifies symmetric polynomials ───────────

/// `x_0^2 + x_1^2 + 2*x_0*x_1` is symmetric.
#[test]
fn is_symmetric_true_for_symmetric_poly() {
    let mut f = MultiPoly::zero(2, P);
    f.add_term(vec![2, 0], 1); // x_0^2
    f.add_term(vec![0, 2], 1); // x_1^2
    f.add_term(vec![1, 1], 2); // 2*x_0*x_1
    assert!(f.is_symmetric(), "x_0^2 + x_1^2 + 2*x_0*x_1 should be symmetric");
}

/// `x_0^2` is not symmetric in 2 variables.
#[test]
fn is_symmetric_false_for_asymmetric_poly() {
    let mut f = MultiPoly::zero(2, P);
    f.add_term(vec![2, 0], 1); // x_0^2 only
    assert!(!f.is_symmetric(), "x_0^2 should not be symmetric in 2 variables");
}

/// The zero polynomial is symmetric (vacuously).
#[test]
fn is_symmetric_zero_poly() {
    let f = MultiPoly::zero(3, P);
    assert!(f.is_symmetric(), "zero polynomial should be symmetric");
}

// ─── KAT 9: symmetrize produces a symmetric polynomial ───────────────────────

/// `symmetrize(x_0^2)` over 2 variables gives `x_0^2 + x_1^2` (symmetric).
#[test]
fn symmetrize_produces_symmetric_poly() {
    let mut f = MultiPoly::zero(2, P);
    f.add_term(vec![2, 0], 1); // x_0^2
    let sym = f.symmetrize();
    assert!(sym.is_symmetric(), "symmetrize(x_0^2) should be symmetric");
    // sym should contain x_0^2 and x_1^2
    assert!(
        sym.terms.contains_key(&vec![2u64, 0]),
        "symmetrized poly should contain x_0^2"
    );
    assert!(
        sym.terms.contains_key(&vec![0u64, 2]),
        "symmetrized poly should contain x_1^2"
    );
}

/// `symmetrize(x_0 * x_1^2)` over 3 variables gives a symmetric polynomial.
#[test]
fn symmetrize_3var_produces_symmetric_poly() {
    let mut f = MultiPoly::zero(3, P);
    f.add_term(vec![1, 2, 0], 1); // x_0 * x_1^2
    let sym = f.symmetrize();
    assert!(sym.is_symmetric(), "symmetrize(x_0*x_1^2) over 3 vars should be symmetric");
    // Evaluate at (2, 3, 5) and (3, 2, 5) — should be equal.
    let v1 = sym.eval(&[2, 3, 5]).unwrap();
    let v2 = sym.eval(&[3, 2, 5]).unwrap();
    assert_eq!(v1, v2, "symmetrized poly: eval at (2,3,5) = eval at (3,2,5)");
}

// ─── KAT: SemaevError display ─────────────────────────────────────────────────

/// Smoke-test `SemaevError` Display impls (no panic).
#[test]
fn semaev_error_display_smoke() {
    let _ = format!("{}", SemaevError::DegreeZero);
    let _ = format!("{}", SemaevError::VariableOutOfRange { index: 5, num_vars: 3 });
    let _ = format!("{}", SemaevError::ArityMismatch { expected: 3, got: 2 });
    let _ = format!("{}", SemaevError::ZeroLeadingCoefficient);
}

// ─── KAT: univariate_in_var ───────────────────────────────────────────────────

/// `univariate_in_var` correctly extracts coefficients.
///
/// `f(x_0, x_1) = 3*x_0^2 + 2*x_0*x_1 + x_1^2 + x_0 + 5`.
/// Viewed as univariate in `x_0`: `3*x_0^2 + (2*x_1 + 1)*x_0 + (x_1^2 + 5)`.
#[test]
fn univariate_in_var_extraction() {
    let mut f = MultiPoly::zero(2, P);
    f.add_term(vec![2, 0], 3); // 3*x_0^2
    f.add_term(vec![1, 1], 2); // 2*x_0*x_1
    f.add_term(vec![0, 2], 1); // x_1^2
    f.add_term(vec![1, 0], 1); // x_0
    f.add_term(vec![0, 0], 5); // 5

    let coeffs = f.univariate_in_var(0).unwrap();
    // coeffs[0] = x_1^2 + 5 (constant term in x_0)
    // coeffs[1] = 2*x_1 + 1 (coefficient of x_0)
    // coeffs[2] = 3 (coefficient of x_0^2)
    assert_eq!(coeffs.len(), 3, "degree 2 in x_0 → 3 coefficient polynomials");

    // coeffs[2] should be the constant 3 (in x_1)
    let v2 = coeffs[2].eval(&[0]).unwrap();
    assert_eq!(v2, 3, "coefficient of x_0^2 should be 3");

    // coeffs[1] at x_1 = 4: 2*4 + 1 = 9
    let v1 = coeffs[1].eval(&[4]).unwrap();
    assert_eq!(v1, 9, "coefficient of x_0 at x_1=4: 2*4+1=9");

    // coeffs[0] at x_1 = 3: 3^2 + 5 = 14
    let v0 = coeffs[0].eval(&[3]).unwrap();
    assert_eq!(v0, 14, "constant term at x_1=3: 3^2+5=14");
}

// ═══════════════════════════════════════════════════════════════════════════════
// E.J.2 — Base cases S_2, S_3 + the vanishing relation (C-SemaevBase)
// ═══════════════════════════════════════════════════════════════════════════════
//
// Known multiples of G on the toy curve y² = x³ + x + 33 over F_47:
//   G  = (10, 3),  −G  = (10, 44)
//   2G = (7, 30),  −2G = (7, 17)
//   3G = (17, 13), −3G = (17, 34)
//   4G = (23, 12), −4G = (23, 35)
//   5G = (32, 36), −5G = (32, 11)
//
// Verified: each point satisfies y² = x³ + x + 33 mod 47.

// ─── helpers (E.J.2) ─────────────────────────────────────────────────────────

/// Construct a finite affine point over `F_47`.
fn pt(x: u64, y: u64) -> AffinePoint<FpNaive> {
    let p_val = Uint::<4>::from(P);
    AffinePoint::Finite {
        x: FpNaive::from_u64(x, &p_val),
        y: FpNaive::from_u64(y, &p_val),
    }
}

// ─── KAT 11: S_2 vanishing ───────────────────────────────────────────────────

/// `S_2(x_1, x_2) = 0 ⟺ x_1 = x_2` (i.e. `P_2 = −P_1`).
///
/// `S_2(10, 10) = 0` and `G + (−G) = ∞` ✓.
#[test]
fn s2_vanishes_for_negation() {
    let c = semaev_toy();
    let g = pt(10, 3);
    let neg_g = pt(10, 44);
    assert!(
        vanishes_s2(&c, &g, &neg_g),
        "S_2 should vanish for G and −G (same x-coordinate)"
    );
}

/// `S_2(10, 7) ≠ 0` and `G + 2G = 3G ≠ ∞`.
#[test]
fn s2_nonzero_for_distinct_x() {
    let c = semaev_toy();
    let g = pt(10, 3);
    let two_g = pt(7, 30);
    assert!(
        !vanishes_s2(&c, &g, &two_g),
        "S_2 should not vanish for G and 2G (distinct x-coordinates)"
    );
}

/// `S_2` vanishes for any point and its negation (multiple spot-checks).
#[test]
fn s2_vanishes_for_multiple_negation_pairs() {
    let c = semaev_toy();
    // (2G, −2G): x=7
    assert!(vanishes_s2(&c, &pt(7, 30), &pt(7, 17)), "S_2 should vanish for 2G and −2G");
    // (3G, −3G): x=17
    assert!(vanishes_s2(&c, &pt(17, 13), &pt(17, 34)), "S_2 should vanish for 3G and −3G");
    // (4G, −4G): x=23
    assert!(vanishes_s2(&c, &pt(23, 12), &pt(23, 35)), "S_2 should vanish for 4G and −4G");
}

// ─── KAT 12: S_3 vanishing ───────────────────────────────────────────────────

/// `S_3(10, 7, 17) = 0`: x-coordinates of `G, 2G, −3G` — there exist y-values making
/// the sum `∞` (specifically `G + 2G + (−3G) = ∞`).
#[test]
fn s3_vanishes_for_g_2g_neg3g_xcoords() {
    let c = semaev_toy();
    assert!(
        vanishes_s3::<FpNaive>(&c, 10, 7, 17),
        "S_3 should vanish for x-coords of G, 2G, −3G"
    );
}

/// `S_3(10, 7, 23) ≠ 0`: x-coordinates of `G, 2G, 4G` — no y-values make sum `∞`.
///
/// All 8 combinations of y-values for x=10,7,23 are checked:
/// - `G+2G+4G = 7G ≠ ∞`, `G+2G+(−4G) = −G ≠ ∞`, `G+(−2G)+4G = 3G ≠ ∞`,
/// - `G+(−2G)+(−4G) = −5G ≠ ∞`, `(−G)+2G+4G = 5G ≠ ∞`, `(−G)+2G+(−4G) = −3G ≠ ∞`,
/// - `(−G)+(−2G)+4G = G ≠ ∞`, `(−G)+(−2G)+(−4G) = −7G ≠ ∞`.
#[test]
fn s3_nonzero_for_g_2g_4g_xcoords() {
    let c = semaev_toy();
    assert!(
        !vanishes_s3::<FpNaive>(&c, 10, 7, 23),
        "S_3 should not vanish for x-coords of G, 2G, 4G (no y-values make sum ∞)"
    );
}

/// `S_3(10, 17, 32) = 0`: x-coordinates of `G, 3G, −5G` — `G + 3G + (−5G) = ∞`.
///
/// `G + 3G = 4G = (23, 12)`. `−5G = (32, 11)`. `4G + (−5G) = −G = (10, 44)`.
/// Wait — that gives `G + 3G + (−5G) = 4G + (−5G) = −G ≠ ∞`.
/// Let's try `G + (−3G) + 2G = ∞`: x-coords 10, 17, 7 — same as KAT 12a.
/// Try `2G + 3G + (−5G) = 5G + (−5G) = ∞`: x-coords 7, 17, 32.
#[test]
fn s3_vanishes_for_2g_3g_neg5g_xcoords() {
    let c = semaev_toy();
    // 2G=(7,30), 3G=(17,13), −5G=(32,11): 2G+3G+(−5G) = 5G+(−5G) = ∞
    assert!(
        vanishes_s3::<FpNaive>(&c, 7, 17, 32),
        "S_3 should vanish for x-coords of 2G, 3G, −5G"
    );
}

/// `S_3(10, 10, 10) ≠ 0`: no choice of y-values for x=10 makes three points sum to `∞`.
///
/// The y-values for x=10 are `{3, 44}` (i.e. `G` and `−G`). All 8 combinations of
/// `(G or −G) + (G or −G) + (G or −G)` give `±G, ±3G` — none is `∞`.
#[test]
fn s3_nonzero_for_all_same_x() {
    let c = semaev_toy();
    assert!(
        !vanishes_s3::<FpNaive>(&c, 10, 10, 10),
        "S_3 should not vanish for x-coords (10,10,10)"
    );
}

/// `S_3(10, 7, 32) ≠ 0`: x-coords of `G, 2G, 5G` — no y-values make sum `∞`.
///
/// `G + 2G + 5G = 8G ≠ ∞`. `G + 2G + (−5G) = 3G + (−5G) = −2G ≠ ∞`.
/// `G + (−2G) + 5G = 4G ≠ ∞`. `G + (−2G) + (−5G) = −6G ≠ ∞`.
/// `(−G) + 2G + 5G = 6G ≠ ∞`. `(−G) + 2G + (−5G) = −4G ≠ ∞`.
/// `(−G) + (−2G) + 5G = 2G ≠ ∞`. `(−G) + (−2G) + (−5G) = −8G ≠ ∞`.
#[test]
fn s3_nonzero_for_g_2g_5g_xcoords() {
    let c = semaev_toy();
    assert!(
        !vanishes_s3::<FpNaive>(&c, 10, 7, 32),
        "S_3 should not vanish for x-coords of G, 2G, 5G"
    );
}

// ─── KAT 13: S_3 symmetry ────────────────────────────────────────────────────

/// `S_3` is symmetric: invariant under all permutations of its three arguments.
///
/// Verified by evaluating at all 6 permutations of `(10, 7, 17)` and checking
/// that all give the same value.
#[test]
fn s3_symmetric_eval_at_all_permutations() {
    let poly = s3(1, 33, P);
    assert!(poly.is_symmetric(), "S_3 should be symmetric");

    // All 6 permutations of (10, 7, 17)
    let perms = [
        [10u64, 7, 17],
        [10, 17, 7],
        [7, 10, 17],
        [7, 17, 10],
        [17, 10, 7],
        [17, 7, 10],
    ];
    let base_val = poly.eval(&perms[0]).unwrap();
    for perm in &perms[1..] {
        let v = poly.eval(perm).unwrap();
        assert_eq!(
            v, base_val,
            "S_3 should be symmetric: eval at {:?} = eval at {:?}",
            perm, perms[0]
        );
    }
}

/// `S_3` symmetry on a non-vanishing triple: all permutations of `(10, 10, 10)` agree.
#[test]
fn s3_symmetric_eval_nonzero_triple() {
    let poly = s3(1, 33, P);
    // All permutations of (10, 10, 10) are the same — trivially symmetric.
    let v = poly.eval(&[10, 10, 10]).unwrap();
    assert_ne!(v, 0, "S_3(10,10,10) should be nonzero");

    // Verify with a non-trivial permutation: (7, 17, 32) — all distinct.
    // (This triple has S_3 ≠ 0 since no y-values make G+3G+5G or similar = ∞.)
    let perms = [
        [7u64, 17, 32],
        [7, 32, 17],
        [17, 7, 32],
        [17, 32, 7],
        [32, 7, 17],
        [32, 17, 7],
    ];
    let base_val = poly.eval(&perms[0]).unwrap();
    for perm in &perms[1..] {
        let v = poly.eval(perm).unwrap();
        assert_eq!(
            v, base_val,
            "S_3 should be symmetric: eval at {:?} = eval at {:?}",
            perm, perms[0]
        );
    }
}

// ─── KAT 14: S_3 degree ──────────────────────────────────────────────────────

/// `S_3` has degree ≤ 2 in each variable.
///
/// Verified by checking that no exponent in any monomial exceeds 2 for any variable.
#[test]
fn s3_degree_at_most_2_in_each_var() {
    let poly = s3(1, 33, P);
    for (exp, _coeff) in &poly.terms {
        for (i, &e) in exp.iter().enumerate() {
            assert!(
                e <= 2,
                "S_3 has degree > 2 in variable {i}: exponent vector {:?}",
                exp
            );
        }
    }
}

/// `S_3` achieves degree exactly 2 in each variable (not just ≤ 2).
///
/// Verified by checking that there exists a monomial with degree 2 in each variable.
#[test]
fn s3_degree_exactly_2_in_each_var() {
    let poly = s3(1, 33, P);
    for var in 0..3 {
        let max_deg = poly.terms.keys().map(|exp| exp[var]).max().unwrap_or(0);
        assert_eq!(
            max_deg, 2,
            "S_3 should have degree exactly 2 in variable {var}, got {max_deg}"
        );
    }
}
