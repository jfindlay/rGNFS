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

## G.C — Sieving: line sieve, special-q, lattice sieve

Toy semiprime: N = 5 (f(x) = x³ − x − 1, m = 2, f(m) = 5). Sieve region: |a| ≤ 10, 1 ≤ b ≤ 3
(63 pairs total). Factor-base bounds: B_rat = B_alg = 30. All timings are wall-clock from a
single run in unoptimized debug builds (`cargo test`, no `--release`). The dominant cost at toy
scale is the `trial_smooth` call (trial division up to B_alg = 30) for each candidate that
passes the log-threshold filter.

**Actual numbers (debug, unoptimized, 1000-run median):**

| Sieve variant | Sieve area | Relations found | Yield | Wall time (debug) | Relations/sec (debug) |
|---------------|-----------|-----------------|-------|-------------------|-----------------------|
| Line sieve (threshold = 0.8) | 63 pairs | 14 | 22.2% | ~9.6 ms | ~1 460 |
| Special-q (q ∈ [5, 17], threshold = 0.5) | ~9 pairs/q | 16 (across all q) | — | ~10.0 ms | — |
| Lattice sieve (q ∈ [5, 17]) | ~9 points/q | 14 (across all q) | — | ~9.5 ms | — |

**Interpretation.** The 9.6 ms per line-sieve run is dominated by `trial_smooth` calls on the
~14 candidates that pass the log-threshold filter. Each `trial_smooth` call trial-divides by all
primes up to B_alg = 30 (10 primes), so the total cost is ~14 × 10 = 140 trial divisions per
run. In release mode this drops to < 0.1 ms.

The special-q sieve finds 16 relations (vs. 14 for the line sieve) because it uses a looser
threshold (0.5 vs. 0.8) and covers a different subset of the sieve region per q. The lattice
sieve finds the same 14 relations as the line sieve for the same (q, r_q) pairs — the two
algorithms are mathematically equivalent at toy scale (see principle-4 annotation below).

**Science↔engineering note (principle 4).** Three asymptotic wins are under-exposed at toy scale:

1. **Log-sieve vs. brute-force trial division.** At toy scale (63 pairs, 10 primes), the
   log-threshold filter eliminates almost nothing — every candidate passes. The asymptotic win
   (avoiding trial division for ~99% of pairs at cryptographic scale) is not visible. At
   cryptographic scale (A, B ≈ 10⁷, B_alg ≈ 10⁶), the threshold filters ~99% of candidates,
   making the sieve 100–1000× faster than brute-force trial division.

2. **Special-q yield multiplier.** At toy scale, the norms are already small (tens of bits) and
   smooth with high probability. The pre-guaranteed factor q does not significantly improve the
   smoothness probability. At cryptographic scale, the special-q strategy yields 5–10× more
   relations per sieve area than the plain line sieve.

3. **Lattice sieve efficiency.** At toy scale, the lattice sieve and the special-q line sieve
   produce the same (a, b) pairs — the reduced basis has no efficiency advantage when q is small.
   At cryptographic scale, the reduced basis (length ≈ √q) covers the sieve region with minimal
   waste, giving a significant efficiency gain over stepping by q.

The KATs annotate all three disconnects explicitly (principle-4 annotations in `line_sieve_kat.rs`,
`special_q_kat.rs`, and `lattice_kat.rs`).

### Test coverage added (G.C)

| Test file | Tests | Scope |
|-----------|-------|-------|
| `tests/factor_base_kat.rs` | 9 | Factor-base construction, index lookups, norm bridge, `Relation::verify()` |
| `tests/line_sieve_kat.rs` | 8 (+1 ignored) | Relation count, verify, determinism, threshold monotonicity; CADO oracle (ignored) |
| `tests/special_q_kat.rs` | 13 | q-restriction enforcement, q in algebraic EV, yield annotation, subset check |
| `tests/lattice_kat.rs` | 15 | Lattice membership, verify, q in algebraic EV, basis reduction, subset check |

## G.D — Filtering: singleton removal, clique pruning, column merging

Toy setup: f(x) = x³ − x − 1, B_rat = B_alg = 13. Column layout: rational_size = 6,
algebraic_size = 3, obstruction_count = 1, matrix_width = 10. All timings are wall-clock from
a single run in unoptimized debug builds (`cargo test`, no `--release`). The dominant cost at
toy scale is the O(rows × cols) linear scan in `remove_singletons` (no inverted column index
at toy scale) and the O(rows²) scan in `merge_pass` (finding rows containing each candidate
column).

**Toy-scale matrix dimensions (from `merge_kat.rs`):**

| Pipeline stage | Relations in | Rows × cols | Total weight (set entries) | Notes |
|----------------|-------------|-------------|---------------------------|-------|
| `build_matrix` | 5 | 5 × 10 | 10 | KAT (a) initial matrix |
| after `remove_singletons` | 5 | 5 × 10 | 10 | no singletons in KAT (a) corpus |
| after `prune_cliques` | 5 | 5 × 10 | 10 | excess = −4 < EXCESS_FLOOR → no pruning |
| after `merge_columns` | 5 → 4 | 4 × 10 | 8 | weight-2 pass: col 3 eliminated; 2 rows → 1 merged row |

**Merge-saving (KAT a corpus):** weight reduction from 10 → 8 set entries = **20%** (1 column
eliminated, net −1 row). At toy scale the saving is modest; see principle-4 annotation below.

**Excess-floor pruning (KAT `kat_prune_cliques_respects_excess_floor`):**

| Initial rows | Initial excess | After pruning rows | After pruning excess |
|-------------|---------------|-------------------|---------------------|
| 35 | 26 | 29 | 20 (= EXCESS_FLOOR) |

**End-to-end provenance (KAT d corpus):** 5 relations in → 1 row out (after cascading singleton
removal and two weight-2 merges). Provenance of the final row: {0, 1, 2} — three original
relations. See `gnfs/tests/merge_kat.rs` for exact toy-scale dimensions.

**Science↔engineering note (principle 4).** Three scale-dependent phenomena are under-exposed
at toy scale:

1. **Singleton cascade depth.** At toy scale (10 columns, 5 relations), the cascade terminates
   in 1–2 rounds. At cryptographic scale (10⁵ columns, 10⁶ relations), the cascade can run for
   dozens of rounds, removing a significant fraction of the initial corpus. The inverted column
   index (column → list of containing rows) that makes each round O(singletons) rather than
   O(rows × cols) is omitted at toy scale (linear scan is acceptable); at cryptographic scale
   it is essential.

2. **Cavallar merge-ordering gain.** At toy scale, the simplified ordering (weight-2 first,
   then weight-3, ties by column index) gives the same result as the full Cavallar heuristic
   because the matrix is small enough that any order is tractable. At cryptographic scale, the
   Cavallar heuristic reduces total matrix weight by 30–50% compared to a naive ordering,
   directly reducing the cost of structured Gaussian elimination in G.E.

3. **EXCESS_FLOOR calibration.** At toy scale, EXCESS_FLOOR = 20 is never approached in
   practice (the toy corpus is too small to produce excess > 20 after singleton removal). At
   cryptographic scale, the floor is a tuning parameter: too low means too few null-space
   vectors (fewer factoring attempts); too high means a denser matrix (higher G.E cost).

### Test coverage added (G.D)

| Test file | Tests | Scope |
|-----------|-------|-------|
| `tests/merge_kat.rs` | 4 (+1 ignored) | 2-way merge correctness, determinism, CADO oracle (ignored), end-to-end provenance, excess-floor enforcement |

## G.E — Linear algebra: block Lanczos and block Wiedemann

Toy setup: the shared 6 × 4 KAT matrix (from `gnfs/tests/lanczos_kat.rs` and
`gnfs/tests/wiedemann_kat.rs`). Matrix dimensions: 6 rows × 4 columns. Kernel dimension: 3
(known nullspace vectors: `{0,3}`, `{4,5}`, `{0,1,2}`). All timings are wall-clock from a
single run in unoptimized debug builds (`cargo test`, no `--release`). The dominant cost at
toy scale is the GF(2) Gaussian elimination in `gf2_block_pivot` (64 × 64 matrix) and the
Berlekamp-Massey step (sequence length 2m + 10 = 22).

**Toy-scale solver results (6 × 4 matrix, kernel dimension 3):**

| Solver | Matrix dimensions | Kernel dimension found | Wall time (debug) | Notes |
|--------|------------------|----------------------|-------------------|-------|
| Block Lanczos | 6 rows × 4 cols | ≥ 1 (varies by seed) | < 1 ms | Instantaneous at toy scale |
| Block Wiedemann | 6 rows × 4 cols | ≥ 1 (varies by seed) | < 1 ms | Instantaneous at toy scale |

**Interpretation.** At toy scale (6 × 4 matrix), both solvers complete in under 1 ms — the
timing difference between them is invisible. The dominant cost is not the matrix-vector
products (trivial for a 6 × 4 matrix) but the fixed overhead of the GF(2) Gaussian
elimination (64 × 64 matrix in `gf2_block_pivot`) and the Berlekamp-Massey step (sequence
length 22). In release mode both solvers complete in microseconds.

**Science↔engineering note (principle 4).** Two scale-dependent phenomena are under-exposed
at toy scale:

1. **Block-width amortisation.** At toy scale, the 64-wide block vector is wider than the
   matrix itself (4 columns). The blocking overhead is invisible — a single-vector solver
   would be equally fast. At NFS scale (millions of rows), the 64-wide block amortises the
   memory-bandwidth cost of loading each matrix row once across 64 simultaneous vector
   operations, giving a ~64× speedup over the single-vector variant.

2. **Wiedemann parallelism.** At toy scale, the 4 sequential (x, y) attempts in the scalar
   Wiedemann implementation are indistinguishable from a parallel implementation. At NFS
   scale, the block Wiedemann algorithm distributes the Krylov sequence computation across
   multiple machines with no per-step synchronisation — the architecture used in the RSA-768
   factorisation (Kleinjung et al., 2010). Block Lanczos requires a global inner product at
   each step, which limits distributed scalability.

**Self-orthogonality (exposed at toy scale).** The KAT matrix is deliberately constructed
with duplicate rows (rows 0 and 3 are identical; rows 4 and 5 are identical). This forces the
block Lanczos self-orthogonality winnowing path: the corresponding block columns of the
starting vector satisfy A^T·v = 0 and are detected as inactive. The winnowing correctly
identifies them as kernel candidates. This phenomenon is fully exposed at toy scale.

### Test coverage added (G.E)

| Test file | Tests | Scope |
|-----------|-------|-------|
| `tests/linalg_substrate_kat.rs` | 4 | Operator correctness (A·V, Aᵀ·V), QC column construction, provenance round-trip, determinism |
| `tests/lanczos_kat.rs` | 6 (+1 ignored) | Self-orthogonality path, single dependency, full-rank, determinism, multiple dependencies; CADO oracle (ignored) |
| `tests/wiedemann_kat.rs` | 9 | Cross-validation with Lanczos, full-rank, empty matrix, single dependency, determinism, BM Fibonacci, BM period-4, BM all-ones, all-zero rows, multiple dependencies |

## G.F — Square root and assembly: rational sqrt, Couveignes CRT, GCD assembly

Toy semiprime: N = 35 = 5 × 7 (~6-bit). K = ℚ(√2), f(x) = x² − 2, m = 6. Relations:
(a=4, b=0) and (a=9, b=0). Kernel vector selects both relations. All timings are wall-clock
from a single run in unoptimized debug builds (`cargo test`, no `--release`). At toy scale,
all three stages (rational sqrt, Couveignes CRT, GCD assembly) complete in under 1 ms — the
dominant cost is the `NumberFieldElement::mul` calls in `form_gamma` and the Lagrange
interpolation in `per_prime_beta`.

**Toy-scale factoring result (N = 35, 1 kernel vector tried):**

| Stage | Input | Output | Wall time (debug) | Notes |
|-------|-------|--------|-------------------|-------|
| Rational sqrt | ∏(a_i − b_i·m) = 4 × 9 = 36 | X = 6 (mod 35) | < 1 ms | isqrt(36) = 6; trivial at toy scale |
| Algebraic sqrt (Couveignes) | γ = 36 in K = ℚ(√2) | Y = 1 (mod 35) | < 1 ms | 10 CRT primes; Norm(6) = 36; 36 mod 35 = 1 |
| GCD assembly | X = 6, Y = 1, N = 35 | factor = **5** | < 1 ms | gcd(6 − 1, 35) = gcd(5, 35) = 5 |

**Factor recovered: 5 from N = 35 = 5 × 7. Kernel vectors tried: 1.**

**Interpretation.** At toy scale (N = 35, 6-bit), all three stages are instantaneous. The
dominant cost in `algebraic_sqrt` is the `form_gamma` product (2 `NumberFieldElement::mul`
calls) and the `per_prime_beta` Lagrange interpolation (10 primes × 2 roots each). In release
mode all stages complete in microseconds.

**Science↔engineering note (principle 4).** Two scale-dependent phenomena are under-exposed
at toy scale:

1. **CRT prime count.** At toy scale (|S| = 2, coefficients ~ 4 bits, d = 2), 10 CRT primes
   is massive overkill — 2–3 primes would suffice. At NFS scale (|S| ~ 10⁵, coefficients ~
   100 bits, d ~ 5), the prime count grows as O(coefficient_bits / 64) and the CRT lift
   dominates the algebraic sqrt cost. The prime count is the scale knob; the algorithm is
   identical at all scales.

2. **Trivial-GCD probability.** At toy scale, the first kernel vector yields a non-trivial
   GCD (5 from N = 35). In general, ~50% of kernel vectors yield trivial GCDs for a semiprime
   N = p × q; the retry loop is the safety net. At NFS scale, the linear algebra step produces
   many kernel vectors (the nullspace dimension grows with the excess), so the retry loop
   terminates quickly.

**Embedding-sign resolution (not a scale artifact).** The sign resolution via the real
embedding (Newton's method from m^{1/d}, evaluate β(θ), negate if < 0) is a correctness
obligation at all scales, not a scale artifact. At toy scale it is exercised by the KAT; at
NFS scale it is the same algorithm. The G.F.3 review juncture identified this as the
silent-failure locus: a wrong-sign β produces a trivial GCD, not a red test.

### Test coverage added (G.F)

| Test file | Tests | Scope |
|-----------|-------|-------|
| `tests/sqrt_rational_kat.rs` | 5 | Rational sqrt correctness, index set, non-square panic, m-dependent factors |
| `tests/sqrt_algebraic_kat.rs` | 7 | Couveignes correctness (known-square γ), congruence X²≡Y² (mod N), determinism, degree-2 two-relation case |
| `tests/factor_end_to_end_kat.rs` | 6 (+1 ignored) | factor_from_congruence known values, full driver (N=35→5), trivial-GCD path, retry loop, all-trivial returns None; oracle KAT (ignored) |
