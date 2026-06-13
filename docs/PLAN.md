<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.F — GF(2^m) field arithmetic: the characteristic-2 substrate)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) on the E.F.1
sibling-trait + word-layout substrate, against a strong lever 5 that would otherwise license an
opt-down.** E.F is a substrate sub-track, and the **GF(2^m) field-trait design (E.F.1) is a
Category-A representation choice consumed by *three* downstream sub-tracks** — E.G (binary curves +
Koblitz automorphism), E.H (GHS/Weil descent), E.I (GF(2^m) hyperelliptic Jacobian). The trait shape
(sibling to `Fp<L>`, not an impl), the polynomial-basis word layout (degree-m element in ⌈m/64⌉ `u64`
limbs), and the irreducible-polynomial representation bound every consumer; getting any wrong is the
most expensive retrofit in the sub-track. Lever 5 is strong and fast (GF(2^m) arithmetic is
*exactly* checkable: field axioms — associativity, distributivity, inverse round-trip — plus the
Frobenius fixed-field law `a^(2^m) = a`, a naive-vs-optimized cross-check mirroring the existing
`FpNaive`/`FpMonty` pattern, and an optional PARI `ffinit`/`fflog` `#[ignore]` sidecar) and *would*
license `juncture-tier: sonnet` in isolation; the user judged the substrate design-error cost (lever
3, three downstream consumers) high enough to hold the ◆ juncture at **Opus**, mirroring the E.E and
E.D calls. *(E.F.1 additionally runs as an **Opus `@architect` session** with an **inline
`@architect` juncture** — the trait freeze is ratified before E.F.2 consumes it, not deferred to the
◆; see the session list. E.F.2–E.F.4 are Sonnet `@build`.)*

Last rewrite: **E.E ◆ boundary crossed; the p-adic branch of Track E is complete** (E.E.1 `4dc5eaf`,
E.E.2 ◆ `8a217f4`; C-AnomalousLift / C-SSA frozen; the Smart–Satoh–Araki polynomial-time attack on
anomalous curves ships, consuming the E.D p-adic substrate). The E.E ◆ Action-frame digest
(2026-06-12) recorded the CM-fixture wrinkle (the `y²=x³+5/F_7` fixture has CM by `Z[ζ₃]`, `v_p(T)=2`,
fixed by an O(1) CM correction and a precision bump) as a permanent annotated note flagged for T.E
pedagogy — **not a blocker for E.W or T.E**, which consume the frozen C-SSA signature. Per the
sequencing order (ROADMAP "Remaining projected sessions": E.D → E.E → **E.F**), the next un-started
sub-track is **E.F — GF(2^m) field arithmetic**, the characteristic-2 field substrate the entire
binary-curve cluster (E.G, E.H, E.I) and the index-calculus track (E.J, E.K) stand on.

**Static-frame debt outstanding (surfaced at this boundary — does NOT block E.F sharding).** The
ROADMAP **Progress** subsection (line ~168) and the **Remaining projected sessions** table were
reconciled only through the **E.C ◆** (2026-06-10): they show Track E "done ~8 (E.A, E.B, E.C)" and
still list E.D and E.E as *remaining*, though both are complete (E.D `0a98148`…`95d03b7`; E.E
`4dc5eaf`/`8a217f4`) and the Discoveries log *is* reconciled through E.D ◆. The Progress table's
Track-E actuals and the Remaining table's E.D/E.E rows are stale by two completed sub-tracks. This is
a **roadmap-frame reconciliation owed at the next sub-track ◆** (the E.F ◆, paired with this PLAN's
close), not an E.F implementation concern — flagged here so it is not lost. *(Surfaced as a
CAPTURE-CANDIDATE below.)*

The substrate survey (forked `@explore`, 2026-06-13) established the shape and confirmed every
planning assumption:

1. **E.F is fully greenfield — no GF(2^m) / characteristic-2 / polynomial-basis / normal-basis /
   Koblitz field-arithmetic code exists anywhere** (zero source hits). The only `GF(2)` occurrences
   are `gnfs/src/linalg/` block-Lanczos/Wiedemann **bit-vector linear algebra over the 2-element
   field** — semantically distinct from field arithmetic over the *extension* GF(2^m); no overlap.

2. **GF(2^m) CANNOT implement the existing `Fp<L>` trait — it needs a sibling trait.** The survey
   confirmed four structural incompatibilities in `shared/field/src/lib.rs`: (a) `inv` is Fermat
   `a^(p-2)` (meaningless in char 2 — inversion is extended-Euclidean over GF(2)[x] or Itoh–Tsujii);
   (b) `legendre`/`sqrt` assume an odd prime `p`; (c) `neg` computes `p - self` but in char 2 `−a = a`
   (negation is the identity); (d) the canonical representation is a `Uint<L>` *integer mod p*, but a
   GF(2^m) element is a *polynomial over GF(2)*, and the per-call `p: &Uint<L>` is semantically a
   prime modulus, not an irreducible polynomial. **E.F.1 defines the new `F2m` sibling trait** —
   mirroring the per-call-modulus design pattern (passing the irreducible as `poly: &…`) but a
   *separate* trait. This is the Category-A design crux.

3. **Placement: a new `shared/gf2m` crate** (user-confirmed), mirroring the `shared/padic` precedent
   exactly (a categorically-new arithmetic substrate → a new sibling crate, depending only on
   `crypto-bigint` for the `Uint<L>` word layout). `shared/padic` itself does **not** depend on
   `shared/field`; `shared/gf2m` likewise stands alone. Adding GF(2^m) to `shared/field` would
   conflate two field categories in a crate whose docstring is "Prime-field arithmetic over GF(p)"
   and risk its **four** dependents (`shared/bigint`, `shared/numth`, `rho`, `gnfs`) on any API
   change. **The `rho → shared-gf2m` edge is E.G's, not E.F's** — E.F ships the standalone crate;
   the binary-curve consumer adds the workspace edge later (mirroring how `rho → shared-padic` was
   E.E's edge, not E.D's).

4. **The downstream curve consumer is NOT generic over a field trait at the struct level.**
   `rho::curve::Curve` is concretely `Uint<4>`-based (prime `p`, short-Weierstrass `y²=x³+ax+b`);
   `is_on_curve`/`scalar_mul`/`add_jacobian` are `<F: Fp<4>>` at the *method* level only. Binary
   curves use a **different group law** (`y²+xy=x³+ax²+b` for non-supersingular, López–Dahab
   projective coordinates, not Jacobian). E.G will need a new `BinaryCurve` type — **this is E.G's
   problem, not E.F's** — but it imposes a *requirement on E.F.1's trait*: the `F2m` surface must
   over-specify (per the substrate rule) to express what a binary curve will need (trace, half-trace
   / quadratic-solve `x²+x=c` for point decompression, the Frobenius `a → a²` as a first-class op).
   The GLV/Koblitz omission the ROADMAP names (rho "explicitly omits" the order-6 Koblitz
   automorphism) lives only in `ROADMAP.md:344`; `glv.rs` implements the **order-3** prime-field
   endomorphism, not the binary Koblitz automorphism — that is **E.G's** to add.

5. **The dual-implementation pedagogical pattern is established and is the model.** `shared/field`
   ships `FpNaive` (schoolbook baseline, "correct, slow, easy to audit") **and** `FpMonty`
   (optimized), cross-checked in `sqrt_legendre_kat.rs` (`naive_monty_agree_p17`). E.F mirrors this:
   a **naive comb multiplier baseline** in E.F.1 (the auditable substrate), an **optimized
   Karatsuba/López–Dahab multiplier** in E.F.4, cross-checked. No `std::arch`/`pclmulqdq`/
   `target_feature` exists anywhere in the tree — software multiplication is the toy-scale baseline;
   a carryless-multiply (`pclmulqdq`) path is a *demonstration-fidelity* option for E.F.4, gated
   behind `#[cfg(target_feature)]` if introduced at all (principle-3-adjacent: it is engineering, so
   it ships at most at demonstration fidelity, never as the green path).

The work splits at **four representation/contract-sharp seams**, **4 sessions** (matching the
ROADMAP's 3–4 ceiling-biased estimate), at the boundaries between the trait-and-substrate, the
inversion algorithm, the second (normal) basis, and the optimized multiplier:

1. **E.F.1 — GF(2^m) polynomial-basis substrate + `F2m` carryless-mul field trait (Opus `@architect`,
   Cat A, inline `@architect`).** The new `shared/gf2m` crate; the `F2m` sibling trait; the
   polynomial-basis representation (degree-m element in ⌈m/64⌉ `u64` limbs); carryless multiply +
   modular reduction by the irreducible polynomial; add (XOR), the Frobenius squaring `a → a²`; a
   **naive comb multiplier** baseline. **Freezes C-F2m** — the trait + polynomial-basis surface the
   whole binary cluster builds on. *Inline `@architect` juncture: the trait freeze is ratified
   before E.F.2 consumes it.*

2. **E.F.2 — GF(2^m) inversion (extended Euclidean over GF(2)[x] + Itoh–Tsujii) + division (Sonnet,
   Cat A).** Inversion in char 2 — algorithmically distinct from multiplication: extended Euclidean
   over GF(2)[x] as the auditable baseline, Itoh–Tsujii (`a^(2^m−2)` via the Frobenius tower) as the
   field-specific alternative; division as `mul`-by-`inv`. **Amends C-F2m additively** (the `inv`/
   `div` methods the trait declared but E.F.1 may have left `unimplemented!`-stubbed or
   `todo!`-guarded — confirm at E.F.1 freeze).

3. **E.F.3 — GF(2^m) normal-basis representation + polynomial↔normal conversion (Sonnet, Cat A).**
   The normal-basis `F2m` implementor (a *second* representation of the same field), where squaring
   is a cyclic shift (the pedagogical payoff); the change-of-basis isomorphism between
   polynomial-basis and normal-basis representations, cross-checked for round-trip fidelity.
   **Consumes C-F2m** (implements the frozen trait in a second representation).

4. **E.F.4 ◆ — GF(2^m) Karatsuba/López–Dahab multiplier + naive-vs-optimized cross-check + sub-track
   close (Sonnet, Cat C, `@architect`).** The optimized polynomial-basis multiplier (Karatsuba
   subquadratic split and/or López–Dahab comb), cross-checked against the E.F.1 naive baseline; the
   optional `pclmulqdq` demonstration path (gated, never green-path); the sub-track-close KAT suite
   (field axioms, Frobenius fixed-field, basis-conversion round-trip, naive↔optimized agreement).
   **Freezes C-F2mOpt** (the optimized-multiplier equivalence contract). Crosses the **E.F ◆
   boundary** — the GF(2^m) substrate ships, ready for E.G to instantiate binary curves.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing binary *curves* — that is
E.G; or a general `GF(p^m)` extension-field tower — E.F is characteristic-2 only, the `FpExt`
prime-extension work already exists in `rho/src/pairing/fpext.rs` for the odd-characteristic pairing
case; or writing the GF(2^m) / binary-curve *textbook chapter* in MATHEMATICS.md — that is **T.E**,
paired with E.W at the Track-E ◆, per the per-track-chapter pairing rule; E.F writes at most a
PEDAGOGY code-tour delta) and **rigidity** (forcing the `F2m` trait into the `Fp<L>` mould the survey
proved incompatible; or re-deriving `Uint<L>` limb primitives rather than consuming `crypto-bigint`'s
`as_words`/`from_words`/`mul_wide`; or shipping a `pclmulqdq` path on the green compute path rather
than as a gated demonstration sidecar).

**Scoping discipline.** E.F builds the GF(2^m) substrate at **demonstration fidelity** (principle 1 —
algorithmic content complete: polynomial-basis arithmetic, both inversion algorithms, normal-basis
representation, the optimized multiplier all implemented head-on) and **toy field sizes** (small `m`
— e.g. GF(2^4), GF(2^8), GF(2^163)-shaped toy parameters — enough to exercise every code path and
exhibit the algebra, not crypto-scale). It **amends no frozen contract** (C-Padic / C-Hensel /
C-PadicLog / C-SSA / the rho curve surface are all untouched — E.F is field-arithmetic-only, curve-
free). It introduces **no new live oracle** on the green path (the field axioms + Frobenius law +
naive↔optimized cross-check are exactly self-checking; an optional PARI `ffinit`/`fflog` `#[ignore]`
sidecar is the established dev-only pattern). The **engineering-vs-mathematics disconnect** (ROADMAP
principle 4) is explicit: the `pclmulqdq` carryless-multiply intrinsic is *engineering* (principle 3
— omitted from the green path, present at most at demonstration fidelity, gated); the toy field
sizes are a principle-4 boundary (the algorithms are crypto-scale-correct; only the *parameters* are
toy), annotated, never presented as crypto-scale.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.F): "*E.F — GF(2^m) field arithmetic. 3-4 sessions. No structural
predecessor. Polynomial-basis and normal-basis representations; this is a categorically new
field-arithmetic implementation. **First session is Opus-tier** — substrate decision (basis choice,
word layout, multiplication algorithm: comb, Karatsuba, López–Dahab) has downstream consequences for
E.G, E.H, E.I.*" E.F is the **characteristic-2 field substrate** the entire binary-curve cluster
stands on: where the existing `shared/field` `Fp<L>` substrate serves the odd-characteristic
prime-field attacks (E.A–E.E), E.F builds the *categorically different* GF(2^m) arithmetic that
binary curves, GHS/Weil descent, and the GF(2^m) hyperelliptic Jacobian all consume.

The "no structural predecessor" the ROADMAP names is literal: E.F depends on no prior Track-E
sub-track. It is a fresh substrate, sequenced here only because the binary half of Track E (E.G–E.K)
follows the p-adic half (E.D–E.E) in the recommended order. It does, however, sit *beside* the
established `shared/field` prime-field substrate as a deliberate sibling — and the central design
tension is exactly that **GF(2^m) is a field, like GF(p), but a different *kind* of field**: the
survey confirmed the existing `Fp<L>` trait cannot host it (Fermat inversion, odd-prime
`legendre`/`sqrt`, integer-mod-p representation all break in characteristic 2). E.F therefore builds
a *parallel* substrate, sharing the design *idiom* (a const-generic trait, per-call modulus, a naive
baseline cross-checked against an optimized implementation) but not the trait itself.

The substrate decomposes into four conceptual units, each a session:

1. **The polynomial-basis substrate + the `F2m` trait (E.F.1).** A GF(2^m) element is a polynomial
   over GF(2) of degree < m, stored as ⌈m/64⌉ `u64` limbs (a bit-vector of coefficients). Addition
   is XOR; multiplication is **carryless** polynomial multiplication followed by **modular reduction**
   by the irreducible polynomial defining the field; squaring `a → a²` is the Frobenius endomorphism
   (a bit-spreading operation). The trait surface must **over-specify** for the downstream curve
   consumer: the Frobenius as a first-class op, the trace `Tr(a) = Σ a^(2^i)`, and the quadratic-solve
   `x² + x = c` (half-trace) for binary-curve point operations — carried now even at the cost of
   declaring methods E.F.4/E.G fill in, because adding them later is the expensive retrofit. The
   **naive comb multiplier** is the auditable baseline (the `FpNaive` analogue). **(E.F.1.)**

2. **Inversion + division (E.F.2).** Characteristic-2 inversion is its own algorithmic unit, not a
   variation on multiplication: **extended Euclidean over GF(2)[x]** (the auditable baseline,
   computing `a⁻¹` via the polynomial GCD with the irreducible) and **Itoh–Tsujii** (`a⁻¹ = a^(2^m−2)`
   computed through the Frobenius tower with a logarithmic number of multiplications — the
   field-specific optimization, exact-checkable against extended Euclidean). Division is
   `mul(a, inv(b))`. **(E.F.2.)**

3. **Normal-basis representation + conversion (E.F.3).** A *second* representation of the same field:
   in a normal basis `{β, β², β⁴, …, β^(2^(m−1))}`, **squaring is a single cyclic shift** — the
   pedagogical payoff of normal bases, and the reason they exist. E.F.3 ships the normal-basis `F2m`
   implementor and the **change-of-basis isomorphism** (polynomial ↔ normal), cross-checked so that
   a field operation gives the same element in either representation. **(E.F.3.)**

4. **The optimized multiplier + cross-check + close (E.F.4).** The optimized polynomial-basis
   multiplier — **Karatsuba** (subquadratic split of the carryless multiply) and/or **López–Dahab**
   (the comb method with windowing) — cross-checked against the E.F.1 naive baseline (the
   `FpNaive`/`FpMonty` agreement pattern). An optional **`pclmulqdq`** carryless-multiply path is a
   gated demonstration-fidelity sidecar (principle 3 — engineering, never the green path). The
   sub-track-close KAT suite exercises the field axioms, the Frobenius fixed-field law, the
   basis-conversion round-trip, and naive↔optimized agreement. **(E.F.4 ◆.)**

E.F is **curve-free** (the binary-curve group law, point decompression, and the Koblitz automorphism
are all E.G), **prime-extension-free** (the odd-characteristic `FpExt` tower already exists in
`rho/src/pairing/fpext.rs` for pairings; E.F is characteristic 2 only), and **chapter-free** (the
GF(2^m) textbook content is T.E, paired with E.W at the Track-E ◆). Re-read this intent at the ◆
boundary to catch defocus (binary curves, a `GF(p^m)` tower, the MATHEMATICS chapter) and rigidity
(forcing `F2m` into `Fp<L>`; re-deriving limb primitives; a green-path `pclmulqdq`).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey confirmed zero hits); raw `cargo` is the only
CI surface (unchanged from E.D/E.E). Oracle KATs are `#[ignore]`-gated only — the exact form is
`#[ignore = "PARI not installed; run manually when available"]`, used identically in
`rho/tests/ssa_kat.rs:338`, `rho/tests/mov_kat.rs:253`, and `shared/padic/tests/log_kat.rs:203`.
`/run-plan` re-discovers at preflight. E.F **adds a new workspace crate (`shared/gf2m`) but no new
*edge*** (the `rho → shared-gf2m` edge is E.G's; E.F.1 only adds `shared/gf2m` to the workspace
`members` list in the root `Cargo.toml`), so the gate is a **correctness + workspace-integrity gate**:

- Each session's KATs (`shared/gf2m/tests/gf2m_kat.rs`, plus inline unit tests in the
  `naive`/`normal`/`opt` modules mirroring `shared/field`'s inline-test idiom) are the primary
  correctness signal — fast and *exactly* decisive (lever 5: the GF(2^m) field axioms, the Frobenius
  fixed-field law `a^(2^m) = a`, the inversion round-trip `a·a⁻¹ = 1`, the basis-conversion
  round-trip, and the naive↔optimized agreement are all self-checking with no external oracle).
- `cargo check --workspace` must confirm the new `shared/gf2m` crate resolves cleanly with **no
  dependency cycle** (`shared/gf2m` depends only on `crypto-bigint`, mirroring the leaf-crate shape;
  it depends on no other workspace member, so it introduces no cycle).
- **The existing rho / gnfs / shared KATs must stay green** after the new crate lands — E.F touches
  no existing crate (it adds `shared/gf2m` and the one-line `members` entry), so the no-regression
  invariant is structurally easy to hold; `cargo test --workspace` is the guard.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.F.1 `@architect` | GF(2^m) polynomial-basis substrate + `F2m` carryless-mul field trait | A | Opus | `crypto-bigint` `Uint<L>` (`as_words`/`from_words`/`mul_wide`, read); `shared/field` `Fp<L>` (read — the *idiom* to mirror, not the trait to impl); `shared/padic/Cargo.toml` (read — the leaf-crate precedent) | `Cargo.toml` (root: add `shared/gf2m` to `members`), `shared/gf2m/Cargo.toml` (new: leaf crate, dep `crypto-bigint`), `shared/gf2m/src/lib.rs` (new: the `F2m` trait + irreducible-poly representation + crate docs), `shared/gf2m/src/naive.rs` (new: polynomial-basis impl — XOR add, carryless comb mul, modular reduction, Frobenius square), `shared/gf2m/tests/gf2m_kat.rs` (new: field-axiom + Frobenius KATs) |
| E.F.2 | GF(2^m) inversion (extended Euclidean over GF(2)[x] + Itoh–Tsujii) + division | A | Sonnet | C-F2m (frozen E.F.1) | `shared/gf2m/src/inv.rs` (new: extended-Euclidean baseline + Itoh–Tsujii via the Frobenius tower), `shared/gf2m/src/naive.rs` (fill the `inv`/`div` trait methods), `shared/gf2m/tests/gf2m_kat.rs` (extend: inversion round-trip + ext-Euclid↔Itoh–Tsujii agreement) |
| E.F.3 | GF(2^m) normal-basis representation + polynomial↔normal conversion | A | Sonnet | C-F2m (frozen E.F.1, implemented in a 2nd representation) | `shared/gf2m/src/normal.rs` (new: normal-basis `F2m` impl — squaring as cyclic shift), `shared/gf2m/src/convert.rs` (new: polynomial↔normal change-of-basis isomorphism), `shared/gf2m/src/lib.rs` (add `pub mod normal; pub mod convert;`), `shared/gf2m/tests/gf2m_kat.rs` (extend: basis-conversion round-trip + cross-representation agreement) |
| E.F.4 ◆ `@architect` | GF(2^m) Karatsuba/López–Dahab multiplier + naive-vs-optimized cross-check + sub-track close | C | Sonnet | C-F2m (frozen E.F.1, read), `shared/field` `FpNaive`/`FpMonty` cross-check idiom (read) | `shared/gf2m/src/opt.rs` (new: Karatsuba + López–Dahab multiplier; optional gated `pclmulqdq` path), `shared/gf2m/src/lib.rs` (add `pub mod opt;` + re-export the optimized type), `shared/gf2m/tests/gf2m_kat.rs` (extend: naive↔optimized agreement + full sub-track-close axiom/Frobenius/basis suite) |

**Sequencing notes.** Strictly serial: **E.F.1 → E.F.2 → E.F.3 → E.F.4.** E.F.1 lands the crate,
the `F2m` trait, the polynomial-basis representation, and the naive multiplier everything stands on;
E.F.2 adds inversion (the second irreducible algorithmic unit); E.F.3 adds the normal-basis
representation + conversion; E.F.4 adds the optimized multiplier and closes the sub-track. **Two
`@architect` markers** sit on the design-critical points: **E.F.1** (inline — the C-F2m trait freeze
ratified *before* E.F.2/E.F.3/E.F.4 consume it; three downstream sub-tracks make the trait shape the
most expensive retrofit, so it is caught at its own commit, not deferred) and **E.F.4 ◆** (the Opus
boundary juncture ratifying C-F2m / C-F2mOpt and confirming the substrate is complete and binary-
curve-ready before the sub-track closes). *(Tradeoff named: the inline E.F.1 juncture doubles the
juncture count on a 4-session sub-track relative to a single-◆ shard. This is the deliberate inverse
of the E.E choice — E.E folded its substrate freeze into the ◆ to economise boundary cost on a
2-session shard, explicitly trading away early-catch insurance. Here the trait has **three** downstream
consumers (vs E.E's lift, consumed only by E.E.2), so the cost-of-wrong lever flips the tradeoff: the
early E.F.1 catch is bought, not traded away. The autonomous first cadence — see Notes — still pages
at both `@architect` markers; "autonomous" means no halt at E.F.2/E.F.3, not no juncture at the
design-critical points.)*

**Why 4 sessions (the ROADMAP's 3–4 ceiling-biased estimate).** The split is taken at four
representation/contract-sharp seams:
- **One-line-commit-title corollary.** "GF(2^m) polynomial-basis substrate + `F2m` trait",
  "GF(2^m) inversion + division", "GF(2^m) normal-basis + conversion", and "GF(2^m)
  Karatsuba/López–Dahab multiplier + close" are **four distinct commit titles** across two categories
  (A substrate ×3, C optimization ×1).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the
  trait-and-representation, the inversion algorithm, the second basis, the optimized multiplier.
  Char-2 inversion is *not* a variation on multiplication (extended Euclidean / Itoh–Tsujii are a
  distinct machine); the normal basis is a *distinct representation* with its own payoff (squaring as
  a shift); the optimized multiplier is a *distinct algorithm* cross-checked against the baseline.
  None fractures below its floor.
- **Contract-sharp boundary (E.F.1 freeze).** E.F.1 **freezes** C-F2m; E.F.2 (additively amends) and
  E.F.3 (implements in a 2nd representation) and E.F.4 (consumes + freezes C-F2mOpt) all build on
  it. The trait freeze is the one hard produce/consume seam, and it is the inline-`@architect` point.

They are **not** further splittable below 4: separating the `F2m` trait *declaration* from the
naive-multiplier *implementation* would leave a trait with no implementor and no testable contract
(a trait whose deliverable can't be a KAT has an undefined contract); separating extended-Euclidean
from Itoh–Tsujii would split the single inversion contract across two rows with no contract-sharp
seam (Itoh–Tsujii is checked *against* extended Euclidean — they are one unit); the normal basis +
its conversion are one unit (a normal basis with no conversion to the polynomial basis is unusable by
any consumer). Merging would violate the one-line-title corollary (E.F.1+E.F.2 = substrate-design +
inversion = two titles, >400 LOC, the Opus freeze muddied by carrying inversion).

---

## Session detail

E.F.1 is specified at near-full fidelity (the `F2m` trait + word-layout is the design crux). E.F.2–4
are lower-fidelity sketches, correct per the substrate-first discipline: they are crisply specified
only after C-F2m freezes.

### E.F.1 — GF(2^m) polynomial-basis substrate + `F2m` carryless-mul field trait (Opus `@architect`, Cat A)

**Deliverable:** the new `shared/gf2m` crate, the `F2m` sibling trait, the polynomial-basis
representation, carryless multiplication + modular reduction, and the naive comb-multiplier baseline
the rest of the sub-track stands on. The design choices (the Opus design call, ratified at the inline
`@architect` juncture):
- **The new crate** (`Cargo.toml` root `members` + `shared/gf2m/Cargo.toml`): a leaf crate depending
  only on `crypto-bigint`, mirroring `shared/padic`'s standalone shape. `cargo check --workspace`
  must confirm it resolves with no cycle. **Placement decision: a new `shared/gf2m` crate** (per the
  `shared/padic` precedent for a categorically-new arithmetic substrate), *not* a module in
  `shared/field` (which would conflate two field kinds and risk four dependents).
- **The `F2m` trait** (`shared/gf2m/src/lib.rs`): the design crux. A **sibling** to `Fp<L>`, not an
  impl. Mirror the const-generic-on-trait idiom (`F2m<const L: usize>` where `L` = limb count for
  ⌈m/64⌉, or an explicit degree parameter — **a design call for the juncture**) and the per-call
  "modulus" idiom (passing the **irreducible polynomial** as `poly: &…` where `Fp` passes `p: &Uint<L>`).
  Surface: `zero`/`one`/`from_*`/`to_*`; `add` (XOR — note `sub == add` in char 2); `mul` (carryless +
  reduce); `square` (the Frobenius `a → a²` bit-spread); `pow`; **and the over-specified curve-facing
  ops** (`frobenius`/`trace`/`half_trace` or `solve_quadratic` for `x²+x=c`) declared now even if
  E.F.4/E.G fill them — the substrate-over-specify rule, because adding a trait method later is the
  expensive retrofit across three consumers. `inv`/`div` are **declared but deferred to E.F.2** —
  decide at the juncture whether to declare-and-stub (`unimplemented!`/`todo!`) or leave the methods
  un-added until E.F.2 (the former freezes the full surface now; the latter keeps E.F.1's KATs honest).
- **The polynomial-basis `naive` impl** (`shared/gf2m/src/naive.rs`): a degree-<m polynomial over
  GF(2) as ⌈m/64⌉ `u64` limbs (`Uint<L>` or a `[u64; L]` bit-vector — a juncture call). Add = XOR
  (`a ^ b`). Mul = **naive comb carryless multiply** (shift-and-XOR, the auditable `FpNaive` analogue)
  producing a degree-<2m intermediate, then **modular reduction** by the irreducible (repeated
  XOR-of-shifted-modulus, or the sparse-trinomial/pentanomial fast-reduce if the irreducible is
  fixed sparse — a juncture call on whether the irreducible is a const or a runtime parameter).
  Square = Frobenius bit-spread (insert a zero bit between each coefficient bit, then reduce).

Consumes `crypto-bigint` `Uint<L>` primitives (read), the `shared/field` `Fp<L>` idiom (read — the
pattern, not the trait), the `shared/padic` leaf-crate precedent (read). **Freezes C-F2m.**

**KAT** (`shared/gf2m/tests/gf2m_kat.rs` + inline unit tests per the `shared/field` idiom): the field
axioms over a toy field (e.g. GF(2^4) with `x⁴+x+1`, GF(2^8) with the AES polynomial `x⁸+x⁴+x³+x+1`):
**associativity** and **distributivity** of `mul` over `add`; `add` is its own inverse (`a+a=0`);
`mul` by `one` is identity; the **Frobenius fixed-field law** `a^(2^m) = a` for all `a` (the
characteristic-2 signature — exactly decisive); `square(a) == mul(a, a)`. **Verify gate:** `cargo test
--workspace` green; `cargo check --workspace` resolves the new crate with no cycle; the existing
rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **`sub == add` in char 2** — subtraction is XOR, identical to
addition; `neg` is the identity (`−a = a`). A `@build` agent porting `Fp`'s `sub`/`neg` (which
compute `p − self`) writes wrong code that *happens to compile* — the trait must document this and
the KAT must check `sub(a,b) == add(a,b)`. (2) **Modular reduction is the correctness heart of `mul`**
— a carryless multiply without reduction gives a degree-<2m polynomial *not in the field*; getting
the reduction wrong (wrong irreducible, off-by-one in the shift) silently gives a wrong product that
still "looks like" a field element. The Frobenius fixed-field KAT `a^(2^m)=a` is the loud signal (it
fails loudly if reduction is wrong). (3) **The irreducible polynomial is the field's identity** —
GF(2^m) is only well-defined relative to a chosen irreducible; the representation must carry it (as a
const or a runtime `poly` parameter — the juncture call), and conversions/comparisons across
*different* irreducibles are meaningless (a guard, like the C-MovBridge modulus-consistency note from
E.C). (4) **Over-specify for the curve consumer** — the trace and half-trace (`solve_quadratic`) are
not needed by E.F itself but *are* needed by E.G's binary-curve point operations; declaring them now
(even stubbed) is cheaper than amending the trait across three consumers later. (5) **Toy field sizes
only** — small `m` exercises every path; a crypto-scale `m` (163, 233, 571) would need the same
algorithms (principle-4 annotate: the parameters are toy, the algorithms are not).

**Deferred:** inversion + division (E.F.2 — declared, deferred); the normal-basis representation +
conversion (E.F.3); the optimized Karatsuba/López–Dahab multiplier + `pclmulqdq` path (E.F.4); any
binary *curve* (E.G — out of scope, the curve group law / Koblitz automorphism); any `GF(p^m)`
odd-characteristic tower (out of scope — `fpext.rs` already covers it); the MATHEMATICS chapter (T.E
at the Track-E ◆).

**`@architect` confirmation (inline, post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at
E.F.1 to ratify the trait freeze *before* E.F.2 consumes it: (1) the `F2m` trait shape is right —
sibling to `Fp<L>` (not an impl), the const-generic / degree parameterisation chosen, the per-call
irreducible-polynomial idiom; (2) the curve-facing over-specification (Frobenius / trace / half-trace)
is present so E.G need not amend the trait; (3) the polynomial-basis word layout (⌈m/64⌉ limbs) and
the irreducible representation (const vs runtime) are settled; (4) `sub==add`/`neg==id` are correct,
not ported from `Fp`; (5) the principle-4 boundary (toy field sizes) is annotated. One-shot findings;
does not implement. Held at **Opus** per the header (lever 3 — the trait bounds E.G/E.H/E.I).

### E.F.2 — GF(2^m) inversion (extended Euclidean + Itoh–Tsujii) + division (Sonnet, Cat A)

**Deliverable:** characteristic-2 inversion and division, filling the `inv`/`div` methods E.F.1
deferred. Lower-fidelity sketch (crisp after C-F2m freezes):
- **Extended Euclidean over GF(2)[x]** (`shared/gf2m/src/inv.rs`): the auditable baseline — compute
  `a⁻¹` via the extended polynomial GCD of `a` with the irreducible, tracking the Bézout cofactor.
- **Itoh–Tsujii** (`shared/gf2m/src/inv.rs`): `a⁻¹ = a^(2^m−2)` computed through the **Frobenius
  tower** (the `2^k`-power chain) with O(log m) multiplications — the field-specific optimization,
  exact-checkable against extended Euclidean.
- **Division** (`shared/gf2m/src/naive.rs`): `div(a, b) = mul(a, inv(b))`; the `inv`/`div` trait
  methods E.F.1 deferred are filled.

Consumes C-F2m (frozen E.F.1). **Amends C-F2m additively** (the deferred `inv`/`div` methods).

**KAT:** inversion round-trip `mul(a, inv(a)) == one` for all non-zero `a` over the toy fields;
**extended-Euclidean ↔ Itoh–Tsujii agreement** (both give the same `a⁻¹`); `inv` of zero errors/panics
per the trait contract; `div(a,b) == mul(a, inv(b))`. **Verify gate:** `cargo test --workspace` green.

**Subtlety:** Itoh–Tsujii's Frobenius tower depends on the squaring being correct (E.F.1's Frobenius)
— a bug in `square` shows up here as a wrong inverse, which the ext-Euclid cross-check catches.

### E.F.3 — GF(2^m) normal-basis representation + polynomial↔normal conversion (Sonnet, Cat A)

**Deliverable:** a second representation of the same field, where squaring is a cyclic shift, plus the
change-of-basis isomorphism. Lower-fidelity sketch:
- **Normal-basis `F2m` impl** (`shared/gf2m/src/normal.rs`): a second implementor of the frozen
  `F2m` trait, in the normal basis `{β, β², …, β^(2^(m−1))}` where **squaring is a cyclic shift** of
  the coefficient vector (the payoff). Multiplication in a normal basis uses the multiplication table
  / λ-matrix (demonstration fidelity — Gaussian-normal-basis optimization is principle-2 scale-only).
- **Change-of-basis** (`shared/gf2m/src/convert.rs`): the polynomial↔normal isomorphism, so a field
  element computed in one representation maps faithfully to the other.

Consumes C-F2m (frozen E.F.1, implemented in a 2nd representation).

**KAT:** the basis-conversion **round-trip** `to_normal(to_poly(x)) == x`; **cross-representation
agreement** (a `mul`/`square` in the polynomial basis equals the same op in the normal basis after
conversion); **squaring-is-a-shift** verified directly in the normal basis. **Verify gate:** `cargo
test --workspace` green.

**Subtlety:** the normal basis exists *for* the squaring-as-shift property; the KAT must check that
property directly (not just that `square` returns the right element, but that it is implemented as a
shift), or the pedagogical point is lost — flag if the deliverable can't exhibit it.

### E.F.4 ◆ — GF(2^m) Karatsuba/López–Dahab multiplier + cross-check + sub-track close (Sonnet, Cat C, `@architect`)

**Deliverable:** the optimized multiplier, the naive↔optimized cross-check, and the sub-track close.
Lower-fidelity sketch:
- **Optimized multiplier** (`shared/gf2m/src/opt.rs`): **Karatsuba** (subquadratic split of the
  carryless multiply) and/or **López–Dahab** (comb method with windowing) — the optimized analogue of
  the E.F.1 naive comb, cross-checked against it. An optional **`pclmulqdq`** carryless-multiply path
  is a **gated** demonstration-fidelity sidecar (`#[cfg(target_feature = "pclmul")]`, never the green
  path — principle 3: engineering, not mathematics).
- **Sub-track-close KAT suite** (`shared/gf2m/tests/gf2m_kat.rs`, extended): the full axiom suite,
  Frobenius fixed-field, basis-conversion round-trip, and **naive↔optimized agreement** (the
  `FpNaive`/`FpMonty` agreement pattern — the optimized multiplier gives byte-identical results to
  the auditable baseline). Optional PARI `ffinit`/`fflog` `#[ignore]` sidecar.

Consumes C-F2m (frozen E.F.1, read), the `shared/field` naive-vs-optimized cross-check idiom (read).
**Freezes C-F2mOpt.**

**KAT (primary correctness signal):** **naive↔optimized agreement** across all toy fields (the
optimized multiplier equals the E.F.1 baseline on every input); the full field-axiom + Frobenius +
basis-conversion suite stays green; the existing rho/gnfs/shared KATs unchanged. Optional PARI
cross-check. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The optimized multiplier must be *equivalent*, not just plausible**
— a Karatsuba/comb bug gives wrong products that still pass weak smoke tests; only the exhaustive (or
proptest) naive↔optimized agreement catches it. This is the `FpNaive`/`FpMonty` discipline applied to
char 2. (2) **`pclmulqdq` is never on the green path** — it is gated, optional, demonstration-fidelity
(principle 3); the green-path multiplier is the portable software Karatsuba/comb. Shipping `pclmulqdq`
as the default is a principle-3 violation. (3) **This is the E.F ◆ boundary** — re-read the Purpose
intent and verify the substrate is complete (polynomial basis, both inversions, normal basis +
conversion, optimized multiplier) and **binary-curve-ready** (the curve-facing over-specification —
Frobenius/trace/half-trace — is present and tested), and that E.F stayed curve-free /
prime-extension-free / chapter-free. (4) **No MATHEMATICS chapter here** — the GF(2^m) textbook
content is T.E, paired with E.W at the *Track-E* ◆; E.F.4 writes at most a PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.F.4 ◆ to confirm: (1) the substrate is complete and composes (the four units — poly basis,
inversion, normal basis, optimized multiplier — all present and cross-checked); (2) C-F2m's
curve-facing over-specification (Frobenius/trace/half-trace) is in place so E.G can build binary
curves without amending the trait — the substrate-over-specify defense; (3) C-F2mOpt's
naive↔optimized equivalence holds (the optimized multiplier is exact, not approximate); (4) E.F
stayed in scope — no binary curves (E.G), no `GF(p^m)` tower (`fpext.rs`), no MATHEMATICS chapter
(T.E), `pclmulqdq` gated not green-path; (5) the principle-4 boundary (toy field sizes; the
engineering-vs-mathematics `pclmulqdq` annotation) is recorded, not silently presented as
crypto-scale. **Also: reconcile the static-frame ROADMAP debt** — the Progress / Remaining tables are
stale by two completed sub-tracks (E.D, E.E); the E.F ◆ is the right boundary to update them.
One-shot findings; does not implement. Held at **Opus** per the header.

---

## Cross-session contracts

E.F **freezes two** contracts (C-F2m at E.F.1, C-F2mOpt at E.F.4) and **amends none of the prior
frozen contracts** (the p-adic / SSA / rho-curve / `Fp` surfaces are all untouched — E.F is a
standalone characteristic-2 field crate). Per the substrate-over-specify rule, C-F2m carries the
curve-facing operations (Frobenius, trace, half-trace) now, even though E.F itself does not consume
them, because E.G/E.H/E.I will.

### C-F2m — GF(2^m) `F2m` field trait + polynomial-basis substrate (compiler- + test-enforced) — *to be frozen at E.F.1*

**Defined in:** E.F.1 (`shared/gf2m/src/lib.rs`, `shared/gf2m/src/naive.rs`). **Consumed by:** E.F.2
(fills `inv`/`div`), E.F.3 (implements in the normal basis), E.F.4 (the optimized multiplier);
**downstream: E.G** (binary curves — the Frobenius/trace/half-trace point operations), **E.H** (GHS
descent over GF(2^m)), **E.I** (GF(2^m) hyperelliptic Jacobian). Compiler-enforced (the `F2m` trait +
the polynomial-basis impl + the irreducible representation) + test-enforced (the field axioms + the
Frobenius fixed-field law `a^(2^m)=a`). Exposes: the `F2m<const L: usize>` (or degree-parameterised)
trait — `add`(XOR)/`sub`(==add)/`neg`(==id)/`mul`(carryless+reduce)/`square`(Frobenius)/`pow`, the
over-specified `frobenius`/`trace`/`half_trace`(`solve_quadratic`), and `inv`/`div` (deferred to
E.F.2); the polynomial-basis `naive` implementor; the irreducible-polynomial representation. *Exact
parameterisation (`F2m<const L>` vs an explicit degree const; irreducible as compile-time const vs
runtime parameter; `Uint<L>` vs `[u64; L]` backing) ratified at the inline E.F.1 `@architect`
juncture and re-ratified at the E.F.4 ◆.* **Char-2 invariants:** `sub == add`, `neg == identity`,
`mul` always reduces by the irreducible. **The irreducible defines the field** — cross-irreducible
operations are meaningless (a consistency guard). **Toy field sizes only** (principle-4 boundary).

**Resolved interface (ratified at the inline E.F.1 `@architect` juncture, 2026-06-13).** The five
open design calls are settled as follows; this is the surface E.F.2/E.F.3/E.F.4 and downstream
E.G/E.H/E.I consume. *(Re-ratify at the E.F.4 ◆.)*

1. **Trait parameterisation — `F2m<const L: usize>` (limb count), NOT an explicit degree const.**
   `L` = ⌈m/64⌉ = the number of `u64` limbs, mirroring `Fp<L>` exactly. The degree `m` is a
   *runtime* property, recovered from the irreducible as `m = poly.bits() − 1` (the degree of the
   irreducible). Rationale: stable Rust (edition 2024) forbids the `Uint<{ceil(M/64)}>` const
   arithmetic an explicit-degree form would need — this is the same `generic_const_exprs`-avoidance
   the `Fp` design note (`shared/field/src/lib.rs:7-18`) documents. *Load-bearing assumption:* no E.F
   consumer needs `m` as a compile-time const for array sizing — the KATs compute `2^m` at runtime;
   the Frobenius law `a^(2^m)=a` reads `m` from `poly`. If a downstream consumer later needs a
   compile-time degree, that is an additive amend, not a break.

2. **Irreducible — a runtime `poly: &Uint<L>` parameter (per-call), NOT a compile-time const.**
   Mirrors `Fp`'s per-call `p: &Uint<L>` exactly: every operation that needs the field's identity
   (`mul`, `square`/`frobenius`, `pow`, and the deferred `inv`/`div`) takes `poly: &Uint<L>`. The
   irreducible is the bit-vector of the degree-m reduction polynomial (bit i set ⟺ coefficient of
   x^i is 1; e.g. GF(2^4) with x⁴+x+1 is `0b1_0011`, GF(2^8) AES x⁸+x⁴+x³+x+1 is `0b1_0001_1011`).
   Rationale: (a) the established idiom the PLAN mandates mirroring; (b) one impl exercises *both*
   toy fields the E.F.1 KAT names (GF(2^4) and GF(2^8)) without monomorphising per-poly. *Tradeoff
   named:* a runtime `poly` is worse at fast sparse (trinomial/pentanomial) reduction than a
   compile-time const would be — it cannot specialise the reduction to a known sparse modulus at
   compile time. This is deliberately deferred: the **sparse fast-reduce is an E.F.4 optimised-path
   concern** (it lives with the optimised multiplier under C-F2mOpt), not a substrate decision; the
   E.F.1 naive reduction (repeated XOR-of-shifted-modulus, degree-driven) is the auditable baseline
   regardless of modulus shape.

3. **Backing storage — `Uint<L>` from `crypto-bigint` (a coefficient bit-vector), NOT a raw
   `[u64; L]`.** Mirrors `FpNaive { v: Uint<L> }`. The `Uint<L>` is consumed for `as_words`/
   `from_words` (limb access for the comb multiply and the Frobenius bit-spread), `BitXor` (add),
   and shifts — and it provides the `Clone`/`PartialEq`/`Eq`/`Debug` the trait bounds need for free.
   The carryless comb multiply produces a degree-<2m intermediate in `Uint<2L>` (the same `($L, $DL =
   2*$L)` widening the `impl_fp_naive!` macro already uses, `shared/field/src/naive.rs:48-49`), then
   reduces back to `Uint<L>`. *Tradeoff named:* `Uint<L>` carries integer-arithmetic methods
   (`wrapping_add`, `rem`, `mul_wide`) that are **meaningless on a GF(2) coefficient vector** — a
   `@build` agent could call `mul_wide` (integer mul-with-carry) where the carryless comb is meant.
   This is the storage analogue of the `sub==add` trap; the crate + impl docs MUST state "the
   `Uint<L>` is a polynomial coefficient bit-vector, not an integer — only XOR, shift, and bit
   operations are meaningful; `mul`/`square` use the carryless comb, never `mul_wide`." (Note:
   `mul_wide` *is* in the Consumes column, but for the *widening-type plumbing* `(lo, hi) →
   Uint<2L>`, not as the multiply itself — the multiply is carryless comb.)

4. **Method surface — full surface frozen now; `inv`/`div` + `trace`/`half_trace` declared-and-
   stubbed.** The trait declares, with `poly: &Uint<L>` threaded through every modulus-dependent op:
   - **Constructors / canonical form:** `zero`/`one`/`from_u64`/`from_uint`/`to_uint`
     (mirroring `Fp`; `from_uint`/`to_uint` treat the `Uint<L>` as the coefficient bit-vector — no
     reduction-mod-integer, but a *polynomial* reduction mod `poly` if the input has degree ≥ m).
   - **Char-2 arithmetic (implemented in E.F.1):** `add` (XOR, `a ^ b`); `sub` (**== `add`** —
     documented, KAT-checked `sub(a,b)==add(a,b)`); `neg` (**== identity**, returns `self` clone);
     `mul` (carryless comb + reduce-by-`poly`); `square` (Frobenius bit-spread + reduce); `pow`
     (square-and-multiply, mirroring `FpNaive::pow`); plus `is_zero`/`is_one` defaults.
   - **Curve-facing over-specification (the substrate-over-specify rule):** `frobenius(self, poly)`
     — first-class `a → a²`, *implemented* in E.F.1 (it **is** `square`; declared distinctly because
     E.G/E.H reach for it by that name and may iterate it as the Frobenius map); `trace(self, poly)`
     — `Σ_{i<m} a^(2^i)`, **declared-and-stubbed** `unimplemented!("E.G")`; `half_trace`/
     `solve_quadratic(c, poly)` — solve `x²+x=c`, **declared-and-stubbed** `unimplemented!("E.G")`.
   - **Deferred to E.F.2 (declared-and-stubbed):** `inv(self, poly)` and `div(self, rhs, poly)` —
     bodies `unimplemented!("filled in E.F.2")`, with a documented zero-input contract (`inv(0)`
     panics/errors, matching `FpNaive::inv`'s zero-guard).

   Rationale for **declare-and-stub** over leave-un-added (the PLAN-flagged call): the trait bounds
   **three** downstream sub-tracks (E.G/E.H/E.I); adding a trait method later is the most expensive
   retrofit in the sub-track (the Cat-A cost-of-wrong that holds this juncture at Opus). Freezing the
   full surface now is the substrate goal. The PLAN's counter-consideration ("leave un-added to keep
   E.F.1's KATs honest") is *reconciled, not traded away*: E.F.1's KATs (axioms + Frobenius, PLAN
   line 344-347) **do not call** `inv`/`div`/`trace`/`half_trace`, so a stub that is never exercised
   keeps the KATs honest while still freezing the surface. *Tradeoff named:* a compiling-but-panicking
   stub is a latent trap if a future `@build` agent calls it before its session fills it — mitigated
   by `unimplemented!` messages that name the filling session ("E.F.2"/"E.G"), and by the inline
   juncture ratifying that no E.F.1-consumed path touches a stub.

5. **`naive` impl shape — `F2mNaive<const L> { c: Uint<L> }`, per-`L` macro mirroring
   `impl_fp_naive!`.** A struct `F2mNaive<const L: usize> { c: Uint<L> }` where `c` is the
   coefficient bit-vector, deriving `Clone, Debug, PartialEq, Eq`. The `F2m<L>` impl is generated by
   a macro `impl_f2m_naive!($L, $DL)` over the same `($L, $DL=2*$L)` pairs `(1,2)/(2,4)/(4,8)/(8,16)`
   the `Fp` naive macro uses (the comb multiply widens to `Uint<$DL>` for the degree-<2m intermediate
   before reducing). `add` = `Uint`-XOR; `mul` = comb (for each set bit i of `rhs`, XOR `self`
   left-shifted by i into the `Uint<2L>` accumulator) then `reduce(acc, poly)` (while the
   accumulator's degree ≥ m, XOR in `poly` shifted to align with the top set bit); `square` =
   bit-spread (insert a zero between each coefficient bit) then reduce; `frobenius` delegates to
   `square`. The irreducible is **not** stored on the struct — it is passed per-call as `poly`
   (mirroring `Fp` storing `p` nowhere on `FpNaive` and threading it through every method). A
   *cross-irreducible* consistency note is documented (operations mixing elements reduced under
   different `poly` are meaningless — the C-MovBridge-style guard).

**Crate scaffold (also ratified):** `shared/gf2m/Cargo.toml` is a leaf crate `name = "shared-gf2m"`,
`edition = "2024"`, single dependency `crypto-bigint = { version = "0.5", features = ["rand_core"] }`
(matching `shared/field`), `[dev-dependencies] proptest = "1"`, and the `[lints]` block mirroring the
`shared/padic` precedent (`unsafe_code = "forbid"`, `missing_docs = "warn"`, clippy `all = "deny"` /
`pedantic = "warn"`). Root `Cargo.toml` adds `"shared/gf2m"` to `members`. No new edge (no existing
crate depends on it — the `rho → shared-gf2m` edge is E.G's). `cargo check --workspace` must resolve
with no cycle.

### C-F2mOpt — GF(2^m) optimized-multiplier equivalence (compiler- + test-enforced) — *to be frozen at E.F.4 ◆*

**Defined in:** E.F.4 (`shared/gf2m/src/opt.rs`). **Consumed by:** any performance-sensitive consumer
of GF(2^m) multiplication (E.G–E.K, where toy-scale benchmarks compare); E.W (the cross-attack
benchmark table). Compiler- + test-enforced. Exposes the optimized polynomial-basis multiplier
(Karatsuba / López–Dahab) and the optional gated `pclmulqdq` path. **The optimized multiplier is
*equivalent* to the E.F.1 naive baseline** (byte-identical results on every input — the
naive↔optimized agreement KAT, mirroring `FpNaive`/`FpMonty`). **`pclmulqdq` is never the green path**
(gated demonstration-fidelity sidecar — principle 3). *Exact algorithm choice (Karatsuba vs
López–Dahab vs both; the windowing parameters) ratified at the E.F.4 ◆.*

### Frozen contracts read by E.F (consumed, not amended)

- **`crypto-bigint` `Uint<L>`** — `as_words`/`from_words`/`mul_wide` for the `u64`-limb word layout.
  Read (an external dependency, not a project contract).
- **`shared/field` `Fp<L>` + `FpNaive`/`FpMonty`** — the *design idiom* E.F mirrors (const-generic
  trait, per-call modulus, naive-baseline-vs-optimized cross-check). **Read for the pattern; NOT
  implemented** (the survey proved GF(2^m) cannot impl `Fp<L>`). Untouched.
- **`shared/padic` (Cargo.toml shape)** — the leaf-crate precedent `shared/gf2m` mirrors. Read.

### New workspace crate (not a contract amendment, not a new edge)

- **`shared/gf2m`** — E.F.1 adds it to the root `Cargo.toml` `members` list. **A new crate, no new
  edge** (it depends only on `crypto-bigint`; no existing crate depends on it yet). The
  `rho → shared-gf2m` edge is **E.G's** to add (mirroring how `rho → shared-padic` was E.E's, not
  E.D's). `cargo check --workspace` confirms the crate resolves with no cycle. *(If E.F found it must
  change an existing crate's API — it should not; E.F is standalone — that would be a discovery
  surfaced at the ◆, never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.F.1 inline `@architect` and the E.F.4 ◆
`@architect` confirmations are not separate ledger rows (paged forks with no commit-shaped
deliverable); their outcomes are recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.F.1 | GF(2^m) polynomial-basis substrate + `F2m` carryless-mul field trait | pending | — | C-F2m (+ `shared/gf2m` crate) |
| E.F.2 | GF(2^m) inversion (ext-Euclid + Itoh–Tsujii) + division | pending | — | C-F2m (additive: `inv`/`div`) |
| E.F.3 | GF(2^m) normal-basis representation + polynomial↔normal conversion | pending | — | — (implements C-F2m, 2nd repr) |
| E.F.4 ◆ | GF(2^m) Karatsuba/López–Dahab multiplier + cross-check + sub-track close | pending | — | C-F2mOpt |

Contracts frozen before this sub-track (NOT read by E.F — it is standalone): the p-adic surface
(C-Padic/C-Hensel/C-PadicLog), the SSA surface (C-AnomalousLift/C-SSA), the rho curve + ECDLP
surface, `Fp<4>`. This sub-track **freezes two new contracts** (C-F2m, C-F2mOpt), serving the
downstream **E.G** (binary curves), **E.H** (GHS descent), **E.I** (GF(2^m) hyperelliptic Jacobian),
and the benchmark/textbook consumers **E.W**/**T.E**, and **opens the binary branch of Track E**.

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.F is greenfield GF(2^m) in a new `shared/gf2m` crate — building the crate + trait + poly basis
  + inversion + normal basis + optimized multiplier is internal-continue (confirmed by survey).** No
  characteristic-2 field code exists. A discovery that the normal-basis multiplication or the
  Itoh–Tsujii tower needs a field-arithmetic primitive C-F2m did not over-specify is an **additive
  amend of C-F2m** surfaced at the next `@architect` juncture (not a silent trait patch).

- **GF(2^m) cannot implement `Fp<L>` — forcing it is a rigidity failure (the central design guard).**
  The survey proved four incompatibilities (Fermat `inv`, odd-prime `legendre`/`sqrt`, `neg` as
  subtraction, integer-mod-p representation). A `@build` agent that tries to `impl Fp<L> for` a
  GF(2^m) type — or ports `Fp`'s `sub`/`neg` semantics — writes wrong code that may compile. The
  `sub==add` / `neg==id` KAT and the sibling-trait design are the defense — **internal-continue →
  corrected** (the inline E.F.1 juncture catches a wrong trait shape before it propagates).

- **Modular reduction by the irreducible is mandatory in every `mul` (the silent-wrong-product
  guard).** A carryless multiply without reduction yields a degree-<2m polynomial not in the field; a
  wrong irreducible or off-by-one reduction silently gives a wrong product. The **Frobenius
  fixed-field law `a^(2^m)=a`** is the loud signal — it fails iff reduction is wrong.
  **Internal-continue → corrected.**

- **The optimized multiplier must be *equivalent* to the naive baseline (the silent-optimization-bug
  guard).** A Karatsuba/López–Dahab bug gives wrong products passing weak tests; only the exhaustive
  (or proptest) **naive↔optimized agreement** catches it (the `FpNaive`/`FpMonty` discipline). A KAT
  that only asserts the optimized `mul` returns *something* has an under-specified contract — flag it.
  **Internal-continue → corrected.**

- **`pclmulqdq` on the green path is a principle-3 violation (the engineering-on-the-compute-path
  guard).** The carryless-multiply intrinsic is *engineering*; it ships at most as a gated
  demonstration-fidelity sidecar (`#[cfg(target_feature)]`), never as the default multiplier. A
  `@build` agent that makes `pclmulqdq` the green-path `mul` violates principle 3 —
  **internal-continue → corrected** (gate it).

- **E.F adds a new crate, not a new edge — a dependency cycle would be a destructive-HALT.**
  `shared/gf2m` depends only on `crypto-bigint` and is depended-on by nothing yet (the
  `rho → shared-gf2m` edge is E.G's). If `cargo check --workspace` reports a cycle (it must not), or
  if E.F finds it must change an *existing* crate's API (it should not — E.F is standalone), that is a
  **destructive-HALT** — stop, surface it.

- **No binary curves in E.F (defocus guard).** The binary-curve group law (`y²+xy=x³+ax²+b`),
  López–Dahab point coordinates, point decompression, and the order-6 Koblitz automorphism are all
  **E.G** (a different sub-track, the consumer of C-F2m). A `@build` agent that implements a binary
  *curve* in E.F is defocus — internal-continue only within the field-arithmetic scope. *(E.F's
  job is to over-specify the trait — Frobenius/trace/half-trace — so E.G can build the curve without
  amending it.)*

- **No `GF(p^m)` odd-characteristic tower in E.F (defocus / scope clarity).** The prime-extension
  field already exists (`rho/src/pairing/fpext.rs`, C-FpExt, for the odd-characteristic pairing case).
  E.F is **characteristic 2 only**. A `@build` agent that generalises to `GF(p^m)` for odd `p` is
  defocus.

- **No MATHEMATICS.md chapter in E.F (defocus / scope clarity).** The GF(2^m) / binary-field textbook
  content is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing), not at the
  E.F sub-track ◆. E.F.4 writes at most a PEDAGOGY code-tour delta.

- **No oracle dependency for correctness (principle-3 / E.D/E.E-consistent).** GF(2^m) arithmetic is
  exactly self-checking (field axioms + Frobenius law + naive↔optimized + basis-conversion
  round-trip); a PARI `ffinit`/`fflog` cross-check is an **optional `#[ignore]` sidecar** (the
  established `#[ignore = "PARI not installed; run manually when available"]` pattern). E.F introduces
  no new live oracle.

- **Toy field sizes only (scope clarity).** E.F fixes small `m` (e.g. GF(2^4), GF(2^8), and
  toy-shaped larger fields). The toy field sizes are a principle-4 boundary — the algorithms are
  crypto-scale-correct; only the *parameters* are toy; the `pclmulqdq` omission is the
  engineering-vs-mathematics annotation. Presenting any as crypto-scale is a documentation defect
  (internal-continue → corrected).

- **Static-frame ROADMAP debt (reconcile at the E.F ◆, does NOT block E.F).** The ROADMAP Progress /
  Remaining tables are reconciled only through E.C ◆ and are stale by two completed sub-tracks (E.D,
  E.E both done). The E.F ◆ juncture should update them. Not an implementation concern.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.F, "*GF(2^m) field arithmetic … polynomial-basis and
  normal-basis representations … categorically new field-arithmetic implementation … First session is
  Opus-tier — substrate decision (basis choice, word layout, multiplication algorithm: comb,
  Karatsuba, López–Dahab) has downstream consequences for E.G, E.H, E.I.*"; the design statement's
  principles 1 + 3 + 4; the "On scale" mathematical-dimension framing — GF(2^m)'s extension degree m
  is *mathematical-dimension scale*, orthogonal to operational scale) and this PLAN before any
  session. **NOTE: the ROADMAP Progress / Remaining tables are stale by two sub-tracks (E.D, E.E
  done); reconcile at the E.F ◆.**
- Read the **templates to mirror**: `shared/field/src/lib.rs` (the `Fp<L>` trait — the *idiom* to
  mirror as a sibling, NOT the trait to implement; note the const-generic-on-trait design note at
  `lib.rs:7-18` and why nightly `generic_const_exprs` is avoided); `shared/field/src/naive.rs` (the
  `FpNaive` schoolbook baseline + the `impl_fp_naive!` macro for per-`L` impls — the "correct, slow,
  easy to audit" baseline E.F.1's naive comb multiplier mirrors); `shared/field/src/monty.rs`
  (`FpMonty` — the optimized sibling E.F.4's Karatsuba/López–Dahab mirrors);
  `shared/field/tests/sqrt_legendre_kat.rs` (the KAT idiom + `naive_monty_agree_p17` — the
  cross-implementation agreement pattern E.F.4 mirrors); `shared/padic/Cargo.toml` (the leaf-crate
  shape `shared/gf2m` mirrors — depends only on the arithmetic primitive, standalone).
- **Register:** E.F is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New crate `shared/gf2m/` with `src/{lib,naive,inv,normal,convert,opt}.rs` and
  `tests/gf2m_kat.rs`, plus the root `Cargo.toml` `members` entry.
- **Tier routing:** **E.F.1 is Opus `@architect`** (the substrate-design session itself, per the
  ROADMAP Opus flag — run on Opus, not `@build`) carrying an **inline `@architect` juncture** (page
  `@plan-juncture`) ratifying C-F2m before E.F.2 consumes it. **E.F.2–E.F.4 are Sonnet `@build`.**
  E.F.4 carries the **◆ `@architect` juncture** (page `@plan-juncture`) ratifying C-F2m/C-F2mOpt and
  confirming binary-curve-readiness before the sub-track closes. juncture-tier (header) is **opus** —
  held by lever 3 (the trait + word-layout bounds three downstream sub-tracks E.G/E.H/E.I); the
  strong lever-5 exactly-checkable KATs (field axioms + Frobenius law + naive↔optimized) would license
  `sonnet` in isolation, but the user judged the substrate design-error cost decisive, mirroring the
  E.D/E.E calls.
- **Invariants to preserve:** **`sub == add`, `neg == identity`** in char 2 (NOT ported from `Fp`).
  **Every `mul` reduces by the irreducible** (the Frobenius fixed-field law `a^(2^m)=a` is the
  guard). **The optimized multiplier is byte-equivalent to the naive baseline** (naive↔optimized KAT).
  **`pclmulqdq` is gated, never the green path** (principle 3). **The trait over-specifies for the
  curve consumer** (Frobenius/trace/half-trace declared so E.G need not amend C-F2m). **E.F is
  standalone** — depends only on `crypto-bigint`, touches no existing crate. **No binary curves**
  (E.G). **No `GF(p^m)` tower** (`fpext.rs`). **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy
  field sizes only; no new live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional `ffinit`/`fflog` GF(2^m) cross-check
  follows the established `#[test] #[ignore = "PARI not installed; run manually when available"]`
  pattern; never on the green path.
- **The new crate (load-bearing for E.F).** E.F.1 adds `shared/gf2m` to the root `Cargo.toml`
  `members`. `cargo check --workspace` must resolve with no cycle (`shared/gf2m` depends only on
  `crypto-bigint`; nothing depends on it yet). **A new crate, not a new edge** — the
  `rho → shared-gf2m` edge is E.G's.
- Suggested first invocation: **`/run-plan docs/PLAN.md`** (autonomous cadence — user-confirmed). The
  shard pattern (a categorically-new leaf field crate mirroring `shared/padic`) is structurally
  proven, so the autonomous cadence is warranted; the **two `@architect` markers** (inline E.F.1, ◆
  E.F.4) still page their junctures regardless — autonomous means no halt at E.F.2/E.F.3, *not* no
  juncture at the two design-critical points. *(Tradeoff vs `halt-at-boundaries`: autonomous trades a
  per-boundary human glance for velocity, accepting that the trait-design risk is concentrated at
  E.F.1, which the inline `@architect` juncture already catches at its own commit. If E.F.1's juncture
  surfaces a trait-shape concern, fall back to `halt-at-boundaries` for E.F.2–E.F.4.)*
