<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-S continues (S.B — Shor's algorithm for integer factoring)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **kept at the default; the lever-5 opt-down was available
and declined (user-adjudicated at shard time, 2026-06-16).** Unlike S.A, the cost-of-design-error
lever (3) has **dropped**: S.B is **Category-B (algorithm-on-substrate)** — a *consumer* of the now-
frozen simulator, not a substrate. It freezes one contract (**C-ModExp**, the controlled modular-
exponentiation circuit interface) consumed only by **its own** order-finding half (S.B.2); there is
**no two-track propagation** of the kind that held S.A at opus. Correctness-criticality (lever 4) is
moderate (toy pedagogical factoring, caught by the canonical 15/21/35/91 KATs), and **lever 5 is
strong** (Shor factoring has deterministic, published, worked-example KATs — the order of `a` mod
`N`, the period→factor extraction, the 15→3×5 textbook trace). On the levers alone this is **the
textbook lever-5 opt-down case** (`@plan-juncture-sonnet`). It was **declined** because the S.B.2 ◆
ratifies the project's **first end-to-end cryptanalytic break** (Shor factoring runnable on the
simulator) for the first time, and the modular-exponentiation reversible circuit is the **most
intricate single piece in Track S** (the irreducible-complexity FLOOR, lever 2). The differential is
single-digit dollars and the `destructive-HALT` invariant caps the downside either way; opus buys
the strongest adjudicator at a genuine milestone. *(Both per-row tiers are **Sonnet** — S.B carries
no Opus-flagged session per the ROADMAP Opus-flagged table; the ◆ fork pages `@plan-juncture` at
opus.)*

**Scope boundary — S.B is Shor-FOR-FACTORING ONLY, NOT the ECDLP attack (the next-sub-track guard).**
S.B builds the **integer-factoring** instance of Shor's algorithm on the frozen simulator: the
**controlled modular-exponentiation quantum circuit** (`|x⟩|1⟩ → |x⟩|aˣ mod N⟩`), the **order-
finding circuit** (exponent-register superposition → controlled-mod-exp → iQFT → measure), the
**classical period post-processing** (continued-fraction recovery of the order `r`, the
even-order / nontrivial-`gcd` factor extraction with retry), and the **end-to-end `factor(N)`
driver**. It builds **nothing for ECDLP**. Shor-for-ECDLP (S.C) is a *separate* sub-track: the
Proos–Zalka elliptic-curve point-addition circuit, the `rho::curve` group-arithmetic edge, and the
ECDLP period extraction are **S.C**, not S.B. A `@build` agent that implements the Proos–Zalka
construction, reaches into `rho::curve`, or builds an elliptic-curve group-arithmetic circuit in S.B
has reached into S.C — that is defocus. S.B's deliverable is integer factoring of 15/21/35/91 via
simulated order-finding, validated against the canonical worked examples.

The substrate S.B consumes was **frozen and ratified at the S.A.2 ◆** (commit `5ec563a`,
`@plan-juncture/opus` verdict, 2026-06-16). The S.A ◆ digest confirmed the exact surface S.B needs is
present — established four grounding facts (confirmed by re-reading the landed `shor` crate at shard
time, 2026-06-16):

1. **The over-specified gate surface was carried for exactly this consumer.** `shor::gates` exposes
   `multi_controlled_x(sv, controls: &[usize], target)` AND the most-general
   `multi_controlled_unitary(sv, controls, target, u00, u01, u10, u11)`, plus `toffoli`,
   `controlled_phase`, `cnot`, `swap`, and the full single-qubit set. The reversible modular-
   arithmetic circuit (controlled-add-mod → controlled-mult-mod → controlled-exp) is assembled
   **entirely from this frozen surface** — S.B adds **no new gate** to the simulator. *(This is the
   Category-A over-specification the S.A.2 ◆ ratified paying off: S.B builds without re-opening
   S.A, exactly as the lever-3 reasoning predicted.)*

2. **The QFT/iQFT and measurement surfaces are frozen and sufficient.** `shor::qft::qft` /
   `shor::qft::iqft` are in-place over `&mut StateVec`, little-endian-output (the input-bit-reversal
   convention is documented load-bearing in `qft/mod.rs`). `shor::measure` exposes
   `measure_all(sv, rng)`, `measure_all_seeded(sv, seed)`, `sample_counts(sv, n_shots, seed)`
   (seeded `ChaCha8Rng`), returning `MeasureAllOutcome { basis_index, collapsed }`. The order-finding
   circuit applies `iqft` to the exponent register then measures it — both primitives are present.

3. **S.B is a self-contained extension of the `shor` crate — no new dependency, no new cross-crate
   edge.** The factoring targets (15, 21, 35, 91) fit in `u64`; the classical post-processing
   (continued fractions, gcd) is **built fresh in `shor`** (no `continued_fraction` exists workspace-
   wide; `gcd` exists in `shared/bigint` over `BigInt` but plain integer gcd is the honest, dependency-
   free choice at toy scale). `shor/Cargo.toml` already carries `num-complex`, `rand`, `rand_chacha`,
   `proptest`. **S.B adds no workspace member and no dependency** — it adds modules to `shor` and
   touches no existing file outside the crate. *(S.B does NOT touch `rho::curve` — that edge appears
   only at S.C.)*

4. **The KAT corpus for Shor factoring is canonical and self-contained (the lever-5 fact).** Shor
   factoring has deterministic worked-example KATs: the order of `a` mod `N` (e.g. `ord₂(15) = 4`,
   `ord₇(15) = 4`, `ord₂(21) = 6`), the continued-fraction recovery of `r` from a measured phase
   `s/r`, the even-order factor extraction (`gcd(a^{r/2} ± 1, N)`), and the 15→3×5 / 21→3×7 /
   35→5×7 / 91→7×13 end-to-end traces. The classical post-processing is **fully deterministic** and
   the quantum half is **seedable** (the measurement sampler is `ChaCha8Rng`-seeded), so the end-to-
   end KAT is reproducible. No external quantum oracle; no `#[ignore]` gating needed.

The work splits at **one quantum↔classical contract-sharp seam**, **2 sessions** (the ROADMAP
"2 sessions"; the soft seam for an additive S.B.1a/S.B.1b is named below):

1. **S.B.1 — Modular-exponentiation quantum circuit (Sonnet, Cat B).** The reversible controlled
   modular-arithmetic circuit over the frozen register: controlled-add-mod → controlled-mult-mod →
   **controlled modular exponentiation** (`|x⟩|y⟩ → |x⟩|y · aˣ mod N⟩`, controlled on the exponent
   register `x`), assembled from the frozen S.A gate set (`multi_controlled_x` /
   `multi_controlled_unitary` / `toffoli` / `controlled_phase`) on ancilla work-registers. **Freezes
   C-ModExp** (the circuit-builder interface S.B.2's order-finding consumes). The intricate
   reversible-arithmetic FLOOR (lever 2) kept whole.

2. **S.B.2 ◆ — Order-finding + continued-fraction period extraction + end-to-end factoring (Sonnet,
   Cat B→I).** The order-finding circuit orchestration (exponent-register Hadamard superposition →
   the C-ModExp circuit → `iqft` on the exponent register → `measure_all`), the **classical
   continued-fraction post-processing** (recover the order `r` from the measured phase `s/r`), the
   even-order / nontrivial-`gcd` factor extraction with the retry loop, the **end-to-end `factor(N)`
   driver**, and the BENCHMARKS.md `## S.B` section. **Consumes C-ModExp. Freezes C-OrderFind +
   C-Factor.** Crosses the **S.B ◆ boundary** — Shor-for-factoring is complete end-to-end and the
   first cryptanalytic break runs on the simulator.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the Proos–Zalka ECDLP
circuit, reaching into `rho::curve`, or building elliptic-curve group arithmetic — those are S.C, the
deferred consumer; or ancilla-optimizing the mod-exp circuit past what the toy targets need; or
pushing the qubit budget past the ~25-qubit ceiling to factor larger N) and **rigidity** (forcing a
qubit-efficient mod-exp circuit where the honest demonstration uses the straightforward reversible
construction; or skipping the continued-fraction KAT because "the gcd will catch it"; or refusing the
91 ceiling-stress fallback when the ancilla budget genuinely hits the wall — principle 4).

**Scoping discipline.** S.B factors at **demonstration fidelity** (15/21/35/91; the mod-exp circuit
uses the straightforward reversible construction, not a qubit-optimized one — no windowed arithmetic,
no Beauregard-style phase-estimation compression beyond what the targets need; principle 3). The
**principle-4 science↔engineering gap is load-bearing and explicit:** the qubit cost of order-finding
is ~`2n+3+ancilla` for an `n`-bit `N`, so the ~25-qubit simulator ceiling (the frozen S.A wall) is
the **resource-scale** boundary — `N=91` (7-bit) sits near the ceiling and its reachability depends
on S.B.1's mod-exp ancilla budget. The BENCHMARKS.md `## S.B` section records this: the algorithm
demonstrates Shor's *factoring mathematics* correctly at toy scale, and the qubit ceiling is the
engineering-scale wall principle 4 says to annotate, not paper over. *(This is the same posture as
the S.A simulator ceiling and the index-calculus "asymptotic win not observable at toy scale" — the
factoring logic is fully exhibited; the quantum speedup requires real hardware, out of scope by
construction.)*

---

## Purpose (design intent)

Per ROADMAP (Phase ε, S.B): "*S.B — Shor for factoring. 2 sessions. Predecessor: S.A. Modular
exponentiation as a quantum circuit; factor 15, 21, 35, 91. Sonnet.*" And per the design statement
(item 6): "*Shor's algorithm via a classical state-vector simulator, for both factoring and ECDLP.*"

S.B is **the factoring instance of Shor's algorithm** — the first *consumer* of the Track-S
simulator substrate and the project's **first quantum-model cryptanalytic break**. Where every
classical track escapes the generic √n / L-notation bound by finding exploitable *classical*
structure, Shor's algorithm dissolves the bound *entirely* via quantum period-finding: factoring
`N` reduces to finding the **order** `r` of a random `a` mod `N` (the period of `x ↦ aˣ mod N`),
which the QFT extracts in polynomial time. S.B is the machine that runs that reduction end-to-end on
the classical simulator at toy scale.

The deliverable is two-fold (the two conceptual units, each a session):

1. **The modular-exponentiation quantum circuit (S.B.1).** The reversible controlled-`aˣ mod N`
   circuit, assembled from the frozen S.A gate set on ancilla registers — the quantum heart of
   order-finding. The substrate S.B.2's order-finding circuit is built from. **Freezes C-ModExp.**

2. **Order-finding + continued-fraction extraction + end-to-end factoring (S.B.2 ◆).** The order-
   finding circuit orchestration (superposition → mod-exp → iQFT → measure), the classical
   continued-fraction recovery of the order, the even-order factor extraction, and the `factor(N)`
   driver. The complete Shor-factoring algorithm. **Freezes C-OrderFind + C-Factor.**

S.B is **ECDLP-free** (it builds no elliptic-curve circuit — S.C is the consumer), **substrate-
reusing** (it adds no gate to the simulator — the S.A over-specification covers it), and
**principle-4-honest** (the order-finding qubit budget against the ~25-qubit ceiling is annotated as
a resource-scale wall, with `N=91` as the ceiling-stress case). Re-read this intent at the ◆ boundary
to catch defocus (a Proos–Zalka circuit, a `rho::curve` reach, ancilla over-optimization) and
rigidity (forcing qubit efficiency, skipping the continued-fraction KAT, refusing the 91 fallback).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only
`[workspace]` members + a `[profile.bench]`); raw `cargo` is the only CI surface (unchanged since
S.A). S.B.2 may add a benchmark, so the third discovered command applies if so:
`VERIFY_BENCH = cargo bench --no-run` (compile-only — the actual `cargo bench` timings are hand-
transcribed into BENCHMARKS.md, the established pattern). `/run-plan` re-discovers at preflight. S.B
**adds no workspace member and no dependency** — it adds modules to the existing `shor` crate and
touches no file outside it — so the gate is a **shor-crate-grows + new-KATs-green gate**, with the
no-regression invariant trivially held for every existing crate (S.B touches no code outside `shor`):

- **The existing rho / gnfs / shared / shor S.A KATs must stay green** — S.B extends the `shor`
  crate; it changes no existing solver path and no S.A-frozen module surface. `cargo test --workspace`
  is the no-regression guard *and* the new S.B KAT gate (the new `shor` factoring KATs join the run).
- **`cargo check --workspace` must stay green** — S.B adds modules to `shor` using only its existing
  dependencies (`num-complex`, `rand`, `rand_chacha`); no new edge, no new member, no cycle risk.
- **`cargo bench --no-run` compiles any S.B.2 bench** — if S.B.2 adds a `shor/benches/` order-finding
  or qubit-budget bench, it must build; the timings are hand-transcribed (matching the S.A / G.W /
  E.W pattern).
- **KATs are canonical worked-example values, self-contained — no oracle on the green path** — the
  factoring KATs are published orders (`ord₂(15)=4`, etc.), continued-fraction recoveries, factor
  extractions, and the 15/21/35/91 end-to-end traces; the measurement sampler is seeded
  (`ChaCha8Rng`) so the end-to-end run is reproducible. No external quantum simulator is called;
  no live quantum oracle exists for Track S. Any cross-check against a reference simulator is
  `#[ignore]`-gated.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| S.B.1 | Modular-exponentiation quantum circuit | B | Sonnet | C-StateVec (frozen S.A.1 — `StateVec` + the `gates` surface, esp. `multi_controlled_x` / `multi_controlled_unitary` / `toffoli` / `controlled_phase`) | `shor/src/arith/mod.rs` (new: reversible controlled-add-mod / controlled-mult-mod / controlled modular-exponentiation circuit builders), `shor/src/lib.rs` (add `pub mod arith;` + the mod-exp/ancilla doc), `shor/tests/modexp_kat.rs` (new: reversibility + permutation-correctness + ancilla-clean KATs) |
| S.B.2 ◆ | Order-finding + continued-fraction extraction + end-to-end factoring | B→I | Sonnet | C-ModExp (frozen S.B.1 — the controlled-`aˣ mod N` circuit); C-QFT (frozen S.A.2 — `iqft`); `shor::measure` (`measure_all_seeded`) | `shor/src/shor/mod.rs` (new: order-finding circuit orchestration + continued-fraction period extraction + `factor(N)` driver + retry loop), `shor/src/lib.rs` (add `pub mod shor;`), `shor/tests/factor_kat.rs` (new: order KATs + continued-fraction KATs + end-to-end 15/21/35/91 + the 91 ceiling-stress case), `docs/BENCHMARKS.md` (add `## S.B` section: qubit-budget-vs-N table + the principle-4 resource-scale note) |

**Sequencing notes.** Strictly serial: **S.B.1 → S.B.2.** S.B.1 lands the mod-exp circuit and freezes
its interface; S.B.2 consumes it (order-finding orchestration + classical post-processing) and closes
the sub-track. **One `@architect` marker:** the **S.B.2 ◆** (the sub-track-boundary juncture —
ratifying the first end-to-end Shor-factoring break). *(Tradeoff named: S.B pages a juncture only at
the ◆-close, NOT at the open — same as S.A, and unlike the Opus-substrate opens E.B.1/E.F.1/G.A.1.
S.B.1's mod-exp circuit is intricate but its construction is canonical (the standard reversible
modular-arithmetic circuit), so no opening fork is warranted; the integrative judgment — does the
factoring run correctly end-to-end and stay in scope? — concentrates at the ◆ close. The juncture-tier
is `opus`, the declined lever-5 opt-down recorded in the header.)*

**Why 2 sessions (the ROADMAP "2 sessions").** The split is taken at the single quantum↔classical
contract-sharp seam:
- **One-line-commit-title corollary.** "Modular-exponentiation quantum circuit" and "Order-finding +
  continued-fraction extraction + end-to-end factoring" are **two distinct commit titles**. Bundling
  them — "build the mod-exp circuit AND the order-finding orchestration AND the period extraction AND
  the factoring driver" — fails the corollary.
- **Irreducible units kept whole (lever 2).** S.B.1 is the reversible modular-arithmetic circuit (the
  intricate FLOOR — controlled-add-mod through controlled-exp as one conceptual unit); S.B.2 is the
  orchestration + classical-extraction layer. Neither fractures below its floor — splitting the mod-
  exp circuit's arithmetic primitives from the exponentiation that composes them would fracture an
  irreducible unit with no external freeze between (the soft seam, named below); the order-finding +
  continued-fraction + factoring driver cohere as the layer that *uses* the circuit to factor.
- **Contract-sharp boundary.** S.B.1 **freezes** C-ModExp (the circuit-builder interface); S.B.2
  **consumes** it (order-finding wraps the mod-exp circuit in superposition + iQFT + measure) and
  **freezes** C-OrderFind + C-Factor. The order-finding circuit is meaningless without the mod-exp
  freeze — it is the circuit's wrapper.
- **Lower lever 3 + strong lever 5 license the small commits.** S.B freezes contracts consumed only
  within S.B (no two-track propagation), and the factoring KATs are deterministic, canonical, and
  reproducible (seeded measurement), so the inner loop catches drift behaviourally — the condition
  that makes small commits safe.

**The softest seam — could the mod-exp circuit split into arithmetic primitives + exponentiation
(S.B.1a / S.B.1b)?** The reversible modular-arithmetic circuit layers as controlled-add-mod →
controlled-mult-mod → controlled-exp, and a planner could split the primitives (add-mod, mult-mod)
from the exponentiation that composes them. The chosen shard keeps **the whole mod-exp circuit in
S.B.1**, because the primitives and the exponentiation are the **same irreducible reversible-
arithmetic unit over the same ancilla layout** — there is **no contract seam between them** (the
exponentiation consumes the primitives in-crate with no external freeze, and the ancilla-management
convention is shared), so splitting them up front would fracture the FLOOR (lever 2) without buying
an early freeze. **If S.B.1 overruns** (the controlled-add-mod + controlled-mult-mod + controlled-exp
+ their reversibility/ancilla-clean KATs push past the session band — plausible, since reversible
modular arithmetic with clean ancilla management is the most intricate construction in Track S), the
escape applies: **split at the arithmetic↔exponentiation layer** (controlled-add-mod +
controlled-mult-mod freezing C-ModArith in S.B.1a; controlled-exp freezing C-ModExp in S.B.1b) — an
additive-reshard surfaced at the S.B.1 ◆ readout or by S.B.1 once the circuit's true size is concrete,
never a silent overrun. This is the one place the 2-vs-3 sizing is genuinely uncertain until the
mod-exp circuit's ancilla budget and reversibility-KAT size are visible.

---

## Session detail

S.B.1 is specified at near-full fidelity (the reversible modular-arithmetic circuit is a canonical
construction; the design choices are the ancilla layout, the controlled-arithmetic decomposition, and
the C-ModExp interface shape, resolved below). S.B.2 is specified at the structural level (the order-
finding + continued-fraction + factoring outline) with the per-piece content sketched — correct per
the substrate-first discipline: the order-finding circuit's exact qubit budget and the 91-target
reachability are crisp only after the mod-exp circuit's ancilla count freezes.

### S.B.1 — Modular-exponentiation quantum circuit (Sonnet, Cat B)

**Deliverable:** the reversible controlled modular-exponentiation circuit — the quantum heart of
order-finding — built entirely from the frozen S.A gate set. The pieces:
- **The reversible controlled modular-arithmetic primitives** (`shor/src/arith/mod.rs`, new):
  controlled-add-mod (`|x⟩ → |x + c mod N⟩` controlled on a qubit) and controlled-mult-mod
  (`|x⟩ → |c · x mod N⟩` controlled) on ancilla work-registers, assembled from `multi_controlled_x`,
  `multi_controlled_unitary`, `toffoli`, `controlled_phase`, and the single-qubit set — the standard
  reversible-arithmetic decomposition (a quantum adder + modular reduction, or a phase-estimation-
  style adder if simpler at toy scale; the construction choice is documented). **Ancilla discipline:
  every borrowed ancilla is returned to `|0⟩`** (the circuit is a clean permutation on the work-
  register; uncomputation restores ancillas) — the load-bearing reversibility invariant.
- **The controlled modular-exponentiation circuit** (`shor/src/arith/mod.rs`): `|x⟩|y⟩ →
  |x⟩|y · aˣ mod N⟩`, controlled on the exponent register `x` — repeated controlled-mult-mod by
  `a^{2ᵏ} mod N` (the classically-precomputed powers), one per exponent-register qubit. This is the
  circuit order-finding applies to a uniform superposition over `x`.
- **The C-ModExp interface**: a circuit-builder that, given `(a, N, exponent register, work register,
  ancilla register)`, applies the controlled-`aˣ mod N` operation in-place on a `&mut StateVec` —
  little-endian throughout (the frozen S.A convention), with the register layout (which qubits are
  exponent / work / ancilla) documented as part of the freeze.

Consumes C-StateVec (frozen S.A.1 — `StateVec` + the `gates` surface). Reads the S.A.2 ◆ digest (the
multi-controlled surface was over-specified for this consumer). **Freezes C-ModExp.**

**KAT:** over reversibility + permutation-correctness invariants (no quantum-superposition needed for
S.B.1's own KATs — the circuit is tested on basis states, where it computes a classical permutation):
(1) **permutation correctness** — `controlled-mult-mod` by `c` on basis state `|x⟩` (control set)
gives `|c · x mod N⟩` for the classically-computed product, across a fixture of `(c, x, N)` triples;
(2) **modular exponentiation correctness** — the controlled-`aˣ mod N` circuit on `|x⟩|1⟩` (control
set) gives `|x⟩|aˣ mod N⟩` for a fixture of `(a, x, N)`; (3) **reversibility** — the circuit followed
by its inverse is the identity on basis states; (4) **ancilla-clean** — every ancilla returns to
`|0⟩` after the circuit (a basis-state check on the ancilla register); (5) **control-off no-op** —
with the control qubit `|0⟩`, the circuit is the identity. The KATs follow the `shor/tests/*_kat.rs`
idiom: a per-fixture helper, many `#[test]` one-liners, classical-value comparison (the circuit on a
basis state is a classical permutation, so comparison is exact, not ε-tolerance — a simplification
over S.A's amplitude KATs). **Verify gate:** `cargo test --workspace` green (the new `modexp` KATs +
no regression); `cargo check --workspace` green (no new edge); `cargo bench --no-run` iff a bench
lands.

**Subtlety (load-bearing):** (1) **Ancilla discipline is the reversibility FLOOR** — every borrowed
ancilla MUST be uncomputed back to `|0⟩`; a circuit that leaves ancillas entangled with the work
register corrupts the order-finding interference and silently produces wrong periods. This is the
single hardest correctness point in S.B and the reason the mod-exp circuit is one irreducible unit.
(2) **Basis-state KATs suffice for S.B.1** — on a basis state the controlled-mod-exp circuit computes
a classical permutation, so the KATs are exact classical-value checks (no superposition, no ε); the
superposition behaviour is exercised in S.B.2's order-finding. (3) **Little-endian throughout** — the
exponent, work, and ancilla registers all use the frozen S.A little-endian convention; the register-
layout map (which qubit index is which) is part of the C-ModExp freeze (a silent layout flip is a
wrong-answer bug for S.B.2). (4) **Demonstration-fidelity circuit, not qubit-optimized** — use the
straightforward reversible construction; no windowed arithmetic, no Beauregard phase-estimation
compression beyond what 15/21/35/91 need (principle 3). The ancilla budget that results is the input
to S.B.2's 91-target reachability check. (5) **Classically-precompute the powers `a^{2ᵏ} mod N`** —
these are classical constants fed to the controlled-mult-mod stages, not computed in the circuit.

**Deferred:** the order-finding circuit orchestration (S.B.2); the continued-fraction post-processing
(S.B.2); the `factor(N)` driver (S.B.2); the BENCHMARKS.md `## S.B` section (S.B.2 — needs the qubit-
budget numbers); any ECDLP circuit (S.C — the deferred consumer).

### S.B.2 ◆ — Order-finding + continued-fraction extraction + end-to-end factoring (Sonnet, Cat B→I)

**Deliverable:** the layer that *uses* the mod-exp circuit to factor — the order-finding circuit, the
classical period extraction, and the `factor(N)` driver — + the BENCHMARKS.md section + the sub-track
◆ close. Structural-fidelity sketch (the per-piece content is crisp once the mod-exp ancilla budget
freezes). The pieces:
- **The order-finding circuit** (`shor/src/shor/mod.rs`, new): prepare the exponent register in
  uniform superposition (`h` on every exponent qubit), apply the C-ModExp circuit (`|x⟩|1⟩ →
  |x⟩|aˣ mod N⟩`), apply `iqft` (frozen S.A.2) to the exponent register, then `measure_all_seeded`
  (frozen S.A measure) the exponent register. Returns a measured value `s` approximating `s ≈ k·2ᵗ/r`
  for register size `t`. The exponent-register size is `t ≈ 2·⌈log₂N⌉` (the standard choice for clean
  continued-fraction recovery).
- **The classical continued-fraction period extraction** (`shor/src/shor/mod.rs`): expand the
  measured phase `s/2ᵗ` as a continued fraction, take convergents with denominator `< N`, and recover
  the candidate order `r` (the smallest denominator whose `aʳ ≡ 1 mod N`). Built fresh (no
  `continued_fraction` exists workspace-wide; plain-integer arithmetic at toy scale).
- **The `factor(N)` driver + retry loop** (`shor/src/shor/mod.rs`): pick a random `a` coprime to `N`
  (seeded), run order-finding, recover `r`; if `r` is even and `a^{r/2} ≢ -1 mod N`, return
  `gcd(a^{r/2} ± 1, N)` (a nontrivial factor); else retry with a new `a` (the standard Shor classical
  wrapper). Handle the classical short-circuits (even `N`, `N` a prime power) per the textbook
  algorithm.
- **The `## S.B` BENCHMARKS.md section** (`docs/BENCHMARKS.md`, append): a **qubit-budget-vs-N table**
  (exponent + work + ancilla qubits required to factor 15/21/35/91, against the ~25-qubit ceiling) +
  the **principle-4 resource-scale note** (the algorithm demonstrates Shor's *factoring mathematics*
  correctly; the qubit ceiling is the engineering/resource wall, with `N=91` as the ceiling-stress
  case — the same posture as the S.A simulator ceiling and the index-calculus annotation). Matches
  the per-sub-track BENCHMARKS.md genre (prose setup + table + science↔engineering note).
- **The S.B ◆ close**: re-read the Purpose intent; verify Shor-for-factoring runs end-to-end on the
  simulator (15/21/35/91, with 91 as the ceiling-stress case); confirm the principle-4 qubit-budget
  ceiling is annotated; confirm S.B stayed ECDLP-free.

Consumes C-ModExp (frozen S.B.1), C-QFT (`iqft`, frozen S.A.2), `shor::measure`. **Freezes
C-OrderFind + C-Factor.**

**KAT:** (1) **order KATs** — the classical order `ord_a(N)` matches the published value
(`ord₂(15)=4`, `ord₇(15)=4`, `ord₂(21)=6`, `ord₂(35)=12`, etc.); (2) **continued-fraction KATs** —
a known measured phase `s/2ᵗ` recovers the correct order `r` via the convergents (a deterministic
classical KAT, no quantum needed); (3) **end-to-end factoring** — `factor(15)→{3,5}`, `factor(21)→
{3,7}`, `factor(35)→{5,7}`, with a **fixed measurement seed** so the run is reproducible; (4) **the
91 ceiling-stress KAT** — `factor(91)→{7,13}` **if** the qubit budget fits under the ceiling, **else**
a documented principle-4 fallback (the test asserts the qubit budget exceeds the ceiling and the
BENCHMARKS.md note records 91 as beyond the toy demonstration — the reachability decided once S.B.1's
ancilla count is concrete). The KATs follow the `shor/tests/*_kat.rs` idiom (per-fixture helper, many
`#[test]` one-liners, seeded measurement for reproducibility, published values in comments).
**Verify gate:** `cargo test --workspace` green (the new KATs + no regression); `cargo check
--workspace` green; `cargo bench --no-run` compiles any qubit-budget bench.

**Subtlety (load-bearing):** (1) **The 91 target is the principle-4 ceiling-stress case** — its
reachability depends on S.B.1's mod-exp ancilla budget (`N=91` is 7-bit, needing ~`2·7 + 7 + ancilla`
qubits, near the ~25-qubit wall). If it fits, KAT it; if it does not, the **documented fallback**
applies (assert the budget exceeds the ceiling, annotate 91 as beyond the toy demonstration in
BENCHMARKS.md) — surfaced at the S.B.1 ◆ readout once the ancilla count is concrete. Do NOT ancilla-
optimize the mod-exp circuit to force 91 (gold-plating a toy circuit — principle 3). (2) **Seeded
measurement for reproducible end-to-end KATs** — the order-finding measurement is `ChaCha8Rng`-seeded
(frozen S.A `measure_all_seeded`), and the `factor(N)` driver's random-`a` choice is seeded, so the
end-to-end factoring KAT is deterministic; pick seeds that land a successful (even-order, nontrivial-
gcd) run so the KAT is a clean pass, and document the retry-loop behaviour separately. (3) **The
continued-fraction recovery is the classical period-extraction crux** — its KAT (known phase → known
order) must be complete, not a smoke test; it is the piece that turns a noisy measurement into the
order `r`. (4) **Exponent-register size `t ≈ 2⌈log₂N⌉`** — too small and the continued-fraction
recovery fails (the phase resolution is insufficient); this is the standard choice and a documented
parameter. (5) **The qubit ceiling is principle-4 (resource scale)** — the BENCHMARKS.md note records
the order-finding qubit budget against the ~25-qubit wall as engineering scale, not mathematical
omission.

**Deferred:** Shor-for-ECDLP (S.C — the Proos–Zalka circuit, the `rho::curve` group-arithmetic edge,
the ECDLP period extraction); the post-quantum writeup (S.D) + T.S (the ch. 11 math chapter, paired
with S.D); all of Track ζ (umbrella) + τ-bind. **S.B is the factoring attack; S.C is the ECDLP
attack — a separate sub-track over the same simulator substrate.**

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
S.B.2 ◆ to confirm: (1) Shor-for-factoring runs end-to-end — the mod-exp circuit + order-finding +
continued-fraction extraction + `factor(N)` driver compose correctly and the 15/21/35 KATs pass with
their fixed seeds; (2) **the mod-exp circuit is reversible and ancilla-clean** — the load-bearing
correctness FLOOR (entangled ancillas would silently corrupt the order-finding interference); confirm
the reversibility + ancilla-clean KATs pass; (3) **the 91 ceiling-stress case is resolved** — either
`factor(91)→{7,13}` passes within the qubit budget, OR the principle-4 fallback is in place
(documented as beyond the toy ceiling), and the mod-exp circuit was NOT ancilla-optimized to force it
(principle 3); (4) the principle-4 qubit-budget ceiling is annotated in BENCHMARKS.md `## S.B` (the
order-finding qubit cost vs `N`, against the ~25-qubit wall); (5) C-ModExp + C-OrderFind + C-Factor
are coherent (S.B's own internal contracts — there is no downstream consumer of these, unlike S.A's
substrate); (6) S.B stayed in scope — **no Proos–Zalka circuit, no `rho::curve` reach, no elliptic-
curve group arithmetic** (those are S.C). **Also: surface the outstanding static-frame ROADMAP debt**
(the Progress/Remaining reconciliation owed since the E.W ◆ and again flagged at the S.A ◆ — the
ROADMAP write was out of `@architect` PLAN-write scope at both prior junctures; the S.B close is a
third reconciliation prompt: Phase δ complete, S.A complete, S.B now landing) — **note it as a capture
candidate, not a PLAN edit** (the ROADMAP write remains out of `@architect` PLAN-write scope). One-shot
findings; does not implement. Held at **opus** per the header (the declined lever-5 opt-down).

---

## Cross-session contracts

S.B **freezes three** contracts (C-ModExp at S.B.1; C-OrderFind + C-Factor at S.B.2 ◆) and **amends
no prior frozen contract** — it extends the `shor` crate, consuming the S.A-frozen C-StateVec / C-QFT
/ measure surfaces without modifying them. S.B adds no workspace member and no dependency. **Unlike
S.A, S.B's three new contracts have no downstream consumer** — they are internal to S.B (C-ModExp is
consumed by S.B.2; C-OrderFind + C-Factor are the sub-track's terminal deliverable). This is the
lever-3 drop that made the juncture-tier opt-down *available* (declined per the header).

### C-ModExp — the controlled modular-exponentiation circuit (compiler-/test-enforced) — *to be frozen at S.B.1*

**Defined in:** S.B.1 (`shor/src/arith/mod.rs`).
**Consumed by:** S.B.2 (the order-finding circuit wraps the mod-exp circuit in superposition + iQFT +
measure). **No downstream sub-track consumes C-ModExp** — S.C builds its own elliptic-curve point-
addition circuit (the Proos–Zalka construction), not the integer mod-exp circuit. Compiler-enforced
(the circuit-builder function signature) + test-enforced (the permutation-correctness + reversibility
+ ancilla-clean KATs).

**Ratified shape (to be confirmed at the S.B ◆).** A circuit-builder that applies the controlled
`|x⟩|y⟩ → |x⟩|y · aˣ mod N⟩` operation in-place on a `&mut StateVec`, given `(a, N)` and the register
layout (exponent / work / ancilla qubit ranges) — **little-endian throughout** (the frozen S.A
convention). Assembled from the frozen S.A gate set (`multi_controlled_x`, `multi_controlled_unitary`,
`toffoli`, `controlled_phase`, single-qubit) — **no new gate added to the simulator**. **Invariants:**
the circuit is a reversible permutation on the work register (tested on basis states, exact); **every
ancilla returns to `|0⟩`** (the ancilla-clean invariant — an entangled ancilla silently corrupts the
order-finding interference); control-off is a no-op; the register-layout map is fixed and documented
(a silent layout flip is a wrong-answer bug for S.B.2). The classically-precomputed powers
`a^{2ᵏ} mod N` are constants fed to the controlled-mult-mod stages. *(The soft-seam candidate: if
S.B.1 overruns, this splits into C-ModArith at S.B.1a + C-ModExp at S.B.1b.)*

### C-OrderFind — the order-finding circuit + period extraction (compiler-/test-enforced) — *to be frozen at S.B.2 ◆*

**Defined in:** S.B.2 (`shor/src/shor/mod.rs`). **Consumed by:** the `factor(N)` driver (C-Factor,
same session). **No downstream sub-track consumes it** (S.C builds its own ECDLP period extraction).
Compiler-enforced (the order-finding + continued-fraction function signatures) + test-enforced (the
order KATs + continued-fraction KATs).

**Ratified shape (to be confirmed at the S.B ◆).** The order-finding routine: exponent-register
superposition (size `t ≈ 2⌈log₂N⌉`) → the C-ModExp circuit → `iqft` (frozen S.A.2) →
`measure_all_seeded` (frozen S.A) the exponent register, returning a measured `s`; plus the classical
continued-fraction recovery of the order `r` from `s/2ᵗ` (convergents with denominator `< N`).
**Invariants:** the recovered `r` satisfies `aʳ ≡ 1 mod N` (the order definition, a KAT); the
exponent-register size `t` is large enough for clean continued-fraction recovery (the standard
`2⌈log₂N⌉` choice); the measurement is seeded (reproducible KATs).

### C-Factor — the end-to-end `factor(N)` driver (compiler-/test-enforced) — *to be frozen at S.B.2 ◆*

**Defined in:** S.B.2 (`shor/src/shor/mod.rs`). **Consumed by:** the end-to-end factoring KATs (the
sub-track's terminal deliverable — no further consumer). Compiler-enforced (the `factor` signature) +
test-enforced (the 15/21/35/91 end-to-end KATs).

**Ratified shape (to be confirmed at the S.B ◆).** `factor(N, seed) -> Option<(factor, factor)>`:
the classical Shor wrapper — pick a random `a` coprime to `N` (seeded), run order-finding (C-OrderFind)
to recover `r`, and if `r` is even with `a^{r/2} ≢ -1 mod N` return `gcd(a^{r/2} ± 1, N)`; else retry;
with the classical short-circuits (even `N`, prime-power `N`) per the textbook algorithm.
**Invariants:** `factor(15)→{3,5}`, `factor(21)→{3,7}`, `factor(35)→{5,7}` with fixed seeds (the
end-to-end KATs); `factor(91)→{7,13}` **iff** the qubit budget fits the ceiling, **else** the
documented principle-4 fallback; the returned factors are nontrivial divisors of `N`. *(The 91 case
is the principle-4 ceiling-stress boundary — resolved once S.B.1's ancilla budget is concrete.)*

### Frozen contracts read by S.B (consumed, not amended)

- **C-StateVec (frozen S.A.1) — the dense register + gate interface.** S.B.1's mod-exp circuit is
  assembled entirely from the `gates` surface (esp. `multi_controlled_x` / `multi_controlled_unitary`
  / `toffoli` / `controlled_phase`); the over-specification the S.A.2 ◆ ratified covers it. **Not
  amended** — S.B adds no gate.
- **C-QFT (frozen S.A.2) — `qft`/`iqft`.** S.B.2's order-finding applies `iqft` to the exponent
  register. **Not amended.**
- **`shor::measure` (frozen S.A.2) — `measure_all_seeded`, `MeasureAllOutcome`.** S.B.2's order-
  finding measures the exponent register, seeded for reproducibility. **Not amended.**
- **C-Sparse (frozen S.A.2) — NOT consumed by S.B.** The order-finding state is fully superposed
  (a Hadamard on every exponent qubit), so the dense register is the honest vehicle; the sparse path
  offers no win here (the principle-4 state-dependence the S.A.2 ◆ annotated). S.B uses the dense
  register.

### Downstream contracts S.B does NOT produce (named, to bound scope)

- **The ECDLP circuit contracts are S.C, not S.B.** S.C (Shor-for-ECDLP) builds the Proos–Zalka
  elliptic-curve point-addition circuit, consumes **both** C-StateVec/C-QFT (this substrate) **and**
  the frozen `rho::curve` surface (`AffinePoint`, `Curve`, `scalar_mul`, `secp_k1_toy` — confirmed
  present and public at `rho/src/curve/`), and freezes its own ECDLP order-finding + period-extraction
  contracts. **S.B touches none of this** — it is named here only to bound S.B's scope (the
  next-sub-track guard) and to remind the S.C shard (later) that the `rho::curve` cross-track edge
  exists.

### Workspace edges (no new member, no new dependency)

- **No new member.** S.B extends the existing `shor` crate; the workspace `Cargo.toml` `members` list
  is unchanged.
- **No new dependency.** S.B uses `shor`'s existing dependencies (`num-complex`, `rand`,
  `rand_chacha`); the classical post-processing (continued fractions, gcd) is built fresh in `shor`
  at toy-integer scale (no `continued_fraction` exists workspace-wide; plain-integer gcd suffices —
  `shared::bigint::gcd` over `BigInt` is the wrong granularity for `u64` toy targets). `cargo check
  --workspace` stays green with no edge change. *(S.B depends on no workspace crate outside `shor` —
  the factoring attack is self-contained. The `rho::curve` edge appears only at S.C.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The S.B.2 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| S.B.1 | Modular-exponentiation quantum circuit | done | 60aa816 | C-ModExp |
| S.B.2 ◆ | Order-finding + continued-fraction extraction + end-to-end factoring | pending | | C-OrderFind, C-Factor |

Contracts frozen before this sub-track: the entire classical-attack arc — all of Track G (GNFS
factoring), Track D (NFS-DL), and Track E (algebraic ECDLP, closed at the E.W ◆) — plus the shared
substrate (C1 smoothness, the field/bigint/numfield/padic/gf2m crates), the Track-τ register
(C-Textbook, frozen T.0), **and the Track-S simulator substrate (C-StateVec, C-Sparse, C-QFT, frozen
at the S.A.2 ◆, commit `5ec563a`)**. **S.B consumes the Track-S substrate (C-StateVec, C-QFT, measure)
and amends none of it** — it is a self-contained extension of the `shor` crate. This sub-track
**freezes three new contracts** (C-ModExp, C-OrderFind, C-Factor), **all internal to S.B** (no
downstream sub-track consumes them — S.C builds its own ECDLP circuit). **With the S.B ◆, Shor-for-
factoring is complete end-to-end (the project's first quantum-model cryptanalytic break); S.C (Shor
ECDLP) and S.D (the PQ writeup + T.S) remain in Phase ε, then Phase ζ (umbrella) + the τ-bind close
the project.**

---

## Action-frame digest

### S.B.1 — 2026-06-17
Discovery/flex: Implementation used direct permutation synthesis (ancilla-free, Gray-code transpositions) rather than the standard ripple-carry adder approach — qubit budget is t+n (no ancilla register), significantly lower than the ~2n+3+ancilla estimate in the PLAN.
Affected: C-ModExp (frozen), qubit budget for S.B.2's 91-target reachability check
Deferred: no — 91 is well within budget (14 qubits total for N=91, t=7); S.B.2 should KAT factor(91)→{7,13} directly (not the fallback path). The ModExpLayout::standard(N, t) interface is the frozen C-ModExp surface.
Texture: All 5 KAT categories pass (41 tests). The ancilla-free approach means the "ancilla-clean" KAT verifies the full state vector is restored (forward + inverse = identity), which is a stronger check than a separate ancilla register check.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **S.B is a self-contained extension of the frozen `shor` crate — internal-continue (confirmed by
  the S.A.2 ◆ ratification + shard-time re-read).** S.B consumes the S.A-frozen substrate
  (C-StateVec / C-QFT / measure) and amends none of it; all S.B code is new modules in `shor`
  (`arith`, `shor`) + two `lib.rs` `mod` decls + new test files. No new crate, no new dependency, no
  edit outside `shor`. A discovery that the mod-exp circuit needs a gate the simulator lacks is an
  **additive edge to S.A** surfaced at the ◆ — but the S.A.2 ◆ over-specification was ratified for
  exactly this consumer, so this is judged unlikely.

- **The mod-exp circuit is the irreducible-complexity FLOOR (lever 2) — keep it whole.** Reversible
  modular arithmetic with clean ancilla management is the most intricate construction in Track S;
  splitting its arithmetic primitives from the exponentiation that composes them fractures the unit
  with no contract seam between (the named soft seam). **Internal-continue → keep S.B.1 whole;
  additive-reshard to S.B.1a/S.B.1b only if it overruns the band.**

- **Ancilla-clean reversibility is the load-bearing correctness invariant.** Every borrowed ancilla
  MUST be uncomputed to `|0⟩`; an entangled ancilla silently corrupts the order-finding interference
  and produces wrong periods with no error. The reversibility + ancilla-clean KATs (basis-state
  checks) are the guard. **Internal-continue → ancilla discipline enforced + KAT'd at S.B.1.**

- **The 91 target may exceed the ~25-qubit ceiling — resolve at the S.B.1 ◆ readout (principle 4).**
  `N=91` (7-bit) needs ~`2·7 + 7 + ancilla` qubits, near the wall; reachability depends on S.B.1's
  mod-exp ancilla budget. **If it fits, KAT `factor(91)→{7,13}`; if not, the documented principle-4
  fallback** (assert the budget exceeds the ceiling, annotate 91 as beyond the toy demonstration in
  BENCHMARKS.md). Do NOT ancilla-optimize the circuit to force 91 (gold-plating — principle 3).
  **Internal-continue → 15/21/35 are the core KATs; 91 is ceiling-stress with a documented fallback.**

- **The qubit budget is principle-4 (resource scale, not mathematical).** Order-finding costs
  ~`2n+3+ancilla` qubits for an `n`-bit `N`; the ~25-qubit ceiling is the simulator's resource wall
  (the same `2^n` wall S.A annotated). The BENCHMARKS.md `## S.B` note records the qubit budget vs
  `N` as the engineering-scale boundary (the index-calculus "asymptotic win not observable" analogue),
  not a mathematical omission. A shard that hides the ceiling or over-engineers to push it is wrong.
  **Internal-continue → qubit-budget table + ceiling annotated.**

- **The mod-exp circuit is demonstration-fidelity, not qubit-optimized (principle 3).** Use the
  straightforward reversible construction; no windowed arithmetic, no Beauregard phase-estimation
  compression beyond what 15/21/35/91 need. Optimizing the toy circuit to factor larger N is gold-
  plating. **Internal-continue → straightforward construction; the resulting ancilla budget is the
  91-reachability input.**

- **No ECDLP circuit / no `rho::curve` reach / no Proos–Zalka in S.B (defocus / scope clarity — the
  deferred consumer).** Shor-for-ECDLP (S.C) builds the Proos–Zalka elliptic-curve point-addition
  circuit and consumes the frozen `rho::curve` surface — **that is a separate sub-track**. A `@build`
  agent that builds an elliptic-curve group-arithmetic circuit, reaches into `rho::curve`, or
  implements the Proos–Zalka construction in S.B has reached into S.C. **S.B is the factoring attack,
  not the ECDLP attack.**

- **Seeded measurement makes the end-to-end factoring KAT reproducible.** The order-finding
  measurement (`measure_all_seeded`) and the `factor(N)` random-`a` choice are seeded; pick seeds
  that land a successful (even-order, nontrivial-gcd) run so the KAT is a clean pass, and document
  the retry-loop behaviour separately. **Internal-continue → fixed-seed end-to-end KATs.**

- **Static-frame ROADMAP debt (surface at the S.B ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried from the E.W ◆, flagged again at the S.A ◆, now compounded by S.B's landing.**
  The ROADMAP Progress table still shows "δ — Algebraic ECDLP (E) … in progress" and "ε — Shor + PQ
  (S) … not started / S.A sharded not executed" — but Track E closed at the E.W ◆ (Phase δ complete),
  S.A landed and ratified at its ◆ (commit `5ec563a`), and Track S has now advanced to S.B. The S.A ◆
  digest named this; the write was deferred (out of `@architect` PLAN-write scope). **The S.B close is
  a third natural prompt** — Phase δ complete, S.A done, S.B done. The full reconciliation (Track E
  Done → E.A–E.W; mark Phase δ complete; mark S.A done; advance Phase ε to S.B) is owed. **This is a
  ROADMAP write — outside the `@architect` PLAN-only write scope; surfaced here as a capture candidate
  for the user to action (via `/note` or a ROADMAP edit), not a PLAN edit.** Not an implementation
  concern; does not block S.B.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase ε — S.B, "*Shor for factoring. 2 sessions. Predecessor: S.A. Modular
  exponentiation as a quantum circuit; factor 15, 21, 35, 91. Sonnet.*"; the design statement item 6
  — "*Shor's algorithm via a classical state-vector simulator, for both factoring and ECDLP*"; the
  Track-τ pairing — T.S/ch. 11 pairs with **S.D**, not S.B) and this PLAN before any session. **NOTE:
  the ROADMAP Progress / Remaining tables are stale (Track E shown in progress though it closed at the
  E.W ◆; Track S shown not-started/S.A-not-executed though S.A landed at `5ec563a` and S.B now
  follows); the S.B close is a third reconciliation prompt — surface it at the ◆, but it is outside
  `@architect` PLAN-write scope (a capture candidate for the user).**
- Read the **frozen substrate to consume**: `shor/src/statevec/mod.rs` + `shor/src/gates/mod.rs`
  (C-StateVec — the `StateVec` type + the gate surface, esp. `multi_controlled_x` /
  `multi_controlled_unitary` / `toffoli` / `controlled_phase`); `shor/src/qft/mod.rs` (C-QFT — `iqft`,
  the load-bearing bit-reversal convention); `shor/src/measure/mod.rs` (`measure_all_seeded`,
  `MeasureAllOutcome`, the `ChaCha8Rng` seeding). Read the **templates to mirror**:
  `shor/tests/qft_kat.rs` + `shor/tests/statevec_kat.rs` (the `*_kat.rs` idiom — a per-fixture helper,
  many `#[test]` one-liners, published values in comments); `docs/BENCHMARKS.md ## S.A` (the per-sub-
  track section genre — prose setup + table + "science↔engineering note (principle 4)", here adapted
  to a qubit-budget-vs-N table).
- **Register:** S.B is **Rust library + benchmark code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; new
  modules in the `shor` crate + their KATs + an optional Criterion bench) **plus prose** (the
  BENCHMARKS.md `## S.B` section). **No PEDAGOGY.md or MATHEMATICS.md chapter at S.B** — the Track-S
  math chapter (T.S, ch. 11) pairs with S.D (the track closeout), not this attack sub-track.
- **Tier routing:** **both S.B.1 and S.B.2 are Sonnet `@build`** (S.B carries no Opus-flagged session
  per the ROADMAP Opus-flagged table — the mod-exp circuit is a canonical construction, order-finding
  is the textbook Shor algorithm; the only judgment is the C-ModExp interface shape + ancilla layout,
  which the ◆ juncture handles). **juncture-tier (header) is `opus`** — the **lever-5 opt-down was
  available and declined** (lever 3 dropped vs S.A: S.B binds no downstream track, freezing C-ModExp
  for its own consumption only; strong quantum-factoring KATs + moderate criticality would license
  `sonnet`, but the first-cryptanalytic-break milestone + the intricate mod-exp FLOOR were judged to
  warrant the strongest adjudicator — user-adjudicated at shard time). The ◆ fork pages
  `@plan-juncture` at opus.
- **Invariants to preserve:** **S.B builds NO ECDLP circuit** (no Proos–Zalka, no `rho::curve` reach,
  no elliptic-curve group arithmetic — those are S.C, the deferred consumer). **S.B amends NO frozen
  contract** (it consumes the S.A substrate read-only; the only new files are `shor` modules + tests +
  the BENCHMARKS.md section). **S.B adds NO gate to the simulator** (the mod-exp circuit is assembled
  from the over-specified frozen gate set). **The mod-exp circuit is reversible and ancilla-clean**
  (every ancilla returns to `|0⟩` — the load-bearing correctness FLOOR; an entangled ancilla silently
  corrupts order-finding). **The register layout (exponent/work/ancilla, little-endian) is fixed and
  documented** (a silent layout flip is a wrong-answer bug). **The mod-exp circuit is demonstration-
  fidelity** (straightforward reversible construction, not qubit-optimized — principle 3). **The
  qubit budget vs the ~25-qubit ceiling is annotated** (principle 4 — engineering, not mathematical;
  91 is the ceiling-stress case with a documented fallback). **Measurement is seeded** (reproducible
  end-to-end KATs).
- **No new crate, no new dependency, all in `shor` (load-bearing for S.B).** S.B extends the existing
  `shor` crate with two new modules (`arith`, `shor`); the workspace `Cargo.toml` and `shor/Cargo.toml`
  dependency lists are unchanged. No existing crate's edge changes; `cargo check --workspace` stays
  green; the no-regression invariant (existing KATs green) holds trivially since no existing code
  changes.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  quantum-circuit session + an orchestration/post-processing session closing at the ◆) is **new for
  this project as a Shor-attack opener** (the first cryptanalytic break in the quantum model), and the
  S.B.2 ◆ confirms an end-to-end factoring break for the first time, so the conservative default is to
  halt at the S.B.2 ◆ for the human glance + the opus juncture fork. Both sessions are Sonnet and the
  constructions are canonical (textbook reversible mod-exp, textbook order-finding), so S.B.1 could
  run autonomously, but the unproven-opener + first-break milestone argues for `halt-at-boundaries` on
  the first invocation; the S.B.2 ◆ fork is itself a halt. *(Tradeoff vs autonomous: `halt-at-
  boundaries` trades a little velocity on the mechanical S.B.1 for a guaranteed human check at the
  first end-to-end break + the 91-ceiling resolution — the right trade for the project's first
  quantum-model cryptanalytic break. If S.B.1 lands clean and its reversibility/ancilla-clean KATs
  pass, S.B.2 can be dispatched immediately after the ◆ glance.)*
