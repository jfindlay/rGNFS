<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E close (E.W — cross-attack benchmarks + the Track-E writeup, paired with T.E, the algebraic-ECDLP textbook chapter)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **set by the ROADMAP's native Opus flag on E.W ("high
integrative judgment load") + the T.E MOV-payoff Opus designation + lever 4 (the Track-E ◆ closeout
adjudicates the whole E.A–E.K arc).** E.W is the Track-E *integrative* close: it does not add an
attack, it *synthesises* the eight attacks the track shipped (Pollard rho, Pohlig–Hellman, MOV,
Smart–Satoh–Araki, GHS descent, Semaev, index calculus) into one comparative picture and verifies the
project's three scoping principles against the realised Track-E implementation. Lever 5 is **weak
here** (unlike every prior E sub-track): the deliverable is a benchmark table + prose synthesis +
a maths chapter, and prose has no fast self-checking oracle — the benchmark KATs are deterministic
(they confirm each attack still solves its instance) but they do not adjudicate whether the *synthesis*
is right. So lever 5 grants **no license to opt the juncture down**; levers 3/4 (the closeout's
cross-arc judgment + the MOV-payoff proof being a designated pedagogical climax) hold it at `opus`.
*(The ◆ fork pages `@plan-juncture` at opus; E.W.2's per-row tier is **Opus** per the ROADMAP —
E.W.1, the benchmark harness, is mechanical Sonnet bench-wiring over frozen solver APIs with no design
crux.)*

**Scope boundary — E.W is documentation + benchmarking, NOT a new attack (user-adjudicated at shard
time, 2026-06-16).** E.W is the **Track-E closeout**: the cross-attack benchmark table ("which attack
wins on which curve"), the Track-E code-tour in `docs/PEDAGOGY.md`, and **T.E** — the maths-first
algebraic-ECDLP chapter in `docs/MATHEMATICS.md` (ch. 10), paired with the E.W writeup per the Track-τ
contract (C-Textbook). It **amends no algorithm contract** — it reads the frozen solver surfaces
(`rho::ecdlp`, `rho::ecdlp::pohlig`, `rho::pairing::mov`, `rho::ssa`, `rho::ghs`, `rho::index_calculus`)
and writes benchmarks + prose. The only code change is **additive Criterion benches** under
`rho/benches/` + their `[[bench]]` manifest entries (criterion is already a `rho` dev-dependency — no
new dependency, no new workspace edge). *(Tradeoff named, load-bearing: the "which attack wins on which
curve" table is NOT a uniform timing race — the attacks are **structural-precondition-conditional**,
each applicable only on the curve whose structure it exploits. The table's pedagogical point is exactly
that conditionality — see the load-bearing finding below. A `@build` agent that tries to race all eight
attacks on one common curve produces a wrong table.)*

The substrate survey (forked `@explore`, 2026-06-16) established the attack-surface map and surfaced
four load-bearing findings:

1. **The benchmark is structural-precondition-conditional, not a uniform race (the load-bearing
   finding).** The eight Track-E attacks do not all apply to one curve — each exploits a *specific*
   structure: Pohlig–Hellman needs composite group order; MOV needs small embedding degree; SSA needs
   an *anomalous* curve (trace = 1, `#E = p`); GHS needs a binary `GF(2^m)` curve; index calculus
   (over `E(F_p)`) needs the Semaev-decomposable toy. The shared anchor `y² = x³ + x + 33 mod 47`
   (`n = 60`) carries **four** attacks (Pohlig–Hellman, MOV, Semaev, index calculus); SSA runs only on
   `y² = x³ + 5 mod 7`; GHS only on `GF(2^6)`; the rho timing baseline runs on `secp_k1_toy` (63-bit).
   **The table's columns are (attack, curve-precondition, applies?, cost-when-applicable)** — the
   pedagogical content is *which structure unlocks which escape*, not "which is fastest on a fixed
   instance." *(This is the E.W realisation of MATHEMATICS.md §"Escape from Search" — the five-family
   structure taxonomy the through-line already names; T.E develops it per-attack.)*

2. **GHS is a *transfer*, not an end-to-end solve — represent it honestly (confirmed).** `rho::ghs`
   exposes `ghs_descend` / `verify_log_preservation` / `transfer_point` — the descent *reduction* to a
   hyperelliptic-Jacobian DLP — but **no `ghs_dlp` solver** (the transfer/structure/solve framing,
   NOTES.md 2026-06-15: E.H transfers, it does not solve). The benchmark column for GHS measures the
   *reduction* (descent + log-preservation verification), annotated as a transfer whose downstream
   solve is index calculus — NOT a timing winner against the direct solvers. A `@build` agent that
   forces GHS into an end-to-end "solve time" column misrepresents the attack.

3. **The index-calculus benchmark counts come from the public re-exports — no contract amend
   (resolved at shard time, the E.K.5-flagged decision).** `index_calculus_dlp(g, q, strategy) →
   Result<Option<u64>, IndexCalcError>` returns only the log, **no decomposition/relation counts**.
   The E.K.5 ◆ digest flagged this as "best decided before E.W shards." **Resolved: E.W derives the
   counts by calling the already-public `collect_relations(...).len()` and `decompose(...)` directly**
   (both are `pub` re-exports from `rho::index_calculus`), alongside `index_calculus_dlp` for the
   answer. **C-IndexCalc is NOT amended** — it stays exactly as frozen at E.K.5. *(Tradeoff named: the
   benchmark re-runs collection to count it, a little redundant — but it is a benchmark, not a hot
   path, and amending a frozen contract at a closeout sub-track is the worse option. If a `@build`
   agent finds the counts genuinely cannot be derived from the public surface, that is an additive-
   amend discovery surfaced at the ◆, not a silent C-IndexCalc patch.)*

4. **The writeup extends an existing synthesis scaffold; it does not originate one (confirmed).**
   `docs/MATHEMATICS.md` already carries the §"Escape from Search: The Through-Line" (the five-family
   structure taxonomy + the L-notation hierarchy table) and a ~20-line "why no index calculus for
   generic EC" passage. The MOV reduction is *named* in the through-line but **has no full chapter**;
   there is **no index-calculus / Semaev chapter** and **no ECDLP-attack comparison table**. T.E (ch.
   10, the stub already in the ToC) develops the five attacks the through-line names — with the MOV
   reduction as the designated payoff proof — and adds the per-attack L-notation comparison. The
   E.W code-tour appends to `docs/PEDAGOGY.md` (the rho-crate code-tour home, currently ~493 lines,
   holding the existing Phase 0–8 rho content). *(Confirms the writeup is a delta on a frozen
   register — C-Textbook, frozen at T.0 — not a new artifact; the rigidity guard is "extend the
   through-line, do not re-derive it.")*

The work splits at **one benchmark↔writeup contract-sharp seam**, **2 sessions** (the ROADMAP ceiling
for E.W, consistent with the project's documented ceiling-bias — G/D both landed at or above their
upper bands):

1. **E.W.1 — Cross-attack benchmark harness + table (Sonnet, Cat C).** New Criterion benches for the
   non-rho attacks (Pohlig–Hellman, MOV, SSA, GHS-reduction, index calculus; rho is already benched in
   `rho/benches/ecdlp.rs`) + the `## E.W` section in `docs/BENCHMARKS.md` — the structural-
   precondition-conditional "which attack wins on which curve" table + the principle-4 science↔
   engineering note. **Freezes C-EWBench** (the benchmark data + the table shape the writeup cites).
   Mechanical bench-wiring over frozen solver APIs; Sonnet.

2. **E.W.2 ◆ `@architect` — Track-E code-tour + T.E maths chapter + Track-E close (Opus, Cat I).** The
   paired writeup: the Track-E code-tour (`docs/PEDAGOGY.md`) + **T.E** (`docs/MATHEMATICS.md` ch. 10
   — the algebraic-ECDLP chapter, with the **MOV-reduction payoff proof**) + the design-statement
   verification (principles 1/3/4 against the realised Track-E) + the Track-E ◆ close. **Consumes
   C-EWBench + all frozen Track-E contracts. Freezes C-TrackE** — the Track-E synthesis surface Z.1
   (umbrella) and T.Z (textbook bind) consume. Crosses the **Track-E ◆ boundary** — the entire
   algebraic-ECDLP arc (E.A → E.K) ships, synthesised and verified.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing a *new* attack or extending
index calculus to `F_{p^n}` — those are the **deferred re-shards** the E.K close named, not E.W; or
racing all attacks on one curve as if the table were a uniform timing contest; or re-deriving the
§"Escape from Search" through-line T.0 already froze) and **rigidity** (forcing GHS into an end-to-end
solve column when it is a transfer; or breaking the C-Textbook register — audience floor, proof-sketch
depth, MathJax markup — for one chapter; or amending C-IndexCalc to surface benchmark counts when the
public re-exports already supply them).

**Scoping discipline.** E.W builds the cross-attack synthesis at **demonstration fidelity** (the
benchmark exercises each frozen solver on its toy instance; the table reports the structural
precondition + the toy-scale cost) and **writes the Track-E chapter at the C-Textbook register** (the
frozen audience/depth/markup). It **amends no algorithm contract** (every Track-E solver surface is
read, not touched; the only code is additive Criterion benches + their manifest entries). It builds
**no new attack**, **no index-calculus `F_{p^n}` lift** (the deferred re-shard), **no GHS Jacobian
solver** (the transfer's downstream is index calculus, already shipped). The **engineering-vs-
mathematics disconnect** (ROADMAP principle 4) is explicit and load-bearing here in two directions:
(a) *under-exposed* — the asymptotic L-notation separations between the attacks (rho's `L[1, 1/2]` vs
index calculus's subexponential-in-the-`F_{p^n}`-setting vs MOV's reduction-to-`L[1/3]`) are **not
observable at toy scale**; the table reports toy-scale costs and the chapter explains why the
asymptotic picture differs; (b) the design-statement verification (E.W.2) must record that Track-E met
principles 1/3/4 (algorithmic content complete; no engineering optimisation crept in; scale-only
phenomena at demonstration fidelity), the G.W §59 / D.W §69 analogue for Track E.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.W): "*E.W — Cross-attack benchmarks + Track E writeup. 1-2 sessions.
Predecessor: most of E. The 'which attack wins on which curve' table; the pedagogical synthesis of
structure-based escape from search. Opus-tier — high integrative judgment load.*" And per the Track-τ
contract: **T.E folds into E.W** — the maths-first algebraic-ECDLP chapter (`docs/MATHEMATICS.md` ch.
10) is written *paired with* the E.W code-tour, "at the track's ◆ boundary," with the **MOV reduction
as the designated Opus payoff proof**.

E.W is the **Track-E integrative close**: it does not add an attack, it *synthesises* the eight
attacks Track E shipped — Pollard rho (the generic √n baseline, E.A-era), Pohlig–Hellman (CRT to
prime-order subgroups, E.A), the MOV/Frey–Rück pairing reduction (E.C, the cross-track bridge to
NFS-DL — the project's pedagogical climax), Smart–Satoh–Araki (the polynomial-time anomalous-curve
attack, E.E), GHS/Weil descent (the binary-curve transfer, E.H), the Semaev primitive (E.J), and
Gaudry–Diem–Joux–Vitse index calculus (E.K, the "solve") — into one comparative picture organised by
the project's through-line: **structure-based escape from search**. Each attack is a story about
finding the exploitable structure (a homomorphism, a pairing, an endomorphism, a smoothness/
decomposition phenomenon) that escapes the generic √n bound; E.W is where that story is told *across*
the attacks, not within one.

The deliverable is three-fold:

1. **The cross-attack benchmark table (E.W.1).** New Criterion benches for the non-rho attacks + a
   `docs/BENCHMARKS.md` section. **The table is structural-precondition-conditional**: its columns are
   (attack, curve-precondition, applies on this curve?, cost when applicable), NOT a uniform race —
   because the attacks apply only on the curves whose structure they exploit. This conditionality *is*
   the pedagogical content (which structure unlocks which escape).

2. **The Track-E code-tour (E.W.2, `docs/PEDAGOGY.md`).** The code-tour chapter, in the genre of G.W /
   D.W: the Track-E attacks at a glance, the per-attack code-tour, the cross-phase contract view, the
   design-statement verification (principles 1/3/4), the KAT summary.

3. **T.E — the algebraic-ECDLP maths chapter (E.W.2, `docs/MATHEMATICS.md` ch. 10).** The maths-first
   chapter the through-line scaffolds: Pohlig–Hellman, the **MOV/Frey–Rück reduction (the payoff
   proof)**, Smart–Satoh–Araki, GHS/Weil descent, and index calculus — each developed at proof-sketch
   depth, with the per-attack L-notation comparison the through-line's hierarchy table anticipates.

The sub-track decomposes into two conceptual units, each a session:

1. **Cross-attack benchmark harness + table (E.W.1).** The empirical layer — the numbers the writeup
   cites. **Freezes C-EWBench. (E.W.1.)**

2. **Track-E code-tour + T.E maths chapter + close (E.W.2 ◆).** The synthesis layer — the prose
   picture + the design-statement verification + the close. **Freezes C-TrackE. (E.W.2 ◆.)**

E.W is **attack-free** (it adds no new ECDLP attack — the index-calculus `F_{p^n}` lift and the
GHS-coupled end-to-end are the deferred re-shards E.K named), **contract-amend-free** (every Track-E
solver surface is read, not touched; the only code is additive benches), and **register-stable** (T.E
obeys the frozen C-Textbook). Re-read this intent at the ◆ boundary to catch defocus (a new attack,
the `F_{p^n}` lift, a uniform-race table, re-deriving the through-line) and rigidity (forcing GHS into
a solve column, breaking the C-Textbook register, amending C-IndexCalc for counts).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-16; the
workspace `Cargo.toml` carries only `[workspace]` members + a `[profile.bench]`); raw `cargo` is the
only CI surface (unchanged from E.D…E.K). The benchmark layer adds a third discovered command:
`VERIFY_BENCH = cargo bench --no-run` (compile-only — confirms the new benches build without running
the full timing pass; the actual `cargo bench` timings are hand-transcribed into BENCHMARKS.md, the
established pattern). Oracle KATs are `#[ignore]`-gated only (`#[ignore = "PARI not installed; run
manually when available"]` / the msolve analogue), used identically across `rho/tests/*_kat.rs`.
`/run-plan` re-discovers at preflight. E.W **adds no new workspace edge and no new crate** (criterion
is already a `rho` dev-dependency; the new benches are additive `[[bench]]` entries in
`rho/Cargo.toml`; the docs are pure-prose additions), so the gate is a **no-regression + build gate**:

- **The existing rho / gnfs / shared KATs must stay green** — E.W changes no solver path; it reads the
  frozen surfaces and adds benches + docs. `cargo test --workspace` is the no-regression guard.
- **`cargo check --workspace` must stay green** — no edge change, no new dependency. The doc edits do
  not touch code; the bench additions are leaf additions to the `rho` crate.
- **`cargo bench --no-run` must compile the new benches** — the benchmark harness (E.W.1) is the only
  new code; it must build against the frozen solver APIs. (Running the full `cargo bench` to harvest
  timings is a manual step; the numbers are transcribed into BENCHMARKS.md, matching the hand-written
  G.W timing section.)
- **No live oracle on the green path** — E.W introduces none (principle 3); the benches call the
  frozen `rho` solvers (no PARI/msolve/CADO). Any oracle cross-check stays `#[ignore]`-gated.
- **Documentation has no compiler/test gate** — the code-tour (`PEDAGOGY.md`) and T.E
  (`MATHEMATICS.md`) are prose; their "verify" is the E.W.2 ◆ juncture review (register conformance to
  C-Textbook, cross-reference correctness, the design-statement verdict), not `cargo`.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.W.1 | Cross-attack ECDLP benchmark harness + "which attack wins on which curve" table | C | Sonnet | C-Pollard/`rho::ecdlp::solve_*` (frozen, read); C-Pohlig/`rho::ecdlp::pohlig::solve_ecdlp_composite` (frozen, read); C-Mov/`rho::pairing::mov::mov_reduce` (frozen, read); C-Ssa/`rho::ssa::ssa_solve` (frozen, read); C-GHSDescent/`rho::ghs::{ghs_descend, verify_log_preservation}` (frozen, read — transfer, not solve); C-IndexCalc/`rho::index_calculus::{index_calculus_dlp, collect_relations, decompose}` (frozen, read — counts via the re-exports); the per-attack toy fixtures (read) | `rho/benches/attacks.rs` (new: Criterion benches for Pohlig–Hellman, MOV, SSA, GHS-reduction, index calculus on their respective toy fixtures), `rho/Cargo.toml` (add `[[bench]] name = "attacks", harness = false`), `docs/BENCHMARKS.md` (add `## E.W` section: the structural-precondition-conditional table + the principle-4 note) |
| E.W.2 ◆ `@architect` | Track-E code-tour + T.E algebraic-ECDLP chapter (MOV payoff) + Track-E close | I | **Opus** | C-EWBench (frozen E.W.1 — the table the writeup cites); all frozen Track-E contracts (C-Pollard, C-Pohlig, C-Mov, C-Ssa, C-GHSDescent, C-Semaev, C-IndexCalc — read for the code-tour + chapter); C-Textbook (frozen T.0 — the register T.E obeys); the §"Escape from Search" through-line (frozen T.0 — extended, not re-derived) | `docs/PEDAGOGY.md` (append: Track-E code-tour — attacks at a glance, per-attack tour, cross-phase contracts, design-statement verification, KAT summary), `docs/MATHEMATICS.md` (append/fill ch. 10 "Algebraic ECDLP Attacks" — T.E: Pohlig–Hellman, MOV payoff proof, SSA, GHS, index calculus + the per-attack L-notation comparison) |

**Sequencing notes.** Strictly serial: **E.W.1 → E.W.2.** E.W.1 lands the benchmark harness + the
table data; E.W.2 writes the code-tour + T.E that *cite* that table and closes the track. **One
`@architect` marker:** the **E.W.2 ◆** (the Track-E boundary juncture — ratifying the cross-attack
synthesis, the design-statement verdict on principles 1/3/4, the C-Textbook register conformance of
T.E, and confirming the Track-E arc E.A–E.K is coherent and closed before the ◆). *(Tradeoff named:
E.W pages a juncture only at the ◆-close, NOT at the open — unlike E.K, whose E.K.1 substrate-contract
design forced an opening fork. E.W.1 is mechanical bench-wiring over already-frozen solver APIs with no
contract-design crux, so no opening fork is warranted; the integrative judgment is concentrated at the
close, where the synthesis + the design-statement verification + the ◆ all land. This matches the G.W
/ D.W closeout pattern, where the writeup session is the single Opus juncture.)*

**Why 2 sessions (the ROADMAP ceiling, confirmed by ceiling-bias).** The split is taken at the single
benchmark↔writeup contract-sharp seam:
- **One-line-commit-title corollary.** "Cross-attack benchmark harness + table" and "Track-E code-tour
  + T.E chapter + close" are **two distinct commit titles** across two categories (C optimization/
  empirical ×1, I integrative ×1). Bundling them into one session fails the corollary — "add benches
  AND write the code-tour AND write the maths chapter AND verify the design statement AND close the
  track" is not one commit-title-shaped sentence.
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: E.W.1 is the
  empirical layer (the numbers); E.W.2 is the synthesis layer (the prose + the verdict + the close).
  Neither is fractured below its floor.
- **Contract-sharp boundary.** E.W.1 **freezes** C-EWBench (the benchmark data + table shape); E.W.2
  **consumes** it (the writeup cites the table) and **freezes** C-TrackE (the synthesis surface Z.1 +
  T.Z consume). The writeup is meaningless without the benchmark freeze — it quotes the numbers.

**The softest seam — could T.E split off as a third session (E.W.3)?** The Track-τ contract explicitly
allows the maths chapter to "split into a dedicated follow-on if it overruns — decided at the
boundary." The chosen shard keeps the **code-tour and T.E together in E.W.2** (the G.W / D.W
precedent), because the τ rationale for pairing them is that mathematics and code-mapping stay
consistent while both are fresh — there is **no contract seam between the code-tour and its paired
chapter** (they reference the same frozen attacks), so splitting them up front would fracture the
writeup unit below its floor without buying an early freeze. **If T.E overruns at E.W.2** (the
MOV-payoff proof + the five-attack development + the L-notation comparison push past the session band),
the τ contract's escape applies: split T.E into a dedicated E.W.3 follow-on — an additive-reshard
surfaced at the ◆, not a silent overrun. This is the one place the 2-vs-3 sizing is genuinely
uncertain until the code-tour lands and the chapter's true size is concrete.

---

## Session detail

E.W.1 is specified at near-full fidelity (the benchmark surface is entirely frozen — the solver APIs
are known, the fixtures are known, the only design choice is the table shape). E.W.2 is specified at
the structural level (the code-tour + chapter outline) with the per-section content sketched — correct
per the substrate-first discipline: the chapter's exact size and the design-statement verdict are
crisp only after the benchmark data freezes and the writeup is underway.

### E.W.1 — Cross-attack ECDLP benchmark harness + table (Sonnet, Cat C)

**Deliverable:** the empirical layer — new Criterion benches for the non-rho attacks + the
`docs/BENCHMARKS.md` `## E.W` section. The pieces:
- **The benches** (`rho/benches/attacks.rs`, new): a Criterion bench group with one benchmark per
  attack that *applies on its fixture*, calling the frozen solver and measuring wall-clock cost:
  - **Pohlig–Hellman** — `solve_ecdlp_composite` on `composite_toy()` (`y² = x³ + x + 33 mod 47`,
    `n = 60 = 2²·3·5`).
  - **MOV/Frey–Rück** — `mov_reduce` on `pairing_toy()` (the same base curve, `ℓ = 3`, `k = 2`,
    `F_{47²}`); note the bench measures the *reduction + the F_{p^k} DLP* the bridge calls.
  - **SSA** — `ssa_solve` on `anomalous_toy()` (`y² = x³ + 5 mod 7`, `#E = 7 = p`).
  - **GHS-reduction** — `ghs_descend` + `verify_log_preservation` on `ghs_toy_curve()` (`GF(2^6)`,
    `m = 6`, `l = 2`); **the transfer, NOT an end-to-end solve** (see subtlety 1).
  - **Index calculus** — `index_calculus_dlp` on `IndexCalcStrategy::toy()` (same base curve, `ℓ = 5`,
    `|FB| = 6`, `m = 2`), with the relation/decomposition counts derived from
    `collect_relations(...).len()` + `decompose(...)` (the public re-exports — no contract amend).
  - *(Pollard rho is already benched in `rho/benches/ecdlp.rs` on `secp_k1_toy` (63-bit) — read and
    cite that bench as the generic-√n baseline column; do not duplicate it.)*
- **The manifest entry** (`rho/Cargo.toml`): add `[[bench]] name = "attacks" harness = false` (the
  established `harness = false` Criterion pattern, matching the three existing `[[bench]]` entries).
- **The `## E.W` BENCHMARKS.md section** (`docs/BENCHMARKS.md`, append): the
  **structural-precondition-conditional table** — columns (attack, curve-precondition, applies?,
  toy-scale cost, escape structure) — + the principle-4 science↔engineering note (the asymptotic
  L-notation separations are NOT observable at toy scale; the table reports toy costs, the chapter
  explains the asymptotic picture). Matches the existing per-sub-track BENCHMARKS.md genre (prose
  setup + table + science↔engineering note, the G.W section as template).

Consumes the frozen solver surfaces (read): `rho::ecdlp::solve_*`, `rho::ecdlp::pohlig::
solve_ecdlp_composite`, `rho::pairing::mov::mov_reduce`, `rho::ssa::ssa_solve`, `rho::ghs::
{ghs_descend, verify_log_preservation}`, `rho::index_calculus::{index_calculus_dlp, collect_relations,
decompose}`, and the per-attack toy fixtures. **Freezes C-EWBench.**

**KAT** (the benches double as the no-regression signal; plus an optional inline correctness assert):
over each toy fixture: **each benched attack still solves its instance** (the bench body asserts the
solver returns the known answer before timing — so the bench is also a smoke test that no frozen path
regressed). **Verify gate:** `cargo test --workspace` green (no regression); `cargo check --workspace`
green (leaf bench additions, no edge change); `cargo bench --no-run` compiles the new bench.

**Subtlety (load-bearing):** (1) **GHS is a transfer, not a solve** — `rho::ghs` has no `ghs_dlp`; the
bench measures the descent reduction + log-preservation verification, annotated as a transfer whose
downstream solve is index calculus. Forcing GHS into an end-to-end "solve time" column is a
misrepresentation. (2) **The table is precondition-conditional, not a uniform race** — each attack
applies only on the curve whose structure it exploits; the table's columns encode the precondition
("applies?"), and the pedagogical point is *which structure unlocks which escape*, not a fixed-instance
timing winner. (3) **Index-calculus counts come from the public re-exports** — `index_calculus_dlp`
returns no counts; derive them from `collect_relations(...).len()` + `decompose(...)` (both `pub`);
**do NOT amend C-IndexCalc**. (4) **Toy-scale costs only** — the asymptotic L-notation separations are
not observable at `p = 47` / `secp_k1_toy`; the principle-4 note records this (the asymptotic picture
is T.E's job). (5) **Criterion is already a dev-dep** — the bench is a leaf addition; no new
dependency, no new edge.

**Deferred:** the code-tour (E.W.2); T.E (E.W.2); the design-statement verdict (E.W.2); the Track-E
close (E.W.2 ◆); any new attack / the `F_{p^n}` index-calculus lift / the GHS Jacobian solve (deferred
re-shards, not E.W).

### E.W.2 ◆ — Track-E code-tour + T.E algebraic-ECDLP chapter + Track-E close (Opus, Cat I)

**Deliverable:** the synthesis layer — the paired writeup (code-tour + T.E) + the design-statement
verification + the Track-E ◆ close. Structural-fidelity sketch (the per-section content is crisp once
the benchmark data freezes and the chapter outline is set). The pieces:
- **The Track-E code-tour** (`docs/PEDAGOGY.md`, append — the rho-crate code-tour home, after the
  existing Phase 0–8 rho content): in the G.W / D.W genre — Track-E attacks at a glance; the per-attack
  code-tour (Pohlig–Hellman, MOV, SSA, GHS, Semaev, index calculus — each: what it exploits, the
  module surface, the toy KAT); the cross-phase contract view (the frozen Track-E contracts unified);
  the **design-statement verification** (principles 1/3/4 against the realised Track-E — the G.W §59 /
  D.W §69 analogue: algorithmic content complete; no engineering optimisation crept in; scale-only
  phenomena at demonstration fidelity, with the toy-scale L-notation non-observability annotated); the
  KAT summary; cross-references to T.E.
- **T.E — the algebraic-ECDLP maths chapter** (`docs/MATHEMATICS.md` ch. 10 — fill the existing stub):
  the maths-first chapter the §"Escape from Search" through-line scaffolds, at the C-Textbook register
  (proof-sketch depth, MathJax markup, the undergraduate-maths audience floor). Develops the five
  attacks the through-line names — Pohlig–Hellman (CRT to prime-order subgroups), **the MOV/Frey–Rück
  reduction (the DESIGNATED PAYOFF PROOF — why the pairing maps ECDLP to `F_{p^k}^*` DLP, where index
  calculus applies; the cross-track climax)**, Smart–Satoh–Araki (the polynomial-time anomalous-curve
  attack), GHS/Weil descent (the binary-curve transfer to a hyperelliptic Jacobian), and index calculus
  (Semaev decomposition over the factor base) — each at proof-sketch depth, with the **per-attack
  L-notation comparison** the through-line's hierarchy table anticipates (rho `L[1, 1/2]`; index
  calculus subexponential in the `F_{p^n}` setting; MOV reducing to `L[1/3]` via NFS-DL; SSA
  polynomial; GHS conditional). Extends the through-line; does NOT re-derive it.
- **The Track-E ◆ close**: re-read the Purpose intent; verify the Track-E arc (E.A → E.K) is coherent
  and complete; record the design-statement verdict; confirm C-TrackE exposes what Z.1 (umbrella) +
  T.Z (textbook bind) consume.

Consumes C-EWBench (frozen E.W.1), all frozen Track-E contracts (read), C-Textbook (frozen T.0 — the
register), the §"Escape from Search" through-line (frozen T.0 — extended). **Freezes C-TrackE.**

**KAT (review-enforced, not compiler-enforced):** documentation has no `cargo` gate — the E.W.2 ◆
juncture review is the verification: T.E conforms to the C-Textbook register (audience floor, proof-
sketch depth, MathJax markup); the MOV payoff proof is complete and correct (the designated climax);
the code-tour cross-references resolve; the design-statement verdict (principles 1/3/4) is recorded;
the cross-attack table (C-EWBench) is cited accurately. **Verify gate:** `cargo test --workspace` /
`cargo check --workspace` green (no code change beyond E.W.1's benches — the no-regression invariant
holds trivially); the prose verification is the ◆ fork.

**Subtlety (load-bearing):** (1) **The MOV payoff proof is the designated Opus climax** — the
through-line names MOV as "the cross-track bridge of this project"; T.E must develop *why* the pairing
reduction works (the bilinear `e: E[n] × E[n] → μ_n` mapping ECDLP to `F_{p^k}^*` DLP), at the
proof-depth C-Textbook reserves for designated payoffs. A `@build`/writer treating MOV as one
bullet among five mis-weights the chapter. (2) **Extend the through-line, do not re-derive it** —
the five-family structure taxonomy + the L-notation hierarchy are frozen at T.0; T.E develops each
attack *under* that scaffold, citing it, not restating it (rigidity guard). (3) **GHS is a transfer**
— the chapter must represent GHS as the binary-curve *reduction* (transfer/structure/solve, NOTES.md),
whose downstream solve is index calculus; not as a standalone end-to-end break. (4) **C-Textbook
register is frozen** — the audience floor (undergraduate maths), proof-sketch depth (complete +
clinical, not exhaustive, not inscrutable; full proofs only at payoffs), and MathJax markup are frozen
at T.0; breaking the register for one chapter is a discovery that must flex C-Textbook at the ◆, not a
silent level-raise. (5) **The design-statement verification is load-bearing** — E.W.2 is "where the
design statement is verified against the actual Track-E implementation" (the G.W / D.W analogue); the
verdict on principles 1/3/4 is recorded in the action-frame digest. (6) **No new attack, no `F_{p^n}`
lift, no GHS Jacobian solver** — these are the deferred re-shards E.K named; E.W is the close, not an
extension.

**Deferred:** Z.1 (the umbrella narrative — Phase ζ); T.Z (the textbook bind — Phase τ); the deferred
Track-E re-shards (the `F_{p^n}` index-calculus asymptotic-win lift; the GHS-coupled binary-curve
end-to-end); Track S (Shor + post-quantum) — all post-Track-E.

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.W.2 ◆ to confirm: (1) the cross-attack synthesis is coherent — the eight Track-E attacks are
organised under the structure-based-escape through-line, the table (C-EWBench) is cited accurately,
and the conditionality (which structure unlocks which escape) is the chapter's spine; (2) the **MOV
payoff proof** is complete and correct at the designated proof-depth (the cross-track climax); (3) T.E
conforms to the frozen C-Textbook register (audience, depth, MathJax) — no silent level-break; (4) the
**design-statement verdict** is recorded — Track E met principles 1/3/4 (algorithmic content complete;
no engineering optimisation crept in; scale-only phenomena at demonstration fidelity, toy-scale
L-notation non-observability annotated); (5) C-TrackE exposes what Z.1 (umbrella) + T.Z (textbook bind)
consume so the downstream integrative sessions build without re-opening Track E; (6) E.W stayed in
scope — no new attack, no `F_{p^n}` index-calculus lift, no GHS Jacobian solver, GHS represented
honestly as a transfer. **Also: surface the outstanding static-frame ROADMAP debt** carried +
compounded from the E.I, E.H, E.J, and E.K ◆ (the Progress / Remaining tables stale by six completed
sub-tracks E.F–E.K + the E.H-before-E.I inversion; see the Discoveries & risks entry) — **and note
that the Track-E ◆ close is the natural reconciliation point** (Track E is now complete end-to-end), so
the ROADMAP write is *owed at this boundary* — though the ROADMAP write itself is out of `@architect`
PLAN-write scope (a capture candidate, not a PLAN edit). One-shot findings; does not implement. Held at
**Opus** per the header.

---

## Cross-session contracts

E.W **freezes two** contracts (C-EWBench at E.W.1, C-TrackE at E.W.2 ◆) and **amends no prior frozen
contract** — every Track-E solver surface (C-Pollard, C-Pohlig, C-Mov, C-Ssa, C-GHSDescent, C-Semaev,
C-IndexCalc) is **read, not touched**; C-Textbook (frozen T.0) is the register T.E obeys, not amended.
E.W adds `rho/benches/attacks.rs` + a `[[bench]]` manifest entry + prose appends to `docs/PEDAGOGY.md`
and `docs/MATHEMATICS.md` — all **additive**, no trait amendment, no new edge.

### C-EWBench — the cross-attack benchmark data + table contract (test-/prose-enforced) — *to be frozen at E.W.1*

**Defined in:** E.W.1 (`rho/benches/attacks.rs` + the `## E.W` section of `docs/BENCHMARKS.md`).
**Consumed by:** E.W.2 (the code-tour + T.E cite the table — the benchmark numbers the synthesis
quotes); **downstream: Z.1** (the umbrella's cross-attack comparison). Test-/prose-enforced (the
benches double as no-regression smoke tests; the table is prose).

**Ratified shape (to be confirmed at the E.W.2 ◆ — the benchmark contract is the structural-
precondition-conditional table, not a uniform timing race).** The table's columns are:
- **Attack** — the named attack (Pollard rho, Pohlig–Hellman, MOV, SSA, GHS-reduction, index calculus).
- **Curve-precondition** — the structure the attack requires (generic / composite order / small
  embedding degree / anomalous (trace = 1) / binary `GF(2^m)` / Semaev-decomposable).
- **Applies?** — whether the attack is applicable on each benchmarked curve (the conditionality that is
  the table's pedagogical content).
- **Toy-scale cost** — the Criterion wall-clock median on the attack's toy fixture (the empirical
  datum; a transfer like GHS reports reduction cost, annotated).
- **Escape structure** — the structure-based-escape family (per the §"Escape from Search" taxonomy).

**Invariants:** every benched attack still solves (or, for GHS, transfers) its toy instance (the bench
body asserts correctness before timing — the no-regression smoke test); GHS is reported as a *transfer*
(reduction cost), never as an end-to-end solve; the index-calculus counts are derived from the public
`collect_relations` / `decompose` re-exports (C-IndexCalc unamended); the toy-scale-cost column carries
the principle-4 caveat (asymptotic L-notation separations not observable at toy scale). *(The table is
the empirical substrate the Track-E synthesis stands on — the closeout analogue of a per-stage
BENCHMARKS.md section, e.g. the G.W timing table.)*

### C-TrackE — the Track-E cross-attack synthesis + design-statement verdict (prose-enforced) — *frozen at E.W.2 ◆ (commit 10c9d54)*

**Defined in:** E.W.2 (`docs/PEDAGOGY.md` Track-E code-tour + `docs/MATHEMATICS.md` ch. 10 / T.E).
**Consumed by:** **Z.1** (the umbrella narrative — the cross-track L-notation synthesis + the
structure-based-escape master chapter) and **T.Z** (the textbook bind — the final consistency pass
across all chapters). Prose-enforced (the ◆ juncture review is the gate). Exposes: the algebraic-ECDLP
chapter (the five-attack development + the MOV payoff proof + the per-attack L-notation comparison), the
Track-E code-tour, and the **design-statement verdict** (Track E met principles 1/3/4). **The frozen
invariant:** the Track-E arc (E.A → E.K) is coherent, complete, and verified against the design
statement; the synthesis obeys the C-Textbook register; the MOV payoff proof is the designated climax.
**Track E is the algebraic-ECDLP attack survey complete — the structure-based-escape-from-search
through-line realised across eight attacks, NOT a new attack.** *Exact chapter scope + the verdict
recorded at the ◆.*

#### Resolved interface (E.W.2 ◆ inflection design — `@plan-juncture`, Opus, 2026-06-16)

The substrate survey confirmed every artifact the writeup mirrors: the C-EWBench table (frozen at
E.W.1, commit 1916c65) is the five-column structural-precondition-conditional table the PLAN
ratified (Attack / Curve-precondition / Applies? / Toy-scale cost / Escape structure); the eight
frozen solver surfaces exist as named (`solve_ecdlp_composite`, `mov_reduce`, `ssa_solve`,
`ghs_descend`/`verify_log_preservation`/`transfer_point`, `index_calculus_dlp` +
`collect_relations`/`decompose` public re-exports, `semaev_poly`); the G.W/D.W code-tour genre
(`gnfs/docs/PEDAGOGY.md` §52–§71) and the C-Textbook register + §"Escape from Search" through-line
(`docs/MATHEMATICS.md`) are the templates. The resolved interface fixes the per-section structure.

**(A) T.E — `docs/MATHEMATICS.md` ch. 10 "Algebraic ECDLP Attacks" (fill the existing stub).** The
maths-first chapter at the C-Textbook register (undergraduate-maths audience floor, proof-sketch
depth, MathJax markup), maths-first sibling to the Track-E code-tour. Section outline:

- **§10.0 The through-line for this chapter.** One paragraph re-stating (citing, not re-deriving)
  the §"Escape from Search" frame: each Track-E attack finds a curve structure that escapes the
  generic $\sqrt n$ bound the Pollard-rho chapter (ch. 6) established. Names the five structures and
  forward-points to the per-attack sections. *(Rigidity guard: extend, do not re-derive the frozen
  taxonomy.)*
- **§10.1 Pohlig–Hellman (composite order, CRT).** The order-reduction warm-up: $\#E$ composite
  $\Rightarrow$ ECDLP factors over prime-order subgroups via CRT (cite §Prerequisites CRT). Cost
  $O(\sum e_i(\log n + \sqrt{p_i}))$. Proof-sketch depth.
- **§10.2 Smart–Satoh–Araki (anomalous, polynomial-time).** $\#E(\mathbb F_p)=p$ $\Rightarrow$
  $p$-adic lift + formal-group logarithm gives a polynomial-time solve. Proof-sketch depth (the
  $p$-adic elliptic logarithm is named + cited, not fully derived).
- **§10.3 GHS / Weil descent (the binary-curve TRANSFER).** $E/\mathbb F_{2^m}$ with a subfield
  tower $\Rightarrow$ Weil restriction transfers ECDLP to a hyperelliptic-Jacobian DLP over
  $\mathbb F_{2^l}$. **Represented as a transfer/structure/solve, NOT an end-to-end break** — the
  downstream solve is index calculus (deferred re-shard). Proof-sketch depth.
- **§10.4 Index calculus over $E(\mathbb F_{p^n})$ (Gaudry–Diem–Joux–Vitse).** The factor-base /
  Semaev-decomposition engine: summation polynomials (cite C-Semaev) build a relation matrix over
  $\mathbb F_\ell$; the asymptotic win is the extension-field setting $E(\mathbb F_{p^n})$, $n>1$
  (the toy is over $E(\mathbb F_p)$ — annotate the principle-4 gap). Proof-sketch depth.
- **§10.5 The MOV / Frey–Rück reduction — THE PAYOFF PROOF (full, not sketch).** The designated
  Opus climax + the one full proof in this chapter (the C-Textbook payoff carve-out, alongside
  T.G's $L$-notation derivation). Develops *why* the bilinear pairing $e:E[n]\times E[n]\to\mu_n$
  is non-degenerate and Galois-equivariant, hence maps ECDLP on $E$ to DLP in $\mathbb F_{p^k}^*$
  (where $k$ is the embedding degree and index calculus applies subexponentially) — the cross-track
  bridge to NFS-DL. Cites the frozen `rho::pairing` realisation (`mov_reduce`, the Tate/Weil
  pairing). Proof given in full.
- **§10.6 The per-attack $L$-notation comparison.** Extends (cites) the frozen §"Escape from
  Search" $L$-notation hierarchy table with a per-attack row: rho $L_n[1,1/2]$; Pohlig–Hellman
  $\sqrt{p_{\max}}$ (largest prime factor); SSA polynomial; GHS conditional (transfer cost +
  downstream Jacobian index calculus); MOV reducing to $\mathbb F_{p^k}^*$ DLP, subexponential
  ($L[1/2]$ direct, $L[1/3]$ via NFS-DL). The principle-4 boundary stated: these asymptotic
  separations are NOT observable at the C-EWBench toy scale.
- **§10.7 Cross-reference** to the Track-E code-tour (the code realisation) + ch. 6 (the rho
  baseline this chapter's bound-breaking extends) + the §"Escape from Search" through-line.
- **§10.8 Further reading** (Menezes–Okamoto–Vanstone; Frey–Rück; Smart; Satoh–Araki; Semaev;
  Gaudry; Diem; GHS), matching the per-chapter Further-Reading genre.

*Section ordering — flagged recommendation (writer holds fiduciary latitude).* The PLAN's
through-line text lists MOV second (immediately after the pairing family). The resolved outline
places MOV **last (§10.5) as the climax**, so the chapter builds Pohlig–Hellman → SSA → GHS →
index-calculus *engine* → and lands MOV as the bridge that connects ECDLP to that engine — which
the PLAN's "MOV is the designated payoff/climax" framing favours. *Tradeoff: this departs from the
ch.-6 "When the bound breaks" bullet order (MOV-first), so a reader cross-walking the two chapters
sees a re-ordering; the code-tour's per-attack tour should follow the same order for consistency.*
Either order is PLAN-consistent; surfaced for the human glance, not halted.

**(B) The Track-E code-tour — `docs/PEDAGOGY.md` (append after the existing Phase 0–8 rho content).**
In the G.W/D.W integrative genre, adapted for an **attack survey (not a linear pipeline)** — the
"at a glance" is a taxonomy table, not a data-flow diagram (the load-bearing precondition-
conditional finding). Section outline (continuing the existing `## N.` numbering):

- **§N. The Track-E attacks at a glance.** A taxonomy table (Attack | Curve structure exploited |
  Module surface | Toy fixture | C-contract), the survey analogue of the G.W §52 pipeline diagram.
  Six rows: Pollard rho (baseline, `rho::ecdlp`), Pohlig–Hellman, MOV, SSA, GHS (transfer), Semaev
  / index calculus. Opens with the through-line: which structure unlocks which escape.
- **§N+1…N+6. The per-attack code-tour** (one section each: Pohlig–Hellman, MOV, SSA, GHS, Semaev,
  index calculus). Each in the G.W per-stage shape: *what it exploits* (the structure), *the module
  surface* (the frozen public API — `solve_ecdlp_composite`, `mov_reduce`, `ssa_solve`,
  `ghs_descend`/`verify_log_preservation`, `semaev_poly`, `index_calculus_dlp` +
  `collect_relations`/`decompose`), *the toy KAT/fixture*, *cross-ref to the matching T.E section*.
  GHS section states it is a **transfer** (no `ghs_dlp`; downstream solve is index calculus).
- **§N+7. The cross-phase contract view.** The G.W §58 analogue: a unified table of the frozen
  Track-E contracts (C-Pollard, C-Pohlig, C-Mov, C-Ssa, C-GHSDescent, C-Semaev, C-IndexCalc — with
  frozen-at session + what each exposes), naming them in one place. All read, none amended by E.W.
- **§N+8. Design-statement verification (principles 1/3/4).** The G.W §59 / D.W §69 analogue — the
  load-bearing section. Walks each principle against the realised Track-E (E.A→E.K):
  - **Principle 1 (algorithmic content complete):** all eight attacks implemented head-on, not
    stubbed — verdict + per-attack one-liner.
  - **Principle 3 (no engineering optimisation crept in):** PARI/msolve oracles stay `#[ignore]`-
    gated dev-only; no production solver acceleration — verdict.
  - **Principle 4 (scale-only at demonstration fidelity):** a toy-scale annotations table (the
    asymptotic $L$-notation separations non-observable at $p=47$/$p=7$; GHS-as-transfer; index
    calculus's extension-field asymptotic win not exhibited at toy scale) — verdict.
  - **Summary verdict** (pass/pass/pass expected, but recorded against the actual realisation — a
    divergence here is a discovery surfaced at the ◆, not pre-judged).
- **§N+9. KAT summary (E.W — integrative).** The G.W §62 / D.W §70 analogue: a table of the
  existing Track-E KATs + the `attacks.rs` bench pre-check asserts (each benched attack still solves
  / transfers its toy instance — the C-EWBench no-regression smoke test). No new KATs (code-tour,
  not new implementation).
- **§N+10. Cross-references** (to T.E ch. 10, the C-EWBench table in `docs/BENCHMARKS.md` ## E.W,
  the per-attack contract definitions) + **Further reading**.

**(C) The C-TrackE frozen invariant — what Z.1 (umbrella) + T.Z (textbook bind) consume.** At the
◆, C-TrackE freezes and exposes, for the downstream integrative sessions to build on without
re-opening Track E:
1. **T.E ch. 10** — the algebraic-ECDLP chapter (five-attack development + the full MOV payoff
   proof + the per-attack $L$-notation comparison row extending the frozen hierarchy table), at the
   C-Textbook register. *Z.1's cross-track $L$-notation synthesis + structure-based-escape master
   chapter consume this; T.Z's consistency pass binds it.*
2. **The Track-E code-tour** in `docs/PEDAGOGY.md` — the attacks-at-a-glance taxonomy + per-attack
   tour + unified contract view. *Z.1 consumes the cross-attack comparison; T.Z binds the
   code-tour↔chapter pairing.*
3. **The design-statement verdict** — Track E met principles 1/3/4 (recorded in the action-frame
   digest at the ◆). *The umbrella's "the project met its scoping principles" claim rests on this.*
**Frozen invariant:** the Track-E arc E.A→E.K is coherent, complete, and verified against the
design statement; the synthesis obeys the C-Textbook register; the MOV payoff proof is the
designated climax given in full; GHS is represented honestly as a transfer; the §"Escape from
Search" through-line is extended (not re-derived); no Track-E solver contract is amended.

### Frozen contracts read by E.W (consumed, not amended)

- **C-Pollard (`rho::ecdlp` surface)** — `solve_brent` / `solve_dp` / `solve_dp_negmap` /
  `solve_dp_batch` / `solve_dp_glv` (the generic-√n baseline). Already benched (`rho/benches/ecdlp.rs`
  on `secp_k1_toy`); E.W.1 cites that bench, does not duplicate it. Read; untouched.
- **C-Pohlig (`rho::ecdlp::pohlig` surface)** — `solve_ecdlp_composite` (CRT to prime-order subgroups)
  on `composite_toy()`. Read; benched. Untouched.
- **C-Mov (`rho::pairing::mov` surface)** — `mov_reduce` (the pairing reduction to `F_{p^k}^*` DLP, the
  cross-track bridge) on `pairing_toy()`. Read; benched; the T.E payoff-proof subject. Untouched.
- **C-Ssa (`rho::ssa` surface)** — `ssa_solve` (the anomalous-curve polynomial-time attack) on
  `anomalous_toy()`. Read; benched. Untouched.
- **C-GHSDescent (`rho::ghs` surface)** — `ghs_descend` / `verify_log_preservation` / `transfer_point`
  (the binary-curve transfer — NOT a solver) on `ghs_toy_curve()` (`GF(2^6)`). Read; benched as a
  *transfer*. Untouched.
- **C-Semaev (`rho::semaev` surface)** — `semaev_poly` + the `MultiPoly` operations (the index-calculus
  primitive). Read for the code-tour + chapter. Untouched.
- **C-IndexCalc (`rho::index_calculus` surface)** — `index_calculus_dlp` + the public `collect_relations`
  / `decompose` re-exports (the latter two supply the benchmark counts — no amend). Read; benched.
  **Untouched** (the E.K.5-flagged counts question resolved by deriving from the re-exports).
- **C-Textbook (frozen T.0)** — the documentation-register contract (audience, proof-sketch depth,
  through-line, MathJax markup, artifact location `docs/MATHEMATICS.md`). The register T.E obeys. Read;
  **not amended** (a register break would be a discovery flexing C-Textbook at the ◆, not a silent
  level-raise).
- **The §"Escape from Search" through-line (frozen T.0)** — the five-family structure taxonomy + the
  L-notation hierarchy table. The synthesis scaffold T.E extends per-attack. Read; extended, not
  re-derived.

### Workspace edges (no new edge, no new crate, no new dependency)

- **No new edge / dependency.** `criterion` is already a `rho` dev-dependency (three `[[bench]]`
  entries exist). E.W.1 adds a fourth bench (`attacks.rs`) + its manifest entry — a leaf addition to
  the `rho` crate. The docs (`docs/PEDAGOGY.md`, `docs/MATHEMATICS.md`) are pure-prose appends. No
  `Cargo.toml` dependency change; `cargo check --workspace` stays green with no cycle risk. *(E.W
  touches no algorithm code — if a `@build` agent finds it must change a frozen solver surface to bench
  it, that is a discovery surfaced at the ◆, never a silent patch. The benches read the frozen public
  APIs only.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.W.2 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.W.1 | Cross-attack ECDLP benchmark harness + table | done | 1916c65 | C-EWBench |
| E.W.2 ◆ | Track-E code-tour + T.E chapter (MOV payoff) + Track-E close | done | 10c9d54 | C-TrackE |

Contracts frozen before this sub-track: the complete Track-E attack surface — C-Pollard (`rho::ecdlp`,
the generic-√n baseline), C-CompositeCurve / C-FactorOrder / C-Pohlig (`rho::ecdlp::pohlig`,
Pohlig–Hellman), C-FpExt / C-PairingCurve / C-Pairing / C-Mov (`rho::pairing`, pairings + the MOV
bridge), C-Padic / C-Hensel / C-PadicLog / C-Ssa (`rho::ssa`, Smart–Satoh–Araki), the GF(2^m) +
hyperelliptic + GHS surfaces (C-GHSDescent etc.), C-Semaev (`rho::semaev`), and the six index-calculus
contracts (C-IndexCalcStrategy, C-EKRelation, C-PointDecomp, C-RelationCollect, C-EKLinAlg,
C-IndexCalc) — **all read by E.W, none amended**; plus C-Textbook (frozen T.0, the register) and the
§"Escape from Search" through-line. This sub-track **freezes two new contracts** (C-EWBench, C-TrackE),
serving the downstream **Z.1** (umbrella narrative) and **T.Z** (textbook bind), and **closes Track E
end-to-end** — the algebraic-ECDLP attack survey (E.A → E.K), synthesised under the structure-based-
escape-from-search through-line and verified against the project's three scoping principles. **With the
Track-E ◆, Phase δ is complete; only Phase ε (Shor + post-quantum) and Phase ζ (umbrella) remain in the
forward arc.**

---

## Action-frame digest

### E.W.2 inflection — 2026-06-16
Discovery/flex: Inflection fork returned design-confident; C-TrackE resolved interface written into PLAN. MOV placed last as climax (§10.5) rather than first — departs from ch. 6 bullet order but matches PLAN's "MOV is the designated payoff" framing; code-tour per-attack order follows suit.
Affected: C-TrackE (interface resolved, not yet frozen — frozen at E.W.2 ◆ commit)
Deferred: no — design-confident, self-continued. One fiduciary-latitude flag: section ordering (MOV last as climax vs. ch. 6 first-bullet order) surfaced for human awareness, not a halt.
Texture: Design-statement verdict assumed pass/pass/pass but recorded against actual realisation — a divergence at write-time is a discovery for the ◆, not pre-judged.

### E.W.2 ◆ close — 2026-06-16
Discovery/flex: Boundary-transform juncture returned still-on-intent. The committed E.W.2 deliverables (docs-only diff: docs/MATHEMATICS.md ch.10 T.E + docs/PEDAGOGY.md §8–§18 Track-E code-tour; 1212 insertions, zero code) track the Purpose intent on all seven verification points. MOV placed last as the climax (§10.5, full payoff proof given in full — Weil pairing bilinearity/non-degeneracy/Galois-equivariance + the 5-step reduction to F_{p^k}* DLP) per the resolved interface; the design-statement verdict is pass/pass/pass (principles 1/3/4), recorded against the actual realisation with no divergence found. C-EWBench cited accurately; GHS represented honestly as a transfer (no ghs_dlp); index-calculus F_{p^n} lift correctly deferred. C-TrackE freezes.
Affected: C-TrackE (frozen at this ◆, commit 10c9d54). Track E closed end-to-end (E.A→E.K synthesised + verified). Phase δ complete.
Deferred: no halt. One capture candidate surfaced (not a PLAN edit, out of @architect PLAN-write scope): the static-frame ROADMAP Progress/Remaining debt — stale by six sub-tracks (E.F–E.K) + the E.H-before-E.I dependency inversion — is owed at this natural reconciliation point (Track E now complete). User to action via /note or a ROADMAP edit.
Texture: No-regression gate green (cargo test --workspace: all completed crates 0 failed; the run's only timeout was a slow rho timing test, not a failure — consistent with the docs-only diff making the no-regression invariant hold trivially). The boundary-transform verification was the gate; documentation has no cargo gate.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.W is documentation + benchmarking on the frozen Track-E surfaces — internal-continue (confirmed
  by survey).** Every attack solver is frozen and public; the benches read them, the writeup describes
  them. All E.W code is additive (the `attacks.rs` bench + its manifest entry); all E.W prose is
  appended to existing docs. A discovery that benching an attack needs a solver operation the frozen
  surface lacks is an **additive amend** surfaced at the ◆ — not a silent patch.

- **The benchmark table is structural-precondition-conditional, NOT a uniform timing race — the
  load-bearing scope finding.** The Track-E attacks apply only on the curves whose structure they
  exploit (Pohlig–Hellman: composite order; MOV: small embedding degree; SSA: anomalous; GHS: binary;
  index calculus: Semaev-decomposable). The table's pedagogical content is *which structure unlocks
  which escape*, encoded in an "applies?" column — not a fixed-instance race. A `@build` agent racing
  all attacks on one curve produces a wrong table. **Internal-continue → the conditional-table shape
  ratified at C-EWBench.**

- **GHS is a transfer, not an end-to-end solve — represent it honestly (confirmed).** `rho::ghs`
  exposes the descent reduction + log-preservation verification, **no `ghs_dlp`** (the transfer/
  structure/solve framing, NOTES.md). The GHS benchmark measures the *reduction*; the table + chapter
  annotate it as a transfer whose downstream solve is index calculus. A `@build` agent forcing GHS into
  an end-to-end "solve time" column misrepresents it. **Internal-continue → GHS benched/written as a
  transfer.**

- **The index-calculus benchmark counts come from the public re-exports — C-IndexCalc NOT amended (the
  E.K.5-flagged decision, resolved at shard time).** `index_calculus_dlp` returns `Result<Option<u64>>`
  with no counts; `collect_relations` (`.len()`) and `decompose` are public re-exports that supply the
  relation/decomposition counts. E.W derives the counts from them. **C-IndexCalc stays as frozen at
  E.K.5.** *(Risk: if a `@build` agent finds the counts genuinely cannot be derived from the public
  surface, that is an additive-amend discovery at the ◆ — a small first-class stats return on
  C-IndexCalc — not a silent patch. The E.K.5 digest's preference was to decide before E.W shards;
  decided: derive, do not amend.)*

- **T.E extends the §"Escape from Search" through-line; it does not originate one (confirmed).** The
  five-family structure taxonomy + the L-notation hierarchy are frozen at T.0; the MOV reduction is
  *named* but unproven, there is no index-calculus chapter, no ECDLP-attack comparison table. T.E
  develops the five attacks under the frozen scaffold (the MOV proof being the designated payoff). A
  writer re-deriving the through-line instead of extending it is rigidity. **Internal-continue →
  extend, do not re-derive.**

- **The MOV payoff proof is the designated Opus climax (the through-line names it "the cross-track
  bridge of this project").** T.E must develop *why* the pairing reduction works at proof-sketch depth
  (the C-Textbook payoff-depth carve-out). Treating MOV as one bullet among five mis-weights the
  chapter — the cross-track bridge to NFS-DL is the pedagogical climax of the whole project. **The
  per-row Opus tier + the juncture-tier opus are set by this + the integrative load.**

- **The C-Textbook register is frozen (T.0) — no per-chapter level-break (rigidity guard).** The
  audience floor (undergraduate maths), proof-sketch depth (complete, clinical, not exhaustive, not
  inscrutable), through-line (structure-based escape), and MathJax markup are frozen. A chapter that
  needs to break the register (a topic needing graduate background) must surface that as a discovery
  flexing C-Textbook at the ◆ — not a silent raise. **Internal-continue (T.E obeys the register).**

- **The design-statement verification is load-bearing — E.W.2 is "where the design statement is
  verified against the realised Track-E."** The G.W §59 / D.W §69 analogue: the verdict on principles
  1 (algorithmic content complete — all eight attacks implemented head-on), 3 (no engineering
  optimisation crept in), 4 (scale-only phenomena at demonstration fidelity; toy-scale L-notation
  non-observability annotated) is recorded in the action-frame digest. **Internal-continue → verdict
  recorded at the ◆.**

- **The code-tour↔T.E pairing may overrun — surface an E.W.3 split if T.E is too large.** The 2-vs-3
  sizing keeps the code-tour + T.E together in E.W.2 (the G.W / D.W precedent; the τ pairing rationale).
  **If T.E overruns** (the MOV payoff + five-attack development + L-notation comparison push past the
  band), the Track-τ contract's escape applies: split T.E into a dedicated E.W.3 — surfaced as an
  additive-reshard at the ◆ (or by E.W.2 once the chapter's size is concrete), never a silent overrun.
  **Additive-reshard if the chapter overruns.**

- **No new attack / no `F_{p^n}` index-calculus lift / no GHS Jacobian solver in E.W (defocus / scope
  clarity — the deferred re-shards).** The extension-field index-calculus asymptotic-win lift and the
  GHS-coupled binary-curve end-to-end are **later, separately-sharded sub-tracks** (named at the E.K
  close), each flexing C-Semaev. A `@build` agent adding an attack or lifting Semaev in E.W is defocus —
  E.W is the *close*, not an extension.

- **Toy-scale costs only — the asymptotic L-notation separations are NOT observable (principle-4
  boundary, both directions).** The benchmark reports toy-scale wall-clock; the asymptotic separations
  between the attacks (rho `L[1, 1/2]` vs index calculus subexponential vs MOV reducing to `L[1/3]` vs
  SSA polynomial) are not visible at `p = 47` / `secp_k1_toy`. The principle-4 note (BENCHMARKS.md) +
  the L-notation comparison (T.E) carry the asymptotic picture. Presenting the toy timings as the
  asymptotic ranking is a documentation defect. **Internal-continue → corrected (toy costs + asymptotic
  chapter).**

- **Static-frame ROADMAP debt (surface at the E.W ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried + compounded from the E.I, E.H, E.J, and E.K ◆, and now AT THE NATURAL
  RECONCILIATION POINT.** The ROADMAP Progress subsection is stale by **six** completed sub-tracks
  (E.F, E.G, E.H, E.I, E.J, E.K; the table shows "Done ~32 (E.A–E.J)" and lists E.K + E.W as remaining);
  the Remaining table lists the now-complete E.K; and the Remaining table historically listed **E.H
  before E.I** (dependency-inverted — E.I shipped first, E.H consumed it; the E.K shard already recorded
  the correction). The E.I ◆ digest recorded this as owed, the E.H ◆ re-recorded it, the E.J ◆ + E.K ◆
  flagged it again. **The Track-E ◆ close (E.W.2) is the natural reconciliation point** — Track E is now
  complete end-to-end, so the full Track-E Progress/Remaining reconciliation (Track E Done → E.A–E.W,
  ~34; strike all E rows from Remaining; record the E.I-before-E.H correction; mark Phase δ complete) is
  owed at this boundary. **This is a ROADMAP write — outside the `@architect` PLAN-only write scope;
  surfaced here as a capture candidate for the user to action (via `/note` or a ROADMAP edit), not a
  PLAN edit.** Not an implementation concern; does not block E.W.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.W, "*Cross-attack benchmarks + Track E writeup. 1-2 sessions.
  Predecessor: most of E. The 'which attack wins on which curve' table; the pedagogical synthesis of
  structure-based escape from search. Opus-tier — high integrative judgment load.*"; the Track-τ scope
  contract — T.E folds into E.W, the MOV reduction is the Opus payoff; the Opus-flagged-sessions table —
  `E.W | Cross-attack synthesis` + `T.E (MOV reduction)`) and this PLAN before any session. **NOTE: the
  ROADMAP Progress / Remaining tables are stale (E.F–E.K done; E.K + the completed E rows still listed
  as remaining) AND historically listed E.H before E.I (dependency-inverted); the Track-E ◆ close
  (E.W.2) is the natural reconciliation point — surface it at the ◆, but it is outside `@architect`
  PLAN-write scope (a capture candidate for the user).**
- Read the **templates to mirror**: `docs/BENCHMARKS.md` (the per-sub-track section genre — prose setup
  + table + "science↔engineering note (principle 4)"; the `## G.W` section, lines 408–453, is the
  closeout template); `rho/benches/ecdlp.rs` (the Criterion `harness = false` bench idiom E.W.1's
  `attacks.rs` mirrors — and the existing rho timing baseline to cite, not duplicate); `gnfs/docs/
  PEDAGOGY.md` §52–§62 (G.W) + §63–§71 (D.W) (the `*.W` code-tour genre — pipeline/attacks at a glance,
  per-stage tour, cross-phase contracts, design-statement verification §59/§69, KAT summary — E.W's
  code-tour mirrors this in `docs/PEDAGOGY.md`); `docs/MATHEMATICS.md` §"Escape from Search" (the frozen
  through-line T.E extends) + the existing chapters 3–7 (the maths-chapter genre + the C-Textbook
  register); `docs/PEDAGOGY.md` Phases 0–8 (the existing rho code-tour the Track-E tour appends after).
- **Register:** E.W.1 is **Rust benchmark code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; Criterion
  idiom; the new file `rho/benches/attacks.rs` + the `[[bench]]` manifest entry) **plus prose** (the
  BENCHMARKS.md section). E.W.2 is **pure documentation** (`STYLE-DOC.md`; the code-tour in
  `docs/PEDAGOGY.md` + T.E in `docs/MATHEMATICS.md`, both at the frozen C-Textbook register: MathJax
  markup, proof-sketch depth, undergraduate-maths audience floor).
- **Tier routing:** **E.W.1 is Sonnet `@build`** (mechanical Criterion bench-wiring over frozen solver
  APIs + the BENCHMARKS.md table — no design crux; the only judgment is the table shape, which the
  C-EWBench ratification fixes). **E.W.2 is Opus `@architect`** (the ROADMAP's native Opus flag — the
  cross-attack synthesis + the MOV payoff proof + the design-statement verification + the ◆ close;
  carries the **◆ `@architect` juncture**, page `@plan-juncture`). juncture-tier (header) is **opus** —
  set by the native Opus flag on E.W + the T.E MOV-payoff designation + lever 4 (the Track-E ◆
  adjudicates the whole E.A–E.K arc). Lever 5 is weak (prose has no fast self-checking oracle; the
  benchmark KATs are deterministic but do not adjudicate the synthesis), so it grants no license to opt
  the juncture down — the inverse of a strong-lever-5 sub-track.
- **Invariants to preserve:** **E.W adds NO new attack** (it is the Track-E close — the `F_{p^n}`
  index-calculus lift + the GHS Jacobian solve are deferred re-shards). **E.W amends NO algorithm
  contract** (every Track-E solver surface is read; the only code is additive Criterion benches + their
  manifest entry; the docs are prose appends). **The benchmark table is structural-precondition-
  conditional, NOT a uniform race** (the "applies?" column is the pedagogical spine). **GHS is a
  transfer, not an end-to-end solve** (benched/written as a reduction; downstream solve is index
  calculus). **Index-calculus counts come from the public `collect_relations`/`decompose` re-exports**
  (C-IndexCalc unamended — the E.K.5-flagged decision). **T.E extends the frozen §"Escape from Search"
  through-line + obeys the frozen C-Textbook register** (no re-derivation, no per-chapter level-break;
  MathJax markup). **The MOV payoff proof is the designated Opus climax.** **The design-statement
  verdict (principles 1/3/4) is recorded** (the G.W §59 / D.W §69 analogue). Toy-scale costs only;
  asymptotic L-notation separations annotated as non-observable (principle 4).
- **No new edge, no new crate, no new dependency (load-bearing for E.W).** `criterion` is already a
  `rho` dev-dep; the new bench is a leaf addition + a `[[bench]]` manifest entry. The docs are
  pure-prose appends to existing files. `cargo check --workspace` stays green with no cycle risk; the
  no-regression invariant (existing KATs green) holds trivially since no solver path changes.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  benchmark-harness session + a paired code-tour/maths-chapter closeout session) is **established for
  this project** (G.W and D.W both ran the closeout-writeup pattern; the only novelty is the
  benchmark-harness front-half, which is mechanical Criterion wiring). The closeout writeup (E.W.2) is
  the high-judgment ◆ — the cross-attack synthesis + the MOV payoff + the design-statement verdict +
  the Track-E close all land there, so halt at the E.W.2 boundary for the human glance. E.W.1 (the
  benches) is mechanical and could run autonomously, but `halt-at-boundaries` is the conservative
  default for the first invocation; the E.W.2 ◆ fork is itself a halt. *(Tradeoff vs autonomous:
  `halt-at-boundaries` trades a little velocity on the mechanical E.W.1 for a guaranteed human check at
  the Track-E ◆ — the right trade for a track-closing synthesis with the project's pedagogical climax
  (the MOV proof) in it. If E.W.1 lands cleanly and its benches confirm every attack still solves, the
  E.W.2 ◆ is where the judgment concentrates.)*
