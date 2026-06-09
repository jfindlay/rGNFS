//! Pohlig–Hellman ECDLP substrate: order factorization and prime-subgroup projection.
//!
//! This module provides the substrate E.A.2 builds on:
//!
//! - [`factor_order`] — prime-power decomposition of a u64 group order.
//! - [`project_to_subgroup`] — map `(G, Q)` to the order-`p^e` subgroup via
//!   `[n / p^e]`-scalar-multiplication.
//!
//! # Scope
//!
//! Toy-order-scoped (u64 group orders). `factor_order` uses trial division via
//! `shared_numth::trial_smooth` over a `√n` factor base, with an `is_prime`
//! short-circuit on the cofactor. For orders beyond u64 (not needed here),
//! `shared_numth::ecm_factor` is available as a fallback — noted as a
//! principle-4 annotation, not wired.

use crypto_bigint::Uint;

use shared_numth::{factor_base_up_to, is_prime, trial_smooth};

use crate::curve::{AffinePoint, Curve};
use crate::field::Fp;

// ── factor_order ──────────────────────────────────────────────────────────────

/// Compute the prime-power decomposition of a u64 group order.
///
/// Returns a sorted `Vec<(prime, exponent)>` such that `∏ pᵢ^{eᵢ} = n`.
/// Uses trial division over a `√n` factor base (`factor_base_up_to(isqrt(n))`)
/// with an `is_prime` short-circuit on the remaining cofactor.
///
/// Toy-order-scoped (u64). For orders that require ECM, `shared_numth::ecm_factor`
/// is available as a fallback (principle-4 annotation — not wired here).
///
/// # Panics
///
/// Panics if `n == 0`.
pub fn factor_order(n: u64) -> Vec<(u64, u32)> {
    assert!(n != 0, "factor_order: n must be non-zero");

    if n == 1 {
        return vec![];
    }

    // Build a factor base up to √n for trial division.
    let sqrt_n = isqrt(n);
    let base = factor_base_up_to(sqrt_n);

    let n_uint = Uint::<4>::from(n);
    let witness = trial_smooth(&n_uint, &base);

    let mut factors = witness.factors.clone();

    // The cofactor is whatever trial_smooth could not factor.
    // If cofactor > 1, it must be prime (since the base covers all primes ≤ √n,
    // and any composite cofactor would have a factor ≤ √cofactor ≤ √n).
    let cofactor_uint = witness.cofactor;
    if cofactor_uint > Uint::<4>::ONE {
        // Sanity: the cofactor should be prime.
        debug_assert!(
            is_prime(&cofactor_uint),
            "factor_order: cofactor is composite — base too small?"
        );
        let cofactor_u64 = cofactor_uint.as_words()[0];
        factors.push((cofactor_u64, 1));
        factors.sort_unstable_by_key(|&(p, _)| p);
    }

    factors
}

// ── isqrt ─────────────────────────────────────────────────────────────────────

/// Integer square root: largest `k` such that `k² ≤ n`.
fn isqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as f64).sqrt() as u64;
    // Correct for floating-point rounding.
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

// ── project_to_subgroup ───────────────────────────────────────────────────────

/// Project `(G, Q)` to the order-`p^e` subgroup of a curve of order `n`.
///
/// Given a curve of order `n` with `p^e | n`, maps both the base point and the
/// target to the unique subgroup of order `p^e` by multiplying by the cofactor
/// `n / p^e`. Returns `(G', Q')` where `G' = [n/p^e]·G` and `Q' = [n/p^e]·Q`.
///
/// `G'` generates the order-`p^e` subgroup (provided `G` has full order `n`).
/// The DLP `Q' = k·G'` in the subgroup gives `k mod p^e`.
///
/// # Preconditions
///
/// - `p^e` must divide `n` exactly.
/// - `G` must have full order `n` (not a subgroup generator).
pub fn project_to_subgroup<F: Fp<4>>(
    curve: &Curve,
    g: &AffinePoint<F>,
    q: &AffinePoint<F>,
    n: u64,
    p_power: u64,
) -> (AffinePoint<F>, AffinePoint<F>) {
    debug_assert!(n % p_power == 0, "project_to_subgroup: p^e does not divide n");
    let cofactor = n / p_power;
    let cofactor_uint = Uint::<4>::from(cofactor);
    let g_sub = curve.scalar_mul(g, &cofactor_uint);
    let q_sub = curve.scalar_mul(q, &cofactor_uint);
    (g_sub, q_sub)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::curve::test_curves::{composite_toy, COMPOSITE_TOY_FACTORS, COMPOSITE_TOY_N};
    use crate::field::FpMonty;

    // ── factor_order KATs ─────────────────────────────────────────────────────

    /// factor_order reproduces the recorded factorization of the composite fixture.
    #[test]
    fn factor_order_composite_toy() {
        let factors = factor_order(COMPOSITE_TOY_N);
        assert_eq!(
            factors, COMPOSITE_TOY_FACTORS,
            "factor_order({COMPOSITE_TOY_N}) = {factors:?}, expected {COMPOSITE_TOY_FACTORS:?}"
        );
    }

    /// The product of p^e for all factors equals COMPOSITE_TOY_N.
    #[test]
    fn factor_order_product_equals_n() {
        let factors = factor_order(COMPOSITE_TOY_N);
        let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
        assert_eq!(
            product, COMPOSITE_TOY_N,
            "∏ pᵢ^eᵢ = {product}, expected {COMPOSITE_TOY_N}"
        );
    }

    /// factor_order handles small known values correctly.
    #[test]
    fn factor_order_small_cases() {
        assert_eq!(factor_order(1), vec![]);
        assert_eq!(factor_order(2), vec![(2, 1)]);
        assert_eq!(factor_order(4), vec![(2, 2)]);
        assert_eq!(factor_order(12), vec![(2, 2), (3, 1)]);
        assert_eq!(factor_order(60), vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(factor_order(360), vec![(2, 3), (3, 2), (5, 1)]);
    }

    /// factor_order handles prime inputs correctly.
    #[test]
    fn factor_order_prime() {
        assert_eq!(factor_order(7), vec![(7, 1)]);
        assert_eq!(factor_order(97), vec![(97, 1)]);
    }

    // ── project_to_subgroup KATs ──────────────────────────────────────────────

    /// Projecting to the order-4 subgroup (p=2, e=2) gives a point of order 4.
    #[test]
    fn project_order_4_subgroup() {
        let curve = composite_toy();
        let g: AffinePoint<FpMonty> = curve.generator();
        let n = COMPOSITE_TOY_N;
        // p^e = 4 (prime 2, exponent 2)
        let (g_sub, _) = project_to_subgroup(&curve, &g, &g, n, 4);
        // g_sub should have order 4: 4*g_sub = infinity
        let four_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(4u64));
        assert!(four_g_sub.is_infinity(), "project order-4: 4*G' should be ∞");
        // And 2*g_sub != infinity (order is exactly 4, not 2)
        let two_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(2u64));
        assert!(!two_g_sub.is_infinity(), "project order-4: 2*G' should not be ∞");
    }

    /// Projecting to the order-3 subgroup gives a point of order 3.
    #[test]
    fn project_order_3_subgroup() {
        let curve = composite_toy();
        let g: AffinePoint<FpMonty> = curve.generator();
        let n = COMPOSITE_TOY_N;
        // p^e = 3
        let (g_sub, _) = project_to_subgroup(&curve, &g, &g, n, 3);
        let three_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(3u64));
        assert!(three_g_sub.is_infinity(), "project order-3: 3*G' should be ∞");
        let one_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(1u64));
        assert!(!one_g_sub.is_infinity(), "project order-3: G' should not be ∞");
    }

    /// Projecting to the order-5 subgroup gives a point of order 5.
    #[test]
    fn project_order_5_subgroup() {
        let curve = composite_toy();
        let g: AffinePoint<FpMonty> = curve.generator();
        let n = COMPOSITE_TOY_N;
        // p^e = 5
        let (g_sub, _) = project_to_subgroup(&curve, &g, &g, n, 5);
        let five_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(5u64));
        assert!(five_g_sub.is_infinity(), "project order-5: 5*G' should be ∞");
        let one_g_sub = curve.scalar_mul(&g_sub, &Uint::<4>::from(1u64));
        assert!(!one_g_sub.is_infinity(), "project order-5: G' should not be ∞");
    }

    // ── isqrt unit tests ──────────────────────────────────────────────────────

    /// isqrt returns the correct integer square root for small values.
    #[test]
    fn isqrt_small() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(3), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(60), 7);
        assert_eq!(isqrt(100), 10);
    }
}
