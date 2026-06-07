<!--
juncture-tier: sonnet
-->

# rGNFS — Current Plan: Phase β, sub-track G.D (Filtering)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: sonnet` (header above) — **opt-down from the G.C `opus` default.** G.D *consumes*
the already-frozen cross-track contracts (C-Relation, C-FactorBase, both frozen at G.C.1 `c1dc0b6`)
and freezes only **C-Matrix**, a contract that is **internal to Track G** (consumed by G.E linear
algebra and, through it, G.F square root — never adapted cross-track the way C-Relation was by D.A).
Applying the six-lever tuning law: lever 5 (inner-loop bandwidth) is **strong** — `cargo test
--workspace` plus deterministic dimension KATs plus the CADO-NFS matrix-dimension oracle catch
filtering drift behaviourally and immediately; levers 3 (design-error cost) and 4
(correctness-criticality) are only **moderate** because a filtering bug surfaces as a wrong matrix
dimension at G.E's first KAT, not as a silent cross-track contract violation. Strong inner loop +
moderate cost-of-wrong is exactly the case the planning doc names as licensing a Sonnet adjudicator.
The ROADMAP flags **no** G.D session as Opus, confirming the opt-down. (Reconsider `opus` only if
G.D.1 surfaces that C-Matrix must encode obstruction/quadratic-character structure in a way that
reaches back into C-FactorBase — see Discoveries & risks.)

Last rewrite: G.C ◆ boundary crossed (G.C.W landed at `23a5222`). G.C fully complete (G.C.1 →
G.C.W); C-Relation and C-FactorBase frozen. This plan opens sub-track G.D — the **filtering** stage
of the GNFS pipeline — over the frozen sieve substrate. The documentation math-rendering format was
recommended at this boundary (Markdown + MathJax; see ROADMAP Discoveries log) — a static-frame doc
call that does not touch G.D code; G.D.W's chapter is the first place it would apply, far downstream.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.D) builds **GNFS filtering** — given the raw **relation corpus** from
G.C sieving, reduce it to the compact, full-rank **sparse matrix** that G.E's linear algebra solves.
Filtering is the bridge between "we collected relations" and "we have a matrix whose nullspace yields
a congruence of squares." It comprises three classic steps:

1. **Singleton removal.** A relation containing a prime/ideal that appears in *no other* relation
   (a singleton column) can never be part of a dependency and must be discarded; removing it can
   create new singletons, so the step iterates to a fixpoint.
2. **Clique removal (pruning).** With more relations than columns (excess), prune relations to
   minimise final matrix weight while preserving a sufficient excess — the "graph view" where
   relations are edges and shared primes induce connectivity.
3. **Merging.** Combine relations that share a low-frequency prime (2-way, then k-way merges) to
   eliminate that column, trading a small density increase for a large dimension reduction — the
   step that makes the matrix tractable for G.E.

This is the third stage of the GNFS pipeline proper: it sits *on* the G.C relation corpus
(C-Relation) and factor-base column indexing (C-FactorBase), and it *produces* the sparse matrix
(C-Matrix) that G.E (linear algebra) consumes and G.F (square root) ultimately depends on.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating the filter beyond
singleton + clique + merge at demonstration fidelity — Cavallar's full merge-cost heuristics, GPU
filtering, and out-of-core relation streaming are out of scope, ROADMAP principle 3) and **rigidity**
(grinding through a matrix format that G.E later shows is wrong, rather than surfacing it at the
boundary).

**Scoping discipline (ROADMAP three-way split, applied to G.D).** Algorithmic content complete:
singleton removal to fixpoint, clique/excess pruning, and 2-way + k-way merging — the graph view of
relations made explicit. **Scale-only** content (Cavallar-style weighted merge-cost minimisation,
large-prime-driven merge ordering) is present at **demonstration fidelity** (the heuristic is in the
code even where its weight-saving doesn't show at toy scale — principle 2, disconnect annotated per
principle 4). **Engineering optimizations** (out-of-core relation streaming, parallel merge, bucket
hashing of columns) are omitted. **CADO-NFS is a dev-only correctness oracle** for the
matrix-dimension cross-check — never on a build path.

---

## Current state

Phase α + sub-tracks G.A, G.B, G.C complete. Workspace crates: `shared/field`, `shared/bigint`,
`shared/numth`, `shared/numfield`, `rho`, `gnfs`. `cargo test --workspace` green at `23a5222`.

The `gnfs` crate now carries `polyselect/` (G.B) and `sieve/` (G.C). G.D adds a sibling `filter/`
module (the layout G.C.1 kept open: `src/sieve/` siblings `src/filter/` siblings the eventual
`src/linalg/` for G.E).

Substrate G.D consumes (all frozen):
- **C-Relation** (`gnfs::sieve`): the `Relation` type — coprime `(a, b)`, `rational_exponents` and
  `algebraic_exponents` as sparse `ExponentVector`s of `(factor-base index, u32 exponent)` pairs,
  `rational_sign`, `verify()`, and `rational_row_gf2` / `algebraic_row_gf2`. Filtering reads the
  *column indices* (the factor-base indices inside each `ExponentVector`) to build the relation→prime
  incidence structure; it does **not** re-derive smoothness. Frozen at G.C.1 (`c1dc0b6`).
- **C-FactorBase** (`gnfs::sieve`): `FactorBase` with `rational_primes`, `algebraic_ideals`
  (`AlgebraicPrime { p, r, index, is_bad_prime }`), `matrix_width()` =
  `rational_size() + algebraic_size() + obstruction_count`, and `rational_index` / `algebraic_index`
  lookups. Filtering uses `matrix_width()` as the column count and the per-side sizes to partition
  columns. `obstruction_count` (=1, the sign column) is carried through to G.E untouched by G.D.
  Frozen at G.C.1 (`c1dc0b6`).

**Substrate gap G.D fills itself (no external dependency):** G.C produced a `Vec<Relation>`; G.E
needs a **sparse matrix over GF(2)** with explicit dimensions and a relation-provenance map (so a
nullspace vector over the *filtered* matrix can be expanded back to a combination of *original*
relations for the square-root step). G.D.1 defines this matrix representation (C-Matrix) and the
provenance map. This is a filter-side construction, not a substrate change — it does not flex
C-Relation or C-FactorBase.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session. G.D adds modules to the *existing* `gnfs`
crate (already in `members`), so no workspace `Cargo.toml` change is required.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default; the substrate-ish
G.D.1 runs to the top of that band). `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection point requiring a
juncture fork + human sign-off before dispatch.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| G.D.1 `@plan` | Filter substrate: sparse GF(2) matrix + provenance map + singleton removal | A | Sonnet | C-Relation, C-FactorBase | new `gnfs/src/filter/mod.rs`, `gnfs/src/filter/matrix.rs`, `gnfs/src/filter/singleton.rs`, `gnfs/src/lib.rs` (add `pub mod filter`), `gnfs/tests/singleton_kat.rs` |
| G.D.2 | Clique/excess pruning + merging (2-way then k-way) | B | Sonnet | G.D.1 (C-Matrix), C-FactorBase | `gnfs/src/filter/merge.rs`, `gnfs/src/filter/mod.rs`, `gnfs/tests/merge_kat.rs` |
| G.D.W ◆ | G.D integrative writeup (filtering chapter) | I | Sonnet | all G.D | `gnfs/docs/PEDAGOGY.md` (append), `docs/BENCHMARKS.md` (append) |

**Sequencing notes.** G.D.1 is the single inflection point (`@plan`): it stands up the `filter/`
module (sibling to `sieve/`) and freezes **C-Matrix** — the sparse GF(2) matrix representation and
the **relation-provenance map** (filtered-column → set-of-original-relation-indices), which G.E
consumes and G.F depends on for expanding a nullspace vector back to original relations. It also
implements **singleton removal** to a fixpoint, the cleanest filtering step and the one that
exercises the relation→column incidence structure the merge step then builds on. Once C-Matrix
freezes, G.D.2 (clique pruning + merging) reduces the matrix over it **without altering the
singleton-removed baseline** (the unfiltered and singleton-only matrices stay constructible for
benchmarking the merge weight-saving). G.D.W is the ◆ boundary.

**Why G.D is 2 sessions + writeup (ROADMAP said 2, listed three steps).** The one-line-commit-title
corollary restructures the ROADMAP's prose list. Singleton removal is contract-sharp (it needs the
matrix representation and the incidence map — the substrate prerequisite the ROADMAP under-named),
so it pairs with the C-Matrix freeze in **G.D.1**. Clique pruning and merging are one irreducible
unit: merging operates on the excess that clique pruning leaves, and neither has a KAT-able matrix
dimension without the other (lever 2 floor — splitting them fractures the "reduce to final
dimensions" deliverable), so they merge into **G.D.2**. The genuinely separate, clean-commit-title
units are: **substrate + singletons** and **clique + merge**. **G.D.W** is the integrative writeup
(the graph-view chapter), allocated its own session per the under-scheduling guidance. No G.D session
is a merge/fold candidate: G.D.1's singleton step is non-trivial (fixpoint iteration over the
incidence structure) and G.D.2's clique+merge is the structural heart — neither reduces to a wrapper.

---

## Session detail

Lower-fidelity rows (G.D.2, G.D.W) are sketched; per the planning philosophy, sessions inside a
sub-track are crisply specified only after the substrate session (G.D.1) lands and freezes C-Matrix.

### G.D.1 — Filter substrate: sparse GF(2) matrix + provenance map + singleton removal (Sonnet, inflection point)

**Deliverable:**
- New `gnfs/src/filter/` module (sibling to `sieve/`). `gnfs/src/lib.rs` adds `pub mod filter` and
  re-exports the filter entry surface.
- `filter/matrix.rs`: **C-Matrix**. The sparse GF(2) matrix built from a `&[Relation]` over a
  `&FactorBase`:
  - Column layout = `FactorBase::matrix_width()`: rational columns `[0, rational_size())`, algebraic
    columns `[rational_size(), rational_size()+algebraic_size())`, obstruction columns last (the
    sign/−1 column at `obstruction_count` — carried through, G.D does **not** fill quadratic
    characters; that is G.E's).
  - Each relation contributes one row: the GF(2) parity of its exponent vectors (via the frozen
    `rational_row_gf2` / `algebraic_row_gf2`), with the sign bit in the obstruction column.
  - **Relation-provenance map.** Each matrix row records which *original* relation index/indices it
    derives from. Singleton removal drops rows; merging *combines* rows (XOR of GF(2) rows = sum of
    provenance sets). G.F needs this to expand a nullspace vector back to a product of original
    `(a, b)` relations. **This is the load-bearing design call of the session** — freeze the
    provenance representation so G.E/G.F don't disagree.
- `filter/singleton.rs`: singleton removal. A column of Hamming weight ≤ 1 (a prime/ideal in at most
  one surviving relation) cannot appear in any dependency; remove every row that has a 1 in such a
  column, recompute column weights, and **iterate to a fixpoint**. Returns the reduced matrix +
  pruned provenance.
- `filter/mod.rs`: the filter entry surface (`build_matrix`, `remove_singletons`) and the public
  types.

**Key design decisions (juncture fork designs C-Matrix and writes it into Cross-session contracts):**
1. **Matrix representation.** Sparse, row-major (each row = sorted `Vec<usize>` of set-column
   indices, GF(2)) vs column-major (each column = set of rows). Singleton removal needs **column
   weights** (column-major is natural); merging needs **row XOR** (row-major is natural). Decide:
   carry both a row store and a column-weight index, or a single store with a derived weight map.
   Freeze the choice — G.E reads this representation directly. (Block Lanczos/Wiedemann at G.E want
   row-major matrix-vector products; bias toward row-major + a maintained column-weight side table.)
2. **Provenance representation.** `Vec<usize>` (original relation indices) per row, combined by set
   union under merge. Decide whether to store it inside the matrix row struct or as a parallel
   `Vec<Vec<usize>>` indexed by current row. Freeze it — G.F's square-root step is the consumer and
   it is far downstream (do not let it drift).
3. **Obstruction/sign columns.** G.D carries `obstruction_count` columns (currently 1, the sign
   column) through untouched: they are *populated* (sign bit set per row) but never *pruned as
   singletons* (a sign column is structural, not a factor-base prime). Decide and freeze the rule
   "obstruction columns are exempt from singleton/merge elimination" so G.E inherits a stable column
   block. **If G.E turns out to need quadratic-character columns added at filter time, that reaches
   back into C-FactorBase — surface it (see Discoveries & risks).**
4. **Excess accounting.** Define `excess = rows − (columns − obstruction_count)`. Singleton removal
   must not drive excess below a floor (or the matrix is rank-deficient). Decide whether G.D.1
   merely reports excess or enforces a floor; freeze the invariant G.D.2's pruning preserves.

**KAT (≥1 required):**
1. **Matrix construction:** for a small hand-built relation set over a toy `FactorBase`, the matrix
   has exactly `matrix_width()` columns, one row per relation, and each row's set columns match the
   relation's GF(2) parities (cross-checked against `rational_row_gf2`/`algebraic_row_gf2`).
2. **Singleton removal correctness:** a relation set with a known singleton column reduces to the
   hand-computed surviving set; the fixpoint terminates (cascading singletons all removed); the
   provenance map of every surviving row is unchanged (singleton removal drops rows, never merges).
3. **Determinism:** for a fixed relation corpus the singleton-removed matrix dimensions and row
   provenance are deterministic.

**Subtlety:** singleton removal is a **fixpoint**, not a single pass — removing a row can drop a
column's weight to 1 and create a new singleton. A single-pass implementation silently leaves
singletons in the matrix (G.E then wastes a column or, worse, the dependency is spurious). KAT 2
must include a *cascading* singleton (removing relation R₁ makes prime q a singleton, forcing R₂
out). **The juncture fork decides** the representation and writes it into C-Matrix.

**Deferred:** clique pruning + merging (G.D.2); the writeup (G.D.W).

### G.D.2 — Clique/excess pruning + merging (Sonnet, on frozen C-Matrix, sketch)

**Deliverable:** over the singleton-removed matrix, (a) **clique/excess pruning** — the graph view
where relations sharing a prime are connected; prune to reduce final weight while keeping excess
above the G.D.1 floor; and (b) **merging** — eliminate a low-weight column by combining the (few)
relations that contain it: 2-way merge (XOR two rows, union provenance, drop the shared column),
then k-way for columns of weight k, demonstration-fidelity Cavallar weight-cost ordering. Returns the
final reduced matrix + provenance. **Never alters** the singleton-removed baseline (Cat-B/C rule:
the baseline stays constructible for benchmarking the merge weight-saving).

Freezes nothing new (consumes C-Matrix, C-FactorBase).

**KAT (≥1 required):** (a) merging a known weight-2 column yields the hand-computed combined row with
unioned provenance and the column removed; (b) final matrix dimensions are **deterministic** for a
fixed corpus; (c) **optional CADO-NFS oracle:** filtered matrix dimensions within tolerance of CADO
at matched parameters (dev-only, gated/ignored if CADO absent — ROADMAP G.D KAT); (d) end-to-end
provenance: expanding a row of the final matrix through the provenance map recovers a set of original
relations whose GF(2) parities XOR to that row.

**Subtlety (principle-4 annotation):** Cavallar's weighted merge-cost minimisation is a *scale*
optimisation — at toy scale the matrix is small enough that any merge order gives a tractable matrix,
so the weight-saving the heuristic buys is under-exposed. Implement the heuristic (demonstration
fidelity) and annotate the disconnect in the docstring + G.D.W + Track τ.

### G.D.W ◆ — Integrative writeup (Sonnet, sketch)

The filtering chapter (`gnfs/docs/PEDAGOGY.md`, append): the relation corpus as a sparse matrix, the
**graph view** (relations as edges, primes as the shared structure), why singletons can never appear
in a dependency, the excess/clique balance, and merging as a dimension-vs-density trade — with the
provenance map as the thread that lets G.F expand a nullspace vector back to original relations.
Append a G.D benchmark row to `docs/BENCHMARKS.md` (relations in / matrix dimensions out / weight /
merge-saving). Per pacing guidance, integrative writeups are under-scheduled — allocate a full
session. This is where C-Matrix gets its public prose articulation and the G.E/G.F downstream reuse
is surfaced. **First chapter where the new doc-format recommendation applies** (ROADMAP Discoveries):
new display math (any matrix/dimension formulae) uses **MathJax `$$…$$`**; existing inline-Unicode
passages may stay. Its Track τ maths-first sibling (T.G) pairs at the **G.W** ◆ boundary, not here —
G.D.W is a code-tour sub-chapter feeding the eventual G.W chapter.

---

## Cross-session contracts

The scaffolding sessions compose through. The juncture fork at G.D.1 writes the resolved **C-Matrix**
interface into this section before implementation is dispatched.

### C-Relation — relation / exponent-vector format (compiler + KAT) — *frozen at G.C.1 (c1dc0b6)*
**Defined:** G.C.1. **Consumed by (in G.D):** G.D.1 (`Relation`, `ExponentVector` column indices,
`rational_row_gf2` / `algebraic_row_gf2`, `rational_sign`). G.D **reads** relations to build matrix
rows; it does **not** amend C-Relation. The integer-exponent over-specification (for D.A's GF(ℓ))
is irrelevant to G.D, which works over GF(2) parities — but G.D must **not** discard the relations
themselves: the provenance map references original relation indices, so the `Vec<Relation>` stays
live alongside the matrix. Stable for G.D.

### C-FactorBase — two-sided factor base + norm computation (compiler + KAT) — *frozen at G.C.1 (c1dc0b6)*
**Defined:** G.C.1. **Consumed by (in G.D):** G.D.1 (`matrix_width()` = column count;
`rational_size()` / `algebraic_size()` to partition columns; `obstruction_count` for the
exempt-column block; `AlgebraicPrime::index` as the algebraic column index). G.D **reads** the
column layout; it does **not** amend C-FactorBase. *Watch:* if G.D.1 finds G.E needs
quadratic-character columns reserved at filter time beyond the single sign column, that is a reach
back into the frozen `obstruction_count` slot — an **additive-reshard** to surface, not an
internal-continue (see Discoveries & risks). Stable for G.D as currently understood.

### C-Matrix — filtered sparse GF(2) matrix + relation-provenance map (compiler + KAT) — *to be frozen at G.D.1*
**Defined:** G.D.1. **Consumed by:** G.D.2 (clique/merge operates on it), **G.E (linear algebra
reads the matrix for the nullspace computation)**, and **G.F (square root expands a nullspace vector
through the provenance map back to original relations)**. The sparse GF(2) matrix (row store +
column-weight index), the relation-provenance map (current row → set of original relation indices,
combined by union under merge), the obstruction-column block (exempt from elimination), and the
excess accounting (`excess = rows − (columns − obstruction_count)`). This is **internal to Track G**
(not adapted cross-track) — but it spans three sub-tracks (G.D → G.E → G.F), so over-specify the
provenance map now: store original relation indices, not pre-reduced row sums, so G.F can recover
the actual `(a, b)` pairs. The juncture fork at G.D.1 writes the resolved interface here before
G.D.2 is dispatched.

**Resolved interface (juncture-designed at G.D.1):**

```rust
// ── gnfs/src/filter/matrix.rs ────────────────────────────────────────────────

/// Minimum excess G.D.2 pruning must preserve.
///
/// `excess = rows − (columns − obstruction_count)`. At toy scale any positive excess
/// suffices for a non-trivial nullspace; 20 is a conservative floor that keeps the
/// matrix well-overdetermined. Annotated as scale-dependent (principle-4): at
/// cryptographic scale the floor is typically set to ~200 or a fraction of the column
/// count. G.D.1 defines the constant; G.D.2 enforces it during clique pruning.
pub const EXCESS_FLOOR: usize = 20;

/// One row of the sparse GF(2) matrix, with its relation-provenance record.
///
/// # Representation
///
/// `cols` is a sorted `Vec<usize>` of column indices where this row has a 1 (GF(2)).
/// The column layout matches `FactorBase::matrix_width()`:
///   - `[0, rational_size())` — rational factor-base columns
///   - `[rational_size(), rational_size() + algebraic_size())` — algebraic columns
///   - `[rational_size() + algebraic_size(), matrix_width())` — obstruction columns
///     (sign at `rational_size() + algebraic_size()`; quadratic-character columns, if
///     any, follow — G.E fills those; G.D carries them as zeros)
///
/// # Provenance
///
/// `provenance` is a sorted, deduplicated `Vec<usize>` of *original* relation indices
/// (indices into the `Vec<Relation>` passed to `build_matrix`). For a freshly built
/// row, `provenance = [original_index]`. Under merge (G.D.2), provenance is combined
/// by sorted union. G.F expands a nullspace vector by collecting `row.provenance` for
/// each selected row and recovering the original `(a, b)` pairs.
///
/// Provenance stores original indices, not pre-reduced row sums — over-specified per
/// the Discoveries & risks note so G.F can recover actual `(a, b)` pairs without
/// re-deriving anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixRow {
    /// Sorted column indices where this GF(2) row has a 1.
    pub cols: Vec<usize>,
    /// Original relation indices this row derives from (sorted, deduplicated).
    pub provenance: Vec<usize>,
}

impl MatrixRow {
    /// XOR this row with `other` in GF(2), unioning their provenance sets.
    ///
    /// Used by G.D.2 merging: combining two rows that share a column eliminates
    /// that column (symmetric difference of `cols`) and unions their provenance.
    ///
    /// :param other: The row to XOR with.
    /// :returns: A new `MatrixRow` with the symmetric-difference column set and
    ///           the union of both provenance sets.
    pub fn xor_merge(&self, other: &MatrixRow) -> MatrixRow { ... }
}

/// Sparse GF(2) matrix over a factor base, with relation-provenance map.
///
/// # Column layout
///
/// Total columns = `FactorBase::matrix_width()` = `rational_size() + algebraic_size()
/// + obstruction_count`. The obstruction block starts at `obstruction_col_start` =
/// `rational_size() + algebraic_size()`. Singleton removal and merging (G.D.2) skip
/// any column `>= obstruction_col_start` — obstruction columns are structural and are
/// never pruned.
///
/// # Column-weight index
///
/// `col_weights[c]` = number of rows with a 1 in column `c`. Maintained in sync with
/// `rows` by all mutating operations (`remove_row`, `xor_merge_rows`). G.D.1's
/// singleton removal reads `col_weights` to find weight-≤1 columns without scanning
/// all rows. G.D.2's merge step reads `col_weights` to find low-weight columns to
/// eliminate.
///
/// # Excess
///
/// `excess()` = `rows.len() − (num_cols − obstruction_count)`. G.D.1 reports it;
/// G.D.2 enforces `excess() >= EXCESS_FLOOR` during clique pruning.
#[derive(Debug, Clone)]
pub struct SparseMatrix {
    /// The rows of the matrix (row-major sparse GF(2) store).
    pub rows: Vec<MatrixRow>,
    /// Total number of columns (= `FactorBase::matrix_width()`).
    pub num_cols: usize,
    /// Column index at which the obstruction block begins
    /// (= `rational_size() + algebraic_size()`).
    pub obstruction_col_start: usize,
    /// Number of obstruction columns (= `FactorBase::obstruction_count`).
    pub obstruction_count: usize,
    /// Per-column Hamming weight (length = `num_cols`), kept in sync with `rows`.
    pub col_weights: Vec<u32>,
}

impl SparseMatrix {
    /// Current excess: `rows.len() − (num_cols − obstruction_count)`.
    ///
    /// A positive excess means the system is overdetermined (more relations than
    /// independent columns), which is required for a non-trivial GF(2) nullspace.
    /// G.D.2 must not drive this below `EXCESS_FLOOR`.
    pub fn excess(&self) -> isize { ... }

    /// Remove a row by index, updating `col_weights`.
    ///
    /// Used by singleton removal (G.D.1) and clique pruning (G.D.2).
    /// Panics if `row_idx` is out of bounds.
    ///
    /// :param row_idx: Index of the row to remove.
    pub fn remove_row(&mut self, row_idx: usize) { ... }
}

// ── gnfs/src/filter/mod.rs ───────────────────────────────────────────────────

/// Build the initial sparse GF(2) matrix from a relation corpus and factor base.
///
/// Each relation in `relations` contributes one row. Column layout:
///   - Rational columns `[0, fb.rational_size())`: GF(2) parities of rational exponents.
///   - Algebraic columns `[fb.rational_size(), fb.rational_size() + fb.algebraic_size())`:
///     GF(2) parities of algebraic exponents.
///   - Obstruction columns `[fb.rational_size() + fb.algebraic_size(), fb.matrix_width())`:
///     sign bit at `fb.rational_size() + fb.algebraic_size()` (from `relation.rational_sign`);
///     remaining obstruction columns (quadratic characters) set to 0 — G.E fills them.
///
/// Note: `Relation::rational_row_gf2` places the sign at local index 0 of its return
/// value; `build_matrix` re-maps it to the global obstruction column index.
///
/// Provenance for row `i` is `[i]` (the original relation index).
///
/// :param relations: The relation corpus (indices are preserved as provenance).
/// :param fb: The factor base (column layout source).
/// :returns: A `SparseMatrix` with `relations.len()` rows and `fb.matrix_width()` columns.
pub fn build_matrix(relations: &[Relation], fb: &FactorBase) -> SparseMatrix { ... }

// ── gnfs/src/filter/singleton.rs ─────────────────────────────────────────────

/// Remove singleton columns to a fixpoint, returning the reduced matrix.
///
/// A column of Hamming weight ≤ 1 (a prime/ideal appearing in at most one surviving
/// relation) cannot be part of any GF(2) dependency. The row containing it is removed,
/// which may reduce other columns to weight 1, creating new singletons. This iterates
/// until no weight-≤1 column remains among the non-obstruction columns.
///
/// Obstruction columns (`>= matrix.obstruction_col_start`) are exempt: they are
/// structural and are never treated as singletons regardless of their weight.
///
/// Provenance is preserved unchanged: singleton removal drops rows, never merges them,
/// so each surviving row's `provenance` is its original singleton set.
///
/// :param matrix: The matrix to reduce (consumed; returns the reduced matrix).
/// :returns: The singleton-removed `SparseMatrix`.
pub fn remove_singletons(matrix: SparseMatrix) -> SparseMatrix { ... }
```

**Design decisions frozen:**

1. **Matrix representation:** Row-major (`rows: Vec<MatrixRow>`, each row a sorted `Vec<usize>` of
   set-column indices) + maintained column-weight side table (`col_weights: Vec<u32>`, length =
   `num_cols`, kept in sync by all mutating operations). G.E reads the row-major store directly for
   block Lanczos/Wiedemann matrix-vector products; singleton removal and G.D.2 merging read
   `col_weights` for O(1) weight lookup.

2. **Provenance representation:** Inline in `MatrixRow` as `provenance: Vec<usize>` (sorted,
   deduplicated original relation indices). Co-located with the row data so row operations keep
   provenance in sync. Under merge (G.D.2), combined by sorted union via `xor_merge`. G.F expands
   a nullspace vector by collecting `row.provenance` for each selected row. Stores original indices,
   not pre-reduced row sums — over-specified per Discoveries & risks.

3. **Obstruction/sign columns:** Obstruction columns occupy `[obstruction_col_start, num_cols)` where
   `obstruction_col_start = rational_size() + algebraic_size()`. Sign bit from `rational_sign` is
   placed at column `obstruction_col_start` (re-mapped from `rational_row_gf2`'s local index-0
   layout). Singleton removal and G.D.2 merging skip all columns `>= obstruction_col_start`. G.E
   inherits a stable obstruction block; quadratic-character columns within the block are zero until
   G.E fills them.

4. **Excess accounting:** `excess() = rows.len() as isize − (num_cols − obstruction_count) as isize`.
   G.D.1 exposes `SparseMatrix::excess()` and reports it; it does **not** enforce a floor. The floor
   invariant is `EXCESS_FLOOR = 20` (named constant, annotated as scale-dependent). G.D.2's clique
   pruning must not drive `excess()` below `EXCESS_FLOOR`.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| G.D.1 | Filter substrate: sparse GF(2) matrix + provenance + singletons | done | a0e854b | C-Matrix |
| G.D.2 | Clique/excess pruning + merging | done | d424f53 | — |
| G.D.W | Integrative writeup (filtering chapter) | done | 7762339 | — |

Contracts frozen before G.D: C-Fp (cf00ed5), C-numth (α.2), C-NF (bdba6f5), C-Ideal (05b27c8),
C-Res (bcd63cd), C-Dedekind (7844773), C-PolyPair (2f43f99), C-Score (00aa32d), C-FactorBase
(c1dc0b6), C-Relation (c1dc0b6). G.D opens over the frozen G.A substrate, G.B polynomial-selection
layer, and G.C sieve layer.

G.D ◆ boundary: still-on-intent (2026-06-07). All G.D sessions complete. C-Matrix frozen and ready for G.E linear algebra. No reach-back into C-FactorBase (obstruction_count=1 unchanged). Cavallar principle-4 annotation present in code, PEDAGOGY.md, and BENCHMARKS.md.

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume. (Empty at sub-track open; the G.D.1 fork writes
the first entry when it freezes C-Matrix.)

### G.D.1 — 2026-06-07
Discovery/flex: Inflection-point design completed; C-Matrix frozen as designed. Row-major SparseMatrix + inline MatrixRow provenance + col_weights side table + EXCESS_FLOOR=20 constant. Obstruction columns exempt from singleton/merge elimination via obstruction_col_start field.
Affected: C-Matrix (frozen at a0e854b)
Deferred: no
Texture: rational_row_gf2 sign-bit remapping (local index 0 → global obstruction_col_start) is the one non-obvious build_matrix detail; KAT 2 cascading-singleton test exercises the fixpoint correctness obligation. EXCESS_FLOOR annotated as scale-dependent (principle-4). G.D.2 consumes C-Matrix directly; no contract flex needed.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **C-Matrix provenance is the load-bearing seam (G.D → G.E → G.F) — over-specify, do not narrow
  later.** Store original relation indices per row (combined by union under merge), not pre-reduced
  row sums. G.F's square-root step expands a nullspace vector back to a *product of original `(a, b)`
  relations*; if provenance is lost during merge, the square root cannot be formed. Re-deriving
  provenance after G.E or G.F consumes the matrix would be a **destructive reshard**. Designing the
  provenance shape now, before G.F exists, is the over-specify rule applied to a compiler+test
  contract internal to Track G.

- **Quadratic-character / obstruction columns at filter vs linalg time (reach into frozen
  C-FactorBase).** G.C.1 reserved `obstruction_count = 1` (the sign column). G.E's GF(2) linear
  algebra also needs quadratic-character columns to guarantee the algebraic square root exists. The
  *expectation* is those columns are added at **G.E** time, not G.D — G.D merely carries the sign
  column and exempts it from elimination. If G.D.1 finds they must be reserved at filter time, that
  is a reach back into the frozen `obstruction_count` slot: an **additive-reshard** (widen
  `obstruction_count`, a mechanical change to a slot designed to be widened) — surface for sign-off,
  do **not** silently grow it. Only if it forces changing the *meaning* of frozen C-FactorBase
  columns is it a **destructive-HALT**.

- **Singleton removal must reach a fixpoint, not a single pass (correctness, lever 4).** Removing a
  relation can drop a column to weight 1, creating a new singleton; a single-pass filter silently
  leaves singletons in the matrix, wasting a G.E column or admitting a spurious dependency. This is
  an *internal-continue* correctness obligation at G.D.1 (iterate to fixpoint, KAT a cascading
  singleton), not a halt.

- **Excess floor (rank deficiency).** Aggressive singleton/clique pruning can drive
  `excess = rows − (columns − obstruction_count)` below the floor needed for a non-trivial
  nullspace, producing a matrix G.E cannot solve. G.D.1 defines the floor invariant and G.D.2's
  pruning preserves it. If a toy corpus is too thin to clear the floor after filtering, that is an
  *internal-continue* (collect more relations / shrink the factor base — a G.C parameter), not a
  halt, unless it reveals C-Relation under-collected in a way that reaches back into G.C (then
  surface; not expected).

- **CADO-NFS oracle availability.** The matrix-dimension cross-check (G.D.2 KAT c) depends on
  CADO-NFS being installed as a dev oracle. If absent, gate that KAT behind an ignored/featured
  test; the deterministic dimension KAT (G.D.2 KAT b) carries the reproducibility burden without
  CADO. Same uniform-gating policy still open project-wide (ROADMAP Discoveries, reference-oracle
  entry) — G.D inherits the per-test workaround; it does not resolve the policy.

- **Merge weight-saving under-exposed at toy scale (principle 4).** Cavallar's weighted merge-cost
  ordering is a *scale* optimisation; at toy scale any merge order yields a tractable matrix, so the
  weight-saving is under-exposed. Annotate the disconnect in code + G.D.W + Track τ. Not a
  correctness risk — a pedagogy-honesty obligation.

- **`filter/` module is fresh ambient surface (lever 1).** Second non-`polyselect`/`sieve` module in
  `gnfs`; G.D.1 sets `src/filter/` layout, which G.E (`src/linalg/`?) and G.F (`src/sqrt/`?) will
  sibling. Keep the crate-internal module structure open rather than over-committing at G.D.1.

- **Documentation math-rendering format (static-frame, does not block G.D).** Recommended at the
  G.C ◆ boundary: **Markdown + MathJax** (`$…$` / `$$…$$`), superseding the earlier "rST vs Markdown
  TBD," to be ratified at T.0 (ROADMAP Discoveries log). G.D produces no prose math until G.D.W;
  that chapter is the first place the convention applies (new display math uses MathJax, existing
  inline-Unicode may stay). Not a code contract — does not gate any G.D implementation session.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase β / Track G section: G.D entry, Cross-track contracts C1/C2/C3,
  Discoveries log — including the documentation-format and reference-oracle entries, and the G.C
  log-sieve / base-m / Dickman-ρ findings) before any G.D session.
- Read `gnfs/docs/PEDAGOGY.md` (the G.C sieving chapter and the G.B polynomial-selection chapter) and
  `shared/numfield/docs/PEDAGOGY.md` for the pedagogical register (rST docstrings, KATs per session,
  narrative chapter at each ◆ boundary). New `gnfs::filter` work matches it.
- **Register: PEDAGOGY.** This is a reference library — code is teaching material. Match the G.C.W /
  G.B.W / G.A.W chapter genre and quality.
- **Tier routing:** G.D.1 is the `@plan` inflection (Sonnet juncture fork per `juncture-tier: sonnet`
  — freezes C-Matrix, a Track-G-internal contract; opt-down justified in the header). G.D.2 and G.D.W
  are Sonnet (`@build`). No G.D session is Opus.
- **Invariants to preserve:** the G.A substrate contracts (C-NF, C-Res, C-numth, C-Ideal,
  C-Dedekind), the G.B contracts (C-PolyPair, C-Score), and the **G.C contracts (C-Relation,
  C-FactorBase)** are frozen — G.D consumes, never amends them. The `rho` crate, `gnfs::polyselect`,
  and `gnfs::sieve` stay untouched (G.D *adds* `gnfs::filter`).
- **CADO-NFS / msieve are dev-only oracles**, never on a build path (ROADMAP scoping principle 3).
- **Doc-format note (for G.D.W only):** new display mathematics uses MathJax (`$$…$$`); inline-Unicode
  may stay. Recommendation pending T.0 ratification (ROADMAP Discoveries log).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the `filter/`-module
  shard pattern is unproven for this crate (first filter module; C-Matrix is the first matrix/
  provenance contract), so halt at the G.D.1 inflection and again at the G.D.W ◆ boundary for review.
