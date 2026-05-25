# Benchmark results

Recorded per-phase as each phase completes. All numbers are wall-clock medians from Criterion
on the build machine. Exact numbers will drift with hardware; the ratios are the pedagogical
signal.

## Phase 1 — Field arithmetic: FpNaive vs FpMonty

Prime: secp256k1 (256-bit, `p = 2^256 − 2^32 − 977`).

| Operation | FpNaive   | FpMonty  | Speedup |
|-----------|-----------|----------|---------|
| `mul`     | 2.660 µs  | 51.1 ns  | ~52×    |
| `square`  | 2.860 µs  | 46.2 ns  | ~62×    |
| `inv`     | 1.335 ms  | 11.75 µs | ~114×   |

The `mul`/`square` speedup is the core Montgomery-form gain: `crypto-bigint`'s `DynResidue`
avoids the 8-limb widening division in `FpNaive::mul` by keeping values in Montgomery form and
reducing with a single CIOS multiply-then-reduce pass.

The `inv` speedup compounds: both implementations use Fermat (`a^(p−2) mod p`, ~256
squarings + ~128 multiplications), so the `inv` ratio approximately tracks the `mul` ratio.

The `square` speedup is slightly larger than `mul` because `DynResidue::square` uses a
dedicated squaring path that saves ~half the cross-term multiplications.

## Phase 2 — Integer factorization: Floyd vs Brent vs Brent+batched-GCD

Semiprimes: products of two roughly equal-size primes.  All timings factor the *same* semiprime
at each bit size, c=1, x₀=2.

| Semiprime | Floyd    | Brent    | brent\_batched (bs=128) | Speedup (F→BB) |
|-----------|----------|----------|-------------------------|----------------|
| 30-bit    | 631 µs   | 865 µs   | 153 µs                  | ~4.1×          |
| 50-bit    | 666 µs   | 1118 µs  | 239 µs                  | ~2.8×          |
| 64-bit    | 18.4 ms  | 17.7 ms  | 3.97 ms                 | ~4.6×          |

**Brent vs Floyd.** For small semiprimes that terminate in few steps (30- and 50-bit), Brent's
constant overhead (skip-ahead phase + doubled window bookkeeping) outweighs its asymptotic
savings, making it appear slower.  At 64-bit the asymptotic advantage (~33% fewer effective
function evaluations vs Floyd's step-by-step tortoise) starts to dominate and Brent edges ahead.

**Batched GCD.** The `brent_batched` speedup is consistent across sizes (~3–5×): each batch of
128 walk steps requires only one GCD call instead of 128, trading 127 GCDs for 127 multiplications.
Since GCD (via the Euclidean algorithm on 128-bit values) costs significantly more than a modular
multiply, the savings are real and consistent.  The expected group-operations count is unchanged;
only the cost per detection step drops.
