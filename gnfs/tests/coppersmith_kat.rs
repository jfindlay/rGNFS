//! Known-answer tests (KATs) for the Coppersmith multi-poly method (G.B.4).
//!
//! Coppersmith's method generates multiple algebraic-side polynomials sharing the same
//! rational-side polynomial `g = x − m`. Each variant is produced by a Kleinjung rotation
//! `f_i = f_0 + (j_i · x + k_i) · g`, which preserves the root `m` mod `N`.
//!
//! # Tests
//!
//! 1. **Verify KAT:** all polynomials returned by `coppersmith_polys` satisfy
//!    `PolyPair::verify()`.
//! 2. **Count KAT:** `coppersmith_polys` returns exactly `config.num_polys` pairs.
//! 3. **Best KAT:** `coppersmith_best` returns a pair with Murphy-E score ≥ the seed's
//!    score (the multi-poly set always includes the seed as variant 0).
//! 4. **Generator KAT:** `CoppersmithGenerator::generate()` yields exactly `num_polys`
//!    candidates.
//! 5. **Principle-4 annotation KAT:** documents the under-exposure of the multi-poly
//!    yield improvement at toy scale — the score improvement is small (< 2×) on toy N,
//!    which is expected and annotated as a science↔engineering disconnect.
//!
//! # Science↔engineering note (principle-4 annotation)
//!
//! In production NFS (e.g., RSA-768), using different polynomials for different sieve
//! regions measurably improves the relation yield. At toy scale (60–100 bit N), the sieve
//! region is too small for this effect to manifest. These KATs verify the mathematical
//! construction (invariant preservation, count, ordering) but do NOT expect a large
//! absolute improvement in Murphy-E score — that payoff requires cryptographic-scale N.

use gnfs::{
    coppersmith_best, coppersmith_polys, score, select_base_m, CoppersmithConfig,
    CoppersmithGenerator, PolyGenerator,
};
use num_bigint::BigInt;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

// ─── KAT 1: Verify ───────────────────────────────────────────────────────────

/// KAT 1: All polynomials returned by `coppersmith_polys` satisfy `PolyPair::verify()`.
///
/// Every rotation `f_i = f_0 + (j_i · x + k_i) · g` preserves the root `m` mod `N`
/// because `g(m) = 0`. This KAT confirms the invariant holds for all generated variants.
#[test]
fn kat1_all_polys_verify() {
    let n = bi(1009 * 1013); // = 1022117
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig::default(); // num_polys = 5, step = 1

    for (i, poly) in coppersmith_polys(&seed, &config).iter().enumerate() {
        poly.verify().unwrap_or_else(|e| {
            panic!("coppersmith variant {i} failed PolyPair::verify(): {e}");
        });
    }
}

/// KAT 1b: Verify holds for a larger num_polys and non-unit step.
///
/// Uses `num_polys = 13, step = 3` to exercise more of the spiral schedule and
/// confirm that larger rotation parameters still preserve the root invariant.
#[test]
fn kat1b_all_polys_verify_large_config() {
    let n = bi(999983i64 * 999979i64); // ≈ 10^12, ~40 bits
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 13, step: 3 };

    for (i, poly) in coppersmith_polys(&seed, &config).iter().enumerate() {
        poly.verify().unwrap_or_else(|e| {
            panic!("coppersmith variant {i} (large config) failed PolyPair::verify(): {e}");
        });
    }
}

// ─── KAT 2: Count ────────────────────────────────────────────────────────────

/// KAT 2: `coppersmith_polys` returns exactly `config.num_polys` pairs.
///
/// The count must be exact — not "at least" — because the caller relies on the
/// returned `Vec` having a predictable length for downstream processing.
#[test]
fn kat2_exact_count() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);

    for num_polys in [1, 3, 5, 9, 17] {
        let config = CoppersmithConfig { num_polys, step: 1 };
        let polys = coppersmith_polys(&seed, &config);
        assert_eq!(
            polys.len(),
            num_polys,
            "coppersmith_polys should return exactly {num_polys} pairs, got {}",
            polys.len()
        );
    }
}

/// KAT 2b: `coppersmith_polys` with `num_polys = 1` returns exactly the seed.
///
/// When `num_polys = 1`, the only variant is variant 0 (the identity rotation),
/// which must reproduce the seed's `f` coefficients exactly.
#[test]
fn kat2b_single_poly_is_seed() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 1, step: 1 };
    let polys = coppersmith_polys(&seed, &config);

    assert_eq!(polys.len(), 1);
    assert_eq!(
        polys[0].f.coeffs, seed.f.coeffs,
        "variant 0 (identity rotation) must reproduce the seed's f coefficients"
    );
}

// ─── KAT 3: Best ─────────────────────────────────────────────────────────────

/// KAT 3: `coppersmith_best` returns a pair with Murphy-E score ≥ the seed's score.
///
/// Because the seed itself is always included as variant 0 (the identity rotation),
/// the best pair from the multi-poly set is always at least as good as the seed.
/// We use `≥` rather than `>` because at toy scale the seed may already be optimal
/// within the generated set (the improvement is under-exposed — see principle-4 note).
#[test]
fn kat3_best_score_ge_seed() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let seed_score = score(&seed);

    let config = CoppersmithConfig::default();
    let best = coppersmith_best(&seed, &config);
    let best_score = score(&best);

    assert!(
        best_score >= seed_score,
        "coppersmith_best should return score ≥ seed score: \
         seed_score = {seed_score:.6e}, best_score = {best_score:.6e}"
    );
}

/// KAT 3b: `coppersmith_best` result satisfies `PolyPair::verify()`.
#[test]
fn kat3b_best_verifies() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig::default();
    let best = coppersmith_best(&seed, &config);
    best.verify().expect("coppersmith_best result must satisfy PolyPair::verify()");
}

/// KAT 3c: `coppersmith_best` score matches the maximum score from `coppersmith_polys`.
///
/// Both paths must agree on the best score, confirming that `coppersmith_best` is
/// consistent with `coppersmith_polys`.
#[test]
fn kat3c_best_matches_max_of_polys() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 9, step: 2 };

    let polys = coppersmith_polys(&seed, &config);
    let max_score = polys.iter().map(score).fold(f64::NEG_INFINITY, f64::max);

    let best = coppersmith_best(&seed, &config);
    let best_score = score(&best);

    assert!(
        (best_score - max_score).abs() < 1e-12,
        "coppersmith_best score {best_score:.6e} should match max of polys {max_score:.6e}"
    );
}

// ─── KAT 4: Generator ────────────────────────────────────────────────────────

/// KAT 4: `CoppersmithGenerator::generate()` yields exactly `num_polys` candidates.
///
/// The generator must produce the same count as `coppersmith_polys` for the same config.
#[test]
fn kat4_generator_exact_count() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 7, step: 1 };
    let expected = config.num_polys;

    let generator = CoppersmithGenerator { seed, config };
    let count = generator.generate().count();

    assert_eq!(
        count, expected,
        "CoppersmithGenerator should yield {expected} candidates, got {count}"
    );
}

/// KAT 4b: All candidates from `CoppersmithGenerator::generate()` satisfy `PolyPair::verify()`.
#[test]
fn kat4b_generator_all_verify() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 9, step: 1 };

    let generator = CoppersmithGenerator { seed, config };
    for (i, candidate) in generator.generate().enumerate() {
        candidate.verify().unwrap_or_else(|e| {
            panic!("CoppersmithGenerator candidate {i} failed verify: {e}");
        });
    }
}

/// KAT 4c: Generator best score matches `coppersmith_best` score.
///
/// Both the generator and `coppersmith_best` search the same set of candidates;
/// their best scores must agree.
#[test]
fn kat4c_generator_best_matches_coppersmith_best() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = CoppersmithConfig { num_polys: 5, step: 1 };

    let best_score = score(&coppersmith_best(&seed, &config));

    let generator = CoppersmithGenerator { seed, config };
    let gen_best_score = generator.generate().map(|p| score(&p)).fold(f64::NEG_INFINITY, f64::max);

    assert!(
        (gen_best_score - best_score).abs() < 1e-12,
        "generator best score {gen_best_score:.6e} should match coppersmith_best score {best_score:.6e}"
    );
}

// ─── KAT 5: Principle-4 annotation ──────────────────────────────────────────

/// KAT 5: The multi-poly score improvement is small (< 2×) on toy N.
///
/// **Science↔engineering disconnect (ROADMAP principle 4):** In production NFS
/// (e.g., RSA-768), using different polynomials for different sieve regions
/// measurably improves the relation yield. At toy scale (60–100 bit N), the sieve
/// region is too small for this effect to manifest.
///
/// This KAT asserts that the Murphy-E improvement from multi-poly is < 2× on toy N.
/// This is NOT a failure condition — it is the expected behaviour at toy scale, and
/// the assertion documents the under-exposure. If this test ever fails (improvement
/// ≥ 2×), it would be surprising and worth investigating.
///
/// The assertion uses `< 2.0` as a generous upper bound: at toy scale the improvement
/// is typically < 1.1× (often exactly 1.0× when the seed is already optimal in the
/// generated set). The bound of 2× is chosen to be clearly in the "toy scale" regime
/// while still being a meaningful assertion.
#[test]
fn kat5_principle4_improvement_small_at_toy_scale() {
    // Use a small toy N to make the under-exposure maximally visible.
    let n = bi(1009 * 1013); // = 1022117, ~20 bits
    let seed = select_base_m(&n, 3);
    let seed_score = score(&seed);

    // Generate a reasonably large set to give multi-poly the best chance.
    let config = CoppersmithConfig { num_polys: 17, step: 1 };
    let best = coppersmith_best(&seed, &config);
    let best_score = score(&best);

    // The improvement ratio must be ≥ 1.0 (best is at least as good as seed).
    let improvement = if seed_score > 0.0 { best_score / seed_score } else { 1.0 };
    assert!(
        improvement >= 1.0,
        "multi-poly best must be at least as good as seed: improvement = {improvement:.4}"
    );

    // At toy scale, the improvement is expected to be < 2×.
    // This assertion documents the science↔engineering disconnect (principle 4):
    // the mathematical construction is correct, but the engineering payoff
    // (large improvement from multi-poly) requires cryptographic-scale N.
    assert!(
        improvement < 2.0,
        "at toy scale, multi-poly improvement should be < 2× (principle-4 under-exposure): \
         improvement = {improvement:.4}. If this fails, the toy-scale assumption may not hold \
         for this N — investigate whether N is large enough to expose the multi-poly effect."
    );
}

/// KAT 5b: Principle-4 annotation — score improvement is consistent across multiple toy N.
///
/// Checks that the < 2× improvement bound holds for several toy N values, confirming
/// that the under-exposure is a systematic property of toy scale, not an accident of
/// a single N.
#[test]
fn kat5b_principle4_consistent_across_toy_n() {
    // Several toy N values spanning ~20–40 bits.
    let toy_ns: &[i64] = &[
        1009 * 1013,           // ~20 bits
        9973 * 9967,           // ~27 bits
        99991 * 99997,         // ~33 bits
        999983 * 999979,       // ~40 bits
    ];

    let config = CoppersmithConfig { num_polys: 9, step: 1 };

    for &n_val in toy_ns {
        let n = bi(n_val);
        let seed = select_base_m(&n, 3);
        let seed_score = score(&seed);
        let best_score = score(&coppersmith_best(&seed, &config));

        let improvement = if seed_score > 0.0 { best_score / seed_score } else { 1.0 };

        assert!(
            improvement >= 1.0,
            "N={n_val}: multi-poly best must be ≥ seed score, improvement = {improvement:.4}"
        );
        assert!(
            improvement < 2.0,
            "N={n_val}: at toy scale, multi-poly improvement should be < 2× (principle-4): \
             improvement = {improvement:.4}"
        );
    }
}
