<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Phase β, sub-track G.E (Linear algebra)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **reverses the G.D `sonnet` opt-down back to the default.**
The reversal is principled, not a regression: G.D opted down because its only frozen contract
(C-Matrix) was *internal to Track G* and a filtering bug surfaced one sub-track downstream as a wrong
matrix dimension (levers 3/4 moderate, recoverable). G.E is the opposite case. Applying the five-lever
law: lever 2 (irreducible complexity) is **high** — block Lanczos over GF(2) carries the
self-orthogonality problem (a nonzero GF(2) vector can be orthogonal to itself under the bilinear
form, unlike over ℝ), which is a genuine FLOOR that cannot be fractured below one working solve;
lever 3 (design-error cost) is **high** — G.E.1 reaches into the frozen `obstruction_count` slot of
C-FactorBase (the quadratic-character columns) **and** freezes C-LinAlg, whose representation is
re-consumed cross-track by D.B (NFS-DL linear algebra over F_ℓ, ROADMAP Phase γ); lever 4
(correctness-criticality) is **high** — a wrong kernel vector silently yields a non-factorization at
G.F, the deepest correctness seam in the GNFS pipeline. Levers 2/3/4 all high is exactly the case the
planning doc names as **holding the adjudicator at the Opus default**. Lever 5 (inner-loop bandwidth)
is strong (`cargo test --workspace` + deterministic-kernel KATs + the CADO end-to-end oracle) but
does **not** license opting down here, because the opt-down condition is "strong tests *coinciding
with* lower correctness-criticality" — and G.E's correctness-criticality is high. The ROADMAP
independently flags **G.E.1 as Opus-tier** (Opus-flagged sessions table); G.D had no Opus session.

The high lever-4 silent-failure risk drives a **second Opus juncture** beyond the G.E.1 design
inflection: a dedicated **correctness-review juncture after G.E.2** (block Lanczos, the lever-2 FLOOR
session). Because a Lanczos bug is silent until G.F (a plausible-but-wrong kernel, not a red test) and
the CADO end-to-end oracle may be absent, the strongest model reviews the landed solver against the
code + action-frame digest before G.E.3 builds on it. This is the juncture instrument the planning doc
sanctions (paged T0 fork, one-shot, reads the digest, does not implement), sited by the cost-of-wrong
of a specific high-risk session rather than by a sub-track boundary.

Last rewrite: G.D ◆ boundary crossed (G.D.W landed at `7762339`; G.D ledger still-on-intent
2026-06-07). G.D fully complete (G.D.1 → G.D.W); C-Matrix frozen, C-FactorBase / C-Relation stable.
This plan opens sub-track G.E — the **linear algebra** stage of the GNFS pipeline — over the frozen
filtered-matrix substrate. G.E reads the C-Matrix `SparseMatrix` and computes the GF(2) nullspace
whose vectors are congruences of squares that G.F turns into a factorization.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.E) builds **GNFS linear algebra** — given the compact, full-rank
**sparse GF(2) matrix** from G.D filtering, find vectors in its **left nullspace**: a subset of
(filtered) rows whose GF(2) sum is zero, i.e. a set of relations whose combined rational and
algebraic norms are each perfect squares. Each nullspace vector is a **congruence of squares**, the
object G.F's square-root step turns into a factor of N. It comprises:

1. **The linear-operator substrate (G.E.1).** A blocked GF(2) vector representation and the view of
   the C-Matrix `SparseMatrix` as a linear operator (sparse matrix–block-vector product), plus the
   **quadratic-character columns** that guarantee the algebraic-side square root exists in K (not
   merely that the algebraic norm is a square in ℤ). This is the substrate both solvers sit on, and
   the seam D.B (NFS-DL over F_ℓ) re-consumes.
2. **Block Lanczos (G.E.2, primary).** The Montgomery block-Lanczos iteration over GF(2): build an
   A-orthogonal basis of the Krylov subspace, handling the self-orthogonality that GF(2)'s bilinear
   form admits (the winnowing of which block columns advance each step), and extract the kernel.
3. **Block Wiedemann (G.E.3, secondary).** The Coppersmith block-Wiedemann alternative: a matrix
   Krylov sequence, Berlekamp–Massey to find a linear generator, and kernel extraction from the
   generator. A second, orthogonal solver over the same C-LinAlg substrate — the ROADMAP's
   "secondary" path, useful as a cross-check and the natural generalisation target for D.B.

This is the fourth stage of the GNFS pipeline proper: it sits *on* the G.D filtered matrix (C-Matrix)
and the factor-base column layout (C-FactorBase), and it *produces* the nullspace-vector representation
(C-LinAlg) that G.F (square root) consumes to form the congruence of squares.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating beyond block Lanczos +
block Wiedemann at demonstration fidelity — Montgomery's full cache-blocking, the GF(2)-word SIMD
packing, MPI-distributed Wiedemann, and out-of-core matrix streaming are out of scope, ROADMAP
principle 3) and **rigidity** (grinding through a kernel representation that G.F or D.B later shows is
wrong, rather than surfacing it at the boundary).

**Scoping discipline (ROADMAP three-way split, applied to G.E).** Algorithmic content complete: block
Lanczos with self-orthogonality handling, block Wiedemann with Berlekamp–Massey, and the
quadratic-character obstruction columns — the full GF(2) nullspace machinery. **Scale-only** content
(block width tuned to machine word size, Montgomery's multi-block cache strategy, large-matrix
convergence behaviour a tiny matrix never stresses) is present at **demonstration fidelity** (the
blocking is real even where its speedup doesn't show at toy scale — principle 2, disconnect annotated
per principle 4). **Engineering optimizations** (SIMD GF(2) word packing, NUMA, MPI distribution,
out-of-core streaming) are omitted. **CADO-NFS / msieve are dev-only correctness oracles** for the
end-to-end "kernel recovers the same factorization" cross-check — never on a build path.

---

## Current state

Phase α + sub-tracks G.A, G.B, G.C, G.D complete. Workspace crates: `shared/field`, `shared/bigint`,
`shared/numth`, `shared/numfield`, `rho`, `gnfs`. `cargo test --workspace` green at `7762339`.

The `gnfs` crate carries `polyselect/` (G.B), `sieve/` (G.C), and `filter/` (G.D). G.E adds a sibling
`linalg/` module (no linalg module exists yet — clean greenfield, the third non-`polyselect`/`sieve`
module after `filter/`).

Substrate G.E consumes (all frozen):
- **C-Matrix** (`gnfs::filter`): the `SparseMatrix` — row-major `rows: Vec<MatrixRow>` (each row a
  sorted `cols: Vec<usize>` of set GF(2) columns + a `provenance: Vec<usize>` of original relation
  indices), `num_cols`, `obstruction_col_start`, `obstruction_count`, and the `col_weights: Vec<u32>`
  side table. G.E reads the row-major store for matrix–vector products and reads the provenance map
  through to G.F. Frozen at G.D.1 (`a0e854b`).
- **C-FactorBase** (`gnfs::sieve`): `FactorBase` with `rational_primes`, `algebraic_ideals`
  (`AlgebraicPrime { p, r, index, is_bad_prime }`), `matrix_width()` =
  `rational_size() + algebraic_size() + obstruction_count`, and the **public `obstruction_count`
  field** (currently 1, the sign column; docstringed "G.E may increase it"). Frozen at G.C.1
  (`c1dc0b6`). **G.E.1 is the first consumer that widens `obstruction_count`** — see the QC reach-back
  below.
- **C-Relation** (`gnfs::sieve`): the `Relation` type and its `rational_row_gf2` / `algebraic_row_gf2`
  GF(2) helpers. `algebraic_row_gf2` returns length `algebraic_size() + obstruction_count` and
  **auto-pads the obstruction tail with zeros** — so widening `obstruction_count` produces wider
  zero-tails with no signature change. Frozen at G.C.1 (`c1dc0b6`).

**Substrate gap G.E fills itself (the QC reach into C-FactorBase):** G.C reserved
`obstruction_count = 1` (the sign/−1 column) and zero-filled it through G.D. G.E's GF(2) linear
algebra additionally needs **quadratic-character columns**: parity conditions that guarantee the
selected relations' algebraic-norm product is a square *in the number field K*, not merely a square
integer (the sign column alone does not ensure the algebraic square root exists). G.E.1 chooses the
number of QC columns, **widens `FactorBase::obstruction_count` to `1 + num_qc`** (additive — the
substrate was built to absorb this; see Discoveries & risks), rebuilds the matrix so QC columns are
real matrix columns, and populates their parities. This is a **reach into frozen C-FactorBase**, so it
is surfaced as an **additive-reshard** (not silently grown) — but it is mechanical (a slot designed
to be widened), not a destructive contract break.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session. G.E adds modules to the *existing* `gnfs`
crate (already in `members`), so no workspace `Cargo.toml` change is required. The CADO-NFS
end-to-end oracle KAT follows the established `#[ignore = "CADO-NFS not installed; ..."]` pattern
(`merge_kat.rs`, `line_sieve_kat.rs`) and skips cleanly when CADO is absent.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default). `Cat` = category
(A substrate / B algorithm / C optimization / I integrative). `◆` marks a sub-track-final session.
`@plan` marks an inflection or review point requiring a juncture fork + human sign-off before the next
session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| G.E.1 `@plan` | Linalg substrate: blocked GF(2) vectors + matrix operator + quadratic-character columns | A | **Opus** | C-Matrix, C-FactorBase, C-Relation | new `gnfs/src/linalg/mod.rs`, `gnfs/src/linalg/blockvec.rs`, `gnfs/src/linalg/operator.rs`, `gnfs/src/lib.rs` (add `pub mod linalg`), `gnfs/tests/linalg_substrate_kat.rs` |
| G.E.2 `@plan` | Block Lanczos GF(2) nullspace solver (primary) | B | Sonnet | G.E.1 (C-LinAlg), C-Matrix | `gnfs/src/linalg/lanczos.rs`, `gnfs/src/linalg/mod.rs`, `gnfs/tests/lanczos_kat.rs` |
| G.E.3 | Block Wiedemann GF(2) nullspace solver (secondary) | B | Sonnet | G.E.1 (C-LinAlg), C-Matrix | `gnfs/src/linalg/wiedemann.rs`, `gnfs/src/linalg/mod.rs`, `gnfs/tests/wiedemann_kat.rs` |
| G.E.W ◆ | G.E integrative writeup (linear-algebra chapter) | I | Sonnet | all G.E | `gnfs/docs/PEDAGOGY.md` (append §31+), `docs/BENCHMARKS.md` (append) |

**Sequencing notes.** G.E carries **two `@plan` junctures of different kinds.** G.E.1 is a *design*
inflection: it stands up the `linalg/` module (sibling to `filter/`) and freezes **C-LinAlg** — the
blocked GF(2) vector representation and the sparse-matrix-as-linear-operator interface both solvers
consume, plus the **quadratic-character column** resolution (widen `obstruction_count`). It
deliberately implements *no solver* — pure substrate, the wide-design surface the ROADMAP front-loads
as Opus. The G.E.2 `@plan` is a *correctness-review* juncture, not a design one: when block Lanczos
(the lever-2 FLOOR, with its silent-wrong-kernel failure mode) lands green, halt and page a T0
`@plan-juncture` fork to review the implementation (self-orthogonality winnowing, kernel validity,
the operator-interface contract) against the landed code + the action-frame digest *before* G.E.3 is
dispatched — because a Lanczos bug is silent until G.F and the CADO oracle may be absent. The fork
returns one-shot findings; it does not implement. Once both solvers' substrate is sound, G.E.2 (block
Lanczos, primary) and G.E.3 (block Wiedemann, secondary) are **two orthogonal Category-B solvers**
over the same C-LinAlg substrate (mutually independent, either order), each freezing nothing new and
each KAT-able against the same nullspace. G.E.W is the ◆ boundary.

**Why G.E is 3 solver/substrate sessions + writeup (ROADMAP said 3-4).** The one-line-commit-title
corollary keeps each row a clean title. G.E.1's substrate (representation + operator + QC columns) is
the wide-design FLOOR the ROADMAP Opus-flags; bundling a solver into it would make it two titles and
overflow the design surface — it stands alone. Block Lanczos and block Wiedemann are **not merged**:
"block Lanczos *and* block Wiedemann" is two commit titles, they are two orthogonal algorithms
(Category B, the ROADMAP's primary/secondary split), and each is independently KAT-able — merging
would Cartesian-product two intricate solvers into one session. The block-Lanczos self-orthogonality
handling (lever-2 FLOOR) cannot be fractured below one working solve, so G.E.2 is one irreducible
unit; same for Wiedemann's Krylov-sequence + Berlekamp–Massey in G.E.3. **G.E.W** is the integrative
writeup, allocated its own session per the under-scheduling guidance. The G.E.2 correctness-review
juncture is **not** a session row: per the planning doc, a review juncture is a paged fork with no
commit-shaped code deliverable, so it rides as a `@plan` marker on G.E.2's boundary, not as its own
ledger line.

---

## Session detail

Lower-fidelity rows (G.E.2, G.E.3, G.E.W) are sketched; per the planning philosophy, sessions inside
a sub-track are crisply specified only after the substrate session (G.E.1) lands and freezes C-LinAlg.

### G.E.1 — Linalg substrate: blocked GF(2) vectors + matrix operator + quadratic-character columns (Opus, design inflection)

**Deliverable:**
- New `gnfs/src/linalg/` module (sibling to `filter/`). `gnfs/src/lib.rs` adds `pub mod linalg` and
  re-exports the solver entry surface (added by G.E.2/G.E.3).
- `linalg/blockvec.rs`: **C-LinAlg, part 1.** A blocked GF(2) vector / block of `N` GF(2) vectors
  (block width chosen at demonstration fidelity — e.g. 64, a machine word, annotated principle-4 as
  the scale knob). The GF(2) inner products and block operations block Lanczos and Wiedemann both
  need.
- `linalg/operator.rs`: **C-LinAlg, part 2.** The view of a C-Matrix `SparseMatrix` as a linear
  operator: the sparse matrix–block-vector product `A·V` and `Aᵀ·V` over GF(2), reading the
  row-major `rows`/`cols`. Both solvers consume *this*, never the `SparseMatrix` directly, so the
  representation is the frozen seam.
- **Quadratic-character columns.** Choose `num_qc`, widen `FactorBase::obstruction_count` to
  `1 + num_qc`, rebuild the matrix (or extend it) so QC columns are real matrix columns, and populate
  each row's QC parities (the Legendre-symbol parity of the algebraic norm at chosen auxiliary
  primes). The sign column stays at `obstruction_col_start`; QC columns follow. **This is the
  load-bearing design call of the session** — it reaches into frozen C-FactorBase and is re-consumed
  by D.B; freeze the QC representation so G.F and D.B don't disagree.
- `linalg/mod.rs`: the linalg entry surface (the operator constructor, the kernel-vector type) and
  the public types.

**Key design decisions (juncture fork designs C-LinAlg and writes it into Cross-session contracts):**
1. **Blocked-vector representation.** Block width `N` and the GF(2) packing (a `u64` per block of 64
   vectors vs a `Vec<bool>`). Block Lanczos/Wiedemann both want word-packed blocks for the
   matrix–vector product; bias toward a packed `u64`-block representation, annotated principle-4 (the
   packing is the scale knob — at toy scale a single word suffices). Freeze it — both solvers read it.
2. **Matrix-operator interface.** Whether `A·V` reads the C-Matrix row store directly or G.E.1
   builds a transposed/CSC companion for `Aᵀ·V`. Decide and freeze: both solvers route every matrix
   access through this interface, so a later change is a cross-solver break.
3. **Quadratic-character columns (the C-FactorBase reach).** `num_qc` (number of QC columns), the
   choice of auxiliary primes, and whether QC columns are added by **widening
   `FactorBase::obstruction_count`** (the confirmed expected approach — QC become real matrix columns,
   cleanest for D.B reuse) and rebuilding via `build_matrix`. Freeze the rule and surface the widen as
   an **additive-reshard** (see Discoveries & risks). **If the QC machinery turns out to require
   changing the *meaning* of an existing C-FactorBase column rather than appending — destructive-HALT,
   not additive.** (Not expected: the substrate auto-pads zero-tails for any `obstruction_count`.)
4. **Kernel-vector representation (the C-LinAlg → G.F seam).** A nullspace vector is a subset of
   *filtered* matrix rows; G.F expands it through the C-Matrix provenance map to a set of *original*
   relations. Decide and freeze how a solver returns a kernel vector (a bit-mask over current rows? a
   `Vec<usize>` of selected row indices?) so G.F and D.B consume a stable shape. Over-specify: carry
   the row-index form G.F needs even if Lanczos internally prefers a bit-mask.

**KAT (≥1 required):**
1. **Operator correctness:** for a small hand-built `SparseMatrix`, `A·V` and `Aᵀ·V` match the
   hand-computed GF(2) products for several block vectors `V`.
2. **QC column construction:** widening `obstruction_count` to `1 + num_qc` yields a matrix of width
   `matrix_width()` with the sign column at `obstruction_col_start` and `num_qc` QC columns following;
   each row's QC parity matches the hand-computed Legendre-symbol parity for a toy relation set.
3. **Round-trip with provenance:** a hand-built kernel vector expands through the C-Matrix provenance
   map to the expected set of original relation indices (the G.F seam exercised before G.F exists).
4. **Determinism:** the operator products and QC columns are deterministic for a fixed matrix.

**Subtlety:** the QC widen must be **additive** — it appends columns at the end of the obstruction
block and leaves every rational/algebraic/sign column index unchanged, so C-Matrix rows built before
the widen stay valid. A QC implementation that *renumbers* columns silently invalidates the frozen
C-Matrix layout (and any G.D provenance reasoning). KAT 2 must assert the pre-QC column indices are
unchanged after the widen. **The juncture fork decides** the representation and writes it into
C-LinAlg.

**Deferred:** block Lanczos (G.E.2); block Wiedemann (G.E.3); the writeup (G.E.W).

### G.E.2 — Block Lanczos GF(2) nullspace solver (Sonnet, on frozen C-LinAlg, + T0 review juncture, sketch)

**Deliverable:** Montgomery's block-Lanczos iteration over GF(2), reading the C-LinAlg operator and
blocked-vector representation. Build the A-orthogonal Krylov basis, handle **self-orthogonality** (the
GF(2)-specific winnowing: at each step select which block columns are A-invertible and advance only
those, carrying the rest forward), iterate to convergence, and extract kernel vectors. Returns
nullspace vectors in the frozen C-LinAlg kernel representation. The **primary** solver.

Freezes nothing new (consumes C-LinAlg, C-Matrix).

**KAT (≥1 required):** (a) for a small matrix with a known nullspace, Lanczos recovers a basis of the
left nullspace (each returned vector `v` satisfies `vᵀA = 0`); (b) the recovered kernel dimension is
deterministic for a fixed matrix; (c) **end-to-end CADO oracle (the ROADMAP G.E KAT):** for a small N,
a Lanczos kernel vector expands (through provenance) to a congruence of squares that yields the same
nontrivial factor CADO-NFS finds — dev-only, `#[ignore]`d if CADO absent, deterministic-kernel KAT (b)
carries reproducibility without it.

**Review juncture (`@plan`, T0/Opus, one-shot — runs after this session lands green, before G.E.3).**
Because a block-Lanczos bug is *silent* (a plausible-but-wrong or trivial kernel, not a red test) and
surfaces only as a non-factorization at G.F — and because the CADO end-to-end oracle may be absent —
a paged T0 `@plan-juncture` fork reviews the landed implementation against the code + action-frame
digest. Its remit: (1) the **self-orthogonality winnowing** is correct (the GF(2)-specific step that
naive Lanczos gets wrong — does it actually carry non-advancing columns forward, or silently drop
them?); (2) the recovered vectors genuinely lie in the left nullspace, not merely pass a
dimension check; (3) the solver honours the frozen C-LinAlg operator + kernel-vector contracts (no
shortcut that G.E.3/G.F/D.B can't consume); (4) the principle-4 annotations (block-width scale knob)
are present and honest. The fork **returns findings one-shot** (accurate / needs-fix / needs-human
discussion) and **does not implement** — fixes, if any, are a follow-on `@build` turn surfaced by
whoever pages the fork. This is the planning doc's review-juncture instrument, sited at the FLOOR
session by its cost-of-wrong, not at a sub-track boundary.

**Subtlety (principle-4 annotation):** self-orthogonality is *the* GF(2) phenomenon block Lanczos must
handle and it is **not** under-exposed at toy scale — even a tiny GF(2) matrix exhibits self-orthogonal
vectors, so KAT (a) must include a matrix that forces the winnowing path (a vector orthogonal to itself
under A). Montgomery's *block-width* tuning, by contrast, *is* a scale optimisation (the speedup from a
word-wide block is invisible at toy scale) — annotate that disconnect in the docstring + G.E.W +
Track τ.

### G.E.3 — Block Wiedemann GF(2) nullspace solver (Sonnet, on frozen C-LinAlg, sketch)

**Deliverable:** Coppersmith's block-Wiedemann alternative over GF(2): compute the matrix Krylov
sequence `{xᵀAⁱy}`, run **Berlekamp–Massey** (block/matrix variant at demonstration fidelity) to find
a linear generator, and extract kernel vectors from the generator polynomial evaluated at A. Reads the
same C-LinAlg operator; returns kernel vectors in the same representation. The **secondary** solver —
an orthogonal cross-check of G.E.2 and the natural generalisation target for D.B (NFS-DL over F_ℓ).

Freezes nothing new (consumes C-LinAlg, C-Matrix).

**KAT (≥1 required):** (a) Wiedemann recovers the *same* left nullspace as G.E.2 on a shared small
matrix (the two solvers cross-validate); (b) deterministic kernel dimension for a fixed matrix;
(c) the Berlekamp–Massey generator degree is the hand-computed value for a toy Krylov sequence.

**Subtlety (principle-4 annotation):** block Wiedemann's payoff is **distributed/parallel** (the
Krylov sequence parallelises across blocks where Lanczos does not) — a scale advantage entirely
under-exposed at toy scale, where Lanczos is simpler and just as fast. Implement Wiedemann at
demonstration fidelity (the matrix Berlekamp–Massey is real) and annotate that the *reason* it exists
at NFS scale (parallelism, no global synchronisation per step) is invisible here.

### G.E.W ◆ — Integrative writeup (Sonnet, sketch)

The linear-algebra chapter (`gnfs/docs/PEDAGOGY.md`, append after §30 as a new
`# Linear Algebra: A Code-Tour Chapter`): the filtered matrix as a GF(2) linear system, the **nullspace
as a congruence of squares**, why the sign and **quadratic-character** columns are needed (the
algebraic square root must exist *in K*, not merely as an integer), block Lanczos and its
self-orthogonality winnowing, block Wiedemann as the parallel alternative, and the kernel-vector →
provenance → original-relations thread G.F consumes. Append a G.E benchmark row to
`docs/BENCHMARKS.md` (matrix dimensions in / kernel dimension out / both solvers' timings).
**Display-math chapter** — the doc-format recommendation (ROADMAP Discoveries) applies directly here:
the block-Lanczos recurrence, the A-orthogonality condition, and the Berlekamp–Massey generator use
**MathJax `$$…$$`** (the heavy LA math the Discoveries entry named as load-bearing display math);
existing inline-Unicode passages may stay, and `$$` is already established in PEDAGOGY.md (from
§1429). Per pacing guidance, integrative writeups are under-scheduled — allocate a full session. This
is where C-LinAlg gets its public prose articulation and the D.B (F_ℓ generalisation) downstream reuse
is surfaced. Its Track τ maths-first sibling (T.G) pairs at the **G.W** ◆ boundary, not here — G.E.W
is a code-tour sub-chapter feeding the eventual G.W chapter. **G.E.W is a candidate Opus session only
if the block-Lanczos / L-notation math is treated as a designated payoff derivation** (ROADMAP T.G
note) — but as a code-tour it is exposition, not the payoff proof, so Sonnet, consistent with G.D.W.

---

## Cross-session contracts

The scaffolding sessions compose through. The juncture fork at G.E.1 writes the resolved **C-LinAlg**
interface (and the resolved QC widen) into this section before implementation is dispatched.

### C-Matrix — filtered sparse GF(2) matrix + relation-provenance map (compiler + KAT) — *frozen at G.D.1 (a0e854b)*
**Defined:** G.D.1. **Consumed by (in G.E):** G.E.1 (the matrix operator reads `rows`/`cols`;
`obstruction_col_start` / `obstruction_count` for the column blocks; provenance through to the
kernel-vector → G.F seam), G.E.2 / G.E.3 (via the C-LinAlg operator, never directly). G.E **reads**
the matrix and **widens `obstruction_count`** (a C-FactorBase reach, below) which *rebuilds* the
matrix to a wider `num_cols`; it does **not** otherwise amend the C-Matrix row/provenance shape. The
provenance map is the load-bearing seam carried through to G.F (already over-specified at G.D.1 to
store original relation indices). Stable for G.E except the additive QC widen.

### C-FactorBase — two-sided factor base + norm computation (compiler + KAT) — *frozen at G.C.1 (c1dc0b6)*
**Defined:** G.C.1. **Consumed by (in G.E):** G.E.1 (`matrix_width()`, `rational_size()` /
`algebraic_size()` for column partitioning; **`obstruction_count` widened from 1 to `1 + num_qc`**).
G.E.1 is the **first consumer to widen `obstruction_count`** — the QC reach the G.D plan anticipated.
The widen is **additive** (the field is `pub`, docstringed "G.E may increase it"; `algebraic_row_gf2`
auto-pads the obstruction tail with zeros for any `obstruction_count`), so it appends columns without
renumbering or changing the meaning of any rational/algebraic/sign column. Surfaced as an
**additive-reshard** for sign-off at the G.E.1 juncture, not silently grown. *Watch:* if QC turns out
to need *re-meaning* an existing column (not appending), that is a **destructive-HALT** (not expected).

### C-Relation — relation / exponent-vector format (compiler + KAT) — *frozen at G.C.1 (c1dc0b6)*
**Defined:** G.C.1. **Consumed by (in G.E):** G.E.1 (`rational_row_gf2` / `algebraic_row_gf2` when the
QC widen rebuilds the matrix; the `Vec<Relation>` stays live for the provenance → G.F expansion). G.E
**reads** relations; it does **not** amend C-Relation. The integer-exponent over-specification (for
D.A's GF(ℓ)) is irrelevant to G.E's GF(2) work. Stable for G.E.

### C-LinAlg — GF(2) nullspace substrate: blocked vectors + matrix operator + kernel representation (compiler + KAT) — *frozen at G.E.1*
**Defined:** G.E.1. **Consumed by:** G.E.2 (block Lanczos), G.E.3 (block Wiedemann), **G.F (square
root expands a kernel vector through the C-Matrix provenance map to original relations)**, and
**D.B (NFS-DL linear algebra over F_ℓ generalises the GF(2) operator and block machinery — ROADMAP
Phase γ)**. The blocked GF(2) vector representation, the sparse-matrix-as-linear-operator interface
(`A·V`, `Aᵀ·V`), the quadratic-character column resolution, and the kernel-vector representation. This
contract is **re-consumed cross-track by D.B** (unlike C-Matrix, which was Track-G-internal) — so
over-specify the operator interface and the kernel representation now, before D.B exists: carry the
F_ℓ-friendly shapes (a general scalar-block view rather than a GF(2)-hardcoded one) where the cost is
low, so D.B generalises rather than re-forks. The G.E.2 review juncture additionally checks that block
Lanczos honours this contract as frozen (no shortcut G.E.3/G.F/D.B can't consume).

**Resolved interface (frozen):**

#### 1. BlockVec — blocked GF(2) vector representation

```rust
/// Block width: 64 vectors packed into machine words.
///
/// Principle-4 annotation: at toy scale a single word suffices and the blocking overhead
/// is invisible; at NFS scale the word-wide block is the inner loop's cache-friendly unit.
/// The width is the scale knob — D.B may widen to 128 or parameterise over block width.
pub const BLOCK_WIDTH: usize = 64;

/// A block of `BLOCK_WIDTH` GF(2) vectors, each of length `num_rows`.
///
/// Representation: `data[i]` is a `u64` whose bit `j` (0 ≤ j < 64) is the `j`-th vector's
/// value at row `i`. This is the "row of words" layout: iterating over rows is contiguous,
/// and the 64 vectors are interleaved bit-by-bit within each word.
///
/// # D.B generalisation note
///
/// For F_ℓ (ℓ > 2), the natural generalisation is `data: Vec<[Scalar; BLOCK_WIDTH]>` where
/// `Scalar` is the field element type. The GF(2) specialisation packs 64 scalars into one
/// `u64`. D.B may introduce a `BlockVec<S>` generic or a parallel `BlockVecFl` type; the
/// *interface* (inner products, apply, apply_transpose) is the stable seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockVec {
    /// Packed row data: `data[row]` bit `j` = vector `j`'s value at `row`.
    pub data: Vec<u64>,
    /// Number of rows (the vector dimension, i.e. matrix height for A·V).
    pub num_rows: usize,
}

impl BlockVec {
    /// Construct a zero block vector of the given dimension.
    pub fn zeros(num_rows: usize) -> Self;

    /// Construct from a dense `num_rows × BLOCK_WIDTH` bool matrix (column-major: `cols[j][i]`).
    /// Used for test construction; solvers use `zeros` + `set_bit`.
    pub fn from_columns(cols: &[Vec<bool>]) -> Self;

    /// Get bit `(row, col)` where `col < BLOCK_WIDTH`.
    pub fn get(&self, row: usize, col: usize) -> bool;

    /// Set bit `(row, col)` to `value`.
    pub fn set(&mut self, row: usize, col: usize, value: bool);

    /// XOR `self` with `other` in place (component-wise GF(2) addition).
    pub fn xor_assign(&mut self, other: &BlockVec);

    /// Compute the `BLOCK_WIDTH × BLOCK_WIDTH` GF(2) inner-product matrix `self^T · other`.
    ///
    /// Returns a `[u64; BLOCK_WIDTH]` where `result[i]` bit `j` = `⟨self.col(i), other.col(j)⟩`
    /// over GF(2) (i.e., parity of the AND of the two columns).
    ///
    /// This is the core primitive for block Lanczos's A-orthogonality check and for
    /// Wiedemann's Krylov-sequence inner products.
    pub fn inner_product_matrix(&self, other: &BlockVec) -> [u64; BLOCK_WIDTH];

    /// Extract column `j` as a dense `Vec<bool>` (for debugging / KAT).
    pub fn column(&self, j: usize) -> Vec<bool>;
}
```

#### 2. MatrixOperator — sparse matrix as linear operator over GF(2)

```rust
/// A view of a `SparseMatrix` as a linear operator over GF(2).
///
/// Provides `apply` (A·V) and `apply_transpose` (Aᵀ·V) for block vectors. Both solvers
/// (Lanczos, Wiedemann) consume this interface exclusively; they never read `SparseMatrix`
/// fields directly. This is the frozen seam: the internal representation (row-major CSR,
/// whether a CSC companion exists) is an implementation detail.
///
/// # Transpose strategy (design decision, frozen)
///
/// `apply_transpose` computes Aᵀ·V **on-the-fly** by iterating over rows and scattering
/// contributions, rather than pre-building a CSC (column-major) companion. Rationale:
///
/// - At toy scale, the matrix fits in cache and the scatter is cheap.
/// - At NFS scale, the matrix is too large to duplicate; production solvers (CADO-NFS)
///   also compute the transpose product on-the-fly with careful cache blocking.
/// - Pre-building CSC would double memory and complicate the C-Matrix contract (which is
///   row-major only).
///
/// Principle-4 annotation: the on-the-fly transpose is the correct algorithm at all scales;
/// the *cache-blocking* that makes it fast at NFS scale is the engineering optimisation
/// out of scope (scoping principle 3).
///
/// # D.B generalisation note
///
/// For F_ℓ, the operator needs the same shape but with scalar multiplication. The natural
/// generalisation is a trait `LinearOperator<V>` with `apply(&self, v: &V) -> V` and
/// `apply_transpose(&self, v: &V) -> V`. G.E implements the concrete GF(2) version; D.B
/// may introduce the trait and have `MatrixOperator` implement it.
pub struct MatrixOperator<'a> {
    matrix: &'a SparseMatrix,
}

impl<'a> MatrixOperator<'a> {
    /// Construct an operator view of the given sparse matrix.
    pub fn new(matrix: &'a SparseMatrix) -> Self;

    /// Number of rows (matrix height).
    pub fn num_rows(&self) -> usize;

    /// Number of columns (matrix width).
    pub fn num_cols(&self) -> usize;

    /// Compute A·V: multiply the matrix by a block vector.
    ///
    /// Input `v` has dimension `num_cols`; output has dimension `num_rows`.
    /// Each output row is the GF(2) dot product of the matrix row with each of the
    /// `BLOCK_WIDTH` input vectors.
    pub fn apply(&self, v: &BlockVec) -> BlockVec;

    /// Compute Aᵀ·V: multiply the transpose by a block vector.
    ///
    /// Input `v` has dimension `num_rows`; output has dimension `num_cols`.
    /// Computed on-the-fly by scattering: for each matrix row `i`, for each column `c`
    /// in that row, XOR `v.data[i]` into `result.data[c]`.
    pub fn apply_transpose(&self, v: &BlockVec) -> BlockVec;
}
```

#### 3. KernelVector — nullspace vector representation (the G.F seam)

```rust
/// A vector in the left nullspace of the matrix: a subset of rows whose GF(2) sum is zero.
///
/// Representation: a sorted, deduplicated `Vec<usize>` of **filtered-matrix row indices**
/// (indices into `SparseMatrix::rows`). G.F expands this to original relation indices by
/// collecting `matrix.rows[i].provenance` for each `i` in `row_indices` and taking the
/// symmetric difference (XOR union).
///
/// # Why row indices, not a bit-mask
///
/// - G.F needs row indices to look up provenance; a bit-mask would require a scan.
/// - Kernel vectors are sparse (typically a small fraction of rows); a bit-mask wastes space.
/// - Solvers (Lanczos, Wiedemann) internally work with bit-packed block vectors, but they
///   convert to `KernelVector` on output — the conversion is O(rows) and happens once per
///   kernel vector, not in the inner loop.
///
/// # D.B generalisation note
///
/// For F_ℓ, a kernel vector is still a subset of rows (those with nonzero coefficient in
/// the nullspace vector). The representation is identical; D.B may add a `coefficients`
/// field (`Vec<Scalar>`) for the non-GF(2) case, but the row-index spine is stable.
///
/// # Invariants
///
/// - `row_indices` is sorted and deduplicated.
/// - Each index is < `matrix.rows.len()` (the filtered matrix, not the original relations).
/// - The GF(2) sum of the selected rows is the zero vector (verified by `verify`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelVector {
    /// Sorted, deduplicated row indices into the filtered matrix.
    pub row_indices: Vec<usize>,
}

impl KernelVector {
    /// Construct from a sorted, deduplicated list of row indices.
    ///
    /// Panics if `row_indices` is not sorted or contains duplicates.
    pub fn new(row_indices: Vec<usize>) -> Self;

    /// Construct from a bit-mask over rows (used by solvers internally).
    ///
    /// `mask[i]` is true iff row `i` is in the kernel vector.
    pub fn from_mask(mask: &[bool]) -> Self;

    /// Verify that this is a valid left-nullspace vector of the given matrix.
    ///
    /// Returns `true` iff the GF(2) sum of `matrix.rows[i].cols` for `i` in `row_indices`
    /// is the empty set (all columns cancel).
    pub fn verify(&self, matrix: &SparseMatrix) -> bool;

    /// Expand through the provenance map to original relation indices.
    ///
    /// Returns the symmetric difference (XOR union) of `matrix.rows[i].provenance` for
    /// each `i` in `row_indices`. This is the set of original relations whose product
    /// yields a congruence of squares.
    ///
    /// The result is sorted and deduplicated.
    pub fn expand_provenance(&self, matrix: &SparseMatrix) -> Vec<usize>;

    /// Number of selected rows.
    pub fn len(&self) -> usize;

    /// True if no rows are selected (the trivial kernel vector).
    pub fn is_empty(&self) -> bool;
}
```

#### 4. Quadratic-character columns — the C-FactorBase reach

**Resolution (frozen):**

- **`num_qc` = 10** (demonstration fidelity). At NFS scale, `num_qc` is typically 20–50 to ensure
  the algebraic square root exists with high probability; at toy scale, 10 suffices and keeps the
  matrix width manageable. Principle-4 annotation: the *number* is the scale knob; the *mechanism*
  (Legendre-symbol parity at auxiliary primes) is the same at all scales.

- **Auxiliary prime selection:** the first `num_qc` primes `q > B_alg` that **split completely**
  in K (i.e., `f(x)` has `deg(f)` distinct roots mod `q`). Rationale: split primes yield
  well-defined Legendre symbols for the algebraic norm; inert or ramified primes complicate the
  character computation. At toy scale with small `B_alg`, finding 10 split primes is fast (trial
  over primes > `B_alg`).

- **Widen mechanism (additive, as anticipated):**
  1. Caller sets `fb.obstruction_count = 1 + num_qc` (was 1, the sign column).
  2. Caller calls `build_matrix(relations, &fb)` — the existing function, unchanged.
  3. `build_matrix` produces a matrix with `num_cols = fb.matrix_width()` = rational + algebraic +
     `1 + num_qc`. The sign column is at `obstruction_col_start`; QC columns are at indices
     `obstruction_col_start + 1` through `obstruction_col_start + num_qc`.
  4. `algebraic_row_gf2` auto-pads the obstruction tail with zeros (existing behaviour), so the
     matrix rows have the correct width with QC columns initially zero.
  5. A new function `populate_qc_columns(matrix: &mut SparseMatrix, relations: &[Relation],
     fb: &FactorBase, qc_primes: &[u64])` fills in the QC parities by computing the Legendre
     symbol of each relation's algebraic norm at each QC prime.

- **Column-index stability (the additive-reshard invariant):** widening `obstruction_count` from 1
  to `1 + num_qc` appends columns at the end. All rational, algebraic, and sign column indices are
  unchanged. A matrix built before the widen (with `obstruction_count = 1`) has fewer columns but
  the same column semantics for indices `0..obstruction_col_start + 1`. KAT must verify this.

```rust
/// Populate quadratic-character columns in a matrix.
///
/// For each row `i`, for each QC prime `qc_primes[k]`, sets column
/// `matrix.obstruction_col_start + 1 + k` to the Legendre-symbol parity of the
/// algebraic norm of `relations[provenance[0]]` at `qc_primes[k]`.
///
/// Preconditions:
/// - `matrix` was built with `fb.obstruction_count = 1 + qc_primes.len()`.
/// - `qc_primes` are primes > `fb.b_alg` that split completely in K.
/// - Each row's provenance is non-empty (true for any matrix from `build_matrix`).
///
/// # Legendre-symbol computation
///
/// For a relation with algebraic norm `N_alg(a, b)` and a QC prime `q`, the QC parity is
/// `(N_alg / q) mod 2` where `(· / q)` is the Legendre symbol. If `N_alg ≡ 0 (mod q)`,
/// the symbol is 0 (even parity). The parity is 1 iff the Legendre symbol is −1.
///
/// For merged rows (provenance has multiple original relations), the QC parity is the
/// XOR (GF(2) sum) of the individual relations' parities — consistent with how merged
/// rows' factor-base columns are the XOR of the originals.
pub fn populate_qc_columns(
    matrix: &mut SparseMatrix,
    relations: &[Relation],
    fb: &FactorBase,
    poly: &PolyPair,
    qc_primes: &[u64],
);

/// Select `num_qc` auxiliary primes for quadratic-character columns.
///
/// Returns the first `num_qc` primes `q > b_alg` such that `f(x)` has `deg(f)` distinct
/// roots mod `q` (i.e., `q` splits completely in K = ℚ[x]/(f)).
///
/// # Panics
///
/// Panics if fewer than `num_qc` split primes exist below some reasonable bound (should
/// not happen for any reasonable `num_qc` and `b_alg`).
pub fn select_qc_primes(f: &IntPoly, b_alg: u64, num_qc: usize) -> Vec<u64>;
```

#### 5. Public module surface of `gnfs::linalg`

```rust
// gnfs/src/linalg/mod.rs

//! Linear algebra substrate for GNFS: blocked GF(2) vectors, sparse matrix operator,
//! and kernel-vector representation.
//!
//! This module provides the substrate for G.E.2 (block Lanczos) and G.E.3 (block Wiedemann).
//! Both solvers consume the `MatrixOperator` interface and return `KernelVector`s; they never
//! access `SparseMatrix` fields directly.
//!
//! # C-LinAlg contract
//!
//! The types and functions in this module implement the C-LinAlg contract frozen at G.E.1.
//! G.E.2, G.E.3, G.F, and D.B consume this interface.

pub mod blockvec;
pub mod operator;

pub use blockvec::{BlockVec, BLOCK_WIDTH};
pub use operator::MatrixOperator;

// Re-export kernel vector and QC utilities from this module's root.
mod kernel;
mod qc;

pub use kernel::KernelVector;
pub use qc::{populate_qc_columns, select_qc_primes};

/// Default number of quadratic-character columns (demonstration fidelity).
///
/// Principle-4 annotation: at NFS scale this is typically 20–50; at toy scale 10 suffices.
pub const DEFAULT_NUM_QC: usize = 10;
```

**Files created by G.E.1:**
- `gnfs/src/linalg/mod.rs` — module root, public surface
- `gnfs/src/linalg/blockvec.rs` — `BlockVec`, `BLOCK_WIDTH`
- `gnfs/src/linalg/operator.rs` — `MatrixOperator`
- `gnfs/src/linalg/kernel.rs` — `KernelVector`
- `gnfs/src/linalg/qc.rs` — `populate_qc_columns`, `select_qc_primes`, `DEFAULT_NUM_QC`
- `gnfs/src/lib.rs` — add `pub mod linalg` and re-exports
- `gnfs/tests/linalg_substrate_kat.rs` — KATs per the session detail

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The G.E.2 review juncture is not a ledger row (a paged
fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| G.E.1 | Linalg substrate: blocked GF(2) vectors + operator + QC columns | done | 416f6db | C-LinAlg (frozen); C-FactorBase obstruction_count widened (additive, confirmed). Extra files: kernel.rs, qc.rs (plainly part of unit — juncture fork split KernelVector and QC into separate files). |
| G.E.2 | Block Lanczos GF(2) nullspace solver (primary) | done | 5145d4c | — (T0 review juncture: review-passed — see digest) |
| G.E.3 | Block Wiedemann GF(2) nullspace solver (secondary) | done | 19936a7 | — |
| G.E.W | Integrative writeup (linear-algebra chapter) | pending | — | — |

Contracts frozen before G.E: C-Fp (cf00ed5), C-numth (α.2), C-NF (bdba6f5), C-Ideal (05b27c8),
C-Res (bcd63cd), C-Dedekind (7844773), C-PolyPair (2f43f99), C-Score (00aa32d), C-FactorBase
(c1dc0b6), C-Relation (c1dc0b6), C-Matrix (a0e854b). G.E opens over the frozen G.A substrate, G.B
polynomial-selection layer, G.C sieve layer, and G.D filter layer.

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume — including the **G.E.2 review-juncture outcome**
(the T0 fork's findings on block Lanczos), which is recorded here rather than in the ledger.

### G.E.1 — 2026-06-07
Discovery/flex: G.E.1 inflection fork returned `design-confident`; C-LinAlg frozen with BlockVec (u64 row-of-words, BLOCK_WIDTH=64), MatrixOperator (on-the-fly transpose, no CSC companion), KernelVector (sorted Vec<usize> row indices), and QC columns (num_qc=10, additive widen of obstruction_count to 1+num_qc via populate_qc_columns). Additive-reshard of C-FactorBase confirmed mechanical (designed-to-widen slot).
Affected: C-LinAlg (frozen at 416f6db); C-FactorBase obstruction_count widened (additive, not destructive).
Deferred: no — all four design decisions resolved; D.B generalisation paths documented in contract.
Texture: The juncture fork split KernelVector and QC into separate files (kernel.rs, qc.rs) beyond the session list's three-file expectation — plainly part of the unit, allowed and noted. The on-the-fly transpose decision (no CSC companion) is frozen; D.B may introduce a LinearOperator trait.

### G.E.2 — 2026-06-07
Discovery/flex: T0 correctness-review juncture returned `review-passed` on all six review points. Self-orthogonality winnowing correct (inactive columns checked for nullspace membership, not silently dropped). Recovered vectors verified via KernelVector::verify in all KATs. C-LinAlg contracts honoured (matrix access exclusively through MatrixOperator, returns KernelVector). Principle-4 annotations present and honest. KAT forces self-orthogonality path via duplicate-row matrix. Three-term recurrence follows Montgomery's standard form correctly.
Affected: none — no contract flex; C-LinAlg consumed correctly.
Deferred: no — review-passed, G.E.3 may proceed.
Texture: Minor observation: dedup_by at end of block_lanczos relies on adjacent duplicates; non-adjacent duplicates possible in theory but not a correctness issue at toy scale. The CADO oracle KAT is present but ignored (CADO not installed); deterministic-kernel KAT carries reproducibility.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **Quadratic-character columns reach into frozen C-FactorBase — additive-reshard, surface don't
  grow silently (lever 3, cross-track into D.B).** G.C reserved `obstruction_count = 1` (sign only)
  and zero-filled it through G.D, anticipating this exact moment (the G.D plan's "watch" note). G.E.1
  widens `obstruction_count` to `1 + num_qc` so QC become real matrix columns. The substrate was
  *built* to absorb this: `obstruction_count` is a `pub` field docstringed "G.E may increase it," and
  `algebraic_row_gf2` auto-pads the obstruction tail with zeros for any count — so the widen appends
  columns without renumbering. This is an **additive-reshard** (mechanical widen of a
  designed-to-widen slot — surface for sign-off at the G.E.1 juncture, do not silently grow). Only if
  QC requires *changing the meaning* of an existing C-FactorBase column is it a **destructive-HALT**
  (not expected). The QC representation is re-consumed by D.B, so freeze it deliberately.

- **C-LinAlg is the first cross-track-reused Track-G linalg contract (D.B generalises it).** Unlike
  C-Matrix (Track-G-internal), the operator + blocked-vector + kernel representation is generalised
  by D.B (NFS-DL linear algebra over F_ℓ). Re-forking it at D.B would be a **destructive reshard**
  one track over; designing the operator interface as a scalar-block view (not GF(2)-hardcoded) where
  the cost is low is the over-specify rule applied to a cross-track contract. If G.E.1 finds the
  GF(2)-specific shortcuts are too entangled to generalise cheaply, *surface it* (additive note for
  D.A/D.B), do not silently hardcode.

- **Self-orthogonality is the GF(2) correctness phenomenon, not a scale artifact (lever 4) — and the
  primary target of the G.E.2 review juncture.** Over ℝ, Lanczos's basis vectors are never
  self-orthogonal; over GF(2) a nonzero vector can be orthogonal to itself under A, which naive
  Lanczos divides by (a silent wrong/trivial kernel). Block Lanczos's winnowing handles it. This is an
  *internal-continue* correctness obligation at G.E.2 (handle the winnowing, KAT a
  self-orthogonal-forcing matrix), present even at toy scale — not a halt. The G.E.2 review juncture
  exists precisely because getting this wrong is silent.

- **A wrong kernel is silent (lever 4, why the end-to-end KAT *and* the G.E.2 review juncture are
  load-bearing).** Unlike G.D filtering (a bug surfaces immediately as a wrong matrix *dimension*), a
  linear-algebra bug can return a plausible kernel vector that is simply *wrong* (the trivial
  solution, or a vector that isn't in the nullspace), surfacing only as a non-factorization at G.F.
  The deterministic-kernel KAT (dimension) is necessary but not sufficient; the **end-to-end CADO
  oracle KAT** (kernel → actual factor) is the behavioural gate — and when CADO is absent, the
  **G.E.2 T0 review juncture** is the compensating control (a strong model reads the solver and the
  digest). This is why lever 5 is strong *only with* the end-to-end KAT, why correctness-criticality
  (lever 4) stays high — holding juncture-tier at Opus — and why the FLOOR session earns its own
  review juncture.

- **CADO-NFS oracle availability.** The end-to-end "kernel recovers CADO's factorization" KAT
  (G.E.2 KAT c) depends on CADO-NFS as a dev oracle. Absent → gate behind the established
  `#[ignore = "CADO-NFS not installed; ..."]` pattern (already used in `merge_kat.rs`,
  `line_sieve_kat.rs`); the deterministic-kernel KAT (G.E.2 KAT b, G.E.3 KAT a cross-validation)
  carries reproducibility without CADO, and the G.E.2 review juncture compensates for the lost
  behavioural oracle. The project-wide oracle-gating policy is still open (ROADMAP Discoveries,
  reference-oracle entry) — G.E inherits the per-test `#[ignore]` workaround; it does not resolve the
  policy. **msieve** is also named as a G.E/G.F oracle (ROADMAP reference-oracle entry) but is not yet
  present; no new msieve dependency is introduced — the CADO end-to-end check suffices.

- **Block-width and Wiedemann-parallelism payoffs under-exposed at toy scale (principle 4).** The
  word-wide block (block Lanczos) and the distributed Krylov sequence (block Wiedemann) are *scale*
  optimisations — at toy scale a single block / a serial Krylov sequence is simpler and just as fast.
  Implement at demonstration fidelity and annotate the disconnect in code + G.E.W + Track τ. Not a
  correctness risk — a pedagogy-honesty obligation. (Distinct from self-orthogonality, which *is*
  exposed at toy scale.)

- **`linalg/` module is fresh ambient surface (lever 1).** Third non-`polyselect`/`sieve` module in
  `gnfs` (after `filter/`); G.E.1 sets `src/linalg/` layout, which G.F (`src/sqrt/`?) will sibling.
  Keep the crate-internal module structure open rather than over-committing at G.E.1.

- **Documentation math-rendering format (now directly relevant — G.E.W is heavy display math).**
  Recommended at the G.C ◆ boundary: **Markdown + MathJax** (`$…$` / `$$…$$`), to be ratified at T.0
  (ROADMAP Discoveries log). Unlike G.D (whose `*.W` produced little prose math), **G.E.W is one of
  the chapters the Discoveries entry explicitly named as load-bearing display math** (block-Lanczos /
  block-Wiedemann linear algebra). G.E.W uses MathJax `$$…$$` for the LA recurrences; `$$` is already
  established in PEDAGOGY.md. Not a code contract — gates no G.E implementation session, only G.E.W's
  prose.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase β / Track G section: G.E entry "block Lanczos primary, block Wiedemann
  secondary"; Opus-flagged-sessions table G.E.1 entry; Cross-track contracts; Discoveries log —
  including the documentation-format and reference-oracle entries) and the prior `docs/PLAN.md` G.D
  history before any G.E session.
- Read `gnfs/docs/PEDAGOGY.md` (the G.D filtering chapter §22–§30, especially the C-Matrix /
  obstruction-column passages forward-referencing G.E) and `shared/numfield/docs/PEDAGOGY.md` for the
  pedagogical register (rST docstrings, KATs per session, narrative chapter at each ◆ boundary). New
  `gnfs::linalg` work matches it.
- **Register: PEDAGOGY.** This is a reference library — code is teaching material. Match the G.C.W /
  G.D.W chapter genre and quality.
- **Tier routing:** G.E.1 is the `@plan` **design** inflection (**Opus** juncture fork per
  `juncture-tier: opus` — freezes C-LinAlg and widens C-FactorBase's `obstruction_count`, a
  cross-track-reused contract; the opt-back-up from G.D's `sonnet` is justified in the header). G.E.2
  is Sonnet (`@build`) **but carries a second `@plan` marker for a T0/Opus correctness-review juncture
  after it lands** (paged `@plan-juncture` fork, one-shot, reviews block Lanczos before G.E.3 — see
  the G.E.2 session detail). G.E.3 and G.E.W are Sonnet (`@build`). The two Opus touchpoints are the
  G.E.1 design fork and the G.E.2 review fork; the ROADMAP Opus-flags G.E.1, and the review fork is
  the lever-4-driven addition.
- **Invariants to preserve:** the G.A substrate contracts (C-NF, C-Res, C-numth, C-Ideal,
  C-Dedekind), the G.B contracts (C-PolyPair, C-Score), the G.C contracts (C-Relation, C-FactorBase),
  and the **G.D contract (C-Matrix)** are frozen — G.E consumes them. The one sanctioned reach is the
  **additive widen of `FactorBase::obstruction_count`** at G.E.1 (surface as additive-reshard). The
  `rho` crate, `gnfs::polyselect`, `gnfs::sieve`, and `gnfs::filter` stay otherwise untouched (G.E
  *adds* `gnfs::linalg`; the QC widen rebuilds the matrix but does not change filter code).
- **CADO-NFS / msieve are dev-only oracles**, never on a build path (ROADMAP scoping principle 3).
  Gate oracle KATs with the established `#[ignore]` pattern.
- **Doc-format note (for G.E.W):** new display mathematics uses MathJax (`$$…$$`); inline-Unicode may
  stay. G.E.W is a heavy-LA-math chapter — one of the Discoveries-log-named load-bearing display-math
  chapters. Recommendation pending T.0 ratification (ROADMAP Discoveries log).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the `linalg/`-module
  shard pattern is unproven for this crate (first linalg module; C-LinAlg is the first cross-track
  linalg contract and the first session to widen `obstruction_count`). With the added G.E.2 review
  marker this halts **three times**: the G.E.1 design inflection, the **G.E.2 correctness-review
  juncture** (the new one — page a T0 `@plan-juncture` fork on block Lanczos before G.E.3), and the
  G.E.W ◆ boundary.
