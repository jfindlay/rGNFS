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
   explicitly omitted. CADO-NFS and msieve serve as dev-only correctness oracles, never on a
   production path.

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
  rST or Markdown TBD at T.0 to match the project's doc tooling.

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
  (`docs/MATHEMATICS.md` vs `docs/textbook/`; rST vs Markdown) and whether T.0 runs early or is
  deferred until more chapters exist to calibrate the register against.

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
