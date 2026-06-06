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
| **Total** | **~70-90** | **23-35 months** | At one session every 3-5 days |

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

Total Opus-tier sessions across the project: **~12-15** out of ~70-90 total.

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

Plus 5-10 inflection-point Opus reviews at sub-track boundaries — these are not pre-scheduled
sessions but are triggered by discoveries that need static-frame updates.

---

## Discoveries log

Entries added at sub-track boundaries when action-frame work reveals roadmap-frame updates.

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
