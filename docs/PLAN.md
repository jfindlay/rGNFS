<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-D continues (D.B — linear algebra over F_ℓ)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default; does not opt down**, on a deliberate
lever-4 call (recorded here, distinct from D.A's rationale). Applying the five-lever law to D.B:
lever 3 (design-error cost) is **moderate** — the substrate frozen here (C-LinAlgFl) is
*sub-track-internal* (consumed by D.B.2 and D.C, **not** E.C directly; the cross-track NFS-DL solver
interface C2 freezes later, at D.C), so it does not carry D.A.1's cross-track weight. Lever 5
(inner-loop bandwidth) is **strong** — mature `cargo test --workspace` gate, 18 KAT files, existing
`lanczos_kat.rs` / `wiedemann_kat.rs` to mirror, a PARI discrete-log cross-check available, and the
F_ℓ work *reuses G.E's already-understood block-solver structure*. Levers 3+5 would license opting
the juncture tier **down** to Sonnet. **Lever 4 (correctness-criticality) overrides**: the F_ℓ
kernel D.B produces *is* the virtual-logarithm table — a silently-wrong nullspace over F_ℓ corrupts
every downstream DL solution (D.C's descent, ultimately E.C's MOV bridge), and the failure mode is
silent (a plausible-looking but wrong log). On the conservative reading, the Opus register is held at
the D.B.1 freeze confirmation and the D.B.2 ◆ boundary despite the strong inner loop. *(Contrast
D.A, where juncture-tier held at Opus on levers 3+4 jointly — the cross-track freeze. Here it is
lever 4 alone; lever 3 has relaxed.)*

Last rewrite: D.A.2 ◆ boundary crossed (Track-D entry complete; the DL relation arc D.A.1 → D.A.2 is
coherent, ledger reconciled 2026-06-08). This plan opens **Track D's second sub-track, D.B — linear
algebra over F_ℓ**: solve the augmented DL relation matrix (from D.A.2) over the prime field F_ℓ to
recover the virtual-logarithm table.

---

## Purpose (design intent)

Per ROADMAP: D.B is "Block Wiedemann generalised; block Lanczos with the F_ℓ care. KAT: cross-check
with PARI's discrete-log functionality." It is the F_ℓ analogue of Track-G's GF(2) linear algebra
(G.E): where factoring solves `A x = 0` over GF(2) to find a dependency, NFS-DL solves the augmented
relation system over F_ℓ (ℓ = the target subgroup order) to recover the **virtual logarithms** of the
factor-base elements. Two sessions:

1. **The F_ℓ linear-algebra substrate + block Lanczos (D.B.1).** A **parallel F_ℓ linalg module**
   (`gnfs/src/dl/linalg/`), distinct from the frozen GF(2) C-LinAlg — *not* a generic refactor of
   it (see the architecture decision in Discoveries). New F_ℓ-native types (an F_ℓ block vector over
   `[Fp; BLOCK_WIDTH]` arrays rather than bit-packed `u64` words; an F_ℓ matrix operator), the
   **matrix-build seam** that turns D.A.2's `DLMatrix` into a concrete F_ℓ system (reduce the `u32`
   exponent columns mod ℓ; the Schirokauer columns arrive already in ℤ/ℓ), and **block Lanczos over
   F_ℓ** as the primary solver — mirroring G.E's "block Lanczos as primary." **Freezes
   C-LinAlgFl** (the F_ℓ block-solver substrate interface). This is the substrate session whose
   interface binds D.B.2 (Wiedemann reuses the same vector/operator types) and D.C (descent reads
   the recovered virtual-log table). Over-specify deliberately within reason.

2. **Block Wiedemann over F_ℓ + virtual-log recovery (D.B.2 ◆).** Block Wiedemann as the **secondary**
   solver (mirroring G.E's "block Wiedemann as secondary") — the F_ℓ generalisation needs
   Berlekamp–Massey over F_ℓ, not GF(2). Then **recover the virtual-log table** from the F_ℓ kernel
   (the solution vector's entries are the virtual logarithms of the factor-base elements mod ℓ), and
   the **end-to-end toy-F_p DL KAT**: recover a known toy discrete log through the full
   relation-collection → F_ℓ-solve → log-recovery path, cross-checked against a hand-computed
   reference and (stub-gated) against PARI's discrete log. This session crosses the **D.B ◆
   boundary**.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing D.C's individual logarithm
or special-q descent here — that is the next sub-track; D.B stops at the virtual-log table for
factor-base elements) and **rigidity** (forcing the F_ℓ solver into the GF(2) C-LinAlg types when
the field arithmetic genuinely needs the parallel `[Fp; W]` representation — the *confirmed*
architecture, not a silent squeeze; and conversely, prematurely unifying GF(2) and F_ℓ behind a
shared trait, which is explicitly **deferred**, see Discoveries).

**Scoping discipline (ROADMAP three-way split, applied here).** Block Lanczos and block Wiedemann
over F_ℓ are **algorithmic content included in full** (principle 1) — the F_ℓ care (inner-product
inversion, the self-orthogonal-vector degeneracy at small ℓ, Berlekamp–Massey over F_ℓ) is the
defining difference from GF(2) and is implemented head-on. Wiedemann stays at the **scalar
(single-pair) demonstration fidelity** the GF(2) version uses (principle 2 — the block-parallel
payoff is an NFS-scale phenomenon, annotated, not engineered). No engineering optimisations
(principle 3). PARI remains a dev-only oracle (D.B.2 cross-check, stub-gated), never on a build path.
C1 `Uint<4>` stays as-is per the ROADMAP width policy (D.B touches factor-base *indices* and F_ℓ
elements, not the smoothness width — the width is not in D.B's surface).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Confirmed by survey:
no Makefile / justfile / xtask wrapper exists in the workspace; raw `cargo` is the only CI surface.
Rust's compiler is the type gate; `cargo test` subsumes it on a clean build, so one green
`cargo test --workspace` satisfies both. D.B is **code** — the gate is a real inner loop (this is
lever 5, strong), which is exactly why the juncture-tier hold at Opus rests on lever 4 alone, not on
a weak inner loop.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| D.B.1 `@plan` | F_ℓ linear-algebra substrate + block Lanczos: build F_ℓ matrix from DLMatrix, solve primary; freeze C-LinAlgFl | A | Sonnet | C-DLRelation (DLMatrix), C-LinAlg (GF(2) pattern, read-only), Fp (`shared-field`) | `gnfs/src/dl/linalg/mod.rs` (new), `gnfs/src/dl/linalg/blockvec_fl.rs` (new), `gnfs/src/dl/linalg/lanczos_fl.rs` (new), `gnfs/src/dl/mod.rs` (re-export), `gnfs/tests/dl_linalg_kat.rs` (new) |
| D.B.2 ◆ | Block Wiedemann over F_ℓ + virtual-log recovery + toy-F_p DL KAT (PARI cross-check) | B | Sonnet | C-LinAlgFl, C-DLRelation, C-Schirokauer, (PARI oracle) | `gnfs/src/dl/linalg/wiedemann_fl.rs` (new), `gnfs/src/dl/linalg/mod.rs` (extend: virtual-log recovery), `gnfs/tests/dl_linalg_kat.rs` (extend), `gnfs/tests/dl_end_to_end_kat.rs` (new) |

**Sequencing notes.** **D.B.1 must precede D.B.2** (D.B.2's Wiedemann reuses the F_ℓ vector/operator
types and the virtual-log recovery reads the solved kernel). The single `@plan` marker sits on
**D.B.1** — a post-landing freeze confirmation for C-LinAlgFl, lighter than D.A.1's cross-track
confirmation because C-LinAlgFl is sub-track-internal (its first consumer is D.B.2, within this
plan); confirm the substrate and the matrix-build correctness before D.B.2 is dispatched.
**D.B.2 ◆** is the D.B sub-track boundary.

**Why 2 sessions (matches ROADMAP allotment).** The one-line-commit-title corollary: D.B.1
("F_ℓ linalg substrate + block Lanczos") and D.B.2 ("block Wiedemann + virtual-log recovery") are
two distinct commit titles, split on a **contract-sharp boundary** (D.B.1 freezes C-LinAlgFl; D.B.2
consumes it). The split is the **solver-type split** (Lanczos-primary | Wiedemann-secondary +
recovery), mirroring G.E's primary/secondary ordering — not the substrate/algorithm split, because
the F_ℓ substrate and the primary solver share the same vector/operator design surface and the
recovery step is the natural KAT vehicle for the *secondary* solver. They are **not** mergeable (the
freeze boundary) and **not** further splittable below the floor — block Lanczos over F_ℓ is the
irreducible substrate+primary unit (lever 2); fracturing the substrate from its first solver would
split an irreducible unit just to hit a LOC number (forbidden). D.B.1 is a Sonnet substrate session;
if the F_ℓ substrate + Lanczos overruns the band, the contract-sharp split surfaced at the D.B.1
`@plan` juncture is the **F_ℓ substrate** (`blockvec_fl` + matrix-build + operator) vs **block
Lanczos** (the solver) — not pre-committed.

---

## Session detail

D.B.1 is crisp (its design surface is the C-LinAlgFl freeze + the DLMatrix→F_ℓ matrix-build seam,
resolved in-session as the substrate's own work). D.B.2 is sketched at post-substrate fidelity —
correct to leave its precise shape open until D.B.1 freezes the F_ℓ types and recovery seam.

### D.B.1 — F_ℓ linear-algebra substrate + block Lanczos (Sonnet, substrate, `@plan`)

**Deliverable:** the F_ℓ linear-algebra substrate and the primary solver.
- **Parallel F_ℓ linalg module.** A new `gnfs/src/dl/linalg/` module, **separate from the frozen
  GF(2) `gnfs/src/linalg/`** (architecture decision, Discoveries: parallel module, not a generic
  refactor; GF(2) C-LinAlg stays frozen and untouched). The GF(2) module's own design notes
  (`blockvec.rs:26–30`, `operator.rs:29–32`) anticipate exactly this seam.
- **F_ℓ block vector.** An `FlBlockVec` (or similar) over `[Fp; BLOCK_WIDTH]` arrays (not bit-packed
  `u64` words — F_ℓ scalars do not pack into bits). Uses the existing `shared-field` `Fp<L>` trait
  (`FpNaive4` / `FpMonty4`), which already provides `add`, `mul`, `inv`, `from_u64`, `from_uint`.
  The F_ℓ inner-product matrix is real field arithmetic (sum of products in F_ℓ), not the GF(2)
  parity trick.
- **The matrix-build seam (DLMatrix → F_ℓ system).** Build a concrete F_ℓ matrix from D.A.2's
  `DLMatrix`: reduce each relation's `u32` rational+algebraic `ExponentVector` entries **mod ℓ**, and
  append the `schirokauer_cols` (already in ℤ/ℓ — D.A.2 stored them reduced; the *exponents* are raw,
  per the D.A.2 ◆ note "DLMatrix stores raw integers; D.B must handle reduction"). Column layout
  follows `DLMatrix::num_cols`: `rational | algebraic | schirokauer`.
- **Block Lanczos over F_ℓ** (primary). Generalise the GF(2) block Lanczos to F_ℓ: the inner-product
  matrices are inverted in F_ℓ (Fermat `inv`), with the **self-orthogonal-vector degeneracy** care
  block Lanczos needs over a field (the GF(2) version's parity pivot becomes an F_ℓ Gaussian pivot
  with explicit inversion). Returns the F_ℓ solution / kernel basis.
- **Freeze C-LinAlgFl** (the F_ℓ block-solver substrate interface — see Cross-session contracts).

**Key design decisions (the C-LinAlgFl freeze surface — the `@plan` confirmation):**
1. **F_ℓ block-vector representation & ℓ-handling:** the `[Fp; BLOCK_WIDTH]` layout vs a
   `Vec<Fp>`-per-vector layout; how ℓ (and the `Uint<L>` modulus it implies) is threaded through the
   types (`Fp<L>` carries the prime as a `&Uint<L>` parameter — the operator/vector must hold or
   thread it). **The `BigInt` → `Uint<L>` / `Fp` conversion gap:** `Fp` has no `from_bigint`;
   D.B.1 must convert the Schirokauer `BigInt` columns and the reduced exponents into `Fp` (decide:
   `BigInt → Uint<L>` helper, or constrain toy ℓ to `u64` and use `from_u64`). Surface this at the
   freeze.
2. **C-LinAlgFl shape:** the operator trait/struct (`apply` / `apply_transpose` over `FlBlockVec`),
   the solver entry signature (`block_lanczos_fl(op, ell, seed) -> FlSolution`), and the matrix-build
   entry (`build_fl_matrix(&DLMatrix, ell) -> ...`). Over-specify lightly: carry the kernel/solution
   shape D.B.2's recovery and D.C's descent will read.
3. **Reuse vs parallel confirmation:** confirm the parallel-module decision held in implementation —
   that no genuine unification opportunity emerged that should instead trigger the deferred
   consolidation (Discoveries). If one did, surface it as a discovery, do not unify inline.

**KAT (≥1 required, in `gnfs/tests/dl_linalg_kat.rs`):** (a) F_ℓ inner-product / arithmetic KAT —
the `FlBlockVec` operations match hand-computed F_ℓ values; (b) matrix-build KAT — a small `DLMatrix`
builds the expected F_ℓ matrix (exponents reduced mod ℓ, Schirokauer cols appended in the right
columns); (c) **block-Lanczos-F_ℓ KAT** — solve a small known F_ℓ system and verify the solution
(`A·x ≡ 0 mod ℓ` for a kernel vector, or the known solution for a non-degenerate system).
`cargo test --workspace` green.

**Subtlety:** the load-bearing F_ℓ-specific care is the **inner-product inversion + self-orthogonal
degeneracy** (block Lanczos over a field can produce a singular inner-product block that GF(2)'s
parity arithmetic masks) and the **`BigInt`/`Fp` conversion seam**. The matrix-build correctness
(reducing the right columns mod ℓ, in the right order) is where a silent error enters the
virtual-log table — KAT it explicitly (decision (b)).

**Deferred:** block Wiedemann over F_ℓ (D.B.2); virtual-log recovery + end-to-end DL (D.B.2);
individual logarithm + special-q descent (D.C); the C2 NFS-DL solver interface (D.C); any GF(2)↔F_ℓ
code unification (deferred consolidation, Discoveries).

**`@plan` confirmation (post-landing, T0/Opus, one-shot).** Page a `@plan-juncture` fork to confirm
the **C-LinAlgFl freeze** before D.B.2 is dispatched: (1) the F_ℓ substrate interface is complete and
mutually consistent (vector/operator/solver signatures, ℓ-threading, the `BigInt`/`Fp` conversion
resolved); (2) the matrix-build correctly reduces DLMatrix exponents mod ℓ and lays out the columns
per `DLMatrix::num_cols` (KAT-confirmed); (3) block Lanczos over F_ℓ recovers a known small kernel
(no self-orthogonal degeneracy mishandled); (4) the parallel-module decision held — no un-surfaced
unification was forced inline. One-shot findings; does not implement. Held at **Opus** on lever 4
(the kernel feeds the downstream virtual-log table), per the header.

### D.B.2 ◆ — Block Wiedemann over F_ℓ + virtual-log recovery (Sonnet, algorithm, sketch)

**Deliverable:** the secondary F_ℓ solver and the virtual-log recovery, closing the D.B arc. Sketch
(crisp shape resolved once D.B.1's C-LinAlgFl freezes):
- **Block Wiedemann over F_ℓ** (secondary). Generalise the GF(2) scalar Wiedemann to F_ℓ: the Krylov
  sequence `s_i = x^T B^i y` is over F_ℓ, and **Berlekamp–Massey runs over F_ℓ** (the GF(2) version's
  bit arithmetic becomes F_ℓ field arithmetic in the minimal-polynomial recurrence — the genuine
  generalisation). Reuses the D.B.1 `FlBlockVec` / operator. Stays scalar (single-pair)
  demonstration fidelity, matching the GF(2) version (principle-4 annotation: block-parallel payoff
  is NFS-scale).
- **Virtual-log recovery.** From the solved F_ℓ system (D.B.1 Lanczos or D.B.2 Wiedemann), extract
  the **virtual logarithms of the factor-base elements** mod ℓ — the solution-vector entries are the
  `log_g` of each factor-base prime / ideal (plus the Schirokauer-column corrections). This is the
  table D.C's individual-log descent consumes.
- **End-to-end toy-F_p DL KAT** (◆ vehicle). Recover a known toy discrete log through the full path:
  relation collection (D.A.2) → F_ℓ matrix-build (D.B.1) → solve (Lanczos and Wiedemann) →
  virtual-log recovery, cross-checked against a hand-computed reference and (stub-gated) PARI.

Consumes C-LinAlgFl, C-DLRelation, C-Schirokauer, (PARI oracle). Freezes nothing new (it is the
algorithm+integration session consuming D.B.1's substrate). Note: **D.B does *not* freeze the
cross-track C2 NFS-DL solver interface** — that is D.C's, once individual-log + descent exist;
D.B.2 produces the virtual-log *table*, not the `solve_dl(g, h, …)` entry point.

**KAT (≥1 required):** (a) block-Wiedemann-F_ℓ KAT — Wiedemann recovers the same kernel/solution as
Lanczos on a known small F_ℓ system (cross-validates the two solvers); (b) **end-to-end toy-F_p DL
KAT** — recover a known toy discrete log, cross-checked against a hand-computed reference; (c) PARI
cross-check — `#[ignore = "PARI not installed; run manually when available"]` stub (matching the
established D.A.2 `kat_h_pari_oracle` pattern; no feature flag, no subprocess in CI). The
deterministic non-PARI KATs carry the reproducibility burden. `cargo test --workspace` green.

**Subtlety:** the load-bearing judgments are **Berlekamp–Massey over F_ℓ** (the GF(2)→F_ℓ
generalisation of the minimal-polynomial recurrence — a real algebra change, not a wrapper) and the
**virtual-log recovery correctness** (which solution-vector entries are which factor-base elements'
logs, and how the Schirokauer columns enter the recovered log). If the recovery reveals the
C-LinAlgFl solution shape can't carry what D.C needs, that is a **contract discovery**
(additive-reshard) surfaced at the ◆ boundary, not a silent squeeze. This is the **D.B ◆ boundary** —
re-read the Purpose intent and verify the D.B arc (D.B.1 → D.B.2) is coherent before crossing into
D.C.

---

## Cross-session contracts

D.B freezes one new code contract (C-LinAlgFl) at D.B.1 and reads the frozen Track-G / Track-D
contracts. **C-LinAlgFl is sub-track-internal** (consumed by D.B.2 and D.C), distinguishing it from
D.A's cross-track C-Schirokauer / C-DLRelation.

### C-LinAlgFl — F_ℓ block-solver substrate (compiler + KAT) — *to be frozen at D.B.1*

**Defined:** D.B.1. **Consumed by:** D.B.2 (Wiedemann reuses the vector/operator types; recovery
reads the solved kernel), D.C (descent reads the recovered virtual-log table). Compiler-enforced
(the vector/operator/solver signatures) + KAT-enforced (known-kernel solve + matrix-build
correctness). **Not** consumed directly by E.C — the cross-track NFS-DL solver interface is C2,
frozen at D.C.

**Frozen interface (`gnfs/src/dl/linalg/`): *to be resolved at D.B.1 and written here by the
`@plan-juncture` fork at execution time.*** The shape, sketched for the freeze (over-specify
lightly):

- **F_ℓ block vector** — `FlBlockVec` over `[Fp; BLOCK_WIDTH]` arrays (the F_ℓ analogue of GF(2)'s
  bit-packed `BlockVec`), with `zeros`, `inner_product_matrix` (F_ℓ, returns an F_ℓ
  `BLOCK_WIDTH × BLOCK_WIDTH` block), and component-wise F_ℓ add. Carries / threads the modulus
  (`Fp<L>` needs `&Uint<L>`).
- **F_ℓ matrix operator** — `apply` / `apply_transpose` over `FlBlockVec`, built from the F_ℓ matrix.
- **Matrix-build entry** — `build_fl_matrix(&DLMatrix, ell) -> <F_ℓ matrix>`: reduces `u32` exponent
  columns mod ℓ, appends Schirokauer columns (already mod ℓ), column layout `rational | algebraic |
  schirokauer` per `DLMatrix::num_cols`.
- **Solver entries** — `block_lanczos_fl(op, ell, seed) -> FlSolution` (D.B.1) and
  `block_wiedemann_fl(...)` (D.B.2, same return shape). The solution shape carries the kernel /
  particular solution D.B.2's virtual-log recovery and D.C's descent read.
- **`BigInt`/`Fp` conversion** — the resolved approach for turning Schirokauer `BigInt` values into
  `Fp` (helper or toy-ℓ-fits-`u64` constraint), recorded here at freeze.

*Marked "to be frozen at D.B.1"; the `@plan-juncture` fork writes the resolved interface into this
subsection at execution time.*

### Frozen contracts read by D.B (not amended)

These are stable; D.B consumes them and amends none.

- **C-DLRelation** — DL relation format (`DLRelation` wrapper + `DLMatrix`) — *frozen D.A.1
  (f2dbf0a), `DLMatrix` assembled D.A.2 (651c17e)*. D.B.1's matrix-build consumes `DLMatrix`
  directly (raw `u32` exponents to reduce mod ℓ + already-reduced Schirokauer cols).
- **C-Schirokauer** — Schirokauer map interface — *frozen D.A.1 (f2dbf0a)*. D.B reads the
  Schirokauer columns through `DLMatrix`; the recovery step (D.B.2) accounts for them in the
  virtual-log. Not re-invoked (the columns are already computed in D.A.2).
- **C-LinAlg** — GF(2) block-solver substrate (`BlockVec`, `MatrixOperator`, `KernelVector`,
  `block_lanczos`, `block_wiedemann`) — *frozen G.E.1 (416f6db)*. **Read-only / pattern reference.**
  D.B builds a parallel F_ℓ module mirroring its structure; C-LinAlg is **not amended or
  generalised** (the parallel-module architecture decision — see Discoveries).
- **`Fp<L>` (`shared-field`)** — prime-field trait (`add`, `mul`, `inv`, `pow`, `from_u64`,
  `from_uint`; `FpNaive4` / `FpMonty4`) — *Phase α substrate*. D.B's F_ℓ arithmetic builds on it.
  Gap: no `from_bigint` — D.B.1 resolves the `BigInt`→`Fp` conversion (recorded in C-LinAlgFl).
- **C-FactorBase** — factor base (rational/algebraic sizes) — *frozen G.C.1 (c1dc0b6)*. Read for the
  column-count layout via `DLMatrix`.
- **C1** — `shared::numth` smoothness (`Uint<4>`) — *frozen α.2*. **Not in D.B's surface** (D.B
  touches factor-base indices and F_ℓ elements, not smoothness width); the ROADMAP width policy is
  untouched.

(Plus the remaining frozen Track-G/Track-D contracts — C-NF, C-Ideal, C-Res, C-Dedekind, C-Score,
C-Matrix, C-AlgSqrt — read where relevant but not foregrounded in D.B.)

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The D.B.1 `@plan` confirmation is not a ledger row (a paged fork
with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| D.B.1 | F_ℓ linalg substrate + block Lanczos; freeze C-LinAlgFl | pending | — | — |
| D.B.2 | Block Wiedemann over F_ℓ + virtual-log recovery + toy-F_p DL KAT | pending | — | — |

Contracts frozen before this sub-track (read by D.B): C-NF (bdba6f5 / extended 20cd263), C-Ideal
(05b27c8), C-Res (bcd63cd), C-Dedekind (7844773), C-Relation (c1dc0b6), C-FactorBase (c1dc0b6),
C-Score (00aa32d), C-Matrix (a0e854b), C-LinAlg (416f6db), C-AlgSqrt (c80a855 + ec69a1f), C1 (α.2),
C-Textbook (5c9b783), C-Schirokauer (f2dbf0a), C-DLRelation (f2dbf0a). This sub-track continues
Phase γ over the frozen GNFS factoring pipeline and the D.A DL relation substrate, and freezes one
new F_ℓ-linalg contract (C-LinAlgFl, at D.B.1).

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **F_ℓ linalg architecture: parallel module, NOT a generic refactor of GF(2) C-LinAlg
  (decided at D.B sharding, 2026-06-08).** D.B builds a *separate* `gnfs/src/dl/linalg/` module with
  F_ℓ-native types (`FlBlockVec` over `[Fp; W]`, F_ℓ operator + solvers); the frozen GF(2) C-LinAlg
  (G.E.1) stays **untouched**. *Rationale:* (1) zero risk to the verified GF(2) factoring pipeline
  (G.E–G.F); (2) the GF(2) bit-packing (64 scalars/`u64`, parity inner products) does not fit a
  uniform `Scalar` trait cleanly — a generic refactor would leak the abstraction and re-touch a
  frozen, KAT-verified substrate (a destructive reshard); (3) the two algorithms read more clearly as
  themselves side-by-side (pedagogy). *Tradeoff named:* some duplication of the block-solver skeleton
  (Lanczos/Wiedemann control flow re-expressed over F_ℓ), and the GF(2)↔F_ℓ commonality is documented
  in prose rather than compiler-enforced. **Deferred consolidation (user-directed):** *if* a genuine
  unification condition emerges during D.B implementation (the F_ℓ and GF(2) skeletons prove
  near-identical and the abstraction does *not* leak), do **not** unify inline — surface it as a
  discovery and defer the GF(2)↔F_ℓ unification to a deliberate later consolidation session (the
  C3-style "premature abstraction is the greater risk than late refactor" pattern). An inline
  unification that re-touches frozen C-LinAlg is a **destructive-HALT**.

- **`BigInt` → `Fp` conversion gap (resolve at D.B.1).** `shared-field`'s `Fp<L>` has `from_u64` /
  `from_uint` but **no `from_bigint`**. The Schirokauer columns are `Vec<BigInt>` and the reduced
  exponents are integers; D.B.1 must bridge `BigInt` → `Uint<L>` → `Fp`. Options: a conversion
  helper, or constraining toy ℓ to fit `u64` (`from_u64`). This is **internal-continue** (a D.B.1
  in-session decision, recorded in C-LinAlgFl), not a contract flex on `shared-field` — unless toy ℓ
  genuinely needs > 64 bits, in which case a `from_bigint`/`from_uint` path on `Fp` is an
  **additive** extension to surface at the freeze.

- **C-LinAlgFl is sub-track-internal — over-specify lightly, surface flexes (lever 4, not lever 3).**
  Frozen at D.B.1, consumed by D.B.2 and D.C. Unlike D.A's cross-track contracts, C-LinAlgFl does
  not bind E.C directly (the cross-track C2 solver interface freezes at D.C). The freeze register is
  Opus on lever 4 (correctness of the virtual-log table), not on cross-track reach. A D.C need the
  D.B.1 solution-shape can't carry is an **additive-reshard** at the D.B ◆ boundary, not a
  destructive-HALT (C-LinAlgFl is D.B's own, freely extensible by its definer's track).

- **Virtual-log recovery shape vs C2 (additive-reshard risk, D.B.2 ◆).** D.B.2 produces the
  virtual-log *table* (factor-base element → log mod ℓ); D.C wraps it into the C2 `solve_dl(g, h, …)`
  interface. If D.B.2's recovery reveals the table shape can't feed D.C's individual-log descent,
  that is an **additive-reshard** discovery at the ◆ boundary — surface it, do not pre-build C2 in
  D.B (defocus).

- **No descent / no C2 in D.B (defocus guard).** D.B stops at the virtual-log table for factor-base
  elements. Individual logarithm + special-q descent and the C2 NFS-DL solver interface are **D.C**.
  Implementing either here is **defocus** — internal-continue only within the D.B scope.

- **PARI oracle gating (resolved policy, D.A boundary — apply uniformly).** D.B.2's DL KAT
  cross-checks against PARI's discrete log. Per the resolved project-wide policy (ROADMAP D.A-boundary
  dev-oracle entry): oracles are **absent-by-default, opt-in, skip cleanly** — the PARI KAT is
  `#[ignore = "PARI not installed; run manually when available"]` (matching the D.A.2
  `kat_h_pari_oracle` stub pattern: no feature flag, no subprocess in CI), and a deterministic
  non-PARI KAT carries the reproducibility burden. No new policy decision is owed (it was resolved at
  D.A).

---

## Notes for executors

- Read `docs/ROADMAP.md` (the D.B spec under Phase γ — "Linear algebra over F_ℓ"; **Contract C2** —
  the NFS-DL solver interface D.B's virtual-log table ultimately feeds, frozen at D.C, *not* D.B;
  **Contract C1 → Width policy** to confirm D.B does not touch the smoothness width) and this PLAN
  before any session.
- Read the substrate D.B adapts: `gnfs/src/linalg/` (the GF(2) C-LinAlg pattern — `blockvec.rs`
  with its D.B generalisation notes at lines 26–30, `operator.rs:29–32`, `lanczos.rs`,
  `wiedemann.rs`; **read-only, the parallel-module template**), `gnfs/src/dl/relation.rs`
  (`DLMatrix`, `DLMatrix::from_relations`, `num_cols`), `gnfs/src/dl/mod.rs` (`DLRelation`),
  `shared/field/src/lib.rs` (the `Fp<L>` trait — note the missing `from_bigint`),
  `gnfs/src/filter/matrix.rs` (`SparseMatrix` — the GF(2) matrix for comparison). The G.E
  linear-algebra code-tour (`gnfs/docs/PEDAGOGY.md`, block Lanczos / Wiedemann sections) gives the
  mathematical background; the T.G textbook chapter covers the linear-algebra payoff.
- **Register:** D.B is **code** (Rust, `STYLE-CODE-RUST.md`), with KATs in `gnfs/tests/*_kat.rs`
  following the existing naming convention (`dl_linalg_kat.rs`, `dl_end_to_end_kat.rs`). No PEDAGOGY
  chapter in D.B — the NFS-DL writeup is D.W (later), paired with T.D.
- **Tier routing:** both D.B.1 and D.B.2 are **Sonnet** (`@build`) per the ROADMAP allotment (the
  Opus table lists D.A.1 and D.C.1, not any D.B session). D.B.1 carries one `@plan` marker: a T0/Opus
  post-landing C-LinAlgFl freeze confirmation (page `@plan-juncture`) before D.B.2 is dispatched —
  held at Opus on lever 4 (correctness of the virtual-log table) despite the strong inner loop. The
  juncture-tier (header) is **opus** on the same lever-4 call.
- **Invariants to preserve:** all Track-G code contracts and the D.A DL contracts (C-Schirokauer,
  C-DLRelation, C-Matrix, C-LinAlg) are **frozen** — D.B reads and mirrors them; it amends none.
  **GF(2) C-LinAlg stays untouched** (parallel F_ℓ module, not a generic refactor — Discoveries).
  **C1 `Uint<4>` stays as-is** (not in D.B's surface). The new contract is C-LinAlgFl (D.B.1).
- **PARI remains a dev-only oracle**, never on a build path; the project-wide gating policy is
  already resolved (D.A boundary) — apply the `#[ignore]` stub pattern uniformly.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — D.B introduces a new
  F_ℓ-linalg substrate on a parallel-module pattern (unproven for this codebase) and freezes a new
  contract (C-LinAlgFl), so halt at every juncture. With the D.B.1 `@plan` marker and the D.B.2 ◆
  boundary it halts **twice**: the **D.B.1 C-LinAlgFl freeze confirmation** (before D.B.2 consumes
  it) and the **D.B.2 ◆ boundary** (D.B sub-track close).
