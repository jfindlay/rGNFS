//! Known-answer tests (KATs) for G.D.2: clique/excess pruning and column merging.
//!
//! Four KATs are required by the G.D.2 session spec:
//!
//! - **KAT (a) — 2-way merge correctness**: a weight-2 column is eliminated; the merged
//!   row has the correct cols (symmetric difference) and provenance (union).
//!
//! - **KAT (b) — Determinism**: the full pipeline (build_matrix + remove_singletons +
//!   prune_cliques + merge_columns) is deterministic for a fixed corpus.
//!
//! - **KAT (c) — CADO-NFS oracle**: gated/ignored when CADO is absent.
//!
//! - **KAT (d) — End-to-end provenance**: for each row in the final matrix, the XOR of
//!   the original relations indexed by its provenance equals that row's column set.
//!
//! # Toy setup
//!
//! All KATs use ``f(x) = x³ − x − 1`` (coefficients: [−1, −1, 0, 1] least-significant
//! first) with ``B_rat = B_alg = 13``.
//!
//! Column layout for B_rat = B_alg = 13:
//! - Rational primes ≤ 13: [2, 3, 5, 7, 11, 13] → rat_size = 6.
//! - Algebraic ideals: (5,2) → index 0, (7,5) → index 1, (11,6) → index 2.
//! - alg_size = 3; obstruction_count = 1; matrix_width = 10.
//! - Col 0: p=2, Col 1: p=3, Col 2: p=5, Col 3: p=7, Col 4: p=11, Col 5: p=13.
//! - Col 6: ideal(5,2), Col 7: ideal(7,5), Col 8: ideal(11,6).
//! - Col 9: sign bit (obstruction column 0).

use gnfs::{
    build_matrix, merge_columns, prune_cliques, remove_singletons, ExponentVector, FactorBase,
    Relation,
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

// ─── KAT (a): 2-way merge correctness ────────────────────────────────────────

/// KAT (a): A weight-2 column is eliminated by merging its two containing rows.
///
/// Setup: build a matrix where column c (p=7, col 3) has weight exactly 2, but the
/// other columns have weight > 2 so that only col 3 triggers a merge in the weight-2
/// pass. We use a fourth relation to give cols 4 and 5 weight 3, preventing them from
/// being merged in the weight-2 pass.
///
/// Relations:
/// - R0: primes {7, 11} → cols {3, 4}
/// - R1: primes {7, 13} → cols {3, 5}
/// - R2: primes {11, 13} → cols {4, 5}
/// - R3: primes {11, 13} → cols {4, 5}  (duplicate to raise col 4 and 5 to weight 3)
///
/// col_weights after build:
///   col 3 (p=7): 2 (R0, R1) ← weight-2 candidate
///   col 4 (p=11): 3 (R0, R2, R3)
///   col 5 (p=13): 3 (R1, R2, R3)
///
/// After merge_columns (weight-2 pass, col 3 processed):
/// - R0 and R1 merged: new_row.cols = {4, 5} (col 3 cancels).
/// - new_row.provenance = {0, 1}.
/// - col_weights[3] = 0.
/// - Row count: 4 → 3 (two rows merged into one, net -1).
///
/// After weight-3 pass: col 4 has weight 3 (R2, R3, new_row) → merge.
/// merged2 = R2 ⊕ R3 ⊕ new_row = {} ⊕ new_row = new_row (since R2=R3, they cancel).
/// Actually R2.cols = R3.cols = {4,5}, so R2⊕R3 = {}, then {}⊕new_row = new_row = {4,5}.
/// merged2.cols = {4, 5}, provenance = {0, 1, 2, 3}.
/// Row count: 3 → 1.
///
/// We verify the intermediate state after the weight-2 pass by using a corpus where
/// col 4 and col 5 have weight exactly 3 (not 2), so the weight-2 pass only merges
/// col 3, and the weight-3 pass handles cols 4 and 5.
///
/// The key assertions for KAT (a) are:
/// - After the full merge_columns: col 3 has weight 0 (eliminated by weight-2 pass).
/// - The intermediate merged row (from the weight-2 merge of R0 and R1) has the correct
///   cols ({4,5}) and provenance ({0,1}) — verified by checking the final provenance.
/// - col_weights are consistent throughout.
///
/// To isolate the 2-way merge, we use a simpler corpus where only col 3 is weight-2
/// and the other columns have weight 1 (so they are singletons and get removed first,
/// leaving only the two rows that share col 3). But that would remove everything.
///
/// Instead, we use a direct unit test of xor_merge to verify the 2-way merge primitive,
/// and then test merge_columns on a corpus where col 3 is the only weight-2 column and
/// the other columns have weight > 2.
///
/// Simplest setup: 4 rows where col 3 has weight 2 and all other active columns have
/// weight ≥ 3. We add a "padding" column (col 4) with weight 3 by including it in
/// three rows.
///
/// R0: cols {3, 4}  (p=7, p=11)
/// R1: cols {3, 4}  (p=7, p=11)  — col 3 weight 2, col 4 weight 2 → both weight-2
///
/// That still gives col 4 weight 2. We need col 4 weight ≥ 3.
///
/// Simplest: use 5 rows.
/// R0: cols {3, 4}
/// R1: cols {3, 5}
/// R2: cols {4, 5}
/// R3: cols {4, 6}  (col 6 = ideal(5,2))
/// R4: cols {5, 6}
///
/// col_weights: col 3=2, col 4=3, col 5=3, col 6=2.
/// Weight-2 pass: col 3 (first by index) → merge R0+R1 → new_row={4,5}, prov={0,1}.
///               col 6 → merge R3+R4 → new_row2={4,5}, prov={3,4}.
/// After weight-2 pass: rows = [R2, new_row, new_row2], col 4 weight=3, col 5 weight=3.
/// Weight-3 pass: col 4 weight=3 → merge R2+new_row+new_row2 → {}, prov={0,1,2,3,4}.
///               col 5 weight=0 now (cancelled in the 3-way merge).
///
/// This is getting complex. Let's use the simplest possible setup that isolates the
/// 2-way merge: a matrix where ONLY col 3 is weight-2 and all other columns are weight ≥ 3,
/// and verify the intermediate state by checking col_weights after merge.
///
/// Simplest: 5 rows, col 3 weight 2, col 4 weight 3, col 5 weight 3, col 6 weight 3.
/// R0: {3, 4}
/// R1: {3, 5}
/// R2: {4, 5, 6}
/// R3: {4, 6}  — wait, this gives col 4 weight 3, col 6 weight 2.
///
/// Let's just use: R0={3,4}, R1={3,5}, R2={4,5}, R3={4,5}, R4={4,5}.
/// col 3=2, col 4=4, col 5=4. Weight-2 pass: col 3 → merge R0+R1 → {4,5}, prov={0,1}.
/// col 4 weight=4 (not 2), col 5 weight=4 (not 2). Weight-3 pass: no weight-3 cols.
/// Final: 4 rows (R2, R3, R4, merged). Row count 5→4 (net -1).
///
/// This cleanly tests the 2-way merge in isolation.
#[test]
fn kat_a_two_way_merge_correctness() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    let idx_p7 = fb.rational_index(7).expect("prime 7 in rational base");   // col 3
    let idx_p11 = fb.rational_index(11).expect("prime 11 in rational base"); // col 4
    let idx_p13 = fb.rational_index(13).expect("prime 13 in rational base"); // col 5

    // Verify column indices match our assumptions.
    assert_eq!(idx_p7, 3, "p=7 should be at col 3");
    assert_eq!(idx_p11, 4, "p=11 should be at col 4");
    assert_eq!(idx_p13, 5, "p=13 should be at col 5");

    // R0: {3, 4}, R1: {3, 5} — col 3 weight 2 (only these two rows).
    // R2, R3, R4: {4, 5} — col 4 weight 4, col 5 weight 4 (not weight-2 or weight-3).
    let relations = vec![
        // R0: primes {7, 11} → cols {3, 4}
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![], false),
        // R1: primes {7, 13} → cols {3, 5}
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![], false),
        // R2, R3, R4: primes {11, 13} → cols {4, 5} (three copies to give weight 4 after merge)
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
        make_relation(17, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
        make_relation(19, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
    ];

    let matrix = build_matrix(&relations, &fb);

    // Verify initial col_weights.
    assert_eq!(matrix.rows.len(), 5, "initial: 5 rows");
    assert_eq!(matrix.col_weights[idx_p7], 2, "col 3 (p=7) initial weight = 2");
    assert_eq!(matrix.col_weights[idx_p11], 4, "col 4 (p=11) initial weight = 4");
    assert_eq!(matrix.col_weights[idx_p13], 4, "col 5 (p=13) initial weight = 4");

    // remove_singletons is a no-op here (no weight-1 columns).
    let after_singletons = remove_singletons(matrix);
    assert_eq!(after_singletons.rows.len(), 5, "after singleton removal: still 5 rows");

    // prune_cliques: excess = 5 - (10 - 1) = -4 < EXCESS_FLOOR (20) → no pruning.
    let after_prune = prune_cliques(after_singletons);
    assert_eq!(after_prune.rows.len(), 5, "prune_cliques: no rows removed (excess < EXCESS_FLOOR)");

    // merge_columns: weight-2 pass processes col 3 (only weight-2 column).
    // R0 and R1 are merged: new_row.cols = {4, 5}, new_row.provenance = {0, 1}.
    // col 4 and col 5 have weight 4 (not 2 or 3) → not merged.
    // Weight-3 pass: no weight-3 columns → no-op.
    // Final: 4 rows (R2, R3, R4, merged_row). Row count 5→4.
    let after_merge = merge_columns(after_prune);

    // Row count decreased by 1 (two rows merged into one, net -1).
    assert_eq!(
        after_merge.rows.len(),
        4,
        "merge should reduce row count by 1: 5 → 4 (R0+R1 merged, R2/R3/R4 unchanged)"
    );

    // Column 3 (p=7) should now have weight 0 (eliminated by the 2-way merge).
    assert_eq!(
        after_merge.col_weights[idx_p7], 0,
        "col 3 (p=7) should have weight 0 after merge (eliminated)"
    );

    // Find the merged row: cols = {4, 5} (symmetric difference of {3,4} and {3,5}),
    // provenance = {0, 1} (union of R0=[0] and R1=[1]).
    let merged_row = after_merge.rows.iter().find(|r| r.provenance == vec![0usize, 1usize]);
    assert!(merged_row.is_some(), "merged row with provenance [0, 1] should exist");

    let merged_row = merged_row.unwrap();

    // Merged row cols = symmetric difference of {3,4} and {3,5} = {4,5}.
    assert_eq!(
        merged_row.cols,
        vec![idx_p11, idx_p13],
        "merged row cols should be {{4, 5}} (col 3 cancelled by XOR)"
    );
    assert!(!merged_row.cols.contains(&idx_p7), "merged row should not contain col 3 (p=7)");

    // Merged row provenance = union of [0] and [1] = [0, 1].
    assert_eq!(
        merged_row.provenance,
        vec![0usize, 1usize],
        "merged row provenance should be [0, 1] (union of R0 and R1)"
    );

    // Verify col_weights consistency: col_weights[c] should equal the number of rows containing c.
    for col in 0..after_merge.num_cols {
        let actual_weight =
            after_merge.rows.iter().filter(|r| r.cols.binary_search(&col).is_ok()).count();
        assert_eq!(
            after_merge.col_weights[col] as usize,
            actual_weight,
            "col_weights[{col}] should be consistent with actual row contents"
        );
    }
}

// ─── KAT (b): Determinism ────────────────────────────────────────────────────

/// KAT (b): The full pipeline is deterministic for a fixed corpus.
///
/// Calls build_matrix + remove_singletons + prune_cliques + merge_columns twice on the
/// same input and asserts identical matrix dimensions and col_weights.
#[test]
fn kat_b_determinism() {
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

    // A corpus with enough relations to survive singleton removal and have mergeable columns.
    let relations = vec![
        make_relation(1, 1, vec![(idx_p2, 1), (idx_p3, 1)], vec![], false),
        make_relation(3, 1, vec![(idx_p3, 1), (idx_p5, 1)], vec![], false),
        make_relation(5, 1, vec![(idx_p5, 1), (idx_p7, 1)], vec![], false),
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![(idx_alg_5_2, 1)], false),
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![(idx_alg_7_5, 1)], true),
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![(idx_alg_5_2, 1)], true),
        make_relation(17, 1, vec![(idx_p2, 1), (idx_p7, 1)], vec![], false),
        make_relation(19, 1, vec![(idx_p3, 1), (idx_p11, 1)], vec![], false),
    ];

    // First run.
    let m1 = build_matrix(&relations, &fb);
    let m1 = remove_singletons(m1);
    let m1 = prune_cliques(m1);
    let m1 = merge_columns(m1);

    // Second run (same input).
    let m2 = build_matrix(&relations, &fb);
    let m2 = remove_singletons(m2);
    let m2 = prune_cliques(m2);
    let m2 = merge_columns(m2);

    // Assert identical dimensions.
    assert_eq!(
        m1.rows.len(), m2.rows.len(),
        "determinism: row count must be identical across runs"
    );
    assert_eq!(
        m1.num_cols, m2.num_cols,
        "determinism: num_cols must be identical"
    );

    // Assert identical col_weights.
    assert_eq!(
        m1.col_weights, m2.col_weights,
        "determinism: col_weights must be identical across runs"
    );

    // Assert identical row contents (cols and provenance).
    for i in 0..m1.rows.len() {
        assert_eq!(
            m1.rows[i].cols, m2.rows[i].cols,
            "determinism: row {i} cols must be identical"
        );
        assert_eq!(
            m1.rows[i].provenance, m2.rows[i].provenance,
            "determinism: row {i} provenance must be identical"
        );
    }
}

// ─── KAT (c): CADO-NFS oracle (gated) ────────────────────────────────────────

/// KAT (c): CADO-NFS oracle — gated when CADO is absent.
///
/// When CADO-NFS is available, this test would:
/// 1. Run the full GNFS filtering pipeline at matched parameters.
/// 2. Compare the filtered matrix dimensions (row count, column count, total weight)
///    against CADO's output within a tolerance (e.g., ±10% on row count).
///
/// This test is ignored when CADO is not installed. To run manually:
///   cargo test kat_c_cado_nfs_oracle -- --ignored
#[test]
#[ignore = "CADO-NFS not installed; run manually when available"]
fn kat_c_cado_nfs_oracle() {
    // Placeholder: when CADO-NFS is available, compare filtered matrix dimensions
    // against CADO's output at matched parameters (same polynomial, same factor base
    // bounds, same relation corpus). The filtered matrix row count, column count, and
    // total Hamming weight should be within ±10% of CADO's output.
    //
    // Steps (when implementing):
    // 1. Generate a relation corpus using the same polynomial and bounds as a CADO run.
    // 2. Run build_matrix + remove_singletons + prune_cliques + merge_columns.
    // 3. Parse CADO's filtered matrix output (e.g., from .mat file).
    // 4. Assert dimensions are within tolerance.
    unimplemented!("CADO-NFS oracle not yet implemented — requires CADO installation");
}

// ─── KAT (d): End-to-end provenance ──────────────────────────────────────────

/// KAT (d): End-to-end provenance correctness.
///
/// For each row in the final matrix, collect its provenance indices, look up the
/// original relations, reconstruct their GF(2) column sets, XOR them together, and
/// verify the result equals the final row's cols.
///
/// This confirms that the provenance map is correct end-to-end: expanding a row through
/// its provenance recovers the original relations whose GF(2) parities XOR to that row.
///
/// Setup: a corpus where singleton removal and merging both occur, so provenance sets
/// are non-trivial (some rows have provenance of length > 1 after merging).
#[test]
fn kat_d_end_to_end_provenance() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    let idx_p7 = fb.rational_index(7).unwrap();   // col 3
    let idx_p11 = fb.rational_index(11).unwrap(); // col 4
    let idx_p13 = fb.rational_index(13).unwrap(); // col 5
    let idx_alg_5_2 = fb.algebraic_index(5, 2).unwrap(); // col 6
    let idx_alg_7_5 = fb.algebraic_index(7, 5).unwrap(); // col 7

    // Corpus: 5 relations, no singletons, with weight-2 columns for merging.
    // R0: cols {3, 4}         (p=7, p=11)
    // R1: cols {3, 5}         (p=7, p=13)
    // R2: cols {4, 5}         (p=11, p=13)
    // R3: cols {3, 6}         (p=7, ideal(5,2))
    // R4: cols {5, 6, 7}      (p=13, ideal(5,2), ideal(7,5))
    //
    // col_weights after build:
    //   col 3 (p=7): 3 (R0, R1, R3)
    //   col 4 (p=11): 2 (R0, R2)
    //   col 5 (p=13): 3 (R1, R2, R4)
    //   col 6 (ideal(5,2)): 2 (R3, R4)
    //   col 7 (ideal(7,5)): 1 (R4) ← singleton!
    //
    // After remove_singletons: R4 removed (col 7 is singleton → R4 removed).
    //   After R4 removed: col 5 weight = 2 (R1, R2), col 6 weight = 1 (R3) → R3 removed.
    //   After R3 removed: col 3 weight = 2 (R0, R1). Fixpoint.
    // Surviving: R0, R1, R2.
    //
    // After prune_cliques: excess = 3 - (10 - 1) = -6 < EXCESS_FLOOR → no pruning.
    //
    // After merge_columns: col 3 (weight 2, R0 and R1) → merge.
    //   merged: cols = {4, 5}, provenance = {0, 1}.
    //   col 4 (weight 2, R2 and merged) → merge.
    //   merged2: cols = {5}, provenance = {0, 1, 2}.
    //   col 5 (weight 1 now) → singleton in col_weights but merge_columns doesn't enforce floor.
    //   Actually after the weight-2 pass, col 5 has weight 1 in merged2 only.
    //   The weight-3 pass: no weight-3 columns remain.
    // Final matrix: 1 row with cols {5}, provenance {0, 1, 2}.

    let relations = vec![
        // R0: primes {7, 11} → cols {3, 4}
        make_relation(7, 1, vec![(idx_p7, 1), (idx_p11, 1)], vec![], false),
        // R1: primes {7, 13} → cols {3, 5}
        make_relation(13, 1, vec![(idx_p7, 1), (idx_p13, 1)], vec![], false),
        // R2: primes {11, 13} → cols {4, 5}
        make_relation(11, 1, vec![(idx_p11, 1), (idx_p13, 1)], vec![], false),
        // R3: primes {7}, ideal(5,2) → cols {3, 6}
        make_relation(5, 1, vec![(idx_p7, 1)], vec![(idx_alg_5_2, 1)], false),
        // R4: primes {13}, ideal(5,2), ideal(7,5) → cols {5, 6, 7}
        make_relation(3, 1, vec![(idx_p13, 1)], vec![(idx_alg_5_2, 1), (idx_alg_7_5, 1)], false),
    ];

    // Build the initial matrix (keep a clone for provenance verification).
    let initial_matrix = build_matrix(&relations, &fb);

    // Record the GF(2) column set for each original relation (from the initial matrix rows).
    // initial_matrix.rows[i].cols is the GF(2) column set for relation i.
    let original_cols: Vec<Vec<usize>> =
        initial_matrix.rows.iter().map(|r| r.cols.clone()).collect();

    // Run the full pipeline.
    let after_singletons = remove_singletons(initial_matrix);
    let after_prune = prune_cliques(after_singletons);
    let final_matrix = merge_columns(after_prune);

    // For each row in the final matrix, verify provenance.
    for (row_idx, row) in final_matrix.rows.iter().enumerate() {
        // XOR the GF(2) column sets of all original relations in the provenance.
        let mut expected_cols: Vec<usize> = Vec::new();
        for &orig_idx in &row.provenance {
            // XOR expected_cols with original_cols[orig_idx] (symmetric difference).
            let orig = &original_cols[orig_idx];
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;
            while i < expected_cols.len() && j < orig.len() {
                match expected_cols[i].cmp(&orig[j]) {
                    std::cmp::Ordering::Less => {
                        merged.push(expected_cols[i]);
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        merged.push(orig[j]);
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        // XOR cancels.
                        i += 1;
                        j += 1;
                    }
                }
            }
            merged.extend_from_slice(&expected_cols[i..]);
            merged.extend_from_slice(&orig[j..]);
            expected_cols = merged;
        }

        assert_eq!(
            row.cols, expected_cols,
            "row {row_idx}: XOR of provenance relations should equal the row's cols"
        );
    }

    // Verify col_weights consistency in the final matrix.
    for col in 0..final_matrix.num_cols {
        let actual_weight =
            final_matrix.rows.iter().filter(|r| r.cols.binary_search(&col).is_ok()).count();
        assert_eq!(
            final_matrix.col_weights[col] as usize,
            actual_weight,
            "final matrix col_weights[{col}] should be consistent with row contents"
        );
    }
}

// ─── Additional: prune_cliques excess floor ───────────────────────────────────

/// Verify that prune_cliques respects EXCESS_FLOOR.
///
/// Build a matrix with excess well above EXCESS_FLOOR, run prune_cliques, and assert
/// that the resulting excess is exactly EXCESS_FLOOR (or as close as possible without
/// going below).
#[test]
fn kat_prune_cliques_respects_excess_floor() {
    use gnfs::EXCESS_FLOOR;

    let f = f_cubic();
    let fb = FactorBase::new(&f, 13, 13);

    // matrix_width = 10, obstruction_count = 1.
    // excess = rows - (10 - 1) = rows - 9.
    // To have excess > EXCESS_FLOOR (20), we need rows > 29.
    // Build 35 rows: each row has a unique pair of columns from {3, 4, 5} plus a
    // unique "heavy" column that appears only in that row (to give varying weights).
    // Actually, let's just build many rows with shared columns.

    let idx_p7 = fb.rational_index(7).unwrap();   // col 3
    let idx_p11 = fb.rational_index(11).unwrap(); // col 4
    let idx_p13 = fb.rational_index(13).unwrap(); // col 5

    // Build 35 relations, all sharing cols {3, 4, 5} (weight 3 each).
    // excess = 35 - 9 = 26 > 20 = EXCESS_FLOOR.
    let mut relations = Vec::new();
    for i in 0..35i64 {
        relations.push(make_relation(
            i + 100,
            1,
            vec![(idx_p7, 1), (idx_p11, 1), (idx_p13, 1)],
            vec![],
            false,
        ));
    }

    let matrix = build_matrix(&relations, &fb);
    assert_eq!(matrix.rows.len(), 35, "initial: 35 rows");
    assert_eq!(matrix.excess(), 35 - 9, "initial excess = 26");

    // remove_singletons: no singletons (all cols have weight 35).
    let after_singletons = remove_singletons(matrix);
    assert_eq!(after_singletons.rows.len(), 35, "no singletons removed");

    let after_prune = prune_cliques(after_singletons);

    // After pruning: excess should be exactly EXCESS_FLOOR (20).
    // rows needed = EXCESS_FLOOR + 9 = 29.
    let expected_rows = EXCESS_FLOOR + (10 - 1); // EXCESS_FLOOR + non-obstruction cols
    assert_eq!(
        after_prune.rows.len(),
        expected_rows,
        "after prune_cliques: rows should be EXCESS_FLOOR + non-obstruction cols = {expected_rows}"
    );
    assert_eq!(
        after_prune.excess(),
        EXCESS_FLOOR as isize,
        "after prune_cliques: excess should be exactly EXCESS_FLOOR"
    );
}
