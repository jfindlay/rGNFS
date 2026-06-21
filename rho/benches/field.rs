//! Field arithmetic benchmark: FpNaive vs FpMonty.
//!
//! Phase 1 deliverable. Measures `mul`, `square`, and `inv` on the secp256k1
//! prime (256-bit) for both implementations, giving the first pedagogical
//! "see the Montgomery-form speedup" moment.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use crypto_bigint::Uint;
use shared_field::{Fp, FpMonty4 as FpMonty, FpNaive4 as FpNaive};

/// secp256k1 prime: p = 2^256 - 2^32 - 977.
fn secp_p() -> Uint<4> {
    Uint::<4>::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
}

// ---------------------------------------------------------------------------
// FpNaive benchmarks
// ---------------------------------------------------------------------------

fn bench_naive_mul(c: &mut Criterion) {
    let p = secp_p();
    let a = FpNaive::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    let b = FpNaive::from_u64(0x1234_5678_9ABC_DEF0, &p);
    c.bench_with_input(BenchmarkId::new("FpNaive", "mul"), &(&a, &b, &p), |bench, (a, b, p)| {
        bench.iter(|| a.mul(b, p));
    });
}

fn bench_naive_square(c: &mut Criterion) {
    let p = secp_p();
    let a = FpNaive::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    c.bench_with_input(BenchmarkId::new("FpNaive", "square"), &(&a, &p), |bench, (a, p)| {
        bench.iter(|| a.square(p));
    });
}

fn bench_naive_inv(c: &mut Criterion) {
    let p = secp_p();
    let a = FpNaive::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    c.bench_with_input(BenchmarkId::new("FpNaive", "inv"), &(&a, &p), |bench, (a, p)| {
        bench.iter(|| a.inv(p));
    });
}

// ---------------------------------------------------------------------------
// FpMonty benchmarks
// ---------------------------------------------------------------------------

fn bench_monty_mul(c: &mut Criterion) {
    let p = secp_p();
    let a = FpMonty::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    let b = FpMonty::from_u64(0x1234_5678_9ABC_DEF0, &p);
    c.bench_with_input(BenchmarkId::new("FpMonty", "mul"), &(&a, &b, &p), |bench, (a, b, p)| {
        bench.iter(|| a.mul(b, p));
    });
}

fn bench_monty_square(c: &mut Criterion) {
    let p = secp_p();
    let a = FpMonty::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    c.bench_with_input(BenchmarkId::new("FpMonty", "square"), &(&a, &p), |bench, (a, p)| {
        bench.iter(|| a.square(p));
    });
}

fn bench_monty_inv(c: &mut Criterion) {
    let p = secp_p();
    let a = FpMonty::from_u64(0xDEAD_BEEF_CAFE_1234, &p);
    c.bench_with_input(BenchmarkId::new("FpMonty", "inv"), &(&a, &p), |bench, (a, p)| {
        bench.iter(|| a.inv(p));
    });
}

criterion_group!(
    naive_benches,
    bench_naive_mul,
    bench_naive_square,
    bench_naive_inv
);
criterion_group!(
    monty_benches,
    bench_monty_mul,
    bench_monty_square,
    bench_monty_inv
);
criterion_main!(naive_benches, monty_benches);
