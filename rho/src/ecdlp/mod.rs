//! ECDLP solver via Pollard rho.
//!
//! Optimization layers (per the plan):
//! 4. r-adding walk (Teske, r≈20) + Brent's cycle detection — Phase 4.
//! 5. Distinguished points + parallel collision search (van Oorschot–Wiener) — this phase.
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
//!
//! # Solver (Phase 5)
//!
//! [`solve_dp`] is the parallel entry point.  It spawns `num_walkers` threads,
//! each running an r-adding walk and emitting a [`dp::DpRecord`] whenever the
//! walk lands on a distinguished point (low `theta` bits of x-coordinate are
//! zero).  A coordinator on the calling thread collects DPs via a channel and
//! detects collisions that allow `k` to be recovered.

pub mod coordinator;
pub mod dp;
pub mod glv;
pub mod negmap;
pub mod walk;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::{bounded, Receiver, Sender};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::curve::{AffinePoint, Curve};
use crate::field::Fp;
use coordinator::Coordinator;
use dp::{DpRecord, is_distinguished};
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

// ── Distinguished-point parallel solver ──────────────────────────────────────

/// Solve `Q = k·G` via the van Oorschot–Wiener parallel rho algorithm.
///
/// Spawns `num_walkers` threads, each running an independent r-adding walk.
/// Walkers emit a [`DpRecord`] whenever the walk lands on a distinguished point
/// (low `theta` bits of the x-coordinate are zero).  A coordinator on the
/// calling thread collects DPs and detects collisions.
///
/// Walker restart policy: if a walker takes more than `2^(theta + 10)` steps
/// without hitting a DP, it restarts from a fresh random `(a₀, b₀)`.  This
/// prevents walkers from being trapped in long fruitless cycles.
///
/// # Arguments
///
/// * `curve` — the curve definition.
/// * `g` — base point G.
/// * `q` — target point Q (`Q = k·G` for the unknown `k`).
/// * `n` — prime group order (64-bit).
/// * `num_walkers` — number of parallel walker threads (≥ 1).
/// * `theta` — DP threshold: a point is distinguished when its x-coordinate
///   has at least `theta` low-order zero bits.  Expected steps between DPs: `2^theta`.
/// * `seed` — RNG seed for reproducibility (each walker gets a derived seed).
///
/// # Returns
///
/// `Some(k)` such that `Q = k·G`, or `None` if the coordinator gives up
/// (should not happen in practice for valid inputs).
pub fn solve_dp<F: Fp>(
    curve: &Curve,
    g: &AffinePoint<F>,
    q: &AffinePoint<F>,
    n: u64,
    num_walkers: usize,
    theta: u32,
    seed: u64,
) -> Option<u64> {
    assert!(num_walkers >= 1, "solve_dp: need at least one walker");

    // Build the addend table once; share it read-only across all walkers.
    // Arc is needed because the table outlives the spawning scope.
    let mut rng0 = ChaCha20Rng::seed_from_u64(seed);
    let table = Arc::new(AddendTable::<F>::new(curve, g, q, n, &mut rng0));

    // Stop flag: coordinator sets this to true when k is found.
    let stop = Arc::new(AtomicBool::new(false));

    // Bounded channel: walkers send DpRecords; coordinator receives them.
    // Capacity = 4 * num_walkers to avoid walkers blocking on a full channel
    // while the coordinator is processing a collision.
    let (tx, rx): (Sender<DpRecord>, Receiver<DpRecord>) = bounded(4 * num_walkers);

    // Spawn walker threads.
    let mut handles = Vec::with_capacity(num_walkers);
    for walk_id in 0..num_walkers {
        let curve_c  = curve.clone();
        let g_c      = g.clone();
        let q_c      = q.clone();
        let table_c  = Arc::clone(&table);
        let stop_c   = Arc::clone(&stop);
        let tx_c     = tx.clone();
        // Each walker gets a distinct seed so they explore different trajectories.
        let walker_seed = seed.wrapping_add(walk_id as u64 + 1);

        let handle = std::thread::spawn(move || {
            run_walker(
                &curve_c, &g_c, &q_c, n, &table_c, theta,
                walk_id, walker_seed, &stop_c, &tx_c,
            );
        });
        handles.push(handle);
    }
    // Drop the original sender so the channel closes when all walkers exit.
    drop(tx);

    // Coordinator loop: receive DPs and detect collisions.
    let mut coord = Coordinator::new(n);
    let mut result = None;

    for dp in &rx {
        if let Some(k) = coord.insert(dp) {
            result = Some(k);
            // Signal all walkers to stop.
            stop.store(true, Ordering::Relaxed);
            break;
        }
    }

    // Drain the channel so walkers are not blocked trying to send.
    // (They will exit once they see the stop flag, but may be mid-send.)
    for _ in &rx {}

    // Join all walker threads.
    for h in handles {
        let _ = h.join();
    }

    result
}

// ── Walker thread body ────────────────────────────────────────────────────────

/// Body of a single walker thread.
///
/// Runs an r-adding walk indefinitely, emitting a [`DpRecord`] each time the
/// walk lands on a distinguished point.  Restarts from a fresh random state
/// after `2^(theta + 10)` steps without a DP (dead-walk escape).
///
/// Exits when `stop` is set or the `tx` channel is closed (coordinator found k).
fn run_walker<F: Fp>(
    curve: &Curve,
    g: &AffinePoint<F>,
    q: &AffinePoint<F>,
    n: u64,
    table: &AddendTable<F>,
    theta: u32,
    walk_id: usize,
    seed: u64,
    stop: &AtomicBool,
    tx: &Sender<DpRecord>,
) {
    // Dead-walk threshold: restart after this many steps without a DP.
    // 2^(theta + 10) gives ~1024 expected DPs per walk before restart.
    let max_steps_without_dp: u64 = 1u64 << (theta.min(53) + 10);

    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut walk = WalkState::<F>::new_random(curve, g, q, n, &mut rng);
    let mut steps_since_dp: u64 = 0;

    loop {
        if stop.load(Ordering::Relaxed) {
            return;
        }

        walk.step(curve, table, n);
        steps_since_dp += 1;

        // Check for distinguished point.
        let pt = walk.to_affine(curve);
        let x_low = match &pt {
            AffinePoint::Infinity => 0u64,
            AffinePoint::Finite { x, .. } => x.to_uint().as_words()[0],
        };

        if is_distinguished(x_low, theta) {
            let dp = DpRecord { x_low, a: walk.a, b: walk.b, walk_id };
            // Send the DP; if the channel is closed the coordinator has finished.
            if tx.send(dp).is_err() {
                return;
            }
            steps_since_dp = 0;
        }

        // Dead-walk escape: restart if we have gone too long without a DP.
        if steps_since_dp >= max_steps_without_dp {
            walk = WalkState::<F>::new_random(curve, g, q, n, &mut rng);
            steps_since_dp = 0;
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::curve::test_curves::{tiny_a, TINY_A_N};
    use crate::field::FpMonty;

    // ── solve_brent tests (Phase 4) ───────────────────────────────────────────

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

    // ── solve_dp tests (Phase 5) ──────────────────────────────────────────────

    /// Solve a known 20-bit DLP on tiny_a with a single walker.
    ///
    /// Uses theta = 4 (1-in-16 DPs) so the coordinator sees enough DPs quickly
    /// on the small curve without excessive memory.
    #[test]
    fn solve_dp_tiny() {
        let k_target: u64 = 77;
        let curve = tiny_a();
        let g: AffinePoint<FpMonty> = curve.generator();
        let q = curve.scalar_mul(&g, &Uint::<4>::from(k_target));

        let k = solve_dp(&curve, &g, &q, TINY_A_N, 1, 4, 0xDEAD_BEEF)
            .expect("solve_dp failed on tiny_a");

        let check = curve.scalar_mul(&g, &Uint::<4>::from(k));
        assert_eq!(check, q, "solve_dp_tiny: k·G ≠ Q (k={k}, expected k={k_target})");
    }

    /// Solve a 20-bit DLP on tiny_a with 2 walkers.
    ///
    /// Uses theta = 4 (1-in-16 DPs) on the 20-bit test curve so the test
    /// completes quickly even in debug mode.  The two-walker setup exercises
    /// the parallel coordination path without the cost of a larger curve.
    #[test]
    fn solve_dp_parallel() {
        let k_target: u64 = 500_000;
        let curve = tiny_a();
        let g: AffinePoint<FpMonty> = curve.generator();
        let q = curve.scalar_mul(&g, &Uint::<4>::from(k_target));

        let k = solve_dp(&curve, &g, &q, TINY_A_N, 2, 4, 0xCAFE_BABE)
            .expect("solve_dp failed on tiny_a with 2 walkers");

        let check = curve.scalar_mul(&g, &Uint::<4>::from(k));
        assert_eq!(check, q, "solve_dp_parallel: k·G ≠ Q (k={k}, expected k={k_target})");
    }
}
