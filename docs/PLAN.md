<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Phase β, sub-track G.C (Sieving)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above): G.C.1 is a substrate session that freezes **C-Relation** (the
relation / exponent-vector format) and **C-FactorBase** (the two-sided factor-base construction).
C-Relation is consumed by G.D (filtering) and G.E (linear algebra) and **adapted cross-track by
D.A** (NFS-DL relation collection) — the same cross-track reach that held G.B at the Opus default
for C-Score. Lever 3 (a relation-format design error propagates through the entire back half of
Track G — filtering, linear algebra, square root) holds the adjudicator up. The frozen number-field
substrate (C-NF, C-PolyPair, C-numth — G.C only *consumes* it) plus the strong `cargo test
--workspace` inner loop and the CADO-NFS relation-count oracle (lever 5) would license `sonnet`, but
the new cross-track C-Relation contract outweighs that relief for this one substrate session.
Reconsider `sonnet` for G.D/G.E where C-Relation is already frozen.

Last rewrite: G.B ◆ boundary crossed (G.B.W landed at `7fa9ab9`). G.B fully complete (G.B.1 →
G.B.W); C-PolyPair and C-Score frozen. This plan opens sub-track G.C — the sieving stage of the
GNFS pipeline — over the frozen polynomial-selection and number-field substrate.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.C) builds **GNFS sieving** — given a polynomial pair `(f, g)` from
G.B, collect **relations**: coprime integer pairs `(a, b)` for which *both* the rational norm
`|g-side| = |a − b·m|` and the algebraic norm `|F(a, b)| = |b^d · f(a/b)|` (the homogenised form of
`f`) are smooth over their respective factor bases. Each smooth pair yields one row of the
relation matrix that G.D filters and G.E solves. This is the second stage of the GNFS pipeline
proper: it sits *on* the G.B polynomial pair (C-PolyPair) and the G.A number-field substrate (norms
via `resultant`/eval) and the S0.2 smoothness predicate (C-numth), and it *produces* the relation
corpus that G.D (filtering), G.E (linear algebra), and ultimately G.F (square root) consume.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating the sieve beyond
line-sieving baseline + special-q + demonstration-fidelity lattice sieving — SIMD/bucket-sieve
engineering is out of scope, ROADMAP principle 3) and **rigidity** (grinding through a relation
format that G.D later shows is wrong, rather than surfacing it at the boundary).

**Scoping discipline (ROADMAP three-way split, applied to G.C).** Algorithmic content complete:
two-sided factor bases, line-sieving baseline over the `(a, b)` rectangle, special-q strategy, *and*
lattice sieving at **demonstration fidelity** (the special-q lattice construction present in code
even where its yield advantage over line sieving doesn't show at toy scale — ROADMAP principle 2,
science↔engineering disconnect annotated per principle 4). Engineering optimizations (bucket
sieving, SIMD, cache-blocked sieve arrays) omitted. **CADO-NFS is a dev-only correctness oracle**
for the relation-count cross-check — never on a build path.

---

## Current state

Phase α + sub-tracks G.A, G.B complete. Workspace crates: `shared/field`, `shared/bigint`,
`shared/numth`, `shared/numfield`, `rho`, `gnfs`. `cargo test --workspace` green at `7fa9ab9`.

Substrate G.C consumes (all frozen):
- `gnfs::polyselect`: `PolyPair` (`f`, `g`, `m`, `n`, `degree`, `skew`, `factor_base_bounds`),
  `select_base_m`, `score` (C-PolyPair, C-Score). The `factor_base_bounds: Option<(u64, u64)>`
  slot was over-specified at G.B.1 *for G.C* — G.C.1 populates it.
- `shared-numfield`: `IntPoly` (`eval`, `degree`, `leading_coeff`, `coeffs`), `resultant(f, g)`,
  `discriminant` (C-NF, C-Res). Algebraic norm `F(a,b) = b^d·f(a/b)` is computed from `f.coeffs`
  directly (homogenisation) or via `resultant(f, a − b·x)`.
- `shared-numth`: `trial_smooth(n: &Uint<4>, factor_base: &[u64]) -> SmoothWitness`,
  `factor_base_up_to`, `SmoothWitness` (`factors: Vec<(u64,u32)>`, `cofactor: Uint<4>`) (C-numth).
  C-numth's module doc already names G.C as consumer #1 and states no witness extension is needed.

**Substrate gap G.C fills itself (no external dependency):** smoothness in `shared-numth` operates
on `Uint<4>` (unsigned, 256-bit). Norms `a − b·m` and `F(a, b)` are **signed** `BigInt` and may
exceed what the caller wants to feed `trial_smooth` directly. G.C.1 provides the norm→`Uint<4>`
bridge (absolute value + range check; toy-scale norms fit 256 bits per the C1 resolution in
ROADMAP). This is a sieve-side adapter, not a substrate change — it does not flex C-numth or C-NF.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session. G.C adds modules to the *existing* `gnfs`
crate (already in `members`), so no workspace `Cargo.toml` change is required.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default; the substrate-ish
G.C.1 runs to the top of that band). `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection point requiring a
juncture fork + human sign-off before dispatch.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| G.C.1 `@plan` | Sieve substrate: two-sided factor bases + `Relation` type + norm bridge | A | Opus | C-PolyPair, C-NF, C-numth | new `gnfs/src/sieve/mod.rs`, `gnfs/src/sieve/factor_base.rs`, `gnfs/src/sieve/norms.rs`, `gnfs/src/lib.rs` (add `pub mod sieve`), `gnfs/tests/factor_base_kat.rs` |
| G.C.2 | Line-sieving baseline over the `(a, b)` rectangle | B | Sonnet | G.C.1 (C-Relation, C-FactorBase), C-numth, C-PolyPair | `gnfs/src/sieve/line.rs`, `gnfs/src/sieve/mod.rs`, `gnfs/tests/line_sieve_kat.rs` |
| G.C.3 | Special-q strategy (per-q relation-yield multiplier) | C | Sonnet | G.C.2, C-Relation | `gnfs/src/sieve/special_q.rs`, `gnfs/tests/special_q_kat.rs` |
| G.C.4 | Lattice sieving (demonstration fidelity) | C | Sonnet | G.C.3, C-Relation | `gnfs/src/sieve/lattice.rs`, `gnfs/tests/lattice_kat.rs` |
| G.C.W ◆ | G.C integrative writeup (sieving chapter) | I | Sonnet | all G.C | `gnfs/docs/PEDAGOGY.md` (append), `docs/BENCHMARKS.md` (append) |

**Sequencing notes.** G.C.1 is the single Opus inflection point: it stands up the `sieve/` module
(sibling to `polyselect/`, the layout G.B.1 kept open) and freezes **C-Relation** (the relation /
exponent-vector type) and **C-FactorBase** (two-sided factor-base construction + the rational/
algebraic norm computation). Once both freeze, G.C.2 (line-sieving baseline) collects relations
over them; G.C.3 (special-q) and G.C.4 (lattice) are serial optimization layers over the baseline,
each reading the previous stage and producing a higher-yield variant **without altering the
baseline** (Category-C rule: the baseline stays available for benchmarking). G.C.W is the ◆
boundary.

**Why G.C is 4 sessions + writeup (ROADMAP said 4–5, listed 5 items).** The one-line-commit-title
corollary restructures the ROADMAP's prose list. The ROADMAP's "rational sieving" and "algebraic
sieving" are **merged** into G.C.2: a relation requires norm-smoothness on *both* sides
simultaneously, so a rational-only sieve has no KAT-able deliverable (it collects no complete
relations) — splitting them fractures one irreducible unit (lever 2 floor). The ROADMAP's "line
sieving baseline" *is* that merged two-sided sieve, so it is not a separate item. The genuinely
separate units, each with a clean commit title, are: **substrate** (factor bases + relation type —
the ROADMAP under-named this; it is the contract-sharp prerequisite), **line-sieving baseline**,
**special-q**, **lattice sieving**. Lever 1 (the `sieve/` module is fresh ambient surface) and lever
3 (a relation-format error propagates into G.D/G.E/G.F) push toward the smaller unit; lever 2 keeps
each algorithm whole. **G.C.4 (lattice sieving) is `Cat C`** (optimization layer over special-q),
demonstration-fidelity only — a candidate for *merge into G.C.3 or fold into G.C.W* if it lands
under ~150 LOC at demonstration fidelity; decide at G.C.3 close (see Discoveries & risks). This
mirrors the G.B.4 (Coppersmith) conditionality, which the G.B chain proved out.

---

## Session detail

Lower-fidelity rows (G.C.3, G.C.4, G.C.W) are sketched; per the planning philosophy, sessions inside
a sub-track are crisply specified only after the substrate session (G.C.1) lands and freezes
C-Relation / C-FactorBase.

### G.C.1 — Sieve substrate: factor bases + `Relation` type + norm bridge (Opus, inflection point)

**Deliverable:**
- New `gnfs/src/sieve/` module (sibling to `polyselect/`). `gnfs/src/lib.rs` adds `pub mod sieve`
  and re-exports the sieve entry surface.
- `sieve/factor_base.rs`: **C-FactorBase**. The two-sided factor base:
  - *Rational* factor base: primes `p ≤ B_rat` (via `factor_base_up_to`).
  - *Algebraic* factor base: prime ideals of ℤ[α] of degree 1, represented as pairs `(p, r)` where
    `f(r) ≡ 0 (mod p)` and `p ≤ B_alg` — the roots of `f mod p` for each prime `p`. A prime `p`
    contributes as many `(p, r)` entries as `f` has roots mod `p`.
  - The factor-base bounds populate `PolyPair::factor_base_bounds` (the slot G.B.1 over-specified).
- `sieve/norms.rs`: rational norm `N_rat(a, b) = a − b·m` and algebraic norm
  `N_alg(a, b) = b^d · f(a/b) = Σ a_i · a^i · b^{d−i}` (homogenisation of `f`), both `BigInt`; plus
  the signed-`BigInt` → `Uint<4>` bridge (abs + 256-bit range check) feeding `trial_smooth`.
- `sieve/mod.rs`: **C-Relation**. The `Relation` type carrying `(a, b)` and both smoothness
  witnesses (rational + algebraic exponent vectors over their factor bases), with a `verify()`
  predicate (both norms reconstruct from their witnesses; both witnesses are fully smooth; gcd(a,b)=1).

**Key design decisions (juncture fork designs C-Relation + C-FactorBase and writes them into
Cross-session contracts):**
1. **`Relation` exponent-vector shape.** The relation must carry enough for G.E to build a matrix
   row over **GF(2)** (factoring) *and* be adaptable by D.A to **GF(ℓ)** (NFS-DL). Decide: store full
   `(prime/ideal, exponent)` pairs (two `SmoothWitness`-like vectors, rational + algebraic) rather
   than pre-reduced GF(2) parities — G.E reduces; D.A needs the integer exponents. This is the
   load-bearing cross-track call (mirrors the C-Score over-specification).
2. **Algebraic factor base = degree-1 prime ideals.** At toy scale the degree-1 ideal `(p, r)`
   representation (root `r` of `f mod p`) suffices; over-specify a slot for the quadratic-character
   / free-relation columns G.E will need (the obstruction columns), even if G.C.2 doesn't fill them.
3. **Sign and units.** Relations carry a sign bit (the rational norm `a − b·m` can be negative) —
   G.E needs the `−1` column. Decide whether the sign lives in the `Relation` or is derived; freeze
   it so G.D/G.E don't disagree.
4. **Bad-prime / index handling on the algebraic side.** Primes `p | disc(f)` (ROADMAP principle-4
   over-exposed phenomenon) need the G.A.3 `dedekind_factor_extended` path, not naive root-finding.
   Decide at G.C.1 whether the toy factor bases include bad primes or exclude them with annotation.

**KAT (≥1 required):**
1. **Factor-base construction:** for a toy `f` and `B_alg = 30`, the algebraic factor base lists
   exactly the `(p, r)` pairs with `f(r) ≡ 0 (mod p)`, cross-checked against brute-force root
   enumeration mod each `p ≤ 30`.
2. **Norm reconstruction:** for a known `(a, b)`, `N_rat` and `N_alg` match hand-computed values;
   `Relation::verify()` holds for a hand-constructed smooth relation and fails when an exponent is
   perturbed.
3. **Norm bridge range:** a toy-scale norm fits `Uint<4>`; the bridge rejects (or flags) an
   out-of-range norm rather than silently truncating.

**Subtlety:** the algebraic norm is `b^d · f(a/b)`, the *homogeneous* form — computing it as
`f.eval(a/b)` (rational) then clearing denominators is error-prone; compute it directly as
`Σ a_i a^i b^{d−i}` from `f.coeffs`. The relationship to `resultant(f, a − b·x)` is the pedagogical
hook for G.C.W. **The juncture fork decides** the representation and writes it into C-FactorBase.

**Deferred:** the actual sieve loop (G.C.2); special-q (G.C.3); lattice (G.C.4).

### G.C.2 — Line-sieving baseline (Sonnet, on frozen C-Relation / C-FactorBase)

**Deliverable:** the line sieve over the `(a, b)` rectangle `|a| ≤ A`, `1 ≤ b ≤ B`. For each `b`,
sieve the rational side (mark `a` where `p | (a − b·m)`) and the algebraic side (mark `a` where
`p | N_alg(a, b)`, i.e. `a ≡ r·b (mod p)` for each `(p, r)`), accumulate approximate `log p`
contributions, and trial-divide the survivors with `trial_smooth` to confirm full smoothness on
both sides. Each confirmed coprime pair becomes a `Relation`. Returns a `Vec<Relation>`.

Freezes nothing new (consumes C-Relation, C-FactorBase, C-numth).

**KAT (≥1 required):** (a) on a toy `N` with small bounds, the sieve produces ≥ k relations and
every returned `Relation::verify()` holds (both norms fully smooth, gcd(a,b)=1); (b) the relation
count is **deterministic** for a fixed `(N, A, B, B_rat, B_alg)`; (c) **optional CADO-NFS oracle:**
relation count within tolerance of CADO at matched parameters (dev-only, gated/ignored if CADO
absent — ROADMAP G.C KAT).

**Subtlety (principle-4 annotation):** the `log p` sieve-then-confirm pattern (mark with cheap
approximate logs, confirm survivors with exact trial division) is the engineering heart of NFS, but
at toy scale the "sieve" is barely faster than direct trial division of every `(a,b)` — the
asymptotic win is under-exposed. Annotate in the docstring + G.C.W + Track τ.

### G.C.3 — Special-q strategy (Sonnet, sketch)

For each special prime `q` in a chosen range, restrict sieving to `(a, b)` with `q | N_alg(a, b)`
(i.e. `a ≡ r_q·b (mod q)`), so every survivor already has a known large algebraic factor — the
relation-yield multiplier. Over a frozen C-Relation. KAT: relations collected per-`q` all satisfy
`verify()` and carry `q` in the algebraic exponent vector; the per-`q` yield exceeds a naive sieve
of the same area; deterministic for a fixed `q`-range + seed.

### G.C.4 — Lattice sieving (Sonnet, demonstration fidelity, sketch)

For a special-`q`, sieve over the **lattice** `L_q = { (a, b) : a ≡ r_q·b (mod q) }` in reduced
basis (the `(V1, V2)` short vectors from a 2D lattice reduction) rather than the full rectangle —
the construction that makes special-q efficient at scale. Demonstration fidelity (ROADMAP). KAT:
the lattice-enumerated `(a, b)` pairs all lie in `L_q` and reproduce a subset of the G.C.3 relations
for the same `q`; the yield-per-area improvement over line sieving is annotated as under-exposed at
toy scale (principle 4). **Merge/defer candidate** — see Discoveries.

### G.C.W ◆ — Integrative writeup (Sonnet)

The sieving chapter (`gnfs/docs/PEDAGOGY.md`, append): the relation as the unit of NFS data, why
smoothness on *both* sides is required, the rational/algebraic factor-base construction (degree-1
prime ideals), the line-sieve `log p` mark-then-confirm pattern and why its asymptotic win is
under-exposed at toy scale, the special-q yield multiplier, and lattice sieving's demonstration-
fidelity role. Append a G.C benchmark row to `docs/BENCHMARKS.md` (relations/second, sieve area vs
yield). Per pacing guidance, integrative writeups are under-scheduled — allocate a full session.
This is where C-Relation / C-FactorBase get their public prose articulation and the G.D/G.E/D.A
downstream reuse is surfaced. Its Track τ maths-first sibling (T.G) pairs at the **G.W** ◆ boundary,
not here — G.C.W is a code-tour sub-chapter feeding the eventual G.W chapter.

---

## Cross-session contracts

The scaffolding sessions compose through. The juncture fork at G.C.1 writes the resolved
**C-Relation** and **C-FactorBase** interfaces into this section before implementation is dispatched.

### C-PolyPair — NFS polynomial pair + selection entry surface (compiler + KAT) — *frozen at G.B.1 (2f43f99)*
**Defined:** G.B.1. **Consumed by (in G.C):** G.C.1 (`f`, `g`, `m`, `n`, `degree`; populates
`factor_base_bounds`), G.C.2 (norms from `f.coeffs`). Stable. G.C does **not** amend it — the
`factor_base_bounds: Option<(u64,u64)>` slot was over-specified at G.B.1 precisely for G.C to fill.

### C-NF — number-field element arithmetic + norm (compiler + KAT) — *frozen at G.A.1a (bdba6f5)*
**Defined:** G.A.1a / G.A.2. **Consumed by (in G.C):** G.C.1 (`IntPoly::eval`, `coeffs`, `degree`;
`resultant` as the norm cross-check), G.C.1 bad-prime path (`dedekind_factor_extended`,
`is_bad_prime` for `p | disc(f)`). Stable.

### C-numth — smoothness + primality (compiler + KAT) — *frozen at α.2*
**Defined:** α.2. **Consumed by (in G.C):** G.C.1 (`factor_base_up_to`), G.C.2 (`trial_smooth`,
`SmoothWitness` for both sides). `Uint<4>`; C1 resolved (256 bits suffice for toy norms — ROADMAP).
The `smooth` module doc already names G.C as consumer #1 and confirms no witness extension is needed
for G.C. Stable for G.C.

### C-FactorBase — two-sided factor base + norm computation (compiler + KAT) — *to be frozen at G.C.1*
**Defined:** G.C.1. **Consumed by:** G.C.2, G.C.3, G.C.4, **G.D (filtering reads the factor-base
column indexing)**. The rational factor base (primes ≤ `B_rat`), the algebraic factor base
(degree-1 prime ideals `(p, r)` with `f(r) ≡ 0 mod p`, ≤ `B_alg`), the rational/algebraic norm
functions, and the signed-`BigInt` → `Uint<4>` bridge. *Over-specify for G.E:* carry a slot for the
quadratic-character / free-relation (obstruction) columns even though G.C.2 doesn't fill them.
**The juncture fork writes the resolved interface here at G.C.1 execution time.**

### C-Relation — relation / exponent-vector format (compiler + KAT) — *to be frozen at G.C.1*
**Defined:** G.C.1. **Consumed by:** G.C.2, G.C.3, G.C.4, **G.D (filtering), G.E (linear algebra)**,
and **adapted cross-track by D.A (NFS-DL relation collection)**. The `Relation` type: the coprime
pair `(a, b)`, the rational and algebraic exponent vectors (full `(prime/ideal, exponent)` pairs —
*not* pre-reduced GF(2) parities, so D.A can read integer exponents over GF(ℓ)), the sign/unit
column, and a `verify()` predicate. *Over-specify for D.A:* store integer exponents and the sign so
the GF(2)→GF(ℓ) adaptation is a read, not a reshard. Per ROADMAP, do **not** pre-extract a shared
relation crate — consolidate after D.A exists. **The juncture fork writes the resolved interface
here at G.C.1 execution time.**

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| G.C.1 | Sieve substrate: factor bases + Relation type | pending | — | C-Relation, C-FactorBase |
| G.C.2 | Line-sieving baseline | pending | — | — |
| G.C.3 | Special-q strategy | pending | — | — |
| G.C.4 | Lattice sieving | pending | — | — |
| G.C.W | Integrative writeup | pending | — | — |

Contracts frozen before G.C: C-Fp (cf00ed5), C-numth (α.2), C-NF (bdba6f5), C-Ideal (05b27c8),
C-Res (bcd63cd), C-Dedekind (7844773), C-PolyPair (2f43f99), C-Score (00aa32d). G.C opens over the
frozen G.A substrate and G.B polynomial-selection layer.

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume.

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **C-Relation is cross-track (G.D, G.E, D.A) — over-specify, do not narrow later.** Store full
  integer exponent vectors + sign, not pre-reduced GF(2) parities, so D.A's GF(ℓ) NFS-DL adaptation
  is a read rather than a reshard. Re-narrowing C-Relation after G.E or D.A consumes it would be a
  **destructive reshard**. Per ROADMAP, do *not* pre-extract a shared relation crate — consolidate
  after D.A exists. Designing the exponent shape now without D.A is the over-specify rule applied to
  a compiler+test contract.

- **Bad primes on the algebraic side (ROADMAP principle-4 over-exposed phenomenon).** Primes
  `p | disc(f)` need the G.A.3 `dedekind_factor_extended` path, not naive `f mod p` root-finding;
  at toy scale with a hand-picked `f` they are *unavoidable and prominent* (e.g. disc small ⇒ p=2
  bad). G.C.1 decides whether toy factor bases include them (with the principle-4 annotation) or
  exclude them. This is an *internal-continue* design call at G.C.1 — but if it forces a change to
  C-NF/C-Dedekind (frozen G.A substrate), that is a **destructive-HALT**. Expectation: resolved on
  the G.C side via the existing `dedekind_factor_extended` surface without touching C-NF.

- **Norm magnitude vs `Uint<4>` (C1 / lever 2).** Algebraic norms `b^d·f(a/b)` grow with degree and
  sieve-region size; the C1 resolution says 256 bits suffice for *toy* norms but G.C.1's bridge must
  **range-check, not silently truncate**. If a chosen toy `N`/region overflows `Uint<4>`, that is an
  *internal-continue* (shrink the region or widen via the documented `Uint<L>` path — ROADMAP C1),
  not a halt, unless it forces widening the frozen C-numth surface (then **HALT**).

- **G.C.4 lattice-sieving conditionality.** Lattice sieving at demonstration fidelity may land under
  ~150 LOC. **Decide at G.C.3 close** whether it is its own session, merges into G.C.3, or folds
  into the G.C.W writeup as a documented construction. If merged/folded, that is an
  *additive/destructive reshard* of the session list — surface for sign-off. (Mirrors the G.B.4
  Coppersmith decision, which resolved to "own session".)

- **CADO-NFS oracle availability.** The relation-count cross-check (G.C.2 KAT c) depends on CADO-NFS
  being installed as a dev oracle. If absent, gate that KAT behind an ignored/featured test; the
  deterministic relation-count KAT (G.C.2 KAT a/b) carries the reproducibility burden without CADO.

- **Sieve asymptotic win under-exposed at toy scale (principle 4).** The `log p` mark-then-confirm
  pattern and lattice sieving's yield advantage are *scale* phenomena; at toy scale the sieve barely
  beats direct trial division. Annotate the disconnect in code + G.C.W + Track τ. Not a correctness
  risk — a pedagogy-honesty obligation.

- **`sieve/` module is fresh ambient surface (lever 1).** First non-`polyselect` module in `gnfs`;
  G.C.1 sets its layout (`src/sieve/`), which G.D (`src/filter/`?) and G.E (`src/linalg/`?) will
  sibling. Keep the crate-internal module structure open rather than over-committing at G.C.1.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase β / Track G section: G.C entry, Cross-track contracts C1/C2/C3,
  Discoveries log including the G.B base-m and Dickman-ρ findings) before any G.C session.
- Read `gnfs/docs/PEDAGOGY.md` (the G.B polynomial-selection chapter) and
  `shared/numfield/docs/PEDAGOGY.md` for the pedagogical register (rST docstrings, KATs per session,
  narrative chapter at each ◆ boundary). New `gnfs::sieve` work matches it.
- **Register: PEDAGOGY.** This is a reference library — code is teaching material. Match the G.B.W /
  G.A.W chapter genre and quality.
- **Tier routing:** G.C.1 is Opus (`@plan-deep` / juncture fork — freezes C-Relation + C-FactorBase,
  the cross-track relation seam). G.C.2–G.C.W are Sonnet (`@build`).
- **Invariants to preserve:** the G.A substrate contracts (C-NF, C-Res, C-numth, C-Ideal,
  C-Dedekind) and the G.B contracts (C-PolyPair, C-Score) are frozen — G.C consumes, never amends
  them. The `rho` crate and `gnfs::polyselect` stay untouched (G.C *adds* `gnfs::sieve`).
- **CADO-NFS / msieve are dev-only oracles**, never on a build path (ROADMAP scoping principle 3).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the `sieve/`-module
  shard pattern is unproven for this crate (first sieve module; first cross-track relation contract
  since C-Score), so halt at the G.C.1 inflection and again at the G.C.W ◆ boundary for review.
