//! Known-answer tests for the toy elliptic curve and the controlled point-addition circuit.
//!
//! # What is tested
//!
//! 1. **Classical group-law correctness** — the toy curve's classical point-addition,
//!    doubling, negation, and scalar-mul match hand-computed values; `r·G = ∞` (order check).
//!
//! 2. **Permutation correctness** — the controlled point-addition circuit on basis state
//!    `|P⟩` (control set) gives `|P + cG⟩` for the classically-computed sum, across a
//!    fixture of `(P, cG)` pairs including exceptional cases (`P = ∞`, `P = cG`, `P = −cG`).
//!
//! 3. **Reversibility** — the circuit followed by its inverse is the identity on basis states.
//!
//! 4. **Scratch-clean** — the λ scratch register returns to |0⟩ after the circuit. Since the
//!    permutation synthesis is ancilla-free, this is verified by the full-state
//!    forward+inverse=identity check (per S.B.1 precedent).
//!
//! 5. **Control-off no-op** — with the control qubit |0⟩, the circuit is the identity.
//!
//! # Curve parameters (C-PointAdd freeze)
//!
//! ```text
//! p = 7, a = 0, b = 3: y² = x³ + 3 mod 7
//! G = (1, 2), r = 13 (prime order)
//! ```
//!
//! # Published reference values
//!
//! Group elements (all 13, in scalar-multiple order from G):
//! ```text
//! 0·G = ∞
//! 1·G = (1, 2)
//! 2·G = (6, 3)   [doubling G]
//! 3·G = (2, 2)
//! 4·G = (4, 5)
//! 5·G = (3, 3)
//! 6·G = (5, 3)
//! 7·G = (5, 4)   [= −6·G]
//! 8·G = (3, 4)   [= −5·G]
//! 9·G = (4, 2)   [= −4·G]
//! 10·G = (2, 5)  [= −3·G]
//! 11·G = (6, 4)  [= −2·G]
//! 12·G = (1, 5)  [= −G]
//! 13·G = ∞       [order check]
//! ```
//!
//! # Basis-indexing convention
//!
//! Little-endian (qubit 0 = LSB), matching the frozen S.A.1 C-StateVec convention.
//! Register layout: qubit 0 = control, qubits [1,4) = x, qubits [4,7) = y, qubits [7,10) = λ.

use shor::arith::find_basis_index;
use shor::curve::{
    self, A, B, G, P, R, Point, PointAddLayout, add, all_affine_points, decode_point, double,
    encode_point, negate, on_curve, scalar_mul,
};
use shor::ecc::{controlled_point_add, controlled_point_add_inv, make_point_state, read_lambda,
                read_point};
use shor::statevec::StateVec;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Amplitude tolerance for basis-state KATs (exact classical permutation, no ε needed).
const EPS: f64 = 1e-9;

/// Assert that `sv` is in basis state `expected_idx`.
fn assert_basis_state(sv: &StateVec, expected_idx: usize, label: &str) {
    let got_idx = find_basis_index(sv);
    assert_eq!(
        got_idx, expected_idx,
        "{label}: expected basis state |{expected_idx}⟩, got |{got_idx}⟩"
    );
}

/// Assert that the point registers encode `expected`.
fn assert_point(sv: &StateVec, layout: &PointAddLayout, expected: Point, label: &str) {
    let got = read_point(sv, layout);
    assert_eq!(got, expected, "{label}: expected {expected:?}, got {got:?}");
}

/// Assert that two state vectors are identical (all amplitudes match within EPS).
fn assert_states_equal(sv: &StateVec, expected: &StateVec, label: &str) {
    assert_eq!(
        sv.n_qubits(),
        expected.n_qubits(),
        "{label}: qubit count mismatch"
    );
    for (i, (&got, &exp)) in sv.amplitudes().iter().zip(expected.amplitudes().iter()).enumerate() {
        let diff = (got - exp).norm();
        assert!(
            diff < EPS,
            "{label}: amplitude[{i}] diff={diff:.2e} (got={got:?}, expected={exp:?})"
        );
    }
}

/// The standard layout for all circuit KATs.
fn layout() -> PointAddLayout {
    PointAddLayout::standard()
}

// ── KAT group 1: classical group-law correctness ──────────────────────────────
//
// Verify the toy curve's classical group law against hand-computed values.
// Reference: the 13 group elements listed in the module doc.

/// Curve parameters: p=7, a=0, b=3.
#[test]
fn curve_params() {
    assert_eq!(P, 7, "field prime");
    assert_eq!(A, 0, "coefficient a");
    assert_eq!(B, 3, "coefficient b");
    assert_eq!(R, 13, "group order");
}

/// Generator G = (1, 2) is on the curve.
#[test]
fn generator_on_curve() {
    assert!(on_curve(G), "G = (1,2) must be on the curve");
    assert_eq!(G, Point::Affine { x: 1, y: 2 });
}

/// All 12 affine points are on the curve.
#[test]
fn all_points_on_curve() {
    let pts = all_affine_points();
    assert_eq!(pts.len(), 12, "curve y²=x³+3 mod 7 has exactly 12 affine points");
    for p in &pts {
        assert!(on_curve(*p), "{p:?} must be on the curve");
    }
}

/// Point (0, 0) is NOT on the curve (used as ∞ encoding).
#[test]
fn origin_not_on_curve() {
    assert!(
        !on_curve(Point::Affine { x: 0, y: 0 }),
        "(0,0) must not be on the curve (used as ∞ sentinel)"
    );
}

/// Negation: −G = (1, 5) = 12·G.
#[test]
fn negation_of_g() {
    let neg_g = negate(G);
    assert_eq!(neg_g, Point::Affine { x: 1, y: 5 }, "−G = (1,5)");
    assert!(on_curve(neg_g), "−G must be on the curve");
}

/// Negation: −∞ = ∞.
#[test]
fn negation_of_infinity() {
    assert_eq!(negate(Point::Infinity), Point::Infinity, "−∞ = ∞");
}

/// P + (−P) = ∞ for G.
#[test]
fn add_point_and_negation() {
    let neg_g = negate(G);
    let sum = add(G, neg_g);
    assert_eq!(sum, Point::Infinity, "G + (−G) = ∞");
}

/// P + ∞ = P and ∞ + P = P (identity law).
#[test]
fn add_identity() {
    assert_eq!(add(G, Point::Infinity), G, "G + ∞ = G");
    assert_eq!(add(Point::Infinity, G), G, "∞ + G = G");
    assert_eq!(add(Point::Infinity, Point::Infinity), Point::Infinity, "∞ + ∞ = ∞");
}

/// Doubling: 2·G = (6, 3).
#[test]
fn double_g() {
    let two_g = double(G);
    assert_eq!(two_g, Point::Affine { x: 6, y: 3 }, "2G = (6,3)");
    assert!(on_curve(two_g), "2G must be on the curve");
}

/// Scalar multiplication: k·G for k = 0..13 matches published values.
///
/// Reference table (hand-computed via the affine addition formula, verified by the
/// negation symmetry `neg(k·G) = (13−k)·G` and the order check `13·G = ∞`):
/// ```text
/// 0·G = ∞,      1·G = (1,2),  2·G = (6,3),  3·G = (2,2),
/// 4·G = (4,5),  5·G = (3,3),  6·G = (5,3),  7·G = (5,4),
/// 8·G = (3,4),  9·G = (4,2),  10·G = (2,5), 11·G = (6,4),
/// 12·G = (1,5), 13·G = ∞
/// ```
/// Negation check: neg(k·G) = (13−k)·G, e.g. neg(4·G)=(4,2)=9·G. ✓
#[test]
fn scalar_mul_all() {
    let expected: &[(u64, Option<(u64, u64)>)] = &[
        (0, None),           // 0·G = ∞
        (1, Some((1, 2))),   // 1·G = G
        (2, Some((6, 3))),   // 2·G
        (3, Some((2, 2))),   // 3·G
        (4, Some((4, 5))),   // 4·G  [λ=(2-2)/(2-1)=0; x₃=0-1-2=4; y₃=0-2=5]
        (5, Some((3, 3))),   // 5·G  [λ=(5-2)/(4-1)=1; x₃=1-1-4=3; y₃=1(1-3)-2=3]
        (6, Some((5, 3))),   // 6·G  [λ=(3-2)/(3-1)=4; x₃=16-1-3=5; y₃=4(1-5)-2=3]
        (7, Some((5, 4))),   // 7·G = −6·G  [neg((5,3))=(5,4)]
        (8, Some((3, 4))),   // 8·G = −5·G  [neg((3,3))=(3,4)]
        (9, Some((4, 2))),   // 9·G = −4·G  [neg((4,5))=(4,2)]
        (10, Some((2, 5))),  // 10·G = −3·G [neg((2,2))=(2,5)]
        (11, Some((6, 4))),  // 11·G = −2·G [neg((6,3))=(6,4)]
        (12, Some((1, 5))),  // 12·G = −G   [neg((1,2))=(1,5)]
        (13, None),          // 13·G = ∞ (order check)
    ];
    for &(k, coords) in expected {
        let got = scalar_mul(k, G);
        let want = match coords {
            None => Point::Infinity,
            Some((x, y)) => Point::Affine { x, y },
        };
        assert_eq!(got, want, "{k}·G: expected {want:?}, got {got:?}");
        if got != Point::Infinity {
            assert!(on_curve(got), "{k}·G = {got:?} must be on the curve");
        }
    }
}

/// Order check: r·G = ∞ (the group order is exactly 13).
#[test]
fn order_check() {
    let r_g = scalar_mul(R, G);
    assert_eq!(r_g, Point::Infinity, "r·G = {R}·G must be ∞ (order check)");
}

/// Commutativity: P + Q = Q + P for a sample of pairs.
#[test]
fn commutativity() {
    let two_g = scalar_mul(2, G);
    let three_g = scalar_mul(3, G);
    assert_eq!(add(two_g, three_g), add(three_g, two_g), "2G + 3G = 3G + 2G");
    assert_eq!(add(G, two_g), add(two_g, G), "G + 2G = 2G + G");
}

/// Associativity: (P + Q) + R = P + (Q + R) for a sample of triples.
#[test]
fn associativity() {
    let two_g = scalar_mul(2, G);
    let three_g = scalar_mul(3, G);
    let four_g = scalar_mul(4, G);
    let lhs = add(add(G, two_g), three_g);
    let rhs = add(G, add(two_g, three_g));
    assert_eq!(lhs, rhs, "(G + 2G) + 3G = G + (2G + 3G)");
    let lhs2 = add(add(two_g, three_g), four_g);
    let rhs2 = add(two_g, add(three_g, four_g));
    assert_eq!(lhs2, rhs2, "(2G + 3G) + 4G = 2G + (3G + 4G)");
}

/// Scalar-mul consistency: (k+1)·G = k·G + G.
#[test]
fn scalar_mul_consistency() {
    for k in 0..12u64 {
        let kg = scalar_mul(k, G);
        let k1g = scalar_mul(k + 1, G);
        let sum = add(kg, G);
        assert_eq!(sum, k1g, "({k}+1)·G = {k}·G + G failed: {sum:?} != {k1g:?}");
    }
}

/// Encoding: encode_point / decode_point round-trip for all group elements.
#[test]
fn encode_decode_roundtrip() {
    // ∞ encodes as (0, 0).
    let (x, y) = encode_point(Point::Infinity);
    assert_eq!((x, y), (0, 0), "∞ encodes as (0,0)");
    assert_eq!(decode_point(0, 0), Point::Infinity, "(0,0) decodes as ∞");

    // All affine points round-trip.
    for p in all_affine_points() {
        let (x, y) = encode_point(p);
        let decoded = decode_point(x, y);
        assert_eq!(decoded, p, "encode/decode round-trip for {p:?}");
    }
}

// ── KAT group 2: permutation correctness ─────────────────────────────────────
//
// For each (P, cG) pair, verify that the controlled point-addition circuit on
// basis state |P⟩ (control set) gives |P + cG⟩.

fn check_point_add_circuit(p: Point, cg: Point, label: &str) {
    let layout = layout();
    let mut sv = make_point_state(1, p, &layout); // control = |1⟩
    let expected = add(p, cg);
    controlled_point_add(&mut sv, cg, &layout);
    assert_point(&sv, &layout, expected, label);
}

/// Permutation correctness: G + G = 2G = (6,3).
#[test]
fn circuit_g_plus_g() {
    check_point_add_circuit(G, G, "G + G");
}

/// Permutation correctness: G + 2G = 3G = (2,2).
#[test]
fn circuit_g_plus_2g() {
    let two_g = scalar_mul(2, G);
    check_point_add_circuit(G, two_g, "G + 2G");
}

/// Permutation correctness: 2G + 3G = 5G = (4,2).
#[test]
fn circuit_2g_plus_3g() {
    let two_g = scalar_mul(2, G);
    let three_g = scalar_mul(3, G);
    check_point_add_circuit(two_g, three_g, "2G + 3G");
}

/// Permutation correctness: ∞ + G = G (exceptional case: P = ∞).
#[test]
fn circuit_infinity_plus_g() {
    check_point_add_circuit(Point::Infinity, G, "∞ + G");
}

/// Permutation correctness: ∞ + 2G = 2G (exceptional case: P = ∞).
#[test]
fn circuit_infinity_plus_2g() {
    let two_g = scalar_mul(2, G);
    check_point_add_circuit(Point::Infinity, two_g, "∞ + 2G");
}

/// Permutation correctness: G + (−G) = ∞ (exceptional case: P = −cG).
#[test]
fn circuit_g_plus_neg_g() {
    let neg_g = negate(G);
    check_point_add_circuit(G, neg_g, "G + (−G)");
}

/// Permutation correctness: 3G + (−3G) = ∞ (exceptional case: P = −cG).
#[test]
fn circuit_3g_plus_neg_3g() {
    let three_g = scalar_mul(3, G);
    let neg_3g = negate(three_g);
    check_point_add_circuit(three_g, neg_3g, "3G + (−3G)");
}

/// Permutation correctness: full sweep — for each group element P and cG = G,
/// verify the circuit gives P + G.
#[test]
fn circuit_sweep_cg_equals_g() {
    let layout = layout();
    for k in 0..R {
        let p = scalar_mul(k, G);
        let expected = add(p, G);
        let mut sv = make_point_state(1, p, &layout);
        controlled_point_add(&mut sv, G, &layout);
        assert_point(&sv, &layout, expected, &format!("{k}·G + G"));
    }
}

/// Permutation correctness: full sweep — for each group element P and cG = 2G,
/// verify the circuit gives P + 2G.
#[test]
fn circuit_sweep_cg_equals_2g() {
    let layout = layout();
    let two_g = scalar_mul(2, G);
    for k in 0..R {
        let p = scalar_mul(k, G);
        let expected = add(p, two_g);
        let mut sv = make_point_state(1, p, &layout);
        controlled_point_add(&mut sv, two_g, &layout);
        assert_point(&sv, &layout, expected, &format!("{k}·G + 2G"));
    }
}

/// Permutation correctness: full sweep — for each group element P and cG = 6G,
/// verify the circuit gives P + 6G. Includes P = 7G = −6G (exceptional case).
#[test]
fn circuit_sweep_cg_equals_6g() {
    let layout = layout();
    let six_g = scalar_mul(6, G);
    for k in 0..R {
        let p = scalar_mul(k, G);
        let expected = add(p, six_g);
        let mut sv = make_point_state(1, p, &layout);
        controlled_point_add(&mut sv, six_g, &layout);
        assert_point(&sv, &layout, expected, &format!("{k}·G + 6G"));
    }
}

// ── KAT group 3: reversibility ────────────────────────────────────────────────
//
// The circuit followed by its inverse is the identity on basis states.

fn check_reversibility(p: Point, cg: Point, label: &str) {
    let layout = layout();
    let original = make_point_state(1, p, &layout);
    let mut sv = original.clone();
    controlled_point_add(&mut sv, cg, &layout);
    controlled_point_add_inv(&mut sv, cg, &layout);
    assert_states_equal(&sv, &original, label);
}

/// Reversibility: forward + inverse = identity for P = G, cG = G.
#[test]
fn reversibility_g_cg_g() {
    check_reversibility(G, G, "reversibility G, cG=G");
}

/// Reversibility: forward + inverse = identity for P = ∞, cG = G.
#[test]
fn reversibility_infinity_cg_g() {
    check_reversibility(Point::Infinity, G, "reversibility ∞, cG=G");
}

/// Reversibility: forward + inverse = identity for P = −G, cG = G.
#[test]
fn reversibility_neg_g_cg_g() {
    let neg_g = negate(G);
    check_reversibility(neg_g, G, "reversibility −G, cG=G");
}

/// Reversibility: full sweep — for each group element P and cG = G.
#[test]
fn reversibility_sweep_cg_g() {
    for k in 0..R {
        let p = scalar_mul(k, G);
        check_reversibility(p, G, &format!("reversibility {k}·G, cG=G"));
    }
}

/// Reversibility: full sweep — for each group element P and cG = 3G.
#[test]
fn reversibility_sweep_cg_3g() {
    let three_g = scalar_mul(3, G);
    for k in 0..R {
        let p = scalar_mul(k, G);
        check_reversibility(p, three_g, &format!("reversibility {k}·G, cG=3G"));
    }
}

/// Reversibility: full sweep — for each group element P and cG = 5G.
#[test]
fn reversibility_sweep_cg_5g() {
    let five_g = scalar_mul(5, G);
    for k in 0..R {
        let p = scalar_mul(k, G);
        check_reversibility(p, five_g, &format!("reversibility {k}·G, cG=5G"));
    }
}

// ── KAT group 4: scratch-clean (ancilla-clean) ────────────────────────────────
//
// Every scratch register (λ) returns to |0⟩ after the circuit.
//
// Since the permutation synthesis is ancilla-free, the λ register is never touched.
// The scratch-clean invariant is verified by:
// (a) Directly reading the λ register after the forward circuit (should be 0).
// (b) The full-state forward+inverse=identity check (subsumes scratch-clean per S.B.1).

/// Scratch-clean: λ register is 0 after the forward circuit (P = G, cG = G).
#[test]
fn scratch_clean_lambda_zero_after_forward() {
    let layout = layout();
    let mut sv = make_point_state(1, G, &layout);
    controlled_point_add(&mut sv, G, &layout);
    let lam = read_lambda(&sv, &layout);
    assert_eq!(lam, 0, "λ register must be 0 after forward circuit");
}

/// Scratch-clean: λ register is 0 after the forward circuit for all group elements.
#[test]
fn scratch_clean_sweep() {
    let layout = layout();
    for k in 0..R {
        let p = scalar_mul(k, G);
        let mut sv = make_point_state(1, p, &layout);
        controlled_point_add(&mut sv, G, &layout);
        let lam = read_lambda(&sv, &layout);
        assert_eq!(lam, 0, "λ register must be 0 after circuit for {k}·G + G");
    }
}

/// Scratch-clean (full-state): forward + inverse = identity, verifying all qubits clean.
///
/// This subsumes the scratch-clean invariant per S.B.1 precedent: if the full state
/// vector is restored after forward + inverse, no qubit (including λ) is left entangled.
#[test]
fn scratch_clean_full_state_sweep() {
    let layout = layout();
    for k in 0..R {
        let p = scalar_mul(k, G);
        let original = make_point_state(1, p, &layout);
        let mut sv = original.clone();
        controlled_point_add(&mut sv, G, &layout);
        controlled_point_add_inv(&mut sv, G, &layout);
        let orig_idx = find_basis_index(&original);
        let got_idx = find_basis_index(&sv);
        assert_eq!(
            got_idx, orig_idx,
            "scratch-clean full-state: {k}·G + G: state {got_idx} != {orig_idx}"
        );
        // Verify all amplitudes match.
        for (i, (&got, &exp)) in sv.amplitudes().iter().zip(original.amplitudes().iter()).enumerate()
        {
            let diff = (got - exp).norm();
            assert!(
                diff < EPS,
                "scratch-clean full-state: {k}·G + G: amplitude[{i}] diff={diff:.2e}"
            );
        }
    }
}

// ── KAT group 5: control-off no-op ───────────────────────────────────────────
//
// With the control qubit |0⟩, the circuit is the identity.

fn check_control_off(p: Point, cg: Point, label: &str) {
    let layout = layout();
    let original = make_point_state(0, p, &layout); // control = |0⟩
    let mut sv = original.clone();
    controlled_point_add(&mut sv, cg, &layout);
    // Point registers must be unchanged.
    assert_point(&sv, &layout, p, &format!("{label}: point register unchanged"));
    // Full state must be unchanged.
    let orig_idx = find_basis_index(&original);
    assert_basis_state(&sv, orig_idx, &format!("{label}: full state unchanged"));
}

/// Control-off no-op: P = G, cG = G, ctrl = |0⟩.
#[test]
fn control_off_g_cg_g() {
    check_control_off(G, G, "control-off G, cG=G");
}

/// Control-off no-op: P = ∞, cG = G, ctrl = |0⟩.
#[test]
fn control_off_infinity_cg_g() {
    check_control_off(Point::Infinity, G, "control-off ∞, cG=G");
}

/// Control-off no-op: P = −G, cG = G, ctrl = |0⟩.
#[test]
fn control_off_neg_g_cg_g() {
    let neg_g = negate(G);
    check_control_off(neg_g, G, "control-off −G, cG=G");
}

/// Control-off no-op: full sweep — for each group element P and cG = G, ctrl = |0⟩.
#[test]
fn control_off_sweep_cg_g() {
    for k in 0..R {
        let p = scalar_mul(k, G);
        check_control_off(p, G, &format!("control-off {k}·G, cG=G"));
    }
}

/// Control-off no-op: full sweep — for each group element P and cG = 4G, ctrl = |0⟩.
#[test]
fn control_off_sweep_cg_4g() {
    let four_g = scalar_mul(4, G);
    for k in 0..R {
        let p = scalar_mul(k, G);
        check_control_off(p, four_g, &format!("control-off {k}·G, cG=4G"));
    }
}

// ── additional correctness KATs ───────────────────────────────────────────────

/// Layout: standard layout has correct qubit counts and total.
#[test]
fn layout_standard() {
    let l = layout();
    assert_eq!(l.ctrl_qubit, 0, "control qubit = 0");
    assert_eq!(l.x_start, 1, "x starts at qubit 1");
    assert_eq!(l.x_len, 3, "x is 3 bits (⌈log₂ 7⌉ = 3)");
    assert_eq!(l.y_start, 4, "y starts at qubit 4");
    assert_eq!(l.y_len, 3, "y is 3 bits");
    assert_eq!(l.lam_start, 7, "λ starts at qubit 7");
    assert_eq!(l.lam_len, 3, "λ is 3 bits");
    assert_eq!(l.total_qubits(), 10, "total = 10 qubits");
}

/// Field arithmetic: field_add, field_sub, field_mul, field_inv are correct mod 7.
#[test]
fn field_arithmetic() {
    use shor::curve::{field_add, field_inv, field_mul, field_sub};
    assert_eq!(field_add(5, 4), 2, "5+4=9=2 mod 7");
    assert_eq!(field_sub(2, 5), 4, "2-5=-3=4 mod 7");
    assert_eq!(field_mul(3, 4), 5, "3*4=12=5 mod 7");
    assert_eq!(field_inv(2), 4, "2⁻¹=4 mod 7 (2*4=8=1)");
    assert_eq!(field_inv(3), 5, "3⁻¹=5 mod 7 (3*5=15=1)");
    assert_eq!(field_inv(4), 2, "4⁻¹=2 mod 7");
    assert_eq!(field_inv(5), 3, "5⁻¹=3 mod 7");
    assert_eq!(field_inv(6), 6, "6⁻¹=6 mod 7 (6*6=36=1)");
    assert_eq!(field_inv(1), 1, "1⁻¹=1 mod 7");
}

/// Circuit: adding cG = 12G = −G to P = G gives ∞ (exceptional: P = −cG).
#[test]
fn circuit_exceptional_p_equals_neg_cg() {
    let layout = layout();
    let twelve_g = scalar_mul(12, G); // = −G = (1,5)
    let mut sv = make_point_state(1, G, &layout);
    controlled_point_add(&mut sv, twelve_g, &layout);
    assert_point(&sv, &layout, Point::Infinity, "G + (−G) = ∞");
}

/// Circuit: adding cG = G to P = 12G = −G gives ∞ (exceptional: P = −cG, other direction).
#[test]
fn circuit_exceptional_neg_p_plus_cg() {
    let layout = layout();
    let twelve_g = scalar_mul(12, G); // = −G = (1,5)
    let mut sv = make_point_state(1, twelve_g, &layout);
    controlled_point_add(&mut sv, G, &layout);
    assert_point(&sv, &layout, Point::Infinity, "(−G) + G = ∞");
}

/// Circuit: adding cG = G to P = ∞ gives G (exceptional: P = ∞).
#[test]
fn circuit_exceptional_infinity_input() {
    let layout = layout();
    let mut sv = make_point_state(1, Point::Infinity, &layout);
    controlled_point_add(&mut sv, G, &layout);
    assert_point(&sv, &layout, G, "∞ + G = G");
}

/// Coord qubits: ⌈log₂ 7⌉ = 3.
#[test]
fn coord_qubits_count() {
    assert_eq!(curve::coord_qubits(), 3, "⌈log₂ 7⌉ = 3");
}
