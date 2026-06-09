# rGNFS — Roadmap

The durable, project-lifetime view of the work. Updated only at sub-track boundaries.
For the current sub-track's actionable detail, see `docs/PLAN.md`.

For the planning philosophy this document embodies, see
`~/Documents/software/opencode-config/multi-session-planning.md`.

---

## Design statement

**A self-consistent, pedagogically clear, complete Rust reference library for discrete-logarithm and
integer-factorization algorithms.** Covers:

1. Pollard rho for integer factorization (existing `rho` crate).
2. Pollard rho for ECDLP with canonical CPU optimizations (existing `rho` crate).
3. GNFS for integer factorization, at toy scale, complete in algorithmic content.
4. NFS-DL for finite-field discrete logarithm, sharing scaffolding with GNFS.
5. The canonical algebraic ECDLP attacks: Pohlig–Hellman, MOV/Frey–Rück, Smart–Satoh–Araki,
   GHS/Weil descent, Gaudry–Diem–Joux–Vitse index calculus.
6. Shor's algorithm via a classical state-vector simulator, for both factoring and ECDLP.
7. A post-quantum context chapter situating the classical work in the broader migration landscape.

**The pedagogical thesis** is that these attacks belong in one library because they share
substrates (finite-field arithmetic, polynomial arithmetic, smoothness testing), share techniques
(birthday-bound walks, index calculus, structure-based escape from search), and the most
illuminating moments — especially the MOV bridge where ECDLP reduces to NFS-DL — only appear when
they are treated together.

**Scoping discipline (the three-way split):**

1. **Algorithmic content** is included completely. Every phase of NFS — polynomial selection,
   sieving (line and lattice), filtering, linear algebra (both block Wiedemann and block Lanczos),
   square root — is implemented from scratch in Rust. Same for every named ECDLP attack.
2. **Scale-only optimizations** (Coppersmith multi-poly, large-prime variations, Galois automorphism
   quotienting) are implemented at *demonstration fidelity*: their mathematical content is present
   in the code, even where their performance contribution doesn't show at toy scale.
 3. **Engineering optimizations** (SIMD, NUMA, custom assembly, MPI distribution, GPU) are
    explicitly omitted. **CADO-NFS** serves as the single live correctness oracle — an opt-in
    *validation sidecar* on rGNFS's (purely demonstrative) runtime, never part of how rGNFS computes
    its answer. (msieve, originally a co-oracle, is retired; PARI and msolve remain narrow per-track
    DL/Gröbner cross-checks. See the dev-oracle policy in the Discoveries log.)

This is stronger than "no optimizations" because it distinguishes mathematics-that-matters-at-scale
from engineering-that-is-orthogonal-to-pedagogy. The former is in scope; the latter is not.

**4. Phenomenology beyond reach — implement the mathematics anyway, and annotate the disconnect.**
Some mathematically real phenomena cannot be *organically* exhibited at the toy scale this project
runs at, precisely because surfacing them naturally would require the engineering scale that
principle 3 puts out of scope. The discipline is: **implement the mathematics regardless, and
explicitly note the gap between the science (the phenomenon) and the means of reaching it (the
out-of-scope engineering / resource scaling).** We never skip a mathematically meaningful phenomenon
merely because toy scale can't trigger it on its own, and we never pretend the toy scale exhibits
something it does not. The disconnect itself is pedagogical content — part of *honestly* surveying
how these algorithms behave at the scales they were designed for.

The gap runs in both directions, and the annotation should say which:
- **Under-exposed at toy scale** — phenomena whose effect only appears at large scale (e.g.
  large-prime variations only paying off once relation yield is in the millions; block-Lanczos
  convergence behaviour that a tiny matrix never stresses). Implemented at demonstration fidelity
  (principle 2), with a note that the *payoff* is unreachable here.
- **Over-exposed at toy scale** — phenomena that real NFS-scale engineering smooths over
  statistically, but that a hand-picked toy instance makes proportionally prominent. **Bad primes**
  (primes p dividing disc(f), where ℤ[α] need not be the full ring of integers ℤ_K and Dedekind's
  theorem does not apply directly — handled via the Dedekind index criterion; see G.A.3) are the
  canonical example: at cryptographic scale their contribution is marginal and largely absorbed by
  polynomial selection, but at toy scale with a hand-picked f they are unavoidable and must be
  implemented and reasoned about head-on. The honest annotation records that this prominence is a
  toy-scale artifact, not the typical NFS-scale picture. (The G.A.W chapter §9 is the template: it
  documents the Round 2 / HNF omission as "engineering scale, not mathematical omission.")

**How the disconnect is annotated.** Wherever such a gap exists it is surfaced in three places that
stay consistent: a marked note in the code (docstring/comment at the relevant function), the
corresponding `PEDAGOGY.md` code-tour, and the Track τ textbook chapter. The textbook is where the
disconnect is explained in full (the science, why toy scale can't reach it organically, what the
NFS-scale picture is); the code and code-tour cross-reference it.

---

## On scale

The disconnect principle (scoping principle 4) rests on a view of *scale* that is worth stating
once, plainly, because it recurs across every track. This section is the clinical statement; the
full exposition — including the part that is genuinely a question of natural philosophy — is owed to
the Track τ textbook (an "On scale" interlude), where it belongs in a register the planning document
should not adopt.

**Scale is not one axis.** Casual talk of "toy scale" vs. "NFS scale" implies a single ladder. It is
not. At least three distinct things travel under the word, and they do not form one ordered line:

1. **Resource / operational scale** — how large an instance one can actually run (toy ≈ 80-bit N on
   a laptop; "NFS scale" ≈ RSA-768/RSA-829 on a cluster-month). This *is* an unbounded ladder
   (bigger N always exists, up to a physical-computation ceiling), but it is **phenomenologically
   flat**: scaling N alone introduces no new mathematics. It makes the same machinery bigger and
   improves the statistics — which is exactly why over-exposed phenomena (bad primes) *wash out* at
   scale rather than changing in kind.
2. **Mathematical-dimension scale** — the degree d = [K:ℚ] of the number field, the embedding degree
   of a pairing, the characteristic-vs-extension shape of a finite field. These are dimensions *of
   the mathematics*, distinct from instance size.
3. **Structural scale** — thresholds where scaling unlocks (or requires) a *different* machine:
   medium/large-prime variations past a yield threshold; the small-characteristic
   quasi-polynomial DLP regime; the asymptotic regime where L-notation heuristics become accurate.

**The three couplings.** Axes 1 and 2 are *sometimes independent, sometimes coupled, sometimes
structural-enabling* — this is the heart of the matter:
- **Independent:** d and N are separate knobs; either can move with the other fixed.
- **Coupled along the efficient frontier:** the *optimal* d for NFS is tied to N by
  d ∼ (3 log N / log log N)^{1/3}, so scaling N "properly" drags d along. Independent as knobs,
  coupled along the frontier. Invisible at toy scale, where optimal d ≈ 3–4 and barely moves.
- **Structural-enabling:** at certain thresholds, scaling one axis changes *which* mathematics
  applies, not just how big it is. This is where new machinery (large-prime variations, the
  quasi-polynomial regime) becomes reachable. The most pedagogically important transitions live
  here.

**On convergence — of the method, not the problem.** A natural question: is NFS asymptotically
convergent, absent new discoveries? The precise answer distinguishes two things the word
"convergence" blurs:
- **Convergence of the method (true).** NFS's heuristic complexity has sat at L_N[1/3, (64/9)^{1/3}]
  since the early 1990s. Three decades of work have improved the *constant* and the *engineering* —
  not the exponent 1/3. Within the NFS paradigm (sieve number-field relations for smoothness, then
  linear algebra) the exponent has converged. Note even this is *heuristic*, resting on unproven
  smoothness assumptions, not a theorem.
- **Convergence of the problem (false / open).** The true complexity of factoring and discrete log
  is **open**. L[1/3] is a believed barrier for the *general* problem, not a proven lower bound, and
  history shows apparent convergence is repeatedly punctured by *structural* discovery: the
  Barbulescu–Gaudry–Joux–Thomé **quasi-polynomial** algorithm (2014) collapsed small-characteristic
  DLP — a regime everyone had taken to be L[1/3] — and Shor's algorithm dissolves the barrier
  entirely in the quantum model (Track ε). The plateau is real but it is a plateau of *one method*;
  it is reset, not approached, by new mathematical structure.

So: NFS scale is not a "toy" relative to a larger operational regime in any *phenomenological* sense
— bigger N is flatter, not richer. The genuinely deep hierarchies (the L-exponent phase changes; the
ramification / arithmetic-geometry tower behind bad primes) are **orthogonal to operational scale**:
reached by choosing richer fields or crossing structural thresholds, not by spending more compute.
The one plausibly-infinite hierarchy is *field-structural* (ramification depth), and a toy instance
over a richly-ramified field can exhibit phenomena a "huge" instance over a tame field never touches.

---

## Scope and effort

| Track | Sessions | Effort | Notes |
|-------|---------:|-------:|-------|
| α — Foundation | 3-5 | 1-2 months | Workspace restructure, shared crates |
| β — GNFS factoring | 16-22 | 6-10 months | Track G: G.A through G.W |
| γ — NFS-DL | 8-10 | 3-5 months | Track D: D.A through D.W |
| δ — Algebraic ECDLP | 25-32 | 9-13 months | Track E: E.A through E.W |
| ε — Shor + PQ | 7-9 | 3-4 months | Track S: S.A through S.D |
| ζ — Umbrella writeup | 2-4 | 1 month | Cross-track integration |
| τ — Mathematical textbook | 2-3 | spread | Track T: spine + bind; chapters paired with `*.W` |
| **Total** | **~72-93** | **23-36 months** | At one session every 3-5 days |

At one session per week (more realistic part-time), 27-42 months. Multi-year commitment by design.

### Progress

*Reconciled at the T.G ◆ boundary (2026-06-08). The Scope table above is the frozen design-time
estimate, kept intact for variance analysis; this subsection tracks actuals against it. Counts are
**commit-shaped sessions** (one commit = one session), the same unit as the Scope table — distinct
from PLANs, where one `docs/PLAN.md` bundles several sessions.*

| Track | Estimate | Done | Remaining | Status |
|-------|---------:|-----:|----------:|--------|
| α — Foundation | 3-5 | ~6 (α.1–α.5 + S0.W backfill) | 0 | **complete** |
| β — GNFS factoring (G) | 16-22 | ~22 (G.A → G.F + G.W) | 0 | **complete** |
| γ — NFS-DL (D) | 8-10 | 0 | 8-10 | not started |
| δ — Algebraic ECDLP (E) | 25-32 | 0 | 25-32 | not started |
| ε — Shor + PQ (S) | 7-9 | 0 | 7-9 | not started |
| ζ — Umbrella | 2-4 | 0 | 2-4 | not started |
| τ — Textbook (T) | 2-3 | 2 (T.0 spine + T.G chapter) | ~1 (T.Z + per-chapter overruns) | spine + Track-G done |
| **Total** | **~72-93** | **~30** | **~43-56** | ~⅓ complete |

**Confirmed-complete spans** (commit-anchored): Phase α through `α.5`/`S0.W`; Track G end-to-end —
G.A (`bdba6f5`…`967e394`), G.B (`2f43f99`…`7fa9ab9`), G.C (`c1dc0b6`…`23a5222`), G.D
(`a0e854b`…`c9f18b9`), G.E (`416f6db`…`f8ca3f8`), G.F (`2af8116`…`e870c82`); and the Track-G-closeout
/ Track-τ-open bundle — T.0 (`5c9b783`), G.W (`76f3633`), T.G (`a896198`). Next up per the
sequencing order: **Track D (NFS-DL)**, opening with the Opus-flagged D.A.1 bridge session.

**Estimation-bias note (inferred, not yet re-baselined):** the per-track *remaining* bands are the
original design-time estimates. The G.C boundary found that demonstration-fidelity sessions run
400–800 LOC (resolving to "own session," not a merge) — a bias toward the **upper** end of each
band. If that pressure holds in Tracks D/E (which carry several demonstration-fidelity sessions),
the realistic remaining count sits nearer the 56 ceiling than the 43 floor. A proper re-baseline is
owed once Track D gives a second data point.

---

## Sequencing

Recommended ordering: **α → β → γ → δ → ε → ζ**.

Reasoning:
- α before everything: substrate must exist first.
- β before γ: NFS-DL is a modification of NFS-factoring, easier to learn as the second pass.
- γ before δ: the MOV bridge in track E (E.C) calls into NFS-DL as a real solver, not a stub. The
  pedagogical payoff only lands if NFS-DL is already real when E.C is implemented.
- δ before ε: Shor's polynomial-time punch lands harder after a deep tour of classical attacks.
- ζ last: the umbrella writeup integrates everything that came before.

**Possible deviation**: track ε (Shor) is fully orthogonal to everything else and can be slotted
anywhere after α. If fatigue with classical attacks builds up, switching to ε is a legitimate move
that doesn't compromise the design.

**Track τ does not sit at a single point in the order — it threads through.** The textbook spine
(T.0) is written early (it sets the register every later chapter obeys); each math chapter is
written at its track's ◆ boundary, paired with that track's `*.W` code-tour session; the final
bind (T.Z) sits at ζ. See the Track τ section for the per-chapter pairing.

---

## Sub-track structure

Each sub-track is named `Letter.SubLetter`. Sessions inside are `Letter.SubLetter.N`. Sub-tracks
are listed below with their inputs (predecessor sub-tracks), outputs (contracts they establish),
estimated sessions, and whether any session inside is Opus-flagged.

### Phase α — Foundation

**S0.1 — Workspace restructure.** 1-2 sessions. No predecessors. Establishes the multi-crate
workspace; extracts shared content from `rho/` into `shared/` crates. All existing `rho` tests must
continue to pass. Sonnet-tier.

**S0.2 — `shared::numth` substrate.** 2-3 sessions. Predecessor: S0.1. Builds primality testing,
smoothness detection, ECM as a standalone factoring sub-step (used inside NFS large-prime
variations and inside Pohlig–Hellman). **Last session is Opus-tier** because the smoothness-trait
interface is consumed by sessions in three tracks (G.C, D.A, E.K) and getting it wrong is costly.

**S0.W — α-substrate integrative writeup (backfill).** 1 session. Predecessor: S0.1, S0.2 (both
complete). Sonnet-tier. **This was not in the original plan and is owed retroactively:** every other
phase has a `*.W` integrative chapter (G.W, D.W, E.W, S.D, Z.1), but Phase α shipped its three
shared crates (`field`, `bigint`, `numth`) with only code-level module docstrings and no integrative
math chapter — despite containing mathematically substantial content (Lenstra ECM stages 1–2 with
Suyama parameterisation and the Montgomery ladder; Miller–Rabin; trial-division smoothness and
`SmoothWitness`; batched inversion; Tonelli–Shanks and the Legendre symbol). The backfill writes
`shared/numth/docs/PEDAGOGY.md` as a code-tour chapter matching the genre and quality of the G.A.W
chapter (`shared/numfield/docs/PEDAGOGY.md`). Its Track τ maths-first sibling pairs at T.0/T.G time.
Drafted at the G.A ◆ boundary (see Discoveries log).

### Phase β — GNFS for integer factoring (Track G)

**G.A — Number field substrate.** 3-4 sessions. Predecessors: α complete. ℤ[α] arithmetic, ideal
representations as ℤ-modules, norm computation, polynomial ring over ℚ with rational
coefficients. **First session is Opus-tier** (substrate design with large downstream consumption).
Subsequent sessions Sonnet.

**G.B — Polynomial selection.** 2-3 sessions. Predecessor: G.A. Kleinjung's algorithm at single-poly
fidelity; Murphy E scoring; root sieve. Demonstration-fidelity Coppersmith multi-poly. KAT: produce
polynomials matching CADO-NFS published examples on RSA-100. Sonnet.

**G.C — Sieving.** 4-5 sessions. Predecessors: G.A, G.B, S0.2 (smoothness). Session order: rational
sieving, algebraic sieving, special-q strategy, line sieving baseline, lattice sieving. KAT:
relation counts match CADO-NFS within tolerance at matched parameters. Sonnet.

**G.D — Filtering.** 2 sessions. Predecessor: G.C. Singleton/clique removal, merging, the graph
view of relations. KAT: matrix dimensions match CADO-NFS. Sonnet.

**G.E — Linear algebra.** 3-4 sessions. Predecessor: G.D. Block Lanczos as primary; block Wiedemann
as secondary. **First session is Opus-tier** — choice of basis representation, blocking strategy,
and the sparse-matrix data layout are decisions that bind subsequent sessions. KAT: kernel vectors
recover the same factorizations as CADO-NFS on small numbers. Sonnet for later sessions.

**G.F — Square root + assembly.** 2 sessions. Predecessor: G.E. Couveignes' algorithm via Chinese
remaindering; final integer GCD. End-to-end KAT: factor a published challenge in the 80-100 bit
range and confirm. Sonnet.

**G.W — GNFS integrative writeup.** 1 session. Predecessor: G.F. The GNFS chapter of PEDAGOGY.md.
**Opus-tier** — this is the moment where the cross-phase contracts get their public articulation
and where the design statement is verified against the actual implementation.

### Phase γ — NFS-DL (Track D)

**D.A — Relation adaptation.** 2 sessions. Predecessors: G.C (sieving), G.A (number fields). What
changes when the target is `log_g(h)` in F_p. Two-number-field setup. Schirokauer maps. **First
session is Opus-tier** — this is the bridge from NFS-factoring to NFS-DL and the contracts
established here are reused in D.B and D.C and consumed by E.C.

**D.B — Linear algebra over F_ℓ.** 2 sessions. Predecessor: G.E (block W/L over GF(2)), D.A. Block
Wiedemann generalised; block Lanczos with the F_ℓ care. KAT: cross-check with PARI's discrete-log
functionality. Sonnet.

**D.C — Individual logarithm + special-q descent.** 3 sessions. Predecessors: D.A, D.B. The
part with no factoring analogue. Special-q descent is mathematically delicate. **First session is
Opus-tier**. Subsequent Sonnet.

**D.W — NFS-DL writeup.** 1 session. Predecessor: D.C. The NFS-DL chapter; explicit comparison
with the NFS-factoring chapter. Sonnet (the contracts already exist in prose from G.W).

### Phase δ — Algebraic ECDLP attacks (Track E)

**E.A — Pohlig–Hellman.** 2 sessions. Predecessor: existing `rho` crate, S0.2 (factoring of group
orders). The cleanest sub-rho attack: reduction to prime-order subgroups via CRT. Sonnet.

**E.B — Pairing arithmetic (Weil, Tate).** 3-4 sessions. Predecessor: G.A (number-field machinery
helps for divisors). **First session is Opus-tier** — pairings are mathematically delicate and the
divisor-arithmetic representation choice bounds E.C. Subsequent Sonnet.

**E.C — MOV/Frey–Rück reduction.** 2 sessions. Predecessors: E.B, D.C (NFS-DL solver). **Both
sessions Opus-tier** — this is *the* cross-track bridge of the entire project. Reduction from
ECDLP on a small-embedding-degree curve to DLP in F_{p^k}, solved by calling into NFS-DL. The
session where the MOV bridge first calls a real NFS-DL solver is the pedagogical climax.

**E.D — p-adic arithmetic.** 3 sessions. Predecessors: shared bigint, G.A (polynomial machinery).
Hensel lifting; p-adic logarithm. Sonnet.

**E.E — Smart–Satoh–Araki.** 2 sessions. Predecessor: E.D. Polynomial-time attack on anomalous
curves (trace = 1). The most surprising single result in classical ECDLP cryptanalysis; the chapter
should make that surprise explicit. Sonnet.

**E.F — GF(2^m) field arithmetic.** 3-4 sessions. No structural predecessor. Polynomial-basis and
normal-basis representations; this is a categorically new field-arithmetic implementation.
**First session is Opus-tier** — substrate decision (basis choice, word layout, multiplication
algorithm: comb, Karatsuba, López–Dahab) has downstream consequences for E.G, E.H, E.I.

**E.G — Binary curves + Koblitz automorphism.** 2-3 sessions. Predecessor: E.F. The order-6 Koblitz
automorphism the existing rho crate explicitly omits. Re-run rho over GF(2^m) curves as baseline
for E.H benchmarks. Sonnet.

**E.H — GHS/Weil descent.** 4-5 sessions. Predecessor: E.G, E.I. Transfer ECDLP on a binary curve
to DLP on a hyperelliptic Jacobian over a subfield. **First session is Opus-tier** — the
descent machinery is the most mathematically intricate single attack in the project.

**E.I — GF(2^m) hyperelliptic Jacobian.** 3-4 sessions. Predecessor: E.F. Cantor's algorithm,
divisor representation, Jacobian group law. Sonnet — well-understood material.

**E.J — Semaev summation polynomials.** 2-3 sessions. Predecessor: G.A (polynomial machinery). The
combinatorial heart of Gaudry–Diem index calculus; mathematically beautiful. Sonnet.

**E.K — Gaudry–Diem–Joux–Vitse index calculus.** 4-5 sessions. Predecessors: E.J, G.B (scoring
methodology), G.E (linear algebra). **First session is Opus-tier**. The Gröbner-basis step
delegates to `msolve` as a dev-only oracle (parallel to CADO-NFS's role).

**E.W — Cross-attack benchmarks + Track E writeup.** 1-2 sessions. Predecessor: most of E. The
"which attack wins on which curve" table; the pedagogical synthesis of structure-based escape from
search. **Opus-tier** — high integrative judgment load.

### Phase ε — Shor + Post-Quantum (Track S)

**S.A — State-vector simulator.** 2-3 sessions. No structural predecessor. Up to ~25 qubits.
Standard gate set; sparse-state optimization. KAT: published small-circuit results. Sonnet.

**S.B — Shor for factoring.** 2 sessions. Predecessor: S.A. Modular exponentiation as a quantum
circuit; factor 15, 21, 35, 91. Sonnet.

**S.C — Shor for ECDLP.** 2 sessions. Predecessor: S.A, curve substrate. Proos–Zalka circuit. Solve
4-bit ECDLP via simulation; cross-check with rho. Sonnet.

**S.D — Post-quantum context writeup.** 1-2 sessions. No structural predecessor. NIST PQC, the
SIDH break, the migration landscape. Prose-only — no PQC implementations. Sonnet (this is a
research-and-write task within an established literature).

### Phase ζ — Umbrella

**Z.1 — Umbrella narrative.** 2-4 sessions. Predecessor: everything. The README + the master
PEDAGOGY.md that ties the whole library together. The L-notation comparison across attacks; the
modularity-theorem speculation chapter; the structure-based-escape-from-search synthesis.
**All sessions Opus-tier** — this is the artifact-as-a-whole and the highest integrative judgment
load in the project.

### Phase τ — Mathematical textbook (Track T)

**Motivation.** The project name "rGNFS" is narrower than the project's intent: the work is a
*survey of discrete-logarithm algorithms, approaches, and applications* (integer factorisation
enters as the DLP's structural sibling, not as the centre). The `PEDAGOGY.md` chapters
(`rho` and the α substrate today; `G.W`/`D.W`/`E.W`/`Z.1` planned) are **code-tours** — organised by
the implementation, assuming the reader already knows the mathematics. Track T adds the complementary
artifact: a **standing mathematical textbook** in the genre of `tetratile/docs/mathematics.rst` —
*maths-first, code-second, learnable on its own*. The two are complementary and cross-reference: each
`*.W` code-tour cites the textbook chapter for the mathematics; each textbook chapter cites the code
for the realisation.

**Scope contract (the register every chapter obeys — frozen as C-Textbook at T.0).**

- **Audience.** The interested mathematics student, human or agent, with an undergraduate maths
  background: comfort with proofs, and a full introductory course each in analysis, algebra,
  probability, and logic. Nothing beyond that is assumed; anything beyond is built up in text or
  cited.
- **Depth: survey with proof-sketch depth.** Every key theorem is *stated and motivated*; proofs are
  *sketches* with citations to the full proof, with complete proofs reserved for the moments where
  the proof is itself the pedagogical payoff (e.g. *why* index calculus is subexponential — the
  L-notation derivation; *why* the MOV reduction works). **Complete** (no key idea silently omitted)
  and **academic and clinical**, but explicitly **not exhaustive** (no encyclopaedic case
  enumeration) and **not inscrutable** (intuition leads, rigour follows).
- **Through-line.** *Structure-based escape from search* — every attack is a story about finding
  exploitable structure (a homomorphism, a smoothness phenomenon, an endomorphism, a pairing, a
  quantum period) that escapes the generic √n / L-notation search bound.
- **Artifact location.** A new standing document, separate from `PEDAGOGY.md` (e.g.
  `docs/MATHEMATICS.md`, or `docs/textbook/` if it grows past single-file scale — decided at T.0).
- **Markup format (recommended at the G.C ◆ boundary; ratify at T.0).** **Markdown with MathJax**
  (`$…$` inline, `$$…$$` display). This supersedes the earlier "rST or Markdown TBD." The driver is
  that the textbook's payoff content — the L-notation subexponentiality derivation, block-Lanczos /
  block-Wiedemann linear algebra, the MOV reduction — needs *true display mathematics* (limits on
  integrals, matrices, multi-line aligned derivations) that the project's current convention of
  inline Unicode glyphs (ℤ[α], 𝔽₂, ∫) in plain Markdown cannot render. MathJax keeps every existing
  `.md` artifact in place (no migration to rST/Sphinx), renders on GitHub and in standard viewers,
  and lets trivial inline glyphs stay as Unicode while non-trivial expressions move to TeX
  delimiters. The same convention is recommended (not mandated) for the `*.W` `PEDAGOGY.md`
  code-tours, so the textbook and the code-tours render consistently. See the
  documentation-format Discoveries-log entry for the full reasoning. Ratification at T.0 is the
  formal freeze (a `tetratile`-style rST/Sphinx path remains the only live alternative and would be
  reopened only if T.0 finds a hard MathJax limitation).

C-Textbook is a documentation-register contract (not a code interface), so it lives here rather than
in the Cross-track contracts section; but it is genuinely cross-track (every chapter obeys it), so a
later chapter that needs to break the register (a topic that truly requires graduate background) must
surface that as a discovery and flex C-Textbook at the next inflection-point review, not silently
raise the level in one chapter.

**Written incrementally, not in one block at the end.** A chapter is most accurate while its
implementation is fresh and its contracts are frozen — the same reasoning that places the `*.W`
code-tours at each ◆ boundary. Track T therefore has a thin spine early, chapters accreted per-track,
and a final bind:

**T.0 — Textbook spine.** 1 session. No predecessor (can run as early as after α; recommended once
the register is felt, i.e. after the `rho` and α-substrate chapters exist and G.A has landed).
Establishes the scope/audience/depth contract above (freezes C-Textbook), notation conventions, the
table of contents across the whole survey, the structure-based-escape-from-search framing chapter,
and a **prerequisites chapter** (the undergraduate-background bridge: the specific theorems from
algebra/analysis/probability/logic later chapters lean on). Includes the "On scale" interlude (the
full natural-philosophy exposition the ROADMAP's `## On scale` section defers here). Also retro-fits
the existing `rho` and α-substrate pedagogy into textbook chapters (or chapters that cite them),
establishing the chapter-pairing pattern. **Opus-tier** — sets the register and through-line that
bind every later chapter; getting the level wrong is expensive to retrofit across all chapters.

**T.G / T.D / T.E / T.S — per-track math chapters.** ~0 net new sessions; **paired with the existing
`G.W` / `D.W` / `E.W` / `S.D` writeup sessions**, not scheduled separately. At each track's ◆
boundary the writeup session produces *two* siblings: the code-tour chapter in `PEDAGOGY.md` (already
planned) **and** the maths-first textbook chapter (new). Pairing keeps mathematics and code-mapping
consistent at the moment both are fresh, and avoids double-counting effort. The textbook chapter is
the larger of the two and may push its paired writeup toward the top of the session-size band (or
split into a dedicated follow-on if it overruns — decided at the boundary). Tier follows the paired
`*.W` session, except where the mathematics is itself a designated payoff (the L-notation
subexponentiality derivation in T.G/T.D; the MOV reduction in T.E), which are **Opus-tier**.

**T.Z — Textbook bind.** 1 session. Predecessor: all per-track chapters + Z.1. Sibling to Z.1: the
final consistency pass binding the accreted chapters into one coherent, learnable document —
cross-references resolved, the comparative L-notation synthesis across all attacks written, notation
unified, the prerequisites chapter reconciled against what the chapters actually used. **Opus-tier**
— highest integrative judgment load in the textbook, paired with Z.1's umbrella synthesis.

**Sequencing note.** T.0 is the only Track-T session that adds calendar time up front; the per-track
chapters fold into existing `*.W` sessions, and T.Z folds against ζ. Net schedule impact is ~2-3
sessions (T.0, T.Z, plus per-chapter overrun allowance), which is why the Scope table lists τ at 2-3
sessions "spread" rather than as a contiguous phase.

---

## Cross-track contracts

Three contracts span tracks; these are the most fragile points in the design and need explicit
management.

### Contract C1 — `shared::numth::is_smooth`

**Defined in:** S0.2. **Consumed by:** G.C (sieving), D.A (relation collection), E.K (index calculus).

The smoothness predicate (and its associated structures for representing a smoothness witness — the
factorisation over the factor base) is reused three times across the project. The trait must
accommodate:
- Integer smoothness with a prime factor base (G.C, D.A).
- Smoothness of points (x, y) on an elliptic curve over F_{p^n} via Semaev polynomials (E.K) —
  semantically different but structurally similar.

The interface should be designed at S0.2 with all three consumers in mind, even though E.K won't
land until much later. This is exactly the "substrate sessions over-specify" rule in action.

**Width policy (prescriptive trigger — decided at the D.A boundary, 2026-06-08).** C1's surface is
hardcoded to `Uint<4>` (256-bit). An impact assessment at the D.A boundary established that this is
**architecturally isolated, not pervasive**: the core number-field/polynomial algebra is `BigInt`
(unbounded, width-independent); factor-base indices are `usize` and exponents `u32`
(width-independent); the relation/matrix contracts (C-Relation, C-Matrix) key on factor-base
*indices*, not integer magnitudes. `Uint<4>` is confined to exactly three sites — `trial_smooth`'s
input, `SmoothWitness::cofactor`, and `norm_to_uint` in `gnfs/sieve/norms.rs` (which carries an
explicit 256-bit **overflow check** — the failure mode is *loud, not silent corruption*). Neither
D.A contract (C-DLRelation, C-Schirokauer) embeds the width in its surface.

*Decision:* the width is **not** widened speculatively. It stays `Uint<4>` while toy scale holds
(per "speculative generality is the greater risk than late refactor"). D.A.1 *confirms-and-records*:
it computes the toy-scale NFS-DL norm bound, asserts it fits 256 bits (a KAT), and a principle-4
annotation in D.W records that this ceiling is an *engineering-scale boundary, not a mathematical
one* (a consumer scaling rGNFS toward real NFS would hit it; the mathematics is unchanged).

*The prescriptive trigger:* **if/when** the `Uint<4>` ceiling becomes constrictive — `norm_to_uint`'s
overflow check fires in a real run, or a consumer legitimately needs to scale past toy N — the
pre-chosen response is the **const-generic widening**: parameterise the three sites over `Uint<L>`
(default `L = 4`), making future scaling a type-parameter change rather than an edit. **This widening
must be executed as its own deliberate, boundary-respecting ROADMAP-then-shard session — never as
spontaneous in-flight scope growth during whatever track first touches the wall.** A `@build` agent
that encounters the overflow mid-session must *halt and surface it as a discovery*, not widen the
type opportunistically. The widening is mechanical and local (three sites; `SmoothWitness::factors`
stays `(u64, u32)` — primes fit in u64 even at NFS scale), so the late-refactor cost is bounded by
design; that boundedness is exactly what licenses deferring it.

*Performance / architecture tradeoffs of the widening (assessed at the D.A boundary — so the future
session inherits them, not rediscovers them):*
- **No toy-path tax.** `Uint<L>` with `L` const-generic and `crypto-bigint`'s monomorphisation means
  `Uint<4>` instantiations compile to byte-identical machine code. Widening imposes nothing on the
  toy regime; it only *relocates the overflow ceiling outward*. The wider-width arithmetic cost
  (O(L²) multiply, limb-linear trial division) is **intrinsic to scaling N** — it would be paid under
  any representation, `BigInt` included — so widening *reveals* that cost, it does not *create* it.
- **Tradeoff 1 — over-wide global.** A single global `L` forces every smoothness consumer to the
  widest width any one needs (e.g. G-track norms at `Uint<8>` would over-pay for E.K's narrower
  Semaev-point smoothness). **Mitigation, and a requirement on the widening session:** make `L`
  *per-call-site instantiable*, not one global alias — that is the whole point of the const-generic
  shape over a hard retype.
- **Tradeoff 2 — const-generic infectiousness.** `trial_smooth<const L>` is infectious upward: callers
  wanting width-polymorphism must also become generic or pin a width. The propagation is *shallow* by
  construction (the `SmoothWitness` result is already width-independent `(u64, u32)` and re-narrows
  immediately), so it stops at `norm_to_uint` + the sieve relation-construction sites — but a careless
  widening could leak `<const L>` across the whole sieve surface. Containing that propagation is
  precisely why this is a *scoped, deliberate* session and not a mid-flight edit.

These tradeoffs *reinforce* the deferral: there is no toy-path cost avoided by widening early and no
contract entanglement created by waiting, so the only rational time to pay the (bounded, contained)
widening cost is when the ceiling actually binds.

### Contract C2 — NFS-DL solver interface

**Defined in:** D.C. **Consumed by:** E.C (MOV bridge).

E.C calls into the NFS-DL solver from D.C. The contract is the function signature `solve_dl(g, h:
F_pk_element, p, k) -> integer` with an associated error type for "cannot solve at this size."
This is the project's most visible cross-track bridge and the pedagogical climax.

E.C should *not* be implemented before D.C is fully landed and KAT-verified. Stubbing E.C against
PARI temporarily during D.* work is acceptable; merging E.C with a PARI stub permanently is not.

### Contract C3 — Polynomial-selection scoring

**Defined in:** G.B (Murphy E for NFS-factoring). **Consumed by:** D.A (NFS-DL polynomial
selection), potentially E.K (factor-base balancing in index calculus).

This contract is *not* extracted to `shared::polysel` in advance. Premature abstraction is the
greater risk than late refactor; the right move is to consolidate after both consumers exist. The
roadmap flags this for a small consolidation session that may be slotted between G.B and D.A or
deferred to ζ.

---

## Opus-flagged sessions

Total Opus-tier sessions across the project: **~14-17** out of ~72-93 total.

| Session | Reason |
|---------|--------|
| S0.2 (last) | C1 substrate design |
| G.A.1 | Number-field substrate |
| G.E.1 | Linear-algebra strategy |
| G.W | Integrative writeup |
| D.A.1 | NFS-factoring → NFS-DL bridge |
| D.C.1 | Special-q descent design |
| E.B.1 | Pairing-arithmetic substrate |
| E.C.1, E.C.2 | MOV cross-track bridge |
| E.F.1 | GF(2^m) substrate |
| E.H.1 | GHS descent design |
| E.K.1 | Index-calculus strategy |
| E.W | Cross-attack synthesis |
| Z.1.* (all) | Umbrella narrative |
| T.0 | Textbook register + through-line (C-Textbook) |
| T.Z | Textbook bind (integrative) |

Plus 5-10 inflection-point Opus reviews at sub-track boundaries — these are not pre-scheduled
sessions but are triggered by discoveries that need static-frame updates.

---

## Discoveries log

Entries added at sub-track boundaries when action-frame work reveals roadmap-frame updates.

### 2026-06 — Track-D / Phase-γ closeout (D.W; D.W ◆ boundary)

The NFS-DL writeup sub-track is complete: D.W.1 (`8c92260`, code-tour → `gnfs/docs/PEDAGOGY.md` §63+)
and D.W.2 / T.D (`541be29`, maths chapter → `docs/MATHEMATICS.md` ch. 9 + the L-notation NFS-DL
payoff). This crosses the **Track-D ◆ boundary** — the NFS-DL arc (D.A → D.B → D.C → D.W) is coherent
and closed, and **Phase γ is complete end-to-end**. Only Track E (the MOV bridge E.C consuming C2)
lies beyond in the δ direction. Three roadmap-frame updates are taken here (D.W.2 ◆ juncture
returned still-on-intent; all five `@plan` confirmation points satisfied).

- **The L-notation NFS-DL payoff landed as a delta on §GNFS, not a re-derivation.** T.D (`MATHEMATICS.md`
  ch. 9 §payoff) establishes that NFS-DL shares the **same** L_N[1/3, (64/9)^{1/3}] complexity as
  NFS-factoring — same exponent 1/3, same constant — with the DL-specific deltas being (a) the
  individual-logarithm special-q descent is **asymptotically subdominant** (a lower-order term, not
  leading-order) and (b) the F_ℓ linear algebra carries the same complexity shape as GF(2). The
  ROADMAP-designated payoff (the DL-vs-factoring asymptotic comparison) is delivered; the rigidity
  guard held (the §GNFS §7 derivation was re-used, not re-derived).

- **Track-D design-statement verification passed on principles 1/3/4** (the G.W §59 analogue for
  NFS-DL). Verdict (recorded in the D.W PLAN action-frame digest): Principle 1 (algorithmic content
  complete — relation adaptation, F_ℓ linalg, special-q descent all implemented head-on) — pass;
  Principle 3 (no engineering optimization crept into D.A–D.C) — pass; Principle 4 (scale-only at
  demonstration fidelity) — pass, with the descent breadth + medium-prime tuning annotated as
  NFS-scale phenomena at demonstration fidelity. No frozen contract was invalidated; no
  additive-reshard was triggered.

- **C2 `solve_dl` is frozen (`9d07c51`, D.C.3) with a recorded F_{p^k} (k > 1) `Unsupported` debt.**
  The cross-track interface E.C will consume is live for k = 1; the k > 1 extension-field path returns
  `Unsupported`. This is an **engineering-/mathematical-dimension-scale boundary, not a toy-scale
  artifact** — the F_{p^k} NFS-DL extension is genuine new mathematics, deferred to a dedicated
  **E.C-prep ROADMAP-then-shard session** (never spontaneous in-flight scope growth during E.C). T.D
  carries this as a principle-4 annotation. *E.C consumers: the MOV bridge first calls a real NFS-DL
  solver at k = 1; the embedding-degree > 1 case waits on the E.C-prep widening.*

### 2026-06 — D.A boundary: dev-oracle policy RESOLVED + CADO-NFS validation-sidecar design statement

Resolves the standing "reference-oracle comparison tests" open question (queued at G.C sharding,
slipped five boundaries — see the now-closed entry below) and adds the build/install + sidecar
design the open item was waiting on. Decided at the Track-D plan-init, as recommended.

**Conceptual model: validation sidecars on the *demonstration* path, never the compute path.**
rGNFS has **no production path** — its runtime is fundamentally *demonstrative/pedagogical* (it
factors toy N and solves toy DL to teach how, not to produce factorizations anyone depends on). This
*dissolves* the principle-3 tension rather than straining it: principle 3 forbids oracles on a
*production* path, and there is none. An oracle is a **validation sidecar** — given the same input
rGNFS just demonstrated on, the sidecar independently confirms rGNFS got the same answer. The oracle
is **never part of how rGNFS computes its answer; only part of how a student gains confidence the
answer is right** (compute-path vs. validation-path — the distinction that keeps it clean).

**Single live reference: CADO-NFS. msieve retired.** The original plan named four scattered oracles
(CADO, msieve, msolve, PARI). Consolidated:
- **CADO-NFS** — the **sole live NFS reference implementation**, and the **designated end-state
  validation sidecar** (see the design statement below). It is the one *living, maintained, serious*
  open NFS implementation; it covers **every** Track-G stage (poly-select, sieve, filter, linalg,
  sqrt) and **also does NFS-DL** (`cado-nfs-dl`), so it complements PARI in Track D rather than
  needing a separate factoring oracle.
- **msieve — RETIRED from the live-sidecar plan.** It is unmaintained (~2015, may need build
  patching against modern GMP/compilers), does **not** do NFS-DL, and its originally-assigned stages
  (G.E linear algebra, G.F square root) are **already complete and KAT-verified against published
  values**. Marginal remaining benefit (a second independent linalg implementation) does not justify
  carrying a dead build dependency. Retained only as an *optional historical* cross-check note for
  G.E/G.F, not a maintained dependency. (Supersedes the line-694 msieve assignment.)
- **PARI/GP** — lightweight DL cross-check (Track D), unchanged. Small, instant at toy scale.
- **msolve** — Track-E Gröbner oracle (E.K), unchanged; the one genuinely distinct tool (and the one
  with unbounded memory appetite — assess at E.K, not here).

**Gating policy (project-wide, uniform).** Oracles are **absent-by-default, opt-in, skip cleanly.**
Every oracle KAT is `#[ignore]`/feature-gated (`--features oracle-tests`) and skips cleanly when the
binary is not found; the deterministic non-oracle KATs *always* carry the reproducibility burden
(matching the established G.C CADO pattern). A demonstration sidecar mode is explicit opt-in (flag /
env var), never automatic. **Nothing ever fails, hangs, or behaves environment-dependently because an
oracle is uninstalled** — the student-with-no-setup path is always green. Dynamic/auto-install is
*not* adopted (principle 3 disfavours it); install is a deliberate contributor step.

**Consumer-hardware / student-budget viability (the question that prompted this).** At the scales
rGNFS targets (toy, ~80–100-bit N on a laptop), both candidate sidecars run **comfortably on
student-grade hardware with zero hosted/provisioned resources** — the hosted-cluster cost that haunts
NFS applies only at *cryptographic* scale, which principle 3 puts out of scope. The friction is
*build-time*, not runtime or RAM:
- **msieve** (were it kept): trivial — small self-contained C build (~1–2 min), sub-second runs, a
  few MB RAM. The *most* comfortable, but retired for the maintenance reasons above.
- **CADO-NFS**: viable but heavier — large CMake C/C++/Python codebase, **several-to-~15 min** build,
  hundreds of MB disk, needs a Python interpreter; at toy scale runtime is a few seconds dominated by
  orchestration overhead, not math. No budget concern, no provisioning. *Build cost is the only real
  friction; confirm exact times on first install per OS (Linux easiest; Windows is CADO's weak spot).*
- **Honest caveat (principle-4-adjacent):** this comfort is *itself* a toy-scale artifact — the same
  tools on the same hardware would be unusable at the cryptographic scales they were built for. Worth
  a one-line annotation wherever the sidecar is documented.

**Build/install detail lives in README/CONTRIBUTING, not here.** This ROADMAP entry fixes the
*policy*; the contributor-facing "dev oracles" how-to (the actual clone/cmake/make invocations) is
written into README/CONTRIBUTING when CADO is first wired live, so the durable roadmap does not carry
a staleness-prone command reference.

### CADO-NFS validation sidecar — destination design (end-state goal)

*Not implemented now; this is the stated destination, built incrementally.* When rGNFS is complete,
**CADO-NFS is its designated full-pipeline validation oracle**: given any instance rGNFS
demonstrates, the CADO sidecar provides **behavior calibration** (do rGNFS's intermediate
quantities — relation counts, matrix dimensions, kernel structure — match a serious implementation's
at matched parameters?) and **results validation** (does the final factor / discrete log agree?).

The path to this end state is *already being walked* by the per-stage cross-checks: G.B matches CADO
published Murphy-E examples, G.C matches CADO relation counts within tolerance, G.D matches CADO
matrix dimensions, G.E recovers the same factorizations — each today against *published* CADO values,
not a live run. The end-state sidecar replaces "published values" with a *live, opt-in* CADO
invocation over the same input, closing the loop into a single demonstration-time "rGNFS says X;
CADO confirms X" experience. Incremental, opt-in, off the compute path — the destination, not a
near-term deliverable.

### 2026-06 — D.A boundary: C1 `Uint<4>` width decision resolved (confirm-record + prescriptive widening trigger)

The α-boundary deferred the C1 `Uint<4>` → `Uint<L>` widening decision to "G.C or D.A." Resolved at
the D.A sharding boundary. An impact assessment (prompted by the question "what is the impact of
`Uint<4>` if a consumer scaled past the toy regime?") established that the width is **isolated at
the smoothness boundary, not a deep/multifarious design commitment**: core algebra is `BigInt`,
contracts key on factor-base indices, and only three sites touch the fixed width — one of which
(`norm_to_uint`) already guards with a loud overflow check. *Verdict:* **no widening now**
(confirm-and-record at D.A.1 + a D.W principle-4 annotation that the ceiling is engineering-scale,
not mathematical). A **prescriptive widening trigger** is now recorded in the C1 contract subsection:
if/when the ceiling binds, the const-generic widening is the pre-chosen response, executed as its own
disciplined ROADMAP-then-shard session — never as spontaneous in-flight scope growth. See **Contract
C1 → Width policy** for the full statement. This is a *static-frame policy* decision; it blocks
nothing in D.A.

### 2026-06 — Track-G closeout + Track-τ open (T.0, G.W, T.G; T.G ◆ boundary)

The Track-G-closeout / Track-τ-open bundle is complete: T.0 (`5c9b783`), G.W (`76f3633`), T.G
(`a896198`). This crosses the **Track-G ◆ boundary** — the GNFS arc (G.A → G.W → T.G) is coherent
and closed. Three roadmap-frame updates are taken here.

- **Roadmap-frame flex resolved: T.0 runs *before* the G.W↔T.G pairing (additive).** The ROADMAP
  Phase τ scope contract folds T.G into G.W "at the G ◆ boundary," assuming C-Textbook is already
  frozen. It was not — no textbook artifact existed. The G.W↔T.G pairing is therefore *blocked on
  T.0*. The G.F-boundary PLAN resolved this by **running T.0 first**, then the paired G.W + T.G, as
  a 3-session bundle. This is **additive** (sequence the spine ahead of its first consumer; no
  contract break) and **answers the open question** flagged at the G.A boundary ("whether T.0 runs
  early or is deferred until more chapters exist to calibrate the register"): T.0 ran early, and the
  register calibrated cleanly against the existing rho/α retrofit content. The "T.0 is the only
  Track-T session that adds calendar time up front" prediction held — T.G folded into G.W as
  planned, net-new cost was T.0 alone.

- **C-Textbook frozen (`5c9b783`).** The cross-track documentation-register contract — audience
  (undergraduate maths background), depth (survey with proof-sketch depth: complete and clinical,
  not exhaustive, not inscrutable; full proofs only at designated payoffs), through-line
  (structure-based escape from search), markup (**Markdown + MathJax**, ratifying the G.C-boundary
  recommendation), and artifact location (**`docs/MATHEMATICS.md`**, single-file; promotion to
  `docs/textbook/` deferred to T.Z). This supersedes and closes the "rST or Markdown TBD" and the
  "`docs/MATHEMATICS.md` vs `docs/textbook/`" open items from the G.A-boundary Track-τ entry. The
  markup ratification also closes the documentation-format Discoveries entry below (recommended at
  G.C ◆, now frozen): no hard MathJax limitation surfaced.

- **G.W design-statement verification passed on all three scoping principles.** The ROADMAP names
  G.W as "where the design statement is verified against the actual implementation." Verdict
  (recorded in the PLAN action-frame digest): Principle 1 (algorithmic content complete) — pass;
  Principle 3 (no engineering optimizations crept in) — pass; Principle 4 (scale-only at
  demonstration fidelity) — pass, with one principle-4 *over-exposed* annotation: bad primes are
  prominent at toy scale (documented in G.W §59), a toy-scale artifact, not a divergence. No frozen
  contract was invalidated; no additive-reshard was triggered. The L-notation derivation of
  L_N[1/3, (64/9)^{1/3}] landed complete (G.W §60; T.G full payoff proof). One stale-doc cleanup:
  the `gnfs/src/lib.rs` `sqrt`-module "stub" docstring was corrected.

### 2026-06 — End of sub-track G.F (square root + assembly, G.F ◆ boundary)

G.F is complete: G.F.1 (`2af8116`), G.F.2 (`11f9065`), G.F.3 (`c80a855` + review fix `ec69a1f`),
G.F.4 (`7f80040`), G.F.W (`f7ebe1d`). **The GNFS factoring pipeline proper (G.A → G.F) is now
complete end-to-end**: the G.F.4 assembly + end-to-end factor driver closes the arc from N to a
recovered factor. **C-AlgSqrt frozen** (`c80a855` + `ec69a1f`) — the Couveignes CRT algebraic
square root, D.B-consumable. One substrate extension: C-NF was additively extended at G.F.1
(`reduce_mod_ideal` frozen, `20cd263`). No roadmap-frame contract changes; the boundary was
still-on-intent.

### 2026-06 — End of sub-track G.E (linear algebra, G.E ◆ boundary)

G.E is complete: G.E.1 (`416f6db`), G.E.2 (`5145d4c`), G.E.3 (`19936a7`), G.E.W (`a985965`). Block
Lanczos landed as the primary GF(2) nullspace solver, block Wiedemann as the secondary — matching
the ROADMAP's "block Lanczos as primary; block Wiedemann as secondary" prescription. **C-LinAlg
frozen** (`416f6db`) — the GF(2) nullspace substrate (`BlockVec`, `MatrixOperator`, `KernelVector`,
QC columns), consumed by G.F and re-consumed by D.B (block W/L over F_ℓ). The G.E.1 substrate-design
session was Opus-tier as flagged. Boundary still-on-intent.

### 2026-06 — End of sub-track G.D (filtering, G.D ◆ boundary)

G.D is complete: G.D.1 (`a0e854b`), G.D.2 (`d424f53`), G.D.W (`7762339`). Singleton removal,
clique/excess pruning, and 2-way-then-k-way merging landed with the graph view of relations.
**C-Matrix frozen** (`a0e854b`) — the filtered sparse GF(2) matrix + provenance map, threading the
relation→matrix provenance forward to G.F's square-root stage. The documentation math-rendering
format discovery (MathJax recommendation, below) was first logged at the G.D plan-init (`d05f26f`)
and carried to the G.C-boundary entry. Boundary still-on-intent; no roadmap-frame contract changes.

### 2026-06 — Documentation math-rendering format: Markdown + MathJax (recommended at G.C ◆)

*Provenance:* raised at the G.C ◆ boundary while setting up the next sub-track. *Finding:* every
documentation artifact in the project today (`docs/PEDAGOGY.md`, `gnfs/docs/PEDAGOGY.md`,
`shared/numfield/docs/PEDAGOGY.md`, the ROADMAP/PLAN themselves) renders mathematics as **inline
Unicode glyphs in plain Markdown** — `ℤ[α]`, `𝔽₂`, `∫`, `≡`, subscripts via Unicode digits. There
are **no math delimiters anywhere** in the tree (no `$…$`, no `$$…$$`, no `.. math::`, no fenced
math blocks). This is adequate for short inline expressions and reads fine as plain text, but it
**cannot render true display mathematics**: integrals with limits, matrices, multi-line aligned
derivations, large fractions, sub/superscript stacks.

*Why it matters now / downstream:* the project's heaviest mathematics is still ahead — the
L-notation subexponentiality derivation (G.W/T.G, T.D), block-Lanczos / block-Wiedemann linear
algebra (G.E, D.B), the MOV reduction (E.C/T.E), and the whole Track τ textbook (modelled on
`tetratile/docs/mathematics.rst`, a maths-first artifact). These are exactly the chapters where
display math is load-bearing pedagogy, not decoration. Deciding the format *now*, while the doc
corpus is still small, avoids a later migration across many chapters.

*Decision (recommended; formal freeze at T.0):* **Markdown + MathJax** — `$…$` for inline,
`$$…$$` for display. Rationale, naming the tradeoff explicitly:
- **Wins:** zero migration (every artifact stays `.md`); renders on GitHub and in standard Markdown
  viewers without a build step; trivial glyphs can stay Unicode while only non-trivial expressions
  take TeX delimiters; the TeX source is the same notation the textbook would use under any tooling.
- **Loses (vs rST + Sphinx):** no native cross-reference/`:math:`-role machinery, no Sphinx
  numbering/indexing, and no enforced build-time math validation — MathJax fails silently in a
  renderer rather than erroring at build. The `tetratile`-style rST/Sphinx path is the one live
  alternative and buys those features at the cost of a build toolchain and an rST migration of the
  existing `.md` corpus.

*Scope of the recommendation:* applies to the Track τ textbook (supersedes the earlier "rST or
Markdown TBD" in the Phase τ scope contract) **and** is recommended — not mandated — for the `*.W`
`PEDAGOGY.md` code-tours, so textbook and code-tours render consistently. Existing chapters need not
be retrofitted wholesale; new display math uses MathJax, and inline-Unicode passages can be migrated
opportunistically.

*Status:* **FROZEN at T.0 (`5c9b783`).** Ratified as part of the C-Textbook freeze (see the
Track-G-closeout Discoveries entry above) — no hard MathJax limitation surfaced. This was a
*static-frame documentation-tooling* call, not a code contract; it blocked nothing in G.D
(filtering produced no prose math beyond its `*.W` chapter). Reopen only if a later track must
target a renderer that lacks MathJax.

### 2026-06 — Open question queued during G.C sharding (reference-oracle comparison tests) — RESOLVED at D.A boundary

**RESOLVED (D.A boundary, 2026-06-08) — see the "dev-oracle policy RESOLVED + CADO-NFS
validation-sidecar design statement" entry at the top of this log.** Outcome in brief: CADO-NFS is
the single live reference (msieve retired); oracles are absent-by-default / opt-in / skip-clean
validation sidecars on the demonstration path; build/install how-to goes to README/CONTRIBUTING; the
sidecars run comfortably on student-grade hardware at toy scale (build-time is the only friction).
The original framing is retained below for provenance.

Surfaced while sharding G.C (sieving): the ROADMAP names four external reference implementations as
dev-only correctness oracles, scattered across tracks — **CADO-NFS** (G.C relation counts, G.B
Murphy-E), **msieve** (cross-check for G.E linear algebra / G.F square root), **msolve** (E.K
Gröbner step), and **PARI** (D.B discrete-log cross-check). Principle 3 fixes their *role* (oracles,
never on a build/production path) but the project has not yet decided *how* a contributor obtains
them or how the oracle-gated tests behave when they are absent. The current G.C PLAN handles CADO
locally (the comparison KAT is feature-gated / `#[ignore]`d and skips cleanly when CADO is absent;
the deterministic relation-count KAT carries the reproducibility burden without it), but that is a
per-test workaround, not a project policy.

**Open — decide at the next sub-track boundary (G.C ◆ or the next inflection):** the project-wide
oracle-comparison-test strategy. Candidate shapes, not yet chosen:
- A single contributor-facing "dev oracles" doc section (README/CONTRIBUTING) listing each oracle,
  what it cross-checks, and how to install it locally — with all oracle-gated tests behind a
  uniform feature flag (`--features oracle-tests`) or `#[ignore]` convention that skips cleanly.
- Whether "dynamic install" of any oracle is ever acceptable, and if so only in a contributor-local
  dev step explicitly fenced off the build/CI path (principle 3 strongly disfavours this).
- Whether further NFS-topic reference implementations beyond the four named will be wanted (e.g. for
  filtering, square root), which would widen the same policy.

This is a *static-frame* documentation/policy question, not a code contract; it does not block G.C
(the per-test gating already works). Resolve it once, project-wide, rather than re-deciding per
oracle.

**Carry-forward history (now closed):** the item slipped four boundaries (G.D, G.E, G.F, T.G) on the
per-test workaround — non-blocking throughout because the `#[ignore]`/feature-gating held — and was
**resolved at the fifth (D.A boundary)** as recommended, before Track-D PARI cross-checks are
written. See the resolution entry at the top of the log.

### 2026-06 — End of sub-track G.C (sieving, G.C ◆ boundary)

G.C is complete: G.C.1 (c1dc0b6), G.C.2 (c4e5fc4), G.C.3 (0ef7231), G.C.4 (a7f9551), G.C.W
(23a5222). Two implementation findings are logged for future sessions.

- **Demonstration-fidelity "merge/fold" sessions tend to run 400–800 LOC, not <150.** The G.C.4
  lattice-sieving conditionality ("decide at G.C.3 close whether to merge into G.C.3 or fold into
  G.C.W if it lands under ~150 LOC") resolved to "own session" at 741 LOC — the same outcome as
  G.B.4 Coppersmith (also its own session). The ~150 LOC threshold in the PLAN was calibrated for
  a *trivial* demonstration (a few lines of construction + a KAT), but demonstration-fidelity
  lattice/Coppersmith sessions include: the mathematical construction, Gauss/LLL reduction, lattice
  enumeration, principle-4 annotations, and a full KAT suite. Future sub-track plans should treat
  demonstration-fidelity sessions as full sessions (150–400 LOC floor) rather than merge candidates
  unless the algorithm genuinely reduces to a wrapper over an existing primitive.

- **Log-sieve threshold calibration at toy scale: use `threshold_scale × log₂(B_alg)`, not
  sum-of-all-logs.** The natural threshold for the log-sieve (sum of log contributions for a fully
  smooth number) is approximately `Σ log p` over the factor base — but at toy scale, individual
  norms are small (e.g. `|N_rat| = 3`, contributing only `log₂(3) ≈ 1.58`), so the sum-of-all-logs
  threshold is far too high and admits no candidates. The working threshold is a fraction of
  `log₂(B_alg)` (the algebraic smoothness bound alone), scaled by a `threshold_scale` parameter
  (default 0.5–1.0). This is a recurring toy-scale calibration issue: the sieve's asymptotic
  threshold derivation assumes norms of size `exp(B^{1/u})`, which does not hold at toy scale.
  Future sieve sessions (G.D, any sieve-adjacent work) should document this calibration explicitly
  and expose `threshold_scale` as a tunable parameter rather than hardcoding the asymptotic formula.

### 2026-06 — End of sub-track G.B (polynomial selection, G.B ◆ boundary)

G.B is complete: G.B.1 (2f43f99), G.B.2 (00aa32d), G.B.3 (3e2ba1b), G.B.4 (c115a1b), G.B.W
(7fa9ab9). Two implementation findings are logged for future sessions.

- **`base_m_for_degree` requires `floor(N^{1/(d+1)}) + 1`, not `floor(...)`.** The floor of the
  (d+1)-th root gives the largest `m` with `m^{d+1} ≤ N`, meaning N *overflows* d+1 base-m digits
  (the leading digit `a_d` equals `m`, making the expansion d+2 digits). Adding 1 gives the
  smallest `m` with `m^{d+1} > N`, guaranteeing N fits in exactly d+1 digits with `a_d < m`. This
  is counterintuitive — "floor of the root" sounds right — and the bug is silent (the expansion
  still satisfies `f(m) = N`, but the degree is wrong). Any future base-m implementation should
  verify with a round-trip test that checks `f.degree() == d`, not just `f(m) == N`.

- **Dickman ρ RK4 integration must pre-compute the full table; a rolling window silently corrupts
  values for u > 2.** The recurrence `ρ(u) = (1/u) ∫_{u-1}^{u} ρ(t) dt` requires `ρ(t-1)` for
  the lookback integral at each step. A rolling window that overwrites old values as it advances
  destroys the lookback values before they are consumed, producing plausible-looking but wrong
  results (the function remains positive and decreasing, so the error is not obvious). The fix is a
  pre-allocated `Vec<f64>` covering the full range. Any future Dickman ρ or similar one-unit-
  lookback recurrence should use a full table, not a sliding buffer.

### 2026-06 — End of sub-track G.A (Opus inflection-point review at the G.A ◆ boundary)

G.A is complete: G.A.1a (bdba6f5), G.A.1b (05b27c8), G.A.2 (bcd63cd), G.A.3 (7844773), G.A.4
(2da009b), G.A.W (967e394). Four roadmap-frame updates were taken at this boundary.

- **Track τ (mathematical textbook) added.** A complementary, maths-first, self-contained survey of
  discrete-logarithm algorithms, modelled on `tetratile/docs/mathematics.rst`, distinct from the
  code-tour `PEDAGOGY.md` chapters. See the Phase τ section for the full scope contract (C-Textbook).
  Three genre decisions, not to be re-litigated absent new information: (1) the textbook is a
  *separate* artifact; the `*.W` chapters stay code-tours and cross-reference it. (2) Depth is
  *survey with proof-sketch depth* — complete and clinical, not exhaustive, not inscrutable; full
  proofs only where the proof is the payoff. (3) The audience floor is a *full undergraduate maths
  background* (proofs + intro analysis/algebra/probability/logic). Chapter-pairing is the
  effort-neutral move: per-track math chapters pair with existing `*.W` writeups; only T.0 and T.Z
  add net sessions. Open at next inflection point: artifact location/format
  (`docs/MATHEMATICS.md` vs `docs/textbook/`; ~~rST vs Markdown~~ — **format resolved at the G.C ◆
  boundary: Markdown + MathJax; see the documentation-format Discoveries entry**) and whether T.0
  runs early or is deferred until more chapters exist to calibrate the register against.

- **Scoping principle 4 added (phenomenology beyond reach), prompted by bad-prime apprehension at
  toy scale.** *Provenance:* during G.A action-frame work a `@build` agent was apprehensive about
  handling bad primes (p | disc(f)) at toy scale; a juncture agent relayed the concern. This
  surfaced a gap: the three-way split said what to *implement* but not what to do when a real
  phenomenon can't be *organically* exhibited at toy scale (or, as with bad primes, is *over*-
  exhibited relative to the NFS-scale picture). *Resolution:* principle 4 — implement the
  mathematics regardless, never pretend toy scale exhibits what it doesn't, annotate the
  science↔engineering disconnect (code + code-tour + Track τ textbook), naming both directions
  (under- vs over-exposed). *On bad primes specifically:* they are a *correctness* phenomenon, not a
  performance one. ℤ[α] is generally a subring of ℤ_K; they differ exactly at primes dividing the
  index [ℤ_K : ℤ[α]], all of which divide disc(f). The G.A.3 implementation already handles the
  linear-factor case for both good and bad primes and flags `index_divisible`; the G.A.W chapter §9
  documents the Round 2 / HNF omission as "engineering scale, not mathematical omission" — the
  template for principle-4 annotations. The `@build` apprehension is legitimate but resolved: bad
  primes are *more* prominent at toy scale (e.g. disc(x²−2)=8, so p=2 is bad), not absent.

- **`## On scale` section added.** A clinical statement of the scale model behind principle 4 (three
  axes — resource/operational, mathematical-dimension, structural; the three couplings between
  field-degree and instance size; method-convergence vs problem-openness of NFS complexity). The
  full natural-philosophy exposition is deferred to a Track τ "On scale" interlude (T.0).

- **S0.W α-substrate writeup owed (backfill).** Audit at the G.A boundary found Phase α shipped its
  three shared crates (`field`, `bigint`, `numth`) with only code-level docstrings and **no
  integrative math chapter** — unlike every other phase (`G.W`/`D.W`/`E.W`/`S.D`/`Z.1`). The α
  substrate contains mathematically substantial content (Lenstra ECM stages 1–2, Suyama, Montgomery
  ladder; Miller–Rabin; smoothness + `SmoothWitness`; batched inversion; Tonelli–Shanks/Legendre)
  that deserves a chapter. Added S0.W (Phase α, Sonnet, 1 session) to write
  `shared/numth/docs/PEDAGOGY.md` matching the G.A.W chapter's genre and quality. Drafted at this
  boundary.

### 2026-05 — End of Phase α (after α.1, α.2, α.3; reviewed at α.4)

- **Contract C1 is currently hardcoded to `Uint<4>`.** `shared::numth`'s entire surface
  (`miller_rabin`, `is_prime`, `trial_smooth`, `SmoothWitness`, `ecm_factor`) operates on
  `Uint<4>`. The factor type in `SmoothWitness::factors` is `(u64, u32)`. This is sufficient for
  the two imminent consumers (G.C, D.A — toy-scale NFS sieving where 256-bit norms suffice) but
  the trait was specified at α.2 to accommodate three consumers including E.K. G.A.1 will revisit
  the contract once number-field arithmetic gives concrete bit-widths for NFS norms; widening to
  `Uint<L>` and parameterising `SmoothWitness` over a Factor type are both mechanical changes
  that can be done in-place at G.A.1 or G.C.1.

  Decision deferred per the principle "speculative generality is the greater risk than late
  refactor" (see `multi-session-planning.md` on Contract C3). The cost of refactoring `Uint<4>`
  → `Uint<L>` at G.C is bounded; the cost of designing the witness shape now without a real
  second consumer is unbounded.

- **`Fp` trait is missing `legendre` and `sqrt`.** Deferred at α.1 because Tonelli–Shanks
  requires primality testing, which only existed after α.2. Now that `shared::numth::is_prime` is
  available the dependency loop is resolved. Will be added in α.5 (patch session) so that G.A.1
  doesn't have to amend the trait while also doing number-field substrate design.

- **`rho::curve` was NOT lifted to `shared::curve` in α.3.** Deliberate: ECM uses Montgomery-form
  curves (Suyama parameterization), `rho` uses short Weierstrass. No shared substrate exists yet.
  Revisit at E.B.1 when pairings need divisor-arithmetic curves.

- **`factor_base_up_to` is O(B·sqrt(B)) via repeated primality tests.** Acknowledged TODO; not
  bottlenecking at α.2's scale (`B ≤ 400` in tests). Replace with sieve of Eratosthenes when G.C
  reveals a real bottleneck.

### Opus-flagged sessions table addendum

- **α.5** added as a single mechanical Sonnet session (not Opus). It is a patch-up session for
  `Fp` trait completion (`legendre`, `sqrt`), not a design session.

---

## Updates to this document

This document is rewritten **only at sub-track boundaries**. Day-to-day session work is captured
in `docs/PLAN.md`. Discoveries that affect this document are queued in the Discoveries log above
and integrated at the next sub-track boundary.

If a discovery is severe enough to require immediate roadmap revision — e.g., a substrate design
that turned out to need a new contract — that triggers an inflection-point Opus session, not an
ad-hoc edit to this document by a Sonnet session.
