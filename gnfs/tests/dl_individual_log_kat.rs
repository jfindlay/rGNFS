//! End-to-end individual-log KAT (full pipeline integration vehicle).
//!
//! Recovers a known discrete log end-to-end through the full path:
//! VirtualLogTable construction → initialization-smoothing → descent → assembly.
//!
//! # Toy setup
//!
//! - p = 11, g = 2 (primitive root mod 11, order 10).
//! - ℓ = 5 (prime factor of p − 1 = 10).
//! - Factor base: rational primes {2, 3} (b_rat = 3, b_alg = 3).
//! - Virtual logs (mod 5): log_2(2) = 1, log_2(3) = 3.
//!
//! # Subgroup recovery note
//!
//! ℓ = 5 is a proper prime factor of p − 1 = 10. The recovered log is mod 5 only.
//! For the full log mod 10, Pohlig–Hellman / CRT across the order's factors would be needed
//! (out of scope for the individual-log descent). The toy KAT uses ℓ = 5 to match the
//! existing toy DL setup.
//!
//! To sidestep Pohlig–Hellman entirely, one would pick ℓ = p − 1 = 10 (the full group order).
//! However, 10 is not prime, so the F_ℓ linear algebra would need to work over ℤ/10ℤ (not a
//! field). The toy KAT uses ℓ = 5 (prime) for clean F_ℓ arithmetic.
//!
//! # Known discrete logs mod 5
//!
//! In (ℤ/11ℤ)*, with g = 2 (order 10):
//! - 2^1 = 2  → log_2(2) = 1 mod 5
//! - 2^2 = 4  → log_2(4) = 2 mod 5
//! - 2^3 = 8  → log_2(8) = 3 mod 5
//! - 2^6 = 9  → log_2(9) = 6 mod 10 → 1 mod 5
//! - 2^8 = 3  → log_2(3) = 8 mod 10 → 3 mod 5
//! - 2^9 = 6  → log_2(6) = 9 mod 10 → 4 mod 5
//!
//! All h values above are smooth over {2, 3} (no descent needed at toy scale).
//!
//! # Pipeline
//!
//! 1. Build VirtualLogTable with known virtual logs [1, 3] mod 5.
//! 2. Build SolveDlContext with the toy polynomial pair and factor base.
//! 3. Call solve_dl_full(g, h, p, 1, ell, &ctx) for each h.
//! 4. Assert Ok(known_log) and cross-check g^known_log mod p == h.
//!
//! # PARI cross-check
//!
//! The PARI oracle `znlog(Mod(h, 11), Mod(2, 11))` returns log_2(h) mod 10.
//! Reduced mod 5: the result should match the recovered log (or a scalar multiple).
//! The PARI stub is gated with `#[ignore]` — no subprocess in CI.

use crypto_bigint::Uint;
use gnfs::dl::{
    DescentSieveConfig, SolveDlContext, SolveDlError, VirtualLogTable, solve_dl_full,
};
use gnfs::{FactorBase, PolyPair};
use num_bigint::BigInt;
use shared_field::{Fp, FpNaive4};
use shared_numfield::IntPoly;

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

fn ell5() -> Uint<4> {
    Uint::<4>::from(5u64)
}

/// Convert `FpNaive4` to `BigInt` for assembly arithmetic.
///
/// For small values (< 2^64), the first word of `to_uint()` is the canonical residue.
fn fp_to_bigint(f: &FpNaive4) -> BigInt {
    BigInt::from(f.to_uint().as_words()[0])
}

/// Modular exponentiation: base^exp mod modulus.
fn mod_pow(base: &BigInt, exp: &BigInt, modulus: &BigInt) -> BigInt {
    use num_traits::Zero;
    if exp.is_zero() {
        return BigInt::from(1);
    }
    let mut result = BigInt::from(1);
    let mut b = base.clone() % modulus;
    let mut e = exp.clone();
    while e > BigInt::from(0) {
        if &e % 2 == BigInt::from(1) {
            result = (result * &b) % modulus;
        }
        b = (&b * &b) % modulus;
        e /= 2;
    }
    result
}

/// `f(x) = x³ − x − 1` (coefficients: [−1, −1, 0, 1]).
fn f_cubic() -> IntPoly {
    IntPoly::from_coeffs(vec![bi(-1), bi(-1), bi(0), bi(1)])
}

/// Build the toy polynomial pair: `f(x) = x³ − x − 1`, `m = 2`, `n = 5`.
fn toy_poly_pair() -> PolyPair {
    let f = f_cubic();
    let m = bi(2);
    let n = bi(5);
    let g = IntPoly::from_coeffs(vec![-m.clone(), bi(1)]);
    let pair = PolyPair::new(f, g, m, n);
    pair.verify().expect("toy polynomial pair should be valid");
    pair
}

// ─── End-to-end individual-log KAT ───────────────────────────────────────────

/// End-to-end individual-log KAT: recover `log_2(h)` mod 5 for multiple h values.
///
/// # Setup
///
/// - p = 11, g = 2, ell = 5.
/// - Factor base: rational primes {2, 3} (b_rat = 3, b_alg = 3).
/// - Virtual logs: log_2(2) = 1 mod 5, log_2(3) = 3 mod 5 (known, bypassing the solver).
///
/// # Test cases
///
/// Each h is smooth over {2, 3} (no descent needed), so the pipeline reduces to:
/// init_descent_frontier → run_descent (leaf lookup only) → assemble_log.
///
/// | h  | factorization | log_2(h) mod 10 | log_2(h) mod 5 |
/// |----|---------------|-----------------|----------------|
/// | 2  | 2             | 1               | 1              |
/// | 4  | 2^2           | 2               | 2              |
/// | 8  | 2^3           | 3               | 3              |
/// | 3  | 3             | 8               | 3              |
/// | 9  | 3^2           | 6               | 1              |
/// | 6  | 2 * 3         | 9               | 4              |
///
/// # Toy KAT note
///
/// ell = 5 is a prime factor of p − 1 = 10. The log is recovered mod 5 only.
/// // Toy KAT: ell = p-1 would give the full group order, but p-1=10 is not prime.
/// // We use ell = 5 (prime) for clean F_ℓ arithmetic. The log is mod 5.
#[test]
fn kat_individual_log_end_to_end() {
    let poly = toy_poly_pair();
    // Factor base: b_rat = 3, b_alg = 3 (rational primes {2, 3}).
    let fb = FactorBase::new(&poly.f, 3, 3);
    let ell = bi(5);
    let g = bi(2);
    let p = bi(11);

    // Build the context with known virtual logs.
    let ell5 = ell5();
    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![
            FpNaive4::from_u64(1, &ell5), // log_2(2) = 1 mod 5
            FpNaive4::from_u64(3, &ell5), // log_2(3) = 3 mod 5
        ],
        algebraic_logs: vec![],
    };
    let sieve_cfg = DescentSieveConfig::new(10, 5);
    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // Test cases: (h, expected_log_mod_5).
    // All h values are smooth over {2, 3} (no descent needed).
    let test_cases: &[(i64, i64)] = &[
        (2, 1), // log_2(2) = 1 mod 5
        (4, 2), // log_2(4) = 2 mod 5
        (8, 3), // log_2(8) = 3 mod 5
        (3, 3), // log_2(3) = 3 mod 5
        (9, 1), // log_2(9) = 6 mod 10 → 1 mod 5
        (6, 4), // log_2(6) = 9 mod 10 → 4 mod 5
    ];

    for &(h_val, expected_log) in test_cases {
        let h = bi(h_val);
        let expected = bi(expected_log);

        let result = solve_dl_full(&g, &h, &p, 1, &ell, &ctx);

        assert_eq!(
            result,
            Ok(expected.clone()),
            "solve_dl_full: log_2({h_val}) should be {expected_log} mod 5; got: {:?}",
            result
        );

        // Cross-check: g^log mod p == h.
        let log = result.unwrap();
        let g_pow = mod_pow(&g, &log, &p);
        // Note: g^log mod p == h only when log is the true log mod (p-1).
        // Since log is mod 5 (a factor of p-1=10), g^log mod p may not equal h directly.
        // Instead, verify that g^(log + 5*k) mod p == h for some k (i.e., log is correct mod 5).
        // Equivalently: g^log mod p is in the same coset as h in the subgroup of order 5.
        // Simplest check: g^log mod p == h OR g^(log+5) mod p == h.
        let g_pow_plus5 = mod_pow(&g, &(log.clone() + bi(5)), &p);
        assert!(
            g_pow == h || g_pow_plus5 == h,
            "cross-check: g^{log} mod {p} = {g_pow} (or g^{} mod {p} = {g_pow_plus5}), \
             expected {h}",
            log.clone() + bi(5)
        );
    }
}

/// End-to-end individual-log KAT: full pipeline with F_ℓ solve.
///
/// This KAT exercises the full pipeline including the F_ℓ linear system:
/// DLMatrix construction → F_ℓ matrix build → block Lanczos solve →
/// VirtualLogTable → solve_dl_full.
///
/// # Setup
///
/// Uses the same toy DLMatrix as `dl_end_to_end_kat.rs`:
/// - 3 relations × 2 columns (rational primes {2, 3}), no algebraic, no Schirokauer.
/// - Each row [e0, e1] satisfies e0 + 3*e1 ≡ 0 (mod 5) (the kernel condition for [1, 3]).
///
/// # Expected result
///
/// The recovered virtual logs [x0, x1] satisfy x0 * 3 ≡ x1 (mod 5) (ratio 1:3).
/// For h = 4 = 2^2: log_2(4) = 2 * x0 / x0 * ... (depends on the scalar).
///
/// # Note on scalar ambiguity
///
/// The F_ℓ solver returns a kernel vector determined up to a nonzero scalar. The virtual
/// logs [x0, x1] may be [1, 3] or [2, 6≡1] or [3, 9≡4] or [4, 12≡2] (mod 5). The
/// individual log for h = 4 is computed as 2 * x0 (since 4 = 2^2), which gives:
/// - If [x0, x1] = [1, 3]: log_2(4) = 2 * 1 = 2 mod 5. ✓
/// - If [x0, x1] = [2, 1]: log_2(4) = 2 * 2 = 4 mod 5. (wrong scalar)
/// - etc.
///
/// To handle scalar ambiguity, we verify g^result mod p == h (the true cross-check).
/// If the scalar is wrong, the cross-check will fail. We try multiple seeds to find
/// the canonical kernel vector [1, 3].
#[test]
fn kat_individual_log_full_pipeline() {
    use gnfs::dl::{
        DLMatrix, DLRelation, FlMatrixOperator, FlSparseMatrix, FlSolution, VirtualLogTable,
        block_lanczos_fl, block_wiedemann_fl, build_fl_matrix, recover_virtual_logs,
    };
    use gnfs::sieve::{ExponentVector, Relation};

    let ell5 = ell5();
    let ell = bi(5);
    let g = bi(2);
    let p = bi(11);

    // Build the toy DLMatrix (same as dl_end_to_end_kat.rs).
    // Rows [3,4], [4,2], [2,1] — all satisfy e0 + 3*e1 ≡ 0 (mod 5).
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
        DLRelation::new(relation, vec![])
    };

    let dl_matrix = DLMatrix {
        relations: vec![make_rel(3, 4), make_rel(4, 2), make_rel(2, 1)],
        rational_size: 2,
        algebraic_size: 0,
        schirokauer_rank: 0,
    };

    // Build the F_ℓ matrix and solve.
    let fl_matrix: FlSparseMatrix<FpNaive4> = build_fl_matrix(&dl_matrix, &ell5);
    let op = FlMatrixOperator::<FpNaive4, 4>::new(&fl_matrix);

    // Find a nontrivial kernel vector.
    let mut found_solution: Option<FlSolution<FpNaive4>> = None;
    'outer: for seed in [0u64, 1, 2, 3, 42, 137, 999, 12345] {
        for sols in [
            block_lanczos_fl::<FpNaive4, 4>(&op, &ell5, seed),
            block_wiedemann_fl::<FpNaive4, 4>(&op, &ell5, seed),
        ] {
            for sol in sols {
                if sol.coefficients.iter().any(|c| !c.is_zero(&ell5)) {
                    found_solution = Some(sol);
                    break 'outer;
                }
            }
        }
    }

    let sol = found_solution.expect("should find a nontrivial kernel vector");

    // Recover virtual logs.
    let raw_vtable: VirtualLogTable<FpNaive4> = recover_virtual_logs(&sol, 2, 0);

    // Normalize the virtual logs so that log_2(2) = 1 mod 5.
    // The F_ℓ solver returns a kernel vector determined up to a nonzero scalar c.
    // The raw virtual log of prime 2 is x0 = c * 1 mod 5. To normalize, multiply
    // all logs by x0^{-1} mod 5 so that log_2(2) = 1.
    //
    // This normalization is required because the individual log depends on the scalar.
    // Without normalization, the recovered log is only correct up to scalar.
    let x0 = &raw_vtable.rational_logs[0];
    assert!(!x0.is_zero(&ell5), "virtual log of prime 2 should be nonzero");
    let x0_inv = x0.inv(&ell5);

    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: raw_vtable
            .rational_logs
            .iter()
            .map(|l| l.mul(&x0_inv, &ell5))
            .collect(),
        algebraic_logs: raw_vtable
            .algebraic_logs
            .iter()
            .map(|l| l.mul(&x0_inv, &ell5))
            .collect(),
    };

    // After normalization: log_2(2) = 1 mod 5, log_2(3) = 3 mod 5.
    assert_eq!(
        vtable.rational_logs[0].to_uint(),
        Uint::<4>::from(1u64),
        "normalized log_2(2) should be 1 mod 5"
    );
    assert_eq!(
        vtable.rational_logs[1].to_uint(),
        Uint::<4>::from(3u64),
        "normalized log_2(3) should be 3 mod 5"
    );

    // Build the toy polynomial pair and factor base.
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);
    let sieve_cfg = DescentSieveConfig::new(10, 5);

    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // Test with h = 4 = 2^2. After normalization, log_2(4) = 2 mod 5.
    let h = bi(4);
    let result = solve_dl_full(&g, &h, &p, 1, &ell, &ctx);

    assert_eq!(
        result,
        Ok(bi(2)),
        "solve_dl_full: log_2(4) should be 2 mod 5 after normalization; got: {:?}",
        result
    );

    let log = result.unwrap();

    // Cross-check: g^log mod p == h (or g^(log+5) mod p == h for the other coset lift).
    let g_pow = mod_pow(&g, &log, &p);
    let g_pow_lift = mod_pow(&g, &(log.clone() + bi(5)), &p);
    assert!(
        g_pow == h || g_pow_lift == h,
        "cross-check: g^{log} mod {p} = {g_pow} (or g^{} = {g_pow_lift}), expected {h}",
        log.clone() + bi(5)
    );
}

/// End-to-end individual-log KAT: verify `solve_dl_full` with k=2 is no longer `Unsupported`.
///
/// The k=2 extension field path is wired. `solve_dl_full` with k=2 delegates to `solve_dl`
/// (which builds the extension context internally). The result must NOT be `Unsupported`.
#[test]
fn kat_individual_log_k2_not_unsupported() {
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);
    let ell5 = ell5();
    let ell = bi(5);

    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![FpNaive4::from_u64(1, &ell5)],
        algebraic_logs: vec![],
    };
    let sieve_cfg = DescentSieveConfig::new(10, 5);
    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    // k=2 is wired (extension field path); must NOT return Unsupported.
    // Note: p=11, k=2 — the k=2 path will try to find an irreducible poly of degree 2 over F_11.
    let result = solve_dl_full(&bi(2), &bi(4), &bi(11), 2, &ell, &ctx);
    assert!(
        !matches!(result, Err(SolveDlError::Unsupported { .. })),
        "k=2 must not return Unsupported (extension field path wired); got: {:?}",
        result
    );
}

/// End-to-end individual-log KAT: verify `solve_dl_full` returns `Unsupported` for k > 2.
///
/// k=3 is beyond the toy ceiling (k>2). `solve_dl_full` must return `Unsupported` for k>2.
#[test]
fn kat_individual_log_unsupported_k3() {
    let poly = toy_poly_pair();
    let fb = FactorBase::new(&poly.f, 3, 3);
    let ell5 = ell5();
    let ell = bi(5);

    let vtable = VirtualLogTable::<FpNaive4> {
        rational_logs: vec![FpNaive4::from_u64(1, &ell5)],
        algebraic_logs: vec![],
    };
    let sieve_cfg = DescentSieveConfig::new(10, 5);
    let ctx = SolveDlContext {
        poly: &poly,
        fb: &fb,
        vtable: &vtable,
        sieve_cfg,
        to_bigint: Box::new(fp_to_bigint),
    };

    let result = solve_dl_full(&bi(2), &bi(4), &bi(11), 3, &ell, &ctx);
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 3 })),
        "k=3 should return Unsupported; got: {:?}",
        result
    );
}

// ─── PARI cross-check stub ────────────────────────────────────────────────────

/// PARI cross-check: verify the individual-log result against PARI's discrete log.
///
/// Run manually:
/// ```text
/// echo 'znlog(Mod(4, 11), Mod(2, 11))' | gp -q
/// ```
/// Expected output: 2 (= log_2(4) mod 10). Reduced mod 5: 2.
///
/// For other h values:
/// - `znlog(Mod(8, 11), Mod(2, 11))` → 3 (log_2(8) mod 10 = 3, mod 5 = 3)
/// - `znlog(Mod(3, 11), Mod(2, 11))` → 8 (log_2(3) mod 10 = 8, mod 5 = 3)
/// - `znlog(Mod(6, 11), Mod(2, 11))` → 9 (log_2(6) mod 10 = 9, mod 5 = 4)
///
/// The recovered log (mod 5) should match `znlog(Mod(h, 11), Mod(2, 11)) mod 5`.
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn kat_pari_individual_log_oracle() {
    // PARI cross-check: run manually.
    // echo 'znlog(Mod(4, 11), Mod(2, 11))' | gp -q
    // Expected: 2 (log_2(4) mod 10). Reduced mod 5: 2.
    todo!("run: echo 'znlog(Mod(4, Mod(2, 11)))' | gp -q")
}
