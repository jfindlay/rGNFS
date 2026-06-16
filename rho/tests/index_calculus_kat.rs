//! Known-answer tests (KATs) for the index-calculus module (E.K.1).
//!
//! # Fixture
//!
//! All tests use the `semaev_toy()` curve: `y² = x³ + x + 33` over `F_47` (`p = 47`,
//! `a = 1`, `b = 33`). The generator `G = (10, 3)` has group order `n = 60 = 2²·3·5`.
//! The prime-order subgroup uses `ℓ = 5` (the largest prime factor of n); the ℓ-order
//! subgroup generator is `G_ℓ = (n/ℓ)·G = 12·G`.
//!
//! # KAT coverage (E.K.1 — C-IndexCalcStrategy + C-EKRelation)
//!
//! 1. **factor-base-on-curve**: every enumerated factor-base point satisfies `is_on_curve`
//!    via the frozen `Curve`. Guards the QR-test + canonical-root-lift enumeration.
//! 2. **subgroup-validity**: `ℓ | n` (i.e., `n % ℓ == 0`); the subgroup generator
//!    `G_ℓ = 12·G` satisfies `ℓ·G_ℓ = ∞` (the identity). Guards the prime-order-subgroup
//!    precondition for the `Z/ℓℤ` linear algebra.
//! 3. **C-EKRelation round-trip**: construct a `Relation` from a known decomposition
//!    (factor-base points 0 and 1), then verify that `Σ e_i·P_i` (using the frozen group
//!    law) equals the expected point sum. Guards the exponent-vector encoding and the
//!    `from_decomposition` / `exponent` API.
//!
//! # Principle-4 boundary
//!
//! The fixture is toy-scale (`p = 47`, `n = 60`). The algorithms are mechanism-correct;
//! the asymptotic index-calculus win (which needs `E(F_{p^n})`) is not observable at
//! this scale — a deferred re-shard.

use crypto_bigint::Uint;
use rho::curve::{AffinePoint, JacobianPoint};
use rho::field::{Fp, FpNaive};
use rho::index_calculus::{IndexCalcStrategy, Relation, TOY_ELL, TOY_FB_SIZE};

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Return the `Uint<4>` prime-order subgroup modulus for the toy fixture.
fn ell() -> Uint<4> {
    Uint::<4>::from(TOY_ELL)
}

/// Add two affine points using the frozen `Curve` group law.
///
/// Converts to Jacobian, adds, converts back to affine.
fn add_points(
    curve: &rho::curve::Curve,
    p1: &AffinePoint<FpNaive>,
    p2: &AffinePoint<FpNaive>,
) -> AffinePoint<FpNaive> {
    let p = &curve.p;
    let j1 = JacobianPoint::from_affine(p1, p);
    let j2 = JacobianPoint::from_affine(p2, p);
    curve.add_jacobian(&j1, &j2).to_affine(p)
}

/// Scalar-multiply an affine point by a `u64` scalar.
fn scalar_mul_u64(
    curve: &rho::curve::Curve,
    pt: &AffinePoint<FpNaive>,
    k: u64,
) -> AffinePoint<FpNaive> {
    curve.scalar_mul(pt, &Uint::<4>::from(k))
}

// ─── KAT 1: factor-base-on-curve ─────────────────────────────────────────────

/// Every enumerated factor-base point satisfies `is_on_curve` via the frozen `Curve`.
///
/// Guards the QR-test + canonical-root-lift enumeration in `enumerate_factor_base`.
/// If any point is off-curve, the enumeration has a bug (wrong y, wrong x, or wrong
/// QR test).
#[test]
fn factor_base_on_curve() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    assert_eq!(
        strategy.fb_size(),
        TOY_FB_SIZE,
        "factor base should have exactly TOY_FB_SIZE = {TOY_FB_SIZE} points"
    );

    for fb in &strategy.factor_base {
        assert!(
            strategy.curve.is_on_curve(&fb.point),
            "factor-base point {} is not on the curve: {:?}",
            fb.index,
            fb.point
        );
        // Also verify it is a finite point (not the point at infinity).
        assert!(
            !fb.point.is_infinity(),
            "factor-base point {} is the point at infinity (unexpected)",
            fb.index
        );
    }
}

// ─── KAT 2: subgroup-validity ─────────────────────────────────────────────────

/// `ℓ | n` and `ℓ·G_ℓ = ∞` (the prime-order-subgroup precondition).
///
/// Guards the `Z/ℓℤ` linear-algebra precondition: the relation exponents live in a
/// field `F_ℓ` only if ℓ is prime and divides n. The KAT checks both the divisibility
/// condition and the order of the subgroup generator `G_ℓ = (n/ℓ)·G`.
#[test]
fn subgroup_validity() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    // ℓ | n.
    let n_u64 = curve.n.as_words()[0];
    let ell_u64 = strategy.ell.as_words()[0];
    assert_eq!(
        n_u64 % ell_u64,
        0,
        "ℓ = {ell_u64} must divide n = {n_u64}"
    );

    // G_ℓ = (n/ℓ)·G is not the identity.
    let g_ell = strategy.subgroup_generator();
    assert!(
        !g_ell.is_infinity(),
        "G_ℓ = (n/ℓ)·G should not be ∞ (it has order ℓ = {ell_u64})"
    );

    // ℓ·G_ℓ = ∞.
    let ell_times_g_ell = curve.scalar_mul(&g_ell, &strategy.ell);
    assert!(
        ell_times_g_ell.is_infinity(),
        "ℓ·G_ℓ should be ∞ (G_ℓ has order ℓ = {ell_u64}), got {:?}",
        ell_times_g_ell
    );

    // Sanity: (ℓ-1)·G_ℓ ≠ ∞ (G_ℓ has exact order ℓ, not a proper divisor).
    let ell_minus_1 = Uint::<4>::from(ell_u64 - 1);
    let ell_minus_1_times_g_ell = curve.scalar_mul(&g_ell, &ell_minus_1);
    assert!(
        !ell_minus_1_times_g_ell.is_infinity(),
        "(ℓ-1)·G_ℓ should not be ∞ (G_ℓ has exact order ℓ = {ell_u64})"
    );
}

// ─── KAT 3: C-EKRelation round-trip ──────────────────────────────────────────

/// C-EKRelation round-trip: `Σ e_i·P_i` reconstructed from the relation equals the
/// expected point sum.
///
/// Constructs a `Relation` from a known decomposition (factor-base points 0 and 1,
/// each appearing once), then reconstructs the sum `e_0·P_0 + e_1·P_1` using the
/// frozen group law and verifies it equals `P_0 + P_1` (the direct sum).
///
/// This guards:
/// - `Relation::from_decomposition` correctly encodes the decomposition as a sparse
///   exponent vector.
/// - `Relation::exponent` correctly retrieves exponents (including zero for absent indices).
/// - The exponent vector faithfully represents the decomposition (the round-trip invariant
///   the whole pipeline relies on: a relation's exponent vector reconstructs its recorded
///   factor-base-point sum).
#[test]
fn c_ek_relation_round_trip() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;
    let ell = ell();

    // Take the first two factor-base points.
    let p0 = &strategy.factor_base[0].point;
    let p1 = &strategy.factor_base[1].point;

    // Compute the expected sum P_0 + P_1 directly via the frozen group law.
    let expected_sum = add_points(curve, p0, p1);

    // Construct a Relation encoding the decomposition [P_0, P_1] (each once).
    // Provenance (a=0, b=0) is placeholder — the round-trip KAT checks the exponent
    // vector, not the provenance.
    let relation = Relation::from_decomposition(0, 0, &[0, 1], &ell);

    // Verify the exponent vector: e_0 = 1, e_1 = 1, all others = 0.
    assert_eq!(
        relation.exponent(0, &ell),
        FpNaive::from_u64(1, &ell),
        "exponent of factor-base point 0 should be 1"
    );
    assert_eq!(
        relation.exponent(1, &ell),
        FpNaive::from_u64(1, &ell),
        "exponent of factor-base point 1 should be 1"
    );
    assert_eq!(
        relation.exponent(2, &ell),
        FpNaive::from_u64(0, &ell),
        "exponent of absent factor-base point 2 should be 0"
    );

    // Reconstruct the sum Σ e_i·P_i from the relation's exponent vector.
    // For each (i, e_i) in the relation, add e_i·P_i to the running sum.
    let p = &curve.p;
    let mut reconstructed = JacobianPoint::<FpNaive>::infinity(p);
    for (i, exp) in &relation.exponents {
        let fb_point = &strategy.factor_base[*i].point;
        // e_i is in F_ℓ; convert to u64 for scalar_mul.
        let exp_u64 = exp.to_uint().as_words()[0];
        if exp_u64 == 0 {
            continue;
        }
        let contribution = scalar_mul_u64(curve, fb_point, exp_u64);
        let contrib_jac = JacobianPoint::from_affine(&contribution, p);
        reconstructed = curve.add_jacobian(&reconstructed, &contrib_jac);
    }
    let reconstructed_affine = reconstructed.to_affine(p);

    assert_eq!(
        reconstructed_affine, expected_sum,
        "round-trip failed: Σ e_i·P_i = {:?}, expected P_0 + P_1 = {:?}",
        reconstructed_affine, expected_sum
    );
}

// ─── KAT 3b: C-EKRelation round-trip with repeated indices ───────────────────

/// C-EKRelation round-trip with a repeated factor-base index.
///
/// Constructs a `Relation` from the decomposition [P_0, P_0] (factor-base point 0
/// appearing twice), then verifies that `Σ e_i·P_i = 2·P_0`.
///
/// Guards the accumulation logic in `from_decomposition` (repeated indices → exponent 2).
#[test]
fn c_ek_relation_round_trip_repeated() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;
    let ell = ell();

    let p0 = &strategy.factor_base[0].point;

    // Expected: 2·P_0.
    let expected_sum = scalar_mul_u64(curve, p0, 2);

    // Decomposition: [P_0, P_0] → exponent vector [(0, 2)].
    let relation = Relation::from_decomposition(0, 0, &[0, 0], &ell);

    assert_eq!(
        relation.exponent(0, &ell),
        FpNaive::from_u64(2, &ell),
        "exponent of factor-base point 0 (appearing twice) should be 2"
    );
    assert_eq!(relation.exponents.len(), 1, "only one non-zero exponent");

    // Reconstruct Σ e_i·P_i.
    let p = &curve.p;
    let mut reconstructed = JacobianPoint::<FpNaive>::infinity(p);
    for (i, exp) in &relation.exponents {
        let fb_point = &strategy.factor_base[*i].point;
        let exp_u64 = exp.to_uint().as_words()[0];
        if exp_u64 == 0 {
            continue;
        }
        let contribution = scalar_mul_u64(curve, fb_point, exp_u64);
        let contrib_jac = JacobianPoint::from_affine(&contribution, p);
        reconstructed = curve.add_jacobian(&reconstructed, &contrib_jac);
    }
    let reconstructed_affine = reconstructed.to_affine(p);

    assert_eq!(
        reconstructed_affine, expected_sum,
        "round-trip (repeated) failed: 2·P_0 via relation = {:?}, direct = {:?}",
        reconstructed_affine, expected_sum
    );
}
