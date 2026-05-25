//! ECDLP benchmark: Brent (single-threaded) vs DP parallel (1, 2, 4 walkers).
//!
//! Phase 5 deliverable.  Measures wall time to solve a fixed 35-bit DLP on
//! `secp_k1_toy` with each solver variant.  The pedagogical signal is the
//! parallel speedup: each doubling of walkers should roughly halve wall time
//! (up to the overhead of thread coordination and the DP channel).
//!
//! # Fixed DLP
//!
//! `k = 12_345_678_901`, `G` = curve generator, `Q = k·G` precomputed.
//! `theta = 8` (1-in-256 DPs).  Expected rho steps ≈ sqrt(k) ≈ 111_111.
//!
//! # Note on Criterion and parallelism
//!
//! Criterion runs each benchmark function many times in the same process.
//! The parallel solver spawns fresh threads per call, so thread-pool reuse is
//! not an issue here (unlike rayon-based solvers).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use crypto_bigint::Uint;
use rho::curve::secp_k1_toy::{secp_k1_toy, N as SECP_N};
use rho::curve::AffinePoint;
use rho::ecdlp::{solve_brent, solve_dp};
use rho::field::FpMonty;

// ── Fixed DLP parameters ──────────────────────────────────────────────────────

/// The discrete logarithm target: `k = 12_345_678_901` (35-bit value).
const K_TARGET: u64 = 12_345_678_901;

/// DP threshold: 1-in-256 points are distinguished.
const THETA: u32 = 8;

/// RNG seed (fixed for reproducibility across runs).
const SEED: u64 = 0x0123_4567_89AB_CDEF;

/// Precompute `Q = k·G` once for the benchmark.
fn make_q() -> (rho::curve::Curve, AffinePoint<FpMonty>) {
    let curve = secp_k1_toy();
    let g: AffinePoint<FpMonty> = curve.generator();
    let q = curve.scalar_mul(&g, &Uint::<4>::from(K_TARGET));
    (curve, q)
}

// ── Brent baseline ────────────────────────────────────────────────────────────

fn bench_brent(c: &mut Criterion) {
    let (curve, q) = make_q();
    let g: AffinePoint<FpMonty> = curve.generator();

    c.bench_function("ecdlp/brent", |b| {
        b.iter(|| {
            solve_brent(&curve, &g, &q, SECP_N, SEED, 20)
                .expect("brent failed")
        });
    });
}

// ── DP parallel solver ────────────────────────────────────────────────────────

fn bench_solve_dp(c: &mut Criterion) {
    let (curve, q) = make_q();
    let g: AffinePoint<FpMonty> = curve.generator();

    let mut group = c.benchmark_group("ecdlp/solve_dp");

    for &num_walkers in &[1usize, 2, 4] {
        group.bench_with_input(
            BenchmarkId::new("walkers", num_walkers),
            &num_walkers,
            |b, &w| {
                b.iter(|| {
                    solve_dp(&curve, &g, &q, SECP_N, w, THETA, SEED)
                        .expect("solve_dp failed")
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_brent, bench_solve_dp);
criterion_main!(benches);
