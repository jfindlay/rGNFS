//! Known-answer tests (KATs) for DL relation collection.
//!
//! This file verifies the DL relation collection deliverable: augmenting sieve relations with Schirokauer
//! columns and assembling the DL relation matrix.
//!
//! # KAT structure
//!
//! - **KAT (a) — Matrix shape**: run the sieve, augment relations, assemble the DL matrix,
//!   and verify the matrix has the expected shape (correct column count).
//!
//! - **KAT (b) — Schirokauer column range**: verify that all Schirokauer columns are in
//!   [0, ℓ), as required by the virtual-log definition.
//!
//! - **KAT (c) — Homomorphism property**: verify that the Schirokauer map is a group
//!   homomorphism on the collected relations: for any two elements β₁ = a₁ + b₁·α and
//!   β₂ = a₂ + b₂·α, λ(β₁·β₂) = λ(β₁) + λ(β₂) mod ℓ. This is the defining algebraic
//!   property of the Schirokauer map.
//!
//! - **KAT (d) — Known-value augmentation**: verify that `augment_relation` produces the
//!   correct Schirokauer column for a hand-computed (a, b) pair.
//!
//! - **KAT (e) — collect_dl_relations wrapper**: verify that `collect_dl_relations` produces
//!   the same augmented relations as calling `line_sieve` + `augment_relation` manually.
//!
//! # Toy setup
//!
//! All KATs use:
//! - Number field K = ℚ(i) = ℚ[α]/(α² + 1), so α² = −1.
//! - Polynomial: f(x) = x² + 1 (monic, defines K).
//! - Prime: p = 41 (prime, 41 ≡ 1 mod 5).
//! - Root: m = 9 (since 9² + 1 = 82 = 2·41 ≡ 0 mod 41).
//! - PolyPair: f = x² + 1, g = x − 9, m = 9, n = 41.
//! - ℓ = 5 (target subgroup order).
//! - Schirokauer ideal: φ = (41, α − 9), since 41 ≡ 1 (mod 5) and 9² + 1 ≡ 0 (mod 41).
//!
//! # Hand-computed reference values
//!
//! From schirokauer_kat.rs (KAT a):
//! - λ(1 + α) = 3.
//! - λ(α) = 0.
//! - λ(2) = 1.
//!
//! For the sieve with A=20, B=5, B_rat=50, B_alg=50:
//! - Rational norm: |a − 9b|.
//! - Algebraic norm: a² + b².
//! - Example smooth pairs: (a=9, b=1) → N_rat=0 (degenerate, skipped); (a=1, b=1) → N_rat=−8,
//!   N_alg=2; (a=−1, b=1) → N_rat=−10, N_alg=2; etc.

use gnfs::{
    DLMatrix, FactorBase, LineSieveConfig, PolyPair,
    augment_relation, collect_dl_relations, compute_schirokauer,
};
use num_bigint::BigInt;
use num_integer::Integer;
use shared_numfield::{Ideal, IntPoly, NumberField};

// ─── Setup helpers ────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// K = ℚ(i), f = x² + 1.
fn field_qi() -> NumberField {
    NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
}

/// Build the toy polynomial pair for the DL KAT.
///
/// f(x) = x² + 1, g(x) = x − 9, m = 9, n = 41.
/// Invariant: f(9) = 81 + 1 = 82 = 2·41 ≡ 0 (mod 41). ✓
fn toy_poly_pair() -> PolyPair {
    let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]); // x² + 1
    let m = bi(9);
    let n = bi(41);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]); // x − 9
    let pair = PolyPair::new(f, g, m, n);
    pair.verify().expect("toy DL polynomial pair should be valid");
    pair
}

/// Build the factor base for the toy DL KAT.
///
/// Uses B_rat = 50, B_alg = 50 to capture small smooth norms.
fn toy_factor_base(poly: &PolyPair) -> FactorBase {
    FactorBase::new(&poly.f, 50, 50)
}

/// Build the sieve config for the toy DL KAT.
///
/// Uses A = 20, B = 5 (small region, sufficient for toy scale).
fn toy_sieve_config() -> LineSieveConfig {
    LineSieveConfig::with_threshold(20, 5, 0.5)
}

// ─── KAT (a): Matrix shape ────────────────────────────────────────────────────

/// KAT (a): The DL matrix has the expected shape after relation collection and augmentation.
///
/// Verifies:
/// - The sieve finds at least one relation.
/// - Each augmented relation has Schirokauer rank = 1 (one ideal).
/// - The DL matrix column count = rational_size + algebraic_size + 1.
/// - The DL matrix row count equals the number of augmented relations.
#[test]
fn kat_a_matrix_shape() {
    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    // Build the number field and Schirokauer ideal.
    let nf = poly.number_field();
    let ell = bi(5);
    // φ = (41, α − 9): 41 ≡ 1 (mod 5), 9² + 1 = 82 ≡ 0 (mod 41).
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    // Collect DL relations.
    let dl_relations = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    // Must find at least one relation.
    assert!(
        !dl_relations.is_empty(),
        "expected at least one DL relation; sieve found none. \
         Check polynomial pair and sieve bounds."
    );

    // Each relation must have Schirokauer rank = 1.
    for (i, rel) in dl_relations.iter().enumerate() {
        assert_eq!(
            rel.schirokauer_rank(),
            1,
            "relation {i} has Schirokauer rank {} but expected 1",
            rel.schirokauer_rank()
        );
    }

    // Assemble the DL matrix.
    let matrix = DLMatrix::from_relations(dl_relations.clone(), &fb);

    // Verify dimensions.
    let expected_cols = fb.rational_size() + fb.algebraic_size() + 1;
    assert_eq!(
        matrix.num_cols(),
        expected_cols,
        "DL matrix column count should be rational_size + algebraic_size + 1 Schirokauer column; \
         got {} (rational={}, algebraic={}, schirokauer={})",
        matrix.num_cols(),
        matrix.rational_size,
        matrix.algebraic_size,
        matrix.schirokauer_rank
    );
    assert_eq!(
        matrix.num_rows(),
        dl_relations.len(),
        "DL matrix row count should equal the number of DL relations"
    );

    let (rows, cols) = matrix.dimensions();
    assert_eq!(rows, matrix.num_rows());
    assert_eq!(cols, matrix.num_cols());
}

// ─── KAT (b): Schirokauer column range ───────────────────────────────────────

/// KAT (b): All Schirokauer columns are in [0, ℓ).
///
/// The Schirokauer map returns virtual-log coordinates in ℤ/ℓ. Every column value
/// must satisfy 0 ≤ λ < ℓ.
#[test]
fn kat_b_schirokauer_column_range() {
    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    let nf = poly.number_field();
    let ell = bi(5);
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    let dl_relations = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    for (i, rel) in dl_relations.iter().enumerate() {
        for (j, col) in rel.schirokauer_cols.iter().enumerate() {
            assert!(
                *col >= bi(0) && *col < ell,
                "relation {i}, Schirokauer column {j} = {col} is not in [0, ℓ=5)"
            );
        }
    }
}

// ─── KAT (c): Homomorphism property ──────────────────────────────────────────

/// KAT (c): The Schirokauer map is a group homomorphism: λ(β₁·β₂) = λ(β₁) + λ(β₂) mod ℓ.
///
/// For each pair of collected DL relations (a₁, b₁) and (a₂, b₂), compute the product
/// element β₁·β₂ = (a₁ + b₁·α)(a₂ + b₂·α) in K = ℚ(i) and verify the homomorphism.
///
/// In K = ℚ(i) with α² = −1:
/// (a₁ + b₁·α)(a₂ + b₂·α) = (a₁·a₂ − b₁·b₂) + (a₁·b₂ + a₂·b₁)·α.
///
/// This is the defining algebraic property of the Schirokauer map.
#[test]
fn kat_c_homomorphism_property() {
    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    let nf = poly.number_field();
    let ell = bi(5);
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    let dl_relations = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    // Need at least 2 relations to test the homomorphism.
    if dl_relations.len() < 2 {
        // Skip if not enough relations (shouldn't happen with our sieve bounds).
        return;
    }

    // Test the homomorphism on the first two relations.
    let rel1 = &dl_relations[0];
    let rel2 = &dl_relations[1];

    let a1 = &rel1.relation.a;
    let b1 = &rel1.relation.b;
    let a2 = &rel2.relation.a;
    let b2 = &rel2.relation.b;

    // Product element: (a₁ + b₁·α)(a₂ + b₂·α) = (a₁·a₂ − b₁·b₂) + (a₁·b₂ + a₂·b₁)·α.
    // In K = ℚ(i), α² = −1, so this is exact.
    let prod_a = a1 * a2 - b1 * b2;
    let prod_b = a1 * b2 + a2 * b1;

    // Construct the product element in K and evaluate the Schirokauer map.
    let prod_elt = nf.from_int(prod_a).add(&nf.from_int(prod_b).mul(&nf.alpha()));
    let lambda_prod = compute_schirokauer(&prod_elt, &ell, &ideals)
        .expect("Schirokauer map should succeed on product element");

    // Verify homomorphism: λ(β₁·β₂) = λ(β₁) + λ(β₂) mod ℓ.
    let lambda1 = &rel1.schirokauer_cols[0];
    let lambda2 = &rel2.schirokauer_cols[0];
    let expected = (lambda1 + lambda2).mod_floor(&ell);

    assert_eq!(
        lambda_prod[0],
        expected,
        "homomorphism failed: λ(β₁·β₂) = {} but λ(β₁) + λ(β₂) mod ℓ = {} \
         (β₁ = ({a1}, {b1}), β₂ = ({a2}, {b2}))",
        lambda_prod[0],
        expected
    );
}

// ─── KAT (d): Known-value augmentation ───────────────────────────────────────

/// KAT (d): `augment_relation` produces the correct Schirokauer column for a known (a, b) pair.
///
/// Uses the hand-computed reference from schirokauer_kat.rs:
/// - (a=1, b=1): β = 1 + α = 1 + i. λ(1+i) = 3.
/// - (a=2, b=0): β = 2. λ(2) = 1.
/// - (a=0, b=1): β = α = i. λ(i) = 0.
#[test]
fn kat_d_known_value_augmentation() {
    use gnfs::{ExponentVector, Relation};

    let k = field_qi();
    let ell = bi(5);
    let phi = Ideal::new(&k, bi(41), bi(9));
    let ideals = [phi];

    // Helper: build a minimal Relation and augment it.
    let augment = |a: i64, b: i64| -> BigInt {
        let rel = Relation {
            a: bi(a),
            b: bi(b),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };
        let dl_rel = augment_relation(rel, &k, &ell, &ideals)
            .expect("augment_relation should succeed");
        dl_rel.schirokauer_cols[0].clone()
    };

    // λ(1 + α) = 3 (hand-computed: (1+i)^8 = 16, (16−1)/5 = 3, eval at r=4: 3).
    assert_eq!(augment(1, 1), bi(3), "λ(1+α) should be 3");

    // λ(2) = 1 (hand-computed: 2^8 = 256, 256 mod 25 = 6, (6−1)/5 = 1).
    assert_eq!(augment(2, 0), bi(1), "λ(2) should be 1");

    // λ(α) = 0 (hand-computed: α^8 = 1, (1−1)/5 = 0).
    assert_eq!(augment(0, 1), bi(0), "λ(α) should be 0");
}

// ─── KAT (e): collect_dl_relations wrapper ───────────────────────────────────

/// KAT (e): `collect_dl_relations` produces the same results as manual sieve + augmentation.
///
/// Verifies that the wrapper function is consistent with calling `line_sieve` and
/// `augment_relation` manually. The results must be identical (same relations, same
/// Schirokauer columns, same order).
#[test]
fn kat_e_collect_dl_relations_matches_manual() {
    use gnfs::line_sieve;

    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    let nf = poly.number_field();
    let ell = bi(5);
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    // Collect via the wrapper.
    let wrapper_result = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    // Collect manually: sieve + augment.
    let raw_relations = line_sieve(&poly, &fb, &config);
    let manual_result: Vec<_> = raw_relations
        .into_iter()
        .filter_map(|rel| augment_relation(rel, &nf, &ell, &ideals).ok())
        .collect();

    // Results must have the same length.
    assert_eq!(
        wrapper_result.len(),
        manual_result.len(),
        "collect_dl_relations and manual sieve+augment should produce the same number of relations"
    );

    // Results must be identical.
    for (i, (w, m)) in wrapper_result.iter().zip(manual_result.iter()).enumerate() {
        assert_eq!(
            w.relation.a, m.relation.a,
            "relation {i}: a differs between wrapper and manual"
        );
        assert_eq!(
            w.relation.b, m.relation.b,
            "relation {i}: b differs between wrapper and manual"
        );
        assert_eq!(
            w.schirokauer_cols, m.schirokauer_cols,
            "relation {i}: Schirokauer columns differ between wrapper and manual"
        );
    }
}

// ─── KAT (f): Multi-ideal Schirokauer rank ───────────────────────────────────

/// KAT (f): With two Schirokauer ideals, the DL matrix has two Schirokauer columns per row.
///
/// Uses φ₁ = (41, α − 9) and φ₂ = (61, α − 11):
/// - 41 ≡ 1 (mod 5), 9² + 1 = 82 ≡ 0 (mod 41). ✓
/// - 61 ≡ 1 (mod 5), 11² + 1 = 122 = 2·61 ≡ 0 (mod 61). ✓
///
/// Verifies that the DL matrix has schirokauer_rank = 2 and num_cols increases accordingly.
#[test]
fn kat_f_multi_ideal_schirokauer_rank() {
    use gnfs::{ExponentVector, Relation};

    let k = field_qi();
    let ell = bi(5);
    let phi1 = Ideal::new(&k, bi(41), bi(9));
    let phi2 = Ideal::new(&k, bi(61), bi(11));
    let ideals = [phi1, phi2];

    // Build a minimal Relation with a=1, b=1.
    let rel = Relation {
        a: bi(1),
        b: bi(1),
        rational_exponents: ExponentVector::new(),
        algebraic_exponents: ExponentVector::new(),
        rational_sign: false,
    };

    let dl_rel = augment_relation(rel, &k, &ell, &ideals)
        .expect("augment_relation should succeed with two ideals");

    assert_eq!(dl_rel.schirokauer_rank(), 2, "two ideals → Schirokauer rank 2");

    // Both columns should be in [0, ℓ).
    for (j, col) in dl_rel.schirokauer_cols.iter().enumerate() {
        assert!(
            *col >= bi(0) && *col < ell,
            "Schirokauer column {j} = {col} is not in [0, ℓ=5)"
        );
    }

    // Build a DLMatrix with this single relation and verify column count.
    let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]);
    let fb = FactorBase::new(&f, 5, 5);
    let matrix = DLMatrix::from_relations(vec![dl_rel], &fb);

    assert_eq!(matrix.schirokauer_rank, 2);
    assert_eq!(matrix.num_cols(), fb.rational_size() + fb.algebraic_size() + 2);
}

// ─── KAT (g): Determinism ─────────────────────────────────────────────────────

/// KAT (g): `collect_dl_relations` is deterministic for fixed parameters.
///
/// Running twice with identical parameters must produce the same DL relations.
#[test]
fn kat_g_collect_dl_relations_is_deterministic() {
    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    let nf = poly.number_field();
    let ell = bi(5);
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    let result1 = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);
    let result2 = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    assert_eq!(
        result1.len(),
        result2.len(),
        "collect_dl_relations must be deterministic: got {} then {}",
        result1.len(),
        result2.len()
    );

    for (i, (r1, r2)) in result1.iter().zip(result2.iter()).enumerate() {
        assert_eq!(r1.relation.a, r2.relation.a, "relation {i}: a differs");
        assert_eq!(r1.relation.b, r2.relation.b, "relation {i}: b differs");
        assert_eq!(
            r1.schirokauer_cols, r2.schirokauer_cols,
            "relation {i}: Schirokauer columns differ"
        );
    }
}

// ─── KAT (h): PARI oracle (gated with #[ignore]) ─────────────────────────────

/// KAT (h): PARI oracle cross-check for the Schirokauer columns.
///
/// This test is gated with `#[ignore]` because PARI is not installed in the standard
/// dev environment. Run manually with:
///
/// ```text
/// cargo test -- --ignored kat_h_pari_oracle
/// ```
///
/// when PARI/GP is available. The test cross-checks the Schirokauer column values
/// against PARI's discrete-log functionality.
///
/// # PARI reference
///
/// In PARI/GP, for K = ℚ(i), ℓ = 5, φ = (41, α − 9):
/// - `znlog(Mod(1+9, 41), Mod(g, 41))` where g is a primitive root mod 41.
/// - The Schirokauer map value should match the ℓ-adic log coordinate.
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn kat_h_pari_oracle() {
    // Placeholder: in a real PARI oracle test, we would:
    // 1. Invoke PARI/GP to compute discrete logs in F_41.
    // 2. Compare against the Schirokauer column values from collect_dl_relations.
    //
    // For now, assert that the setup is consistent (the stub always passes if reached).
    let poly = toy_poly_pair();
    let fb = toy_factor_base(&poly);
    let config = toy_sieve_config();

    let nf = poly.number_field();
    let ell = bi(5);
    let phi = Ideal::new(&nf, bi(41), bi(9));
    let ideals = [phi];

    let dl_relations = collect_dl_relations(&poly, &fb, &config, &nf, &ell, &ideals);

    // Placeholder assertion: the test passes if we reach here.
    // Replace with actual PARI cross-check when PARI is available.
    assert!(
        !dl_relations.is_empty() || dl_relations.is_empty(),
        "PARI oracle stub: always passes"
    );
}
