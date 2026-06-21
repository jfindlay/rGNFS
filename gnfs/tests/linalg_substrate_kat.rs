//! Known-answer tests (KATs) for the GF(2) linear-algebra substrate.
//!
//! Four KATs:
//!
//! - **KAT 1 — Operator correctness**: for a small hand-built `SparseMatrix`, `A·V` and
//!   `Aᵀ·V` match the hand-computed GF(2) products for several block vectors `V`.
//!
//! - **KAT 2 — QC column construction**: widening `obstruction_count` to `1 + num_qc`
//!   yields a matrix of width `matrix_width()` with the sign column at
//!   `obstruction_col_start` and `num_qc` QC columns following; each row's QC parity
//!   matches the hand-computed Legendre-symbol parity for a toy relation set.
//!
//! - **KAT 3 — Round-trip with provenance**: a hand-built kernel vector expands through
//!   the filtering provenance map to the expected set of original relation indices (the
//!   linear algebra → square root seam exercised early).
//!
//! - **KAT 4 — Determinism**: the operator products and QC columns are deterministic for
//!   a fixed matrix.
//!
//! # Toy setup
//!
//! All KATs use `f(x) = x³ − x − 1` (coefficients: [−1, −1, 0, 1] least-significant
//! first) with `B_rat = B_alg = 13`.

use gnfs::{
    build_matrix, ExponentVector, FactorBase, Relation,
    linalg::{BlockVec, BLOCK_WIDTH, KernelVector, MatrixOperator, populate_qc_columns,
             select_qc_primes},
};
use gnfs::filter::{MatrixRow, SparseMatrix};
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

/// Build a small hand-crafted 3×4 sparse matrix (no factor base needed).
///
/// ```text
///     col: 0 1 2 3
/// row 0:   1 0 1 0
/// row 1:   0 1 0 1
/// row 2:   1 1 0 0
/// ```
///
/// Provenance: row 0 → [0], row 1 → [1], row 2 → [2].
fn hand_matrix_3x4() -> SparseMatrix {
    SparseMatrix {
        rows: vec![
            MatrixRow { cols: vec![0, 2], provenance: vec![0] },
            MatrixRow { cols: vec![1, 3], provenance: vec![1] },
            MatrixRow { cols: vec![0, 1], provenance: vec![2] },
        ],
        num_cols: 4,
        obstruction_col_start: 4,
        obstruction_count: 0,
        col_weights: vec![2, 2, 1, 1],
    }
}

/// Compute the GF(2) matrix-vector product A·v for a dense matrix `a` (rows × cols)
/// and a single dense vector `v` (length cols). Returns a dense vector of length rows.
fn dense_matvec_gf2(a: &[Vec<bool>], v: &[bool]) -> Vec<bool> {
    a.iter()
        .map(|row| {
            row.iter().zip(v.iter()).filter(|&(&r, &vi)| r && vi).count() % 2 == 1
        })
        .collect()
}

// ─── KAT 1: Operator correctness ─────────────────────────────────────────────

/// KAT 1: `A·V` and `Aᵀ·V` match hand-computed GF(2) products.
///
/// Matrix:
/// ```text
///     col: 0 1 2 3
/// row 0:   1 0 1 0
/// row 1:   0 1 0 1
/// row 2:   1 1 0 0
/// ```
///
/// We test several block vectors and verify each column of the output matches the
/// hand-computed GF(2) matrix-vector product.
#[test]
fn kat_1_operator_correctness() {
    let matrix = hand_matrix_3x4();
    let op = MatrixOperator::new(&matrix);

    assert_eq!(op.num_rows(), 3);
    assert_eq!(op.num_cols(), 4);

    // Dense representation of the matrix for reference computation.
    let a_dense: Vec<Vec<bool>> = vec![
        vec![true, false, true, false],  // row 0
        vec![false, true, false, true],  // row 1
        vec![true, true, false, false],  // row 2
    ];

    // Dense representation of Aᵀ (4×3).
    let at_dense: Vec<Vec<bool>> = vec![
        vec![true, false, true],   // row 0 of Aᵀ = col 0 of A
        vec![false, true, true],   // row 1 of Aᵀ = col 1 of A
        vec![true, false, false],  // row 2 of Aᵀ = col 2 of A
        vec![false, true, false],  // row 3 of Aᵀ = col 3 of A
    ];

    // Test several input vectors for apply (A·V).
    let test_vecs_apply: Vec<Vec<bool>> = vec![
        // Standard basis vectors.
        vec![true, false, false, false],
        vec![false, true, false, false],
        vec![false, false, true, false],
        vec![false, false, false, true],
        // Mixed vectors.
        vec![true, true, false, false],
        vec![true, false, true, false],
        vec![true, true, true, true],
        vec![false, false, false, false],
    ];

    for (vi, test_vec) in test_vecs_apply.iter().enumerate() {
        // Build a BlockVec with this vector in column 0.
        let mut bv = BlockVec::zeros(4);
        for (r, &bit) in test_vec.iter().enumerate() {
            bv.set(r, 0, bit);
        }

        let result = op.apply(&bv);
        let expected = dense_matvec_gf2(&a_dense, test_vec);

        for r in 0..3 {
            assert_eq!(
                result.get(r, 0),
                expected[r],
                "apply: test vector {vi}, row {r}: expected {}, got {}",
                expected[r],
                result.get(r, 0)
            );
        }
    }

    // Test several input vectors for apply_transpose (Aᵀ·V).
    let test_vecs_transpose: Vec<Vec<bool>> = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, false],
        vec![true, true, true],
        vec![false, false, false],
    ];

    for (vi, test_vec) in test_vecs_transpose.iter().enumerate() {
        let mut bv = BlockVec::zeros(3);
        for (r, &bit) in test_vec.iter().enumerate() {
            bv.set(r, 0, bit);
        }

        let result = op.apply_transpose(&bv);
        let expected = dense_matvec_gf2(&at_dense, test_vec);

        for r in 0..4 {
            assert_eq!(
                result.get(r, 0),
                expected[r],
                "apply_transpose: test vector {vi}, row {r}: expected {}, got {}",
                expected[r],
                result.get(r, 0)
            );
        }
    }

    // Test with multiple columns in a single BlockVec.
    // Use all 4 standard basis vectors simultaneously (columns 0..3).
    let mut bv_multi = BlockVec::zeros(4);
    for j in 0..4 {
        bv_multi.set(j, j, true); // column j = e_j
    }
    let result_multi = op.apply(&bv_multi);
    // Column j of result should be A·e_j = column j of A.
    for j in 0..4 {
        for r in 0..3 {
            assert_eq!(
                result_multi.get(r, j),
                a_dense[r][j],
                "apply multi: col {j}, row {r}"
            );
        }
    }

    // Verify Aᵀ·(A·e_j) = Aᵀ·(col j of A).
    for j in 0..4 {
        let mut bv_col = BlockVec::zeros(3);
        for r in 0..3 {
            bv_col.set(r, 0, a_dense[r][j]);
        }
        let result_at = op.apply_transpose(&bv_col);
        let expected_at = dense_matvec_gf2(&at_dense, &a_dense.iter().map(|row| row[j]).collect::<Vec<_>>());
        for r in 0..4 {
            assert_eq!(
                result_at.get(r, 0),
                expected_at[r],
                "apply_transpose of col {j}: row {r}"
            );
        }
    }
}

// ─── KAT 2: QC column construction ───────────────────────────────────────────

/// KAT 2: QC column construction with hand-computed Legendre-symbol parities.
///
/// Setup: `f(x) = x³ − x − 1`, `B_rat = B_alg = 13`.
/// We widen `obstruction_count` to `1 + num_qc` and call `populate_qc_columns`.
/// For each row, we verify the QC parity matches the hand-computed Legendre symbol.
///
/// # Hand computation
///
/// For a relation with `(a, b)`, the algebraic norm is `N_alg(a, b) = b³·f(a/b)`.
/// For `b = 1`: `N_alg(a, 1) = f(a) = a³ − a − 1`.
///
/// We use `select_qc_primes` to find the first QC prime `q > 13` that splits completely
/// in K, then verify the QC parity for each relation by computing the Legendre symbol
/// `(N_alg(a,b) / q)` directly.
#[test]
fn kat_2_qc_column_construction() {
    use gnfs::polyselect::PolyPair;

    let f = f_cubic();
    let mut fb = FactorBase::new(&f, 13, 13);

    // Select 2 QC primes.
    let num_qc = 2usize;
    let qc_primes = select_qc_primes(&f, 13, num_qc);
    assert_eq!(qc_primes.len(), num_qc, "should have {num_qc} QC primes");
    for &q in &qc_primes {
        assert!(q > 13, "QC prime {q} should be > b_alg = 13");
    }

    // Widen obstruction_count to 1 + num_qc.
    fb.obstruction_count = 1 + num_qc;

    // Verify matrix_width() reflects the widened obstruction_count.
    let expected_width = fb.rational_size() + fb.algebraic_size() + 1 + num_qc;
    assert_eq!(fb.matrix_width(), expected_width, "matrix_width should include QC columns");

    // Build a toy PolyPair for norm computation.
    // N = 503 (prime), m = 7: f(7) = 343 - 7 - 1 = 335; 335 mod 503 ≠ 0.
    // Use m = 8: f(8) = 512 - 8 - 1 = 503 ≡ 0 (mod 503). ✓
    let n = bi(503);
    let m = bi(8);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let poly = PolyPair::new(f.clone(), g, m, n);

    // Build relations with known (a, b) pairs.
    // We use b=1 so N_alg(a, 1) = f(a) = a³ - a - 1.
    // Relations: (a=2, b=1), (a=3, b=1), (a=4, b=1).
    // N_alg(2, 1) = 8 - 2 - 1 = 5
    // N_alg(3, 1) = 27 - 3 - 1 = 23
    // N_alg(4, 1) = 64 - 4 - 1 = 59
    let idx_p5 = fb.rational_index(5).unwrap_or(2);
    let idx_p23 = fb.rational_index(23).unwrap_or(0); // 23 > 13, not in base
    let idx_p59 = fb.rational_index(59).unwrap_or(0); // 59 > 13, not in base

    // Use simple relations with rational primes in the factor base.
    // The exact exponent vectors don't matter for QC testing; we just need valid (a, b).
    let relations = vec![
        make_relation(2, 1, vec![(idx_p5, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p5, 1)], vec![], false),
        make_relation(4, 1, vec![(idx_p5, 1)], vec![], false),
    ];
    let _ = (idx_p23, idx_p59); // suppress unused warnings

    // Build the matrix with the widened obstruction_count.
    let mut matrix = build_matrix(&relations, &fb);

    // Verify the matrix width matches fb.matrix_width().
    assert_eq!(matrix.num_cols, fb.matrix_width(), "matrix num_cols should equal fb.matrix_width()");

    // Verify obstruction_col_start is correct.
    let expected_obs_start = fb.rational_size() + fb.algebraic_size();
    assert_eq!(
        matrix.obstruction_col_start, expected_obs_start,
        "obstruction_col_start should be rational_size + algebraic_size"
    );

    // Before populate_qc_columns: QC columns should all be zero.
    for (row_idx, row) in matrix.rows.iter().enumerate() {
        for k in 0..num_qc {
            let qc_col = matrix.obstruction_col_start + 1 + k;
            assert!(
                !row.cols.contains(&qc_col),
                "row {row_idx}: QC column {qc_col} should be zero before populate_qc_columns"
            );
        }
    }

    // Populate QC columns.
    populate_qc_columns(&mut matrix, &relations, &fb, &poly, &qc_primes);

    // Verify QC parities match hand-computed Legendre symbols.
    // For each relation (a, b), compute N_alg(a, b) mod q and check the Legendre symbol.
    let norms = vec![5u64, 23u64, 59u64]; // N_alg(2,1), N_alg(3,1), N_alg(4,1)

    for (row_idx, row) in matrix.rows.iter().enumerate() {
        let norm = norms[row_idx];
        for (k, &q) in qc_primes.iter().enumerate() {
            let qc_col = matrix.obstruction_col_start + 1 + k;
            let norm_mod_q = norm % q;
            // Legendre symbol via Euler's criterion.
            let exp = (q - 1) / 2;
            let symbol = pow_mod_test(norm_mod_q, exp, q);
            let expected_parity = symbol == q - 1;
            let actual_parity = row.cols.contains(&qc_col);
            assert_eq!(
                actual_parity, expected_parity,
                "row {row_idx}, QC prime {q}: expected parity {expected_parity}, got {actual_parity} \
                 (norm={norm}, norm mod q={norm_mod_q}, symbol={symbol})"
            );
        }
    }

    // Verify col_weights are consistent with row contents.
    for col in 0..matrix.num_cols {
        let actual_weight =
            matrix.rows.iter().filter(|r| r.cols.binary_search(&col).is_ok()).count();
        assert_eq!(
            matrix.col_weights[col] as usize,
            actual_weight,
            "col_weights[{col}] should be consistent with row contents after populate_qc_columns"
        );
    }
}

// ─── KAT 3: Round-trip with provenance ───────────────────────────────────────

/// KAT 3: A hand-built kernel vector expands through the provenance map to the expected
/// set of original relation indices (the linear algebra → square root seam).
///
/// Setup: a matrix with merged rows (provenance sets of size > 1), and a kernel vector
/// that selects a subset of rows. We verify that `expand_provenance` returns the correct
/// symmetric difference of the provenance sets.
///
/// # Provenance structure
///
/// - Row 0: provenance = [0, 1] (merged from original relations 0 and 1)
/// - Row 1: provenance = [2]    (original relation 2)
/// - Row 2: provenance = [1, 3] (merged from original relations 1 and 3)
///
/// Kernel vector: rows {0, 2}.
/// Expected expansion: sym_diff([0,1], [1,3]) = [0, 3] (1 cancels).
///
/// Kernel vector: rows {0, 1, 2}.
/// Expected expansion: sym_diff([0,1], [2], [1,3]) = [0, 2, 3] (1 cancels from 0 and 2).
#[test]
fn kat_3_round_trip_provenance() {
    // Build a matrix with hand-crafted provenance.
    // The column sets are chosen so that rows {0, 1, 2} form a valid nullspace vector:
    // row 0 cols = {0, 1}, row 1 cols = {1, 2}, row 2 cols = {0, 2}.
    // XOR: {0,1} XOR {1,2} XOR {0,2} = {} ✓
    let matrix = SparseMatrix {
        rows: vec![
            MatrixRow { cols: vec![0, 1], provenance: vec![0, 1] },
            MatrixRow { cols: vec![1, 2], provenance: vec![2] },
            MatrixRow { cols: vec![0, 2], provenance: vec![1, 3] },
        ],
        num_cols: 3,
        obstruction_col_start: 3,
        obstruction_count: 0,
        col_weights: vec![2, 2, 2],
    };

    // KAT 3a: kernel vector {0, 2}.
    let kv_02 = KernelVector::new(vec![0, 2]);
    // Not a nullspace vector (rows 0 and 2 don't cancel), but we test provenance expansion.
    let expanded_02 = kv_02.expand_provenance(&matrix);
    // sym_diff([0,1], [1,3]) = [0, 3].
    assert_eq!(expanded_02, vec![0, 3], "KAT 3a: expansion of rows {{0,2}} should be [0, 3]");

    // KAT 3b: kernel vector {0, 1, 2} — a valid nullspace vector.
    let kv_012 = KernelVector::new(vec![0, 1, 2]);
    assert!(kv_012.verify(&matrix), "KAT 3b: rows {{0,1,2}} should form a valid nullspace vector");
    let expanded_012 = kv_012.expand_provenance(&matrix);
    // sym_diff([0,1], [2], [1,3]):
    // Step 1: sym_diff([0,1], [2]) = [0, 1, 2].
    // Step 2: sym_diff([0,1,2], [1,3]) = [0, 2, 3] (1 cancels).
    assert_eq!(
        expanded_012, vec![0, 2, 3],
        "KAT 3b: expansion of rows {{0,1,2}} should be [0, 2, 3]"
    );

    // KAT 3c: single-row kernel vector {1}.
    let kv_1 = KernelVector::new(vec![1]);
    let expanded_1 = kv_1.expand_provenance(&matrix);
    assert_eq!(expanded_1, vec![2], "KAT 3c: expansion of row {{1}} should be [2]");

    // KAT 3d: empty kernel vector.
    let kv_empty = KernelVector::new(vec![]);
    let expanded_empty = kv_empty.expand_provenance(&matrix);
    assert_eq!(expanded_empty, vec![], "KAT 3d: expansion of empty kernel vector should be []");

    // KAT 3e: from_mask constructor.
    let mask = vec![true, true, true];
    let kv_from_mask = KernelVector::from_mask(&mask);
    assert_eq!(kv_from_mask.row_indices, vec![0, 1, 2], "KAT 3e: from_mask should produce [0,1,2]");
    assert!(kv_from_mask.verify(&matrix), "KAT 3e: from_mask kernel vector should be valid");

    // KAT 3f: verify that a non-nullspace vector returns false.
    let kv_invalid = KernelVector::new(vec![0, 1]);
    assert!(!kv_invalid.verify(&matrix), "KAT 3f: rows {{0,1}} should not be a nullspace vector");
}

// ─── KAT 4: Determinism ──────────────────────────────────────────────────────

/// KAT 4: Operator products and QC columns are deterministic for a fixed matrix.
///
/// Calls the operator and QC functions twice on the same input and asserts identical
/// results. This verifies there is no hidden state or randomness.
#[test]
fn kat_4_determinism() {
    use gnfs::polyselect::PolyPair;

    let f = f_cubic();
    let mut fb = FactorBase::new(&f, 13, 13);

    let num_qc = 2usize;
    let qc_primes = select_qc_primes(&f, 13, num_qc);
    fb.obstruction_count = 1 + num_qc;

    let n = bi(503);
    let m = bi(8);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let poly = PolyPair::new(f.clone(), g, m, n);

    let idx_p5 = fb.rational_index(5).unwrap_or(2);
    let relations = vec![
        make_relation(2, 1, vec![(idx_p5, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p5, 1)], vec![], false),
    ];

    // First run: build matrix, populate QC columns, apply operator.
    let mut matrix1 = build_matrix(&relations, &fb);
    populate_qc_columns(&mut matrix1, &relations, &fb, &poly, &qc_primes);
    let op1 = MatrixOperator::new(&matrix1);

    // Build a test block vector.
    let mut bv1 = BlockVec::zeros(op1.num_cols());
    for j in 0..BLOCK_WIDTH.min(op1.num_cols()) {
        bv1.set(j % op1.num_cols(), j % BLOCK_WIDTH, true);
    }
    let result1_apply = op1.apply(&bv1);
    let mut bv1t = BlockVec::zeros(op1.num_rows());
    for j in 0..BLOCK_WIDTH.min(op1.num_rows()) {
        bv1t.set(j % op1.num_rows(), j % BLOCK_WIDTH, true);
    }
    let result1_transpose = op1.apply_transpose(&bv1t);

    // Second run: identical inputs.
    let mut matrix2 = build_matrix(&relations, &fb);
    populate_qc_columns(&mut matrix2, &relations, &fb, &poly, &qc_primes);
    let op2 = MatrixOperator::new(&matrix2);

    let mut bv2 = BlockVec::zeros(op2.num_cols());
    for j in 0..BLOCK_WIDTH.min(op2.num_cols()) {
        bv2.set(j % op2.num_cols(), j % BLOCK_WIDTH, true);
    }
    let result2_apply = op2.apply(&bv2);
    let mut bv2t = BlockVec::zeros(op2.num_rows());
    for j in 0..BLOCK_WIDTH.min(op2.num_rows()) {
        bv2t.set(j % op2.num_rows(), j % BLOCK_WIDTH, true);
    }
    let result2_transpose = op2.apply_transpose(&bv2t);

    // Assert identical results.
    assert_eq!(
        result1_apply, result2_apply,
        "KAT 4: apply results must be identical across runs"
    );
    assert_eq!(
        result1_transpose, result2_transpose,
        "KAT 4: apply_transpose results must be identical across runs"
    );

    // Assert identical matrix contents (QC columns are deterministic).
    assert_eq!(matrix1.rows.len(), matrix2.rows.len(), "KAT 4: row counts must match");
    for i in 0..matrix1.rows.len() {
        assert_eq!(
            matrix1.rows[i].cols, matrix2.rows[i].cols,
            "KAT 4: row {i} cols must be identical across runs"
        );
    }
    assert_eq!(
        matrix1.col_weights, matrix2.col_weights,
        "KAT 4: col_weights must be identical across runs"
    );

    // Assert QC primes are deterministic.
    let qc_primes2 = select_qc_primes(&f, 13, num_qc);
    assert_eq!(qc_primes, qc_primes2, "KAT 4: select_qc_primes must be deterministic");
}

// ─── Helper: modular exponentiation (for KAT 2 hand computation) ─────────────

/// Compute `base^exp mod q` using fast exponentiation.
fn pow_mod_test(mut base: u64, mut exp: u64, q: u64) -> u64 {
    if q == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= q;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % q as u128) as u64;
        }
        base = ((base as u128 * base as u128) % q as u128) as u64;
        exp >>= 1;
    }
    result
}
