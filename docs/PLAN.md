<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-G closeout + Track-τ open (G.W, T.G; T.0 spine)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default; does not opt down.** Applying the
five-lever law to this bundle: lever 3 (design-error cost) is **high** for T.0 — it freezes
**C-Textbook**, the cross-track documentation-register contract that *every* later chapter
(T.D / T.E / T.S / T.Z, and the recommended `*.W` code-tours) obeys; getting the register wrong is
"expensive to retrofit across all chapters" (ROADMAP Phase τ). Lever 2 (irreducible complexity) is
**high** at two payoff proofs — the **L-notation subexponentiality derivation** (G.W code-tour +
T.G textbook) is a designated payoff proof the ROADMAP flags Opus. Lever 5 (inner-loop bandwidth)
is the decisive asymmetry here, and it points the *wrong way for opting down*: this is a
**docs-dominant** bundle, and `cargo test --workspace` does **not** gate prose quality, register
fidelity, or proof correctness. There is effectively **no fast inner loop** for the load-bearing
content. Per the planning law, opting the juncture tier down to Sonnet is licensed only when *strong*
test-suite quality coincides with low correctness-criticality; here the inner loop is *weak/absent*
for the work that matters, so lever 5 does **not** license opting down — it reinforces holding at
Opus. Levers 1 (ambient complexity) and 4 (correctness-criticality) are low-moderate (the code is
frozen and clean; prose has no silent-failure seam), but they do not overturn levers 2/3/5.

**Roadmap-frame flex (logged, additive).** The ROADMAP schedules G.W as a standalone 1-session
Opus writeup and folds T.G into it "at the G ◆ boundary," but T.G obeys **C-Textbook**, which **T.0
freezes** — and T.0 has not yet run (no `docs/MATHEMATICS.md` / `docs/textbook/` exists). The
G.W↔T.G pairing the ROADMAP names is therefore **blocked on T.0**. This plan resolves the
open boundary-transform decision (ROADMAP Discoveries: "whether T.0 runs early or is deferred")
by **running T.0 first**, then the paired G.W + T.G, as a **3-session Track-G-closeout /
Track-τ-open bundle**. The flex is **additive** (sequence the spine ahead of its first consumer; no
contract break) and is surfaced for the ROADMAP Discoveries log at the T.G ◆ boundary (see
Discoveries).

Last rewrite: G.F ◆ boundary crossed (G.F.W landed at `f7ebe1d`; G.F ledger still-on-intent
2026-06-07, `e870c82`). **G.F fully complete** (G.F.1 → G.F.W); **C-AlgSqrt frozen** (Couveignes CRT
algebraic square root, D.B-consumable); the GNFS factoring pipeline proper (G.A → G.F) is complete
end-to-end. This plan opens the **Track-G closeout + Track-τ spine** — the integrative writeups that
articulate the whole GNFS arc and establish the standing mathematical textbook.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms, complemented by a maths-first standing textbook (Track τ). This bundle produces the
**integrative articulation** of the now-complete GNFS factoring pipeline and **establishes the
textbook spine** every later chapter obeys. Three sessions:

1. **The textbook spine (T.0).** Establish the standing mathematical textbook (genre:
   `tetratile/docs/mathematics.rst` — maths-first, code-second, learnable on its own), distinct
   from the code-tour `PEDAGOGY.md` chapters. **Freeze C-Textbook** (audience: undergraduate maths
   background; depth: survey with proof-sketch depth — complete and clinical, not exhaustive, not
   inscrutable; through-line: *structure-based escape from search*; markup: Markdown + MathJax,
   ratifying the G.C-boundary recommendation). Write the table of contents across the whole survey,
   the escape-from-search framing chapter, the **prerequisites chapter** (the undergraduate-bridge:
   the specific algebra/analysis/probability/logic theorems later chapters lean on), and the **"On
   scale" interlude** (the full natural-philosophy exposition the ROADMAP `## On scale` section
   defers here). **Retrofit** the existing `rho` (ECDLP) and α-substrate code-tours into textbook
   chapters (or chapters that cite them), establishing the chapter-pairing pattern every later
   `*.W` follows.

2. **The GNFS integrative writeup (G.W).** The whole-pipeline code-tour chapter
   (`gnfs/docs/PEDAGOGY.md`): polynomial selection → sieving → filtering → linear algebra → square
   root → factor, finally visible end-to-end in one narrative. The moment where the cross-phase
   contracts (C-PolyPair, C-Score, C-Relation, C-FactorBase, C-Matrix, C-LinAlg, C-AlgSqrt) get
   their **public articulation** and where the **design statement is verified against the actual
   implementation** (ROADMAP: G.W "is the moment where the design statement is verified against the
   actual implementation"). Carries the **L-notation complexity analysis** of GNFS as a payoff
   derivation.

3. **The Track-G textbook chapter (T.G ◆).** The maths-first sibling to G.W in the standing
   textbook: the GNFS chapter written for a reader who does *not* already know the implementation.
   Carries the **L-notation subexponentiality derivation** as a complete payoff proof (the ROADMAP
   names this an Opus payoff) and the Couveignes-correctness sketch. Cross-references G.W (code) and
   T.0 (spine). This session crosses the **Track-G ◆ boundary**.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating beyond survey-with-
proof-sketch depth — encyclopaedic case enumeration, full proofs where a sketch suffices, ROADMAP
C-Textbook depth contract) and **rigidity** (forcing the ROADMAP's "G.W as a lone session" when the
T.0-dependency evidence shows the spine must precede the paired chapter).

**Scoping discipline (ROADMAP three-way split, applied here).** This bundle is *integrative and
expository*: no new algorithmic content (G.A → G.F is complete). The payoff proofs (L-notation
subexponentiality) are **mathematical content included in full** (principle 1 — the proof *is* the
payoff). The "On scale" interlude and the principle-4 disconnect annotations are the **honest
science↔engineering gap** narration (principle 4). No new engineering optimizations (principle 3).
CADO-NFS / msieve remain dev-only oracles, unchanged.

---

## Current state

Phase α + Track G sub-tracks G.A, G.B, G.C, G.D, G.E, **G.F complete**. Workspace crates:
`shared/field`, `shared/bigint`, `shared/numfield`, `shared/numth`, `rho`, `gnfs`.
`cargo test --workspace` green at `e870c82`.

**Existing documentation corpus** (the substrate T.0/G.W/T.G work over):
- `docs/PEDAGOGY.md` — Pollard rho for ECDLP code-tour (§1–§7, ~487 lines). **T.0 retrofits.**
- `shared/numth/docs/PEDAGOGY.md` — α-substrate code-tour (§1–§8, ~438 lines; ECM, Miller–Rabin,
  smoothness, batched inversion, Tonelli–Shanks). **T.0 retrofits** (S0.W backfill, done).
- `shared/numfield/docs/PEDAGOGY.md` — G.A number-field code-tour (~742 lines). **G.W/T.G cite.**
- `gnfs/docs/PEDAGOGY.md` — G.B–G.F code-tours (5 chapters §1–§51, ~3532 lines: polyselect,
  sieving, filtering, linear algebra, square root). **G.W appends the integrative chapter; T.G is
  its maths-first sibling.**

**No textbook artifact exists yet** — `docs/MATHEMATICS.md` and `docs/textbook/` are both absent.
T.0 creates it (location decided at T.0 — see C-Textbook §location).

**Known stale-doc cleanup (G.W-era, additive).** `gnfs/src/lib.rs:17–19` still describes the `sqrt`
module's algebraic/assembly entry points as "stub" (stale since G.F.3/G.F.4 landed). G.W's
design-statement-verification pass should correct this docstring (a `cargo check`-touching edit,
trivially part of the writeup unit) — surfaced, not silently grown.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). **This is a docs-dominant bundle:** T.0 and T.G are pure prose (new `.md` files,
no code), and G.W is prose + the one stale-docstring cleanup in `gnfs/src/lib.rs`. The VERIFY gate
therefore guards only that the docstring edit keeps the doc-test/build green; **it does not and
cannot gate prose quality, register fidelity, or proof correctness** — which is precisely why the
juncture tier holds at Opus (lever 5: no inner loop for the load-bearing content). No workspace
`Cargo.toml` change is required. New textbook `.md` files are not compiled.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| T.0 `@plan` | Textbook spine: freeze C-Textbook + ToC + escape-from-search framing + prerequisites + "On scale" interlude + rho/α retrofit | A | **Opus** | (existing rho + α code-tours) | new `docs/MATHEMATICS.md` (spine + framing + prerequisites + On-scale + retrofit chapters), `docs/PEDAGOGY.md` (cross-reference to textbook), `shared/numth/docs/PEDAGOGY.md` (cross-reference) |
| G.W | GNFS integrative writeup: whole-pipeline code-tour + design-statement verification + L-notation | I | **Opus** | C-PolyPair, C-Score, C-Relation, C-FactorBase, C-Matrix, C-LinAlg, C-AlgSqrt | `gnfs/docs/PEDAGOGY.md` (append integrative chapter after §51), `gnfs/src/lib.rs` (correct stale "stub" docstring), `docs/BENCHMARKS.md` (append pipeline-wide row) |
| T.G ◆ | Track-G maths-first textbook chapter (L-notation subexponentiality payoff proof + Couveignes sketch) | I | **Opus** | C-Textbook, G.W | `docs/MATHEMATICS.md` (append the GNFS chapter) |

**Sequencing notes.** **T.0 must precede T.G** (T.G obeys C-Textbook, frozen by T.0) — this is the
hard dependency that drives the bundle order. **G.W and T.G pair** (the ROADMAP's writeup-pairing:
code-tour + maths-first siblings written while both are fresh), but T.G additionally consumes
C-Textbook, so the order is T.0 → G.W → T.G. G.W is independent of T.0 in principle (it is a
code-tour, not a textbook chapter) and *could* run first; the listed order front-loads the spine so
the C-Textbook register is frozen before *either* chapter is written, keeping the G.W code-tour and
T.G textbook chapter mutually consistent in notation and framing from the start. The single `@plan`
marker sits on **T.0** (a post-landing C-Textbook freeze confirmation — the register binds every
future chapter cross-track, so confirm the freeze before T.G consumes it). **T.G ◆** is the
Track-G boundary.

**Why 3 sessions (ROADMAP folded T.G into G.W).** The one-line-commit-title corollary: T.0
("textbook spine + register freeze") and G.W ("GNFS integrative writeup") and T.G ("Track-G maths
chapter") are three distinct commit titles. The ROADMAP's "T.G pairs with G.W, ~0 net new sessions"
assumed C-Textbook was already frozen; it is not, so **T.0 is the net-new session** the ROADMAP
front-loads exactly once ("T.0 is the only Track-T session that adds calendar time up front"). T.0
and T.G are **not** mergeable (different artifacts: spine+retrofit vs the GNFS chapter; and T.G
cannot be written before T.0 freezes its register). G.W and T.G are **not** mergeable (different
genres: code-tour in `PEDAGOGY.md` vs maths-first in `MATHEMATICS.md` — the ROADMAP's core Track-τ
distinction). Each is one clean title.

**Flagged overrun-split points (contract-sharp, only if a session exceeds the band).**
- **T.0** is a front-loaded substrate session (ROADMAP: T.0 "adds calendar time up front"; planning
  doc: substrate sessions run 1.5–2×). If it overruns, the **rho/α retrofit chapters** are the
  contract-sharp split: the spine (freeze C-Textbook + framing + prerequisites + On-scale) is the
  irreducible unit; the retrofit consumes the just-frozen register and can fold into a follow-on
  `T.0b` without fracturing the freeze. Surface at the T.0 `@plan` juncture if it lands oversized.
- **T.G** is the larger of the G.W/T.G pair (ROADMAP: "the textbook chapter is the larger of the
  two and may push toward the top of the band or split into a dedicated follow-on if it overruns —
  decided at the boundary"). If it overruns, the contract-sharp split is the **L-notation payoff
  proof** (a self-contained derivation) vs the GNFS-chapter narration — decided at the T.G ◆
  boundary.

---

## Session detail

T.0 is crisp (its design surface is the C-Textbook contract, resolved in-session as the spine's own
work). G.W and T.G are sketched at integrative fidelity — their precise shape is correct to leave
open until T.0 freezes the register and G.W's design-statement-verification pass reports.

### T.0 — Textbook spine: freeze C-Textbook + framing + prerequisites + On-scale + rho/α retrofit (Opus, substrate, `@plan`)

**Deliverable:** the standing mathematical textbook's spine.
- **Create the textbook artifact.** Default `docs/MATHEMATICS.md` (single-file start; promote to
  `docs/textbook/` only if it outgrows single-file — decide in-session and record in C-Textbook).
- **Freeze C-Textbook** (the documentation-register contract — see Cross-session contracts): the
  audience floor (undergraduate maths: proofs + intro analysis/algebra/probability/logic), the
  depth (survey with proof-sketch depth — complete, clinical; full proofs only where the proof is
  the payoff), the through-line (*structure-based escape from search*), and the markup (Markdown +
  MathJax — `$…$` inline, `$$…$$` display; ratifying the G.C-boundary recommendation, superseding
  "rST or Markdown TBD").
- **Table of contents** across the whole survey (rho, α-substrate, GNFS, NFS-DL, ECDLP attacks,
  Shor, PQ) — the chapter skeleton every `*.W` later fills.
- **Escape-from-search framing chapter** — the through-line stated once: every attack finds
  exploitable structure (homomorphism, smoothness, endomorphism, pairing, quantum period) that
  escapes the generic √n / L-notation search bound.
- **Prerequisites chapter** — the undergraduate-background bridge: the specific theorems from
  algebra/analysis/probability/logic the later chapters lean on.
- **"On scale" interlude** — the full natural-philosophy exposition the ROADMAP `## On scale`
  section defers here (three axes — resource/operational, mathematical-dimension, structural; the
  three couplings; method-convergence vs problem-openness).
- **Retrofit** the `rho` ECDLP and α-substrate code-tours into textbook chapters (or chapters that
  cite them), establishing the **chapter-pairing pattern** (maths-first chapter ↔ code-tour) that
  T.G and every later `*.W` follows.

**Key design decisions (the C-Textbook freeze surface — the `@plan` confirmation):**
1. **Artifact location:** `docs/MATHEMATICS.md` vs `docs/textbook/`. Bias single-file until it
   grows; record the chosen path in C-Textbook.
2. **Markup ratification:** Markdown + MathJax (recommended at G.C ◆). The `@plan` confirmation
   ratifies the freeze or reopens only on a hard MathJax limitation (ROADMAP Discoveries).
3. **Register depth calibration:** "survey with proof-sketch depth" against the *actual* rho/α
   content being retrofitted — the first real test of whether the level is right before it binds
   every later chapter. The cross-track cost of a wrong register is the reason for the `@plan`.

**KAT (≥1 required) — prose-contract analogue.** A textbook has no `cargo` KAT; the contract is
**prose-enforced** (C-Textbook). The session's verification obligation is: (a) every C-Textbook
clause (audience/depth/through-line/markup/location) is stated explicitly in the spine and is
self-consistent; (b) the retrofit rho/α chapters demonstrably obey the just-frozen register (the
pairing pattern is exhibited, not merely asserted); (c) MathJax renders (spot-check a display
expression). *A row whose deliverable can't be a KAT has an undefined contract* — here the
"KAT" is the register-conformance check, which is the prose-contract enforcement mechanism.

**Subtlety:** C-Textbook is **cross-track** (every chapter in T.D/T.E/T.S/T.Z obeys it) and
**prose-enforced** (nothing automatic catches drift). Over-specify deliberately: state the audience
floor and depth precisely enough that a later chapter that *needs* to break the register (a topic
requiring graduate background) must surface it as a discovery and flex C-Textbook at an inflection
review, not silently raise the level. This is the substrate-over-specify rule applied to a
documentation contract.

**Deferred:** the GNFS code-tour (G.W); the GNFS textbook chapter (T.G); all later per-track
chapters (T.D/T.E/T.S, folded into their `*.W` siblings).

**`@plan` confirmation (post-landing, T0/Opus, one-shot).** Page a `@plan-juncture` fork to
confirm the **C-Textbook freeze** before T.G (its first consumer) is dispatched: (1) the register
clauses are complete and mutually consistent; (2) the markup choice is ratified (no hard MathJax
limitation surfaced); (3) the retrofit chapters genuinely exhibit the pairing pattern; (4) the
artifact-location choice is recorded. The fork returns one-shot findings; it does not implement.
This is the contract-freeze confirmation the planning doc prescribes for a cross-track substrate.

### G.W — GNFS integrative writeup: whole-pipeline code-tour + design-statement verification + L-notation (Opus, integrative, sketch)

**Deliverable:** the GNFS integrative chapter of `gnfs/docs/PEDAGOGY.md` (append after §51 as a new
`# The GNFS Pipeline End-to-End: An Integrative Chapter`). Sketch (crisp shape resolved in-session
once T.0's register is frozen and the verification pass reports):
- **The whole pipeline in one narrative:** polynomial selection (G.B) → sieving (G.C) → filtering
  (G.D) → linear algebra (G.E) → square root + assembly (G.F), traced as a single data-flow from N
  to a factor, with each stage's cross-phase contract named in prose for the first time
  (C-PolyPair, C-Score, C-Relation, C-FactorBase, C-Matrix, C-LinAlg, C-AlgSqrt).
- **Design-statement verification.** The ROADMAP names G.W as "where the design statement is
  verified against the actual implementation." Walk the three-way scoping split (algorithmic
  content complete; scale-only at demonstration fidelity; engineering omitted) against what G.A–G.F
  actually shipped, and **report any divergence as a discovery** (additive-reshard if a gap; not a
  silent pass). Includes correcting the stale `gnfs/src/lib.rs:17–19` "stub" docstring.
- **L-notation complexity analysis** of GNFS (L_N[1/3, (64/9)^{1/3}]) — the heuristic-complexity
  derivation as a payoff, cross-referencing the fuller T.G treatment.
- **Benchmark row.** Append a pipeline-wide row to `docs/BENCHMARKS.md` (end-to-end factor: N
  bit-length / stage timings / factor recovered).

Consumes all Track-G contracts (reads; freezes nothing new — integrative).

**KAT (≥1 required) — prose-contract analogue + the one code touch.** (a) Every named cross-phase
contract in the chapter matches its frozen definition (cross-check against Cross-session contracts +
the per-stage `PEDAGOGY.md` chapters); (b) the design-statement-verification produces an explicit
pass/divergence verdict for each of the three scoping principles; (c) the `cargo test --workspace`
gate stays green after the lib.rs docstring correction (the one behavioural check).

**Subtlety:** the design-statement verification is the load-bearing judgment, not the narrative
summary (planning doc: integrative writeups are "where the cross-track implications are surfaced,"
consistently under-scheduled). If the verification finds a real divergence (e.g. a scale-only
phenomenon implemented below demonstration fidelity, or an engineering optimization that crept in),
that is a **discovery** for the ROADMAP Discoveries log, surfaced at the T.G ◆ boundary — not
papered over to keep the writeup clean.

### T.G ◆ — Track-G maths-first textbook chapter (Opus, integrative, sketch)

**Deliverable:** the GNFS chapter of `docs/MATHEMATICS.md` — the maths-first sibling to G.W, written
for a reader who does not already know the implementation (the C-Textbook register). Sketch:
- The GNFS algorithm developed mathematically from the difference-of-squares / congruence-of-squares
  idea through the number-field bridge, the factor-base smoothness phenomenon, the linear-algebra
  dependency, and the square-root recovery — *structure-based escape from search* as the through-line.
- **L-notation subexponentiality derivation as the payoff proof** (ROADMAP: Opus). The full
  derivation of L_N[1/3, (64/9)^{1/3}] — why the smoothness-probability / sieving-cost tradeoff
  optimises to exponent 1/3 — stated and proved at the depth the C-Textbook depth contract reserves
  for payoff proofs (complete, not a sketch).
- **Couveignes-correctness sketch** (the algebraic square root) at proof-sketch depth, cross-
  referencing the G.F code-tour.
- Cross-references: cites G.W (the code realisation) and the T.0 prerequisites/On-scale chapters.

Consumes **C-Textbook** (register) and **G.W** (the code-tour it pairs with). Freezes nothing new.

**KAT (≥1 required) — prose-contract analogue.** (a) The chapter obeys C-Textbook (audience floor,
depth, through-line, markup) — register-conformance check; (b) the L-notation derivation is
complete and self-contained (a reader with the prerequisites can follow it without the code); (c)
cross-references to G.W and T.0 resolve.

**Subtlety:** this is the **first textbook chapter to consume C-Textbook** — it is the real test of
whether the T.0 register holds. If the GNFS chapter cannot be written at "survey with proof-sketch
depth" without either over-running into exhaustiveness or under-running into inscrutability, that is
a **C-Textbook flex** to surface at the ◆ boundary (a register discovery), not a silent level
change. This is also the **Track-G ◆ boundary** — re-read the Purpose intent and verify the whole
Track-G arc (G.A → G.W → T.G) is coherent before crossing into Track γ (D.*) or a later Track-τ
chapter.

---

## Cross-session contracts

The integrative sessions read the frozen Track-G contracts and articulate them in prose; they
freeze no new *code* contracts. The one new contract is **C-Textbook** (documentation-register),
frozen at T.0.

### C-Textbook — textbook documentation-register contract (prose-enforced) — *to be frozen at T.0*

**Defined:** T.0. **Consumed by:** T.G (and every later textbook chapter — T.D, T.E, T.S, T.Z — and,
as a recommendation, the `*.W` `PEDAGOGY.md` code-tours). **Cross-track** (every chapter obeys it),
so over-specified deliberately at T.0. Prose-enforced — nothing automatic catches drift; a later
chapter that needs to break the register must surface it as a discovery and flex C-Textbook at an
inflection review, not silently raise the level.

*To be frozen at T.0* — the resolved clauses (audience floor / depth / through-line / markup /
artifact location) are written into this subsection by the T.0 session and confirmed at its `@plan`
juncture. Provisional content (from the ROADMAP Phase τ scope contract, pending T.0 ratification):
- **Audience:** interested maths student (human or agent), undergraduate maths background — comfort
  with proofs; intro analysis, algebra, probability, logic. Anything beyond is built up or cited.
- **Depth:** survey with proof-sketch depth. Key theorems stated and motivated; proofs are sketches
  with citations, *except* where the proof is the pedagogical payoff (L-notation subexponentiality
  in T.G/T.D; the MOV reduction in T.E — full proofs there). Complete (no key idea silently
  omitted), academic and clinical, **not** exhaustive, **not** inscrutable (intuition leads, rigour
  follows).
- **Through-line:** structure-based escape from search.
- **Markup:** Markdown + MathJax (`$…$` / `$$…$$`). Supersedes "rST or Markdown TBD." Ratified at
  T.0's `@plan` confirmation; reopened only on a hard MathJax limitation.
- **Artifact location:** `docs/MATHEMATICS.md` (single-file) or `docs/textbook/` (if it grows) —
  *decided and recorded at T.0.*

### Frozen Track-G contracts (read by G.W / T.G; not amended)

These are stable; G.W articulates them in prose and T.G cites their mathematics. None is amended by
this bundle.

- **C-PolyPair** — polynomial pair + number-field constructor (compiler + KAT) — *frozen G.B.1
  (2f43f99)*. G.W: the polynomial-selection stage.
- **C-Score** — Murphy-E scoring (compiler + KAT) — *frozen G.B.2 (00aa32d)*. G.W: poly-selection
  quality; C3 cross-track note (D.A / E.K reuse).
- **C-Relation** — relation / exponent-vector format (compiler + KAT) — *frozen G.C.1 (c1dc0b6)*.
  G.W: the sieving stage's data unit.
- **C-FactorBase** — two-sided factor base + sign/QC columns (compiler + KAT) — *frozen G.C.1
  (c1dc0b6)*. G.W: smoothness + why the QC/sign columns exist.
- **C-Matrix** — filtered sparse GF(2) matrix + provenance map (compiler + KAT) — *frozen G.D.1
  (a0e854b)*. G.W: the filtering stage + the provenance thread to G.F.
- **C-LinAlg** — GF(2) nullspace substrate: blocked vectors + kernel representation (compiler +
  KAT) — *frozen G.E.1 (416f6db)*. G.W: the linear-algebra stage; the `expand_provenance` seam.
- **C-AlgSqrt** — Couveignes algebraic-square-root contract (compiler + KAT) — *frozen G.F.3
  (c80a855 + ec69a1f)*. G.W: the square-root stage; D.B re-consumption note.

(Plus the G.A substrate contracts — C-NF, C-Res, C-numth, C-Ideal, C-Dedekind — and C1
`shared::numth::is_smooth`, read but not foregrounded in this bundle.)

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The T.0 `@plan` confirmation is not a ledger row (a
paged fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| T.0 | Textbook spine: C-Textbook + framing + prerequisites + On-scale + rho/α retrofit | done | 5c9b783 | C-Textbook |
| G.W | GNFS integrative writeup + design-statement verification + L-notation | pending | — | — |
| T.G | Track-G maths-first textbook chapter (L-notation payoff proof) | pending | — | — |

Contracts frozen before this bundle: C-Fp (cf00ed5 / α.5), C-numth (α.2), C-NF (bdba6f5 / 7844773),
C-Ideal (05b27c8), C-Res (bcd63cd), C-Dedekind (7844773), C-PolyPair (2f43f99), C-Score (00aa32d),
C-FactorBase (c1dc0b6), C-Relation (c1dc0b6), C-Matrix (a0e854b), C-LinAlg (416f6db), C-AlgSqrt
(c80a855 + ec69a1f). This bundle opens over the complete, frozen GNFS factoring pipeline (G.A → G.F)
and freezes one new documentation-register contract (C-Textbook, at T.0).

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume — including the **T.0 `@plan` confirmation
outcome** (the C-Textbook freeze verdict) and any **G.W design-statement-verification divergence**,
recorded here rather than in the ledger.

### T.0 — 2026-06-08
Discovery/flex: C-Textbook freeze confirmed at `@plan` juncture (design-confident). All four confirmation points satisfied: register clauses complete and mutually consistent, MathJax ratified, retrofit chapters exhibit the pairing pattern, artifact location recorded.
Affected: C-Textbook (now frozen at 5c9b783)
Deferred: no — the register is well-calibrated; T.G can proceed. If the GNFS chapter cannot hold "survey with proof-sketch depth," that is a C-Textbook flex to surface at the T.G ◆ boundary.
Texture: Single-file artifact location (docs/MATHEMATICS.md) confirmed; promotion to docs/textbook/ deferred to T.Z. Register calibration verified against rho/α content — "survey with proof-sketch depth" exhibited correctly in both retrofit chapters.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **Roadmap-frame flex: T.0 runs before the G.W↔T.G pairing (surface at the T.G ◆ boundary for the
  ROADMAP Discoveries log).** The ROADMAP folds T.G into G.W "at the G ◆ boundary" assuming
  C-Textbook is already frozen; it is not (no textbook artifact exists). This plan runs T.0 first,
  resolving the ROADMAP's open question ("whether T.0 runs early or is deferred until more chapters
  exist to calibrate the register"). **Additive** (sequence, no contract break). Log at the ◆
  boundary; this answers the open question rather than breaking the roadmap.

- **C-Textbook is a cross-track prose-enforced contract — over-specify, surface flexes, never raise
  the register silently (lever 3).** Frozen at T.0, consumed by every later textbook chapter
  (T.D/T.E/T.S/T.Z). A later chapter that needs graduate-level depth must flex C-Textbook at an
  inflection review, not silently raise the level in one chapter. T.G is the first consumer and the
  first real test of the register; if the GNFS chapter cannot hold "survey with proof-sketch depth,"
  that is a **C-Textbook flex** (additive-reshard), surfaced at the ◆ boundary.

- **Markup ratification (MathJax) is a T.0 freeze, not a re-litigation.** The G.C ◆ boundary
  recommended Markdown + MathJax (ROADMAP Discoveries); T.0 ratifies it. Reopen only on a hard
  MathJax limitation (a renderer the project must target that lacks MathJax) — otherwise the freeze
  stands. Not a code contract; gates no implementation, only the textbook's and `*.W` chapters'
  prose rendering.

- **Design-statement verification may surface a divergence (G.W, additive-reshard).** G.W's mandate
  to verify the design statement against the implementation may find a scale-only phenomenon
  implemented below demonstration fidelity, or an engineering optimization that crept in. If so,
  that is an **additive-reshard discovery** (a corrective follow-on session), surfaced at the T.G ◆
  boundary — not silently passed. Only a divergence requiring a *change to a frozen contract* would
  be a destructive-HALT (not expected — the pipeline is complete and KAT-green).

- **Stale `lib.rs` "stub" docstring (G.W cleanup, additive).** `gnfs/src/lib.rs:17–19` describes the
  `sqrt` algebraic/assembly entries as "stub," stale since G.F.3/G.F.4 landed. G.W corrects it as
  part of the design-statement-verification pass — a trivial `cargo check`-touching edit, plainly
  part of the writeup unit; surfaced here, not silently grown.

- **No fast inner loop for prose (lever 5, holds juncture-tier at Opus).** Unlike every prior
  Track-G sub-track, this bundle's load-bearing content (register fidelity, L-notation proof
  correctness, design-statement verification) is **not** gated by `cargo test`. The inner control
  loop is weak/absent, which is the *opposite* of the opt-down condition — so juncture-tier holds at
  Opus and the T.0 `@plan` confirmation is the compensating control for the C-Textbook freeze.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase τ scope contract / C-Textbook provisional clauses; the G.W entry
  "where the design statement is verified against the actual implementation"; the documentation-
  format Discoveries entry recommending MathJax; the "On scale" section deferred to T.0) and the
  prior `docs/PLAN.md` G.F history before any session in this bundle. **Note the roadmap-frame flex**
  (T.0 runs before the G.W↔T.G pairing — logged in Discoveries above).
- Read the existing documentation corpus T.0/G.W/T.G work over: `docs/PEDAGOGY.md` (rho ECDLP code-
  tour, retrofitted by T.0), `shared/numth/docs/PEDAGOGY.md` (α-substrate, retrofitted by T.0),
  `shared/numfield/docs/PEDAGOGY.md` (G.A, cited by G.W/T.G), `gnfs/docs/PEDAGOGY.md` §1–§51 (G.B–G.F
  code-tours; G.W appends the integrative chapter, T.G is its maths-first sibling). The textbook
  genre target is `tetratile/docs/mathematics.rst` (maths-first, learnable on its own).
- **Register:** T.0 *establishes* the textbook register (C-Textbook); G.W is **PEDAGOGY** (code-tour,
  matching the G.D.W/G.E.W/G.F.W chapter genre and quality); T.G is **textbook** (maths-first,
  obeying the just-frozen C-Textbook). Do not conflate the two genres — that distinction is the core
  of Track τ.
- **Tier routing:** all three sessions are **Opus** (`@plan-deep` for the writing, or `@build`-Opus
  if the chain dispatches build-tier; the payoff proofs and C-Textbook freeze are the Opus drivers).
  T.0 carries one `@plan` marker: a T0/Opus **post-landing C-Textbook-freeze confirmation** (page
  `@plan-juncture`) before T.G is dispatched. There is no design-juncture *before* T.0 — T.0 *is* the
  spine-design session.
- **Invariants to preserve:** all Track-G code contracts (C-PolyPair, C-Score, C-Relation,
  C-FactorBase, C-Matrix, C-LinAlg, C-AlgSqrt) and the G.A substrate contracts are **frozen** — this
  bundle reads and articulates them; it amends no code contract. The only code touch is the G.W
  stale-docstring correction in `gnfs/src/lib.rs`. The new contract is **C-Textbook** (prose-
  enforced, frozen at T.0).
- **CADO-NFS / msieve remain dev-only oracles**, unchanged. No new dependency.
- **Doc-format:** new display mathematics uses MathJax (`$$…$$`); ratified at T.0. The textbook is
  Markdown + MathJax; the `*.W` code-tours are recommended (not mandated) to match.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — this bundle opens a
  new artifact (the textbook) and freezes a new cross-track contract (C-Textbook), and prose has no
  inner-loop test, so halt at every juncture. With the T.0 `@plan` marker and the T.G ◆ boundary it
  halts **twice**: the **T.0 C-Textbook-freeze confirmation** (before G.W/T.G consume the register)
  and the **T.G ◆ boundary** (the Track-G close + roadmap-flex log).
