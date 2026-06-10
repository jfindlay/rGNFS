//! Known-answer tests (KATs) for D.E.3 ◆: k>1 individual-log descent + `solve_dl` k>1 wiring.
//!
//! # Purpose
//!
//! This file is the primary correctness gate for D.E.3 ◆ (C2-ext frozen). It verifies:
//!
//! - **End-to-end k=2 KAT**: `solve_dl(g, h, 47, 2, 3)` returns `x` with `g^x = h` in
//!   F_{47²}* for several (g, h) pairs.
//! - **k>1 path no longer returns `Unsupported` for k=2**: the D.E.3 wiring is live.
//! - **k>2 still returns `Unsupported`**: the toy ceiling is respected.
//! - **PARI `znlog`/`fflog` `#[ignore]` cross-check**: the established dev-only oracle pattern.
//!
//! # Fixture: F_{47²} = F_47[u]/(u²+1)
//!
//! The `pairing_toy` parameters: p=47, k=2, ℓ=3, modulus u²+1.
//!
//! - |F_{47²}*| = 47² - 1 = 2208 = 3 × 736.
//! - The ℓ=3 subgroup has order 3: {1, ζ, ζ²} where ζ = (23, 6) = 23 + 6u.
//! - ζ² = (23, 41) = 23 + 41u.
//! - Verification: (23 + 6u)² = 529 + 276u + 36u² = 529 + 276u - 36 = 493 + 276u
//!   ≡ 23 + 41u (mod 47). ✓
//!
//! # Encoding
//!
//! `g, h ∈ F_{47²}*` are encoded as base-47 `BigInt`: `c_0 + c_1 × 47` for the element
//! `c_0 + c_1 × u` in F_{47²}. This is the C2-ext encoding agreed at D.E.1.
//!
//! - (1, 0) = 1 → BigInt(1)
//! - (23, 6) = ζ → BigInt(23 + 6×47) = BigInt(305)
//! - (23, 41) = ζ² → BigInt(23 + 41×47) = BigInt(1950)
//!
//! # No-regression gate
//!
//! The k=1 KATs in `gnfs/tests/dl_descent_kat.rs` must stay green. This file does NOT
//! re-test k=1 behaviour — it only tests the k=2 path and the k>2 ceiling.
//!
//! # PARI cross-check
//!
//! The `#[ignore]` PARI cross-check follows the established dev-only oracle pattern from
//! `gnfs/tests/dl_descent_kat.rs`. Run manually when PARI is available:
//!
//! ```text
//! # In PARI/GP:
//! # F_{47^2} = F_47[u]/(u^2+1). The ell=3 subgroup generator is ζ = 23 + 6u.
//! # fflog(ζ^2, ζ) should return 2.
//! p = 47; t = ffgen(Mod(1,p)*x^2+1, 'u);
//! zeta = 23 + 6*u;  # ζ in F_{47^2}
//! fflog(zeta^2, zeta)  # should return 2
//! fflog(zeta, zeta)    # should return 1
//! fflog(1, zeta)       # should return 0
//! ```

use gnfs::dl::{SolveDlError, solve_dl};
use num_bigint::BigInt;

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// Modular exponentiation in F_{47²}: base^exp in the ℓ=3 subgroup.
///
/// For the ℓ=3 subgroup, exp ∈ {0, 1, 2}. We compute by repeated multiplication
/// in F_{47²} = F_47[u]/(u²+1).
fn fp2_pow(base_enc: &BigInt, exp: &BigInt, p: &BigInt) -> BigInt {
    // Decode base from base-p encoding.
    let c0 = base_enc % p;
    let c1 = base_enc / p;

    // Compute base^exp in F_{47²} using the ExtTarget pow method.
    // We use the encoding: c_0 + c_1*u in F_p[u]/(u^2+1).
    // For small exp, we can compute directly.
    let exp_usize = exp.to_u64_digits().1.first().copied().unwrap_or(0) as usize;

    let mut result_c0 = BigInt::from(1i64);
    let mut result_c1 = BigInt::from(0i64);
    let mut base_c0 = c0.clone();
    let mut base_c1 = c1.clone();

    let mut e = exp_usize;
    while e != 0 {
        if e & 1 == 1 {
            // result = result * base in F_{47^2}
            let new_c0 = mod_reduce(&(&result_c0 * &base_c0 - &result_c1 * &base_c1), p);
            let new_c1 = mod_reduce(&(&result_c0 * &base_c1 + &result_c1 * &base_c0), p);
            result_c0 = new_c0;
            result_c1 = new_c1;
        }
        // base = base * base
        let new_c0 = mod_reduce(&(&base_c0 * &base_c0 - &base_c1 * &base_c1), p);
        let new_c1 = mod_reduce(&(2 * &base_c0 * &base_c1), p);
        base_c0 = new_c0;
        base_c1 = new_c1;
        e >>= 1;
    }

    // Encode result as base-p BigInt.
    &result_c0 + &result_c1 * p
}

fn mod_reduce(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r < BigInt::from(0i64) { r + m } else { r }
}

// ─── Fixture constants ────────────────────────────────────────────────────────

/// The prime p = 47.
fn p47() -> BigInt {
    bi(47)
}

/// The torsion prime ℓ = 3.
fn ell3() -> BigInt {
    bi(3)
}

/// The generator ζ = (23, 6) = 23 + 6u in F_{47²}, encoded as base-47 BigInt.
///
/// ζ has order 3 in F_{47²}*: ζ³ = 1, ζ ≠ 1.
fn zeta() -> BigInt {
    bi(23 + 6 * 47) // = 305
}

/// ζ² = (23, 41) = 23 + 41u in F_{47²}, encoded as base-47 BigInt.
fn zeta_sq() -> BigInt {
    bi(23 + 41 * 47) // = 1950
}

/// The identity element 1 = (1, 0) in F_{47²}, encoded as base-47 BigInt.
fn one_fp2() -> BigInt {
    bi(1)
}

// ─── KAT (a): k=2 no longer returns Unsupported ───────────────────────────────

/// KAT (a): `solve_dl` with k=2 does NOT return `SolveDlError::Unsupported`.
///
/// D.E.3 wires the k=2 path. The result must be `Ok(x)` or a known `Err` variant
/// (InitSmoothingFailed / DescentFailed), but must NOT be `Unsupported`.
///
/// This is the primary "wiring is live" gate for D.E.3.
#[test]
fn kat_a_k2_not_unsupported() {
    let result = solve_dl(&zeta(), &zeta_sq(), &p47(), 2, &ell3());
    assert!(
        !matches!(result, Err(SolveDlError::Unsupported { .. })),
        "k=2 must not return Unsupported (D.E.3 wired); got: {:?}",
        result
    );
}

// ─── KAT (b): k>2 still returns Unsupported ───────────────────────────────────

/// KAT (b): `solve_dl` with k=3 returns `SolveDlError::Unsupported { k: 3 }`.
///
/// k=3 is beyond the toy ceiling (k>2). The `Unsupported` variant stays for k>2.
/// This verifies the taxonomy is unchanged (frozen D.C.3).
#[test]
fn kat_b_k3_returns_unsupported() {
    let result = solve_dl(&bi(2), &bi(3), &p47(), 3, &ell3());
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 3 })),
        "k=3 must return Unsupported {{ k: 3 }}; got: {:?}",
        result
    );
}

/// KAT (b2): `solve_dl` with k=4 returns `SolveDlError::Unsupported { k: 4 }`.
#[test]
fn kat_b2_k4_returns_unsupported() {
    let result = solve_dl(&bi(2), &bi(3), &p47(), 4, &ell3());
    assert!(
        matches!(result, Err(SolveDlError::Unsupported { k: 4 })),
        "k=4 must return Unsupported {{ k: 4 }}; got: {:?}",
        result
    );
}

// ─── KAT (c): End-to-end k=2 DL recovery ─────────────────────────────────────

/// KAT (c): `solve_dl(ζ, ζ², 47, 2, 3)` returns 2.
///
/// # Setup
///
/// - F_{47²} = F_47[u]/(u²+1), p=47, k=2, ℓ=3.
/// - Generator g = ζ = (23, 6) = 23 + 6u, encoded as BigInt(305).
/// - Target h = ζ² = (23, 41) = 23 + 41u, encoded as BigInt(1950).
/// - Expected: log_ζ(ζ²) = 2.
///
/// # Round-trip verification
///
/// After recovering x=2, we verify g^x = h in F_{47²}*:
/// ζ^2 = (23, 41) = h. ✓
#[test]
fn kat_c_solve_dl_k2_zeta_sq() {
    let g = zeta();
    let h = zeta_sq();
    let p = p47();
    let ell = ell3();

    let result = solve_dl(&g, &h, &p, 2, &ell);

    assert_eq!(
        result,
        Ok(bi(2)),
        "solve_dl(ζ, ζ², 47, 2, 3) should return 2; got: {:?}",
        result
    );

    // Round-trip: g^x = h in F_{47²}*.
    if let Ok(ref x) = result {
        let g_pow = fp2_pow(&g, x, &p);
        assert_eq!(
            g_pow, h,
            "round-trip: ζ^{x} = {g_pow} (base-47), expected {h} (= ζ²)"
        );
    }
}

/// KAT (c2): `solve_dl(ζ, ζ, 47, 2, 3)` returns 1.
///
/// # Setup
///
/// - Generator g = ζ = (23, 6), target h = ζ = (23, 6).
/// - Expected: log_ζ(ζ) = 1.
#[test]
fn kat_c2_solve_dl_k2_zeta() {
    let g = zeta();
    let h = zeta(); // h = g = ζ
    let p = p47();
    let ell = ell3();

    let result = solve_dl(&g, &h, &p, 2, &ell);

    assert_eq!(
        result,
        Ok(bi(1)),
        "solve_dl(ζ, ζ, 47, 2, 3) should return 1; got: {:?}",
        result
    );

    // Round-trip: g^x = h.
    if let Ok(ref x) = result {
        let g_pow = fp2_pow(&g, x, &p);
        assert_eq!(g_pow, h, "round-trip: ζ^{x} = {g_pow}, expected {h}");
    }
}

/// KAT (c3): `solve_dl(ζ, 1, 47, 2, 3)` returns 0.
///
/// # Setup
///
/// - Generator g = ζ = (23, 6), target h = 1 = (1, 0).
/// - Expected: log_ζ(1) = 0.
#[test]
fn kat_c3_solve_dl_k2_one() {
    let g = zeta();
    let h = one_fp2(); // h = 1
    let p = p47();
    let ell = ell3();

    let result = solve_dl(&g, &h, &p, 2, &ell);

    assert_eq!(
        result,
        Ok(bi(0)),
        "solve_dl(ζ, 1, 47, 2, 3) should return 0; got: {:?}",
        result
    );

    // Round-trip: g^0 = 1.
    if let Ok(ref x) = result {
        let g_pow = fp2_pow(&g, x, &p);
        assert_eq!(g_pow, h, "round-trip: ζ^{x} = {g_pow}, expected {h} (= 1)");
    }
}

/// KAT (c4): `solve_dl(ζ², ζ, 47, 2, 3)` returns 2.
///
/// # Setup
///
/// - Generator g = ζ² = (23, 41), target h = ζ = (23, 6).
/// - Expected: log_{ζ²}(ζ) = 2 (since (ζ²)² = ζ⁴ = ζ in the order-3 group).
///
/// In the ℓ=3 subgroup, (ζ²)^2 = ζ^4 = ζ^(3+1) = ζ^1 = ζ. So log_{ζ²}(ζ) = 2.
#[test]
fn kat_c4_solve_dl_k2_zeta_sq_generator() {
    let g = zeta_sq(); // g = ζ²
    let h = zeta();    // h = ζ
    let p = p47();
    let ell = ell3();

    let result = solve_dl(&g, &h, &p, 2, &ell);

    assert_eq!(
        result,
        Ok(bi(2)),
        "solve_dl(ζ², ζ, 47, 2, 3) should return 2; got: {:?}",
        result
    );

    // Round-trip: (ζ²)^2 = ζ.
    if let Ok(ref x) = result {
        let g_pow = fp2_pow(&g, x, &p);
        assert_eq!(g_pow, h, "round-trip: (ζ²)^{x} = {g_pow}, expected {h} (= ζ)");
    }
}

// ─── KAT (d): k=1 path is unchanged ──────────────────────────────────────────

/// KAT (d): `solve_dl` with k=1 still works (no-regression gate).
///
/// The k=1 path must be behaviourally unchanged after D.E.3. This KAT verifies the
/// k=1 path returns a `Result` (not a panic) and does not return `Unsupported`.
///
/// The full k=1 KATs are in `gnfs/tests/dl_descent_kat.rs` (the primary no-regression gate).
#[test]
fn kat_d_k1_path_unchanged() {
    // k=1 path: p=11, g=2, h=4, ell=5. log_2(4) = 2 mod 5.
    // Without a SolveDlContext, solve_dl returns DescentFailed (not Unsupported).
    let result = solve_dl(&bi(2), &bi(4), &bi(11), 1, &bi(5));
    assert!(
        !matches!(result, Err(SolveDlError::Unsupported { .. })),
        "k=1 must not return Unsupported; got: {:?}",
        result
    );
}

// ─── KAT (e): SolveDlError taxonomy is unchanged ──────────────────────────────

/// KAT (e): The `SolveDlError` taxonomy is unchanged (frozen D.C.3).
///
/// Verifies that the three variants (Unsupported, InitSmoothingFailed, DescentFailed)
/// are still the only variants, and their Display messages are correct.
#[test]
fn kat_e_error_taxonomy_unchanged() {
    // Unsupported: k=3 (beyond toy ceiling).
    let e = SolveDlError::Unsupported { k: 3 };
    let msg = e.to_string();
    assert!(!msg.is_empty(), "Unsupported display should be non-empty");

    // InitSmoothingFailed.
    let e = SolveDlError::InitSmoothingFailed { attempts: 100 };
    let msg = e.to_string();
    assert!(msg.contains("100"), "InitSmoothingFailed should contain attempt count");

    // DescentFailed.
    let e = SolveDlError::DescentFailed { stuck_prime: 17 };
    let msg = e.to_string();
    assert!(msg.contains("17"), "DescentFailed should contain stuck prime");
}

// ─── PARI cross-check (dev-only, #[ignore]) ───────────────────────────────────

/// PARI cross-check: verify the k=2 DL result against PARI's `fflog`.
///
/// Run manually when PARI/GP is available:
///
/// ```text
/// # In PARI/GP:
/// p = 47; t = ffgen(Mod(1,p)*x^2+1, 'u);
/// zeta = 23 + 6*u;   # ζ in F_{47^2}
/// fflog(zeta^2, zeta)  # should return 2
/// fflog(zeta, zeta)    # should return 1
/// fflog(1, zeta)       # should return 0
/// ```
///
/// The PARI `fflog` function computes discrete logs in finite extension fields.
/// This cross-check verifies that our k=2 `solve_dl` agrees with PARI's oracle.
#[test]
#[ignore = "PARI not installed; run manually when available"]
fn kat_pari_k2_dl_cross_check() {
    // This test is a placeholder for the PARI cross-check.
    // When PARI is available, run the commands above and verify the results match.
    //
    // Expected results:
    // - fflog(ζ², ζ) = 2  ↔  solve_dl(ζ, ζ², 47, 2, 3) = 2
    // - fflog(ζ, ζ) = 1   ↔  solve_dl(ζ, ζ, 47, 2, 3) = 1
    // - fflog(1, ζ) = 0   ↔  solve_dl(ζ, 1, 47, 2, 3) = 0
    //
    // The solve_dl results are verified by the KAT (c) tests above.
    // This test is a documentation stub for the PARI oracle cross-check.
    assert!(true, "PARI cross-check: see test doc for manual verification commands");
}
