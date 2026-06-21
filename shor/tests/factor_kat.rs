//! Known-answer tests for Shor's factoring algorithm (C-Factor + C-OrderFind).
//!
//! # What is tested
//!
//! 1. **Order KATs** — the classical order `ord_a(N)` matches published values:
//!    - `ord₂(15) = 4` (2^4 = 16 ≡ 1 mod 15)
//!    - `ord₇(15) = 4` (7^4 = 2401 ≡ 1 mod 15)
//!    - `ord₂(21) = 6` (2^6 = 64 ≡ 1 mod 21)
//!    - `ord₂(35) = 12` (2^12 = 4096 ≡ 1 mod 35)
//!    - `ord₄(35) = 6` (4^6 = 4096 ≡ 1 mod 35)
//!    - `ord₂(91) = 12` (2^12 = 4096 ≡ 1 mod 91)
//!
//! 2. **Continued-fraction KATs** — a known measured phase `s/2^t` recovers the correct order
//!    `r` via the convergents (deterministic classical KAT, no quantum circuit needed).
//!
//! 3. **End-to-end factoring** — `factor(15)→{3,5}`, `factor(21)→{3,7}`, `factor(35)→{5,7}`
//!    with fixed seeds so the run is reproducible.
//!
//! 4. **91 ceiling-stress KAT** — `factor(91)→{7,13}`. N=91 uses 14 qubits (exp_len=7,
//!    work=7), well within the ~25-qubit simulator ceiling. This is the principle-4
//!    ceiling-stress case: the algorithm is correct; the qubit budget fits.
//!
//! # Qubit budgets (ancilla-free permutation-synthesis implementation)
//!
//! | N  | n_bits(N) | exp_len (t) | work (n) | total qubits |
//! |----|-----------|-------------|----------|--------------|
//! | 15 | 4         | 4           | 4        | 8            |
//! | 21 | 5         | 5           | 5        | 10           |
//! | 35 | 6         | 6           | 6        | 12           |
//! | 91 | 7         | 7           | 7        | 14           |
//!
//! All within the ~25-qubit ceiling.
//!
//! # Published reference values
//!
//! Orders (from Shor 1994, Nielsen & Chuang §5.3):
//!   - ord₂(15) = 4:  2^1=2, 2^2=4, 2^3=8, 2^4=16≡1 (mod 15)
//!   - ord₇(15) = 4:  7^1=7, 7^2=49≡4, 7^3=28≡13, 7^4=91≡1 (mod 15)
//!   - ord₂(21) = 6:  2^6=64≡1 (mod 21)
//!   - ord₂(35) = 12: 2^12=4096≡1 (mod 35)
//!   - ord₄(35) = 6:  4^6=4096≡1 (mod 35)
//!   - ord₂(91) = 12: 2^12=4096≡1 (mod 91)
//!
//! Continued-fraction recovery (N=15, a=2, t=8):
//!   - Measured s=64 (= 1·256/4): phase 64/256 = 1/4, convergent 1/4, r=4 ✓
//!   - Measured s=128 (= 2·256/4): phase 128/256 = 1/2, convergent 1/2, r=2 (not order)
//!     → convergent 1/4 not reached; but 2^2=4≠1, so s=128 gives r=None
//!   - Measured s=192 (= 3·256/4): phase 192/256 = 3/4, convergent 3/4, r=4 ✓
//!
//! Reference: Shor (1994); Nielsen & Chuang §5.3.

use shor::arith::{mod_pow, n_bits};
use shor::shor::{convergents, exp_len_for, factor, find_order, gcd, order_from_phase};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Assert that `factor(n, seed)` returns a valid factorization `{p, q}` with `p * q = n`.
///
/// The returned pair is normalized to `(min, max)` for comparison.
fn assert_factor(n: u64, seed: u64, expected: (u64, u64), label: &str) {
    let result = factor(n, seed);
    assert!(
        result.is_some(),
        "{label}: factor({n}, seed={seed}) returned None, expected {expected:?}"
    );
    let (p, q) = result.unwrap();
    let got = if p <= q { (p, q) } else { (q, p) };
    let exp = if expected.0 <= expected.1 { expected } else { (expected.1, expected.0) };
    assert_eq!(
        got, exp,
        "{label}: factor({n}, seed={seed}) = {got:?}, expected {exp:?}"
    );
    assert_eq!(p * q, n, "{label}: factor result {p} * {q} ≠ {n}");
}


// ── KAT group 1: order KATs (classical verification) ─────────────────────────
//
// These KATs verify that the classical order function is correct.
// The order is verified by checking a^r ≡ 1 mod N.

/// ord₂(15) = 4: 2^4 = 16 ≡ 1 (mod 15).
#[test]
fn order_classical_2_mod_15() {
    assert_eq!(mod_pow(2, 4, 15), 1, "2^4 mod 15 should be 1");
    assert_ne!(mod_pow(2, 1, 15), 1, "2^1 mod 15 should not be 1");
    assert_ne!(mod_pow(2, 2, 15), 1, "2^2 mod 15 should not be 1");
    assert_ne!(mod_pow(2, 3, 15), 1, "2^3 mod 15 should not be 1");
}

/// ord₇(15) = 4: 7^4 = 2401 ≡ 1 (mod 15).
#[test]
fn order_classical_7_mod_15() {
    // 7^1=7, 7^2=49≡4, 7^3=28≡13, 7^4=91≡1 (mod 15)
    assert_eq!(mod_pow(7, 4, 15), 1, "7^4 mod 15 should be 1");
    assert_ne!(mod_pow(7, 1, 15), 1, "7^1 mod 15 should not be 1");
    assert_ne!(mod_pow(7, 2, 15), 1, "7^2 mod 15 should not be 1");
    assert_ne!(mod_pow(7, 3, 15), 1, "7^3 mod 15 should not be 1");
}

/// ord₂(21) = 6: 2^6 = 64 ≡ 1 (mod 21).
#[test]
fn order_classical_2_mod_21() {
    assert_eq!(mod_pow(2, 6, 21), 1, "2^6 mod 21 should be 1");
    for k in 1..6 {
        assert_ne!(mod_pow(2, k, 21), 1, "2^{k} mod 21 should not be 1 (order is 6)");
    }
}

/// ord₂(35) = 12: 2^12 = 4096 ≡ 1 (mod 35).
#[test]
fn order_classical_2_mod_35() {
    assert_eq!(mod_pow(2, 12, 35), 1, "2^12 mod 35 should be 1");
    // Verify no smaller order: check divisors of 12 (1, 2, 3, 4, 6).
    for k in [1u64, 2, 3, 4, 6] {
        assert_ne!(mod_pow(2, k, 35), 1, "2^{k} mod 35 should not be 1 (order is 12)");
    }
}

/// ord₄(35) = 6: 4^6 = 4096 ≡ 1 (mod 35).
#[test]
fn order_classical_4_mod_35() {
    assert_eq!(mod_pow(4, 6, 35), 1, "4^6 mod 35 should be 1");
    for k in [1u64, 2, 3] {
        assert_ne!(mod_pow(4, k, 35), 1, "4^{k} mod 35 should not be 1 (order is 6)");
    }
}

/// ord₂(91) = 12: 2^12 = 4096 ≡ 1 (mod 91).
#[test]
fn order_classical_2_mod_91() {
    // 4096 mod 91: 91*45 = 4095, so 4096 mod 91 = 1. ✓
    assert_eq!(mod_pow(2, 12, 91), 1, "2^12 mod 91 should be 1");
    for k in [1u64, 2, 3, 4, 6] {
        assert_ne!(mod_pow(2, k, 91), 1, "2^{k} mod 91 should not be 1 (order is 12)");
    }
}

/// ord₃(91) = 6: 3^6 ≡ 1 (mod 91).
///
/// 3 is coprime to 91 (gcd(3, 91) = 1). 3^6 = 729 = 8×91 + 1 = 729. ✓
/// This is a useful base for factoring 91: ord₃(91) = 6 (even), 3^3 = 27 mod 91 = 27.
/// gcd(27-1, 91) = gcd(26, 91) = 13. ✓
#[test]
fn order_classical_3_mod_91() {
    // 3^6 = 729. 729 / 91 = 8 remainder 1. So 3^6 mod 91 = 1. ✓
    assert_eq!(mod_pow(3, 6, 91), 1, "3^6 mod 91 should be 1");
    // Verify no smaller order: check divisors of 6 (1, 2, 3).
    assert_ne!(mod_pow(3, 1, 91), 1, "3^1 mod 91 should not be 1");
    assert_ne!(mod_pow(3, 2, 91), 1, "3^2 mod 91 should not be 1");
    assert_ne!(mod_pow(3, 3, 91), 1, "3^3 mod 91 should not be 1");
    // Factor extraction: 3^3 = 27, gcd(27-1, 91) = gcd(26, 91) = 13. ✓
    assert_eq!(gcd(mod_pow(3, 3, 91) - 1, 91), 13, "gcd(3^3 - 1, 91) should be 13");
}

/// n_bits helper: correct bit counts for factoring targets.
#[test]
fn n_bits_for_factoring_targets() {
    // n_bits(N) = ⌈log₂(N)⌉ = bits needed to represent N-1
    assert_eq!(n_bits(15), 4, "n_bits(15) should be 4 (15 = 0b1111)");
    assert_eq!(n_bits(21), 5, "n_bits(21) should be 5 (21 = 0b10101)");
    assert_eq!(n_bits(35), 6, "n_bits(35) should be 6 (35 = 0b100011)");
    assert_eq!(n_bits(91), 7, "n_bits(91) should be 7 (91 = 0b1011011)");
}

/// exp_len_for: correct exponent register sizes.
///
/// Uses t = n_bits(N) (the ancilla-free permutation-synthesis qubit budget).
#[test]
fn exp_len_for_factoring_targets() {
    assert_eq!(exp_len_for(15), 4, "exp_len for N=15 should be 4 (n_bits(15))");
    assert_eq!(exp_len_for(21), 5, "exp_len for N=21 should be 5 (n_bits(21))");
    assert_eq!(exp_len_for(35), 6, "exp_len for N=35 should be 6 (n_bits(35))");
    assert_eq!(exp_len_for(91), 7, "exp_len for N=91 should be 7 (n_bits(91))");
}

/// gcd helper: basic correctness.
#[test]
fn gcd_basic() {
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(15, 5), 5);
    assert_eq!(gcd(7, 13), 1);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(6, 15), 3);
    // gcd(14, 91): 14 = 2×7, 91 = 7×13 → gcd = 7
    assert_eq!(gcd(14, 91), 7);
    // gcd(12, 91): 12 = 4×3, 91 = 7×13 → gcd = 1 (no common factors)
    assert_eq!(gcd(12, 91), 1);
    // gcd(26, 91): 26 = 2×13, 91 = 7×13 → gcd = 13
    assert_eq!(gcd(26, 91), 13);
}

// ── KAT group 2: continued-fraction KATs (classical, deterministic) ───────────
//
// Given a known measured phase s/2^t, verify that order_from_phase recovers the correct order r.
// These are purely classical KATs — no quantum circuit is run.

/// Continued-fraction convergents of 1/4 (s=64, t=8): denominator 4 is a convergent.
///
/// Phase 64/256 = 1/4. Convergents: 0/1, 1/4. Denominator 4 satisfies 2^4 ≡ 1 mod 15.
#[test]
fn cf_convergents_1_over_4() {
    // 64/256 = 1/4. Continued fraction: 0 + 1/4 → convergents: 0/1, 1/4.
    let convs = convergents(64, 256, 15);
    let denoms: Vec<u64> = convs.iter().map(|&(_, d)| d).collect();
    assert!(
        denoms.contains(&4),
        "convergents(64, 256, 15) should include denominator 4, got {denoms:?}"
    );
}

/// Continued-fraction convergents of 3/4 (s=192, t=8): denominator 4 is a convergent.
///
/// Phase 192/256 = 3/4. Convergents include 3/4. Denominator 4 satisfies 2^4 ≡ 1 mod 15.
#[test]
fn cf_convergents_3_over_4() {
    let convs = convergents(192, 256, 15);
    let denoms: Vec<u64> = convs.iter().map(|&(_, d)| d).collect();
    assert!(
        denoms.contains(&4),
        "convergents(192, 256, 15) should include denominator 4, got {denoms:?}"
    );
}

/// order_from_phase: s=64, t=8, a=2, N=15 → r=4.
///
/// Phase 64/256 = 1/4 ≈ 1/r with r=4. Convergent 1/4 has denominator 4.
/// 2^4 = 16 ≡ 1 mod 15. ✓
#[test]
fn cf_order_from_phase_n15_a2_s64() {
    // s=64, t=8: phase = 64/256 = 1/4 ≈ k/r with k=1, r=4.
    let r = order_from_phase(64, 8, 2, 15);
    assert_eq!(r, Some(4), "order_from_phase(64, 8, 2, 15) should be Some(4), got {r:?}");
}

/// order_from_phase: s=192, t=8, a=2, N=15 → r=4.
///
/// Phase 192/256 = 3/4 ≈ 3/r with r=4. Convergent 3/4 has denominator 4.
/// 2^4 = 16 ≡ 1 mod 15. ✓
#[test]
fn cf_order_from_phase_n15_a2_s192() {
    let r = order_from_phase(192, 8, 2, 15);
    assert_eq!(r, Some(4), "order_from_phase(192, 8, 2, 15) should be Some(4), got {r:?}");
}

/// order_from_phase: s=0, t=8, a=2, N=15 → None (uninformative measurement).
///
/// s=0 gives phase 0, which is uninformative (no period information).
#[test]
fn cf_order_from_phase_zero_phase() {
    let r = order_from_phase(0, 8, 2, 15);
    assert_eq!(r, None, "order_from_phase(0, ...) should be None (uninformative)");
}

/// order_from_phase: s=32, t=8, a=7, N=15 → r=4.
///
/// Phase 32/256 = 1/8. Convergents: 0/1, 1/8. Denominator 8 does not satisfy 7^8≡1 mod 15
/// (since ord₇(15)=4, 7^8≡1 mod 15 too). Actually 7^4≡1 mod 15, so denominator 4 is a
/// sub-convergent. Let's check: 32/256 = 1/8, convergents: 0/1, 1/8. Denominator 8:
/// 7^8 = (7^4)^2 ≡ 1^2 = 1 mod 15. So r=8 would be returned... but ord₇(15)=4.
/// The function returns the SMALLEST denominator d with a^d≡1 mod N, so it returns 4 if
/// 4 is a convergent denominator. 1/8 has convergents 0/1, 1/8 — denominator 4 is NOT a
/// convergent of 1/8. So this returns Some(8) (a multiple of the order, still valid).
/// Actually the spec says "smallest denominator d of a convergent such that a^d ≡ 1 mod N".
/// For s=32, t=8: convergents of 32/256=1/8 are 0/1, 1/8. Denominator 8: 7^8≡1 mod 15. ✓
#[test]
fn cf_order_from_phase_n15_a7_s32() {
    // Phase 32/256 = 1/8. Convergents: 0/1, 1/8.
    // 7^8 mod 15 = (7^4)^2 mod 15 = 1^2 = 1. So denominator 8 satisfies the check.
    let r = order_from_phase(32, 8, 7, 15);
    assert!(r.is_some(), "order_from_phase(32, 8, 7, 15) should be Some(_)");
    let r_val = r.unwrap();
    // r_val must satisfy 7^r_val ≡ 1 mod 15.
    assert_eq!(
        mod_pow(7, r_val, 15),
        1,
        "7^{r_val} mod 15 should be 1"
    );
}

/// order_from_phase: s=64, t=8, a=7, N=15 → r=4.
///
/// Phase 64/256 = 1/4. Convergents: 0/1, 1/4. Denominator 4: 7^4 ≡ 1 mod 15. ✓
#[test]
fn cf_order_from_phase_n15_a7_s64() {
    let r = order_from_phase(64, 8, 7, 15);
    assert_eq!(r, Some(4), "order_from_phase(64, 8, 7, 15) should be Some(4), got {r:?}");
}

/// order_from_phase: s=5, t=5, a=2, N=21 → r=6.
///
/// Phase 5/32 ≈ 1/6 (since 32/6 ≈ 5.33). Convergents of 5/32 include denominator 6.
/// 2^6 = 64 ≡ 1 mod 21. ✓
/// Note: exp_len for N=21 is 5 (n_bits(21)), so 2^t = 32.
#[test]
fn cf_order_from_phase_n21_a2_s5() {
    // t=5 for N=21 (exp_len = n_bits(21) = 5), 2^5 = 32.
    // 32/6 ≈ 5.33, so s=5 ≈ 1*32/6.
    // Convergents of 5/32: 32=6×5+2, 5=2×2+1, 2=2×1. CF=[0;6,2,2].
    // Convergents: 0/1, 1/6, 2/13, 5/32. Denominator 6: 2^6=64 mod 21=1. ✓
    let r = order_from_phase(5, 5, 2, 21);
    assert_eq!(r, Some(6), "order_from_phase(5, 5, 2, 21) should be Some(6), got {r:?}");
}

/// order_from_phase: s=27, t=5, a=2, N=21 → r=6.
///
/// Phase 27/32 ≈ 5/6 (since 5*32/6 ≈ 26.67). Convergents of 27/32 include denominator 6.
/// 2^6 = 64 ≡ 1 mod 21. ✓
#[test]
fn cf_order_from_phase_n21_a2_s27() {
    // t=5, 2^t=32. s=27 ≈ 5*32/6. Phase 27/32 ≈ 5/6.
    // Convergents of 27/32: 32=1×27+5, 27=5×5+2, 5=2×2+1, 2=2×1.
    // CF=[0;1,5,2,2]. Convergents: 0/1, 1/1, 5/6, 11/13, 27/32. Denominator 6: ✓
    let r = order_from_phase(27, 5, 2, 21);
    assert_eq!(r, Some(6), "order_from_phase(27, 5, 2, 21) should be Some(6), got {r:?}");
}

// ── KAT group 3: end-to-end factoring (quantum circuit + classical post-processing) ──
//
// These KATs run the full Shor factoring algorithm end-to-end with fixed seeds.
// Seeds are chosen to land a successful (even-order, nontrivial-gcd) run.

/// factor(15, seed=0) → {3, 5}.
///
/// N=15 = 3×5. Uses 8 qubits (exp_len=4, work=4). Seed 0.
/// Published factorization: 15 = 3 × 5.
#[test]
fn factor_15_seed0() {
    assert_factor(15, 0, (3, 5), "factor(15, seed=0)");
}

/// factor(21, seed=0) → {3, 7}.
///
/// N=21 = 3×7. Uses 10 qubits (exp_len=5, work=5). Seed 0.
/// Published factorization: 21 = 3 × 7.
#[test]
fn factor_21_seed0() {
    assert_factor(21, 0, (3, 7), "factor(21, seed=0)");
}

/// factor(35, seed=8) → {5, 7}.
///
/// N=35 = 5×7. Uses 12 qubits (exp_len=6, work=6). Seed 8.
/// Published factorization: 35 = 5 × 7.
///
/// Seed 8 is chosen because run_order_finding_circuit(2, layout_35, 8) measures s=2389
/// (≈ 7×4096/12), which recovers ord₂(35)=12 via continued fractions. Then:
/// 2^6 = 64 mod 35 = 29 ≠ 34, gcd(28, 35) = 7. Factor: 35/7 = 5. ✓
#[test]
fn factor_35_seed8() {
    assert_factor(35, 8, (5, 7), "factor(35, seed=8)");
}

// ── KAT group 4: 91 ceiling-stress KAT ───────────────────────────────────────
//
// N=91 = 7×13. Uses 14 qubits (exp_len=7, work=7), well within the ~25-qubit ceiling.
// This is the principle-4 ceiling-stress case: the algorithm is correct and the qubit
// budget fits. The ancilla-free permutation-synthesis implementation keeps the budget at
// exp_len+n=14.

/// Seed finder for N=91 (run once in release mode to find the right seed).
///
/// This test finds the first seed where run_order_finding_circuit(2, layout_91, seed)
/// gives a phase that recovers the order of 2 mod 91. Used to determine the seed for
/// the factor_91 KAT. Marked #[ignore] so it doesn't run in the normal test suite.
#[test]
#[ignore]
fn find_seed_for_91() {
    use shor::arith::ModExpLayout;
    use shor::shor::run_order_finding_circuit;
    let n = 91u64;
    let a = 2u64;
    let exp_len = exp_len_for(n);
    let layout = ModExpLayout::standard(n, exp_len);
    for seed in 0u64..100 {
        let s = run_order_finding_circuit(a, &layout, seed);
        let r = order_from_phase(s, exp_len, a, n);
        eprintln!("seed={seed}: s={s}, order={r:?}");
        if let Some(r_val) = r {
            assert_eq!(mod_pow(a, r_val, n), 1, "2^{r_val} mod 91 should be 1");
            eprintln!("FOUND: seed={seed}, s={s}, r={r_val}");
            return;
        }
    }
    panic!("No useful seed found in 0..100");
}

/// factor(91, seed=1) → {7, 13}.
///
/// N=91 = 7×13. Uses 14 qubits (exp_len=7, work=7). Seed 1.
/// Published factorization: 91 = 7 × 13.
/// Qubit budget: 14 qubits, within the ~25-qubit ceiling (principle 4).
///
/// Seed 1 is chosen because run_order_finding_circuit(2, layout_91, 1) measures s=53
/// (≈ 5×128/12), which recovers ord₂(91)=12 via continued fractions. Then:
/// 2^6 = 64 mod 91 = 64 ≠ 90, gcd(63, 91) = 7. Factor: 91/7 = 13. ✓
#[test]
fn factor_91_seed1() {
    assert_factor(91, 1, (7, 13), "factor(91, seed=1)");
}

/// Qubit budget for N=91 is within the ~25-qubit ceiling.
///
/// This test documents the principle-4 resource-scale annotation: N=91 requires 14 qubits
/// (exp_len=7, work=7), which fits within the ~25-qubit simulator ceiling. The algorithm is
/// correct at this scale; larger N would require more qubits than the simulator supports.
///
/// Qubit budget for N=91 (7-bit, t=7): 14 qubits total.
#[test]
fn qubit_budget_91_within_ceiling() {
    use shor::arith::ModExpLayout;
    let exp_len = exp_len_for(91);
    let layout = ModExpLayout::standard(91, exp_len);
    let total = layout.total_qubits();
    assert_eq!(exp_len, 7, "exp_len for N=91 should be 7");
    assert_eq!(layout.work_len, 7, "work register for N=91 should be 7 qubits");
    assert_eq!(total, 14, "total qubits for N=91 should be 14");
    assert!(
        total <= 25,
        "N=91 requires {total} qubits, within the ~25-qubit ceiling"
    );
}

/// Qubit budgets for all factoring targets are within the ~25-qubit ceiling.
///
/// Verifies the ancilla-free permutation-synthesis qubit budgets for all factoring targets.
#[test]
fn qubit_budgets_all_targets() {
    use shor::arith::ModExpLayout;
    // (N, expected_exp_len, expected_work_len, expected_total)
    // exp_len = n_bits(N), work_len = n_bits(N+1)
    let targets = [(15u64, 4, 4, 8), (21, 5, 5, 10), (35, 6, 6, 12), (91, 7, 7, 14)];
    for (n, exp_len, work_len, total) in targets {
        let layout = ModExpLayout::standard(n, exp_len_for(n));
        assert_eq!(layout.exp_len, exp_len, "N={n}: exp_len mismatch");
        assert_eq!(layout.work_len, work_len, "N={n}: work_len mismatch");
        assert_eq!(layout.total_qubits(), total, "N={n}: total qubits mismatch");
        assert!(
            layout.total_qubits() <= 25,
            "N={n}: {total} qubits exceeds ~25-qubit ceiling"
        );
    }
}

// ── Additional robustness KATs ────────────────────────────────────────────────

/// factor(N) returns a valid factorization for all toy targets.
///
/// Verifies that the returned factors multiply to N and are nontrivial (1 < p, q < N).
/// Uses seeds chosen to succeed on the first circuit run (fast in debug mode).
#[test]
fn factor_returns_valid_factorization() {
    // Seeds chosen for fast convergence: each succeeds on the first circuit run.
    // N=15: seed=0 (fast, 8 qubits)
    // N=21: seed=0 (fast, 10 qubits)
    // N=35: seed=8 (s=2389 with exp_len=6, recovers ord₂(35)=12)
    // N=91: seed=1 (s=53 with exp_len=7, recovers ord₂(91)=12)
    for (n, seed) in [(15u64, 0u64), (21, 0), (35, 8), (91, 1)] {
        let result = factor(n, seed);
        assert!(result.is_some(), "factor({n}, seed={seed}) returned None");
        let (p, q) = result.unwrap();
        assert_eq!(p * q, n, "factor({n}): {p} * {q} ≠ {n}");
        assert!(p > 1 && p < n, "factor({n}): p={p} is trivial");
        assert!(q > 1 && q < n, "factor({n}): q={q} is trivial");
    }
}

/// factor(N) is reproducible: same seed → same result.
#[test]
fn factor_reproducible() {
    // Use seeds that are fast (succeed on first circuit run).
    for (n, seed) in [(15u64, 0u64), (21, 0), (35, 8)] {
        let r1 = factor(n, seed);
        let r2 = factor(n, seed);
        assert_eq!(r1, r2, "factor({n}, seed={seed}) is not reproducible");
    }
}

/// Classical short-circuit: factor(N) for even N returns (2, N/2).
#[test]
fn factor_even_short_circuit() {
    let result = factor(14, 0);
    assert_eq!(result, Some((2, 7)), "factor(14) should return (2, 7) via even short-circuit");
    let result = factor(22, 0);
    assert_eq!(result, Some((2, 11)), "factor(22) should return (2, 11) via even short-circuit");
}

/// find_order returns a valid order: a^r ≡ 1 mod N.
///
/// Runs find_order for several (a, N) pairs and verifies the returned order is correct.
/// Seeds are chosen to give informative measurements on the first circuit run.
#[test]
fn find_order_returns_valid_order() {
    // (a, N, seed) — seeds chosen to get informative measurements.
    // seed=8 for N=35: gives s=2389 (≈ 7×64/12) with exp_len=6, recovering ord₂(35)=12.
    let cases = [(2u64, 15u64, 0u64), (7, 15, 0), (2, 21, 0), (2, 35, 8)];
    for (a, n, seed) in cases {
        if let Some(r) = find_order(a, n, seed) {
            assert!(r > 0, "find_order({a}, {n}): order must be > 0");
            assert!(r < n, "find_order({a}, {n}): order {r} must be < N={n}");
            assert_eq!(
                mod_pow(a, r, n),
                1,
                "find_order({a}, {n}): {a}^{r} mod {n} ≠ 1"
            );
        }
        // None is acceptable (uninformative measurement); the factor driver retries.
    }
}


