//! ECDLP benchmark: Brent (single-threaded) vs DP parallel (1, 2, 4 walkers)
//! vs DP with negation map (Phase 6).
//!
//! Phase 5 deliverable extended in Phase 6.  Measures wall time to solve a
//! fixed 35-bit DLP on `secp_k1_toy` with each solver variant.
//!
//! The Phase 5 pedagogical signal is the parallel speedup: each doubling of
//! walkers should roughly halve wall time.
//!
//! The Phase 6 pedagogical signal is the negation-map speedup: `solve_dp_negmap`
//! with 2 walkers should be ~√2 faster than `solve_dp` with 2 walkers, because
//! the negation map halves the effective group size.
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
use rho::ecdlp::{solve_brent, solve_dp, solve_dp_batch, solve_dp_glv, solve_dp_negmap};
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

// ── Phase 6: DP vs DP+negmap comparison ──────────────────────────────────────

/// Compare `solve_dp` and `solve_dp_negmap` at 2 walkers on the same 35-bit DLP.
///
/// The negation map should reduce wall time by ~√2 vs plain DP.
fn bench_negmap_vs_dp(c: &mut Criterion) {
    let (curve, q) = make_q();
    let g: AffinePoint<FpMonty> = curve.generator();

    let mut group = c.benchmark_group("ecdlp/negmap_vs_dp");

    // Plain DP with 2 walkers (Phase 5 baseline).
    group.bench_function("solve_dp/walkers=2", |b| {
        b.iter(|| {
            solve_dp(&curve, &g, &q, SECP_N, 2, THETA, SEED)
                .expect("solve_dp failed")
        });
    });

    // DP + negation map with 2 walkers (Phase 6).
    group.bench_function("solve_dp_negmap/walkers=2", |b| {
        b.iter(|| {
            solve_dp_negmap(&curve, &g, &q, SECP_N, 2, THETA, SEED)
                .expect("solve_dp_negmap failed")
        });
    });

    group.finish();
}

// ── Phase 7: batched-inversion vs negmap comparison ──────────────────────────

/// Compare `solve_dp_negmap` (Phase 6) vs `solve_dp_batch` (Phase 7) at 2 walkers
/// on the same 35-bit DLP.
///
/// The Phase 7 pedagogical signal is the batched-inversion speedup: with
/// `batch_size = 16`, each thread performs 1 inversion + 45 multiplications
/// instead of 16 inversions per 16 steps, reducing the dominant per-step cost.
fn bench_batch_vs_negmap(c: &mut Criterion) {
    let (curve, q) = make_q();
    let g: AffinePoint<FpMonty> = curve.generator();

    let mut group = c.benchmark_group("ecdlp/batch_vs_negmap");

    // Phase 6 baseline: negmap with 2 walkers.
    group.bench_function("solve_dp_negmap/walkers=2", |b| {
        b.iter(|| {
            solve_dp_negmap(&curve, &g, &q, SECP_N, 2, THETA, SEED)
                .expect("solve_dp_negmap failed")
        });
    });

    // Phase 7: batched inversion with 2 walkers, batch_size=16.
    group.bench_function("solve_dp_batch/walkers=2/batch=16", |b| {
        b.iter(|| {
            solve_dp_batch(&curve, &g, &q, SECP_N, 2, 16, THETA, SEED)
                .expect("solve_dp_batch failed")
        });
    });

    group.finish();
}

// ── Phase 8: GLV vs batch comparison ─────────────────────────────────────────

/// Compare `solve_dp_negmap` (Phase 6), `solve_dp_batch` (Phase 7), and
/// `solve_dp_glv` (Phase 8) at 2 walkers on the same 35-bit DLP.
///
/// The Phase 8 pedagogical signal is the GLV speedup: `solve_dp_glv` collapses
/// the 6-orbit `{W, φ(W), φ²(W), −W, −φ(W), −φ²(W)}` to a single canonical
/// representative, reducing the effective group size by 6 and the expected
/// rho steps by √6 vs the plain walk (√3 vs negmap alone).
fn bench_glv_vs_batch(c: &mut Criterion) {
    let (curve, q) = make_q();
    let g: AffinePoint<FpMonty> = curve.generator();

    let mut group = c.benchmark_group("ecdlp/glv_vs_batch");

    // Phase 6 baseline: negmap with 2 walkers.
    group.bench_function("solve_dp_negmap/walkers=2", |b| {
        b.iter(|| {
            solve_dp_negmap(&curve, &g, &q, SECP_N, 2, THETA, SEED)
                .expect("solve_dp_negmap failed")
        });
    });

    // Phase 7: batched inversion with 2 walkers, batch_size=16.
    group.bench_function("solve_dp_batch/walkers=2/batch=16", |b| {
        b.iter(|| {
            solve_dp_batch(&curve, &g, &q, SECP_N, 2, 16, THETA, SEED)
                .expect("solve_dp_batch failed")
        });
    });

    // Phase 8: GLV endomorphism with 2 walkers, batch_size=16.
    group.bench_function("solve_dp_glv/walkers=2/batch=16", |b| {
        b.iter(|| {
            solve_dp_glv(&curve, &g, &q, SECP_N, 2, 16, THETA, SEED)
                .expect("solve_dp_glv failed")
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_brent,
    bench_solve_dp,
    bench_negmap_vs_dp,
    bench_batch_vs_negmap,
    bench_glv_vs_batch,
);
criterion_main!(benches);
