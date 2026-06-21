<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: D. REFACTOR (arc-2 layout + code-depth de-provenancing)

The rolling, current-sub-track view of the work, in `/plan-run`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`. The completed A. SURVEY plan this
replaces is preserved in git history (PLAN.md at `89dcc8a`).

`juncture-tier: opus` (header above) — **held at the default; the lever-5 opt-down was available but
not taken.** REFACTOR is the first arc-2 *campaign* (SURVEY was the substrate). It does two things:
(1) **collapse the `rho` re-export wrappers** and freeze the final crate layout (C-Layout, the one
compiler-enforced contract that binds CONSOLIDATE and EXTEND), and (2) **execute the code-depth
de-provenancing** SURVEY cataloged — re-anchoring ~524 planning-frame tokens in `//!`/`///`/`//`
doc-comments onto topic-native names, plus the single identifier rename SURVEY found. The five levers:

1. **Ambient complexity — MODERATE (a garden).** The arc-1 codebase is coherent and KAT-disciplined —
   the manual's "garden" case, not spaghetti. The de-provenancing surface is wide (9 crates, ~524
   tokens) but shallow per file. Pushes mildly smaller; does not force fine grain.
2. **Irreducible complexity (the FLOOR) — LOW for de-provenancing, MODERATE for the wrapper collapse.**
   De-provenancing is mechanical re-anchoring against an *already-frozen* classification (SURVEY's
   residue/grouping split, C-Coherence code-half, is done; REFACTOR executes it). The conceptual unit
   per file is tiny. The one real floor is **D.1's wrapper collapse** — it carries compiler blast
   radius across `rho/src/` and freezes C-Layout, so it is a genuine structural conceptual unit that
   must not be fractured.
3. **Cost of a design error — SPLIT.** For de-provenancing: **LOW** — a comment scrub that picks a
   weak topic-native phrase is trivially reversible; no downstream chain consumes the exact wording.
   For the D.1 collapse + C-Layout freeze: **HIGH** — the ROADMAP names REFACTOR "the arc's
   rigidity/blast-radius node"; C-Layout is compiler-enforced and a wrong freeze propagates into
   CONSOLIDATE and EXTEND. Open-Q 1 (keep Track-E with the rho baseline — no crate split) removed the
   *largest* layout decision, but the wrapper-collapse is a real structural change, so the
   blast-radius concern is live, not discharged.
4. **Correctness-criticality — LOW.** This is what distinguishes REFACTOR from SURVEY: **REFACTOR has
   a real inner loop.** Every change is caught by `cargo check --workspace` (the collapse + the one
   identifier rename are compiler-checked) and `cargo test --workspace` (the dedup is KAT-checked;
   doc-comment scrubs are behaviourally inert). A wrong scrub cannot ship a behavioural regression.
5. **Inner-loop bandwidth — STRONG (the opposite of SURVEY's weak-lever-5).** The ROADMAP states it:
   "arc-1's codebase is a *garden* … lever 5 is strong, which licenses smaller, denser commits and
   opting juncture-tier down to sonnet where correctness-criticality is also low." For REFACTOR,
   lever 5 is strong **and** lever 4 is low — the exact coincidence the manual names for the
   juncture-tier opt-down to Sonnet.

On the levers: **4 (low criticality) and 5 (strong loop) made the `sonnet` juncture-tier opt-down
genuinely available** — this is the first arc-2 sub-track where it is. The human elected to **hold
`opus`** anyway, honoring the ROADMAP's explicit pre-commitment of REFACTOR as the blast-radius node:
C-Layout is compiler-enforced and binds two downstream campaigns, and the D.1 wrapper-collapse turned
out to be a real structural change (not the 2-file test-deletion first imagined), so the
rigidity-node framing holds and the stronger adjudicator earns its (cheap, one-shot) differential at
the single ◆ freeze. **One ◆ boundary — D.6 ◆** (freeze C-Coherence code-half + ratify C-Layout +
fold discoveries up), an `@architect` inflection point.

**The decomposition is by crate-weight, larger-grained than SURVEY (lever-5 strong).** Six sessions:
one structural substrate (D.1, the collapse + C-Layout freeze, goes first so later sessions
de-provenance final paths), four de-provenance sessions split by crate weight (`rho` is 55% of the
work and splits at a contract-sharp token-class seam; `gnfs`, then `shor`+`shared/*`), and one
integrative ◆ freeze. The **one-line-commit-title corollary** holds for every row.

---

## Purpose (design intent)

Re-read at every ◆ boundary (anti-defocus anchor). From the arc-2 ROADMAP design intent:

> **The artifact must stand on its own terms, not as the realization of an executed plan.** A reader
> who never saw the roadmap should encounter a library organized by *topic* — the mathematics of
> discrete logarithms — not by *track* and *phase*, the order in which the work was built. The git
> history is where the construction narrative belongs; the code, docs, and names should cohere on the
> subject's own principles.

REFACTOR is the **code-half** of that principle made concrete (CONSOLIDATE owns the prose-half).
Its charge is narrow and bounded by the SURVEY findings (C-Findings): collapse the contrived `rho`
re-export wrappers into the genuinely-generic `shared/*` substrate they shadow, and re-anchor every
**code-adjacent** planning-frame token (`//!`/`///`/`//` doc-comments, the one identifier) onto
topic-native names — exactly the residue SURVEY cataloged, nothing more. The anti-defocus question at
the ◆: *does the refactored code read as a topic-organized library without inventing structure SURVEY
did not find (defocus) or leaving a cataloged token un-scrubbed (rigidity)?* The single most
important property: **REFACTOR touches code-adjacent artifacts only — human prose is CONSOLIDATE's,
config is ALIGN's, the CADO sidecar is ORACLE's** (the scope-routing invariant; misrouting is the
primary cross-campaign defocus mode).

The user's stated standard for the layout work: **code simplicity at the expense of plan-execution
complexity — non-contrived elegance, transparency, durability, symmetry.** The wrapper collapse (D.1)
is the load-bearing application of this: the `rho::field`/`rho::util` wrappers add a third naming
layer over an upstream alias (`shared_field::FpMonty4`) that already exists, preserving no
parameterization the trait's genericity does not already give. Collapsing them removes contrivance,
not capability.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only
`[workspace]` with 9 members + `resolver = "2"` + a `[profile.bench]`); raw `cargo` is the only CI
surface (unchanged since the arc-1 terminus and confirmed at this shard's preflight survey).
`/plan-run` re-discovers at preflight. Unlike SURVEY, **REFACTOR's VERIFY gate is a real correctness
gate, not a pure regression guard** — this is the lever-5-strong reality:

- **`cargo check --workspace` is the primary gate for the collapse + the identifier rename.** D.1
  rewrites `rho` call-sites from `crate::field::{FpMonty, FpNaive}` to `shared_field::{FpMonty4,
  FpNaive4}` (or equivalent); the compiler finds every missed site. D.3's rename of the one test
  identifier is likewise compiler-checked. A green `cargo check` is the proof the structural change
  is complete.
- **`cargo test --workspace` is the gate for the dedup + a behavioural-inertness check.** D.1 removes
  the three duplicated `batch_invert` tests; `shared/bigint` must still cover them (it does — same
  three cases). The doc-comment de-provenance (D.2–D.5) is behaviourally inert: the full KAT suite
  must stay **identically green** before and after each scrub. A test delta in a de-provenance session
  means a scrub touched code, not a comment — a defocus red flag to HALT.
- **No `cargo bench` involvement on the green path.** D.2 re-anchors bench *doc-comments and labels*
  (`rho/benches/ecdlp.rs` Phase-N, `rho/benches/attacks.rs` `E.W.1`); it does not run or alter bench
  *logic*. `VERIFY_BENCH` is N/A to the gate, but benches must still compile (`cargo check` covers
  `--benches` where the workspace builds them).
- **The de-provenance correctness gate is review + behavioural-inertness, not a new test.** A scrub's
  correctness = *completeness* (every cataloged token re-anchored) + *fidelity* (topic-native name is
  accurate) + *inertness* (no behavioural change). The compiler+KAT loop catches inertness violations;
  completeness/fidelity is the ◆ review's job.

---

## Session list

One commit-shaped session per row. `Cat`: **A** substrate (D.1, the structural collapse + C-Layout
freeze) / **I** integrative (D.6 ◆); the de-provenance sessions D.2–D.5 are **A**-flavoured
(code-adjacent substrate cleanup, not new algorithm). `Tier`: D.2–D.5 are **Sonnet** `@build`
(mechanical re-anchoring against the frozen C-Coherence code-half catalog — the classification is
done, the executor applies it under the compiler+KAT loop); **D.1 and the D.6 ◆ are Opus** (D.1 is
the compiler-blast structural change that freezes C-Layout; D.6 is the inflection freeze).
`Consumes`: the frozen SURVEY contracts (C-Coherence code-half, C-Findings REFACTOR ledger row,
C-Layout sketch). `◆` marks the sub-track-final freeze; `@architect` marks the inflection point.

| #   | Session                                                              | Cat | Tier   | Consumes                       | Expected files |
|-----|---------------------------------------------------------------------|-----|--------|--------------------------------|----------------|
| D.1 | Collapse `rho` re-export wrappers + freeze final crate layout       | A   | **Opus** | C-Layout sketch, F-D5-02/03  | `rho/src/field/` (remove), `rho/src/util/batch_inv.rs` + `mp.rs` (remove), `rho/src/util/mod.rs`, `rho/src/lib.rs`, and `rho/src/**` call-sites importing `crate::field`/`crate::util` (compiler-driven) |
| D.2 | Re-anchor `rho` baseline doc-comment provenance (`Phase N` → topic) | A   | Sonnet | C-Coherence code-half, F-D9-01 | `rho/src/ecdlp/{mod,cli,walk}.rs`, `rho/src/curve/*.rs`, `rho/src/factor/*.rs`, `rho/src/field/monty.rs`, `rho/benches/{ecdlp,field,factor}.rs`, `rho/tests/ecdlp_kat.rs` |
| D.3 | Re-anchor `rho` attack-module provenance + rename the one test id   | A   | Sonnet | C-Coherence code-half, F-D9-03…06 | `rho/src/{pairing,ghs,semaev,index_calculus,ssa,binary_curve,binary_ecdlp,hyperelliptic}/**`, `rho/tests/*_kat.rs`, `rho/benches/attacks.rs`, `rho/src/lib.rs` |
| D.4 | Re-anchor `gnfs` doc-comment provenance (`Track D` + sub-track IDs) | A   | Sonnet | C-Coherence code-half, F-D9-02/03 | `gnfs/src/{dl,linalg,polyselect,sieve,filter,sqrt}/**`, `gnfs/src/lib.rs`, `gnfs/tests/*_kat.rs` |
| D.5 | Re-anchor `shor` + `shared/*` doc-comment provenance                | A   | Sonnet | C-Coherence code-half, F-D9-03/05 | `shor/src/**`, `shor/tests/*_kat.rs`, `shared/{padic,gf2m,numth,numfield}/src/**` + `tests/**` |
| D.6 | **◆ @architect** Freeze C-Layout + C-Coherence code-half; fold up   | I   | **Opus** | D.1–D.5, C-Coherence sketch  | `docs/PLAN.md` (contracts + ledger), `docs/ROADMAP.md` (status + discoveries), `docs/NOTES.md` |

---

## Session detail

Per-row deliverable, ≥1 KAT (or, for behaviourally-inert de-provenance rows, the **inertness KAT
invariant** — the full suite stays identically green; a row that *changes* a KAT result has broken
scope and is flagged), subtleties, deferrals. D.1 is crisply specified (the collapse direction is
decided below); D.2–D.5 apply the frozen C-Coherence code-half catalog; D.6's shape is correctly
lower-fidelity until D.1–D.5 land.

**Every REFACTOR session shares one hard invariant: code-adjacent only, behaviourally inert (except
D.1's structural collapse, which is compiler+KAT-verified equivalent).** A session that edits human
prose (`PEDAGOGY.md`/`MATHEMATICS.md`/`README.md` — CONSOLIDATE's), a config/manifest (ALIGN's), the
CADO sidecar (ORACLE's), or that *changes a KAT result* in a de-provenance row has broken scope — a
HALT-and-surface defocus signal, the scope-routing invariant enforced.

### D.1 — Collapse `rho` re-export wrappers + freeze final crate layout — **Opus**

- **Deliverable.** Remove the contrived `rho`-local re-export wrappers and rewrite call-sites to use
  the `shared/*` substrate directly, then freeze C-Layout. Specifically: (a) delete `rho/src/field/`
  (the 25-line `mod.rs` aliasing `shared_field::Fp`/`FpNaive<4>`/`FpMonty<4>`) and have `rho` import
  `shared_field::{Fp, FpNaive4, FpMonty4}` (the aliases `shared_field` *already exports*, lib.rs:40-43)
  at call-sites; (b) delete the duplicated `batch_invert` tests and the `pub use` shim in
  `rho/src/util/batch_inv.rs`, having `rho` import `shared_bigint::batch_invert` directly; (c) remove
  the empty `rho/src/util/mp.rs` stub; (d) collapse or remove `rho/src/util/mod.rs` accordingly. The
  `F: Fp<4>` *trait bounds* stay as-is (the trait is genuinely generic — only the local *aliases* and
  the duplicated *tests* are contrivance). Green `cargo check --workspace` + `cargo test --workspace`
  is the proof of equivalence.
- **KAT (≥1).** No *new* KAT — the contract here is **behavioural equivalence under the existing
  suite**: every `rho` KAT (ecdlp, factor, curve, all attack-module KATs) stays identically green,
  and `shared/bigint`'s `batch_invert` tests (the three the wrapper duplicated) remain the sole
  coverage. *Freezes C-Layout (compiler-enforced).*
- **Subtleties — why Opus, and the design rationale.** SURVEY (F-D5-02) found these wrappers are thin
  re-exports existing "for backward compatibility with the rest of `rho`" — i.e. construction-history
  convenience. The load-bearing finding (this shard's preflight read): `shared_field` is *correctly*
  generic over the limb count `L`, and **already exports `FpMonty4`/`FpNaive4` aliases** (`shared/
  field/src/lib.rs:40-43). The `rho` wrappers therefore add a *third* naming layer over an existing
  upstream alias and preserve **no** parameterization the trait does not already give — the collapse
  loses nothing. This is the user's "non-contrived elegance" standard applied: remove the indirection.
  Opus because this is the campaign's compiler-blast-radius node — the call-site rewrite spans
  `rho/src/{curve,ssa,ecdlp,pairing,binary_ecdlp,factor,...}` and a wrong layout freeze invalidates
  CONSOLIDATE + EXTEND.
- **Ordering (load-bearing).** D.1 goes **first** so D.2–D.5 de-provenance *final* file paths. D.1 is
  structural (compiler change); D.2+ are comment-only on the result — a clean contract seam, no
  overlapping-edit fracture even though D.1 and D.2 both touch `rho/src/field/monty.rs` (D.1 rewrites
  its imports; D.2 scrubs its `Phase 1` doc-comment, on the post-collapse file).
- **Deferral.** No de-provenancing here (that is D.2–D.5) beyond what the file deletions incidentally
  remove. No human-prose, no config.

### D.2 — Re-anchor `rho` baseline doc-comment provenance — Sonnet

- **Deliverable.** Re-anchor the **`Phase N` pure-residue tokens** (F-D9-01) in the Pollard-rho
  baseline + optimization stack onto optimization-layer names: `Phase 4`→r-adding walk, `Phase 5`→
  distinguished-point search, `Phase 6`→negation map, `Phase 7`→batched inversion, `Phase 8`→GLV
  endomorphism; remove "Phase 1/2 deliverable" framing from bench doc-comments. Files: `rho/src/ecdlp/
  {mod,cli,walk}.rs`, `rho/src/curve/{mod,secp_k1_toy,generic,test_curves}.rs`, `rho/src/field/
  monty.rs`, `rho/benches/{ecdlp,field,factor}.rs`, `rho/tests/ecdlp_kat.rs` (the `Phase 3,4` +
  `Phase E.A.2` section headers). ~60 tokens.
- **KAT (inertness invariant).** No KAT result changes — comment-only. The full `rho` suite stays
  identically green. *Applies C-Coherence code-half; freezes nothing (D.6 ratifies).*
- **Subtleties — the algorithmic-phase exclusion (load-bearing).** SURVEY explicitly carved out
  **algorithmic** `Phase 1/2/3` labels that are *step indices inside a single function* — in
  `rho/src/ecdlp/walk.rs` (batched-inversion loop steps) and `rho/src/index_calculus/solve.rs`
  (Gaussian-elim steps). **These are NOT residue and must NOT be scrubbed.** The executor distinguishes
  planning-frame `Phase N` (session-phase labels in module/bench doc-comments) from algorithmic
  `Phase N` (numbered steps of one algorithm). Misclassifying either way is a fidelity error caught at
  D.6 review.
- **Deferral.** `rho/benches/attacks.rs` (`E.W.1`) and the attack modules belong to D.3 (distinct
  token class). Human-prose PEDAGOGY `Phase N` is CONSOLIDATE's (F-D9-10).

### D.3 — Re-anchor `rho` attack-module provenance + rename the one test id — Sonnet

- **Deliverable.** Two coupled re-anchors over the 8 Track-E attack modules + their tests (the bulk of
  `rho`'s residue, ~sub-track-ID-class): (a) replace **sub-track IDs** (`E.B`, `E.C`, `E.E`, `E.H.n`,
  `E.J.n`, `E.K.n`, `E.G.n`) and **contract-label tokens** (`C-AnomalousLift`, `C-IndexCalc`,
  `C-Pairing`, etc.) in `///`/`//!`/`//` comments across `rho/src/{pairing,ghs,semaev,index_calculus,
  ssa,binary_curve,binary_ecdlp,hyperelliptic}/**` and `rho/tests/*_kat.rs` with topic-native
  references (module paths, function names, or removal where the ID is the only content); replace the
  `PLAN.md`/`session contract`/`◆ sub-track close` references (F-D9-05) and the `E.W.1` bench label
  (F-D9-06); (b) **rename the single identifier** `sub_track_close_curve_axioms_intact` →
  `binary_curve_axioms_intact` (or `curve_axioms_hold_after_full_suite`) in `rho/tests/
  binary_curve_kat.rs:582` (F-D9-04). ~230 tokens + 1 rename.
- **KAT (≥1 + inertness).** The renamed test still passes (compiler+test-checked — the one
  non-inert change, verified by the suite staying green with the renamed test). All other changes
  comment-only. *Applies C-Coherence code-half.*
- **Subtleties.** `rho/tests/index_calculus_kat.rs` is the single densest file (~41 tokens, incl. the
  `E.K.5 ◆` milestone labels and `principle-4 boundary` references). Re-anchor on the mathematics
  ("principle-4 boundary: index calculus over `E(F_p)` at toy scale"). The `index_calculus/solve.rs`
  `Phase 1/2` *algorithmic* labels are excluded here too (same carve-out as D.2). The contract-label
  tokens (`C-IndexCalc` etc.) are pure residue — the mathematical content is the interface, not the
  planning contract name.
- **Deferral.** `gnfs`/`shor`/`shared` IDs are D.4/D.5. Human prose is CONSOLIDATE's.

### D.4 — Re-anchor `gnfs` doc-comment provenance — Sonnet

- **Deliverable.** Replace `Track D` (F-D9-02, `gnfs/src/dl/mod.rs` + descent sub-modules — "NFS-DL
  bridge sub-track (Track D)" → "NFS-DL substrate"; "callers within Track D" → "callers within
  `gnfs::dl`") and the sub-track IDs (`D.A.n`, `D.B.n`, `D.C.n`, `D.E.n`, `G.B.n`…`G.F.n`) +
  `PLAN.md`/`session contract` references in `///`/`//!`/`//` comments across `gnfs/src/{dl,linalg,
  polyselect,sieve,filter,sqrt}/**` and `gnfs/tests/*_kat.rs`. ~110 tokens.
- **KAT (inertness invariant).** Comment-only; full `gnfs` suite stays identically green. *Applies
  C-Coherence code-half.*
- **Subtleties.** `gnfs/src/dl/` is already topic-named (`dl` is topic-native) — the `Track D` token
  adds only provenance. Coarse pipeline-stage groupings (`G.B`–`G.W` = GNFS pipeline stages) map onto
  real stages; per open-Q 4 (preserve-under-topic-label), keep the *grouping* where a comment
  describes a real pipeline stage, drop only the *letter-id*. This is the borderline-default applied
  at code depth.
- **Deferral.** `gnfs/docs/PEDAGOGY.md` prose is CONSOLIDATE's (F-D9-13). No layout change in `gnfs`
  (F-D5-03: no dedup needed).

### D.5 — Re-anchor `shor` + `shared/*` doc-comment provenance — Sonnet

- **Deliverable.** Replace sub-track IDs (`S.X.n`, `E.D`, `E.E`, `E.F.1`, `E.H.1`, `E.K`) + `◆`
  marks + `PLAN.md`/`session contract` references in `///`/`//!`/`//` comments across `shor/src/**`,
  `shor/tests/*_kat.rs`, and `shared/{padic,gf2m,numth,numfield}/src/**` + `tests/**`. Notable: the
  `PLAN.md E.D.2` ref in `shared/padic/tests/hensel_kat.rs:6` → "Hand-computed Newton iteration for
  `f(x)=x²−2`, `p=7`"; the `G.A.1a session contract` ref in `shared/numfield/tests/numfield_kat.rs:3`
  → "KATs for number-field arithmetic over ℤ[α]". ~124 tokens (`shor` ~83 + `shared/*` ~41).
- **KAT (inertness invariant).** Comment-only; full `shor` + `shared/*` suites stay identically green.
  *Applies C-Coherence code-half.*
- **Subtleties — two carve-outs (load-bearing).** (a) `shor/tests/factor_kat.rs` `Phase N/M` tokens
  are **quantum phase estimation** (a mathematical term: "Phase 64/256 = 1/4"), NOT planning residue —
  do NOT scrub. (b) The coarse `S.A/S.B/S.C` Shor sub-track groupings are the open-Q-4 borderline:
  per preserve-under-topic-label, keep coarse groupings that name a real Shor stage, dissolve only
  fine-grained `S.X.Y` IDs. `shared/field` and `shared/bigint` are already clean (0 tokens).
- **Deferral.** `shor/docs/PEDAGOGY.md` prose + `◆` marks there are CONSOLIDATE's (F-D9-12/14).

### D.6 — **◆ @architect** Freeze C-Layout + C-Coherence code-half; fold up — **Opus**

- **Deliverable (lower-fidelity by design — consumes D.1–D.5).** Ratify the final crate layout into
  **C-Layout (compiler-enforced, now FROZEN)** — the collapse landed, `rho` imports `shared/*`
  directly, layout is final and binds CONSOLIDATE + EXTEND; **freeze the C-Coherence code-half** (the
  cataloged code-depth residue is now scrubbed — record the completion against the SURVEY catalog);
  verify every F-D9-01…06 token is re-anchored (completeness) and the renames/scrubs are
  topic-faithful (fidelity); fold the durable discoveries up into the ROADMAP Discoveries log and mark
  D. REFACTOR done; surface any residual decision (e.g. a token that resisted clean re-anchoring) to
  the human.
- **KAT (≥1).** The frozen C-Layout *is* the contract — proven by green `cargo check`/`cargo test`
  `--workspace`. The C-Coherence code-half freeze is proven by the catalog being fully consumed (every
  F-D9 code-row scrubbed) with the suite identically green.
- **Subtleties — why ◆ + @architect + Opus.** The inflection: REFACTOR's static→action transform for
  the *code* layer. C-Layout is compiler-enforced and binds two downstream campaigns; freezing it
  wrong propagates. Unlike SURVEY, lever-5 is strong here, so the inner loop *already* proved
  behavioural correctness session-by-session — the ◆'s job is **completeness/fidelity adjudication +
  the contract freeze**, not catching behavioural drift. The juncture fork pages `@plan-juncture` at
  **opus** (`juncture-tier: opus`, held despite the lever-5 opt-down being available, per the
  ROADMAP's blast-radius pre-commitment).
- **Deferral.** D.6 freezes C-Layout + C-Coherence-code-half only. It does **not** touch C-DocsLayer/
  C-MathSpine (CONSOLIDATE), C-Oracle (ORACLE), or the prose-half of C-Coherence (CONSOLIDATE owns
  F-D9-07…14). Those stay "to be frozen at <campaign>."

---

## Cross-session contracts

One subsection per contract, tagged compiler-/test-/prose-enforced, with Defined-in and Consumed-by.
REFACTOR freezes **C-Layout** (compiler) and the **code-half of C-Coherence** (prose) at the D.6 ◆.
The SURVEY-frozen contracts (C-Findings, C-Testing-Philosophy, C-Coherence prose-half) are *consumed*
here and restated only where REFACTOR binds them; the not-yet-frozen ones (C-DocsLayer, C-MathSpine,
C-Oracle) are untouched and remain "to be frozen at <campaign>."

### C-Layout *(compiler)* — **FROZEN at D.6 ◆**

- **Defined-in:** D.1 (the collapse) + D.6 ◆ (the ratified freeze). A.2 (SURVEY) produced the
  layout-audit findings; open-Q 1 resolved the crate-split question.
- **Consumed-by:** CONSOLIDATE (docs reference the final layout), EXTEND (new code sits in it).
- **Governing statement (the resolved layout).**
  - **No crate split (open-Q 1 = keep-with-baseline, human-ratified).** The 8 Track-E attack modules
    stay co-located with the Pollard-rho baseline in the `rho` crate. Rationale: the E.W cross-attack
    bench measures the attacks *against* the rho baseline and they share `rho::curve`/`rho::field`
    types directly; the "attacks live with the baseline they're measured against" cohesion is
    load-bearing pedagogy. *(Worse at: a reader still meets a larger surface than "Pollard rho"
    advertises — accepted.)* The `rho` crate description should be updated to name the Track-E content
    (a REFACTOR-internal doc-comment fix, folded into D.3).
  - **Wrappers collapsed (D.1, human-ratified).** `rho/src/field/` and `rho/src/util/batch_inv.rs`'s
    re-export shims + duplicated tests + the `mp.rs` stub are removed; `rho` imports `shared_field::
    {Fp, FpNaive4, FpMonty4}` and `shared_bigint::batch_invert` directly. The `F: Fp<4>` trait bounds
    remain (the trait is genuinely generic; only the local aliases were contrivance). This is the
    "non-contrived elegance" standard: the wrappers shadowed an upstream alias (`shared_field::
    FpMonty4`) that already existed and preserved no parameterization the trait did not already give.
  - **No other `shared/*` dedup (F-D5-03).** The six `shared/*` crates have distinct, non-overlapping
    concerns. REFACTOR inventing further dedup is defocus.
- **Falsifiability self-test.** "REFACTOR peers Track-E into a new crate" → violates open-Q-1 ratify;
  "REFACTOR keeps the duplicated `batch_invert` tests" → violates D.1; "REFACTOR dedups two `shared/*`
  crates" → defocus (F-D5-03). The frozen layout is the boundary every later campaign is checked
  against.

### C-Coherence *(prose)* — code-half **FROZEN at D.6 ◆**; prose-half **to be frozen at CONSOLIDATE**

- **Defined-in:** SURVEY A.7 (the catalog + classification, already frozen). REFACTOR D.6 freezes the
  **code-half execution** (the cataloged code-depth tokens are scrubbed); CONSOLIDATE freezes the
  prose-half.
- **Consumed-by:** REFACTOR (code half — this sub-track), CONSOLIDATE (prose half), binding on EXTEND
  naming.
- **Code-half statement (what REFACTOR executes and D.6 freezes).** Every cataloged code-depth token
  (F-D9-01…06) is re-anchored or removed: `Phase N` planning labels → optimization-layer names
  (excluding algorithmic step-indices in `walk.rs`/`solve.rs`); `Track D` → topic-native language;
  sub-track IDs + contract-label tokens (`C-*`) → topic-native references or removal; the single
  identifier `sub_track_close_curve_axioms_intact` → topic-native test name. Coarse groupings that
  coincide with a real topic (open-Q 4 = preserve-under-topic-label) keep the grouping, drop the
  letter-id; pure-residue fine-grained IDs dissolve. Quantum-phase-estimation `Phase` in
  `shor/tests/factor_kat.rs` is a math term, excluded.
- **Falsifiability self-test.** "REFACTOR scrubs `walk.rs` algorithmic `Phase 1`" → fidelity error
  (not residue); "REFACTOR edits `MATHEMATICS.md` `Track ρ`" → mis-routed (that is CONSOLIDATE,
  F-D9-07); "REFACTOR leaves `E.K.5 ◆` in `index_calculus_kat.rs`" → incompleteness.

### C-Findings *(prose)* — **FROZEN at SURVEY A.7; consumed here**

- **Consumed-by (REFACTOR row):** the authoritative scope source. REFACTOR's in-scope set is
  F-D5-02/03 (layout/dedup) + F-D9-01…06 (code-depth de-provenancing). *Defocus boundary:* REFACTOR
  touches code-adjacent artifacts only; human prose is CONSOLIDATE's, config ALIGN's, CADO sidecar
  ORACLE's. The **scope-routing invariant** is the primary cross-campaign defocus mode and binds every
  session here.

### C-Testing-Philosophy *(prose)* — **FROZEN at SURVEY; consumed here**

- **Consumed-by (REFACTOR):** REFACTOR adds no new tests; it *removes* the three duplicated
  `batch_invert` tests (D.1) and *renames* one (D.3), honoring the two-tier norm and the
  "one-file-per-algorithm-family" norm. The dedup removal is licensed because `shared/bigint` already
  covers the cases (no coverage lost). No KAT result changes in any de-provenance session (inertness).

### C-DocsLayer / C-MathSpine *(prose)* — **to be frozen at CONSOLIDATE** *(untouched here)*

REFACTOR does not touch these. Recorded for completeness; CONSOLIDATE freezes them after consuming the
final layout C-Layout freezes here.

### C-Oracle *(test)* — **to be frozen at ORACLE** *(untouched here)*

REFACTOR does not touch the CADO sidecar.

---

## Progress ledger

All rows `pending` at shard time. `/plan-run` maintains this (Status pending→done, Commit hash, the
contract(s) each session froze).

| #   | Session                                                              | Status  | Commit | Froze |
|-----|---------------------------------------------------------------------|---------|--------|-------|
| D.1 | Collapse `rho` re-export wrappers + freeze final crate layout       | done    | 42e2356 | C-Layout (compiler, pending D.6 ratification); extras: `shor/Cargo.toml` (dev-dep forced by collapse), `shor/tests/ecdlp_kat.rs`, `Cargo.lock` |
| D.2 | Re-anchor `rho` baseline doc-comment provenance                     | done    | 89f1e22 |       |
| D.3 | Re-anchor `rho` attack-module provenance + rename the one test id   | done    | 84b993c |       |
| D.4 | Re-anchor `gnfs` doc-comment provenance                             | pending |        |       |
| D.5 | Re-anchor `shor` + `shared/*` doc-comment provenance                | pending |        |       |
| D.6 | ◆ Freeze C-Layout + C-Coherence code-half; fold up                  | pending |        |       |

---

## Action-frame digest

The externalized action frame `/plan-run` appends to on non-trivial iterations (a call-site the
collapse missed that the compiler caught, a token that resists clean topic-native re-anchoring, a
borderline grouping where open-Q-4's preserve-under-topic-label is ambiguous, an algorithmic-vs-
planning `Phase N` judgment call) and that the D.6 ◆ juncture fork consumes.

### D.1 — 2026-06-21
Discovery/flex: `rho/src/field/` had 3 files (monty.rs, naive.rs were dead code never declared in mod.rs); `shor/tests/ecdlp_kat.rs` imported `rho::field::FpMonty` and required `shor/Cargo.toml` dev-dep addition.
Affected: C-Layout (no change to layout premise; extras are mechanical consequences of the collapse)
Deferred: no
Texture: Both discoveries are internal-continue; the `shor` dev-dep addition is a structural necessity (compiler-forced), not a REFACTOR-scope violation. C-Layout is established; D.6 ratifies.

---

## Discoveries & risks

Carried *down* from the ROADMAP Discoveries log (D5, D9) and the SURVEY findings, phrased as
`/plan-run` reads for discovery adjudication (internal-continue / additive-reshard / destructive-HALT).
The reverse flow — discoveries accrued here folded back *up* into the ROADMAP — happens at the D.6 ◆.

- **The code-adjacent-vs-prose line is the central REFACTOR defocus risk.** A session that edits
  `PEDAGOGY.md`/`MATHEMATICS.md`/`README.md` content, a config/manifest, or the CADO sidecar has
  broken scope (those are CONSOLIDATE/ALIGN/ORACLE). **Internal-continue → REFACTOR scrubs
  code-adjacent `//!`/`///`/`//` doc-comments + identifiers only.** A prose/config edit is a
  HALT-and-surface defocus signal — the scope-routing invariant enforced.
- **The behavioural-inertness line (de-provenance sessions D.2–D.5).** A doc-comment scrub that
  changes a KAT result means a scrub touched *code*, not a comment. **Internal-continue → comment-only
  edits keep the suite identically green;** a test delta is a HALT-and-surface red flag, not an
  accommodation.
- **The algorithmic-vs-planning `Phase N` distinction (D.2, D.3, D.5) — known concrete carve-outs.**
  `walk.rs`/`solve.rs` algorithmic step-indices and `shor/tests/factor_kat.rs` quantum-phase tokens
  are NOT residue. **Internal-continue → the executor distinguishes by context (numbered step of one
  algorithm vs session-phase label);** a misclassification is caught at the D.6 fidelity review, not a
  reshard. If a *new* ambiguous case appears beyond the cataloged carve-outs, note it in the digest.
- **D.1 collapse blast radius (confirmed bounded) — open-Q-1 keep-with-baseline ratified.** The
  call-site rewrite spans `rho/src/{curve,ssa,ecdlp,pairing,binary_ecdlp,factor,...}` but every site
  is compiler-checked. **Internal-continue → `cargo check --workspace` finds every missed site;** a
  collapse that fails to compile is fixed in-session, not a HALT. The design rationale (wrappers
  shadow an existing upstream alias, preserve no parameterization) is recorded in NOTES.
- **C-Coherence borderline groupings (open-Q 4 = preserve-under-topic-label, ratified) — D.4/D.5.**
  Coarse pipeline/Shor-stage groupings keep the grouping, drop the letter-id; fine-grained IDs
  dissolve. **Internal-continue → apply the ratified default;** a genuinely-new borderline case the
  catalog did not anticipate is a digest note for D.6, not a reshard.
- **Risk — additive reshard at D.6.** If D.1–D.5 surface a *layout* concern SURVEY missed (a real
  dedup or peering need beyond F-D5-02/03), that is an **additive reshard** surfaced at D.6, never a
  silent scope expansion. If a finding *invalidates C-Layout's premise* (e.g. the collapse breaks a
  downstream consumer that cannot be fixed in-layout), that is a **destructive-HALT** to the human.

---

## Notes for executors

- **Read `docs/ROADMAP.md` first** (the arc-2 design intent — the artifact-stands-on-its-own-terms
  principle — and the D. REFACTOR charge: "restructure crate peering/organisation and eliminate
  duplicative code, strictly gated on the SURVEY layout audit; also owns the code-depth half of
  de-provenancing") and the SURVEY findings in `docs/SURVEY.md` (F-D5-01…03, F-D9-01…06) before any
  session.
- **Tier routing.** **D.1 is Opus `@build`** (the compiler-blast structural collapse + C-Layout
  freeze — the campaign's rigidity node). **D.2–D.5 are Sonnet `@build`** (mechanical re-anchoring
  against the frozen C-Coherence code-half catalog, under the compiler+KAT inner loop). **D.6 ◆ is
  Opus `@architect`** (the inflection freeze + static→action transform). The D.6 ◆ fork pages
  `@plan-juncture` at **opus** — `juncture-tier: opus`. **Note: the lever-5 opt-down to
  `@plan-juncture-sonnet` was *available* here** (strong test loop + low criticality, the first arc-2
  sub-track where it is) **but was not taken**, honoring the ROADMAP's pre-commitment of REFACTOR as
  the blast-radius node and the D.1 collapse being a genuine structural change.
- **Register: Rust code + code-adjacent doc-comments** (`STYLE-CODE-RUST.md` + `STYLE-DOC.md`). D.1
  edits real code (imports, call-sites, test deletion/rename) — full `STYLE-CODE-RUST.md` register
  applies. D.2–D.5 edit `//!`/`///`/`//` doc-comments — the thin/mechanical docstring register; the
  re-anchored text names the mathematics, not the construction history. **No human-prose register**
  (PEDAGOGY/MATHEMATICS/README are CONSOLIDATE's).
- **Invariants to preserve.** **Code-adjacent only** (no human prose, no config, no CADO — the
  scope-routing invariant; a cross-boundary edit is a HALT-and-surface). **Behaviourally inert**
  (D.2–D.5 change comments, not behaviour; a KAT-result delta is a red flag). **The `F: Fp<4>` bounds
  stay** (the trait is genuinely generic; only the local aliases + duplicated tests were contrivance).
  **The algorithmic-`Phase N` carve-outs hold** (`walk.rs`/`solve.rs`/`shor factor_kat.rs` are not
  residue). **Open-Q-1 (no crate split) and open-Q-4 (preserve-under-topic-label) are ratified** — do
  not re-open them mid-session.
- **The VERIFY gate is a real correctness gate** (lever-5 strong). `cargo check --workspace` proves
  the D.1 collapse + the D.3 rename are complete; `cargo test --workspace` proves the dedup keeps
  coverage and the de-provenance is inert. Both must be green at every session boundary.
- **Suggested first invocation:** **`/plan-run docs/PLAN.md halt-at-boundaries`** — REFACTOR is the
  first arc-2 *campaign* (a new contract shape: executing the frozen SURVEY catalog rather than
  producing findings) with **one load-bearing inflection** (D.6 freezes C-Layout, which binds
  CONSOLIDATE + EXTEND). The conservative default halts at the D.6 ◆ for the layout/coherence freeze
  review. *(Tradeoff vs `autonomous`: `halt-at-boundaries` costs little — there is one boundary, the
  D.6 ◆ — and buys a guaranteed human check at the compiler-enforced contract that propagates into two
  downstream campaigns. The strong inner loop (lever 5) means D.1–D.5 run autonomously with high
  confidence up to the ◆; only the freeze pauses. Given C-Layout's blast radius (the reason the
  juncture stays opus), the halt is clearly worth it.)*
