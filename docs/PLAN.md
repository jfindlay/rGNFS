<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Phase β, sub-track G.B (Polynomial selection)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above): G.B.1 is not pure substrate — but it freezes **C-PolyPair**
(the NFS polynomial-pair type G.C sieving consumes) and **C-Score** (the Murphy-E scoring contract
C3 that D.A's NFS-DL poly selection, and possibly E.K, reuse cross-track). Lever 3 (a
poly-selection design error propagates into the entire sieve pipeline) and the cross-track reach of
C-Score hold the adjudicator at the Opus default. The strong `cargo test --workspace` inner loop
(lever 5) plus lower correctness-criticality than the number-field core would license `sonnet`, but
the cross-track C-Score contract outweighs it for this one sub-track. Reconsider `sonnet` for G.C/G.D
where the substrate *and* the scoring contract are both frozen.

Last rewrite: G.A ◆ boundary crossed (S0.W backfill landed at `01a385a`). G.A fully complete
(α.5 → G.A.W). This plan opens sub-track G.B over the frozen number-field substrate.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.B) builds **GNFS polynomial selection** — given a target N, produce
the NFS polynomial pair `(f, m)` (an algebraic-side polynomial `f` of degree `d` and a rational-side
`m` with `f(m) ≡ 0 mod N`), score candidates by Murphy-E, and improve them by root sieving. This is
the first session of the **GNFS pipeline proper**: it sits *on* the G.A number-field substrate
(`IntPoly`, `resultant`, `discriminant`) and *produces* the polynomial pair that G.C sieving, G.D
filtering, and the rest of Track G consume.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating poly selection beyond
single-poly Kleinjung + demonstration-fidelity Coppersmith — full Coppersmith multi-poly tuning is a
scale optimization, demonstration fidelity only) and **rigidity** (grinding through a poly-pair
contract that G.C later shows is wrong).

**Scoping discipline (ROADMAP three-way split, applied to G.B).** Algorithmic content complete:
base-m generation, Murphy-E, root sieve, *and* Coppersmith multi-poly at **demonstration fidelity**
(its mathematical content present in code even where its yield improvement doesn't show at toy
scale — ROADMAP scoping principle 2, with the science↔engineering disconnect annotated per principle
4). Engineering optimizations (SIMD root-sieve, parallel candidate search) omitted. **CADO-NFS is a
dev-only correctness oracle** for the RSA-100 published-polynomial cross-check — never on a build
path.

---

## Current state

Phase α + sub-track G.A complete. Workspace crates: `shared/field`, `shared/bigint`, `shared/numth`,
`shared/numfield`, `rho`. `cargo test --workspace` green at `01a385a`.

Substrate G.B consumes (all frozen):
- `shared-numfield`: `IntPoly` / `RatPoly` (arithmetic, `eval`, `degree`, `leading_coeff`,
  `to_rat_poly`), `resultant(f, g) -> BigInt`, `discriminant(f) -> BigInt`, `is_bad_prime`,
  `dedekind_factor[_extended]` (C-NF, C-Res, C-Dedekind).
- `shared-numth`: `is_prime`, `trial_smooth`, `SmoothWitness` (`Uint<4>` — C-numth).

**Substrate gap G.B fills itself (no external dependency):** `IntPoly` exposes no public formal
derivative and no floating-point root-finding. Murphy-E scoring needs real-root approximation of
`f` (to integrate the smoothness yield over the sieve region). G.B.2 provides this internally
(f64 root-finding for scoring only; exact arithmetic stays in `numfield`). This is a scoring-side
numerical tool, not a substrate change — it does not flex any G.A contract.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session. The new `gnfs` crate must be added to the
workspace `members` list at G.B.1 so it is covered by `--workspace`.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default; the substrate-ish
G.B.1 runs to the top of that band). `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection point requiring a
juncture fork + human sign-off before dispatch.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| G.B.1 `@plan` | New `gnfs` crate + base-m polynomial-pair generation | A | Opus | C-NF, C-numth | new `gnfs/` crate (`Cargo.toml`, `src/lib.rs`, `src/polyselect/mod.rs`, `src/polyselect/base_m.rs`), workspace `Cargo.toml` |
| G.B.2 | Murphy-E scoring of a polynomial pair | B | Sonnet | G.B.1 (C-PolyPair), C-Res | `gnfs/src/polyselect/murphy.rs`, `gnfs/src/polyselect/roots.rs`, `gnfs/tests/murphy_kat.rs` |
| G.B.3 | Root sieve (Kleinjung rotation/translation search) | B | Sonnet | G.B.2 (C-Score), C-PolyPair | `gnfs/src/polyselect/root_sieve.rs`, `gnfs/src/polyselect/mod.rs`, `gnfs/tests/root_sieve_kat.rs` |
| G.B.4 | Coppersmith multi-poly (demonstration fidelity) | C | Sonnet | G.B.3, C-Score, C-PolyPair | `gnfs/src/polyselect/coppersmith.rs`, `gnfs/tests/coppersmith_kat.rs` |
| G.B.W ◆ | G.B integrative writeup (polynomial-selection chapter) | I | Sonnet | all G.B | `gnfs/docs/PEDAGOGY.md`, `docs/BENCHMARKS.md` (append) |

**Sequencing notes.** G.B.1 is the single Opus inflection point: it stands up the `gnfs` crate and
freezes **C-PolyPair** (the polynomial-pair type and the `select`/`generate` entry surface). Once
C-PolyPair is frozen, G.B.2 (scoring) and G.B.3 (root sieve) are serial over it — G.B.3 consumes
G.B.2's **C-Score** because the root sieve searches for *better-scoring* polynomials, so it needs the
scorer. G.B.4 (Coppersmith) consumes both. G.B.W is the ◆ boundary.

**Why G.B is 4 sessions + writeup (ROADMAP said 2–3).** The one-line-commit-title corollary splits
the ROADMAP's prose lump. Base-m generation, Murphy-E scoring, root sieving, and Coppersmith
multi-poly are four conceptual units each with a clean commit title; lumping any two yields a
two-clause title (e.g. "root sieve AND Coppersmith"). The split boundaries are contract-sharp:
G.B.1 freezes C-PolyPair before scoring consumes it; G.B.2 freezes C-Score before the root sieve
consumes it. Lever 1 (the `gnfs` crate is new ambient surface) and lever 3 (poly-selection errors
propagate into the whole sieve) push toward the smaller unit; lever 2 (each algorithm is its own
irreducible unit) keeps each whole rather than fracturing it. G.B.4 (Coppersmith) is `Cat C`
(optimization layer over the base selector), demonstration-fidelity only — a candidate for *merge
into G.B.3 or deferral to G.B.W* if it lands under ~150 LOC at demonstration fidelity; decide at
G.B.3 close (see Discoveries & risks).

---

## Session detail

Lower-fidelity rows (G.B.3, G.B.4, G.B.W) are sketched; per the planning philosophy, sessions inside
a sub-track are crisply specified only after the substrate session (G.B.1) lands and freezes
C-PolyPair.

### G.B.1 — `gnfs` crate + base-m polynomial-pair generation (Opus, inflection point)

**Deliverable:**
- New top-level `gnfs` crate (sibling to `rho`; name confirmed at session start). Added to
  workspace `members`. Deps: `shared-numfield` (path), `shared-numth` (path), `num-bigint`,
  `num-traits`; dev-deps `proptest`, `criterion` (matching `rho`'s convention).
- `src/polyselect/base_m.rs`: base-m expansion. Given `N`, degree `d`, and base `m ≈ N^{1/(d+1)}`,
  write `N` in base `m` to get the algebraic-side polynomial `f(x) = Σ aᵢ xⁱ` (the base-m digits,
  so `f(m) = N`), and the rational-side polynomial `g(x) = x − m`. The shared root mod N is `m`.
- The **C-PolyPair** type: a struct carrying `(f: IntPoly, g: IntPoly, m: BigInt, n: BigInt)` with
  the invariant `f(m) ≡ 0 (mod N)` and `g = x − m`, plus a `verify()` predicate that checks it.
- Entry surface `select_base_m(n: &BigInt, degree: usize) -> PolyPair` and a `degree`-heuristic
  helper (`optimal_degree(n) ≈ (3 ln N / ln ln N)^{1/3}`, clamped to 3–6 at toy scale).

**Key design decisions (juncture fork designs C-PolyPair and writes it into Cross-session
contracts):**
1. **PolyPair shape and ownership.** Does `PolyPair` own its polynomials or borrow the
   `NumberField`? G.A's `NumberFieldElement<'a>` borrows its field; poly selection produces `f`
   *before* any `NumberField` is constructed (the field is *defined by* the selected `f`), so
   `PolyPair` should **own** `IntPoly` values and expose `fn number_field(&self) -> NumberField`
   to construct K = ℚ(α) from `f`. Confirm this ownership direction — it is the seam between
   selection (G.B) and the algebraic side (G.C).
2. **Over-specify for G.C.** Per the substrate-over-specifies rule, carry fields G.C sieving will
   need even if G.B.1 doesn't: the skew (rational/algebraic norm balance), the degree `d`, and a
   place for the eventual factor-base bounds. Adding them now is cheaper than amending C-PolyPair
   after G.C consumes it.
3. **`select` vs `generate` surface.** Base-m is one generator; Kleinjung (G.B.3 root sieve) and
   Coppersmith (G.B.4) are others. Design the entry surface so all three feed a common
   `score`-and-rank pipeline rather than three parallel APIs.

**KAT (≥1 required):**
1. **Base-m round-trip:** for `N = 1009·1013` (toy), `d = 3`, the generated `f` satisfies
   `f(m) == N` exactly (base-m is exact by construction) and `g(m) == 0`, and `PolyPair::verify()`
   holds.
2. **RSA-100 base-m (deterministic):** base-m expansion of RSA-100 at the CADO-NFS-published `m`
   and degree `d=5` reproduces the published *base-m* polynomial coefficients. (This is the
   reproducible half of the ROADMAP's CADO cross-check — base-m is deterministic given `(N, m, d)`;
   the *scored/optimized* CADO polynomial is the G.B.3 target, not G.B.1.)
3. `optimal_degree` returns 3–4 for toy N (60–100 bit) and ~5 for RSA-100.

**Subtlety:** base-m `f` is generally **not monic** (leading digit `a_d < m`). G.A's `NumberField`
requires monic `f`. G.B.1 must either (a) carry the non-monic `f` and document that `number_field()`
homogenises/monicises it (the standard NFS `f(x) → a_d^{d−1} f(x/a_d)` substitution), or (b) restrict
toy examples to monic-leading cases. **The juncture fork decides** and writes the resolution into
C-PolyPair — this is the load-bearing interface seam with G.A.

**Deferred:** Murphy-E scoring (G.B.2); root sieve (G.B.3); multi-poly (G.B.4).

### G.B.2 — Murphy-E scoring (Sonnet, on frozen C-PolyPair)

**Deliverable:** Murphy-E score `E(f, g) = Σ ρ(u_alg)·ρ(u_rat)` over sample points in the sieve
region, where `ρ` is the Dickman rho approximation and `u` is the smoothness-exponent ratio. Needs:
- `roots.rs`: real-root approximation of `f` (f64 Newton/Durand-Kerner, scoring-only — exact
  arithmetic stays in `numfield`). Used to compute the size of algebraic norms over the region.
- Dickman-ρ approximation (the standard piecewise/series approximation).
- `murphy.rs`: the integral over the sieve region as a finite sample sum.

Freezes **C-Score** (the `score(&PolyPair) -> f64` contract, C3 cross-track).

**KAT (≥1 required):** Murphy-E is a heuristic float, so the KAT is *ordering + self-consistency*,
not an exact value: (a) a known-better polynomial (smaller coefficients, more real roots) scores
higher than a known-worse one on the same N; (b) score is invariant under the trivial
`x → x` identity and changes monotonically under a coefficient blow-up; (c) **optional** CADO-NFS
oracle cross-check: score within tolerance of CADO's reported Murphy-E for an RSA-100 example
(dev-only, gated behind a feature/ignored test if CADO isn't installed).

**Subtlety (principle-4 annotation):** Murphy-E's *predictive* value (that higher E → more relations)
only manifests at sieve scale; at toy scale it's a ranking heuristic whose payoff is under-exposed.
Annotate the science↔engineering disconnect in the docstring + G.B.W + the eventual Track τ chapter.

### G.B.3 — Root sieve (Sonnet, sketch)

Kleinjung-style rotation (`f → f + (j·x + k)·g`) and translation search over a small `(j, k)` grid,
scoring each candidate with C-Score and keeping the best. Single-poly fidelity (ROADMAP). KAT: the
sieve improves the Murphy-E score of a seed base-m polynomial on a toy N (post-sieve E ≥ pre-sieve
E), and the search is deterministic for a fixed grid+seed.

### G.B.4 — Coppersmith multi-poly (Sonnet, demonstration fidelity, sketch)

Coppersmith's multiple-polynomial method at demonstration fidelity: generate several algebraic-side
polynomials sharing the rational side, present the mathematical construction. KAT: the constructed
polynomials each satisfy the `PolyPair::verify()` invariant; the multi-poly *yield improvement* is
annotated as under-exposed at toy scale (principle 4). **Merge/defer candidate** — see Discoveries.

### G.B.W ◆ — Integrative writeup (Sonnet)

The polynomial-selection chapter (`gnfs/docs/PEDAGOGY.md`): base-m construction, why degree is
chosen by the L-notation balance, what Murphy-E predicts and why its payoff is under-exposed at toy
scale, the root-sieve search, and Coppersmith multi-poly's demonstration-fidelity role. Append a
G.B benchmark row to `docs/BENCHMARKS.md` (scoring/sieve timing). Per pacing guidance, integrative
writeups are under-scheduled — allocate a full session. This is where C-PolyPair / C-Score get their
public prose articulation and the C3 cross-track (D.A, E.K) reuse is surfaced. Its Track τ maths-first
sibling (T.G) pairs at the G.W ◆ boundary, not here — G.B.W is a code-tour sub-chapter feeding the
eventual G.W chapter.

---

## Cross-session contracts

The scaffolding sessions compose through. The juncture fork at G.B.1 writes the resolved
**C-PolyPair** interface into this section before implementation is dispatched.

### C-NF — number-field element arithmetic + norm (compiler + KAT) — *frozen at G.A.1a (bdba6f5)*
**Defined:** G.A.1a. **Consumed by (in G.B):** G.B.1 (`IntPoly`, `NumberField::new`), G.B.2 (norms).
`shared-numfield`: `IntPoly`, `RatPoly`, `NumberField`, `NumberFieldElement`. Stable.

### C-Res — resultant / discriminant (compiler + KAT) — *frozen at G.A.2 (bcd63cd)*
**Defined:** G.A.2 / G.A.3. **Consumed by (in G.B):** G.B.2 (discriminant for the root-property
term of Murphy-E). `shared-numfield`: `resultant(f, g)`, `discriminant(f)`, `is_bad_prime`. Stable.

### C-numth — `shared::numth` smoothness + primality (compiler + KAT) — *frozen at α.2*
**Defined:** α.2. **Consumed by (in G.B):** G.B.3 (primality in root-sieve prime grid). `Uint<4>`;
C1 resolved at G.A.1a (256 bits suffice for toy norms — see ROADMAP). Stable for G.B.

### C-PolyPair — NFS polynomial pair + selection entry surface (compiler + KAT) — *frozen at G.B.1 (2f43f99)*
**Defined:** G.B.1. **Consumed by:** G.B.2, G.B.3, G.B.4, **G.C (sieving), G.D (filtering)**, and
adapted by **D.A (NFS-DL)**. The polynomial-pair type `(f, g, m, n)` with the `f(m) ≡ 0 mod N`
invariant, the `number_field()` constructor (resolving the non-monic seam with C-NF), and the
common select/score/rank entry surface. *Over-specify for G.C* (skew, degree, factor-base-bound
slot).

**Resolved interface (juncture adjudicator, G.B.1 inflection):**

```rust
// gnfs/src/polyselect/mod.rs — re-exports from submodules

use num_bigint::BigInt;
use shared_numfield::{IntPoly, NumberField};

/// NFS polynomial pair: algebraic-side f, rational-side g = x − m, shared root m mod n.
///
/// Invariants (checked by `verify()`):
/// - `f.eval(&m) % &n == 0` (f has m as a root mod n)
/// - `g = x − m` (rational side is always linear)
/// - `f.degree() == Some(degree)` (degree field matches polynomial)
///
/// The algebraic polynomial `f` is stored in its *original* form (generally non-monic for
/// base-m expansion). The `number_field()` method performs the standard homogenisation
/// `f(x) → a_d^{d−1} f(x/a_d)` to produce a monic polynomial suitable for `NumberField::new`.
#[derive(Debug, Clone)]
pub struct PolyPair {
    /// Algebraic-side polynomial f ∈ ℤ[x]. Generally non-monic for base-m; stored as-is.
    pub f: IntPoly,
    /// Rational-side polynomial g = x − m ∈ ℤ[x].
    pub g: IntPoly,
    /// The shared root: f(m) ≡ 0 (mod n) and g(m) = 0.
    pub m: BigInt,
    /// The integer to factor.
    pub n: BigInt,
    /// Polynomial degree (redundant with f.degree() but convenient for pattern matching).
    pub degree: usize,
    /// Skew parameter s: the ratio (sieve-region width)/(sieve-region height) that balances
    /// algebraic and rational norm sizes. `None` until scoring (G.B.2) computes it.
    pub skew: Option<f64>,
    /// Factor-base bounds (rational_bound, algebraic_bound). `None` until sieving (G.C) sets them.
    pub factor_base_bounds: Option<(u64, u64)>,
}

impl PolyPair {
    /// Construct a new polynomial pair. Does NOT verify invariants; call `verify()` after.
    pub fn new(f: IntPoly, g: IntPoly, m: BigInt, n: BigInt) -> Self;

    /// Verify the polynomial-pair invariants. Returns `Ok(())` if valid, `Err(reason)` otherwise.
    ///
    /// Checks:
    /// 1. `f(m) ≡ 0 (mod n)` — the algebraic polynomial has m as a root mod n.
    /// 2. `g = x − m` — the rational polynomial is the expected linear form.
    /// 3. `f.degree() == Some(self.degree)` — degree field is consistent.
    /// 4. `f` is non-zero and has degree ≥ 1.
    pub fn verify(&self) -> Result<(), PolyPairError>;

    /// Construct the number field K = ℚ(α) where α is a root of the *monic* form of f.
    ///
    /// For non-monic f with leading coefficient a_d, this performs the standard homogenisation:
    /// `f_monic(x) = a_d^{d−1} · f(x / a_d)`, which is monic and has roots a_d · α_i where α_i
    /// are the roots of f. The resulting `NumberField` uses this monic polynomial.
    ///
    /// This is the seam between poly selection (which produces non-monic f) and number-field
    /// arithmetic (which requires monic f). Sieving uses the original f for norm computation;
    /// element arithmetic in K uses the monic form via this method.
    pub fn number_field(&self) -> NumberField;

    /// Return the monic form of f via homogenisation, without constructing the full NumberField.
    ///
    /// `f_monic(x) = a_d^{d−1} · f(x / a_d)` where a_d = f.leading_coeff().
    /// If f is already monic, returns a clone of f.
    pub fn monic_f(&self) -> IntPoly;
}

/// Error type for `PolyPair::verify()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolyPairError {
    /// f(m) is not divisible by n.
    RootCheckFailed { f_of_m: BigInt, n: BigInt },
    /// g is not the expected x − m form.
    RationalPolyMismatch { expected_g: IntPoly, actual_g: IntPoly },
    /// Degree field does not match f.degree().
    DegreeMismatch { field_degree: usize, poly_degree: Option<usize> },
    /// f is zero or constant.
    InvalidAlgebraicPoly,
}

// ─── Entry surfaces ──────────────────────────────────────────────────────────

/// Compute the optimal degree d for NFS polynomial selection given n.
///
/// Uses the heuristic `d ≈ (3 ln N / ln ln N)^{1/3}`, clamped to [3, 6] for toy-scale N.
/// At cryptographic scale (RSA-768+), this would return 5–6; at toy scale (60–100 bit),
/// returns 3–4.
pub fn optimal_degree(n: &BigInt) -> usize;

/// Generate a polynomial pair via base-m expansion.
///
/// Given n and degree d, computes `m = floor(n^{1/(d+1)})` and writes n in base m:
/// `n = a_0 + a_1·m + a_2·m² + ... + a_d·m^d`, yielding `f(x) = Σ a_i x^i` with `f(m) = n`.
/// The rational side is `g(x) = x − m`.
///
/// The resulting f is generally non-monic (a_d < m). This is the simplest polynomial
/// generator; Murphy-E scoring (G.B.2) and root sieving (G.B.3) improve upon it.
pub fn select_base_m(n: &BigInt, degree: usize) -> PolyPair;

/// Generate a polynomial pair via base-m expansion with a specified m.
///
/// Used for reproducibility testing (e.g., matching CADO-NFS published polynomials where
/// the exact m is known). The caller is responsible for ensuring m is appropriate for the
/// given n and degree.
pub fn select_base_m_with_m(n: &BigInt, m: &BigInt, degree: usize) -> PolyPair;

// ─── Generator trait (common pipeline for G.B.3, G.B.4) ──────────────────────

/// A polynomial generator produces candidate `PolyPair`s for scoring and ranking.
///
/// All generators (base-m, root sieve, Coppersmith) implement this trait, feeding a common
/// `score`-and-rank pipeline. The generator is responsible for producing candidates; the
/// scorer (C-Score, G.B.2) ranks them.
pub trait PolyGenerator {
    /// Generate polynomial-pair candidates.
    ///
    /// Returns an iterator of `PolyPair` values. The iterator may be finite (base-m produces
    /// exactly one candidate per (n, d) pair) or unbounded (root sieve searches a grid).
    /// Callers should use `.take(limit)` or score-based early termination.
    fn generate(&self) -> impl Iterator<Item = PolyPair>;
}

/// Base-m generator: produces a single polynomial pair via base-m expansion.
pub struct BaseMGenerator {
    pub n: BigInt,
    pub degree: usize,
}

impl PolyGenerator for BaseMGenerator {
    fn generate(&self) -> impl Iterator<Item = PolyPair> {
        std::iter::once(select_base_m(&self.n, self.degree))
    }
}

// ─── Module layout ───────────────────────────────────────────────────────────
//
// gnfs/
// ├── Cargo.toml
// ├── src/
// │   ├── lib.rs              — crate root, re-exports polyselect
// │   └── polyselect/
// │       ├── mod.rs          — PolyPair, PolyGenerator, entry surfaces, re-exports
// │       └── base_m.rs       — select_base_m, optimal_degree, BaseMGenerator
// └── tests/
//     └── base_m_kat.rs       — KAT: toy round-trip, RSA-100 deterministic, optimal_degree
//
// G.B.2 adds: polyselect/murphy.rs, polyselect/roots.rs
// G.B.3 adds: polyselect/root_sieve.rs
// G.B.4 adds: polyselect/coppersmith.rs
```

**Non-monic seam resolution (design decision):**

`PolyPair` stores the *original* non-monic `f` as produced by base-m expansion. The `number_field()`
method performs the standard homogenisation `f(x) → a_d^{d−1} f(x/a_d)` to produce a monic polynomial
for `NumberField::new`. This is the correct direction because:

1. **Sieving uses the original f.** Norm computation in G.C uses the original polynomial coefficients;
   the monic transformation would change the norms.
2. **The transformation is internal to `number_field()`.** Consumers that need `NumberField` element
   arithmetic call `poly_pair.number_field()`; consumers that need the original f (sieving, scoring)
   access `poly_pair.f` directly.
3. **The homogenisation is reversible and well-understood.** The roots of `f_monic` are `a_d · α_i`
   where `α_i` are the roots of `f`. This is standard NFS practice (CADO-NFS, msieve).

**Tradeoff acknowledged:** Carrying both the original f and exposing `monic_f()` / `number_field()`
adds a small API surface. The alternative — storing only the monic form and reverse-transforming for
sieving — would complicate norm computation and obscure the base-m construction's direct relationship
to N. The chosen design keeps the pedagogically clear path (base-m digits → f → sieving) while
providing the monic form when needed.

**Over-specification for G.C:** The `skew` and `factor_base_bounds` fields are `Option` types, set to
`None` at construction and populated by later stages (G.B.2 scoring, G.C sieving). This follows the
substrate-over-specifies rule: adding them now is cheaper than amending C-PolyPair after G.C consumes
it.

**Generator trait rationale:** The `PolyGenerator` trait provides a common interface for all
polynomial generators (base-m, root sieve, Coppersmith). This avoids three parallel APIs and enables
a unified score-and-rank pipeline. The trait uses `impl Iterator<Item = PolyPair>` (RPITIT) for
ergonomic iteration without boxing overhead. G.B.3 and G.B.4 will implement this trait for their
respective generators.

### C-Score — Murphy-E scoring contract (compiler + KAT) — *frozen at G.B.2 (00aa32d)*
**Defined:** G.B.2. **Consumed by:** G.B.3 (root sieve ranks by score), G.B.4, and **cross-track
C3**: D.A (NFS-DL poly selection) and possibly E.K (factor-base balancing in index calculus). Per
ROADMAP C3, this is *not* extracted to `shared::polysel` in advance — premature abstraction is the
greater risk; consolidate after both consumers (G.B, D.A) exist. The `score(&PolyPair) -> f64`
signature is the contract; the float value is heuristic (KAT enforces ordering, not exact value).

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| G.B.1 | `gnfs` crate + base-m generation | done | 2f43f99 | C-PolyPair |
| G.B.2 | Murphy-E scoring | done | 00aa32d | C-Score |
| G.B.3 | Root sieve | done | 3e2ba1b | — |
| G.B.4 | Coppersmith multi-poly | done | c115a1b | — |
| G.B.W | Integrative writeup | pending | — | — |

Contracts frozen before G.B: C-Fp (cf00ed5), C-numth (α.2), C-NF (bdba6f5), C-Ideal (05b27c8),
C-Res (bcd63cd), C-Dedekind (7844773). G.B opens over the frozen G.A substrate.

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume.

### G.B.1 — 2026-06-06
Discovery/flex: Inflection-point design completed; C-PolyPair frozen as designed. Non-monic seam resolved on the G.B side via `number_field()` homogenisation — C-NF not touched.
Affected: C-PolyPair (now frozen at 2f43f99)
Deferred: no
Texture: `PolyGenerator` trait uses RPITIT (Rust 1.75+, workspace on 1.95 — no boxing needed). `base_m_for_degree` uses `floor(N^{1/(d+1)}) + 1` to guarantee N fits in d+1 digits. Cargo.lock included as allowed extra (standard side effect of new crate).

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **Non-monic base-m `f` vs. C-NF's monic requirement.** Base-m `f` is generally non-monic, but
  `NumberField::new` asserts monic. G.B.1's juncture fork must resolve this in C-PolyPair (homogenise
  via `f(x) → a_d^{d−1} f(x/a_d)`, or restrict toy examples). This is an *internal-continue* design
  call at G.B.1, not a halt — but if it forces a change to C-NF (G.A substrate), that is a
  **destructive-HALT** (frozen cross-track contract). Expectation: resolved on the G.B side without
  touching C-NF.

- **C-Score is cross-track C3 — do not narrow it later.** Murphy-E scoring is consumed by D.A and
  possibly E.K. Design `score`'s signature at G.B.2 with both in mind (the ROADMAP "substrate
  over-specifies" rule applied to a prose-flavoured contract). Re-narrowing C-Score after D.A
  consumes it would be a destructive reshard. Per ROADMAP C3, do *not* pre-extract to
  `shared::polysel` — consolidate after D.A exists.

- **G.B.4 Coppersmith conditionality.** Coppersmith multi-poly at demonstration fidelity may land
  under ~150 LOC. **Decide at G.B.3 close** whether it is its own session, merges into G.B.3, or
  folds into the G.B.W writeup as a documented construction. If merged/folded, that is an
  *additive/destructive reshard* of the session list — surface for sign-off.

- **CADO-NFS oracle availability.** The RSA-100 Murphy-E cross-check (G.B.2 KAT 3c) depends on
  CADO-NFS being installed as a dev oracle. If absent, gate that KAT behind an ignored/featured test;
  the deterministic base-m KAT (G.B.1 KAT 2) carries the reproducibility burden without CADO.

- **Murphy-E payoff under-exposed at toy scale (principle 4).** The scorer's *predictive* value is a
  scale phenomenon; at toy scale it ranks but doesn't visibly pay off. Annotate the disconnect in
  code + G.B.W + Track τ. Not a risk to correctness — a pedagogy-honesty obligation.

- **New `gnfs` crate is fresh ambient surface (lever 1).** First GNFS-pipeline crate; G.B.1 sets its
  module layout (`src/polyselect/`) which G.C (`src/sieve/`?) extends. Keep the crate-internal module
  structure open for G.C/G.D rather than over-committing at G.B.1.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase β / Track G section, Cross-track contracts C1/C3, Discoveries log)
  before any G.B session.
- Read `shared/numfield/docs/PEDAGOGY.md` for the pedagogical register (rST docstrings, KATs per
  session, narrative chapter at each ◆ boundary). New `gnfs` work matches it.
- **Register: PEDAGOGY.** This is a reference library — code is teaching material. Match the G.A.W /
  S0.W chapter genre and quality.
- **Tier routing:** G.B.1 is Opus (`@plan-deep` / juncture fork — freezes C-PolyPair + the G.A seam).
  G.B.2–G.B.W are Sonnet (`@build`).
- **Invariants to preserve:** the G.A substrate contracts (C-NF, C-Res, C-numth, C-Ideal,
  C-Dedekind) are frozen — G.B consumes, never amends them. The `rho` crate stays untouched.
- **CADO-NFS / msieve are dev-only oracles**, never on a build path (ROADMAP scoping principle 3).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the `gnfs`-crate
  shard pattern is unproven (first non-`shared`, non-`rho` crate; first GNFS-pipeline sub-track), so
  halt at the G.B.1 inflection and again at the G.B.W ◆ boundary for review.
