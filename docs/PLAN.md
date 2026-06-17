<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-S closes (S.D — post-quantum context + T.S, the Track-S writeup)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **kept at the default; the lever-5 opt-down is NOT available
here (a different posture from S.C, where it was available-but-declined).** S.D is a **prose-only
writeup sub-track** — the Track-S `*.W`-equivalent closeout (the code-tour) paired with **T.S** (the
Track-S mathematics chapter, `MATHEMATICS.md` ch. 11, per the Track-τ folding rule). It freezes **no
code contract**, consumes the documentation-register contract **C-Textbook** (frozen T.0)
read-only, and produces **no input to any downstream sub-track** (Phase ζ's Z.1 umbrella consumes
the *completed library*, not a Track-S code surface). The five levers:

1. **Ambient complexity — moderate.** A mature doc corpus with a frozen register (C-Textbook), clean
   genre templates (the `## E.W` integrative code-tour at `docs/PEDAGOGY.md` §8–18; the ch. 10
   `## Algebraic ECDLP Attacks` chapter in `MATHEMATICS.md`), and a settled markup convention
   (Markdown + MathJax). Low ambient risk for prose.
2. **Irreducible complexity (the FLOOR) — two natural conceptual units.** The Track-S **code-tour**
   (simulator + Shor-factoring + Shor-ECDLP as one integrative survey) is one unit; the **T.S math
   chapter + post-quantum context** (Shor's algorithm mathematics — the *designated payoff* — plus
   the PQC migration landscape) is the second. Neither fractures cleanly below its floor.
3. **Cost of a design error — LOW.** Prose-only; no downstream code consumer; no contract freeze. A
   wrong framing is cheaply re-editable. **(This is the lever that, with lever 4, licenses the larger
   2-session shape over a 3-session split.)**
4. **Correctness-criticality — moderate-low.** Expository: "correctness" is mathematical accuracy +
   register-fidelity (C-Textbook), caught by review, not by a test suite.
5. **Inner-loop bandwidth — WEAK, and this is load-bearing.** **There is no test inner-loop for
   prose.** The VERIFY gate (`cargo test --workspace`) is trivially green throughout — S.D touches no
   code — so no behavioural signal catches drift. Per the tuning law this pushes *toward smaller*
   sessions **and forbids the juncture-tier opt-down**. The compensating inner loop for prose is the
   **◆ juncture fork + human review** (which is why the ◆ is load-bearing here, not ceremonial).

On the levers: lever 3/4-low would license one large session, but the one-line-commit-title corollary
+ lever-5-weak split it into **2 sessions** at the code-tour↔math-chapter seam (the two genres, two
files, two registers — code-first vs maths-first — are a clean conceptual boundary). The lever-5
weakness means the **opt-down is unavailable** (no `@plan-juncture-sonnet`): `juncture-tier: opus`,
and S.D.2 is itself **Opus-tier** because the T.S Shor's-algorithm exposition is a **designated
payoff** (the ROADMAP's Track-τ rule: "the mathematics is itself a designated payoff … Opus-tier" —
the period-finding → hidden-subgroup reduction is the quantum analogue of the MOV/L-notation
payoffs that made T.E/T.G Opus). The ◆ fork pages `@plan-juncture` at opus.

**Scope boundary — S.D is the Track-S WRITEUP ONLY; it is PROSE-ONLY; it writes NO code and NO PQC
implementation.** S.D produces three prose bodies across two sessions: **(1)** the **Track-S
code-tour** (a new `shor/docs/PEDAGOGY.md`, the `*.W`-equivalent closeout surveying the landed S.A
simulator + S.B Shor-factoring + S.C Shor-ECDLP); **(2)** **T.S** — `MATHEMATICS.md` ch. 11, the
maths-first Shor's-algorithm chapter (quantum period-finding, the QFT, order-finding → factoring,
the two-register hidden-subgroup → ECDLP); **(3)** the **post-quantum context** folded into ch. 11's
second half (NIST PQC standardisation, the SIDH/SIKE break, the migration landscape). A `@build`
agent that implements *any* PQC primitive (a lattice scheme, an isogeny, Kyber/Dilithium), adds a
crate or a dependency, writes a benchmark, or touches `shor/src/` has reached past S.D's scope — S.D
is **research-and-write within established literature**, no code (ROADMAP: "Prose-only — no PQC
implementations").

The substrate S.D documents is the **complete, frozen Track-S code** — the simulator (C-StateVec,
C-Sparse, C-QFT, frozen S.A.2 ◆, `5ec563a`), Shor-factoring (C-ModExp/C-OrderFind/C-Factor, frozen
across S.B, `60aa816`/`6cc4c6e`), and Shor-ECDLP (C-PointAdd/C-ECDLPSolve, frozen across S.C,
`82fb198`/`a97e42d`). The shard-time survey (2026-06-17, `@explore` fork against the landed `shor`
crate + the doc corpus) established five grounding facts:

1. **The `shor` crate is entirely undocumented in any PEDAGOGY.md — the Track-S code-tour is written
   from scratch (the load-bearing discovery).** The crate has `## S.A`/`## S.B`/`## S.C` sections in
   `docs/BENCHMARKS.md` (performance) but **zero presence in any code-tour**: no Track-S section in
   `docs/PEDAGOGY.md` (which holds Pollard-rho + the E.W integrative chapter, §1–18) and none in
   `gnfs/docs/PEDAGOGY.md` (which holds the G/D code-tours, §1–71). `shor/docs/` does not exist. This
   **revises the ROADMAP's "S.D = 1–2 sessions"** assumption: the ROADMAP folded T.S into a *post-
   quantum writeup* and assumed the code-tour existed; in fact the code-tour is owed too. This is an
   **additive discovery** (more prose owed, no contradiction) — surfaced as a capture candidate.

2. **`MATHEMATICS.md` ch. 11 (T.S) is a ToC stub only — also written from scratch.** Ch. 11
   ("Shor's Algorithm and Post-Quantum Context") is named in the ToC (line 155, *"to be appended"*)
   with **zero body text**. The highest existing chapter is ch. 10 (Algebraic ECDLP Attacks,
   `docs/MATHEMATICS.md:2671`, ~535 lines), so T.S is correctly **ch. 11**. Scattered one-line Shor
   mentions exist in the Through-Line (§3), On Scale (§5), and references (#14, Shor 1994) — these
   are the hooks T.S resolves, not existing content. **No "post-quantum", "NIST", "SIDH", "lattice"
   (PQC sense) content exists anywhere in the repo.**

3. **The code-tour location is `shor/docs/PEDAGOGY.md` (new) — resolving a stale-pairing-table
   ambiguity (user-adjudicated at shard time, 2026-06-17).** The `MATHEMATICS.md` chapter-pairing
   table (line 104, written before the `shor` crate existed) points the Track-S code-tour at
   `gnfs/docs/PEDAGOGY.md` — but `shor` is **not part of `gnfs`**, so that target is structurally
   wrong. The chosen location mirrors the per-crate precedent `shared/numth/docs/PEDAGOGY.md`. **The
   pairing-table row is stale and owes a one-line fix** — out of `@architect` PLAN-write scope,
   surfaced as a capture candidate for the ROADMAP/MATHEMATICS.md.

4. **`docs/BENCHMARKS.md` is complete for Track S — S.D touches it NOT.** `## S.A` (line 527),
   `## S.B` (605), `## S.C` (676) all exist, each with the established genre (intro prose + qubit-
   budget/results tables + a `### Science↔engineering note (principle 4)` + a test-coverage table).
   S.D adds **no benchmark** (it adds no code), so it appends **no `## S.D`** section. The code-tour
   *cites* the existing BENCHMARKS sections; it does not extend them.

5. **C-Textbook is the only contract in play, consumed read-only.** The documentation register —
   audience (undergraduate maths background), depth (survey with proof-sketch depth; full proofs only
   at designated payoffs), through-line (structure-based escape from search), markup (Markdown +
   MathJax) — is frozen (`5ec563a`-era T.0). T.S obeys it; the **Shor period-finding → HSP reduction
   is the chapter's designated full-proof payoff** (the C-Textbook carve-out for payoff proofs). The
   code-tour follows the code-first genre (module surface + KAT + one-line cross-reference to the
   math chapter), no MathJax beyond inline Unicode, per the existing §8–18 / §1–71 precedent.

The work splits at **one code-tour↔math-chapter contract-sharp seam (the two genres / two files /
two registers), 2 sessions** (the ROADMAP "1–2 sessions", revised additively for the missing
code-tour; the soft seam for an additive S.D.2a/S.D.2b — math chapter vs PQC survey — is named
below):

1. **S.D.1 — Track-S code-tour: simulator + Shor-factoring + Shor-ECDLP (Sonnet, Cat I).** A fresh
   `shor/docs/PEDAGOGY.md` — the `*.W`-equivalent integrative code-tour for the whole Track-S code
   surface, following the `docs/PEDAGOGY.md` §8–18 (Track-E integrative) genre: an at-a-glance table
   (the three sub-tracks S.A/S.B/S.C, their module surfaces, frozen contracts, and toy fixtures), a
   per-piece code-first narrative (the state-vector simulator + gate set + sparse path + measurement +
   QFT; the reversible modular arithmetic + order-finding + `factor` driver; the toy curve + reversible
   point-addition + two-register ECDLP solve), the cross-phase contract view (C-StateVec → C-ModExp →
   C-PointAdd, the substrate-reuse story), the design-statement verification (principles 1/3/4 against
   the realised Track-S code — the ~25-qubit ceiling as the principle-4 resource wall), a KAT summary,
   and cross-references to T.S (ch. 11, to be written next) + the BENCHMARKS `## S.A`/`## S.B`/`## S.C`
   sections. **Freezes nothing** (prose; the only "contract" is register-conformance to C-Textbook).

2. **S.D.2 ◆ — T.S: Shor's-algorithm mathematics (payoff) + post-quantum context (Opus, Cat I).**
   `MATHEMATICS.md` ch. 11 — the maths-first Shor chapter: the **quantum period-finding payoff
   proof** (superposition → modular exponentiation / point-addition → QFT phase estimation →
   continued-fraction / 2D-lattice extraction; *why* the period of `a ↦ g^a mod N` yields a factor,
   and *why* the 2D hidden subgroup of `(a,b) ↦ a·G + b·Q` yields the discrete log — the quantum
   "escape from search" that **dissolves** the L-notation bound, the chapter's through-line payload),
   followed by the **post-quantum context** (NIST PQC standardisation — lattice/code/hash/isogeny
   families; the SIDH/SIKE break as the cautionary counter-example; the migration landscape). Folds
   T.S into ch. 11 per the Track-τ pairing. Resolves the ToC stub; extends the references section
   (Proos–Zalka, Nielsen–Chuang, the NIST PQC standards, Castryck–Decru). Crosses the **S.D ◆
   boundary** — **Track S / Phase ε is complete end-to-end** (the quantum-attack arc closed,
   contextualised in the broader cryptanalytic and post-quantum landscape).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing any PQC primitive — a
lattice scheme, an isogeny, a signature — instead of *surveying* it; adding a crate/dependency/
benchmark; touching `shor/src/`; writing the code-tour into `gnfs/docs/PEDAGOGY.md` per the stale
table instead of the chosen `shor/docs/PEDAGOGY.md`; re-deriving Shor's mathematics in the code-tour
instead of citing T.S) and **rigidity** (compressing the period-finding payoff proof to a sketch
because "the code-tour shows it works" — T.S's Shor exposition *is* the designated full-proof payoff,
principle of the C-Textbook carve-out; or refusing to extend the references / prerequisites where the
quantum material genuinely needs a new citation — principle 4).

**Scoping discipline.** S.D documents and contextualises at **survey-with-proof-sketch depth**
(C-Textbook), with the **Shor period-finding reduction as the one full-proof payoff**. The
**principle-4 science↔engineering gap is load-bearing and explicit in two registers:** (a) the
~25-qubit simulator ceiling (the resource-scale wall already annotated in BENCHMARKS `## S.A`–`## S.C`)
is restated in the code-tour's principle-4 summary and in T.S — the simulator exhibits Shor's
*mathematics* (period-finding, the HSP reduction) correctly at toy scale, while the quantum *speedup*
requires real quantum hardware out of scope by construction; (b) the post-quantum context is *itself*
a principle-4 statement at project scale — the entire classical-attack arc (Tracks G/D/E) and the
quantum arc (Track S) demonstrate *why* the migration is happening, without rGNFS implementing any
PQC replacement (the migration landscape is surveyed, not built — the project's pedagogical terminus,
not a new construction track).

---

## Purpose (design intent)

Per ROADMAP (Phase ε, S.D): "*S.D — Post-quantum context writeup. 1–2 sessions. No structural
predecessor. NIST PQC, the SIDH break, the migration landscape. Prose-only — no PQC implementations.
Sonnet (this is a research-and-write task within an established literature).*" Per the Track-τ
folding rule (ROADMAP §Phase τ): "*T.S … folds into S.D … the writeup session produces two siblings:
the code-tour chapter in PEDAGOGY.md and the maths-first textbook chapter … except where the
mathematics is itself a designated payoff … which are Opus-tier.*" And per the design statement (item
7): "*A post-quantum context chapter situating the classical work in the broader migration
landscape.*"

S.D is **the Track-S closeout and the project's penultimate content sub-track** — the writeup that
turns the landed Shor simulator + factoring + ECDLP code into the two complementary durable artifacts
every track produces (a code-first code-tour and a maths-first textbook chapter), and that closes the
quantum arc by situating it in the post-quantum migration landscape (design-statement item 7). Where
Tracks G/D/E demonstrated the *classical* structure-based escapes from the √n / L-notation search
bound — and where Shor's algorithm (Track S code) **dissolves** that bound entirely in the quantum
model — S.D is where the project *says what that means*: the mathematics of the quantum escape (T.S,
the designated payoff), and the cryptographic consequence (the migration to post-quantum primitives,
surveyed not built). It is the natural terminus of the project's through-line ("structure-based escape
from search") before the Phase-ζ umbrella (Z.1) and the textbook bind (T.Z) close the whole library.

The deliverable is two-fold (the two conceptual units, each a session):

1. **The Track-S code-tour (S.D.1).** A fresh `shor/docs/PEDAGOGY.md` — the integrative code-first
   survey of the landed Track-S code (S.A simulator, S.B Shor-factoring, S.C Shor-ECDLP), in the
   `*.W` genre (at-a-glance table → per-piece narrative → contract view → design-statement
   verification → KAT summary → cross-references). The `shor` crate is currently undocumented in any
   code-tour; this writes it from scratch. **Freezes nothing.**

2. **T.S + post-quantum context (S.D.2 ◆).** `MATHEMATICS.md` ch. 11 — the maths-first Shor chapter
   (quantum period-finding as the designated full-proof payoff, applied to factoring and ECDLP) +
   the post-quantum context (NIST PQC, the SIDH break, the migration landscape). The maths-first
   sibling of the S.D.1 code-tour; closes Track S. **Freezes nothing** (consumes C-Textbook
   read-only).

S.D is **prose-only** (it implements no PQC, adds no crate, no dependency, no benchmark, touches no
`shor/src/`), **substrate-documenting** (it explains the complete frozen Track-S code rather than
extending it), and **principle-4-honest** (the simulator's ~25-qubit ceiling and the project-scale
"migration surveyed, not built" posture are both annotated). Re-read this intent at the ◆ boundary to
catch defocus (any PQC implementation, a crate/dep/bench, the wrong code-tour file, re-deriving Shor
in the code-tour) and rigidity (compressing the period-finding payoff proof, refusing a needed
reference/prerequisite extension).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only
`[workspace]` members + a `[profile.bench]`); raw `cargo` is the only CI surface (unchanged since
S.A / S.B / S.C). **S.D writes NO code** — it adds prose files (`shor/docs/PEDAGOGY.md`) and edits
prose files (`docs/MATHEMATICS.md`) only. The VERIFY gate is therefore a **pure no-regression gate**
(every existing crate's KATs stay green because S.D changes no source, no manifest, no module
surface), with **no new test** to add and **no new compile** target. `/run-plan` re-discovers at
preflight. The honest gate for a prose sub-track is **the no-regression VERIFY (trivially green) +
the C-Textbook register-conformance review at the ◆** — the weak-lever-5 reality made explicit:

- **The entire existing workspace KAT suite must stay green** — S.D touches no `.rs`, no
  `Cargo.toml`, no module. `cargo test --workspace` is a pure regression guard; it adds nothing and
  must change nothing. *(If a session run reports any test delta, that is a red flag that S.D has
  drifted into code — a defocus signal to HALT, not to accommodate.)*
- **`cargo check --workspace` must stay green** — trivially, for the same reason.
- **No `cargo bench` involvement** — S.D adds no benchmark (`docs/BENCHMARKS.md` is unchanged; the
  `## S.A`/`## S.B`/`## S.C` sections are cited, not extended). `VERIFY_BENCH` is N/A for S.D.
- **The real correctness gate is review, not tests (lever-5-weak made explicit).** Prose correctness
  = mathematical accuracy + C-Textbook register-conformance + cross-reference integrity (the
  code-tour↔T.S↔BENCHMARKS citations resolve). This is **not** machine-checkable; it is the human
  review + the ◆ juncture's job. The MathJax renders (no build-time validation — a known C-Textbook
  tradeoff, frozen at T.0); a renderer spot-check is the substitute.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| S.D.1 | Track-S code-tour: simulator + Shor-factoring + Shor-ECDLP | I | Sonnet | C-Textbook (frozen T.0 — the documentation register; the code-tour follows the code-first `*.W` genre, citing the math chapter for the mathematics); the frozen Track-S code surface (C-StateVec/C-Sparse/C-QFT — S.A; C-ModExp/C-OrderFind/C-Factor — S.B; C-PointAdd/C-ECDLPSolve — S.C) as the documented subject; `docs/BENCHMARKS.md` `## S.A`/`## S.B`/`## S.C` (cited, not edited) | `shor/docs/PEDAGOGY.md` (new: the Track-S integrative code-tour — at-a-glance table, per-piece narrative for S.A/S.B/S.C, contract view, design-statement verification, KAT summary, cross-references) |
| S.D.2 ◆ | T.S: Shor's-algorithm mathematics (payoff) + post-quantum context | I | Opus | C-Textbook (frozen T.0 — the register + the designated-payoff carve-out for full proofs); the S.D.1 code-tour (cited as the realisation sibling); the existing `MATHEMATICS.md` Through-Line (§3), On Scale (§5), and references (#14 Shor 1994) hooks | `docs/MATHEMATICS.md` (edit: fill ch. 11 "Shor's Algorithm and Post-Quantum Context" — currently a ToC stub at line 155; add the chapter body after ch. 10 which ends at EOF ~line 3206; extend the references section ~line 1180–1226; reconcile the chapter-pairing table row at line 104 IF in scope, else flag) |

**Sequencing notes.** Strictly serial: **S.D.1 → S.D.2.** S.D.1 writes the code-first code-tour (the
realisation survey); S.D.2 writes the maths-first chapter (which the code-tour cross-references) and
closes the sub-track at the ◆. *(Order rationale: code-tour first so the math chapter can cite a
stable `shor/docs/PEDAGOGY.md` §-structure for the "code realisation" pointers — the reverse order
was considered and declined at shard time, see the why-2-sessions note. Tradeoff named: writing the
code-tour first means S.D.1 cannot yet cite a finished T.S for "the mathematics"; it cites "ch. 11,
to be appended" forward-references, which S.D.2 then satisfies — a minor asymmetry, accepted because
the code-tour's cross-references are one-liners and the math chapter's "code realisation §N"
pointers benefit more from a stable target.)* **One `@architect` marker:** the **S.D.2 ◆** (the
sub-track-boundary juncture — ratifying the Track-S closeout + the post-quantum terminus). *(Tradeoff
named: S.D pages a juncture only at the ◆-close, NOT at the open — same as S.A/S.B/S.C. There is no
substrate-design judgment at the open: the code-tour genre is settled (§8–18 / §1–71 precedent) and
the math chapter obeys frozen C-Textbook; the integrative judgment — does the writeup faithfully
represent the landed code, land the period-finding payoff at full-proof depth, survey the PQC
landscape accurately, and stay prose-only? — concentrates at the ◆ close. The juncture-tier is
`opus`, the opt-down UNAVAILABLE per lever-5-weak, recorded in the header.)*

**Why 2 sessions (the ROADMAP "1–2 sessions", revised additively).** The split is taken at the single
code-tour↔math-chapter contract-sharp seam (two genres, two files, two registers):
- **One-line-commit-title corollary.** "Track-S code-tour: simulator + Shor-factoring + Shor-ECDLP"
  and "T.S: Shor's-algorithm mathematics + post-quantum context" are **two distinct commit titles**.
  Bundling them — "write the code-tour AND the Shor math chapter AND the post-quantum survey" — fails
  the corollary, and the survey's discovery (the code-tour is owed, not just the PQC writeup) makes
  the bundle materially larger than the ROADMAP's 1-session minimum assumed.
- **Two conceptual units kept whole (lever 2).** S.D.1 is the code-first integrative survey (one
  unit — the realisation tour across S.A/S.B/S.C); S.D.2 is the maths-first chapter + PQC context
  (the second unit). Neither fractures cleanly: the code-tour is one `*.W`-genre document; the math
  chapter is one ToC entry (ch. 11, titled to include the PQC context as its second half).
- **Contract-sharp boundary (the genre/register seam).** The two halves obey *different* registers —
  the code-tour is code-first (module surface + KAT + cross-reference, no full proofs), the math
  chapter is maths-first with the C-Textbook designated-payoff carve-out (the period-finding full
  proof). The math chapter *consumes* the code-tour (cites its §N for "code realisation"); the
  code-tour *forward-references* the math chapter (cites "ch. 11" for "the mathematics"). The seam
  is the project's settled code-tour↔textbook pairing pattern (`MATHEMATICS.md` §Chapter-pairing).
- **Lower lever 3/4 license the larger unit; weak lever 5 forbids the opt-down.** Prose drift is
  cheap to fix (lever 3/4-low → 2 sessions over 3), but there is no test inner-loop to catch it
  (lever 5-weak → the ◆ + review IS the inner loop, and the juncture stays at opus).

**The softest seam — could S.D.2 split into T.S the math chapter (S.D.2a) and the post-quantum context
(S.D.2b)?** The ROADMAP titles ch. 11 "Shor's Algorithm **and** Post-Quantum Context" — one chapter,
two halves: the Shor mathematics (the designated full-proof payoff, Opus) and the PQC migration survey
(research-and-write within established literature, Sonnet-shaped). A planner could split them. The
chosen shard keeps **the whole ch. 11 in S.D.2**, because (a) the ToC entry, the chapter-pairing
table, and the design statement all treat it as *one* chapter; (b) the PQC survey is a chapter
*subsection* (a few thousand words situating the work), **not a full session's worth** — splitting it
out would produce a sub-band commit with no contract seam worth taking (there is no freeze between the
Shor-math half and the PQC-survey half — both are prose in one chapter, one register). **If S.D.2
overruns** (the period-finding full-proof payoff — the QFT phase-estimation argument plus the 2D
hidden-subgroup lattice for ECDLP, the chapter's hardest exposition — plus a thorough PQC survey
pushes past the session band; plausible for a payoff chapter with two distinct payloads), the escape
applies: **split at the math↔survey seam** (T.S the Shor mathematics in S.D.2a, Opus, the payoff; the
post-quantum context in S.D.2b ◆, Sonnet, the survey) — an additive-reshard surfaced at the S.D.2
readout or by S.D.2 once the payoff-proof length is concrete, never a silent overrun. This is the one
place the 2-vs-3 sizing is genuinely uncertain until the period-finding payoff's true length is
visible — the prose analogue of S.C's "circuit reversibility-KAT size" uncertainty.

---

## Session detail

S.D.1 is specified at near-full fidelity (the code-tour genre is settled — the `docs/PEDAGOGY.md`
§8–18 Track-E integrative chapter and the `gnfs/docs/PEDAGOGY.md` §1–71 per-stage tours are the
templates; the subject — the landed S.A/S.B/S.C code — is fully frozen and surveyed; the design
choices are the section breakdown and the §-numbering scheme, resolved below). S.D.2 is specified at
the structural level (the ch. 11 outline + the per-half content sketched) — correct per the
substrate-first discipline: the math chapter's exact cross-reference targets (the code-tour §N
pointers) are crisp only after S.D.1's §-structure freezes.

### S.D.1 — Track-S code-tour: simulator + Shor-factoring + Shor-ECDLP (Sonnet, Cat I)

**Deliverable:** a fresh `shor/docs/PEDAGOGY.md` — the integrative code-first code-tour for the
complete Track-S code surface, in the `*.W` genre (modelled on `docs/PEDAGOGY.md` §8–18, the Track-E
integrative chapter). The pieces:
- **At-a-glance section** — a taxonomy-style table over the three sub-tracks: **S.A** (state-vector
  simulator: `statevec`/`gates`/`sparse`/`measure`/`qft`; C-StateVec, C-Sparse, C-QFT), **S.B**
  (Shor-factoring: `arith`/`shor`; C-ModExp, C-OrderFind, C-Factor; factors 15/21/35/91), **S.C**
  (Shor-ECDLP: `curve`/`ecc`/`ecdlp`; C-PointAdd, C-ECDLPSolve; 4-bit toy curve) — each row naming
  the module surface, the frozen contract, the toy fixture, and the BENCHMARKS section.
- **Per-piece code-first narrative** — for each sub-track, a "what it realises / the module surface
  (frozen contract) / the toy KAT / cross-reference" passage (the §9–§14 genre): the dense+sparse
  state-vector register and the gate/QFT/measurement surface (S.A); the reversible modular-arithmetic
  builders + order-finding orchestration + continued-fraction extraction + `factor` driver (S.B); the
  `u64` toy curve + reversible controlled point-addition (permutation-synthesis) + two-register
  period-finding + 2D-lattice extraction + `solve_ecdlp` driver (S.C). **Code-first: assume the
  reader knows the mathematics, cite T.S (ch. 11) for it.**
- **The cross-phase contract view** — the substrate-reuse story (the §15/§58 genre): C-StateVec (the
  dense register + gate set, S.A) is the foundation; C-QFT + measure (S.A) are reused by both S.B and
  S.C; C-ModExp's `arith` primitives (S.B) are *re-consumed* by S.C's point-addition circuit (the
  S.B over-specification paying off); C-PointAdd (S.C) wraps into C-ECDLPSolve. The "Track S adds no
  gate after S.A" invariant.
- **Design-statement verification (principles 1/3/4)** — the §16/§59 genre, against the realised
  Track-S code: principle 1 (algorithmic content complete — the full Shor-factoring and the
  two-register Shor-ECDLP implemented head-on); principle 3 (no engineering optimization — the
  point-addition uses the straightforward permutation-synthesis construction, not a qubit-optimized
  one); principle 4 (the ~25-qubit simulator ceiling as the resource-scale wall, citing the
  BENCHMARKS `### Science↔engineering note` in `## S.A`/`## S.B`/`## S.C`).
- **KAT summary** — the §17/§62 genre: the Track-S test corpus at a glance (the simulator gate KATs,
  the factoring KATs at 15/21/35/91, the ECDLP KATs + the `#[ignore]`-gated `rho` cross-check),
  pointing at the actual `shor/tests/*_kat.rs` files.
- **Cross-references + further reading** — to T.S (`MATHEMATICS.md` ch. 11, the maths-first sibling —
  a forward reference S.D.2 satisfies), the BENCHMARKS sections, and the Shor (1994) / Proos–Zalka
  literature.

Consumes C-Textbook (frozen T.0 — the code-first code-tour register) and the frozen Track-S code as
the documented subject. Reads the existing code-tour templates (`docs/PEDAGOGY.md` §8–18,
`gnfs/docs/PEDAGOGY.md` §1–71) for genre, and the BENCHMARKS `## S.A`–`## S.C` for the performance
facts to cite. **Freezes nothing** (prose; the only "contract" is register-conformance to C-Textbook).

**Quality gate (the prose analogue of a KAT — there is no machine check):** (1) **register-conformance**
— code-first genre, MathJax/inline-Unicode per C-Textbook, the §-structure mirrors the §8–18
precedent; (2) **factual fidelity** — every module surface, contract name, fixture, and KAT named
matches the landed code (the `@build` agent reads `shor/src/` + `shor/tests/` and cites accurately,
not from memory); (3) **cross-reference integrity** — the citations to T.S (ch. 11) and the BENCHMARKS
sections are well-formed (T.S is a forward reference S.D.2 resolves); (4) **§-numbering** — the
Track-S §N continues the project-wide code-tour numbering convention coherently (resolve the scheme:
`shor/docs/PEDAGOGY.md` is a new file — decide whether its sections restart at §1 with a clear
"Track S" chapter header, mirroring how `shared/numth/docs/PEDAGOGY.md` and `gnfs/docs/PEDAGOGY.md`
each carry their own §-sequences, or continue a global count; the per-file restart is the survey-
indicated precedent). **Verify gate:** `cargo test --workspace` green (pure regression — S.D.1
changes no code); `cargo check --workspace` green; MathJax renders (spot-check).

**Subtlety (load-bearing):** (1) **The code-tour is written from scratch — the `shor` crate has zero
prior code-tour** (the survey's load-bearing discovery). This is *not* extending a stub; it is the
full Track-S `*.W` chapter that the ROADMAP's "1–2 sessions" estimate did not account for. (2)
**Code-first discipline — cite T.S, do not re-derive.** The code-tour shows *how the code realises*
Shor's algorithm; the *why* (the period-finding → factor / discrete-log mathematics) is T.S's job.
Re-deriving the QFT phase-estimation argument in the code-tour is rigidity (and duplicates S.D.2). (3)
**Factual accuracy over recall — read the code.** The point-addition circuit landed via
*permutation synthesis* (the S.C.1 digest: the group law computed classically inside
`build_point_add_permutation`, the `λ` register allocated but unused at runtime), NOT the explicit
reversible affine formula the S.C PLAN originally sketched — the code-tour must describe **what
landed**, not what was planned (a `@build` agent citing the planned-but-not-built reversible-inverse
construction would misrepresent the code). (4) **The location is `shor/docs/PEDAGOGY.md`, NOT
`gnfs/docs/PEDAGOGY.md`** (the stale-pairing-table resolution) — writing it into the gnfs file is the
defocus the scope boundary names. (5) **No code, no BENCHMARKS edit** — the code-tour *cites* the
existing `## S.A`–`## S.C` sections; it adds no `## S.D`, no benchmark, no `.rs`.

**Deferred:** the maths-first T.S chapter (S.D.2 — the period-finding payoff proof, the PQC context);
the chapter-pairing-table fix (a ROADMAP/MATHEMATICS.md capture candidate, out of PLAN scope, surfaced
at the ◆); any PQC implementation (out of project scope — S.D is prose-only).

### S.D.2 ◆ — T.S: Shor's-algorithm mathematics (payoff) + post-quantum context (Opus, Cat I)

**Deliverable:** `MATHEMATICS.md` ch. 11 ("Shor's Algorithm and Post-Quantum Context") — the
maths-first sibling of the S.D.1 code-tour + the Track-S close. Structural-fidelity sketch (the
per-section content is crisp once the S.D.1 §-structure freezes the cross-reference targets). The
pieces:
- **The chapter through-line** (the §10.0 genre) — Shor's algorithm as the **quantum** member of the
  "escape from search" taxonomy (the hook already planted in `MATHEMATICS.md` §3 / §5): where the
  classical attacks find *algebraic* structure to escape the √n / L-notation bound, Shor finds a
  *quantum period* and **dissolves** the bound to polynomial time.
- **The quantum period-finding payoff proof (the designated full proof — C-Textbook carve-out)** —
  superposition over an exponent register → modular exponentiation (factoring) / controlled
  point-addition (ECDLP) entangling the work register → QFT phase estimation concentrating amplitude
  on multiples of the inverse period → measurement + continued-fraction (factoring) / 2D-lattice
  (ECDLP) extraction. *Why* the period of `a ↦ g^a mod N` yields a non-trivial factor (the
  order-finding → factoring reduction); *why* the 2D hidden subgroup of `(a,b) ↦ a·G + b·Q` yields
  the discrete log `k` via `b·k ≡ −a mod r`. **This is the chapter's payoff — full proof, not sketch**
  (the quantum analogue of the L-notation payoff in T.G/T.D and the MOV-bridge payoff in T.E). Heavy
  MathJax (display blocks for the QFT, the phase-estimation sum, the continued-fraction / lattice
  recovery).
- **The post-quantum context** (the chapter's second half, ROADMAP design-statement item 7) — NIST
  PQC standardisation (the lattice family — Kyber/ML-KEM, Dilithium/ML-DSA; code-based, hash-based,
  isogeny-based families at survey depth); the **SIDH/SIKE break** (Castryck–Decru 2022) as the
  cautionary counter-example — a candidate that *looked* post-quantum-safe and fell to a classical
  attack, the honest "structure cuts both ways" coda to the through-line; the migration landscape
  (why the classical+quantum arc this project demonstrates *is the reason* the migration is happening).
  **Survey depth, prose-only — no PQC implementation, no code, no construction.**
- **The principle-4 annotations** — the ~25-qubit simulator ceiling (resource-scale wall, the toy
  scale exhibits the *mathematics* not the *speedup*); the project-scale principle-4 statement (the
  migration is *surveyed, not built* — the pedagogical terminus, not a new construction track).
- **References + ToC + pairing-table reconciliation** — extend the references section (~line 1180):
  Proos–Zalka (the ECDLP circuit), Nielsen–Chuang (the quantum-computation text), the NIST PQC
  standards (FIPS 203/204/205), Castryck–Decru (the SIDH break). Resolve the ch. 11 ToC stub (line
  155 — drop the "to be appended"). **The chapter-pairing-table row (line 104, pointing at
  `gnfs/docs/PEDAGOGY.md`) is stale** — fixing it to `shor/docs/PEDAGOGY.md` is in *this file's*
  scope (T.S edits `MATHEMATICS.md`), so the fix is in-session IF the agent judges it part of the
  chapter freeze; otherwise flag it for the ROADMAP capture. *(This is a same-file one-line edit, so
  it is plainly-part-of-unit — distinct from the ROADMAP static-frame debt, which is a different
  file and out of scope.)*
- **The S.D ◆ close** — re-read the Purpose intent; verify the code-tour (S.D.1) + the math chapter
  (S.D.2) are coherent complementary siblings (cross-references resolve both ways); verify the
  period-finding payoff lands at full-proof depth (not compressed to a sketch); verify the PQC
  context is accurate and prose-only (no implementation crept in); verify Track S is closed
  end-to-end (the quantum arc documented + contextualised); confirm S.D stayed prose-only (no code,
  no crate, no dep, no bench, no `shor/src/` edit).

Consumes C-Textbook (frozen T.0 — the register + the designated-payoff carve-out) and the S.D.1
code-tour (the realisation sibling it cross-references). **Freezes nothing.**

**Quality gate (prose analogue):** (1) **payoff depth** — the period-finding reduction is a *full
proof*, the C-Textbook carve-out honoured (the rigidity guard: not compressed because "the code
works"); (2) **register-conformance** — maths-first, MathJax display math for the QFT / lattice
recovery, survey-with-proof-sketch depth elsewhere, C-Textbook audience floor; (3) **factual /
literature accuracy** — the PQC survey is accurate to the established literature (NIST FIPS 203/204/
205, the Castryck–Decru break), prose-only; (4) **cross-reference integrity** — the "code realisation
§N" pointers resolve into the S.D.1 `shor/docs/PEDAGOGY.md` §-structure, the ToC stub is resolved,
the references extended. **Verify gate:** `cargo test --workspace` green (pure regression — S.D.2
changes no code); `cargo check --workspace` green; MathJax renders (spot-check the new display blocks).

**Subtlety (load-bearing):** (1) **The period-finding payoff is the designated FULL proof, not a
sketch** — this is *why* S.D.2 is Opus-tier (the Track-τ rule: "the mathematics is itself a designated
payoff"). The quantum escape from the L-notation bound is the chapter's payload; compressing it is the
central rigidity risk. (2) **Prose-only — survey PQC, do not build it** — the central defocus risk.
NIST PQC, the SIDH break, the migration landscape are *surveyed within established literature*; a
`@build` agent that implements a lattice scheme, an isogeny, or any PQC primitive has broken the scope
(design statement item 7: "no PQC implementations"). (3) **The SIDH break is the honest coda, not a
footnote** — the through-line is "structure-based escape from search"; the SIDH break shows structure
*also* enables the attacker, the principled close to the survey (the same "structure cuts both ways"
honesty as the principle-4 annotations). (4) **Two distinct payloads in one chapter — the soft-seam
risk** — the Shor-math half (full-proof payoff, Opus) and the PQC-survey half (research-and-write,
Sonnet-shaped) cohere as one ch. 11 but are the named additive-reshard seam (S.D.2a/S.D.2b) if the
session overruns. (5) **No code, no BENCHMARKS, no `shor/src/`** — T.S is `MATHEMATICS.md`-only (+ the
same-file ToC/references/pairing edits); the BENCHMARKS `## S.A`–`## S.C` are cited, not extended.

**Deferred:** the Phase-ζ umbrella (Z.1 — the README + master PEDAGOGY.md tying the whole library
together; the cross-track L-notation comparison; all-Opus); the textbook bind (T.Z — the final
consistency pass over all accreted chapters, paired with Z.1); the ROADMAP static-frame reconciliation
(the compounded Progress/Remaining debt — a capture candidate, out of PLAN scope); the chapter-pairing
table fix IF judged out of the chapter-freeze unit (then a MATHEMATICS.md/ROADMAP capture). **S.D
closes Track S / Phase ε; Phase ζ (Z.1) + T.Z close the project.**

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
S.D.2 ◆ to confirm: (1) **the code-tour faithfully represents the landed code** — every module
surface, contract, fixture, and KAT in `shor/docs/PEDAGOGY.md` matches `shor/src/` + `shor/tests/`
(in particular, the point-addition circuit is described as the *permutation-synthesis* construction
that landed, NOT the planned-but-unbuilt reversible-affine-inverse one — the S.C.1/S.C.2 digest
finding); (2) **the period-finding payoff lands at full-proof depth** — the QFT phase-estimation +
order-finding→factoring + 2D-hidden-subgroup→ECDLP reduction is a *proof*, not a sketch (the
designated payoff; the rigidity guard); (3) **the PQC context is accurate and prose-only** — NIST PQC
families, the SIDH/Castryck–Decru break, the migration landscape surveyed within established
literature, **no PQC implementation, no code, no crate, no dep, no bench** (the defocus guard); (4)
**the two siblings are coherent and cross-reference both ways** — `shor/docs/PEDAGOGY.md` ↔
`MATHEMATICS.md` ch. 11, the citations resolve, the ToC stub is filled, the references extended; (5)
**C-Textbook register-conformance** — audience floor, depth, MathJax, through-line all obeyed across
both artifacts; (6) **Track S is closed end-to-end** — the quantum arc (S.A simulator → S.B factoring
→ S.C ECDLP) is documented (code-tour) + contextualised (T.S + PQC), Phase ε complete; (7) S.D stayed
in scope — **prose-only, no code, the code-tour in `shor/docs/PEDAGOGY.md` (not the gnfs file)**.
**Also: surface the compounded static-frame ROADMAP debt** (the Progress/Remaining reconciliation owed
since the E.W ◆, flagged at the S.A ◆, S.B ◆, and S.C ◆ — now a FIFTH consecutive juncture; with S.D
landing, Track S / Phase ε is complete and the ROADMAP tables still show ε at "0 sessions") **and the
stale chapter-pairing-table row** (line 104 → `shor/docs/PEDAGOGY.md`, if not fixed in-session) —
**note both as capture candidates, not PLAN edits**. One-shot findings; does not implement. Held at
**opus** per the header (the lever-5-weak forbids the opt-down — UNAVAILABLE, not merely declined).

---

## Cross-session contracts

S.D **freezes no contract** — it is a prose-only writeup sub-track. It **amends no prior frozen
contract** (it documents the frozen Track-S code surface read-only) and **consumes exactly one
contract read-only: C-Textbook** (the documentation register, frozen T.0). S.D adds no workspace
member, no `shor` dependency (library *or* dev), no benchmark, and no `.rs`. **Unlike S.A/S.B/S.C, S.D
produces no new code contract** — there is nothing for a downstream sub-track to consume; the Phase-ζ
umbrella (Z.1) consumes the *completed library and its documentation*, not a Track-S code interface.
This is the lever-3-low (no design-error propagation) that — combined with lever-4-low — licensed the
larger 2-session shape; but the **absent test inner-loop (lever-5-weak) made the juncture-tier
opt-down UNAVAILABLE** (a different posture from S.C, where it was available-but-declined).

### C-Textbook — the documentation-register contract (prose-enforced) — *frozen T.0, consumed read-only by S.D*

**Defined in:** T.0 (`5ec563a`-era; the contract is documented in `MATHEMATICS.md` §1 and the ROADMAP
§Phase τ scope contract). **Consumed by:** every textbook chapter and every `*.W` code-tour — here,
S.D.1 (the code-tour follows the code-first genre) and S.D.2 (T.S obeys the maths-first register +
the designated-payoff carve-out). Prose-enforced (no compiler/test gate — the C-Textbook tradeoff,
frozen at T.0: MathJax fails silently in a renderer, there is no build-time math validation).

**Ratified shape (consumed as-is — S.D does NOT amend it).** Audience: the interested mathematics
student with a full undergraduate maths background (proofs + intro analysis/algebra/probability/logic).
Depth: **survey with proof-sketch depth** — complete and clinical, not exhaustive, not inscrutable;
**full proofs only at designated payoffs** (T.S's period-finding reduction is one such payoff — the
carve-out S.D.2 invokes). Through-line: **structure-based escape from search** (Shor is the quantum
member; the SIDH break is the "structure cuts both ways" coda). Markup: **Markdown + MathJax** (`$…$`
inline, `$$…$$` display; trivial glyphs may stay Unicode). Location: `docs/MATHEMATICS.md`
(single-file; the promotion to `docs/textbook/` is deferred to T.Z, not S.D). **Invariant for S.D:**
S.D obeys the register; it does not flex it. If the quantum material genuinely needs a register break
(it should not — undergraduate quantum-computation background is within the C-Textbook audience floor
given the prerequisites chapter), that is a discovery to surface at the ◆, not a silent raise.

### Frozen Track-S code surface read by S.D (documented, not amended)

S.D documents — and amends none of — the complete frozen Track-S code:
- **C-StateVec / C-Sparse / C-QFT (frozen S.A.2 ◆, `5ec563a`)** — the simulator substrate (dense +
  sparse register, gate set, QFT/iQFT, seeded measurement). S.D.1 surveys it; T.S explains its
  mathematics.
- **C-ModExp / C-OrderFind / C-Factor (frozen across S.B, `60aa816`/`6cc4c6e`)** — the Shor-factoring
  arithmetic + order-finding + `factor` driver. S.D.1 surveys it (factors 15/21/35/91); T.S proves the
  order-finding → factoring reduction.
- **C-PointAdd / C-ECDLPSolve (frozen across S.C, `82fb198`/`a97e42d`)** — the toy curve + reversible
  point-addition (permutation-synthesis) + two-register period-finding + 2D-lattice extraction +
  `solve_ecdlp`. S.D.1 surveys it (**describing the permutation-synthesis construction that landed**,
  per the S.C digests); T.S proves the two-register hidden-subgroup → discrete-log reduction.

### Downstream contracts S.D does NOT produce (named, to bound scope)

- **No code contract.** S.D is prose; it freezes nothing for any consumer.
- **No PQC primitive, no crate, no dependency, no benchmark.** The post-quantum context is *surveyed*,
  not *built* (design statement item 7). A `@build` agent that implements any PQC scheme, adds a
  crate/dep/bench, or touches `shor/src/` has reached past S.D's prose-only scope.
- **The Phase-ζ umbrella (Z.1) and the textbook bind (T.Z) are NOT S.D.** Z.1 (the master README +
  cross-track synthesis, all-Opus) consumes the completed library; T.Z (the final textbook
  consistency pass) binds all chapters including ch. 11. S.D produces ch. 11 and the Track-S code-tour
  as *inputs* to those closers, nothing more.

### Workspace edges (no new member, no new dependency, no code)

- **No new member, no new dependency (library or dev), no `.rs`, no benchmark.** S.D adds one new
  prose file (`shor/docs/PEDAGOGY.md`) and edits one prose file (`docs/MATHEMATICS.md`). The workspace
  `Cargo.toml`, every crate's `Cargo.toml`, and all source are unchanged; `cargo check`/`cargo test`
  are pure regression guards (trivially green).

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked (S.D freezes none — the column reads "—"). The S.D.2 ◆
`@architect` confirmation is not a separate ledger row (a paged fork with no commit-shaped
deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| S.D.1 | Track-S code-tour: simulator + Shor-factoring + Shor-ECDLP | pending | — | — (prose) |
| S.D.2 ◆ | T.S: Shor's-algorithm mathematics (payoff) + post-quantum context | pending | — | — (prose) |

Contracts frozen before this sub-track: the entire classical-attack arc — all of Track G (GNFS
factoring), Track D (NFS-DL), and Track E (algebraic ECDLP, closed at the E.W ◆) — plus the shared
substrate (C1 smoothness, the field/bigint/numfield/padic/gf2m crates), the Track-τ register
(C-Textbook, frozen T.0), the Track-S simulator substrate (C-StateVec, C-Sparse, C-QFT, frozen at the
S.A.2 ◆, `5ec563a`), the Shor-factoring arithmetic (C-ModExp, C-OrderFind, C-Factor, frozen across
S.B, `60aa816`/`6cc4c6e`), and the Shor-ECDLP circuit (C-PointAdd, C-ECDLPSolve, frozen across S.C,
`82fb198`/`a97e42d`). **S.D consumes C-Textbook read-only and documents the frozen Track-S code
surface; it freezes no new contract and amends none.** **With the S.D ◆, Track S / Phase ε is complete
end-to-end (the quantum arc documented + contextualised in the post-quantum migration landscape);
Phase ζ (the Z.1 umbrella) + the textbook bind (T.Z) remain to close the project.**

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **The Track-S code-tour is owed and was never written — the ROADMAP "1–2 sessions" is revised
  additively (the load-bearing shard-time discovery, 2026-06-17).** The `shor` crate (S.A/S.B/S.C) has
  `## S.A`–`## S.C` BENCHMARKS sections but **zero code-tour** in any PEDAGOGY.md, and ch. 11 (T.S) is
  a ToC stub. The ROADMAP folded T.S into a *post-quantum writeup*, assuming the code-tour existed; it
  does not. **Additive-reshard (taken at this shard): S.D = 2 sessions** (the code-tour + the math
  chapter/PQC context), not 1. A discovery that the code-tour + math chapter together still overrun is
  an **additive S.D.2a/S.D.2b split** (the named soft seam) surfaced at the ◆, never a silent overrun.

- **S.D is PROSE-ONLY — the central defocus risk is implementing PQC (or any code).** Design statement
  item 7: "no PQC implementations." A `@build` agent that implements a lattice scheme, an isogeny, any
  PQC primitive; adds a crate/dependency/benchmark; or touches `shor/src/` has broken scope. **Internal-
  continue → S.D writes `shor/docs/PEDAGOGY.md` + edits `docs/MATHEMATICS.md` and nothing else;** any
  `.rs`/`Cargo.toml`/`benches/` change is a HALT-and-surface defocus signal, not an accommodation.

- **The period-finding payoff is the designated FULL proof — the central rigidity risk is compressing
  it.** T.S's Shor exposition is the chapter's designated payoff (the Track-τ rule → Opus-tier);
  compressing the QFT phase-estimation + order-finding→factoring + 2D-HSP→ECDLP reduction to a sketch
  because "the code-tour shows it works" is rigidity. **Internal-continue → full proof at C-Textbook
  payoff depth; the code-tour cites it, does not duplicate it.**

- **The code-tour describes WHAT LANDED, not what was planned (the S.C permutation-synthesis finding).**
  The S.C point-addition circuit landed via *permutation synthesis* (group law computed classically in
  `build_point_add_permutation`; the `λ` register allocated but unused) — NOT the reversible-affine-
  inverse construction the S.C PLAN originally sketched. **Internal-continue → the code-tour reads
  `shor/src/ecc/` + the S.C digests and describes the permutation-synthesis reality;** citing the
  planned-but-unbuilt reversible-inverse construction would misrepresent the code.

- **The code-tour location is `shor/docs/PEDAGOGY.md` (new), not `gnfs/docs/PEDAGOGY.md` (the stale
  pairing-table target).** The `MATHEMATICS.md` pairing table (line 104) predates the `shor` crate and
  points at the gnfs file, which is structurally wrong (`shor` ∉ `gnfs`). **Internal-continue → write
  to `shor/docs/PEDAGOGY.md` (per-crate, mirroring `shared/numth/docs/PEDAGOGY.md`); the pairing-table
  fix is a same-file one-liner T.S may take in-session, or a capture candidate.**

- **There is no test inner-loop for prose (lever-5-weak) — the ◆ + review IS the inner loop.** The
  VERIFY gate is trivially green throughout (S.D touches no code), so no behavioural signal catches
  drift. **Internal-continue → the quality gate is human review + the opus ◆ juncture (register-
  conformance, factual fidelity, cross-reference integrity, payoff depth); the juncture-tier opt-down
  is UNAVAILABLE.** A test delta in any session run is a defocus red flag (S.D has drifted into code).

- **Static-frame ROADMAP debt (surface at the S.D ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried from the E.W ◆, flagged at the S.A ◆, S.B ◆, and S.C ◆, now compounded by S.D's
  landing into a FIFTH consecutive carry.** The ROADMAP Progress table still shows "ε — Shor + PQ (S)
  … 0 (S.A sharded, not yet executed)" and the Remaining table still lists "S.A" and "S.B–S.D" as
  un-started — stale by the full Track-S arc: S.A (`5ec563a`), S.B (`6cc4c6e`/`2a79bd9`), S.C
  (`82fb198`/`a97e42d`) all landed, and with S.D **Phase ε is complete end-to-end**. The S.A/S.B/S.C
  ◆ digests each named this; the write was deferred (out of `@architect` PLAN-write scope) every time.
  **The S.D close is the fifth and natural-terminal prompt** (Track S done; only Phase ζ + T.Z
  remain). The full reconciliation (mark S.A–S.D done; mark Phase ε complete; advance Remaining to
  Z.1 + T.Z only; reconcile the Track-τ row for T.S folded into S.D) is owed. **This is a ROADMAP
  write — outside the `@architect` PLAN-only write scope; surfaced here as a capture candidate for the
  user to action (via `/note` or a ROADMAP edit), not a PLAN edit.** Not an implementation concern;
  does not block S.D.

- **The chapter-pairing-table row (MATHEMATICS.md line 104) is stale — a same-file capture candidate.**
  It points the Track-S code-tour at `gnfs/docs/PEDAGOGY.md`; the chosen location is
  `shor/docs/PEDAGOGY.md`. The fix is a one-line edit *within `MATHEMATICS.md`*, so T.S (S.D.2) may
  take it as plainly-part-of-the-chapter-freeze unit — or, if judged out of unit, flag it. **Internal-
  continue → fix in-session if part of the ch. 11 freeze, else surface at the ◆.**

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase ε — S.D: "*Post-quantum context writeup. 1–2 sessions … NIST PQC, the
  SIDH break, the migration landscape. Prose-only — no PQC implementations. Sonnet.*"; the Track-τ
  pairing — §Phase τ, "*T.S … folds into S.D … except where the mathematics is itself a designated
  payoff … Opus-tier*"; the design statement item 7 — "*A post-quantum context chapter situating the
  classical work in the broader migration landscape*") and this PLAN before any session. **NOTE: the
  ROADMAP Progress / Remaining tables are stale by the FULL Track-S arc (S.A + S.B + S.C landed; with
  S.D, Phase ε is complete); the S.D close is the fifth reconciliation prompt — surface it at the ◆,
  but it is outside `@architect` PLAN-write scope (a capture candidate).** **CAUTION on the ROADMAP's
  "1–2 sessions":** that estimate assumed S.D = a post-quantum writeup with the Track-S code-tour
  already written; in fact the code-tour was never written (the `shor` crate has zero code-tour
  anywhere) — so S.D is **2 sessions** (code-tour + math/PQC chapter), an additive revision recorded
  in Discoveries.
- Read the **frozen code to document** (do NOT modify): `shor/src/lib.rs` (the crate root + the
  little-endian / QFT-bit-reversal / ~25-qubit conventions); `shor/src/statevec/`, `shor/src/gates/`,
  `shor/src/sparse/`, `shor/src/measure/`, `shor/src/qft/` (S.A — the simulator surface);
  `shor/src/arith/`, `shor/src/shor/` (S.B — Shor-factoring; **note the order-finding + `factor`
  driver, factors 15/21/35/91**); `shor/src/curve/`, `shor/src/ecc/`, `shor/src/ecdlp/` (S.C — the toy
  curve `y²=x³+3 mod 7`, the **permutation-synthesis** point-addition circuit, the two-register
  solve). Read the **test corpus to summarise**: `shor/tests/*_kat.rs`. Read the **templates to
  mirror**: `docs/PEDAGOGY.md` §8–18 (the Track-E *integrative* code-tour — the closest genre for the
  Track-S integrative code-tour: at-a-glance table → per-piece narrative → contract view → design-
  statement verification → KAT summary → cross-references); `gnfs/docs/PEDAGOGY.md` §1–71 (the
  per-stage code-tour genre — the "what it exploits / module surface / toy KAT / cross-reference"
  passage shape); `docs/MATHEMATICS.md` ch. 10 (`## Algebraic ECDLP Attacks`, ~535 lines — the
  maths-first chapter genre: through-line → per-topic subsections with proof sketches and the
  designated-payoff full proofs → L-notation comparison → cross-references → references; heavy
  MathJax). Read the **performance facts to cite** (do NOT edit): `docs/BENCHMARKS.md` `## S.A`/
  `## S.B`/`## S.C` (the qubit-budget tables + `### Science↔engineering note (principle 4)` — the
  ~25-qubit ceiling).
- **Register:** S.D is **PROSE ONLY** (`STYLE-DOC.md`). Two registers: **S.D.1** the code-first
  code-tour register (module surface + KAT + one-line cross-reference, inline-Unicode math per the
  §8–18 / §1–71 precedent — MathJax optional); **S.D.2** the maths-first textbook register
  (C-Textbook: survey-with-proof-sketch depth, full proof at the period-finding payoff, MathJax
  display math). **No `STYLE-CODE.md` register applies** — S.D writes no code.
- **Tier routing:** **S.D.1 is Sonnet `@build`** (the code-tour is a faithful survey of frozen code in
  a settled genre — the judgment is accurate representation + register-conformance, not design).
  **S.D.2 is Opus `@build`** (the T.S period-finding exposition is a **designated payoff** per the
  Track-τ rule — the quantum escape-from-search proof is the chapter's payload, the same Opus
  justification as the L-notation payoff in T.G/T.D and the MOV bridge in T.E). **juncture-tier
  (header) is `opus`** — the **lever-5 opt-down is UNAVAILABLE** (no prose test inner-loop, so the ◆ +
  review is the only correctness gate; this is a stricter posture than S.C, where the opt-down was
  available-but-declined). The ◆ fork pages `@plan-juncture` at opus.
- **Invariants to preserve:** **S.D is prose-only** (no `.rs`, no `Cargo.toml`, no `benches/`, no
  crate, no dependency — a code change is a HALT-and-surface defocus signal). **S.D amends NO frozen
  contract** (it documents the Track-S code surface and consumes C-Textbook read-only). **The
  code-tour describes WHAT LANDED** (the permutation-synthesis point-addition, not the planned
  reversible-affine-inverse construction — read `shor/src/ecc/` + the S.C digests). **The code-tour
  location is `shor/docs/PEDAGOGY.md`** (per-crate, NOT `gnfs/docs/PEDAGOGY.md` per the stale pairing
  table). **The period-finding reduction is a FULL proof** (the C-Textbook designated-payoff carve-out
  — not compressed to a sketch). **The PQC context is SURVEYED, not built** (NIST PQC / SIDH break /
  migration landscape within established literature — no implementation). **The SIDH break is the
  honest "structure cuts both ways" coda** (the through-line's principled close). **Both principle-4
  annotations are present** (the ~25-qubit ceiling = resource-scale wall; the migration =
  surveyed-not-built project terminus). **BENCHMARKS is cited, not edited** (no `## S.D` section — S.D
  adds no benchmark). **C-Textbook is consumed read-only** (the register is obeyed, not flexed).
- **No code, no crate, no dependency, no benchmark, two prose files (load-bearing for S.D).** S.D adds
  one new prose file (`shor/docs/PEDAGOGY.md`) and edits one prose file (`docs/MATHEMATICS.md` — ch. 11
  body + the ToC stub + the references section + optionally the line-104 pairing row). No `.rs`, no
  manifest, no `benches/`; `cargo check`/`cargo test --workspace` are pure regression guards (trivially
  green — a non-green result means S.D drifted into code).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  code-tour session, then a math-chapter + PQC-context session closing at the ◆) is a *new* pattern
  for this project (the first prose-only Track closeout where the code-tour was *missing*, not merely
  paired — every prior `*.W` either had its code already toured or wrote the tour alongside fresh
  code), and S.D introduces **two new judgment surfaces** (a from-scratch Track-S code-tour spanning
  three sub-tracks; the Shor period-finding *full-proof payoff*), the S.D.2 ◆ confirms the Track-S /
  Phase-ε closeout, and **there is no test inner-loop to catch prose drift** — so the conservative
  default is to halt at the S.D.2 ◆ for the human glance + the opus juncture fork. Both sessions are
  prose in settled genres, so S.D.1 could run autonomously, but the missing-code-tour-from-scratch +
  the no-test-loop reality + the Phase-ε-closeout milestone argue for `halt-at-boundaries` on the
  first invocation; the S.D.2 ◆ fork is itself a halt. *(Tradeoff vs autonomous: `halt-at-boundaries`
  trades a little velocity on the mechanical S.D.1 for a guaranteed human check at the Track-S close +
  the payoff-depth verification + the prose-only-scope confirmation — the right trade given lever-5 is
  weak. If S.D.1 lands clean and reads faithfully against the code, S.D.2 can be dispatched
  immediately after the ◆ glance.)*
