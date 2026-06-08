//! Known-answer tests (KATs) for G.F.4: assembly + end-to-end factor driver.
//!
//! Three KATs are required by the G.F.4 session spec:
//!
//! - **KAT (a) — Deterministic end-to-end**: for a small hand-prepared N with a known matrix
//!   + kernel vector, the driver recovers a known non-trivial factor of N. No external oracle
//!   — carries reproducibility.
//!
//! - **KAT (b) — Trivial-gcd path**: the trivial-gcd path (X ≡ ±Y mod N) is detected and the
//!   retry loop advances to the next kernel vector.
//!
//! - **KAT (c) — Oracle KAT** (gated with `#[ignore]`): factor a published challenge in the
//!   80–100-bit range. Ignored by default; KAT (a) carries reproducibility.
//!
//! # Toy setup
//!
//! ## KAT (a) — Deterministic end-to-end
//!
//! N = 35 = 5 × 7. K = ℚ(√2), f = x² − 2. m = 6.
//!
//! Relations:
//! - rel[0]: (a=4, b=0) → rational factor = 4, algebraic factor = 4 (rational in K).
//! - rel[1]: (a=9, b=0) → rational factor = 9, algebraic factor = 9 (rational in K).
//!
//! Kernel vector selects both relations:
//! - Rational product: 4 × 9 = 36 = 6². X = isqrt(36) = 6. X mod 35 = 6.
//! - Algebraic: γ = 4 × 9 = 36 = 6² in K. β = 6 (rational). Norm(6) = 6² = 36 (degree-2).
//!   Y = 36 mod 35 = 1.
//! - gcd(X − Y, N) = gcd(5, 35) = 5. Non-trivial factor: 5. ✓
//!
//! ## KAT (b) — Trivial-gcd path + retry loop
//!
//! Same N = 35, f = x² − 2, m = 6.
//!
//! Three relations:
//! - rel[0]: (a=1, b=0) → rational factor = 1, algebraic factor = 1.
//! - rel[1]: (a=4, b=0) → rational factor = 4.
//! - rel[2]: (a=9, b=0) → rational factor = 9.
//!
//! Two kernel vectors:
//! - KV1 = {row 0} → S = {0} → product = 1 → X = 1, Y = 1. gcd(0, 35) = 35 (trivial). Skip.
//! - KV2 = {row 1, row 2} → S = {1, 2} → product = 36 → X = 6, Y = 1. gcd(5, 35) = 5. ✓
//!
//! The driver must skip KV1 (trivial gcd) and return 5 from KV2.

use gnfs::{
    ExponentVector, PolyPair, Relation,
    filter::{MatrixRow, SparseMatrix},
    linalg::KernelVector,
    sqrt::{factor, factor_from_congruence},
};
use num_bigint::BigInt;
use shared_numfield::IntPoly;

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// Construct a `Relation` directly from field values (bypasses smoothness checks).
fn make_relation(a: i64, b: i64) -> Relation {
    Relation {
        a: bi(a),
        b: bi(b),
        rational_exponents: ExponentVector { entries: vec![] },
        algebraic_exponents: ExponentVector { entries: vec![] },
        rational_sign: false,
    }
}

/// Build a `PolyPair` with the given f, m, n.
///
/// g is always x − m (the rational polynomial).
fn make_poly_pair(f: IntPoly, m: i64, n: i64) -> PolyPair {
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

// ─── KAT (a): Deterministic end-to-end ───────────────────────────────────────

/// KAT (a): `factor_from_congruence` with known X, Y, N returns the expected factor.
///
/// N = 35, X = 6, Y = 1. X − Y = 5. gcd(5, 35) = 5. Non-trivial factor: 5. ✓
#[test]
fn kat_a_factor_from_congruence_known_values() {
    let n = bi(35);
    let x = bi(6);
    let y = bi(1);
    let f = factor_from_congruence(&x, &y, &n).expect("should find a non-trivial factor");
    assert_eq!(f, bi(5), "gcd(6 − 1, 35) = gcd(5, 35) = 5");
    assert!(f > bi(1) && f < n, "factor must be non-trivial: 1 < {f} < {n}");
}

/// KAT (a) extended: the full `factor` driver recovers a known non-trivial factor of N = 35.
///
/// Setup:
/// - N = 35 = 5 × 7. K = ℚ(√2), f = x² − 2. m = 6.
/// - Relations: (a=4, b=0) and (a=9, b=0).
/// - Kernel vector selects both relations.
/// - Rational product: 4 × 9 = 36 = 6². X = 6. X mod 35 = 6.
/// - Algebraic: γ = 36 = 6² in K. β = 6. Norm(6) = 36 (degree-2). Y = 36 mod 35 = 1.
/// - gcd(X − Y, N) = gcd(5, 35) = 5. ✓
#[test]
fn kat_a_factor_driver_recovers_known_factor() {
    // f = x² − 2 (monic, degree 2). K = ℚ(√2).
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair(f, 6, 35);

    let relations = vec![
        make_relation(4, 0), // rational factor = 4, algebraic factor = 4 (rational in K)
        make_relation(9, 0), // rational factor = 9, algebraic factor = 9 (rational in K)
    ];

    // Matrix: 2 rows, each with provenance pointing to its own relation.
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]), // row 0: provenance [0]
            (vec![0], vec![1]), // row 1: provenance [1]
        ],
        1,
    );

    // Kernel vector selects both rows → S = {0, 1}.
    let kv = KernelVector::new(vec![0, 1]);
    let kernel_vectors = vec![kv];

    let result = factor(&poly, &matrix, &relations, &kernel_vectors);
    let found = result.expect("factor driver should find a non-trivial factor of 35");

    // The factor must be non-trivial (5 or 7).
    let n = bi(35);
    assert!(
        found > bi(1) && found < n,
        "factor must be non-trivial: 1 < {found} < {n}"
    );
    assert!(
        found == bi(5) || found == bi(7),
        "factor of 35 must be 5 or 7, got {found}"
    );
}

// ─── KAT (b): Trivial-gcd path + retry loop ──────────────────────────────────

/// KAT (b) part 1: `factor_from_congruence` returns `None` when X = Y (trivial gcd).
///
/// N = 35, X = 6, Y = 6. X − Y = 0. gcd(0, 35) = 35 (trivial). X + Y = 12. gcd(12, 35) = 1.
/// Both gcds are trivial → `None`.
#[test]
fn kat_b_trivial_gcd_x_equals_y() {
    let n = bi(35);
    let x = bi(6);
    let y = bi(6);
    assert!(
        factor_from_congruence(&x, &y, &n).is_none(),
        "X = Y should give trivial gcd (None)"
    );
}

/// KAT (b) part 2: `factor_from_congruence` returns `None` when X ≡ −Y (mod N).
///
/// N = 35, X = 6, Y = 29. X + Y = 35 ≡ 0 (mod 35). gcd(35, 35) = 35 (trivial).
/// X − Y = −23. gcd(23, 35) = 1 (trivial). Both gcds are trivial → `None`.
#[test]
fn kat_b_trivial_gcd_x_equals_neg_y_mod_n() {
    let n = bi(35);
    let x = bi(6);
    let y = bi(29); // 6 + 29 = 35 ≡ 0 (mod 35)
    assert!(
        factor_from_congruence(&x, &y, &n).is_none(),
        "X ≡ −Y (mod N) should give trivial gcd (None)"
    );
}

/// KAT (b) part 3: the retry loop advances past a trivial-gcd kernel vector to find a factor.
///
/// Setup:
/// - N = 35 = 5 × 7. K = ℚ(√2), f = x² − 2. m = 6.
/// - Three relations: rel[0]=(a=1,b=0), rel[1]=(a=4,b=0), rel[2]=(a=9,b=0).
/// - KV1 = {row 0} → S = {0} → product = 1 → X = 1, Y = 1. gcd(0, 35) = 35 (trivial). Skip.
/// - KV2 = {row 1, row 2} → S = {1, 2} → product = 36 → X = 6, Y = 1. gcd(5, 35) = 5. ✓
///
/// The driver must skip KV1 and return 5 from KV2.
#[test]
fn kat_b_retry_loop_skips_trivial_and_finds_factor() {
    // f = x² − 2 (monic, degree 2). K = ℚ(√2). N = 35, m = 6.
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair(f, 6, 35);

    let relations = vec![
        make_relation(1, 0), // rel[0]: factor = 1 (product = 1, X = 1, Y = 1 → trivial)
        make_relation(4, 0), // rel[1]: factor = 4
        make_relation(9, 0), // rel[2]: factor = 9
    ];

    // Matrix: 3 rows, each with provenance pointing to its own relation.
    let matrix = make_matrix(
        vec![
            (vec![0], vec![0]), // row 0: provenance [0] → rel[0]
            (vec![0], vec![1]), // row 1: provenance [1] → rel[1]
            (vec![0], vec![2]), // row 2: provenance [2] → rel[2]
        ],
        1,
    );

    // KV1: selects row 0 → S = {0} → product = 1 → X = 1, Y = 1 → trivial gcd.
    let kv1 = KernelVector::new(vec![0]);
    // KV2: selects rows 1 and 2 → S = {1, 2} → product = 36 → X = 6, Y = 1 → factor = 5.
    let kv2 = KernelVector::new(vec![1, 2]);
    let kernel_vectors = vec![kv1, kv2];

    let result = factor(&poly, &matrix, &relations, &kernel_vectors);
    let found = result.expect("factor driver should find a non-trivial factor after skipping KV1");

    let n = bi(35);
    assert!(
        found > bi(1) && found < n,
        "factor must be non-trivial: 1 < {found} < {n}"
    );
    assert!(
        found == bi(5) || found == bi(7),
        "factor of 35 must be 5 or 7, got {found}"
    );
}

/// KAT (b) part 4: `factor` returns `None` when all kernel vectors yield trivial gcds.
///
/// Setup: N = 35, f = x² − 2, m = 6. One relation (a=1, b=0): X = 1, Y = 1. Trivial.
/// With a single trivial kernel vector, `factor` must return `None`.
#[test]
fn kat_b_all_trivial_returns_none() {
    let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
    let poly = make_poly_pair(f, 6, 35);

    let relations = vec![
        make_relation(1, 0), // factor = 1 → product = 1 → X = 1, Y = 1 → trivial
    ];
    let matrix = make_matrix(vec![(vec![0], vec![0])], 1);
    let kv = KernelVector::new(vec![0]);
    let kernel_vectors = vec![kv];

    let result = factor(&poly, &matrix, &relations, &kernel_vectors);
    assert!(result.is_none(), "all trivial kernel vectors should return None");
}

// ─── KAT (c): Oracle KAT (ignored by default) ────────────────────────────────

/// KAT (c): Factor a published 80–100-bit challenge using the full GNFS pipeline.
///
/// This KAT requires CADO-NFS or msieve as an external oracle to verify the result.
/// It is ignored by default; KAT (a) carries reproducibility without the oracle.
///
/// To run manually:
/// ```
/// cargo test --test factor_end_to_end_kat kat_c_oracle -- --ignored
/// ```
#[test]
#[ignore = "CADO-NFS not installed; run manually when available to verify end-to-end \
            factorisation of 80-100 bit challenge"]
fn kat_c_oracle_80_100_bit_challenge() {
    // Placeholder: factor a published 80–100-bit semiprime using the full GNFS pipeline.
    // The pipeline stages (polyselect → sieve → filter → linalg → sqrt) must all be
    // wired end-to-end. This KAT is the ROADMAP G.F oracle KAT.
    //
    // Example target: RSA-100 (330 bits) is too large for toy scale; use a hand-chosen
    // 80-bit semiprime such as N = p × q where p and q are 40-bit primes.
    //
    // When CADO-NFS is available, verify the recovered factor against its output.
    todo!(
        "Oracle KAT: wire the full GNFS pipeline (polyselect → sieve → filter → linalg → sqrt) \
         and factor an 80–100-bit semiprime. Verify against CADO-NFS / msieve."
    );
}
