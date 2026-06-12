<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.E — Smart–Satoh–Araki: the p-adic attack on anomalous curves)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) on the E.E.1
point-lift substrate, against a strong lever 5 that would otherwise license an opt-down.** E.E is a
Sonnet sub-track (the ROADMAP flags both E.E sessions Sonnet — the attack is a known algorithm on a
now-frozen p-adic substrate), but the **lift of a curve point from E(F_p) up to E(Z_p) (E.E.1) is a
Category-A representation choice that bounds the whole attack**: how an F_p coordinate lifts to Z_p,
whether E.E lifts x and Hensel-solves the curve equation for y or lifts the affine point wholesale,
and the elliptic formal-group parametrisation the lift must be compatible with — get any of these
wrong and the formal-group log reads a garbage value with no error. Lever 5 is strong and fast (a toy
anomalous curve has a *known* group order p and the ECDLP scalar k is hand-chosen — the recovered k
is an exactly decisive pass/fail KAT, cross-checkable against the existing rho ECDLP solver) and
*would* license `juncture-tier: sonnet` in isolation; the user judged the point-lift design-error
cost (lever 3) high enough to hold the ◆ juncture at **Opus** anyway, mirroring the E.D call. *(Both
E.E sessions are Sonnet `@build`; only the ◆ juncture fork at E.E.2 is Opus.)*

Last rewrite: **E.D ◆ boundary crossed and ROADMAP reconciled** (E.D.1 `0a98148`, E.D.2 `2e84836`,
E.D.3 ◆ `95d03b7`; C-Padic / C-Hensel / C-PadicLog frozen; the p-adic substrate ships — Z_p
arithmetic, Hensel root-lifting, the general formal-group log series). The E.D ◆ Discoveries entry
(ROADMAP 2026-06-10) recorded the `Fp<L>`-is-not-a-ring durable cross-track note and confirmed the
substrate composes (Z_p → Hensel → log homomorphism). **No static-frame debt is outstanding.** Per
the sequencing order, the next un-started sub-track is **E.E — Smart–Satoh–Araki**, the immediate
consumer the E.D substrate was built for: the second structure-based escape in Track E, escaping the
ECDLP search bound through the *p-adic* structure of anomalous curves.

The substrate survey (forked `@explore`, 2026-06-10) established the shape and confirmed every
planning assumption from the E.D intent prose:

1. **E.E is fully greenfield — no SSA / anomalous / formal-group / Satoh / Smart code exists
   anywhere** (zero source hits). E.E builds the attack from scratch in a **new `rho/src/ssa/`
   module**, mirroring the MOV bridge's placement (`rho/src/pairing/mov.rs`) as the structural
   precedent for an alternative-ECDLP-solver-as-a-bridge-into-a-shared-substrate.

2. **E.E adds the `rho → shared-padic` dependency edge** (confirmed absent today: `rho` depends on
   `shared-field`/`shared-bigint`/`shared-numth`/`gnfs`, not `shared-padic`). This is **E.E's edge to
   add**, exactly as the E.D intent prose foretold ("the `rho → shared-padic` edge is E.E's, not
   E.D's"). It is a new workspace dependency, not a new crate.

3. **rho has NO point-counting, trace-of-Frobenius, or anomalous-curve detection** (the stored
   `Curve::n` is a *parameter*, never computed; zero hits for `trace`/`anomalous`/`cardinality`). The
   anomalous fixture (#E(F_p) = p, trace 1) must be **hand-constructed** — a hardcoded toy curve with
   a *naive O(p) point-count verification* helper (the user's decision), **not** a general
   Schoof–SEA point-counting algorithm (out of scope — a different machine). How the fixture was
   *found* is a principle-4 annotation (the disconnect: real anomalous-curve construction is an
   engineering search; the toy fixture is hand-picked).

4. **The p-adic substrate is frozen and general-only.** `shared/padic` exposes `Zp` (C-Padic:
   `add`/`sub`/`mul`/`inv`-of-units/`valuation`/`precision`/`lift`/`truncate`), `hensel_lift(f, r0,
   p, k)` (C-Hensel), and `padic_log(z)` (C-PadicLog — the **general** `log(1+x)` series, requiring
   `v_p(z−1) ≥ 1`). `log.rs` explicitly states "E.E supplies the elliptic-curve specialisation; this
   module ships the general series." **E.E supplies the elliptic formal-group parametrisation** that
   feeds the general log — confirmed, not refuted, by the code.

5. **The curve substrate is `rho`-local and `Uint<4>`-based.** `rho::curve::Curve` (short Weierstrass
   over GF(p), parameters `Uint<4>`), `AffinePoint<F>` / `JacobianPoint<F>` with `F: Fp<4>`,
   `scalar_mul`, `add_jacobian`, `is_on_curve`. `Fp<L>` passes the modulus *per-call* (`p: &Uint<L>`).
   A coordinate lifts to Z_p via `x.to_uint()` → extract the toy-scale limb → `Zp::new(&BigInt, &p,
   k)`. The existing ECDLP solvers (`solve_brent`/`solve_dp*`) take `(curve, g, q, n, …) -> Option<u64>`;
   **E.E mirrors this instance shape but returns `Result<u64, SsaError>`** (deterministic — succeeds,
   or errors "curve not anomalous").

The work splits at **one contract-sharp seam**, **2 sessions** (matching the ROADMAP's 2-session
Sonnet estimate), at the boundary between the *curve substrate E.E must build* and *the attack that
consumes it*:

1. **E.E.1 — Anomalous-curve fixture + point-lift E(F_p) → E(Z_p) (Sonnet, Cat A).** A hardcoded
   anomalous toy curve with an O(p) `#E = p` verification helper; the lift of an affine F_p point to
   Z_p coordinates (lift x, Hensel-solve the curve equation `y² = x³ + ax + b` over Z_p for y). New
   `rho/src/ssa/` module; the `rho → shared-padic` edge. **Freezes C-AnomalousLift** — the curve-lift
   interface E.E.2 builds the attack on.

2. **E.E.2 ◆ — Elliptic formal-group log + log-division reduction + sub-track close (Sonnet,
   Cat B, `@plan`).** The elliptic formal-group logarithm specialising C-PadicLog; the
   "multiply-by-p into the kernel of reduction, then ψ(p·Q̃)/ψ(p·G̃) mod p" SSA reduction; the
   end-to-end ECDLP KAT (recover a hand-chosen k on the anomalous fixture, cross-checked against the
   rho solver). **Freezes C-SSA.** Crosses the **E.E ◆ boundary** (and the Track-E p-adic-branch arc:
   E.D substrate → E.E attack).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing general point-counting /
Schoof–SEA — the fixture is hand-picked with an O(p) verify, not discovered; or building a Q_p
field-of-fractions tower beyond what the formal-group log needs — surface as an additive-reshard if
E.E.2 finds Z_p insufficient; or writing the *p-adic / SSA textbook chapter* in MATHEMATICS.md —
that is **T.E, paired with E.W at the Track-E ◆**, per the per-track-chapter pairing rule; E.E writes
at most a PEDAGOGY code-tour delta) and **rigidity** (re-deriving p-adic arithmetic in `rho` rather
than consuming the frozen `shared/padic` surface; or trying to make `padic_log` curve-aware rather
than supplying the elliptic parametrisation as the E.E layer the survey confirmed it expects).

**Scoping discipline.** E.E builds the SSA attack at **demonstration fidelity** (principle 1 —
algorithmic content complete: the lift, the formal-group log, the division reduction all implemented
head-on) and **toy precision / toy curve** (a small hand-picked anomalous p; fixed small precision k
— enough to recover the scalar, not crypto-scale). It **amends no frozen contract** (C-Padic /
C-Hensel / C-PadicLog / the rho curve surface are all **read**; the `rho → shared-padic` edge is
additive). It introduces **no new live oracle** (the recovered k is exactly checkable against the
known scalar and against the rho ECDLP solver; an optional PARI `Qp`/`elllog` `#[ignore]`
cross-check is the established dev-only pattern). **The "surprise" is the pedagogy** (ROADMAP: "the
most surprising single result in classical ECDLP cryptanalysis; the chapter should make that surprise
explicit") — that an O(log p) division replaces an O(√p) search the instant the curve is anomalous —
and it belongs whole in E.E.2's reduction, exercised by the end-to-end KAT.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.E): "*E.E — Smart–Satoh–Araki. 2 sessions. Predecessor: E.D. Polynomial-time
attack on anomalous curves (trace = 1). The most surprising single result in classical ECDLP
cryptanalysis; the chapter should make that surprise explicit. Sonnet.*" E.E is the **immediate
consumer the E.D p-adic substrate was built for**: where MOV (E.C) escaped the ECDLP search bound via
a *pairing* into a finite field, Smart–Satoh–Araki escapes it via the ***p-adic* structure of
anomalous curves** — and where E.D delivered the general machinery, E.E wires it onto curves and
reads the discrete log off.

The structure-based-escape-from-search through-line, p-adic branch, completed: an **anomalous** curve
(#E(F_p) = p, trace of Frobenius = 1) has the rare property that its order-p group lifts compatibly to
the p-adic numbers. The SSA reduction:

1. **Lift** the base point G and target Q from E(F_p) to *arbitrary* lifts G̃, Q̃ in E(Z_p) reducing
   to them (lift the x-coordinate to Z_p, Hensel-solve `y² = x³ + ax + b` for ỹ). **(E.E.1.)**

2. **Multiply by p** to land p·G̃ and p·Q̃ in the **kernel of reduction** E₁(Z_p) — the points
   reducing to the identity, where the formal group lives and the p-adic elliptic logarithm
   *converges* (this is precisely where C-PadicLog's `v_p ≥ 1` guard is satisfied — the
   multiply-by-p is the load-bearing step that makes the log legal). **(E.E.2.)**

3. **Apply the elliptic formal-group logarithm** ψ (a power series in the local parameter `t = −x/y`,
   specialising C-PadicLog's general series), mapping the kernel isomorphically onto p·Z_p / p²·Z_p ≅
   F_p. The ECDLP `Q = k·G` becomes a **division in F_p**: `k ≡ ψ(p·Q̃) / ψ(p·G̃) mod p`. **No
   search.** **(E.E.2.)**

E.E delivers the two pieces the reduction needs that E.D deliberately left to it (item 5 of the E.D
intent: "E.D delivers the *general* p-adic substrate; E.E consumes it to attack anomalous curves"):

1. **The anomalous-curve fixture + point-lift (E.E.1).** rho has no point-counting; the anomalous
   curve is a hand-picked toy fixture (with an O(p) `#E = p` verify), and the lift of its points to
   Z_p is the Category-A representation choice that bounds the attack. The lift Hensel-solves the
   curve equation over Z_p — the first real *consumer* of C-Hensel outside E.D's own KAT.

2. **The elliptic formal-group log + division reduction (E.E.2).** The elliptic specialisation
   `log.rs` explicitly defers to E.E, the multiply-by-p kernel step, the F_p division, and the
   end-to-end ECDLP recovery — the escape itself, the pedagogical surprise made concrete.

The substrate survey established the shape precisely:

1. **E.E is greenfield in `rho`** — new `rho/src/ssa/` module, the `rho → shared-padic` edge, zero
   prior SSA code. It consumes the frozen `shared/padic` surface (`Zp`, `hensel_lift`, `padic_log`)
   and the existing `rho::curve` machinery (`Curve`, `AffinePoint`, `scalar_mul`, `is_on_curve`).

2. **The p-adic log is general-only; E.E supplies the elliptic parametrisation** (the local parameter
   `t = −x/y` and the formal-group series feeding `padic_log`) — confirmed by `log.rs:41-43`.

3. **The anomalous fixture is hand-picked with an O(p) verify** — no Schoof–SEA (defocus guard). How
   it was found is a principle-4 annotation.

4. **E.E mirrors the existing ECDLP-solver instance shape** (`curve, g, q, n`) but returns
   `Result<u64, SsaError>` (deterministic, mirroring `mov_reduce`'s `Result<u64, E>`, not the rho
   walkers' `Option<u64>`).

Re-read this intent at the ◆ boundary to catch defocus (point-counting, a Q_p tower, the MATHEMATICS
chapter) and rigidity (re-deriving p-adic arithmetic in rho; forcing `padic_log` to be curve-aware).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from E.D; oracle KATs are `#[ignore]`-gated only — the exact form is `#[ignore = "PARI not
installed; run manually when available"]`, used identically in `rho/tests/mov_kat.rs:252` and
`shared/padic/tests/log_kat.rs:202`). `/run-plan` re-discovers at preflight. E.E **adds a new
workspace dependency edge (`rho → shared-padic`)** but **no new crate**, so the gate is a
**correctness + dependency-integrity gate**: each session's KATs (`rho/tests/ssa_kat.rs`) are the
primary correctness signal — fast and *exactly* decisive (lever 5: the anomalous curve's order is a
known p, the ECDLP scalar k is hand-chosen, and the recovered k is cross-checkable against the rho
solver). `cargo check --workspace` must confirm the `rho → shared-padic` edge resolves with **no
dependency cycle** (`shared-padic` depends on `shared-bigint` + `shared-numfield`; `rho` already
depends on `gnfs` + `shared-field/bigint/numth` — adding `shared-padic` introduces no cycle since
`shared-padic` does not depend on `rho`). **The existing rho ECDLP / MOV / pairing KATs must stay
green** after the new module + edge — the gate guards the no-regression invariant on the rho crate.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.E.1 | Anomalous-curve fixture + point-lift E(F_p) → E(Z_p) | A | Sonnet | C-Hensel (frozen E.D.2), C-Padic (frozen E.D.1), `rho::curve` (`Curve`, `AffinePoint`, frozen, read), `Fp<4>` (`shared-field`, read) | `rho/Cargo.toml` (add `shared-padic` dep — the new edge), `rho/src/lib.rs` (add `pub mod ssa;`), `rho/src/ssa/mod.rs` (new: module root + `SsaError` + the anomalous fixture + O(p) `#E=p` verify), `rho/src/ssa/lift.rs` (new: affine F_p point → Z_p lift via Hensel), `rho/tests/ssa_kat.rs` (new) |
| E.E.2 ◆ `@plan` | Elliptic formal-group log + log-division reduction + sub-track close | B | Sonnet | C-AnomalousLift (frozen E.E.1), C-PadicLog (frozen E.D.3, read), C-Padic (read), `rho::ecdlp::solve_brent` (read — KAT cross-check) | `rho/src/ssa/formal_log.rs` (new: elliptic formal-group log specialising `padic_log`), `rho/src/ssa/reduce.rs` (new: multiply-by-p kernel step + F_p division + `ssa_solve` entry), `rho/src/ssa/mod.rs` (add `pub mod formal_log; pub mod reduce;` + re-export `ssa_solve`), `rho/tests/ssa_kat.rs` (extend: end-to-end ECDLP KAT + rho-solver cross-check + optional PARI `#[ignore]`) |

**Sequencing notes.** Strictly serial: **E.E.1 → E.E.2.** E.E.1 lands the module, the dependency
edge, the fixture, and the lift everything stands on; E.E.2 builds the formal-group log + reduction
and closes the sub-track. The single `@plan` marker sits on **E.E.2 ◆** — the Opus boundary juncture
(juncture-tier: opus) ratifying C-AnomalousLift / C-SSA and confirming the attack composes onto the
frozen p-adic substrate before the sub-track closes. E.E.1, though it freezes the Category-A
lift substrate, carries **no** inline `@plan` (C-AnomalousLift is compiler-/test-checkable and is
re-ratified at the E.E.2 ◆ alongside C-SSA — an inline juncture would double the boundary cost on a
2-row Sonnet shard). *(Tradeoff named: the E.E.1 lift is the highest-design-cost session, and a wrong
lift representation is cheapest to catch right after E.E.1 rather than at the ◆. The opt for a single
◆ juncture trades that early-catch insurance for boundary-cost economy on a short shard; `/run-plan
halt-at-boundaries` mitigates by surfacing E.E.1 at its own commit for human eyes even without a
paged fork.)*

**Why 2 sessions (the ROADMAP's Sonnet estimate).** The split is taken at the single contract-sharp
seam between the substrate E.E builds and the attack that consumes it:
- **One-line-commit-title corollary.** "Anomalous-curve fixture + point-lift E(F_p) → E(Z_p)" and
  "Elliptic formal-group log + log-division reduction + sub-track close" are **two distinct commit
  titles** across two categories (A substrate, B algorithm).
- **Contract-sharp boundary (legitimate, not LOC-driven).** E.E.1 **freezes** C-AnomalousLift;
  E.E.2 **consumes** it and **freezes** C-SSA. One real produce/consume seam (the lifted points are
  the input the formal-group log + division operate on).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the lift, the
  attack. Neither fractures below its floor; neither merges across the freeze.

They are **not** further splittable: the fixture + its lift are one unit (the lift exists *for* the
fixture's points; splitting the fixture from the lift leaves a fixture with no consumer and no
contract-sharp seam between them); the formal-group log + the division reduction + the end-to-end KAT
are the irreducible attack unit (a formal-group log with no reduction-and-KAT has an undefined
contract — the log is built *for* the division, and the "surprise" the ROADMAP names lives in the
log→division composition, which must stay whole). Merging E.E.2 into E.E.1 would put the lift, the
log, and the reduction in one >400-LOC two-title session with no freeze checkpoint between the
substrate and the attack.

---

## Session detail

E.E.1 is specified at near-full fidelity (the point-lift representation is the design crux). E.E.2 is
a lower-fidelity sketch, correct per the substrate-first discipline: it is crisply specified only
after C-AnomalousLift freezes.

### E.E.1 — Anomalous-curve fixture + point-lift E(F_p) → E(Z_p) (Sonnet, Cat A)

**Deliverable:** the new `rho/src/ssa/` module, the `rho → shared-padic` dependency edge, a hardcoded
anomalous toy curve with verification, and the lift of an affine F_p point to Z_p coordinates the
attack stands on. The design choices:
- **The new module + edge** (`rho/Cargo.toml`, `rho/src/lib.rs`, `rho/src/ssa/mod.rs`): add
  `shared-padic` to `rho`'s dependencies; `pub mod ssa;`. `cargo check --workspace` must confirm no
  cycle. **Placement decision: `rho/src/ssa/` (a new module in `rho`), per the MOV precedent
  (`rho/src/pairing/mov.rs`)** — the attack lives where the curves and the existing ECDLP solvers
  live, consuming `shared-padic` as MOV consumes `gnfs`.
- **The anomalous fixture + verify** (`rho/src/ssa/mod.rs`): a hardcoded small anomalous `Curve`
  (#E(F_p) = p, trace 1 — a literature or hand-computed toy example), plus a **naive O(p)
  point-count verify helper** (`fn verify_anomalous(curve) -> bool`, iterating x ∈ F_p, counting
  points via the Legendre symbol, asserting `#E = p`). **No Schoof–SEA** (defocus guard). The
  `SsaError` enum lives here (`NotAnomalous`, `LiftFailed(HenselError)`, `Padic(ZpError)`, …).
- **The point-lift** (`rho/src/ssa/lift.rs`): the design crux. Given an `AffinePoint<F>` on E(F_p)
  and a target precision k, lift to Z_p coordinates: lift x via `Zp::new(x.to_uint() limb, p, k)`,
  then **Hensel-solve `y² − (x³+ax+b) = 0` over Z_p** (build the `IntPoly` `g(y) = y² − c` with `c`
  the lifted RHS, call `hensel_lift(g, y0, p, k)` with `y0` the F_p y-coordinate as the simple root
  mod p). Returns the lifted (x̃, ỹ) as a Z_p point. The Opus design call (deferred to the ◆ for
  ratification, proposed here): represent the lifted point as a `(Zp, Zp)` pair vs. a dedicated
  `ZpPoint` struct; lift x-then-solve-y vs. lift both and verify on-curve mod p^k; and the toy-scale
  limb extraction (`to_uint().as_words()[0]`) vs. a full `Uint<4>`→`BigInt` conversion.

Consumes C-Hensel (frozen E.D.2), C-Padic (frozen E.D.1), `rho::curve` (read), `Fp<4>` (read).
**Freezes C-AnomalousLift.**

**KAT** (`rho/tests/ssa_kat.rs`): `verify_anomalous` returns true on the fixture and **false on a
non-anomalous control curve** (the anomalous-detection guard); the lift of a known point matches a
hand-computed Z_p coordinate to precision k; the lifted point **satisfies the curve equation mod p^k**
(`ỹ² ≡ x̃³ + ax̃ + b mod p^k` — the lift-correctness check); a non-simple-root lift case (if
constructible) errors via `HenselError`. **Verify gate:** `cargo test --workspace` green; `cargo
check --workspace` resolves the `rho → shared-padic` edge with no cycle; the existing rho KATs
unchanged.

**Subtlety (load-bearing):** (1) **The lift is *arbitrary*** — SSA lifts G and Q to *any* points
reducing to them; the attack's correctness does not depend on *which* lift (the multiply-by-p and the
log-of-ratio cancel the lift-dependence). Lift x exactly and Hensel-solve y is the clean canonical
choice. (2) **The y-Hensel-solve needs a simple root** — `f'(y0) = 2·y0 ≢ 0 mod p`, i.e. `y0 ≠ 0`
and `p ≠ 2`; a 2-torsion point (y = 0) or p = 2 is a degenerate lift — error or pick a non-2-torsion
base point (the fixture must avoid this). (3) **Toy-scale limb extraction** — `Uint<4>` coordinates
at toy p fit one `u64` limb; extracting `as_words()[0]` is correct *only* at toy scale (principle-4
annotate: a crypto-scale p would need the full `Uint<4>`→`BigInt` path). (4) **No point-counting
machine** — the O(p) verify is a *fixture check*, not Schoof–SEA; presenting it as general
point-counting is defocus.

**Deferred:** the elliptic formal-group log (E.E.2); the multiply-by-p kernel step + division
reduction (E.E.2); the end-to-end ECDLP KAT (E.E.2); any general point-counting (out of scope —
principle-4 boundary); a Q_p field-of-fractions layer (surface at the ◆ if E.E.2's formal-group log
needs denominators Z_p can't carry — an additive-reshard).

### E.E.2 ◆ — Elliptic formal-group log + log-division reduction + sub-track close (Sonnet, Cat B, `@plan`)

**Deliverable:** the elliptic formal-group logarithm specialising C-PadicLog, the SSA division
reduction, the end-to-end ECDLP recovery, and the sub-track close. Lower-fidelity sketch (crisp after
C-AnomalousLift freezes):
- **The elliptic formal-group log** (`rho/src/ssa/formal_log.rs`): given a kernel-of-reduction point
  (the result of multiply-by-p, with `v_p(t) ≥ 1` for the local parameter `t = −x/y`), compute the
  elliptic formal-group logarithm — the local parameter `t` and the formal-group series feeding
  `padic_log` (or the direct elliptic-log series `t + …` truncated to precision k). This is the
  elliptic specialisation `log.rs` explicitly defers to E.E. **Whether E.E.2 calls `padic_log` on
  `1+t`-shaped input or implements the elliptic series directly over `Zp` is the ◆ design call** —
  proposed: compute the local parameter and feed/compose with the frozen general `padic_log` where
  the series shapes match, implementing only the elliptic-specific parametrisation.
- **The reduction** (`rho/src/ssa/reduce.rs`): the `ssa_solve<F: Fp<4>>(curve, g, q, n) ->
  Result<u64, SsaError>` entry. (1) verify the curve is anomalous (`verify_anomalous`, error
  `NotAnomalous` otherwise); (2) lift G and Q (C-AnomalousLift); (3) **multiply both lifts by p**
  (`scalar_mul` over Z_p, or repeated Z_p point-add) to enter the kernel of reduction; (4) apply the
  formal-group log to both; (5) **divide in F_p**: `k ≡ ψ(p·Q̃) · ψ(p·G̃)⁻¹ mod p` (the unit
  inversion is in F_p, the reduction of the p-adic ratio). Return k.
- **End-to-end KAT** (`rho/tests/ssa_kat.rs`, extended): on the anomalous fixture, pick a known
  scalar k, form Q = k·G (via the existing rho `scalar_mul`), run `ssa_solve`, assert it recovers k;
  **cross-check against the rho ECDLP solver** (`solve_brent` recovers the same k — the independent
  confirmation); assert `ssa_solve` **errors `NotAnomalous` on a non-anomalous curve** (the attack's
  precondition guard). **Optional PARI `Qp`/`elllog` `#[ignore]` cross-check** (the established
  dev-only oracle pattern).

Consumes C-AnomalousLift (frozen E.E.1), C-PadicLog (frozen E.D.3, read), C-Padic (read),
`rho::ecdlp::solve_brent` (read). **Freezes C-SSA.**

**KAT (primary correctness signal):** **end-to-end** — `ssa_solve` recovers a hand-chosen k on the
anomalous fixture; the rho ECDLP solver recovers the same k (independent cross-check); `ssa_solve`
errors on a non-anomalous curve; the existing rho + shared KATs stay green. Optional PARI
cross-check. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **Multiply-by-p is the convergence-enabling step** — the
formal-group log converges *only* on the kernel of reduction (`v_p(t) ≥ 1`); the raw lifts G̃, Q̃ are
generally *not* in the kernel, so the log of them would violate C-PadicLog's guard and error/diverge.
The multiply-by-p (since #E(F_p) = p, p·P reduces to the identity, landing in the kernel) is what
makes the log legal. **This is the subtle correctness heart of E.E.2** — name it; the `v_p ≥ 1`
guard from C-PadicLog firing is the loud signal that the kernel step was skipped. (2) **The
lift-dependence cancels** — different arbitrary lifts give different ψ(p·G̃), ψ(p·Q̃), but the
*ratio* is invariant mod p (the formal-group log is a homomorphism; the lift ambiguity is in the
kernel of reduction's higher terms, killed by the mod-p reduction). The KAT should ideally check two
different lifts give the same k (the lift-invariance check). (3) **The final division is in F_p, not
Z_p** — `ψ(p·Q̃) / ψ(p·G̃)` is computed by taking each log's `residue/p mod p` (the p²Z_p/pZ_p ≅
F_p identification) and dividing in F_p; getting the precision bookkeeping wrong (dividing in Z_p, or
at the wrong precision) silently gives a wrong k. (4) **This is the E.E ◆ boundary** — re-read the
Purpose intent and verify the attack composes (lift → multiply-by-p → log → F_p division → recovered
k) and is genuinely the SSA escape (the "surprise": O(log p) division replacing O(√p) search).
(5) **No MATHEMATICS chapter here** — the p-adic/SSA textbook chapter is T.E, paired with E.W at the
*Track-E* ◆ (ROADMAP per-track pairing); E.E.2 writes at most a PEDAGOGY code-tour delta.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.E.2 ◆
to confirm: (1) the attack composes correctly (lift → multiply-by-p into the kernel → formal-group
log → F_p division recovers k; the rho-solver cross-check agrees); (2) C-AnomalousLift's
lift-correctness (`ỹ² ≡ x̃³+ax̃+b mod p^k`) and C-SSA's anomalous-precondition guard (`NotAnomalous`
error on non-anomalous curves) are both in place — the two correctness defenses; (3) the
multiply-by-p kernel step is present and the `v_p ≥ 1` convergence guard from C-PadicLog is
*satisfied* (not bypassed); (4) E.E stayed in scope — no Schoof–SEA / general point-counting (the
fixture is hand-picked + O(p)-verified), no Q_p tower beyond what the log needs, no MATHEMATICS
chapter; (5) the principle-4 boundaries (toy curve, toy precision, hand-picked-fixture-not-discovered,
toy-scale limb extraction) are annotated, not silently presented as crypto-scale. One-shot findings;
does not implement. Held at **Opus** per the header (lever 3 on the E.E.1 point-lift dominates the
strong lever-5 KATs).

---

## Cross-session contracts

E.E **freezes two** contracts and **amends none** (the p-adic substrate and the rho curve surface are
all **read**; the `rho → shared-padic` edge is an additive dependency, not a contract amendment). Per
the substrate-over-specify rule, C-AnomalousLift carries the on-curve-mod-p^k verification and the
explicit precision now even though a toy fixture would "work" with a looser lift.

### C-AnomalousLift — anomalous-curve fixture + point-lift E(F_p) → E(Z_p) (compiler- + test-enforced) — *to be frozen at E.E.1*

**Defined in:** E.E.1 (`rho/src/ssa/lift.rs`, `rho/src/ssa/mod.rs`). **Consumed by:** E.E.2 (the
reduction lifts G and Q through it). Compiler-enforced (the lift fn + `SsaError` + fixture
signatures) + test-enforced (the on-curve-mod-p^k check + the anomalous-vs-control verify). Exposes:
the hardcoded anomalous `Curve` fixture + `verify_anomalous(curve) -> bool` (O(p), toy-scale); the
lift `(point: &AffinePoint<F>, curve, p, k) → (Zp, Zp)` Hensel-solving the curve equation for ỹ;
the `SsaError` enum. *Exact representation (`(Zp, Zp)` vs. a `ZpPoint` struct; lift-x-solve-y vs.
lift-both-verify; the toy-limb extraction convention) ratified at E.E.1 and re-ratified at the E.E.2
◆.* **The lift must satisfy `ỹ² ≡ x̃³+ax̃+b mod p^k`** (the lift-correctness defense). **The y-solve
needs a simple root** (`y0 ≠ 0`, `p ≠ 2`; the fixture avoids 2-torsion base points). **Toy curve /
toy precision only** (principle-4 boundary; no Schoof–SEA).

### C-SSA — Smart–Satoh–Araki ECDLP reduction (compiler- + test-enforced) — *to be frozen at E.E.2 ◆*

**Defined in:** E.E.2 (`rho/src/ssa/reduce.rs`, `rho/src/ssa/formal_log.rs`). **Consumed by:** E.E's
own end-to-end KAT now; **E.W** (the cross-attack benchmark table — "which attack wins on which
curve"; SSA is the polynomial-time winner on anomalous curves); **T.E** (the p-adic/SSA textbook
chapter at the Track-E ◆, the documentation consumer). Compiler- + test-enforced. Exposes the
reduction: `ssa_solve<F: Fp<4>>(curve, g, q, n) -> Result<u64, SsaError>` (mirroring the rho ECDLP
solver instance shape but deterministic, like `mov_reduce`); the elliptic formal-group log
specialising C-PadicLog. *Exact signature (direct elliptic series vs. composing `padic_log`; the
local-parameter convention) ratified at the E.E.2 ◆.* **Multiply-by-p enters the kernel of reduction
before the log** (the convergence-enabling step; C-PadicLog's `v_p ≥ 1` guard must be *satisfied*,
not bypassed). **The final division is in F_p** (the p²Z_p/pZ_p ≅ F_p identification). **Errors
`NotAnomalous` on non-anomalous curves** (the attack's precondition — the structure-based escape
exists *only* for trace-1 curves).

### Frozen contracts read by E.E (consumed, not amended)

- **C-Padic** (`shared/padic/src/zp.rs`, frozen E.D.1) — `Zp` arithmetic; the ring the lift and log
  compute over. Read, not changed.
- **C-Hensel** (`shared/padic/src/hensel.rs`, frozen E.D.2) — `hensel_lift(f, r0, p, k)`; the lift
  E.E.1's y-solve calls. **First real consumer outside E.D's own KAT.** Read.
- **C-PadicLog** (`shared/padic/src/log.rs`, frozen E.D.3) — `padic_log(z)`, the general
  formal-group series with the `v_p(z−1) ≥ 1` convergence guard; **E.E.2 supplies the elliptic
  specialisation that feeds it** (the deferral `log.rs` explicitly names). Read.
- **`rho::curve`** (`Curve`, `AffinePoint`, `scalar_mul`, `is_on_curve`, frozen) — the curve
  substrate E.E lifts from and recovers the scalar on. Read.
- **`rho::ecdlp::solve_brent`** (frozen) — the independent ECDLP solver E.E.2's KAT cross-checks
  against. Read.
- **`Fp<4>`** (`shared/field`, per-call modulus, frozen) — the field the coordinates live in. Read.

### Additive dependency edge (not a contract amendment)

- **`rho → shared-padic`** — E.E.1 adds `shared-padic` to `rho/Cargo.toml`. **Additive** (a new
  dependency; no `shared-padic` or `rho` API changes). `cargo check --workspace` confirms no cycle
  (`shared-padic` does not depend on `rho`). *(If E.E found it must change a `shared-padic` signature
  — it should not; the surface is general by E.D's over-specify design — that is an additive-reshard
  of E.D's frozen contract surfaced at the ◆, never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.E.2 ◆ `@plan` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.E.1 | Anomalous-curve fixture + point-lift E(F_p) → E(Z_p) | done | 4dc5eaf | C-AnomalousLift (frozen) + `rho → shared-padic` edge |
| E.E.2 ◆ | Elliptic formal-group log + log-division reduction + sub-track close | done | 8a217f4 | C-SSA (frozen) |

Contracts frozen before this sub-track (read by E.E): C-Padic / C-Hensel / C-PadicLog
(`shared/padic`); the rho curve + ECDLP surface; `Fp<4>`. This sub-track **freezes two new
contracts** (C-AnomalousLift, C-SSA), serving the downstream **E.W** (cross-attack benchmark) and
**T.E** (p-adic textbook chapter) consumers, and **completes the p-adic branch of Track E** (E.D
substrate → E.E attack).

---

## Action-frame digest

### E.E.2 — 2026-06-12
Discovery/flex: Fixture y²=x³+5/F_7 has CM by Z[ζ₃] (a=0, p≡1 mod 3); v_p(T)=2 instead of 1; raw SSA formula gives k_raw=2k mod p. Discovery adjudication: internal-continue. ◆ juncture: still-on-intent. Fix applied: O(p) verify_and_search replaced with O(1) CM correction (k_raw·inv(2,p) mod p, verified by one scalar_mul). Precision bumped k=4→k=8 (fixture-dependent: k > 2·v_p(T)). All five ◆ confirmation points satisfied.
Affected: C-SSA (frozen at this ◆)
Deferred: yes — ◆ juncture flagged for T.E pedagogy: the CM correction is a permanent annotated wrinkle; if T.E wants the canonical SSA presentation without a CM caveat, a non-CM fixture (a≠0, v_p(T)=1) is the cleaner option. Not a blocker for E.W or T.E consumers of the frozen C-SSA signature.
Texture: The multiply-by-p kernel step, formal-group log, and F_p division are all structurally correct and present. The convergence guard v_p≥1 is satisfied (not bypassed) — elliptic_log_proj pre-checks vt≥1 before calling padic_log. The rho-solver cross-check (solve_brent) independently confirms k recovery.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.E is greenfield SSA in `rho` — building the module + edge + fixture + lift + log + reduction is
  internal-continue (confirmed by survey).** No SSA code exists; E.E writes it in `rho/src/ssa/`. A
  discovery that the elliptic formal-group log needs a Q_p field-of-fractions layer Z_p can't carry
  (denominators with negative valuation) is an **additive-reshard** surfaced at the E.E.2 ◆ (the same
  Q_p-tower deferral E.D flagged).

- **The multiply-by-p kernel step is mandatory before the log (the silent-divergence guard).** The
  elliptic formal-group log converges only on the kernel of reduction (`v_p(t) ≥ 1`). A `@build`
  agent that applies the log to the *raw* lifts G̃, Q̃ (not p·G̃, p·Q̃) hits C-PadicLog's convergence
  guard (loud error) or, worse, computes a precision-limited garbage value — **internal-continue →
  corrected**. The multiply-by-p (exploiting #E = p so p·P reduces to identity) is the
  convergence-enabling step; its omission is the subtle failure mode.

- **The lift must satisfy the curve equation mod p^k (the lift-correctness guard).** A `@build` agent
  that lifts x and y *independently* (without Hensel-solving y from the lifted x via the curve
  equation) produces a point *not on* E(Z_p) — `ỹ² ≢ x̃³+ax̃+b mod p^k` — and the formal-group log
  reads a garbage value. The on-curve-mod-p^k KAT is the loud signal — **internal-continue →
  corrected**.

- **The attack errors on non-anomalous curves (the precondition guard).** SSA's escape exists *only*
  for trace-1 (anomalous) curves; `ssa_solve` must verify `#E = p` and error `NotAnomalous`
  otherwise. A `@build` agent that runs the reduction on a non-anomalous curve and returns a
  plausible-but-wrong k (the division still computes a value) has a missing precondition guard —
  **internal-continue → corrected**. The non-anomalous-control KAT catches it.

- **E.E adds the `rho → shared-padic` edge additively — a dependency cycle is a destructive-HALT.**
  Adding `shared-padic` to `rho`'s deps is additive (internal-continue). If `cargo check --workspace`
  reports a cycle (it must not — `shared-padic` does not depend on `rho`), or if E.E finds it must
  change a frozen `shared-padic` signature, that is a **destructive-HALT** — stop, surface it.

- **No general point-counting in E.E (defocus guard).** The anomalous fixture is hand-picked with an
  O(p) `#E = p` verify; Schoof–SEA / general point-counting is **out of scope** (a different
  machine). A `@build` agent that implements general point-counting is defocus — internal-continue
  only within the fixture-verify scope.

- **No MATHEMATICS.md chapter in E.E (defocus / scope clarity).** The p-adic/SSA textbook chapter is
  **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing rule), not at the E.E
  sub-track ◆. A `@build` agent that writes a MATHEMATICS.md SSA chapter during E.E is defocus; E.E.2
  writes at most a PEDAGOGY code-tour delta. **(The "surprise" the ROADMAP names is the *code's*
  pedagogical payoff — exercised by the KAT — not a chapter E.E owns.)**

- **The end-to-end KAT must recover a HAND-CHOSEN k AND cross-check the rho solver (correctness
  discipline).** A wrong reduction can return a plausible-but-wrong scalar; only the known-k check
  *plus* the independent `solve_brent` cross-check (and, ideally, the two-different-lifts invariance
  check) catches it. A KAT that only asserts `ssa_solve` returns *something* has an under-specified
  contract — flag it.

- **No oracle dependency for correctness (principle-3 / E.D/E.C-consistent).** SSA's recovered k is
  exactly checkable (known scalar + rho-solver cross-check); a PARI `Qp`/`elllog` cross-check is an
  **optional `#[ignore]` sidecar** (the established `#[ignore = "PARI not installed; run manually
  when available"]` pattern). E.E introduces no new live oracle.

- **Toy curve / toy precision only (scope clarity).** E.E fixes a small anomalous p and small
  precision k (principle 1 — complete algorithm; principle 4 — toy scale). The toy-scale limb
  extraction (`to_uint().as_words()[0]`), the hand-picked-not-discovered fixture, and the small k are
  principle-4 boundaries, annotated — presenting any as crypto-scale is a documentation defect
  (internal-continue → corrected).

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.E, "*Smart–Satoh–Araki … polynomial-time attack on anomalous
  curves (trace = 1) … the most surprising single result … make that surprise explicit. Sonnet.*";
  the design statement's principles 1 + 4; the "On scale" mathematical-dimension framing) and this
  PLAN before any session. **The ROADMAP is reconciled through the E.D ◆ boundary (2026-06-10)** —
  no static-frame debt is outstanding; the Progress table and Discoveries log are current.
- Read the **templates to mirror**: `rho/src/pairing/mov.rs` (the MOV bridge — the structural
  precedent for an alternative-ECDLP-solver-into-a-shared-substrate: module placement, `Result<u64,
  E>` return, the `<F: Fp<4>>` generic shape); `rho/src/ecdlp/mod.rs` (`solve_brent` — the ECDLP
  instance shape `(curve, g, q, n)` to mirror, and the cross-check target); `rho/src/curve/mod.rs`
  (`Curve`, `AffinePoint`, `scalar_mul`, `is_on_curve` — the curve surface E.E lifts from);
  `shared/padic/src/{zp.rs,hensel.rs,log.rs}` (the frozen `Zp` / `hensel_lift` / `padic_log` surface
  — read the `log.rs:41-43` deferral note: E.E supplies the elliptic specialisation);
  `shared/padic/tests/log_kat.rs` and `rho/tests/mov_kat.rs` (the `_kat.rs` idiom + the exact PARI
  `#[ignore = "PARI not installed; run manually when available"]` dev-oracle pattern).
- **Register:** E.E is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module `rho/src/ssa/` with `{mod,lift,formal_log,reduce}.rs` and the
  extended `rho/tests/ssa_kat.rs`. The `rho → shared-padic` edge goes in `rho/Cargo.toml`.
- **Tier routing:** **both E.E sessions are Sonnet** (`@build` on Sonnet) — the ROADMAP flags E.E
  Sonnet (known attack on a frozen substrate). E.E.2 carries the single `@plan` marker: a
  ◆-boundary juncture (page `@plan-juncture`) ratifying C-AnomalousLift / C-SSA and confirming the
  attack composes onto the p-adic substrate before the sub-track closes. juncture-tier (header) is
  **opus** — held by lever 3 (the E.E.1 point-lift is the design crux bounding the attack); the
  strong lever-5 exactly-checkable KATs (known k + rho-solver cross-check) would license `sonnet` in
  isolation but the user judged the lift design-error cost decisive, mirroring the E.D call.
- **Invariants to preserve:** **the lift satisfies the curve equation mod p^k** (the on-curve check
  — C-AnomalousLift). **Multiply-by-p before the log** (the kernel-of-reduction /
  convergence-enabling step — C-SSA; C-PadicLog's `v_p ≥ 1` guard must be satisfied, not bypassed).
  **The final division is in F_p** (the p²Z_p/pZ_p ≅ F_p identification). **`ssa_solve` errors on
  non-anomalous curves** (the precondition guard — C-SSA). **E.E consumes `shared/padic`, never
  re-derives p-adic arithmetic in rho.** **No general point-counting** (hand-picked fixture + O(p)
  verify). **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy curve / toy precision only; no new
  live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional `Qp`/`elllog` SSA cross-check follows
  the established `#[test] #[ignore = "PARI not installed; run manually when available"]` pattern;
  never on the green path.
- **The new dependency edge (load-bearing for E.E).** E.E.1 adds `shared-padic` to `rho/Cargo.toml`.
  `cargo check --workspace` must resolve with no cycle (`shared-padic` depends only on
  `shared-bigint` + `shared-numfield`; it does not depend on `rho`). No new crate — a module + an
  edge.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — E.E opens a new
  attack module + a new cross-crate dependency edge (the point-lift is a Category-A design crux), and
  the shard pattern (an alternative ECDLP solver bridging rho into the p-adic substrate) is unproven
  for this sub-track; the conservative halt-at-◆ cadence is warranted, and it surfaces the E.E.1
  lift substrate at its own commit for human eyes even though the only paged ◆/`@plan` juncture sits
  on E.E.2. *(Tradeoff vs default cadence: one extra halt-confirm on a 2-session Sonnet sub-track is
  cheap insurance on a new attack-module substrate + a new dependency edge.)*
