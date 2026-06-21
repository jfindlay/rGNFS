<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: A. SURVEY (arc-2 substrate — findings-only audit)

The rolling, current-sub-track view of the work, in `/plan-run`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **kept at the default; the lever-5 opt-down is NOT available
here.** This PLAN shards **A. SURVEY**, the arc-2 substrate and the root of the campaign DAG: a
**findings-only audit** whose sole deliverable is a findings ledger that scopes every downstream
campaign (B. ALIGN, C. ORACLE, D. REFACTOR, E. CONSOLIDATE, F. EXTEND). **No design, no code, no
refactor** — SURVEY audits and records; it never fixes. The five levers:

1. **Ambient complexity — HIGH, and load-bearing.** The audit surface is the *entire* arc-1 library:
   9 workspace crates across 5 tracks (ρ/G/D/E/S), ~15K lines of human docs (an 11-chapter ~4600-line
   `MATHEMATICS.md`, 5 per-crate `PEDAGOGY.md` tours, README, BENCHMARKS), ~300 planning-frame
   provenance tokens spread across doc-comments and prose, and a workspace-root formalities gap. This
   pushes **strongly toward smaller sessions** — auditing all nine dimensions in one pass is the
   cardinal survey failure (treating heterogeneous dimensions as uniform).
2. **Irreducible complexity (the FLOOR) — nine audit dimensions, unevenly sized.** D1–D9 are not
   uniform: some are tightly coupled by shared substrate (D3 docs-layer + D4 math-continuity +
   D9-prose all read the doc corpus; D5 layout + D9-code both read the crate tree), some are
   heavyweight standalone (D6 oracle design; D7 spectrum-completeness is abductive; D8 reads 4
   external references). The floor: never fracture one dimension across sessions, but *do* group
   dimensions that read the same substrate. This is what sets the 7-session grain.
3. **Cost of a design error — HIGH (the ROADMAP's own assessment).** "A missed finding mis-scopes
   everything downstream … the findings ledger is the highest-leverage artifact in arc 2." A wrong or
   absent finding propagates into a mis-scoped campaign. Pushes **smaller** + **opus juncture**.
4. **Correctness-criticality — LOW-as-code (no code is written) but HIGH-as-judgment.** Findings are
   not machine-checkable; their correctness is *completeness* and *falsifiability*, caught only by
   review. Pushes smaller.
5. **Inner-loop bandwidth — WEAK, and load-bearing (the same reality as the arc-1 prose terminus).**
   **There is no test inner-loop for an audit.** The VERIFY gate (`cargo test --workspace`) is
   trivially green throughout — SURVEY touches no code — so no behavioural signal catches a wrong
   finding. Per the tuning law this pushes *toward smaller* sessions **and forbids the juncture-tier
   opt-down**. The compensating inner loop is the **◆ findings-ledger review** (load-bearing here,
   not ceremonial).

On the levers: **1 (HIGH ambient), 3 (HIGH design-error cost), 4 (HIGH judgment), and 5 (weak loop)
all push toward small, and the ROADMAP pre-commits opus juncture at SURVEY close.** The decomposition
is therefore **fine-grained: 7 sessions** — six dimension-audits grouped by shared substrate, each a
distinct commit title (the one-line-commit-title corollary applied), plus one integrative ◆
ledger-freeze. The lever-5 weakness means the **opt-down is UNAVAILABLE** (no `@plan-juncture-sonnet`):
`juncture-tier: opus`. One ◆ boundary — **A.7 ◆** (the findings-ledger consolidation + contract
freeze, an `@architect` inflection point: it is the static→action frame transform that scopes all
five downstream campaigns).

**The contract shape is unusual and must be stated up front.** A normal substrate session freezes a
*compiler* contract (a trait). SURVEY freezes **prose** contracts — the findings ledger (C-Findings),
the testing doctrine (C-Testing-Philosophy), the coherence catalog (C-Coherence) — plus *sketches* of
the contracts later campaigns freeze precisely (C-Layout at REFACTOR, C-DocsLayer/C-MathSpine at
CONSOLIDATE, C-Oracle at ORACLE). The manual's "every row freezes ≥1 KAT, or its contract is
undefined" rule still binds, transposed: **SURVEY's analogue of a KAT is a *falsifiable findings
entry*** — a finding stated concretely enough that a downstream campaign can be checked against it
("`deny.toml` absent → ALIGN adds it" is falsifiable; "docs could be better" is not). Every audit
session's deliverable is a set of falsifiable findings; a session that can only emit vague
impressions has an undefined contract and is flagged.

---

## Purpose (design intent)

Re-read at every ◆ boundary (anti-defocus anchor). From the arc-2 ROADMAP:

> Take the complete arc-1 rGNFS — a self-consistent, pedagogically clear Rust reference library for
> discrete-logarithm and integer-factorisation algorithms — and bring it to a finished, disciplined,
> internally-coherent standard. The arc-2 discipline is **survey-first**: the project's actual current
> shape is *discovered by audit* before any consolidation or refactoring commits to scope.

SURVEY *is* that survey-first discipline made concrete. Its charge is narrow and absolute: **audit
every dimension and emit a findings/recommendations ledger — no design, no code, no refactor.** The
anti-defocus question at the ◆: *does the findings ledger scope the campaigns to what the audit
actually found, neither inventing scope (defocus) nor ignoring a real finding (rigidity)?* The
single most important property of the deliverable: **the findings ledger is the authoritative scope
source for all five campaigns** (C-Findings). A campaign that exceeds its findings is defocus; one
that ignores a finding is rigidity.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only `[workspace]`
members + a `[profile.bench]`); raw `cargo` is the only CI surface (unchanged since the arc-1
terminus). **SURVEY writes NO code** — it edits prose findings files only (`docs/PLAN.md` ledger,
`docs/NOTES.md`, and the survey-findings doc). The VERIFY gate is therefore a **pure no-regression
gate** (every existing crate's KATs stay green because SURVEY changes no source, no manifest, no
module surface), with no new test to add and no new compile target. `/plan-run` re-discovers at
preflight. The honest gate for a findings-only audit is **the no-regression VERIFY (trivially green) +
the findings-ledger completeness/falsifiability review at the A.7 ◆** — the weak-lever-5 reality made
explicit:

- **The entire existing workspace KAT suite must stay green** — SURVEY touches no `.rs`, no
  `Cargo.toml`, no module. `cargo test --workspace` is a pure regression guard; it adds nothing and
  must change nothing. *(If a session run reports any test delta, that is a red flag that SURVEY has
  drifted into code/refactor — a defocus signal to HALT, not to accommodate.)*
- **`cargo check --workspace` must stay green** — trivially, for the same reason.
- **No `cargo bench` involvement** — SURVEY runs no benchmarks (it may *cite* `docs/BENCHMARKS.md`
  facts in a testing-balance finding, but adds/edits no bench). `VERIFY_BENCH` is N/A.
- **The real correctness gate is review, not tests (lever-5-weak made explicit).** Findings
  correctness = *completeness* (no dimension under-audited) + *falsifiability* (each finding is
  concrete enough to check a campaign against) + *scope-fidelity* (findings, not fixes). This is
  **not** machine-checkable; it is the human review + the A.7 ◆ juncture's job.

---

## Session list

One commit-shaped session per row. `Cat`: **A** substrate(audit) — all SURVEY rows are audit
substrate. `Tier`: all six audit sessions are **Sonnet** `@build` (mechanical-judgment audit against
explicit dimension charges; the substrate is read, not designed) **except A.6** and the **A.7 ◆**,
which are **Opus** (A.6 is abductive — "what is genuinely missing"; A.7 is the inflection
ledger-freeze). `Consumes`: SURVEY is the DAG root — it consumes only the ROADMAP dimension charges
(no upstream contract). `◆` marks the sub-track-final ledger-freeze; `@architect` marks the
inflection point.

| #   | Session                                                          | Cat | Tier   | Consumes        | Expected files |
|-----|-----------------------------------------------------------------|-----|--------|-----------------|----------------|
| A.1 | Audit template-formalities gap, both flow directions (D1)       | A   | Sonnet | ROADMAP D1      | `docs/SURVEY.md` (new), root config read-only (`Cargo.toml`, absent `deny.toml`/`rustfmt.toml`/`rust-toolchain.toml`/`LICENSE`), `~/Source/rust-template` read-only |
| A.2 | Audit crate layout/dedup + catalog code-depth provenance (D5+D9-code) | A | Sonnet | ROADMAP D5,D9 | `docs/SURVEY.md`, `Cargo.toml` + all `*/src/**` + `*/benches/**` + `*/tests/**` read-only |
| A.3 | Audit testing balance + resolve the coverage-doctrine question (D2) | A | Sonnet | ROADMAP D2    | `docs/SURVEY.md`, all `*/tests/**` + inline `#[test]` + `docs/BENCHMARKS.md` read-only |
| A.4 | Audit docs-layer discipline + math-continuity + prose provenance (D3+D4+D9-prose) | A | Sonnet | ROADMAP D3,D4,D9 | `docs/SURVEY.md`, `docs/MATHEMATICS.md` + 5 `*/PEDAGOGY.md` + `README.md` read-only |
| A.5 | Audit CADO-NFS sidecar build/trigger design needs (D6)          | A   | Sonnet | ROADMAP D6      | `docs/SURVEY.md`, arc-1 dev-oracle policy read-only; external CADO-NFS docs |
| A.6 | Audit spectrum completeness + distil 4 quantum-DLP references (D7+D8) | A | **Opus** | ROADMAP D7,D8 | `docs/SURVEY.md`, `docs/MATHEMATICS.md` matrix read-only; 4 external references (web) |
| A.7 | **◆ @architect** Consolidate findings ledger + freeze SURVEY contracts | A | **Opus** | A.1–A.6 findings | `docs/SURVEY.md` (final ledger), `docs/PLAN.md` (contracts + ledger), `docs/ROADMAP.md` discoveries (juncture-scope), `docs/NOTES.md` |

---

## Session detail

Per-row deliverable, ≥1 falsifiable-findings unit (the SURVEY analogue of a KAT — a row that can only
emit vague impressions has an undefined contract and is flagged), subtleties, deferrals. The audit
sessions A.1–A.6 are crisply specified (their charges are the ROADMAP dimensions D1–D9, already
concrete); A.7's shape is correctly lower-fidelity until A.1–A.6's findings land (it *consumes* them).

**Every SURVEY session shares one hard invariant: findings, not fixes.** A session that edits a
`.rs`, a manifest, a doc's *content* (vs the SURVEY findings file), adds a config file, or refactors
anything has broken scope — that is a HALT-and-surface defocus signal, the audit-substrate analogue
of "Z.1/T.Z must not write code." SURVEY *records what should change*; ALIGN/ORACLE/REFACTOR/
CONSOLIDATE/EXTEND *change it*.

### A.1 — Audit template-formalities gap, both flow directions (D1)

- **Deliverable.** A `docs/SURVEY.md` section enumerating every formalities item present in
  `~/Source/rust-template` but absent at the rGNFS workspace root (`deny.toml`, `rustfmt.toml`,
  workspace-root `rust-toolchain.toml`, `LICENSE`, the `[lints]` table, a coverage gate,
  `development.md`), each with a flow-direction decision (template→rGNFS backport vs
  rGNFS→template forward-seed) and a scope note (per-crate vs workspace-level — e.g. `[lints]` exists
  in only 3 of 9 crates today). Plus the reverse list: rGNFS-originated discipline worth seeding back
  (multisession docs layering, dev-oracle policy, docs-register contracts).
- **Falsifiable findings (≥1).** "`deny.toml` absent at root → ALIGN adds it" / "`[lints]` present in
  `shor`+`shared/padic`+`shared/gf2m` only → ALIGN promotes to workspace-level". Each finding names
  the item, its direction, and the consuming campaign. *Feeds ALIGN; seeds C-TemplateSeed.*
- **Subtleties.** The rho-only `rust-toolchain.toml` is a stale per-crate artifact — note whether it
  should promote to root or be removed. The coverage-gate item is *coupled to A.3's doctrine* (don't
  decide the gate value here; A.3 owns the doctrine, A.1 only records the gap).
- **Deferral.** No config is written (that is ALIGN). A.1 records the gap and direction only.

### A.2 — Audit crate layout/dedup + catalog code-depth provenance (D5 + D9-code)

- **Deliverable.** Two coupled findings sets that read the same crate tree: (a) the **layout audit** —
  9 workspace members; is `rho` overloaded (it hosts both Track ρ *and* the large Track E with its
  binary_curve/binary_ecdlp/ghs/hyperelliptic/index_calculus/pairing/semaev/ssa modules)? Should E
  peer out? Is there duplicative code across `shared/*`? — and (b) the **code-depth provenance
  catalog** — every planning-frame token in *code identifiers, module/file/dir names, benchmark
  labels, test names* (the survey found ~1 genuine identifier hit, ~15 `Phase N` doc-comment tokens
  in `rho/src/ecdlp/`, ~300 `X.Y` tokens overwhelmingly in doc-comments), each classified **pure
  residue** (re-anchor on topic) or **grouping-coincides-with-topic** (keep grouping, change label).
- **Falsifiable findings (≥1).** "`rho` hosts 8 Track-E attack modules → REFACTOR decision: peer-out
  vs keep-with-baseline (open Q for human)" / "`Phase 4`–`Phase 8` tokens in `rho/src/ecdlp/cli.rs` →
  REFACTOR re-anchors on the optimization-layer names". *Feeds REFACTOR; seeds C-Layout (sketch) +
  the code-half of C-Coherence.*
- **Subtleties.** The survey's headline finding — *de-provenancing blast radius is doc-comment-heavy,
  identifier-light* — is load-bearing for sizing REFACTOR. Record it precisely (identifier renames
  are cheap; doc-comment scrubbing is the bulk, and doc-comments are a REFACTOR/CONSOLIDATE boundary
  question: inline `//!`/`///` are code-adjacent → REFACTOR, human prose → CONSOLIDATE).
- **Deferral.** No rename, no module move (that is REFACTOR). The `rho`-overload *decision* is a human
  open-Q surfaced at A.7, not resolved here.

### A.3 — Audit testing balance + resolve the coverage-doctrine question (D2)

- **Deliverable.** A findings section on testing balance (unit/integration mix — the survey counts
  ~23 KAT files in `gnfs`, 11 + 4 benches in `rho`, etc.; over-long tests; inline-vs-`tests/`
  distribution; the `shared/numth` no-`tests/`-dir case), *and* a resolved answer to the meta-question
  **"is full/100% line coverage the right target for toy-scale-but-compute-heavy KAT-driven
  pedagogical code, or does KAT + property coverage of *mathematical* behavior matter more?"** —
  stated as a coverage doctrine (target, unit/integration norm, test-length norm).
- **Falsifiable findings (≥1).** The resolved doctrine *is* the falsifiable artifact: "coverage
  doctrine = X (e.g. mathematical-behavior KAT coverage over line coverage; gate at N%) → ALIGN's
  coverage-gate honors it; new EXTEND/CONSOLIDATE tests honor it". *Feeds ALIGN, CONSOLIDATE, F;
  freezes C-Testing-Philosophy.*
- **Subtleties.** This is the one audit session with a genuine **design tension** to resolve (not just
  catalog) — the meta-question is abductive, not mechanical. It is still Sonnet-tier because the
  ROADMAP frames it as a *bounded* doctrine choice with a strong default (math-behavior coverage), and
  it surfaces the human's lean as an open-Q at A.7 rather than deciding unilaterally.
- **Deferral.** The coverage *gate value/config* is ALIGN (A.3 sets the doctrine; A.1 recorded the
  gap; ALIGN implements). No test is written or deleted.

### A.4 — Audit docs-layer discipline + math-continuity + prose provenance (D3 + D4 + D9-prose)

- **Deliverable.** Three coupled findings sets that read the same doc corpus: (a) **docs-layer
  discipline** (D3) — do the three layers (inline / human-code-ref `PEDAGOGY.md` / agent-docs) honor
  their prescribed reference directions across the ~257KB `MATHEMATICS.md` + 5 `PEDAGOGY.md` + README?
  (b) **math-exposition continuity** (D4) — does `MATHEMATICS.md` (11 chapters, ~4600 lines, assembled
  chapter-by-chapter at arc-1 track boundaries) read as a *continuous textbook with a suggested path*
  or a stitched companion reference? Audit notation/voice/audience/dir-file-name consistency +
  code↔math cross-reference health. (c) **prose-depth provenance catalog** (D9-prose) — every
  planning-frame token in PEDAGOGY/MATHEMATICS/README prose, classified residue vs
  grouping-coincides-with-topic.
- **Falsifiable findings (≥1).** "`MATHEMATICS.md` ToC lines reference 'to be appended' chapters that
  now exist → CONSOLIDATE reconciles" / "chapter-pairing table Track-E row points at `gnfs/` but the
  E.W chapter lives in `docs/PEDAGOGY.md` → CONSOLIDATE fixes" / "`Track ρ` appears in N prose
  locations → CONSOLIDATE re-anchors as 'Pollard rho'". *Feeds CONSOLIDATE; seeds C-DocsLayer +
  C-MathSpine (sketches) + the prose-half of C-Coherence.*
- **Subtleties.** This is the largest read surface in SURVEY (~15K doc lines). The session must
  *sample and characterize*, not exhaustively re-read every line — the falsifiability bar is "concrete
  enough to scope CONSOLIDATE," not "complete line-by-line edit list" (that is CONSOLIDATE's own
  first step). Note the borderline groupings (track ≈ topic but not exactly) as open-Qs for A.7.
- **Deferral.** No prose is rewritten (that is CONSOLIDATE). The math-spine *design* is CONSOLIDATE's
  opus-juncture work; A.4 only records whether the spine coheres today and where it breaks.

### A.5 — Audit CADO-NFS sidecar build/trigger design needs (D6)

- **Deliverable.** A findings section specifying the **design space** (not the build) for the dynamic
  CADO-NFS sidecar: build-trigger model (CI-eager vs lazy/on-demand), version pinning strategy, what
  regression/comparison tests it would gate, and how it honors the arc-1 dev-oracle policy (CADO as
  opt-in validation sidecar, *never* part of how rGNFS computes).
- **Falsifiable findings (≥1).** "sidecar trigger = lazy on-demand + opt-in CI flag; pins CADO vX.Y →
  ORACLE builds to this interface" — each design choice is a checkable spec ORACLE consumes. *Feeds
  ORACLE; seeds C-Oracle (sketch).*
- **Subtleties.** This is design-*heavy* but still **findings-only** — A.5 records the recommended
  design and its open questions; ORACLE *builds* it. The line is sharp: a build script or CI YAML is
  ORACLE scope, a build-design recommendation is SURVEY scope. If the build automation proves
  intricate enough that the design can't be settled as findings, that is an A.7 open-Q (the ROADMAP
  flags "opus only if build automation proves intricate" — note it, don't resolve it here).
- **Deferral.** No build script, no CI config, no CADO checkout (that is ORACLE).

### A.6 — Audit spectrum completeness + distil 4 quantum-DLP references (D7 + D8) — **Opus**

- **Deliverable.** The arc's one **abductive** audit, answering *"what, if anything, is genuinely
  missing from the integer / finite-field / elliptic-curve × classical / quantum DLP matrix?"* Two
  coupled parts: (a) **spectrum-completeness** (D7) — map the current matrix (`MATHEMATICS.md`'s 11
  chapters + the 5 tracks) against the intent's discriminant (full spectrum, constrained to
  mathematically-necessary algorithms, excluding hardware/distributed specifics); identify omitted
  subject areas. (b) **4-reference distillation** (D8) — read and distil the four external
  quantum-DLP references; **first verify the arxiv identifiers resolve** (`2603.28627`, `2606.02235`
  are dated beyond construction time 2026-06 — if they 404, surface for correction, do not fabricate
  content), then distil what belongs in a pedagogical/reference DLP library.
- **Falsifiable findings (≥1).** "spectrum gap: [specific omitted algorithm / subject area] → F-EXTEND
  candidate" OR the equally-valid **deliberate non-extension** finding "spectrum complete for the
  stated discriminant → F-EXTEND reduces to a recommendations note (a successful outcome, not a gap)".
  Each candidate names the algorithm, its matrix cell, and the reference justifying it. *Feeds F.*
- **Subtleties — why Opus.** This is the abductive session: "what is *missing*" is a generative
  question the eliminative/cataloging mode of A.1–A.5 cannot answer, and the cost of a wrong call is
  high (a false gap mis-scopes EXTEND into defocus; a missed gap leaves the spectrum incomplete). The
  D7+D8 merge (confirmed at shard) is because the distillation's purpose *is* to inform the gap
  finding — one coherent question. **Risk to flag at preflight:** the arxiv IDs may not resolve;
  `/plan-run`'s discovery-adjudication should treat a non-resolving ID as an *internal-continue* (note
  the unverifiable reference, distil the resolvable ones) not a HALT.
- **Deferral.** No new chapter, no new code, no algorithm (that is F-EXTEND). The **EXTEND
  scope-ceiling question** (is arc 2 the place to fill a found gap, or does it become an arc-3
  roadmap?) is a human open-Q surfaced at A.7, not decided here.

### A.7 — **◆ @architect** Consolidate findings ledger + freeze SURVEY contracts — **Opus**

- **Deliverable (lower-fidelity by design — it consumes A.1–A.6).** Assemble all six dimension
  findings into the **authoritative findings ledger** in `docs/SURVEY.md`; **freeze C-Findings,
  C-Testing-Philosophy, C-Coherence** (the three SURVEY-owned prose contracts) into this PLAN's
  Cross-session contracts section; record the *sketches* of C-Layout/C-DocsLayer/C-MathSpine/C-Oracle
  (frozen later, at REFACTOR/CONSOLIDATE/ORACLE) with their "to be frozen at <campaign>" tags;
  resolve (or surface to the human) the four ROADMAP open questions (the `rho`-overload split,
  C-Testing-Philosophy lean, F-EXTEND scope ceiling, C-Coherence borderline-grouping default); fold
  the durable findings up into the ROADMAP Discoveries log.
- **Falsifiable findings (≥1).** The frozen C-Findings ledger *is* the contract — it is the
  authoritative scope source every campaign is checked against. Falsifiability = each ledger entry
  names a finding, its consuming campaign, and the boundary between "in scope" and "defocus" for that
  campaign.
- **Subtleties — why ◆ + @architect + Opus.** This is the inflection point: the static→action frame
  transform that scopes all five campaigns. It consumes the action-frame texture of six audit
  sessions (the digest) and produces static-frame updates (the frozen contracts + ROADMAP
  discoveries). The lever-5-weak reality makes this ◆ the *only* correctness gate for the whole
  sub-track — there is no test that catches a mis-scoped campaign. Both the ROADMAP and the lever read
  hold it at Opus; the opt-down is unavailable.
- **Deferral.** A.7 freezes *prose* contracts and *sketches* the compiler/test ones — it does **not**
  freeze C-Layout (that needs REFACTOR's actual restructuring), C-DocsLayer/C-MathSpine (CONSOLIDATE),
  or C-Oracle (ORACLE). Marking those "to be frozen at <campaign>" is correct, not incomplete.

---

## Cross-session contracts

One subsection per contract, tagged compiler-/test-/prose-enforced, with Defined-in and Consumed-by.
SURVEY freezes the three **prose** contracts it owns at the A.7 ◆; the others are **sketched here and
frozen later** at the named campaign (marked *"to be frozen at …"*). These are carried down from the
ROADMAP's coarse cross-track contract table and sharpened to SURVEY grain.

### C-Findings *(prose)* — **to be frozen at A.7**

- **Defined-in:** A.7 ◆ (the consolidated findings ledger in `docs/SURVEY.md`).
- **Consumed-by:** all five campaigns (B/C/D/E/F).
- **Statement:** The survey findings ledger is the authoritative scope source for every campaign. A
  campaign that exceeds its findings is defocus; one that ignores a finding is rigidity. Each entry
  names the finding, its consuming campaign, and the in-scope/defocus boundary.

### C-Testing-Philosophy *(prose)* — **to be frozen at A.3, ratified at A.7**

- **Defined-in:** A.3 (resolved), A.7 (ratified into the ledger).
- **Consumed-by:** ALIGN (coverage-gate decision), CONSOLIDATE, F-EXTEND (new tests honor it).
- **Statement:** The resolved doctrine for "what is the right testing target for toy-scale
  compute-heavy KAT-driven pedagogical code" — coverage target, unit/integration balance, test-length
  norms. *(Content TBD by A.3; the human's lean is an A.7 open-Q.)*

### C-Coherence *(prose)* — **to be frozen at A.7** (catalog assembled across A.2 + A.4)

- **Defined-in:** A.2 (code-depth catalog), A.4 (prose-depth catalog), A.7 (consolidated
  classification).
- **Consumed-by:** REFACTOR (identifier/file/dir/bench/test renames — code half), CONSOLIDATE (prose
  re-anchoring — prose half), binding on EXTEND naming.
- **Statement:** The artifact-stands-on-its-own-terms principle: the finished library is organized and
  named by mathematical topic, not by planning track/phase. The catalog of every provenance token at
  every depth, each classified **pure residue** (re-anchor on the mathematics) or
  **grouping-coincides-with-topic** (keep the grouping, change only the planning label).

### C-Layout *(compiler)* — **to be frozen at REFACTOR** *(SURVEY sketches only)*

- **Defined-in:** REFACTOR (not SURVEY). A.2 produces the *layout-audit findings* that scope it.
- **Consumed-by:** CONSOLIDATE, EXTEND.
- **Sketch:** the final crate-peering/module structure. SURVEY records whether `rho` is overloaded and
  whether `shared/*` carries dedup — it does **not** decide the new layout (REFACTOR does, at its own
  opus juncture).

### C-DocsLayer *(prose)* & C-MathSpine *(prose)* — **to be frozen at CONSOLIDATE** *(SURVEY sketches)*

- **Defined-in:** CONSOLIDATE. A.4 produces the docs-layer + math-continuity findings that scope them.
- **Consumed-by:** all future docs (C-DocsLayer); EXTEND chapters (C-MathSpine).
- **Sketch:** the three-layer reference-direction discipline (C-DocsLayer); the textbook's
  suggested-path spine, notation, voice, audience (C-MathSpine). SURVEY records where they cohere and
  break today; CONSOLIDATE designs and freezes them.

### C-Oracle *(test)* — **to be frozen at ORACLE** *(SURVEY sketches only)*

- **Defined-in:** ORACLE. A.5 produces the build-design findings that scope it.
- **Consumed-by:** the regression/comparison suite.
- **Sketch:** the CADO-NFS sidecar build + invocation interface (automatic + on-demand). SURVEY
  recommends the design (trigger model, pinning, gated comparisons); ORACLE builds and freezes it.

### C-TemplateSeed *(prose)* — informational, recorded at A.1

- **Defined-in:** A.1 (the bidirectional formalities set), refined by ALIGN.
- **Consumed-by:** ALIGN; future template-derived projects.
- **Statement:** the agreed bidirectional formalities set — what flows template→rGNFS and what flows
  rGNFS→template. SURVEY records the gap + recommended direction; ALIGN reconciles.

---

## Progress ledger

All rows `pending` at shard time. `/plan-run` maintains this (Status pending→done, Commit hash, the
contract(s) each session froze).

| #   | Session                                                                | Status  | Commit | Froze |
|-----|------------------------------------------------------------------------|---------|--------|-------|
| A.1 | Audit template-formalities gap, both flow directions                   | done    | b7bb139 | C-TemplateSeed (informational, recorded) |
| A.2 | Audit crate layout/dedup + catalog code-depth provenance               | done    | 8be4edd | C-Layout sketch (rho overload open-Q); C-Coherence code-half (all pure residue) |
| A.3 | Audit testing balance + resolve the coverage-doctrine question         | done    | be31c37 | C-Testing-Philosophy (frozen: math-behavior KAT coverage; 80% line gate; gate value open-Q at A.7) |
| A.4 | Audit docs-layer discipline + math-continuity + prose provenance       | done    | a99b61e | C-DocsLayer sketch; C-MathSpine sketch; C-Coherence prose-half (S.A/S.B/S.C borderline open-Q at A.7) |
| A.5 | Audit CADO-NFS sidecar build/trigger design needs                      | done    | ab62f21 | C-Oracle sketch (trigger=lazy/on-demand; pin=git-2.0.1; 4 tolerance-bounded comparisons) |
| A.6 | Audit spectrum completeness + distil 4 quantum-DLP references          | done    | 61ef5ff | F-D7-01 (hyperelliptic Jacobian DLP gap — F-EXTEND candidate); arxiv IDs resolved (risk flag not triggered) |
| A.7 | ◆ Consolidate findings ledger + freeze SURVEY contracts                | pending | —      | —     |

---

## Action-frame digest

The externalized action frame `/plan-run` appends to on non-trivial iterations (a surprising finding,
a dimension that audits larger than expected, a provenance token that resists residue/grouping
classification, a non-resolving arxiv ID) and that the A.7 ◆ juncture fork consumes.

### A.6 — 2026-06-21
Discovery/flex: Genuine spectrum gap found: hyperelliptic Jacobian DLP (Gaudry index calculus) is absent — the GHS chapter reduces ECDLP to it but does not develop the solve; substrate already present in `rho/src/hyperelliptic/`.
Affected: F-EXTEND scope (C-Findings will name this as the only F-EXTEND algorithmic candidate)
Deferred: yes — EXTEND scope-ceiling (arc-2 vs arc-3) is a human open-Q at A.7
Texture: All 4 arxiv IDs resolved (risk flag not triggered); 3 x 2026 hardware/circuit papers are outside the discriminant but feed CONSOLIDATE citation updates (§11.4.4, §11.5.1). F-EXTEND scope is narrow: one genuine gap, not a broad expansion.

---

## Discoveries & risks

Carried *down* from the ROADMAP Discoveries log (D1–D9), phrased as `/plan-run` reads for discovery
adjudication (internal-continue / additive-reshard / destructive-HALT). The reverse flow —
discoveries accrued here folded back *up* into the ROADMAP — happens at the A.7 ◆ and at the next
`/plan-shard` boundary reconciliation, not mid-session.

- **The findings-vs-fixes line is the central SURVEY defocus risk.** A session that edits a `.rs`,
  manifest, config, or doc *content* (vs the SURVEY findings file) has broken scope.
  **Internal-continue → SURVEY records what should change in `docs/SURVEY.md`; the campaigns change
  it.** Any code/config/refactor edit is a HALT-and-surface defocus signal, not an accommodation —
  the audit-substrate analogue of "Z.1/T.Z must not write code."
- **D1 formalities gap (confirmed partial) — A.1.** Root lacks `deny.toml`, `rustfmt.toml`,
  workspace-root `rust-toolchain.toml`, `LICENSE`, the `[lints]` table (present in only 3/9 crates), a
  coverage gate, `development.md`. **Internal-continue → A.1 records each item + flow direction;**
  ALIGN implements. A surprise (item present but mis-scoped) is an internal note, not a reshard.
- **D2 testing-doctrine meta-question (open, abductive) — A.3.** Is 100% line coverage right for
  toy-scale compute-heavy KAT-driven pedagogical code? **Internal-continue → A.3 resolves a doctrine
  (math-behavior KAT/property coverage as the strong default) and surfaces the human's lean at A.7;**
  this freezes C-Testing-Philosophy. Not a reshard trigger.
- **D5 + D9-code: layout/dedup + code provenance (open) — A.2.** Is `rho` overloaded (hosts ρ + the 8
  Track-E attack modules)? Dedup across `shared/*`? Provenance: ~1 genuine identifier hit, ~15
  `Phase N` doc-comment tokens, ~300 `X.Y` tokens (doc-comment-heavy, identifier-light).
  **Internal-continue → A.2 catalogs + classifies residue/grouping;** REFACTOR renames. The
  `rho`-split is a human open-Q at A.7, not an A.2 decision.
- **D3 + D4 + D9-prose: docs-layer + math-continuity + prose provenance (open) — A.4.** Does the
  11-chapter MATHEMATICS.md read as a continuous textbook or a stitched reference? Known concrete
  defects (stale ToC "to be appended" labels; chapter-pairing-table Track-E row pointing at the wrong
  file). **Internal-continue → A.4 characterizes + lists concrete defects;** CONSOLIDATE rewrites.
  Sample-and-characterize, not exhaustive re-read.
- **D6 oracle design (open, design-heavy) — A.5.** Build-trigger model, version pinning, gated
  comparisons, dev-oracle-policy fidelity. **Internal-continue → A.5 recommends the design as
  findings;** ORACLE builds. If the build proves intricate, that is an A.7 open-Q (opus-ORACLE
  flag), not an A.5 HALT.
- **D7 + D8: spectrum-completeness + 4-reference distillation (open, abductive) — A.6.** **Risk
  flag:** the arxiv IDs `2603.28627` / `2606.02235` are dated beyond construction time and may not
  resolve. **Internal-continue → A.6 verifies the IDs; a non-resolving ID is noted as unverifiable
  and the resolvable references distilled — NOT a HALT, NOT a fabrication.** A genuine spectrum gap →
  F-EXTEND candidate; spectrum complete → deliberate-non-extension finding (a successful outcome).
  The EXTEND scope-ceiling (arc-2 vs arc-3) is a human open-Q at A.7.
- **Risk — additive reshard at A.7.** If A.1–A.6 surface *more* dimensions than the 9 chartered (an
  audit dimension the ROADMAP missed), that is an **additive reshard** surfaced at A.7, never a silent
  scope expansion. If a finding *invalidates the arc-2 ROADMAP itself* (e.g. a campaign is moot), that
  is a **destructive-HALT** to the human — the one thing the weak inner loop cannot ride through.

---

## Notes for executors

- **Read `docs/ROADMAP.md` first** (the arc-2 design intent + the Sub-track DAG charge for A. SURVEY:
  "audit every dimension and emit a findings/recommendations ledger; **no design, no code, no
  refactor**", dimensions 1–9) and this PLAN before any session.
- **Tier routing.** A.1–A.5 are **Sonnet `@build`** (mechanical-judgment audit against explicit
  dimension charges — the substrate is *read*, the findings are *recorded*, not designed). **A.6 is
  Opus** (abductive: "what is genuinely missing" is a generative question Sonnet's eliminative mode
  under-serves, and the cost of a false/missed gap is high). **A.7 ◆ is Opus `@architect`** (the
  inflection ledger-freeze + static→action frame transform). The A.7 ◆ fork pages `@plan-juncture` at
  **opus** — `juncture-tier: opus`, the **lever-5 opt-down UNAVAILABLE** (no prose/audit test
  inner-loop; the ◆ findings-ledger review is the only correctness gate).
- **Register: findings prose only** (`STYLE-DOC.md`). SURVEY writes `docs/SURVEY.md` (the findings
  doc), this PLAN's ledger/contracts (A.7), `docs/NOTES.md` (durable framings), and the A.7 ROADMAP
  discoveries fold-up. **No `STYLE-CODE.md` register applies** — SURVEY writes no code. The findings
  register is concrete and falsifiable, not impressionistic: every finding names its consuming
  campaign and its in-scope/defocus boundary.
- **Invariants to preserve.** **Findings, not fixes** (the load-bearing SURVEY invariant — no `.rs`,
  no manifest, no config, no refactor, no doc-*content* edit; a code/config delta is a HALT-and-surface
  defocus signal). **The findings ledger is the authoritative campaign scope** (C-Findings — exceeding
  it is defocus, ignoring it is rigidity). **De-provenancing is cataloged-and-classified, not
  executed** (residue vs grouping-coincides-with-topic; renames are REFACTOR, prose is CONSOLIDATE).
  **A deliberate non-extension is a successful F-finding, not a gap.** **A non-resolving arxiv ID is
  noted, never fabricated.** **The four ROADMAP open-Qs are surfaced to the human at A.7, not decided
  unilaterally.**
- **The VERIFY gate is a pure regression guard** (trivially green — SURVEY touches no code). A test
  delta in any session run means SURVEY drifted into code/refactor — a defocus red flag to HALT.
- **Suggested first invocation:** **`/plan-run docs/PLAN.md halt-at-boundaries`** — the shard pattern
  (6 findings-only audits closing at one `@architect` ◆ ledger-freeze) is an **unproven arc-2 pattern**
  (the first sub-track of a new arc, on a new findings-substrate contract shape), with **no test
  inner-loop to catch a wrong finding** (lever-5 weak) and **one load-bearing inflection** (A.7 scopes
  all five campaigns). The conservative default halts at the A.7 ◆ for the project-critical
  findings-ledger review. *(Tradeoff vs `autonomous`: `halt-at-boundaries` here costs almost nothing —
  there is only one boundary, the A.7 ◆ — and buys a guaranteed human check at the single
  highest-leverage artifact in arc 2. The six audit sessions A.1–A.6 run autonomously up to that ◆;
  only the contract-freeze pauses. Given lever-5 is weak and lever-3 is high, the halt is clearly
  worth it.)*
