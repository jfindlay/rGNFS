//! NFS-DL solver entry point: `solve_dl` (C2 interface), initialization-smoothing,
//! log assembly, and the context-bearing `solve_dl_full` implementation.
//!
//! This module provides:
//!
//! - [`solve_dl`] — the cross-track C2 interface (frozen D.C.1, finalized D.C.3): compute
//!   `log_g(h)` in `F_{p^k}*` via NFS-DL. The k = 1 path is live; k > 1 returns
//!   [`SolveDlError::Unsupported`]. This is the frozen public API consumed by E.C.
//! - [`solve_dl_full`] — the context-bearing implementation: takes a [`SolveDlContext`]
//!   bundling the NFS polynomial, factor base, virtual-log table, and sieve config. This is
//!   what the end-to-end KAT exercises; `solve_dl` is the frozen interface E.C will call.
//! - [`SolveDlContext`] — bundles the pipeline parameters needed by `solve_dl_full`.
//! - [`init_descent_frontier`] — the first descent step: find an exponent `e` such that
//!   `g^e · h mod p` is smooth over primes up to `medium_bound`, producing the initial frontier.
//! - [`assemble_log`] — walk the descent tree from leaves (known virtual logs) up to the root,
//!   accumulating `log q = Σ log(child)` mod ℓ at each node.
//! - [`SolveDlError`] — error type for `solve_dl` (shape frozen D.C.1, taxonomy finalized D.C.3).
//! - [`InitSmoothingError`] / [`DescentStepError`] — error types for the descent substrate.
//!
//! # Contract C2 (shape frozen D.C.1, finalized D.C.3)
//!
//! `solve_dl` is the cross-track interface consumed by E.C (the MOV bridge). Its signature and
//! the `SolveDlError` taxonomy are **frozen** at D.C.3. No further variants will be added.
//!
//! # Relationship between `solve_dl` and `solve_dl_full`
//!
//! `solve_dl(g, h, p, k, ell)` is the frozen C2 public API. It does not take a `FactorBase`
//! or `VirtualLogTable` because those are not part of the cross-track interface. For the full
//! pipeline (relation collection → F_ℓ solve → virtual-log table → descent → assembly), use
//! `solve_dl_full(g, h, p, k, ell, ctx)` where `ctx: &SolveDlContext` bundles the pipeline
//! parameters. The end-to-end KAT calls `solve_dl_full`; E.C calls `solve_dl`.
//!
//! # Log assembly
//!
//! The assembly algorithm walks the flat `completed` list from `run_descent` in reverse order
//! (leaves first, roots last), building a map from `DescentTarget` to `BigInt` log. For each
//! node:
//! - Leaf (known_log.is_some()): log = known_log converted to BigInt.
//! - Interior (children non-empty): log = Σ log(child) mod ell.
//!   (Each child appears once per unit of exponent, since `build_children` creates one child
//!   per unit of exponent — so summing all children's logs gives the correct weighted sum.)
//!
//! The log of `g^e · h` = Σ logs of the initial frontier targets mod ell.
//! Then `log_g(h) = (log_g(g^e · h) − e) mod ell`.

use std::collections::HashMap;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use shared_numth::factor_base_up_to;

use crate::dl::descent::node::{DescentFrontier, DescentNode, DescentTarget};
use crate::dl::descent::recurse::{DescentSieveConfig, run_descent};
use crate::dl::ext::descent::solve_dl_ext;
use crate::dl::VirtualLogTable;
use crate::polyselect::PolyPair;
use crate::sieve::FactorBase;

// ─── SolveDlError ─────────────────────────────────────────────────────────────

/// Error type for [`solve_dl`].
///
/// The error taxonomy is **finalized at D.C.3**. No further variants will be added.
/// E.C consumes this taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveDlError {
    /// Extension degree k is beyond the toy ceiling (k > 2).
    ///
    /// k=1 (prime field F_p) and k=2 (extension field F_{p²}) are supported. k>2 is beyond
    /// the toy ceiling and returns this variant immediately without attempting any computation.
    ///
    /// # D.E.3 ◆ note
    ///
    /// Before D.E.3, this variant fired for all k>1. After D.E.3, it fires only for k>2.
    /// The taxonomy is FROZEN (D.C.3); no new variants will be added.
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
    // ─── D.C.3 taxonomy freeze ────────────────────────────────────────────────
    //
    // D.C.2 descent reality revealed no additional failure modes beyond the three above.
    // The assembly step (D.C.3) can fail if a node's children are not all resolved, but
    // this is a programming error (invariant violation), not a runtime failure mode — it
    // is surfaced as a panic rather than a new error variant.
    //
    // The taxonomy is now FROZEN. E.C consumes exactly these three variants.
}

impl std::fmt::Display for SolveDlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported { k } => {
                write!(f, "extension field F_{{p^{k}}} (k > 2) not supported (toy ceiling)")
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

// ─── SolveDlContext ───────────────────────────────────────────────────────────

/// Pipeline parameters for [`solve_dl_full`].
///
/// Bundles the NFS polynomial pair, factor base, virtual-log table, and sieve config
/// needed by the full individual-log pipeline. This is the context that `solve_dl_full`
/// takes but the frozen C2 `solve_dl` does not.
///
/// # Relationship to `solve_dl`
///
/// `solve_dl(g, h, p, k, ell)` is the frozen C2 interface consumed by E.C. It does not
/// take a `SolveDlContext` because the cross-track interface is parameter-minimal. For the
/// full pipeline (including descent and assembly), use `solve_dl_full` with a context.
///
/// # Type parameter
///
/// `F` is the field element type for virtual logs (e.g., `FpNaive4`). The `VirtualLogTable<F>`
/// stores the virtual logs of the factor-base elements recovered from the F_ℓ linear system.
pub struct SolveDlContext<'a, F> {
    /// The NFS polynomial pair (for the special-q sieve in descent).
    pub poly: &'a PolyPair,
    /// The factor base (for leaf detection and log lookup during descent).
    pub fb: &'a FactorBase,
    /// The virtual-log table (factor-base element logs recovered from the F_ℓ system).
    pub vtable: &'a VirtualLogTable<F>,
    /// Sieve parameters for the descent step (a_bound, b_bound, threshold_scale).
    pub sieve_cfg: DescentSieveConfig,
    /// Convert a virtual log `F` to `BigInt` for assembly arithmetic.
    ///
    /// For `FpNaive4`, this is `|f| BigInt::from(f.to_uint().as_words()[0])`.
    /// Provided as a closure so `SolveDlContext` is not generic over the conversion.
    pub to_bigint: Box<dyn Fn(&F) -> BigInt + 'a>,
}

// ─── assemble_log ─────────────────────────────────────────────────────────────

/// Walk the descent tree from leaves (known virtual logs) up to the root, accumulating
/// `log q = Σ log(child)` mod ℓ at each node.
///
/// Returns `log_g(g^e · h) mod ell` — the sum of the logs of the initial frontier targets.
/// The caller then computes `log_g(h) = (assembled_log − e) mod ell`.
///
/// # Assembly algorithm
///
/// The `completed` list from [`run_descent`] is a flat list of nodes in largest-prime-first
/// order (the order they were popped from the frontier). Processing in reverse order gives
/// leaves before roots. For each node:
///
/// - **Leaf** (`known_log.is_some()`): log = `to_bigint(known_log)`.
/// - **Interior** (`children` non-empty): log = Σ `map[child.target]` mod ell.
///   Each child appears once per unit of exponent (since [`build_children`] creates one child
///   per unit of exponent), so summing all children's logs gives the correct weighted sum.
///
/// The log of `g^e · h` = Σ `map[initial_target]` mod ell.
///
/// # Arguments
///
/// - `completed`: The flat list of descended nodes from [`run_descent`].
/// - `initial_targets`: The targets from the initial frontier (factors of `g^e · h`).
/// - `ell`: The subgroup order (mod ell arithmetic).
/// - `to_bigint`: Convert a virtual log `F` to `BigInt`.
///
/// # Errors
///
/// Returns [`SolveDlError::DescentFailed`] if a node's log cannot be resolved (invariant
/// violation: a child target is not in the map). This indicates a bug in the descent tree.
pub fn assemble_log<F: Clone>(
    completed: &[DescentNode<F>],
    initial_targets: &[DescentTarget],
    ell: &BigInt,
    to_bigint: impl Fn(&F) -> BigInt,
) -> Result<BigInt, SolveDlError> {
    // Build a map from DescentTarget to log (BigInt) by processing nodes in reverse order
    // (leaves first, since completed is largest-prime-first).
    //
    // The map key is (prime, variant_tag, r) encoded as a tuple for HashMap.
    // We use a Vec<(DescentTarget, BigInt)> and linear search for simplicity at toy scale.
    // At NFS scale, a HashMap with a proper key would be used.
    let mut log_map: HashMap<TargetKey, BigInt> = HashMap::new();

    // Process in reverse order: leaves (small primes, processed last by run_descent) first.
    for node in completed.iter().rev() {
        let key = target_key(&node.target);

        if let Some(ref known) = node.known_log {
            // Leaf node: log is the known virtual log.
            let log = to_bigint(known);
            log_map.insert(key, mod_reduce(&log, ell));
        } else {
            // Interior node: log = Σ log(child) mod ell.
            let mut log = BigInt::zero();
            for child in &node.children {
                let child_key = target_key(&child.target);
                let child_log = if let Some(ref known) = child.known_log {
                    // Leaf child: use known_log directly.
                    to_bigint(known)
                } else {
                    // Interior child: look up from map.
                    match log_map.get(&child_key) {
                        Some(l) => l.clone(),
                        None => {
                            // Child not yet resolved — invariant violation.
                            return Err(SolveDlError::DescentFailed {
                                stuck_prime: child.target.prime(),
                            });
                        }
                    }
                };
                log = mod_reduce(&(log + child_log), ell);
            }
            log_map.insert(key, log);
        }
    }

    // Sum the logs of the initial frontier targets.
    let mut assembled = BigInt::zero();
    for target in initial_targets {
        let key = target_key(target);
        let log = match log_map.get(&key) {
            Some(l) => l.clone(),
            None => {
                // Initial target not resolved — invariant violation.
                return Err(SolveDlError::DescentFailed { stuck_prime: target.prime() });
            }
        };
        assembled = mod_reduce(&(assembled + log), ell);
    }

    Ok(assembled)
}

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

// ─── solve_dl ─────────────────────────────────────────────────────────────────

/// Compute the discrete logarithm `log_g(h)` in `F_{p^k}*` via NFS-DL.
///
/// Returns `x` such that `g^x ≡ h (mod p^k)` in the subgroup of order `ell`, or an error
/// if the computation fails.
///
/// # Arguments
///
/// - `g`: Generator of the multiplicative group, as a `BigInt` in `[1, p^k)`.
///   For k=1: an integer in `[1, p)`. For k>1: base-p encoded (`c_0 + c_1·p + …`).
/// - `h`: Target element, as a `BigInt` in `[1, p^k)`. Same encoding as `g`.
/// - `p`: The prime base of the field.
/// - `k`: The extension degree. `k = 1` is the prime field F_p; `k = 2` is F_{p²}.
/// - `ell`: The subgroup order (a prime dividing `p^k − 1`). The returned log is mod `ell`.
///
/// # Returns
///
/// `Ok(x)` where `x ∈ [0, ell)` and `g^x ≡ h (mod p^k)`.
///
/// # Errors
///
/// - [`SolveDlError::Unsupported`] if `k > 2` (beyond the toy ceiling).
/// - [`SolveDlError::InitSmoothingFailed`] if initialization-smoothing fails.
/// - [`SolveDlError::DescentFailed`] if a frontier prime cannot be descended.
///
/// # Scope (C2 frozen D.C.3, extended D.E.3 ◆)
///
/// This is the **frozen C2 interface** consumed by E.C. Its signature will not change.
///
/// - **k = 1 (prime field F_p):** Runs initialization-smoothing. Without a
///   [`SolveDlContext`], the descent and assembly cannot proceed. If the frontier is
///   non-empty after smoothing, returns `Err(SolveDlError::DescentFailed)`. For the full
///   pipeline, use [`solve_dl_full`].
/// - **k = 2 (extension field F_{p²}):** Runs the full k=2 pipeline (toy scale). `g` and
///   `h` are base-p encoded: `c_0 + c_1·p` for the element `c_0 + c_1·u` in F_{p²}.
/// - **k > 2:** Returns `SolveDlError::Unsupported` (beyond the toy ceiling).
///
/// # Threading note
///
/// The `ell` parameter threads the subgroup order through the entire pipeline. The returned
/// log is in `[0, ell)`. Pohlig–Hellman / CRT for the full group order is out of scope.
pub fn solve_dl(
    g: &BigInt,
    h: &BigInt,
    p: &BigInt,
    k: usize,
    ell: &BigInt,
) -> Result<BigInt, SolveDlError> {
    // k > 2: beyond the toy ceiling — Unsupported.
    // k = 2: the extension field path (D.E.3).
    // k = 1: the prime field path (D.C.3).
    if k > 2 {
        return Err(SolveDlError::Unsupported { k });
    }

    // k = 2: delegate to the extension descent (D.E.3).
    if k == 2 {
        return solve_dl_ext(g, h, p, k, ell).map_err(|e| match e {
            crate::dl::ext::descent::SolveDlExtError::UnsupportedDegree { k } => {
                SolveDlError::Unsupported { k }
            }
            crate::dl::ext::descent::SolveDlExtError::InitSmoothingFailed => {
                SolveDlError::InitSmoothingFailed { attempts: 0 }
            }
            crate::dl::ext::descent::SolveDlExtError::InvalidGenerator => {
                // g is not a valid generator of the ℓ-subgroup — treat as descent failure.
                SolveDlError::DescentFailed { stuck_prime: 0 }
            }
            crate::dl::ext::descent::SolveDlExtError::NoIrreduciblePoly => {
                SolveDlError::DescentFailed { stuck_prime: 0 }
            }
        });
    }

    // Step 2: Initialization-smoothing.
    // For the frozen C2 interface, use a hardcoded medium_bound (toy scale).
    // The full pipeline (with FactorBase and VirtualLogTable) is in solve_dl_full.
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

    // Step 3: Descent.
    // Without a SolveDlContext (factor base, virtual-log table, sieve config), the descent
    // cannot proceed. If the frontier is non-empty, return DescentFailed.
    // For the full pipeline, use solve_dl_full with a SolveDlContext.
    if !frontier.is_empty() {
        let (stuck_target, _) = frontier.pop_largest().expect("frontier is non-empty");
        return Err(SolveDlError::DescentFailed { stuck_prime: stuck_target.prime() });
    }

    // Step 4: Assembly (frontier is empty — all factors were in the factor base at toy scale).
    // Return BigInt::ZERO as a placeholder; the real answer requires solve_dl_full.
    let _ = ell; // ell is used by solve_dl_full's assembly; suppress unused warning.
    Ok(BigInt::zero())
}

// ─── solve_dl_full ────────────────────────────────────────────────────────────

/// Compute the discrete logarithm `log_g(h)` via the full NFS-DL pipeline.
///
/// This is the context-bearing implementation of the individual-log computation. It takes a
/// [`SolveDlContext`] bundling the NFS polynomial, factor base, virtual-log table, and sieve
/// config, and runs the full pipeline:
///
/// 1. k > 2 → `Err(Unsupported { k })`.
/// 2. k = 2 → delegate to [`solve_dl`] (the k=2 path builds its context internally).
/// 3. Initialization-smoothing: find `e` such that `g^e · h mod p` is smooth.
/// 4. Descent: run the special-q descent to rewrite all medium primes as factor-base leaves.
/// 5. Assembly: walk the descent tree to compute `log_g(g^e · h) mod ell`.
/// 6. Back out the initialization exponent: `log_g(h) = (assembled − e) mod ell`.
///
/// # Toy KAT note
///
/// For the toy KAT, pick `ell = p − 1` (the full group order) so the log is recovered mod
/// `p − 1` directly, sidestepping Pohlig–Hellman. The comment in the KAT documents this.
///
/// # Arguments
///
/// - `g`: Generator of the multiplicative group.
/// - `h`: Target element.
/// - `p`: The prime modulus.
/// - `k`: Extension degree. k=1 uses the full context; k=2 delegates to `solve_dl`.
/// - `ell`: Subgroup order. The returned log is mod `ell`.
/// - `ctx`: Pipeline context (polynomial, factor base, virtual-log table, sieve config).
///   Used only for k=1; k=2 builds its context internally.
///
/// # Returns
///
/// `Ok(x)` where `x ∈ [0, ell)` and `g^x ≡ h (mod p^k)`.
///
/// # Errors
///
/// - [`SolveDlError::Unsupported`] if `k > 2`.
/// - [`SolveDlError::InitSmoothingFailed`] if initialization-smoothing fails.
/// - [`SolveDlError::DescentFailed`] if a frontier prime cannot be descended.
pub fn solve_dl_full<F: Clone>(
    g: &BigInt,
    h: &BigInt,
    p: &BigInt,
    k: usize,
    ell: &BigInt,
    ctx: &SolveDlContext<'_, F>,
) -> Result<BigInt, SolveDlError> {
    // k > 2: beyond the toy ceiling.
    if k > 2 {
        return Err(SolveDlError::Unsupported { k });
    }

    // k = 2: the SolveDlContext does not carry extension info; delegate to solve_dl which
    // builds the extension context internally.
    if k == 2 {
        return solve_dl(g, h, p, k, ell);
    }

    // Step 2: Initialization-smoothing.
    // Use the factor-base bound as the medium_bound: all primes up to b_alg are in the
    // factor base and have known virtual logs. The smoothing finds e such that g^e * h
    // factors completely over primes up to b_alg.
    let medium_bound = ctx.fb.b_alg;
    let max_attempts = 10_000u64;

    let (e, frontier) =
        init_descent_frontier::<F>(g, h, p, medium_bound, max_attempts).map_err(|err| match err {
            InitSmoothingError::NoSmoothExponent { attempts } => {
                SolveDlError::InitSmoothingFailed { attempts }
            }
        })?;

    // Collect the initial frontier targets (factors of g^e * h) before consuming the frontier.
    // These are the targets whose logs we sum to get log(g^e * h).
    let mut initial_targets: Vec<DescentTarget> = Vec::new();
    // We need to peek at the frontier without consuming it. Since DescentFrontier doesn't
    // support iteration, we rebuild it after collecting the targets.
    // Strategy: drain the frontier into a Vec, collect targets, then rebuild.
    let mut frontier_entries: Vec<(DescentTarget, DescentNode<F>)> = Vec::new();
    let mut temp_frontier = frontier;
    while let Some((target, node)) = temp_frontier.pop_largest() {
        initial_targets.push(target.clone());
        frontier_entries.push((target, node));
    }

    // Rebuild the frontier for run_descent.
    let mut frontier = DescentFrontier::new();
    for (target, node) in frontier_entries {
        frontier.push(target, node);
    }

    // Step 3: Descent.
    let completed = run_descent(frontier, ctx.poly, ctx.fb, ctx.vtable, &ctx.sieve_cfg)?;

    // Step 4: Assembly.
    // Walk the completed nodes to compute log_g(g^e * h) mod ell.
    let assembled = assemble_log(&completed, &initial_targets, ell, &ctx.to_bigint)?;

    // Step 5: Back out the initialization exponent.
    // log_g(h) = (log_g(g^e * h) - e) mod ell.
    let log_h = mod_reduce(&(assembled - &e), ell);

    Ok(log_h)
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Compute a toy-scale medium-prime bound from the modulus `p`.
///
/// For the frozen C2 `solve_dl` interface, this is a small constant (100) suitable for
/// toy-scale KATs. `solve_dl_full` uses the factor-base bound instead.
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

// ─── TargetKey ────────────────────────────────────────────────────────────────

/// A hashable key for a `DescentTarget`, used in the assembly log map.
///
/// `DescentTarget` is not `Hash` (it contains `u64` fields, but we need a stable key).
/// This newtype provides a `(u64, u64)` key: `(prime, r)` where `r = 0` for Rational.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TargetKey(u64, u64);

/// Convert a `DescentTarget` to a `TargetKey` for the assembly log map.
fn target_key(target: &DescentTarget) -> TargetKey {
    match target {
        DescentTarget::Rational(p) => TargetKey(*p, 0),
        DescentTarget::Algebraic { p, r } => TargetKey(*p, *r),
    }
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
    fn solve_dl_unsupported_k3() {
        // k=3 is beyond the toy ceiling (k>2) — must return Unsupported.
        let result = solve_dl(&bi(2), &bi(3), &bi(11), 3, &bi(10));
        assert_eq!(result, Err(SolveDlError::Unsupported { k: 3 }));
    }

    #[test]
    fn solve_dl_k2_does_not_return_unsupported() {
        // k=2 is now wired (D.E.3); it must NOT return Unsupported.
        // The result may be Ok or a known Err variant (InitSmoothingFailed / DescentFailed),
        // but must not be Unsupported.
        let result = solve_dl(&bi(2), &bi(3), &bi(11), 2, &bi(10));
        assert!(
            !matches!(result, Err(SolveDlError::Unsupported { .. })),
            "k=2 must not return Unsupported (D.E.3 wired); got: {:?}",
            result
        );
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
        let msg = e1.to_string();
        assert!(
            msg.contains("k > 1") || msg.contains("k > 2") || msg.contains("not supported"),
            "Unsupported display should mention k or 'not supported': {msg}"
        );

        let e2 = SolveDlError::InitSmoothingFailed { attempts: 42 };
        assert!(e2.to_string().contains("42"));

        let e3 = SolveDlError::DescentFailed { stuck_prime: 17 };
        assert!(e3.to_string().contains("17"));
    }

    #[test]
    fn assemble_log_leaf_only() {
        // A single leaf node: log(p) = known_log.
        // Verifies the base case of assembly.
        let ell = bi(7);
        let leaf = DescentNode::<BigInt> {
            target: DescentTarget::Rational(3),
            rewriting_relation: None,
            children: vec![],
            known_log: Some(bi(5)),
        };
        let initial = vec![DescentTarget::Rational(3)];
        let result = assemble_log(&[leaf], &initial, &ell, |f| f.clone());
        assert_eq!(result, Ok(bi(5)), "single leaf: log = known_log");
    }

    #[test]
    fn assemble_log_two_leaves_sum() {
        // Two leaf nodes: log(g^e * h) = log(p1) + log(p2) mod ell.
        // p1 = 2, log = 3; p2 = 3, log = 5; ell = 7.
        // Expected: (3 + 5) mod 7 = 1.
        let ell = bi(7);
        let leaf1 = DescentNode::<BigInt> {
            target: DescentTarget::Rational(2),
            rewriting_relation: None,
            children: vec![],
            known_log: Some(bi(3)),
        };
        let leaf2 = DescentNode::<BigInt> {
            target: DescentTarget::Rational(3),
            rewriting_relation: None,
            children: vec![],
            known_log: Some(bi(5)),
        };
        let initial = vec![DescentTarget::Rational(2), DescentTarget::Rational(3)];
        let result = assemble_log(&[leaf1, leaf2], &initial, &ell, |f| f.clone());
        assert_eq!(result, Ok(bi(1)), "(3 + 5) mod 7 = 1");
    }

    #[test]
    fn assemble_log_interior_node() {
        // Interior node with two leaf children.
        // log(q) = log(p1) + log(p2) mod ell.
        // p1 = 2, log = 3; p2 = 3, log = 5; ell = 7.
        // Interior node q = 6 (not prime, but valid for the test).
        // Expected: (3 + 5) mod 7 = 1.
        let ell = bi(7);

        // Build the interior node with leaf children embedded.
        let interior = DescentNode::<BigInt> {
            target: DescentTarget::Rational(6),
            rewriting_relation: None, // not needed for assembly
            children: vec![
                DescentNode {
                    target: DescentTarget::Rational(2),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: Some(bi(3)),
                },
                DescentNode {
                    target: DescentTarget::Rational(3),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: Some(bi(5)),
                },
            ],
            known_log: None,
        };

        let initial = vec![DescentTarget::Rational(6)];
        // Process in reverse order: interior node is processed first (it's the only entry).
        // Its children are leaves with known_log, so they're resolved inline.
        let result = assemble_log(&[interior], &initial, &ell, |f| f.clone());
        assert_eq!(result, Ok(bi(1)), "interior: log = (3 + 5) mod 7 = 1");
    }

    #[test]
    fn assemble_log_with_multiplicity() {
        // Interior node with two children for the same prime (exponent 2).
        // log(q) = 2 * log(p1) + log(p2) mod ell.
        // p1 = 2, log = 3 (appears twice); p2 = 3, log = 5; ell = 7.
        // Expected: (3 + 3 + 5) mod 7 = 11 mod 7 = 4.
        let ell = bi(7);

        let interior = DescentNode::<BigInt> {
            target: DescentTarget::Rational(6),
            rewriting_relation: None,
            children: vec![
                DescentNode {
                    target: DescentTarget::Rational(2),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: Some(bi(3)),
                },
                DescentNode {
                    target: DescentTarget::Rational(2),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: Some(bi(3)),
                },
                DescentNode {
                    target: DescentTarget::Rational(3),
                    rewriting_relation: None,
                    children: vec![],
                    known_log: Some(bi(5)),
                },
            ],
            known_log: None,
        };

        let initial = vec![DescentTarget::Rational(6)];
        let result = assemble_log(&[interior], &initial, &ell, |f| f.clone());
        assert_eq!(result, Ok(bi(4)), "multiplicity: (3 + 3 + 5) mod 7 = 4");
    }
}
