//! Known-answer tests (KATs) for Hensel lifting in `shared-padic`.
//!
//! All KATs use `f(x) = x² − 2` with `p = 7` and `r_0 = 3` (since `3² = 9 ≡ 2 mod 7`) unless
//! otherwise noted.
//!
//! Hand-computed Newton iteration (from PLAN.md E.D.2):
//!
//! - `r_0 = 3`: `3² = 9 ≡ 2 mod 7` ✓
//! - `r_1 = 10`: Newton step mod 49.
//!   `f(3) = 7`, `f'(3) = 6`, `inv(6) mod 49 = 41` (since `6·41 = 246 = 5·49 + 1`).
//!   `r_1 = 3 − 7·41 mod 49 = 3 − 287 mod 49 = 3 − 287 + 6·49 = 10`.
//!   Check: `10² = 100 = 2·49 + 2 ≡ 2 mod 49` ✓
//! - `r_2 = 2166`: Newton step mod 2401.
//!   `f(10) = 98`, `f'(10) = 20`, `inv(20) mod 2401 = 2281` (since `20·2281 = 45620 = 19·2401 + 1`).
//!   `r_2 = 10 − 98·2281 mod 2401 = 10 − 223538 mod 2401`.
//!   `223538 = 93·2401 + 245`, so `r_2 = 10 − 245 mod 2401 = −235 mod 2401 = 2166`.
//!   Check: `2166² mod 2401 = (2401−235)² mod 2401 = 235² mod 2401 = 55225 mod 2401`.
//!   `55225 = 23·2401 + 2`, so `2166² ≡ 2 mod 2401` ✓
//!
//! KAT coverage:
//! - `IntPoly::derivative` on a sample polynomial.
//! - `IntPoly::eval_mod` on a sample polynomial.
//! - Hensel lift of `r_0 = 3` through `f(x) = x² − 2` to mod `7^4 = 2401`, expected root `2166`.
//! - Non-simple-root error: `f(x) = x²`, `r_0 = 0`, `f'(0) = 0 mod p` → `HenselError::NonSimpleRoot`.
//! - Not-a-root error: `f(x) = x² − 2`, `r_0 = 2` (since `2² = 4 ≢ 2 mod 7`) → `HenselError::NotARoot`.

use num_bigint::BigInt;
use num_integer::Integer;
use shared_numfield::poly::IntPoly;
use shared_padic::hensel::{HenselError, hensel_lift};

// ─── helpers ──────────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// Build `f(x) = x² − 2` (coefficients least-significant first: [-2, 0, 1]).
fn f_x2_minus_2() -> IntPoly {
    IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)])
}

// ─── IntPoly::derivative KATs ─────────────────────────────────────────────────

/// `f(x) = x³ + 2x² + 3x + 4` → `f'(x) = 3x² + 4x + 3`.
///
/// Coefficients (least-significant first):
/// - `f`:  [4, 3, 2, 1]
/// - `f'`: [3, 4, 3]
#[test]
fn kat_derivative_cubic() {
    let f = IntPoly::from_coeffs(vec![bi(4), bi(3), bi(2), bi(1)]);
    let df = f.derivative();
    assert_eq!(df.coeffs, vec![bi(3), bi(4), bi(3)], "f'(x) = 3x² + 4x + 3");
}

/// `f(x) = x² − 2` → `f'(x) = 2x`.
///
/// Coefficients: `f` = [-2, 0, 1], `f'` = [0, 2] → trimmed to [0, 2].
#[test]
fn kat_derivative_quadratic() {
    let f = f_x2_minus_2();
    let df = f.derivative();
    // f'(x) = 2x: coefficients [0, 2] (constant 0 is not trailing, so it stays).
    assert_eq!(df.coeffs, vec![bi(0), bi(2)], "f'(x) = 2x: coefficients [0, 2]");
}

/// Derivative of a constant is zero.
#[test]
fn kat_derivative_constant() {
    let f = IntPoly::from_coeffs(vec![bi(5)]);
    let df = f.derivative();
    assert_eq!(df, IntPoly::zero(), "derivative of constant = 0");
}

/// Derivative of the zero polynomial is zero.
#[test]
fn kat_derivative_zero() {
    let df = IntPoly::zero().derivative();
    assert_eq!(df, IntPoly::zero(), "derivative of zero polynomial = 0");
}

// ─── IntPoly::eval_mod KATs ───────────────────────────────────────────────────

/// `f(x) = x² − 2` evaluated at `x = 3` mod `7`: `9 − 2 = 7 ≡ 0 mod 7`.
#[test]
fn kat_eval_mod_root() {
    let f = f_x2_minus_2();
    let result = f.eval_mod(&bi(3), &bi(7));
    assert_eq!(result, bi(0), "f(3) ≡ 0 mod 7");
}

/// `f(x) = x² − 2` evaluated at `x = 10` mod `49`: `100 − 2 = 98 ≡ 0 mod 49`.
#[test]
fn kat_eval_mod_root_mod49() {
    let f = f_x2_minus_2();
    let result = f.eval_mod(&bi(10), &bi(49));
    assert_eq!(result, bi(0), "f(10) ≡ 0 mod 49");
}

/// `f(x) = x³ + 2x² + 3x + 4` evaluated at `x = 2` mod `100`.
///
/// `f(2) = 8 + 8 + 6 + 4 = 26`. `26 mod 100 = 26`.
#[test]
fn kat_eval_mod_cubic() {
    let f = IntPoly::from_coeffs(vec![bi(4), bi(3), bi(2), bi(1)]);
    let result = f.eval_mod(&bi(2), &bi(100));
    assert_eq!(result, bi(26), "f(2) = 26 mod 100 = 26");
}

/// `eval_mod` with a small modulus reduces correctly.
///
/// `f(x) = x² − 2`, `x = 5`, `m = 7`: `25 − 2 = 23 ≡ 2 mod 7`.
#[test]
fn kat_eval_mod_small_modulus() {
    let f = f_x2_minus_2();
    let result = f.eval_mod(&bi(5), &bi(7));
    assert_eq!(result, bi(2), "f(5) = 23 ≡ 2 mod 7");
}

// ─── Hensel lift KATs ─────────────────────────────────────────────────────────

/// Lift `r_0 = 3` through `f(x) = x² − 2` to mod `7^1 = 7`.
///
/// k=1 is the base case; the result is just `r_0 mod p = 3`.
#[test]
fn kat_hensel_lift_k1() {
    let f = f_x2_minus_2();
    let r = hensel_lift(&f, &bi(3), &bi(7), 1).expect("lift to k=1 should succeed");
    assert_eq!(r, bi(3), "root mod 7 = 3");
}

/// Lift `r_0 = 3` through `f(x) = x² − 2` to mod `7^2 = 49`.
///
/// Hand-computed: `r_1 = 10` (see module-level comment).
#[test]
fn kat_hensel_lift_k2() {
    let f = f_x2_minus_2();
    let r = hensel_lift(&f, &bi(3), &bi(7), 2).expect("lift to k=2 should succeed");
    assert_eq!(r, bi(10), "root mod 49 = 10");
    // Verify: r² ≡ 2 mod 49.
    let r_sq_mod_49 = (&r * &r).mod_floor(&bi(49));
    assert_eq!(r_sq_mod_49, bi(2), "10² ≡ 2 mod 49");
}

/// Lift `r_0 = 3` through `f(x) = x² − 2` to mod `7^4 = 2401`.
///
/// Hand-computed: `r_2 = 2166` (see module-level comment).
/// This is the primary KAT: two Newton steps, doubling precision 1 → 2 → 4.
#[test]
fn kat_hensel_lift_k4() {
    let f = f_x2_minus_2();
    let r = hensel_lift(&f, &bi(3), &bi(7), 4).expect("lift to k=4 should succeed");
    assert_eq!(r, bi(2166), "root mod 7^4 = 2166");
    // Verify: r² ≡ 2 mod 2401.
    let r_sq_mod_2401 = (&r * &r).mod_floor(&bi(2401));
    assert_eq!(r_sq_mod_2401, bi(2), "2166² ≡ 2 mod 2401");
}

/// Lift `r_0 = 3` through `f(x) = x² − 2` to mod `7^3 = 343`.
///
/// The lift must land at exactly mod 7^3 (not overshoot to 7^4).
/// Verify: r² ≡ 2 mod 343.
#[test]
fn kat_hensel_lift_k3() {
    let f = f_x2_minus_2();
    let r = hensel_lift(&f, &bi(3), &bi(7), 3).expect("lift to k=3 should succeed");
    // r must be in [0, 343).
    assert!(r >= bi(0) && r < bi(343), "root mod 7^3 must be in [0, 343), got {r}");
    // Verify: r² ≡ 2 mod 343.
    let r_sq_mod_343 = (&r * &r).mod_floor(&bi(343));
    assert_eq!(r_sq_mod_343, bi(2), "r² ≡ 2 mod 343");
}

// ─── non-simple-root error KAT ────────────────────────────────────────────────

/// `f(x) = x²`, `r_0 = 0`: `f'(x) = 2x`, `f'(0) = 0 ≡ 0 mod 7` → non-simple root.
///
/// This is the uniqueness-precondition guard: a non-simple root has no unique lift.
#[test]
fn kat_hensel_nonroot_error() {
    // f(x) = x²: coefficients [0, 0, 1].
    let f = IntPoly::from_coeffs(vec![bi(0), bi(0), bi(1)]);
    let result = hensel_lift(&f, &bi(0), &bi(7), 4);
    assert!(
        matches!(result, Err(HenselError::NonSimpleRoot { .. })),
        "f'(0) = 0 mod 7 must return NonSimpleRoot, got: {result:?}"
    );
}

// ─── not-a-root error KAT ─────────────────────────────────────────────────────

/// `f(x) = x² − 2`, `r_0 = 2`: `f(2) = 2 ≢ 0 mod 7` → not a root.
#[test]
fn kat_hensel_not_a_root_error() {
    let f = f_x2_minus_2();
    // f(2) = 4 - 2 = 2 ≢ 0 mod 7
    let result = hensel_lift(&f, &bi(2), &bi(7), 4);
    assert!(
        matches!(result, Err(HenselError::NotARoot { .. })),
        "r_0=2 is not a root of f mod 7, must return NotARoot, got: {result:?}"
    );
}
