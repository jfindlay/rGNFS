<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-D open (D.A — NFS-DL relation adaptation)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default; does not opt down.** Applying the
five-lever law: lever 3 (design-error cost) is **high** — D.A is the NFS-factoring → NFS-DL bridge,
and the contracts frozen here (C-Schirokauer, C-DLRelation) are "reused in D.B and D.C and consumed
by E.C" (ROADMAP), the project's most visible cross-track bridge. Lever 4 (correctness-criticality)
is **high** — a wrong virtual-logarithm column silently corrupts every DL solution. Lever 5
(inner-loop bandwidth) is *strong* (mature `cargo test --workspace` gate, 16 existing KAT files, a
PARI discrete-log cross-check available for D.A.2) — but the opt-down law licenses Sonnet only when
strong tests coincide with *low* correctness-criticality, which does **not** hold here. Levers 3+4
dominate; juncture-tier holds at Opus. The D.A.1 substrate freeze is exactly the cross-track contract
a juncture should adjudicate at Opus.

Last rewrite: T.G ◆ boundary crossed (Track-G complete end-to-end; the ROADMAP Progress subsection
reconciled at 2026-06-08). This plan opens **Phase γ / Track D (NFS-DL)** at its first sub-track,
**D.A — relation adaptation**: the bridge from NFS-factoring (the now-complete `gnfs` pipeline) to
NFS discrete-log in F_p.

---

## Purpose (design intent)

Per ROADMAP: NFS-DL is "a modification of NFS-factoring, easier to learn as the second pass." D.A is
the bridge sub-track — *what changes when the target is `log_g(h)` in F_p instead of a factorisation
of N*. Two sessions:

1. **The bridge substrate (D.A.1).** The two-number-field setup and the **Schirokauer map** — the
   new mathematical object NFS-DL needs and NFS-factoring does not. The Schirokauer map sends a
   number-field element to its ℓ-adic virtual-logarithm coordinates (via β^((p^f−1)/ℓ) − 1),
   supplying the extra columns that make the DL linear system solvable over F_ℓ. **Freezes
   C-Schirokauer** (the map interface) and **C-DLRelation** (the DL relation format: the existing
   `Relation` exponent vectors plus Schirokauer/virtual-log columns). This is the substrate session
   whose interface binds D.B (linalg over F_ℓ), D.C (individual log + descent), and ultimately E.C
   (the MOV bridge calls the NFS-DL solver). Over-specify deliberately.

2. **DL relation collection (D.A.2 ◆).** Adapt the sieve output into DL relations targeting
   `log_g(h)`: collect relations as today (reusing `line_sieve` / `special_q_sieve` and the frozen
   C1 smoothness path), then augment each with its Schirokauer columns from D.A.1, producing the DL
   relation matrix the F_ℓ linear algebra (D.B) will consume. KAT: recover a known toy F_p discrete
   log end-to-end (relation collection → augmented matrix), cross-checked against a hand-computed or
   PARI-computed reference. This session crosses the **Track-D-entry ◆ boundary** (D.A close).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing D.B's F_ℓ linear algebra
or D.C's descent here — both are later sub-tracks; D.A stops at producing the augmented relation
matrix) and **rigidity** (forcing the relation collection into the factoring `Relation` type if the
Schirokauer augmentation genuinely needs a DL-specific wrapper — a contract-sharp discovery, not a
silent squeeze).

**Scoping discipline (ROADMAP three-way split, applied here).** The Schirokauer map and two-NF setup
are **algorithmic content included in full** (principle 1 — the map *is* NFS-DL's defining
machinery). The `Uint<4>` width stays as-is per the C1 width policy (ROADMAP) — D.A.1
*confirms-and-records* that toy F_p norms fit 256 bits (a KAT) and defers the const-generic widening
behind the ROADMAP's prescriptive trigger; the width ceiling is a **principle-4 engineering-scale
annotation** (deferred to D.W), not a mathematical limit. No engineering optimisations (principle 3).
PARI remains a dev-only oracle (D.A.2 cross-check), never on a build path.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Confirmed by survey:
no Makefile / justfile / xtask wrapper exists in the workspace; raw `cargo` is the only CI surface.
Rust's compiler is the type gate; `cargo test` subsumes it on a clean build, so one green
`cargo test --workspace` satisfies both. D.A is **code** (unlike the prior docs-dominant bundle) —
the gate is a real inner loop, which is why lever 5 is strong even though juncture-tier still holds
at Opus on levers 3+4.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| D.A.1 `@plan` | Schirokauer map + two-number-field setup: freeze C-Schirokauer + C-DLRelation; confirm Uint<4> width KAT | A | **Opus** | C-NF, C-Ideal (`reduce_mod_ideal`), C-Relation, C1 | `gnfs/src/dl/mod.rs` (new), `gnfs/src/dl/schirokauer.rs` (new), `gnfs/src/lib.rs` (re-export `dl`), `gnfs/tests/schirokauer_kat.rs` (new) |
| D.A.2 ◆ | DL relation collection for log_g(h): augment sieve relations with Schirokauer columns → DL matrix; toy-F_p KAT | B | Sonnet | C-Schirokauer, C-DLRelation, C-FactorBase, C1, (PARI oracle) | `gnfs/src/dl/relation.rs` (new), `gnfs/src/dl/mod.rs` (extend), `gnfs/tests/dl_relation_kat.rs` (new) |

**Sequencing notes.** **D.A.1 must precede D.A.2** (D.A.2 consumes the frozen Schirokauer map and DL
relation format). The single `@plan` marker sits on **D.A.1** — a post-landing freeze confirmation
for C-Schirokauer + C-DLRelation, because the register binds D.B/D.C/E.C cross-track; confirm the
freeze before D.A.2 (its first consumer) is dispatched. **D.A.2 ◆** is the Track-D-entry boundary.

**Why 2 sessions (matches ROADMAP allotment).** The one-line-commit-title corollary: D.A.1
("Schirokauer map + two-NF setup") and D.A.2 ("DL relation collection") are two distinct commit
titles. They are **not** mergeable — D.A.1 freezes a substrate contract D.A.2 consumes (a
contract-sharp boundary, lever 2: the Schirokauer map is the irreducible unit). They are **not**
further splittable below the floor — the two-NF setup and the map share the same algebraic substrate
(`reduce_mod_ideal`, `NumberFieldElement::pow`) and fracturing them would split an irreducible unit
just to hit a LOC number (forbidden). D.A.1 is a front-loaded Opus substrate session (planning doc:
substrate sessions run 1.5–2× the band); if it overruns, the contract-sharp split is the
**two-NF setup** (the scaffolding) vs the **Schirokauer map** (the new object) — surfaced at the
D.A.1 `@plan` juncture, not pre-committed.

---

## Session detail

D.A.1 is crisp (its design surface is the C-Schirokauer + C-DLRelation freeze, resolved in-session
as the substrate's own work). D.A.2 is sketched at post-substrate fidelity — correct to leave its
precise shape open until D.A.1 freezes the map and relation format.

### D.A.1 — Schirokauer map + two-number-field setup (Opus, substrate, `@plan`)

**Deliverable:** the NFS-DL bridge substrate.
- **Two-number-field setup.** The DL analogue of the single algebraic/rational split: NFS-DL
  typically uses two number fields (or a rational + algebraic side adapted for the DL target).
  Establish how the existing `PolyPair` / `NumberField` machinery is configured for the `log_g(h)`
  target in F_p (the prime p of the finite field, the target g and h).
- **The Schirokauer map.** Implement the map λ: K* → (ℤ/ℓ)^r sending a number-field element to its
  ℓ-adic virtual-logarithm coordinates, via the ε = (p^f − 1)/ℓ exponentiation and the
  log-extraction. Consumes `NumberFieldElement::pow` and `reduce_mod_ideal` (both confirmed present
  in `shared/numfield`). This is the object that supplies the extra matrix columns making the DL
  system solvable.
- **Freeze C-Schirokauer** (the map interface — see Cross-session contracts) and **C-DLRelation**
  (the DL relation format: factoring `Relation`'s `u32` exponent vectors — already DL-ready, per the
  survey's confirmed design note — plus the Schirokauer columns).
- **Confirm-and-record the C1 width** (per ROADMAP C1 width policy): compute the toy-scale NFS-DL
  norm bound and assert it fits `Uint<4>` (a KAT). No widening — the const-generic widening stays
  behind the ROADMAP's prescriptive trigger. Record the verdict in Discoveries.

**Key design decisions (the C-Schirokauer / C-DLRelation freeze surface — the `@plan` confirmation):**
1. **Schirokauer map signature & ℓ-handling:** how ℓ (the target subgroup order) enters the map; the
   return shape (the r virtual-log coordinates); the error type for bad/ramified primes where the map
   is undefined. Over-specify: carry the multi-coordinate (r > 1) shape even if toy instances use
   r = 1, since D.C's descent will need it.
2. **C-DLRelation shape:** confirm the factoring `Relation` (u32 exponents) is reused directly (the
   survey confirms it was *designed* for this — re-narrowing would be a "destructive reshard" per the
   `sieve/mod.rs` doc) vs. a DL-specific wrapper carrying the Schirokauer columns alongside. Bias
   reuse + augmentation wrapper, not a re-typed relation.
3. **Two-NF configuration:** whether D.A reuses `PolyPair` as-is or needs a `DLPolyPair` carrying the
   second field — decided in-session against the actual `log_g(h)` setup.

**KAT (≥1 required):** (a) Schirokauer map KAT — the map evaluates correctly on a hand-computed toy
number-field element (known ℓ-adic coordinates); (b) the map is a homomorphism on a small sample
(λ(xy) = λ(x) + λ(y) mod ℓ) — the defining algebraic property; (c) **width KAT** — the toy-scale
NFS-DL norm bound fits `Uint<4>` (the confirm-record obligation). `cargo test --workspace` green.

**Subtlety:** C-Schirokauer is **cross-track** (D.B consumes the columns, D.C the descent, E.C the
solver). Over-specify the map interface deliberately — the r > 1 multi-coordinate shape, the
bad-prime error path — so a later sub-track that needs them doesn't force a reshard. The Schirokauer
map is mathematically delicate (the ℓ-adic log extraction); this is the irreducible Opus unit (lever
2). If the two-NF setup proves larger than expected, split it off (the flagged overrun boundary), but
do not fracture the map itself.

**Deferred:** F_ℓ linear algebra (D.B); individual logarithm + special-q descent (D.C); the const-
generic width widening (ROADMAP prescriptive trigger); the principle-4 width annotation (D.W).

**`@plan` confirmation (post-landing, T0/Opus, one-shot).** Page a `@plan-juncture` fork to confirm
the **C-Schirokauer + C-DLRelation freeze** before D.A.2 is dispatched: (1) the map interface is
complete (signature, ℓ-handling, r > 1 shape, bad-prime error path) and mutually consistent; (2) the
DL relation format reuses C-Relation cleanly (no destructive re-narrow); (3) the width verdict is
recorded; (4) the project-wide PARI oracle-test policy is resolved (carried from the ROADMAP
G.C-boundary open question — see Discoveries). One-shot findings; does not implement.

### D.A.2 ◆ — DL relation collection for log_g(h) (Sonnet, algorithm, sketch)

**Deliverable:** DL relation collection producing the augmented relation matrix for the F_ℓ linear
algebra. Sketch (crisp shape resolved once D.A.1's contracts freeze):
- **Collect relations** for the `log_g(h)` target by reusing the frozen sieve entry points
  (`line_sieve` / `special_q_sieve`) and the C1 smoothness path — the relation-collection machinery
  is shared with factoring (ROADMAP: "easier to learn as the second pass").
- **Augment with Schirokauer columns.** For each collected relation, evaluate the D.A.1 Schirokauer
  map and append the virtual-log coordinates, producing the DL relation matrix (the factoring
  exponent columns + the Schirokauer columns) that D.B's F_ℓ linear algebra consumes.
- **Assemble the DL matrix** in the shape D.B expects (defer the actual F_ℓ solve to D.B; D.A.2
  stops at producing the matrix + a verification that the relations are consistent).

Consumes C-Schirokauer, C-DLRelation, C-FactorBase, C1. Freezes nothing new (it is the algorithm
session consuming D.A.1's substrate).

**KAT (≥1 required):** recover a **known toy F_p discrete log** end-to-end through relation
collection + matrix assembly — cross-checked against a hand-computed reference or **PARI's
discrete-log** functionality (dev-only oracle, `#[ignore]`/feature-gated so it skips cleanly when
PARI is absent, per the oracle-test pattern). At minimum a deterministic KAT that does not require
PARI carries the reproducibility burden (matching the G.C CADO-oracle pattern). `cargo test
--workspace` green.

**Subtlety:** the load-bearing judgment is the **augmentation seam** — wiring the Schirokauer columns
into the relation/matrix format without re-narrowing the factoring `Relation`. If the DL matrix
genuinely needs a shape the C-Matrix format can't carry, that is a **contract discovery** (additive-
reshard) surfaced at the ◆ boundary, not a silent squeeze. This is the **Track-D-entry ◆ boundary** —
re-read the Purpose intent and verify the D.A bridge (D.A.1 → D.A.2) is coherent before crossing into
D.B.

---

## Cross-session contracts

D.A freezes two new code contracts (C-Schirokauer, C-DLRelation) at D.A.1 and reads the frozen
Track-G / G.A-substrate contracts.

### C-Schirokauer — Schirokauer map interface (compiler + KAT) — *frozen D.A.1 (f2dbf0a)*

**Defined:** D.A.1. **Consumed by:** D.A.2 (relation augmentation), D.B (the virtual-log columns in
the F_ℓ system), D.C (individual-log descent), and ultimately E.C (via the NFS-DL solver). **Cross-
track** — over-specified deliberately at D.A.1. Compiler-enforced (the map signature) + KAT-enforced
(homomorphism property + known-value evaluation).

**Frozen interface (`gnfs/src/dl/schirokauer.rs`):**

```rust
pub fn schirokauer<'a>(
    elt: &NumberFieldElement<'a>,
    ell: &BigInt,
    ideals: &[PrimeIdeal<'a>],
) -> Result<Vec<BigInt>, SchirokauerError>
```

- **Signature:** Takes a number-field element `elt`, the target subgroup order `ell` (BigInt), and a
  slice of prime ideals. Returns the r ℓ-adic virtual-log coordinates `[λ_1(β), ..., λ_r(β)]` ∈
  (ℤ/ℓ)^r, one coordinate per ideal.
- **r > 1 multi-coordinate shape:** The return type `Vec<BigInt>` carries r coordinates where r =
  `ideals.len()`. This shape is carried even when toy instances use r = 1, since D.C's descent and
  E.C's solver will need it.
- **ℓ-handling:** The `ell` parameter is passed explicitly as a `BigInt`. The map computes ε =
  (p−1)/ℓ per ideal and performs the ℓ-adic log extraction (β^ε − 1)/ℓ evaluated at α ≡ r (mod ℓ).
- **Error type (`SchirokauerError`):** Four variants covering the undefined-map cases:
  - `RamifiedPrime { p, ell }` — p ≢ 1 (mod ℓ); the map is undefined for this ideal.
  - `ElementDivisibleByEll { ell }` — ℓ divides a coefficient of the element.
  - `ExponentOverflow { p, ell }` — ε = (p−1)/ℓ overflows (internal BigInt pow handles this).
  - `NotDivisibleByEll { coeff_index }` — β^ε − 1 not divisible by ℓ (indicates non-integer element).
- **Re-export:** `PrimeIdeal<'a>` is re-exported as a type alias for `Ideal<'a>` from `shared-numfield`
  (the C-Ideal contract). The public surface is `gnfs::compute_schirokauer`, `gnfs::SchirokauerError`,
  `gnfs::PrimeIdeal`.

**KAT coverage (`gnfs/tests/schirokauer_kat.rs`):** (a) known-value KAT (hand-computed λ(1+α), λ(α),
λ(2)); (b) homomorphism KAT (λ(xy) = λ(x) + λ(y) mod ℓ); (c) width KAT (toy F_p norms fit Uint<4>);
(d) error-path KAT (RamifiedPrime for p ≢ 1 mod ℓ); (e) multi-coordinate shape KAT (two ideals → two
coordinates).

### C-DLRelation — DL relation format (compiler + KAT) — *frozen D.A.1 (f2dbf0a)*

**Defined:** D.A.1. **Consumed by:** D.A.2, D.B. Compiler + KAT.

**Frozen interface (`gnfs/src/dl/mod.rs`):**

```rust
pub struct DLRelation {
    pub relation: Relation,           // The factoring Relation (u32 exponent vectors)
    pub schirokauer_cols: Vec<BigInt>, // Virtual-log coordinates from compute_schirokauer
}

impl DLRelation {
    pub fn new(relation: Relation, schirokauer_cols: Vec<BigInt>) -> Self;
    pub fn schirokauer_rank(&self) -> usize;  // = schirokauer_cols.len()
}
```

- **Augmentation wrapper, not re-typed relation:** The factoring `Relation` (C-Relation contract,
  `gnfs/src/sieve/mod.rs`) is reused directly — `u32` exponent vectors, DL-ready by design. The
  `DLRelation` wrapper adds the Schirokauer columns alongside, not inside, the relation. This
  preserves C-Relation and avoids a destructive reshard.
- **Shape:** `relation` carries the (a, b) pair and both exponent vectors (rational + algebraic);
  `schirokauer_cols` carries the r virtual-log coordinates from `compute_schirokauer`. D.B reads the
  integer exponents mod ℓ (not mod 2) and appends the Schirokauer columns as extra matrix columns.
- **Usage (D.A.2 → D.B):** D.A.2 constructs `DLRelation` by (1) collecting a smooth relation via the
  sieve (reusing `line_sieve` / `special_q_sieve`), (2) evaluating `compute_schirokauer` on the
  algebraic element a + b·α, (3) wrapping: `DLRelation::new(relation, schirokauer_cols)`. D.B
  assembles the DL matrix from a collection of `DLRelation` values.
- **Re-export:** `gnfs::DLRelation` is the public surface.

### Frozen contracts read by D.A (not amended)

These are stable; D.A consumes them and amends none.

- **C-Relation** — relation / exponent-vector format (`u32` exponents, DL-ready) — *frozen G.C.1
  (c1dc0b6)*. D.A reuses directly.
- **C-FactorBase** — two-sided factor base + sign/QC columns — *frozen G.C.1 (c1dc0b6)*. D.A.2 reuses
  for relation collection.
- **C-NF** — number-field substrate (ℤ[α] arithmetic, `NumberFieldElement::pow`) — *frozen G.A.1
  (bdba6f5) / extended G.F.1 (20cd263 — `reduce_mod_ideal`)*. D.A.1's Schirokauer map consumes
  `reduce_mod_ideal` + `pow`.
- **C-Ideal** — prime-ideal representation (p, α − r) — *frozen G.A (05b27c8)*. D.A consumes for the
  per-prime Schirokauer evaluation.
- **C1** — `shared::numth` smoothness (`trial_smooth`, `SmoothWitness`, `Uint<4>`) — *frozen α.2*.
  D.A.2 reuses for DL relation collection. **Width: confirm-record at D.A.1 per ROADMAP C1 width
  policy; no widening.**

(Plus the remaining G.A substrate + Track-G contracts — C-Res, C-Dedekind, C-Score, C-Matrix,
C-LinAlg, C-AlgSqrt — read where relevant but not foregrounded in D.A.)

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The D.A.1 `@plan` confirmation is not a ledger row (a paged fork
with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| D.A.1 | Schirokauer map + two-NF setup; confirm Uint<4> width | done | f2dbf0a | C-Schirokauer, C-DLRelation |
| D.A.2 | DL relation collection for log_g(h) + toy-F_p KAT | pending | — | — |

Contracts frozen before this sub-track (read by D.A): C-NF (bdba6f5 / extended 20cd263), C-Ideal
(05b27c8), C-Res (bcd63cd), C-Dedekind (7844773), C-Relation (c1dc0b6), C-FactorBase (c1dc0b6),
C-Score (00aa32d), C-Matrix (a0e854b), C-LinAlg (416f6db), C-AlgSqrt (c80a855 + ec69a1f), C1 (α.2),
C-Textbook (5c9b783). This sub-track opens Phase γ over the complete, frozen GNFS factoring pipeline
and freezes two new DL contracts (C-Schirokauer, C-DLRelation, both at D.A.1).

---

## Action-frame digest

**D.A.1 `@plan` juncture confirmation (post-landing freeze, T0/Opus one-shot):** `design-confident`.
All four confirmation points passed:

1. **Map interface completeness (C-Schirokauer):** ✓ Complete. Signature
   `schirokauer(elt, ell, ideals) -> Result<Vec<BigInt>, SchirokauerError>` is mutually consistent.
   The r > 1 multi-coordinate shape is carried (KAT-verified with two ideals). Error type covers all
   undefined-map cases (RamifiedPrime, ElementDivisibleByEll, ExponentOverflow, NotDivisibleByEll).
   Sufficient for D.B (virtual-log columns), D.C (descent), E.C (solver).

2. **C-DLRelation shape:** ✓ Reuses C-Relation cleanly. `DLRelation` is an augmentation wrapper
   (`relation: Relation` + `schirokauer_cols: Vec<BigInt>`), not a re-typed relation. No destructive
   re-narrow. Sufficient for D.A.2 (construction) and D.B (matrix assembly).

3. **Width verdict:** ✓ KAT `kat_c_width_uint4` confirms toy F_p norms (p ≈ 2^35, f = x²+1, B ≈
   p^(1/3)) fit Uint<4> with > 230 bits headroom. Confirm-record obligation satisfied. No widening.

4. **PARI oracle-test policy:** ✓ D.A.1 respects the resolved policy (ROADMAP D.A boundary). No PARI
   dependency on the build path. All KATs are deterministic with hand-computed references. No oracle
   test that fails when PARI is absent.

Frozen contracts written to `## Cross-session contracts`: C-Schirokauer (f2dbf0a), C-DLRelation
(f2dbf0a). D.A.2 may proceed.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **C1 `Uint<4>` width — confirm-record at D.A.1, do NOT widen (ROADMAP C1 width policy).** The
  width decision is *resolved*: D.A.1 asserts toy F_p norms fit 256 bits (a KAT) and consumes C1
  as-is. The const-generic widening is pre-chosen but trigger-gated (ROADMAP: "if/when the ceiling
  binds, as its own deliberate ROADMAP-then-shard session — never spontaneous in-flight scope
  growth"). If D.A.1's width KAT *fails* (toy norms exceed 256 bits — not expected), that is a
  **destructive-HALT**: stop and page the ROADMAP widening session, do not widen inline.

- **C-Schirokauer is cross-track prose+compiler-enforced — over-specify, surface flexes (lever 3).**
  Frozen at D.A.1, consumed by D.B/D.C/E.C. A later sub-track needing a map shape D.A.1 didn't
  provide (e.g. D.C's descent needing multi-coordinate r > 1) must flex C-Schirokauer at an
  inflection review, not silently extend it. D.A.1 over-specifies (carry r > 1) to pre-empt this.

- **C-DLRelation reuse vs. re-narrow (additive-reshard risk).** The factoring `Relation` is DL-ready
  by design (u32 exponents). D.A.1 augments, not re-narrows. If D.A.2 finds the augmentation needs a
  matrix shape C-Matrix can't carry, that is an **additive-reshard** discovery at the ◆ boundary —
  not a silent squeeze, and not a destructive-HALT (C-Matrix is read, not amended, by D.A).

- **PARI oracle gating (carried from ROADMAP G.C-boundary open question).** D.A.2's DL KAT may
  cross-check against PARI's discrete log. The ROADMAP flags the project-wide oracle-test policy as
  *still open, recommended for resolution at this Track-D plan-init*. For D.A.2, follow the G.C
  pattern: PARI cross-check `#[ignore]`/feature-gated (skips cleanly when absent); a deterministic
  non-PARI KAT carries the reproducibility burden. **Surface the project-wide policy decision at the
  D.A.1 `@plan` juncture** — D.B leans harder on PARI, so resolving it now (not per-test) is owed.

- **No descent / no F_ℓ solve in D.A (defocus guard).** D.A stops at producing the augmented DL
  relation matrix. The F_ℓ linear algebra is D.B; individual log + special-q descent is D.C.
  Implementing either here is **defocus** — internal-continue only within the D.A scope.

---

## Notes for executors

- Read `docs/ROADMAP.md` (the D.A spec under Phase γ; **Contract C1 → Width policy** for the
  confirm-record obligation and the prescriptive widening trigger; Contract C2 — the NFS-DL solver
  interface D.A's contracts ultimately feed; Contract C3 — the polysel-scoring reuse note) and this
  PLAN before any session.
- Read the substrate D.A adapts: `gnfs/src/sieve/mod.rs` (the `Relation` / `ExponentVector` C-Relation
  contract, u32-exponent DL-readiness note), `gnfs/src/sieve/factor_base.rs` (C-FactorBase),
  `shared/numfield/src/element.rs` (`NumberFieldElement::pow`, `reduce_mod_ideal`),
  `shared/numfield/src/ideal.rs` (C-Ideal), `shared/numth/src/smooth.rs` (C1, `Uint<4>`). The G.A
  number-field code-tour (`shared/numfield/docs/PEDAGOGY.md`) and the T.G textbook chapter give the
  mathematical background.
- **Register:** D.A is **code** (Rust, `STYLE-CODE-RUST.md`), with KATs in `gnfs/tests/*_kat.rs`
  following the existing naming convention (`schirokauer_kat.rs`, `dl_relation_kat.rs`). No PEDAGOGY
  chapter in D.A — the NFS-DL writeup is D.W (later), paired with T.D.
- **Tier routing:** D.A.1 is **Opus** (Schirokauer-map substrate + cross-track contract freeze —
  `@plan-deep` for design or `@build`-Opus if the chain dispatches build-tier). D.A.2 is **Sonnet**
  (`@build`). D.A.1 carries one `@plan` marker: a T0/Opus post-landing C-Schirokauer + C-DLRelation
  freeze confirmation (page `@plan-juncture`) before D.A.2 is dispatched.
- **Invariants to preserve:** all Track-G code contracts (C-Relation, C-FactorBase, C-Matrix,
  C-LinAlg, C-AlgSqrt) and the G.A substrate contracts (C-NF, C-Ideal, C-Res, C-Dedekind) are
  **frozen** — D.A reads and adapts them; it amends no Track-G contract. **C1 `Uint<4>` stays as-is**
  (confirm-record, not widen). The new contracts are C-Schirokauer + C-DLRelation (D.A.1).
- **PARI remains a dev-only oracle**, never on a build path. Resolve the project-wide oracle-test
  gating policy at the D.A.1 `@plan` juncture (carried from the ROADMAP G.C-boundary open question).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — D.A opens a new
  sub-track (Track D) and freezes two new cross-track contracts on an unproven (for DL) shard pattern,
  so halt at every juncture. With the D.A.1 `@plan` marker and the D.A.2 ◆ boundary it halts
  **twice**: the **D.A.1 C-Schirokauer freeze confirmation** (before D.A.2 consumes it) and the
  **D.A.2 ◆ boundary** (Track-D-entry close).
