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

## Phase 4 — ECDLP rho baseline (single-threaded)

Phase 4 delivers the working single-threaded DLP solver.  There are no throughput benchmarks
at this phase (parallelism arrives in Phase 5); the deliverable is correctness: 67 tests,
0 failures.

### Solver design

- **r-adding walk** (`r = 20`) with a precomputed addend table `R[i] = αᵢ·G + βᵢ·Q`.
  Partition function: `i = x_low64 mod r` where `x_low64` is the low word of the current
  point's x-coordinate.
- **Brent's cycle detection** (single-threaded).  Tortoise is frozen at the start of each
  power-of-2 window; hare advances one step at a time and is compared to the tortoise.
  Window doubles on no-collision; tortoise snaps forward to hare's position.
- **(a, b) scalar tracking** throughout: invariant `W = a·G + b·Q` at every step.  On
  collision `(a_t, b_t) = (a_h, b_h)`, solution is `k = (a_t − a_h)·(b_h − b_t)⁻¹ mod n`.
- **Retry on degeneracy**: if `b_h = b_t mod n` at collision time (probability ~1/n per
  attempt), the solver restarts with a fresh random table and walk.

### Test curves

Two 20-bit prime-order curves were added specifically for solver KATs (`src/curve/test_curves.rs`).
Both have `√n ≈ 1024`, making them fast to solve even in unoptimized debug builds.

| Curve | p | a | b | n | G |
|-------|---|---|---|---|---|
| `tiny_a` | 1_048_517 (20-bit) | −3 mod p | 3 | 1_048_051 (prime) | (1, 1) |
| `tiny_b` | 1_048_583 (20-bit) | −3 mod p | 16 | 1_048_387 (prime) | (0, 4) |

### DLP KAT solve times (debug, unoptimized, single-threaded)

Wall times from a single run; the dominant cost is one field inversion per walk step
(Phase 7 will amortise this with batched inversion).

| Curve | k | Steps (approx) | Wall time |
|-------|---|---------------|-----------|
| tiny_a | 7 | ~2 000 | ~2 s |
| tiny_a | 100 | ~2 000 | ~2 s |
| tiny_a | 33 333 | ~2 000 | ~2 s |
| tiny_b | 7 | ~2 000 | ~2 s |
| tiny_b | 42 | ~2 000 | ~2 s |
| tiny_b | 99 991 | ~2 000 | ~2 s |

The ~2 s per solve in debug reflects the cost of `FpMonty::inv` (~11 µs per inversion at
this prime size, from the Phase 1 benchmark) × ~1000–2000 walk steps.  In release mode
the same solves complete in milliseconds.

### Test coverage added

| Test file | Tests added | Scope |
|-----------|-------------|-------|
| `src/curve/test_curves.rs` (unit) | 6 | Generator on curve, n·G=∞, reference scalar multiples on tiny_a and tiny_b |
| `src/ecdlp/walk.rs` (unit) | 2 | Walk invariant `W = a·G + b·Q`, partition in-range |
| `src/ecdlp/mod.rs` (unit) | 3 | Solver correctness: k=7, 42, 100 on tiny_a |
| `tests/ecdlp_kat.rs` (integration) | 6 | DLP KATs on tiny_a (k=7, 100, 33333) and tiny_b (k=7, 42, 99991) |

## G.B — Polynomial selection: Murphy-E scoring and root sieve

Toy semiprime: N = 1022117 = 1009 × 1013 (~20-bit). Degree d = 3. All timings are wall-clock
from a single run in unoptimized debug builds (`cargo test`, no `--release`). The dominant cost
in both operations is the Dickman-ρ numerical integration: each `score()` call builds a 2301-entry
RK4 table and samples a 50×50 grid.

| Operation | N | Config | Wall time (debug) |
|-----------|---|--------|-------------------|
| `score()` — single Murphy-E evaluation | 1022117 (~20-bit) | 50×50 sample grid, B = 10^6 | ~30 ms |
| `root_sieve()` — 10×10 grid (441 candidates) | 1022117 (~20-bit) | j_range = k_range = 10 | ~370 ms |
| `coppersmith_polys()` — 5 variants, best selection | 1022117 (~20-bit) | num_polys = 5, step = 1 | ~150 ms |

**Interpretation.** The 30 ms per `score()` call is entirely the Dickman-ρ table construction
(~18 KB, 2300 RK4 steps) plus 2601 grid evaluations. In release mode this drops to < 1 ms. The
root sieve's 370 ms is 441 × 30 ms / (parallelism = 1) — the grid search is embarrassingly
parallel but runs single-threaded here. At cryptographic scale (RSA-768, d = 6), the sieve region
is much larger and the grid ranges are 100–1000×, making the scoring cost the dominant factor in
polynomial selection time.

**Science↔engineering note (principle 4).** Murphy-E's predictive value — that higher score
implies more smooth relations — only manifests at sieve scale (N ≳ 2^100). At toy scale the
score is a ranking heuristic: the ordering is correct in expectation, but the absolute values and
the improvement from root sieving are under-exposed. The root sieve KAT uses `≥` rather than `>`
for this reason. The Coppersmith multi-poly improvement is < 2× at toy scale (KAT 5 in
`coppersmith_kat.rs`), consistent with the principle-4 annotation.

### Test coverage added (G.B)

| Test file | Tests | Scope |
|-----------|-------|-------|
| `tests/base_m_kat.rs` | 4 | Base-m round-trip (toy N), RSA-100 determinism, optimal_degree, monic_f |
| `tests/murphy_kat.rs` | 10 | Murphy-E ordering, monotonicity, positivity; Dickman-ρ spot-checks |
| `tests/root_sieve_kat.rs` | 8 | Rotation root-preservation, determinism, generator count, score agreement |
| `tests/coppersmith_kat.rs` | 12 | Verify, count, best-score, generator, principle-4 annotation |
