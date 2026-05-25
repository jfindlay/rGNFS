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

## Phase 3 — Curve arithmetic

Phase 3 delivers the elliptic-curve group law and two concrete curves.  There are no
performance benchmarks at this phase (curve operations are not the bottleneck until Phase 5+);
the deliverable is correctness: 50 tests, 0 failures.

### Group-law implementation

- **`AffinePoint<F>`** — finite point `(x, y)` or point at infinity; generic over `F: Fp`.
- **`JacobianPoint<F>`** — Jacobian projective `(X:Y:Z)`; avoids inversions during
  doubling/addition chains.
- **`Curve`** — parameter struct (`p`, `a`, `b`, `n`, `gx`, `gy`); three operations:
  `double_jacobian`, `add_jacobian` (J+J, 16M+4S), `add_mixed` (J+A, 8M+3S).
- **`scalar_mul`** — left-to-right double-and-add using `add_mixed` for the hot path.

### Concrete curves

| Curve | Prime p | a | b | n |
|-------|---------|---|---|---|
| `generic` | 2^63 − 25 (63-bit) | p − 3 (≡ −3) | 1 | unknown† |
| `secp_k1_toy` | 4_611_686_018_427_395_203 (63-bit) | 0 | 7 | 4_611_686_022_420_787_627 (prime) |

† The generic curve's group order is not required for Phases 3–4; tests use small-scalar
  reference points rather than n·G = ∞.

### secp_k1_toy endomorphism

The GLV endomorphism `φ(x, y) = (β·x mod p, y)` satisfies `φ(P) = λ·P`:

| Constant | Value |
|----------|-------|
| β | `2_535_098_114_878_923_204` |
| λ | `441_215_077_713_529_363` |

Verified: β³ ≡ 1 (mod p), λ² + λ + 1 ≡ 0 (mod n), φ(G) = λ·G.

The curve was constructed via the CM method (Cornacchia): 4p = t² + 3v², n = p + 1 − t,
both p and n are prime, both ≡ 1 (mod 3).

### Test coverage

| Test file | Tests | Scope |
|-----------|-------|-------|
| `src/curve/mod.rs` (unit) | 8 | Group-law axioms on a tiny 5-point curve over GF(7) |
| `src/curve/generic.rs` (unit) | 4 | Reference scalar multiples, consistency, negation |
| `src/curve/secp_k1_toy.rs` (unit) | 6 | Reference multiples, n·G=∞, endomorphism, β and λ constants |
| `tests/ecdlp_kat.rs` (integration) | 8 | Both curves, linearity, endomorphism KAT, k256 cross-check |
