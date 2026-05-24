# PLAN — Rust Pollard rho (factoring + ECDLP)

## Scope

Single Rust crate implementing two Pollard rho variants with all canonical optimizations:

- **Integer factorization rho** (Pollard 1975, Brent 1980, Montgomery 1987).
- **ECDLP rho** (Pollard 1978, Teske 1998, van Oorschot–Wiener 1999, plus negation map and GLV).

Target audience: pedagogical / research-grade. Goal is a clean, fast, *readable* CPU implementation
that a reader can extend or benchmark. Not a record-attempt platform. Not constant-time. Not a
cryptographic library — these are *attacks on* cryptographic problems, run on toy parameters.

**Out of scope (deliberately).** Binary-field (GF(2^m)) curves and the order-6 Koblitz automorphism;
GPU / FPGA backends; AVX-512 hand-tuning; distributed (multi-machine) deployment; durable DP
storage. The architecture leaves a seam at the DP submission boundary so a distributed layer could
be added later, but that layer is not built.

## Architectural decisions

### Crate structure: one crate, two modules, no shared trait

Honest about the asymmetry. The two algorithms share only the cycle-finding control flow, and even
that diverges (Brent's algorithm for factoring; distinguished points for ECDLP). A `WalkStep` trait
over both would abstract over the wrong axis.

```
src/
  lib.rs           # re-exports
  field/
    mod.rs         # Fp trait
    naive.rs       # FpNaive: schoolbook modular arithmetic
    monty.rs       # FpMonty: Montgomery form (crypto-bigint MontyForm)
  curve/
    mod.rs         # EcPoint (affine + Jacobian), Curve struct, group law
    secp_k1_toy.rs # downsized GLV-friendly curve
    generic.rs     # generic Weierstrass GF(p)
  util/
    batch_inv.rs   # Montgomery's batched inversion trick (shared)
    mp.rs          # multi-precision helpers if needed beyond crypto-bigint
  factor/
    rho.rs         # Floyd + Brent + Montgomery batched GCD
    cli.rs
  ecdlp/
    walk.rs        # r-adding walk, partition function
    dp.rs          # distinguished-point predicate + store
    coordinator.rs # multi-threaded driver
    negmap.rs      # negation map + fruitless-cycle escape
    glv.rs         # GLV endomorphism
    cli.rs
benches/
  field.rs         # FpNaive vs FpMonty
  factor.rs        # Floyd vs Brent vs Brent+batched-GCD
  ecdlp.rs         # baseline → DPs → negmap → GLV (composing speedups)
tests/
  field_kat.rs     # known-answer tests vs k256/p256
  factor_kat.rs    # known semiprimes
  ecdlp_kat.rs     # small known DLPs
```

### Field arithmetic: roll our own on `crypto-bigint`, two implementations behind a trait

`Fp` is a trait; `FpNaive` and `FpMonty` are both implementations. Pedagogical: the benchmark
literally shows the Montgomery-form speedup. Cost: doubles the field-arithmetic test surface (both
implementations must pass the same KATs against a reference).

We do not depend on `k256`/`p256`/`ark-ec` for curve arithmetic. Their APIs are built for
constant-time cryptographic use; they make negation-map and batched-inversion optimizations awkward
or impossible. We use `k256`/`p256` *as reference implementations in tests only*, to KAT-check our
field and curve operations.

### Curve scope: GF(p) only

- One generic Weierstrass curve over GF(p), ~60–80 bit prime, used for the baseline + negation map.
- One GLV-friendly curve (downsized secp256k1-style, with the order-3 endomorphism), used for the
  GLV phase.

Binary-field curves would add an entirely separate field-arithmetic implementation for the sake of
one extra optimization (order-6 Koblitz automorphism). Not worth it at this scope. Document the
exclusion in README.

### Parallelism: `std::thread` + `crossbeam` channels, in-process client/server

ECDLP rho with distinguished points is the textbook van Oorschot–Wiener architecture: N walker
threads submit DPs over a channel; one coordinator thread maintains the DP hash table and detects
collisions. `rayon` is the wrong shape (walks are unbounded; no join). `tokio` is overkill in-process.

The DP-submission channel is the seam where a distributed layer would cut. Leaving the seam clean
costs nothing now and preserves the option.

For factoring: trivially parallel via multi-c restarts. `rayon::par_iter` over a `Vec<c-value>`,
first thread to find a factor wins, others abort.

## Optimization inventory (the "all canonical optimizations" check-list)

### Factoring rho

| Optimization                  | Phase | Notes                                                       |
|-------------------------------|-------|-------------------------------------------------------------|
| Floyd's cycle detection       | 1     | Baseline; kept for benchmark comparison only                |
| Brent's cycle detection       | 1     | ~24% fewer group ops than Floyd                             |
| Polynomial choice `x² + c`    | 1     | Reject c ∈ {0, -2}                                          |
| Multi-c parallel restart      | 1     | rayon over c-values                                         |
| Montgomery batched GCD        | 1     | Accumulate ∏(xᵢ - xⱼ) mod N over batch of ~100, one GCD     |
| Pollard–Brent backup on `gcd=N` | 1   | Failure-mode recovery                                       |

### ECDLP rho

| Optimization                       | Phase | Notes                                                       |
|------------------------------------|-------|-------------------------------------------------------------|
| r-adding walk (Teske)              | 3     | r ≈ 20; partition by hash of x-coord                        |
| Brent's cycle detection            | 3     | Single-threaded baseline only; DPs replace it later         |
| Distinguished points (vOW)         | 4     | Predicate: low θ bits of x-coord = 0                        |
| Parallel collision search          | 4     | N walkers + 1 coordinator                                   |
| DP hash table + match-on-insert    | 4     | `HashMap<DpKey, (a, b, walk_id)>`                           |
| Negation map (±P equivalence)      | 5     | Canonical representative = lex-smaller of x(P), x(-P)       |
| Fruitless-cycle detection + escape | 5     | Bos–Kleinjung–Niederhagen–Schwabe deterministic escape      |
| Batched field inversion            | 6     | Montgomery's trick across walks within a thread             |
| GLV endomorphism (order-3)         | 7     | √2 speedup, composes with negation map                      |
| Affine coordinates                 | 6     | Made viable by batched inversion                            |

## Phases

Each phase delivers a runnable artifact and a benchmark.

### Phase 0 — Crate skeleton + `FpNaive`

- `cargo new --lib rho`
- Dependencies: `crypto-bigint` (Uint arithmetic), `rand`, `rand_chacha`, `crossbeam-channel`,
  `rayon`, `clap` (CLI), `criterion` (benches). Dev-deps: `k256`, `p256`, `proptest`.
- `Fp` trait: `add`, `sub`, `mul`, `square`, `inv`, `pow`, `from_u64`, `to_uint`, `zero`, `one`.
- `FpNaive` implementation: schoolbook mod-p arithmetic on `Uint<L>`.
- KATs: random-ops against `num-bigint` reference and against `k256::Scalar` for the secp256k1 prime.

**Artifact:** `cargo test` proves `FpNaive` is correct.

### Phase 1 — `FpMonty` + field benchmark

- Wrap `crypto_bigint::modular::MontyForm` into `FpMonty`.
- Same trait, same KATs. (Both implementations pass the same property tests.)
- `benches/field.rs`: mul/square/inv microbenchmarks comparing `FpNaive` vs `FpMonty`.

**Artifact:** benchmark numbers showing the Montgomery-form speedup. *This is the first pedagogical
"see the optimization work" moment.*

### Phase 2 — Factoring rho

- Floyd's, then Brent's, then Brent + Montgomery batched GCD, then multi-c restart.
- Test: factor a set of known semiprimes from 30 to 80 bits.
- `benches/factor.rs`: total group ops and wall time per variant on the same input.
- CLI: `rho-factor <N> [--threads N] [--batch-size B]`.

**Artifact:** working factorizer + benchmark showing Brent < Floyd and batched-GCD < Brent in wall
time.

### Phase 3 — Curve arithmetic

- `EcPoint`: affine and Jacobian projective representations; conversion both ways.
- Group law (add, double, scalar-mult via double-and-add).
- Two curves defined: one generic Weierstrass (random ~64-bit prime), one GLV-friendly
  secp256k1-toy (~64 bits with explicit order-3 endomorphism λ, β).
- KATs: scalar multiplication on secp256k1-toy cross-checked against `k256` for the full secp256k1
  curve (same group law, different parameters).

**Artifact:** `cargo test` proves group law is correct.

### Phase 4 — ECDLP rho baseline (single-threaded)

- r-adding walk (r=20). Partition function: `i = hash(x) mod r`.
- Brent's cycle detection (single-threaded — DPs only pay off with parallelism).
- Track (a, b) alongside each walk-point.
- Test: solve known DLPs at 25–35 bits, on both curves.
- CLI: `rho-dlog <curve> <P> <Q>` returns k such that Q = kP.

**Artifact:** working DLP solver at small scale.

### Phase 5 — Distinguished points + parallel collision search

- DP predicate parametrized by θ bits.
- `crossbeam-channel` for DP submission from walkers to coordinator.
- Coordinator: `HashMap<CompressedPoint, (a, b, walker_id)>`; on insert-collision, solve linear
  system for k, signal walkers to stop.
- Walker: continuous r-adding walk, emit DP on hit, restart from random (a₀, b₀) on dead walk.
- Test: solve 45–55 bit DLPs in seconds-to-minutes on commodity multicore.
- `benches/ecdlp.rs`: walks/second vs thread count (should scale near-linearly).

**Artifact:** parallel solver. *Second pedagogical moment: the speedup is N, not √N.*

### Phase 6 — Negation map + fruitless-cycle escape

- Canonical representative: `r(P) = P if x(P).lex_le(x(-P)) else -P`. Walk step composes with `r`.
- Adjust (a, b) tracking through `r`: if the step negates, flip the sign of the accumulated
  coefficients for the new walk-point.
- Fruitless-cycle detection: keep a tiny sliding window (16 steps), check for length-2 loops.
- Escape: deterministic perturbation per BKNS (jump to `2P` of the current point, recording the
  coefficient change).
- Verify total walk steps drop by ~√2 vs Phase 5 on the same DLP instance.

**Artifact:** measurable √2 speedup. *Third pedagogical moment: the negmap pays off — net of the
fruitless-cycle handling overhead.*

### Phase 7 — Batched field inversion + affine walks

- Within each walker thread, run a mini-batch of B walks in lock-step (B = 8 to 32).
- Each step needs one inversion per walk; Montgomery's trick computes B inversions with 1 inversion
  + 3(B−1) multiplications.
- Switch the walk from Jacobian to affine coordinates (now viable because inversions are amortized).
- Verify the inner-loop cycles/step drop.

**Artifact:** measurable batched-inversion speedup. Compose with negmap.

### Phase 8 — GLV endomorphism

- On the GLV-friendly curve, define `φ(P) = (β·x, y)`, satisfying `φ(P) = λP`.
- Extend equivalence classes from {P, -P} (Phase 6) to {P, -P, λP, -λP, λ²P, -λ²P} (order-3
  endomorphism + negation).
- Adjust (a, b) coefficient bookkeeping accordingly.
- Verify additional √3 speedup on the GLV curve (composing with negmap, total speedup is √6 vs
  Phase 5 baseline).

**Artifact:** measurable GLV speedup. *Fourth pedagogical moment: speedups compose multiplicatively
across optimization layers.*

### Phase 9 (optional) — Pedagogical writeup

- `docs/PEDAGOGY.md`: phase-by-phase narrative with the benchmark table showing the cumulative
  speedup as each optimization is added. *This is the actual deliverable at pedagogical scope —
  the code is the example, the doc is the lesson.*

## Risks and open questions

- **Negation map correctness** is notoriously easy to get subtly wrong. The fruitless-cycle escape
  is the specific failure mode that has cost real research teams real time. Plan: write a
  stress-test that runs Phase 6 against Phase 5's solver on the same instance ~100 times and verifies
  the answer matches. The negmap is wrong if it ever returns the wrong k, not just if it's slow.

- **`crypto-bigint` Montgomery API** may or may not expose batched operations cleanly. If it
  doesn't, Phase 7 may need a small custom Montgomery wrapper. Re-evaluate at the start of Phase 1.

- **CLI parameter parsing for curves** is awkward: curves have ~5 large integer parameters. Plan to
  define curves as `&'static` constants in the source and select by name (`--curve secp-toy`),
  rather than parse them from CLI.

- **DP threshold θ tuning** affects memory vs. expected-time-to-collision tradeoff. The textbook
  value is θ such that 2^θ ≈ √n / (worker-count · target-walks). At Phase 5, default to a value that
  gives ~1000 DPs at solution time; expose as `--theta` flag.

## Handoff

This plan is a handoff target for `@build` (Sonnet 4.6 / T1). Each phase is small enough to fit in
one focused build session. Recommend:

1. Start a fresh `@build` session per phase.
2. The session ends with: tests passing, benchmark recorded in `docs/BENCHMARKS.md`, brief phase
   retrospective appended to this PLAN.
3. Phases 0–4 are mechanical-enough for `@build` without further `@plan-deep` consultation. Phases
   5, 6, 8 (DPs, negmap, GLV) have real design decisions inside them; if those decisions arise,
   surface them and consider a short `@plan-deep` consult before continuing.
