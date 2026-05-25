//! Pollard rho factorization: Floyd, Brent, and batched-GCD variants.
//!
//! All variants share the pseudorandom map `f(x) = (x² + c) mod N` and detect
//! a non-trivial factor via `gcd(|xᵢ − xⱼ|, N)`.  Arithmetic is in `u128`, which
//! covers the plan's 30–80-bit target range comfortably.  For inputs where
//! `N < 2^64`, products `x*x` fit directly in `u128`.  For 64–80-bit N the
//! `mulmod` helper uses `u128::carrying_mul` to form the 256-bit product and
//! reduces it mod N in two u128 steps.  N up to 80 bits is the supported range;
//! larger N is not tested and may panic in debug mode.
//!
//! ## Variants
//!
//! - [`floyd`] — baseline Floyd cycle detection.  Two pointers advance at speed
//!   1 and 2; GCD tested at every step.
//! - [`brent`] — Brent's improved cycle detection, ~24% fewer `f` evaluations
//!   than Floyd on average.
//! - [`brent_batched`] — Brent + Montgomery's batched-GCD trick: accumulate a
//!   product of `|xᵢ − y|` values over a batch of `batch_size` steps, take one
//!   GCD per batch.  Falls back to step-by-step on `gcd = N`.
//! - [`factor`] — public entry point: runs `brent_batched` in parallel over
//!   multiple `c` values using `rayon`, returning the first non-trivial factor
//!   found.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use rayon::prelude::*;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Compute `2^exp mod m` via iterative doubling-and-reduction.
///
/// At each iteration the accumulator stays below `m`, so doubling it gives a
/// value below `2m`.  For `m < 2^127` this fits in a `u128` and the reduction
/// is a single conditional subtract.  This avoids the need for any widening
/// multiplication during the exponentiation itself.
#[inline]
fn pow2_mod(exp: u32, m: u128) -> u128 {
    let mut r = 1u128;
    for _ in 0..exp {
        // r < m < 2^127; doubling gives r < 2^128, safe in u128.
        r <<= 1;
        if r >= m {
            r -= m;
        }
    }
    r
}

/// Modular multiplication `a * b mod m` for `a, b < m < 2^80`.
///
/// Uses `u128::carrying_mul` to form the exact 256-bit product `(lo, hi)`, then
/// reduces in two steps:
///
/// 1. If `hi == 0` the 128-bit `lo` is reduced directly.
/// 2. Otherwise: `a*b = hi * 2^128 + lo ≡ hi * r2_128 + lo (mod m)`, where
///    `r2_128 = 2^128 mod m`.  For `m < 2^80` the `hi` word satisfies
///    `hi < m^2 / 2^128 < 2^(160-128) = 2^32`, so `hi * r2_128 < 2^112` and
///    the final multiplication fits in a `u128`.
///
/// # Panics (debug)
///
/// Panics in debug builds if `a >= m || b >= m || m >= 2^127`.
#[inline]
fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    debug_assert!(a < m && b < m && m < (1u128 << 127), "mulmod: inputs out of range");
    let (lo, hi) = a.carrying_mul(b, 0);
    if hi == 0 {
        return lo % m;
    }
    // r2_128 = 2^128 mod m.  For m < 2^80, r2_128 < m < 2^80.
    let r2_128 = pow2_mod(128, m);
    // hi < 2^32 (for m < 2^80), r2_128 < 2^80: product < 2^112, safe.
    (hi * r2_128 % m + lo % m) % m
}

/// Step function: `f(x) = (x² + c) mod n`.
#[inline]
fn f(x: u128, c: u128, n: u128) -> u128 {
    (mulmod(x, x, n) + c) % n
}

/// Euclidean GCD.
#[inline]
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// Absolute difference: `|a − b|`.
#[inline]
fn abs_diff(a: u128, b: u128) -> u128 {
    if a >= b { a - b } else { b - a }
}

// ---------------------------------------------------------------------------
// Floyd's cycle detection
// ---------------------------------------------------------------------------

/// Pollard rho with Floyd's cycle detection (baseline).
///
/// Two pointers — tortoise (speed 1) and hare (speed 2) — advance together;
/// at each step we test `gcd(|tortoise − hare|, n)`.  Terminates when either
/// a non-trivial factor is found or the pointers collide with `gcd = n`
/// (rare degenerate failure).
///
/// # Arguments
///
/// * `n` — composite number to factor (must be > 1, not prime, `n < 2^80`).
/// * `c` — polynomial constant; avoid 0 and `n − 2` (degenerate cycles).
/// * `x0` — starting point (typically 2).
///
/// # Returns
///
/// A non-trivial factor of `n`, or `None` on degenerate failure (retry with
/// a different `c`).
pub fn floyd(n: u128, c: u128, x0: u128) -> Option<u128> {
    let mut tortoise = x0;
    let mut hare = x0;
    loop {
        tortoise = f(tortoise, c, n);
        hare = f(f(hare, c, n), c, n);
        let d = gcd(abs_diff(tortoise, hare), n);
        if d == n {
            return None;
        }
        if d != 1 {
            return Some(d);
        }
    }
}

// ---------------------------------------------------------------------------
// Brent's cycle detection
// ---------------------------------------------------------------------------

/// Pollard rho with Brent's cycle detection (~24% fewer f-evaluations vs Floyd).
///
/// Brent's algorithm advances the hare freely for a power-of-2 window `r`, then
/// tests GCD against the tortoise (fixed at the start of the window).  This
/// eliminates the redundant second-pointer evaluation Floyd requires, and the
/// tortoise advances only O(log λ) times total where λ is the cycle length.
///
/// # Arguments
///
/// * `n`, `c`, `x0` — same as [`floyd`].
pub fn brent(n: u128, c: u128, x0: u128) -> Option<u128> {
    let mut y = x0;
    let mut r: u64 = 1;
    loop {
        let x = y; // tortoise: fixed for this r-step window
        // Advance hare r steps (skip-ahead phase — not compared to tortoise yet).
        for _ in 0..r {
            y = f(y, c, n);
        }
        // Comparison phase: advance hare another r steps, testing against tortoise.
        let mut ys = y;
        for _ in 0..r {
            ys = f(ys, c, n);
            let d = gcd(abs_diff(x, ys), n);
            if d == n {
                return None;
            }
            if d != 1 {
                return Some(d);
            }
        }
        r <<= 1;
    }
}

// ---------------------------------------------------------------------------
// Brent + Montgomery batched GCD
// ---------------------------------------------------------------------------

/// Pollard rho with Brent's cycle detection and Montgomery's batched-GCD trick.
///
/// Instead of testing `gcd(|x − y|, n)` at every step, accumulate the product
/// `Π |x − yᵢ|` over `batch_size` hare steps and take a single GCD per batch.
/// One GCD + `batch_size` multiplications dominates `batch_size` GCDs by a
/// factor of ~4–8× when `batch_size` is 64–128.
///
/// On batch failure (`gcd = n`), falls back to step-by-step [`brent_fallback`]
/// from the pre-batch snapshot to pinpoint the actual factor.
///
/// # Arguments
///
/// * `n`, `c`, `x0` — same as [`floyd`].
/// * `batch_size` — number of steps accumulated before each GCD (typical: 128).
pub fn brent_batched(n: u128, c: u128, x0: u128, batch_size: usize) -> Option<u128> {
    let mut y = x0;
    let mut r: u64 = 1;
    loop {
        let x = y; // tortoise fixed for this window
        for _ in 0..r {
            y = f(y, c, n);
        }
        let mut k: u64 = 0;
        let mut ys = y;
        while k < r {
            let snapshot = ys; // hare position before this batch
            let batch = batch_size.min((r - k) as usize);
            let mut product = 1u128;
            for _ in 0..batch {
                ys = f(ys, c, n);
                let delta = abs_diff(x, ys);
                if delta == 0 {
                    // Zero delta ⟹ accumulated product stays zero; fall back to pinpoint.
                    return brent_fallback(n, c, snapshot, x);
                }
                product = mulmod(product, delta, n);
            }
            let d = gcd(product, n);
            if d == n {
                return brent_fallback(n, c, snapshot, x);
            }
            if d != 1 {
                return Some(d);
            }
            k += batch as u64;
        }
        r <<= 1;
    }
}

/// Step-by-step fallback used by [`brent_batched`] when a batch GCD equals `n`.
///
/// Resumes from the hare snapshot `ys` one step at a time, testing GCD against
/// the tortoise `x`, to pinpoint the factor that the batch overshot.
fn brent_fallback(n: u128, c: u128, mut ys: u128, x: u128) -> Option<u128> {
    loop {
        ys = f(ys, c, n);
        let d = gcd(abs_diff(x, ys), n);
        if d == n {
            return None;
        }
        if d != 1 {
            return Some(d);
        }
    }
}

// ---------------------------------------------------------------------------
// Multi-c parallel entry point
// ---------------------------------------------------------------------------

/// Factor `n` using parallel Pollard rho (Brent + batched GCD) over multiple `c` values.
///
/// Spawns rayon tasks over `c ∈ 1..=max_c`, skipping degenerate constants:
/// c = 0 gives a fixed point at 0; c = n−2 gives a trivial cycle via the
/// identity `x² − 2 = (x−1)² − 1`.  The first task to find a factor signals
/// all others to stop via a shared atomic flag.
///
/// # Arguments
///
/// * `n` — composite number to factor (`1 < n < 2^80`).
/// * `max_c` — number of `c` values to try in parallel (typical: logical CPU count).
/// * `batch_size` — batch size for GCD accumulation (typical: 128).
///
/// # Returns
///
/// A non-trivial factor of `n`, or `None` if all `c` values are degenerate
/// (should not occur in practice for composite `n` with sufficient `max_c`).
pub fn factor(n: u128, max_c: u64, batch_size: usize) -> Option<u128> {
    if n <= 1 {
        return None;
    }
    // Fast path: even numbers have 2 as a trivial factor.
    if n % 2 == 0 {
        return Some(2);
    }
    let found = Arc::new(AtomicBool::new(false));
    (1u64..=max_c)
        .into_par_iter()
        .filter(|&c| c != 0 && (c as u128) != n.wrapping_sub(2))
        .find_map_any(|c| {
            if found.load(Ordering::Relaxed) {
                return None;
            }
            let r = brent_batched(n, c as u128, 2, batch_size);
            if r.is_some() {
                found.store(true, Ordering::Relaxed);
            }
            r
        })
}
