<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Phase α.5 + entry into Phase β (G.A — Number-field substrate)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view,
see `docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above): G.A is the highest cost-of-wrong sub-track in β — substrate
design with wide downstream consumption (levers 2/3/4 of the commit-size tuning law all hold the
adjudicator at the Opus default). The strong, fast `cargo test --workspace` inner loop (lever 5)
would license opting down to Sonnet, but it is outweighed here by the substrate's reach and the
silent-wrongness risk of a reference cryptanalysis library. Reconsider `sonnet` for the
algorithm-heavy sub-tracks (G.C, G.D) where the substrate is frozen.

Last rewrite: entry into Phase β (Opus inflection-point review, converting the prose plan to
session-list form). Phase α.1–α.3 complete; α.5 patch session pending; then G.A begins.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.A) builds the **number-field substrate** — ℤ[α] arithmetic, norms,
and ideal representation — that Phase β's GNFS sieving/filtering/linear-algebra sits on, and that
γ (NFS-DL) and parts of δ (E.B, E.D, E.J) reuse. Re-read this intent at every ◆ boundary to catch
defocus (gold-plating the number-field library beyond what GNFS needs) and rigidity (grinding
through a substrate G.A.1a discovers is wrong).

**Scoping discipline (the ROADMAP three-way split, applied to G.A).** Algorithmic content is
included completely; scale-only optimizations at demonstration fidelity (mathematical content
present even where it doesn't pay at toy scale); engineering optimizations (SIMD, custom assembly,
GPU, MPI) omitted, with CADO-NFS as a dev-only correctness oracle. This is the rule that adjudicates
the G.A.1a `BigInt`/`BigRational` decision: `num_bigint` is a correctness-oracle dependency for the
underlying integer arithmetic (like CADO-NFS), while the number-field abstraction and every
algorithm above it stay first-party.

---

## Current state

Phase α delivered three commits:

- `5a8e4a3` α.1 — `shared::field` (generic over `L`), `shared::bigint::batch_invert`, rho migrated.
- `e2714e7` α.2 — `shared::numth`: Miller–Rabin + `SmoothWitness` + trial-division smoothness.
- `09590e3` α.3 — `shared::numth::ecm`: Lenstra ECM stage 1 + stage 2 (Suyama, Montgomery ladder).

`cargo test --workspace`: 34 passed, 0 failed. `cargo bench --workspace --no-run`: clean. Workspace
crates: `shared/field`, `shared/bigint`, `shared/numth`, `rho`.

α.4 reviewed the deliverables against the ROADMAP contracts; findings and deferrals are in
`docs/ROADMAP.md` Discoveries log. Three decisions stand: (1) defer the `Uint<4>` → `Uint<L>`
widening of `shared::numth` to G.A or G.C; (2) add `legendre`/`sqrt` to `Fp` in α.5 (mechanical
Sonnet patch); (3) Phase β (G.A) is the right next move.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler
is the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default; substrate rows
run to the top of that band). `Cat` = category (A substrate / B algorithm / C optimization / I
integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection point requiring a
juncture fork + human sign-off before dispatch.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| α.5 | `Fp` trait completion: Tonelli–Shanks `sqrt` + `legendre` | A | Sonnet | C-Fp | `shared/field/src/lib.rs`, `shared/field/src/naive.rs`, `shared/field/src/monty.rs`, `shared/field/tests/*` |
| G.A.1a `@plan` | Number-field substrate: `ℚ(α)`, element arithmetic, norm via resultant | A | Opus | α.5, C-Fp, C-numth | new `shared/numfield/` crate (`Cargo.toml`, `src/lib.rs`, `src/poly.rs`, `src/element.rs`), workspace `Cargo.toml` |
| G.A.1b | Ideal representation (two-element primary) + ideal norm | A | Sonnet | G.A.1a (C-NF) | `shared/numfield/src/ideal.rs`, `shared/numfield/src/lib.rs`, `shared/numfield/tests/*` |
| G.A.2 | Resultants / subresultant GCD over ℤ[x] | A | Sonnet | G.A.1a (C-NF) | `shared/numfield/src/resultant.rs`, `shared/numfield/src/poly.rs`, tests |
| G.A.3 | Dedekind factorisation of `(p)` in ℤ[α] | A/B | Sonnet | G.A.1b (C-Ideal), G.A.2 (C-Res), C-numth | `shared/numfield/src/dedekind.rs`, tests |
| G.A.4 (cond.) | Discriminant-dividing (bad) prime handling | patch | Sonnet | G.A.3 | `shared/numfield/src/dedekind.rs`, tests |
| G.A.W ◆ | G.A integrative writeup (number-field chapter) | I | Sonnet | all G.A | `shared/numfield/docs/PEDAGOGY.md` (or project PEDAGOGY) |

**Sequencing notes.** α.5 must land before G.A.1a (G.A.1a's resultant-based norm and any
QR-dependent step assume `sqrt`/`legendre` exist). G.A.1a is the single Opus inflection point; once
its element-arithmetic + norm contract (C-NF) is frozen, G.A.1b/G.A.2 are independent Sonnet
sessions over that frozen substrate and may run in either order. G.A.3 consumes both. G.A.4 is
conditional — decide at G.A.3's close (see Discoveries & risks). G.A.W is the ◆ boundary.

**Why G.A.1 was split (was one ~600–1000 LOC row in the prior prose plan).** The current
commit-size default is ~150–400 LOC / 2–4 files, and the "if it can't be a one-line commit title,
it's not one session" corollary applies: number-field element arithmetic + norm is one conceptual
unit; ideal representation is another. The split boundary is contract-sharp (element arithmetic
freezes as C-NF before ideals consume it), so it is a clean boundary, not a fracture of an
irreducible unit. G.A.1a stays at the top of the band (substrate front-loading, 1.5–2× a typical
session per the pacing guidance).

---

## Session detail

Lower-fidelity rows (G.A.2–G.A.W) are sketched; per the planning philosophy, sessions inside a
sub-track are crisply specified only after the substrate session (G.A.1a) lands and freezes C-NF.

### α.5 — `Fp` trait completion (Sonnet)

**Deliverable:**
- `fn legendre(&self, p: &Uint<L>) -> i8` (−1, 0, 1) via Euler's criterion `a^((p−1)/2) mod p`.
  Cheap given existing `pow`. No Jacobi generalisation (trait assumes `p` prime).
- `fn sqrt(&self, p: &Uint<L>) -> Option<Self>` via Tonelli–Shanks. `p ≡ 3 (mod 4)` shortcut
  `a^((p+1)/4)`; general loop for `p ≡ 1 (mod 4)` (needed by pairing-friendly primes in E.B).
- Default-method bodies on the trait (in terms of `pow`/`mul`/`square`) so existing `Fp` impls are
  not forced to add methods; verify on `FpNaive<L>` and `FpMonty<L>`.

**KAT (≥1 required):** legendre values vs. reference for hand-picked (a,p) pairs spanning QR/QNR;
`sqrt(a,p)^2 == a` round-trip for residues and `None` for non-residues, over primes
{5, 7, 13, 17, 1009, 1048517}. **Property test:** `legendre(a,p) ∈ {−1,0,1}` and multiplicativity
`legendre(a·b) == legendre(a)·legendre(b)`.

**Subtlety:** Tonelli–Shanks needs a QNR — find by trial `n = 2,3,5,…` until `legendre(n,p) == −1`
(within ~10 trials for all project primes).

**Deferred:** sqrt over `F_{p^k}`, k>1 (E.D/E.B territory).

### G.A.1a — Number-field substrate (Opus, inflection point)

**Deliverable:**
- New `shared/numfield` crate (name confirmed at session start). `K = ℚ(α)`, α a root of monic
  irreducible `f(x) ∈ ℤ[x]`.
- `IntPoly` / `RatPoly` — polynomials over ℤ / ℚ. Decision at session start (both may be needed).
- `NumberFieldElement` — degree-`<deg(f)` polynomial in α with rational coefficients, reduced
  mod `f`. Eager canonicalisation in `Mul` (reduce mod `f`); equality needs no re-reduction.
- Norm via the resultant `Res_x(f(x), g(x))` (algebraic; no embeddings). Mention the numerical
  embedding version in PEDAGOGY.

**Key design decisions (juncture fork designs C-NF and writes it into Cross-session contracts):**
1. **Coefficient type.** `i64` (overflows — rejected), `crypto_bigint::Uint<L>` (aligns with `Fp`,
   awkward for rationals), or `num_bigint::BigInt` + `num_rational::BigRational` (heap, slower,
   correctness-first). Recommendation: `BigInt`/`BigRational`; document the deviation from
   `crypto-bigint` as a correctness-oracle dependency in PEDAGOGY (parallel to CADO-NFS's role).
2. **Norm bit-width feedback to C1.** If G.A.1a surfaces NFS norms bounded by 256 bits for the
   target factor base, leave `shared::numth` at `Uint<4>`; else schedule the widening (see
   Discoveries & risks). This is the discovery that resolves the α.4 C1 deferral.

**KAT (≥1 required):** ℤ[α] arithmetic for `f = x²−2` (`(1+α)(1−α) = −1`); norm for `β = 1+α`,
`Norm(β) = −1`; a non-trivial cubic from a published source (Crandall–Pomerance §6 or LMFDB).

**Subtlety:** the norm has resultant and numerical-embedding implementations; implement the
resultant version. (G.A.2 provides the standalone resultant; G.A.1a may inline a minimal version
and let G.A.2 generalise it — the juncture fork decides whether to stub.)

**Deferred:** polynomial selection (G.B); sieving (G.C); algebraic-side smoothness (G.C).

### G.A.1b — Ideal representation (Sonnet, on frozen C-NF)

Ideal as a ℤ-module: two-element representation `(p, α − r)` primary; HNF-basis added only if a
concrete need surfaces (simpler-first). Ideal norm; KAT `Norm(IJ) = Norm(I)·Norm(J)`.

### G.A.2 — Resultants / subresultant GCD (Sonnet, sketch)

Polynomial GCD over ℤ[x] (or ℚ[x]) via subresultants; resultant as a polynomial in the inputs'
coefficients. Consumed by G.A.1a's norm (if stubbed) and by G.B's poly-selection scores.

### G.A.3 — Dedekind factorisation (Sonnet, sketch)

Given prime `p ∤ disc(f)`, decompose `(p)` in ℤ[α] via the factorisation of `f mod p` (Dedekind's
theorem, concrete form). Entry point to NFS's algebraic-side machinery. KAT against a worked
example.

### G.A.4 — Bad-prime handling (Sonnet, conditional, sketch)

Primes dividing `disc(f)` where Dedekind doesn't apply directly. Skippable at toy scale if the
target polynomial's discriminant is small relative to the factor base. **Decide at G.A.3 close**
whether needed here or foldable into G.B.

### G.A.W ◆ — Integrative writeup (Sonnet)

The number-field chapter: ℤ[α] arithmetic and its ideal-theoretic substrate. Per the pacing
guidance, integrative writeups are consistently under-scheduled — allocate a full session. This is
where C-NF / C-Ideal / C-Res get their public prose articulation and the cross-track implications
(γ, E.B/E.D/E.J reuse) are surfaced.

---

## Cross-session contracts

The scaffolding sessions compose through. The juncture fork at G.A.1a writes the resolved **C-NF**
interface into this section before implementation is dispatched.

### C-Fp — `Fp<L>` field trait (compiler + KAT)
**Defined:** α.1, completed α.5. **Consumed by:** G.A.1a, all field-arithmetic downstream.
`shared/field/src/lib.rs::Fp`. After α.5 includes `legendre` and `sqrt` as default methods.

### C-numth — `shared::numth` smoothness + primality (compiler + KAT)
**Defined:** α.2. **Consumed by:** G.A.1a (norm bit-width), G.A.3, and (per ROADMAP C1) G.C, D.A,
E.K. Currently hardcoded to `Uint<4>`; widening deferred (see Discoveries & risks / ROADMAP C1).

### C-NF — number-field element arithmetic + norm (compiler + KAT) — *frozen at G.A.1a*

**Defined:** G.A.1a. **Consumed by:** G.A.1b, G.A.2, G.A.3, G.B, G.C, D.A, E.D.

#### Coefficient type

`num_bigint::BigInt` for integer polynomials; `num_rational::BigRational` for rational polynomials
and number-field elements. This is a correctness-oracle dependency (like CADO-NFS): the underlying
integer/rational arithmetic is third-party; the number-field abstraction and all algorithms above it
are first-party. Document in PEDAGOGY.

#### Polynomial types

```rust
/// Polynomial over ℤ with BigInt coefficients.
/// Coefficients stored least-significant first: coeffs[i] is the coefficient of x^i.
/// Invariant: trailing zeros are trimmed (degree is coeffs.len() - 1, or -∞ for zero poly).
pub struct IntPoly {
    pub coeffs: Vec<BigInt>,
}

impl IntPoly {
    pub fn zero() -> Self;
    pub fn one() -> Self;
    pub fn from_coeffs(coeffs: Vec<BigInt>) -> Self;  // trims trailing zeros
    pub fn degree(&self) -> Option<usize>;            // None for zero poly
    pub fn leading_coeff(&self) -> Option<&BigInt>;
    pub fn eval(&self, x: &BigInt) -> BigInt;
    pub fn is_monic(&self) -> bool;

    // Arithmetic (returns new poly; does not mutate)
    pub fn add(&self, rhs: &Self) -> Self;
    pub fn sub(&self, rhs: &Self) -> Self;
    pub fn neg(&self) -> Self;
    pub fn mul(&self, rhs: &Self) -> Self;
    pub fn scale(&self, c: &BigInt) -> Self;          // multiply all coeffs by c

    // Conversion
    pub fn to_rat_poly(&self) -> RatPoly;             // embed ℤ[x] ↪ ℚ[x]
}

/// Polynomial over ℚ with BigRational coefficients.
/// Same storage convention as IntPoly.
pub struct RatPoly {
    pub coeffs: Vec<BigRational>,
}

impl RatPoly {
    pub fn zero() -> Self;
    pub fn one() -> Self;
    pub fn from_coeffs(coeffs: Vec<BigRational>) -> Self;
    pub fn degree(&self) -> Option<usize>;
    pub fn leading_coeff(&self) -> Option<&BigRational>;
    pub fn eval(&self, x: &BigRational) -> BigRational;
    pub fn is_monic(&self) -> bool;

    // Arithmetic
    pub fn add(&self, rhs: &Self) -> Self;
    pub fn sub(&self, rhs: &Self) -> Self;
    pub fn neg(&self) -> Self;
    pub fn mul(&self, rhs: &Self) -> Self;
    pub fn scale(&self, c: &BigRational) -> Self;

    // Division with remainder: self = q * divisor + r, deg(r) < deg(divisor).
    // Panics if divisor is zero.
    pub fn div_rem(&self, divisor: &Self) -> (Self, Self);
    pub fn rem(&self, divisor: &Self) -> Self;        // convenience: div_rem(...).1
}
```

#### Number field and element types

```rust
/// A number field K = ℚ(α) where α is a root of the monic irreducible polynomial f ∈ ℤ[x].
pub struct NumberField {
    /// The defining polynomial f(x). Must be monic and irreducible over ℚ.
    /// Degree d = f.degree().unwrap() is the field extension degree [K : ℚ].
    pub f: IntPoly,
}

impl NumberField {
    /// Construct a number field from a monic irreducible polynomial.
    /// Panics if f is not monic or has degree < 1.
    /// Does NOT verify irreducibility (caller's responsibility; KAT coverage).
    pub fn new(f: IntPoly) -> Self;

    /// Extension degree [K : ℚ] = deg(f).
    pub fn degree(&self) -> usize;

    /// Construct the element α (the primitive element, i.e., the polynomial x mod f).
    pub fn alpha(&self) -> NumberFieldElement;

    /// Construct a rational element (embedding ℚ ↪ K).
    pub fn from_rational(&self, r: BigRational) -> NumberFieldElement;

    /// Construct an integer element (embedding ℤ ↪ K).
    pub fn from_int(&self, n: BigInt) -> NumberFieldElement;
}

/// An element of a number field K = ℚ(α), represented as a polynomial in α
/// of degree < [K : ℚ] with rational coefficients.
///
/// Invariant: self.poly.degree() < self.field.degree() (enforced by Mul).
pub struct NumberFieldElement<'a> {
    /// Reference to the ambient number field.
    pub field: &'a NumberField,
    /// The element as a polynomial in α. Degree < field.degree().
    pub poly: RatPoly,
}

impl<'a> NumberFieldElement<'a> {
    // Arithmetic: all operations reduce mod f eagerly.
    pub fn add(&self, rhs: &Self) -> Self;
    pub fn sub(&self, rhs: &Self) -> Self;
    pub fn neg(&self) -> Self;
    pub fn mul(&self, rhs: &Self) -> Self;            // reduces mod f
    pub fn square(&self) -> Self;                     // reduces mod f
    pub fn pow(&self, exp: u64) -> Self;              // square-and-multiply

    /// Multiplicative inverse via extended Euclidean algorithm in ℚ[x] mod f.
    /// Panics if self is zero.
    pub fn inv(&self) -> Self;

    /// Field norm N_{K/ℚ}(self) = Res_x(f, self.poly) for monic f.
    /// Returns a BigRational (always an integer for algebraic integers).
    pub fn norm(&self) -> BigRational;

    /// Trace Tr_{K/ℚ}(self) = sum of conjugates.
    /// (Optional for G.A.1a; may defer to G.A.W if not needed by G.A.1b.)
    pub fn trace(&self) -> BigRational;

    // Predicates
    pub fn is_zero(&self) -> bool;
    pub fn is_one(&self) -> bool;
    pub fn is_rational(&self) -> bool;                // degree 0

    // Equality: compares poly coefficients directly (no re-reduction needed).
}

impl PartialEq for NumberFieldElement<'_> { ... }
impl Eq for NumberFieldElement<'_> { ... }
```

#### Mul canonicalisation contract

`NumberFieldElement::mul` (and `square`, `pow`) **eagerly reduces mod f** after polynomial
multiplication. This ensures:
1. The invariant `poly.degree() < field.degree()` is always satisfied.
2. Equality comparison is coefficient-wise: two elements are equal iff their `poly` fields are
   equal. No re-reduction is needed before comparison.

#### Norm implementation

G.A.1a inlines a minimal resultant via the Sylvester matrix determinant:

```rust
impl NumberFieldElement<'_> {
    /// Compute Res_x(f, g) via the Sylvester matrix determinant.
    /// For monic f, this equals N_{K/ℚ}(g(α)).
    fn sylvester_resultant(f: &IntPoly, g: &RatPoly) -> BigRational;

    pub fn norm(&self) -> BigRational {
        // For monic f: Norm(β) = Res_x(f, β) where β = self.poly.
        // The resultant of f (degree d) and g (degree < d) is the determinant
        // of the (2d-1) × (2d-1) Sylvester matrix... but since deg(g) < deg(f),
        // the matrix simplifies to d × d.
        Self::sylvester_resultant(&self.field.f, &self.poly)
    }
}
```

This is a complete implementation for the norm use case. G.A.2 provides a general-purpose
`resultant(f: &IntPoly, g: &IntPoly) -> BigInt` via the subresultant PRS algorithm; the two
coexist (G.A.1a's inline version is not replaced).

#### C1 resolution (norm bit-width)

For a toy 60–80 bit semiprime N with factor base B ≤ 10^4:
- Polynomial degree d = 3–5, coefficients C ~ N^{1/d} ~ 2^{20–27}.
- Sieve bound M ~ 10^4 = 2^{13}.
- Norm bound: |Norm| ≤ (d+1)^{(d+1)/2} · (M · C)^d ≈ 2^{120–150}.

**256 bits (Uint<4>) suffices.** Leave `shared::numth` at `Uint<4>`; no widening needed. This
resolves the α.4 C1 deferral. If a future sub-track targets larger N, revisit — but the trait was
designed at α.2 to support widening.

#### Downstream consumption summary

| Consumer | Consumes from C-NF |
|----------|-------------------|
| G.A.1b (Ideals) | `NumberField`, `NumberFieldElement`, `IntPoly`; element arithmetic, `norm()` |
| G.A.2 (Resultants) | `IntPoly`, `RatPoly`; polynomial arithmetic (no element types) |
| G.A.3 (Dedekind) | `NumberField`, `IntPoly`; polynomial reduction mod p (new in G.A.3) |
| G.B (Poly selection) | `IntPoly`; resultant (from G.A.2), discriminant |
| G.C (Sieving) | `NumberFieldElement::norm()`, ideal representation (from C-Ideal) |
| D.A (NFS-DL) | Same as G.C |
| E.D (Pairings) | `NumberField`, `NumberFieldElement`; extension field arithmetic |

#### KAT requirements (G.A.1a must pass)

1. **ℤ[α] arithmetic for f = x² − 2**: `(1 + α)(1 − α) = 1 − α² = 1 − 2 = −1`.
2. **Norm for β = 1 + α in ℚ(√2)**: `Norm(1 + α) = (1 + √2)(1 − √2) = 1 − 2 = −1`.
3. **Cubic from Crandall–Pomerance or LMFDB**: e.g., f = x³ − x − 1 (discriminant −23), verify
   `Norm(α) = 1` and `Norm(α − 1) = 1`. (Norm(x−c) = (−1)^d · f(c); d=3, f(1)=−1, so (−1)³·(−1)=1.)

### C-Ideal — ideal representation + ideal norm (compiler + KAT) — *frozen at G.A.1b*
**Defined:** G.A.1b. **Consumed by:** G.A.3, G.C, D.A. Two-element representation primary.

### C-Res — resultant / subresultant GCD (compiler + KAT) — *frozen at G.A.2*
**Defined:** G.A.2. **Consumed by:** G.A.1a (norm), G.B, D.A.

### C-Dedekind — Dedekind factorisation of `(p)` (KAT) — *frozen at G.A.3*
**Defined:** G.A.3. **Consumed by:** G.C (factor-base construction).

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| α.5 | `Fp` completion | done | cf00ed5 | C-Fp (+legendre/sqrt) |
| G.A.1a | Number-field substrate | done | bdba6f5 | C-NF |
| G.A.1b | Ideal representation | done | 05b27c8 | C-Ideal |
| G.A.2 | Resultants / subresultant GCD | done | bcd63cd | C-Res |
| G.A.3 | Dedekind factorisation | pending | — | C-Dedekind |
| G.A.4 | Bad-prime handling | pending | — | (extends C-Dedekind) |
| G.A.W | Integrative writeup | pending | — | — |

Contracts frozen so far: C-Fp (α.1, completed α.5 cf00ed5), C-numth (α.2).

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume. Empty at chain start.

### G.A.1a — 2026-06-06
Discovery/flex: C-NF KAT spec had a sign error: `Norm(α−1) = −1` should be `+1` for f = x³−x−1 (the (−1)^d factor was missing from the example). Implementation is mathematically correct; KAT test passes with +1.
Affected: C-NF (documentary typo in KAT spec, corrected in PLAN; semantic contract intact)
Deferred: no
Texture: Norm formula is Norm(x−c) = (−1)^d · f(c) for monic f of degree d. Downstream sessions consuming norm() are unaffected — they consume the function, not the example value.

---

## Discoveries & risks

Carried forward from the prior plan's risk register and the ROADMAP Discoveries log, in the form
`/run-plan` reads for discovery adjudication.

- **C1 / `Uint<4>` widening (deferred at α.4).** `shared::numth`'s whole surface operates on
  `Uint<4>`; `SmoothWitness::factors` is `(u64, u32)`. Sufficient for toy-scale 256-bit norms. The
  α.4 decision was to revisit once G.A gives concrete norm bit-widths. **G.A.1a is the resolving
  session:** if NFS norms for the target factor base fit in 256 bits, leave `numth` at `Uint<4>`;
  else run a small `shared::numth` widening as its own session (additive-reshard) before G.C —
  never paper over with `BigInt`. The trait was specified at α.2 to support widening.

  **Resolve C1 against all three of its consumers, not just G.A's.** Per ROADMAP, C1's smoothness
  predicate + `SmoothWitness` shape was designed at S0.2 with three consumers in mind: integer
  smoothness with a prime factor base (G.C sieving, D.A relation collection) *and* smoothness of
  curve points via Semaev polynomials (E.K index calculus — semantically different, structurally
  similar). When G.A.1a widens or re-parameterises the witness type, design the change so it still
  serves E.K, even though E.K lands much later — this is the "substrate over-specifies" rule, and
  re-narrowing the witness later would invalidate a frozen cross-track contract (a destructive-HALT,
  not an additive reshard).

- **`BigInt`/`BigRational` vs. the "everything from scratch" thesis.** *Mitigation:* document the
  dependency in PEDAGOGY as a correctness oracle (like CADO-NFS) — the number-field abstraction and
  all algorithms above it are first-party.

- **Ideal representation (two-element vs HNF) may be wrong for G.C.** *Mitigation:* two-element
  first (G.A.1b); add HNF only if G.C surfaces a concrete need.

- **G.A.4 conditionality.** Decide at G.A.3 close whether bad-prime handling is needed in G.A or
  folds into G.B. If folded, this is a destructive-reshard of the session list — HALT for sign-off.

- **`rho::curve` not lifted to `shared::curve`** (α.3, deliberate — ECM Montgomery vs rho
  Weierstrass). Revisit at E.B.1, not in G.A.

---

## Notes for executors

- Read `docs/ROADMAP.md` Discoveries log before any β session.
- Read `rho/docs/PEDAGOGY.md` for the pedagogical register (rST docstrings, KATs per phase,
  narrative chapter at each ◆ boundary). New shared-crate work matches it.
- G.A.1a is Opus (`@plan-deep` / juncture fork). α.5 and G.A.1b–G.A.W are Sonnet (`@build`).
- The `rho` crate and its tests stay untouched by β work except via `Cargo.toml` dependency
  additions and field/curve shim files.
- This plan is `/run-plan`-executable: `/run-plan docs/PLAN.md halt-at-boundaries` for the first
  run of this sub-track (the shard pattern is unproven for a Rust workspace).
