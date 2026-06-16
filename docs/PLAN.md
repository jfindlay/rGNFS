<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.K — Gaudry–Diem–Joux–Vitse index calculus: the elliptic-curve DLP *solver* that consumes the Semaev primitive)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **set by the ROADMAP's Opus flag on E.K.1 (index-calculus
strategy) and lever-3 (cost of design error).** Unlike E.J — where the tuning law pointed to `sonnet`
and the user overrode to `opus` on lever 3 — E.K carries a *native* Opus session (E.K.1, the
strategy/factor-base/relation-contract design the ROADMAP marks `✓ E.K.1`), so the boundary juncture
sits at `opus` by the same judgment register the session itself demands. The cost-of-wrong is high
on two fronts: (a) **the relation/matrix contract (C-EKRelation)** is consumed by three downstream
sessions (decomposition, collection, linear algebra) and is the index-calculus analogue of NFS's
C-Relation — getting its shape wrong is a retrofit across the whole pipeline; (b) **the factor-base
+ prime-order-subgroup strategy** determines whether the linear algebra is over a field `Z/ℓℤ` at
all. Lever 5 is **strong** (the green-path correctness signal is **agreement with the frozen
`rho::ecdlp` Pollard-rho solver** on the same toy ECDLP instance — a fast, decisive, oracle-free KAT,
like E.I's group axioms and E.J's vanishing relation, *unlike* E.H's oracle-leaning
log-preservation), but lever 5 only licenses opting the juncture *down*; here the native Opus flag
(E.K.1) and lever 3 hold it *up*. *(The ◆ forks page `@plan-juncture` at opus; E.K.1's per-row tier
is **Opus** per the ROADMAP — E.K is the one Track-E sub-track since E.H with a native Opus session;
E.K.2–E.K.5 are Sonnet.)*

**Field-target — adjudicated E.K = index calculus over `E(F_p)` (the prime-field mechanism),
user-adjudicated at shard time.** The Semaev polynomials have three classical index-calculus homes:
(1) the prime field `E(F_p)`; (2) the extension field `E(F_{p^n})` — the genuine Gaudry–Diem setting
where index calculus *beats* Pollard-rho asymptotically; (3) the binary-curve / GHS-descended
Jacobian coupled to the frozen E.H chain. **The ROADMAP itself carried a tension here** (C1, line
498, names "*smoothness of points over `F_{p^n}` via Semaev (E.K)*" — leaning toward (2) — while the
E.J PLAN froze C-Semaev over `E/F_p` *only* and labelled the descended-curve coupling "E.K's job" —
(3)). Within the ROADMAP's 4–5-session, single-Opus-session budget these cannot all be E.K; and the
**frozen substrate (C-Semaev over `E/F_p`) makes (1) the only target E.K can build without first
flexing a frozen contract.** **The user adjudicated (1):** E.K is the **complete, self-contained
index-calculus *mechanism* over `E(F_p)`** — factor base → point decomposition (via the frozen F_p
Semaev `semaev_poly`) → relation collection → `Z/ℓℤ` linear algebra → discrete-log recovery —
cross-checked against the frozen `rho::ecdlp` Pollard-rho solver. *(Tradeoff named, load-bearing:
**E.K-over-F_p does NOT exhibit the asymptotic index-calculus win.** Over `E(F_p)` with points as the
factor base, index calculus is **not** faster than rho — the speed-up needs the extension-field
structure of (2). E.K is **mechanism-correct at toy scale, with the asymptotic win NOT observable** —
the same principle-4 posture G.E and the NFS end-to-end KAT already took (BENCHMARKS.md: "the
pipeline is correct but the asymptotic win is not observable"). The asymptotic-win `F_{p^n}` case (2)
and the GHS-coupled end-to-end (3) are **deferred to later, separately-sharded sub-tracks, each
opening by flexing C-Semaev (the `F_{p^n}` lift) — re-shards, not exclusions.** This is recorded as a
ROADMAP capture candidate. The decoupling is bought for the same reason E.J bought it: keeping two
independently-delicate designs — extension-field Semaev and the index-calculus solver — from
entangling in one shard, the coupling NOTES.md and the inflection-juncture discipline warn against.)*

**Scope boundary — E.K is the "solve" in the transfer/structure/solve triad (NOTES.md, 2026-06-15).**
E.K is the long-anticipated index-calculus *solver* the project's transfer-attack framing names:
E.H *transfers* (GHS descent, frozen), E.J builds the *structure* (Semaev polynomials, frozen), **E.K
*solves*** (index calculus over the factor base). E.K consumes the **frozen** C-Semaev surface (the
F_p `semaev_poly(m)`, the `MultiPoly` partial-evaluation / one-variable-elimination operations
E.J.1 *over-specified* for exactly this consumer), the frozen `rho::curve::Curve` (factor-base point
enumeration + the group law), the frozen C1 `shared::numth::trial_smooth` (the smoothness witness),
and the **frozen `gnfs::dl::linalg` block-Lanczos/Wiedemann engine** (the `Fp<L>`-generic
`FlSparseMatrix` solver — reused, with an E.K-specific *adapter*; see the load-bearing linalg finding
below). It builds **no new field substrate, no Semaev extension, no GHS coupling** — those are the
deferred re-shards.

The substrate survey (forked `@explore`, 2026-06-16) established the shape and surfaced three
load-bearing findings:

1. **The Semaev surface E.J.1 over-specified is exactly what E.K's point-decomposition step
   consumes (confirmed).** `rho::semaev` exposes `semaev_poly(m, a, b, p) → MultiPoly` and the
   `MultiPoly` operations `partial_eval(assignment)`, `elim_var_resultant(other, var)`, `eval`,
   `is_symmetric` — the partial-assignment evaluation and one-variable resultant-elimination E.J.1
   carried "for E.K's point decomposition" land exactly where intended. **E.K amends no Semaev
   contract** — it reads C-Semaev as the decomposition oracle. *(Confirmed the over-specification
   discipline paid off: the substrate-rule bet at E.J.1 — "carry the operations E.K needs" — is
   collected here with no retrofit.)*

2. **The `Z/ℓℤ` linear algebra is *partial* reuse, not greenfield and not full reuse (the
   load-bearing finding — corrects the ROADMAP's "predecessor: G.E linear algebra" to a partial
   truth).** The survey found two G.E linalg surfaces: `gnfs::linalg` (block-Lanczos/Wiedemann over
   **GF(2)**, NFS-square-root-specific — *not* reusable) and `gnfs::dl::linalg` (block-Lanczos/
   Wiedemann **generic over `Fp<L>`** — `block_lanczos_fl`, `block_wiedemann_fl`, `FlSparseMatrix`,
   `FlMatrixOperator`). The **solver engine is reusable** (it is `Fp<L>`-generic and the prime-order
   subgroup makes `Z/ℓℤ = F_ℓ`, so the engine fits exactly); but the **matrix *construction*** is
   bound to NFS (`build_fl_matrix(dl_matrix: &DLMatrix, …)`). **E.K.4 reuses the engine and writes
   only an E.K-relation → `FlSparseMatrix` adapter** (analogous to `build_fl_matrix` but from
   C-EKRelation). *(Tradeoff named: this creates a read-coupling from `rho`'s E.K into `gnfs::dl`
   internals — the F_ℓ solver was designed for NFS-DL. Bought because duplicating a tested,
   `Fp<L>`-generic block-Lanczos/Wiedemann implementation is the worse option, and the prime-order
   subgroup makes the engine a genuine fit. If the NFS coupling proves ill-fit for the
   prime-order-subgroup case at E.K.4, the adapter session grows — surfaced as a discovery.)*

3. **Factor-base point enumeration is greenfield; the prime-order-subgroup is the linear-algebra
   precondition.** No `points_on_curve()` iterator exists; E.K.1 builds factor-base enumeration
   (`{x : x³+ax+b is a QR mod p}`, lift `y`) from scratch. The toy fixture's group order `n = 60 =
   2²·3·5` is composite — **the index-calculus linear algebra must run in a prime-order subgroup
   `Z/ℓℤ`** (a large prime `ℓ | n`) for the matrix to be over a field. E.K.1's strategy decision
   fixes `ℓ` and the factor base relative to it. *(This is the index-calculus analogue of NFS's
   "work mod ℓ" — the C-EKRelation exponents live in `Z/ℓℤ`, mirroring NFS-DL's C-DLRelation.)*

The work splits at **five pipeline/contract-sharp seams**, **5 sessions** (the ROADMAP ceiling for
E.K, consistent with the project's documented ceiling-bias — G/D both landed at or above their upper
bands), at the boundaries between strategy/substrate, point decomposition, relation collection,
linear algebra, and the DLP recovery + close:

1. **E.K.1 ◆-start `@architect` — Index-calculus strategy + factor base + relation/matrix contract
   (Opus, Cat A).** The strategy session the ROADMAP flags Opus: fix the prime-order subgroup `ℓ`,
   the factor-base shape (the point enumeration + the decomposition arity `m`), and **freeze
   C-EKRelation** (the relation/matrix contract the whole pipeline consumes — the index-calculus
   analogue of NFS's C-Relation) + **C-IndexCalcStrategy** (the factor base + subgroup + `m`). The
   design crux: the relation representation (exponent vector over `Z/ℓℤ` indexed by factor-base
   points) consumed by decomposition, collection, and linear algebra. *(The substrate-design
   session; Opus per the ROADMAP. Carries the `@architect` juncture as a ◆-start fork — the inflection
   point the ROADMAP pre-schedules.)*

2. **E.K.2 — Point decomposition via Semaev (Sonnet, Cat B).** Given a point `Q`, decompose it into a
   sum of `m` factor-base points by finding roots of `semaev_poly(m+1)` (specialised at `Q`'s
   x-coordinate) with all coordinates in the factor base — native root-finding / resultant-
   elimination over the factor base on the green path (the frozen `MultiPoly::partial_eval` +
   `elim_var_resultant`), with an optional `#[ignore]` msolve cross-check. **Consumes C-Semaev,
   C-IndexCalcStrategy. Freezes C-PointDecomp.**

3. **E.K.3 — Relation collection (Sonnet, Cat B).** The loop that gathers ≥ (factor-base size + 1)
   relations by decomposing random multiples `a·G + b·Q` over the factor base, recording each as a
   C-EKRelation row. **Consumes C-PointDecomp, C-EKRelation. Freezes C-RelationCollect.**

4. **E.K.4 — `Z/ℓℤ` linear algebra: relation→`FlSparseMatrix` adapter + solve (Sonnet, Cat B).** The
   E.K-relation → `FlSparseMatrix` adapter + the block-Lanczos/Wiedemann solve over `Z/ℓℤ` (reusing
   the frozen `gnfs::dl::linalg` engine). **Consumes C-RelationCollect, C-EKRelation, the frozen
   `gnfs::dl::linalg` engine. Freezes C-EKLinAlg.**

5. **E.K.5 ◆ `@architect` — Discrete-log recovery + `rho::ecdlp` cross-check + sub-track close
   (Sonnet, Cat I).** Assemble the pipeline: recover `log_G(Q)` from the relation-matrix kernel,
   **cross-check against the frozen `rho::ecdlp` Pollard-rho solver** on the same toy instance (the
   green-path correctness signal), the principle-4 annotation (mechanism-correct, asymptotic win not
   observable), the sub-track close. **Consumes C-EKLinAlg + all prior. Freezes C-IndexCalc** — the
   surface E.W's cross-attack benchmark consumes. Crosses the **E.K ◆ boundary** — the index-calculus
   solver ships, rho-cross-checked, the Track-E "solve" complete.

Re-read this intent at the ◆ boundaries to catch **defocus** (lifting Semaev to `F_{p^n}` or coupling
to the GHS-descended Jacobian — those are the **deferred re-shards**, not E.K; or re-implementing a
block-Lanczos solver `gnfs::dl::linalg` already ships; or writing the index-calculus *textbook
chapter* in MATHEMATICS.md — that is **T.E**, paired with E.W at the Track-E ◆; E.K.5 writes at most
a PEDAGOGY code-tour delta) and **rigidity** (forcing the `F_{p^n}` asymptotic-win setting when the
adjudicated target is `E(F_p)` mechanism-only; or re-deriving the Semaev `MultiPoly` operations E.J.1
already froze when E.K needs partial-evaluation + one-variable elimination; or amending the frozen
`Curve`/`Fp` surfaces when E.K only reads them).

**Scoping discipline.** E.K builds the index-calculus solver at **demonstration fidelity**
(principle 1 — algorithmic content complete: factor-base enumeration, Semaev point decomposition,
relation collection, the `Z/ℓℤ` linear algebra, and discrete-log recovery all implemented head-on)
and **toy field sizes** (the `semaev_toy()` fixture `F_47`, `n = 60`, a prime-order subgroup `ℓ | n`;
small decomposition arity `m`). It **amends no frozen contract** (`rho::semaev` C-Semaev, `rho::curve`
`Curve`/`AffinePoint`, C1 `trial_smooth`, the `gnfs::dl::linalg` engine are all consumed-or-untouched;
E.K adds the `rho::index_calculus` module + the relation→matrix adapter). It builds **no Semaev
extension** (the `F_{p^n}` lift is a deferred re-shard) and **no GHS coupling** (a deferred re-shard).
The correctness signal is **agreement with the frozen `rho::ecdlp` Pollard-rho solver** (E.K's
recovered `log_G(Q)` matches `rho::ecdlp::solve_*` on the same instance) — **exactly self-checking,
oracle-free** (no msolve/PARI on the green path; the lever-5 strength). The **engineering-vs-
mathematics disconnect** (ROADMAP principle 4) is explicit and load-bearing: E.K is the index-calculus
*mechanism* over `E(F_p)`, **mechanism-correct but with the asymptotic win NOT observable** (the win
needs the deferred `F_{p^n}` setting); the toy `F_p`/`m`/`ℓ` are a principle-4 boundary, annotated,
never presented as crypto-scale *or* as the asymptotic-win demonstration.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.K): "*E.K — Gaudry–Diem–Joux–Vitse index calculus. 4-5 sessions.
Predecessors: E.J, G.B (scoring methodology), G.E (linear algebra). First session is Opus-tier. The
Gröbner-basis step delegates to `msolve` as a dev-only oracle (parallel to CADO-NFS's role).*" E.K
builds the **index-calculus discrete-logarithm solver** over `E(F_p)`: the algorithm that solves
ECDLP not by generic √n search (Pollard-rho, E.A's baseline) but by **finding exploitable structure**
— a smoothness/decomposition phenomenon. The pipeline: (1) a **factor base** of points on the curve;
(2) **relation collection** — decompose random multiples `a·G + b·Q` into sums of factor-base points,
each decomposition a linear relation among discrete logs; (3) **linear algebra** over `Z/ℓℤ` to
solve the relation system and recover `log_G(Q)`. The **point-decomposition step is where the Semaev
polynomials enter** (E.J's primitive): "does this point decompose over the factor base?" becomes
"does `semaev_poly(m+1)` have a root with all coordinates in the factor base?" — the step E.J built
the Semaev machinery to feed.

E.K is the **"solve" in the project's transfer/structure/solve triad** (NOTES.md, 2026-06-15): E.H
*transfers* (GHS/Weil descent, frozen), E.J builds the *structure* (Semaev summation polynomials,
frozen), **E.K *solves*** (index calculus). It is the consumer the over-specified C-Semaev surface
was built for — E.J.1 carried `partial_eval` + `elim_var_resultant` on `MultiPoly` "for E.K's point
decomposition", and E.K collects that bet here.

The adjudicated target is **`E(F_p)`** (the prime-field index-calculus *mechanism*), built on the
frozen `rho::curve::Curve` + `rho::semaev` + C1 `trial_smooth` surfaces. The central design tension,
recorded load-bearing: **E.K demonstrates the index-calculus *mechanism* but NOT its asymptotic
win.** Over `E(F_p)` index calculus is not faster than Pollard-rho — the asymptotic speed-up needs
the **extension-field** structure of `E(F_{p^n})` (the genuine Gaudry–Diem setting). E.K is
mechanism-correct at toy scale with the win not observable (the principle-4 posture G.E and the NFS
end-to-end KAT already took). The `F_{p^n}` asymptotic-win case and the GHS-coupled end-to-end attack
are **deferred to later, separately-sharded sub-tracks**, each opening by flexing C-Semaev — re-shards
the post-ROADMAP cohesion/coverage rereads will weigh (per the project's preference for a natural,
accessible, coherent treatment over a strained total covering), not exclusions.

The sub-track decomposes into five conceptual units, each a session:

1. **Strategy + factor base + relation contract (E.K.1 ◆-start, Opus).** Fix the prime-order subgroup
   `ℓ`, the factor base, the decomposition arity `m`; freeze the relation/matrix contract the whole
   pipeline consumes. The substrate the solver stands on; the ROADMAP's Opus session.
   **Freezes C-IndexCalcStrategy, C-EKRelation. (E.K.1.)**

2. **Point decomposition via Semaev (E.K.2).** Decompose a point into factor-base points by finding
   Semaev-polynomial roots with all coordinates in the factor base — the step the Semaev primitive
   feeds. **Freezes C-PointDecomp. (E.K.2.)**

3. **Relation collection (E.K.3).** The loop gathering enough relations to over-determine the system.
   **Freezes C-RelationCollect. (E.K.3.)**

4. **`Z/ℓℤ` linear algebra (E.K.4).** The relation→`FlSparseMatrix` adapter + block-Lanczos/Wiedemann
   solve over the prime-order subgroup (reusing the frozen G.E `gnfs::dl::linalg` engine).
   **Freezes C-EKLinAlg. (E.K.4.)**

5. **DLP recovery + cross-check + close (E.K.5 ◆).** Recover `log_G(Q)`, cross-check against the frozen
   `rho::ecdlp` solver, the principle-4 annotation, the close. **Freezes C-IndexCalc. (E.K.5 ◆.)**

E.K is **Semaev-extension-free** (the `F_{p^n}` lift is a deferred re-shard), **GHS-free** (the
descended-Jacobian coupling is a deferred re-shard), and **chapter-free** (the index-calculus textbook
content is T.E, paired with E.W at the Track-E ◆). Re-read this intent at the ◆ boundaries to catch
defocus (the `F_{p^n}` Semaev lift, the GHS coupling, a duplicate linalg solver, the MATHEMATICS
chapter) and rigidity (forcing the `F_{p^n}` asymptotic-win setting; re-deriving the frozen Semaev
`MultiPoly` operations; amending the frozen `Curve`/`Fp` surfaces).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-16); raw
`cargo` is the only CI surface (unchanged from E.D…E.J). Oracle KATs are `#[ignore]`-gated only —
the exact form is `#[ignore = "PARI not installed; run manually when available"]` (and the
msolve analogue `#[ignore = "msolve not installed; run manually when available"]`), used identically
in `rho/tests/ssa_kat.rs`, `rho/tests/semaev_kat.rs` (KAT 21, `s4_pari_cross_check`),
`rho/tests/mov_kat.rs`, `rho/tests/ghs_kat.rs`, and `shared/padic/tests/log_kat.rs`. `/run-plan`
re-discovers at preflight. E.K **adds no new workspace edge and no new crate** (`rho` already depends
on `gnfs` — where the `gnfs::dl::linalg` engine lives — and `shared-numth` — where C1 lives; the
index-calculus solver is a new `rho::index_calculus` module), so the gate is a **correctness +
no-regression gate**:

- Each session's KATs are the primary correctness signal — fast, decisive, and **oracle-free**
  (lever 5): for E.K.1, the factor-base enumeration identities (every enumerated point is on the
  curve via the frozen `is_on_curve`; the subgroup `ℓ` divides `n`; the C-EKRelation round-trip) for
  C-EKRelation/C-IndexCalcStrategy; for E.K.2, **decomposition correctness** (a decomposition the
  Semaev step returns actually sums to `Q` via the frozen `Curve` group law — checked directly, not
  via an oracle) for C-PointDecomp; for E.K.3, **relation validity** (every collected relation's
  factor-base points sum as recorded, and the system is over-determined) for C-RelationCollect; for
  E.K.4, **kernel correctness** (the recovered kernel vector satisfies the relation matrix over
  `Z/ℓℤ`) for C-EKLinAlg; for E.K.5, **the decisive signal — agreement with `rho::ecdlp`**
  (`log_G(Q)` from index calculus equals the Pollard-rho answer on the same toy instance). This is
  **exactly self-checking** — no msolve / PARI on the green path.
- `cargo check --workspace` must stay green — **no edge change** (the `gnfs::dl::linalg` engine and
  C1 are already `rho` dependencies), so no cycle risk. The `rho::index_calculus` module + the
  relation→matrix adapter are leaf additions.
- **The existing rho / gnfs / shared KATs must stay green** after the index-calculus code lands — E.K
  adds new modules and changes no existing field / curve / Semaev / linalg path, so the no-regression
  invariant is structurally easy to hold; `cargo test --workspace` is the guard.
- **No live oracle:** the green-path correctness signal is the self-checking `rho::ecdlp` agreement.
  An optional `msolve` cross-check on the point-decomposition Gröbner step (E.K.2) follows the
  established `#[ignore]` pattern; never on the green path. E.K introduces **no new live oracle**
  (principle 3) — the ROADMAP's "msolve as a dev-only oracle, parallel to CADO-NFS" is realised as
  an `#[ignore]`-gated sidecar, exactly as PARI is.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.K.1 ◆-start `@architect` | Index-calculus strategy + factor base + relation/matrix contract | A | **Opus** | C-Semaev (frozen E.J — `rho::semaev`, read); `rho::curve::Curve`/`AffinePoint` (frozen, read — `is_on_curve`/`scalar_mul`/`n`/generator); C-Fp (read); C1 `shared::numth::trial_smooth`/`SmoothWitness` (frozen, read) | `rho/src/index_calculus/mod.rs` (new: module skeleton + `IndexCalcError` enum + the toy fixture wiring `semaev_toy()` + the prime-order-subgroup `ℓ` choice), `rho/src/index_calculus/strategy.rs` (new: factor-base point enumeration + `m` + the `Relation` type C-EKRelation), `rho/src/lib.rs` (add `pub mod index_calculus;`), `rho/tests/index_calculus_kat.rs` (new: factor-base-on-curve, `ℓ | n`, C-EKRelation round-trip KATs) |
| E.K.2 | Index-calculus point decomposition via Semaev | B | Sonnet | C-IndexCalcStrategy (frozen E.K.1); C-Semaev (frozen E.J — `semaev_poly`/`MultiPoly::partial_eval`/`elim_var_resultant`); `rho::curve::Curve` (read — sum-to-`Q` check) | `rho/src/index_calculus/decompose.rs` (new: `decompose(Q) → Option<Vec<FbPoint>>` via Semaev root-finding over the factor base), `rho/src/index_calculus/mod.rs` (add `pub mod decompose;`), `rho/tests/index_calculus_kat.rs` (extend: a returned decomposition sums to `Q` via the frozen group law; optional `#[ignore]` msolve cross-check) |
| E.K.3 | Index-calculus relation collection | B | Sonnet | C-PointDecomp (frozen E.K.2); C-EKRelation (frozen E.K.1); `rho::curve::Curve`/`scalar_mul` (read — random `a·G + b·Q`) | `rho/src/index_calculus/collect.rs` (new: `collect_relations(g, q) → Vec<Relation>` loop), `rho/src/index_calculus/mod.rs` (add `pub mod collect;`), `rho/tests/index_calculus_kat.rs` (extend: every relation's points sum as recorded; the system is over-determined) |
| E.K.4 | Index-calculus `Z/ℓℤ` linear algebra: relation→`FlSparseMatrix` adapter + solve | B | Sonnet | C-RelationCollect (frozen E.K.3); C-EKRelation (frozen E.K.1); `gnfs::dl::linalg::{FlSparseMatrix, block_lanczos_fl, block_wiedemann_fl, FlMatrixOperator}` (frozen G.E engine, read — reused over `Z/ℓℤ = F_ℓ`) | `rho/src/index_calculus/linalg.rs` (new: `build_ek_matrix(&[Relation]) → FlSparseMatrix` adapter + the `Z/ℓℤ` solve wrapper), `rho/src/index_calculus/mod.rs` (add `pub mod linalg;`), `rho/tests/index_calculus_kat.rs` (extend: the recovered kernel satisfies the relation matrix over `Z/ℓℤ`) |
| E.K.5 ◆ `@architect` | Index-calculus DLP recovery + `rho::ecdlp` cross-check + sub-track close | I | Sonnet | C-EKLinAlg (frozen E.K.4); all prior C-EK contracts; `rho::ecdlp::solve_*` (frozen, read — the cross-check oracle); `rho::curve::Curve` (read) | `rho/src/index_calculus/solve.rs` (new: `index_calculus_dlp(g, q) → Option<log>` assembling the pipeline + `pub use`), `rho/src/index_calculus/mod.rs` (add `pub mod solve;` + `pub use`), `rho/tests/index_calculus_kat.rs` (extend: `log_G(Q)` agrees with `rho::ecdlp` on the toy instance; principle-4 annotation; sub-track-close suite; optional `#[ignore]` msolve sidecar) |

**Sequencing notes.** Strictly serial: **E.K.1 → E.K.2 → E.K.3 → E.K.4 → E.K.5.** E.K.1 lands the
strategy + factor base + the relation contract the whole pipeline consumes; E.K.2 the Semaev point
decomposition; E.K.3 the relation-collection loop; E.K.4 the `Z/ℓℤ` linear algebra; E.K.5 the DLP
recovery + cross-check + close. **Two `@architect` markers:** the **E.K.1 ◆-start** (the ROADMAP's
pre-scheduled Opus inflection — the strategy/relation-contract design, paged *before* dispatch as a
juncture fork to ratify the C-EKRelation shape the three downstream sessions consume) and the **E.K.5
◆** (the boundary juncture ratifying the five frozen contracts and confirming the index-calculus
solver is rho-cross-checked and E.W-ready before the sub-track closes). *(Tradeoff named: E.K pages a
juncture at BOTH the opening (E.K.1, unusual — most sub-tracks fork only at the ◆-close) and the close
(E.K.5). The opening fork is bought because C-EKRelation is the index-calculus analogue of NFS's
C-Relation — a wrong relation/matrix shape is a retrofit across decomposition, collection, AND linear
algebra, and the ROADMAP's native Opus flag on E.K.1 signals exactly this cost-of-wrong. This mirrors
the G.A.1 / D.A.1 / E.H.1 substrate-session Opus pattern, where the first session's contract is the
expensive one.)*

**Why 5 sessions (the ROADMAP ceiling, confirmed by ceiling-bias).** The split is taken at five
pipeline/contract-sharp seams:
- **One-line-commit-title corollary.** "Index-calculus strategy + factor base + relation contract",
  "point decomposition via Semaev", "relation collection", "`Z/ℓℤ` linear algebra adapter + solve",
  and "DLP recovery + cross-check + close" are **five distinct commit titles** across three categories
  (A substrate ×1, B algorithm ×3, I integrative ×1).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit of the index-calculus
  pipeline. None is fractured below its floor: the strategy/contract design is one unit (Opus); the
  decomposition is one unit (the Semaev consumption); collection wraps decomposition in a loop; linear
  algebra is the adapter+solve; recovery+cross-check is the assembly.
- **Contract-sharp boundary.** E.K.1 **freezes** C-IndexCalcStrategy + C-EKRelation; E.K.2 consumes
  the strategy and **freezes** C-PointDecomp; E.K.3 consumes both and **freezes** C-RelationCollect;
  E.K.4 consumes collection + the relation contract and **freezes** C-EKLinAlg; E.K.5 consumes the
  linalg and **freezes** C-IndexCalc. Each later session is meaningless without the earlier freeze.

**The softest seam — decomposition↔collection (E.K.2↔E.K.3), the one place the 5-vs-4 sizing is
genuinely uncertain (shard-time decision, user-adjudicated).** Relation collection (E.K.3) is largely
a *loop around* point decomposition (E.K.2): generate a random `a·G + b·Q`, decompose it, record the
relation, repeat. The two *could* be one "relation generation" session (giving 4 sessions). The user
chose **5: split at the decomposition↔collection seam**, matching the ROADMAP's ceiling and the
project's documented ceiling-bias (G/D both landed at or above their upper bands). The split buys an
early C-PointDecomp freeze (the Semaev-consumption interface) and keeps the relation-generation
session under the LOC band. **If E.K.2's decomposition and E.K.3's collection prove tightly coupled**
(collection is a thin loop with no genuine reusable decomposition seam), the split is artificial and
**E.K.2/E.K.3 should re-merge** — a judgment E.K.2 can surface once the decomposition interface is
concrete (an additive-reshard discovery, not a silent merge). This is the one place the 5-vs-4 sizing
is genuinely uncertain until decomposition lands.

---

## Session detail

E.K.1 and E.K.2 are specified at near-full fidelity (the strategy/relation-contract and the Semaev
decomposition are the design crux the whole pipeline — and downstream E.W — stand on). E.K.3–E.K.5
are lower-fidelity sketches, correct per the substrate-first discipline: they are crisply specified
only after C-EKRelation and C-PointDecomp freeze.

### E.K.1 ◆-start — Index-calculus strategy + factor base + relation/matrix contract (Opus, Cat A)

**Deliverable:** the strategy substrate the index-calculus solver stands on — the ROADMAP's Opus
session. The pieces:
- **The prime-order subgroup `ℓ`** (`rho/src/index_calculus/mod.rs`): the toy fixture's group order
  `n = 60 = 2²·3·5` is composite; the linear algebra runs over `Z/ℓℤ` for a prime `ℓ | n` (so the
  matrix is over a field `F_ℓ`). Fix `ℓ` (the largest convenient prime factor, e.g. `ℓ = 5`) and the
  generator's `ℓ`-order subgroup. *(This is the index-calculus analogue of NFS-DL working mod `ℓ`.)*
- **Factor-base point enumeration** (`rho/src/index_calculus/strategy.rs`): greenfield (no
  `points_on_curve()` exists). Enumerate `{(x, y) : y² = x³ + ax + b, x ∈ small set}` — for each `x`,
  test whether `x³ + ax + b` is a QR mod `p` and lift `y`; the factor base is a chosen subset (the
  small-x-coordinate points). Each point gets a factor-base index.
- **The relation/matrix contract C-EKRelation** (`rho/src/index_calculus/strategy.rs`): the `Relation`
  type — an exponent vector over `Z/ℓℤ` indexed by factor-base points, plus the `(a, b)` recording
  the multiple `a·G + b·Q` it came from. **The design crux** (the index-calculus analogue of NFS's
  C-Relation): the representation consumed by decomposition (which produces relations), collection
  (which accumulates them), and linear algebra (which builds the matrix from them). **Over-specify
  (substrate rule):** carry the fields E.K.4's `FlSparseMatrix` adapter and E.K.5's recovery need
  (the factor-base index map, the `(a, b)` provenance, the `Z/ℓℤ` exponents) if confidence is
  reasonable — adding them later is costlier.
- **The module skeleton + fixture** (`rho/src/index_calculus/mod.rs`): an `IndexCalcError` enum (the
  established attack-module idiom — cf. `rho::ssa::SsaError`, `rho::ghs::GhsError`,
  `rho::semaev::SemaevError`), the toy fixture wiring `semaev_toy()` (`F_47`, `n = 60`, `G = (10, 3)`)
  + the chosen `ℓ` + factor base.
- **Confirm-and-record (the load-bearing E.K.1 acts):** (a) **the prime-order subgroup `ℓ`** — record
  the chosen `ℓ | n` and why (the matrix-over-a-field precondition); (b) **the factor base + arity
  `m`** — record the factor-base shape and the decomposition arity (how many factor-base points a
  decomposition targets); (c) **the C-EKRelation representation** — record the relation type and why
  (the property decomposition/collection/linalg exploit); (d) **the `gnfs::dl::linalg` reuse decision**
  — record that E.K.4 reuses the `Fp<L>`-generic engine with an E.K adapter (the survey finding),
  ratified here so E.K.4 is not surprised.

Consumes C-Semaev (frozen E.J — `rho::semaev`, read), `rho::curve::Curve`/`AffinePoint` (frozen, read
— `is_on_curve`/`scalar_mul`/`n`/generator), C-Fp (read), C1 `shared::numth::trial_smooth`/
`SmoothWitness` (frozen, read — though E.K's "smoothness" is *decomposes-over-the-factor-base*, not
integer smoothness; whether C1 is genuinely consumed or merely structurally analogous is an E.K.1
finding). **Freezes C-IndexCalcStrategy, C-EKRelation.**

**KAT** (`rho/tests/index_calculus_kat.rs` + inline unit tests): over the toy fixture: **factor-base
on-curve** (every enumerated factor-base point satisfies `is_on_curve` via the frozen `Curve`);
**subgroup validity** (`ℓ | n`; the generator's `ℓ`-multiple is the identity); **C-EKRelation
round-trip** (a relation's exponent vector reconstructs the recorded factor-base-point sum). **Verify
gate:** `cargo test --workspace` green; `cargo check --workspace` green (leaf additions, no edge
change); existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **The prime-order subgroup is the linear-algebra precondition** — a
`@build` agent that builds the relation system over `Z/nℤ` (composite `n`) gets a matrix over a ring,
not a field, and the block-Lanczos engine fails; the subgroup `ℓ` is non-negotiable. (2) **C-EKRelation
is the expensive contract** — wrong shape retrofits decomposition + collection + linalg; over-specify
(carry the factor-base index map + `(a, b)` provenance + `Z/ℓℤ` exponents). (3) **The `gnfs::dl::linalg`
reuse is partial** — the engine (`FlSparseMatrix`, `block_lanczos_fl`) is reusable; the construction
(`build_fl_matrix` from `DLMatrix`) is not; E.K.4 writes its own adapter. Record this here so E.K.4
plans for the adapter, not a duplicate solver. (4) **"Smoothness" is decomposes-over-the-factor-base,
not integer smoothness** — C1 `trial_smooth` is the *structural analogue* (the ROADMAP's "semantically
different but structurally similar"); whether E.K literally calls `trial_smooth` or just mirrors the
`SmoothWitness`-shaped relation idiom is an E.K.1 call. (5) **Mechanism, not asymptotic win** — record
the principle-4 boundary up front: E.K-over-`F_p` is the index-calculus *mechanism*; the asymptotic
win needs the deferred `F_{p^n}` re-shard.

**Deferred:** the decomposition (E.K.2); collection (E.K.3); linear algebra (E.K.4); recovery (E.K.5);
the `F_{p^n}` Semaev lift + GHS coupling (deferred re-shards); the MATHEMATICS chapter (T.E at the
Track-E ◆).

**`@architect` ◆-start fork (pre-dispatch, Opus, one-shot).** Page a `@plan-juncture` fork *before*
E.K.1 is dispatched (the ROADMAP's pre-scheduled Opus inflection) to ratify: (1) the C-EKRelation
shape (the exponent-vector-over-`Z/ℓℤ` representation) is the right interface for the three downstream
consumers; (2) the factor base + `ℓ` + arity `m` strategy is sound for the toy fixture (the factor
base is large enough to over-determine, small enough to decompose); (3) the `gnfs::dl::linalg` reuse
plan (engine reused, adapter written) is confirmed so E.K.4 is not surprised; (4) the principle-4
boundary (mechanism, not asymptotic win) is recorded. One-shot findings; does not implement. Held at
**Opus** per the header.

### E.K.2 — Index-calculus point decomposition via Semaev (Sonnet, Cat B)

**Deliverable:** the point-decomposition step — the heart of index calculus and the consumer the
Semaev primitive was built for. Near-full fidelity. The pieces:
- **`decompose(Q)`** (`rho/src/index_calculus/decompose.rs`): given a point `Q`, find a sum of `m`
  factor-base points equal to `Q` (equivalently, `Q − P_{i_1} − ⋯ − P_{i_m} = ∞`) by finding roots of
  the Semaev polynomial. Specialise `semaev_poly(m+1)` at `Q`'s x-coordinate (partial-evaluate one
  argument), then find roots with all remaining coordinates in the factor base via the frozen
  `MultiPoly::partial_eval` + `elim_var_resultant` (native root-finding / resultant-elimination over
  the factor-base x-coordinates — no live oracle on the green path). Returns `Option<Vec<FbPoint>>`
  (the decomposition, or `None` if `Q` does not decompose).
- **The green-path engine is native, the msolve sidecar is `#[ignore]`** — at toy scale (small `m`,
  small factor base) the decomposition is found by enumerating factor-base x-coordinate assignments
  and checking the Semaev vanishing via the frozen `elim_var_resultant`; an optional msolve cross-check
  (`#[ignore = "msolve not installed; run manually when available"]`) realises the ROADMAP's
  "Gröbner-basis step delegates to msolve as a dev-only oracle" without a green-path dependency.

Consumes C-IndexCalcStrategy (frozen E.K.1), C-Semaev (frozen E.J — `semaev_poly`/`partial_eval`/
`elim_var_resultant`), `rho::curve::Curve` (read — the sum-to-`Q` check). **Freezes C-PointDecomp.**

**KAT** (`rho/tests/index_calculus_kat.rs`, extended): over the toy fixture: **decomposition
correctness** (a decomposition `decompose(Q)` returns actually sums to `Q` via the frozen `Curve`
group law — checked directly); **decomposition completeness** (a point known to decompose is found;
a point with no factor-base decomposition returns `None`); optional **msolve cross-check** (the
Gröbner root set agrees, `#[ignore]`-gated). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **This is the Semaev-consumption step** — the frozen `MultiPoly`
operations (`partial_eval` to specialise at `Q`'s x-coordinate, `elim_var_resultant` to eliminate
variables one at a time) are exactly what E.J.1 over-specified for this consumer; a `@build` agent
re-deriving Semaev evaluation instead of consuming the frozen surface is rigidity. (2) **The
correctness signal is sum-to-`Q` via the group law** — a returned decomposition is correct iff its
factor-base points sum to `Q` (the frozen group law) — exactly self-checking, no oracle. (3) **Native
green path, msolve sidecar** — the green path must not depend on msolve (principle 3); native
enumeration is the toy-scale engine (the principle-4 boundary: native enumeration does not scale past
toy `m`, annotated). (4) **The decomposition↔collection seam check** — if collection (E.K.3) is a
thin loop with no genuine reusable decomposition interface, this is the loud signal to surface the
E.K.2/E.K.3 merge.

**Deferred:** collection (E.K.3); linear algebra (E.K.4); recovery (E.K.5); the deferred re-shards;
the MATHEMATICS chapter (T.E).

### E.K.3 — Index-calculus relation collection (Sonnet, Cat B)

**Deliverable:** the relation-collection loop. Lower-fidelity sketch (crisp after C-PointDecomp
freezes):
- **`collect_relations(g, q)`** (`rho/src/index_calculus/collect.rs`): repeatedly generate a random
  multiple `R = a·G + b·Q` (via the frozen `Curve::scalar_mul`), call `decompose(R)`, and on success
  record a C-EKRelation row (the `(a, b)` provenance + the factor-base exponent vector). Loop until
  the system is over-determined (≥ factor-base-size + 1 relations).
- **Over-determination + de-duplication** — collect slightly more relations than factor-base points
  so the kernel is found; drop duplicate/dependent relations if cheap.

Consumes C-PointDecomp (frozen E.K.2), C-EKRelation (frozen E.K.1), `rho::curve::Curve`/`scalar_mul`
(read). **Freezes C-RelationCollect.**

**KAT** (`rho/tests/index_calculus_kat.rs`, extended): over the toy fixture: **relation validity**
(every collected relation's factor-base points sum to `a·G + b·Q` via the frozen group law);
**over-determination** (the collected system has more relations than factor-base points). **Verify
gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The seam check** — if collection is a thin loop around
`decompose`, surface the E.K.2/E.K.3 merge (additive-reshard). (2) **Over-determination is required**
— too few relations and the kernel is trivial / the log is not recoverable; the loop must collect
enough. (3) **The provenance `(a, b)` is load-bearing** — the recovered log depends on the `a·G + b·Q`
each relation records; a relation missing its provenance is useless for recovery.

**Deferred:** linear algebra (E.K.4); recovery (E.K.5); the deferred re-shards; the chapter (T.E).

### E.K.4 — Index-calculus `Z/ℓℤ` linear algebra: relation→`FlSparseMatrix` adapter + solve (Sonnet, Cat B)

**Deliverable:** the linear-algebra step — the **partial-reuse** of the frozen G.E engine (the
load-bearing survey finding). Lower-fidelity sketch (crisp after C-RelationCollect freezes):
- **The adapter** (`rho/src/index_calculus/linalg.rs`): `build_ek_matrix(&[Relation]) → FlSparseMatrix`
  — the E.K analogue of `gnfs::dl::linalg::build_fl_matrix`, but from C-EKRelation rows (not
  `DLMatrix`). Each relation becomes a sparse row; the matrix is over `Z/ℓℤ = F_ℓ`.
- **The solve** (`rho/src/index_calculus/linalg.rs`): call the frozen `block_lanczos_fl` (or
  `block_wiedemann_fl`) on the `FlMatrixOperator` view of the adapter's matrix to find the kernel /
  the virtual logs over `Z/ℓℤ`. **Reuses the frozen engine; writes no new solver.**

Consumes C-RelationCollect (frozen E.K.3), C-EKRelation (frozen E.K.1), `gnfs::dl::linalg::
{FlSparseMatrix, block_lanczos_fl, block_wiedemann_fl, FlMatrixOperator}` (frozen G.E engine, read).
**Freezes C-EKLinAlg.**

**KAT** (`rho/tests/index_calculus_kat.rs`, extended): over the toy fixture: **kernel correctness**
(the recovered kernel vector satisfies `M·v = 0` over `Z/ℓℤ` for the relation matrix `M`); **adapter
fidelity** (`build_ek_matrix` produces a matrix whose row `i` matches relation `i`'s exponent vector).
**Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **Reuse the engine, write the adapter** — `block_lanczos_fl` is
`Fp<L>`-generic and reusable; only the `DLMatrix`-bound construction is not. A `@build` agent writing
a fresh block-Lanczos solver is defocus (the engine exists). (2) **`Z/ℓℤ = F_ℓ` makes the engine fit**
— the prime-order subgroup is what licenses reusing the `F_ℓ` solver; if E.K.1 chose `ℓ` composite by
mistake, this session fails loudly. (3) **The read-coupling into `gnfs::dl`** — E.K reads `gnfs::dl::
linalg` internals (the F_ℓ solver designed for NFS-DL); if the surface proves ill-fit for the
prime-order-subgroup case, this session grows — surfaced as a discovery, not silently absorbed.

**Deferred:** recovery (E.K.5); the deferred re-shards; the chapter (T.E).

### E.K.5 ◆ — Index-calculus DLP recovery + `rho::ecdlp` cross-check + sub-track close (Sonnet, Cat I)

**Deliverable:** the pipeline assembly + the decisive cross-check + the sub-track close.
Lower-fidelity sketch (crisp after C-EKLinAlg freezes):
- **`index_calculus_dlp(g, q)`** (`rho/src/index_calculus/solve.rs`): assemble the pipeline —
  enumerate the factor base (E.K.1), collect relations (E.K.3 via E.K.2), solve the `Z/ℓℤ` system
  (E.K.4), and recover `log_G(Q)` from the kernel + the relation provenance (the standard
  index-calculus recovery: a kernel relation `Σ e_i·log(P_i) ≡ 0` combined with a relation involving
  `Q` yields `log_G(Q)` mod `ℓ`). Returns `Option<log>`.
- **The decisive cross-check + close KAT** (`rho/tests/index_calculus_kat.rs`, extended):
  `index_calculus_dlp(G, Q)` agrees with `rho::ecdlp::solve_*` on the same toy instance — the
  green-path correctness signal; the principle-4 annotation (mechanism-correct, asymptotic win not
  observable); the sub-track-close suite; an optional `#[ignore]` msolve sidecar.

Consumes C-EKLinAlg (frozen E.K.4), all prior C-EK contracts, `rho::ecdlp::solve_*` (frozen, read —
the cross-check oracle), `rho::curve::Curve` (read). **Freezes C-IndexCalc.**

**KAT (primary correctness signal):** over the toy fixture: **`rho::ecdlp` agreement** (the index-
calculus `log_G(Q)` equals the Pollard-rho answer — the decisive signal, exactly self-checking);
**recovery soundness** (`log_G(Q)·G = Q` via the frozen `scalar_mul`); **end-to-end** (the full
pipeline solves a non-trivial toy ECDLP). Optional msolve cross-check (`#[ignore]`). **Verify gate:**
`cargo test --workspace` green; existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **`rho::ecdlp` agreement is the correctness signal** — the recovered
log is correct iff it matches the independent Pollard-rho solver on the same instance; this is the
green-path guard (exactly self-checking, no oracle — the lever-5 strength). (2) **This is the E.K ◆
boundary** — re-read the Purpose intent and verify the solver is complete (factor base → decomposition
→ collection → linalg → recovery all present and rho-cross-checked) and **E.W-ready** (C-IndexCalc
exposes what the cross-attack benchmark consumes — `index_calculus_dlp` + the relation/decomposition
counts for the benchmark table), and that E.K stayed Semaev-extension-free / GHS-free / chapter-free.
(3) **Mechanism, not asymptotic win** — the principle-4 annotation must record that E.K-over-`F_p` is
the index-calculus *mechanism* (the asymptotic win needs the deferred `F_{p^n}` re-shard), never
presented as faster-than-rho. (4) **No Semaev extension, no GHS coupling** — the `F_{p^n}` lift and
the descended-Jacobian coupling are deferred re-shards, not E.K. (5) **No MATHEMATICS chapter** — the
index-calculus textbook content is T.E, paired with E.W at the *Track-E* ◆; E.K.5 writes at most a
PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.K.5 ◆ to confirm: (1) the index-calculus solver is complete and composes (factor base →
decomposition → collection → linalg → recovery all present and rho-cross-checked — the factor-base
on-curve, decomposition sum-to-`Q`, relation validity, kernel correctness, and `rho::ecdlp` agreement
all pass); (2) C-IndexCalc exposes what E.W's cross-attack benchmark consumes (`index_calculus_dlp` +
the per-run decomposition/relation counts for the "which attack wins on which curve" table) so E.W can
build the benchmark without amending the solver surface; (3) E.K stayed in scope — no `F_{p^n}` Semaev
lift, no GHS coupling, no MATHEMATICS chapter (T.E), the index calculus is a mechanism cross-checked
against rho, not an asymptotic-win demonstration; (4) the principle-4 boundary (mechanism-correct,
asymptotic win NOT observable, toy `F_p`/`m`/`ℓ`) is recorded, not silently presented as the
index-calculus speed-up; (5) **the strategy / factor-base / `gnfs::dl::linalg`-reuse resolutions** —
confirm the chosen `ℓ`, factor base, arity `m`, the C-EKRelation shape held across all four consumers,
and the linalg-engine reuse worked (or the adapter grew, surfaced). **Also: surface the outstanding
static-frame ROADMAP debt** carried + compounded from the E.I, E.H, and E.J ◆ (see the Discoveries &
risks entry) — though the ROADMAP write itself is out of `@architect` PLAN-write scope (a capture
candidate, not a PLAN edit). One-shot findings; does not implement. Held at **Opus** per the header.

---

## Cross-session contracts

E.K **freezes six** contracts (C-IndexCalcStrategy + C-EKRelation at E.K.1, C-PointDecomp at E.K.2,
C-RelationCollect at E.K.3, C-EKLinAlg at E.K.4, C-IndexCalc at E.K.5) and **amends no prior frozen
contract** (C-Semaev / `rho::curve::Curve` / `AffinePoint` / C-Fp / C1 `trial_smooth` / the
`gnfs::dl::linalg` engine are all consumed-or-untouched). E.K adds the `rho::index_calculus` module +
the relation→matrix adapter — all **additive**, no trait amendment.

### C-EKRelation — the index-calculus relation/matrix contract (compiler- + test-enforced) — *frozen at E.K.1; ratified at the ◆-start fork 2026-06-16*

**Defined in:** E.K.1 (`rho/src/index_calculus/strategy.rs`). **Consumed by:** E.K.2 (decomposition
*produces* relations), E.K.3 (collection *accumulates* them), E.K.4 (the adapter *builds the matrix*
from them), E.K.5 (recovery *reads the provenance*); **downstream: E.W** (the per-run relation counts
for the benchmark table). Compiler- + test-enforced. **Over-specified** (substrate rule) for E.K.4's
adapter + E.K.5's recovery.

**Ratified representation (◆-start fork).** The relation carries (i) the `(a, b)` provenance of the
`R = a·G + b·Q` multiple, (ii) the factor-base decomposition as a **sparse exponent vector over
`F_ℓ = Z/ℓℤ`**, stored in the **exact shape E.K.4's adapter feeds to `FlSparseMatrix`** — `Vec<(usize,
F)>` of `(factor-base index, exponent mod ℓ)` pairs. The field element type is the workspace `Fp`
trait (`rho::field::Fp` *is* a re-export of `shared_field::Fp`, so the curve field and the
`gnfs::dl::linalg` engine share one trait — confirmed at the fork; no cross-crate adapter on the
scalar type). The factor-base index map lives in C-IndexCalcStrategy (`IndexCalcStrategy::factor_base:
Vec<FbPoint>`), not duplicated here — a `Relation` references factor-base points by their index.

```rust
use crypto_bigint::Uint;
use crate::field::FpNaive;            // = shared_field::FpNaive<4>; the F_ℓ scalar the linalg engine consumes
type Fl = FpNaive;                    // F_ℓ element type; L = 4 fixes the Uint<4> ceiling (toy p, ℓ)

/// One index-calculus relation: a decomposition of `R = a·G + b·Q` over the factor base.
///
/// The exponent vector is **sparse over `F_ℓ`** (`Vec<(fb_index, exp mod ℓ)>`) — exactly the row
/// shape E.K.4's `build_ek_matrix` pushes into `FlSparseRow`, so the adapter is a near-identity
/// copy (no re-encoding). Provenance `(a, b)` is load-bearing for E.K.5 recovery (the kernel
/// relation involving `Q` yields `log_G(Q)` mod ℓ); never dropped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    /// Provenance: the multiple `R = a·G + b·Q` this relation decomposes. Recovery (E.K.5) reads it.
    pub a: u64,
    pub b: u64,
    /// Sparse exponent vector over `F_ℓ`, indexed by factor-base point index.
    /// `exponents[k] = (i, e_i)` means factor-base point `i` appears with coefficient `e_i mod ℓ`
    /// in the decomposition `R = Σ e_i · P_i`. Entries are sorted by index; no index repeats.
    pub exponents: Vec<(usize, Fl)>,
}

impl Relation {
    /// Construct from a list of factor-base indices the decomposition `R = Σ P_{i_k}` hit.
    /// Accumulates repeated indices into `F_ℓ` exponents, sorts, drops zero entries.
    pub fn from_decomposition(a: u64, b: u64, fb_indices: &[usize], ell: &Uint<4>) -> Self { /* E.K.1 */ }

    /// The exponent of factor-base point `i` (zero `F_ℓ` element if absent).
    pub fn exponent(&self, i: usize, ell: &Uint<4>) -> Fl { /* E.K.1 */ }
}
```

**Invariants:** exponents live in `F_ℓ` (the prime-order subgroup — the matrix-over-a-field
precondition); a `Relation`'s exponent vector reconstructs its recorded factor-base-point sum (the
round-trip KAT: `Σ e_i·P_i` over the frozen group law equals `a·G + b·Q`); `exponents` is sorted by
index with no duplicate index (the `FlSparseRow` CSR invariant, so `build_ek_matrix` need not re-sort).
*(The index-calculus analogue of NFS's C-Relation — the expensive contract the Opus E.K.1 freezes.
The sparse-`Vec<(usize, Fl)>` shape is chosen specifically so E.K.4's adapter is a near-identity map
onto `FlSparseRow { entries: Vec<(usize, F)> }` — the partial-reuse survey finding cashed out in the
type.)*

### C-IndexCalcStrategy — the factor base + prime-order subgroup + decomposition arity (compiler- + test-enforced) — *frozen at E.K.1; ratified at the ◆-start fork 2026-06-16*

**Defined in:** E.K.1 (`rho/src/index_calculus/strategy.rs` + `mod.rs`). **Consumed by:** E.K.2
(decomposes over the factor base at arity `m`), E.K.3 (collects relative to the factor base), E.K.4
(the matrix is over `Z/ℓℤ`), E.K.5 (recovery works mod `ℓ`). Compiler- + test-enforced.

**Ratified values (◆-start fork).**
- **Prime-order subgroup `ℓ = 5`** — the largest prime factor of `n = 60 = 2²·3·5`. The relation
  exponents and the linear algebra live over `F_ℓ = F_5` (a field — the block-Lanczos precondition).
  The `ℓ`-order subgroup is generated by `G_ℓ = (n/ℓ)·G = 12·G`; recovery yields `log_G(Q) mod ℓ`.
  *(Mechanism-only: a single small prime `ℓ` demonstrates the index-calculus pipeline; full
  `log_G(Q) mod n` would CRT over all prime-power factors — out of scope, a principle-4 boundary.)*
- **Factor base** — the small-x-coordinate points: enumerate `x ∈ {0, 1, 2, …}` in ascending order,
  keep `x` for which `x³ + ax + b` is a QR mod `p`, lift the **canonical** root `y` (the smaller of
  `±y`), take the first `FB_SIZE` such points. Ratified `FB_SIZE = 6` (over-determinable at `m = 2`:
  the collection target is `≥ FB_SIZE + 1 = 7` relations; the toy curve's ~30 affine x-coordinates
  with a QR supply ample candidates). Each point gets a stable factor-base index (its position in the
  enumeration). *(Tradeoff named: a fixed small `FB_SIZE` is worse at decomposition success-rate than
  a larger base — fewer target points means more `(a,b)` trials per relation — but better at keeping
  the `F_5` matrix small and the round-trip KAT auditable. If E.K.3's collection cannot over-determine
  at `FB_SIZE = 6`, grow it — an additive-reshard, not a contract break, since `FB_SIZE` is a strategy
  constant, not a type.)*
- **Decomposition arity `m = 2`** — a point `Q` decomposes as a sum of `m = 2` factor-base points,
  found via `semaev_poly(m + 1) = semaev_poly(3) = S_3` specialised at `Q`'s x-coordinate (E.K.2).
  `m = 2` is the smallest non-trivial arity (`S_3` is already built and KAT-covered in `rho::semaev`);
  it keeps the native root-enumeration green path at `O(FB_SIZE)` per decomposition. *(Tradeoff: `m = 2`
  is worse at decomposition density than larger `m` — fewer points decompose as a 2-sum than as a
  3-sum — but `S_3` is the frozen, tested polynomial and `m = 2` is the minimal mechanism. If density
  is too low to over-determine, E.K.2/E.K.3 surface raising to `m = 3` via `S_4` — additive-reshard.)*

```rust
use crypto_bigint::Uint;
use crate::curve::{AffinePoint, Curve};
use crate::field::FpNaive;            // = shared_field::FpNaive<4>

/// The prime-order subgroup modulus for the toy fixture: ℓ = 5 (the largest prime factor of n = 60).
pub const TOY_ELL: u64 = 5;
/// Factor-base size for the toy fixture (over-determinable at m = 2).
pub const TOY_FB_SIZE: usize = 6;
/// Decomposition arity: a point decomposes as a sum of `m` factor-base points (via S_{m+1}).
pub const TOY_M: usize = 2;

/// A factor-base point: a curve point plus its stable factor-base index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FbPoint {
    /// Index into the factor base (the relation exponent-vector column).
    pub index: usize,
    /// The curve point (a finite affine point on the frozen `Curve`).
    pub point: AffinePoint<FpNaive>,
}

/// The index-calculus strategy substrate: factor base + subgroup + arity, bound to a curve.
#[derive(Clone, Debug)]
pub struct IndexCalcStrategy {
    /// The curve (the frozen `semaev_toy()` fixture for the toy instance).
    pub curve: Curve,
    /// The prime-order subgroup modulus ℓ | n (ℓ = 5 for the toy).
    pub ell: Uint<4>,
    /// The factor base: small-x-coordinate points, each with its index.
    pub factor_base: Vec<FbPoint>,
    /// The decomposition arity m (m = 2 for the toy).
    pub m: usize,
}

impl IndexCalcStrategy {
    /// Build the toy strategy: `semaev_toy()` curve, ℓ = 5, the first `TOY_FB_SIZE` small-x points,
    /// m = 2. Enumerates the factor base via QR-test + canonical-root lift over the frozen field.
    pub fn toy() -> Result<Self, IndexCalcError> { /* E.K.1 */ }

    /// Enumerate factor-base points: ascending `x`, keep QR, lift canonical `y`, take `fb_size`.
    pub fn enumerate_factor_base(
        curve: &Curve,
        fb_size: usize,
    ) -> Result<Vec<FbPoint>, IndexCalcError> { /* E.K.1 */ }

    /// The factor-base size (the relation exponent-vector dimension).
    pub fn fb_size(&self) -> usize { self.factor_base.len() }

    /// The ℓ-order subgroup generator `(n/ℓ)·G` (recovery and KAT use it).
    pub fn subgroup_generator(&self) -> AffinePoint<FpNaive> { /* E.K.1 */ }
}
```

**Invariants:** every factor-base point is on the curve (frozen `is_on_curve`); `ℓ` is prime and
divides `n` (the matrix-over-a-field precondition — KAT: `ℓ | n` and `ℓ·G_ℓ = ∞`); the factor base is
large enough to over-determine the system at arity `m` (`FB_SIZE + 1` relations collectible). The
factor-base index is stable (a point's index is fixed by enumeration order — relations across E.K.3
share one column indexing).

**The `IndexCalcError` enum (`rho/src/index_calculus/mod.rs`)** — mirrors the attack-module idiom
(`rho::ssa::SsaError`, `rho::ghs::GhsError`, `rho::semaev::SemaevError`): a small `Debug + Clone +
PartialEq + Eq` enum with `Display` + `std::error::Error` impls. Ratified variants (E.K.1 lands these;
E.K.2–E.K.5 extend additively as their steps need):

```rust
/// Errors from the index-calculus ECDLP solver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexCalcError {
    /// The chosen subgroup modulus ℓ does not divide the group order n, or is not prime.
    InvalidSubgroup { ell: u64, n: u64 },
    /// Factor-base enumeration could not find `requested` QR points (curve too small).
    FactorBaseTooSmall { requested: usize, found: usize },
    /// A Semaev / curve operation surfaced an arity or variable error (wraps `SemaevError`).
    Semaev(crate::semaev::SemaevError),
    // E.K.2+ extend additively: DecompositionFailed, UnderdeterminedSystem, NoKernel,
    // RecoveryFailed, CrossCheckMismatch — each session adds the variant its step needs.
}
```

**C1 `trial_smooth` decision (subtlety 4, resolved).** E.K does **not** literally call `trial_smooth`
— E.K's "smoothness" is *decomposes-over-the-factor-base* (a points-on-a-curve sum), not integer
trial division, and `SmoothWitness` is `(prime, exponent)`-shaped over `Uint<4>`, structurally wrong
for an `F_ℓ` exponent vector indexed by curve points. The `Relation` type **mirrors the
`SmoothWitness` idiom** (a witness carrying the factored decomposition + a reconstruct/verify method —
cf. `SmoothWitness::verify`) but does not consume the type. C1 is read for the pattern, untouched —
exactly the PLAN's "structural analogue, not literal consumer" anticipation.

### C-PointDecomp — the Semaev point-decomposition step (compiler- + test-enforced) — *to be frozen at E.K.2*

**Defined in:** E.K.2 (`rho/src/index_calculus/decompose.rs`). **Consumed by:** E.K.3 (the collection
loop calls it). Compiler- + test-enforced. Exposes: `decompose(Q) → Option<Vec<FbPoint>>` — the
factor-base decomposition of `Q` via Semaev root-finding (the frozen `semaev_poly` + `partial_eval` +
`elim_var_resultant`). **Invariants:** a returned decomposition sums to `Q` via the frozen `Curve`
group law (the correctness signal — exactly self-checking); the green path is native (no live msolve);
the msolve cross-check is `#[ignore]`-gated. *Exact decomposition signature + the `m`-vs-`m+1` Semaev
specialisation ratified at the E.K.5 ◆.*

### C-RelationCollect — the relation-collection loop (test-enforced) — *to be frozen at E.K.3*

**Defined in:** E.K.3 (`rho/src/index_calculus/collect.rs`). **Consumed by:** E.K.4 (builds the matrix
from the collected relations). Test-enforced. Exposes: `collect_relations(g, q) → Vec<Relation>` — the
loop gathering an over-determined relation system. **Invariants:** every collected relation is valid
(its points sum to `a·G + b·Q` via the frozen group law); the system is over-determined (≥ factor-base
size + 1). *Exact over-determination margin + de-duplication policy ratified at the E.K.5 ◆.*

### C-EKLinAlg — the `Z/ℓℤ` linear-algebra adapter + solve (compiler- + test-enforced) — *to be frozen at E.K.4*

**Defined in:** E.K.4 (`rho/src/index_calculus/linalg.rs`). **Consumed by:** E.K.5 (recovery reads the
kernel / virtual logs). Compiler- + test-enforced. Exposes: `build_ek_matrix(&[Relation]) →
FlSparseMatrix` (the adapter) + the `Z/ℓℤ` solve wrapper around the frozen `block_lanczos_fl` /
`block_wiedemann_fl`. **Invariants:** the matrix is over `Z/ℓℤ = F_ℓ` (reuses the frozen
`Fp<L>`-generic engine — no new solver); the recovered kernel satisfies `M·v = 0`. *Exact reuse
boundary (which `gnfs::dl::linalg` items E.K reads) ratified at the E.K.5 ◆.*

### C-IndexCalc — the index-calculus ECDLP solver (compiler- + test-enforced) — *to be frozen at E.K.5 ◆*

**Defined in:** E.K.5 (`rho/src/index_calculus/solve.rs`). **Consumed by:** **E.W** (the
cross-attack benchmark table — the highest-stakes consumer). Compiler- + test-enforced. Exposes:
`index_calculus_dlp(g, q) → Option<log>` — the full pipeline (factor base → decomposition → collection
→ linalg → recovery). **The frozen invariant:** **agreement with `rho::ecdlp`** — `index_calculus_dlp`
returns `log_G(Q)` matching the frozen Pollard-rho solver on the same toy instance; `log·G = Q`.
**E.K is the index-calculus *mechanism* over `E(F_p)`; the asymptotic win (the `F_{p^n}` setting) is a
deferred re-shard.** **The index calculus is a structure-based DLP solver verified by agreement with
the generic-search baseline, NOT an asymptotic-win demonstration at this scale.** *Exact solver
signature + the benchmark counts E.W consumes ratified at the ◆.*

### Frozen contracts read by E.K (consumed, not amended)

- **C-Semaev (`rho::semaev` surface)** — `semaev_poly(m) → MultiPoly`, the `MultiPoly` operations
  `partial_eval` / `elim_var_resultant` / `eval` / `is_symmetric`, the `FpPoly`/`resultant`, the
  `semaev_toy()` fixture. The **point-decomposition oracle** (E.K.2 specialises `semaev_poly` at `Q`
  and finds factor-base roots). Read. **Unchanged — E.K amends no Semaev contract** (and collects the
  over-specification E.J.1 carried for exactly this consumer).
- **`rho::curve::Curve` / `AffinePoint` (frozen surface)** — `is_on_curve` (factor-base enumeration),
  `scalar_mul` (random multiples `a·G + b·Q`), `add_jacobian`/`negate` (the group law the
  decomposition + relation-validity checks use), `n` / generator. **Read; NOT amended.** *(Open:
  `Curve` is hardcoded `Uint<4>` — the C1 ceiling; E.K's toy `p = 47` fits it trivially.)*
- **`rho::ecdlp::solve_*` (frozen surface)** — the Pollard-rho solver (`solve_brent` / `solve_dp` /
  `solve_dp_negmap` / …). The **cross-check oracle** (E.K.5 verifies `index_calculus_dlp` agrees with
  it). Read; untouched.
- **C1 `shared::numth::trial_smooth` / `SmoothWitness`** — the integer-smoothness predicate + witness.
  The **structural analogue** of E.K's "decomposes-over-the-factor-base" (the ROADMAP's "semantically
  different but structurally similar"); whether E.K literally calls `trial_smooth` or mirrors the
  `SmoothWitness`-shaped relation idiom is an E.K.1 finding. Read for the pattern; untouched.
- **`gnfs::dl::linalg` engine** — `FlSparseMatrix`, `FlMatrixOperator`, `block_lanczos_fl`,
  `block_wiedemann_fl` (the `Fp<L>`-generic block-Lanczos/Wiedemann solver over `F_ℓ`). **Reused** by
  E.K.4 (the engine, not the `DLMatrix`-bound `build_fl_matrix` construction — E.K writes its own
  adapter). Read; untouched.
- **The attack-module idiom** — `rho::ssa` / `rho::ghs` / `rho::semaev` (the `*Error` enum + toy
  fixture + module skeleton + the `rho/tests/*_kat.rs` + `#[ignore]` oracle-gating template E.K's
  `index_calculus` module mirrors structurally). Read for the pattern; untouched.

### Workspace edges (no new edge, no new crate)

- **No new edge.** `rho` already depends on `gnfs` (where the `gnfs::dl::linalg` engine lives) and
  `shared-numth` (where C1 lives), plus `shared-field`, `shared-numfield`, `shared-gf2m`,
  `shared-bigint`, `shared-padic`. The index-calculus solver is a new module in the existing `rho`
  crate (`rho::index_calculus`); the relation→matrix adapter lives with it. No `Cargo.toml` changes;
  `cargo check --workspace` stays green with no cycle risk. *(If E.K found it must change a frozen
  trait surface — it should not, it only adds the `index_calculus` module + the adapter — that would
  be a discovery surfaced at the ◆, never a silent patch. The one genuine coupling-risk is E.K.4
  reading `gnfs::dl::linalg` internals; if that surface is ill-fit, the adapter grows — a discovery,
  not an edge change.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.K.1 ◆-start and E.K.5 ◆ `@architect` confirmations
are not separate ledger rows (paged forks with no commit-shaped deliverable); their outcomes are
recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.K.1 ◆-start | Index-calculus strategy + factor base + relation/matrix contract | done | 1f6757b | C-IndexCalcStrategy, C-EKRelation |
| E.K.2 | Index-calculus point decomposition via Semaev | done | 07c4b70 | C-PointDecomp |
| E.K.3 | Index-calculus relation collection | done | bbabf10 | C-RelationCollect |
| E.K.4 | Index-calculus `Z/ℓℤ` linear algebra adapter + solve | done | a5d6751 | C-EKLinAlg |
| E.K.5 ◆ | Index-calculus DLP recovery + `rho::ecdlp` cross-check + close | done | 592c593 | C-IndexCalc |

Contracts frozen before this sub-track: the prime-field surface (C-Fp — read by E.K, unchanged), the
prime-field curve + ECDLP surface (`rho::curve::Curve`/`AffinePoint` + `rho::ecdlp` — read as the
factor-base geometry, the group law, and the cross-check oracle), the **Semaev primitive**
(C-SemaevPoly/C-SemaevBase/C-Semaev from E.J — `rho::semaev`, read as the point-decomposition oracle —
the consumer the over-specified surface was built for), the C1 smoothness contract
(`shared::numth::trial_smooth` — read as the structural relation analogue), and the **G.E linear
algebra engine** (`gnfs::dl::linalg` block-Lanczos/Wiedemann over `F_ℓ` — *reused* by E.K.4 with an
E.K adapter). This sub-track **freezes six new contracts** (C-IndexCalcStrategy, C-EKRelation,
C-PointDecomp, C-RelationCollect, C-EKLinAlg, C-IndexCalc), serving the downstream **E.W** (cross-
attack benchmarks — the "which attack wins on which curve" table), and completing the **"solve" leg of
the transfer/structure/solve triad** (E.H transfers, E.J builds the structure, **E.K solves**) — the
Track-E index-calculus attack the project's NOTES.md framing anticipated.

---

## Action-frame digest

### E.K.1 — 2026-06-16
Discovery/flex: The ◆-start inflection fork confirmed the C-EKRelation sparse-Vec<(usize, Fl)> shape is a near-identity map onto FlSparseRow — the partial-reuse survey finding cashed out in the type. Also confirmed rho::field::Fp is a re-export of shared_field::Fp, so no cross-crate scalar adapter is needed for the linalg reuse.
Affected: C-EKRelation (ratified, not changed — confirmed sound)
Deferred: no — all four ratification questions resolved affirmatively; ℓ=5/FB_SIZE=6/m=2 additive-reshard escapes pre-authorized if density falls short at E.K.3
Texture: The inflection fork returned design-confident; self-continued. C1 trial_smooth confirmed NOT literally consumed — Relation mirrors the SmoothWitness idiom but does not call it.

### E.K.5 ◆ — 2026-06-16
Discovery/flex: Boundary fork returned still-on-intent. Two flags surfaced (non-halting): (1) C-IndexCalc returns only Option<u64> — E.W benchmark counts (decomp/relation counts) must be derived from the public collect_relations/decompose re-exports, not from a single solver call; additive in E.W's shard. (2) ROADMAP Progress/Remaining tables now stale by six completed sub-tracks (E.F–E.K) + E.H-before-E.I inversion — carried capture candidate, not a blocker. Cross-check oracle: solve_ecdlp_composite (correct for composite n=60, hands off to Pollard-rho) — reconciled, not a deviation.
Affected: C-IndexCalc (frozen as-is; E.W benchmark ergonomics flagged as additive)
Deferred: yes — E.W benchmark counts: if E.W needs counts as a first-class return, a small additive amend to C-IndexCalc is best decided before E.W shards. ROADMAP write owed.
Texture: The adapter did NOT grow (near-identity as ratified); additive-reshard escapes (grow FB_SIZE, raise m, merge E.K.2/E.K.3) were not needed — density held at m=2/FB_SIZE=6.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.K builds the index-calculus solver on the frozen `rho::semaev` + `rho::curve` + `gnfs::dl::linalg`
  surfaces — internal-continue (confirmed by survey).** The Semaev surface E.J.1 over-specified
  (`partial_eval` + `elim_var_resultant`) is exactly the point-decomposition oracle; the `gnfs::dl::
  linalg` engine is `Fp<L>`-generic and reusable. All index-calculus code is greenfield (the
  `rho::index_calculus` module). A discovery that the construction needs a curve / Semaev / linalg
  operation the frozen surfaces lack is an **additive amend** surfaced at the ◆ — not a silent patch.

- **E.K-over-`F_p` is the index-calculus MECHANISM, NOT the asymptotic win — the load-bearing scope
  decision (user-adjudicated).** Over `E(F_p)` index calculus is not faster than Pollard-rho; the
  asymptotic speed-up needs the extension-field structure of `E(F_{p^n})` (the genuine Gaudry–Diem
  setting). E.K is mechanism-correct at toy scale with the win NOT observable (the principle-4 posture
  G.E + the NFS end-to-end KAT took). **The `F_{p^n}` asymptotic-win case and the GHS-coupled
  end-to-end attack are deferred to later, separately-sharded sub-tracks, each flexing C-Semaev — NOT
  exclusions.** *(ROADMAP capture candidate — the ROADMAP's C1 named `F_{p^n}` for E.K while E.J froze
  F_p only; the resolution is E.K = F_p mechanism, the asymptotic-win + GHS cases deferred to the
  post-ROADMAP cohesion/coverage rereads. Internal-continue → recorded.)*

- **The `Z/ℓℤ` linear algebra is PARTIAL reuse of G.E — corrects the ROADMAP's "predecessor: G.E
  linear algebra" to a partial truth.** The `gnfs::dl::linalg` engine (`block_lanczos_fl`,
  `FlSparseMatrix`, `Fp<L>`-generic) is reusable; the construction (`build_fl_matrix` from `DLMatrix`)
  is NFS-bound. E.K.4 reuses the engine and writes its own E.K-relation → `FlSparseMatrix` adapter. A
  `@build` agent writing a fresh block-Lanczos solver is defocus (the engine exists); a `@build` agent
  expecting `build_fl_matrix` to accept E.K relations gets a `DLMatrix` mismatch. **Internal-continue
  → corrected** (reuse engine, write adapter). *(Risk: the read-coupling into `gnfs::dl` internals; if
  the F_ℓ surface is ill-fit for the prime-order-subgroup case, E.K.4 grows — surfaced as a discovery,
  not absorbed.)*

- **The prime-order subgroup `ℓ` is the linear-algebra precondition (the matrix-over-a-field trap).**
  The toy fixture's group order `n = 60` is composite; the relation system must run over `Z/ℓℤ` for a
  prime `ℓ | n` (so the matrix is over a field and the block-Lanczos engine applies). A `@build` agent
  building the system over `Z/nℤ` (composite) gets a matrix over a ring and the solver fails. E.K.1
  fixes `ℓ`; the subgroup-validity KAT is the guard. **Internal-continue → corrected.**

- **The point-decomposition green path is native, msolve is an `#[ignore]` sidecar — no new live oracle
  (principle-3, lever-5 strength).** At toy scale the decomposition is found by native root-finding /
  resultant-elimination over the factor base (the frozen `MultiPoly` operations); the ROADMAP's
  "Gröbner-basis step delegates to msolve as a dev-only oracle" is realised as an `#[ignore = "msolve
  not installed; run manually when available"]` cross-check, mirroring the established PARI pattern. A
  `@build` agent shelling out to msolve on the green path introduces a new live oracle (principle-3
  violation). **Internal-continue → corrected (native green path).**

- **The correctness signal is agreement with `rho::ecdlp` — exactly self-checking (lever-5
  strength).** E.K's recovered `log_G(Q)` is correct iff it matches the frozen Pollard-rho solver on
  the same toy instance — a fast, decisive, oracle-free KAT (like E.I's group axioms and E.J's
  vanishing relation). E.K introduces no new live oracle. *(Lever-5 note: this strong self-checking
  signal would license a `sonnet` juncture-tier, but the ROADMAP's native Opus flag on E.K.1 +
  lever-3 (the C-EKRelation cost-of-wrong) hold the juncture at opus — the inverse of E.J, where
  lever 5 was strong and only a user lever-3 override held it up; here the Opus flag is native.)*

- **E.K is the "solve" in the transfer/structure/solve triad (NOTES.md) — not a transfer, not a
  structure-build.** E.H transferred (GHS, frozen), E.J built the structure (Semaev, frozen), E.K
  solves (index calculus). A `@build` agent re-deriving the Semaev polynomials, or coupling to the
  GHS descent, is rigidity/defocus. **Internal-continue (E.K = solve).**

- **The decomposition↔collection seam (E.K.2↔E.K.3) may be artificial — surface a merge if collection
  is a thin loop.** The 5-vs-4 sizing splits at the decomposition↔collection seam (buying an early
  C-PointDecomp freeze). **If E.K.2's `decompose` and E.K.3's `collect_relations` prove tightly coupled
  (collection is a thin loop with no genuine reusable decomposition seam), the split is artificial and
  E.K.2/E.K.3 should merge** — surfaced as an additive-reshard at the ◆ (or by E.K.2 once the
  decomposition interface is concrete), never a silent merge. **Additive-reshard if the seam proves
  false.**

- **No `F_{p^n}` Semaev lift / no GHS coupling in E.K (defocus / scope clarity — the deferred
  re-shards).** The extension-field Semaev (the asymptotic-win setting) and the descended-Jacobian
  coupling (the binary-curve end-to-end) are **later, separately-sharded sub-tracks**, each flexing
  C-Semaev. A `@build` agent lifting Semaev to `F_{p^n}` or wiring the GHS descent in E.K is defocus.

- **No MATHEMATICS.md chapter in E.K (defocus / scope clarity).** The index-calculus / Gaudry–Diem
  textbook content is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing),
  not at the E.K sub-track ◆. E.K.5 writes at most a PEDAGOGY code-tour delta.

- **Toy `F_p` (`p = 47`, `n = 60`) + small `m` + prime-order subgroup `ℓ` only (scope clarity).** E.K
  uses the `semaev_toy()` fixture and computes index calculus for small arity `m`. The toy sizes are a
  principle-4 boundary — index calculus is mechanism-correct; the *asymptotic win* is a separate
  matter (needs `F_{p^n}`, deferred). Presenting the toy run as the asymptotic speed-up is a
  documentation defect (internal-continue → corrected).

- **Static-frame ROADMAP debt (surface at the E.K ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried + compounded from the E.I, E.H, and E.J ◆.** The ROADMAP Progress subsection is
  stale by **five** completed sub-tracks (E.F, E.G, E.H, E.I, E.J; table shows "Done ~13 (E.A–E.E)");
  the Remaining table lists the now-complete E.F/E.G/E.H/E.I/E.J; and the Remaining table listed
  **E.H before E.I** (dependency-inverted — E.I shipped first, E.H followed). The E.I ◆ digest
  recorded this as owed, the E.H ◆ re-recorded it, the E.J ◆ digest flagged it again (E.J closed
  without writing it into the ROADMAP). The reconciliation owed: update Progress (Track E Done →
  E.A–E.J, ~25), strike E.F/E.G/E.H/E.I/E.J from Remaining, record the E.I-before-E.H correction, and
  strike E.K on completion. **This is a ROADMAP write — outside the `@architect` PLAN-only write scope;
  surfaced here as a capture candidate for the user to action (via `/note` or a ROADMAP edit), not a
  PLAN edit.** Not an implementation concern; does not block E.K.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.K, "*Gaudry–Diem–Joux–Vitse index calculus. 4-5 sessions.
  Predecessors: E.J, G.B, G.E. First session is Opus-tier. The Gröbner-basis step delegates to msolve
  as a dev-only oracle.*"; the C1 contract — `is_smooth` consumed by E.K for "smoothness of points via
  Semaev"; the C3 note — G.B scoring "potentially E.K for factor-base balancing"; the Opus-flagged-
  sessions table — `E.K.1 | Index-calculus strategy`) and this PLAN before any session. **NOTE: the
  ROADMAP Progress / Remaining tables are stale (E.F, E.G, E.H, E.I, E.J done; all five still listed
  as remaining) AND listed E.H before E.I (dependency-inverted); this is a ROADMAP write owed since
  the E.I ◆ — surface it at the E.K ◆, but it is outside `@architect` PLAN-write scope (a capture
  candidate for the user).**
- Read the **templates to mirror**: `rho/src/ssa/mod.rs` + `rho/src/ghs/mod.rs` + `rho/src/semaev/
  mod.rs` (the attack-module idiom — `*Error` enum + toy fixture + module skeleton — E.K's
  `index_calculus` module mirrors this); `rho/src/semaev/{poly,recursion}.rs` (the **frozen** Semaev
  surface — `semaev_poly`, `MultiPoly::partial_eval`/`elim_var_resultant` — read for the
  point-decomposition oracle, NOT to amend); `rho/src/curve/mod.rs` (the **frozen** `Curve`/
  `AffinePoint` — `is_on_curve`/`scalar_mul`/`add_jacobian`/`n`/generator — read for factor-base
  geometry + the group law, NOT to amend); `rho/src/ecdlp/mod.rs` (the **frozen** Pollard-rho
  `solve_*` — read as the E.K.5 cross-check oracle); `gnfs/src/dl/linalg/{mod,blockvec_fl,lanczos_fl,
  wiedemann_fl}.rs` (the **frozen, reused** F_ℓ block-Lanczos/Wiedemann engine — `FlSparseMatrix`,
  `block_lanczos_fl`, `build_fl_matrix` as the *adapter template* E.K.4 mirrors from C-EKRelation, NOT
  from `DLMatrix`); `rho/tests/{ssa_kat,ghs_kat,semaev_kat}.rs` (the attack-KAT + `#[ignore]` oracle
  idioms E.K.2/E.K.5 mirror).
- **Register:** E.K is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New modules `rho/src/index_calculus/{mod,strategy,decompose,collect,linalg,solve}.rs`
  (the index-calculus solver) and new KATs in `rho/tests/index_calculus_kat.rs`.
- **Tier routing:** **E.K.1 is Opus `@architect`** (the ROADMAP's native Opus flag — index-calculus
  strategy + the C-EKRelation contract design; paged as a ◆-start `@plan-juncture` fork *before*
  dispatch to ratify the relation/matrix shape the three downstream sessions consume). **E.K.2–E.K.5
  are Sonnet `@build`.** E.K.5 carries the **◆ `@architect` juncture** (page `@plan-juncture`)
  ratifying the six frozen contracts and confirming E.W-readiness before the sub-track closes.
  juncture-tier (header) is **opus** — set by the native Opus flag on E.K.1 + lever 3 (the
  C-EKRelation cost-of-wrong). The tuning-law read agrees with opus here (unlike E.J, where it pointed
  to sonnet and was overridden): the native Opus session and the expensive relation contract both hold
  the juncture up directly.
- **Invariants to preserve:** **E.K is the index-calculus MECHANISM over `E(F_p)`, NOT the asymptotic
  win** (the win needs the deferred `F_{p^n}` re-shard; the principle-4 annotation records this). **The
  correctness signal is agreement with `rho::ecdlp`** (`log_G(Q)` matches the Pollard-rho solver;
  exactly self-checking, no oracle). **The prime-order subgroup `ℓ` is the linalg precondition** (the
  matrix is over `Z/ℓℤ = F_ℓ`, not the composite `Z/nℤ`). **Reuse the `gnfs::dl::linalg` engine, write
  the E.K adapter** (no duplicate block-Lanczos solver). **The point-decomposition green path is
  native** (the frozen Semaev `MultiPoly` operations; msolve is an `#[ignore]` sidecar — no new live
  oracle). **E.K consumes the frozen `Curve` + `Semaev` + C1 + `gnfs::dl::linalg` surfaces unchanged**
  (adds the `rho::index_calculus` module + the relation→matrix adapter). **No Semaev `F_{p^n}` lift, no
  GHS coupling** (deferred re-shards). **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy `F_p` +
  small `m` + prime-order `ℓ` only.
- **No new edge, no new crate (load-bearing for E.K).** `rho` already depends on `gnfs` (the
  `gnfs::dl::linalg` engine) and `shared-numth` (C1); the index-calculus solver is a new
  `rho::index_calculus` module (the relation→matrix adapter lives with it). `cargo check --workspace`
  stays green with no cycle risk. The one genuine coupling-risk is E.K.4 reading `gnfs::dl::linalg`
  internals; if that surface is ill-fit for the prime-order-subgroup case, the adapter grows — a
  discovery surfaced at the ◆, never a silent patch.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (an
  index-calculus pipeline — strategy/factor-base/relation-contract, then Semaev decomposition, then
  collection, then `Z/ℓℤ` linear algebra reusing the NFS-DL engine, then DLP recovery) is **new to this
  project** (no prior index-calculus / factor-base / relation-collection-over-a-curve machinery exists,
  and E.K.1 is a native Opus session). Per the unproven-shard-pattern guidance, halt at each boundary
  for a human glance until the pattern proves out. The **E.K.1 ◆-start fork is itself a halt** (the
  ROADMAP's pre-scheduled Opus inflection — the C-EKRelation contract is the expensive one). *(Tradeoff
  vs autonomous: `halt-at-boundaries` trades velocity for a per-boundary check on a novel pattern — the
  strategy/relation contract (E.K.1) is the design crux all four downstream sessions consume, and a
  wrong C-EKRelation shape is a pipeline-wide retrofit. If E.K.1 lands cleanly and its factor-base +
  relation-round-trip KATs confirm the contract shape, and E.K.2's decomposition confirms the Semaev
  consumption, fall back to autonomous for E.K.3–E.K.4. The decomposition↔collection seam
  (E.K.2↔E.K.3) is itself a reason to halt at the E.K.2 boundary — that is where a merge-back would be
  surfaced.)*
