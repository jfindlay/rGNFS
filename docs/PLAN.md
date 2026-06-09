<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-D closes (D.W / T.D — NFS-DL writeup + maths chapter)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default, but on a different lever than D.C.**
D.W is a **writeup** sub-track: it freezes no code contract (lever 3 absent), and a wrong sentence is
an editable doc bug, not a silent propagating failure (lever 4 low at the chapter level). On the
writeup levers alone this would opt **down** to Sonnet. It does **not**, for one reason: D.W.2 carries
the **L-notation NFS-DL derivation — a ROADMAP-designated payoff proof**, tiered Opus by explicit
ROADMAP flag, and D.W.2 is the **Track-D / Phase-γ ◆ close**. The boundary juncture therefore
ratifies (a) the payoff proof's DL-vs-factoring asymptotic comparison — the pedagogical-correctness
apex of the whole NFS-DL arc — and (b) the design-statement verification for *all* of Track D
(principles 1/3/4, mirroring G.W's verification role), then integrates D.W discoveries into the
ROADMAP at the Phase-γ boundary. That ratification is a ROADMAP-frame, correctness-critical judgment
— the lever-4 register the ROADMAP asserts on the payoff carries to the boundary that signs it off.
**The juncture-tier hold is scoped to D.W.2's ◆ close; D.W.1 (the code-tour) needs no Opus juncture**
(it freezes nothing and cites only frozen contracts). *(Contrast D.C: held Opus on a live cross-track
interface freeze, C2. D.W has no interface to freeze — it holds Opus on a designated payoff + the
phase-closing design-statement verification, the same role G.W played for Track G.)*

Last rewrite: D.C.3 ◆ boundary crossed (Track-D's algorithmic content complete — D.C.1 → D.C.2 →
D.C.3 coherent; individual logarithm + special-q descent landed, C2 `solve_dl` frozen at 9d07c51,
ledger reconciled 2026-06-09, commits 1651993 / 52ee232 / 9d07c51). This plan opens **Track D's final
sub-track, D.W — the NFS-DL writeup**: the code-tour chapter (D.W.1, appended to
`gnfs/docs/PEDAGOGY.md`) and the maths-first textbook chapter (D.W.2 / **T.D**, appended to
`docs/MATHEMATICS.md`), paired per the ROADMAP τ-threading rule. Crossing D.W.2 ◆ closes Phase γ
(NFS-DL) end-to-end; only Track E (the MOV bridge E.C consumes C2) lies beyond.

---

## Purpose (design intent)

Per ROADMAP: D.W is "the NFS-DL writeup — the NFS-DL chapter; explicit comparison with the
NFS-factoring chapter." Per the Phase-τ threading rule, the writeup session produces **two siblings**
at the Track-D ◆ boundary — the code-tour chapter (`PEDAGOGY.md`, organised by the implementation)
**and** the maths-first textbook chapter (`MATHEMATICS.md` / **T.D**, learnable on its own,
maths-first). Both already have reserved skeletons: `MATHEMATICS.md` TOC entry 9
("NFS-DL: Discrete Logarithm via the Number Field Sieve *(T.D — to be appended)*") and the
`PEDAGOGY.md` §63+ slot after the G.W integrative chapter (§52–§62). The work has two stages, which
are the two sessions:

1. **NFS-DL code-tour chapter (D.W.1, Sonnet, integrative-writeup).** Append the NFS-DL chapter to
   `gnfs/docs/PEDAGOGY.md` (§63+), mirroring the G.W integrative chapter (§52–§62) one phase over:
   the NFS-DL pipeline at a glance, stage-by-stage against the **frozen** Track-D contracts
   (C-DLRelation, C-Schirokauer, C-LinAlgFl, C-Descent, C2), the **explicit comparison with the
   NFS-factoring code-tour** (what changes when the target is `log_g(h)` rather than a factor: the
   two-number-field setup, Schirokauer maps replacing quadratic characters, F_ℓ linear algebra
   replacing GF(2), and the individual-logarithm descent that has *no factoring analogue*), the
   principle-4 annotations (toy-scale DL phenomenology), and the KAT summary citing the existing DL
   KATs (no new KATs — a code-tour, not an implementation). Cites the T.D maths chapter for the
   mathematics.

2. **NFS-DL maths chapter + L-notation payoff (D.W.2 / T.D ◆, Opus, integrative-payoff).** Append the
   NFS-DL chapter to `docs/MATHEMATICS.md` (TOC entry 9), mirroring the §GNFS chapter (§1–§8) one
   phase over: the DL problem and the number-field bridge for DL, the factor-base / virtual-logarithm
   construction, **Schirokauer maps** (the obstruction-to-principality correction the DL setting
   needs, with proof-sketch depth), the **F_ℓ linear algebra** (block Wiedemann/Lanczos over a prime
   field vs GF(2)), and the **individual-logarithm special-q descent** (the no-factoring-analogue
   core). The **designated payoff proof**: the **L-notation complexity of NFS-DL** as a *delta on the
   already-written §GNFS L_N[1/3, (64/9)^{1/3}] derivation* — same exponent 1/3, same constant; the
   DL-specific content is that the descent cost is asymptotically subdominant and the F_ℓ linear
   algebra carries the same complexity shape as GF(2). This session **crosses the D.W ◆ boundary**
   (Track-D / Phase-γ complete) and runs the **design-statement verification for the whole NFS-DL
   arc** (principles 1/3/4, mirroring G.W §59).

Re-read this intent at the ◆ boundary to catch **defocus** (writing the **F_{p^k} extension-field**
mathematics — that is the deferred E.C-prep content, not D.W; or writing **E.C / the MOV bridge**
mathematics — that is Track E / the T.E chapter) and **rigidity** (re-deriving the L_N[1/3] exponent
from scratch when §GNFS already carries it and T.D needs only the DL *delta*; or letting the code-tour
and maths chapter drift apart — they must cross-reference, the same consistency the G.W↔T.G pair holds).

**Scoping discipline (Phase-τ register, applied here).** Both chapters obey **C-Textbook** (frozen at
T.0, 5c9b783): audience = undergraduate maths background; depth = survey-with-proof-sketch (complete
and clinical, not exhaustive, not inscrutable; full proofs only at the designated payoff — here, the
L-notation DL derivation); through-line = structure-based escape from search; markup = Markdown +
MathJax; location = `docs/MATHEMATICS.md` (single-file). The textbook chapter is **maths-first**
(learnable without the code); the code-tour is **code-first** (assumes the reader knows the maths,
cites T.D). **No new code, no new contracts, no new KATs** — D.W documents the frozen Track-D
substrate; it amends nothing. The **F_{p^k} extension** stays out of scope (C2's k > 1 `Unsupported`
debt is *recorded* in T.D as a principle-4 annotation, not *resolved* — that is the deferred E.C-prep
session, see Discoveries). The L-notation derivation is a **delta**, not a re-derivation (rigidity guard).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Unchanged from D.C
and confirmed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface; **no doc
build toolchain** (MathJax renders client-side in GitHub/standard Markdown viewers — confirmed in the
ROADMAP C-Textbook freeze; there is no Sphinx/`.. math::` build to run). D.W is a **writeup** —
it adds **no new code and no new KATs** (per G.W §62, the integrative chapter adds none), so the gate's
role here is a **regression guard**: `cargo test --workspace` must stay green (a doc edit must not
break a citation-referenced code path or the existing DL KATs the chapters cite). Discovery is
unambiguous; `/run-plan` re-discovers at preflight. *(The "inner loop" for prose correctness is human
+ the Opus ◆ juncture, not the test suite — which is why the writeup levers, not lever 5, drive the
tier story.)*

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| D.W.1 | NFS-DL code-tour chapter: stage-by-stage against frozen Track-D contracts + explicit comparison with the NFS-factoring code-tour | I | Sonnet | C-DLRelation, C-Schirokauer, C-LinAlgFl, C-Descent, C2 (all frozen, read), G.W code-tour (§52–§62, parallel), C-Textbook (obeyed) | `gnfs/docs/PEDAGOGY.md` (append §63+) |
| D.W.2 ◆ `@plan` | NFS-DL maths chapter (T.D) + L-notation DL derivation (designated payoff) + Track-D design-statement verification | I | **Opus** | C-Textbook (obeyed), MATHEMATICS.md §GNFS (§1–§8, parallel + L-notation §7 template), D.W.1 code-tour (cross-ref), all frozen Track-D contracts (read) | `docs/MATHEMATICS.md` (append ch. 9), `gnfs/docs/PEDAGOGY.md` (cross-ref backlink to T.D, if needed) |

**Sequencing notes.** Strictly serial: **D.W.1 → D.W.2.** The code-tour lands first (it fixes the
section structure and the contract-by-contract narrative the maths chapter cross-references); D.W.2's
maths chapter then cites the code-tour for the realisation and carries the payoff. The single `@plan`
marker sits on **D.W.2 ◆** — the Opus boundary juncture that ratifies the L-notation payoff and runs
the whole-Track-D design-statement verification before Phase γ is declared closed. D.W.1 carries **no**
`@plan` (it freezes nothing; default cadence suffices to land it).

**Why 2 sessions (one over the ROADMAP's literal allotment — split decided at this boundary).** The
ROADMAP allots D.W = 1 session with T.D folded in (effort-neutral), but flags two escape hatches: the
textbook chapter "is the larger of the two and may split into a dedicated follow-on if it overruns,"
and the T.D L-notation derivation is a "designated payoff → Opus-tier." Both hatches fire here, and
the split is taken **up-front** rather than discovered mid-session:
- **One-line-commit-title corollary.** "NFS-DL code-tour chapter" and "NFS-DL maths chapter +
  L-notation derivation" are **two distinct commit titles** — and folding both into one would also
  fold two *registers* (code-first vs maths-first) and two *artifacts* into one session.
- **Contract-sharp doc boundary (the split is legitimate, not LOC-driven).** D.W.1 produces the
  code-tour the maths chapter *consumes* as a cross-reference; D.W.2 consumes it. The split is at a
  real produce/consume seam, exactly as the G.W↔T.G pairing split code-tour from maths chapter.
- **Tier boundary.** D.W.1 is Sonnet (code-tour against frozen contracts — mechanical, lever-3/4
  low); D.W.2 is **Opus** (the ROADMAP-designated payoff + the phase-closing design-statement
  verification). A single session cannot be both tiers; the tier seam *is* the session seam.
- **Size precedent.** The T.G maths sibling ran ~660 lines (`MATHEMATICS.md` §GNFS, §1229–§1892);
  the G.W code-tour ran ~930 lines (§52–§62). Folding the DL equivalents of both into one session
  would push it well past the band — the "may split if it overruns" hatch, taken before the overrun.

They are **not** further splittable: the L-notation DL derivation is a *delta* on the written §GNFS
derivation (same exponent, same constant — the DL difference is descent-cost-subdominance + the F_ℓ
linear-algebra shape), so fracturing it into its own third session would split a single coherent
payoff-proof at a non-contract-sharp boundary just to isolate the Opus unit — forbidden. D.W.2 holds
the maths chapter *and* its payoff whole.

---

## Session detail

D.W.1 is crisply specified (it mirrors the frozen G.W integrative chapter one phase over, against
already-frozen contracts). D.W.2 is specified at full fidelity too (its template — the §GNFS chapter
and its §7 L-notation derivation — is frozen and in-tree), with the one open judgment (the depth of
the F_{p^k}-deferral annotation) flagged for the ◆ juncture.

### D.W.1 — NFS-DL code-tour chapter (Sonnet, integrative-writeup)

**Deliverable:** the NFS-DL code-tour chapter appended to `gnfs/docs/PEDAGOGY.md` (§63+), the
code-first sibling to the T.D maths chapter. Mirrors the G.W integrative chapter (§52–§62) one phase
over. Structure (parallel to §52–§62):
- **The NFS-DL pipeline at a glance** (parallel to §52) — relation collection (D.A) → F_ℓ linear
  algebra (D.B) → virtual-log table → individual-logarithm descent (D.C) → `solve_dl`.
- **Stage-by-stage against the frozen contracts** (parallel to §53–§57): C-DLRelation (`DLMatrix`,
  `collect_dl_relations`, the rational|algebraic|Schirokauer column layout), C-Schirokauer (the
  Schirokauer map, `PrimeIdeal`), C-LinAlgFl (`VirtualLogTable`, `recover_virtual_logs`), C-Descent
  (`DescentNode`, `DescentFrontier`, `init_descent_frontier`, `descend_node`, `run_descent`), and
  **C2** (`solve_dl` — the cross-track interface E.C will consume, k = 1 live / k > 1 `Unsupported`).
- **The explicit NFS-factoring comparison** (the ROADMAP's named requirement; parallel to §58's
  unified-contract view) — a side-by-side: *what changes when the target is `log_g(h)`*: two number
  fields and the shared rational side; Schirokauer maps replacing the quadratic-character columns;
  F_ℓ (prime-field) linear algebra replacing GF(2); virtual logs replacing the congruence of squares;
  and the **individual-logarithm special-q descent — the stage with no factoring analogue**.
- **Design-statement note + principle-4 annotations** (parallel to §59 — the *full* verification is
  D.W.2's, but the code-tour carries its toy-scale annotations): the descent-tree breadth and
  medium-prime tuning as NFS-scale phenomena annotated at demonstration fidelity; the C2 F_{p^k}
  k > 1 `Unsupported` debt noted as an engineering-scale, not mathematical, boundary.
- **KAT summary** (parallel to §62) — cites the **existing** DL KATs (`dl_descent_kat.rs`,
  `dl_individual_log_kat.rs`, the D.B end-to-end DL KAT); **no new KATs** (a code-tour adds none).
- **Cross-references + Further Reading** (parallel to §61 / §GNFS-references) — cite the T.D maths
  chapter for the mathematics; the D.A/D.B/D.C contract definitions; the G.W code-tour for the
  factoring sibling.

Consumes the frozen Track-D contracts (read-only narration) and the G.W code-tour (the parallel to
mirror). Freezes nothing. Obeys C-Textbook (markup, register).

**KAT:** none new (writeup). **Regression gate:** `cargo test --workspace` green — no citation edit
breaks a referenced code path or an existing DL KAT.

**Subtlety:** the load-bearing judgment is the **NFS-factoring comparison** (the chapter's reason to
exist per the ROADMAP — it must be *explicit and structural*, not a passing mention) and **fidelity
to the frozen contracts** (every signature the chapter narrates must match the in-tree code — a
code-tour that describes a stale interface is the failure mode). Mechanical otherwise: the §52–§62
template is frozen and high-quality.

**Deferred:** the maths chapter + L-notation payoff (D.W.2); the F_{p^k} mathematics (E.C-prep,
Discoveries); the MOV-bridge / T.E mathematics (Track E).

### D.W.2 ◆ — NFS-DL maths chapter (T.D) + L-notation payoff + design-statement verification (Opus, integrative-payoff, `@plan`)

**Deliverable:** the NFS-DL maths-first chapter appended to `docs/MATHEMATICS.md` (TOC entry 9,
"NFS-DL: Discrete Logarithm via the Number Field Sieve"), the maths-first sibling to D.W.1, learnable
on its own. Mirrors the §GNFS chapter (§1–§8) one phase over. Structure (parallel to §GNFS):
- **Introduction + through-line** (parallel to §1) — the DLP as a search problem; the structure NFS-DL
  exploits (smoothness in two number fields + the homomorphism to F_p*); how it escapes the generic
  √n bound (the C-Textbook through-line).
- **The number-field bridge for DL** (parallel to §3) — two number fields sharing the rational side;
  the maps to F_p*; the factor-base / **virtual-logarithm** construction (logs of factor-base
  elements as the unknowns of an F_ℓ linear system).
- **Schirokauer maps** (the DL-specific algebra, proof-sketch depth) — the obstruction to
  principality that the unit group introduces, and the Schirokauer-map correction that the factoring
  setting (with its quadratic characters) does not need. The clearest single "what's different about
  DL" moment.
- **F_ℓ linear algebra** (parallel to §5) — recovering virtual logs as the kernel/solution of a
  linear system over the prime field F_ℓ (vs GF(2) for factoring); block Wiedemann/Lanczos with the
  F_ℓ care.
- **Individual-logarithm special-q descent** (the no-factoring-analogue core) — initialization-
  smoothing, the special-q descent recursion, log-assembly along the descent tree; subgroup recovery
  (the log mod ℓ vs mod p−1 note).
- **§ The L-Notation Complexity of NFS-DL (the designated payoff proof)** (parallel to §7) — **a delta
  on the §GNFS L_N[1/3, (64/9)^{1/3}] derivation, not a re-derivation**: the relation-collection +
  linear-algebra balance gives the **same exponent 1/3 and the same constant** as factoring; the DL
  delta is (a) the individual-logarithm descent is **asymptotically subdominant** (a lower-order term
  in the L-notation), so it does not change the leading complexity, and (b) the F_ℓ linear algebra
  carries the same complexity shape as GF(2). The payoff is the *explicit asymptotic comparison*:
  *why NFS-DL and NFS-factoring share the same L_N[1/3] complexity despite solving different
  problems* — the structure-based-escape-from-search through-line at its sharpest.
- **§ Cross-References + References** (parallel to §8 / §GNFS-references) — cite D.W.1 for the code
  realisation; §GNFS §7 for the shared derivation core; §Prerequisites for the L-notation /
  Canfield–Erdős–Pomerance engine; §On Scale for the F_{p^k} / toy-scale annotations.

**Design-statement verification for the whole NFS-DL arc** (the G.W §59 analogue, the ROADMAP's
"verified against the actual implementation" role for Track D): principle 1 (algorithmic content
complete — relation adaptation, F_ℓ linalg, special-q descent all implemented head-on); principle 3
(no engineering optimisations crept into D.A–D.C); principle 4 (scale-only at demonstration fidelity
— the descent breadth, medium-prime tuning, and the **F_{p^k} k > 1 `Unsupported` debt** annotated as
engineering-scale, not mathematical, boundaries). Verdict recorded in the action-frame digest and
integrated into the ROADMAP Discoveries log at the Phase-γ ◆.

Consumes C-Textbook (obeyed), the §GNFS chapter (parallel + the §7 L-notation template), D.W.1 (cross-
reference), and the frozen Track-D contracts (read). Freezes nothing (C-Textbook is obeyed, not amended).

**KAT:** none new (writeup). **Regression gate:** `cargo test --workspace` green.

**Subtlety:** the load-bearing judgments are (1) the **L-notation DL-vs-factoring comparison** — the
designated payoff, where a subtly-wrong asymptotic claim (e.g. mis-stating the descent cost as
leading-order, or the F_ℓ vs GF(2) complexity difference) is the silent pedagogical-correctness
failure the Opus tier guards against; (2) the **delta discipline** (re-using the §GNFS §7 derivation
rather than re-deriving — rigidity guard); and (3) the **F_{p^k}-deferral annotation depth** — how much
of the extension-field gap to surface here as a principle-4 annotation vs leave to the T.E chapter
(decided at the ◆ juncture). This is the **D.W ◆ / Phase-γ boundary** — re-read the Purpose intent and
verify the whole Track-D arc (D.A → D.B → D.C → D.W) is coherent and that NFS-DL is complete (only
Track E, the MOV bridge consuming C2, lies beyond) before crossing.

**`@plan` confirmation (post-landing, T0/Opus, one-shot).** Page a `@plan-juncture` fork at the D.W.2
◆ to confirm: (1) the L-notation DL payoff is correct and is a *delta* on §GNFS (not a flawed
re-derivation), with the DL-vs-factoring asymptotic comparison sound; (2) the design-statement
verification for the whole NFS-DL arc passes on principles 1/3/4 (the G.W §59 analogue); (3) the
code-tour (D.W.1) and maths chapter (T.D) cross-reference consistently and neither drifts from the
frozen contracts; (4) the F_{p^k} deferral is annotated as a recorded debt, not silently dropped; (5)
the Phase-γ ◆ ROADMAP Discoveries integration is identified. One-shot findings; does not implement.
Held at **Opus** on the designated-payoff flag + the phase-closing design-statement verification, per
the header.

---

## Cross-session contracts

D.W is a **writeup** sub-track: it **freezes no new contract** and **amends none**. The only
contract-shaped object in scope is **C-Textbook**, which both chapters **obey** (it is already
frozen). All Track-D code contracts are **read** (narrated), not touched.

### C-Textbook — documentation-register contract (prose-enforced) — *frozen T.0 (5c9b783); obeyed, not amended*

**Defined:** T.0. **Consumed by:** every textbook chapter, here T.D (D.W.2) and — for register
consistency — the D.W.1 code-tour. Prose-enforced (the chapters' register is checked at the ◆
juncture against the contract). **D.W obeys it; it does not flex it.** A chapter that needed to break
the register (a topic requiring graduate background) would surface that as a discovery and flex
C-Textbook at the next inflection — D.W has no such need (NFS-DL sits within the undergraduate-plus
floor the contract sets). The register: audience = undergraduate maths background; depth =
survey-with-proof-sketch (complete, clinical, not exhaustive, not inscrutable; full proofs only at
designated payoffs — here the L-notation DL derivation); through-line = structure-based escape from
search; markup = Markdown + MathJax (`$…$` / `$$…$$`); location = `docs/MATHEMATICS.md` (single-file).

### Frozen contracts read by D.W (narrated, not amended)

The D.W chapters narrate these; none is touched. (Full definitions in the D.C plan's
Cross-session-contracts section and the source.)

- **C2** — NFS-DL solver interface (`solve_dl(g, h, p, k, ell) -> Result<BigInt, SolveDlError>`;
  `gnfs/src/dl/descent/solve.rs`) — *frozen D.C.3 (9d07c51)*. The cross-track interface E.C consumes;
  the code-tour narrates its shape and the k > 1 `Unsupported` debt; the maths chapter annotates the
  F_{p^k} deferral.
- **C-Descent** — individual-log descent substrate (`DescentNode`, `DescentFrontier`,
  `init_descent_frontier`, `descend_node`, `run_descent`; `gnfs/src/dl/descent/`) — *frozen D.C.1
  (1651993)*. Narrated as the no-factoring-analogue stage.
- **C-LinAlgFl** — F_ℓ block-solver substrate (`VirtualLogTable`, `recover_virtual_logs`;
  `gnfs/src/dl/linalg/blockvec_fl.rs`) — *frozen D.B.1 (652cfa6)*. Narrated as the F_ℓ-vs-GF(2) stage.
- **C-DLRelation** — `DLRelation` + `DLMatrix` (`collect_dl_relations`, the rational|algebraic|
  Schirokauer column layout; `gnfs/src/dl/relation.rs`) — *frozen D.A.1 (f2dbf0a)*.
- **C-Schirokauer** — the Schirokauer map (`schirokauer` / `compute_schirokauer`, `PrimeIdeal`;
  `gnfs/src/dl/schirokauer.rs`) — *frozen D.A.1 (f2dbf0a)*. The maths chapter's "what's different
  about DL" payoff-adjacent section.
- **The G.W code-tour (§52–§62) and the §GNFS maths chapter (§1–§8, incl. the §7 L-notation
  derivation)** — *frozen at G.W (76f3633) / T.G (a896198)*. The **templates** D.W.1 and D.W.2 mirror
  one phase over, and the L-notation derivation D.W.2 takes a delta on.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The D.W.2 ◆ `@plan` confirmation is not a separate ledger row (a
paged fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest and
the ROADMAP Discoveries log.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| D.W.1 | NFS-DL code-tour chapter (PEDAGOGY.md §63+) + NFS-factoring comparison | pending | — | — (writeup; freezes nothing) |
| D.W.2 ◆ | NFS-DL maths chapter (T.D) + L-notation payoff + Track-D design-statement verification | pending | — | — (writeup; obeys C-Textbook, freezes nothing) |

Contracts frozen before this sub-track (read by D.W): C-NF (bdba6f5 / 20cd263), C-Ideal (05b27c8),
C-Res (bcd63cd), C-Dedekind (7844773), C-Relation / C-FactorBase (c1dc0b6), C-Score (00aa32d),
C-Matrix (a0e854b), C-LinAlg (416f6db), C-AlgSqrt (c80a855 + ec69a1f), C1 (α.2), C-Textbook (5c9b783),
C-Schirokauer / C-DLRelation (f2dbf0a), C-LinAlgFl (652cfa6), C-Descent (1651993), C2 (9d07c51). This
sub-track is the Phase-γ writeup over the now-complete NFS-DL pipeline; **it freezes no new contract.**

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **D.W freezes nothing; the discovery surface is documentation fidelity, not contract drift.** A D.W
  session "discovering" that a frozen contract's narrated shape is wrong means the *chapter* is wrong
  (fix the prose — **internal-continue**), not that the contract changed. A genuine discovery that an
  in-tree contract is itself broken (a code bug the writeup exposes) is **additive** at most (a fix
  session), and a *destructive* edit to a frozen Track-D contract to make a chapter's claim true is a
  **destructive-HALT** — the writeup documents the code, never the reverse.

- **F_{p^k} extension deferral — annotate, do not resolve (decided at D.C, carried into D.W).** C2's
  k > 1 path returns `Unsupported`; the F_{p^k} NFS-DL extension is genuine new mathematics deferred
  to an E.C-prep ROADMAP-then-shard session. In D.W this is an **annotation target**, not a work item:
  the code-tour notes the debt; the maths chapter (T.D) carries a principle-4 annotation
  (engineering-/mathematical-dimension-scale boundary, not a toy-scale artifact). Writing the F_{p^k}
  mathematics *here* is **defocus** — internal-continue only within the annotation scope. The depth of
  the annotation (how much of the extension-field picture to sketch vs leave to T.E) is the one open
  judgment, resolved at the D.W.2 ◆ juncture.

- **L-notation DL derivation is a delta, not a re-derivation (rigidity guard, D.W.2).** The §GNFS §7
  chapter already derives L_N[1/3, (64/9)^{1/3}] in full. T.D must **re-use** it: same exponent, same
  constant; the DL content is descent-cost-subdominance + the F_ℓ-vs-GF(2) linear-algebra shape.
  Re-deriving the exponent from scratch is rigidity (wasted Opus budget on settled mathematics);
  **internal-continue** is to write the delta and cross-reference §7. A genuine finding that the DL
  complexity *differs* from factoring at leading order (it does not, at this fidelity) would be a real
  discovery — surface it, do not paper over it.

- **Code-tour ↔ maths-chapter consistency (D.W.1 → D.W.2).** The two siblings must cross-reference and
  not drift (the G.W↔T.G consistency requirement, one phase over). D.W.1 fixes the structure D.W.2
  cites; if D.W.2 finds the code-tour's stage decomposition awkward for the maths narrative, adjusting
  the *maths chapter's* framing is **internal-continue**; reopening D.W.1 to re-structure the code-tour
  is an **additive-reshard** (a small follow-on), surfaced at the ◆.

- **Phase-γ ◆ is a ROADMAP-frame event (design-statement verification + Discoveries integration).**
  D.W.2 ◆ closes Track D / Phase γ. Per the ROADMAP, a phase-closing design-statement verification and
  the Discoveries-log integration are **inflection-point Opus** work (the G.W precedent) — this is why
  the ◆ carries an `@plan` Opus juncture, not default cadence. The juncture's verdict (principles
  1/3/4 for the whole NFS-DL arc) is integrated into the ROADMAP at the boundary, not mid-session.

- **No new KATs, regression-gate only (writeup invariant).** D.W adds no KATs (G.W §62 precedent:
  the integrative chapter adds none). The gate's role is to confirm no doc edit breaks a referenced
  code path. A "discovery" that a cited KAT fails is a real regression — **HALT and surface**, not a
  prose fix.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase γ — "D.W — NFS-DL writeup"; Phase τ — the chapter-pairing rule and the
  **C-Textbook** scope contract every chapter obeys; the "On scale" section feeding the principle-4
  annotations) and this PLAN before any session.
- Read the **templates to mirror**: `gnfs/docs/PEDAGOGY.md` §52–§62 (the G.W integrative code-tour —
  D.W.1's one-phase-over model) and `docs/MATHEMATICS.md` §GNFS §1–§8 incl. the §7 L-notation
  derivation (the T.G maths chapter — D.W.2's model and the derivation D.W.2 deltas on). Read the
  **frozen substrate D.W narrates**: `gnfs/src/dl/relation.rs`, `gnfs/src/dl/schirokauer.rs`,
  `gnfs/src/dl/linalg/blockvec_fl.rs`, `gnfs/src/dl/descent/` (node, recurse, solve), and the existing
  DL KATs (`gnfs/tests/dl_descent_kat.rs`, `gnfs/tests/dl_individual_log_kat.rs`).
- **Register:** D.W is **documentation** (Markdown + MathJax, `STYLE-DOC.md`), obeying **C-Textbook**.
  The code-tour is code-first (cites the maths chapter); the maths chapter is maths-first (learnable
  on its own). 100-char wrap, MathJax for non-trivial display math, Unicode glyphs for trivial inline.
- **Tier routing:** **D.W.1 is Sonnet** (code-tour against frozen contracts — mechanical, lever-3/4
  low). **D.W.2 is Opus** (`@build` on Opus) — the ROADMAP-designated L-notation payoff + the
  phase-closing design-statement verification. D.W.2 carries the single `@plan` marker: a T0/Opus
  ◆-boundary juncture (page `@plan-juncture`) ratifying the payoff and the whole-Track-D verification
  before Phase γ is closed. The juncture-tier (header) is **opus** on the same designated-payoff +
  design-statement-verification call.
- **Invariants to preserve:** **all Track-D and Track-G code contracts are frozen — D.W reads and
  narrates them; it amends none.** No new code, no new KATs. The writeup documents the code, never the
  reverse (a destructive contract edit to satisfy a chapter claim is a destructive-HALT). C-Textbook
  is obeyed, not flexed. The F_{p^k} debt is **annotated, not resolved** (E.C-prep owns it).
- **PARI / CADO remain dev-only oracles** — D.W narrates the resolved gating policy where relevant
  (the D.C.3 PARI cross-check stub), introduces no new oracle dependency.
- Suggested first invocation: **`/run-plan docs/PLAN.md`** (default cadence). The single `@plan`/◆
  juncture on D.W.2 forces the one halt that matters — the Opus payoff + Phase-γ design-statement
  verification before Track D is declared closed. D.W.1 is a clean Sonnet writeup against frozen
  contracts and needs no boundary halt. *(Tradeoff: this is one notch less conservative than a
  `halt-at-boundaries` run, justified because the only correctness-critical juncture — the payoff
  ratification — is already `@plan`-gated, and D.W.1 freezes nothing.)*
