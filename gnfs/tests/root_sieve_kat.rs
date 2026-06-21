//! Known-answer tests (KATs) for the root sieve.
//!
//! The root sieve applies Kleinjung-style rotation `f' = f + (j·x + k)·g` over a grid
//! of `(j, k)` values, scoring each candidate with Murphy-E and returning the best.
//!
//! # Tests
//!
//! 1. **Improvement KAT:** `root_sieve` returns a pair with Murphy-E score ≥ the seed's
//!    score (the sieve finds at least as good a polynomial).
//! 2. **Determinism KAT:** calling `root_sieve` twice with the same seed and config
//!    returns the same result (same `f` coefficients).
//! 3. **Verify KAT:** the returned pair satisfies `PolyPair::verify()`.
//! 4. **Generator KAT:** `RootSieveGenerator::generate()` yields exactly
//!    `(2·j_range + 1) × (2·k_range + 1)` candidates.
//!
//! # Science↔engineering note (principle-4 annotation)
//!
//! Murphy-E's predictive value (higher E → more relations) only manifests at sieve scale
//! (N ≳ 2^100). At toy scale, these KATs verify the ordering property and self-consistency
//! of the implementation. The improvement KAT uses `≥` (not `>`) because at toy scale the
//! seed may already be optimal within the search grid.

use gnfs::{root_sieve, score, select_base_m, PolyGenerator, RootSieveConfig, RootSieveGenerator};
use num_bigint::BigInt;

// ─── helpers ─────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

// ─── KAT 1: Improvement ──────────────────────────────────────────────────────

/// KAT 1: `root_sieve` returns a pair with Murphy-E score ≥ the seed's score.
///
/// The seed is the base-m polynomial for `N = 1009 * 1013 = 1022117`. The sieve
/// searches over `j ∈ [−10, 10]`, `k ∈ [−10, 10]` (441 candidates). The returned
/// pair must score at least as well as the seed, because the seed itself (at `j=0, k=0`)
/// is always a candidate.
///
/// We use `≥` rather than `>` because at toy scale the seed may already be the best
/// polynomial in the grid (the improvement is under-exposed at small N).
#[test]
fn kat1_improvement_score_ge_seed() {
    let n = bi(1009 * 1013); // = 1022117
    let seed = select_base_m(&n, 3);
    let seed_score = score(&seed);

    let config = RootSieveConfig::default(); // j_range = k_range = 10
    let best = root_sieve(&seed, &config);
    let best_score = score(&best);

    assert!(
        best_score >= seed_score,
        "root_sieve should return a pair with score ≥ seed score: \
         seed_score = {seed_score:.6e}, best_score = {best_score:.6e}"
    );
}

// ─── KAT 2: Determinism ──────────────────────────────────────────────────────

/// KAT 2: `root_sieve` is deterministic — calling it twice with the same seed and
/// config returns the same `f` coefficients.
///
/// The search is deterministic because the grid is traversed in a fixed order and
/// ties are broken by keeping the first maximum found.
#[test]
fn kat2_determinism() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig::default();

    let result1 = root_sieve(&seed, &config);
    let result2 = root_sieve(&seed, &config);

    assert_eq!(
        result1.f.coeffs, result2.f.coeffs,
        "root_sieve should be deterministic: got different f coefficients on two calls"
    );
    assert_eq!(
        result1.m, result2.m,
        "root_sieve should be deterministic: got different m on two calls"
    );
}

// ─── KAT 3: Verify ───────────────────────────────────────────────────────────

/// KAT 3: The pair returned by `root_sieve` satisfies `PolyPair::verify()`.
///
/// Every rotation preserves the root `m` mod `n` because `g(m) = 0`, so
/// `f'(m) = f(m) + (j·m + k)·g(m) = 0 (mod n)`. This KAT confirms the invariant
/// holds for the best pair found by the sieve.
#[test]
fn kat3_verify() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig::default();
    let best = root_sieve(&seed, &config);

    best.verify().expect("root_sieve result must satisfy PolyPair::verify()");
}

/// KAT 3b: Every candidate in the generator also satisfies `PolyPair::verify()`.
///
/// This is a stronger check: all 441 candidates in the default grid must be valid.
#[test]
fn kat3b_all_candidates_verify() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig { j_range: 5, k_range: 5 }; // 121 candidates
    let sieve_gen = RootSieveGenerator { seed, config };

    for (idx, candidate) in sieve_gen.generate().enumerate() {
        candidate.verify().unwrap_or_else(|e| {
            panic!("candidate {idx} failed verify: {e}");
        });
    }
}

// ─── KAT 4: Generator count ──────────────────────────────────────────────────

/// KAT 4: `RootSieveGenerator::generate()` yields exactly `(2·j_range + 1) × (2·k_range + 1)`
/// candidates.
///
/// For `j_range = 3, k_range = 4`: `(2·3+1) × (2·4+1) = 7 × 9 = 63` candidates.
#[test]
fn kat4_generator_candidate_count() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig { j_range: 3, k_range: 4 };
    let expected = (2 * config.j_range + 1) as usize * (2 * config.k_range + 1) as usize;

    let sieve_gen = RootSieveGenerator { seed, config };
    let count = sieve_gen.generate().count();

    assert_eq!(
        count, expected,
        "generator should yield {expected} candidates, got {count}"
    );
}

/// KAT 4b: Generator with default config yields at least `(2·j_range+1) × (2·k_range+1)`
/// candidates (441 for the default j_range = k_range = 10).
#[test]
fn kat4b_generator_default_count() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig::default();
    let expected = (2 * config.j_range + 1) as usize * (2 * config.k_range + 1) as usize;

    let sieve_gen = RootSieveGenerator { seed, config };
    let count = sieve_gen.generate().count();

    assert!(
        count >= expected,
        "generator should yield at least {expected} candidates, got {count}"
    );
}

// ─── KAT 5: Generator best matches root_sieve ────────────────────────────────

/// KAT 5: The best candidate from `RootSieveGenerator::generate()` has the same score
/// as the result of `root_sieve`.
///
/// Both paths search the same grid; the best score must agree.
#[test]
fn kat5_generator_best_matches_root_sieve() {
    let n = bi(1009 * 1013);
    let seed = select_base_m(&n, 3);
    let config = RootSieveConfig { j_range: 5, k_range: 5 };

    let sieve_result = root_sieve(&seed, &config);
    let sieve_score = score(&sieve_result);

    let sieve_gen = RootSieveGenerator { seed, config };
    let gen_best_score = sieve_gen
        .generate()
        .map(|p| score(&p))
        .fold(f64::NEG_INFINITY, f64::max);

    assert!(
        (gen_best_score - sieve_score).abs() < 1e-12,
        "generator best score {gen_best_score:.6e} should match root_sieve score {sieve_score:.6e}"
    );
}

// ─── KAT 6: Larger N ─────────────────────────────────────────────────────────

/// KAT 6: `root_sieve` works correctly for a larger toy N.
///
/// Uses `N = 999983 * 999979` (≈ 10^12, about 40 bits). The sieve must return a valid
/// pair with score ≥ the seed's score.
#[test]
fn kat6_larger_n() {
    let n = bi(999983i64 * 999979i64);
    let seed = select_base_m(&n, 3);
    let seed_score = score(&seed);

    let config = RootSieveConfig { j_range: 5, k_range: 5 };
    let best = root_sieve(&seed, &config);
    let best_score = score(&best);

    best.verify().expect("root_sieve result for larger N must verify");
    assert!(
        best_score >= seed_score,
        "root_sieve should return score ≥ seed for larger N: \
         seed_score = {seed_score:.6e}, best_score = {best_score:.6e}"
    );
}
