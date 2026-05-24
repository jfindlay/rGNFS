# rho — Pollard rho: integer factorization and ECDLP

A single Rust crate implementing two Pollard rho variants with all canonical optimizations layered
in progressively:

- **Integer factorization rho** (Pollard 1975, Brent 1980, Montgomery 1987).
- **ECDLP rho** (Pollard 1978, Teske 1998, van Oorschot–Wiener 1999, plus negation map and GLV).

**Target audience:** pedagogical and research-grade. The goal is a clean, fast, *readable* CPU
implementation where each optimization phase produces a measurable speedup that can be verified
against the prior phase's benchmark. Not a record-attempt platform. Not constant-time. Not a
cryptographic library — these are *attacks on* cryptographic problems, run on toy parameters.

**Out of scope (deliberately):** binary-field (GF(2^m)) curves and the order-6 Koblitz
automorphism; GPU/FPGA backends; AVX-512 hand-tuning; distributed (multi-machine) deployment;
durable DP storage. The architecture leaves a clean seam at the DP-submission boundary so a
distributed layer could be added later without restructuring.

## Status

| Phase | Description                                        | Status        |
|-------|----------------------------------------------------|---------------|
| 0     | Crate skeleton + `FpNaive`                         | ✅ complete   |
| 1     | `FpMonty` + field benchmark                        | ✅ implemented; bench stub only |
| 2     | Factoring rho (Floyd/Brent/batched-GCD/multi-c)    | stub          |
| 3     | Curve arithmetic (EcPoint, group law, two curves)  | stub          |
| 4     | ECDLP baseline (r-adding walk, Brent cycle detect) | stub          |
| 5     | Distinguished points + parallel collision search   | stub          |
| 6     | Negation map + fruitless-cycle escape              | stub          |
| 7     | Batched field inversion + affine walks             | stub          |
| 8     | GLV endomorphism                                   | stub          |
| 9     | Pedagogical writeup (`docs/PEDAGOGY.md`)           | not started   |

## Build and test

```sh
# Requires stable Rust (rust-toolchain.toml pins to stable; no nightly)
cargo build
cargo test

cargo build --release
```

Tests currently exercise `FpNaive` (9 inline unit tests) and `FpMonty` (8 inline unit tests,
including cross-checks against `FpNaive`). Integration tests (`tests/field_kat.rs`,
`tests/factor_kat.rs`, `tests/ecdlp_kat.rs`) are planned but not yet written.

## Benchmarks

```sh
cargo bench                  # all bench targets
cargo bench --bench field    # FpNaive vs FpMonty mul/square/inv
cargo bench --bench factor   # Floyd vs Brent vs Brent+batched-GCD
cargo bench --bench ecdlp    # baseline → DPs → negmap → GLV
```

The bench bodies are stubs; `criterion` is not yet in `[dev-dependencies]` and will be added at
Phase 1. The bench profile uses `opt-level = 3, lto = "thin"`.

## CLIs

```sh
cargo run --bin rho-factor -- <N> [--threads N] [--batch-size B]
cargo run --bin rho-dlog   -- <curve> <P> <Q>
```

Both are stubs that print "not yet implemented" and exit 1. Curves will be selected by name
(`--curve secp-toy`) rather than parsed from CLI parameters.

## Crate structure

```
rho/
  src/
    lib.rs              # re-exports
    field/
      mod.rs            # Fp trait
      naive.rs          # FpNaive: schoolbook mod-p on Uint<4>  [implemented]
      monty.rs          # FpMonty: Montgomery form via DynResidue  [implemented]
    curve/
      mod.rs
      generic.rs        # generic Weierstrass y²=x³+ax+b over GF(p)  [stub, Phase 3]
      secp_k1_toy.rs    # downsized GLV-friendly secp256k1-style curve  [stub, Phase 3]
    factor/
      mod.rs
      rho.rs            # Floyd + Brent + Montgomery batched GCD  [stub, Phase 2]
      cli.rs            # rho-factor binary  [stub]
    ecdlp/
      mod.rs
      walk.rs           # r-adding walk, partition function  [stub, Phase 4]
      dp.rs             # distinguished-point predicate + store  [stub, Phase 5]
      coordinator.rs    # multi-threaded driver  [stub, Phase 5]
      negmap.rs         # negation map + BKNS escape  [stub, Phase 6]
      glv.rs            # GLV endomorphism  [stub, Phase 8]
      cli.rs            # rho-dlog binary  [stub]
    util/
      mod.rs
      batch_inv.rs      # Montgomery batched inversion  [stub, Phase 7]
      mp.rs             # multi-precision helpers  [stub, on-demand]
  benches/
    field.rs            # FpNaive vs FpMonty  [stub, Phase 1]
    factor.rs           # Floyd vs Brent vs Brent+batched-GCD  [stub, Phase 2]
    ecdlp.rs            # baseline → DPs → negmap → GLV  [stub, Phase 5+]
  tests/                # planned; directory not yet created
    field_kat.rs        # KATs vs k256/p256 reference
    factor_kat.rs       # known semiprimes
    ecdlp_kat.rs        # small known DLPs
```

## Dependencies

**Runtime:**

| Crate               | Role                                                         |
|---------------------|--------------------------------------------------------------|
| `crypto-bigint 0.5` | `Uint<4>` multi-precision integers; `DynResidue` for Montgomery form |
| `rand 0.8`          | RNG traits                                                   |
| `rand_chacha 0.3`   | Deterministic ChaCha RNG for reproducible walks              |
| `crossbeam-channel 0.5` | DP submission channel (walkers → coordinator)            |
| `rayon 1`           | Parallel multi-c restarts for factoring                      |
| `clap 4`            | CLI argument parsing                                         |

**Dev-only:**

| Crate        | Role                                                              |
|--------------|-------------------------------------------------------------------|
| `k256 0.13`  | Reference secp256k1 implementation for field/curve KATs           |
| `p256 0.13`  | Reference P-256 implementation for field/curve KATs               |
| `proptest 1` | Property-based testing                                            |

`k256`/`p256` are used only as reference implementations in tests, not for any production path.
Their APIs are built for constant-time cryptographic use, which makes negation-map and
batched-inversion optimizations awkward; that is why curve arithmetic is implemented from scratch.

## Architectural decisions

### One crate, two modules, no shared trait

The two algorithms share only cycle-finding control flow, and even that diverges: Brent's algorithm
for factoring; distinguished points for ECDLP. A `WalkStep` trait over both would abstract over the
wrong axis.

### Field arithmetic: `Fp` trait with two implementations

`Fp` is a trait; `FpNaive` and `FpMonty` are both implementations. The benchmark literally shows
the Montgomery-form speedup. The modulus is passed as a runtime parameter (`&Uint<4>`) rather than
baked into a type parameter — simpler generics at the cost of a small per-call overhead, which is
the point of the benchmark.

`Uint<4>` = 256-bit throughout. The toy curves use ~60–80 bit primes, so this is over-provisioned
but consistent.

### Parallelism

**Factoring:** trivially parallel via multi-c restarts. `rayon::par_iter` over a `Vec<c>`;
first thread to find a factor wins, others abort.

**ECDLP:** van Oorschot–Wiener architecture. N walker threads submit distinguished points over a
`crossbeam-channel`; one coordinator thread maintains the DP hash table and detects collisions.
`rayon` is the wrong shape (walks are unbounded; no join). `tokio` is overkill in-process. The
DP-submission channel is the seam where a distributed layer would cut.

### Curve scope: GF(p) only

One generic Weierstrass curve (~60–80 bit prime) for the baseline + negation-map phases. One
GLV-friendly curve (downsized secp256k1-style, explicit order-3 endomorphism λ, β) for the GLV
phase. Binary-field curves are out of scope (see above).

## Optimization inventory

### Factoring rho

| Optimization                    | Phase | Notes                                              |
|---------------------------------|-------|----------------------------------------------------|
| Floyd's cycle detection         | 2     | Baseline; kept for benchmark comparison only       |
| Brent's cycle detection         | 2     | ~24% fewer group ops than Floyd                    |
| Polynomial choice `x² + c`      | 2     | Reject c ∈ {0, −2}                                 |
| Multi-c parallel restart        | 2     | rayon over c-values                                |
| Montgomery batched GCD          | 2     | Accumulate ∏(xᵢ−xⱼ) mod N over batch ~100, one GCD |
| Pollard–Brent backup on `gcd=N` | 2     | Failure-mode recovery                              |

### ECDLP rho

| Optimization                        | Phase | Notes                                            |
|-------------------------------------|-------|--------------------------------------------------|
| r-adding walk (Teske)               | 4     | r ≈ 20; partition by hash of x-coord             |
| Brent's cycle detection             | 4     | Single-threaded baseline; DPs replace it later   |
| Distinguished points (vOW)          | 5     | Predicate: low θ bits of x-coord = 0             |
| Parallel collision search           | 5     | N walkers + 1 coordinator                        |
| DP hash table + match-on-insert     | 5     | `HashMap<CompressedPoint, (a, b, walker_id)>`    |
| Negation map (±P equivalence)       | 6     | Canonical rep = lex-smaller of x(P), x(−P)       |
| Fruitless-cycle detection + escape  | 6     | BKNS deterministic escape                        |
| Batched field inversion             | 7     | Montgomery's trick across walks within a thread  |
| Affine coordinates                  | 7     | Made viable by batched inversion                 |
| GLV endomorphism (order-3)          | 8     | √2 speedup; composes with negation map           |

## Known risks

**Negation map correctness.** Easy to get subtly wrong. The fruitless-cycle escape is the specific
failure mode. Mitigation: stress-test Phase 6 against Phase 5's solver on the same instance ~100
times; the negmap is wrong if it ever returns a wrong k, not just if it is slow.

**`crypto-bigint` Montgomery API.** May not expose batched operations cleanly. If it doesn't,
Phase 7 may need a small custom Montgomery wrapper. Re-evaluate at the start of Phase 7.

**DP threshold θ tuning.** Affects memory vs. expected-time-to-collision. Default to θ such that
`2^θ ≈ √n / (worker-count · target-walks)`, giving ~1000 DPs at solution time. Expose as
`--theta`.
