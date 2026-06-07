//! Known-answer tests (KATs) for G.E.3: block Wiedemann GF(2) nullspace solver.
//!
//! Three KATs are required by the G.E.3 session spec:
//!
//! - **KAT (a) — Cross-validation with block Lanczos**: for a shared small matrix, both
//!   solvers find valid kernel vectors. Every vector returned by Wiedemann passes
//!   `KernelVector::verify`; the nullspace dimension found by Wiedemann is at least 1
//!   whenever Lanczos also finds at least 1.
//!
//! - **KAT (b) — Deterministic kernel dimension**: for a fixed matrix and seed,
//!   `block_wiedemann` returns the same kernel vectors across two calls.
//!
//! - **KAT (c) — Berlekamp-Massey generator degree**: for a hand-constructed GF(2)
//!   sequence (the Fibonacci sequence mod 2), the BM algorithm returns a polynomial of
//!   the expected degree with the expected coefficients.
//!
//! # Shared matrix (used in KAT (a) and KAT (b))
//!
//! The same 6 × 4 matrix used in the Lanczos KATs:
//!
//! ```text
//!     col: 0 1 2 3
//! row 0:   1 1 0 0   ← provenance [0]
//! row 1:   0 1 1 0   ← provenance [1]
//! row 2:   1 0 1 0   ← provenance [2]
//! row 3:   1 1 0 0   ← provenance [3]  (duplicate of row 0)
//! row 4:   0 0 0 1   ← provenance [4]
//! row 5:   0 0 0 1   ← provenance [5]  (duplicate of row 4)
//! ```
//!
//! Known left nullspace vectors:
//! - `{0, 3}`: rows 0 and 3 are identical → XOR = 0.
//! - `{4, 5}`: rows 4 and 5 are identical → XOR = 0.
//! - `{0, 1, 2}`: {0,1} XOR {1,2} XOR {0,2} = {} → XOR = 0.

use gnfs::{
    linalg::{block_lanczos, block_wiedemann, KernelVector, MatrixOperator},
};
use gnfs::filter::{MatrixRow, SparseMatrix};
use gnfs::linalg::wiedemann::berlekamp_massey;

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Build a `SparseMatrix` from a list of (cols, provenance) pairs.
fn make_matrix(rows: Vec<(Vec<usize>, Vec<usize>)>, num_cols: usize) -> SparseMatrix {
    SparseMatrix {
        rows: rows
            .into_iter()
            .map(|(cols, provenance)| MatrixRow { cols, provenance })
            .collect(),
        num_cols,
        obstruction_col_start: num_cols,
        obstruction_count: 0,
        col_weights: vec![0u32; num_cols],
    }
}

/// Build the shared 6 × 4 matrix used across KAT (a) and KAT (b).
fn shared_matrix() -> SparseMatrix {
    make_matrix(
        vec![
            (vec![0, 1], vec![0]), // row 0
            (vec![1, 2], vec![1]), // row 1
            (vec![0, 2], vec![2]), // row 2
            (vec![0, 1], vec![3]), // row 3 — duplicate of row 0
            (vec![3], vec![4]),    // row 4
            (vec![3], vec![5]),    // row 5 — duplicate of row 4
        ],
        4,
    )
}

/// Verify that every returned kernel vector is a valid left nullspace vector of `matrix`.
///
/// Returns `(all_valid, count)`.
fn verify_all(results: &[KernelVector], matrix: &SparseMatrix) -> (bool, usize) {
    let all_valid = results.iter().all(|kv| kv.verify(matrix));
    (all_valid, results.len())
}

// ─── KAT (a): Cross-validation with block Lanczos ────────────────────────────

/// KAT (a): Wiedemann and Lanczos cross-validate on the shared matrix.
///
/// For each kernel vector found by Wiedemann, it must pass `KernelVector::verify`.
/// Whenever Lanczos finds at least one kernel vector (across the seed range), Wiedemann
/// must also find at least one valid kernel vector (not necessarily the same one, but
/// the nullspace is nontrivial for both).
///
/// This cross-validates that both solvers agree on the existence of a nontrivial left
/// nullspace, even though they may return different basis vectors.
#[test]
fn kat_a_cross_validation_with_lanczos() {
    let matrix = shared_matrix();

    // Verify the known nullspace vectors by hand.
    {
        let kv_03 = KernelVector::new(vec![0, 3]);
        assert!(kv_03.verify(&matrix), "rows {{0,3}} should be a valid nullspace vector");
        let kv_45 = KernelVector::new(vec![4, 5]);
        assert!(kv_45.verify(&matrix), "rows {{4,5}} should be a valid nullspace vector");
        let kv_012 = KernelVector::new(vec![0, 1, 2]);
        assert!(kv_012.verify(&matrix), "rows {{0,1,2}} should be a valid nullspace vector");
    }

    let op = MatrixOperator::new(&matrix);

    // Check Lanczos finds at least one kernel vector across the seed range.
    let mut lanczos_found_any = false;
    for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        let results = block_lanczos(&op, seed);
        let (all_valid, count) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "Lanczos seed {seed}: all returned kernel vectors must be valid (got {count})"
        );
        if count > 0 {
            lanczos_found_any = true;
        }
    }
    assert!(
        lanczos_found_any,
        "block_lanczos should find at least one kernel vector across multiple seeds"
    );

    // Check Wiedemann finds at least one kernel vector across the seed range.
    // Every returned vector must be valid.
    let mut wiedemann_found_any = false;
    for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        let results = block_wiedemann(&op, seed);
        let (all_valid, count) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "Wiedemann seed {seed}: all returned kernel vectors must be valid (got {count})"
        );
        if count > 0 {
            wiedemann_found_any = true;
        }
    }

    // Cross-validation: if Lanczos finds a nontrivial nullspace, Wiedemann must too.
    if lanczos_found_any {
        assert!(
            wiedemann_found_any,
            "block_wiedemann should find at least one kernel vector when Lanczos does \
             (both solvers must agree on the existence of a nontrivial left nullspace)"
        );
    }
}

/// KAT (a2): Wiedemann handles a full-rank matrix (no nullspace) gracefully.
///
/// A 3 × 3 identity matrix has no left nullspace. Wiedemann should return no kernel
/// vectors (or only valid ones — the verify check catches false positives).
#[test]
fn kat_a2_full_rank_no_nullspace() {
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]),
            (vec![1], vec![1]),
            (vec![2], vec![2]),
        ],
        3,
    );

    let op = MatrixOperator::new(&matrix);

    for seed in [0u64, 1, 42, 137] {
        let results = block_wiedemann(&op, seed);
        let (all_valid, _count) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "seed {seed}: all returned kernel vectors must be valid even for full-rank matrix"
        );
    }
}

/// KAT (a3): Wiedemann handles an empty matrix gracefully.
#[test]
fn kat_a3_empty_matrix() {
    let matrix = make_matrix(vec![], 0);
    let op = MatrixOperator::new(&matrix);
    let results = block_wiedemann(&op, 42);
    assert!(results.is_empty(), "empty matrix should return no kernel vectors");
}

/// KAT (a4): Wiedemann finds the nullspace of a matrix with a single dependency.
///
/// Matrix: 4 × 3, rows {0, 3} are identical → left nullspace contains {0, 3}.
#[test]
fn kat_a4_single_dependency() {
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]), // row 0
            (vec![1], vec![1]), // row 1
            (vec![2], vec![2]), // row 2
            (vec![0], vec![3]), // row 3 — duplicate of row 0
        ],
        3,
    );

    {
        let kv = KernelVector::new(vec![0, 3]);
        assert!(kv.verify(&matrix), "rows {{0,3}} should be a valid nullspace vector");
    }

    let op = MatrixOperator::new(&matrix);

    let mut found_any = false;
    for seed in [0u64, 1, 2, 3, 42, 100, 200, 300] {
        let results = block_wiedemann(&op, seed);
        let (all_valid, count) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "seed {seed}: all returned kernel vectors must be valid (got {count})"
        );
        if count > 0 {
            found_any = true;
        }
    }

    assert!(
        found_any,
        "block_wiedemann should find the dependency {{0,3}} across multiple seeds"
    );
}

// ─── KAT (b): Deterministic kernel dimension ─────────────────────────────────

/// KAT (b): `block_wiedemann` is deterministic for a fixed matrix and seed.
///
/// Two calls with the same matrix and seed must return identical kernel vectors.
/// This verifies there is no hidden mutable state or OS-level randomness.
#[test]
fn kat_b_determinism() {
    let matrix = shared_matrix();
    let op = MatrixOperator::new(&matrix);

    // Run twice with the same seed; results must be identical.
    for seed in [0u64, 1, 42, 137, 999] {
        let results1 = block_wiedemann(&op, seed);
        let results2 = block_wiedemann(&op, seed);

        assert_eq!(
            results1.len(),
            results2.len(),
            "seed {seed}: both runs must return the same number of kernel vectors"
        );

        for (i, (kv1, kv2)) in results1.iter().zip(results2.iter()).enumerate() {
            assert_eq!(
                kv1.row_indices,
                kv2.row_indices,
                "seed {seed}: kernel vector {i} must be identical across runs"
            );
        }
    }

    // Different seeds may return different results — verify each run is valid.
    let results_seed0 = block_wiedemann(&op, 0);
    let results_seed1 = block_wiedemann(&op, 1);
    let (valid0, _) = verify_all(&results_seed0, &matrix);
    let (valid1, _) = verify_all(&results_seed1, &matrix);
    assert!(valid0, "seed 0: all kernel vectors must be valid");
    assert!(valid1, "seed 1: all kernel vectors must be valid");
}

// ─── KAT (c): Berlekamp-Massey generator degree ──────────────────────────────

/// KAT (c): The Berlekamp-Massey algorithm returns the expected polynomial for a
/// hand-constructed GF(2) sequence.
///
/// # Sequence: Fibonacci mod 2
///
/// The Fibonacci sequence mod 2 starting from (s_0=0, s_1=1):
///   0, 1, 1, 0, 1, 1, 0, 1, 1, 0, ...
///
/// The recurrence is `s_n = s_{n-1} XOR s_{n-2}`, so the minimal polynomial is:
///   f(z) = 1 + z + z^2
///
/// In coefficient form: `[f_0, f_1, f_2] = [1, 1, 1]` (all true).
///
/// # Hand-verification
///
/// Check that f(z) = 1 + z + z^2 generates the sequence:
/// - n=2: f_0*s_2 + f_1*s_1 + f_2*s_0 = 1 + 1 + 0 = 0 ✓ (mod 2)
/// - n=3: f_0*s_3 + f_1*s_2 + f_2*s_1 = 0 + 1 + 1 = 0 ✓ (mod 2)
/// - n=4: f_0*s_4 + f_1*s_3 + f_2*s_2 = 1 + 0 + 1 = 0 ✓ (mod 2)
///
/// The degree is 2, which is the hand-computed value.
#[test]
fn kat_c_bm_fibonacci_degree() {
    // Fibonacci sequence mod 2: 0, 1, 1, 0, 1, 1, 0, 1, 1, 0
    let s: Vec<bool> = vec![false, true, true, false, true, true, false, true, true, false];

    let f = berlekamp_massey(&s);

    // The degree must be 2 (polynomial has 3 coefficients).
    assert_eq!(
        f.len(),
        3,
        "Fibonacci mod 2 minimal polynomial must have degree 2 (got degree {})",
        f.len() - 1
    );

    // Coefficients: f(z) = 1 + z + z^2, so f[0]=1, f[1]=1, f[2]=1.
    assert!(f[0], "f[0] (constant term) must be 1");
    assert!(f[1], "f[1] (z coefficient) must be 1 for Fibonacci mod 2");
    assert!(f[2], "f[2] (z^2 coefficient) must be 1 for Fibonacci mod 2");

    // Verify the polynomial generates the sequence: sum_{k=0}^{2} f[k] * s[n-k] = 0 for n >= 2.
    for n in 2..s.len() {
        let check = f[0] && s[n] ^ (f[1] && s[n - 1]) ^ (f[2] && s[n - 2]);
        assert!(
            !check,
            "f(z) must generate the sequence: discrepancy at n={n} is nonzero"
        );
    }
}

/// KAT (c2): BM on a period-4 sequence with known minimal polynomial.
///
/// Sequence: 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0 (period 4: 0,0,1,0 repeating).
///
/// The recurrence is `s_n = s_{n-4}`, so the minimal polynomial is f(z) = 1 + z^4.
/// Coefficients: [1, 0, 0, 0, 1] (f[0]=1, f[1]=0, f[2]=0, f[3]=0, f[4]=1).
///
/// # Hand-verification
///
/// Check that f(z) = 1 + z^4 generates the sequence:
/// - n=4: s_4 XOR s_0 = 0 XOR 0 = 0 ✓
/// - n=5: s_5 XOR s_1 = 0 XOR 0 = 0 ✓
/// - n=6: s_6 XOR s_2 = 1 XOR 1 = 0 ✓
/// - n=7: s_7 XOR s_3 = 0 XOR 0 = 0 ✓
#[test]
fn kat_c2_bm_period4_sequence() {
    // Period-4 sequence: 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0
    let s: Vec<bool> =
        vec![false, false, true, false, false, false, true, false, false, false, true, false];

    let f = berlekamp_massey(&s);

    // The degree must be 4 (polynomial has 5 coefficients).
    assert_eq!(
        f.len(),
        5,
        "period-4 sequence minimal polynomial must have degree 4 (got degree {})",
        f.len() - 1
    );

    // Coefficients: f(z) = 1 + z^4, so f[0]=1, f[1]=0, f[2]=0, f[3]=0, f[4]=1.
    assert!(f[0], "f[0] must be 1");
    assert!(!f[1], "f[1] must be 0");
    assert!(!f[2], "f[2] must be 0");
    assert!(!f[3], "f[3] must be 0");
    assert!(f[4], "f[4] must be 1");

    // Verify the polynomial generates the sequence.
    for n in 4..s.len() {
        let check = (f[0] && s[n]) ^ (f[4] && s[n - 4]);
        assert!(
            !check,
            "f(z) must generate the sequence: discrepancy at n={n} is nonzero"
        );
    }
}

/// KAT (c3): BM on an all-ones sequence returns degree-1 polynomial.
///
/// Sequence: 1, 1, 1, 1, 1, 1, 1, 1 (all ones).
/// Recurrence: s_n = s_{n-1}, minimal polynomial f(z) = 1 + z.
#[test]
fn kat_c3_bm_all_ones_degree() {
    let s = vec![true; 8];
    let f = berlekamp_massey(&s);

    assert_eq!(
        f.len(),
        2,
        "all-ones sequence minimal polynomial must have degree 1 (got degree {})",
        f.len() - 1
    );
    assert!(f[0], "f[0] must be 1");
    assert!(f[1], "f[1] must be 1");
}

// ─── Additional edge-case tests ───────────────────────────────────────────────

/// Wiedemann handles a matrix with all-zero rows gracefully.
///
/// A matrix with all-zero rows has every vector in its left nullspace (trivially, since
/// A^T * v = 0 for any v when A = 0). However, the algorithm should not return the zero
/// vector as a kernel vector.
#[test]
fn kat_all_zero_rows() {
    // 3 × 3 matrix with all-zero rows.
    let matrix = make_matrix(
        vec![
            (vec![], vec![0]),
            (vec![], vec![1]),
            (vec![], vec![2]),
        ],
        3,
    );

    let op = MatrixOperator::new(&matrix);

    for seed in [0u64, 1, 42] {
        let results = block_wiedemann(&op, seed);
        // Every returned vector must be valid (verify checks A^T * v = 0).
        let (all_valid, _) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "seed {seed}: all returned kernel vectors must be valid for all-zero matrix"
        );
        // No returned vector should be empty (the zero vector is not a valid kernel vector).
        for kv in &results {
            assert!(!kv.is_empty(), "zero vector should not be returned as a kernel vector");
        }
    }
}

/// Wiedemann finds multiple kernel vectors for a matrix with multiple dependencies.
///
/// Matrix: 8 × 6, with three independent nullspace vectors.
#[test]
fn kat_multiple_dependencies() {
    // Group 1: rows 0,1,2 form a nullspace vector ({0,1} XOR {1,2} XOR {0,2} = {}).
    // Group 2: rows 3,5 are identical ({3,4} XOR {3,4} = {}).
    // Group 3: rows 6,7 are identical ({0,1} XOR {0,1} = {}).
    let matrix = make_matrix(
        vec![
            (vec![0, 1], vec![0]), // row 0
            (vec![1, 2], vec![1]), // row 1
            (vec![0, 2], vec![2]), // row 2
            (vec![3, 4], vec![3]), // row 3
            (vec![4, 5], vec![4]), // row 4
            (vec![3, 4], vec![5]), // row 5 — duplicate of row 3
            (vec![0, 1], vec![6]), // row 6
            (vec![0, 1], vec![7]), // row 7 — duplicate of row 6
        ],
        6,
    );

    {
        let kv_012 = KernelVector::new(vec![0, 1, 2]);
        assert!(kv_012.verify(&matrix), "rows {{0,1,2}} should be a valid nullspace vector");
        let kv_35 = KernelVector::new(vec![3, 5]);
        assert!(kv_35.verify(&matrix), "rows {{3,5}} should be a valid nullspace vector");
        let kv_67 = KernelVector::new(vec![6, 7]);
        assert!(kv_67.verify(&matrix), "rows {{6,7}} should be a valid nullspace vector");
    }

    let op = MatrixOperator::new(&matrix);

    let mut found_any = false;
    for seed in [0u64, 1, 2, 3, 42, 100, 200, 300, 999, 12345] {
        let results = block_wiedemann(&op, seed);
        let (all_valid, count) = verify_all(&results, &matrix);
        assert!(
            all_valid,
            "seed {seed}: all returned kernel vectors must be valid (got {count})"
        );
        if count > 0 {
            found_any = true;
        }
    }

    assert!(
        found_any,
        "block_wiedemann should find at least one of the known dependencies across multiple seeds"
    );
}
