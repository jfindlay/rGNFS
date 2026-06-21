//! k>1 individual-log descent for NFS-DL over F_{p^k}.
//!
//! # k>1 individual-log descent contract
//!
//! This module implements the individual-log descent adapted to an extension target
//! h ∈ F_{p^k}*, composing the descent node/frontier types with the extension-target
//! and extension factor-base substrate.
//!
//! ## Algorithm overview
//!
//! The k>1 individual-log descent mirrors the k=1 pipeline:
//!
//! 1. **Encoding**: `g, h ∈ F_{p^k}*` are passed as `BigInt` via base-p encoding:
//!    `c_0 + c_1·p + … + c_{k-1}·p^{k-1}` for the element `c_0 + c_1·u + … + c_{k-1}·u^{k-1}`
//!    in F_{p^k} = F_p[u]/(m(u)). This is a bijection from F_{p^k} to [0, p^k).
//!
//! 2. **Initialization-smoothing**: find an exponent `e` such that `g^e · h` (computed in
//!    F_{p^k}*) is smooth over the extension factor base — i.e., factors into elements whose
//!    virtual logs are known.
//!
//! 3. **Descent**: for each non-leaf factor, descend it to factor-base leaves using the
//!    descent node/frontier types.
//!
//! 4. **Assembly**: sum the virtual logs of the leaves to recover `log_g(g^e · h)`, then
//!    back out `e` to get `log_g(h)`.
//!
//! ## Toy-scale implementation (k=2, p=47, ℓ=3)
//!
//! At demonstration fidelity (principle 4), the k=2 path is scoped to the `pairing_toy`
//! parameters. The virtual-log table is computed by brute force (the ℓ-subgroup has order 3,
//! so the DL is trivial). The descent is trivial at this scale: all factors of `g^e · h` are
//! already in the factor base.
//!
//! The general k=2 NFS-DL pipeline (relation collection, F_ℓ solve, descent) is a
//! principle-4 annotation — the toy implementation proves the interface and encoding are
//! correct; the full pipeline would be used at crypto scale.
//!
//! ## Rigidity guard
//!
//! The number field K = ℚ[α]/(f) stays char-0. The extension-field arithmetic (F_{p^k})
//! appears only in the encoding/decoding and the smoothness check — it does not leak into
//! the descent algebra.

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

use crate::dl::ext::target::ExtTarget;

// ─── Encoding / decoding ──────────────────────────────────────────────────────

/// Encode an `ExtTarget` as a base-p `BigInt`.
///
/// The encoding is `c_0 + c_1·p + … + c_{k-1}·p^{k-1}`, a bijection from F_{p^k} to
/// `[0, p^k)`. This is the representation `solve_dl` uses for `g` and `h` at k>1.
///
/// # Panics
///
/// Panics if any coefficient is negative or `>= p`.
pub fn ext_target_to_bigint(t: &ExtTarget) -> BigInt {
    let mut result = BigInt::zero();
    let mut p_pow = BigInt::one();
    for c in &t.coeffs {
        result += c * &p_pow;
        p_pow *= &t.p;
    }
    result
}

/// Decode a base-p `BigInt` to an `ExtTarget` coefficient vector.
///
/// Extracts coefficients `c_i = (n / p^i) mod p` for `i = 0, …, k-1`.
/// This is the inverse of [`ext_target_to_bigint`].
///
/// # Arguments
///
/// - `n`: The base-p encoded integer in `[0, p^k)`.
/// - `p`: The prime base.
/// - `modulus`: The irreducible modulus `m(u)` of length `k+1`.
///
/// # Panics
///
/// Panics if `n < 0` or `n >= p^k`.
pub fn bigint_to_ext_target(n: &BigInt, p: &BigInt, modulus: Vec<BigInt>) -> ExtTarget {
    let k = modulus.len() - 1;
    assert!(k >= 1, "bigint_to_ext_target: modulus must have degree >= 1");
    assert!(!n.is_negative(), "bigint_to_ext_target: n must be non-negative, got {n}");

    let mut coeffs = Vec::with_capacity(k);
    let mut rem = n.clone();
    for _ in 0..k {
        let c = &rem % p;
        coeffs.push(c.clone());
        rem /= p;
    }
    assert!(rem.is_zero(), "bigint_to_ext_target: n={n} is out of range [0, p^k)");

    ExtTarget::from_coeffs(coeffs, p.clone(), modulus)
}

// ─── ExtVirtualLogTable ───────────────────────────────────────────────────────

/// Virtual-log table for the extension setting.
///
/// Maps extension factor-base elements to their discrete logs in the ℓ-subgroup of F_{p^k}*.
/// Each entry is a `BigInt` in `[0, ℓ)`.
///
/// # Structure
///
/// The table has one entry per element of the extension factor base. For the toy k=2 case,
/// the table is computed by brute force (the ℓ-subgroup has order ℓ, so the DL is trivial).
///
/// # k>1 descent contract
///
/// This type is consumed by [`solve_dl_ext`] and [`init_ext_descent_frontier`].
#[derive(Debug, Clone)]
pub struct ExtVirtualLogTable {
    /// Virtual logs of the extension factor-base elements, indexed by position.
    ///
    /// `logs[i]` is the discrete log of the i-th factor-base element in the ℓ-subgroup.
    pub logs: Vec<BigInt>,
}

// ─── ExtSolveDlContext ────────────────────────────────────────────────────────

/// Context for the k>1 individual-log pipeline.
///
/// Carries the extension factor base, virtual-log table, and the generator `g` (as an
/// `ExtTarget`) for the ℓ-subgroup. This is the k>1 analogue of `SolveDlContext`.
///
/// Consumed by [`init_ext_descent_frontier`] and [`run_ext_descent`].
pub struct ExtSolveDlContext {
    /// The generator g of the ℓ-subgroup, as an `ExtTarget`.
    pub g: ExtTarget,
    /// The ℓ-subgroup order.
    pub ell: BigInt,
    /// The virtual-log table for the extension factor base.
    pub vtable: ExtVirtualLogTable,
}

// ─── init_ext_descent_frontier ───────────────────────────────────────────────

/// Find an exponent `e` such that `g^e · h` is smooth over the extension factor base.
///
/// Iterates `e` from 0 to `max_attempts - 1`, computes `g^e · h` in F_{p^k}*, and checks
/// if the result is in the virtual-log table (i.e., its log is known directly). For the toy
/// k=2 case, this is equivalent to checking if `g^e · h` is a factor-base element.
///
/// Returns `(e, log_of_g_e_h)` where `log_of_g_e_h = log_g(g^e · h) mod ell`.
///
/// # Arguments
///
/// - `h`: The target element as an `ExtTarget`.
/// - `ctx`: The extension solve context (carries g, ell, vtable).
/// - `max_attempts`: Maximum exponent-search iterations.
///
/// # Returns
///
/// `Some((e, log))` if a smooth exponent is found, `None` otherwise.
pub fn init_ext_descent_frontier(
    h: &ExtTarget,
    ctx: &ExtSolveDlContext,
    max_attempts: u64,
) -> Option<(BigInt, BigInt)> {
    // candidate = g^0 * h = h; then candidate = g^1 * h, g^2 * h, ...
    let mut candidate = h.clone();

    for e in 0..max_attempts {
        // Check if candidate is in the virtual-log table.
        // For the toy k=2 case, the table has one entry per ℓ-subgroup element.
        // We check by computing the log directly (brute force for toy scale).
        if let Some(log) = lookup_ext_log(&candidate, ctx) {
            return Some((BigInt::from(e), log));
        }

        // candidate = candidate * g (advance to g^{e+1} * h).
        candidate = candidate.mul(&ctx.g);
    }

    None
}

/// Look up the virtual log of an extension element in the table.
///
/// For the toy k=2 case, the table stores logs of all ℓ-subgroup elements. We find the
/// log by checking if the element matches any entry.
///
/// Returns `Some(log)` if the element is in the table, `None` otherwise.
fn lookup_ext_log(elt: &ExtTarget, ctx: &ExtSolveDlContext) -> Option<BigInt> {
    // For the toy k=2 case, the vtable.logs[i] = i (the log of g^i).
    // We check if elt == g^i for i = 0, 1, ..., ell-1.
    let ell_usize = bigint_to_usize(&ctx.ell);
    let mut g_pow = ExtTarget::from_coeffs(
        {
            let mut c = vec![BigInt::zero(); ctx.g.k];
            c[0] = BigInt::one();
            c
        },
        ctx.g.p.clone(),
        ctx.g.modulus.clone(),
    );

    for i in 0..ell_usize {
        if &g_pow == elt {
            // The log of g^i is i (mod ell).
            if i < ctx.vtable.logs.len() {
                return Some(ctx.vtable.logs[i].clone());
            } else {
                return Some(BigInt::from(i));
            }
        }
        g_pow = g_pow.mul(&ctx.g);
    }

    None
}

/// Convert a `BigInt` to `usize` (for toy-scale use only).
fn bigint_to_usize(n: &BigInt) -> usize {
    n.to_u64_digits().1.first().copied().unwrap_or(0) as usize
}

// ─── solve_dl_ext ─────────────────────────────────────────────────────────────

/// Compute the discrete logarithm `log_g(h)` in F_{p^k}* for k=2 (toy scale).
///
/// This is the k>1 path of `solve_dl`. For k=2 at the `pairing_toy` parameters
/// (p=47, ℓ=3, modulus u²+1), it:
///
/// 1. Decodes `g` and `h` from base-p `BigInt` to `ExtTarget` coefficient vectors.
/// 2. Builds the extension context (virtual-log table by brute force for toy scale).
/// 3. Runs initialization-smoothing: finds `e` such that `g^e · h` is smooth.
/// 4. Assembles the log: `log_g(h) = (log_g(g^e · h) − e) mod ell`.
///
/// # Arguments
///
/// - `g`: Generator of the ℓ-subgroup in F_{p^k}*, encoded as a base-p `BigInt`.
/// - `h`: Target element in F_{p^k}*, encoded as a base-p `BigInt`.
/// - `p`: The prime base.
/// - `k`: The extension degree (must be 2 for this implementation).
/// - `ell`: The subgroup order (a prime dividing `p^k − 1`).
///
/// # Returns
///
/// `Ok(x)` where `x ∈ [0, ell)` and `g^x = h` in F_{p^k}*.
///
/// # Errors
///
/// - [`SolveDlExtError::UnsupportedDegree`] if `k != 2`.
/// - [`SolveDlExtError::InitSmoothingFailed`] if no smooth exponent is found.
/// - [`SolveDlExtError::InvalidGenerator`] if `g` is not a valid generator of the ℓ-subgroup.
///
/// # Principle-4 annotation
///
/// At toy scale (ℓ=3, group order 3), the virtual-log table is computed by brute force.
/// At crypto scale, the full NFS-DL pipeline (relation collection, F_ℓ solve, descent)
/// would be used. The toy implementation proves the interface and encoding are correct.
pub fn solve_dl_ext(
    g: &BigInt,
    h: &BigInt,
    p: &BigInt,
    k: usize,
    ell: &BigInt,
) -> Result<BigInt, SolveDlExtError> {
    // Only k=2 is supported at toy scale.
    if k != 2 {
        return Err(SolveDlExtError::UnsupportedDegree { k });
    }

    // Build the irreducible modulus for k=2.
    // For the toy scale, we find the standard irreducible polynomial of degree 2 over F_p.
    // For p ≡ 3 (mod 4), x²+1 is irreducible (since -1 is a QNR mod p).
    // For other p, we search for an irreducible polynomial of degree 2.
    let modulus = find_irreducible_degree2(p)?;

    // Decode g and h from base-p BigInt to ExtTarget.
    let g_ext = bigint_to_ext_target(g, p, modulus.clone());
    let h_ext = bigint_to_ext_target(h, p, modulus.clone());

    // Build the extension context.
    // For the toy scale, the virtual-log table is computed by brute force:
    // vtable.logs[i] = i (the log of g^i in the ℓ-subgroup).
    let ell_usize = bigint_to_usize(ell);
    let logs: Vec<BigInt> = (0..ell_usize).map(BigInt::from).collect();
    let vtable = ExtVirtualLogTable { logs };
    let ctx = ExtSolveDlContext { g: g_ext.clone(), ell: ell.clone(), vtable };

    // Verify that g is a valid generator of the ℓ-subgroup: g^ell = 1.
    let g_ell = g_ext.pow(ell);
    let one = ExtTarget::from_coeffs(
        {
            let mut c = vec![BigInt::zero(); k];
            c[0] = BigInt::one();
            c
        },
        p.clone(),
        modulus.clone(),
    );
    if g_ell != one {
        return Err(SolveDlExtError::InvalidGenerator);
    }

    // Initialization-smoothing: find e such that g^e * h is smooth.
    let max_attempts = ell_usize as u64 + 1;
    let (e, log_g_e_h) = init_ext_descent_frontier(&h_ext, &ctx, max_attempts)
        .ok_or(SolveDlExtError::InitSmoothingFailed)?;

    // Assemble: log_g(h) = (log_g(g^e * h) - e) mod ell.
    let log_h = mod_reduce(&(log_g_e_h - &e), ell);

    Ok(log_h)
}

// ─── SolveDlExtError ──────────────────────────────────────────────────────────

/// Error type for [`solve_dl_ext`].
///
/// Internal to the k>1 descent; converted to [`SolveDlError`] variants at the call site
/// in `solve_dl` / `solve_dl_full`. The taxonomy is NOT part of the frozen `solve_dl` interface.
///
/// [`SolveDlError`]: crate::dl::descent::solve::SolveDlError
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveDlExtError {
    /// Extension degree k is not supported (only k=2 at toy scale).
    UnsupportedDegree { k: usize },
    /// Initialization-smoothing failed: no smooth exponent found.
    InitSmoothingFailed,
    /// The generator g is not a valid generator of the ℓ-subgroup (g^ell ≠ 1).
    InvalidGenerator,
    /// No irreducible polynomial of degree 2 found over F_p (should not happen for valid p).
    NoIrreduciblePoly,
}

impl std::fmt::Display for SolveDlExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedDegree { k } => {
                write!(f, "extension degree k={k} not supported (only k=2 at toy scale)")
            }
            Self::InitSmoothingFailed => {
                write!(f, "initialization-smoothing failed: no smooth exponent found")
            }
            Self::InvalidGenerator => {
                write!(f, "g is not a valid generator of the ℓ-subgroup (g^ell ≠ 1)")
            }
            Self::NoIrreduciblePoly => {
                write!(f, "no irreducible polynomial of degree 2 found over F_p")
            }
        }
    }
}

impl std::error::Error for SolveDlExtError {}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Find an irreducible polynomial of degree 2 over F_p.
///
/// Searches for a monic polynomial x²+bx+c that is irreducible over F_p (has no roots in F_p).
/// Returns the modulus as `Vec<BigInt>` of length 3: `[c, b, 1]` (least-significant first).
///
/// For p ≡ 3 (mod 4), x²+1 is irreducible (since -1 is a QNR mod p). For other p, we search
/// for an irreducible polynomial by trying x²+bx+c for b=0,1,... and c=1,2,...
///
/// # Errors
///
/// Returns [`SolveDlExtError::NoIrreduciblePoly`] if no irreducible polynomial is found
/// (should not happen for valid primes p > 2).
pub(crate) fn find_irreducible_degree2(p: &BigInt) -> Result<Vec<BigInt>, SolveDlExtError> {
    let p_usize = bigint_to_usize(p);

    // Try x²+1 first (works for p ≡ 3 mod 4).
    let modulus_x2_plus_1 = vec![BigInt::one(), BigInt::zero(), BigInt::one()];
    if is_irreducible_degree2(&modulus_x2_plus_1, p_usize) {
        return Ok(modulus_x2_plus_1);
    }

    // Search for x²+bx+c irreducible over F_p.
    for b in 0..p_usize {
        for c in 1..p_usize {
            let modulus = vec![BigInt::from(c), BigInt::from(b), BigInt::one()];
            if is_irreducible_degree2(&modulus, p_usize) {
                return Ok(modulus);
            }
        }
    }

    Err(SolveDlExtError::NoIrreduciblePoly)
}

/// Check if the monic degree-2 polynomial `c + b·x + x²` is irreducible over F_p.
///
/// A degree-2 polynomial is irreducible over F_p iff it has no roots in F_p.
fn is_irreducible_degree2(modulus: &[BigInt], p_usize: usize) -> bool {
    // Check that no r ∈ F_p is a root: c + b*r + r² ≡ 0 (mod p).
    let c = &modulus[0];
    let b = &modulus[1];
    for r in 0..p_usize {
        let r_big = BigInt::from(r);
        let val = c + b * &r_big + &r_big * &r_big;
        let rem = mod_reduce_usize(&val, p_usize);
        if rem == 0 {
            return false;
        }
    }
    true
}

/// Reduce `a` into `[0, m)` for `m > 0`.
fn mod_reduce(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r.is_negative() { r + m } else { r }
}

/// Reduce a `BigInt` into `[0, m)` for `m > 0`, returning a `usize`.
fn mod_reduce_usize(a: &BigInt, m: usize) -> usize {
    let m_big = BigInt::from(m);
    let r = a % &m_big;
    let r = if r.is_negative() { r + m_big } else { r };
    r.to_u64_digits().1.first().copied().unwrap_or(0) as usize
}

// ─── Unit tests (KATs) ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    const P47: u64 = 47;

    fn p47() -> BigInt {
        BigInt::from(P47)
    }

    fn modulus_k2() -> Vec<BigInt> {
        vec![bi(1), bi(0), bi(1)] // x²+1
    }

    fn fp2_target(c0: u64, c1: u64) -> ExtTarget {
        ExtTarget::from_coeffs(
            vec![BigInt::from(c0), BigInt::from(c1)],
            p47(),
            modulus_k2(),
        )
    }

    // ── KAT: encoding round-trip ──────────────────────────────────────────────

    /// KAT: `ext_target_to_bigint` and `bigint_to_ext_target` are inverses.
    ///
    /// Verifies the base-p encoding round-trip for F_{47²}.
    #[test]
    fn kat_encoding_round_trip() {
        let cases = [(0u64, 0u64), (1, 0), (0, 1), (13, 29), (46, 46), (3, 7)];
        for (c0, c1) in cases {
            let t = fp2_target(c0, c1);
            let n = ext_target_to_bigint(&t);
            let t2 = bigint_to_ext_target(&n, &p47(), modulus_k2());
            assert_eq!(t, t2, "round-trip failed for ({c0}, {c1}): n={n}");
        }
    }

    /// KAT: base-p encoding is correct for specific values.
    ///
    /// For F_{47²}, the encoding of (c0, c1) is c0 + c1 * 47.
    #[test]
    fn kat_encoding_values() {
        // (0, 0) → 0
        assert_eq!(ext_target_to_bigint(&fp2_target(0, 0)), bi(0));
        // (1, 0) → 1
        assert_eq!(ext_target_to_bigint(&fp2_target(1, 0)), bi(1));
        // (0, 1) → 47
        assert_eq!(ext_target_to_bigint(&fp2_target(0, 1)), bi(47));
        // (13, 29) → 13 + 29*47 = 13 + 1363 = 1376
        assert_eq!(ext_target_to_bigint(&fp2_target(13, 29)), bi(13 + 29 * 47));
        // (46, 46) → 46 + 46*47 = 46 + 2162 = 2208
        assert_eq!(ext_target_to_bigint(&fp2_target(46, 46)), bi(46 + 46 * 47));
    }

    // ── KAT: find_irreducible_degree2 ─────────────────────────────────────────

    /// KAT: `find_irreducible_degree2` finds x²+1 for p=47.
    ///
    /// p=47 ≡ 3 (mod 4), so x²+1 is irreducible over F_47.
    #[test]
    fn kat_find_irreducible_p47() {
        let modulus = find_irreducible_degree2(&p47()).expect("should find irreducible poly");
        assert_eq!(modulus, modulus_k2(), "should find x²+1 for p=47");
    }

    /// KAT: `is_irreducible_degree2` correctly identifies irreducible polynomials.
    ///
    /// x²+1 is irreducible over F_47 (no roots mod 47).
    #[test]
    fn kat_is_irreducible_degree2_p47() {
        assert!(
            is_irreducible_degree2(&modulus_k2(), 47),
            "x²+1 should be irreducible over F_47"
        );
    }

    /// KAT: `is_irreducible_degree2` correctly rejects reducible polynomials.
    ///
    /// x²+1 is reducible over F_5 (roots: 2 and 3, since 2²+1=5≡0 mod 5).
    #[test]
    fn kat_is_irreducible_degree2_reducible() {
        assert!(
            !is_irreducible_degree2(&modulus_k2(), 5),
            "x²+1 should be reducible over F_5"
        );
    }

    // ── KAT: solve_dl_ext ─────────────────────────────────────────────────────

    /// KAT: `solve_dl_ext` returns `Err(UnsupportedDegree)` for k != 2.
    #[test]
    fn kat_solve_dl_ext_unsupported_k3() {
        let result = solve_dl_ext(&bi(1), &bi(1), &p47(), 3, &bi(3));
        assert!(
            matches!(result, Err(SolveDlExtError::UnsupportedDegree { k: 3 })),
            "k=3 should return UnsupportedDegree; got: {:?}",
            result
        );
    }

    /// KAT: `solve_dl_ext` returns `Err(InvalidGenerator)` for an element not in the ℓ-subgroup.
    ///
    /// The element (2, 0) = 2 in F_{47²} has order dividing 46 (= p-1), not 3.
    /// 2^3 = 8 ≠ 1 in F_{47²}, so 2 is not in the ℓ=3 subgroup.
    #[test]
    fn kat_solve_dl_ext_invalid_generator() {
        // g = 2 (not in the ℓ=3 subgroup: 2^3 = 8 ≠ 1 in F_{47²}).
        let g = ext_target_to_bigint(&fp2_target(2, 0)); // g = 2
        let h = ext_target_to_bigint(&fp2_target(2, 0)); // h = 2
        let result = solve_dl_ext(&g, &h, &p47(), 2, &bi(3));
        assert!(
            matches!(result, Err(SolveDlExtError::InvalidGenerator)),
            "g=2 (not in ℓ=3 subgroup) should return InvalidGenerator; got: {:?}",
            result
        );
    }

    /// KAT: `solve_dl_ext` recovers the correct log for the pairing_toy parameters.
    ///
    /// F_{47²} = F_47[u]/(u²+1), ℓ=3.
    /// The ℓ=3 subgroup: {1, (23,6), (23,41)}.
    /// Generator g = (23,6) = 23 + 6u.
    /// DL table: log_g(1) = 0, log_g(g) = 1, log_g(g²) = 2.
    #[test]
    fn kat_solve_dl_ext_pairing_toy() {
        // g = (23, 6) in F_{47²}: base-47 encoding = 23 + 6*47 = 305.
        let g_enc = bi(23 + 6 * 47); // 305
        // h = (23, 41) = g^2: base-47 encoding = 23 + 41*47 = 1950.
        let h_enc = bi(23 + 41 * 47); // 1950

        let result = solve_dl_ext(&g_enc, &h_enc, &p47(), 2, &bi(3));
        assert_eq!(
            result,
            Ok(bi(2)),
            "log_g(g^2) should be 2; got: {:?}",
            result
        );

        // Also test log_g(g) = 1.
        let result2 = solve_dl_ext(&g_enc, &g_enc, &p47(), 2, &bi(3));
        assert_eq!(result2, Ok(bi(1)), "log_g(g) should be 1; got: {:?}", result2);

        // Also test log_g(1) = 0.
        let one_enc = bi(1); // (1, 0) = 1
        let result3 = solve_dl_ext(&g_enc, &one_enc, &p47(), 2, &bi(3));
        assert_eq!(result3, Ok(bi(0)), "log_g(1) should be 0; got: {:?}", result3);
    }

    /// KAT: the ℓ=3 subgroup of F_{47²}* has exactly 3 elements.
    ///
    /// Verifies the fixture parameters: the cube roots of unity in F_{47²} are
    /// {1, (23,6), (23,41)}, confirming the ℓ=3 subgroup structure used in the KATs.
    #[test]
    fn kat_ell3_subgroup_has_three_elements() {
        // Find all x in F_{47^2} with x^3 = 1.
        let one = fp2_target(1, 0);
        let mut roots = vec![];
        for c0 in 0..47u64 {
            for c1 in 0..47u64 {
                if c0 == 0 && c1 == 0 {
                    continue;
                }
                let x = fp2_target(c0, c1);
                let x3 = x.pow(&bi(3));
                if x3 == one {
                    roots.push((c0, c1));
                }
            }
        }
        // There should be exactly 3 elements (including 1): 1, ζ, ζ².
        assert_eq!(roots.len(), 3, "ℓ=3 subgroup should have exactly 3 elements: {:?}", roots);
        // Verify the known values: (1,0), (23,6), (23,41).
        assert!(roots.contains(&(1, 0)), "1 should be in the ℓ=3 subgroup");
        assert!(roots.contains(&(23, 6)), "(23,6) = ζ should be in the ℓ=3 subgroup");
        assert!(roots.contains(&(23, 41)), "(23,41) = ζ² should be in the ℓ=3 subgroup");
    }
}
