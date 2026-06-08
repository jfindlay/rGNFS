//! Known-answer tests (KATs) for G.F.2: rational square root from a kernel vector.
//!
//! Three KATs are required by the G.F.2 session spec:
//!
//! - **KAT (a) — Algebraic correctness**: for a hand-built relation set whose rational-norm
//!   product is a known perfect square, the recovered X satisfies X² ≡ product (mod N).
//!
//! - **KAT (b) — Correct index set S**: the product is computed over the correct S, matching
//!   a hand-traced `expand_provenance` on a small SparseMatrix with known provenance.
//!
//! - **KAT (c) — Non-square triggers panic**: a deliberately non-square product triggers the
//!   `isqrt = None` upstream-bug path (verified via `#[should_panic]`).
//!
//! # Toy setup
//!
//! All KATs use N = 35, m = 6. Relations are constructed directly (bypassing smoothness checks)
//! using the `Relation` struct fields. The SparseMatrix is built by hand with explicit provenance.
//!
//! ## KAT (a) scenario
//!
//! Relations:
//! - rel[0]: (a=4, b=0) → rational factor = 4 − 0·6 = 4
//! - rel[1]: (a=9, b=0) → rational factor = 9 − 0·6 = 9
//!
//! Product = 4 · 9 = 36 = 6².
//! X = isqrt(36) = 6.
//! X mod 35 = 6.
//! X² mod 35 = 36 mod 35 = 1.
//! product mod 35 = 36 mod 35 = 1. ✓
//!
//! ## KAT (b) scenario
//!
//! SparseMatrix with 2 rows:
//! - row 0: provenance = [0]
//! - row 1: provenance = [1]
//!
//! KernelVector with row_indices = [0, 1].
//! expand_provenance gives symmetric difference of {0} and {1} = {0, 1}.
//! So S = [0, 1] — both relations are used.
//!
//! ## KAT (c) scenario
//!
//! Relations:
//! - rel[0]: (a=2, b=0) → rational factor = 2 (not a perfect square alone)
//! - rel[1]: (a=3, b=0) → rational factor = 3
//!
//! Product = 2 · 3 = 6 (not a perfect square). `rational_sqrt` must panic.

use gnfs::{
    ExponentVector, PolyPair, Relation,
    filter::{MatrixRow, SparseMatrix},
    linalg::KernelVector,
    sqrt::rational_sqrt,
};
use num_bigint::BigInt;
use shared_numfield::IntPoly;

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// Construct a `Relation` directly from field values (bypasses smoothness checks).
fn make_relation(a: i64, b: i64, rational_sign: bool) -> Relation {
    Relation {
        a: bi(a),
        b: bi(b),
        rational_exponents: ExponentVector { entries: vec![] },
        algebraic_exponents: ExponentVector { entries: vec![] },
        rational_sign,
    }
}

/// Build a `PolyPair` with N = 35, m = 6.
///
/// Uses f(x) = x − 6 (degree 1, coeffs [−6, 1]) as a placeholder algebraic polynomial.
/// The KATs only read `poly.m` and `poly.n`; the polynomial itself is not evaluated.
fn make_poly_pair(n: i64, m: i64) -> PolyPair {
    // f(x) = x − m (degree 1): coeffs [−m, 1] least-significant first.
    let f = IntPoly::from_coeffs(vec![bi(-m), bi(1)]);
    // g(x) = x − m: same form.
    let g = IntPoly::from_coeffs(vec![bi(-m), bi(1)]);
    PolyPair::new(f, g, bi(m), bi(n))
}

/// Build a `SparseMatrix` with the given rows (cols, provenance) and num_cols.
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

// ─── KAT (a): Algebraic correctness ──────────────────────────────────────────

/// KAT (a): X² ≡ product (mod N) for a hand-built perfect-square product.
///
/// Relations: (a=4, b=0) and (a=9, b=0).
/// Product = 4 · 9 = 36 = 6². X = 6. X² mod 35 = 36 mod 35 = 1 = product mod 35.
#[test]
fn kat_a_x_squared_congruent_to_product_mod_n() {
    let poly = make_poly_pair(35, 6);
    let relations = vec![
        make_relation(4, 0, false), // factor = 4 − 0·6 = 4 (positive)
        make_relation(9, 0, false), // factor = 9 − 0·6 = 9 (positive)
    ];
    // Matrix: 2 rows, each with provenance pointing to its own relation.
    // KernelVector selects both rows → S = {0, 1}.
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]), // row 0: col 0, provenance [0]
            (vec![0], vec![1]), // row 1: col 0, provenance [1]
        ],
        1,
    );
    // KernelVector: both rows selected (their GF(2) XOR is {0} XOR {0} = {} — valid nullspace).
    let kv = KernelVector::new(vec![0, 1]);

    let x = rational_sqrt(&kv, &matrix, &relations, &poly);

    // X should be 6 (isqrt(36) = 6, 6 mod 35 = 6).
    assert_eq!(x, bi(6), "X = isqrt(36) mod 35 should be 6");

    // Verify X² ≡ product (mod N).
    let product = bi(36);
    let n = bi(35);
    let x_sq_mod_n = (&x * &x) % &n;
    let product_mod_n = &product % &n;
    assert_eq!(
        x_sq_mod_n, product_mod_n,
        "X² mod N should equal product mod N: {} ≡ {} (mod {})",
        x_sq_mod_n, product_mod_n, n
    );
}

// ─── KAT (b): Correct index set S ────────────────────────────────────────────

/// KAT (b): the product is computed over the correct S from `expand_provenance`.
///
/// Uses a matrix with explicit provenance and verifies that only the correct relations
/// contribute to the product.
///
/// Setup:
/// - 3 relations: rel[0] factor=4, rel[1] factor=9, rel[2] factor=25.
/// - Matrix row 0 has provenance [0, 2] (covers rel[0] and rel[2]).
/// - Matrix row 1 has provenance [1, 2] (covers rel[1] and rel[2]).
/// - KernelVector selects rows [0, 1].
/// - expand_provenance: symmetric diff of {0,2} and {1,2} = {0, 1} (2 cancels).
/// - So S = {0, 1}: product = 4 · 9 = 36. rel[2] (factor=25) is NOT included.
#[test]
fn kat_b_correct_index_set_from_expand_provenance() {
    let poly = make_poly_pair(35, 6);
    let relations = vec![
        make_relation(4, 0, false),  // rel[0]: factor = 4
        make_relation(9, 0, false),  // rel[1]: factor = 9
        make_relation(25, 0, false), // rel[2]: factor = 25 (should NOT be included)
    ];
    // Matrix: 2 rows with overlapping provenance.
    // Row 0: provenance [0, 2] — covers rel[0] and rel[2].
    // Row 1: provenance [1, 2] — covers rel[1] and rel[2].
    // Symmetric difference: {0,2} XOR {1,2} = {0,1} (2 cancels).
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0, 2]), // row 0: col 0, provenance [0, 2]
            (vec![0], vec![1, 2]), // row 1: col 0, provenance [1, 2]
        ],
        1,
    );
    let kv = KernelVector::new(vec![0, 1]);

    // Verify expand_provenance gives {0, 1} (not {0, 1, 2}).
    let s = kv.expand_provenance(&matrix);
    assert_eq!(s, vec![0, 1], "expand_provenance should give {{0, 1}} (rel[2] cancels)");

    // Compute rational_sqrt: product = 4 · 9 = 36 (rel[2] excluded), X = 6.
    let x = rational_sqrt(&kv, &matrix, &relations, &poly);
    assert_eq!(x, bi(6), "X should be 6 (product = 4·9 = 36, not 4·9·25 = 900)");

    // Cross-check: if rel[2] were included, product = 4·9·25 = 900, X = 30.
    // Verify we did NOT get 30 (which would indicate rel[2] was wrongly included).
    assert_ne!(x, bi(30), "X must not be 30 (that would mean rel[2] was wrongly included)");
}

// ─── KAT (c): Non-square product triggers panic ───────────────────────────────

/// KAT (c): a deliberately non-square product triggers the `isqrt = None` upstream-bug path.
///
/// Relations: (a=2, b=0) and (a=3, b=0).
/// Product = 2 · 3 = 6 (not a perfect square).
/// `rational_sqrt` must panic with a clear message.
#[test]
#[should_panic(expected = "rational norm product is not a perfect square")]
fn kat_c_non_square_product_panics() {
    let poly = make_poly_pair(35, 6);
    let relations = vec![
        make_relation(2, 0, false), // factor = 2
        make_relation(3, 0, false), // factor = 3
    ];
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]), // row 0: provenance [0]
            (vec![0], vec![1]), // row 1: provenance [1]
        ],
        1,
    );
    let kv = KernelVector::new(vec![0, 1]);

    // Product = 2 · 3 = 6, not a perfect square. This must panic.
    let _ = rational_sqrt(&kv, &matrix, &relations, &poly);
}

// ─── Bonus: single-relation perfect square ────────────────────────────────────

/// Bonus: a single relation with a perfect-square factor works correctly.
///
/// rel[0]: (a=25, b=0) → factor = 25 = 5². X = 5. X mod 35 = 5.
#[test]
fn single_relation_perfect_square() {
    let poly = make_poly_pair(35, 6);
    let relations = vec![
        make_relation(25, 0, false), // factor = 25 = 5²
    ];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let x = rational_sqrt(&kv, &matrix, &relations, &poly);
    assert_eq!(x, bi(5), "X = isqrt(25) mod 35 = 5");

    let n = bi(35);
    let x_sq_mod_n = (&x * &x) % &n;
    let product_mod_n = bi(25) % &n;
    assert_eq!(x_sq_mod_n, product_mod_n, "X² mod N = product mod N");
}

// ─── Bonus: m-dependent factors ──────────────────────────────────────────────

/// Bonus: relations with b ≠ 0 use the correct m-dependent factor a − b·m.
///
/// N = 35, m = 6.
/// rel[0]: (a=10, b=1) → factor = 10 − 1·6 = 4.
/// rel[1]: (a=15, b=1) → factor = 15 − 1·6 = 9.
/// Product = 4 · 9 = 36 = 6². X = 6.
#[test]
fn m_dependent_factors_correct() {
    let poly = make_poly_pair(35, 6);
    let relations = vec![
        make_relation(10, 1, false), // factor = 10 − 1·6 = 4
        make_relation(15, 1, false), // factor = 15 − 1·6 = 9
    ];
    let matrix = make_matrix(
        vec![(vec![0], vec![0]), (vec![0], vec![1])],
        1,
    );
    let kv = KernelVector::new(vec![0, 1]);

    let x = rational_sqrt(&kv, &matrix, &relations, &poly);
    assert_eq!(x, bi(6), "X = isqrt((10−6)·(15−6)) = isqrt(36) mod 35 = 6");
}
