# rGNFS: integer factorization and ECDLP

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

For the full pedagogical narrative see [`docs/PEDAGOGY.md`](docs/PEDAGOGY.md).

When fully implemented, rGNFS is intended to be a completel treatment of discrete logarithm
solutions and strategies for integer factorization, number fields, and fields over elliptic curves,
in both classical and quantum form.  It is yet unknown whether this collection of focus areas is
coherent and intuitive and complete enough to be useful without straining for a total covering.  The
latter is usually futile, while the former, incomplete form is both more natural and accessible.

## Build and test

```sh
# Requires stable Rust (rust-toolchain.toml pins to stable; no nightly needed)
cargo build
cargo test        # 90 unit tests + 20 integration tests (KATs)
cargo build --release
```

## Benchmarks

```sh
cargo bench                  # all bench targets
cargo bench --bench field    # FpNaive vs FpMonty mul/square/inv
cargo bench --bench factor   # Floyd vs Brent vs Brent+batched-GCD
cargo bench --bench ecdlp    # baseline → DPs → negmap → batched-inv → GLV
```

The bench profile uses `opt-level = 3, lto = "thin"`.

## CLIs

**Factor an integer:**

```sh
cargo run --release --bin rho-factor -- <N>
cargo run --release --bin rho-factor -- <N> --threads 4 --batch-size 128
```

**Solve an ECDLP** (`Q = k·P`, returns `k`):

```sh
# Single-threaded Brent baseline
cargo run --release --bin rho-dlog -- --curve secp-toy --q <x,y>

# Parallel distinguished-point solver
cargo run --release --bin rho-dlog -- --curve secp-toy --q <x,y> --walkers 4 --theta 8

# Add negation map
cargo run --release --bin rho-dlog -- --curve secp-toy --q <x,y> --walkers 4 --negmap

# Add batched inversion (B=16 walks per thread)
cargo run --release --bin rho-dlog -- --curve secp-toy --q <x,y> --walkers 4 --batch-size 16

# Add GLV endomorphism (secp-toy only)
cargo run --release --bin rho-dlog -- --curve secp-toy --q <x,y> --walkers 4 --batch-size 16 --glv
```

Curves: `secp-toy` (63-bit GLV-friendly secp256k1-style) or `generic` (63-bit Weierstrass).
Points are given as `x,y` in decimal with no spaces.

## Crate structure

```
rho/
  src/
    lib.rs              # re-exports
    field/
      mod.rs            # Fp trait
      naive.rs          # FpNaive: schoolbook mod-p on Uint<4>
      monty.rs          # FpMonty: Montgomery form via DynResidue
    curve/
      mod.rs            # AffinePoint, JacobianPoint, Curve, group law
      generic.rs        # generic Weierstrass y²=x³+ax+b over GF(p)
      secp_k1_toy.rs    # downsized GLV-friendly secp256k1-style curve
      test_curves.rs    # small curves used in unit tests
    factor/
      mod.rs
      rho.rs            # Floyd + Brent + Montgomery batched GCD + multi-c
      cli.rs            # rho-factor binary
    ecdlp/
      mod.rs            # solve_brent, solve_dp, solve_dp_negmap, solve_dp_batch, solve_dp_glv
      walk.rs           # r-adding walk, AffineWalkState, BatchedWalker
      dp.rs             # distinguished-point predicate + DpRecord
      coordinator.rs    # DP hash table, collision detection
      negmap.rs         # canonical representative, FruitlessCycleDetector, BKNS escape
      glv.rs            # GLV endomorphism φ, glv_canonical (6-orbit)
      cli.rs            # rho-dlog binary
    util/
      mod.rs
      batch_inv.rs      # Montgomery's batched inversion trick
      mp.rs             # multi-precision helpers (reserved; not yet needed)
  benches/
    field.rs            # FpNaive vs FpMonty
    factor.rs           # Floyd vs Brent vs Brent+batched-GCD
    ecdlp.rs            # baseline → DPs → negmap → batched-inv → GLV
  tests/
    factor_kat.rs       # known semiprimes 15–2^64, all four factor variants
    ecdlp_kat.rs        # group-law KATs, k256 cross-check, ECDLP solver KATs
```

## Dependencies

**Runtime:**

| Crate                   | Role                                                                 |
|-------------------------|----------------------------------------------------------------------|
| `crypto-bigint 0.5`     | `Uint<4>` multi-precision integers; `DynResidue` for Montgomery form |
| `rand 0.8`              | RNG traits                                                           |
| `rand_chacha 0.3`       | Deterministic ChaCha RNG for reproducible walks                      |
| `crossbeam-channel 0.5` | DP submission channel (walkers → coordinator)                        |
| `rayon 1`               | Parallel multi-c restarts for factoring                              |
| `clap 4`                | CLI argument parsing                                                 |

**Dev-only:**

| Crate           | Role                                                                |
|-----------------|---------------------------------------------------------------------|
| `k256 0.13`     | Reference secp256k1 implementation for group-law cross-check KATs   |
| `p256 0.13`     | Reference P-256 implementation (available for future KAT extension) |
| `proptest 1`    | Property-based testing                                              |
| `criterion 0.5` | Benchmark harness                                                   |

`k256`/`p256` are used only as reference implementations in tests, not in any production path.
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

One generic Weierstrass curve (~63-bit prime) for the baseline and negation-map phases. One
GLV-friendly curve (downsized secp256k1-style, explicit order-3 endomorphism λ, β) for the GLV
phase. Binary-field curves are out of scope: they would require an entirely separate
field-arithmetic implementation for the sake of one extra optimization (the order-6 Koblitz
automorphism).

## Optimization inventory

### Factoring rho

| Optimization                    | Notes                                              |
|---------------------------------|----------------------------------------------------|
| Floyd's cycle detection         | Baseline; kept for benchmark comparison only       |
| Brent's cycle detection         | ~24% fewer group ops than Floyd                    |
| Polynomial choice `x² + c`      | Reject c ∈ {0, −2}                                 |
| Multi-c parallel restart        | `rayon` over c-values                              |
| Montgomery batched GCD          | Accumulate ∏(xᵢ−xⱼ) mod N over batch ~128, one GCD |
| Pollard–Brent backup on `gcd=N` | Failure-mode recovery                              |

### ECDLP rho

| Optimization                        | Notes                                                 |
|-------------------------------------|-------------------------------------------------------|
| r-adding walk (Teske)               | r = 20; partition by low bits of x-coord              |
| Brent's cycle detection             | Single-threaded baseline; DPs supersede it            |
| Distinguished points (vOW)          | Predicate: low θ bits of x-coord = 0                  |
| Parallel collision search           | N walkers + 1 coordinator; speedup scales as N        |
| DP hash table + match-on-insert     | `HashMap<x_low, DpRecord>`                            |
| Negation map (±P equivalence)       | Canonical rep = point with smaller y (≤ p/2)          |
| Fruitless-cycle detection + escape  | 16-entry sliding window; BKNS deterministic escape    |
| Batched field inversion             | Montgomery's trick: B inversions → 1 inv + 3(B−1) mul |
| Affine coordinates                  | Viable once inversions are amortised via batching     |
| GLV endomorphism (order-3)          | 6-orbit equivalence on secp-toy; ~√3 further speedup  |
