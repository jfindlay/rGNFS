<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.J — Semaev summation polynomials: "the combinatorial heart of Gaudry–Diem index calculus; mathematically beautiful")

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **set by lever-3 (cost of design error), a deliberate
user override of the lever-5 `sonnet` recommendation.** The default tuning-law read pointed to
`sonnet`: lever 5 is **strong** (the Semaev vanishing relation `S_m(x_1,…,x_m)=0 ⟺ ∃ y_i: Σ P_i = ∞`
is **exactly self-checking** — a fast, decisive, oracle-free KAT, like E.I's group axioms and
*unlike* E.H's oracle-leaning log-preservation that held E.H at opus), and correctness-criticality
(lever 4) is moderate (the ROADMAP marks E.J Sonnet throughout with **no Opus-flagged session**).
That coincidence — strong lever 5 + moderate lever 4 — is the textbook lever-5 opt-down license.
**The user held the ◆ juncture at opus on lever 3:** the multivariate/symmetric-polynomial
representation E.J freezes (C-SemaevPoly) is consumed by the **Opus-flagged E.K** (Gaudry–Diem–
Joux–Vitse index calculus), and the user judges the cost-of-wrong on that substrate worth paying
Opus at the boundary to de-risk — mirroring the E.I precedent (where lever 5 was strong and the
user likewise overrode on lever 3). *(This is the inverse situation to E.H, where levers 2+3 held
the juncture up directly; here lever 5 would license the opt-down and the override is a deliberate
lever-3 call, recorded so the disagreement stays visible. The ◆ fork pages `@plan-juncture` at
opus; per-row session tiers are all Sonnet — E.J has no Opus-flagged session.)*

**Scope boundary — load-bearing, surfaced and adjudicated at shard time.** E.J is the **Semaev
summation-polynomial machinery**, NOT the index-calculus solver that consumes it. The ROADMAP
separates **E.J** (Semaev polynomials — the combinatorial primitive) from **E.K** (Gaudry–Diem–
Joux–Vitse index calculus — the DLP *solver* that uses the Semaev polynomials for the
point-decomposition / relation-collection step, consuming E.J + G.B scoring + G.E linear algebra).
**The user adjudicated E.J = Semaev-only**, verified structurally by the **vanishing relation**
(`S_m` vanishes on the x-coordinates of `m` points summing to the identity). E.J builds and verifies
the summation polynomials; it does **not** run index calculus (relation collection over a factor
base, linear algebra over `Z/nZ`) — that is E.K. *(Tradeoff named: no end-to-end "DLP solved via
index calculus" signal in E.J; bought to keep the delicate Semaev construction decoupled from the
equally-delicate index-calculus design — the same transfer/structure/solve decoupling NOTES.md
records from the E.H shard, and the coupling the inflection-juncture discipline warns against.)*

**Target field — adjudicated F_p / Weierstrass (the classical Gaudry–Diem setting).** Semaev
polynomials have two classical homes: the prime/Weierstrass curve `E/F_p` (or `F_{p^n}`) and the
binary-curve / descended-Jacobian setting coupled to the E.H GHS chain. **The user adjudicated the
prime/Weierstrass target:** E.J builds `S_m` over the short-Weierstrass curve `E/F_p` using the
existing `rho::curve::Curve` (`y² = x³ + ax + b`) + `shared-field` `Fp` surface — the standalone
"combinatorial heart" the ROADMAP names (predecessor **G.A polynomial machinery only**, *not* the
GF(2^m) chain). This keeps E.J independent of E.H/E.I and defers the binary-field coupling to E.K.
*(Tradeoff named: the F_p Semaev polynomials are not the binary-curve ones E.K's GHS-descended
attack will ultimately need; the F_p construction is the textbook primitive — E.K adapts the
representation to its field. Bought because the ROADMAP's G.A-predecessor framing implies the F_p
setting and it keeps the two delicate designs decoupled.)*

The substrate survey (forked `@explore`, 2026-06-15) established the shape and confirmed the
planning assumptions:

1. **E.J's polynomial substrate is doubly greenfield** (the load-bearing finding). The survey
   confirmed: (a) **no univariate `Poly` over a prime field exists** — the only resultant surfaces
   are `shared-numfield::resultant` (over `ℤ[x]`, Sylvester-matrix, with `subresultant_gcd`) and
   `gf2m::Poly::resultant` (over `GF(2^m)`); **neither is an `F_p[x]` resultant**. (b) **No
   multivariate polynomial type exists anywhere** — and Semaev's `S_m(X_1,…,X_m)` is genuinely
   multivariate and *symmetric* in its `m` arguments. **E.J.1 builds both** — the `F_p[x]`
   resultant and the multivariate/symmetric-polynomial representation — the substrate the whole
   construction stands on. This is the design crux (the symmetric representation — dense vs
   elementary-symmetric-basis vs Lagrange — is a genuine, downstream-consumed decision), which is
   why the user held the ◆ juncture at opus despite the strong lever-5 signal.

2. **The source curve is `rho::curve::Curve` over `F_p`, hardcoded `Uint<4>`** (the C1 ceiling the
   ROADMAP flags for E.K). The survey verified the public API: `Curve` (`y² = x³ + ax + b`),
   `AffinePoint<F>` (`Infinity` / `Finite { x, y }`, generic over `Fp<4>`), point addition /
   doubling. E.J **amends no curve contract** — it reads `Curve`/`AffinePoint` as the geometry the
   summation relation is checked against (the `S_m` vanishing KAT computes `Σ P_i` via the frozen
   group law and asserts `S_m` vanishes on the x-coordinates). The toy `F_p` scale and the
   `Uint<4>` ceiling are a **principle-4 boundary** (Semaev is crypto-scale-correct; only the
   parameters are toy), recorded not silently presented as crypto-scale.

3. **The Semaev construction is fully greenfield** — the survey found zero `semaev` /
   `summation_poly` / `summation_polynomial` hits anywhere. E.J builds `S_2`, `S_3`, and the
   resultant recursion `S_m = Res_X(S_{m-1}(X_1,…,X_{m-2},X), S_3(X_{m-1},X_m,X))` from scratch in
   a new `rho::semaev` module.

4. **`rho` already depends on every needed crate; no new edge, no new crate.** `rho → shared-field`,
   `rho → shared-numfield`, `rho → shared-gf2m`, `rho → gnfs` all exist. The Semaev machinery lives
   entirely in `rho` (new `rho::semaev`); the `F_p[x]` resultant + multivariate-symmetric type live
   with it (or, if judged field-substrate-shaped at E.J.1, in a `shared` module — a placement call
   ratified at the ◆). No `Cargo.toml` edge changes; `cargo check --workspace` stays green.

The work splits at **three representation/contract-sharp seams**, **3 sessions** (top of the
ROADMAP's 2–3-session band, consistent with the project's documented ceiling-bias — G/D both landed
at or above their upper bands), at the boundaries between the polynomial substrate, the low-order
base cases, and the resultant recursion:

1. **E.J.1 — Polynomial substrate: `F_p[x]` resultant + multivariate/symmetric-polynomial type
   (Sonnet, Cat A).** The `F_p[x]` univariate resultant (greenfield — distinct from the `ℤ[x]` and
   `GF(2^m)` resultants) and the multivariate/symmetric-polynomial representation `S_m` lives in.
   **Freezes C-SemaevPoly** — the substrate the whole construction (and E.K) stands on. Confirms-
   and-records the F_p toy fixture and the symmetric-representation choice. *(The substrate-design
   session; the ◆ juncture is held at opus for the cost-of-wrong on this representation, but the
   per-row tier is Sonnet per the ROADMAP — E.J has no Opus-flagged session.)*

2. **E.J.2 — Base cases `S_2`, `S_3` + the vanishing relation (Sonnet, Cat B).** The explicit
   low-order summation polynomials: `S_2(X_1,X_2) = X_1 − X_2` and the symmetric `S_3(X_1,X_2,X_3)`
   that vanishes when three points sum to `∞`; the **vanishing KAT** (`S_m` vanishes on the
   x-coordinates of points summing to the identity, checked against the frozen `Curve` group law).
   **Consumes C-SemaevPoly. Freezes C-SemaevBase.**

3. **E.J.3 ◆ — Resultant recursion `S_m` + sub-track close (Sonnet, Cat B, `@architect`).** The
   elimination ladder `S_m = Res_X(S_{m-1}(X_1,…,X_{m-2},X), S_3(X_{m-1},X_m,X))` building the
   higher summation polynomials; the recursion-correctness + vanishing KATs at `S_4` (and `S_5` if
   cheap); the sub-track close. **Consumes C-SemaevBase, C-SemaevPoly. Freezes C-Semaev** — the
   surface E.K's index calculus (point decomposition / relation collection) consumes. Crosses the
   **E.J ◆ boundary** — the Semaev machinery ships, vanishing-verified, ready for E.K.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the index-calculus *solver*
— relation collection, the factor base, linear algebra over `Z/nZ`, the Gröbner/`msolve` step —
that is **E.K**; or building the GHS descent — that is E.H, frozen; or writing the Semaev *textbook
chapter* in MATHEMATICS.md — that is **T.E**, paired with E.W at the Track-E ◆; E.J.3 writes at most
a PEDAGOGY code-tour delta) and **rigidity** (forcing the binary-curve / descended-Jacobian Semaev
when the adjudicated F_p/Weierstrass target uses the frozen `rho::curve::Curve`; or re-deriving the
`ℤ[x]` resultant `shared-numfield` already ships when E.J needs an `F_p[x]` one; or coupling the
Semaev construction to the E.H GHS chain in-flight — that coupling is E.K's job, not E.J's).

**Scoping discipline.** E.J builds the Semaev summation polynomials at **demonstration fidelity**
(principle 1 — algorithmic content complete: the `F_p[x]` resultant, the multivariate-symmetric
representation, the `S_2`/`S_3` base cases, and the resultant recursion `S_m` all implemented
head-on) and **toy field sizes** (small `F_p`, `Uint<4>` ceiling; the polynomials computed for small
`m` — `S_2…S_4`, possibly `S_5`). It **amends no frozen contract** (`rho::curve::Curve` /
`AffinePoint` / the `Fp` field surface / the `ℤ[x]` and `GF(2^m)` resultants are all
consumed-or-untouched; E.J adds the `F_p[x]` resultant + the multivariate-symmetric type + the
`rho::semaev` module). It introduces **no index-calculus solver** (Semaev-only, verified
structurally by the vanishing relation — the solve is E.K). The correctness signal is the
**vanishing relation** (`S_m` vanishes on the x-coordinates of `m` points summing to `∞`, via the
frozen `Curve` group law) — **exactly self-checking, oracle-free** (no PARI/`msolve` dependency on
the green path; the lever-5 strength the header records). The **engineering-vs-mathematics
disconnect** (ROADMAP principle 4) is explicit: the toy `F_p`/`m` are a principle-4 boundary (Semaev
is crypto-scale-correct; only the *parameters* are toy), annotated, never presented as crypto-scale.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.J): "*E.J — Semaev summation polynomials. 2-3 sessions. Predecessor: G.A
(polynomial machinery). The combinatorial heart of Gaudry–Diem index calculus; mathematically
beautiful. Sonnet.*" E.J builds the **Semaev summation polynomials** `S_m(X_1,…,X_m)` — the symmetric
multivariate polynomials whose vanishing characterises `m`-tuples of points on an elliptic curve
that sum to the group identity. `S_m(x_1,…,x_m) = 0` precisely when there exist `y_i` such that
`P_i = (x_i, y_i)` are points on `E` with `P_1 + ⋯ + P_m = ∞`. This is the combinatorial primitive
at the heart of the Gaudry–Diem–Joux–Vitse index calculus (E.K): it turns "does this point
decompose over the factor base?" into "does this Semaev polynomial have a root with all coordinates
in the factor base?", the step that makes index calculus on elliptic/hyperelliptic curves work.

E.J's structural predecessor is **G.A** (the polynomial machinery — number-field / resultant
substrate, frozen, read) — *not* the GF(2^m) / hyperelliptic chain (E.F/E.G/E.H/E.I). The
adjudicated target is the **prime/Weierstrass curve `E/F_p`** (the classical Gaudry–Diem setting),
built on the frozen `rho::curve::Curve` + `shared-field` `Fp` surface. The central design tension is
that **E.J builds the primitive, not the solver**: the Semaev polynomials are *combinatorial input*
to index calculus (E.K); E.J's job is to construct them correctly and prove they vanish on the right
configurations, and the index-calculus speed-up is realised only when E.K runs relation collection
using them.

The sub-track decomposes into three conceptual units, each a session:

1. **The polynomial substrate (E.J.1).** The `F_p[x]` univariate resultant (greenfield — distinct
   from the `ℤ[x]` resultant `shared-numfield` ships and the `GF(2^m)` resultant `gf2m` ships) and
   the multivariate/symmetric-polynomial representation `S_m` lives in (also greenfield — no
   multivariate type exists). The substrate the construction stands on; reusable (E.K consumes).
   **Freezes C-SemaevPoly. (E.J.1.)**

2. **The base cases `S_2`, `S_3` (E.J.2).** The explicit low-order summation polynomials — `S_2 =
   X_1 − X_2` and the symmetric `S_3(X_1,X_2,X_3)` — and the vanishing relation verified against the
   frozen `Curve` group law. **Freezes C-SemaevBase. (E.J.2.)**

3. **The resultant recursion `S_m` + close (E.J.3 ◆).** The elimination ladder `S_m =
   Res_X(S_{m-1}, S_3)` building the higher summation polynomials; the recursion-correctness +
   vanishing KATs; the sub-track close. **Freezes C-Semaev. (E.J.3 ◆.)**

E.J is **solver-free** (the index-calculus DLP solver is E.K — E.J builds the Semaev primitive the
solver consumes, not the solver), **GHS-free** (it does not couple to the E.H descent — that
coupling is E.K's), and **chapter-free** (the Semaev textbook content is T.E, paired with E.W at the
Track-E ◆). Re-read this intent at the ◆ boundary to catch defocus (the index-calculus solver /
relation collection / linear algebra / `msolve`, the MATHEMATICS chapter) and rigidity (the
binary-curve Semaev when the adjudicated target is F_p; re-deriving the `ℤ[x]` resultant; coupling
to the GHS chain in-flight).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-15); raw
`cargo` is the only CI surface (unchanged from E.D…E.H). Oracle KATs are `#[ignore]`-gated only —
the exact form is `#[ignore = "PARI not installed; run manually when available"]`, used identically
in `rho/tests/ssa_kat.rs`, `rho/tests/mov_kat.rs`, `rho/tests/hyperelliptic_kat.rs`,
`rho/tests/ghs_kat.rs`, and `shared/padic/tests/log_kat.rs`. `/run-plan` re-discovers at preflight.
E.J **adds no new workspace edge and no new crate** (`rho` already depends on
`shared-field`/`shared-numfield`/`shared-gf2m`/`gnfs`; the Semaev machinery is a new `rho` module,
the `F_p[x]` resultant + multivariate type live with it or in `shared`), so the gate is a
**correctness + no-regression gate**:

- Each session's KATs are the primary correctness signal — fast, decisive, and **oracle-free**
  (lever 5): for E.J.1, the `F_p[x]` resultant identities (`Res(f,g) = 0 ⟺ gcd(f,g) ≠ 1`, symmetry
  up to sign, the multivariate-symmetric round-trip) for C-SemaevPoly; for E.J.2, the
  **vanishing relation** at `S_2`/`S_3` (`S_m(x_1,…,x_m) = 0` for points summing to `∞`, computed
  via the frozen `Curve` group law) for C-SemaevBase; for E.J.3, the **recursion correctness**
  (`S_m` built by `Res_X(S_{m-1}, S_3)` agrees with the direct construction at `S_4`) + the
  vanishing relation at `S_4` for C-Semaev. The vanishing relation is the decisive
  Semaev-correctness signal (the polynomial is right iff it vanishes on exactly the configurations
  summing to the identity) and it is **exactly self-checking** — no PARI / `msolve` on the green
  path.
- `cargo check --workspace` must stay green — **no edge change**, so no cycle risk. The `F_p[x]`
  resultant + multivariate type + `rho::semaev` are leaf additions.
- **The existing rho / gnfs / shared KATs must stay green** after the Semaev code lands — E.J adds
  new modules and changes no existing field / curve / resultant path, so the no-regression invariant
  is structurally easy to hold; `cargo test --workspace` is the guard.
- **No live oracle:** the green-path Semaev-correctness signal is the self-checking vanishing
  relation. An optional `msolve` / PARI cross-check (e.g. cross-checking `S_m`'s factorisation or a
  decomposition root) would follow the established `#[ignore = "PARI not installed; run manually when
  available"]` pattern; never on the green path. E.J introduces **no new live oracle** (principle 3).

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.J.1 | Semaev polynomial substrate: `F_p[x]` resultant + multivariate/symmetric-polynomial type | A | Sonnet | C-Fp (frozen — `shared-field::Fp<L>` read; no amend); `shared-numfield::resultant`/`subresultant_gcd` (read — the `ℤ[x]` Sylvester-resultant idiom to mirror over `F_p`); `gf2m::Poly::resultant` (read — the field-resultant idiom) | `rho/src/semaev/mod.rs` (new: module skeleton + the toy `F_p`/curve fixture + `SemaevError`), `rho/src/semaev/poly.rs` (new: `F_p[x]` univariate resultant + the multivariate/symmetric-polynomial type `S_m` lives in), `rho/src/lib.rs` (add `pub mod semaev;`), `rho/tests/semaev_kat.rs` (new: `F_p[x]` resultant identities, multivariate-symmetric round-trip KATs) |
| E.J.2 | Semaev base cases `S_2`, `S_3` + the vanishing relation | B | Sonnet | C-SemaevPoly (frozen E.J.1); `rho::curve::Curve`/`AffinePoint` (frozen, read — the group law the vanishing relation checks against); C-Fp (read) | `rho/src/semaev/base.rs` (new: `S_2 = X_1 − X_2`, the symmetric `S_3(X_1,X_2,X_3)`, the vanishing predicate), `rho/src/semaev/mod.rs` (add `pub mod base;`), `rho/tests/semaev_kat.rs` (extend: `S_2`/`S_3` vanish on points summing to `∞` via the frozen group law) |
| E.J.3 ◆ `@architect` | Semaev resultant recursion `S_m = Res_X(S_{m-1}, S_3)` + sub-track close | B | Sonnet | C-SemaevBase (frozen E.J.2); C-SemaevPoly (frozen E.J.1, read — the resultant + multivariate type); `rho::curve::Curve` (read — vanishing at `S_4`) | `rho/src/semaev/recursion.rs` (new: `semaev_poly(m) → S_m` via the resultant ladder + `pub use`), `rho/src/semaev/mod.rs` (add `pub mod recursion;` + `pub use`), `rho/tests/semaev_kat.rs` (extend: recursion agrees with direct `S_4`, `S_4` vanishing, sub-track-close suite; optional `#[ignore]` `msolve`/PARI sidecar) |

**Sequencing notes.** Strictly serial: **E.J.1 → E.J.2 → E.J.3.** E.J.1 lands the polynomial
substrate the construction stands on (the `F_p[x]` resultant + the multivariate-symmetric type);
E.J.2 the `S_2`/`S_3` base cases + the vanishing relation; E.J.3 the resultant recursion and close.
**One `@architect` marker** sits on the **E.J.3 ◆** (the boundary juncture ratifying the three frozen
contracts and confirming the Semaev machinery is vanishing-verified and E.K-ready before the
sub-track closes). *(Tradeoff named: E.J freezes two contracts before the ◆ — C-SemaevPoly (E.J.1),
C-SemaevBase (E.J.2) — but pages NO inline juncture there, mirroring the E.G/E.H/E.I calls. The
in-crate orthogonality (each later session consumes the earlier freeze immediately, where a wrong
shape fails the next session's vanishing KAT loudly) plus the single primary downstream consumer
(E.K, with its own Opus-flagged E.K.1) makes the early-catch insurance less valuable than a separate
inline fork would cost. The ◆ juncture is held at Opus (juncture-tier — the user's lever-3 override
on the representation E.K consumes), but separate inline forks are not bought.)*

**Why 3 sessions (the top of the ROADMAP's 2–3-session band, confirmed by ceiling-bias).** The split
is taken at three representation/contract-sharp seams:
- **One-line-commit-title corollary.** "Semaev polynomial substrate (`F_p[x]` resultant +
  multivariate type)", "Semaev base cases `S_2`/`S_3` + vanishing", and "Semaev resultant recursion
  `S_m` + close" are **three distinct commit titles** across two categories (A substrate ×1, B
  algorithm ×2).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the polynomial
  substrate, the base cases, the recursion. The **substrate↔base seam** (E.J.1↔E.J.2) is the
  deliberated split (see the shard-time decision below): the `F_p[x]` resultant + multivariate-
  symmetric representation (the substrate) is contract-distinct from the `S_2`/`S_3` base cases that
  populate it — freezing C-SemaevPoly at E.J.1 buys an early contract on the doubly-greenfield
  polynomial machinery E.K consumes.
- **Contract-sharp boundary.** E.J.1 **freezes** C-SemaevPoly; E.J.2 consumes it and **freezes**
  C-SemaevBase; E.J.3 consumes both and **freezes** C-Semaev. Each later session is meaningless
  without the earlier freeze — which is what licenses and bounds the 3-way split.

**The deliberated substrate↔base split (shard-time decision, user-adjudicated).** The substrate and
the `S_2`/`S_3` base cases *could* be one session (set up the polynomial machinery and the low-order
cases together, giving 2 sessions). The user chose **3: split at the substrate↔base seam**, matching
the ROADMAP's upper estimate and the project's documented ceiling-bias (G/D both landed at or above
their upper bands). The split buys an early contract freeze (C-SemaevPoly) on the doubly-greenfield
polynomial machinery (the `F_p[x]` resultant AND the multivariate-symmetric type — the
representation crux E.K depends on). **If E.J.1's substrate and E.J.2's base cases prove tightly
coupled** (the base cases are a thin instantiation of the substrate with no clean seam), the split is
artificial and **E.J.1/E.J.2 should re-merge** — a judgment E.J.1 can surface once the
representation shape is concrete (an additive-reshard discovery, not a silent merge). This is the one
place the 3-vs-2 sizing is genuinely uncertain until the substrate lands.

---

## Session detail

E.J.1 and E.J.2 are specified at near-full fidelity (the polynomial substrate and the base cases are
the design crux the whole construction — and downstream E.K — stand on). E.J.3 is a lower-fidelity
sketch, correct per the substrate-first discipline: it is crisply specified only after C-SemaevPoly
and C-SemaevBase freeze.

### E.J.1 — Semaev polynomial substrate: `F_p[x]` resultant + multivariate/symmetric-polynomial type (Sonnet, Cat A)

**Deliverable:** the polynomial substrate the Semaev construction stands on — **doubly greenfield**
(the survey confirmed: no `F_p[x]` univariate resultant exists — only `ℤ[x]` in `shared-numfield`
and `GF(2^m)` in `gf2m`; and no multivariate polynomial type exists anywhere). The pieces:
- **The `F_p[x]` univariate resultant** (`rho/src/semaev/poly.rs`): `Res(f, g) ∈ F_p` for `f, g ∈
  F_p[x]`, via the Euclidean/subresultant remainder sequence (mirroring the `gf2m::Poly::resultant`
  field-resultant idiom and the `shared-numfield` `ℤ[x]` Sylvester idiom). Zero iff `gcd(f,g) ≠ 1`.
- **The multivariate/symmetric-polynomial type** (`rho/src/semaev/poly.rs`): the representation `S_m`
  lives in — a multivariate polynomial over `F_p`, *symmetric* in its arguments (Semaev polynomials
  are symmetric). **The representation choice is the design crux** (dense multivariate vs
  elementary-symmetric-basis vs a hybrid). Over-specify (substrate rule): carry the operations E.K's
  point-decomposition step will need (evaluation at a partial assignment, the resultant-elimination
  one variable at a time, symmetric-reduction) if confidence is reasonable — adding them later is
  costlier.
- **The module skeleton + fixture** (`rho/src/semaev/mod.rs`): a `SemaevError` enum (the established
  reduction-attack idiom — cf. `rho::ssa::SsaError`, `rho::ghs::GhsError`), the toy `F_p` + Weierstrass
  curve fixture (a small prime `p`, a curve `y² = x³ + ax + b` with known small-order points).
- **Confirm-and-record (the load-bearing E.J.1 acts):** (a) **the symmetric representation** — record
  the chosen multivariate-symmetric representation and why (the property the vanishing relation and
  the resultant recursion exploit); (b) **the F_p fixture** — choose a toy `p` and curve with enough
  small points to exhibit `S_3`/`S_4` vanishing non-vacuously (`Uint<4>` ceiling — a principle-4
  boundary, annotated); (c) **the module home** — `rho::semaev` for the construction; the `F_p[x]`
  resultant + multivariate type live with it unless judged field-substrate-shaped enough for a
  `shared` module (a placement call ratified at the ◆, no edge consequence either way).

Consumes C-Fp (frozen — `shared-field::Fp<L>` read; **no amend**), `shared-numfield::resultant` /
`subresultant_gcd` (read — the `ℤ[x]` idiom to mirror over `F_p`), `gf2m::Poly::resultant` (read —
the field-resultant idiom). **Freezes C-SemaevPoly.**

**KAT** (`rho/tests/semaev_kat.rs` + inline unit tests): over the toy `F_p` fixture: **resultant
zero-iff-common-factor** (`Res(f,g) = 0 ⟺ gcd(f,g) ≠ 1`); **resultant symmetry** (`Res(f,g) =
±Res(g,f)`); **resultant of coprime quadratics is nonzero**; **multivariate-symmetric round-trip**
(a symmetric polynomial round-trips through the representation; evaluation at a permutation of its
arguments is invariant). **Verify gate:** `cargo test --workspace` green; `cargo check --workspace`
green (leaf additions, no edge change); existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **The `F_p[x]` resultant is distinct from the `ℤ[x]` and `GF(2^m)`
ones** — a `@build` agent reaching for `shared-numfield::resultant` gets a `ℤ[x]` Sylvester-matrix
resultant (wrong ring); the construction needs the *field* resultant over `F_p` (the
`gf2m::Poly::resultant` field idiom, ported to `F_p`). (2) **The symmetric representation is the
crux** — `S_m` is symmetric, and the resultant recursion (E.J.3) eliminates one variable at a time;
a representation that does not make symmetric-reduction and partial-evaluation cheap fights the
recursion. (3) **Over-specify the substrate** — carry the partial-evaluation and one-variable-
elimination operations E.K needs (the point-decomposition step assigns factor-base x-coordinates and
asks for roots). (4) **Module placement** — `rho::semaev` for the construction; the resultant +
multivariate type live with it unless field-substrate-shaped (the principled home, ratified at the
◆). (5) **`Uint<4>` ceiling** — the source curve is hardcoded `Uint<4>` (the C1 ceiling the ROADMAP
flags for E.K); E.J's toy `p` fits it; if E.K later needs a wider field, that is the C1-widening
discovery, not E.J's concern.

**Deferred:** the base cases (E.J.2); the recursion (E.J.3); the index-calculus solver (E.K); the
MATHEMATICS chapter (T.E at the Track-E ◆).

### E.J.2 — Semaev base cases `S_2`, `S_3` + the vanishing relation (Sonnet, Cat B)

**Deliverable:** the explicit low-order summation polynomials and the vanishing relation that makes
them correct. Near-full fidelity (the base cases anchor the recursion). The pieces:
- **`S_2`** (`rho/src/semaev/base.rs`): `S_2(X_1, X_2) = X_1 − X_2` — two points `P_1, P_2` sum to
  `∞` iff `P_2 = −P_1`, which for the x-coordinates means `x_1 = x_2` (the negation `−(x,y) = (x,−y)`
  fixes `x`). The degenerate but foundational base case.
- **`S_3`** (`rho/src/semaev/base.rs`): the symmetric `S_3(X_1, X_2, X_3)` that vanishes when three
  points `P_1 + P_2 + P_3 = ∞`. This is the genuine first summation polynomial — derived from the
  curve's group law (the collinearity / addition relation), symmetric in `X_1, X_2, X_3`, of degree
  2 in each. The seed the resultant recursion (E.J.3) builds all higher `S_m` from.
- **The vanishing predicate** (`rho/src/semaev/base.rs`): given `m` points on the frozen `Curve`,
  compute `Σ P_i` via the group law and assert `S_m(x_1, …, x_m) = 0 ⟺ Σ P_i = ∞`.

Consumes C-SemaevPoly (frozen E.J.1), `rho::curve::Curve` / `AffinePoint` (frozen, read — the group
law the vanishing relation checks against), C-Fp (read). **Freezes C-SemaevBase.**

**KAT** (`rho/tests/semaev_kat.rs`, extended): over the toy `F_p` curve fixture: **`S_2` vanishing**
(`S_2(x_1, x_2) = 0 ⟺ P_2 = −P_1`); **`S_3` vanishing** (`S_3(x_1, x_2, x_3) = 0` for triples with
`P_1 + P_2 + P_3 = ∞`, computed via the frozen group law; nonzero for triples that do not sum to
`∞`); **`S_3` symmetry** (invariant under permuting its three arguments); **`S_3` degree** (degree 2
in each variable). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The vanishing relation is the correctness signal** — `S_m` is
correct iff it vanishes on *exactly* the x-coordinate tuples of points summing to `∞`; the vanishing
KAT (checked against the frozen `Curve` group law) is the decisive guard and it is **exactly
self-checking** (no oracle — the lever-5 strength). (2) **`S_3` is derived from the group law, not
guessed** — the symmetric `S_3` encodes the curve's addition/collinearity relation; a `@build` agent
must derive it from `y² = x³ + ax + b` and the group law, not transcribe a formula for the wrong
curve form. (3) **Symmetry is load-bearing** — `S_3` (and all `S_m`) are symmetric; a non-symmetric
construction breaks the recursion. (4) **The substrate↔base seam check** — if the base cases are a
thin instantiation of the C-SemaevPoly substrate (no genuine seam), this is the loud signal to
surface the E.J.1/E.J.2 merge.

**Deferred:** the recursion (E.J.3); the index-calculus solver (E.K); the MATHEMATICS chapter (T.E).

### E.J.3 ◆ — Semaev resultant recursion `S_m = Res_X(S_{m-1}, S_3)` + sub-track close (Sonnet, Cat B, `@architect`)

**Deliverable:** the resultant recursion that builds the higher summation polynomials, and the
sub-track close. Lower-fidelity sketch (crisp after C-SemaevBase freezes):
- **The recursion** (`rho/src/semaev/recursion.rs`): `semaev_poly(m) → S_m` via the elimination
  ladder `S_m(X_1, …, X_m) = Res_X(S_{m-1}(X_1, …, X_{m-2}, X), S_3(X_{m-1}, X_m, X))` — eliminate a
  shared variable `X` between `S_{m-1}` and `S_3` using the frozen C-SemaevPoly `F_p[x]` resultant.
  Builds `S_4` (and `S_5` if cheap) from `S_3`.
- **Recursion correctness** (`rho/src/semaev/recursion.rs` + KAT): `S_4` built by the recursion
  agrees with the direct construction (the vanishing relation holds for `S_4`); the recursion
  preserves symmetry and the expected degree growth.
- **Sub-track-close KAT suite** (`rho/tests/semaev_kat.rs`, extended): `S_4` vanishing, recursion-vs-
  direct agreement, and (optional) an `msolve`/PARI cross-check on a Semaev-root decomposition,
  `#[ignore]`-gated.

Consumes C-SemaevBase (frozen E.J.2), C-SemaevPoly (frozen E.J.1, read — the `F_p[x]` resultant +
multivariate type), `rho::curve::Curve` (read — vanishing at `S_4`). **Freezes C-Semaev.**

**KAT (primary correctness signal):** over the toy `F_p` curve fixture: **recursion correctness**
(`S_4 = Res_X(S_3, S_3)` agrees with the direct construction); **`S_4` vanishing**
(`S_4(x_1, …, x_4) = 0 ⟺ P_1 + P_2 + P_3 + P_4 = ∞`, via the frozen group law — the decisive Semaev-
correctness signal); **symmetry preservation** (`S_4` is symmetric); **degree growth** (`S_4` has
the expected degree from `m`). Optional `msolve`/PARI cross-check (`#[ignore = "PARI not installed;
run manually when available"]`). **Verify gate:** `cargo test --workspace` green; existing
rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **Semaev-only — NO index-calculus solver here** — E.J builds `S_m`
and verifies the vanishing relation; it does NOT run relation collection / a factor base / linear
algebra over `Z/nZ` / the Gröbner/`msolve` step (that is E.K, the central scope boundary). A
`@build` agent implementing point decomposition over a factor base is defocus. (2) **The vanishing
relation is the correctness signal** — `S_m` is correct iff it vanishes on exactly the configurations
summing to `∞`; the vanishing KAT (known points, computed sum) is the green-path guard, the
`msolve`/PARI cross-check the optional sidecar. (3) **This is the E.J ◆ boundary** — re-read the
Purpose intent and verify the Semaev machinery is complete (substrate + base cases + recursion all
present and vanishing-verified) and **E.K-ready** (C-Semaev exposes what E.K's index calculus
consumes — `semaev_poly(m)`, partial evaluation / one-variable elimination for the point-
decomposition step), and that E.J stayed solver-free / GHS-free / chapter-free. (4) **No index
calculus and no GHS coupling** — E.K runs the index calculus and couples to the descended curve; E.J
builds the F_p Semaev primitive. (5) **No MATHEMATICS chapter** — the Semaev textbook content is
T.E, paired with E.W at the *Track-E* ◆; E.J.3 writes at most a PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.J.3 ◆ to confirm: (1) the Semaev machinery is complete and composes (substrate → base cases →
recursion all present and vanishing-verified — the `F_p[x]` resultant identities, the `S_2`/`S_3`/`S_4`
vanishing relations, and the recursion-vs-direct agreement all pass); (2) C-Semaev exposes what E.K
descends into (`semaev_poly(m)`, the partial-evaluation / one-variable-elimination operations the
point-decomposition step uses) so E.K can build the index-calculus solver without amending the
Semaev surface — the substrate-readiness defense; (3) E.J stayed in scope — no index-calculus solver
(E.K), no GHS coupling, no MATHEMATICS chapter (T.E), the Semaev polynomials are a primitive verified
by the vanishing relation, not a solve; (4) the principle-4 boundary (toy `F_p`, small `m`, the
`Uint<4>` ceiling; Semaev is crypto-scale-correct) is recorded, not silently presented as
crypto-scale; (5) **the representation / fixture / substrate-base-seam resolutions** — confirm the
chosen multivariate-symmetric representation, the toy `F_p` fixture exhibited non-vacuous `S_3`/`S_4`
vanishing, and the E.J.1/E.J.2 split held (or was merged via surfaced additive-reshard). **Also:
reconcile the outstanding static-frame ROADMAP debt** carried + compounded from the E.I and E.H ◆ —
(a) the Progress table is stale by **four** completed sub-tracks (E.F, E.G, E.H, E.I; table still
shows "Done ~13 (E.A–E.E)"); (b) the Remaining table lists the now-complete E.F/E.G/E.H/E.I; (c) the
**E.I-before-E.H sequencing correction** (the Remaining table listed E.H before E.I — dependency-
inverted; E.I shipped first, E.H followed it, both now done) should be recorded; and (d) **strike
E.J** from Remaining on completion. *(Per the E.H ◆ digest, this debt was owed but not yet written
into the ROADMAP — E.H closed without reconciling it; the E.J ◆ inherits it.)* One-shot findings;
does not implement. Held at **Opus** per the header (juncture-tier — the user's lever-3 override on
the representation E.K consumes, despite the strong lever-5 self-checking signal).

---

## Cross-session contracts

E.J **freezes three** contracts (C-SemaevPoly at E.J.1, C-SemaevBase at E.J.2, C-Semaev at E.J.3)
and **amends no prior frozen contract** (C-Fp / `rho::curve::Curve` / `AffinePoint` / the `ℤ[x]` and
`GF(2^m)` resultants / the GHS / hyperelliptic / binary-curve surfaces are all consumed-or-untouched).
E.J adds the `F_p[x]` resultant + the multivariate-symmetric type + the `rho::semaev` module — all
**additive**, no trait amendment.

### C-SemaevPoly — the polynomial substrate: `F_p[x]` resultant + multivariate/symmetric-polynomial type (compiler- + test-enforced) — *to be frozen at E.J.1*

**Defined in:** E.J.1 (`rho/src/semaev/poly.rs` + `rho/src/semaev/mod.rs`). **Consumed by:** E.J.2
(the base cases live in the multivariate type), E.J.3 (the recursion uses the `F_p[x]` resultant to
eliminate a variable); **downstream: E.K** (the index-calculus point-decomposition step evaluates
`S_m` at partial factor-base assignments and eliminates variables). Compiler- + test-enforced.
Exposes: the `F_p[x]` univariate `resultant(f, g) → F_p` (zero iff `gcd ≠ 1`); the
multivariate/symmetric-polynomial type `S_m` (evaluation, partial-assignment evaluation, one-variable
resultant-elimination, symmetric-reduction — **over-specified** for E.K's point decomposition if
confidence is reasonable); the `SemaevError` enum; the toy `F_p`/curve fixture. **Invariants:** the
`F_p[x]` resultant is the *field* resultant (distinct from `shared-numfield`'s `ℤ[x]` Sylvester
resultant and `gf2m`'s `GF(2^m)` resultant); `S_m` is symmetric in its arguments. *Exact symmetric
representation (dense vs elementary-symmetric-basis) and whether the resultant + type ship in
`rho::semaev` or a `shared` module ratified at the E.J.3 ◆.*

### C-SemaevBase — the base summation polynomials `S_2`, `S_3` + the vanishing relation (compiler- + test-enforced) — *to be frozen at E.J.2*

**Defined in:** E.J.2 (`rho/src/semaev/base.rs`). **Consumed by:** E.J.3 (the recursion builds all
higher `S_m` from `S_3`). Compiler- + test-enforced. Exposes: `S_2(X_1, X_2) = X_1 − X_2`; the
symmetric `S_3(X_1, X_2, X_3)` (degree 2 in each variable, derived from the `Curve` group law); the
vanishing predicate (`S_m(x_1, …, x_m) = 0 ⟺ Σ P_i = ∞`, checked against the frozen group law).
**Invariants:** `S_3` is symmetric and derived from `y² = x³ + ax + b` + the group law (not
transcribed for the wrong curve form); the vanishing relation is **exactly self-checking** (no
oracle). *Exact `S_3` form (the explicit symmetric polynomial) frozen here; the recursion seed it
provides is read by E.J.3.*

### C-Semaev — the Semaev summation polynomials `S_m` via the resultant recursion (compiler- + test-enforced) — *to be frozen at E.J.3 ◆*

**Defined in:** E.J.3 (`rho/src/semaev/recursion.rs`). **Consumed by:** **E.K** (the Gaudry–Diem–
Joux–Vitse index-calculus point-decomposition / relation-collection step — the highest-stakes
consumer, Opus-flagged E.K.1), **E.W** (the cross-attack benchmark table). Compiler- + test-enforced.
Exposes: `semaev_poly(m) → S_m` via the recursion `S_m = Res_X(S_{m-1}, S_3)`. **The frozen
invariant:** **the vanishing relation** — `S_m(x_1, …, x_m) = 0 ⟺ ∃ y_i: P_i = (x_i, y_i) ∈ E ∧
Σ P_i = ∞`; the recursion agrees with the direct construction; `S_m` is symmetric. **E.J freezes the
Semaev primitive; the index-calculus DLP solve consuming it is E.K** (the scope boundary). **The
Semaev polynomials are a combinatorial primitive verified by the vanishing relation, NOT a solver.**
*Exact recursion signature (the `m` range built, the partial-evaluation API E.K consumes) ratified
at the ◆.*

### Frozen contracts read by E.J (consumed, not amended)

- **C-Fp (`shared-field::Fp<L>` surface)** — the prime-field arithmetic (`add`/`mul`/`neg`/`inv`/
  `pow`, threaded `poly`/modulus per the field idiom) consumed by the `F_p[x]` resultant, the
  multivariate type, and the vanishing relation. Read. **Unchanged — E.J amends no field contract.**
- **`rho::curve::Curve` / `AffinePoint` (frozen surface)** — the short-Weierstrass curve
  `y² = x³ + ax + b` over `F_p` (hardcoded `Uint<4>` — the C1 ceiling), point addition / doubling /
  negation — the **geometry the vanishing relation checks against** (`S_m` vanishes iff `Σ P_i = ∞`,
  computed via this group law). **Read; NOT amended.** *(Open: `Curve` is hardcoded `Uint<4>`; E.J's
  toy `p` fits it; if E.K later needs a wider field, that is the C1-widening discovery, never E.J's
  concern.)*
- **`shared-numfield::resultant` / `subresultant_gcd`** — the `ℤ[x]` Sylvester-matrix resultant +
  subresultant PRS (the G.A polynomial machinery) — **read for the resultant idiom to mirror over
  `F_p`** (E.J's `F_p[x]` resultant is the *field* analogue, not this `ℤ[x]` one). Untouched.
- **`gf2m::Poly::resultant`** — the `GF(2^m)` field resultant — **read for the field-resultant
  idiom** the `F_p[x]` resultant ports. Untouched.
- **`rho::ghs` / `rho::ssa` reduction idioms** — the `GhsError` / `SsaError` enum + fixture + module
  shape (the attack-module template E.J's `semaev` module mirrors structurally). Read for the
  pattern; untouched.

### Workspace edges (no new edge, no new crate)

- **No new edge.** `rho` already depends on `shared-field`, `shared-numfield`, `shared-gf2m`, `gnfs`,
  `shared-bigint`, `shared-numth`, `shared-padic`. The Semaev machinery is a new module in the
  existing `rho` crate (`rho::semaev`); the `F_p[x]` resultant + multivariate type live with it (or,
  if field-substrate-shaped, in a `shared` module — no edge consequence either way, `rho` already
  depends on the `shared` crates). No `Cargo.toml` changes; `cargo check --workspace` stays green
  with no cycle risk. *(If E.J found it must change a frozen trait surface — it should not, it only
  adds the `semaev` module and the resultant/type — that would be a discovery surfaced at the ◆,
  never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.J.3 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.J.1 | Semaev polynomial substrate: `F_p[x]` resultant + multivariate type | done | 937cf82 | C-SemaevPoly (frozen) |
| E.J.2 | Semaev base cases `S_2`, `S_3` + the vanishing relation | done | 9208ffb | C-SemaevBase (frozen) |
| E.J.3 ◆ | Semaev resultant recursion `S_m` + sub-track close | pending | — | C-Semaev (to freeze) |

Contracts frozen before this sub-track: the prime-field surface (C-Fp — read by E.J, unchanged), the
prime-field curve + ECDLP surface (`rho::curve::Curve`/`AffinePoint` — read as the Semaev geometry,
unchanged), the G.A polynomial machinery (`shared-numfield` resultant/subresultant — read as the
resultant idiom), the GF(2^m) field + poly surface (C-F2m/C-GF2mPoly — `gf2m::Poly::resultant` read
as the field-resultant idiom), and the full Track-E GF(2^m) chain (C-BinaryCurve/C-Koblitz from E.G,
C-HyperCurve/C-Jacobian from E.I, C-Subfield/C-DescentAlgebra/C-GHSCurve/C-DescentMap/C-GHSDescent
from E.H — none consumed by E.J, which targets the F_p setting). This sub-track **freezes three new
contracts** (C-SemaevPoly, C-SemaevBase, C-Semaev), serving the downstream **E.K** (Gaudry–Diem–
Joux–Vitse index calculus — consumes the Semaev polynomials for the point-decomposition / relation-
collection step, plus G.B scoring + G.E linear algebra) and **E.W** (cross-attack benchmarks), and
building the **combinatorial-primitive** half of the small-characteristic index-calculus attack (E.J
builds the Semaev polynomials; E.K solves with them).

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.J builds the Semaev polynomials on the frozen `rho::curve::Curve` F_p surface + a new
  polynomial substrate — internal-continue (confirmed by survey).** All Semaev code greenfield (zero
  `semaev`/`summation` hits). The `F_p[x]` resultant and the multivariate-symmetric type are
  greenfield (no prime-field `Poly`, no multivariate type — only `ℤ[x]` and `GF(2^m)` resultants
  exist). A discovery that the construction needs a curve / field operation the frozen surface lacks
  is an **additive amend** surfaced at the ◆ — not a silent patch.

- **The Semaev polynomials are a PRIMITIVE, not a solve — implementing the index-calculus solver is a
  defocus failure (the central scope boundary).** E.J builds `S_m` and verifies the vanishing
  relation; the index-calculus *solve* (point decomposition over a factor base, relation collection,
  linear algebra over `Z/nZ`, the Gröbner/`msolve` step) is **E.K**, consuming E.J + G.B + G.E. A
  `@build` agent implementing index calculus or relation collection in E.J is defocus.
  **Internal-continue → corrected** (the vanishing KAT is the green-path terminus; the solve is
  delegated).

- **The `F_p[x]` resultant is distinct from the `ℤ[x]` and `GF(2^m)` resultants — the resultant-ring
  trap.** `shared-numfield::resultant` is `ℤ[x]` (Sylvester-matrix, integer); `gf2m::Poly::resultant`
  is `GF(2^m)`. E.J needs the *field* resultant over `F_p` (the `gf2m` field idiom ported to `F_p`).
  A `@build` agent reaching for `shared-numfield::resultant` builds over the wrong ring; the
  resultant-identity KATs are the loud signal. **Internal-continue → corrected.**

- **The Semaev polynomials are symmetric, and `S_3` is derived from the group law, not guessed — the
  representation trap.** `S_m` is symmetric in its arguments and `S_3` encodes the curve's
  addition/collinearity relation (derived from `y² = x³ + ax + b` + the group law). A `@build` agent
  transcribing a formula for the wrong curve form, or choosing a non-symmetric representation, breaks
  the recursion. The symmetry + vanishing KATs are the guard. **Internal-continue → corrected.**

- **The vanishing relation is the correctness signal and is exactly self-checking — no oracle on the
  green path (lever-5 strength, principle-3).** `S_m(x_1,…,x_m) = 0 ⟺ Σ P_i = ∞`, computed via the
  frozen `Curve` group law — a fast, decisive, oracle-free KAT (like E.I's group axioms, *unlike*
  E.H's oracle-leaning log-preservation). An optional `msolve`/PARI cross-check is an `#[ignore]`
  sidecar (the established `#[ignore = "PARI not installed; run manually when available"]` pattern).
  E.J introduces no new live oracle. *(Lever-5 note: this strong, self-checking signal is what
  licensed a `sonnet` juncture-tier; the user overrode to opus on lever 3 — the representation E.K
  consumes — so the green-path strength stands but the boundary fork is held at Opus.)*

- **Target is F_p / Weierstrass, NOT the binary-curve / descended-Jacobian setting — the
  field-target rigidity guard.** E.J builds Semaev over `E/F_p` (the classical Gaudry–Diem setting,
  predecessor G.A only); it does NOT couple to the E.H GHS descent or the binary curve. A `@build`
  agent building the binary-curve Semaev, or coupling to the descended Jacobian, is rigidity (forcing
  the wrong target). The coupling to the descended curve is **E.K's** job. **Internal-continue
  (F_p target).**

- **The substrate↔base seam (E.J.1↔E.J.2) may be artificial — surface a merge if the base cases are a
  thin instantiation.** The 3-vs-2 sizing splits at the substrate↔base seam (buying an early
  C-SemaevPoly freeze). **If E.J.1's substrate and E.J.2's `S_2`/`S_3` prove tightly coupled (the
  base cases are a thin instantiation with no genuine reusable substrate seam), the split is
  artificial and E.J.1/E.J.2 should merge** — surfaced as an additive-reshard at the ◆ (or by E.J.1
  once the representation shape is concrete), never a silent merge. **Additive-reshard if the seam
  proves false.**

- **No index-calculus solver / relation collection / `msolve` in E.J (defocus / scope clarity).** The
  DLP *attack* using the Semaev polynomials (point decomposition over a factor base, relation
  collection, linear algebra over `Z/nZ`, the Gröbner/`msolve` step) is **E.K** (consuming E.J + G.B
  scoring + G.E linear algebra). E.J exposes the primitive; the attack consumes it. A `@build` agent
  implementing any of these in E.J is defocus.

- **No MATHEMATICS.md chapter in E.J (defocus / scope clarity).** The Semaev / summation-polynomial
  textbook content is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing),
  not at the E.J sub-track ◆. E.J.3 writes at most a PEDAGOGY code-tour delta.

- **Toy `F_p` + small `m` + the `Uint<4>` ceiling only (scope clarity).** E.J fixes a small prime
  `p` (fitting the `Uint<4>` curve ceiling) and computes `S_m` for small `m` (`S_2…S_4`, possibly
  `S_5`). The toy sizes are a principle-4 boundary — Semaev is crypto-scale-correct; only the
  *parameters* are toy. Presenting any as crypto-scale is a documentation defect (internal-continue
  → corrected). *(The `Uint<4>` ceiling is the C1 boundary the ROADMAP flags for E.K; if E.K needs a
  wider field for its smoothness/decomposition step, that is the C1-widening discovery at E.K, not
  E.J.)*

- **The polynomial substrate's home is `rho::semaev` (module-placement call, ratified at the ◆).** The
  Semaev construction is attack-shaped and lives in `rho::semaev`. The `F_p[x]` resultant +
  multivariate type live with it unless judged field-substrate-shaped enough for a `shared` module —
  a ◆ ratification decision, not a blocker (no edge consequence either way — `rho` already depends on
  the `shared` crates).

- **Static-frame ROADMAP debt (reconcile at the E.J ◆, does NOT block E.J) — carried + compounded
  from the E.I and E.H ◆.** The ROADMAP Progress subsection is stale by **four** completed sub-tracks
  (E.F, E.G, E.H, E.I; table shows "Done ~13 (E.A–E.E)"); the Remaining table lists the now-complete
  E.F/E.G/E.H/E.I; and the Remaining table listed **E.H before E.I** (dependency-inverted — E.I
  shipped first, E.H followed). The E.I ◆ digest recorded this as owed; the E.H ◆ digest re-recorded
  it as still owed (E.H closed without writing it into the ROADMAP). The E.J ◆ should: update
  Progress (Track E Done → E.A–E.I, ~22), strike E.F/E.G/E.H/E.I from Remaining, record the
  E.I-before-E.H correction, and strike E.J on completion. Not an implementation concern.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.J, "*Semaev summation polynomials … The combinatorial heart of
  Gaudry–Diem index calculus; mathematically beautiful. Sonnet.*"; predecessor **G.A** polynomial
  machinery; the design statement's principles 1 + 3 + 4; the C1 `Uint<4>`-ceiling note flagged for
  E.K's Semaev-point smoothness) and this PLAN before any session. **NOTE: the ROADMAP Progress /
  Remaining tables are stale (E.F, E.G, E.H, E.I done; all four still listed as remaining) AND listed
  E.H before E.I (dependency-inverted — E.I shipped first); reconcile at the E.J ◆ (debt carried from
  both the E.I and E.H ◆, unreconciled).**
- Read the **templates to mirror**: `rho/src/ssa/mod.rs` + `rho/src/ghs/mod.rs` (the attack-module
  idiom — `SsaError`/`GhsError` enum + toy fixture + module skeleton — E.J's `semaev` module mirrors
  this); `rho/src/curve/mod.rs` (the **frozen** short-Weierstrass `Curve`/`AffinePoint` the vanishing
  relation checks against — read for `add`/`double`/`negate`, NOT to amend); `shared/numfield/src/
  resultant.rs` (the `ℤ[x]` Sylvester/subresultant resultant idiom to mirror over `F_p` — read for
  the *shape*, not the ring); `shared/gf2m/src/poly.rs` (the `GF(2^m)` field `resultant` the `F_p[x]`
  one ports — `poly.rs:425`); `rho/tests/{ssa_kat,ghs_kat,hyperelliptic_kat}.rs` (the attack-KAT +
  `#[ignore]` oracle idioms E.J.3 mirrors).
- **Register:** E.J is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New modules `rho/src/semaev/{mod,poly,base,recursion}.rs` (the Semaev
  construction) and new KATs in `rho/tests/semaev_kat.rs`.
- **Tier routing:** **all three sessions are Sonnet `@build`** (E.J has no Opus-flagged session per
  the ROADMAP — it is marked Sonnet throughout). E.J.3 carries the **◆ `@architect` juncture** (page
  `@plan-juncture`) ratifying C-SemaevPoly/C-SemaevBase/C-Semaev and confirming E.K-readiness before
  the sub-track closes. juncture-tier (header) is **opus** — **a deliberate user override of the
  lever-5 `sonnet` recommendation**, on lever 3 (the multivariate-symmetric representation E.J
  freezes is consumed by the Opus-flagged E.K; the user judges the cost-of-wrong worth paying Opus at
  the boundary to de-risk, mirroring the E.I precedent). The default tuning-law read was `sonnet`
  (strong lever-5 self-checking vanishing KAT + moderate lever-4 correctness-criticality + no
  Opus-flagged session); the override is recorded so the disagreement stays visible.
- **Invariants to preserve:** **The Semaev polynomials are a PRIMITIVE, not a solve** (no
  index-calculus solver / relation collection / linear algebra over `Z/nZ` / `msolve` — those are
  E.K; the vanishing KAT is the terminus). **The vanishing relation is the correctness signal**
  (`S_m(x_1,…,x_m) = 0 ⟺ Σ P_i = ∞` via the frozen `Curve` group law; exactly self-checking, no
  oracle). **The `F_p[x]` resultant is the field resultant** (NOT the `ℤ[x]` Sylvester one in
  `shared-numfield`, NOT the `GF(2^m)` one in `gf2m`). **`S_m` is symmetric and `S_3` is derived from
  the group law** (not transcribed for the wrong curve form). **The target is F_p / Weierstrass** (NOT
  the binary curve / descended Jacobian — that coupling is E.K's). **E.J consumes the frozen `Curve`
  + `Fp` surfaces unchanged** (adds the `F_p[x]` resultant + multivariate type + the `rho::semaev`
  module). **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy `F_p` + small `m` + the `Uint<4>`
  ceiling only; no new live oracle.
- **No new edge, no new crate (load-bearing for E.J).** `rho` already depends on `shared-field`/
  `shared-numfield`/`shared-gf2m`/`gnfs`; the Semaev machinery is a new `rho::semaev` module (the
  `F_p[x]` resultant + multivariate type live with it or in a `shared` module — no edge consequence).
  `cargo check --workspace` stays green with no cycle risk. If E.J finds it must change a frozen trait
  surface (it should not — it only adds the `semaev` module and the resultant/type), that is a
  discovery surfaced at the ◆.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  doubly-greenfield polynomial substrate — an `F_p[x]` resultant + a multivariate/symmetric type —
  then base cases, then a resultant recursion) is **new to this project** (no prior multivariate /
  symmetric-polynomial machinery exists, and the Semaev construction has no in-repo precedent). Per
  the unproven-shard-pattern guidance, halt at each boundary for a human glance until the pattern
  proves out. *(Tradeoff vs autonomous: `halt-at-boundaries` trades velocity for a per-boundary check
  on a novel pattern — the polynomial substrate (E.J.1) is the design crux E.K consumes, and a wrong
  symmetric-representation shape is a retrofit. If E.J.1 lands cleanly and its resultant + symmetry
  KATs confirm the substrate shape, fall back to autonomous for E.J.2–E.J.3. The substrate↔base seam
  uncertainty (E.J.1↔E.J.2) is itself a reason to halt at the E.J.1 boundary — that is where a
  merge-back would be surfaced.)*
