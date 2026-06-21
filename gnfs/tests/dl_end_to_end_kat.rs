//! End-to-end toy-F_p discrete-log KAT (F_ℓ linear-algebra integration vehicle).
//!
//! Recovers a known toy discrete log through the full NFS-DL linear-algebra path:
//! DLMatrix construction → F_ℓ matrix build → solve (Lanczos and Wiedemann) →
//! virtual-log recovery → cross-check against hand-computed reference.
//!
//! # Toy setup
//!
//! - p = 11, g = 2 (primitive root mod 11, order 10).
//! - ℓ = 5 (prime dividing p-1 = 10).
//! - Factor base: rational primes {2, 3} (indices 0, 1).
//! - No algebraic factor base, no Schirokauer columns (toy simplification).
//!
//! # Known virtual logs (mod 5)
//!
//! In (ℤ/11ℤ)*, with g = 2:
//! - log_2(2) = 1 mod 10 → 1 mod 5.
//! - log_2(3) = 8 mod 10 → 3 mod 5.
//!
//! So the true virtual-log vector is [1, 3] mod 5 (up to scalar in the kernel).
//!
//! # DLMatrix construction
//!
//! Relations are constructed directly (bypassing the sieve) as rows [e0, e1] satisfying
//! e0 * log_2(2) + e1 * log_2(3) ≡ 0 (mod 5), i.e., e0 + 3*e1 ≡ 0 (mod 5):
//!
//! - Row 0: [3, 4] → 3*1 + 4*3 = 15 ≡ 0 (mod 5) ✓
//! - Row 1: [4, 2] → 4*1 + 2*3 = 10 ≡ 0 (mod 5) ✓
//! - Row 2: [2, 1] → 2*1 + 1*3 = 5 ≡ 0 (mod 5) ✓
//!
//! These rows span a 1-dimensional subspace of F_5^2 (all are scalar multiples of [3, 4]),
//! so the right kernel is 1-dimensional, spanned by [1, 3] (the true virtual-log vector).
//!
//! # Verification
//!
//! The recovered virtual logs [x0, x1] must satisfy x0 * 3 ≡ x1 * 1 (mod 5), i.e.,
//! x0 / x1 = 1/3 = 2 (mod 5). This is the ratio log_2(2) / log_2(3) = 1/3 mod 5.
//!
//! # PARI cross-check (stub)
//!
//! The PARI oracle `znlog(Mod(3, 11), Mod(2, 11))` returns 8 (= log_2(3) mod 10).
//! Reduced mod 5: 8 mod 5 = 3. The recovered virtual log of prime 3 should be 3 (mod 5)
//! or a nonzero scalar multiple (since the kernel is determined up to scalar).

use crypto_bigint::Uint;
use gnfs::dl::{
    DLMatrix, DLRelation,
    FlBlockVec, FlMatrixOperator, FlSparseMatrix, FlSparseRow, FlSolution,
    VirtualLogTable, block_lanczos_fl, block_wiedemann_fl, build_fl_matrix, recover_virtual_logs,
};
use num_bigint::BigInt;
use shared_field::{Fp, FpNaive4};

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn ell5() -> Uint<4> {
    Uint::<4>::from(5u64)
}

fn fp5(v: u64) -> FpNaive4 {
    FpNaive4::from_u64(v, &ell5())
}

// ─── Toy DLMatrix construction ────────────────────────────────────────────────

/// Build the toy DLMatrix for the end-to-end KAT.
///
/// 3 relations × 2 columns (rational primes {2, 3}), no algebraic, no Schirokauer.
/// Each row [e0, e1] satisfies e0 + 3*e1 ≡ 0 (mod 5) (the kernel condition for
/// virtual logs [1, 3]).
fn build_toy_dl_matrix() -> DLMatrix {
    use gnfs::sieve::{ExponentVector, Relation};

    // Helper: build a DLRelation with the given rational exponents.
    let make_rel = |e0: u32, e1: u32| -> DLRelation {
        let mut rat = ExponentVector::new();
        if e0 > 0 {
            rat.entries.push((0, e0));
        }
        if e1 > 0 {
            rat.entries.push((1, e1));
        }
        let relation = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: rat,
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };
        // No Schirokauer columns in this toy example.
        DLRelation::new(relation, vec![])
    };

    // Row 0: [3, 4] → 3*1 + 4*3 = 15 ≡ 0 (mod 5).
    // Row 1: [4, 2] → 4*1 + 2*3 = 10 ≡ 0 (mod 5).
    // Row 2: [2, 1] → 2*1 + 1*3 = 5 ≡ 0 (mod 5).
    let relations = vec![make_rel(3, 4), make_rel(4, 2), make_rel(2, 1)];

    DLMatrix {
        relations,
        rational_size: 2,
        algebraic_size: 0,
        schirokauer_rank: 0,
    }
}

// ─── KAT: end-to-end toy-F_p DL via Lanczos ──────────────────────────────────

/// End-to-end toy-F_p DL KAT via block Lanczos.
///
/// Constructs a toy DLMatrix, builds the F_ℓ matrix, solves with `block_lanczos_fl`,
/// recovers virtual logs, and cross-checks against the hand-computed reference.
///
/// # Expected result
///
/// The recovered virtual logs [x0, x1] must satisfy x0 * 3 ≡ x1 * 1 (mod 5),
/// i.e., x0 = 2 * x1 mod 5 (since 1/3 = 2 mod 5).
#[test]
fn kat_toy_fp_dl_lanczos() {
    let ell = ell5();
    let dl_matrix = build_toy_dl_matrix();

    // Build the F_ℓ matrix.
    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);

    assert_eq!(fl_matrix.rows.len(), 3, "should have 3 rows");
    assert_eq!(fl_matrix.num_cols, 2, "should have 2 columns (rational primes only)");

    let op = FlMatrixOperator::<FpNaive4, 4>::new(&fl_matrix);

    // Solve with block Lanczos.
    let mut found_solution: Option<FlSolution<FpNaive4>> = None;
    for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        let solutions = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);
        for sol in solutions {
            if sol.coefficients.iter().any(|c| !c.is_zero(&ell)) {
                found_solution = Some(sol);
                break;
            }
        }
        if found_solution.is_some() {
            break;
        }
    }

    let sol = found_solution.expect("Lanczos should find a nontrivial kernel vector");

    // Verify A·x = 0 mod ℓ.
    let x_bv = FlBlockVec::<FpNaive4, 4>::from_columns(&[sol.coefficients.clone()], &ell);
    let ax = op.apply(&x_bv, &ell);
    for r in 0..3 {
        assert!(
            ax.data[r][0].is_zero(&ell),
            "A·x[{r}] should be 0 for the kernel vector"
        );
    }

    // Recover virtual logs: 2 rational primes, 0 algebraic, 0 Schirokauer.
    let table: VirtualLogTable<FpNaive4> = recover_virtual_logs(&sol, 2, 0);

    assert_eq!(table.rational_logs.len(), 2, "should have 2 rational logs");
    assert!(table.algebraic_logs.is_empty(), "should have 0 algebraic logs");

    let x0 = &table.rational_logs[0]; // virtual log of prime 2
    let x1 = &table.rational_logs[1]; // virtual log of prime 3

    // Both must be nonzero (the kernel vector is nontrivial).
    assert!(!x0.is_zero(&ell), "virtual log of prime 2 should be nonzero");
    assert!(!x1.is_zero(&ell), "virtual log of prime 3 should be nonzero");

    // Cross-check: x0 * log_2(3) ≡ x1 * log_2(2) (mod 5).
    // True logs: log_2(2) = 1, log_2(3) = 3 (mod 5).
    // So: x0 * 3 ≡ x1 * 1 (mod 5).
    let lhs = x0.mul(&fp5(3), &ell); // x0 * 3
    let rhs = x1.mul(&fp5(1), &ell); // x1 * 1
    assert_eq!(
        lhs, rhs,
        "virtual logs must satisfy x0 * log_2(3) ≡ x1 * log_2(2) (mod 5): \
         x0={:?}, x1={:?}",
        x0.to_uint(),
        x1.to_uint()
    );
}

/// End-to-end toy-F_p DL KAT via block Wiedemann.
///
/// Same as `kat_toy_fp_dl_lanczos` but using `block_wiedemann_fl`. Cross-validates
/// that both solvers recover consistent virtual logs.
#[test]
fn kat_toy_fp_dl_wiedemann() {
    let ell = ell5();
    let dl_matrix = build_toy_dl_matrix();

    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);
    let op = FlMatrixOperator::<FpNaive4, 4>::new(&fl_matrix);

    // Solve with block Wiedemann.
    let mut found_solution: Option<FlSolution<FpNaive4>> = None;
    for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        let solutions = block_wiedemann_fl::<FpNaive4, 4>(&op, &ell, seed);
        for sol in solutions {
            if sol.coefficients.iter().any(|c| !c.is_zero(&ell)) {
                found_solution = Some(sol);
                break;
            }
        }
        if found_solution.is_some() {
            break;
        }
    }

    let sol = found_solution.expect("Wiedemann should find a nontrivial kernel vector");

    // Verify A·x = 0 mod ℓ.
    let x_bv = FlBlockVec::<FpNaive4, 4>::from_columns(&[sol.coefficients.clone()], &ell);
    let ax = op.apply(&x_bv, &ell);
    for r in 0..3 {
        assert!(
            ax.data[r][0].is_zero(&ell),
            "A·x[{r}] should be 0 for the kernel vector"
        );
    }

    // Recover virtual logs.
    let table: VirtualLogTable<FpNaive4> = recover_virtual_logs(&sol, 2, 0);

    let x0 = &table.rational_logs[0]; // virtual log of prime 2
    let x1 = &table.rational_logs[1]; // virtual log of prime 3

    assert!(!x0.is_zero(&ell), "virtual log of prime 2 should be nonzero");
    assert!(!x1.is_zero(&ell), "virtual log of prime 3 should be nonzero");

    // Cross-check: x0 * 3 ≡ x1 * 1 (mod 5).
    let lhs = x0.mul(&fp5(3), &ell);
    let rhs = x1.mul(&fp5(1), &ell);
    assert_eq!(
        lhs, rhs,
        "virtual logs must satisfy x0 * log_2(3) ≡ x1 * log_2(2) (mod 5): \
         x0={:?}, x1={:?}",
        x0.to_uint(),
        x1.to_uint()
    );
}

/// End-to-end KAT: Lanczos and Wiedemann recover consistent virtual logs.
///
/// Both solvers must find kernel vectors whose virtual-log ratios agree.
/// This cross-validates that the two independent implementations produce
/// consistent results on the same toy DL problem.
#[test]
fn kat_toy_fp_dl_lanczos_wiedemann_consistent() {
    let ell = ell5();
    let dl_matrix = build_toy_dl_matrix();

    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);
    let op = FlMatrixOperator::<FpNaive4, 4>::new(&fl_matrix);

    // Collect kernel vectors from both solvers.
    let mut lanczos_sol: Option<FlSolution<FpNaive4>> = None;
    let mut wiedemann_sol: Option<FlSolution<FpNaive4>> = None;

    for seed in [0u64, 1, 2, 3, 42, 137, 999] {
        if lanczos_sol.is_none() {
            let sols = block_lanczos_fl::<FpNaive4, 4>(&op, &ell, seed);
            for sol in sols {
                if sol.coefficients.iter().any(|c| !c.is_zero(&ell)) {
                    lanczos_sol = Some(sol);
                    break;
                }
            }
        }
        if wiedemann_sol.is_none() {
            let sols = block_wiedemann_fl::<FpNaive4, 4>(&op, &ell, seed);
            for sol in sols {
                if sol.coefficients.iter().any(|c| !c.is_zero(&ell)) {
                    wiedemann_sol = Some(sol);
                    break;
                }
            }
        }
        if lanczos_sol.is_some() && wiedemann_sol.is_some() {
            break;
        }
    }

    let l_sol = lanczos_sol.expect("Lanczos should find a kernel vector");
    let w_sol = wiedemann_sol.expect("Wiedemann should find a kernel vector");

    // Both solutions must be in the kernel.
    for (name, sol) in [("Lanczos", &l_sol), ("Wiedemann", &w_sol)] {
        let x_bv = FlBlockVec::<FpNaive4, 4>::from_columns(&[sol.coefficients.clone()], &ell);
        let ax = op.apply(&x_bv, &ell);
        for r in 0..3 {
            assert!(
                ax.data[r][0].is_zero(&ell),
                "{name}: A·x[{r}] should be 0"
            );
        }
    }

    // Both must satisfy the virtual-log ratio condition x0 * 3 ≡ x1 * 1 (mod 5).
    for (name, sol) in [("Lanczos", &l_sol), ("Wiedemann", &w_sol)] {
        let table: VirtualLogTable<FpNaive4> = recover_virtual_logs(sol, 2, 0);
        let x0 = &table.rational_logs[0];
        let x1 = &table.rational_logs[1];
        let lhs = x0.mul(&fp5(3), &ell);
        let rhs = x1.mul(&fp5(1), &ell);
        assert_eq!(
            lhs, rhs,
            "{name}: virtual logs must satisfy x0 * 3 ≡ x1 (mod 5): \
             x0={:?}, x1={:?}",
            x0.to_uint(),
            x1.to_uint()
        );
    }
}

/// End-to-end KAT: verify the matrix build step produces the expected F_ℓ values.
///
/// The toy DLMatrix has rows [3,4], [4,2], [2,1] with ℓ = 5.
/// After `build_fl_matrix`, the F_ℓ matrix should have the same values (all < 5).
#[test]
fn kat_toy_fp_dl_matrix_build() {
    let ell = ell5();
    let dl_matrix = build_toy_dl_matrix();

    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);

    assert_eq!(fl_matrix.rows.len(), 3, "should have 3 rows");
    assert_eq!(fl_matrix.num_cols, 2, "should have 2 columns");

    // Helper: look up a value in a sparse row by column index.
    let get_entry = |row: &FlSparseRow<FpNaive4>, col: usize| -> Option<u64> {
        row.entries.iter().find(|&&(c, _)| c == col).map(|(_, v)| v.to_uint().as_words()[0])
    };

    // Row 0: [3, 4] mod 5 = [3, 4].
    let row0 = &fl_matrix.rows[0];
    assert_eq!(get_entry(row0, 0), Some(3), "row 0, col 0: 3 mod 5 = 3");
    assert_eq!(get_entry(row0, 1), Some(4), "row 0, col 1: 4 mod 5 = 4");

    // Row 1: [4, 2] mod 5 = [4, 2].
    let row1 = &fl_matrix.rows[1];
    assert_eq!(get_entry(row1, 0), Some(4), "row 1, col 0: 4 mod 5 = 4");
    assert_eq!(get_entry(row1, 1), Some(2), "row 1, col 1: 2 mod 5 = 2");

    // Row 2: [2, 1] mod 5 = [2, 1].
    let row2 = &fl_matrix.rows[2];
    assert_eq!(get_entry(row2, 0), Some(2), "row 2, col 0: 2 mod 5 = 2");
    assert_eq!(get_entry(row2, 1), Some(1), "row 2, col 1: 1 mod 5 = 1");
}

/// End-to-end KAT: verify the known kernel vector [1, 3] is in the kernel.
///
/// The true virtual-log vector [1, 3] (log_2(2)=1, log_2(3)=3 mod 5) must satisfy
/// M * [1, 3]^T = 0 mod 5 for the toy DLMatrix.
#[test]
fn kat_toy_fp_dl_known_kernel_vector() {
    let ell = ell5();
    let dl_matrix = build_toy_dl_matrix();

    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell);
    let op = FlMatrixOperator::<FpNaive4, 4>::new(&fl_matrix);

    // The true virtual-log vector: log_2(2) = 1, log_2(3) = 3 (mod 5).
    let x = FlBlockVec::<FpNaive4, 4>::from_columns(
        &[vec![fp5(1), fp5(3)]],
        &ell,
    );

    // Verify M * [1, 3]^T = 0 mod 5.
    let mx = op.apply(&x, &ell);
    for r in 0..3 {
        assert!(
            mx.data[r][0].is_zero(&ell),
            "M * [1, 3]^T should be 0 at row {r}: got {:?}",
            mx.data[r][0].to_uint()
        );
    }
}

// ─── PARI cross-check stub ────────────────────────────────────────────────────

/// PARI cross-check: verify the toy DL result against PARI's discrete log.
///
/// Run manually: `pari -q -e "znlog(Mod(3, 11), Mod(2, 11))"`
/// Expected output: 8 (= log_2(3) mod 10). Reduced mod 5: 8 mod 5 = 3.
///
/// The recovered virtual log of prime 3 should be 3 (mod 5) or a nonzero scalar
/// multiple (since the kernel is determined up to scalar).
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn kat_pari_dl_oracle() {
    // Cross-check the toy-F_p DL result against PARI's discrete log.
    // Run manually: pari -q -e "znlog(Mod(3, 11), Mod(2, 11))"
    // Expected: 8 (log_2(3) mod 10). Reduced mod 5: 3.
    //
    // The virtual log of prime 3 recovered by the solver should be 3 (mod 5)
    // or a nonzero scalar multiple c*3 mod 5 (for some c ≠ 0).
    // Equivalently, the ratio (virtual log of 2) / (virtual log of 3) = 1/3 = 2 mod 5.
    todo!("PARI cross-check: run manually")
}
