//! Known-answer tests (KATs) for the algebraic square root via Couveignes' CRT algorithm.
//!
//! Three KATs:
//!
//! - **KAT (a) — Known-square test**: For a hand-built K and a γ that IS a known square β²
//!   in K, Couveignes recovers β (up to sign) and |Norm(β)| matches the hand-computed Y.
//!
//! - **KAT (b) — Congruence test**: The recovered Y satisfies X² ≡ Y² (mod N) for a toy N,
//!   where X is from `rational_sqrt` on the same kernel vector.
//!
//! - **KAT (c) — Determinism**: For a fixed γ and prime set, `algebraic_sqrt` returns the
//!   same Y on repeated calls.
//!
//! # Toy setup
//!
//! ## KAT (a) — Known-square test
//!
//! K = ℚ(√2), f = x² − 2 (monic, degree 2). N = 7, m = 3 (f(3) = 9 − 2 = 7 ≡ 0 mod 7).
//!
//! One relation: (a=9, b=0). Algebraic factor: 9 − 0·α = 9 (rational).
//! γ = 9 = 3². β = 3 (rational). Norm(3) = 3² = 9 (degree-2 field, Norm(r) = r^d).
//! Y = |Norm(β)| mod N = 9 mod 7 = 2.
//!
//! ## KAT (b) — Congruence test
//!
//! K = ℚ (trivial degree-1 field), f = x − 3 (monic, degree 1). N = 7, m = 3.
//! f(3) = 3 − 3 = 0 ≡ 0 mod 7. ✓
//!
//! One relation: (a=9, b=0). Rational factor: 9 − 0·3 = 9. Algebraic factor: 9 − 0·α = 9.
//! X = isqrt(9) = 3. X mod 7 = 3.
//! γ = 9 = 3². β = 3. Norm(3) = 3 (degree-1 field). Y = 3 mod 7 = 3.
//! X² mod 7 = 9 mod 7 = 2. Y² mod 7 = 9 mod 7 = 2. X² ≡ Y² (mod 7). ✓
//!
//! ## KAT (c) — Determinism
//!
//! Same setup as KAT (a). Two calls to `algebraic_sqrt` with identical inputs return the same Y.

use gnfs::{
    ExponentVector, PolyPair, Relation,
    filter::{MatrixRow, SparseMatrix},
    linalg::KernelVector,
    sqrt::{algebraic_sqrt, rational_sqrt},
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

/// Build a `PolyPair` with the given f, m, n.
///
/// g is always x − m (the rational polynomial).
fn make_poly_pair_with_f(f: IntPoly, m: i64, n: i64) -> PolyPair {
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

// ─── KAT (a): Known-square test ───────────────────────────────────────────────

/// KAT (a): For a hand-built K and a γ that IS a known square β², Couveignes recovers β
/// (up to sign) and |Norm(β)| matches the hand-computed Y.
///
/// Setup:
/// - K = ℚ(√2), f = x² − 2 (monic, degree 2).
/// - N = 7, m = 3 (f(3) = 7 ≡ 0 mod 7).
/// - One relation: (a=9, b=0). γ = 9 = 3². β = 3. Norm(3) = 9. Y = 9 mod 7 = 2.
#[test]
fn kat_a_known_square_norm_matches() {
    // f = x² − 2 (monic). f(3) = 9 − 2 = 7 ≡ 0 (mod 7). ✓
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);

    // One relation: a=9, b=0. Algebraic factor = 9 − 0·α = 9 (rational).
    // γ = 9 = 3². β = 3. Norm(3) in ℚ(√2) = 3² = 9. Y = 9 mod 7 = 2.
    let relations = vec![make_relation(9, 0, false)];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let y = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    // Y = |Norm(β)| mod N = 9 mod 7 = 2.
    assert_eq!(y, bi(2), "Y = |Norm(3)| mod 7 = 9 mod 7 = 2, got {y}");
}

/// KAT (a) extended: verify that Y² ≡ Norm(γ) (mod N).
///
/// Norm(γ) = Norm(9) = 9² = 81 (degree-2 field). Y = 2. Y² = 4. 81 mod 7 = 4. ✓
#[test]
fn kat_a_y_squared_congruent_to_norm_gamma() {
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);
    let relations = vec![make_relation(9, 0, false)];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let y = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    // Norm(γ) = Norm(9) = 9^2 = 81 (degree 2). 81 mod 7 = 4. Y² mod 7 = 4. ✓
    let n = bi(7);
    let y_sq_mod_n = (&y * &y) % &n;
    let norm_gamma_mod_n = bi(81) % &n; // Norm(9) = 9^2 = 81 for degree-2 field
    assert_eq!(
        y_sq_mod_n, norm_gamma_mod_n,
        "Y² mod N should equal Norm(γ) mod N: {} ≡ {} (mod {})",
        y_sq_mod_n, norm_gamma_mod_n, n
    );
}

// ─── KAT (b): Congruence test ─────────────────────────────────────────────────

/// KAT (b): The recovered Y satisfies X² ≡ Y² (mod N) for a toy N.
///
/// Setup:
/// - K = ℚ (trivial degree-1 field), f = x − 3 (monic, degree 1).
/// - N = 7, m = 3 (f(3) = 0 ≡ 0 mod 7).
/// - One relation: (a=9, b=0).
/// - Rational: X = isqrt(9) = 3. X mod 7 = 3.
/// - Algebraic: γ = 9, β = 3, Norm(3) = 3 (degree-1 field). Y = 3 mod 7 = 3.
/// - X² mod 7 = 9 mod 7 = 2. Y² mod 7 = 9 mod 7 = 2. X² ≡ Y² (mod 7). ✓
///
/// For a degree-1 field, Norm(a − b·α) = a − b·m, so the rational and algebraic products
/// are identical. This guarantees X = Y and hence X² ≡ Y² (mod N) exactly.
#[test]
fn kat_b_x_squared_congruent_to_y_squared_mod_n() {
    // f = x − 3 (degree 1, monic). f(3) = 0 ≡ 0 (mod 7). ✓
    let f = IntPoly::from_coeffs(vec![bi(-3), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);

    let relations = vec![make_relation(9, 0, false)];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let x = rational_sqrt(&kv, &matrix, &relations, &poly);
    let y = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    let n = bi(7);
    let x_sq_mod_n = (&x * &x) % &n;
    let y_sq_mod_n = (&y * &y) % &n;

    assert_eq!(
        x_sq_mod_n, y_sq_mod_n,
        "X² ≡ Y² (mod N): X={x}, Y={y}, X²={x_sq_mod_n}, Y²={y_sq_mod_n} (mod {n})"
    );
}

/// KAT (b) extended: multi-relation congruence test.
///
/// Two relations: (a=4, b=0) and (a=9, b=0).
/// Rational product: 4 · 9 = 36. X = isqrt(36) = 6. X mod 7 = 6.
/// Algebraic: γ = 4 · 9 = 36 (degree-1 field, factors are rational). β = 6. Norm(6) = 6.
/// Y = 6 mod 7 = 6. X = Y = 6. X² mod 7 = 36 mod 7 = 1. Y² mod 7 = 36 mod 7 = 1. ✓
#[test]
fn kat_b_multi_relation_congruence() {
    // f = x − 3 (degree 1, monic). N = 7, m = 3.
    let f = IntPoly::from_coeffs(vec![bi(-3), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);

    let relations = vec![
        make_relation(4, 0, false),
        make_relation(9, 0, false),
    ];
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]),
            (vec![0], vec![1]),
        ],
        1,
    );
    let kv = KernelVector::new(vec![0, 1]);

    let x = rational_sqrt(&kv, &matrix, &relations, &poly);
    let y = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    let n = bi(7);
    let x_sq_mod_n = (&x * &x) % &n;
    let y_sq_mod_n = (&y * &y) % &n;

    assert_eq!(
        x_sq_mod_n, y_sq_mod_n,
        "X² ≡ Y² (mod N): X={x}, Y={y}, X²={x_sq_mod_n}, Y²={y_sq_mod_n} (mod {n})"
    );
}

// ─── KAT (c): Determinism ─────────────────────────────────────────────────────

/// KAT (c): For a fixed γ and prime set, `algebraic_sqrt` returns the same Y on repeated calls.
///
/// Uses the same setup as KAT (a). Two calls with identical inputs must return the same Y.
#[test]
fn kat_c_determinism() {
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);
    let relations = vec![make_relation(9, 0, false)];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let y1 = algebraic_sqrt(&kv, &matrix, &relations, &poly);
    let y2 = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    assert_eq!(y1, y2, "algebraic_sqrt must be deterministic: first call={y1}, second call={y2}");
}

/// KAT (c) extended: determinism with a degree-1 field.
#[test]
fn kat_c_determinism_degree1() {
    let f = IntPoly::from_coeffs(vec![bi(-3), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);
    let relations = vec![make_relation(9, 0, false)];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);

    let y1 = algebraic_sqrt(&kv, &matrix, &relations, &poly);
    let y2 = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    assert_eq!(y1, y2, "algebraic_sqrt must be deterministic");
}

// ─── Bonus: degree-2 field with non-trivial b ─────────────────────────────────

/// Bonus: K = ℚ(√2), two relations with b=0, product = 4·9 = 36 = 6².
///
/// β = 6 (rational). Norm(6) = 36 (degree-2 field). Y = 36 mod 7 = 1.
#[test]
fn bonus_degree2_two_relations() {
    // f = x² − 2 (monic). N = 7, m = 3.
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair_with_f(f, 3, 7);

    let relations = vec![
        make_relation(4, 0, false),
        make_relation(9, 0, false),
    ];
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]),
            (vec![0], vec![1]),
        ],
        1,
    );
    let kv = KernelVector::new(vec![0, 1]);

    let y = algebraic_sqrt(&kv, &matrix, &relations, &poly);

    // γ = 4 · 9 = 36 = 6². β = 6. Norm(6) = 36 (degree-2). Y = 36 mod 7 = 1.
    let n = bi(7);
    let expected_y = bi(36) % &n; // = 1
    assert_eq!(y, expected_y, "Y = |Norm(6)| mod 7 = 36 mod 7 = 1, got {y}");
}
