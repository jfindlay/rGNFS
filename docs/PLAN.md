<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: the project terminus (Phase ζ — Z.1 umbrella + T.Z textbook bind)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **kept at the default; the lever-5 opt-down is NOT available
here (the same posture as S.D, and the firmest in the project).** This PLAN shards the **two paired
closeout sub-tracks that end the project**: **Z.1** (the Phase-ζ umbrella narrative — the README +
the master `docs/PEDAGOGY.md` tying the whole library together) and **T.Z** (the Phase-τ textbook
bind — the final consistency pass over `docs/MATHEMATICS.md`). Both are **prose-only** and **all-Opus**
(the ROADMAP: "*Z.1 … all sessions Opus-tier — this is the artifact-as-a-whole and the highest
integrative judgment load in the project*"; "*T.Z … Opus-tier — highest integrative judgment load in
the textbook*"). They freeze **no code contract**, consume the documentation-register contract
**C-Textbook** (frozen T.0) read-only, and produce **no input to any downstream sub-track** (there is
none — this is the terminus). The five levers:

1. **Ambient complexity — HIGH, and this is load-bearing.** Z.1/T.Z synthesise the **entire library**:
   9 workspace crates across 5 tracks (ρ, G, D, E, S), **five existing code-tours** (`docs/PEDAGOGY.md`
   ρ+E §1–18; `gnfs/docs/PEDAGOGY.md` G+D §1–71; `shared/numth/docs/PEDAGOGY.md`;
   `shared/numfield/docs/PEDAGOGY.md`; `shor/docs/PEDAGOGY.md`), and an **11-chapter `MATHEMATICS.md`**
   (~4000 lines, all chapters now present through ch. 11). The cross-references that must resolve span
   *every prior artifact in the project*. This pushes **strongly toward smaller sessions** (the
   opposite of S.D's low-ambient license for the larger unit).
2. **Irreducible complexity (the FLOOR) — four natural conceptual units.** The **README umbrella**
   (supersede the rho-scoped README with a workspace-level narrative over all 5 tracks); the **master
   PEDAGOGY.md cross-track synthesis** (turn the ρ+E-only master tour into the library-wide synthesis
   of all 5 code-tours + the cross-track L-notation comparison); the **modularity-theorem speculation
   chapter** (genuinely *new* content — it exists nowhere in the corpus, the one Z.1 deliverable that
   is authored rather than aggregated); and the **T.Z textbook bind** (the maths-first consistency pass
   over all 11 chapters). Each is a whole unit; none fractures cleanly below its floor.
3. **Cost of a design error — LOW-to-MODERATE.** Prose; no downstream code consumer (this *is* the
   terminus — nothing consumes it). **But the README is the project's front-door artifact** — a wrong
   *framing* there is the most-read error in the project, and T.Z's bind is the last word on every
   chapter. Moderate, not low (a different posture from S.D's lever-3-low).
4. **Correctness-criticality — moderate.** Expository: "correctness" = cross-reference integrity
   (every citation across README ↔ 5 PEDAGOGY.md ↔ 11-chapter MATHEMATICS.md resolves) +
   register-fidelity (C-Textbook) + mathematical accuracy of the *new* modularity chapter. Caught by
   review, not by a test suite.
5. **Inner-loop bandwidth — WEAK, and this is load-bearing (the same reality as S.D).** **There is no
   test inner-loop for prose.** The VERIFY gate (`cargo test --workspace`) is trivially green
   throughout — Z.1/T.Z touch no code — so no behavioural signal catches drift. Per the tuning law this
   pushes *toward smaller* sessions **and forbids the juncture-tier opt-down**. The compensating inner
   loop is the **◆ juncture fork + human review** (load-bearing here, not ceremonial).

On the levers: **lever 1 (HIGH ambient) and lever 5 (weak loop) both push toward small**, and the
ROADMAP already flags every session Opus. This is the *opposite* posture from S.D, where lever-3/4-low
licensed the larger unit. The decomposition is therefore **fine-grained: Z.1 splits into 3 sessions**
(README / master-tour synthesis / modularity chapter — three distinct commit titles, the
one-line-commit-title corollary applied), and **T.Z is its own session** (the maths-first bind, a
different register and file from Z.1's code-first umbrella). The lever-5 weakness means the
**opt-down is UNAVAILABLE** (no `@plan-juncture-sonnet`): `juncture-tier: opus`. Two ◆ boundaries —
**Z.1.3 ◆** (the umbrella close, before T.Z consumes the finished synthesis) and **T.Z ◆** (the
**project terminus**).

**Scope boundary — Z.1 + T.Z are PROSE-ONLY; they write NO code.** The work produces four prose bodies
across four sessions: **(1)** a rewritten **`README.md`** — the workspace-level umbrella superseding the
rho-scoped front page; **(2)** the **master `docs/PEDAGOGY.md` cross-track synthesis** — the library-wide
code-tour that names the through-line once and shows each of the 5 existing tours as a chapter in one
story, plus the comparative L-notation table across all 5 tracks; **(3)** the **modularity-theorem
speculation chapter** (new content, location decided in-session — a Z.1 section in the master
PEDAGOGY.md or a new MATHEMATICS.md chapter, see the open design choice in S.D detail); **(4)** **T.Z**
— the consistency-bind pass over `docs/MATHEMATICS.md` (cross-references resolved, the stale ToC /
chapter-pairing-table reconciled, notation unified, the prerequisites chapter checked against what the
chapters actually used). A `@build` agent that writes *any* code, adds a crate/dependency/benchmark,
touches any `*/src/`, or implements anything has reached past scope — Z.1/T.Z are
**research-and-synthesise within the project's own completed artifacts**, no code.

The substrate Z.1/T.Z document is the **complete, frozen library**: all of Tracks ρ/G/D/E/S and the
shared substrate (field/bigint/numth/numfield/padic/gf2m), every code-tour, and every MATHEMATICS.md
chapter through ch. 11. The shard-time survey (2026-06-17, `@explore` fork against the full doc corpus)
established the grounding facts:

1. **The current `README.md` is rho-scoped, not a workspace umbrella (the load-bearing Z.1.1 fact).**
   It (206 lines) describes only the original single-crate Pollard-rho work (Phases 0–8): build/test,
   CLIs, benchmarks, the rho architectural decisions and optimization inventory. It names the other 8
   crates nowhere and carries no cross-track narrative. The one umbrella hook is a single
   forward-looking sentence (lines 21–25, "*intended to be a complete treatment…*"). **Z.1.1 rewrites
   it from a rho front-page into a 5-track umbrella** — this is a supersede-and-restructure, not an
   append.

2. **The master `docs/PEDAGOGY.md` aggregates only ρ + E today (the load-bearing Z.1.2 fact).** It
   (1166 lines) holds the rho code-tour (§1–7) and the E.W integrative Track-E chapter (§8–18) —
   **nothing else**. The four other code-tours live in their own files (`gnfs/docs/PEDAGOGY.md` G+D
   §1–71; `shared/numth/`; `shared/numfield/`; `shor/docs/PEDAGOGY.md`). **Z.1.2 turns this master
   file into the library-wide synthesis** that names the through-line once and threads all 5 tours,
   plus the cross-track L-notation comparison (per-track L-data exists; the *all-5-track* synthesis in
   one narrative does not).

3. **The modularity-theorem speculation chapter exists NOWHERE — it is the one authored (not
   aggregated) Z.1 deliverable.** The ROADMAP names it as a Z.1 deliverable (Phase ζ: "*the
   modularity-theorem speculation chapter*"); the only occurrence of "modularity" in the entire corpus
   is that single ROADMAP line. **Z.1.3 writes it from scratch** — its location (a master-PEDAGOGY.md
   section vs a new MATHEMATICS.md chapter) is the one genuine design choice, resolved in-session
   against the through-line and C-Textbook (see the S.D detail).

4. **All 11 MATHEMATICS.md chapters are present; the bind targets are the STALE cross-references (the
   load-bearing T.Z fact).** Ch. 11 (Shor + PQC, S.D.2) is complete. The bind work is *consistency*,
   not authoring: the **ToC (lines 143–160) still carries stale "*(T.G/T.D/T.E — to be appended)*"
   labels** for chapters that now exist; the **chapter-pairing table (lines 97–105) is stale** — it
   points the Track-E sibling at `gnfs/docs/PEDAGOGY.md (Track E chapter)` but the E.W integrative
   chapter actually lives in the **master `docs/PEDAGOGY.md` §8–18** (a structural mis-pairing T.Z must
   reconcile); the **comparative L-notation synthesis across all attacks** (ROADMAP T.Z deliverable)
   must be unified into the textbook; the **prerequisites chapter** must be checked against what the
   chapters actually leaned on; the **two S.D.2-flagged copy-edit nits** (a cosmetic ket typo in §11.2,
   a conversational aside in §11.4.1) are T.Z fixes. **T.Z authors little; it binds much.**

5. **C-Textbook is the only contract in play, consumed read-only.** The documentation register —
   audience (undergraduate maths background), depth (survey with proof-sketch depth; full proofs only
   at designated payoffs), through-line (structure-based escape from search), markup (Markdown +
   MathJax) — is frozen (T.0). Z.1's code-first umbrella follows the code-tour genre; T.Z's bind obeys
   the maths-first register. **T.Z is the chartered moment to ratify the `docs/textbook/` promotion
   question** (C-Textbook §location explicitly defers the single-file-vs-directory decision to T.Z) —
   a register *decision* T.Z is empowered to make, distinct from a register *break*.

The work splits at **three contract-sharp seams** (README↔master-tour; master-tour↔modularity; Z.1
umbrella↔T.Z bind), **4 sessions** across the two sub-tracks:

1. **Z.1.1 — README umbrella: workspace-level front page over all five tracks (Opus, Cat I).** Rewrite
   `README.md` from the rho-scoped front page into the project umbrella: the 5-track narrative (ρ/G/D/E/S),
   the crate/track map (all 9 crates), the escape-from-search framing, pointers to the master
   `docs/PEDAGOGY.md` and `docs/MATHEMATICS.md`, and the design-statement summary. **Freezes nothing.**

2. **Z.1.2 — master PEDAGOGY.md cross-track synthesis + the L-notation comparison (Opus, Cat I).**
   Transform `docs/PEDAGOGY.md` from the ρ+E-only file into the library-wide tour: name the through-line
   once, thread all 5 existing code-tours as chapters in one story (citing each per-crate tour, not
   duplicating it), and write the **comparative L-notation table across all 5 tracks** (ρ $L[1,1/2]$ →
   GNFS/NFS-DL $L[1/3,\cdot]$ → algebraic-ECDLP per §10.6 → Shor $L[0]$). **Freezes nothing.**

3. **Z.1.3 ◆ — the modularity-theorem speculation chapter; close Z.1 (Opus, Cat I).** Author the new
   modularity-theorem speculation chapter (the one aggregated-from-nothing Z.1 deliverable), location
   resolved in-session. Crosses the **Z.1 ◆** — the umbrella is complete and coherent before T.Z
   consumes its finished synthesis.

4. **T.Z ◆ — textbook bind: consistency pass over MATHEMATICS.md; close the project (Opus, Cat I).**
   The maths-first consistency-bind over all 11 chapters: resolve every cross-reference, reconcile the
   stale ToC + chapter-pairing table, unify the comparative L-notation synthesis into the textbook,
   reconcile the prerequisites chapter against actual usage, ratify the `docs/textbook/` promotion
   decision, fix the S.D.2-flagged nits. Crosses the **T.Z ◆ — the PROJECT TERMINUS** (the whole
   library bound, every chapter consistent, rGNFS complete).

Re-read this intent at each ◆ boundary to catch **defocus** (writing *any* code; adding a
crate/dependency/benchmark; re-deriving in the umbrella mathematics that belongs in the textbook
chapters; the README drifting back into rho-only scope) and **rigidity** (compressing the modularity
chapter to a footnote because "it's speculative" — the ROADMAP charters it as a chapter; T.Z merely
copy-editing instead of doing the *structural* bind — resolving the stale pairing table, the
cross-track L-notation synthesis, the promotion decision).

**Scoping discipline.** Z.1/T.Z synthesise and bind at **survey depth** (C-Textbook); the modularity
chapter is **explicitly speculative** (it is the project's one chartered speculation — *what if the
modularity theorem's structure suggests further escapes?* — flagged as speculation, not theorem). The
**principle-4 honesty is the umbrella's closing note**: the whole project demonstrates the *mathematics*
of each escape at toy scale, never the cryptographically-relevant scale — the umbrella states this once,
for the artifact as a whole, as the project's honest terminus.

---

## Purpose (design intent)

Per ROADMAP (Phase ζ, Z.1): "*Z.1 — Umbrella narrative. 2-4 sessions. Predecessor: everything. The
README + the master PEDAGOGY.md that ties the whole library together. The L-notation comparison across
attacks; the modularity-theorem speculation chapter; the structure-based-escape-from-search synthesis.
All sessions Opus-tier — this is the artifact-as-a-whole and the highest integrative judgment load in
the project.*" Per ROADMAP (Phase τ, T.Z): "*T.Z — Textbook bind. 1 session. Predecessor: all per-track
chapters + Z.1. Sibling to Z.1: the final consistency pass binding the accreted chapters into one
coherent, learnable document — cross-references resolved, the comparative L-notation synthesis across
all attacks written, notation unified, the prerequisites chapter reconciled against what the chapters
actually used. Opus-tier — highest integrative judgment load in the textbook, paired with Z.1's
umbrella synthesis.*"

Z.1 + T.Z are **the project terminus** — the closeout that turns the completed library (all 5 tracks,
9 crates, 5 code-tours, 11 textbook chapters) into the two artifacts-as-a-whole every reader meets
first: the umbrella README + master code-tour (Z.1, code-first) and the bound, learnable textbook (T.Z,
maths-first). Where each track demonstrated *one* structure-based escape from the search bound, Z.1/T.Z
are where the project *says what the whole arc means*: the comparative L-notation hierarchy across all
attacks (the quantitative spine), the structure-based-escape-from-search synthesis (the conceptual
spine), and the one chartered speculation (the modularity chapter). It is the natural terminus of the
project's through-line — after it, rGNFS is complete.

The deliverable is four-fold (the four conceptual units, each a session):

1. **The README umbrella (Z.1.1).** A rewritten `README.md` — the workspace-level front page over all
   5 tracks, superseding the rho-scoped original. **Freezes nothing.**

2. **The master PEDAGOGY.md cross-track synthesis (Z.1.2).** `docs/PEDAGOGY.md` transformed into the
   library-wide code-tour synthesising all 5 existing tours + the comparative L-notation table.
   **Freezes nothing.**

3. **The modularity-theorem speculation chapter (Z.1.3 ◆).** The one authored-new Z.1 deliverable;
   closes the umbrella. **Freezes nothing.**

4. **The textbook bind (T.Z ◆).** The maths-first consistency pass over all 11 MATHEMATICS.md chapters;
   closes the project. **Freezes nothing** (consumes C-Textbook read-only; consumes Z.1's finished
   synthesis for the cross-track L-notation material).

Z.1/T.Z are **prose-only** (no code, no crate, no dependency, no benchmark, no `*/src/` edit),
**library-synthesising** (they explain and bind the complete frozen corpus rather than extending it),
and **principle-4-honest** (the toy-scale ceiling across the whole project is the umbrella's honest
closing note). Re-read this intent at each ◆ to catch defocus (any code; the README narrowing to rho;
re-deriving textbook mathematics in the umbrella) and rigidity (a footnote-sized modularity chapter; a
copy-edit-only T.Z that skips the structural bind).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only
`[workspace]` members + a `[profile.bench]`); raw `cargo` is the only CI surface (unchanged since
S.A–S.D). **Z.1/T.Z write NO code** — they add one prose file at most (a possible new
modularity-chapter file, if that location is chosen) and edit prose files (`README.md`,
`docs/PEDAGOGY.md`, `docs/MATHEMATICS.md`) only. The VERIFY gate is therefore a **pure no-regression
gate** (every existing crate's KATs stay green because Z.1/T.Z change no source, no manifest, no module
surface), with **no new test** to add and **no new compile** target. `/run-plan` re-discovers at
preflight. The honest gate for a prose closeout is **the no-regression VERIFY (trivially green) + the
C-Textbook register-conformance and cross-reference-integrity review at each ◆** — the weak-lever-5
reality made explicit:

- **The entire existing workspace KAT suite must stay green** — Z.1/T.Z touch no `.rs`, no
  `Cargo.toml`, no module. `cargo test --workspace` is a pure regression guard; it adds nothing and
  must change nothing. *(If a session run reports any test delta, that is a red flag that Z.1/T.Z have
  drifted into code — a defocus signal to HALT, not to accommodate.)*
- **`cargo check --workspace` must stay green** — trivially, for the same reason.
- **No `cargo bench` involvement** — Z.1/T.Z add no benchmark (`docs/BENCHMARKS.md` is unchanged; its
  Phase-1–4 / G.B–G.W / E.W / S.A–S.C sections are cited, not extended). `VERIFY_BENCH` is N/A.
- **The real correctness gate is review, not tests (lever-5-weak made explicit).** Prose correctness =
  cross-reference integrity (every citation across README ↔ 5 PEDAGOGY.md ↔ 11-chapter MATHEMATICS.md
  resolves) + C-Textbook register-conformance + mathematical accuracy of the new modularity chapter +
  the bind's structural completeness (stale ToC/pairing-table reconciled, L-notation synthesis unified).
  This is **not** machine-checkable; it is the human review + the ◆ juncture's job. The MathJax renders
  (no build-time validation — a known C-Textbook tradeoff); a renderer spot-check is the substitute.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| Z.1.1 | README umbrella: workspace-level front page over all five tracks | I | Opus | C-Textbook (frozen T.0 — the register; the README follows the code-first umbrella genre); the complete library as the documented subject (all 9 crates, all 5 tracks); the existing `README.md` (superseded); the 5 code-tours + `docs/MATHEMATICS.md` (cited as the deep-dive pointers) | `README.md` (rewrite: rho-scoped → 5-track umbrella — narrative over ρ/G/D/E/S, the 9-crate/5-track map, escape-from-search framing, pointers to `docs/PEDAGOGY.md` + `docs/MATHEMATICS.md`, design-statement summary) |
| Z.1.2 | Master PEDAGOGY.md cross-track synthesis + L-notation comparison | I | Opus | C-Textbook (frozen T.0); the 5 existing code-tours (`docs/PEDAGOGY.md` ρ+E §1–18; `gnfs/docs/PEDAGOGY.md` G+D §1–71; `shared/numth/`; `shared/numfield/`; `shor/docs/PEDAGOGY.md`) as the synthesised subject; `MATHEMATICS.md` "Escape from Search" §3 + §10.6 L-notation tables (cited) | `docs/PEDAGOGY.md` (extend: ρ+E-only master file → library-wide synthesis — through-line named once, all 5 tours threaded as chapters in one story, the comparative all-5-track L-notation table; cites each per-crate tour, does not duplicate) |
| Z.1.3 ◆ | Modularity-theorem speculation chapter; close Z.1 | I | Opus | C-Textbook (frozen T.0 — incl. the speculation register, flagged as speculation not theorem); the through-line synthesis (Z.1.2, the home it slots into); `MATHEMATICS.md` §3 (the escape-from-search taxonomy it speculates beyond) | `docs/PEDAGOGY.md` (new section) OR `docs/MATHEMATICS.md` (new chapter) — location resolved in-session against the through-line + C-Textbook; the genuinely new authored content |
| T.Z ◆ | Textbook bind: consistency pass over MATHEMATICS.md; close the project | I | Opus | C-Textbook (frozen T.0 — the register + the chartered `docs/textbook/` promotion decision); Z.1's finished umbrella synthesis (the cross-track L-notation comparison T.Z unifies into the textbook); all 11 existing MATHEMATICS.md chapters (the bound subject) | `docs/MATHEMATICS.md` (edit: resolve cross-references; reconcile stale ToC "to be appended" labels lines 143–160 + the stale chapter-pairing table lines 97–105 — the Track-E sibling mis-points at `gnfs/` but lives in `docs/PEDAGOGY.md` §8–18; unify the comparative L-notation synthesis; reconcile the prerequisites chapter; ratify the `docs/textbook/` promotion; fix the §11.2 ket typo + §11.4.1 aside) |

**Sequencing notes.** Strictly serial: **Z.1.1 → Z.1.2 → Z.1.3 ◆ → T.Z ◆.** The README (Z.1.1) is the
front page that points at the master tour; the master-tour synthesis (Z.1.2) establishes the
through-line home the modularity chapter (Z.1.3) slots into and closes Z.1; T.Z then **consumes Z.1's
finished synthesis** (it unifies the cross-track L-notation comparison Z.1.2 wrote into the textbook —
the real cross-sub-track contract seam) and binds the project closed. *(Order rationale: README first
so the deeper artifacts have a stable front page to be pointed at; master-tour before the modularity
chapter so the chapter slots into a settled through-line; T.Z last because the bind's L-notation
synthesis literally consumes Z.1.2's table — writing T.Z before Z.1 would force it to invent the
cross-track comparison Z.1 owns. Tradeoff named: Z.1.1's README cites a `docs/PEDAGOGY.md` master
synthesis that Z.1.2 has not yet written — a forward reference, accepted because the README's pointer
is a one-liner and the alternative, README-after-synthesis, would leave the project's front page stale
longest.)*

**Two `@architect` markers:** the **Z.1.3 ◆** (the umbrella close — ratifying README + master-tour +
modularity chapter as a coherent whole before T.Z binds against it) and the **T.Z ◆** (the
**project-final juncture** — the whole textbook bound, every chapter consistent, rGNFS complete).
*(Tradeoff named: two ◆ halts cost two human glances rather than one continuous closeout; accepted
because weak-lever-5 means no test catches a wrong umbrella synthesis, and T.Z binds *against* that
synthesis — an intervening Z.1 ◆ checkpoint stops a wrong synthesis from propagating silently into the
final bind. The juncture-tier is `opus`, the opt-down UNAVAILABLE per lever-5-weak, recorded in the
header. Neither sub-track pages a juncture at its open: there is no substrate-design judgment — the
umbrella and bind genres are settled (the `*.W` precedent + C-Textbook), and the integrative judgment
concentrates at each ◆ close.)*

**Why 4 sessions (the ROADMAP "Z.1: 2–4" + "T.Z: 1").** The split is taken at three contract-sharp
seams; lever 1 (HIGH ambient) and lever 5 (weak loop) both push toward the fine-grained end of the
ROADMAP band:
- **One-line-commit-title corollary.** "README umbrella", "master PEDAGOGY.md cross-track synthesis",
  "modularity-theorem speculation chapter", and "textbook bind: consistency pass over MATHEMATICS.md"
  are **four distinct commit titles**. Bundling any pair — e.g. "rewrite the README AND synthesise the
  master tour" — fails the corollary.
- **Four conceptual units kept whole (lever 2).** Each session is one unit: a front page, a code-first
  synthesis, an authored speculation chapter, a maths-first bind. None fractures cleanly.
- **Contract-sharp boundaries.** README↔master-tour (front page vs deep tour — different artifacts,
  different files); master-tour↔modularity (aggregation vs authored-new — different work register);
  Z.1-umbrella↔T.Z-bind (code-first vs maths-first — different register, different file, and T.Z
  *consumes* Z.1's L-notation synthesis: a genuine cross-sub-track seam).
- **HIGH ambient + weak loop drive the fine grain; lever-3-moderate keeps it from over-splitting.** The
  whole-library cross-reference surface (lever 1) and the absent test loop (lever 5) argue for small
  sessions, but cost-of-error is only moderate (lever 3), so 4 sessions — not a maximal split that
  would carve sub-band commits with no contract seam (e.g. the L-notation table is a Z.1.2 subsection,
  not its own session; the README sections are one unit).

**The softest seam — could Z.1.2 split the L-notation comparison into its own session?** Considered and
declined at shard time. The comparative all-5-track L-notation table is a *subsection* of the master-tour
synthesis (a few hundred words + a table situating each track's complexity), **not a full session's
worth**, and there is **no contract seam** between "thread the tours" and "tabulate their complexities"
— both are the same cross-track-aggregation register in one file. Splitting it out would produce a
sub-band commit with no freeze worth taking. **If Z.1.2 overruns** (the 5-tour synthesis + the L-notation
comparison together push past the band — plausible given the breadth), the escape applies: **split at the
synthesis↔comparison seam** (Z.1.2a the tour synthesis, Z.1.2b the L-notation comparison) — an
additive-reshard surfaced at the Z.1.2 readout or by Z.1.2 once the synthesis length is concrete, never
a silent overrun. This is the one place the sizing is genuinely uncertain until the master-tour synthesis's
true length is visible.

**The modularity chapter location — the one genuine in-session design choice.** Z.1.3's content is new;
its *home* is a design decision deferred to the session (it has the full corpus in view): a section in
the master `docs/PEDAGOGY.md` (code-first, speculative-but-grounded-in-the-library) vs a new chapter in
`docs/MATHEMATICS.md` (maths-first, a textbook chapter). The PLAN does not pre-decide it because the
right call depends on how the chapter reads once drafted; the Z.1.3 detail names the decision criteria,
and the Z.1.3 ◆ confirms the choice.

---

## Session detail

Z.1.1 and Z.1.2 are specified at near-full fidelity (the umbrella + master-tour genres are settled — the
`*.W` integrative chapters across the project and the existing `docs/PEDAGOGY.md` §8–18 / §1–7 structure
are the templates; the subject — the complete frozen library — is fully surveyed). Z.1.3 (the new
modularity chapter) and T.Z (the bind) are specified at the structural level — correct per the
substrate-first discipline: Z.1.3's exact home and T.Z's exact cross-reference targets are crisp only
after Z.1.2's master-tour §-structure freezes.

### Z.1.1 — README umbrella: workspace-level front page over all five tracks (Opus, Cat I)

**Deliverable:** a rewritten `README.md` — the workspace-level umbrella superseding the rho-scoped front
page. The pieces:
- **The umbrella opening** — replace the rho-Phases-0–8 opening with the project's actual scope: a
  *survey of discrete-logarithm algorithms and integer factorisation, classical and quantum*, organised
  by the structure-based-escape-from-search through-line. Promote the lines-21–25 hook into the lead.
- **The 5-track / 9-crate map** — ρ (Pollard rho + the algebraic-ECDLP attacks, `rho`), G (GNFS,
  `gnfs`), D (NFS-DL, `gnfs`), E (algebraic ECDLP, `rho` + `shared/padic` + `shared/gf2m`), S (Shor,
  `shor`), plus the shared substrate (`shared/field`/`bigint`/`numth`/`numfield`/`padic`/`gf2m`). Each
  track gets a one-paragraph "what structure it exploits" gloss; the crate-structure section (currently
  rho-only) is rebuilt for the workspace.
- **The two-artifact pointer** — the master `docs/PEDAGOGY.md` (code-first tour) and
  `docs/MATHEMATICS.md` (maths-first textbook) as the two complementary deep-dives, plus `docs/BENCHMARKS.md`.
- **The preserved rho operational content** — build/test, CLIs, benchmarks: keep what is still accurate,
  generalise it to the workspace (the build/test commands are workspace-wide), and demote the
  rho-specific optimization inventory to its rho subsection.
- **The design-statement + principle-4 honesty summary** — one paragraph: the project demonstrates the
  *mathematics* of each escape at toy scale, never cryptographic scale (the honest front-page terminus).

Consumes C-Textbook (frozen T.0) and the complete library as the documented subject. Reads the existing
`README.md` (to preserve the accurate operational content) and the 5 code-tours + MATHEMATICS.md (to
write accurate one-line pointers). **Freezes nothing.**

**Quality gate (prose analogue — no machine check):** (1) **scope-fidelity** — the README describes the
*workspace*, not just `rho` (the central Z.1.1 risk is leaving it rho-scoped); (2) **factual fidelity** —
every crate, track, CLI, and pointer named matches the actual workspace (read `Cargo.toml` + the dirs,
do not recall); (3) **cross-reference integrity** — the pointers to `docs/PEDAGOGY.md` /
`docs/MATHEMATICS.md` / `docs/BENCHMARKS.md` are well-formed (the master-PEDAGOGY pointer is a forward
reference Z.1.2 satisfies). **Verify gate:** `cargo test --workspace` green (pure regression — Z.1.1
changes no code); `cargo check --workspace` green.

**Subtlety (load-bearing):** (1) **It is a supersede, not an append** — the README must stop being the
rho front page; leaving the rho framing as the lead is the central defocus. (2) **Preserve the accurate
operational content** — the build/test/CLI/benchmark sections are still useful; generalise, do not
delete. (3) **No code, no `Cargo.toml` edit** — the crate map is *described*, not changed.

**Deferred:** the master-tour synthesis (Z.1.2); the modularity chapter (Z.1.3); the textbook bind (T.Z).

### Z.1.2 — master PEDAGOGY.md cross-track synthesis + L-notation comparison (Opus, Cat I)

**Deliverable:** `docs/PEDAGOGY.md` transformed from the ρ+E-only file into the library-wide synthesis.
Structural-near-full fidelity (the genre is the existing §8–18 integrative chapter; the subject is the 5
frozen tours). The pieces:
- **The umbrella through-line** — name the structure-based-escape-from-search spine *once* at the master
  level (citing `MATHEMATICS.md` §3 for the full taxonomy), then frame the whole tour as one story.
- **The 5-tour synthesis** — thread each existing code-tour as a chapter in the umbrella: ρ (`docs/PEDAGOGY.md`
  §1–7, already here), the α-substrate (`shared/numth/`), the number-field substrate
  (`shared/numfield/`), G+D (`gnfs/docs/PEDAGOGY.md` §1–71), E (the §8–18 chapter already here), S
  (`shor/docs/PEDAGOGY.md`). **Cite each per-crate tour for the detail; do not duplicate it** — the
  master file is the *synthesis*, the per-crate files remain the depth.
- **The comparative all-5-track L-notation table** — ρ $L[1,1/2]$ → GNFS/NFS-DL $L_N[1/3,(64/9)^{1/3}]$
  → algebraic-ECDLP attacks (per `MATHEMATICS.md` §10.6) → Shor $L[0]$ (polynomial). The per-track data
  exists across MATHEMATICS.md §7/§9.7/§10.6 and the through-line §3 hierarchy table; this is the
  *one-narrative cross-track* version, the quantitative spine of the umbrella.
- **The cross-track connections** — the MOV bridge (E→D), the shared substrate reuse (the
  smoothness/field/number-field substrate threading G/D/E), the classical→quantum arc (ρ/G/D/E→S).

Consumes C-Textbook (frozen T.0) and the 5 tours as the synthesised subject. Reads all 5 PEDAGOGY.md
files (for accurate citation) and `MATHEMATICS.md` §3/§7/§9.7/§10.6 (for the L-notation facts). **Freezes
nothing.**

**Quality gate (prose analogue):** (1) **synthesis-not-duplication** — the master file *cites* the
per-crate tours, it does not re-tour them (the central Z.1.2 risk is bloating into a copy); (2)
**cross-reference integrity** — every citation into the 5 tours + MATHEMATICS.md resolves; (3)
**L-notation accuracy** — the comparative table matches the per-track derivations (read §7/§9.7/§10.6,
do not recall the constants); (4) **register-conformance** — code-first umbrella genre, C-Textbook.
**Verify gate:** `cargo test --workspace` + `cargo check --workspace` green (pure regression); MathJax
renders (spot-check the L-notation table).

**Subtlety (load-bearing):** (1) **Synthesis, not re-tour** — the master file is small-and-connective,
not a 5× concatenation; the per-crate tours stay the depth. (2) **The current §1–18 content stays** —
the ρ tour (§1–7) and the E.W chapter (§8–18) are *part of* the synthesis, re-framed under the umbrella
through-line, not deleted. (3) **The soft synthesis↔comparison seam** — if the session overruns, split
at the L-notation-table seam (the named additive-reshard), surfaced at the readout. (4) **No code, no
BENCHMARKS edit** — the performance facts are cited from `docs/BENCHMARKS.md`, not extended.

**Deferred:** the modularity chapter (Z.1.3); the textbook bind (T.Z); the L-notation-table split (the
soft seam, only if overrun).

### Z.1.3 ◆ — modularity-theorem speculation chapter; close Z.1 (Opus, Cat I)

**Deliverable:** the new modularity-theorem speculation chapter — the one Z.1 deliverable authored from
scratch (it exists nowhere in the corpus). Structural-fidelity sketch (the content is genuinely new; the
detail crisps in-session). The pieces:
- **The chapter framing** — modularity (the Taniyama–Shimura–Weil correspondence between elliptic curves
  and modular forms) as a *structural* phenomenon in the same escape-from-search family the project
  surveys: a deep correspondence that *could* suggest further structure to exploit. **Explicitly flagged
  as speculation** (the project's one chartered speculation), not a theorem the project implements.
- **The connection to the through-line** — how the modularity structure relates to the five escape
  families (it is a structure-revealing correspondence; the speculation is whether such correspondences
  point at attacks not yet realised). Survey depth; the honest "this is speculative" register.
- **The location decision (the one in-session design choice)** — a section in the master
  `docs/PEDAGOGY.md` (code-first umbrella, where it reads as "a further direction the library gestures
  at") vs a new `docs/MATHEMATICS.md` chapter (maths-first, where it reads as a textbook speculation
  chapter). **Decision criteria:** if the chapter is mathematically substantial enough to warrant
  textbook treatment (display math, theorem statements), it belongs in MATHEMATICS.md as a new chapter
  (and T.Z binds it); if it is a connective speculation gesturing at directions, it belongs in the
  master PEDAGOGY.md umbrella. Resolve at draft time; record the choice at the ◆.
- **The Z.1 ◆ close** — re-read the Purpose intent; verify the README (Z.1.1) + master-tour (Z.1.2) +
  modularity chapter (Z.1.3) are a coherent umbrella (cross-references resolve, the through-line is
  named once and threaded consistently); verify the umbrella stayed prose-only; confirm the L-notation
  synthesis is complete (T.Z will consume it).

Consumes C-Textbook (frozen T.0 — incl. the speculation register) and the Z.1.2 through-line synthesis
(the home it slots into). **Freezes nothing.**

**Quality gate (prose analogue):** (1) **chartered-as-a-chapter, not a footnote** — the modularity
chapter is a real section/chapter (the rigidity guard: the ROADMAP charters it; do not compress it to a
footnote because it is speculative); (2) **flagged-as-speculation** — it is explicitly the project's one
speculation, not a claimed result (the honesty guard); (3) **location-coherence** — the home chosen
reads naturally for the register (code-first vs maths-first); (4) **cross-reference integrity** — it
slots into the Z.1.2 through-line. **Verify gate:** `cargo test`/`cargo check --workspace` green (pure
regression); MathJax renders (if it carries display math).

**Subtlety (load-bearing):** (1) **It is chartered as a chapter, not a footnote** — the central rigidity
risk; the ROADMAP names it a Z.1 deliverable. (2) **Speculation, explicitly flagged** — the central
honesty risk; it must not read as a claimed attack. (3) **The location is an in-session design choice** —
resolved against the decision criteria, recorded at the ◆. (4) **No code** — it surveys/speculates, it
implements nothing.

**Deferred:** the textbook bind (T.Z); the ROADMAP static-frame reconciliation (a capture candidate, out
of `@architect` PLAN scope).

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
Z.1.3 ◆ to confirm: (1) **the README is a workspace umbrella, not rho-scoped** — it describes all 5
tracks / 9 crates accurately; (2) **the master PEDAGOGY.md synthesises, not duplicates** — it cites the
5 per-crate tours and threads them under one through-line; (3) **the comparative L-notation table is
accurate and complete** across all 5 tracks (T.Z will consume it); (4) **the modularity chapter is
chartered-as-a-chapter and flagged-as-speculation** — not a footnote, not a claimed result, with its
location-choice recorded; (5) **the umbrella is internally coherent** — README ↔ master-tour ↔
modularity cross-references resolve; (6) **Z.1 stayed prose-only** — no code, no crate, no dep, no
bench, no `*/src/` edit. **Also: surface the compounded static-frame ROADMAP debt** (Phase ε still shows
"0 done" despite Track S complete — now the SIXTH consecutive carry, and Phase ζ now opening) **as a
capture candidate, not a PLAN edit.** One-shot findings; does not implement. Held at **opus** per the
header (lever-5-weak forbids the opt-down — UNAVAILABLE).

### T.Z ◆ — textbook bind: consistency pass over MATHEMATICS.md; close the project (Opus, Cat I)

**Deliverable:** `docs/MATHEMATICS.md` bound into one coherent, learnable document — the maths-first
sibling of Z.1's code-first umbrella, and the **project terminus**. Structural-fidelity sketch (the exact
edits crisp once Z.1's synthesis freezes). The pieces:
- **Cross-reference resolution** — every chapter's "code realisation" pointer resolves into the correct
  PEDAGOGY.md (incl. the corrected master-tour structure Z.1.2 froze); every textbook-internal
  cross-reference (ch. N → ch. M) resolves.
- **The stale-ToC reconciliation** — the ToC (lines 143–160) still carries "*(T.G/T.D/T.E — to be
  appended)*" for chapters that now exist; drop the stale labels, confirm the chapter numbers.
- **The stale chapter-pairing-table reconciliation** — the pairing table (lines 97–105) points the
  Track-E sibling at `gnfs/docs/PEDAGOGY.md (Track E chapter)`, but the E.W integrative chapter actually
  lives in the master `docs/PEDAGOGY.md` §8–18 (a structural mis-pairing); fix the row. Confirm the other
  rows against the frozen master-tour structure.
- **The comparative L-notation synthesis (the ROADMAP T.Z deliverable)** — unify the cross-track
  L-notation comparison into the textbook (consuming Z.1.2's table), as the textbook's quantitative
  spine across all attacks; reconcile it against §3's hierarchy table, §7, §9.7, §10.6.
- **The prerequisites-chapter reconciliation** — check the prerequisites chapter (lines 252–539) against
  what the chapters *actually* leaned on (the ROADMAP T.Z deliverable: "reconciled against what the
  chapters actually used"); add/trim as the accreted chapters demand.
- **Notation unification** — one notation across all 11 chapters (the bind's core consistency job).
- **The `docs/textbook/` promotion decision (chartered to T.Z)** — ratify whether the single-file
  MATHEMATICS.md is promoted to a `docs/textbook/` directory (C-Textbook §location defers this to T.Z);
  record the decision in C-Textbook.
- **The S.D.2-flagged nits** — fix the §11.2 ket typo and the §11.4.1 conversational aside.
- **The T.Z ◆ — the PROJECT TERMINUS** — re-read the Purpose intent; verify the whole textbook is
  bound (cross-references resolve, ToC/pairing-table reconciled, L-notation synthesis unified, notation
  uniform, prerequisites reconciled, promotion decided); verify T.Z stayed prose-only; **confirm rGNFS
  is complete** (all 5 tracks documented + contextualised + bound, both the code-first umbrella and the
  maths-first textbook coherent).

Consumes C-Textbook (frozen T.0 — the register + the chartered promotion decision) and Z.1's finished
umbrella synthesis (the L-notation comparison it unifies). **Freezes nothing** (it may *amend* C-Textbook's
location clause via the chartered promotion decision — a decision T.Z is empowered to make, recorded as a
register decision, not a register break).

**Quality gate (prose analogue):** (1) **structural bind, not copy-edit** — the central T.Z rigidity
risk is doing only typo-fixing and skipping the stale-pairing-table / L-notation-synthesis /
prerequisites-reconciliation / promotion-decision structural work; (2) **cross-reference integrity** —
every citation across all 11 chapters + into the PEDAGOGY.md tours resolves; (3) **L-notation synthesis
unified** — the cross-track comparison is in the textbook, consistent with §3/§7/§9.7/§10.6; (4)
**notation uniform** — one notation across all chapters; (5) **register-conformance** — C-Textbook
obeyed, the promotion decision recorded. **Verify gate:** `cargo test`/`cargo check --workspace` green
(pure regression); MathJax renders (spot-check edited display blocks).

**Subtlety (load-bearing):** (1) **The bind is STRUCTURAL, not copy-edit** — the central rigidity risk;
T.Z must resolve the stale pairing table, write the cross-track L-notation synthesis, reconcile
prerequisites, and decide the promotion — not merely fix typos. (2) **The pairing-table Track-E row is
structurally wrong** — it points at `gnfs/` but the chapter is in `docs/PEDAGOGY.md` §8–18; the fix is a
T.Z structural reconciliation. (3) **The promotion decision is chartered to T.Z** — it is empowered to
amend C-Textbook's location clause (a decision, not a break). (4) **T.Z consumes Z.1's L-notation table** —
the cross-sub-track seam; T.Z must not re-derive what Z.1.2 owns. (5) **No code** — `MATHEMATICS.md`
(+ possibly the modularity chapter if Z.1.3 placed it here) only.

**Deferred:** nothing downstream — **T.Z is the project terminus.** The ROADMAP static-frame
reconciliation (the compounded Progress/Remaining debt) is a capture candidate, out of `@architect` PLAN
scope, surfaced at the ◆.

**`@architect` ◆ confirmation (post-landing, Opus, one-shot — the PROJECT-FINAL juncture).** Page a
`@plan-juncture` fork at the T.Z ◆ to confirm: (1) **the bind is structural, not copy-edit** — the stale
ToC + pairing table reconciled, the cross-track L-notation synthesis written, the prerequisites chapter
reconciled, notation unified, the promotion decision made and recorded; (2) **every cross-reference
resolves** across all 11 chapters and into the PEDAGOGY.md tours (incl. the corrected Track-E pairing);
(3) **the L-notation synthesis is unified and consistent** with §3/§7/§9.7/§10.6 and with Z.1.2's table;
(4) **C-Textbook register-conformance** holds across the whole textbook, and the `docs/textbook/`
promotion decision is recorded; (5) **the two siblings are coherent** — Z.1's code-first umbrella ↔ the
maths-first textbook, the project's two artifacts-as-a-whole; (6) **rGNFS is complete end-to-end** — all
5 tracks documented (code-tours) + contextualised + bound (textbook), every escape-from-search structure
in one coherent survey; (7) T.Z stayed prose-only — **no code**. **Also: surface the compounded
static-frame ROADMAP debt** (now owed at the project's close — the Progress/Remaining tables need the
full reconciliation marking every phase complete) **as the project-final capture candidate, not a PLAN
edit.** One-shot findings; does not implement. Held at **opus** per the header (lever-5-weak forbids the
opt-down — UNAVAILABLE). **This is the last juncture of the project.**

---

## Cross-session contracts

Z.1 and T.Z **freeze no code contract** — they are prose-only closeout sub-tracks. They **amend no prior
frozen code contract** (they document and bind the complete frozen library read-only) and **consume
exactly one contract: C-Textbook** (the documentation register, frozen T.0). They add no workspace
member, no dependency, no benchmark, and no `.rs`. **There is no downstream consumer** — this is the
terminus; nothing follows. The one intra-PLAN producer→consumer edge is **prose, not code: T.Z consumes
Z.1.2's comparative L-notation synthesis** (T.Z unifies it into the textbook). This is the lever-1-high
(whole-library cross-reference surface) + lever-5-weak (no test loop) that drove the fine-grained
4-session split; the **absent test inner-loop made the juncture-tier opt-down UNAVAILABLE** (the same
firm posture as S.D).

### C-Textbook — the documentation-register contract (prose-enforced) — *frozen T.0; consumed read-only by Z.1; the chartered `docs/textbook/` promotion decided by T.Z*

**Defined in:** T.0 (`5c9b783`; the contract is documented in `MATHEMATICS.md` §1 "C-Textbook" lines
8–111, and the ROADMAP §Phase τ scope contract). **Consumed by:** every textbook chapter and every
`*.W` code-tour — here, Z.1.1/Z.1.2/Z.1.3 (the umbrella follows the code-first genre + the chartered
speculation register) and T.Z (the bind obeys the maths-first register). Prose-enforced (no
compiler/test gate — the C-Textbook tradeoff, frozen at T.0: MathJax fails silently in a renderer, there
is no build-time math validation).

**Ratified shape (consumed as-is by Z.1; one chartered amendment available to T.Z).** Audience: the
interested mathematics student with a full undergraduate maths background. Depth: **survey with
proof-sketch depth** — full proofs only at designated payoffs. Through-line: **structure-based escape
from search** (the umbrella synthesis names it once; the modularity chapter speculates beyond it). Markup:
**Markdown + MathJax**. Location: `docs/MATHEMATICS.md` single-file — **the promotion to `docs/textbook/`
is explicitly chartered to T.Z** (C-Textbook §location: "*decide at T.Z*"). **Invariant for Z.1:** Z.1
obeys the register; it does not flex it. **T.Z's chartered carve-out:** T.Z may *decide* the
single-file-vs-directory promotion and record it — a register *decision* the contract explicitly defers
to T.Z, distinct from a register *break* (a break still surfaces at the ◆, not silently).

### Frozen library surface read by Z.1/T.Z (synthesised/bound, not amended)

Z.1/T.Z synthesise and bind — and amend none of — the complete frozen library:
- **The 5 tracks** — ρ (`rho`: Pollard rho + the Track-E algebraic attacks), G (`gnfs`: GNFS), D
  (`gnfs`: NFS-DL), E (`rho` + `shared/padic` + `shared/gf2m`: algebraic ECDLP), S (`shor`: Shor). Z.1
  synthesises them; T.Z binds their textbook chapters.
- **The 5 code-tours** — `docs/PEDAGOGY.md` (ρ §1–7 + E.W §8–18), `gnfs/docs/PEDAGOGY.md` (G+D §1–71),
  `shared/numth/docs/PEDAGOGY.md`, `shared/numfield/docs/PEDAGOGY.md`, `shor/docs/PEDAGOGY.md`. Z.1.2
  threads them under the umbrella; `docs/PEDAGOGY.md` is the one Z.1 *edits* (the master synthesis).
- **The 11 MATHEMATICS.md chapters** — C-Textbook §1, ToC §2, through-line §3, prerequisites §4, On
  Scale §5, ρ §6, α-substrate §7, GNFS (§§1–8 GNFS block), NFS-DL (§§9.1–9.8), ch. 10 algebraic ECDLP,
  ch. 11 Shor + PQC. T.Z binds them into one coherent document.
- **`docs/BENCHMARKS.md`** (Phase 1–4 / G.B–G.W / E.W / S.A–S.C) — cited by the umbrella, **not edited**.

### Downstream contracts Z.1/T.Z do NOT produce (named, to bound scope)

- **No code contract.** Z.1/T.Z are prose; they freeze nothing for any consumer.
- **No downstream consumer at all.** This is the project terminus — nothing follows Z.1/T.Z. (The one
  intra-PLAN edge is prose: T.Z consumes Z.1.2's L-notation synthesis.)
- **No new content beyond the chartered deliverables.** Z.1 authors exactly one new body (the
  modularity chapter); everything else is synthesis/bind of existing artifacts. A `@build` agent that
  writes code, adds a crate/dep/bench, or implements anything has reached past scope.

### Workspace edges (no new member, no new dependency, no code)

- **No new member, no new dependency (library or dev), no `.rs`, no benchmark.** Z.1/T.Z edit prose
  files (`README.md`, `docs/PEDAGOGY.md`, `docs/MATHEMATICS.md`) and may add at most one new prose file
  (the modularity chapter, if Z.1.3 places it standalone). The workspace `Cargo.toml`, every crate's
  `Cargo.toml`, and all source are unchanged; `cargo check`/`cargo test` are pure regression guards
  (trivially green).

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked (Z.1/T.Z freeze none — the column reads "—"). The Z.1.3 ◆ and T.Z ◆
`@architect` confirmations are not separate ledger rows (paged forks with no commit-shaped deliverable);
their outcomes are recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| Z.1.1 | README umbrella: workspace-level front page over all five tracks | done | `a78d36d` | — (prose) |
| Z.1.2 | Master PEDAGOGY.md cross-track synthesis + L-notation comparison | done | `cfc2575` | — (prose) |
| Z.1.3 ◆ | Modularity-theorem speculation chapter; close Z.1 | pending | | — (prose) |
| T.Z ◆ | Textbook bind: consistency pass over MATHEMATICS.md; close the project | pending | | — (prose) |

Contracts frozen before these sub-tracks: the **entire library** — all of Tracks ρ (Pollard rho + the
Track-E algebraic attacks), G (GNFS, complete), D (NFS-DL, complete), E (algebraic ECDLP, closed at the
E.W ◆), and S (Shor, closed at the S.D ◆) — plus the shared substrate (the field/bigint/numth/numfield/
padic/gf2m crates), the Track-τ register (C-Textbook, frozen T.0, `5c9b783`), and all 11 MATHEMATICS.md
chapters + 5 code-tours. **Z.1/T.Z consume C-Textbook read-only and synthesise/bind the frozen library;
they freeze no new contract and amend none (save T.Z's chartered `docs/textbook/` promotion decision).**
**With the T.Z ◆, rGNFS is complete end-to-end — every structure-based escape from the search bound
surveyed, toured, contextualised, and bound into the project's two artifacts-as-a-whole (the code-first
umbrella + the maths-first textbook).**

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **The README is rho-scoped, not a workspace umbrella — the load-bearing Z.1.1 fact (shard-time
  survey, 2026-06-17).** The current `README.md` (206 lines) describes only the original Pollard-rho
  crate; it names the other 8 crates nowhere and carries no cross-track narrative. **Internal-continue →
  Z.1.1 rewrites it into a 5-track umbrella (supersede, not append);** leaving it rho-scoped is the
  central Z.1.1 defocus.

- **The master `docs/PEDAGOGY.md` aggregates only ρ + E — the load-bearing Z.1.2 fact.** It holds the
  rho tour (§1–7) and the E.W chapter (§8–18); the 4 other tours live in their own files. **Internal-
  continue → Z.1.2 synthesises (cites the per-crate tours, threads them under one through-line, writes
  the cross-track L-notation table); it does NOT duplicate the per-crate tours** (bloating into a 5×
  concatenation is the central Z.1.2 risk). A discovery that the synthesis + L-notation table overrun is
  an **additive Z.1.2a/Z.1.2b split** (the named soft seam) surfaced at the readout, never a silent
  overrun.

- **Z.1/T.Z are PROSE-ONLY — the central defocus risk is writing code.** A `@build` agent that writes
  any `.rs`, adds a crate/dependency/benchmark, or touches any `*/src/` has broken scope. **Internal-
  continue → Z.1/T.Z edit `README.md` + `docs/PEDAGOGY.md` + `docs/MATHEMATICS.md` (and at most add one
  new prose file for the modularity chapter) and nothing else;** any code change is a HALT-and-surface
  defocus signal, not an accommodation.

- **The modularity-theorem speculation chapter exists NOWHERE — it is authored from scratch, and is
  chartered as a chapter (the central Z.1.3 rigidity risk).** The ROADMAP names it a Z.1 deliverable;
  the only "modularity" mention in the corpus is that ROADMAP line. **Internal-continue → Z.1.3 writes
  it as a real chapter/section, flagged explicitly as speculation;** compressing it to a footnote
  because it is speculative is rigidity. Its location (master PEDAGOGY.md section vs new MATHEMATICS.md
  chapter) is an in-session design choice, recorded at the ◆.

- **All 11 MATHEMATICS.md chapters are present; the T.Z bind is STRUCTURAL consistency, not authoring
  (the central T.Z rigidity risk).** Ch. 11 is complete. The bind work is: resolve cross-references;
  reconcile the stale ToC "to be appended" labels (lines 143–160); fix the **structurally-wrong
  chapter-pairing-table Track-E row** (lines 97–105 — points at `gnfs/`, but the E.W chapter is in
  `docs/PEDAGOGY.md` §8–18); unify the cross-track L-notation synthesis; reconcile the prerequisites
  chapter against actual usage; ratify the `docs/textbook/` promotion; fix the §11.2/§11.4.1 nits.
  **Internal-continue → T.Z does the STRUCTURAL bind, not a copy-edit-only pass;** skipping the
  structural work is the central T.Z rigidity.

- **T.Z consumes Z.1.2's L-notation synthesis — the cross-sub-track seam.** T.Z unifies the comparative
  L-notation comparison Z.1.2 writes into the textbook. **Internal-continue → if Z.1.2's synthesis is
  wrong, the Z.1.3 ◆ catches it before T.Z inherits it;** this is why the Z.1 ◆ is load-bearing (weak-
  lever-5: no test catches a wrong synthesis).

- **There is no test inner-loop for prose (lever-5-weak) — the ◆ + review IS the inner loop.** The
  VERIFY gate is trivially green throughout (Z.1/T.Z touch no code), so no behavioural signal catches
  drift. **Internal-continue → the quality gate is human review + the two opus ◆ junctures
  (cross-reference integrity, register-conformance, synthesis-not-duplication, structural-bind-not-copy-
  edit); the juncture-tier opt-down is UNAVAILABLE.** A test delta in any session run is a defocus red
  flag (Z.1/T.Z drifted into code).

- **The `docs/textbook/` promotion is chartered to T.Z — a register DECISION, not a break.** C-Textbook
  §location explicitly defers the single-file-vs-directory question to T.Z. **Internal-continue → T.Z
  decides and records it in C-Textbook;** this is the one chartered amendment, distinct from a register
  break (which would still surface at the ◆).

- **Static-frame ROADMAP debt (surface at the Z.1.3 + T.Z ◆ — out of `@architect` PLAN-write scope; a
  capture candidate) — now compounded to a SIXTH consecutive carry.** The ROADMAP Progress table still
  shows "ε — Shor + PQ (S) … 0 (S.A sharded, not yet executed)" and lists S.A–S.D as un-started —
  **stale by the full Track-S arc** (all of S.A/S.B/S.C/S.D landed; Phase ε is complete) — and now Phase
  ζ is opening with Z.1 sharded. The S.A/S.B/S.C/S.D ◆ digests each named this; the write was deferred
  (out of `@architect` PLAN-write scope) every time. **With the project closing at the T.Z ◆, the full
  reconciliation is owed** (mark every phase complete; mark the whole project done). **This is a ROADMAP
  write — outside the `@architect` PLAN-only write scope; surfaced here as a capture candidate for the
  user to action (via `/note` or a ROADMAP edit), not a PLAN edit.** Not an implementation concern; does
  not block Z.1/T.Z.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase ζ — Z.1: "*Umbrella narrative. 2-4 sessions. Predecessor: everything.
  The README + the master PEDAGOGY.md … The L-notation comparison across attacks; the modularity-theorem
  speculation chapter; the structure-based-escape-from-search synthesis. All sessions Opus-tier.*"; Phase
  τ — T.Z: "*Textbook bind. 1 session. Predecessor: all per-track chapters + Z.1 … cross-references
  resolved, the comparative L-notation synthesis across all attacks written, notation unified, the
  prerequisites chapter reconciled … Opus-tier.*") and this PLAN before any session. **NOTE: the ROADMAP
  Progress / Remaining tables are stale by the FULL Track-S arc (S.A–S.D landed; Phase ε complete) and
  now Phase ζ is opening; this is the SIXTH reconciliation prompt — surface it at the ◆, but it is
  outside `@architect` PLAN-write scope (a capture candidate).**
- Read the **artifacts to synthesise/bind** (do NOT modify the code): `README.md` (the rho-scoped front
  page Z.1.1 supersedes); the **5 code-tours** — `docs/PEDAGOGY.md` (ρ §1–7 + E.W §8–18, the file Z.1.2
  edits), `gnfs/docs/PEDAGOGY.md` (G+D §1–71), `shared/numth/docs/PEDAGOGY.md`,
  `shared/numfield/docs/PEDAGOGY.md`, `shor/docs/PEDAGOGY.md`; the **11 chapters** of
  `docs/MATHEMATICS.md` (esp. §3 through-line, §7 GNFS L-notation payoff, §9.7 NFS-DL delta, §10.6
  algebraic-ECDLP comparison, the ToC lines 143–160, the chapter-pairing table lines 97–105). Read the
  **crate/track map** from `Cargo.toml` (9 members across 5 tracks + shared substrate). Read the
  **performance facts to cite** (do NOT edit): `docs/BENCHMARKS.md`.
- **Register:** Z.1/T.Z are **PROSE ONLY** (`STYLE-DOC.md`). Two registers: **Z.1 (.1/.2/.3)** the
  code-first umbrella register (narrative + cross-reference + the chartered speculation register for the
  modularity chapter); **T.Z** the maths-first textbook register (C-Textbook: survey-with-proof-sketch
  depth, MathJax display math, the chartered promotion decision). **No `STYLE-CODE.md` register
  applies** — Z.1/T.Z write no code.
- **Tier routing:** **all four sessions are Opus `@build`** (the ROADMAP: "*Z.1 — all sessions Opus-tier
  … the highest integrative judgment load in the project*"; "*T.Z — Opus-tier — highest integrative
  judgment load in the textbook*"). The whole-library synthesis (Z.1) and the whole-textbook bind (T.Z)
  are the project's peak integrative judgment. **juncture-tier (header) is `opus`** — the **lever-5
  opt-down is UNAVAILABLE** (no prose test inner-loop; the ◆ + review is the only correctness gate; the
  same firm posture as S.D). Both ◆ forks page `@plan-juncture` at opus.
- **Invariants to preserve:** **Z.1/T.Z are prose-only** (no `.rs`, no `Cargo.toml`, no `benches/`, no
  crate, no dependency — a code change is a HALT-and-surface defocus signal). **They amend NO frozen
  code contract** (they synthesise/bind the library and consume C-Textbook read-only; the one chartered
  exception is T.Z's `docs/textbook/` promotion *decision*). **The README is a workspace umbrella, not
  rho-scoped.** **The master PEDAGOGY.md synthesises, not duplicates** (cites the per-crate tours). **The
  modularity chapter is chartered-as-a-chapter and flagged-as-speculation** (not a footnote, not a
  claimed result). **The T.Z bind is STRUCTURAL** (the stale pairing table + ToC reconciled, the
  cross-track L-notation synthesis written, prerequisites reconciled, promotion decided — not a
  copy-edit-only pass). **The comparative L-notation synthesis is accurate** (read §7/§9.7/§10.6, do not
  recall constants). **BENCHMARKS is cited, not edited.** **C-Textbook is consumed read-only** (save the
  chartered T.Z promotion decision).
- **No code, no crate, no dependency, no benchmark; prose files only (load-bearing).** Z.1/T.Z edit
  `README.md`, `docs/PEDAGOGY.md`, `docs/MATHEMATICS.md` and may add at most one new prose file (the
  modularity chapter). No `.rs`, no manifest, no `benches/`; `cargo check`/`cargo test --workspace` are
  pure regression guards (trivially green — a non-green result means Z.1/T.Z drifted into code).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  3-session umbrella closing at the Z.1 ◆, then the textbook bind closing the project at the T.Z ◆) is
  the **project terminus**, the highest-judgment closeout in the project, with **two new judgment
  surfaces** (the whole-library cross-track synthesis spanning 5 tours + 11 chapters; the authored-new
  modularity speculation chapter) and **no test inner-loop to catch prose drift**, plus a genuine
  cross-sub-track seam (T.Z consumes Z.1's L-notation synthesis). The conservative default is to halt at
  **both** ◆ — the Z.1 ◆ (so a wrong umbrella synthesis does not propagate silently into the final bind)
  and the T.Z ◆ (the project-final human glance + opus juncture). All four sessions are prose in settled
  genres, so Z.1.1/Z.1.2 could run autonomously between halts, but the whole-library breadth + the
  no-test-loop reality + the project-close milestone argue for `halt-at-boundaries`. *(Tradeoff vs
  autonomous: `halt-at-boundaries` trades velocity on the mechanical README/synthesis work for a
  guaranteed human check at the umbrella close + the project terminus — the right trade given lever-5 is
  weak and lever-1 is high. If Z.1.1/Z.1.2 land clean and read faithfully against the corpus, Z.1.3 can
  be dispatched immediately after each glance.)*
