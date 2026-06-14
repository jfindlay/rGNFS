<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.G — Binary curves + Koblitz automorphism: the first GF(2^m) curve consumer)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) on the
C-BinaryCurve surface, against a strong lever 5 that would otherwise license an opt-down.** E.G is the
first consumer of the frozen GF(2^m) field substrate (C-F2m), and the **binary-curve surface it
freezes (C-BinaryCurve) is descended into by E.H — "the most mathematically intricate single attack
in the project" (ROADMAP:357-358).** The curve representation (point coordinates, the
`y²+xy=x³+ax²+b` group law, point decompression via the half-trace) bounds what GHS/Weil descent
reads; getting the curve surface wrong is an expensive retrofit through the single hardest attack.
Lever 5 is strong and fast (binary-curve arithmetic is *exactly* checkable: point-on-curve, group
axioms, `k·G = Q` round-trip, the Frobenius-orbit law `τ(x,y)=(x²,y²)` with `τ^m = id`, a
naive-vs-optimized field cross-check inherited from C-F2m, and an optional PARI `ellinit`/`elllog`
`#[ignore]` sidecar) and *would* license `juncture-tier: sonnet` in isolation — **the user judged the
C-BinaryCurve design-error cost (lever 3, E.H descends from it) decisive and held the ◆ juncture at
Opus**, mirroring the E.D/E.E/E.F calls where a substrate-adjacent juncture feeding a high-stakes
downstream consumer is held up despite exactly-checkable KATs. *(E.G carries NO session-level Opus
flag — the ROADMAP Opus-flagged-sessions table lists E.F.1/E.H.1/E.K.1 but not any E.G session; E.G.1
is **Sonnet `@build`**, not an Opus `@architect` session like E.F.1 was. The juncture-tier governs
only the paged `@plan-juncture` fork at the ◆, not the session tier.)*

Last rewrite: **E.F ◆ boundary crossed; the GF(2^m) field substrate is complete and binary-
curve-ready** (E.F.1 `2ca3061` froze C-F2m, E.F.2 `c512c2d` filled inv/div, E.F.3 `84f6d07`
normal-basis, E.F.4 ◆ `51c8c8d` froze C-F2mOpt; ledger `dbd86b0`). The E.F ◆ Action-frame digest
(2026-06-13) recorded the `pclmulqdq` omit-not-gate call (the `unsafe_code = "forbid"` crate lint
makes the intrinsic cost real at zero toy-scale demonstration value) and confirmed C-F2m carries the
curve-facing over-specification (`trace`/`solve_quadratic`) that E.G now consumes. Per the sequencing
order (ROADMAP "Remaining projected sessions": E.F → **E.G** → E.H → …), the next un-started sub-track
is **E.G — Binary curves + Koblitz automorphism**, the first GF(2^m)-curve consumer, which re-runs
Pollard rho over binary curves as the baseline for E.H's descent benchmarks.

**Static-frame debt outstanding (surfaced at this boundary — does NOT block E.G sharding).** The E.F
PLAN flagged that the ROADMAP **Progress** subsection (~line 168) and **Remaining projected sessions**
table were stale by two completed sub-tracks (E.D, E.E) as of E.F sharding. The E.F ◆ digest
(2026-06-13) recorded "ROADMAP true-boundary reconciliation owed as housekeeping (not a blocker)" —
**this reconciliation, plus striking the now-complete E.F row from the Remaining table, is still
owed.** The Progress table still shows Track E "Done ~13 (E.A–E.E)" and the Remaining table still
lists E.F as remaining (it is done). This is a **roadmap-frame reconciliation owed at the E.G ◆**
(paired with this PLAN's close), not an E.G implementation concern — flagged here so it is not lost.
*(Surfaced as a CAPTURE-CANDIDATE below.)*

The substrate survey (forked `@explore`, 2026-06-13) established the shape and confirmed every
planning assumption:

1. **C-F2m carries two `unimplemented!("E.G")` stubs — `trace` and `solve_quadratic` — in all three
   implementors (`F2mNaive`/`F2mOpt`/`F2mNormal`).** E.F.1 deliberately over-specified the trait
   surface (the substrate-over-specify rule) and tagged these curve-facing operations for E.G.
   Filling them is the first thing E.G.1 does, and it is an **additive C-F2m amendment** (the trait
   *surface* is unchanged — the methods already exist, frozen at E.F.1; only the bodies go from
   `unimplemented!` to real), mirroring exactly how E.F.2 filled the deferred `inv`/`div`. The
   `frobenius`/`square`/`pow`/`add`/`mul`/`inv`/`div` the curve law consumes are all **already live**.

2. **The binary curve is fully greenfield — no `BinaryCurve` / López–Dahab / Koblitz code exists.**
   `rho::curve::Curve` is concretely `Uint<4>`-based (prime `p`, short-Weierstrass `y²=x³+ax+b`); its
   group-law methods (`double_jacobian`/`add_jacobian`/`add_mixed`/`scalar_mul`/`is_on_curve`) are
   `<F: Fp<4>>` at the *method* level. Binary curves use a **different group law**
   (`y²+xy=x³+ax²+b` for the non-supersingular case) and **López–Dahab projective coordinates**, not
   Jacobian (Jacobian's doubling formula divides by `2y`, which is zero in char 2 — the standard
   formulae break). E.G.1 builds a **parallel `BinaryCurve` type** mirroring the `Curve` /
   `AffinePoint` / projective-point idiom but with the binary group law and an `F2m`-generic surface.

3. **The existing rho walk is `Fp<4>`+`Curve`-bound and cannot be reused — E.G.2 writes a parallel
   binary-curve rho.** `rho/src/ecdlp/{walk,dp,negmap,coordinator}.rs` are generic over `F: Fp<4>` and
   the concrete `Curve`; `solve_brent`/`solve_dp`/`solve_dp_negmap` cannot instantiate over `F2m` +
   `BinaryCurve` without a parallel walk. E.G.2 re-runs Pollard rho over the binary curve as the
   **E.H benchmark baseline** (ROADMAP:353-354 "Re-run rho over GF(2^m) curves as baseline for E.H
   benchmarks"), mirroring the `rho/tests/ecdlp_kat.rs` `check_solver_on_curve` (`k·G=Q`) idiom.

4. **The order-6 Koblitz automorphism is fully greenfield and structurally distinct from `glv.rs`.**
   `rho/src/ecdlp/glv.rs` implements an **order-3 prime-field endomorphism** `φ(x,y)=(βx, y)` (a
   scalar x-scaling by a cube root of unity, with a 6-element orbit and canonical-representative
   collapse). The binary Koblitz automorphism is the **Frobenius endomorphism** `τ(x,y)=(x², y²)` —
   it acts on *both* coordinates via the field Frobenius, not a scalar multiply — and gives a
   τ-adic scalar decomposition. The ROADMAP names this as "the order-6 Koblitz automorphism the
   existing rho crate explicitly omits" (ROADMAP:352-353). E.G.3 adds it as the Cat-B/C speedup the
   prime-field `glv.rs` *parallels in spirit* (orbit collapse in the rho walk) but shares no code with.

5. **E.G adds the first `rho → shared-gf2m` edge.** No workspace crate depends on `shared-gf2m` yet
   (E.F shipped it standalone). E.G.1 adds `shared-gf2m = { path = "../shared/gf2m" }` to
   `rho/Cargo.toml` (mirroring how E.C added `rho → gnfs` and the C-Padic consumer added
   `rho → shared-padic` — the substrate ships standalone, the curve consumer adds the edge). The
   binary curve lives in `rho` (not a new crate) because it parallels the existing `rho::curve` /
   `rho::ecdlp` modules and re-uses the rho walk machinery's structure. `cargo check --workspace`
   confirms the new edge introduces no cycle (`shared-gf2m` depends only on `crypto-bigint`).

The work splits at **three representation/contract-sharp seams**, **3 sessions** (matching the
ROADMAP's 2–3 ceiling-biased estimate), at the boundaries between the curve-substrate, the baseline
rho, and the Koblitz optimization:

1. **E.G.1 — Binary-curve substrate + `trace`/`solve_quadratic` (Sonnet, Cat A).** Fill the C-F2m
   `trace`/`solve_quadratic` stubs (additive amend); the `BinaryCurve` type + the `y²+xy=x³+ax²+b`
   group law in López–Dahab projective coordinates; point decompression (recover `y` from `x` via the
   half-trace `solve_quadratic`). Adds the `rho → shared-gf2m` edge. **Freezes C-BinaryCurve** — the
   curve surface E.H descends from.

2. **E.G.2 — Pollard rho over the binary curve + end-to-end ECDLP KAT (Sonnet, Cat B).** A parallel
   `F2m`+`BinaryCurve` rho walk (the existing `Fp<4>` walk can't be reused); the end-to-end binary-
   curve ECDLP solver recovering `k` from `k·G = Q`. **Consumes C-BinaryCurve.** The E.H benchmark
   baseline. **Freezes C-BinaryRho.**

3. **E.G.3 ◆ — Koblitz τ-automorphism + τ-orbit rho speedup + sub-track close (Sonnet, Cat B/C,
   `@architect`).** The order-6 Koblitz Frobenius automorphism `τ(x,y)=(x²,y²)`; the τ-adic scalar
   decomposition; the τ-orbit collapse in the rho walk (the `glv.rs` analogue for Frobenius); the
   sub-track-close KAT suite. **Freezes C-Koblitz.** Crosses the **E.G ◆ boundary** — binary curves
   ship, ready for E.H to descend.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing GHS/Weil descent — that is
E.H; or Cantor's algorithm / the hyperelliptic Jacobian — that is E.I; or writing the binary-curve
*textbook chapter* in MATHEMATICS.md — that is **T.E**, paired with E.W at the Track-E ◆; E.G writes
at most a PEDAGOGY code-tour delta) and **rigidity** (forcing the binary group law into the Jacobian
formulae the char-2 doubling-by-`2y` breaks; re-deriving GF(2^m) field primitives rather than
consuming the frozen C-F2m `mul`/`square`/`frobenius`/`inv`/`solve_quadratic`; or porting `glv.rs`'s
scalar-x-scaling `φ` where the Frobenius `τ` on both coordinates is meant).

**Scoping discipline.** E.G builds binary curves at **demonstration fidelity** (principle 1 —
algorithmic content complete: the binary group law, point decompression, the baseline rho, the
Koblitz automorphism all implemented head-on) and **toy field/curve sizes** (small `m` — e.g.
GF(2^4), GF(2^8), and toy-shaped binary curves — enough to exercise every code path and exhibit the
algebra, not crypto-scale). It **amends C-F2m additively** (the `trace`/`solve_quadratic` bodies — no
trait-surface change) and **amends no other frozen contract** (C-Padic / C-Hensel / C-PadicLog /
C-SSA / the prime-field rho-curve surface are untouched — E.G is the *binary* curve, a parallel
type). It introduces **no new live oracle** on the green path (point-on-curve + group axioms +
`k·G=Q` round-trip + the Frobenius-orbit law are self-checking; an optional PARI `ellinit`/`elllog`
`#[ignore]` sidecar is the established dev-only pattern). The **engineering-vs-mathematics disconnect**
(ROADMAP principle 4) is explicit: the toy curve sizes are a principle-4 boundary (the group law and
the Koblitz speedup are crypto-scale-correct; only the *parameters* are toy), annotated, never
presented as crypto-scale.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.G): "*E.G — Binary curves + Koblitz automorphism. 2-3 sessions. Predecessor:
E.F. The order-6 Koblitz automorphism the existing rho crate explicitly omits. Re-run rho over
GF(2^m) curves as baseline for E.H benchmarks. Sonnet.*" E.G is the **first GF(2^m)-curve consumer**:
where E.F built the characteristic-2 *field* substrate (C-F2m), E.G builds the *curves* over those
fields — the non-supersingular binary curve `y²+xy=x³+ax²+b`, its group law in López–Dahab projective
coordinates, point decompression via the half-trace, and the Pollard-rho ECDLP solver over them. It
is the baseline E.H (GHS/Weil descent) benchmarks against, and the home of the order-6 Koblitz
Frobenius automorphism that the prime-field rho crate "explicitly omits."

E.G's predecessor is E.F (now complete): it consumes the frozen C-F2m field trait directly — `add`
(XOR), `mul` (carryless + reduce), `square`/`frobenius` (the bit-spread that *is* the curve's `τ`),
`inv`/`div`, and the two over-specified curve-facing ops `trace` and `solve_quadratic` that E.F.1
declared-and-stubbed *for this sub-track*. The central design tension is that **a binary curve is an
elliptic curve, like a prime-field curve, but with a different group law**: the survey confirmed the
existing `Curve`'s Jacobian formulae break in char 2 (doubling divides by `2y = 0`), so E.G builds a
*parallel* `BinaryCurve` type, sharing the design *idiom* (a const-generic-`F2m`-method curve, affine
+ projective point types, a baseline rho walk, an orbit-collapsing automorphism speedup) but not the
type itself.

The sub-track decomposes into three conceptual units, each a session:

1. **The binary-curve substrate + the curve-facing field ops (E.G.1).** Fill the C-F2m
   `trace`/`solve_quadratic` stubs (`trace(a) = Σ_{i<m} a^(2^i)`, valued in GF(2) ⊂ GF(2^m);
   `solve_quadratic(c)` solves `x²+x=c` via the half-trace, the operation point decompression needs).
   Build `BinaryCurve` — the non-supersingular `y²+xy=x³+ax²+b` curve, `BinaryAffinePoint<F>`, and
   López–Dahab projective coordinates (the char-2 analogue of Jacobian, where the group law avoids the
   `2y` division). Point decompression: given `x` and a bit, recover `y` by solving `x²+x = a + b/x²`
   (the curve equation in the `λ = y/x` substitution) with `solve_quadratic`. **Freezes
   C-BinaryCurve.** **(E.G.1.)**

2. **Pollard rho over the binary curve (E.G.2).** A parallel `F2m`+`BinaryCurve` rho walk (the
   existing `Fp<4>`+`Curve` walk cannot instantiate over the binary types). The r-adding walk, the
   distinguished-point collision, the linear-recovery of `k` from `k·G=Q` — re-run over binary
   curves, mirroring `rho/tests/ecdlp_kat.rs`'s `check_solver_on_curve` end-to-end idiom. This is the
   baseline E.H benchmarks the descent against. **Freezes C-BinaryRho.** **(E.G.2.)**

3. **The Koblitz τ-automorphism + the rho speedup + close (E.G.3).** The order-6 Koblitz Frobenius
   automorphism `τ(x,y)=(x², y²)` (an endomorphism of the binary curve, satisfying `τ² − tτ + 2 = 0`
   for the curve's trace `t`); the τ-adic scalar decomposition; the τ-orbit collapse in the rho walk
   (the binary analogue of `glv.rs`'s 6-element-orbit canonical-representative collapse — the speedup
   the prime-field rho parallels). The sub-track-close KAT suite. **Freezes C-Koblitz.** **(E.G.3 ◆.)**

E.G is **descent-free** (GHS/Weil descent is E.H; Cantor's algorithm and the hyperelliptic Jacobian
are E.I), **field-arithmetic-frozen** (it consumes C-F2m — the only change is filling the two stubs
E.F.1 left for it, an additive amend; no new GF(2^m) primitive), and **chapter-free** (the binary-
curve textbook content is T.E, paired with E.W at the Track-E ◆). Re-read this intent at the ◆
boundary to catch defocus (descent, Cantor, the MATHEMATICS chapter) and rigidity (Jacobian formulae
in char 2; re-deriving field primitives; porting `glv.rs`'s `φ` where `τ` is meant).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey confirmed zero hits); raw `cargo` is the only
CI surface (unchanged from E.D/E.E/E.F). Oracle KATs are `#[ignore]`-gated only — the exact form is
`#[ignore = "PARI not installed; run manually when available"]`, used identically in
`rho/tests/ssa_kat.rs`, `rho/tests/mov_kat.rs`, and `shared/padic/tests/log_kat.rs`. `/run-plan`
re-discovers at preflight. E.G **adds a new workspace *edge* (`rho → shared-gf2m`) but no new crate**
(the `BinaryCurve` + binary rho live in the existing `rho` crate), so the gate is a **correctness +
edge-integrity gate**:

- Each session's KATs (`rho/tests/binary_curve_kat.rs` for E.G.1/E.G.3, the end-to-end binary ECDLP
  KAT in the same file or `binary_ecdlp_kat.rs` for E.G.2, plus inline unit tests mirroring the
  `rho::curve` idiom) are the primary correctness signal — fast and *exactly* decisive (lever 5:
  point-on-curve `y²+xy = x³+ax²+b`, the group axioms, the decompression round-trip
  `decompress(compress(P)) = P`, the Frobenius-orbit law `τ^m(P) = P`, the τ²−tτ+2 characteristic
  relation, and the end-to-end `k·G = Q` recovery are all self-checking with no external oracle).
- `cargo check --workspace` must confirm the new `rho → shared-gf2m` edge resolves with **no
  dependency cycle** (`shared-gf2m` depends only on `crypto-bigint`; nothing in `shared-gf2m` depends
  on `rho`, so the new edge is acyclic).
- **The existing rho / gnfs / shared KATs must stay green** after the binary-curve code lands — E.G
  adds new modules (`rho::binary_curve`, `rho::binary_ecdlp` or similar) and fills two `shared/gf2m`
  stubs but changes no existing prime-field-curve / pairing / SSA path, so the no-regression invariant
  is structurally easy to hold; `cargo test --workspace` is the guard. *(One subtlety: filling the
  C-F2m `trace`/`solve_quadratic` stubs touches `shared/gf2m` — the existing `gf2m_kat.rs` must stay
  green and gain new tests for the now-live methods.)*

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.G.1 | Binary-curve substrate + `trace`/`solve_quadratic` + `rho → shared-gf2m` edge | A | Sonnet | C-F2m (frozen E.F.1: `mul`/`square`/`frobenius`/`inv`/`div` read; `trace`/`solve_quadratic` stubs filled — additive amend); `rho::curve::Curve` (read — the idiom to mirror, not the type to reuse); `shared/gf2m` re-exports (`F2m`, `F2mNaive`, `F2mOpt`) | `shared/gf2m/src/naive.rs` (fill `trace`/`solve_quadratic`), `shared/gf2m/src/opt.rs` (fill `trace`/`solve_quadratic`), `shared/gf2m/src/normal.rs` (fill `trace`/`solve_quadratic`), `shared/gf2m/tests/gf2m_kat.rs` (extend: trace + solve_quadratic KATs), `rho/Cargo.toml` (add `shared-gf2m` dep edge), `rho/src/binary_curve/mod.rs` (new: `BinaryCurve` + `BinaryAffinePoint` + López–Dahab projective point + group law + decompression), `rho/src/lib.rs` (add `pub mod binary_curve;`), `rho/tests/binary_curve_kat.rs` (new: point-on-curve + group-axiom + decompression-round-trip KATs) |
| E.G.2 | Pollard rho over the binary curve + end-to-end ECDLP KAT | B | Sonnet | C-BinaryCurve (frozen E.G.1); C-F2m (read); `rho::ecdlp` walk idiom (read — `Fp<4>`-bound, mirrored not reused) | `rho/src/binary_ecdlp/mod.rs` (new: the `F2m`+`BinaryCurve` rho walk — r-adding walk, distinguished-point collision, linear recovery of `k`), `rho/src/lib.rs` (add `pub mod binary_ecdlp;`), `rho/tests/binary_ecdlp_kat.rs` (new: end-to-end `k·G = Q` recovery on toy binary curves, mirroring `check_solver_on_curve`) |
| E.G.3 ◆ `@architect` | Koblitz τ-automorphism + τ-orbit rho speedup + sub-track close | C | Sonnet | C-BinaryCurve (frozen E.G.1, read), C-BinaryRho (frozen E.G.2, read), `rho::ecdlp::glv` (read — the prime-field orbit-collapse idiom, structurally distinct) | `rho/src/binary_ecdlp/koblitz.rs` (new: `τ(x,y)=(x²,y²)` automorphism + τ-adic decomposition + τ-orbit canonical collapse), `rho/src/binary_ecdlp/mod.rs` (add the τ-orbit walk variant + `pub mod koblitz;`), `rho/tests/binary_curve_kat.rs` (extend: Frobenius-orbit `τ^m=id`, the τ²−tτ+2 relation, sub-track-close suite), `rho/tests/binary_ecdlp_kat.rs` (extend: τ-orbit rho recovers the same `k` as the E.G.2 baseline) |

**Sequencing notes.** Strictly serial: **E.G.1 → E.G.2 → E.G.3.** E.G.1 lands the curve-facing field
ops, the `BinaryCurve` type, the group law, and decompression everything stands on; E.G.2 adds the
baseline rho (the algorithm over the substrate); E.G.3 adds the Koblitz optimization and closes the
sub-track. **One `@architect` marker** sits on the **E.G.3 ◆** (the boundary juncture ratifying
C-BinaryCurve / C-BinaryRho / C-Koblitz and confirming the substrate is complete and descent-ready
before the sub-track closes). *(Tradeoff named: unlike E.F — which carried an inline E.F.1 juncture
because its trait bound three downstream sub-tracks — E.G freezes C-BinaryCurve at E.G.1 but does NOT
page an inline juncture there. The Cat-C orthogonality (the baseline E.G.2 and the Koblitz E.G.3 both
consume C-BinaryCurve in-crate, immediately, where a wrong curve shape fails the E.G.2 `k·G=Q` KAT
loudly at the next session) plus the single high-stakes consumer (E.H, which has its own Opus-flagged
E.H.1 to design its descent) makes the early-catch insurance less valuable than it was for E.F's
three-consumer trait. The ◆ juncture is held at Opus (juncture-tier) for the C-BinaryCurve → E.H
cost-of-wrong, but a separate inline E.G.1 fork is not bought. If E.G.2's `k·G=Q` KAT surfaces a
curve-shape concern, that is the loud signal that substitutes for an inline juncture.)*

**Why 3 sessions (the ROADMAP's 2–3 ceiling-biased estimate).** The split is taken at three
representation/contract-sharp seams:
- **One-line-commit-title corollary.** "Binary-curve substrate + trace/solve_quadratic", "Pollard rho
  over the binary curve", and "Koblitz τ-automorphism + τ-orbit speedup + close" are **three distinct
  commit titles** across three categories (A substrate, B algorithm, C optimization).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the
  curve-and-group-law (with the field ops it needs), the baseline rho, the Koblitz automorphism.
  None fractures below its floor — the binary group law + decompression is one unit (a curve with no
  decompression is unusable by the rho walk's point-on-curve checks); the baseline rho is one unit;
  the Koblitz automorphism + its rho speedup is one unit (an automorphism with no orbit-collapse in
  the walk has no consumer).
- **Contract-sharp boundary + Cat-B/C orthogonality.** E.G.1 **freezes** C-BinaryCurve; E.G.2
  consumes it (the algorithm over the substrate); E.G.3 reads the baseline and adds the optimization
  — and per the Cat-C rule (`multi-session-planning.md:217` "never alters the baseline"), the Koblitz
  speedup must *read* the E.G.2 baseline's output and produce a new variant, **not replace it** (the
  baseline must stay available for the E.H benchmark comparison). This is the contract-sharp seam that
  forbids merging E.G.2 and E.G.3.

They are **not** further splittable below 3: separating the `trace`/`solve_quadratic` stub-fill from
the `BinaryCurve` would leave the field-op fill with no curve consumer to express its KAT against
(`solve_quadratic`'s test is the decompression round-trip — it needs the curve; flagging a row whose
deliverable can't be a curve-facing KAT). Separating the binary group law from point decompression
would split the curve substrate across two rows with no contract-sharp seam (decompression *is* the
half-trace consumer that justifies the `solve_quadratic` fill — they are one unit). Merging E.G.2 and
E.G.3 would violate both the one-line-title corollary (baseline + optimization = two titles, likely
>400 LOC) and the Cat-C "never alter the baseline" rule (the Koblitz orbit-collapse would be entangled
with the baseline walk it must leave intact for benchmarking).

---

## Session detail

E.G.1 is specified at near-full fidelity (the `BinaryCurve` surface is the design crux that E.H
descends from). E.G.2–3 are lower-fidelity sketches, correct per the substrate-first discipline: they
are crisply specified only after C-BinaryCurve freezes.

### E.G.1 — Binary-curve substrate + `trace`/`solve_quadratic` + `rho → shared-gf2m` edge (Sonnet, Cat A)

**Deliverable:** the curve-facing field ops C-F2m deferred (filling the `trace`/`solve_quadratic`
stubs — an additive C-F2m amend), the `BinaryCurve` type, the `y²+xy=x³+ax²+b` group law in López–
Dahab projective coordinates, and point decompression, plus the first `rho → shared-gf2m` edge. The
pieces:
- **Fill the C-F2m `trace`/`solve_quadratic` stubs** (`shared/gf2m/src/{naive,opt,normal}.rs`):
  `trace(a) = Σ_{i<m} a^(2^i)` — the absolute trace `Tr_{GF(2^m)/GF(2)}(a)`, valued in GF(2) ⊂
  GF(2^m) (it is `0` or `1`); computed by iterating `frobenius` m−1 times and summing (XOR).
  `solve_quadratic(c)` — solve `x²+x = c` (the half-trace): a solution exists iff `trace(c) = 0`; the
  half-trace `H(c) = Σ_{i} c^(2^(2i))` is the closed form for odd `m`. Both go from `unimplemented!`
  to real in all three implementors. **This is an additive C-F2m amendment** (the trait surface is
  unchanged — the methods were frozen at E.F.1; only the bodies fill), exactly mirroring how E.F.2
  filled `inv`/`div`. The existing `gf2m_kat.rs` stays green and gains `trace`/`solve_quadratic` KATs.
- **The `BinaryCurve` type** (`rho/src/binary_curve/mod.rs`): the design crux. A **parallel** to
  `rho::curve::Curve`, not a reuse. Mirror the idiom — a struct holding the curve parameters
  (`a`, `b` as `Uint<L>` coefficient bit-vectors, the irreducible `poly`, the group order `n`, the
  base point), with group-law methods `<F: F2m<L>>` at the *method* level (the `Curve` pattern of
  not owning `PhantomData<F>`, threading `F` per-call). The curve is the **non-supersingular**
  `y²+xy = x³+ax²+b` (the standard binary-curve form; `b ≠ 0`). A `BinaryAffinePoint<F>` enum
  (`Infinity | Finite { x, y }`) mirroring `AffinePoint`, and a **López–Dahab projective** point
  (the char-2 analogue of Jacobian — the affine representative is `(X/Z, Y/Z²)`, chosen because the
  doubling formula avoids the `2y` division that breaks Jacobian in char 2).
- **The binary group law** (`rho/src/binary_curve/mod.rs`): doubling and addition in López–Dahab
  coordinates (Hankerson–Menezes–Vanstone §3.2.1 / Algorithm 3.24 for char-2 LD addition; the
  doubling formula uses `square` heavily — cheap in char 2 — and one `inv`-free projective step).
  `is_on_curve` checks `y²+xy = x³+ax²+b` over `F2m`. `negate`: in char 2, `−(x,y) = (x, x+y)` (NOT
  `(x, −y)` — `−y = y` in char 2, so the negation is `(x, x+y)`, a char-2-specific trap mirroring the
  `sub==add`/`neg==id` field trap).
- **Point decompression** (`rho/src/binary_curve/mod.rs`): given `x` and a sign bit, recover `y`.
  Substituting `λ = y/x` into the curve equation gives `λ²+λ = x + a + b/x²`; solve with
  `solve_quadratic`, then `y = λ·x`. The two roots `λ` and `λ+1` give the two y-values; the sign bit
  (e.g. `trace`-based) selects. This is the consumer that justifies the `solve_quadratic` fill.
- **The `rho → shared-gf2m` edge** (`rho/Cargo.toml`): add `shared-gf2m = { path = "../shared/gf2m" }`.
  `cargo check --workspace` confirms no cycle (the first edge into the standalone E.F crate).

Consumes C-F2m (frozen E.F.1 — `mul`/`square`/`frobenius`/`inv`/`div`/`add` read; `trace`/
`solve_quadratic` stubs filled), the `rho::curve::Curve` idiom (read — mirrored, not reused), the
`shared/gf2m` re-exports. **Freezes C-BinaryCurve; amends C-F2m additively.**

**KAT** (`rho/tests/binary_curve_kat.rs` + extended `shared/gf2m/tests/gf2m_kat.rs` + inline unit
tests per the `rho::curve` idiom): over toy binary curves (e.g. a curve over GF(2^4) with `x⁴+x+1`,
and a slightly larger one over GF(2^8)): **point-on-curve** `y²+xy = x³+ax²+b` for the base point and
its multiples; the **group axioms** (`P+∞=P`, `P+(−P)=∞` with `−P=(x,x+y)`, associativity on a
sample); **doubling consistency** (`2P` via doubling equals `P+P` via addition); the **decompression
round-trip** `decompress(x, bit) ` lands on the curve and recovers the original `y`; the **field-op
KATs** for the now-live `trace` (`trace(a) ∈ {0,1}`; `trace(a+b)=trace(a)⊕trace(b)`; `trace` is
Frobenius-invariant `trace(a²)=trace(a)`) and `solve_quadratic` (`solve_quadratic(c)` returns `x`
with `x²+x=c` when `trace(c)=0`). **Verify gate:** `cargo test --workspace` green; `cargo check
--workspace` resolves the new edge with no cycle; the existing rho/gnfs/shared KATs unchanged
(including `gf2m_kat.rs` now with `trace`/`solve_quadratic` live).

**Subtlety (load-bearing):** (1) **The binary group law is NOT the Jacobian formula** — the char-2
doubling avoids the `2y` division (zero in char 2). A `@build` agent porting `rho::curve`'s
`double_jacobian` writes code that divides by zero or is silently wrong. López–Dahab (or the affine
char-2 formulae with one `inv`) is mandatory; the curve docs and the doubling-consistency KAT
(`2P == P+P`) are the defense. (2) **`−P = (x, x+y)`, not `(x, −y)`** — in char 2 `−y = y`, so naive
negation gives `P`, not `−P`; the `P+(−P)=∞` KAT is the loud signal. (3) **`solve_quadratic` has a
solvability precondition** — `x²+x=c` is solvable iff `trace(c)=0`; decompression of an `x`-coordinate
not on the curve must fail cleanly (or the half-trace returns a wrong `y` that fails point-on-curve).
The decompression KAT must check the round-trip lands on the curve. (4) **The trace is valued in
GF(2)** — `trace(a)` is `0` or `1` (the field-element `zero`/`one`), not an arbitrary field element;
a KAT asserting `trace(a) == F2m::zero() || trace(a) == F2m::one()` catches a wrong trace loop.
(5) **Toy curve sizes only** — small `m` and toy curve parameters exercise every path; a crypto-scale
binary curve (m = 163, 233, 571 — the NIST/SECG binary curves) needs the same algorithms (principle-4
annotate: the parameters are toy, the group law is not). (6) **The first `rho → shared-gf2m` edge** —
`cargo check --workspace` must resolve with no cycle; if it reports one (it must not — `shared-gf2m`
is a leaf), that is a destructive-HALT.

**Deferred:** the baseline Pollard rho over the binary curve (E.G.2); the Koblitz τ-automorphism +
τ-orbit speedup (E.G.3); GHS/Weil descent (E.H — out of scope, the descent machinery); Cantor's
algorithm / the hyperelliptic Jacobian (E.I); the MATHEMATICS chapter (T.E at the Track-E ◆).

### E.G.2 — Pollard rho over the binary curve + end-to-end ECDLP KAT (Sonnet, Cat B)

**Deliverable:** a parallel `F2m`+`BinaryCurve` Pollard-rho ECDLP solver — the baseline E.H benchmarks
against. Lower-fidelity sketch (crisp after C-BinaryCurve freezes):
- **The binary-curve rho walk** (`rho/src/binary_ecdlp/mod.rs`): the r-adding walk (partition the
  group into `r` classes, each with a precomputed addend `a_i·G + b_i·Q`), the distinguished-point
  collision detection, and the linear recovery of `k` from a collision (`a₁G+b₁Q = a₂G+b₂Q ⟹
  k = (a₁−a₂)/(b₂−b₁) mod n`). The existing `rho::ecdlp` walk is `Fp<4>`+`Curve`-bound and cannot be
  instantiated over `F2m`+`BinaryCurve` — this is a *parallel* walk over the binary types, mirroring
  the existing walk's structure (`walk.rs`/`dp.rs` shape) but consuming `BinaryCurve`'s group law.
- **End-to-end ECDLP solver** (`rho/src/binary_ecdlp/mod.rs`): `solve(curve, G, Q) -> k` recovering
  the scalar, mirroring `rho/tests/ecdlp_kat.rs`'s `check_solver_on_curve` (`k·G == Q`).

Consumes C-BinaryCurve (frozen E.G.1), C-F2m (read), the `rho::ecdlp` walk idiom (read — mirrored).
**Freezes C-BinaryRho.**

**KAT:** the end-to-end `k·G = Q` recovery on toy binary curves (small known group order; the solver
recovers the planted `k`), mirroring `check_solver_on_curve`; the walk's distinguished-point /
collision invariant (`a·G + b·Q = walk point`) held across steps. **Verify gate:** `cargo test
--workspace` green.

**Subtlety:** the walk-state invariant `W = a·G + b·Q` (the prose contract preserved across the rho
sessions, mirroring the prime-field rho's invariant) must hold across the binary walk and be the
loud signal — a wrong addend table or a group-law bug shows up as a recovered `k` with `k·G ≠ Q`,
which the end-to-end KAT catches.

### E.G.3 ◆ — Koblitz τ-automorphism + τ-orbit rho speedup + sub-track close (Sonnet, Cat C, `@architect`)

**Deliverable:** the order-6 Koblitz Frobenius automorphism, the τ-orbit rho speedup (reading the
E.G.2 baseline, not replacing it), and the sub-track close. Lower-fidelity sketch:
- **The Koblitz automorphism** (`rho/src/binary_ecdlp/koblitz.rs`): `τ(x,y) = (x², y²)` — the
  Frobenius endomorphism of the binary curve (well-defined because the curve coefficients are in the
  base field, so Frobenius fixes the curve). It satisfies the characteristic relation `τ² − tτ + 2 = 0`
  where `t` is the curve's trace of Frobenius (`#E = 2^m + 1 − t`). The τ-adic (non-adjacent-form)
  decomposition of a scalar. This is structurally distinct from `glv.rs`'s order-3 `φ(x,y)=(βx,y)`
  scalar-x-scaling — `τ` acts on *both* coordinates via the field `frobenius` (the C-F2m op).
- **The τ-orbit rho speedup** (`rho/src/binary_ecdlp/mod.rs`): collapse the τ-orbit
  `{P, τP, τ²P, …, τ^(m−1)P}` (and negatives) to a canonical representative in the walk — the binary
  analogue of `glv.rs`'s 6-element-orbit canonical collapse (`glv_canonical`). The Cat-C discipline:
  this **reads** the E.G.2 baseline walk and produces a *new variant*; the baseline stays intact for
  the E.H benchmark comparison.
- **Sub-track-close KAT suite** (`rho/tests/binary_curve_kat.rs` + `binary_ecdlp_kat.rs`, extended):
  the Frobenius-orbit law `τ^m(P) = P` (the order of `τ` divides `m`); the characteristic relation
  `τ²(P) − t·τ(P) + 2P = ∞`; the τ-orbit rho recovers the same `k` as the E.G.2 baseline; the full
  binary-curve axiom + decompression suite stays green.

Consumes C-BinaryCurve (frozen E.G.1, read), C-BinaryRho (frozen E.G.2, read), the `rho::ecdlp::glv`
orbit-collapse idiom (read — structurally distinct). **Freezes C-Koblitz.**

**KAT (primary correctness signal):** **τ^m(P) = P** (the automorphism has order dividing m); the
**τ²−tτ+2 = 0 characteristic relation** applied to a point (`τ(τP) − t·(τP) + 2P = ∞`); the **τ-orbit
rho recovers the same `k`** as the E.G.2 baseline on every toy curve; the full curve + rho suite stays
green; the existing rho/gnfs/shared KATs unchanged. Optional PARI `ellinit`/`elllog` cross-check
(`#[ignore]`-gated). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **`τ` is the field `frobenius`, not a scalar multiply** — a `@build`
agent that ports `glv.rs`'s `φ(x,y)=(βx,y)` (a coordinate scaling) writes the wrong automorphism;
`τ(x,y)=(x²,y²)` applies the C-F2m `frobenius`/`square` to *both* coordinates. The `τ^m=id` KAT is the
loud signal. (2) **The Koblitz speedup must NOT alter the baseline** (Cat-C "never alter the
baseline") — the E.G.2 baseline rho stays available for the E.H benchmark; the τ-orbit variant is a
*new* walk variant reading it. (3) **This is the E.G ◆ boundary** — re-read the Purpose intent and
verify the substrate is complete (curve group law, decompression, baseline rho, Koblitz automorphism)
and **descent-ready** (C-BinaryCurve exposes what E.H's GHS/Weil descent reads — the curve over
GF(2^m), the group law, the trace), and that E.G stayed descent-free / Cantor-free / chapter-free.
(4) **No MATHEMATICS chapter here** — the binary-curve / Koblitz textbook content is T.E, paired with
E.W at the *Track-E* ◆; E.G.3 writes at most a PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.G.3 ◆ to confirm: (1) the binary-curve substrate is complete and composes (curve + group law +
decompression + baseline rho + Koblitz automorphism all present and cross-checked); (2) C-BinaryCurve
exposes what E.H descends from (the binary curve over GF(2^m), the group law, the Frobenius trace `t`)
so E.H can build GHS/Weil descent without amending the curve surface — the substrate-readiness defense;
(3) C-Koblitz's τ-orbit speedup reads the baseline and does not alter it (the E.H benchmark baseline
is intact); (4) E.G stayed in scope — no GHS/Weil descent (E.H), no Cantor/hyperelliptic Jacobian
(E.I), no MATHEMATICS chapter (T.E), `τ` is the Frobenius not a `glv.rs`-style scaling; (5) the
principle-4 boundary (toy curve sizes; the algorithms are crypto-scale-correct) is recorded, not
silently presented as crypto-scale. **Also: reconcile the static-frame ROADMAP debt** — the Progress
table is stale by two completed sub-tracks (E.D, E.E) and the Remaining table still lists the
now-complete E.F; the E.G ◆ is the right boundary to update them. One-shot findings; does not
implement. Held at **Opus** per the header (juncture-tier — C-BinaryCurve → E.H cost-of-wrong).

---

## Cross-session contracts

E.G **freezes three** contracts (C-BinaryCurve at E.G.1, C-BinaryRho at E.G.2, C-Koblitz at E.G.3) and
**amends C-F2m additively** (filling the `trace`/`solve_quadratic` stubs E.F.1 left for it — no trait-
surface change). It amends no other prior frozen contract (the p-adic / SSA / prime-field rho-curve /
`Fp` surfaces are untouched — the binary curve is a parallel type).

### C-F2m (additive amendment) — `trace` / `solve_quadratic` bodies filled (compiler- + test-enforced)

**Defined in:** E.F.1 (frozen). **Amended in:** E.G.1 (`shared/gf2m/src/{naive,opt,normal}.rs` — the
`trace`/`solve_quadratic` bodies go from `unimplemented!("E.G")` to real). **The trait surface is
unchanged** — both methods were declared-and-frozen at E.F.1 per the substrate-over-specify rule; E.G
only fills the bodies, exactly as E.F.2 filled the deferred `inv`/`div`. **This is an additive amend,
NOT a re-freeze and NOT a break:** no downstream consumer of C-F2m's frozen surface is invalidated
(the methods existed; calling them now succeeds where it previously panicked). **Semantics frozen
here:** `trace(a) = Σ_{i<m} a^(2^i)` is the absolute trace `Tr_{GF(2^m)/GF(2)}`, valued in GF(2) ⊂
GF(2^m) (returns `zero` or `one`); `solve_quadratic(c)` returns an `x` with `x²+x = c` when
`trace(c) = 0` (solvability precondition), via the half-trace closed form. *Resolved at E.G.1; the
exact half-trace form (odd-m closed form vs general linearized-polynomial solve) is an E.G.1
implementation call recorded at the ◆.*

### C-BinaryCurve — binary elliptic curve `y²+xy=x³+ax²+b` over GF(2^m) (compiler- + test-enforced) — *to be frozen at E.G.1*

**Defined in:** E.G.1 (`rho/src/binary_curve/mod.rs`). **Consumed by:** E.G.2 (the baseline rho),
E.G.3 (the Koblitz automorphism); **downstream: E.H** (GHS/Weil descent — descends from the binary
curve over GF(2^m); the single highest-stakes consumer, "the most mathematically intricate single
attack" — the cost-of-wrong that holds the ◆ juncture at Opus), **E.W** (the cross-attack benchmark
table). Compiler-enforced (the `BinaryCurve` type + `BinaryAffinePoint` + López–Dahab projective point
+ the `<F: F2m<L>>` group-law methods) + test-enforced (point-on-curve, the group axioms, the
decompression round-trip). Exposes: `BinaryCurve` (the non-supersingular `y²+xy=x³+ax²+b` curve, `b≠0`,
holding `a`/`b`/`poly`/`n`/base-point as `Uint<L>`); `BinaryAffinePoint<F>` (`Infinity | Finite{x,y}`);
the López–Dahab projective point (affine repr `(X/Z, Y/Z²)`); `is_on_curve`/`negate` (with the char-2
`−P=(x,x+y)`)/`double`/`add`/`scalar_mul`/`generator`/decompression. **Char-2 curve invariants:**
`−P = (x, x+y)` (NOT `(x,−y)`); doubling uses the char-2 López–Dahab formula (NOT Jacobian — the `2y`
division is zero); decompression's `solve_quadratic` needs `trace(·)=0`. **The curve is defined
relative to its irreducible `poly`** — the field identity threads through (C-MovBridge-style
consistency guard). **Toy curve sizes only** (principle-4 boundary). *Exact point-representation choice
(López–Dahab vs affine-with-one-inv; the projective coordinate convention) ratified at the E.G.3 ◆.*

### C-BinaryRho — Pollard rho ECDLP over the binary curve (compiler- + test-enforced) — *to be frozen at E.G.2*

**Defined in:** E.G.2 (`rho/src/binary_ecdlp/mod.rs`). **Consumed by:** E.G.3 (the Koblitz τ-orbit
speedup reads it — the baseline it must not alter), **E.W** (the benchmark table — binary rho is the
baseline the descent and Koblitz variants are timed against; **E.H** benchmarks the descent against
it). Compiler- + test-enforced. Exposes the `F2m`+`BinaryCurve` rho walk (r-adding walk, distinguished
points, linear recovery) and the end-to-end `solve(curve, G, Q) -> k`. **The walk-state invariant
`W = a·G + b·Q`** is the prose contract preserved across E.G.2/E.G.3 (mirroring the prime-field rho
invariant). **The baseline must stay intact** (Cat-C rule) — E.G.3's Koblitz variant reads it, never
replaces it, so E.W/E.H can benchmark against the un-optimized walk. *Exact walk parameters (the
partition count `r`, the distinguished-point criterion) ratified at the E.G.3 ◆.*

### C-Koblitz — order-6 Koblitz Frobenius automorphism + τ-orbit rho speedup (compiler- + test-enforced) — *to be frozen at E.G.3 ◆*

**Defined in:** E.G.3 (`rho/src/binary_ecdlp/koblitz.rs`). **Consumed by:** **E.W** (the benchmark
table — the τ-orbit speedup is one of the curves "which optimization wins" compares). Compiler- +
test-enforced. Exposes `τ(x,y)=(x²,y²)` (the Frobenius endomorphism, applying C-F2m `frobenius`/
`square` to both coordinates — structurally distinct from `glv.rs`'s scalar `φ`), the τ-adic scalar
decomposition, and the τ-orbit canonical-collapse rho variant. **The characteristic relation
`τ²−tτ+2 = 0`** (for the curve's Frobenius trace `t`) and **`τ^m = id`** are the frozen invariants.
**`τ` is the field Frobenius, NOT a coordinate scaling** (the `glv.rs`-port trap). **The τ-orbit
variant reads the E.G.2 baseline, does not alter it** (Cat-C). *Exact τ-adic NAF width / orbit-collapse
canonicalization ratified at the ◆.*

### Frozen contracts read by E.G (consumed, not amended)

- **C-F2m (frozen surface)** — `add`(XOR)/`mul`(carryless+reduce)/`square`/`frobenius`/`inv`/`div`/
  `pow` consumed by the binary group law and the Koblitz `τ`. Read. *(The `trace`/`solve_quadratic`
  bodies are filled — additive amend, above; the rest of the surface is read unchanged.)*
- **`rho::curve::Curve` + `AffinePoint`/`JacobianPoint`** — the *design idiom* E.G's `BinaryCurve`
  mirrors (a const-generic-method curve, affine + projective point types, per-call field threading).
  **Read for the pattern; NOT reused** (the Jacobian group law breaks in char 2). Untouched.
- **`rho::ecdlp` walk + `rho::ecdlp::glv`** — the rho-walk idiom (E.G.2 mirrors) and the orbit-collapse
  idiom (E.G.3 parallels, structurally distinct). Read. Untouched (the prime-field rho is unchanged).

### New workspace edge (not a new crate)

- **`rho → shared-gf2m`** — E.G.1 adds `shared-gf2m = { path = "../shared/gf2m" }` to `rho/Cargo.toml`.
  **A new edge, no new crate** (the binary curve + binary rho live in the existing `rho` crate; this
  is the first edge into the standalone E.F crate, mirroring how `rho → shared-padic` was E.E's edge
  into the standalone E.D crate). `cargo check --workspace` confirms the edge resolves with no cycle
  (`shared-gf2m` is a leaf depending only on `crypto-bigint`). *(If E.G found it must change
  `shared-gf2m`'s frozen surface beyond filling the two stubs — it should not — that would be a
  discovery surfaced at the ◆, never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.G.3 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.G.1 | Binary-curve substrate + `trace`/`solve_quadratic` + `rho → shared-gf2m` edge | done | 178c0aa | C-BinaryCurve (+ C-F2m additive: `trace`/`solve_quadratic`) |
| E.G.2 | Pollard rho over the binary curve + end-to-end ECDLP KAT | pending | — | C-BinaryRho |
| E.G.3 ◆ | Koblitz τ-automorphism + τ-orbit rho speedup + sub-track close | pending | — | C-Koblitz |

Contracts frozen before this sub-track: the GF(2^m) field surface (C-F2m/C-F2mOpt — read by E.G, with
C-F2m's two stubs filled additively), the p-adic surface (C-Padic/C-Hensel/C-PadicLog), the SSA
surface (C-AnomalousLift/C-SSA), the prime-field rho curve + ECDLP surface, `Fp<4>`. This sub-track
**freezes three new contracts** (C-BinaryCurve, C-BinaryRho, C-Koblitz), serving the downstream **E.H**
(GHS/Weil descent — descends from C-BinaryCurve), **E.W** (cross-attack benchmarks — baselines against
C-BinaryRho), and **opens the binary-curve consumer cluster of Track E**.

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.G builds binary curves on the frozen C-F2m substrate — building the curve + group law +
  decompression + baseline rho + Koblitz automorphism is internal-continue (confirmed by survey).**
  No binary-curve code exists. A discovery that the curve law or the Koblitz decomposition needs a
  field primitive C-F2m did not over-specify is an **additive amend of C-F2m** (like filling the
  `trace`/`solve_quadratic` stubs) surfaced at the ◆ juncture — not a silent trait patch.

- **The binary group law is NOT the Jacobian formula — forcing it is a rigidity failure (the central
  design guard).** The char-2 doubling divides by `2y = 0`; the standard Jacobian formulae break. A
  `@build` agent porting `rho::curve`'s `double_jacobian` writes divide-by-zero / silently-wrong code.
  López–Dahab (or affine-with-one-`inv`) char-2 formulae are mandatory; the `2P == P+P`
  doubling-consistency KAT is the defense — **internal-continue → corrected.**

- **`−P = (x, x+y)`, not `(x, −y)` — the char-2 negation trap.** In char 2 `−y = y`, so naive
  negation gives `P`, not `−P`. The `P + (−P) = ∞` KAT is the loud signal. **Internal-continue →
  corrected** (mirrors the field-level `sub==add`/`neg==id` trap from E.F).

- **`τ` is the field Frobenius `(x²,y²)`, NOT a `glv.rs`-style coordinate scaling.** The prime-field
  GLV `φ(x,y)=(βx,y)` is a scalar x-scaling; the binary Koblitz `τ(x,y)=(x²,y²)` applies the C-F2m
  `frobenius`/`square` to *both* coordinates. A `@build` agent that ports `glv_phi` writes the wrong
  automorphism. The `τ^m = id` and `τ²−tτ+2 = 0` KATs are the loud signal — **internal-continue →
  corrected.**

- **The Koblitz speedup must NOT alter the E.G.2 baseline (the Cat-C "never alter the baseline"
  guard).** The baseline binary rho must stay available for E.H/E.W benchmarking; E.G.3's τ-orbit
  variant *reads* it and produces a new variant. A `@build` agent that rewrites the baseline walk
  in-place destroys the benchmark baseline — **internal-continue → corrected** (add a variant, don't
  replace).

- **E.G adds a new edge, not a new crate — a dependency cycle would be a destructive-HALT.**
  `rho → shared-gf2m` is the first edge into the standalone E.F crate. If `cargo check --workspace`
  reports a cycle (it must not — `shared-gf2m` is a leaf depending only on `crypto-bigint`), or if E.G
  finds it must change `shared-gf2m`'s frozen surface beyond filling the two `trace`/`solve_quadratic`
  stubs (it should not), that is a **destructive-HALT** — stop, surface it.

- **No GHS/Weil descent in E.G (defocus guard).** The descent machinery (transferring ECDLP on a
  binary curve to DLP on a hyperelliptic Jacobian over a subfield) is **E.H** (a different sub-track,
  the consumer of C-BinaryCurve, Opus-flagged at E.H.1). A `@build` agent that implements descent in
  E.G is defocus — internal-continue only within the binary-curve + baseline-rho + Koblitz scope.
  *(E.G's job is to expose C-BinaryCurve so E.H can descend without amending it.)*

- **No Cantor's algorithm / hyperelliptic Jacobian in E.G (defocus / scope clarity).** The GF(2^m)
  hyperelliptic Jacobian (Cantor's algorithm, divisor representation, the Jacobian group law) is
  **E.I** (predecessor E.F, a sibling consumer of the field substrate). A `@build` agent that
  implements the Jacobian in E.G is defocus.

- **No MATHEMATICS.md chapter in E.G (defocus / scope clarity).** The binary-curve / Koblitz textbook
  content is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing), not at the
  E.G sub-track ◆. E.G.3 writes at most a PEDAGOGY code-tour delta.

- **No oracle dependency for correctness (principle-3 / E.D/E.E/E.F-consistent).** Binary-curve
  arithmetic is exactly self-checking (point-on-curve + group axioms + decompression round-trip +
  Frobenius-orbit law + end-to-end `k·G=Q`); a PARI `ellinit`/`elllog` cross-check is an **optional
  `#[ignore]` sidecar** (the established `#[ignore = "PARI not installed; run manually when
  available"]` pattern). E.G introduces no new live oracle.

- **Toy curve sizes only (scope clarity).** E.G fixes small `m` and toy binary curves. The toy sizes
  are a principle-4 boundary — the group law and the Koblitz speedup are crypto-scale-correct; only
  the *parameters* are toy. Presenting any as crypto-scale (e.g. claiming the toy curve is a NIST
  binary curve) is a documentation defect (internal-continue → corrected).

- **Static-frame ROADMAP debt (reconcile at the E.G ◆, does NOT block E.G).** The ROADMAP Progress
  subsection is stale by two completed sub-tracks (E.D, E.E) and the Remaining projected sessions
  table still lists the now-complete E.F. The E.G ◆ juncture should update them (Progress: Track E
  Done → E.A–E.F; Remaining: strike E.F). Not an implementation concern.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.G, "*Binary curves + Koblitz automorphism … The order-6 Koblitz
  automorphism the existing rho crate explicitly omits. Re-run rho over GF(2^m) curves as baseline for
  E.H benchmarks. Sonnet.*"; the design statement's principles 1 + 3 + 4; the "On scale"
  mathematical-dimension framing — the binary curve's field GF(2^m) is *mathematical-dimension scale*,
  orthogonal to operational scale) and this PLAN before any session. **NOTE: the ROADMAP Progress /
  Remaining tables are stale by two sub-tracks (E.D, E.E done) and list the now-complete E.F as
  remaining; reconcile at the E.G ◆.**
- Read the **templates to mirror**: `rho/src/curve/mod.rs` (the `Curve` / `AffinePoint` /
  `JacobianPoint` idiom — the *pattern* E.G's `BinaryCurve` mirrors as a parallel type, NOT the type
  to reuse; the Jacobian group law breaks in char 2); `rho/src/ecdlp/{mod,walk,dp}.rs` (the rho-walk
  idiom E.G.2's binary rho mirrors — `Fp<4>`-bound, parallel not reused); `rho/src/ecdlp/glv.rs` (the
  order-3 prime-field endomorphism + orbit-collapse — the idiom E.G.3's Koblitz `τ` *parallels in
  spirit* but shares no code with, `φ(x,y)=(βx,y)` scaling vs `τ(x,y)=(x²,y²)` Frobenius);
  `shared/gf2m/src/{naive,opt,normal}.rs` (the three implementors whose `trace`/`solve_quadratic`
  stubs E.G.1 fills); `rho/tests/ecdlp_kat.rs` (the `check_solver_on_curve` end-to-end `k·G=Q` idiom
  E.G.2 mirrors).
- **Register:** E.G is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New modules `rho/src/binary_curve/` and `rho/src/binary_ecdlp/`, the
  `rho/Cargo.toml` edge, the three `shared/gf2m` stub-fills, and new KATs in `rho/tests/`.
- **Tier routing:** **All three E.G sessions are Sonnet `@build`** — the ROADMAP Opus-flagged-sessions
  table lists E.F.1/E.H.1/E.K.1 but NO E.G session (unlike E.F.1, which ran as an Opus `@architect`
  session). E.G.3 carries the **◆ `@architect` juncture** (page `@plan-juncture`) ratifying
  C-BinaryCurve/C-BinaryRho/C-Koblitz and confirming descent-readiness before the sub-track closes.
  juncture-tier (header) is **opus** — held by lever 3 (C-BinaryCurve is descended-into by E.H, "the
  most mathematically intricate single attack in the project"); the strong lever-5 exactly-checkable
  KATs (point-on-curve + group axioms + `k·G=Q` + Frobenius-orbit) would license `sonnet` in
  isolation, but the user judged the C-BinaryCurve → E.H design-error cost decisive, mirroring the
  E.D/E.E/E.F substrate-adjacent-juncture calls.
- **Invariants to preserve:** **`−P = (x, x+y)`** in char 2 (NOT `(x, −y)`). **The binary group law
  uses the char-2 López–Dahab formula** (NOT Jacobian — the `2P==P+P` KAT is the guard). **`τ(x,y) =
  (x², y²)` is the field Frobenius** (NOT a `glv.rs`-style scaling; the `τ^m=id` / `τ²−tτ+2=0` KATs
  are the guard). **The Koblitz τ-orbit variant reads the E.G.2 baseline, never alters it** (Cat-C —
  the E.H/E.W benchmark baseline must stay intact). **The walk-state invariant `W = a·G + b·Q`** holds
  across the binary rho (the `k·G=Q` end-to-end KAT is the guard). **E.G consumes the frozen C-F2m
  surface** — the only field change is filling the two stubs E.F.1 left for it (additive amend). **No
  GHS/Weil descent** (E.H). **No Cantor / hyperelliptic Jacobian** (E.I). **No MATHEMATICS chapter**
  (T.E at the Track-E ◆). Toy curve sizes only; no new live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional `ellinit`/`elllog` binary-curve
  cross-check follows the established `#[test] #[ignore = "PARI not installed; run manually when
  available"]` pattern; never on the green path.
- **The new edge (load-bearing for E.G).** E.G.1 adds `rho → shared-gf2m` to `rho/Cargo.toml`. `cargo
  check --workspace` must resolve with no cycle (`shared-gf2m` is a leaf depending only on
  `crypto-bigint`; this is the first edge into it). **A new edge, not a new crate** — the binary curve
  + binary rho live in the existing `rho` crate.
- Suggested first invocation: **`/run-plan docs/PLAN.md`** (autonomous cadence — the shard pattern is
  a parallel-type curve + baseline rho + orbit-collapse optimization, structurally proven by the
  prime-field `rho::curve` / `rho::ecdlp` / `glv.rs` precedent it mirrors, so the autonomous cadence is
  warranted). The **single `@architect` marker** (◆ E.G.3) still pages its juncture regardless —
  autonomous means no halt at E.G.1/E.G.2, *not* no juncture at the ◆. *(Tradeoff vs
  `halt-at-boundaries`: autonomous trades a per-boundary human glance for velocity, accepting that the
  curve-design risk concentrated at E.G.1 fails loudly at the E.G.2 `k·G=Q` KAT — the loud signal that
  substitutes for an inline E.G.1 juncture. If E.G.2's end-to-end KAT surfaces a curve-shape concern,
  fall back to `halt-at-boundaries` for E.G.3.)*
