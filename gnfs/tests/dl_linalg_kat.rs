//! Known-answer tests (KATs) for D.B.1: F_ℓ linear-algebra substrate + block Lanczos.
//!
//! Three KATs are required by the D.B.1 session spec:
//!
//! - **KAT (a) — F_ℓ arithmetic KAT**: `FlBlockVec` operations match hand-computed F_ℓ values.
//!   Tests `inner_product_matrix` on a small known example with ℓ = 7, verifying the result
//!   matches hand-computed F_ℓ arithmetic.
//!
//! - **KAT (b) — Matrix-build KAT**: construct a small `DLMatrix` (2 relations, small factor
//!   base), call `build_fl_matrix`, verify the resulting `FlSparseMatrix` has the expected F_ℓ
//!   values (exponents reduced mod ℓ, Schirokauer cols in the right columns).
//!
//! - **KAT (c) — Block-Lanczos-F_ℓ KAT**: construct a small known F_ℓ system (4×3 matrix
//!   with a known right kernel), call `block_lanczos_fl`, verify A·x ≡ 0 mod ℓ for the
//!   returned solution vector.
//!
//! # Toy setup
//!
//! All KATs use `FpNaive4` (L=4, 256-bit) for simplicity. The modulus ℓ = 7 is used for
//! KATs (a) and (c); ℓ = 5 is used for KAT (b) (matching the existing DL relation KATs).

use crypto_bigint::Uint;
use gnfs::dl::{
    DLMatrix, DLRelation,
    FlBlockVec, FlMatrixOperator, FlSparseMatrix, FlSparseRow,
    FL_BLOCK_WIDTH, bigint_to_fp, block_lanczos_fl, build_fl_matrix,
};
use num_bigint::BigInt;
use shared_field::{Fp, FpNaive4};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn ell7() -> Uint<4> {
    Uint::<4>::from(7u64)
}

fn ell5() -> Uint<4> {
    Uint::<4>::from(5u64)
}

fn fp(v: u64, ell: &Uint<4>) -> FpNaive4 {
    FpNaive4::from_u64(v, ell)
}

// ─── KAT (a): F_ℓ arithmetic KAT ─────────────────────────────────────────────

/// KAT (a): `FlBlockVec` inner_product_matrix matches hand-computed F_ℓ values.
///
/// # Setup
///
/// Use ℓ = 7 and a 2-row block vector with two columns:
/// - col0 = [2, 3] (values at rows 0 and 1)
/// - col1 = [4, 1]
///
/// # Hand-computed inner-product matrix (self^T · self):
///
/// IP[i][j] = sum_r col_i[r] * col_j[r]  (mod 7)
///
/// IP[0][0] = 2*2 + 3*3 = 4 + 9 = 13 ≡ 6 (mod 7)
/// IP[0][1] = 2*4 + 3*1 = 8 + 3 = 11 ≡ 4 (mod 7)
/// IP[1][0] = 4*2 + 1*3 = 8 + 3 = 11 ≡ 4 (mod 7)
/// IP[1][1] = 4*4 + 1*1 = 16 + 1 = 17 ≡ 3 (mod 7)
#[test]
fn kat_a_fl_arithmetic_inner_product() {
    let ell = ell7();

    // Construct a 2-row block vector with two non-zero columns.
    let col0 = vec![fp(2, &ell), fp(3, &ell)];
    let col1 = vec![fp(4, &ell), fp(1, &ell)];
    let v = FlBlockVec::<FpNaive4, 4>::from_columns(&[col0, col1], &ell);

    // Compute self^T · self.
    let ip = v.inner_product_matrix(&v, &ell);

    // Verify hand-computed values.
    assert_eq!(
        ip[0][0].to_uint(),
        Uint::<4>::from(6u64),
        "IP[0][0] = 2*2 + 3*3 = 13 ≡ 6 (mod 7)"
    );
    assert_eq!(
        ip[0][1].to_uint(),
        Uint::<4>::from(4u64),
        "IP[0][1] = 2*4 + 3*1 = 11 ≡ 4 (mod 7)"
    );
    assert_eq!(
        ip[1][0].to_uint(),
        Uint::<4>::from(4u64),
        "IP[1][0] = 4*2 + 1*3 = 11 ≡ 4 (mod 7)"
    );
    assert_eq!(
        ip[1][1].to_uint(),
        Uint::<4>::from(3u64),
        "IP[1][1] = 4*4 + 1*1 = 17 ≡ 3 (mod 7)"
    );

    // Verify symmetry: IP[i][j] = IP[j][i] (since self^T · self is symmetric).
    assert_eq!(ip[0][1], ip[1][0], "inner-product matrix should be symmetric");

    // Verify all other entries (columns 2..FL_BLOCK_WIDTH) are zero.
    for j in 2..FL_BLOCK_WIDTH {
        assert!(ip[0][j].is_zero(&ell), "IP[0][{j}] should be zero (column {j} is zero)");
        assert!(ip[j][0].is_zero(&ell), "IP[{j}][0] should be zero (column {j} is zero)");
        assert!(ip[1][j].is_zero(&ell), "IP[1][{j}] should be zero (column {j} is zero)");
        assert!(ip[j][1].is_zero(&ell), "IP[{j}][1] should be zero (column {j} is zero)");
    }
}

/// KAT (a2): `FlBlockVec::add_assign` matches hand-computed F_ℓ addition.
///
/// col0 = [3, 5] + [6, 2] = [9, 7] ≡ [2, 0] (mod 7)
#[test]
fn kat_a2_fl_add_assign() {
    let ell = ell7();
    let mut a = FlBlockVec::<FpNaive4, 4>::from_columns(
        &[vec![fp(3, &ell), fp(5, &ell)]],
        &ell,
    );
    let b = FlBlockVec::<FpNaive4, 4>::from_columns(
        &[vec![fp(6, &ell), fp(2, &ell)]],
        &ell,
    );
    a.add_assign(&b, &ell);
    // 3 + 6 = 9 ≡ 2 (mod 7)
    assert_eq!(a.get(0, 0).to_uint(), Uint::<4>::from(2u64), "3 + 6 ≡ 2 (mod 7)");
    // 5 + 2 = 7 ≡ 0 (mod 7)
    assert!(a.get(1, 0).is_zero(&ell), "5 + 2 ≡ 0 (mod 7)");
}

/// KAT (a3): `bigint_to_fp` handles positive, negative, and zero BigInt values.
#[test]
fn kat_a3_bigint_to_fp() {
    let ell = ell7();

    // Positive: 10 mod 7 = 3.
    let fp_pos: FpNaive4 = bigint_to_fp(&bi(10), &ell);
    assert_eq!(fp_pos.to_uint(), Uint::<4>::from(3u64), "10 mod 7 = 3");

    // Negative: -3 + 7 = 4.
    let fp_neg: FpNaive4 = bigint_to_fp(&bi(-3), &ell);
    assert_eq!(fp_neg.to_uint(), Uint::<4>::from(4u64), "-3 mod 7 = 4");

    // Zero.
    let fp_zero: FpNaive4 = bigint_to_fp(&bi(0), &ell);
    assert!(fp_zero.is_zero(&ell), "0 mod 7 = 0");

    // ℓ = 5: value 3 stays 3.
    let ell5 = ell5();
    let fp_3: FpNaive4 = bigint_to_fp(&bi(3), &ell5);
    assert_eq!(fp_3.to_uint(), Uint::<4>::from(3u64), "3 mod 5 = 3");

    // ℓ = 5: value -2 → -2 + 5 = 3.
    let fp_neg2: FpNaive4 = bigint_to_fp(&bi(-2), &ell5);
    assert_eq!(fp_neg2.to_uint(), Uint::<4>::from(3u64), "-2 mod 5 = 3");
}

// ─── KAT (b): Matrix-build KAT ───────────────────────────────────────────────

/// KAT (b): `build_fl_matrix` produces the expected F_ℓ matrix from a small DLMatrix.
///
/// # Setup
///
/// Construct a DLMatrix with 2 relations and a small factor base:
/// - rational_size = 2 (primes at indices 0, 1)
/// - algebraic_size = 2 (ideals at indices 0, 1)
/// - schirokauer_rank = 1 (one Schirokauer column)
///
/// Relation 0:
/// - rational_exponents: [(0, 3), (1, 1)]  → col 0 = 3 mod 5 = 3, col 1 = 1 mod 5 = 1
/// - algebraic_exponents: [(0, 2), (1, 4)] → col 2 = 2 mod 5 = 2, col 3 = 4 mod 5 = 4
/// - schirokauer_cols: [BigInt(3)]          → col 4 = 3
///
/// Relation 1:
/// - rational_exponents: [(0, 6)]           → col 0 = 6 mod 5 = 1
/// - algebraic_exponents: [(1, 7)]          → col 3 = 7 mod 5 = 2
/// - schirokauer_cols: [BigInt(-2)]         → col 4 = -2 + 5 = 3
///
/// # Verification
///
/// The resulting FlSparseMatrix should have:
/// - 2 rows, 5 columns.
/// - Row 0: entries at cols 0,1,2,3,4 with values 3,1,2,4,3.
/// - Row 1: entries at cols 0,3,4 with values 1,2,3.
#[test]
fn kat_b_matrix_build() {
    use gnfs::sieve::{ExponentVector, Relation};

    let ell = ell5();

    // Build the DLMatrix directly.
    // Relation 0: rational [(0,3),(1,1)], algebraic [(0,2),(1,4)], schirokauer [3].
    let rel0 = {
        let mut rat = ExponentVector::new();
        rat.entries.push((0, 3));
        rat.entries.push((1, 1));
        let mut alg = ExponentVector::new();
        alg.entries.push((0, 2));
        alg.entries.push((1, 4));
        let relation = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: rat,
            algebraic_exponents: alg,
            rational_sign: false,
        };
        DLRelation::new(relation, vec![bi(3)])
    };

    // Relation 1: rational [(0,6)], algebraic [(1,7)], schirokauer [-2].
    let rel1 = {
        let mut rat = ExponentVector::new();
        rat.entries.push((0, 6));
        let mut alg = ExponentVector::new();
        alg.entries.push((1, 7));
        let relation = Relation {
            a: bi(2),
            b: bi(1),
            rational_exponents: rat,
            algebraic_exponents: alg,
            rational_sign: false,
        };
        DLRelation::new(relation, vec![bi(-2)])
    };

    let dl_matrix = DLMatrix {
        relations: vec![rel0, rel1],
        rational_size: 2,
        algebraic_size: 2,
        schirokauer_rank: 1,
    };

    // Build the F_ℓ matrix.
    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);

    // Verify dimensions.
    assert_eq!(fl_matrix.rows.len(), 2, "should have 2 rows");
    assert_eq!(fl_matrix.num_cols, 5, "should have 5 columns (2 rat + 2 alg + 1 schiro)");

    // Helper: look up a value in a sparse row by column index.
    let get_entry = |row: &FlSparseRow<FpNaive4>, col: usize| -> Option<u64> {
        row.entries.iter().find(|&&(c, _)| c == col).map(|(_, v)| v.to_uint().as_words()[0])
    };

    // Verify row 0.
    let row0 = &fl_matrix.rows[0];
    assert_eq!(get_entry(row0, 0), Some(3), "row 0, col 0: 3 mod 5 = 3");
    assert_eq!(get_entry(row0, 1), Some(1), "row 0, col 1: 1 mod 5 = 1");
    assert_eq!(get_entry(row0, 2), Some(2), "row 0, col 2: 2 mod 5 = 2");
    assert_eq!(get_entry(row0, 3), Some(4), "row 0, col 3: 4 mod 5 = 4");
    assert_eq!(get_entry(row0, 4), Some(3), "row 0, col 4: schirokauer 3");

    // Verify row 1.
    let row1 = &fl_matrix.rows[1];
    assert_eq!(get_entry(row1, 0), Some(1), "row 1, col 0: 6 mod 5 = 1");
    assert_eq!(get_entry(row1, 1), None, "row 1, col 1: not present (exponent 0)");
    assert_eq!(get_entry(row1, 2), None, "row 1, col 2: not present (exponent 0)");
    assert_eq!(get_entry(row1, 3), Some(2), "row 1, col 3: 7 mod 5 = 2");
    assert_eq!(get_entry(row1, 4), Some(3), "row 1, col 4: schirokauer -2 + 5 = 3");
}

/// KAT (b2): `build_fl_matrix` handles zero exponents correctly (they are omitted).
///
/// A relation with all-zero exponents should produce an empty sparse row.
#[test]
fn kat_b2_matrix_build_zero_exponents() {
    use gnfs::sieve::{ExponentVector, Relation};

    let ell = ell7();

    let rel = {
        let relation = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };
        DLRelation::new(relation, vec![bi(0)])
    };

    let dl_matrix = DLMatrix {
        relations: vec![rel],
        rational_size: 3,
        algebraic_size: 3,
        schirokauer_rank: 1,
    };

    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);

    assert_eq!(fl_matrix.rows.len(), 1);
    assert_eq!(fl_matrix.num_cols, 7);
    // All exponents are zero → empty sparse row.
    assert!(fl_matrix.rows[0].entries.is_empty(), "all-zero relation should produce empty row");
}

// ─── KAT (c): Block-Lanczos-F_ℓ KAT ─────────────────────────────────────────

/// KAT (c): `block_lanczos_fl` finds a kernel vector for a small known F_ℓ system.
///
/// # Setup
///
/// Use ℓ = 7 and a 4×3 F_ℓ matrix:
///
/// ```text
///     col: 0  1  2
/// row 0:   1  0  1
/// row 1:   0  1  1
/// row 2:   2  3  5
/// row 3:   1  2  3
/// ```
///
/// # Known kernel vector
///
/// x = [1, 1, 6] (mod 7) is in the right kernel (A·x = 0 mod 7):
/// - Row 0: 1·1 + 0·1 + 1·6 = 1 + 0 + 6 = 7 ≡ 0 (mod 7) ✓
/// - Row 1: 0·1 + 1·1 + 1·6 = 0 + 1 + 6 = 7 ≡ 0 (mod 7) ✓
/// - Row 2: 2·1 + 3·1 + 5·6 = 2 + 3 + 30 = 35 ≡ 0 (mod 7) ✓
/// - Row 3: 1·1 + 2·1 + 3·6 = 1 + 2 + 18 = 21 ≡ 0 (mod 7) ✓
///
/// # Verification
///
/// For each returned solution, verify A·x ≡ 0 mod ℓ by applying the matrix operator.
#[test]
fn kat_c_block_lanczos_fl_known_kernel() {
    let ell = ell7();

    // Build the 4×3 F_ℓ sparse matrix.
    // Row 0: (0, 1), (2, 1)
    // Row 1: (1, 1), (2, 1)
    // Row 2: (0, 2), (1, 3), (2, 5)
    // Row 3: (0, 1), (1, 2), (2, 3)
    let matrix = FlSparseMatrix::<FpNaive4> {
        rows: vec![
            FlSparseRow { entries: vec![(0, fp(1, &ell)), (2, fp(1, &ell))] },
            FlSparseRow { entries: vec![(1, fp(1, &ell)), (2, fp(1, &ell))] },
            FlSparseRow { entries: vec![(0, fp(2, &ell)), (1, fp(3, &ell)), (2, fp(5, &ell))] },
            FlSparseRow { entries: vec![(0, fp(1, &ell)), (1, fp(2, &ell)), (2, fp(3, &ell))] },
        ],
        num_cols: 3,
    };

    let op = FlMatrixOperator::<FpNaive4, 4>::new(&matrix);

    // Verify the known kernel vector by hand before running Lanczos.
    {
        let x = FlBlockVec::<FpNaive4, 4>::from_columns(
            &[vec![fp(1, &ell), fp(1, &ell), fp(6, &ell)]],
            &ell,
        );
        // x has 3 rows (one per column of A); apply A to it.
        // But x is a block vector with num_rows = 3 (= num_cols of A).
        // We need to apply A to x as a single-column block vector.
        let ax = op.apply(&x, &ell);
        // ax should be all zero (4 rows, 1 column).
        for r in 0..4 {
            assert!(
                ax.data[r][0].is_zero(&ell),
                "A·x[{r}] should be 0 for the known kernel vector"
            );
        }
    }

    // Run block Lanczos with multiple seeds to find the kernel.
    let mut found_valid = false;
    for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        let solutions = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);

        for sol in &solutions {
            assert!(sol.is_kernel, "all returned solutions should be kernel vectors");
            assert_eq!(
                sol.coefficients.len(),
                3,
                "solution vector should have length 3 (= num_cols)"
            );

            // Verify A·x = 0 mod ℓ.
            let x_bv = FlBlockVec::<FpNaive4, 4>::from_columns(&[sol.coefficients.clone()], &ell);
            let ax = op.apply(&x_bv, &ell);
            for r in 0..4 {
                assert!(
                    ax.data[r][0].is_zero(&ell),
                    "seed {seed}: A·x[{r}] should be 0 for returned kernel vector"
                );
            }

            // Check that the solution is nontrivial (not all zero).
            let is_nontrivial = sol.coefficients.iter().any(|c| !c.is_zero(&ell));
            if is_nontrivial {
                found_valid = true;
            }
        }
    }

    assert!(
        found_valid,
        "block_lanczos_fl should find at least one nontrivial kernel vector across multiple seeds"
    );
}

/// KAT (c2): `block_lanczos_fl` returns empty for a matrix with trivial kernel.
///
/// A 3×3 identity matrix over F_ℓ has no nontrivial right kernel.
/// Lanczos should return no nontrivial solutions.
#[test]
fn kat_c2_block_lanczos_fl_trivial_kernel() {
    let ell = ell7();

    // 3×3 identity matrix over F_7.
    let matrix = FlSparseMatrix::<FpNaive4> {
        rows: vec![
            FlSparseRow { entries: vec![(0, fp(1, &ell))] },
            FlSparseRow { entries: vec![(1, fp(1, &ell))] },
            FlSparseRow { entries: vec![(2, fp(1, &ell))] },
        ],
        num_cols: 3,
    };

    let op = FlMatrixOperator::<FpNaive4, 4>::new(&matrix);

    // For a full-rank matrix, no nontrivial kernel vectors should be found.
    for seed in [0u64, 1, 42, 137] {
        let solutions = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);

        // All returned solutions must satisfy A·x = 0 (even if the algorithm returns
        // false positives, the verify check will catch them).
        for sol in &solutions {
            let x_bv = FlBlockVec::<FpNaive4, 4>::from_columns(&[sol.coefficients.clone()], &ell);
            let ax = op.apply(&x_bv, &ell);
            for r in 0..3 {
                assert!(
                    ax.data[r][0].is_zero(&ell),
                    "seed {seed}: A·x[{r}] should be 0 for any returned solution"
                );
            }
        }
    }
}

/// KAT (c3): `block_lanczos_fl` is deterministic for a fixed matrix and seed.
#[test]
fn kat_c3_block_lanczos_fl_deterministic() {
    let ell = ell7();

    let matrix = FlSparseMatrix::<FpNaive4> {
        rows: vec![
            FlSparseRow { entries: vec![(0, fp(1, &ell)), (2, fp(1, &ell))] },
            FlSparseRow { entries: vec![(1, fp(1, &ell)), (2, fp(1, &ell))] },
            FlSparseRow { entries: vec![(0, fp(2, &ell)), (1, fp(3, &ell)), (2, fp(5, &ell))] },
            FlSparseRow { entries: vec![(0, fp(1, &ell)), (1, fp(2, &ell)), (2, fp(3, &ell))] },
        ],
        num_cols: 3,
    };

    let op = FlMatrixOperator::<FpNaive4, 4>::new(&matrix);

    // Run twice with the same seed; results must be identical.
    for seed in [0u64, 1, 42, 137] {
        let results1 = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);
        let results2 = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);

        assert_eq!(
            results1.len(),
            results2.len(),
            "seed {seed}: both runs must return the same number of solutions"
        );

        for (i, (s1, s2)) in results1.iter().zip(results2.iter()).enumerate() {
            assert_eq!(
                s1.coefficients, s2.coefficients,
                "seed {seed}: solution {i} must be identical across runs"
            );
        }
    }
}

/// KAT (c4): `block_lanczos_fl` handles an empty matrix gracefully.
#[test]
fn kat_c4_block_lanczos_fl_empty_matrix() {
    let ell = ell7();

    let matrix = FlSparseMatrix::<FpNaive4> { rows: vec![], num_cols: 0 };
    let op = FlMatrixOperator::<FpNaive4, 4>::new(&matrix);
    let solutions = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, 42);
    assert!(solutions.is_empty(), "empty matrix should return no solutions");
}
