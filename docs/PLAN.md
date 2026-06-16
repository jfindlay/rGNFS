<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-S open (S.A — state-vector quantum simulator, the Shor substrate)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **set by lever 3 (cost of design error), held against the
lever-5 opt-down.** S.A is **Category-A substrate**: the `StateVec` register + gate-set interface
(C-StateVec) is consumed by S.B (Shor-for-factoring) and S.C (Shor-for-ECDLP), so a wrong substrate
interface propagates through two downstream sub-tracks — the high-cost-of-design-error case lever 3
holds the adjudicator at the strongest tier even though no S.A session is itself Opus-flagged.
**Lever 5 is strong here** (quantum circuits have exceptionally good KATs — deterministic published
amplitudes, unitarity checks, Bell/GHZ/QFT known vectors — and S.A is a toy-scale pedagogical
simulator, ≤25 qubits, moderate correctness-criticality), which *would* license opting the juncture
down to `sonnet`; that opt-down was **considered and declined** (user-adjudicated at shard time,
2026-06-16) in favour of lever 3, because the substrate-interface-binds-two-tracks risk is judged to
outweigh the strong-inner-loop economy. The differential is single-digit dollars and the
`destructive-HALT` invariant caps the downside either way. *(The ◆ fork pages `@plan-juncture` at
opus; both per-row tiers are **Sonnet** — S.A carries no Opus-flagged session per the ROADMAP
Opus-flagged table.)*

**Scope boundary — S.A is the simulator substrate ONLY, NOT Shor itself (the next-sub-track guard).**
S.A builds the **classical state-vector quantum-circuit simulator** — the register, the standard
gate set, the sparse-state optimization, measurement, and the QFT subroutine — and **nothing that
calls it to break a cryptosystem**. Shor-for-factoring (S.B) and Shor-for-ECDLP (S.C) are the
*consumers* of this substrate, separately sharded. A `@build` agent that implements modular
exponentiation as a quantum circuit, the Proos–Zalka ECDLP circuit, or the order-finding /
period-extraction classical post-processing in S.A has reached into S.B/S.C — that is defocus.
S.A's deliverable is the simulator and its primitives, validated against **published small-circuit
results** (the ROADMAP KAT), not an end-to-end factorization.

The substrate survey (forked `@explore`, 2026-06-16) established four grounding facts:

1. **`num-complex` is absent workspace-wide — S.A adds the first complex-number dependency (the one
   new edge).** Every existing crate uses real/integer arithmetic (`crypto-bigint`, `num-bigint`,
   `num-integer`, `num-traits`); no `Complex<·>` anywhere. A state-vector simulator needs complex
   amplitudes, so `shor/Cargo.toml` adds `num-complex = "0.4"` (which provides `Complex<f64>`;
   `num-traits` is already universal for the `Float` bounds). This is **the sub-track's only new
   workspace dependency** and it is leaf-confined to the new `shor` crate.

2. **The new crate is `shor/`, a top-level track crate (peer to `gnfs`/`rho`), per the track-as-crate
   convention (user-adjudicated at shard time).** The survey confirmed the workspace idiom: track
   crates live top-level (`gnfs`, `rho`), `shared/*` holds cross-track-reusable substrate. Track S
   ships as one cohesive crate (simulator + Shor-factoring + Shor-ECDLP + the PQ writeup); the
   simulator is Shor-specific, not general substrate, so `shor/` (not `shared/statevec/`) is the
   home. The crate skeleton mirrors the established pattern: `edition = "2024"`, a `[lints]` block
   (`unsafe_code = "forbid"`, `missing_docs = "warn"`, `clippy::all = "deny"`,
   `clippy::pedantic = "warn"`), `src/lib.rs` + shallow module dirs, `tests/*_kat.rs`, and the
   workspace `Cargo.toml` members list gains `"shor"`.

3. **No paired `*.W` math chapter at S.A — T.S pairs with S.D, the track closeout (confirmed).** The
   `docs/MATHEMATICS.md` ToC already stubs **ch. 11 "Shor's Algorithm and Post-Quantum Context"
   (T.S — to be appended)**, and the Track-τ pairing rule binds the per-track math chapter to the
   track's `*.W` writeup session — which for Track S is **S.D** (the post-quantum writeup), not S.A.
   S.A is a mid-track substrate sub-track: it ships code + KATs + benchmark numbers, no integrative
   prose chapter. *(This is the substrate-session pacing the planning manual names: front-loaded
   design surface, no integrative writeup — the writeup is owed at the track ◆, which is S.D.)*

4. **The KAT corpus for a simulator is exceptionally strong (the lever-5 fact) — published small-
   circuit results + unitarity + known state vectors (confirmed).** Quantum-circuit simulators have
   deterministic, fast, published-value KATs: Bell/GHZ state amplitudes, gate unitarity
   (`U U† = I`), the QFT on a known input matching published Fourier amplitudes, single-qubit
   rotation identities. This is why lever 5 is strong (and why the opt-down was *available* even
   though lever 3 declined it). The KATs follow the established `rho/tests/*_kat.rs` idiom: a
   per-fixture-class helper, many `#[test]` one-liners, semantic verification, Python/published
   reference values noted in comments, no oracle gating (the published values are self-contained).

The work splits at **one dense↔sparse contract-sharp seam**, **2 sessions** (the ROADMAP low end of
the 2–3 band; the soft seam for an additive S.A.3 is named below):

1. **S.A.1 — State-vector register + standard gate set (Sonnet, Cat A).** The `StateVec` dense
   register (`Vec<Complex<f64>>` over n qubits, little-endian basis indexing) + the standard gate
   set (X, Y, Z, H, S, T, phase, CNOT, controlled-phase, Toffoli / multi-controlled) applied by
   index arithmetic + the unitarity/Bell/GHZ KATs. **Freezes C-StateVec** (the register + gate-
   application interface S.A.2, S.B, S.C consume). **Over-specified substrate** (Category-A rule):
   carries the controlled-/multi-controlled-gate surface S.B's modular-exponentiation circuit and
   S.C's Proos–Zalka circuit will need, even though S.A itself does not exercise them.

2. **S.A.2 ◆ — Sparse-state optimization + measurement + QFT (Sonnet, Cat A→B).** The sparse-state
   representation (a hashmap of nonzero basis-amplitudes, for circuits whose state stays sparse) +
   measurement/sampling (Born-rule collapse + a deterministic-seed sampler) + the **QFT** (the
   Shor period-finding workhorse) over the frozen register. **Consumes C-StateVec. Freezes
   C-Sparse + C-QFT.** Crosses the **S.A ◆ boundary** — the simulator substrate is complete and the
   gate/QFT/measurement interface the Shor sub-tracks consume is frozen.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing Shor's modular
exponentiation, the Proos–Zalka ECDLP circuit, or the order-finding/continued-fraction post-
processing — those are S.B/S.C, the deferred consumers; or adding a gate the gate set does not need;
or scaling past the ~25-qubit demonstration ceiling) and **rigidity** (forcing the sparse
representation where the dense register is the honest demonstration vehicle; or skipping the QFT KAT
because "Shor will test it"; or over-engineering the simulator for performance it does not need at
toy scale — principle 3).

**Scoping discipline.** S.A builds the simulator at **demonstration fidelity** (≤25 qubits, dense
register with a sparse-state *option*; the gate set is mathematically complete, not engineering-
optimized — no SIMD, no GPU, no tensor-network contraction; principle 3). The **principle-4
science↔engineering gap is load-bearing and explicit:** a state-vector simulator is *exponential* in
qubit count — the ~25-qubit ceiling is a **resource-scale** wall, not a mathematical one (the
mathematics is the same at 25 or 250 qubits; only the `2^n`-amplitude array makes 250 unreachable on
a laptop). The BENCHMARKS.md `## S.A` section records this: the simulator demonstrates Shor's
*mathematics* correctly at toy scale, and the qubit ceiling is exactly the engineering-scale boundary
principle 4 says to annotate, not paper over. *(This is the S-track analogue of the index-calculus
"asymptotic win not observable at toy scale" posture — the simulator exhibits the algorithm's logic,
not its quantum speedup, which requires real quantum hardware out of scope by construction.)*

---

## Purpose (design intent)

Per ROADMAP (Phase ε, S.A): "*S.A — State-vector simulator. 2-3 sessions. No structural predecessor.
Up to ~25 qubits. Standard gate set; sparse-state optimization. KAT: published small-circuit results.
Sonnet.*" And per the design statement (item 6): "*Shor's algorithm via a classical state-vector
simulator, for both factoring and ECDLP.*"

S.A is the **substrate for Track S** — the classical quantum-circuit simulator on which Shor's
algorithm (S.B factoring, S.C ECDLP) is built. It is the project's first quantum-model component:
where every prior track escapes the generic √n / L-notation search bound by finding *classical*
exploitable structure (a homomorphism, a pairing, a smoothness phenomenon), Track S dissolves the
bound *entirely* in the quantum model via period-finding — and S.A is the machine that makes that
demonstration runnable on classical hardware at toy scale.

The deliverable is two-fold (the two conceptual units, each a session):

1. **The state-vector register + standard gate set (S.A.1).** The dense `StateVec` amplitude register
   + the universal gate set + gate application by index arithmetic. The substrate every quantum
   circuit in Track S is assembled from. **Freezes C-StateVec.**

2. **The sparse-state optimization + measurement + QFT (S.A.2 ◆).** The sparse representation (for
   sparse-state circuits), Born-rule measurement, and the Quantum Fourier Transform — the period-
   finding subroutine Shor's algorithm is built around. **Freezes C-Sparse + C-QFT.**

S.A is **Shor-free** (it builds no cryptanalytic circuit — S.B/S.C are the consumers), **substrate-
over-specified** (the controlled-gate surface is carried for S.B/S.C up front, the Category-A rule),
and **principle-4-honest** (the exponential qubit ceiling is annotated as a resource-scale wall, not
hidden). Re-read this intent at the ◆ boundary to catch defocus (a Shor circuit, the Proos–Zalka
construction, period post-processing) and rigidity (forcing sparsity, skipping the QFT KAT, perf
over-engineering).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-16; the
workspace `Cargo.toml` carries only `[workspace]` members + a `[profile.bench]`); raw `cargo` is the
only CI surface (unchanged from the Track-E sub-tracks). S.A.2 may add a benchmark, so the third
discovered command applies if so: `VERIFY_BENCH = cargo bench --no-run` (compile-only — the actual
`cargo bench` timings are hand-transcribed into BENCHMARKS.md, the established pattern). `/run-plan`
re-discovers at preflight. S.A **adds one new workspace member (`shor`) and one new dependency
(`num-complex`)** — both leaf-confined to the new crate — so the gate is a **new-crate-builds +
new-crate-tests-green gate**, with the no-regression invariant trivially held for every existing
crate (S.A touches no existing code):

- **The existing rho / gnfs / shared KATs must stay green** — S.A adds a new crate; it changes no
  existing solver path. `cargo test --workspace` is the no-regression guard *and* the new-crate KAT
  gate (the `shor` KATs join the workspace test run).
- **`cargo check --workspace` must stay green with the new `shor` member + the `num-complex` edge** —
  the new dependency is leaf (only `shor` depends on it); no existing crate's edge changes; no cycle
  risk.
- **`cargo bench --no-run` compiles any S.A.2 bench** — if S.A.2 adds a `shor/benches/` qubit-scaling
  bench, it must build; the timings are hand-transcribed (matching the G.W / E.W pattern).
- **KATs are published-value, self-contained — no oracle on the green path** — the simulator's KATs
  are published small-circuit amplitudes + unitarity + QFT known vectors; no external quantum
  simulator is called. (No PARI/msolve/CADO analogue exists for Track S; there is no live quantum
  oracle. Cross-checks against a reference simulator, if any are written, are `#[ignore]`-gated.)

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| S.A.1 | State-vector register + standard gate set | A | Sonnet | (none — no structural predecessor; new `shor` crate); the workspace crate-skeleton convention (read: `shared/gf2m`, `rho` for the `[lints]` + `edition` + `tests/*_kat.rs` idiom) | `Cargo.toml` (add `"shor"` to `[workspace] members`), `shor/Cargo.toml` (new: `num-complex = "0.4"` + `[lints]` + `criterion` dev-dep if benched), `shor/src/lib.rs` (new: crate docstring + module decls), `shor/src/statevec/mod.rs` + `shor/src/gates/mod.rs` (new: `StateVec` register + gate set), `shor/tests/statevec_kat.rs` (new: unitarity / Bell / GHZ KATs) |
| S.A.2 ◆ | Sparse-state optimization + measurement + QFT | A | Sonnet | C-StateVec (frozen S.A.1 — the dense register + gate interface) | `shor/src/sparse/mod.rs` (new: sparse-state representation), `shor/src/measure/mod.rs` (new: Born-rule measurement + seeded sampler), `shor/src/qft/mod.rs` (new: QFT over the register), `shor/tests/qft_kat.rs` (new: QFT published-amplitude + measurement-distribution KATs), `docs/BENCHMARKS.md` (add `## S.A` section: dense-vs-sparse + qubit-scaling table + the principle-4 resource-scale note) |

**Sequencing notes.** Strictly serial: **S.A.1 → S.A.2.** S.A.1 lands the dense register + gate set
and freezes the interface; S.A.2 consumes it (sparse variant, measurement, QFT all build on the
frozen register/gate surface) and closes the sub-track. **One `@architect` marker:** the **S.A.2 ◆**
(the sub-track-boundary juncture — ratifying the frozen simulator substrate before S.B/S.C consume
it). *(Tradeoff named: S.A pages a juncture only at the ◆-close, NOT at the open — unlike the
Opus-substrate opens E.B.1/E.F.1/G.A.1, whose substrate-design crux forced an opening fork. S.A.1's
gate-set design is well-understood (the universal gate set is canonical), so no opening fork is
warranted; the integrative judgment — is the substrate interface right for two downstream tracks? —
concentrates at the ◆ freeze. The juncture-tier is `opus` regardless, per lever 3.)*

**Why 2 sessions (the ROADMAP low end of 2–3).** The split is taken at the single dense↔sparse
contract-sharp seam:
- **One-line-commit-title corollary.** "State-vector register + standard gate set" and "Sparse-state
  optimization + measurement + QFT" are **two distinct commit titles**. Bundling them — "build the
  register AND the gate set AND the sparse representation AND measurement AND the QFT" — fails the
  corollary.
- **Irreducible units kept whole (lever 2).** S.A.1 is the dense substrate (register + gates as one
  conceptual unit); S.A.2 is the optimized/derived layer (sparse + measure + QFT). Neither fractures
  below its floor — the gate set is one unit (splitting "X/Y/Z gates" from "controlled gates" would
  fracture the universal set), and sparse+measure+QFT cohere as the layer that makes the register
  *usable* by a circuit.
- **Contract-sharp boundary.** S.A.1 **freezes** C-StateVec (the register + gate interface); S.A.2
  **consumes** it (sparse/measure/QFT all build on the frozen surface) and **freezes** C-Sparse +
  C-QFT. The sparse representation and QFT are meaningless without the register freeze — they operate
  on it.
- **Strong lever 5 licenses the small dense commits.** The simulator's KATs are deterministic and
  published (unitarity, Bell/GHZ, QFT vectors), so the inner loop catches drift behaviourally — the
  exact condition that makes small commits safe.

**The softest seam — could the QFT split off as a third session (S.A.3)?** The ROADMAP 2–3 band
allows it, and the ceiling-bias note (G and D both landed at/above their bands) argues the real count
may be 3. The chosen shard keeps **sparse + measurement + QFT together in S.A.2**, because they are
the cohesive "make the register usable" layer over the *same* frozen interface — there is **no
contract seam between them** (sparse-state, measurement, and the QFT all consume C-StateVec and freeze
sibling contracts), so splitting them up front would over-shard without buying an early freeze.
**If S.A.2 overruns** (the sparse representation + the QFT + the measurement sampler + their KATs push
past the session band — plausible, since the QFT is the load-bearing Shor subroutine and deserves a
full KAT), the escape applies: **split the QFT into a dedicated S.A.3 ◆** (sparse + measurement in
S.A.2, freezing C-Sparse; QFT in S.A.3, freezing C-QFT and carrying the ◆) — an additive-reshard
surfaced at the S.A.1 ◆ readout or by S.A.2 once the layer's true size is concrete, never a silent
overrun. This is the one place the 2-vs-3 sizing is genuinely uncertain until the dense substrate
freezes and S.A.2's true size is visible.

---

## Session detail

S.A.1 is specified at near-full fidelity (the gate set is canonical — the universal standard gate set
is known; the register representation is the standard dense amplitude array; the only design choices
are the basis-indexing convention and the controlled-gate surface breadth, both resolved below).
S.A.2 is specified at the structural level (the sparse/measure/QFT outline) with the per-piece content
sketched — correct per the substrate-first discipline: the sparse representation's exact shape and the
QFT's true KAT size are crisp only after the register interface freezes.

### S.A.1 — State-vector register + standard gate set (Sonnet, Cat A)

**Deliverable:** the dense quantum-circuit substrate — the `StateVec` register + the universal gate
set + the new `shor` crate skeleton. The pieces:
- **The crate skeleton** (`shor/Cargo.toml` + `Cargo.toml` workspace edit): a new top-level track
  crate `shor`, `edition = "2024"`, the established `[lints]` block (`unsafe_code = "forbid"`,
  `missing_docs = "warn"`, `clippy::all = "deny"`, `clippy::pedantic = "warn"`),
  `num-complex = "0.4"` as the sole new dependency, `proptest = "1"` dev-dep (the established
  property-test idiom), and `criterion` dev-dep + a `[[bench]]` entry *iff* S.A.1 lands a bench
  (else deferred to S.A.2). The workspace `Cargo.toml` `members` list gains `"shor"`.
- **The `StateVec` register** (`shor/src/statevec/mod.rs`, new): a dense `Vec<Complex<f64>>` of
  `2^n` amplitudes over `n` qubits, little-endian basis indexing (qubit 0 is the least-significant
  bit), with constructors (`|0…0⟩`, a basis state, an arbitrary normalized vector), a normalization
  invariant (`Σ|aᵢ|² = 1`, checked in debug + a KAT), and the ~25-qubit ceiling documented (the
  `2^25` amplitude array is the resource wall).
- **The standard gate set** (`shor/src/gates/mod.rs`, new): the universal set applied by index
  arithmetic over the register — single-qubit X, Y, Z, H, S, T, phase(θ), arbitrary single-qubit
  unitary; two-qubit CNOT, controlled-phase, SWAP; multi-qubit Toffoli / **multi-controlled gates**
  (the over-specified surface S.B's modular exponentiation + S.C's Proos–Zalka circuit consume).
  Each gate is applied in-place by iterating the amplitude pairs the gate couples (the standard
  `O(2^n)`-per-gate state-vector update), not by materializing a `2^n × 2^n` matrix.

Consumes nothing (no structural predecessor; new crate). Reads the workspace crate-skeleton
convention (`shared/gf2m/Cargo.toml`, `rho/Cargo.toml` for the `[lints]`/`edition`/`tests` idiom).
**Freezes C-StateVec.**

**KAT:** over published small-circuit results + algebraic invariants: (1) **unitarity** — each gate
`U` satisfies `U U† = I` (applied to basis states, the output amplitudes match); (2) **Bell state** —
`H` on qubit 0 then `CNOT(0→1)` on `|00⟩` gives `(|00⟩ + |11⟩)/√2` (the published amplitudes);
(3) **GHZ state** — the `n`-qubit GHZ amplitudes; (4) **normalization** — `Σ|aᵢ|² = 1` preserved
across any gate sequence; (5) **gate identities** — `HH = I`, `XX = I`, `S² = Z`, `T² = S` (published
single-qubit relations). The KATs follow the `rho/tests/*_kat.rs` idiom: a per-fixture helper, many
`#[test]` one-liners, semantic amplitude comparison (within an `f64` tolerance), published values
noted in comments. **Verify gate:** `cargo test --workspace` green (the new `shor` KATs pass + no
regression); `cargo check --workspace` green (new member + `num-complex` edge, no cycle); `cargo bench
--no-run` iff a bench lands.

**Subtlety (load-bearing):** (1) **Over-specify the controlled-gate surface (Category-A rule)** —
S.A itself only needs single + two-qubit gates for its KATs, but S.B's modular exponentiation and
S.C's Proos–Zalka circuit need multi-controlled gates; carry them now (the cost of adding a frozen-
interface method later is higher than carrying an unused one). (2) **Basis-indexing convention is a
freeze** — little-endian (qubit 0 = LSB) must be fixed at S.A.1 and documented; S.B/S.C and the QFT
all index into the register and a silent convention flip is a wrong-answer bug. (3) **`f64`
amplitudes, not exact** — the simulator uses floating-point complex amplitudes; KATs compare within
tolerance, and the normalization/unitarity checks carry an explicit ε. (4) **No matrix
materialization** — gates apply by amplitude-pair iteration (`O(2^n)` per gate), never a `2^n × 2^n`
matrix (`O(4^n)` memory) — the standard state-vector method, and a principle-3 honesty point (no
engineering accel, but also no naive blowup). (5) **The ~25-qubit ceiling is principle-4** — the
`2^n` array is the resource wall; document it, do not engineer around it.

**Deferred:** the sparse-state optimization (S.A.2); measurement (S.A.2); the QFT (S.A.2); the
BENCHMARKS.md `## S.A` section (S.A.2 — needs the sparse-vs-dense numbers); any Shor circuit (S.B/S.C
— the deferred consumers).

### S.A.2 ◆ — Sparse-state optimization + measurement + QFT (Sonnet, Cat A→B)

**Deliverable:** the layer that makes the register usable by a circuit — the sparse representation,
Born-rule measurement, and the QFT — + the BENCHMARKS.md section + the sub-track ◆ close. Structural-
fidelity sketch (the per-piece content is crisp once the register interface freezes). The pieces:
- **The sparse-state representation** (`shor/src/sparse/mod.rs`, new): a hashmap (or sorted vector)
  of `(basis_index, Complex<f64>)` nonzero amplitudes, for circuits whose state stays sparse (the
  ROADMAP's "sparse-state optimization"). Same gate-application semantics as the dense register
  (consumes C-StateVec's gate interface), but iterating only nonzero entries. A conversion to/from
  the dense register. The honest scope: sparse helps *only* while the state is sparse — a Hadamard
  on every qubit makes it dense, so this is a demonstration of the technique, not a universal speedup
  (a principle-4 annotation).
- **Measurement** (`shor/src/measure/mod.rs`, new): Born-rule measurement — sample a basis state with
  probability `|aᵢ|²`, collapse the register, with a deterministic seeded sampler (the
  `rand_chacha` idiom the workspace already uses) so KATs are reproducible. Single-qubit and
  full-register measurement.
- **The QFT** (`shor/src/qft/mod.rs`, new): the Quantum Fourier Transform over the register — the
  Hadamard + controlled-phase ladder (`O(n²)` gates), the period-finding workhorse Shor's algorithm
  is built around. Built from the frozen S.A.1 gate set (H + controlled-phase), with the bit-reversal
  convention documented (it interacts with the little-endian basis indexing).
- **The `## S.A` BENCHMARKS.md section** (`docs/BENCHMARKS.md`, append): the dense-vs-sparse
  comparison + a qubit-scaling table (wall-clock vs n, showing the `2^n` wall) + the **principle-4
  resource-scale note** (the simulator demonstrates Shor's *mathematics* correctly; the ~25-qubit
  ceiling is the engineering/resource wall, not a mathematical one — the same posture as the index-
  calculus "asymptotic win not observable at toy scale"). Matches the per-sub-track BENCHMARKS.md
  genre (prose setup + table + science↔engineering note).
- **The S.A ◆ close**: re-read the Purpose intent; verify the simulator substrate is complete and the
  interface S.B/S.C consume (register + gates + measurement + QFT) is coherent and frozen; confirm
  the principle-4 ceiling is annotated.

Consumes C-StateVec (frozen S.A.1). **Freezes C-Sparse + C-QFT.**

**KAT:** (1) **QFT published amplitudes** — the QFT on `|0…0⟩` gives the uniform superposition; the
QFT on a basis state gives the published Fourier amplitudes (`(1/√N) Σ ω^{jk} |k⟩`); QFT then
inverse-QFT is the identity. (2) **Measurement distribution** — a seeded measurement of a known
superposition recovers the Born-rule frequencies within tolerance over many samples (a deterministic-
seed statistical KAT). (3) **Sparse-dense agreement** — a gate sequence on the sparse representation
yields the same amplitudes as on the dense register (the sparse path is a no-regression mirror of the
dense path). **Verify gate:** `cargo test --workspace` green (the new KATs + no regression);
`cargo check --workspace` green; `cargo bench --no-run` compiles any qubit-scaling bench.

**Subtlety (load-bearing):** (1) **The QFT is the load-bearing Shor subroutine** — it is the piece
S.B/S.C's period-finding depends on; its KAT (published Fourier amplitudes + QFT/iQFT identity) must
be complete, not a smoke test. If the QFT + its KAT + the sparse layer overruns the band, **split the
QFT into S.A.3** (the named soft seam). (2) **Bit-reversal × little-endian interaction** — the QFT
conventionally outputs bit-reversed; combined with S.A.1's little-endian indexing, the convention
must be fixed and documented, else S.B/S.C read the period out of the wrong qubit order (a silent
wrong-answer bug). (3) **Sparse is a demonstration, not a universal win (principle 4)** — sparsity
helps only while the state is sparse; annotate that a fully-superposed state is dense and the sparse
path then matches the dense cost. (4) **Seeded measurement for reproducible KATs** — the sampler is
deterministic-by-seed (`rand_chacha`), so the measurement-distribution KAT is reproducible. (5) **The
~25-qubit ceiling is principle-4 (resource scale)** — the BENCHMARKS.md note records the exponential
wall as engineering scale, not mathematical omission.

**Deferred:** Shor-for-factoring (S.B — the modular-exponentiation circuit + order-finding post-
processing); Shor-for-ECDLP (S.C — the Proos–Zalka circuit); the post-quantum writeup (S.D) + T.S
(the ch. 11 math chapter, paired with S.D); all of Track ζ (umbrella) + τ-bind. **S.A is the
substrate; the Shor attacks are its consumers.**

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
S.A.2 ◆ to confirm: (1) the simulator substrate is complete — the register + universal gate set +
sparse option + measurement + QFT compose correctly and the published-value KATs pass; (2) **the
interface is right for two downstream tracks** — the gate surface (including multi-controlled gates),
the QFT, and measurement expose what S.B's modular-exponentiation circuit + S.C's Proos–Zalka circuit
will consume, so neither has to re-open S.A to add a primitive (the lever-3 reason the juncture is
held at opus); (3) the basis-indexing + bit-reversal conventions are fixed and documented (the silent-
wrong-answer guard); (4) the principle-4 resource-scale ceiling is annotated (the `2^n` wall is
engineering, not mathematical); (5) C-StateVec + C-Sparse + C-QFT expose what S.B/S.C consume so the
Shor sub-tracks build without re-opening S.A; (6) S.A stayed in scope — no Shor circuit, no period
post-processing, no perf over-engineering. **Also: surface the outstanding static-frame ROADMAP debt**
(the Track-E Progress/Remaining reconciliation owed since the E.W ◆ — the Progress table still shows
E in progress and S not started; the Track-E ◆ close was the named reconciliation point but the
ROADMAP write was out of `@architect` PLAN-write scope, and now Track S has opened) — **note that the
S.A open is a natural second reconciliation prompt** (Phase δ is closed, Phase ε has started), though
the ROADMAP write itself is out of `@architect` PLAN-write scope (a capture candidate, not a PLAN
edit). One-shot findings; does not implement. Held at **opus** per the header (lever 3).

---

## Cross-session contracts

S.A **freezes three** contracts (C-StateVec at S.A.1; C-Sparse + C-QFT at S.A.2 ◆) and **amends no
prior frozen contract** — it is a new crate with no structural predecessor; every existing contract
is untouched. S.A adds the `shor` crate + the `num-complex` edge — both leaf-confined; no existing
crate's edge changes.

### C-StateVec — the dense register + gate-application interface (compiler-/test-enforced) — *to be frozen at S.A.1*

**Defined in:** S.A.1 (`shor/src/statevec/mod.rs` + `shor/src/gates/mod.rs`).
**Consumed by:** S.A.2 (sparse/measure/QFT build on the register + gate interface); **downstream:
S.B** (the modular-exponentiation circuit), **S.C** (the Proos–Zalka ECDLP circuit). Compiler-enforced
(the `StateVec` type + gate function signatures) + test-enforced (the unitarity/Bell/GHZ KATs).

**Ratified shape (to be confirmed at the S.A ◆).** The register is a dense `Vec<Complex<f64>>` of
`2^n` amplitudes over `n` qubits, **little-endian basis indexing (qubit 0 = LSB)** — the convention
the QFT and every Shor circuit index into. The gate interface applies a gate in-place by amplitude-
pair iteration: single-qubit (X, Y, Z, H, S, T, phase(θ), arbitrary unitary), two-qubit (CNOT,
controlled-phase, SWAP), multi-qubit (Toffoli, **multi-controlled** — over-specified for S.B/S.C).
**Invariants:** normalization (`Σ|aᵢ|² = 1`) preserved across any gate; each gate is unitary
(`U U† = I`, a KAT); the little-endian convention is fixed (a silent flip is a wrong-answer bug);
`f64` amplitudes compared within ε; gates apply by pair-iteration (`O(2^n)`), never matrix
materialization (`O(4^n)`). *(The Category-A over-specification: the multi-controlled-gate surface is
carried for S.B/S.C even though S.A's own KATs do not exercise it.)*

### C-Sparse — the sparse-state representation (compiler-/test-enforced) — *to be frozen at S.A.2 ◆*

**Defined in:** S.A.2 (`shor/src/sparse/mod.rs`). **Consumed by:** S.B/S.C *optionally* (circuits
whose state stays sparse may use it). Compiler-enforced (the sparse type + conversion signatures) +
test-enforced (the sparse-dense agreement KAT).

**Ratified shape (to be confirmed at the S.A ◆).** A map of `(basis_index, Complex<f64>)` nonzero
amplitudes with the same gate semantics as the dense register (consumes C-StateVec's gate interface,
iterating only nonzero entries) + a dense↔sparse conversion. **Invariants:** a gate sequence on the
sparse path yields the same amplitudes as the dense path (the sparse-dense agreement KAT — the
no-regression mirror); sparsity helps only while the state is sparse (a fully-superposed state is
dense — the principle-4 annotation, not a universal-speedup claim).

### C-QFT — the Quantum Fourier Transform subroutine (compiler-/test-enforced) — *to be frozen at S.A.2 ◆*

**Defined in:** S.A.2 (`shor/src/qft/mod.rs`). **Consumed by:** **S.B/S.C** (period-finding — the
load-bearing Shor subroutine). Compiler-enforced (the `qft`/`iqft` signatures) + test-enforced (the
published-Fourier-amplitude + QFT/iQFT-identity KATs).

**Ratified shape (to be confirmed at the S.A ◆).** The QFT over the register, built from the frozen
S.A.1 gate set (H + controlled-phase ladder, `O(n²)` gates), with the **bit-reversal convention
documented** (it interacts with the little-endian basis indexing — a silent mismatch reads the period
out of the wrong qubit order). **Invariants:** QFT on `|0…0⟩` = uniform superposition; QFT on a basis
state = the published Fourier amplitudes `(1/√N) Σ ω^{jk} |k⟩`; QFT∘iQFT = identity (the KAT); the
bit-reversal × little-endian convention is fixed and documented (the silent-wrong-answer guard for
S.B/S.C's period extraction). *(The QFT is the soft-seam candidate: if S.A.2 overruns, this contract
moves to a dedicated S.A.3 ◆.)*

### Frozen contracts read by S.A (consumed, not amended)

- **None.** S.A has no structural predecessor (the ROADMAP names "No structural predecessor"). It is
  a new crate that reads only the workspace crate-skeleton *convention* (the `[lints]` block, the
  `edition = "2024"`, the `tests/*_kat.rs` idiom — a style template, not a code contract). No frozen
  algorithm or substrate contract is consumed.

### Downstream contracts S.A produces for later sub-tracks (named, not frozen here)

- **C-CurveSubstrate (`rho::curve` — read by S.C, NOT by S.A).** S.C (Shor-for-ECDLP) will consume
  *both* C-StateVec/C-QFT (this sub-track) *and* the frozen `rho::curve` surface (the
  `AffinePoint`/`Curve` types + `scalar_mul`, for the Proos–Zalka circuit's group arithmetic and the
  rho cross-check the ROADMAP names). **S.A does not touch `rho::curve`** — it is named here only so
  the S.C shard (later) knows the cross-track edge exists. The survey confirmed the `rho::curve`
  surface (`AffinePoint`, `Curve`, `scalar_mul`, `secp_k1_toy`) is frozen and public.

### Workspace edges (one new crate, one new dependency, both leaf-confined)

- **One new member: `shor`.** Added to the workspace `Cargo.toml` `members` list. A new top-level
  track crate, peer to `gnfs`/`rho`.
- **One new dependency: `num-complex = "0.4"`.** Absent workspace-wide before S.A; added only to
  `shor/Cargo.toml` (leaf — no existing crate depends on it). Provides `Complex<f64>` for the
  amplitudes. `num-traits` (already universal) supplies the `Float` bounds. No cycle risk;
  `cargo check --workspace` stays green. *(S.A depends on no workspace crate in S.A.1/S.A.2 — the
  simulator is self-contained. The `rho::curve` edge appears only at S.C, a later sub-track.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The S.A.2 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| S.A.1 | State-vector register + standard gate set | done | 396cf84 | C-StateVec |
| S.A.2 ◆ | Sparse-state optimization + measurement + QFT | pending | — | — |

Contracts frozen before this sub-track: the entire classical-attack arc — all of Track G (GNFS
factoring), Track D (NFS-DL), and Track E (algebraic ECDLP, E.A–E.W, closed at the E.W ◆), plus the
shared substrate (C1 smoothness, the field/bigint/numfield/padic/gf2m crates) and the Track-τ
register (C-Textbook, frozen T.0). **S.A consumes none of them** — it is the first Track-S sub-track
and a self-contained new crate. This sub-track **freezes three new contracts** (C-StateVec, C-Sparse,
C-QFT), serving the downstream **S.B** (Shor factoring) and **S.C** (Shor ECDLP). **With the S.A ◆,
the Track-S substrate is complete; S.B/S.C (the Shor attacks) and S.D (the PQ writeup + T.S) remain
in Phase ε, then Phase ζ (umbrella) + the τ-bind close the project.**

---

## Action-frame digest

*(S.A.1 was a clean trivial iteration — no discovery, no contract flex, no surprises. No digest entry.)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **S.A is a self-contained new crate on no structural predecessor — internal-continue (confirmed by
  survey).** The `shor` crate consumes no frozen algorithm contract; it reads only the workspace
  crate-skeleton convention. All S.A code is in the new crate + two leaf edits (the workspace
  `members` list + the `num-complex` dependency). A discovery that the simulator needs a primitive
  from an existing crate is an **additive edge** surfaced at the ◆ — not a silent cross-crate reach.

- **`num-complex` is the one new dependency — leaf-confined (confirmed).** No `Complex<·>` exists
  workspace-wide; `shor` adds `num-complex = "0.4"`. It touches only `shor/Cargo.toml`; no existing
  crate's edge changes. A `@build` agent adding it to a `shared/*` crate (speculative "general
  complex substrate") is defocus — it belongs in `shor` until a second consumer exists.

- **Over-specify the controlled-gate surface (Category-A substrate rule — the load-bearing design
  call).** S.A's own KATs need only single/two-qubit gates, but S.B's modular exponentiation and
  S.C's Proos–Zalka circuit need multi-controlled gates. Carry them at S.A.1 (the cost of adding a
  frozen-interface method later exceeds the cost of carrying an unused one). A shard that ships only
  the gates S.A's KATs exercise under-specifies the substrate and forces S.B/S.C to re-open S.A.
  **Internal-continue → over-specify the gate surface.**

- **The basis-indexing + bit-reversal conventions are silent-wrong-answer guards (confirmed).**
  Little-endian basis indexing (S.A.1) and the QFT's bit-reversal (S.A.2) must be fixed and
  documented; S.B/S.C and the QFT all index into the register, and a silent convention flip reads the
  period out of the wrong qubit order with no error. **Internal-continue → conventions frozen +
  documented at the ◆.**

- **The ~25-qubit ceiling is principle-4 (resource scale, not mathematical) — annotate, do not
  engineer around it.** A state-vector simulator is `O(2^n)` in memory; ≤25 qubits is the laptop
  resource wall, and the mathematics is identical at 25 or 250 qubits. The BENCHMARKS.md `## S.A`
  note records this as the engineering-scale boundary (the index-calculus "asymptotic win not
  observable" analogue), not a mathematical omission. A shard that hides the ceiling or over-engineers
  to push it (SIMD, tensor networks — principle 3) is wrong. **Internal-continue → ceiling annotated.**

- **The QFT may overrun — surface an S.A.3 split if it does (additive-reshard).** The 2-vs-3 sizing
  keeps sparse + measurement + QFT together in S.A.2. **If the QFT + its full KAT + the sparse layer
  push past the band** (plausible — the QFT is the load-bearing Shor subroutine and deserves a
  complete KAT), split the QFT into a dedicated S.A.3 ◆ (sparse + measurement freeze C-Sparse in
  S.A.2; QFT freezes C-QFT in S.A.3) — surfaced at the S.A.1 ◆ readout or by S.A.2 once the layer's
  size is concrete, never a silent overrun. **Additive-reshard if S.A.2 overruns.**

- **No Shor circuit / no period post-processing / no Proos–Zalka in S.A (defocus / scope clarity —
  the deferred consumers).** Shor-for-factoring (S.B), Shor-for-ECDLP (S.C), and the order-finding /
  continued-fraction period extraction are **later, separately-sharded sub-tracks**. A `@build` agent
  that builds a modular-exponentiation circuit or extracts a period in S.A has reached into S.B/S.C —
  S.A is the *substrate*, not the attack.

- **Sparse-state is a demonstration, not a universal win (principle 4).** Sparsity helps only while
  the state is sparse; a Hadamard on every qubit makes it dense, after which the sparse path matches
  the dense cost. The sparse-dense agreement KAT confirms correctness; the BENCHMARKS.md note records
  that the speedup is state-dependent, not universal. Presenting sparse as an unconditional speedup is
  a documentation defect. **Internal-continue → sparse benched + annotated honestly.**

- **Static-frame ROADMAP debt (surface at the S.A ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried from the E.W ◆, now compounded by the Phase-ε open.** The ROADMAP Progress
  table still shows "δ — Algebraic ECDLP (E) … in progress" and "ε — Shor + PQ (S) … not started" —
  but Track E closed end-to-end at the E.W ◆ (Phase δ complete) and Track S has now *opened* with this
  S.A shard. The E.W ◆ digest named the Track-E ◆ close as the natural reconciliation point; the write
  was deferred (out of `@architect` PLAN-write scope). **The S.A open is a second natural prompt** —
  Phase δ should read complete and Phase ε in-progress. The full reconciliation (Track E Done →
  E.A–E.W; strike E from Remaining; mark Phase δ complete; add Track S in-progress) is owed.
  **This is a ROADMAP write — outside the `@architect` PLAN-only write scope; surfaced here as a
  capture candidate for the user to action (via `/note` or a ROADMAP edit), not a PLAN edit.** Not an
  implementation concern; does not block S.A.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase ε — S.A, "*State-vector simulator. 2-3 sessions. No structural
  predecessor. Up to ~25 qubits. Standard gate set; sparse-state optimization. KAT: published
  small-circuit results. Sonnet.*"; the design statement item 6 — "*Shor's algorithm via a classical
  state-vector simulator, for both factoring and ECDLP*"; the Track-τ pairing — T.S/ch. 11 pairs with
  **S.D**, not S.A) and this PLAN before any session. **NOTE: the ROADMAP Progress / Remaining tables
  are stale (Track E still shown in progress though it closed at the E.W ◆; Track S shown not started
  though this shard opens it); the S.A open is a natural reconciliation prompt — surface it at the ◆,
  but it is outside `@architect` PLAN-write scope (a capture candidate for the user).**
- Read the **templates to mirror**: `shared/gf2m/Cargo.toml` + `rho/Cargo.toml` (the crate-skeleton
  idiom — `edition = "2024"`, the `[lints]` block, the `[[bench]]` `harness = false` pattern);
  `rho/src/lib.rs` + `rho/src/ecdlp/` (the module-decomposition idiom — a crate as `lib.rs` + shallow
  module dirs each with a `mod.rs`); `rho/tests/ecdlp_kat.rs` (the `*_kat.rs` idiom — a per-fixture
  helper, many `#[test]` one-liners, semantic comparison, published values in comments);
  `docs/BENCHMARKS.md` (the per-sub-track section genre — prose setup + table + "science↔engineering
  note (principle 4)"; the `## G.W` / `## E.W` sections as the closeout template, here adapted to a
  dense-vs-sparse + qubit-scaling table).
- **Register:** S.A is **Rust library + benchmark code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; the
  new `shor` crate + its KATs + an optional Criterion bench) **plus prose** (the BENCHMARKS.md `## S.A`
  section). **No PEDAGOGY.md or MATHEMATICS.md chapter at S.A** — the Track-S math chapter (T.S,
  ch. 11) pairs with S.D (the track closeout), not this substrate sub-track.
- **Tier routing:** **both S.A.1 and S.A.2 are Sonnet `@build`** (S.A carries no Opus-flagged session
  per the ROADMAP Opus-flagged table — the gate set is canonical, the register is standard, the QFT
  is a known construction; the only design judgment is the interface breadth, which the C-StateVec
  over-specification rule + the ◆ juncture handle). **juncture-tier (header) is `opus`** — set by
  lever 3 (the Category-A substrate interface binds S.B + S.C; a wrong interface propagates through
  two downstream sub-tracks), holding the default *against* the available lever-5 opt-down (strong
  quantum KATs + moderate criticality would license `sonnet`, but the two-track-binding risk was
  judged to outweigh it — user-adjudicated at shard time). The ◆ fork pages `@plan-juncture` at opus.
- **Invariants to preserve:** **S.A builds NO Shor circuit** (it is the substrate — S.B/S.C are the
  consumers; no modular exponentiation, no Proos–Zalka, no period post-processing). **S.A amends NO
  existing contract** (new self-contained crate; the only edits to existing files are the workspace
  `members` list + the new `num-complex` dependency). **The gate surface is over-specified**
  (multi-controlled gates carried for S.B/S.C — the Category-A rule). **The basis-indexing
  (little-endian) + bit-reversal conventions are fixed and documented** (the silent-wrong-answer
  guard for S.B/S.C period extraction). **Gates apply by amplitude-pair iteration** (`O(2^n)`), never
  matrix materialization (`O(4^n)`). **The ~25-qubit ceiling is annotated as a resource-scale wall**
  (principle 4 — engineering, not mathematical). **Measurement is seeded** (reproducible KATs).
  **Sparse-state is a state-dependent demonstration, not a universal speedup** (principle-4 honesty).
- **One new crate, one new dependency, both leaf-confined (load-bearing for S.A).** `shor` is a new
  top-level member; `num-complex = "0.4"` is added only to `shor/Cargo.toml`. No existing crate's edge
  changes; `cargo check --workspace` stays green with no cycle risk; the no-regression invariant
  (existing KATs green) holds trivially since no existing code changes.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  substrate-register session + a derived-layer/optimization session closing at the ◆) is **new for
  this project as a Track-S opener** (the first quantum-model component), and the simulator is the
  substrate two downstream sub-tracks bind to, so the conservative default is to halt at the S.A.2 ◆
  for the human glance + the opus juncture fork. Both sessions are Sonnet and mechanical-ish (canonical
  gate set, standard register, known QFT), so S.A.1 could run autonomously, but the unproven-opener +
  two-track-binding-substrate argues for `halt-at-boundaries` on the first invocation; the S.A.2 ◆
  fork is itself a halt. *(Tradeoff vs autonomous: `halt-at-boundaries` trades a little velocity on the
  mechanical S.A.1 for a guaranteed human check at the substrate freeze — the right trade for a
  Category-A interface that binds S.B and S.C. If S.A.1 lands clean and its unitarity/Bell/GHZ KATs
  pass, the S.A.2 ◆ is where the judgment — is the interface right for two downstream tracks? —
  concentrates.)*
