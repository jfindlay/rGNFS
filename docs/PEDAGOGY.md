# Pollard Rho for ECDLP: A Phase-by-Phase Guide

This document explains the optimization sequence implemented in this crate. It is aimed at a reader
who knows group theory and has seen the discrete logarithm problem before, but is new to the
specific algorithmic techniques used here. The code is the example; this document is the lesson.

---

## 1. Introduction

### The discrete logarithm problem

Let G be a cyclic group of prime order n, and let G be a generator. Given a target element Q, the
*discrete logarithm problem* (DLP) asks for the integer k ∈ [0, n) such that Q = k·G. On an
elliptic curve over a finite field, this is the *elliptic curve discrete logarithm problem* (ECDLP).

The hardness of the ECDLP is the security foundation of elliptic-curve cryptography. The best known
general-purpose algorithm for solving it runs in O(√n) group operations — not O(n), not O(log n).
That square-root barrier is why a 256-bit elliptic curve provides roughly 128-bit security: an
attacker needs ~2^128 operations, not ~2^256.

### Why Pollard rho gives O(√n) expected time

The key insight is the birthday paradox. If you sample ~√n random elements from a group of size n,
you expect a collision — two samples that land on the same element. Pollard's rho algorithm
(Pollard 1978) exploits this by generating a pseudorandom walk on the group and detecting when the
walk revisits a point. Because the walk is deterministic given its starting state, a revisit implies
a cycle, and the cycle structure encodes the discrete logarithm.

More precisely: the walk maintains the invariant W = a·G + b·Q for tracked scalars a, b ∈ ℤ/nℤ.
When two walk states W₁ = a₁·G + b₁·Q and W₂ = a₂·G + b₂·Q collide at the same group element,
we have (a₁ − a₂)·G = (b₂ − b₁)·Q = (b₂ − b₁)·k·G, so k = (a₁ − a₂)/(b₂ − b₁) mod n (when
b₂ ≠ b₁ mod n). The expected number of steps to collision is O(√n) by the birthday bound.

### What this codebase is

This crate implements two Pollard rho variants — integer factorization and ECDLP — with all
canonical optimizations, built up phase by phase. Each phase adds one optimization, delivers a
runnable artifact, and produces a measurable speedup. The goal is a clean, readable CPU
implementation that a researcher can extend or benchmark. It is not a constant-time cryptographic
library; these are *attacks on* cryptographic problems, run on toy parameters.

---

## 2. Phase-by-Phase Narrative

### Phase 0 — Crate skeleton and `FpNaive`

**What it adds:** the crate scaffolding and a schoolbook modular arithmetic implementation.

The `Fp` trait defines the field operations needed throughout: `add`, `sub`, `mul`, `square`,
`inv`, `pow`, `from_u64`, `to_uint`, `zero`, `one`. `FpNaive` implements this trait using
schoolbook arithmetic on `crypto_bigint::Uint<4>` — straightforward reduction after every
operation. This is correct but slow: every multiplication requires a full modular reduction.

**Code reference:** `src/field/naive.rs` (the `FpNaive` implementation).

**Speedup:** baseline. No optimization yet; this phase establishes correctness.

---

### Phase 1 — `FpMonty` and the Montgomery-form speedup

**What it adds:** Montgomery-form field arithmetic, the first measurable optimization.

The key insight is that modular reduction is expensive because it requires division. Montgomery's
representation (Montgomery 1985) avoids division by working in a transformed domain: instead of
storing x, store x·R mod p, where R = 2^256 (or another power of 2 chosen for the word size). In
this domain, multiplication of two Montgomery-form elements x·R and y·R gives x·y·R² mod p, which
can be reduced to (x·y·R) mod p using only shifts and additions — no division. The cost is a
one-time conversion into and out of Montgomery form per computation session.

`FpMonty` wraps `crypto_bigint::modular::MontyForm`, which handles the representation internally.
Both `FpNaive` and `FpMonty` implement the same `Fp` trait and pass the same known-answer tests.
The benchmark at `benches/field.rs` shows the speedup directly.

**Code reference:** `src/field/monty.rs` (the `FpMonty` implementation).

**Speedup:** ~2–4× for field multiplication over `FpNaive`, depending on the prime size. This
speedup propagates to every subsequent phase because all curve arithmetic is built on top of it.

---

### Phase 2 — Integer factorization rho

**What it adds:** Pollard rho for integer factorization, with Floyd, Brent, and batched-GCD
variants.

The factoring variant uses the pseudorandom map f(x) = (x² + c) mod N. Floyd's cycle detection
runs two pointers (tortoise at speed 1, hare at speed 2) and tests gcd(|x_tortoise − x_hare|, N)
at every step. Brent's improvement freezes the tortoise at the start of each power-of-2 window and
advances only the hare, reducing the number of f-evaluations by ~24% on average.

The batched-GCD trick (Montgomery 1987) amortises the GCD cost further. Instead of computing a GCD
at every step, accumulate the product ∏(xᵢ − y) over a batch of B steps, then take a single GCD
of that product against N. A non-trivial factor is almost always preserved in the product. The only
failure mode is gcd = N (the product is divisible by all prime factors simultaneously), which is
handled by falling back to step-by-step GCD on the offending batch.

**Code reference:** `src/factor/rho.rs` — `floyd`, `brent`, `brent_batched`, and `factor`.

**Speedup:** Brent gives ~24% fewer f-evaluations than Floyd. Batched GCD (batch size ~100) reduces
wall time by another 2–5× by replacing expensive GCD calls with cheap multiplications.

---

### Phase 3 — Curve arithmetic

**What it adds:** elliptic curve group law in affine and Jacobian projective coordinates.

An elliptic curve over GF(p) in short Weierstrass form is y² = x³ + ax + b. The group law (point
addition and doubling) is defined by explicit formulas. Affine coordinates are conceptually simple
but require a field inversion per addition — the dominant cost. Jacobian projective coordinates
represent a point (X : Y : Z) with affine coordinates (X/Z², Y/Z³), allowing addition and doubling
with only multiplications and squarings (no inversions), at the cost of a single inversion when
converting back to affine.

Two curves are defined: a generic Weierstrass curve over a ~64-bit prime (used for Phases 4–7), and
`secp_k1_toy` — a 63-bit prime-field curve with a = 0 and an explicit order-3 endomorphism (used
for Phase 8). The secp_k1_toy curve mirrors the structure of secp256k1 (y² = x³ + 7, j-invariant
0) but over a much smaller prime, making it suitable for pedagogical experiments.

**Code reference:** `src/curve/mod.rs` (group law), `src/curve/secp_k1_toy.rs` (GLV curve
parameters and derivation).

**Speedup:** baseline for ECDLP. The choice of Jacobian vs affine coordinates becomes important in
Phase 7.

---

### Phase 4 — ECDLP rho baseline: r-adding walk and Brent's cycle detection

**What it adds:** a working single-threaded ECDLP solver.

The naive pseudorandom walk on an elliptic curve — "add a random point at each step" — has poor
mixing properties. Teske (1998) showed that an *r-adding walk* mixes much better: precompute a
table of r random addends R[0], …, R[r−1], where R[i] = αᵢ·G + βᵢ·Q with random scalars αᵢ, βᵢ.
At each step, select the addend by i = x mod r (where x is the low word of the current point's
x-coordinate), then set W ← W + R[i], a ← a + αᵢ mod n, b ← b + βᵢ mod n. With r ≈ 20, the
walk behaves like a random function on the group, which is the assumption the birthday-bound
analysis requires.

Brent's cycle detection is applied on top of this walk. The tortoise is frozen at the start of each
power-of-2 window; the hare advances one step at a time and is compared to the frozen tortoise at
each step. After r comparisons without collision, the tortoise snaps forward to the hare's position
and the window doubles. This terminates when the hare catches the tortoise on the cycle, using ~24%
fewer walk steps than Floyd's algorithm.

When a collision is detected at walk states (a_t, b_t) and (a_h, b_h), the DLP is recovered as:

```
k = (a_t − a_h) / (b_h − b_t)  mod n
```

The degenerate case b_h = b_t mod n (probability ~1/n) triggers a retry with a fresh random walk
table and starting point.

**Code reference:** `src/ecdlp/walk.rs::AddendTable`, `src/ecdlp/walk.rs::WalkState::step`,
`src/ecdlp/mod.rs::solve_brent`.

**Speedup:** O(√n) expected walk steps — this is the theoretical baseline. All subsequent phases
reduce the constant factor.

---

### Phase 5 — Distinguished points and parallel collision search

**What it adds:** the van Oorschot–Wiener (vOW) parallel rho architecture.

Brent's cycle detection is inherently single-threaded: the tortoise and hare must stay in sync.
Distinguished points (van Oorschot and Wiener 1999) break this dependency. A point is
*distinguished* when its x-coordinate has at least θ low-order zero bits — a property that can be
checked independently by any walker. The expected number of steps between distinguished points is
2^θ.

The architecture is a client/server design: N walker threads each run an independent r-adding walk
and emit a `DpRecord` (carrying the current x-coordinate and the accumulated scalars a, b) whenever
the walk lands on a distinguished point. A single coordinator thread maintains a hash table keyed
on the x-coordinate. When two DPs with the same x-coordinate but different (a, b) pairs arrive,
the coordinator solves the linear system for k and signals all walkers to stop.

The key insight is that two walks that ever visit the same point will, from that point on, follow
identical trajectories and will therefore hit the same distinguished points. So a collision in the
DP table implies a collision in the underlying walks, which implies the DLP can be recovered.

The θ parameter trades memory against time: larger θ means fewer DPs stored (less memory) but more
steps per walker between DPs (more time before a collision is detected). The textbook choice is
θ such that 2^θ ≈ √n / (N · target-walks), giving ~1000 DPs at solution time.

**Code reference:** `src/ecdlp/dp.rs::is_distinguished`, `src/ecdlp/dp.rs::DpRecord`,
`src/ecdlp/coordinator.rs`, `src/ecdlp/mod.rs::solve_dp`.

**Speedup:** N× (linear in thread count). This is the second pedagogical moment: the speedup from
parallelism is linear, not √N. Each additional walker independently explores the group, and the
birthday bound applies to the combined pool of DPs. Doubling the number of walkers halves the
expected time to collision.

---

### Phase 6 — Negation map and fruitless-cycle escape

**What it adds:** a ~√2 reduction in expected walk length by collapsing {P, −P} pairs.

On an elliptic curve, P and −P share the same x-coordinate (negation flips only the y-coordinate).
The negation map exploits this: instead of treating P and −P as distinct walk states, collapse them
to a single canonical representative. The canonical element is the one with the numerically smaller
y-coordinate (equivalently, y ≤ p/2). After each walk step, apply the canonical map: if the
current point is not canonical, replace it with its negation and flip the signs of the accumulated
scalars (a ← n − a, b ← n − b, since −W = (n−a)·G + (n−b)·Q).

This halves the effective group size: the walk now explores only the ~n/2 canonical representatives
rather than all n group elements. By the birthday bound, halving the group size reduces the
expected number of steps to collision by √2.

The complication is *fruitless cycles*. When the walk oscillates between a point W and its negation
−W, the canonical map maps both to the same representative, so the canonical x-coordinate repeats
every two steps. The walk is trapped in a length-2 cycle and will never hit a distinguished point.
The BKNS escape (Bos, Kleinjung, Niederhagen, Schwabe) handles this deterministically: detect the
period-2 pattern in a sliding window of recent canonical x-coordinates, then perturb by doubling
the current point (W ← 2W, a ← 2a mod n, b ← 2b mod n) and re-canonicalising. The doubling
breaks the cycle because 2W is generically not in the same {W, −W} pair.

**Code reference:** `src/ecdlp/negmap.rs::canonical_rep`, `src/ecdlp/negmap.rs::negate_scalars`,
`src/ecdlp/negmap.rs::FruitlessCycleDetector`, `src/ecdlp/mod.rs::solve_dp_negmap`.

**Speedup:** ~√2 ≈ 1.41× reduction in expected walk steps vs Phase 5. The fruitless-cycle
overhead is small in practice (escapes are rare for a well-mixing walk), so the net speedup is
close to the theoretical √2.

---

### Phase 7 — Batched field inversion and affine coordinates

**What it adds:** 2–4× reduction in per-step cost by amortising field inversions across a batch of
walks.

In the Jacobian walk (Phase 4), each step requires one field inversion to convert the current point
to affine coordinates for the partition function. Field inversion is the most expensive field
operation — typically 50–100× the cost of a multiplication. This cost dominates the per-step
budget.

The key insight is that if B walks are advanced in lock-step, their B inversion denominators can be
computed together using Montgomery's batched inversion trick: compute the prefix products
p[0] = d[0], p[1] = d[0]·d[1], …, p[B−1] = d[0]·…·d[B−1]; invert the last prefix product (one
inversion); then recover each individual inverse in a backward pass using 3(B−1) multiplications.
Total cost: 1 inversion + 3(B−1) multiplications, versus B inversions individually.

This makes affine coordinates viable throughout the walk: instead of maintaining Jacobian
coordinates and converting to affine only for partitioning, keep the current point in affine form
at all times. The affine addition formula is:

```
λ = (y₂ − y₁) / (x₂ − x₁)
x₃ = λ² − x₁ − x₂
y₃ = λ·(x₁ − x₃) − y₁
```

The denominator (x₂ − x₁) is the quantity that requires inversion. Collecting B such denominators
and batch-inverting them reduces the inversion count from B to 1 per round of B steps.

**Code reference:** `src/util/batch_inv.rs::batch_invert`, `src/ecdlp/walk.rs::BatchedWalker`,
`src/ecdlp/walk.rs::BatchedWalker::step_all`, `src/ecdlp/mod.rs::solve_dp_batch`.

**Speedup:** 2–4× reduction in wall time per step, depending on batch size B and the relative cost
of inversion vs multiplication on the target hardware. Typical batch sizes are B = 8–32; beyond
B ≈ 32 the marginal gain diminishes as the inversion cost becomes negligible relative to the
3(B−1) multiplications.

---

### Phase 8 — GLV endomorphism and the 6-orbit canonical map

**What it adds:** a further ~√3 reduction in expected walk length by collapsing 6-element orbits.

The secp_k1_toy curve has an order-3 endomorphism φ: for any point P = (x, y), φ(P) = (β·x mod p,
y), where β is a cube root of unity mod p (β³ ≡ 1 mod p, β ≠ 1). This endomorphism satisfies
φ(P) = λ·P for all curve points P, where λ is a root of x² + x + 1 ≡ 0 mod n (the minimal
polynomial of a primitive cube root of unity in ℤ/nℤ). Because φ has order 3, the orbit of any
generic point P under the group generated by φ and negation is:

```
{P,  φ(P),  φ²(P),  −P,  −φ(P),  −φ²(P)}
```

This orbit has size 6. Collapsing it to a single canonical representative (the orbit member with
the smallest x-coordinate) reduces the effective group size by a factor of 6, dropping the expected
walk length by √6 vs the plain walk, or √3 vs the negation-map-only walk.

The scalar bookkeeping is more involved than for the negation map alone. If the current walk state
satisfies W = a·G + b·Q, then:

| Orbit member | Point   | Adjusted a    | Adjusted b    |
|--------------|---------|---------------|---------------|
| W            | W       | a             | b             |
| φ(W)         | φ(W)    | λa mod n      | λb mod n      |
| φ²(W)        | φ²(W)   | λ²a mod n     | λ²b mod n     |
| −W           | −W      | n − a         | n − b         |
| −φ(W)        | −φ(W)   | n − λa mod n  | n − λb mod n  |
| −φ²(W)       | −φ²(W)  | n − λ²a mod n | n − λ²b mod n |

After each walk step, all six orbit members are computed (the three x-coordinates require two
multiplications by β; the y-coordinates are shared with their negations), the minimum-x member is
selected, and the scalars are adjusted accordingly. The batched-inversion infrastructure from
Phase 7 is reused: each thread still owns a `BatchedWalker` with B walk states.

Fruitless cycles can still occur in the GLV walk (the canonical walk can oscillate between two
orbit members that share the same canonical representative). The BKNS doubling escape handles this
identically to Phase 6.

**Code reference:** `src/ecdlp/glv.rs::glv_phi`, `src/ecdlp/glv.rs::glv_canonical`,
`src/curve/secp_k1_toy.rs` (curve constants β, λ), `src/ecdlp/mod.rs::solve_dp_glv`.

**Speedup:** ~√3 ≈ 1.73× reduction in expected walk steps vs Phase 6 (negmap alone), or ~√6 ≈
2.45× vs Phase 5 (plain DP walk). The GLV endomorphism is essentially free to evaluate — one
multiplication by β per orbit member — so the theoretical speedup is nearly fully realised in
practice.

---

## 3. Cumulative Speedup Table

The table below summarises the expected speedup of each phase relative to the previous phase and
relative to the Phase 4 single-threaded baseline. Speedups marked with √ are reductions in
expected walk length (and therefore in expected time, assuming constant per-step cost); the
batched-inversion speedup is a reduction in per-step cost rather than walk length.

| Phase | Optimization                     | Speedup vs prev        | Cumulative vs Phase 4  |
|-------|----------------------------------|------------------------|------------------------|
| 4     | r-adding walk + Brent (baseline) | —                      | 1×                     |
| 5     | Distinguished points + N walkers | N× (linear parallelism)| N×                     |
| 6     | Negation map + BKNS escape       | ~√2 ≈ 1.41×            | ~N√2                   |
| 7     | Batched inversion + affine walks | ~2–4× (per-step cost)  | ~N√2 · B-factor        |
| 8     | GLV 6-orbit canonical map        | ~√3 vs Phase 6 alone   | ~N√6 · B-factor        |

Notes:

- The Phase 5 speedup is N (linear in thread count) because each additional walker independently
  contributes DPs to the collision pool. This is not a constant-factor improvement — it scales with
  hardware.
- Phases 6 and 8 give √2 and √3 reductions in expected walk *length*. These multiply together
  because they reduce the effective group size independently: Phase 6 halves it (factor of 2),
  Phase 8 reduces it by 3 (factor of 3), for a combined factor of 6 and a combined walk-length
  reduction of √6.
- Phase 7 reduces per-step *cost* rather than walk length. Its speedup factor (the "B-factor")
  depends on the ratio of inversion cost to multiplication cost on the target hardware, and on the
  batch size B. It composes multiplicatively with the walk-length reductions from Phases 6 and 8.

---

## 4. How Speedups Compose

The speedups from Phases 5, 6, 7, and 8 multiply rather than add because they act on independent
axes of the total computation cost.

The total expected work is (expected walk steps) × (cost per step). Phase 5 reduces expected walk
steps by a factor of N (more walkers means the birthday collision happens sooner in wall time).
Phases 6 and 8 each reduce expected walk steps by reducing the effective group size: Phase 6 by a
factor of 2 (negation map), Phase 8 by a factor of 3 (GLV orbit). Because these reductions apply
to the same quantity — expected walk steps — they multiply: the combined reduction is √(2 · 3) =
√6. Phase 7 reduces the cost per step by amortising inversions; this multiplies with the walk-step
reductions because it is a separate factor.

The practical ceiling is set by the birthday bound itself: no matter how many optimizations are
applied, the expected number of *collisions* needed is O(1), and each collision requires O(√(n/m))
walk steps where m is the effective group size after all orbit-collapsing maps. For the GLV curve
with negation map, m = n/6, so the expected steps per walker is O(√(n/6)). With N walkers, the
expected wall-time steps is O(√(n/6) / N). Beyond this, further speedup requires either a larger
orbit-collapsing map (not available for this curve family) or more parallelism.

---

## 5. Integer Factorization Rho (Phases 0–2)

The factoring side of the codebase (Phases 0–2) implements three variants of Pollard rho for
integers. The pseudorandom map is f(x) = (x² + c) mod N; a non-trivial factor is detected when
gcd(|xᵢ − xⱼ|, N) ∈ (1, N).

Floyd's algorithm (the baseline) runs two pointers at speeds 1 and 2 and tests a GCD at every
step. Brent's algorithm freezes the slow pointer at the start of each power-of-2 window, reducing
f-evaluations by ~24%. The batched-GCD variant (Montgomery 1987) accumulates the product of B
differences before taking a single GCD, replacing B expensive GCD calls with B−1 cheap
multiplications and one GCD. Multi-c parallelism runs independent instances with different c values
via `rayon::par_iter`, with the first successful thread winning.

The factoring and ECDLP sides share only the high-level structure (pseudorandom walk, cycle
detection, birthday-bound analysis). The specific optimizations diverge: Brent's cycle detection
is used in both, but distinguished points are specific to ECDLP (factoring uses Brent's algorithm
directly), and the negation map and GLV endomorphism have no factoring analogue.

**Code reference:** `src/factor/rho.rs` — `floyd`, `brent`, `brent_batched`, `factor`.

---

## 6. Running the Experiments Yourself

### Prerequisites

```
cargo build --release
```

### Known-answer tests

```
cargo test
```

This runs all KATs: field arithmetic (both `FpNaive` and `FpMonty` against reference values),
curve arithmetic (scalar multiplication cross-checked against reference points), factoring (known
semiprimes), and ECDLP (known discrete logarithms on the toy curves). All tests must pass before
trusting benchmark results.

### ECDLP benchmark suite

```
cargo bench --bench ecdlp
```

This runs the full ECDLP benchmark suite, measuring walk throughput (steps/second) and time-to-
solution for each solver variant: `solve_brent` (Phase 4), `solve_dp` (Phase 5), `solve_dp_negmap`
(Phase 6), `solve_dp_batch` (Phase 7), and `solve_dp_glv` (Phase 8). The benchmark output shows
the cumulative speedup as each optimization is added.

### CLI: stepping through the optimization layers

The `rho-dlog` binary exposes the solver variants via command-line flags. The following sequence
demonstrates the progression from the single-threaded baseline to the fully-optimized solver:

```sh
# Phase 4: single-threaded Brent baseline
rho-dlog --curve secp-toy --walkers 1

# Phase 5: 4 parallel walkers with distinguished points
rho-dlog --curve secp-toy --walkers 4

# Phase 7: 4 walkers with batched inversion (batch size 16)
rho-dlog --curve secp-toy --walkers 4 --batch-size 16

# Phase 8: 4 walkers, batch size 16, GLV endomorphism (secp-toy only)
rho-dlog --curve secp-toy --walkers 4 --batch-size 16 --glv
```

The `--theta` flag controls the distinguished-point threshold (default: auto-tuned to give ~1000
DPs at solution time). Increasing θ reduces memory usage at the cost of more steps per walker;
decreasing it increases memory usage but detects collisions sooner.

```sh
# Explicit theta: 1-in-256 DPs (theta=8)
rho-dlog --curve secp-toy --walkers 4 --batch-size 16 --glv --theta 8
```

---

## 7. Further Reading

1. **Pollard, J. M. (1978).** "Monte Carlo methods for index computation (mod p)." *Mathematics of
   Computation*, 32(143), 918–924. The original rho algorithm for discrete logarithms.

2. **Teske, E. (1998).** "Speeding up Pollard's rho method for computing discrete logarithms."
   *Algorithmic Number Theory Symposium (ANTS-III)*, LNCS 1423, 541–554. Introduces the r-adding
   walk and proves it mixes better than the naive random walk.

3. **van Oorschot, P. C., and Wiener, M. J. (1999).** "Parallel collision search with
   cryptanalytic applications." *Journal of Cryptology*, 12(1), 1–28. The definitive reference for
   distinguished points and the parallel rho architecture.

4. **Bos, J. W., Kleinjung, T., Niederhagen, R., and Schwabe, P. (2012).** "ECC2K-130 on Cell
   CPUs." *Progress in Cryptology — AFRICACRYPT 2010*, LNCS 6055, 225–242. Introduces the
   deterministic fruitless-cycle escape (the BKNS doubling perturbation) used in Phase 6.

5. **Gallant, R. P., Lambert, R. J., and Vanstone, S. A. (2001).** "Faster point multiplication on
   elliptic curves with efficient endomorphisms." *Advances in Cryptology — CRYPTO 2001*, LNCS
   2139, 190–200. The GLV decomposition; the endomorphism used in Phase 8 is the order-3 special
   case.

6. **Montgomery, P. L. (1987).** "Speeding the Pollard and elliptic curve methods of
   factorization." *Mathematics of Computation*, 48(177), 243–264. Introduces both the batched-GCD
   trick (Phase 2) and the batched-inversion trick (Phase 7).

7. **Bernstein, D. J., and Lange, T. (2007).** "Faster addition and doubling on elliptic curves."
   *Advances in Cryptology — ASIACRYPT 2007*, LNCS 4833, 29–50. A comprehensive reference for
   explicit addition formulas in various coordinate systems, including the affine formulas used in
   Phase 7.

8. **Brent, R. P. (1980).** "An improved Monte Carlo factorization algorithm." *BIT*, 20(2),
   176–184. Brent's cycle detection algorithm, used in both the factoring (Phase 2) and ECDLP
   (Phase 4) solvers.
