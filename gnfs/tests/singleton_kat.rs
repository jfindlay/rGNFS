//! Known-answer tests (KATs) for the filter substrate: sparse GF(2) matrix and singleton removal.
//!
//! Three KATs:
//!
//! 1. **Matrix construction KAT**: build a small hand-crafted relation set over a toy FactorBase.
//!    Assert column count, row count, column sets, and provenance.
//!
//! 2. **Singleton removal correctness KAT**: build a relation set with a three-level cascade.
//!    Assert that R0, R1, R2 are removed in cascade and R3, R4, R5 survive.
//!
//! 3. **Determinism KAT**: for a fixed relation corpus, call build_matrix + remove_singletons
//!    twice and assert identical results.
//!
//! # Toy polynomial and factor base
//!
//! All KATs use ``f(x) = x³ − x − 1`` (coefficients: [−1, −1, 0, 1] least-significant first).
//! KAT 1 uses ``B_rat = B_alg = 7``; KATs 2 and 3 use ``B_rat = B_alg = 13``.
//!
//! # Column layout for B_rat = B_alg = 7
//!
//! Rational primes ≤ 7: [2, 3, 5, 7] → rat_size = 4.
//! Algebraic ideals for f mod p ≤ 7:
//! - p=5: root r=2 (f(2) = 5 ≡ 0 mod 5) → index 0.
//! - p=7: root r=5 (f(5) = 119 ≡ 0 mod 7) → index 1.
//! alg_size = 2; obstruction_count = 1; matrix_width = 7.
//!
//! Column layout:
//! - Col 0: rational prime 2.
//! - Col 1: rational prime 3.
//! - Col 2: rational prime 5.
//! - Col 3: rational prime 7.
//! - Col 4: algebraic ideal (5, 2).
//! - Col 5: algebraic ideal (7, 5).
//! - Col 6: sign bit (obstruction column 0).
//!
//! # Column layout for B_rat = B_alg = 13
//!
//! Rational primes ≤ 13: [2, 3, 5, 7, 11, 13] → rat_size = 6.
//! Algebraic ideals for f mod p ≤ 13:
//! - p=5: root r=2 → index 0.
//! - p=7: root r=5 → index 1.
//! - p=11: root r=6 (f(6) = 209 = 19×11 ≡ 0 mod 11) → index 2.
//! - p=13: no roots.
//! alg_size = 3; obstruction_count = 1; matrix_width = 10.
//!
//! Column layout:
//! - Col 0: rational prime 2.
//! - Col 1: rational prime 3.
//! - Col 2: rational prime 5.
//! - Col 3: rational prime 7.
//! - Col 4: rational prime 11.
//! - Col 5: rational prime 13.
//! - Col 6: algebraic ideal (5, 2).
//! - Col 7: algebraic ideal (7, 5).
//! - Col 8: algebraic ideal (11, 6).
//! - Col 9: sign bit (obstruction column 0).

use gnfs::{
    build_matrix, remove_singletons, ExponentVector, FactorBase, Relation,
};
use num_bigint::BigInt;
use shared_numfield::IntPoly;

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// f(x) = x³ − x − 1.
fn f_cubic() -> IntPoly {
    IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
}

/// Construct a Relation directly from field values (bypasses smoothness checks).
///
/// Used in KATs where we want to control the GF(2) column pattern exactly, without
/// needing actual smooth norms. The ``a`` and ``b`` values are arbitrary coprime integers
/// that satisfy the structural requirements of the type.
fn make_relation(
    a: i64,
    b: i64,
    rational_entries: Vec<(usize, u32)>,
    algebraic_entries: Vec<(usize, u32)>,
    rational_sign: bool,
) -> Relation {
    Relation {
        a: bi(a),
        b: bi(b),
        rational_exponents: ExponentVector { entries: rational_entries },
        algebraic_exponents: ExponentVector { entries: algebraic_entries },
        rational_sign,
    }
}

// ─── KAT 1: Matrix construction ───────────────────────────────────────────────

/// KAT 1: Build a small hand-crafted relation set over a toy FactorBase.
///
/// Factor base: f(x) = x³ − x − 1, B_rat = B_alg = 7.
/// matrix_width = 7 (4 rational + 2 algebraic + 1 obstruction).
///
/// One relation is constructed with:
/// - rational_exponents: {prime 3 (index 1) → exp 1, prime 7 (index 3) → exp 1}
/// - algebraic_exponents: {ideal (5,2) (index 0) → exp 1}
/// - rational_sign: true
///
/// Expected GF(2) columns:
/// - rational_row_gf2 returns [sign=true, p2=0, p3=1, p5=0, p7=1].
///   - Local index 0 (sign=true) → global col 6 (obstruction_col_start = 6).
///   - Local index 2 (p3, exp 1 mod 2 = 1) → global col 1.
///   - Local index 4 (p7, exp 1 mod 2 = 1) → global col 3.
/// - algebraic_row_gf2 returns [ideal(5,2)=1, ideal(7,5)=0, obstruction=0].
///   - Local index 0 (ideal(5,2), exp 1 mod 2 = 1) → global col 4.
///
/// Expected cols in MatrixRow: [1, 3, 4, 6] (sorted ascending).
/// Provenance: [0] (the only relation, at index 0).
#[test]
fn kat1_matrix_construction() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 7, 7);

    // Verify the factor base structure matches our column-layout assumptions.
    assert_eq!(fb.rational_size(), 4, "B_rat=7 should give 4 rational primes: [2,3,5,7]");
    assert_eq!(fb.algebraic_size(), 2, "B_alg=7 should give 2 algebraic ideals: (5,2) and (7,5)");
    assert_eq!(fb.obstruction_count, 1, "obstruction_count should be 1");
    assert_eq!(fb.matrix_width(), 7, "matrix_width should be 4 + 2 + 1 = 7");

    // Verify column indices for the primes we use.
    let idx_p3 = fb.rational_index(3).expect("prime 3 should be in rational base");
    let idx_p7 = fb.rational_index(7).expect("prime 7 should be in rational base");
    let idx_alg_5_2 = fb.algebraic_index(5, 2).expect("ideal (5,2) should be in algebraic base");
    let _idx_alg_7_5 = fb.algebraic_index(7, 5).expect("ideal (7,5) should be in algebraic base");

    // Construct the relation.
    let relation = make_relation(
        1, 1,
        vec![(idx_p3, 1), (idx_p7, 1)],   // rational: 3^1 * 7^1
        vec![(idx_alg_5_2, 1)],            // algebraic: ideal(5,2)^1
        true,                              // rational_sign: negative
    );

    // Build the matrix.
    let matrix = build_matrix(&[relation.clone()], &fb);

    // Assert: exactly fb.matrix_width() columns.
    assert_eq!(matrix.num_cols, fb.matrix_width(), "num_cols should equal fb.matrix_width()");

    // Assert: one row per relation.
    assert_eq!(matrix.rows.len(), 1, "one row per relation");

    // Assert: obstruction_col_start is correct.
    assert_eq!(
        matrix.obstruction_col_start,
        fb.rational_size() + fb.algebraic_size(),
        "obstruction_col_start should be rational_size + algebraic_size"
    );

    // Cross-check the row's column set against rational_row_gf2 / algebraic_row_gf2.
    let rat_row = relation.rational_row_gf2(&fb);
    let alg_row = relation.algebraic_row_gf2(&fb);

    // Reconstruct expected cols from the GF(2) rows.
    let rat_size = fb.rational_size();
    let alg_size = fb.algebraic_size();
    let obstruction_col_start = rat_size + alg_size;

    let mut expected_cols: Vec<usize> = Vec::new();
    // Rational columns: local indices 1..=rat_size → global 0..rat_size.
    for k in 0..rat_size {
        if rat_row[1 + k] {
            expected_cols.push(k);
        }
    }
    // Algebraic columns: local indices 0..alg_size → global rat_size..obstruction_col_start.
    for k in 0..alg_size {
        if alg_row[k] {
            expected_cols.push(rat_size + k);
        }
    }
    // Sign bit: local rat_row[0] → global obstruction_col_start.
    if rat_row[0] {
        expected_cols.push(obstruction_col_start);
    }
    // expected_cols is already sorted (pushed in ascending order).

    assert_eq!(
        matrix.rows[0].cols, expected_cols,
        "row cols should match GF(2) reconstruction from rational_row_gf2 / algebraic_row_gf2"
    );

    // Spot-check: cols should contain col 1 (p3), col 3 (p7), col 4 (ideal(5,2)), col 6 (sign).
    assert!(matrix.rows[0].cols.contains(&1), "col 1 (prime 3) should be set");
    assert!(matrix.rows[0].cols.contains(&3), "col 3 (prime 7) should be set");
    assert!(matrix.rows[0].cols.contains(&4), "col 4 (ideal (5,2)) should be set");
    assert!(matrix.rows[0].cols.contains(&6), "col 6 (sign bit) should be set");
    assert!(!matrix.rows[0].cols.contains(&0), "col 0 (prime 2) should not be set");
    assert!(!matrix.rows[0].cols.contains(&2), "col 2 (prime 5) should not be set");
    assert!(!matrix.rows[0].cols.contains(&5), "col 5 (ideal (7,5)) should not be set");

    // Assert: provenance for row 0 is exactly [0].
    assert_eq!(matrix.rows[0].provenance, vec![0usize], "provenance should be [0]");

    // Assert: col_weights are consistent with the row.
    for col in 0..matrix.num_cols {
        let expected_weight = if matrix.rows[0].cols.contains(&col) { 1u32 } else { 0u32 };
        assert_eq!(
            matrix.col_weights[col], expected_weight,
            "col_weights[{col}] should be {expected_weight}"
        );
    }
}

/// KAT 1b: Multiple relations — one row per relation, provenance is [i] for row i.
#[test]
fn kat1b_multiple_relations_provenance() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 7, 7);

    let idx_p2 = fb.rational_index(2).expect("prime 2 in rational base");
    let idx_p3 = fb.rational_index(3).expect("prime 3 in rational base");
    let idx_p5 = fb.rational_index(5).expect("prime 5 in rational base");

    let relations = vec![
        make_relation(1, 1, vec![(idx_p2, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p3, 1)], vec![], false),
        make_relation(5, 1, vec![(idx_p5, 1)], vec![], false),
    ];

    let matrix = build_matrix(&relations, &fb);

    assert_eq!(matrix.rows.len(), 3, "three rows for three relations");
    assert_eq!(matrix.num_cols, fb.matrix_width(), "num_cols = matrix_width");

    for i in 0..3 {
        assert_eq!(
            matrix.rows[i].provenance,
            vec![i],
            "provenance of row {i} should be [{i}]"
        );
    }

    // Each row should have exactly one set column (the rational prime column).
    assert_eq!(matrix.rows[0].cols, vec![idx_p2], "row 0 should have col for prime 2");
    assert_eq!(matrix.rows[1].cols, vec![idx_p3], "row 1 should have col for prime 3");
    assert_eq!(matrix.rows[2].cols, vec![idx_p5], "row 2 should have col for prime 5");
}

// ─── KAT 2: Singleton removal with cascading singletons ───────────────────────

/// KAT 2: Singleton removal correctness with a three-level cascade.
///
/// Factor base: f(x) = x³ − x − 1, B_rat = B_alg = 13.
/// matrix_width = 10 (6 rational + 3 algebraic + 1 obstruction).
///
/// Relation design (rational exponents only; algebraic exponents empty):
///
/// - R0: primes {2, 3} → cols {0, 1}
/// - R1: primes {3, 5} → cols {1, 2}
/// - R2: primes {5, 7} → cols {2, 3}
/// - R3: primes {7, 11} → cols {3, 4}
/// - R4: primes {11, 13} → cols {4, 5}
/// - R5: primes {7, 13} → cols {3, 5}
///
/// Initial column weights (non-obstruction):
/// - Col 0 (p=2): weight 1 → singleton → R0 removed.
/// - Col 1 (p=3): weight 2 (R0, R1) → after R0 removed: weight 1 → R1 removed.
/// - Col 2 (p=5): weight 2 (R1, R2) → after R1 removed: weight 1 → R2 removed.
/// - Col 3 (p=7): weight 3 (R2, R3, R5) → after R2 removed: weight 2 → survives.
/// - Col 4 (p=11): weight 2 (R3, R4) → survives.
/// - Col 5 (p=13): weight 2 (R4, R5) → survives.
///
/// Surviving relations after fixpoint: R3, R4, R5.
#[test]
fn kat2_singleton_removal_cascade() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    // Verify factor base structure.
    assert_eq!(fb.rational_size(), 6, "B_rat=13 should give 6 rational primes: [2,3,5,7,11,13]");
    assert_eq!(fb.rational_primes, vec![2u64, 3, 5, 7, 11, 13]);

    let idx_p2 = fb.rational_index(2).unwrap();   // col 0
    let idx_p3 = fb.rational_index(3).unwrap();   // col 1
    let idx_p5 = fb.rational_index(5).unwrap();   // col 2
    let idx_p7 = fb.rational_index(7).unwrap();   // col 3
    let idx_p11 = fb.rational_index(11).unwrap(); // col 4
    let idx_p13 = fb.rational_index(13).unwrap(); // col 5

    // Construct the six relations.
    let relations = vec![
        // R0: primes {2, 3} — col 0 is a singleton (only R0 has it).
        make_relation(1, 1, vec![(idx_p2, 1), (idx_p3, 1)], vec![], false),
        // R1: primes {3, 5} — col 1 becomes singleton after R0 removed.
        make_relation(3, 1, vec![(idx_p3, 1), (idx_p5, 1)], vec![], false),
        // R2: primes {5, 7} — col 2 becomes singleton after R1 removed.
        make_relation(5, 1, vec![(idx_p5, 1), (idx_p7, 1)], vec![], false),
        // R3: primes {7, 11} — survives (cols 3 and 4 have weight ≥ 2 at fixpoint).
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![], false),
        // R4: primes {11, 13} — survives.
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
        // R5: primes {7, 13} — survives.
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![], false),
    ];

    let matrix = build_matrix(&relations, &fb);

    // Verify initial state: 6 rows, correct col weights.
    assert_eq!(matrix.rows.len(), 6, "initial matrix should have 6 rows");
    assert_eq!(matrix.col_weights[idx_p2], 1, "col 0 (p=2) initial weight = 1 (singleton)");
    assert_eq!(matrix.col_weights[idx_p3], 2, "col 1 (p=3) initial weight = 2");
    assert_eq!(matrix.col_weights[idx_p5], 2, "col 2 (p=5) initial weight = 2");
    assert_eq!(matrix.col_weights[idx_p7], 3, "col 3 (p=7) initial weight = 3");
    assert_eq!(matrix.col_weights[idx_p11], 2, "col 4 (p=11) initial weight = 2");
    assert_eq!(matrix.col_weights[idx_p13], 2, "col 5 (p=13) initial weight = 2");

    // Run singleton removal.
    let reduced = remove_singletons(matrix);

    // Assert: exactly 3 surviving rows (R3, R4, R5).
    assert_eq!(reduced.rows.len(), 3, "after singleton removal: 3 rows should survive (R3, R4, R5)");

    // Assert: fixpoint — no non-obstruction column has weight ≤ 1.
    for col in 0..reduced.obstruction_col_start {
        assert!(
            reduced.col_weights[col] != 1,
            "col {col} has weight 1 at fixpoint — singleton removal should have eliminated it"
        );
    }

    // Assert: provenance of surviving rows is unchanged (singleton removal drops, never merges).
    // The surviving rows correspond to original relations R3, R4, R5 (indices 3, 4, 5).
    // After removal of R0, R1, R2, the surviving rows are at positions 0, 1, 2 in the reduced
    // matrix (ordered removal preserves relative order).
    let surviving_provenances: Vec<Vec<usize>> =
        reduced.rows.iter().map(|r| r.provenance.clone()).collect();

    // Each surviving row should have a singleton provenance (original index, not merged).
    for prov in &surviving_provenances {
        assert_eq!(prov.len(), 1, "each surviving row should have provenance of length 1");
    }

    // The surviving provenance indices should be {3, 4, 5}.
    let mut prov_indices: Vec<usize> =
        surviving_provenances.iter().map(|p| p[0]).collect();
    prov_indices.sort_unstable();
    assert_eq!(
        prov_indices,
        vec![3, 4, 5],
        "surviving rows should have provenance [3], [4], [5] (original R3, R4, R5)"
    );

    // Assert: col_weights for surviving columns are correct.
    // After removal: col 3 (p=7) appears in R3 and R5 → weight 2.
    //                col 4 (p=11) appears in R3 and R4 → weight 2.
    //                col 5 (p=13) appears in R4 and R5 → weight 2.
    assert_eq!(reduced.col_weights[idx_p7], 2, "col 3 (p=7) should have weight 2 after removal");
    assert_eq!(reduced.col_weights[idx_p11], 2, "col 4 (p=11) should have weight 2 after removal");
    assert_eq!(reduced.col_weights[idx_p13], 2, "col 5 (p=13) should have weight 2 after removal");

    // Removed columns should have weight 0.
    assert_eq!(reduced.col_weights[idx_p2], 0, "col 0 (p=2) should have weight 0 after removal");
    assert_eq!(reduced.col_weights[idx_p3], 0, "col 1 (p=3) should have weight 0 after removal");
    assert_eq!(reduced.col_weights[idx_p5], 0, "col 2 (p=5) should have weight 0 after removal");
}

/// KAT 2b: Verify the cascade terminates (fixpoint property).
///
/// After remove_singletons, calling it again should produce an identical result.
#[test]
fn kat2b_fixpoint_is_idempotent() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    let idx_p2 = fb.rational_index(2).unwrap();
    let idx_p3 = fb.rational_index(3).unwrap();
    let idx_p5 = fb.rational_index(5).unwrap();
    let idx_p7 = fb.rational_index(7).unwrap();
    let idx_p11 = fb.rational_index(11).unwrap();
    let idx_p13 = fb.rational_index(13).unwrap();

    let relations = vec![
        make_relation(1, 1, vec![(idx_p2, 1), (idx_p3, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p3, 1), (idx_p5, 1)], vec![], false),
        make_relation(5, 1, vec![(idx_p5, 1), (idx_p7, 1)], vec![], false),
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![], false),
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![], false),
    ];

    let matrix = build_matrix(&relations, &fb);
    let reduced_once = remove_singletons(matrix);
    let reduced_twice = remove_singletons(reduced_once.clone());

    // Applying remove_singletons to an already-reduced matrix should be a no-op.
    assert_eq!(
        reduced_once.rows.len(),
        reduced_twice.rows.len(),
        "remove_singletons should be idempotent: same row count"
    );
    assert_eq!(
        reduced_once.col_weights,
        reduced_twice.col_weights,
        "remove_singletons should be idempotent: same col_weights"
    );

    // Verify no weight-1 non-obstruction columns remain.
    for col in 0..reduced_twice.obstruction_col_start {
        assert_ne!(
            reduced_twice.col_weights[col], 1,
            "col {col} should not have weight 1 at fixpoint"
        );
    }
}

// ─── KAT 3: Determinism ───────────────────────────────────────────────────────

/// KAT 3: For a fixed relation corpus, build_matrix + remove_singletons is deterministic.
///
/// Calls the pipeline twice on the same input and asserts identical results.
#[test]
fn kat3_determinism() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    let idx_p2 = fb.rational_index(2).unwrap();
    let idx_p3 = fb.rational_index(3).unwrap();
    let idx_p5 = fb.rational_index(5).unwrap();
    let idx_p7 = fb.rational_index(7).unwrap();
    let idx_p11 = fb.rational_index(11).unwrap();
    let idx_p13 = fb.rational_index(13).unwrap();
    let idx_alg_5_2 = fb.algebraic_index(5, 2).unwrap();
    let idx_alg_7_5 = fb.algebraic_index(7, 5).unwrap();

    // A mixed corpus: some relations with algebraic exponents, some with sign bits.
    let relations = vec![
        make_relation(1, 1, vec![(idx_p2, 1), (idx_p3, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p3, 1), (idx_p5, 1)], vec![], false),
        make_relation(5, 1, vec![(idx_p5, 1), (idx_p7, 1)], vec![], false),
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![(idx_alg_5_2, 1)], false),
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![(idx_alg_7_5, 1)], true),
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![(idx_alg_5_2, 1)], true),
    ];

    // First run.
    let matrix1 = build_matrix(&relations, &fb);
    let reduced1 = remove_singletons(matrix1);

    // Second run (same input).
    let matrix2 = build_matrix(&relations, &fb);
    let reduced2 = remove_singletons(matrix2);

    // Assert identical results.
    assert_eq!(
        reduced1.rows.len(),
        reduced2.rows.len(),
        "determinism: row count should be identical across runs"
    );
    assert_eq!(
        reduced1.num_cols,
        reduced2.num_cols,
        "determinism: num_cols should be identical"
    );
    assert_eq!(
        reduced1.obstruction_col_start,
        reduced2.obstruction_col_start,
        "determinism: obstruction_col_start should be identical"
    );
    assert_eq!(
        reduced1.col_weights,
        reduced2.col_weights,
        "determinism: col_weights should be identical"
    );

    for i in 0..reduced1.rows.len() {
        assert_eq!(
            reduced1.rows[i].cols,
            reduced2.rows[i].cols,
            "determinism: row {i} cols should be identical"
        );
        assert_eq!(
            reduced1.rows[i].provenance,
            reduced2.rows[i].provenance,
            "determinism: row {i} provenance should be identical"
        );
    }
}

/// KAT 3b: Determinism with an empty relation corpus.
///
/// An empty corpus should produce an empty matrix, and remove_singletons should be a no-op.
#[test]
fn kat3b_determinism_empty_corpus() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 7, 7);

    let matrix1 = build_matrix(&[], &fb);
    let reduced1 = remove_singletons(matrix1);

    let matrix2 = build_matrix(&[], &fb);
    let reduced2 = remove_singletons(matrix2);

    assert_eq!(reduced1.rows.len(), 0, "empty corpus: no rows");
    assert_eq!(reduced2.rows.len(), 0, "empty corpus: no rows (second run)");
    assert_eq!(reduced1.num_cols, fb.matrix_width(), "empty corpus: num_cols = matrix_width");
    assert_eq!(reduced1.col_weights, reduced2.col_weights, "determinism: col_weights identical");
}
