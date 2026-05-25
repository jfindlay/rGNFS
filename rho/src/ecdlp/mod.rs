//! ECDLP solver via Pollard rho.
//!
//! Optimization layers (per the plan):
//! 4. r-adding walk (Teske, r≈20) + Brent's cycle detection — this phase.
//! 5. Distinguished points + parallel collision search (van Oorschot–Wiener).
//! 6. Negation map + fruitless-cycle escape (BKNS).
//! 7. Batched field inversion + affine coordinates.
//! 8. GLV endomorphism (order-3, secp-toy curve only).
//!
//! # Solver (Phase 4)
//!
//! [`solve_brent`] is the single-threaded entry point.  It runs Brent's cycle
//! detection on top of an r-adding walk, tracking `(a, b)` scalars throughout
//! so that when the tortoise and hare collide at the same point `W`, the DLP
//! `Q = k·G` can be recovered as `k = (a_t − a_h) / (b_h − b_t) mod n`.
//!
//! The solver retries with a fresh random walk table and starting point on
//! degenerate failures (`b_h = b_t mod n`), which occur with probability `~1/n`
//! per attempt.

pub mod coordinator;
pub mod dp;
pub mod glv;
pub mod negmap;
pub mod walk;

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::curve::{AffinePoint, Curve};
use crate::field::Fp;
use walk::{AddendTable, WalkState};

// ── Modular arithmetic helpers ────────────────────────────────────────────────

/// Modular multiplication of two 64-bit values mod `n`.
#[inline]
fn mulmod64(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

/// Modular subtraction: `(a - b) mod n`, result in `[0, n)`.
#[inline]
fn submod64(a: u64, b: u64, n: u64) -> u64 {
    if a >= b { a - b } else { a + n - b }
}

/// Modular inverse via Fermat's little theorem: `a^(n-2) mod n`.
///
/// Requires `n` to be prime and `a ≠ 0 mod n`.
///
/// # Panics
///
/// Panics if `a == 0`.
fn inv_mod_prime(a: u64, n: u64) -> u64 {
    assert!(a != 0, "inv_mod_prime: zero has no inverse");
    // Compute a^(n-2) mod n by iterative square-and-multiply.
    let mut result: u64 = 1;
    let mut base: u64 = a % n;
    let mut exp: u64 = n - 2;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mulmod64(result, base, n);
        }
        base = mulmod64(base, base, n);
        exp >>= 1;
    }
    result
}

// ── Collision extraction ─────────────────────────────────────────────────────

/// Attempt to extract `k` from a tortoise/hare collision.
///
/// When `W_tortoise = W_hare`, we have `a_t·G + b_t·Q = a_h·G + b_h·Q`, so
/// `(b_h − b_t)·Q = (a_t − a_h)·G`, giving `k = (a_t − a_h) / (b_h − b_t) mod n`.
///
/// Returns `None` if `b_h == b_t mod n` (degenerate — retry with new walk).
fn extract_k(a_t: u64, b_t: u64, a_h: u64, b_h: u64, n: u64) -> Option<u64> {
    let db = submod64(b_h, b_t, n); // b_h - b_t mod n
    if db == 0 {
        return None; // degenerate; caller should retry
    }
    let da = submod64(a_t, a_h, n); // a_t - a_h mod n
    let db_inv = inv_mod_prime(db, n);
    Some(mulmod64(da, db_inv, n))
}

// ── Brent's cycle detection ───────────────────────────────────────────────────

/// Solve `Q = k·G` via Pollard rho with Brent's cycle detection.
///
/// Builds an r-adding walk table from `g` and `q`, then runs tortoise and hare
/// pointers according to Brent's algorithm.  When a collision is detected,
/// the DLP is extracted from the tracked `(a, b)` scalars.
///
/// Brent's algorithm for a pseudorandom sequence `x₀, x₁, x₂, …`:
///
/// - The tortoise is "frozen" at the start of each power-of-2 window.
/// - The hare advances one step at a time; at each step it is compared to the
///   frozen tortoise.
/// - After `r` comparisons without collision, the tortoise is snapped forward to
///   the hare's current position, and the window doubles: `r ← 2r`.
///
/// This terminates when the hare catches the tortoise on the cycle, using
/// ~24% fewer function evaluations than Floyd's algorithm on average.
///
/// The solver retries up to `max_retries` times on degenerate failures (i.e., when
/// two distinct walk states share the same `b` coefficient at collision time).
///
/// # Arguments
///
/// * `curve` — the curve definition.
/// * `g` — base point G.
/// * `q` — target point Q (`Q = k·G` for the unknown `k`).
/// * `n` — prime group order (64-bit).
/// * `seed` — RNG seed for reproducibility.
/// * `max_retries` — maximum number of fresh attempts (typical: 10).
///
/// # Returns
///
/// `Some(k)` such that `Q = k·G`, or `None` if all retries were degenerate.
pub fn solve_brent<F: Fp>(
    curve: &Curve,
    g: &AffinePoint<F>,
    q: &AffinePoint<F>,
    n: u64,
    seed: u64,
    max_retries: usize,
) -> Option<u64> {
    for attempt in 0..max_retries {
        // Fresh RNG per attempt so each retry explores a different trajectory.
        let mut rng = ChaCha20Rng::seed_from_u64(seed.wrapping_add(attempt as u64));

        let table = AddendTable::new(curve, g, q, n, &mut rng);

        // Initialise both pointers from the same starting state (standard Brent).
        let start = WalkState::<F>::new_random(curve, g, q, n, &mut rng);
        let mut tortoise = start.clone();
        let mut hare     = start;

        let mut r: u64 = 1;   // current window size (power of 2)
        let mut count: u64 = 0; // steps taken in the current window

        let mut tortoise_affine = tortoise.to_affine(curve);

        loop {
            // Advance hare one step.
            hare.step(curve, &table, n);
            let hare_affine = hare.to_affine(curve);
            count += 1;

            // Check for collision.
            if hare_affine == tortoise_affine {
                let ta = tortoise.a;
                let tb = tortoise.b;
                let ha = hare.a;
                let hb = hare.b;
                if let Some(k) = extract_k(ta, tb, ha, hb, n) {
                    return Some(k);
                }
                break; // degenerate — retry outer attempt loop
            }

            // If hare has taken `r` steps from the last tortoise snapshot:
            // snap tortoise forward to hare's current position and double the window.
            if count == r {
                tortoise = hare.clone();
                tortoise_affine = hare_affine;
                count = 0;
                r <<= 1;

                // Safety cap: give up after 2^28 total evaluations on this attempt.
                if r > (1 << 28) {
                    break;
                }
            }
        }
    }

    None
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::curve::test_curves::{tiny_a, TINY_A_N};
    use crate::field::FpMonty;

    /// Solve a known DLP on the 20-bit test curve A and verify the result.
    ///
    /// The solver does not need to find the specific k that was used to construct Q —
    /// any k' satisfying k'·G = Q is correct.  The assertion checks Q, not k.
    fn check_dlog(k_target: u64) {
        let curve = tiny_a();
        let g: AffinePoint<FpMonty> = curve.generator();
        let q = curve.scalar_mul(&g, &Uint::<4>::from(k_target));

        let k = solve_brent(&curve, &g, &q, TINY_A_N, 0, 20)
            .unwrap_or_else(|| panic!("solve_brent failed for k={k_target}"));

        let check = curve.scalar_mul(&g, &Uint::<4>::from(k));
        assert_eq!(check, q, "k·G ≠ Q for computed k={k} (expected k={k_target})");
    }

    #[test]
    fn solve_tiny_a_dlp_7() {
        check_dlog(7);
    }

    #[test]
    fn solve_tiny_a_dlp_42() {
        check_dlog(42);
    }

    #[test]
    fn solve_tiny_a_dlp_100() {
        check_dlog(100);
    }
}
