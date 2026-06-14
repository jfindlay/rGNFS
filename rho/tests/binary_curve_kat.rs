//! Known-answer tests for the binary elliptic curve group law.
//!
//! Coverage:
//! - Point-on-curve: `is_on_curve` holds for the base point and its multiples.
//! - Group axioms: `P+∞=P`, `P+(−P)=∞` with `−P=(x,x+y)`, associativity.
//! - Doubling consistency: `2P` via `double` equals `P+P` via `add`.
//! - Scalar multiplication: `n·G = ∞` (group order check).
//! - Decompression round-trip: `decompress(x, bit)` lands on the curve and
//!   recovers the original y-coordinate.
//! - Char-2 negation trap: `−(x,y) = (x, x+y)`, NOT `(x, −y)`.
//! - Koblitz τ-automorphism: `τ^m(P) = P` (order divides m); the characteristic
//!   relation `τ²(P) − t·τ(P) + 2·P = ∞`; sub-track-close suite.
//!
//! # Toy curve
//!
//! All tests use the curve `y²+xy = x³+x²+1` over GF(2^4) with `x⁴+x+1`
//! (poly = 0x13).  This curve has group order 4 (3 affine points + ∞):
//! - G  = (1, 6)
//! - 2G = (0, 1)
//! - 3G = (1, 7) = −G  (since −(1,6) = (1, 1⊕6) = (1, 7))
//! - 4G = ∞
//!
//! The algorithms are crypto-scale-correct; only the parameters are toy
//! (principle-4 boundary: the group law and decompression are not toy).

use crypto_bigint::Uint;
use rho::binary_curve::{BinaryAffinePoint, BinaryCurve};
use rho::binary_ecdlp::koblitz::{tau, tau_pow};
use shared_gf2m::{F2m, F2mNaive};

// ── Curve and field parameters ────────────────────────────────────────────────

/// GF(2^4) irreducible: x⁴+x+1 = 0x13.
fn poly4() -> Uint<1> {
    Uint::<1>::from(0x13u64)
}

/// Toy binary curve: y²+xy = x³+x²+1 over GF(2^4) with x⁴+x+1.
///
/// Group order 4: G=(1,6), 2G=(0,1), 3G=(1,7), 4G=∞.
fn toy_curve() -> BinaryCurve {
    BinaryCurve {
        poly: poly4(),
        a: Uint::<1>::ONE,
        b: Uint::<1>::ONE,
        n: Uint::<1>::from(4u64),
        gx: Uint::<1>::ONE,
        gy: Uint::<1>::from(6u64),
    }
}

/// Construct a GF(2^4) field element from a u64 value.
fn f4(v: u64) -> F2mNaive<1> {
    F2mNaive::<1>::from_u64(v, &poly4())
}

// ── Point-on-curve KATs ───────────────────────────────────────────────────────

/// The base point G = (1, 6) is on the curve.
///
/// LHS: 6²+1·6 = 7+6 = 1 (in GF(2^4)).
/// RHS: 1³+1·1²+1 = 1+1+1 = 1. ✓
#[test]
fn generator_is_on_curve() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    assert!(c.is_on_curve(&g), "G not on curve");
}

/// All multiples of G are on the curve.
///
/// Checks 1G, 2G, 3G, and ∞ (which is trivially on the curve).
#[test]
fn multiples_of_g_are_on_curve() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();

    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);
    let four_g = c.add(&three_g, &g); // should be ∞

    assert!(c.is_on_curve(&g), "G not on curve");
    assert!(c.is_on_curve(&two_g), "2G not on curve");
    assert!(c.is_on_curve(&three_g), "3G not on curve");
    assert!(c.is_on_curve(&four_g), "4G not on curve (∞ is trivially on curve)");
}

/// Known-answer: 2G = (0, 1).
///
/// Computed by the affine doubling formula:
///   λ = x + y/x = 1 + 6/1 = 1+6 = 7 (in GF(2^4))
///   x₃ = λ²+λ+a = 49+7+1 = ... in GF(2^4): 7²=x²+x+1²=... let me use field arithmetic.
///   7 = x²+x+1, 7² = x⁴+x²+1 = (x+1)+x²+1 = x²+x = 6.
///   x₃ = 6+7+1 = 0 (in GF(2^4): 6⊕7⊕1 = 0b0110⊕0b0111⊕0b0001 = 0b0000 = 0). ✓
///   y₃ = x²+(λ+1)·x₃ = 1+(7+1)·0 = 1+0 = 1. ✓
#[test]
fn two_g_known_answer() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let expected = BinaryAffinePoint::Finite { x: f4(0), y: f4(1) };
    assert_eq!(two_g, expected, "2G ≠ (0, 1)");
}

/// Known-answer: 3G = (1, 7) = −G.
///
/// 3G = 2G + G = (0,1) + (1,6).
/// λ = (1+6)/(0+1) = 7/1 = 7.
/// x₃ = 7²+7+1+0+1 = 6+7+1+0+1 = 6⊕7⊕1⊕0⊕1 = 1. ✓
/// y₃ = 7·(0+1)+1+1 = 7+1+1 = 7. ✓
#[test]
fn three_g_known_answer() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);
    let expected = BinaryAffinePoint::Finite { x: f4(1), y: f4(7) };
    assert_eq!(three_g, expected, "3G ≠ (1, 7)");
}

/// Known-answer: 4G = ∞ (group order is 4).
#[test]
fn four_g_is_infinity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let four_g = c.scalar_mul(&g, &Uint::<1>::from(4u64));
    assert!(four_g.is_infinity(), "4G should be ∞ (group order = 4)");
}

// ── Group axiom KATs ──────────────────────────────────────────────────────────

/// `P + ∞ = P` for all points.
#[test]
fn add_infinity_right_identity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let inf = BinaryAffinePoint::Infinity;

    assert_eq!(c.add(&g, &inf), g, "G + ∞ ≠ G");
    assert_eq!(c.add(&two_g, &inf), two_g, "2G + ∞ ≠ 2G");
    assert_eq!(c.add(&inf, &inf), inf, "∞ + ∞ ≠ ∞");
}

/// `∞ + P = P` for all points.
#[test]
fn add_infinity_left_identity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let inf = BinaryAffinePoint::Infinity;

    assert_eq!(c.add(&inf, &g), g, "∞ + G ≠ G");
    assert_eq!(c.add(&inf, &two_g), two_g, "∞ + 2G ≠ 2G");
}

/// `P + (−P) = ∞` with `−P = (x, x+y)` (char-2 negation).
///
/// This is the load-bearing guard for the char-2 negation trap.
/// If negation were implemented as `(x, −y) = (x, y)` (wrong in char 2),
/// then `P + (−P)` would equal `P + P = 2P`, not ∞.
#[test]
fn add_neg_is_infinity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let neg_g = c.negate(&g);

    // Verify −G = (1, 7): -(1,6) = (1, 1⊕6) = (1, 7).
    let expected_neg = BinaryAffinePoint::Finite { x: f4(1), y: f4(7) };
    assert_eq!(neg_g, expected_neg, "−G ≠ (1, 7)");

    // G + (−G) = ∞.
    let sum = c.add(&g, &neg_g);
    assert!(sum.is_infinity(), "G + (−G) should be ∞");
}

/// Char-2 negation: `−(x,y) = (x, x+y)`, NOT `(x, −y)`.
///
/// In char 2, `−y = y`, so `(x, −y) = (x, y) = P`.  The correct negation
/// is `(x, x+y)`.  This test verifies the x-coordinate is unchanged and
/// the y-coordinate is `x+y`.
#[test]
fn negate_is_x_plus_y() {
    let c = toy_curve();
    let poly = poly4();

    // G = (1, 6): −G = (1, 1⊕6) = (1, 7).
    let g = c.generator::<F2mNaive<1>>();
    let neg_g = c.negate(&g);
    assert_eq!(neg_g.x(), Some(&f4(1)), "−G: x-coordinate changed");
    assert_eq!(neg_g.y(), Some(&f4(7)), "−G: y ≠ x+y");

    // 2G = (0, 1): −(2G) = (0, 0⊕1) = (0, 1) = 2G (self-negation since x=0).
    let two_g = c.double(&g);
    let neg_two_g = c.negate(&two_g);
    let x_plus_y = f4(0).add(&f4(1));
    assert_eq!(neg_two_g.y(), Some(&x_plus_y), "−(2G): y ≠ x+y");
    // For (0,1): x+y = 0+1 = 1, so −(0,1) = (0,1) — it's its own negation.
    assert_eq!(neg_two_g, two_g, "2G should be its own negation (x=0)");

    // Verify −(2G) + 2G = ∞.
    let sum = c.add(&neg_two_g, &two_g);
    assert!(sum.is_infinity(), "−(2G) + 2G should be ∞");

    let _ = poly;
}

/// Associativity: `(P+Q)+R = P+(Q+R)` for a sample of points.
///
/// Tests (G+2G)+3G = G+(2G+3G).
#[test]
fn add_associativity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    // (G + 2G) + 3G
    let lhs = c.add(&c.add(&g, &two_g), &three_g);
    // G + (2G + 3G)
    let rhs = c.add(&g, &c.add(&two_g, &three_g));

    assert_eq!(lhs, rhs, "(G+2G)+3G ≠ G+(2G+3G)");
}

// ── Doubling consistency KATs ─────────────────────────────────────────────────

/// `double(P) = add(P, P)` for all non-trivial points.
///
/// This is the load-bearing guard against Jacobian-formula contamination.
/// The Jacobian doubling formula divides by `2y = 0` in char 2, producing
/// wrong results.  The LD formula avoids this.
#[test]
fn double_equals_add_self() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    // 2G via double == 2G via add(G, G).
    assert_eq!(c.double(&g), c.add(&g, &g), "double(G) ≠ add(G,G)");

    // 2·(2G) via double == (2G)+(2G) via add.
    assert_eq!(c.double(&two_g), c.add(&two_g, &two_g), "double(2G) ≠ add(2G,2G)");

    // 2·(3G) via double == (3G)+(3G) via add.
    assert_eq!(c.double(&three_g), c.add(&three_g, &three_g), "double(3G) ≠ add(3G,3G)");
}

/// `scalar_mul(P, 2) = double(P)` for all non-trivial points.
#[test]
fn scalar_mul_two_matches_double() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    assert_eq!(
        c.scalar_mul(&g, &Uint::<1>::from(2u64)),
        c.double(&g),
        "scalar_mul(G, 2) ≠ double(G)"
    );
    assert_eq!(
        c.scalar_mul(&two_g, &Uint::<1>::from(2u64)),
        c.double(&two_g),
        "scalar_mul(2G, 2) ≠ double(2G)"
    );
    assert_eq!(
        c.scalar_mul(&three_g, &Uint::<1>::from(2u64)),
        c.double(&three_g),
        "scalar_mul(3G, 2) ≠ double(3G)"
    );
}

// ── Scalar multiplication KATs ────────────────────────────────────────────────

/// `1·G = G`.
#[test]
fn scalar_mul_one_is_identity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    assert_eq!(c.scalar_mul(&g, &Uint::<1>::ONE), g, "1·G ≠ G");
}

/// `n·G = ∞` where n is the group order.
#[test]
fn scalar_mul_order_is_infinity() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();
    let n = c.n;
    let ng = c.scalar_mul(&g, &n);
    assert!(ng.is_infinity(), "n·G should be ∞ (group order check)");
}

/// Scalar multiplication consistency: `k·G = G+G+...+G` (k times).
///
/// Verifies that scalar_mul agrees with repeated addition for k=1,2,3,4.
#[test]
fn scalar_mul_agrees_with_repeated_add() {
    let c = toy_curve();
    let g = c.generator::<F2mNaive<1>>();

    // Compute multiples by repeated addition.
    let one_g = g.clone();
    let two_g = c.add(&one_g, &g);
    let three_g = c.add(&two_g, &g);
    let four_g = c.add(&three_g, &g); // = ∞

    // Verify scalar_mul agrees.
    assert_eq!(c.scalar_mul(&g, &Uint::<1>::from(1u64)), one_g, "1·G mismatch");
    assert_eq!(c.scalar_mul(&g, &Uint::<1>::from(2u64)), two_g, "2·G mismatch");
    assert_eq!(c.scalar_mul(&g, &Uint::<1>::from(3u64)), three_g, "3·G mismatch");
    assert_eq!(c.scalar_mul(&g, &Uint::<1>::from(4u64)), four_g, "4·G mismatch");
}

// ── Decompression round-trip KATs ─────────────────────────────────────────────

/// Decompression of x=1 recovers the original y-coordinate.
///
/// For x=1 on the toy curve: the two y-values are 6 and 7.
/// - decompress(1, false) should give (1, 6) = G.
/// - decompress(1, true)  should give (1, 7) = −G = 3G.
///
/// Derivation: λ²+λ = x+a+b/x² = 1+1+1/1 = 1 (in GF(2^4)).
/// solve_quadratic(1): x²+x=1 has solutions x=6 (0b0110) and x=7 (0b0111).
/// bit_0(6) = 0, bit_0(7) = 1.
/// sign_bit=false → use λ with bit_0(λ)=0 → λ=6 → y=6·1=6.
/// sign_bit=true  → use λ with bit_0(λ)=1 → λ=7 → y=7·1=7.
#[test]
fn decompress_round_trip_x1() {
    let c = toy_curve();
    let x = f4(1);

    // decompress(1, false) = (1, 6) = G.
    let pt_false = c.decompress(&x, false);
    assert!(c.is_on_curve(&pt_false), "decompress(1, false) not on curve");
    assert_eq!(
        pt_false,
        BinaryAffinePoint::Finite { x: f4(1), y: f4(6) },
        "decompress(1, false) ≠ (1, 6)"
    );

    // decompress(1, true) = (1, 7) = −G.
    let pt_true = c.decompress(&x, true);
    assert!(c.is_on_curve(&pt_true), "decompress(1, true) not on curve");
    assert_eq!(
        pt_true,
        BinaryAffinePoint::Finite { x: f4(1), y: f4(7) },
        "decompress(1, true) ≠ (1, 7)"
    );
}

/// Decompression round-trip: compress then decompress recovers the original point.
///
/// For each finite point P = (x, y) on the curve with x ≠ 0:
/// 1. Compute λ = y/x.
/// 2. Compute sign_bit = bit_0(λ) (the constant term of λ).
/// 3. decompress(x, sign_bit) should recover P.
///
/// The bit_0 convention works for all m (even and odd), since λ and λ+1
/// always differ in bit 0.
#[test]
fn decompress_round_trip_all_points() {
    let c = toy_curve();
    let poly = poly4();

    // All affine points on the toy curve: (1,6), (0,1), (1,7).
    // Skip (0,1) since decompress requires x ≠ 0.
    let points = vec![
        BinaryAffinePoint::Finite { x: f4(1), y: f4(6) },
        BinaryAffinePoint::Finite { x: f4(1), y: f4(7) },
    ];

    for pt in &points {
        let (x, y) = match pt {
            BinaryAffinePoint::Finite { x, y } => (x, y),
            BinaryAffinePoint::Infinity => unreachable!(),
        };

        // Compute the sign bit: bit_0(y/x).
        let lambda = y.div(x, &poly);
        let sign_bit: bool = lambda.to_uint().bit(0).into();

        // Decompress and verify.
        let recovered = c.decompress(x, sign_bit);
        assert!(c.is_on_curve(&recovered), "decompress: recovered point not on curve");
        assert_eq!(
            &recovered, pt,
            "decompress round-trip failed for ({}, {})",
            x.to_uint(),
            y.to_uint()
        );
    }
}

/// Decompression produces two distinct y-values for the same x.
///
/// The two roots of λ²+λ=rhs are λ and λ+1, giving y=λx and y=(λ+1)x.
/// These two y-values are distinct (they differ by x).
/// The two points are negations of each other: −(x,y) = (x, x+y).
#[test]
fn decompress_two_distinct_y_values() {
    let c = toy_curve();
    let x = f4(1);

    // sign_bit=false gives (1,6), sign_bit=true gives (1,7).
    let pt0 = c.decompress(&x, false);
    let pt1 = c.decompress(&x, true);

    // The two points are distinct.
    assert_ne!(pt0, pt1, "decompress: two sign bits should give distinct points");

    // Both are on the curve.
    assert!(c.is_on_curve(&pt0), "decompress(false) not on curve");
    assert!(c.is_on_curve(&pt1), "decompress(true) not on curve");

    // They are negations of each other: −(x,y) = (x, x+y).
    let neg_pt0 = c.negate(&pt0);
    assert_eq!(neg_pt0, pt1, "decompress: pt1 should be −pt0");
}

// ── Koblitz τ-automorphism KATs ───────────────────────────────────────────────
//
// These tests verify the C-Koblitz contract:
//   - τ(x,y) = (x², y²) applies the field Frobenius to BOTH coordinates.
//   - τ^m(P) = P for all P (automorphism order divides m=4).
//   - The characteristic relation τ²(P) − t·τ(P) + 2·P = ∞ holds.
//
// Toy curve: y²+xy = x³+x²+1 over GF(2^4), group order n=4, m=4.
// Frobenius trace: t = 2^4 + 1 − 4 = 13.
// Known τ values (squaring in GF(2^4) with x⁴+x+1):
//   6² = (x²+x)² = x⁴+x² = (x+1)+x² = x²+x+1 = 7
//   7² = (x²+x+1)² = x⁴+x²+1 = (x+1)+x²+1 = x²+x = 6
//   1² = 1, 0² = 0
// So: τ(G) = τ(1,6) = (1,7) = 3G; τ(3G) = τ(1,7) = (1,6) = G; τ(2G) = 2G.

/// τ(G) = (1, 7) = 3G: the Frobenius maps G to its negation.
///
/// In GF(2^4): 6² = 7 (since (x²+x)² = x⁴+x² = x²+x+1 = 7 mod x⁴+x+1).
/// So τ(1, 6) = (1², 6²) = (1, 7) = 3G = −G. ✓
#[test]
fn tau_maps_g_to_three_g() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    let tau_g = tau(&g, &poly);
    assert_eq!(tau_g, three_g, "τ(G) ≠ 3G");
}

/// τ(2G) = 2G: the point (0, 1) is a fixed point of τ.
///
/// 2G = (0, 1). τ(0, 1) = (0², 1²) = (0, 1) = 2G. ✓
#[test]
fn tau_fixes_two_g() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);

    let tau_two_g = tau(&two_g, &poly);
    assert_eq!(tau_two_g, two_g, "τ(2G) ≠ 2G");
}

/// τ(∞) = ∞: the point at infinity is fixed by τ.
#[test]
fn tau_fixes_infinity() {
    let poly = poly4();
    let inf: BinaryAffinePoint<F2mNaive<1>> = BinaryAffinePoint::Infinity;
    let tau_inf = tau(&inf, &poly);
    assert!(tau_inf.is_infinity(), "τ(∞) ≠ ∞");
}

/// τ images are on the curve for all non-trivial points.
#[test]
fn tau_image_on_curve() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    for (label, pt) in [("G", g.clone()), ("2G", two_g), ("3G", three_g)] {
        let tau_pt = tau(&pt, &poly);
        assert!(c.is_on_curve(&tau_pt), "τ({label}) not on curve");
    }
}

/// τ^m(P) = P for all P: the Frobenius automorphism has order dividing m=4.
///
/// This is the primary C-Koblitz correctness signal.  For GF(2^m), the
/// Frobenius `a → a²` has order m (since `a^(2^m) = a` for all `a ∈ GF(2^m)`).
/// Applied to both coordinates: `τ^m(x, y) = (x^(2^m), y^(2^m)) = (x, y)`.
#[test]
fn tau_order_divides_m() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    // m = 4 for GF(2^4).
    let m = 4usize;

    for (label, pt) in [("G", g.clone()), ("2G", two_g), ("3G", three_g)] {
        let tau_m = tau_pow(&pt, m, &poly);
        assert_eq!(tau_m, pt, "τ^{m}({label}) ≠ {label}");
    }
}

/// τ^2(G) = G: the orbit of G has size 2 (divides m=4).
///
/// τ(G) = 3G, τ²(G) = τ(3G) = G. ✓
#[test]
fn tau_squared_g_is_g() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();

    let tau2_g = tau_pow(&g, 2, &poly);
    assert_eq!(tau2_g, g, "τ²(G) ≠ G");
}

/// Characteristic relation: τ²(P) − t·τ(P) + 2·P = ∞ for all P.
///
/// For the toy curve: t = 13, n = 4.
/// In the group, scalar multiplication is mod n=4:
///   t mod n = 13 mod 4 = 1.
///   2 mod n = 2.
///
/// For P = G:
///   τ²(G) = G, τ(G) = 3G.
///   τ²(G) − t·τ(G) + 2·G = G − 13·3G + 2·G
///   = G − (13 mod 4)·3G + 2·G  [scalar mult is mod n]
///   = G − 1·3G + 2·G
///   = G + (−3G) + 2·G
///   = G + G + 2·G  [since −3G = G in this group]
///   = 4·G = ∞. ✓
///
/// Note: in the group, "−t·τ(P)" means scalar multiplication by `(n − (t mod n))`.
/// The relation holds over ℤ/nℤ scalars, not over ℤ.
#[test]
fn tau_characteristic_relation() {
    let c = toy_curve();
    let poly = poly4();
    let n = 4u64;
    // Frobenius trace: t = 2^4 + 1 − n = 13.
    let t: u64 = 13;

    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);

    for (label, pt) in [("G", g.clone()), ("2G", two_g.clone()), ("3G", three_g.clone())] {
        let tau1 = tau(&pt, &poly);
        let tau2 = tau(&tau1, &poly);

        // Compute t·τ(P) and 2·P as scalar multiplications.
        let t_mod_n = t % n;
        let t_tau1 = c.scalar_mul(&tau1, &Uint::<1>::from(t_mod_n));
        let two_pt = c.scalar_mul(&pt, &Uint::<1>::from(2u64));

        // τ²(P) − t·τ(P) + 2·P = τ²(P) + (−t·τ(P)) + 2·P.
        // In the group, −Q = negate(Q).
        let neg_t_tau1 = c.negate(&t_tau1);
        let sum = c.add(&c.add(&tau2, &neg_t_tau1), &two_pt);

        assert!(
            sum.is_infinity(),
            "characteristic relation failed for {label}: τ²(P) − t·τ(P) + 2·P ≠ ∞"
        );
    }
}

/// Sub-track-close: the full binary-curve axiom suite stays green after adding τ.
///
/// This test re-runs the core group axioms to confirm the τ implementation
/// does not disturb the underlying curve group law.
#[test]
fn sub_track_close_curve_axioms_intact() {
    let c = toy_curve();
    let poly = poly4();
    let g = c.generator::<F2mNaive<1>>();
    let two_g = c.double(&g);
    let three_g = c.add(&two_g, &g);
    let four_g = c.scalar_mul(&g, &Uint::<1>::from(4u64));

    // Group order.
    assert!(four_g.is_infinity(), "4G ≠ ∞ (group order check)");

    // Negation.
    let neg_g = c.negate(&g);
    assert_eq!(neg_g, three_g, "−G ≠ 3G");
    assert!(c.add(&g, &neg_g).is_infinity(), "G + (−G) ≠ ∞");

    // τ does not disturb the group law: τ(P+Q) = τ(P) + τ(Q).
    let tau_g_plus_two_g = tau(&c.add(&g, &two_g), &poly);
    let tau_g_plus_tau_two_g = c.add(&tau(&g, &poly), &tau(&two_g, &poly));
    assert_eq!(
        tau_g_plus_two_g, tau_g_plus_tau_two_g,
        "τ is not a group homomorphism: τ(G+2G) ≠ τ(G)+τ(2G)"
    );
}
