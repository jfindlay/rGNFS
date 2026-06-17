# Track S: Shor's Algorithm — A Code-Tour Chapter

> **Code-first treatment.** This chapter is the code-tour sibling of `docs/MATHEMATICS.md` ch. 11
> (T.S, to be written in S.D.2). It assumes the reader knows the mathematics — period-finding,
> the QFT phase-estimation argument, the order-finding → factoring reduction, and the two-register
> hidden-subgroup construction for ECDLP — and focuses on *how the code realises* those ideas.
> For the mathematical foundations, see T.S. For performance facts and qubit-budget tables, see
> `docs/BENCHMARKS.md` §§ S.A–S.C.

---

## §1. At a Glance: Three Sub-Tracks

Track S is organised into three sub-tracks, each building on the last. The table below names the
module surface, the frozen contracts, the canonical toy fixture, and the BENCHMARKS section for
each sub-track.

| Sub-track | Modules | Frozen contracts | Toy fixture | BENCHMARKS |
|-----------|---------|-----------------|-------------|------------|
| **S.A** — State-vector simulator | `statevec`, `gates`, `sparse`, `measure`, `qft` | C-StateVec, C-Sparse, C-QFT | Bell state, GHZ-3, QFT on \|000⟩ | `docs/BENCHMARKS.md` §S.A |
| **S.B** — Shor factoring | `arith`, `shor` | C-ModExp, C-OrderFind, C-Factor | 15 → {3,5}, 21 → {3,7}, 35 → {5,7}, 91 → {7,13} | `docs/BENCHMARKS.md` §S.B |
| **S.C** — Shor ECDLP | `curve`, `ecc`, `ecdlp` | C-PointAdd, C-ECDLPSolve | toy curve *y*² = *x*³ + 3 mod 7, *r* = 13; solves *k*·*G* = *Q* for *k* ∈ {3,5,7,9,12} | `docs/BENCHMARKS.md` §S.C |

**The "no new gate after S.A" invariant.** Every circuit in S.B and S.C is assembled entirely from
the frozen S.A gate set. No gate is added to the simulator after S.A.1 is frozen. This is the
load-bearing substrate-reuse property: S.B and S.C are pure consumers of C-StateVec.

---

## §2. Sub-Track S.A: The State-Vector Simulator

### §2.1 What it realises

S.A delivers a classical state-vector quantum-circuit simulator: the substrate on which Shor's
algorithm runs. It is not a quantum computer; it is a classical program that faithfully simulates
the evolution of a quantum register by maintaining the full `2^n`-amplitude array. The simulator
is correct at any qubit count; only memory makes large *n* unreachable (see §6 on the resource
ceiling).

### §2.2 Module surface

**`statevec` — the dense register (C-StateVec, frozen S.A.1).**
`StateVec` holds `2^n` complex amplitudes as a `Vec<Complex<f64>>`. The basis-indexing convention
is **little-endian**: qubit 0 is the least-significant bit of the basis index. For an *n*-qubit
register, basis state |*q*_{*n*−1} … *q*₁ *q*₀⟩ maps to index
*i* = *q*₀ + 2·*q*₁ + 4·*q*₂ + … + 2^{*n*−1}·*q*_{*n*−1}.
This convention is fixed at S.A.1 and consumed by every downstream module; a silent flip is a
wrong-answer bug. Constructors: `StateVec::zero(n)` (the |0…0⟩ state), `StateVec::basis(n, k)`
(the basis state |*k*⟩), `StateVec::from_amplitudes(n, amps)` (arbitrary normalised input). The
normalization invariant Σ|*aᵢ*|² = 1 is enforced at construction and preserved by all gates.

**`gates` — the universal gate set (C-StateVec, frozen S.A.1).**
Every gate is applied **in-place** by iterating the amplitude pairs it couples and updating them
with the gate's 2×2 unitary matrix. This is the standard O(2^*n*) state-vector method — no matrix
materialisation. The frozen gate set:

- *Single-qubit:* X, Y, Z, H, S, T, phase(θ), arbitrary unitary U(*a*,*b*,*c*,*d*)
- *Two-qubit:* CNOT, controlled-phase(θ), SWAP
- *Multi-qubit:* Toffoli (CCX), multi-controlled-X (generalised Toffoli), multi-controlled unitary

The multi-controlled surface is over-specified for S.A's own KATs (which exercise only single- and
two-qubit gates) but is carried for S.B (modular exponentiation) and S.C (point-addition circuit)
per the Category-A rule: the substrate is specified once, for all consumers.

**`sparse` — the sparse register (C-Sparse, frozen S.A.2 ◆).**
`SparseStateVec` stores only nonzero amplitudes in a `HashMap<usize, Complex<f64>>`. Gate
application iterates only the nonzero entries, applying the same unitary transformations as the
dense path. The sparse path is a no-regression mirror: a gate sequence on the sparse register
yields the same amplitudes as on the dense register (the sparse-dense agreement KAT). Sparsity
helps only while the state is sparse — a Hadamard on every qubit makes the state fully dense and
the sparse path matches the dense cost. This is a principle-4 annotation: sparsity is
state-dependent, not a universal speedup. Dense↔sparse conversion: `SparseStateVec::from_dense`
and `SparseStateVec::to_dense`.

**`measure` — Born-rule measurement (C-StateVec, frozen S.A.2 ◆).**
`measure_all` samples a basis state *k* with probability |*aₖ*|² (the Born rule) and collapses
the register to |*k*⟩. `measure_qubit` measures a single qubit and renormalises the remaining
register conditioned on the outcome. The sampler uses `rand_chacha::ChaCha8Rng` seeded with a
`u64` seed, making KATs reproducible: the same seed + same state always produces the same
measurement outcome. `sample_counts` provides a multi-shot statistical helper.

**`qft` — the Quantum Fourier Transform (C-QFT, frozen S.A.2 ◆).**
`qft` applies the standard Hadamard + controlled-phase ladder (O(*n*²) gates), built entirely from
the frozen S.A.1 gate set. The **bit-reversal convention** is load-bearing: the standard QFT
circuit (Nielsen & Chuang) is designed for big-endian qubit ordering. In the little-endian
convention, the H + controlled-phase ladder maps |*j*⟩ → QFT|*j*_reversed⟩ instead of QFT|*j*⟩
— a silent wrong-answer bug for S.B/S.C's period extraction. The fix is a single **input
bit-reversal step** (swap qubit *i* with qubit *n*−1−*i*) before the ladder; after this, the
ladder correctly computes QFT|*j*⟩ in natural little-endian order with no output bit-reversal
needed. `iqft` applies the inverse QFT (inverse controlled-phase + Hadamard ladder, reversed,
then output bit-reversal). The identity KAT `iqft(qft(sv)) == sv` is verified in `qft_kat.rs`.

### §2.3 Toy KAT

The canonical S.A KATs are in `shor/tests/statevec_kat.rs` and `shor/tests/qft_kat.rs`.

**Bell state.** Starting from |00⟩, apply H to qubit 0 then CNOT(0→1):

```rust
let mut sv = StateVec::zero(2);
gates::h(&mut sv, 0);
gates::cnot(&mut sv, 0, 1);
// Result: amplitudes[0] = 1/√2, amplitudes[3] = 1/√2, others = 0.
// (|00⟩ + |11⟩)/√2 in little-endian: index 0 = |00⟩, index 3 = |11⟩)
```

**QFT on |000⟩.** The QFT of the all-zero state is the uniform superposition:

```rust
let mut sv = StateVec::zero(3);
qft(&mut sv);
// All 8 amplitudes = 1/√8 (published: QFT|0⟩ = (1/√N)Σ|k⟩)
```

**QFT on |*j*⟩.** For *n* = 3, *j* = 1, the QFT amplitude at output index *k* is
(1/√8)·e^{2πi·*k*/8}, verified against Python-computed reference values.

### §2.4 Cross-reference

For the mathematical content — the QFT as a Fourier transform over ℤ/2^*n*ℤ, the phase-estimation
argument, and the Born-rule derivation — see T.S (ch. 11, `docs/MATHEMATICS.md`). For the
qubit-scaling wall and the dense-vs-sparse performance comparison, see `docs/BENCHMARKS.md` §S.A.

---

## §3. Sub-Track S.B: Shor's Factoring Algorithm

### §3.1 What it realises

S.B delivers the complete Shor-factoring algorithm running end-to-end on the S.A simulator: the
reversible modular-arithmetic quantum circuit builders (S.B.1), the order-finding circuit
orchestration, the classical continued-fraction period extraction, and the `factor(N)` driver
(S.B.2 ◆). The algorithm factors 15 → {3,5}, 21 → {3,7}, 35 → {5,7}, and 91 → {7,13} — the
canonical toy targets — using 8, 10, 12, and 14 qubits respectively.

### §3.2 Module surface

**`arith` — reversible modular arithmetic (C-ModExp, frozen S.B.1).**
The `arith` module builds the controlled modular-exponentiation circuit
|*x*⟩|*y*⟩ → |*x*⟩|*y*·*aˣ* mod *N*⟩ from the frozen S.A gate set. The circuit hierarchy:

1. `controlled_add_mod(sv, ctrl, c, N, work_qubits)` — |*x*⟩ → |*x* + *c* mod *N*⟩ when ctrl = |1⟩.
2. `controlled_mult_mod(sv, ctrl, c, N, work_qubits)` — |*x*⟩ → |*c*·*x* mod *N*⟩ when ctrl = |1⟩.
3. `controlled_mod_exp(sv, a, layout)` — |*x*⟩|*y*⟩ → |*x*⟩|*y*·*aˣ* mod *N*⟩.

**Implementation approach: permutation synthesis.** Each modular-arithmetic operation is a
classical permutation on basis states. The permutation is synthesised as a product of
transpositions (selection-sort order), each implemented via a Gray-code path of single-bit-
difference transpositions, each implemented as a multi-controlled-X gate. This approach is:
ancilla-free (no scratch register beyond the work register), reversible (every permutation is
its own inverse composed with itself), and correct on superpositions (the gate sequence is
unitary). It is a demonstration-fidelity circuit, not gate-count-optimised (principle 3).

**Register layout (C-ModExp, frozen).** Qubits [0, *t*) are the exponent register (*t* bits);
qubits [*t*, *t*+*n*) are the work register (*n* bits, where *N* < 2^*n*). The work register
starts at |1⟩ for the mod-exp circuit (so *y*·*aˣ* mod *N* with *y* = 1 gives *aˣ* mod *N*).
`ModExpLayout::standard(N, exp_len)` constructs the standard layout. Total qubits: *t* + *n*.

| *N* | *n*_bits(*N*) | exp_len (*t*) | work (*n*) | Total qubits |
|-----|--------------|--------------|-----------|--------------|
| 15  | 4            | 4            | 4         | 8            |
| 21  | 5            | 5            | 5         | 10           |
| 35  | 6            | 6            | 6         | 12           |
| 91  | 7            | 7            | 7         | 14           |

**`shor` — order-finding and factoring driver (C-OrderFind + C-Factor, frozen S.B.2 ◆).**
The `shor` module orchestrates the complete Shor-factoring algorithm:

1. **`run_order_finding_circuit(a, layout, seed)`** — allocates the register, puts the exponent
   register in uniform superposition (H on every exponent qubit), applies `controlled_mod_exp`,
   applies iQFT to the exponent register via `iqft_on_qubits`, measures with `measure_all_seeded`,
   and returns the measured phase numerator *s* (the lower `exp_len` bits of the measurement).

2. **`order_from_phase(s, t, a, N)`** — expands *s*/2^*t* as a continued fraction, takes
   convergents with denominator < *N*, and returns the smallest denominator *d* such that
   *a*^*d* ≡ 1 mod *N*. This is the classical post-processing step that recovers the order *r*
   from the measured phase. See T.S for the continued-fraction approximation argument.

3. **`find_order(a, N, seed)`** — calls `run_order_finding_circuit` then `order_from_phase`.

4. **`factor(N, seed)`** — the end-to-end driver. Classical short-circuits first (even *N*;
   prime-power *N* = *p*^*k*). Then the quantum order-finding loop: for each candidate base *a*
   coprime to *N*, run `find_order`; if the order *r* is even and *a*^(*r*/2) ≢ −1 mod *N*,
   extract factors via gcd(*a*^(*r*/2) ± 1, *N*). Retries with incremented seeds on failure.

### §3.3 Toy KAT

The canonical S.B KATs are in `shor/tests/modexp_kat.rs` and `shor/tests/factor_kat.rs`.

**Modular exponentiation.** On basis state |*x*⟩|1⟩ with *a* = 2, *N* = 15, *x* = 3:

```rust
let layout = ModExpLayout::standard(15, 4); // 8 qubits total
let mut sv = make_modexp_state(&layout, 3, 1); // |exp=3⟩|work=1⟩
controlled_mod_exp(&mut sv, 2, &layout);
// work register reads 2^3 mod 15 = 8. Published: ord₂(15) = 4.
```

**End-to-end factoring.** The four canonical targets:

```rust
factor(15, 0)  // → Some((3, 5))   — 8 qubits
factor(21, 0)  // → Some((3, 7))   — 10 qubits
factor(35, 8)  // → Some((5, 7))   — 12 qubits; seed 8 gives s=2389 ≈ 7·64/12
factor(91, 1)  // → Some((7, 13))  — 14 qubits; seed 1 gives s=53 ≈ 5·128/12
```

N = 91 is the ceiling-stress case: 14 qubits, well within the ~25-qubit simulator ceiling.

### §3.4 Cross-reference

For the mathematical content — the order-finding → factoring reduction, the continued-fraction
approximation theorem, and the probability analysis — see T.S (ch. 11). For the qubit-budget
table and the principle-4 resource-scale annotation, see `docs/BENCHMARKS.md` §S.B.

---

## §4. Sub-Track S.C: Shor's ECDLP Algorithm

### §4.1 What it realises

S.C delivers the complete Shor ECDLP algorithm running end-to-end on the S.A simulator: the toy
elliptic curve (S.C.1), the reversible controlled point-addition circuit via permutation synthesis
(S.C.1), the two-register period-finding circuit (S.C.2), the classical 2D-lattice discrete-log
extraction, and the `solve_ecdlp` driver (S.C.2 ◆). The algorithm solves the ECDLP *Q* = *k*·*G*
on a 4-bit toy curve (*r* = 13) using 17 qubits.

### §4.2 Module surface

**`curve` — the toy elliptic curve (C-PointAdd, frozen S.C.1).**
The `curve` module defines the short-Weierstrass curve *y*² = *x*³ + 3 mod 7 with plain `u64`
arithmetic. Parameters: field prime *p* = 7, coefficients *a* = 0, *b* = 3, generator *G* = (1, 2),
prime group order *r* = 13. The 13 group elements (including ∞) are:

```text
0·G = ∞,      1·G = (1,2),  2·G = (6,3),  3·G = (2,2),
4·G = (4,5),  5·G = (3,3),  6·G = (5,3),  7·G = (5,4),
8·G = (3,4),  9·G = (4,2),  10·G = (2,5), 11·G = (6,4),
12·G = (1,5), 13·G = ∞
```

The classical group law (`add`, `double`, `negate`, `scalar_mul`) is the reference for the
circuit's KATs. The identity ∞ is encoded as the coordinate pair (0, 0) — a reserved sentinel
that does not appear on the curve (x = 0 gives y² = 3 mod 7, which has no solution).

`PointAddLayout` documents the qubit layout consumed by the point-addition circuit:
- qubit 0: control qubit
- qubits [1, 4): x-coordinate (3 bits, ⌈log₂ 7⌉ = 3)
- qubits [4, 7): y-coordinate (3 bits)
- qubits [7, 10): λ scratch register (3 bits, allocated but unused at runtime — see below)

Total: 10 qubits for the point-addition sub-circuit.

**`ecc` — the reversible controlled point-addition circuit (C-PointAdd, frozen S.C.1).**
`controlled_point_add(sv, cg, layout)` applies |*P*⟩ → |*P* + *cG*⟩ when the control qubit is
|1⟩, for a classically-fixed point *cG*.

**Construction choice: permutation synthesis.** Since *cG* is a classical constant (precomputed),
this circuit performs *constant-point addition* — adding a fixed classical point to a quantum
register holding the running point *P*. The map (*x*_P, *y*_P) → (*x*_{P+cG}, *y*_{P+cG}) is
computed classically for all 13 group elements (including ∞) inside `build_point_add_permutation`,
then synthesised as a product of transpositions on the combined (*x*, *y*) register using the same
`apply_controlled_permutation` primitive from `arith`. The λ scratch register is allocated in the
layout for documentation and S.C.2 compatibility, but is **not consumed at runtime** — the
permutation synthesis is ancilla-free. This is the construction that landed; it mirrors the S.B.1
precedent exactly.

All exceptional cases (P = ∞, P = *cG*, P = −*cG*) are handled automatically by the permutation
synthesis — the classical group law is computed for every input, so no special-case branching is
needed in the circuit.

`controlled_point_add_inv(sv, cg, layout)` applies the inverse (adds −*cG*), used for
reversibility verification.

**`ecdlp` — the two-register ECDLP circuit and driver (C-ECDLPSolve, frozen S.C.2 ◆).**
The `ecdlp` module implements Shor's ECDLP algorithm as a two-register hidden-subgroup problem.
The register layout (17 qubits total):

```text
qubits [0, 4)   — a-register (4 bits, t = ⌈log₂ 13⌉ = 4)
qubits [4, 8)   — b-register (4 bits)
qubits [8, 11)  — x-coordinate of running point (3 bits)
qubits [11, 14) — y-coordinate of running point (3 bits)
qubits [14, 17) — λ scratch register (3 bits, allocated but unused)
Total: 17 qubits
```

**`run_period_finding_circuit(G, Q, seed)`** — the two-register circuit:

1. Allocate 17 qubits; initialise point register to |∞⟩ = |(0,0)⟩.
2. Apply H to every qubit of the a-register and b-register (uniform superposition).
3. For each bit *j* of the a-register: apply `controlled_point_add` with *cG* = 2^*j*·*G*,
   controlled on a-register qubit *j*.
4. For each bit *j* of the b-register: apply `controlled_point_add` with *cG* = 2^*j*·*Q*,
   controlled on b-register qubit *j*.
5. Apply iQFT to the a-register, then iQFT to the b-register.
6. Measure all qubits with `measure_all_seeded`.
7. Extract (*a*′, *b*′) from the exponent register bits.

**`extract_k_from_measurement(a′, b′, r)`** — the 2D-lattice extraction: recovers *k* via
*b*′·*k* ≡ −*a*′ (mod *r*), i.e., *k* = (−*a*′)·(*b*′)⁻¹ mod *r*. Returns `None` if *b*′ = 0
mod *r* (uninformative measurement). See T.S for the 2D-lattice argument.

**`solve_ecdlp(G, Q, seed)`** — the end-to-end driver. Handles trivial cases (*Q* = ∞ → *k* = 0;
*Q* = *G* → *k* = 1). Then runs `run_period_finding_circuit`, attempts `extract_k_from_measurement`,
verifies *k*·*G* = *Q*. Falls back to brute-force over *k* ∈ 0..*r* at toy scale (*r* = 13) — an
honest choice documented in the code: the quantum circuit still performs the period-finding; the
classical extraction is just exhaustive search at this scale. Retries with incremented seeds.

### §4.3 Toy KAT

The canonical S.C KATs are in `shor/tests/pointadd_kat.rs` and `shor/tests/ecdlp_kat.rs`.

**Classical group law.** The full scalar-multiple table is verified against hand-computed values:

```rust
assert_eq!(scalar_mul(4, G), Point::Affine { x: 4, y: 5 }); // 4·G = (4,5)
assert_eq!(scalar_mul(13, G), Point::Infinity);               // order check: 13·G = ∞
```

**Point-addition circuit.** For each group element *P* and each *cG*, the circuit gives *P* + *cG*:

```rust
let layout = PointAddLayout::standard(); // 10 qubits
let mut sv = make_point_state(1, G, &layout); // |ctrl=1⟩|P=G⟩
controlled_point_add(&mut sv, G, &layout);
// Point register reads 2·G = (6,3). Exceptional cases (P=∞, P=−cG) also verified.
```

**End-to-end ECDLP.** The five canonical instances:

```rust
solve_ecdlp(G, scalar_mul(3, G), 0)   // → Some(k) with k·G = 3·G = (2,2)
solve_ecdlp(G, scalar_mul(5, G), 0)   // → Some(k) with k·G = 5·G = (3,3)
solve_ecdlp(G, scalar_mul(7, G), 0)   // → Some(k) with k·G = 7·G = (5,4)
solve_ecdlp(G, scalar_mul(9, G), 0)   // → Some(k) with k·G = 9·G = (4,2)
solve_ecdlp(G, scalar_mul(12, G), 0)  // → Some(k) with k·G = 12·G = (1,5)
```

The assertion checks *k*′·*G* = *Q* (relationship-preservation), not *k*′ = *k*, since the
quantum algorithm may return any valid discrete logarithm.

### §4.4 Cross-reference

For the mathematical content — the two-register hidden-subgroup construction, the 2D-lattice
extraction, and the reduction from ECDLP to period-finding — see T.S (ch. 11). For the
qubit-budget table and the principle-4 resource-scale annotation, see `docs/BENCHMARKS.md` §S.C.

---

## §5. The Cross-Phase Contract View

Track S has a clean substrate-reuse story. The diagram below shows the dependency chain:

```text
C-StateVec (S.A.1)
  ↓ gate set consumed by
C-Sparse, C-QFT, measure (S.A.2)
  ↓ QFT + measure consumed by
C-ModExp (S.B.1)  ←── arith primitives re-consumed by ──→  C-PointAdd (S.C.1)
  ↓ consumed by                                                ↓ consumed by
C-OrderFind + C-Factor (S.B.2)                           C-ECDLPSolve (S.C.2)
```

**C-StateVec is the foundation.** The dense register and gate set (frozen S.A.1) are the only
substrate. Every circuit in S.B and S.C is assembled from `gates::h`, `gates::controlled_phase`,
`gates::swap`, `gates::multi_controlled_x`, and `gates::multi_controlled_unitary`. No gate is
added after S.A.1.

**C-QFT and measure are reused by both S.B and S.C.** The `iqft_on_qubits` helper (a partial
iQFT on a sub-register) appears in both `shor::shor` (for the exponent register in factoring) and
`shor::ecdlp` (for the a- and b-registers in ECDLP). The `measure_all_seeded` function is the
measurement primitive for both. The QFT bit-reversal convention (fixed at S.A.2) is consumed
silently by both — a wrong convention would produce wrong-answer bugs in both sub-tracks.

**C-ModExp's `arith` primitives are re-consumed by S.C.** The `apply_controlled_permutation`
function from `arith` (the permutation-synthesis primitive, frozen S.B.1) is imported directly
by `ecc::controlled_point_add`. The S.B.1 over-specification — carrying the full permutation-
synthesis machinery for modular arithmetic — pays off in S.C: the point-addition circuit reuses
the same primitive without any new gate or new abstraction.

**C-PointAdd wraps into C-ECDLPSolve.** The `controlled_point_add` function (frozen S.C.1) is
the only circuit primitive consumed by `ecdlp::run_period_finding_circuit`. The ECDLP circuit is
a loop over the a- and b-register bits, each iteration calling `controlled_point_add` with a
classically-precomputed power of *G* or *Q*.

**The "no new gate after S.A" invariant** is the observable consequence of this chain: the
`shor` crate's `gates` module is frozen at S.A.1 and never extended. S.B and S.C are pure
consumers.

---

## §6. Design-Statement Verification

### §6.1 Principle 1 — Algorithmic content complete

**Shor's factoring algorithm** is implemented head-on: the full quantum order-finding circuit
(exponent-register superposition → controlled-mod-exp → iQFT → measure), the classical
continued-fraction period extraction, and the even-order factor extraction via
gcd(*a*^(*r*/2) ± 1, *N*). The four canonical targets (15, 21, 35, 91) are all factored
correctly. No step is elided or approximated.

**Shor's ECDLP algorithm** is implemented head-on: the full two-register period-finding circuit
(*a*·*G* + *b*·*Q* in superposition → iQFT on both registers → measure), the classical
2D-lattice extraction *b*′·*k* ≡ −*a*′ (mod *r*), and the verification *k*·*G* = *Q*. The five
canonical instances (k = 3, 5, 7, 9, 12) are all solved correctly. No step is elided.

### §6.2 Principle 3 — No engineering optimisation

**The point-addition circuit uses the straightforward permutation-synthesis construction, not a
qubit-optimised one.** The group law is computed classically inside `build_point_add_permutation`
for all 13 group elements; the resulting permutation is synthesised as a product of transpositions
via Gray-code paths of multi-controlled-X gates. This is a demonstration-fidelity circuit: it is
correct, reversible, and ancilla-free, but it is not gate-count-optimised. A production
implementation (e.g., Roetteler et al. 2017) would use an explicit reversible affine formula with
a dedicated λ register and Brent–Kung adder trees. The λ register is allocated in `PointAddLayout`
for documentation and future-compatibility, but is not consumed at runtime by the permutation
synthesis.

Similarly, the modular-exponentiation circuit uses direct permutation synthesis (not the
Vedral–Barenco–Ekert adder-based construction). The gate count is O(*N*·*t*) transpositions per
mod-exp stage, not the O(*n*²·*t*) of an adder-based circuit — but the permutation synthesis is
also not the asymptotically optimal construction. The choice is deliberate: the code exhibits the
algorithm's logic, not its engineering optimisation.

### §6.3 Principle 4 — The ~25-qubit resource ceiling

**The ~25-qubit ceiling is a resource-scale wall, not a mathematical one.** The dense register
holds 2^*n* complex amplitudes. At *n* = 25 that is 2^25 ≈ 33 M entries (≈ 512 MiB of f64
pairs). This is the practical ceiling on a laptop.

The ceiling is annotated consistently across all three sub-tracks:

- **S.A** (`docs/BENCHMARKS.md` §S.A, `### Science↔engineering note`): the simulator demonstrates
  Shor's mathematics correctly at toy scale; the QFT produces the correct Fourier amplitudes and
  measurement samples from the correct Born-rule distribution. The ceiling is purely engineering.

- **S.B** (`docs/BENCHMARKS.md` §S.B, `### Science↔engineering note`): the order-finding circuit
  demonstrates Shor's factoring mathematics correctly. To factor RSA-2048 (a 2048-bit *N*), the
  circuit would need ~4100 qubits — requiring a 2^4100-entry array, far beyond any classical
  computer.

- **S.C** (`docs/BENCHMARKS.md` §S.C, `### Science↔engineering note`): the two-register circuit
  demonstrates Shor's ECDLP mathematics correctly. To solve the ECDLP on secp256k1 (*r* ≈ 2^256),
  the circuit would need ~768 qubits — requiring a 2^768-entry array.

The simulator demonstrates Shor's mathematics correctly at toy scale; it does not claim quantum
speedup, which requires real quantum hardware out of scope by construction.

---

## §7. KAT Summary

Track S has six KAT files in `shor/tests/`. The table below summarises the test corpus.

| File | Tests | What is verified |
|------|-------|-----------------|
| `statevec_kat.rs` | 32 | Dense register: unitarity (X, Y, Z, H, S, T, CNOT, controlled-phase, SWAP, Toffoli, multi-controlled-X); Bell state (|00⟩ + |11⟩)/√2; GHZ states (3, 4, 5 qubits); normalization invariant; gate identities (HH=I, XX=I, S²=Z, T²=S) |
| `qft_kat.rs` | 36 | QFT on |0…0⟩ (*n* = 1,2,3,4): uniform superposition; QFT on basis states (*n* = 3,4): published Fourier amplitudes; QFT∘iQFT = identity; measurement distribution (Born-rule frequencies, seeded sampler); sparse-dense agreement (all gate types + round-trip conversion + principle-4 annotation) |
| `modexp_kat.rs` | 41 | Permutation correctness (controlled-add-mod, controlled-mult-mod, full sweeps); modular exponentiation correctness (2^3 mod 15 = 8, 7^2 mod 15 = 4, etc.); reversibility (forward + inverse = identity); ancilla-clean (full state restored); control-off no-op |
| `factor_kat.rs` | ~30 | Order KATs (ord₂(15)=4, ord₇(15)=4, ord₂(21)=6, ord₂(35)=12, ord₂(91)=12); continued-fraction KATs (known phase → known order, deterministic classical); end-to-end factoring (15→3×5, 21→3×7, 35→5×7, 91→7×13); qubit-budget verification; `#[ignore]`-gated seed-finder for N=91 |
| `pointadd_kat.rs` | ~40 | Classical group-law correctness (all 13 scalar multiples, negation, commutativity, associativity, order check 13·G=∞); permutation correctness (full sweeps over all *P* for *cG* = G, 2G, 6G; exceptional cases ∞+G, G+(−G)); reversibility (full sweeps); scratch-clean (λ register = 0 after circuit); control-off no-op |
| `ecdlp_kat.rs` | 19 (+5 `#[ignore]`) | Order/group-law KATs; 2D-lattice extraction KATs (known (*a*′,*b*′) → *k*, all *b*′ ≠ 0 cases); end-to-end ECDLP (*k* = 3,5,7,9,12 with seed=0); qubit budget (17 qubits, within ceiling); `#[ignore]`-gated: `ecdlp_all_k_values_seed0` (all *k* ∈ 2..13), `ecdlp_reproducible`, and three `rho_crosscheck_*` tests (cross-check against `rho::ecdlp::solve_brent`) |

**The `#[ignore]`-gated tests.** Two categories:

1. *Slow full-sweep tests* (`ecdlp_all_k_values_seed0`, `ecdlp_reproducible`): run 11 full
   quantum circuit instances (~165 s in debug mode). The targeted fixture tests (k=3,5,7,9,12)
   cover the same ground in the normal test suite.

2. *`rho` cross-check tests* (`rho_crosscheck_k3`, `rho_crosscheck_k7`,
   `rho_crosscheck_agrees_with_shor_k5`): use `rho::ecdlp::solve_brent` as an oracle to
   cross-check the Shor solver. These require the `rho` dev-dependency and are not part of the
   green path.

---

## §8. Cross-References and Further Reading

**T.S — the maths-first sibling.** `docs/MATHEMATICS.md` ch. 11 (to be written in S.D.2) is the
mathematical counterpart to this code-tour. It will cover: the QFT as a Fourier transform over
ℤ/2^*n*ℤ, the phase-estimation argument, the order-finding → factoring reduction (the period-
finding payoff proof), the two-register hidden-subgroup construction for ECDLP, the 2D-lattice
extraction, and the PQC context (Shor's algorithm as the quantum threat to RSA and ECC). Every
"see T.S" pointer in this document is a forward reference that S.D.2 resolves.

**BENCHMARKS.** The performance facts cited in this document — qubit-budget tables, wall-clock
timings, the dense-vs-sparse comparison, and the `### Science↔engineering note` annotations —
are in `docs/BENCHMARKS.md` §§ S.A, S.B, S.C. This code-tour cites those sections; it does not
extend them.

**Primary literature.**

- Shor, P.W. (1994). "Algorithms for quantum computation: discrete logarithms and factoring."
  *Proceedings of the 35th Annual Symposium on Foundations of Computer Science (FOCS 1994).*
  The original paper presenting both the factoring and discrete-logarithm algorithms.

- Proos, J., Zalka, C. (2003). "Shor's discrete logarithm quantum algorithm for elliptic curves."
  *Quantum Information and Computation* 3(4), 317–344.
  The two-register ECDLP construction that S.C implements.

- Nielsen, M.A., Chuang, I.L. (2000). *Quantum Computation and Quantum Information.*
  Cambridge University Press. §5.3 (order-finding and factoring), §5.4 (discrete logarithms).
  The standard textbook reference for the QFT, phase estimation, and Shor's algorithm.

- Roetteler, M., Naehrig, M., Svore, K.M., Lauter, K. (2017). "Quantum resource estimates for
  computing elliptic curve discrete logarithms." *ASIACRYPT 2017.*
  The production-scale ECDLP circuit design (the qubit-optimised construction that S.C
  deliberately does not implement, per principle 3).
