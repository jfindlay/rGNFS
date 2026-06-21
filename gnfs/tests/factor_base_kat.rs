//! Known-answer tests (KATs) for the sieve substrate: factor bases, norms, and relations.
//!
//! Three KATs:
//!
//! 1. **Factor-base construction KAT**: for ``f(x) = x³ − x − 1`` and ``B_alg = 30``, the
//!    algebraic factor base lists exactly the ``(p, r)`` pairs with ``f(r) ≡ 0 (mod p)``,
//!    cross-checked against brute-force root enumeration mod each ``p ≤ 30``.
//!
//! 2. **Norm reconstruction KAT**: for a known ``(a, b)``, ``rational_norm`` and
//!    ``algebraic_norm`` match hand-computed values; ``Relation::verify()`` holds for a
//!    hand-constructed smooth relation and fails when an exponent is perturbed.
//!
//! 3. **Norm bridge range KAT**: a toy-scale norm fits ``Uint<4>``; the bridge returns
//!    ``NormBridgeError::Overflow`` for a norm that exceeds 256 bits.
//!
//! # Polynomial used throughout
//!
//! ``f(x) = x³ − x − 1`` (coefficients: [−1, −1, 0, 1] least-significant first).
//! This is a standard NFS toy polynomial with discriminant −23.
//! ``disc(f) = −23``, so ``p = 23`` is the only bad prime ≤ 30.
//!
//! The polynomial pair uses ``m = 2``, ``n = f(2) = 8 − 2 − 1 = 5``.
//! (Toy scale: n = 5 is not a useful factoring target, but it is a valid polynomial pair
//! for testing the sieve substrate.)

use gnfs::{
    algebraic_norm, norm_sign, norm_to_uint, rational_norm, AlgebraicPrime,
    FactorBase, NormBridgeError, Relation, RelationError,
};
use num_bigint::BigInt;
use shared_numfield::IntPoly;
use shared_numth::{factor_base_up_to, trial_smooth, SmoothWitness};

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// f(x) = x³ − x − 1.
fn f_cubic() -> IntPoly {
    IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
}

/// Brute-force root enumeration: all r ∈ [0, p) with f(r) ≡ 0 (mod p).
fn brute_force_roots(f: &IntPoly, p: u64) -> Vec<u64> {
    let p_big = BigInt::from(p);
    let mut roots = Vec::new();
    for r in 0..p {
        let r_big = BigInt::from(r);
        let val = f.eval(&r_big);
        let rem = {
            let r = &val % &p_big;
            if r < BigInt::from(0i64) { r + &p_big } else { r }
        };
        if rem == BigInt::from(0i64) {
            roots.push(r);
        }
    }
    roots
}

// ─── KAT 1: Factor-base construction ─────────────────────────────────────────

/// KAT 1: Algebraic factor base for ``f(x) = x³ − x − 1`` with ``B_alg = 30``.
///
/// Verifies that the algebraic factor base lists exactly the ``(p, r)`` pairs with
/// ``f(r) ≡ 0 (mod p)`` for each prime ``p ≤ 30``, cross-checked against brute-force
/// root enumeration.
///
/// # Expected roots (hand-computed)
///
/// - p = 2: f(0) = −1 ≡ 1, f(1) = −1 ≡ 1 → no roots.
/// - p = 3: f(0) = −1 ≡ 2, f(1) = −1 ≡ 2, f(2) = 5 ≡ 2 → no roots.
/// - p = 5: f(0) = −1 ≡ 4, f(1) = −1 ≡ 4, f(2) = 5 ≡ 0, f(3) = 23 ≡ 3, f(4) = 59 ≡ 4
///   → root r = 2.
/// - p = 7: f(5) = 119 = 7×17 ≡ 0 → root r = 5.
/// - p = 11: check all r ∈ 0..11.
/// - p = 13: check all r ∈ 0..13.
/// - p = 17: f(5) = 119 = 7×17 ≡ 0 → root r = 5.
/// - p = 19: check all r ∈ 0..19.
/// - p = 23: f(r) mod 23 for r ∈ 0..23 (p = 23 is a bad prime: disc(f) = −23).
/// - p = 29: check all r ∈ 0..29.
#[test]
fn kat1_algebraic_factor_base_matches_brute_force() {
    let f = f_cubic();
    let b_alg = 30u64;
    let b_rat = 30u64;

    let fb = FactorBase::new(&f, b_rat, b_alg);

    // Collect the (p, r) pairs from the factor base.
    let fb_pairs: Vec<(u64, u64)> =
        fb.algebraic_ideals.iter().map(|ap| (ap.p, ap.r)).collect();

    // Brute-force: for each prime p ≤ B_alg, enumerate all roots of f mod p.
    let primes = factor_base_up_to(b_alg);
    let mut expected_pairs: Vec<(u64, u64)> = Vec::new();
    for p in &primes {
        let roots = brute_force_roots(&f, *p);
        for r in roots {
            expected_pairs.push((*p, r));
        }
    }

    // The factor base should match the brute-force enumeration exactly.
    assert_eq!(
        fb_pairs, expected_pairs,
        "algebraic factor base should match brute-force root enumeration"
    );

    // Verify that each entry in the factor base has the correct index.
    for (i, ap) in fb.algebraic_ideals.iter().enumerate() {
        assert_eq!(
            ap.index, i,
            "AlgebraicPrime at position {i} should have index {i}, got {}",
            ap.index
        );
    }

    // Verify that p = 23 is flagged as a bad prime (disc(f) = −23).
    let bad_primes: Vec<u64> = fb.algebraic_ideals.iter()
        .filter(|ap| ap.is_bad_prime)
        .map(|ap| ap.p)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    assert!(
        bad_primes.contains(&23),
        "p = 23 should be flagged as a bad prime (disc(x³−x−1) = −23); bad primes: {bad_primes:?}"
    );

    // Verify that p = 5 is NOT a bad prime.
    let p5_ideals: Vec<&AlgebraicPrime> =
        fb.algebraic_ideals.iter().filter(|ap| ap.p == 5).collect();
    for ap in &p5_ideals {
        assert!(!ap.is_bad_prime, "p = 5 should not be a bad prime for x³−x−1");
    }

    // Spot-check: p = 5 should have root r = 2.
    // f(2) = 8 − 2 − 1 = 5 ≡ 0 (mod 5).
    assert!(
        fb_pairs.contains(&(5, 2)),
        "algebraic factor base should contain (p=5, r=2): f(2) = 5 ≡ 0 (mod 5)"
    );

    // Spot-check: p = 7 should have root r = 5.
    // f(5) = 125 − 5 − 1 = 119 = 7 × 17 ≡ 0 (mod 7).
    assert!(
        fb_pairs.contains(&(7, 5)),
        "algebraic factor base should contain (p=7, r=5): f(5) = 119 ≡ 0 (mod 7)"
    );

    // Spot-check: p = 17 should have root r = 5.
    // f(5) = 119 = 7 × 17 ≡ 0 (mod 17).
    assert!(
        fb_pairs.contains(&(17, 5)),
        "algebraic factor base should contain (p=17, r=5): f(5) = 119 ≡ 0 (mod 17)"
    );

    // Verify the rational factor base.
    let expected_rat_primes = factor_base_up_to(b_rat);
    assert_eq!(
        fb.rational_primes, expected_rat_primes,
        "rational factor base should be all primes ≤ B_rat"
    );

    // Verify matrix_width = rational_size + algebraic_size + obstruction_count.
    assert_eq!(
        fb.matrix_width(),
        fb.rational_size() + fb.algebraic_size() + fb.obstruction_count,
        "matrix_width should equal rational + algebraic + obstruction"
    );
    assert_eq!(fb.obstruction_count, 1, "obstruction_count should be 1 (sign column)");
}

/// KAT 1b: algebraic_index and rational_index lookups are consistent with the factor base.
#[test]
fn kat1b_index_lookups_consistent() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 30, 30);

    // Every ideal in the algebraic factor base should be findable by lookup.
    for ap in &fb.algebraic_ideals {
        let idx = fb.algebraic_index(ap.p, ap.r);
        assert_eq!(
            idx,
            Some(ap.index),
            "algebraic_index({}, {}) should return Some({})",
            ap.p,
            ap.r,
            ap.index
        );
    }

    // Every prime in the rational factor base should be findable by lookup.
    for (i, &p) in fb.rational_primes.iter().enumerate() {
        let idx = fb.rational_index(p);
        assert_eq!(
            idx,
            Some(i),
            "rational_index({p}) should return Some({i})"
        );
    }

    // A prime not in the rational base should return None.
    assert_eq!(fb.rational_index(31), None, "31 > B_rat=30 should not be in rational base");

    // A (p, r) pair not in the algebraic base should return None.
    // (p=2, r=0) — p=2 has no roots for f(x)=x³−x−1, so this should not be present.
    assert_eq!(
        fb.algebraic_index(2, 0),
        None,
        "(p=2, r=0) should not be in algebraic base (f has no roots mod 2)"
    );
}

// ─── KAT 2: Norm reconstruction ──────────────────────────────────────────────

/// KAT 2: Norm computation and Relation::verify for a hand-constructed smooth relation.
///
/// Uses ``f(x) = x³ − x − 1``, ``m = 2``, ``n = 5``, ``(a, b) = (5, 1)``.
///
/// Hand-computed values:
/// - ``N_rat(5, 1) = 5 − 1·2 = 3``.
/// - ``N_alg(5, 1) = 1·5³·1⁰ + 0·5²·1¹ + (−1)·5¹·1² + (−1)·5⁰·1³ = 125 − 5 − 1 = 119 = 7 × 17``.
///
/// The rational norm 3 is smooth over {3}; the algebraic norm 119 = 7 × 17 is smooth over {7, 17}.
/// The algebraic ideals are (7, 5) and (17, 5) since ``5 ≡ 5·1 (mod 7)`` and ``5 ≡ 5·1 (mod 17)``.
#[test]
fn kat2_norm_reconstruction_and_relation_verify() {
    let f = f_cubic();

    // Hand-computed norms.
    let a = bi(5);
    let b = bi(1);
    let m = bi(2);

    let rat_norm = rational_norm(&a, &b, &m);
    assert_eq!(rat_norm, bi(3), "N_rat(5, 1) = 5 − 2 = 3");

    let alg_norm = algebraic_norm(&a, &b, &f);
    assert_eq!(alg_norm, bi(119), "N_alg(5, 1) = 125 − 5 − 1 = 119");

    // Verify that 119 = 7 × 17.
    assert_eq!(bi(119), bi(7) * bi(17));

    // Sign: N_rat = 3 > 0, so rational_sign = false.
    assert!(!norm_sign(&rat_norm), "N_rat = 3 is positive");

    // Build the polynomial pair: f(x) = x³ − x − 1, g(x) = x − 2, m = 2, n = 5.
    // n = 5 = f(2) = 8 − 2 − 1.
    let n = bi(5);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let poly = gnfs::PolyPair::new(f.clone(), g, m.clone(), n);
    poly.verify().expect("polynomial pair should be valid");

    // Build the factor base with B_rat = 30, B_alg = 30.
    let fb = FactorBase::new(&f, 30, 30);

    // Construct the rational smoothness witness for |N_rat| = 3.
    let rat_fb = factor_base_up_to(30);
    let rat_uint = norm_to_uint(&rat_norm).expect("N_rat = 3 fits in Uint<4>");
    let rat_witness = trial_smooth(&rat_uint, &rat_fb);
    assert!(rat_witness.is_smooth(), "N_rat = 3 should be smooth over B_rat = 30");
    assert_eq!(rat_witness.factors, vec![(3u64, 1u32)], "3 = 3^1");

    // Construct the algebraic smoothness witness for |N_alg| = 119 = 7 × 17.
    let alg_fb = factor_base_up_to(30);
    let alg_uint = norm_to_uint(&alg_norm).expect("N_alg = 119 fits in Uint<4>");
    let alg_witness = trial_smooth(&alg_uint, &alg_fb);
    assert!(alg_witness.is_smooth(), "N_alg = 119 = 7×17 should be smooth over B_alg = 30");
    assert_eq!(alg_witness.factors, vec![(7u64, 1u32), (17u64, 1u32)], "119 = 7^1 × 17^1");

    // Construct the Relation.
    let relation = Relation::new(
        a.clone(),
        b.clone(),
        &rat_witness,
        &alg_witness,
        false, // rational_sign: N_rat = 3 > 0
        &fb,
    );
    let relation = relation.expect("Relation::new should succeed for a valid smooth pair");

    // Verify the relation.
    relation.verify(&poly, &fb).expect("Relation::verify should pass for a valid relation");

    // Verify the rational exponent vector: 3 = 3^1, so index of 3 in rational base.
    let idx_3 = fb.rational_index(3).expect("3 should be in rational factor base");
    assert_eq!(relation.rational_exponents.get(idx_3), 1, "exponent of 3 should be 1");

    // Verify the algebraic exponent vector: 119 = 7^1 × 17^1.
    // Ideal (7, 5): a=5 ≡ 5·b=5 (mod 7), so r=5.
    let idx_7_5 = fb.algebraic_index(7, 5).expect("(7, 5) should be in algebraic factor base");
    assert_eq!(relation.algebraic_exponents.get(idx_7_5), 1, "exponent of ideal (7,5) should be 1");

    // Ideal (17, 5): a=5 ≡ 5·b=5 (mod 17), so r=5.
    let idx_17_5 = fb.algebraic_index(17, 5).expect("(17, 5) should be in algebraic factor base");
    assert_eq!(
        relation.algebraic_exponents.get(idx_17_5),
        1,
        "exponent of ideal (17,5) should be 1"
    );

    // GF(2) rows.
    let rat_row = relation.rational_row_gf2(&fb);
    // Bit 0: sign = false (0). Bit 1+idx_3: exponent of 3 mod 2 = 1.
    assert!(!rat_row[0], "sign column should be 0 (N_rat > 0)");
    assert!(rat_row[1 + idx_3], "exponent of 3 mod 2 should be 1");

    let alg_row = relation.algebraic_row_gf2(&fb);
    assert!(alg_row[idx_7_5], "exponent of ideal (7,5) mod 2 should be 1");
    assert!(alg_row[idx_17_5], "exponent of ideal (17,5) mod 2 should be 1");
}

/// KAT 2b: Relation::verify fails when a rational exponent is perturbed.
///
/// Perturbing an exponent breaks the norm reconstruction check.
#[test]
fn kat2b_verify_fails_on_perturbed_exponent() {
    let f = f_cubic();
    let a = bi(5);
    let b = bi(1);
    let m = bi(2);
    let n = bi(5);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let poly = gnfs::PolyPair::new(f.clone(), g, m.clone(), n);
    let fb = FactorBase::new(&f, 30, 30);

    let rat_fb = factor_base_up_to(30);
    let alg_fb = factor_base_up_to(30);

    let rat_norm = rational_norm(&a, &b, &m);
    let alg_norm = algebraic_norm(&a, &b, &f);

    let rat_uint = norm_to_uint(&rat_norm).unwrap();
    let alg_uint = norm_to_uint(&alg_norm).unwrap();

    let rat_witness = trial_smooth(&rat_uint, &rat_fb);
    let alg_witness = trial_smooth(&alg_uint, &alg_fb);

    let relation = Relation::new(a.clone(), b.clone(), &rat_witness, &alg_witness, false, &fb)
        .expect("valid relation");

    // Perturb: change the exponent of 3 from 1 to 2 in the rational exponent vector.
    let mut perturbed = relation.clone();
    let idx_3 = fb.rational_index(3).unwrap();
    // Find the entry for idx_3 and change its exponent.
    for (idx, exp) in perturbed.rational_exponents.entries.iter_mut() {
        if *idx == idx_3 {
            *exp = 2; // was 1, now 2 → reconstructed norm = 9 ≠ 3
        }
    }

    let result = perturbed.verify(&poly, &fb);
    assert!(
        matches!(result, Err(RelationError::RationalMismatch { .. })),
        "verify should fail with RationalMismatch when exponent is perturbed; got: {result:?}"
    );
}

/// KAT 2c: Relation::new returns None for a non-coprime pair.
#[test]
fn kat2c_relation_new_rejects_non_coprime() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 30, 30);

    // (a, b) = (4, 2): gcd(4, 2) = 2 ≠ 1.
    let a = bi(4);
    let b = bi(2);

    // Dummy witnesses (content doesn't matter since gcd check comes first).
    let dummy_witness = SmoothWitness {
        factors: vec![],
        cofactor: crypto_bigint::Uint::<4>::ONE,
    };

    let result = Relation::new(a, b, &dummy_witness, &dummy_witness, false, &fb);
    assert!(result.is_none(), "Relation::new should return None for non-coprime (a, b)");
}

/// KAT 2d: Relation::new returns None when a witness has cofactor > 1 (not fully smooth).
#[test]
fn kat2d_relation_new_rejects_partial_smoothness() {
    let f = f_cubic();
    let fb = FactorBase::new(&f, 30, 30);

    let a = bi(5);
    let b = bi(1);

    // Rational witness with cofactor = 7 (not fully smooth).
    let partial_witness = SmoothWitness {
        factors: vec![(3u64, 1u32)],
        cofactor: crypto_bigint::Uint::<4>::from(7u64),
    };
    let smooth_witness = SmoothWitness {
        factors: vec![(7u64, 1u32), (17u64, 1u32)],
        cofactor: crypto_bigint::Uint::<4>::ONE,
    };

    let result = Relation::new(a, b, &partial_witness, &smooth_witness, false, &fb);
    assert!(
        result.is_none(),
        "Relation::new should return None when rational witness has cofactor > 1"
    );
}

// ─── KAT 3: Norm bridge range ─────────────────────────────────────────────────

/// KAT 3: Norm bridge correctly handles toy-scale norms and rejects overflow.
///
/// Verifies:
/// - A toy-scale norm (e.g., 119) fits in ``Uint<4>`` and converts correctly.
/// - A norm of 0 converts to ``Uint<4>::ZERO``.
/// - A negative norm takes its absolute value.
/// - A norm exceeding 256 bits returns ``NormBridgeError::Overflow``.
#[test]
fn kat3_norm_bridge_range() {
    // Toy-scale norm: 119 = 7 × 17 (the algebraic norm from KAT 2).
    let norm_119 = bi(119);
    let u = norm_to_uint(&norm_119).expect("119 should fit in Uint<4>");
    assert_eq!(u, crypto_bigint::Uint::<4>::from(119u64), "norm_to_uint(119) should be 119");

    // Negative norm: |−119| = 119.
    let norm_neg = bi(-119);
    let u_neg = norm_to_uint(&norm_neg).expect("−119 should fit in Uint<4> (abs = 119)");
    assert_eq!(u_neg, crypto_bigint::Uint::<4>::from(119u64), "norm_to_uint(−119) should be 119");

    // Zero norm.
    let norm_zero = bi(0);
    let u_zero = norm_to_uint(&norm_zero).expect("0 should fit in Uint<4>");
    assert_eq!(u_zero, crypto_bigint::Uint::<4>::ZERO, "norm_to_uint(0) should be 0");

    // Maximum 256-bit value: 2^256 − 1.
    let max_256 = (BigInt::from(1i64) << 256) - BigInt::from(1i64);
    let u_max = norm_to_uint(&max_256).expect("2^256 − 1 should fit in Uint<4>");
    assert_eq!(u_max, crypto_bigint::Uint::<4>::MAX, "norm_to_uint(2^256 − 1) should be Uint::MAX");

    // Overflow: 2^256 requires 257 bits.
    let overflow = BigInt::from(1i64) << 256;
    let result = norm_to_uint(&overflow);
    assert!(
        matches!(result, Err(NormBridgeError::Overflow { bits_required }) if bits_required > 256),
        "2^256 should overflow Uint<4>; got: {result:?}"
    );

    // Overflow: a large norm (e.g., 2^300).
    let very_large = BigInt::from(1i64) << 300;
    let result2 = norm_to_uint(&very_large);
    assert!(
        matches!(result2, Err(NormBridgeError::Overflow { bits_required }) if bits_required > 256),
        "2^300 should overflow Uint<4>; got: {result2:?}"
    );

    // Negative overflow: |−2^257| also overflows.
    let neg_overflow = BigInt::from(-1i64) * (BigInt::from(1i64) << 257);
    let result3 = norm_to_uint(&neg_overflow);
    assert!(
        matches!(result3, Err(NormBridgeError::Overflow { bits_required }) if bits_required > 256),
        "−2^257 should overflow Uint<4> (abs = 2^257); got: {result3:?}"
    );
}

/// KAT 3b: norm_sign correctly identifies the sign of a norm.
#[test]
fn kat3b_norm_sign() {
    assert!(!norm_sign(&bi(0)), "0 is non-negative");
    assert!(!norm_sign(&bi(1)), "1 is positive");
    assert!(!norm_sign(&bi(119)), "119 is positive");
    assert!(norm_sign(&bi(-1)), "−1 is negative");
    assert!(norm_sign(&bi(-119)), "−119 is negative");
}

/// KAT 3c: norm_to_uint round-trips through trial_smooth for a toy norm.
///
/// Verifies the full pipeline: algebraic norm → Uint<4> → trial_smooth → SmoothWitness.
#[test]
fn kat3c_norm_bridge_round_trip_with_trial_smooth() {
    let f = f_cubic();
    let a = bi(5);
    let b = bi(1);

    // N_alg(5, 1) = 119 = 7 × 17.
    let alg_norm = algebraic_norm(&a, &b, &f);
    assert_eq!(alg_norm, bi(119));

    // Convert to Uint<4>.
    let alg_uint = norm_to_uint(&alg_norm).expect("119 fits in Uint<4>");

    // Trial smooth over B = 30.
    let fb = factor_base_up_to(30);
    let witness = trial_smooth(&alg_uint, &fb);

    assert!(witness.is_smooth(), "119 should be smooth over B = 30");
    assert_eq!(witness.factors, vec![(7u64, 1u32), (17u64, 1u32)]);
    assert!(witness.verify(&alg_uint), "witness should verify");
}
