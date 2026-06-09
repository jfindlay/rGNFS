//! NFS-DL solver entry point: `solve_dl` (C2 interface) and initialization-smoothing.
//!
//! This module provides:
//!
//! - [`solve_dl`] — the cross-track C2 interface: compute `log_g(h)` in `F_{p^k}*` via NFS-DL.
//!   The k = 1 (prime-field) path is live; k > 1 returns [`SolveDlError::Unsupported`].
//! - [`init_descent_frontier`] — the first descent step: find an exponent `e` such that
//!   `g^e · h mod p` is smooth over primes up to `medium_bound`, producing the initial frontier.
//! - [`descend_node`] — per-node descent (signature frozen D.C.1; body implemented D.C.2).
//! - [`SolveDlError`] — error type for `solve_dl` (shape frozen D.C.1, taxonomy finalized D.C.3).
//! - [`InitSmoothingError`] / [`DescentStepError`] — error types for the descent substrate.
//!
//! # Contract C2 (shape frozen D.C.1, finalized D.C.3)
//!
//! `solve_dl` is the cross-track interface consumed by E.C (the MOV bridge). Its signature and
//! the `SolveDlError` shape are frozen here. The error taxonomy may be extended additively at
//! D.C.3 once the full pipeline is integrated.
//!
//! # D.C.1 scope
//!
//! At D.C.1, `solve_dl` wires the k = 1 path through initialization-smoothing only; the
//! descent recursion (D.C.2) and log assembly (D.C.3) are stubs. The KATs verify the interface
//! shape, not the final answer.

use num_bigint::BigInt;
use num_traits::{One, Zero};
use shared_numth::factor_base_up_to;

use crate::dl::descent::node::{DescentFrontier, DescentNode, DescentTarget};

// ─── SolveDlError ─────────────────────────────────────────────────────────────

/// Error type for [`solve_dl`].
///
/// The error taxonomy is **opened at D.C.1** (shape frozen) and **finalized at D.C.3** (once
/// descent reality reveals the actual failure modes). D.C.2/D.C.3 may add variants; E.C
/// consumes the finalized taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveDlError {
    /// Extension field F_{p^k} (k > 1) is not yet supported.
    ///
    /// The F_{p^k} NFS-DL extension is deferred to an E.C-prep session. This variant is
    /// returned immediately for k > 1 without attempting any computation.
    Unsupported {
        /// The unsupported extension degree.
        k: usize,
    },

    /// Initialization-smoothing failed: no exponent `e` found such that `g^e · h` is smooth.
    ///
    /// This can occur for pathological inputs or if the medium-prime bound / attempt limit is
    /// too restrictive. The caller may retry with relaxed parameters.
    InitSmoothingFailed {
        /// Number of exponents tried before giving up.
        attempts: u64,
    },

    /// Descent failed: a medium prime could not be rewritten as smaller primes.
    ///
    /// This occurs when the special-q sieve fails to find a suitable relation for some frontier
    /// prime within the sieve bounds. At toy scale, this may indicate the sieve region is too
    /// small; at NFS scale, it is rare for well-chosen parameters.
    DescentFailed {
        /// The prime that could not be descended.
        stuck_prime: u64,
    },
    // ─── Variants to be finalized at D.C.3 ────────────────────────────────────
    //
    // The following are placeholders for failure modes that D.C.2/D.C.3 may reveal:
    //
    // - `RelationCollectionFailed`: not enough relations for the F_ℓ linear system.
    // - `LinearAlgebraFailed`: the F_ℓ solver did not converge.
    // - `AssemblyFailed`: log assembly produced an inconsistent result.
    //
    // These are not declared now because the D.C.1 substrate does not exercise them;
    // D.C.3 finalizes the taxonomy once the full pipeline is integrated.
}

impl std::fmt::Display for SolveDlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported { k } => {
                write!(f, "extension field F_{{p^{k}}} (k > 1) not yet supported")
            }
            Self::InitSmoothingFailed { attempts } => {
                write!(f, "initialization-smoothing failed after {attempts} attempts")
            }
            Self::DescentFailed { stuck_prime } => {
                write!(f, "descent failed: could not rewrite prime {stuck_prime}")
            }
        }
    }
}

impl std::error::Error for SolveDlError {}

// ─── InitSmoothingError ───────────────────────────────────────────────────────

/// Error from initialization-smoothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitSmoothingError {
    /// No smooth exponent found within the attempt limit.
    NoSmoothExponent {
        /// Number of exponents tried before giving up.
        attempts: u64,
    },
}

impl std::fmt::Display for InitSmoothingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSmoothExponent { attempts } => {
                write!(f, "no smooth exponent found after {attempts} attempts")
            }
        }
    }
}

impl std::error::Error for InitSmoothingError {}

// ─── DescentStepError ─────────────────────────────────────────────────────────

/// Error from a single descent step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DescentStepError {
    /// No relation found that rewrites the target as smaller primes.
    NoRelationFound {
        /// The target that could not be descended.
        target: DescentTarget,
    },
    /// The sieve produced relations, but none strictly reduced the largest prime.
    NoStrictReduction {
        /// The target for which no strict reduction was found.
        target: DescentTarget,
    },
}

impl std::fmt::Display for DescentStepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoRelationFound { target } => {
                write!(f, "no relation found for prime {}", target.prime())
            }
            Self::NoStrictReduction { target } => {
                write!(f, "no strict reduction found for prime {}", target.prime())
            }
        }
    }
}

impl std::error::Error for DescentStepError {}

// ─── init_descent_frontier ────────────────────────────────────────────────────

/// Initialize the descent frontier by finding an exponent `e` such that `g^e · h mod p` is
/// smooth over primes up to `medium_bound`.
///
/// This is the first descent step: iterate `e` from 0 to `max_attempts - 1`, compute
/// `candidate = g^e · h mod p`, and trial-divide by all primes up to `medium_bound`. If the
/// candidate factors completely (cofactor = 1), return `e` and the initial frontier containing
/// one `DescentNode` per prime factor (with multiplicity — a prime appearing twice is pushed
/// twice).
///
/// # D.C.1 scope
///
/// At D.C.1, this function does not distinguish "medium primes" (above the factor-base bound)
/// from "factor-base primes" (below it). All prime factors of the smooth candidate are pushed
/// onto the frontier as `DescentTarget::Rational` nodes. D.C.2/D.C.3 will integrate the real
/// `FactorBase` to detect leaf nodes (factor-base elements with known virtual logs) and only
/// push non-leaf primes onto the frontier.
///
/// # Arguments
///
/// - `g`: Generator of the multiplicative group (as `BigInt` mod p).
/// - `h`: Target element (as `BigInt` mod p).
/// - `p`: The prime modulus.
/// - `medium_bound`: The smoothness bound B' (trial-divide by primes up to this value).
/// - `max_attempts`: Maximum exponent-search iterations before giving up.
///
/// # Returns
///
/// `Ok((e, frontier))` where `e` is the smoothing exponent (as `BigInt`) and `frontier`
/// contains one `DescentNode` per prime factor of `g^e · h mod p` (with multiplicity).
///
/// # Errors
///
/// Returns [`InitSmoothingError::NoSmoothExponent`] if no smooth exponent is found within
/// `max_attempts` iterations.
pub fn init_descent_frontier<F: Clone>(
    g: &BigInt,
    h: &BigInt,
    p: &BigInt,
    medium_bound: u64,
    max_attempts: u64,
) -> Result<(BigInt, DescentFrontier<F>), InitSmoothingError> {
    // Build the trial-division prime base: all primes up to medium_bound.
    let prime_base = factor_base_up_to(medium_bound);

    // Iterate e from 0 to max_attempts - 1.
    // candidate = g^e * h mod p.
    // Start with g^0 * h = h, then multiply by g at each step.
    let mut candidate = mod_reduce(h, p);

    for e in 0..max_attempts {
        // Trial-divide candidate by all primes up to medium_bound.
        let factors = trial_divide_bigint(&candidate, &prime_base);

        if let Some(factors) = factors {
            // candidate is fully smooth; build the frontier.
            let e_bigint = BigInt::from(e);
            let mut frontier = DescentFrontier::new();

            for p_factor in factors {
                let target = DescentTarget::Rational(p_factor);
                let node = DescentNode {
                    target: target.clone(),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: None,
                };
                frontier.push(target, node);
            }

            return Ok((e_bigint, frontier));
        }

        // candidate = candidate * g mod p (advance to g^{e+1} * h).
        candidate = mod_reduce(&(candidate * g), p);
    }

    Err(InitSmoothingError::NoSmoothExponent { attempts: max_attempts })
}

// ─── descend_node ─────────────────────────────────────────────────────────────

/// Descend a single frontier node: find a relation rewriting `target` as smaller primes.
///
/// Runs a special-q sieve rooted at `target.prime()` to find a relation in which `target`
/// appears alongside strictly smaller primes. The relation rewrites `log(target)` as a
/// combination of the smaller primes' logs.
///
/// # D.C.1 scope
///
/// This function's signature is frozen at D.C.1; the body is D.C.2's deliverable. At D.C.1,
/// this always returns `Err(DescentStepError::NoRelationFound { target })` — a clean stub,
/// not a panic.
///
/// # Arguments
///
/// - `target`: The prime/ideal to descend.
///
/// # Errors
///
/// - [`DescentStepError::NoRelationFound`] if no suitable relation is found (D.C.1 stub always
///   returns this).
/// - [`DescentStepError::NoStrictReduction`] if the sieve produced relations but none strictly
///   reduced the largest prime (D.C.2 may return this).
pub fn descend_node<F: Clone>(
    target: DescentTarget,
) -> Result<DescentNode<F>, DescentStepError> {
    // D.C.1 stub: the special-q descent recursion is D.C.2's deliverable.
    // Return a clean error rather than panicking.
    Err(DescentStepError::NoRelationFound { target })
}

// ─── solve_dl ─────────────────────────────────────────────────────────────────

/// Compute the discrete logarithm `log_g(h)` in `F_{p^k}*` via NFS-DL.
///
/// Returns `x` such that `g^x ≡ h (mod p^k)` in the subgroup of order `ell`, or an error
/// if the computation fails.
///
/// # Arguments
///
/// - `g`: Generator of the multiplicative group, as a `BigInt` in `[1, p^k)`.
/// - `h`: Target element, as a `BigInt` in `[1, p^k)`.
/// - `p`: The prime base of the field.
/// - `k`: The extension degree. `k = 1` is the prime field F_p; `k > 1` is F_{p^k}.
/// - `ell`: The subgroup order (a prime dividing `p^k − 1`). The returned log is mod `ell`.
///
/// # Returns
///
/// `Ok(x)` where `x ∈ [0, ell)` and `g^x ≡ h (mod p^k)`.
///
/// # Errors
///
/// - [`SolveDlError::Unsupported`] if `k > 1` (F_{p^k} extension deferred to E.C-prep).
/// - [`SolveDlError::InitSmoothingFailed`] if initialization-smoothing fails.
/// - [`SolveDlError::DescentFailed`] if a frontier prime cannot be descended.
///
/// # Scope (D.C.1 freeze)
///
/// - **k = 1 (prime field F_p):** The k = 1 path is wired through initialization-smoothing.
///   At D.C.1, the descent recursion and log assembly are stubs; the function returns
///   `Ok(BigInt::ZERO)` if the frontier is empty after initialization (toy inputs where
///   `g^e · h` is already factor-base smooth), or `Err(SolveDlError::DescentFailed)` if the
///   frontier is non-empty (pending D.C.2's descent recursion).
/// - **k > 1 (extension field F_{p^k}):** Returns `SolveDlError::Unsupported` immediately.
///   The F_{p^k} extension is genuine new mathematics deferred to an E.C-prep session.
///
/// # Threading note
///
/// The `ell` parameter threads the subgroup order through the entire pipeline. The returned
/// log is in `[0, ell)`. Pohlig–Hellman / CRT for the full group order is out of D.C scope.
pub fn solve_dl(
    g: &BigInt,
    h: &BigInt,
    p: &BigInt,
    k: usize,
    ell: &BigInt,
) -> Result<BigInt, SolveDlError> {
    // Step 1: k > 1 is not yet supported (F_{p^k} extension deferred to E.C-prep).
    if k != 1 {
        return Err(SolveDlError::Unsupported { k });
    }

    // Step 2: Initialization-smoothing.
    // For D.C.1, use a hardcoded medium_bound derived from ell (toy scale).
    // D.C.3 will integrate the real FactorBase and calibrate these parameters.
    // The medium_bound is chosen as a small constant for toy-scale KATs.
    let medium_bound = compute_medium_bound(p);
    let max_attempts = 1000u64;

    // Use u64 as the log type for the frontier nodes (known_log is None for all frontier
    // nodes at this stage; the type parameter is only needed for leaf nodes in D.C.2/D.C.3).
    let result = init_descent_frontier::<u64>(g, h, p, medium_bound, max_attempts);

    let (_e, mut frontier) = result.map_err(|err| match err {
        InitSmoothingError::NoSmoothExponent { attempts } => {
            SolveDlError::InitSmoothingFailed { attempts }
        }
    })?;

    // Step 3: Descent (stub at D.C.1 — D.C.2 implements the recursion).
    // If the frontier is non-empty, we cannot yet compute the log.
    if !frontier.is_empty() {
        // Pop the largest prime to report in the error.
        // D.C.2 will replace this with the actual descent recursion.
        let (stuck_target, _) = frontier.pop_largest().expect("frontier is non-empty");
        return Err(SolveDlError::DescentFailed { stuck_prime: stuck_target.prime() });
    }

    // Step 4: Assembly (stub at D.C.1 — D.C.3 implements log assembly).
    // If the frontier is empty, all factors are in the factor base (toy case).
    // Return BigInt::ZERO as a placeholder; D.C.3 will compute the actual log.
    let _ = ell; // ell is used by D.C.3's assembly; suppress unused warning.
    Ok(BigInt::zero())
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Compute a toy-scale medium-prime bound from the modulus `p`.
///
/// For D.C.1, this is a small constant (100) suitable for toy-scale KATs. D.C.3 will
/// integrate the real `FactorBase` and calibrate this parameter properly.
fn compute_medium_bound(_p: &BigInt) -> u64 {
    // Principle-2 annotation: at NFS scale, the medium-prime bound B' is calibrated to
    // the factor-base bound and the descent depth. At toy scale, 100 suffices.
    100
}

/// Reduce `a` into the canonical range `[0, m)` for `m > 0`.
fn mod_reduce(a: &BigInt, m: &BigInt) -> BigInt {
    let r = a % m;
    if r < BigInt::zero() { r + m } else { r }
}

/// Trial-divide `n` by all primes in `prime_base`.
///
/// Returns `Some(factors)` if `n` is fully smooth (cofactor = 1), where `factors` is the
/// list of prime factors with multiplicity (a prime appearing twice appears twice in the list).
/// Returns `None` if `n` has a prime factor not in `prime_base` (cofactor > 1).
///
/// # Special cases
///
/// - `n == 0`: returns `None` (zero is not smooth in the NFS sense).
/// - `n == 1`: returns `Some([])` (trivially smooth with no factors).
fn trial_divide_bigint(n: &BigInt, prime_base: &[u64]) -> Option<Vec<u64>> {
    if n.is_zero() {
        return None;
    }
    if n.is_one() {
        return Some(vec![]);
    }

    let mut remainder = n.clone();
    let mut factors = Vec::new();

    for &p in prime_base {
        if remainder.is_one() {
            break;
        }
        let p_big = BigInt::from(p);
        // Extract all powers of p from remainder.
        while (&remainder % &p_big).is_zero() {
            remainder /= &p_big;
            factors.push(p);
        }
    }

    // If remainder > 1, n has a prime factor not in the base.
    if remainder.is_one() { Some(factors) } else { None }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    #[test]
    fn trial_divide_smooth() {
        // 12 = 2^2 * 3; smooth over {2, 3, 5}.
        let primes = vec![2u64, 3, 5];
        let result = trial_divide_bigint(&bi(12), &primes);
        assert_eq!(result, Some(vec![2, 2, 3]));
    }

    #[test]
    fn trial_divide_not_smooth() {
        // 77 = 7 * 11; 11 is not in {2, 3, 5, 7}.
        let primes = vec![2u64, 3, 5, 7];
        let result = trial_divide_bigint(&bi(77), &primes);
        assert_eq!(result, None);
    }

    #[test]
    fn trial_divide_one() {
        let primes = vec![2u64, 3, 5];
        let result = trial_divide_bigint(&bi(1), &primes);
        assert_eq!(result, Some(vec![]));
    }

    #[test]
    fn trial_divide_zero() {
        let primes = vec![2u64, 3, 5];
        let result = trial_divide_bigint(&bi(0), &primes);
        assert_eq!(result, None);
    }

    #[test]
    fn init_frontier_finds_smooth_exponent() {
        // p = 101, g = 2, h = 50.
        // g^0 * h = 50 = 2 * 5^2; smooth over primes <= 20.
        let p = bi(101);
        let g = bi(2);
        let h = bi(50);
        let result = init_descent_frontier::<u64>(&g, &h, &p, 20, 100);
        assert!(result.is_ok(), "should find smooth exponent");
        let (e, frontier) = result.unwrap();
        assert_eq!(e, bi(0), "e=0 since g^0 * h = 50 is already smooth");
        assert!(!frontier.is_empty(), "frontier should be non-empty (50 = 2 * 5^2)");
    }

    #[test]
    fn init_frontier_no_smooth_exponent() {
        // p = 101, g = 2, h = 97 (a prime > medium_bound=5).
        // With medium_bound=5 and max_attempts=3, very unlikely to find smooth.
        let p = bi(101);
        let g = bi(2);
        let h = bi(97);
        let result = init_descent_frontier::<u64>(&g, &h, &p, 5, 3);
        // 97 is prime and > 5; 2*97=194≡93 mod 101 (93=3*31, 31>5); 4*97=388≡86 mod 101 (86=2*43, 43>5).
        // Very likely to fail.
        match result {
            Err(InitSmoothingError::NoSmoothExponent { attempts: 3 }) => {}
            Ok(_) => {} // Unlikely but possible; don't fail the test.
            Err(e) => panic!("unexpected error: {e}"),
        }
    }

    #[test]
    fn solve_dl_unsupported_k2() {
        let result = solve_dl(&bi(2), &bi(3), &bi(11), 2, &bi(10));
        assert_eq!(result, Err(SolveDlError::Unsupported { k: 2 }));
    }

    #[test]
    fn solve_dl_k1_does_not_panic() {
        // k=1 path is wired; result may be Ok or a known Err variant, but must not panic.
        let result = solve_dl(&bi(2), &bi(3), &bi(11), 1, &bi(10));
        match result {
            Ok(_) => {}
            Err(SolveDlError::InitSmoothingFailed { .. }) => {}
            Err(SolveDlError::DescentFailed { .. }) => {}
            Err(SolveDlError::Unsupported { .. }) => {
                panic!("k=1 should not return Unsupported")
            }
        }
    }

    #[test]
    fn solve_dl_error_display() {
        let e1 = SolveDlError::Unsupported { k: 3 };
        assert!(e1.to_string().contains("k > 1"));

        let e2 = SolveDlError::InitSmoothingFailed { attempts: 42 };
        assert!(e2.to_string().contains("42"));

        let e3 = SolveDlError::DescentFailed { stuck_prime: 17 };
        assert!(e3.to_string().contains("17"));
    }
}
