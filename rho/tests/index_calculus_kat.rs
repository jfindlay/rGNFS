//! Known-answer tests (KATs) for the index-calculus module (E.K.1 + E.K.2 + E.K.3 + E.K.4).
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
//! # KAT coverage (E.K.2 — C-PointDecomp)
//!
//! 4. **decomposition-correctness**: `decompose(Q)` returns `Some(pts)` for a point `Q`
//!    that is a sum of two factor-base points; the returned decomposition sums to `Q` via
//!    the frozen group law.
//! 5. **decomposition-none-for-non-decomposable**: `decompose(Q)` returns `None` for a
//!    point `Q` that is not a sum of any two factor-base points (verified by exhaustive
//!    enumeration of all factor-base pairs).
//! 6. **decomposition-sum-check**: for any returned decomposition `[P_i, P_j]`, verify
//!    `P_i + P_j = Q` using the group law (the primary correctness signal).
//!
//! # KAT coverage (E.K.3 — C-RelationCollect)
//!
//! 7. **relation-validity**: for every relation returned by `collect_relations`, verify
//!    that `Σ e_i·P_i = a·G + b·Q` via the frozen group law. Guards the provenance
//!    `(a, b)` and the exponent-vector encoding.
//! 8. **over-determination**: the returned collection has at least `fb_size + 1` relations.
//!    Guards the loop termination condition.
//!
//! # KAT coverage (E.K.4 — C-EKLinAlg)
//!
//! 9. **adapter-fidelity**: `build_ek_matrix` produces a matrix whose row `i` matches
//!    `relations[i].exponents` exactly (same (index, value) pairs). Guards the near-identity
//!    adapter from `Relation.exponents` to `FlSparseRow`.
//! 10. **kernel-correctness**: `solve_ek_linalg` returns a kernel vector `v` that satisfies
//!    `M·v = 0` over F_ℓ (i.e., for each relation row, the dot product of the exponent
//!    vector with `v` is 0 mod ℓ). Guards the Z/ℓZ linear algebra step.
//!
//! # Principle-4 boundary
//!
//! The fixture is toy-scale (`p = 47`, `n = 60`). The algorithms are mechanism-correct;
//! the asymptotic index-calculus win (which needs `E(F_{p^n})`) is not observable at
//! this scale — a deferred re-shard.

use crypto_bigint::Uint;
use rho::curve::{AffinePoint, JacobianPoint};
use rho::field::{Fp, FpNaive};
use rho::index_calculus::{
    build_ek_matrix, collect_relations, decompose, solve_ek_linalg, IndexCalcStrategy, Relation,
    TOY_ELL, TOY_FB_SIZE,
};

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

// ─── KAT 4: decomposition-correctness (C-PointDecomp) ────────────────────────

/// Decomposition correctness: `decompose(Q)` returns `Some(pts)` for a point `Q` that
/// is a sum of two factor-base points; the returned decomposition sums to `Q` via the
/// frozen group law.
///
/// Constructs `Q = P_0 + P_1` (the sum of the first two factor-base points) directly
/// via the frozen group law, then calls `decompose(Q)` and verifies:
/// - The result is `Some(...)` (the point is decomposable).
/// - The returned decomposition has exactly `m = 2` points.
/// - The sum of the returned points equals `Q` via the frozen group law.
///
/// Guards the Semaev root-finding step (the primary correctness signal for C-PointDecomp).
#[test]
fn decomposition_correctness() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    // Construct Q = P_0 + P_1 directly via the frozen group law.
    // This Q is guaranteed to be decomposable (it is a sum of two factor-base points).
    let p0 = &strategy.factor_base[0].point;
    let p1 = &strategy.factor_base[1].point;
    let q = add_points(curve, p0, p1);

    assert!(
        !q.is_infinity(),
        "P_0 + P_1 should not be ∞ for the toy fixture (they are distinct non-negation points)"
    );

    // Call decompose(Q) — must return Some(...).
    let decomp = decompose(q.clone(), &strategy)
        .expect("decompose(P_0 + P_1) should return Some(...): Q is a sum of two FB points");

    // The decomposition must have exactly m = 2 points.
    assert_eq!(
        decomp.len(),
        strategy.m,
        "decomposition should have exactly m = {} points",
        strategy.m
    );

    // All returned points must be factor-base points (valid indices).
    for fb_pt in &decomp {
        assert!(
            fb_pt.index < strategy.fb_size(),
            "decomposition point index {} is out of factor-base range [0, {})",
            fb_pt.index,
            strategy.fb_size()
        );
        assert!(
            curve.is_on_curve(&fb_pt.point),
            "decomposition point {} is not on the curve: {:?}",
            fb_pt.index,
            fb_pt.point
        );
    }

    // The sum of the returned points must equal Q via the frozen group law.
    let sum = add_points(curve, &decomp[0].point, &decomp[1].point);
    assert_eq!(
        sum, q,
        "decompose returned a decomposition that does not sum to Q: \
         decomp[0] = {:?}, decomp[1] = {:?}, sum = {:?}, Q = {:?}",
        decomp[0].point, decomp[1].point, sum, q
    );
}

// ─── KAT 5: decomposition-none-for-non-decomposable (C-PointDecomp) ──────────

/// Decomposition returns `None` for a point that is not a sum of any two factor-base
/// points.
///
/// Finds a point `Q` (a multiple of `G`) that is not the sum of any two factor-base
/// points by exhaustive enumeration of all factor-base pairs. Then calls `decompose(Q)`
/// and verifies it returns `None`.
///
/// Guards the `None` path of `decompose` — the Semaev polynomial correctly identifies
/// non-decomposable points.
#[test]
fn decomposition_none_for_non_decomposable() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    // Build the set of all points that ARE sums of two factor-base points.
    // This is the set { P_i + P_j : 0 ≤ i, j < FB_SIZE, P_i + P_j ≠ ∞ }.
    let mut decomposable_points: Vec<AffinePoint<FpNaive>> = Vec::new();
    for fb_i in &strategy.factor_base {
        for fb_j in &strategy.factor_base {
            let sum = add_points(curve, &fb_i.point, &fb_j.point);
            if !sum.is_infinity() {
                decomposable_points.push(sum);
            }
        }
    }

    // Find a multiple of G that is NOT in the decomposable set.
    // Try k·G for k = 1, 2, ..., n-1 until we find a non-decomposable point.
    let g: AffinePoint<FpNaive> = curve.generator();
    let n_u64 = curve.n.as_words()[0]; // n = 60

    let mut non_decomposable_q: Option<AffinePoint<FpNaive>> = None;
    for k in 1..n_u64 {
        let q = scalar_mul_u64(curve, &g, k);
        if q.is_infinity() {
            continue;
        }
        if !decomposable_points.contains(&q) {
            non_decomposable_q = Some(q);
            break;
        }
    }

    let q = non_decomposable_q
        .expect("should find a non-decomposable multiple of G in the toy fixture");

    // decompose(Q) must return None.
    let result = decompose(q.clone(), &strategy);
    assert!(
        result.is_none(),
        "decompose(Q) should return None for a non-decomposable point Q = {:?}, \
         but returned {:?}",
        q,
        result
    );
}

// ─── KAT 6: decomposition-sum-check (C-PointDecomp) ─────────────────────────

/// For every returned decomposition `[P_i, P_j]`, verify `P_i + P_j = Q` using the
/// frozen group law.
///
/// Exhaustively tries all factor-base pairs as candidate Q values, calls `decompose(Q)`
/// for each, and verifies that every returned decomposition sums to Q. This is the
/// primary correctness signal: the Semaev polynomial correctly identifies decomposable
/// points and the returned decomposition is valid.
///
/// Guards the sum-to-Q invariant across all decomposable points in the factor-base
/// span (not just one example).
#[test]
fn decomposition_sum_check() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    let mut checked = 0usize;

    // Try all sums of two factor-base points as candidate Q values.
    for fb_i in &strategy.factor_base {
        for fb_j in &strategy.factor_base {
            let q = add_points(curve, &fb_i.point, &fb_j.point);
            if q.is_infinity() {
                continue;
            }

            // Call decompose(Q).
            if let Some(decomp) = decompose(q.clone(), &strategy) {
                assert_eq!(
                    decomp.len(),
                    strategy.m,
                    "decomposition should have exactly m = {} points",
                    strategy.m
                );

                // Verify P_i + P_j = Q via the frozen group law.
                let sum = add_points(curve, &decomp[0].point, &decomp[1].point);
                assert_eq!(
                    sum, q,
                    "decomposition sum check failed: decomp[0] = {:?}, decomp[1] = {:?}, \
                     sum = {:?}, Q = {:?}",
                    decomp[0].point, decomp[1].point, sum, q
                );

                // Verify all returned points are on the curve.
                for fb_pt in &decomp {
                    assert!(
                        curve.is_on_curve(&fb_pt.point),
                        "decomposition point {} is not on the curve: {:?}",
                        fb_pt.index,
                        fb_pt.point
                    );
                }

                checked += 1;
            }
        }
    }

    assert!(
        checked > 0,
        "no decomposable Q found among all factor-base pairs — \
         unexpected for the toy fixture (factor base should span some decomposable points)"
    );
}

// ─── KAT 7: relation-validity (C-RelationCollect) ────────────────────────────

/// For every relation returned by `collect_relations`, verify `Σ e_i·P_i = a·G + b·Q`.
///
/// Calls `collect_relations(G, Q, &strategy)` for the toy fixture. For each returned
/// `Relation`, reconstructs `Σ e_i·P_i` from the exponent vector using the frozen group
/// law, and verifies it equals `a·G + b·Q` (computed from the provenance fields).
///
/// Guards:
/// - The provenance `(a, b)` is faithfully recorded.
/// - The exponent vector correctly encodes the decomposition.
/// - The `collect_relations` loop only records valid decompositions.
#[test]
fn relation_validity() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;
    let p = &curve.p;

    // Use G and Q = 7·G as the toy fixture points.
    let g: AffinePoint<FpNaive> = curve.generator();
    let q = scalar_mul_u64(curve, &g, 7);

    let relations = collect_relations(g.clone(), q.clone(), &strategy)
        .expect("collect_relations should succeed for the toy fixture");

    assert!(
        !relations.is_empty(),
        "collect_relations returned an empty relation set"
    );

    for (idx, relation) in relations.iter().enumerate() {
        // Compute a·G + b·Q from the provenance fields.
        let ag = scalar_mul_u64(curve, &g, relation.a);
        let bq = scalar_mul_u64(curve, &q, relation.b);
        let expected = if bq.is_infinity() {
            ag.clone()
        } else if ag.is_infinity() {
            bq.clone()
        } else {
            add_points(curve, &ag, &bq)
        };

        // Reconstruct Σ e_i·P_i from the relation's exponent vector.
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
            reconstructed_affine, expected,
            "relation {} (a={}, b={}): Σ e_i·P_i = {:?}, expected a·G + b·Q = {:?}",
            idx, relation.a, relation.b, reconstructed_affine, expected
        );
    }
}

// ─── KAT 8: over-determination (C-RelationCollect) ───────────────────────────

/// The returned collection has at least `fb_size + 1` relations.
///
/// Calls `collect_relations(G, Q, &strategy)` for the toy fixture and verifies that
/// the returned `Vec<Relation>` has at least `strategy.fb_size() + 1` entries. This
/// is the over-determination condition required for the index-calculus linear algebra
/// (E.K.4) to have a non-trivial kernel.
///
/// Guards the loop termination condition in `collect_relations`.
#[test]
fn over_determination() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    let g: AffinePoint<FpNaive> = curve.generator();
    let q = scalar_mul_u64(curve, &g, 7);

    let relations = collect_relations(g, q, &strategy)
        .expect("collect_relations should succeed for the toy fixture");

    let min_required = strategy.fb_size() + 1;
    assert!(
        relations.len() >= min_required,
        "over-determination check failed: got {} relations, need at least {} (fb_size + 1 = {} + 1)",
        relations.len(),
        min_required,
        strategy.fb_size()
    );
}

// ─── KAT 9: adapter-fidelity (C-EKLinAlg) ────────────────────────────────────

/// Adapter fidelity: `build_ek_matrix` row `i` matches `relations[i].exponents` exactly.
///
/// Calls `collect_relations` to get a real relation set, then calls `build_ek_matrix`
/// and verifies that each row of the resulting `FlSparseMatrix` has the same (index, value)
/// pairs as the corresponding `Relation.exponents`. Guards the near-identity adapter from
/// `Relation.exponents` to `FlSparseRow`.
///
/// The adapter is a near-identity copy: `Relation.exponents` is already in the
/// `Vec<(usize, FpNaive)>` shape that `FlSparseRow` expects (sorted by index, no zeros,
/// no duplicates — invariants enforced by `Relation::from_decomposition`).
#[test]
fn adapter_fidelity() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;

    let g: AffinePoint<FpNaive> = curve.generator();
    let q = scalar_mul_u64(curve, &g, 7);

    let relations = collect_relations(g, q, &strategy)
        .expect("collect_relations should succeed for the toy fixture");

    let matrix = build_ek_matrix(&relations, &strategy);

    // Verify dimensions.
    assert_eq!(
        matrix.rows.len(),
        relations.len(),
        "matrix should have one row per relation"
    );
    assert_eq!(
        matrix.num_cols,
        strategy.fb_size(),
        "matrix should have fb_size columns"
    );

    // Verify each row matches the corresponding relation's exponent vector.
    for (i, (row, rel)) in matrix.rows.iter().zip(relations.iter()).enumerate() {
        assert_eq!(
            row.entries.len(),
            rel.exponents.len(),
            "row {i}: entry count should match exponents.len()"
        );
        for (j, ((ri, rv), (ei, ev))) in
            row.entries.iter().zip(rel.exponents.iter()).enumerate()
        {
            assert_eq!(
                ri, ei,
                "row {i}, entry {j}: column index {ri} should match exponent index {ei}"
            );
            assert_eq!(
                rv, ev,
                "row {i}, entry {j}: value should match exponent value"
            );
        }
    }
}

// ─── KAT 10: kernel-correctness (C-EKLinAlg) ─────────────────────────────────

/// Kernel correctness: the kernel vector `v` returned by `solve_ek_linalg` satisfies
/// `M·v = 0` over F_ℓ.
///
/// Calls `collect_relations` then `solve_ek_linalg`. For each relation row (exponent
/// vector), computes the dot product with `v` over F_ℓ and verifies it is zero. This
/// is the primary correctness signal for the Z/ℓZ linear algebra step (C-EKLinAlg).
///
/// Guards:
/// - `build_ek_matrix` correctly encodes the relation system.
/// - `solve_ek_linalg` finds a genuine kernel vector (not a spurious solution).
/// - The kernel vector has length `fb_size` (one entry per factor-base point).
#[test]
fn kernel_correctness() {
    let strategy = IndexCalcStrategy::toy().expect("toy strategy should build");
    let curve = &strategy.curve;
    let ell = ell();

    let g: AffinePoint<FpNaive> = curve.generator();
    let q = scalar_mul_u64(curve, &g, 7);

    let relations = collect_relations(g, q, &strategy)
        .expect("collect_relations should succeed for the toy fixture");

    let kernel = solve_ek_linalg(&relations, &strategy)
        .expect("solve_ek_linalg should find a kernel vector for the over-determined toy system");

    // The kernel vector must have length fb_size.
    assert_eq!(
        kernel.len(),
        strategy.fb_size(),
        "kernel vector length should equal fb_size = {}",
        strategy.fb_size()
    );

    // The kernel vector must not be all-zero (a trivial kernel is not useful).
    let is_nontrivial = kernel.iter().any(|v| !v.is_zero(&ell));
    assert!(
        is_nontrivial,
        "kernel vector should be non-trivial (not all zero)"
    );

    // For each relation row, verify the dot product with the kernel is 0 mod ℓ.
    // dot(row_i, v) = Σ_{(j, e_j) in exponents_i} e_j * v[j]  (mod ℓ)
    for (i, rel) in relations.iter().enumerate() {
        let mut dot = FpNaive::zero(&ell);
        for (j, exp) in &rel.exponents {
            let prod = exp.mul(&kernel[*j], &ell);
            dot = dot.add(&prod, &ell);
        }
        assert!(
            dot.is_zero(&ell),
            "kernel check failed for relation {i} (a={}, b={}): \
             dot product = {:?}, expected 0 mod ℓ",
            rel.a,
            rel.b,
            dot
        );
    }
}
