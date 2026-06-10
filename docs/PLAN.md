<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.D — p-adic arithmetic: Z_p tower, Hensel lifting, p-adic log)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) on the E.D.1
substrate, against a strong lever 5 that would otherwise license an opt-down.** E.D is a Sonnet
sub-track (the ROADMAP flags every E.D session Sonnet — p-adic arithmetic is well-understood
material), but the **Z_p / Z/p^k arithmetic substrate (E.D.1) is a Category-A representation choice
that bounds Hensel lifting, the p-adic log, and the downstream E.E (Smart–Satoh–Araki) attack** —
get the precision/valuation model or the prime-power-vs-prime distinction wrong and the rework
propagates through the whole sub-track and into E.E. Lever 5 is strong and fast (p-adic results are
*exactly* hand-checkable — a Hensel lift to mod p^k and a formal-group log are deterministic
known-answer values, decisive KATs) and *would* license `juncture-tier: sonnet` in isolation; the
user judged the substrate design-error cost (lever 3) high enough to hold the ◆ juncture at **Opus**
anyway. *(All three E.D sessions are Sonnet `@build`; only the ◆ juncture fork at E.D.3 is Opus.)*

Last rewrite: **E.C ◆ boundary crossed and ROADMAP reconciled** (E.C.1 `840608c`, E.C.2 ◆
`2e1edf8`; C-MovBridge / C-Mov frozen; the MOV/Frey–Rück cross-track bridge ships, ECDLP→DLP through
a real NFS-DL solver proven at k=2). The E.C ◆ Discoveries entry (ROADMAP 2026-06-10) reconciled the
Progress table (done ~57, remaining ~35–47) and recorded the E.A/E.B/D.E closeouts; **no static-frame
debt is outstanding.** Per the sequencing order, the next un-started sub-track is **E.D — p-adic
arithmetic**, opening Track E's second half (the structure-based attacks that escape search through
*p-adic* structure rather than pairings).

The substrate survey (forked `@explore`, 2026-06-10) established the shape and surfaced the design
crux:

1. **E.D is fully greenfield — no p-adic code exists anywhere** (`padic`/`hensel`/`valuation`/`Zp`/`Qp`
   return zero source hits). E.D builds the substrate from scratch in a **new `shared/padic`
   workspace crate** (the user's placement decision), mirroring the existing
   `shared/{bigint,field,numth,numfield}` decomposition.

2. **`Fp<L>` cannot be reused for the p-adic tower — this is the load-bearing substrate fact.**
   `shared/field`'s `Fp<L>` trait assumes a **prime** modulus (`inv` uses Fermat's little theorem,
   `sqrt` assumes p prime). Z/p^k for k>1 is a ring with zero divisors, *not* a field; `Fp::inv`
   would silently return wrong results for a composite modulus. E.D.1 introduces a distinct
   prime-power arithmetic type. **This is the highest-cost-of-wrong silent-failure mode in E.D** — a
   units-vs-non-units confusion in Z/p^k produces wrong lifts with no error.

3. **The polynomial machinery needs two additive extensions.** Hensel lifting evaluates `f` and `f'`
   mod p^k; `IntPoly` (`shared/numfield/src/poly.rs`, G.A-frozen) has **neither a formal
   `derivative()` nor any mod-m coefficient reduction / `eval_mod`**. Per the user's decision, E.D
   **extends `IntPoly` additively** (new `derivative()` + `eval_mod(x, m)` methods, no change to any
   existing signature) rather than duplicating polynomial logic in `shared/padic`. This is an
   additive edit to a frozen shared type — recorded, low-risk, surfaced at the ◆.

4. **E.D adds a `rho → shared-padic` edge for the downstream consumer, but E.D itself is
   crate-internal to `shared`.** The p-adic substrate, Hensel lift, and p-adic log all live in
   `shared/padic`; they consume only `shared/bigint` (`BigInt`, `gcd`) and `shared/numfield`
   (`IntPoly`). The `rho → shared-padic` dependency is added by **E.E** (Smart–Satoh–Araki, which
   lifts curve points), *not* by E.D. E.D's KATs live in `shared/padic/tests/` and exercise the
   arithmetic + lift + log against hand-computed values — no curve, no rho dependency.

5. **The downstream consumer is E.E (Smart–Satoh–Araki), not this sub-track.** E.D delivers the
   *general* p-adic substrate (Z_p arithmetic, Hensel root-lifting, the formal-group / p-adic
   logarithm series). E.E consumes it to attack anomalous curves (#E(F_p) = p, trace 1). **E.D does
   NOT touch curves, point-counting, or trace-of-Frobenius** (the survey confirmed none exists; that
   is E.E's concern, against a hardcoded anomalous fixture). Building any of that in E.D is defocus.

The work splits at two contract-sharp seams, **3 sessions** (matching the ROADMAP's 3-session
Sonnet estimate):

1. **E.D.1 — `shared/padic` crate + Z_p / Z/p^k arithmetic substrate (Sonnet, Cat A).** New
   workspace member `shared/padic`; the prime-power-modulus arithmetic type (the p-adic number as
   valuation + unit, or fixed-precision residue mod p^N — the representation choice is the design
   crux); add/sub/mul/inv-of-units/valuation/precision. **Freezes C-Padic** — the p-adic-number
   interface E.D.2 and E.D.3 build on.

2. **E.D.2 — Hensel lifting + `IntPoly` derivative/mod-eval (Sonnet, Cat B).** Newton iteration
   lifting a simple root of `f` mod p to mod p^k (the quadratic-convergence lift, `r ← r −
   f(r)/f'(r) mod p^{2k}`); the additive `IntPoly::derivative()` + `IntPoly::eval_mod()` the lift
   consumes. **Freezes C-Hensel.**

3. **E.D.3 ◆ — p-adic logarithm + sub-track close (Sonnet, Cat B, `@plan`).** The formal-group /
   p-adic logarithm series E.E's SSA reduction consumes; the end-to-end KAT (lift + log recovers a
   hand-computed p-adic value); the sub-track close. **Freezes C-PadicLog.** Crosses the **E.D ◆
   boundary.**

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the *Smart–Satoh–Araki
attack itself* — the curve lift, anomalous-curve detection, trace-of-Frobenius / point-counting —
that is E.E, not E.D; E.D delivers the general p-adic substrate. Or writing the *p-adic textbook
chapter* in MATHEMATICS.md — that is **T.E, paired with E.W at the Track-E ◆**, per the ROADMAP's
per-track-chapter pairing rule; E.D writes at most a PEDAGOGY code-tour delta, not a textbook
chapter. Or adding a `rho → shared-padic` edge — that is E.E's, not E.D's; E.D is `shared`-internal)
and **rigidity** (using `Fp` for the p-adic tower because "it's already there" — `Fp::inv` is wrong
mod p^k; the prime-power type is mandatory. Or duplicating `IntPoly`'s polynomial logic in
`shared/padic` rather than extending `IntPoly` — the derivative/mod-eval are general polynomial ops
and belong on `IntPoly`, per the placement decision).

**Scoping discipline.** E.D builds p-adic arithmetic at **demonstration fidelity** (principle 4) and
**toy precision** (fixed, small k — enough to exhibit Hensel convergence and the log, not
crypto-scale precision towers). It **amends no frozen contract** except the **additive** `IntPoly`
extension (new methods only). It introduces **no new live oracle** (p-adic values are exactly
hand-computable; an optional PARI/`Qp` `#[ignore]` cross-check is the established dev-only pattern).
A crypto-scale anomalous-curve SSA attack is a principle-4 boundary and an E.E work item, not E.D.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.D): "*E.D — p-adic arithmetic. 3 sessions. Predecessors: shared bigint, G.A
(polynomial machinery). Hensel lifting; p-adic logarithm. Sonnet.*" E.D is the substrate sub-track
for the **second structure-based escape in Track E**: where MOV (E.C) escaped the ECDLP search bound
via a *pairing* homomorphism into a finite field, Smart–Satoh–Araki (E.E, the immediate consumer)
escapes it via the ***p-adic* structure of anomalous curves** — lifting the curve to Z_p and reading
the discrete log off the formal-group logarithm. E.D builds the p-adic machinery that escape stands
on; it does not build the attack.

The structure-based-escape-from-search through-line, p-adic branch: an anomalous curve (#E(F_p) = p,
trace of Frobenius = 1) has the rare property that its group lifts compatibly to the p-adic numbers,
and the p-adic *elliptic* logarithm — a power series convergent on the kernel of reduction — maps the
order-p subgroup isomorphically onto the additive group p·Z_p / p²·Z_p ≅ F_p. The ECDLP `Q = k·G`
becomes a *division* in F_p: `k = log_p(Q) / log_p(G)`. No search. E.D delivers the three substrate
pieces that reduction calls:

1. **Z_p / Z/p^k arithmetic (E.D.1).** The ring the lift lives in. The crux is that Z/p^k is *not* a
   field: only units (elements coprime to p) are invertible, and the p-adic valuation `v_p(·)` is the
   organising invariant. The representation (valuation + unit form, or fixed-precision residue) is
   the Category-A decision that bounds everything above it.

2. **Hensel lifting (E.D.2).** Newton's method over Z_p: a simple root of `f` mod p lifts *uniquely*
   to a root mod p^k, with the iteration doubling precision each step (`r_{n+1} = r_n − f(r_n) ·
   f'(r_n)^{-1}` mod the next power). This is the lift that takes a curve point's F_p coordinates up
   to Z_p coordinates — and the reason E.D.2 needs `IntPoly::derivative` and a mod-p^k evaluation.

3. **The p-adic logarithm (E.D.3).** The formal-group log series `log(1+x) = x − x²/2 + x³/3 − …`
   (and the elliptic-curve formal-group log E.E specialises), convergent p-adically on the kernel of
   reduction. This is the homomorphism that turns the multiplicative ECDLP into additive division —
   the escape itself, exercised at toy precision by E.D's KAT and consumed in full by E.E.

The substrate survey established the shape precisely:

1. **E.D is greenfield — no p-adic code, no chapter, no fixture.** The substrate is built from
   scratch in a new `shared/padic` crate. The only existing pieces it consumes are `shared/bigint`
   (`BigInt`, `gcd`) and `shared/numfield`'s `IntPoly` (extended).

2. **`Fp` is the wrong tool for the tower (the rigidity guard).** `shared/field`'s `Fp<L>` assumes a
   prime modulus; Z/p^k arithmetic must be a distinct type with units-only inversion and an explicit
   valuation. Reusing `Fp` is the silent-wrong-answer failure mode.

3. **`IntPoly` is extended additively (the placement decision).** `derivative()` and `eval_mod(x, m)`
   are added to `IntPoly` in `shared/numfield` — general polynomial ops that belong there, consumed
   by E.D.2's Hensel iteration. No existing `IntPoly` signature changes.

4. **E.D is `shared`-internal; the `rho → shared-padic` edge is E.E's.** E.D's tests live in
   `shared/padic/tests/` and check arithmetic/lift/log against hand-computed values. The curve-side
   wiring (and the dependency edge) is E.E.

Re-read this intent at the ◆ boundary to catch defocus (the SSA attack, point-counting, the
MATHEMATICS chapter — all out of E.D scope) and rigidity (`Fp` misuse, `IntPoly` duplication).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from E.C; oracle KATs are `#[ignore]`-gated only, no `oracle-tests` feature). `/run-plan`
re-discovers at preflight. E.D **adds a new workspace member (`shared/padic`)**, so the gate is a
**correctness + workspace-integrity gate**: each session's KATs (`shared/padic/tests/*_kat.rs`) are
the primary correctness signal — fast and *exactly* decisive (lever 5: p-adic lifts and the log are
hand-computable known-answer values). `cargo check --workspace` must confirm the new `shared/padic`
member resolves cleanly (added to root `Cargo.toml` `members`, depends only on `shared-bigint` +
`shared-numfield`, no cycle). **The G.A `IntPoly` consumers (the NFS pipeline in `gnfs`) must stay
green** after the additive `IntPoly` extension — the gate guards the no-regression invariant on the
shared polynomial type.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.D.1 | New `shared/padic` crate + Z_p / Z/p^k arithmetic substrate | A | Sonnet | `shared/bigint` (`BigInt`, `gcd`, frozen, read) | `Cargo.toml` (add `shared/padic` member), `shared/padic/Cargo.toml` (new), `shared/padic/src/lib.rs` (new: crate root + `pub mod`), `shared/padic/src/zp.rs` (new: the Z/p^k / Z_p arithmetic type), `shared/padic/tests/zp_kat.rs` (new) |
| E.D.2 | Hensel lifting + `IntPoly::derivative`/`eval_mod` | B | Sonnet | C-Padic (frozen E.D.1), `IntPoly` (`shared/numfield`, frozen G.A, **extend additively**) | `shared/numfield/src/poly.rs` (add `derivative()` + `eval_mod()` to `IntPoly`, additive), `shared/padic/src/hensel.rs` (new: Newton root-lift), `shared/padic/src/lib.rs` (add `pub mod hensel;`), `shared/padic/Cargo.toml` (add `shared-numfield` dep), `shared/padic/tests/hensel_kat.rs` (new) |
| E.D.3 ◆ `@plan` | p-adic logarithm + sub-track close | B | Sonnet | C-Padic (frozen E.D.1, read), C-Hensel (frozen E.D.2, read) | `shared/padic/src/log.rs` (new: formal-group / p-adic log series), `shared/padic/src/lib.rs` (add `pub mod log;`), `shared/padic/tests/log_kat.rs` (new: end-to-end lift+log KAT + optional PARI `#[ignore]` cross-check) |

**Sequencing notes.** Strictly serial: **E.D.1 → E.D.2 → E.D.3.** E.D.1 lands the crate and the
arithmetic everything stands on; E.D.2 builds the lift (and the polynomial support it needs); E.D.3
builds the log and closes the sub-track. The single `@plan` marker sits on **E.D.3 ◆** — the Opus
boundary juncture (juncture-tier: opus) ratifying C-Padic / C-Hensel / C-PadicLog before the
sub-track closes. E.D.1, though it freezes the Category-A substrate, carries **no** inline `@plan`
(C-Padic is compiler-/test-checkable and is re-ratified at the E.D.3 ◆ alongside C-Hensel /
C-PadicLog — an inline juncture would double the boundary cost on a 3-row Sonnet shard).
*(Tradeoff named: the E.D.1 substrate is the highest-design-cost session, and a wrong representation
is cheapest to catch right after E.D.1 rather than two sessions later at the ◆. The opt for a single
◆ juncture trades that early-catch insurance for boundary-cost economy on a short shard; `/run-plan
halt-at-boundaries` partially mitigates by surfacing E.D.1 at its own commit for human eyes even
without a paged fork.)*

**Why 3 sessions (the ROADMAP's Sonnet estimate).** The split is taken at the two contract-sharp
seams:
- **One-line-commit-title corollary.** "New `shared/padic` crate + Z_p arithmetic substrate",
  "Hensel lifting + `IntPoly` derivative/mod-eval", and "p-adic logarithm + sub-track close" are
  **three distinct commit titles** across two categories (A substrate, B algorithm ×2).
- **Contract-sharp boundaries (legitimate, not LOC-driven).** E.D.1 **freezes** C-Padic; E.D.2
  **consumes** it and **freezes** C-Hensel; E.D.3 **consumes** both and **freezes** C-PadicLog. Two
  real produce/consume seams.
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the arithmetic
  ring, the lift, the log. None fractures below its floor; none merges across a freeze.

They are **not** further splittable: the Z_p arithmetic is one ring (splitting add/mul from inversion
fractures a unit with no contract-sharp seam between them); Hensel + its polynomial support is one
lift (the `derivative`/`eval_mod` exist *for* the lift); the log + its KAT is the irreducible
end-to-end unit (a log with no lift+log KAT has an undefined contract). Merging E.D.2 into E.D.1 would
put the arithmetic ring, the lift, and the frozen-`IntPoly` edit in one >400-LOC two-title session
with no freeze checkpoint between the substrate and its first consumer.

---

## Session detail

E.D.1 is specified at near-full fidelity (the Z_p representation is the design crux). E.D.2 and E.D.3
are lower-fidelity sketches, correct per the substrate-first discipline: they are crisply specified
only after C-Padic freezes.

### E.D.1 — `shared/padic` crate + Z_p / Z/p^k arithmetic substrate (Sonnet, Cat A)

**Deliverable:** a new `shared/padic` workspace crate and the prime-power-modulus arithmetic type the
whole sub-track stands on. The design choices:
- **The new crate** (`Cargo.toml` member + `shared/padic/Cargo.toml`): add `shared/padic` to the
  workspace `members`; the crate depends only on `shared-bigint` (and `num-bigint`/`num-traits` as
  needed). `cargo check --workspace` must confirm no cycle. **Crate placement decision: `shared/padic`
  (a new sibling crate), per the user — not a module in `shared/field` (the prime-field crate, where
  Z/p^k would invite the exact prime-modulus confusion) nor `shared/numfield`.**
- **The Z_p / Z/p^k arithmetic type** (`shared/padic/src/zp.rs`): the design crux. Decide the
  representation — **the recommended shape is a `BigInt` residue carried with an explicit precision
  `k` (the modulus is p^k) plus the p-adic valuation as the organising invariant**, exposing
  `add`/`sub`/`mul`, **unit inversion** (`inv` defined only on elements with `v_p = 0`; a
  non-unit-inversion attempt is an error, *not* a silent wrong answer — the C-Padic guard), `valuation`,
  `precision`, and lift/truncate between precisions. The Opus design call (deferred to the ◆ for
  ratification, but proposed here): valuation+unit form vs. fixed-precision residue, and whether `p`
  is a `BigInt` field on the element or threaded per-call (mirroring `Fp`'s pass-the-modulus
  convention for consistency, or storing it for ergonomics in a precision tower).

Consumes `shared/bigint` (`BigInt`, `gcd`, read). **Freezes C-Padic.**

**KAT** (`shared/padic/tests/zp_kat.rs`): hand-computed Z/p^k arithmetic at a toy prime (e.g. p=7,
k=4): `add`/`mul` against known residues; **unit inversion** correct on a unit and **errors loudly on
a non-unit** (the prime-power-non-field guard — the highest-cost-of-wrong check); `valuation`
correct on sample elements (e.g. `v_7(49) = 2`). **Verify gate:** `cargo test --workspace` green;
`cargo check --workspace` resolves the new member with no cycle; the existing `shared` KATs unchanged.

**Subtlety (load-bearing):** (1) **Z/p^k is not a field** — only units (`v_p = 0`) invert; inversion
of a non-unit must error, never return a plausible-wrong value. This is the `Fp`-misuse defense made
explicit in the type. (2) **Precision is finite and explicit** — every operation carries a precision
`k`; mixing precisions must be defined (truncate to the min, the standard convention). (3) **Toy
precision only** — k is small (principle 4); a crypto-scale precision tower is out of scope. (4) **Do
not reuse `Fp`** — `Fp::inv` (Fermat) is wrong for composite p^k; the new type is mandatory.

**Deferred:** Hensel lifting (E.D.2); the p-adic log (E.D.3); the `IntPoly` extension (E.D.2, where
it is needed); any curve/point lift (E.E); a Q_p (field-of-fractions) layer beyond Z_p if the log
needs denominators — surface at the ◆ if E.D.3 finds Z_p insufficient (an additive-reshard).

### E.D.2 — Hensel lifting + `IntPoly::derivative`/`eval_mod` (Sonnet, Cat B)

**Deliverable:** Newton-iteration Hensel lifting over Z_p, plus the additive `IntPoly` support it
consumes. Lower-fidelity sketch (crisp after C-Padic freezes):
- **The `IntPoly` extension** (`shared/numfield/src/poly.rs`, additive): `IntPoly::derivative() ->
  IntPoly` (formal derivative, `c_i·i` shifted down) and `IntPoly::eval_mod(&self, x: &BigInt, m:
  &BigInt) -> BigInt` (Horner reduced mod m at each step). **Additive only — no existing signature
  changes; the G.A NFS consumers are untouched.**
- **The Hensel lift** (`shared/padic/src/hensel.rs`): given `f: &IntPoly`, a simple root `r_0` of `f`
  mod p (with `f'(r_0) ≢ 0 mod p`), and a target precision k, lift to the unique root mod p^k via
  Newton's method (`r ← r − f(r)·f'(r)^{-1}` over C-Padic, doubling precision per step). Errors if
  the root is not simple (`f'(r_0) ≡ 0`) — the lift is only unique for simple roots.

Consumes C-Padic (frozen E.D.1) and `IntPoly` (extended). **Freezes C-Hensel.**

**KAT** (`shared/padic/tests/hensel_kat.rs`): lift a known simple root — e.g. the square root of 2
mod 7^k (`3² = 9 ≡ 2 mod 7`), lift `r_0 = 3` through `f(x) = x² − 2` to mod 7^4 and check against the
hand-computed value; assert the non-simple-root case errors; `IntPoly::derivative` and `eval_mod`
checked directly on a sample polynomial. **Verify gate:** `cargo test --workspace` green; **the G.A
`IntPoly` NFS consumers in `gnfs` stay green** (the additive-extension no-regression check).

**Subtlety (load-bearing):** (1) **Only simple roots lift uniquely** — `f'(r_0) ≢ 0 mod p` is the
precondition; a non-simple root needs the general (slower-convergence) Hensel and is out of toy
scope — error on it. (2) **The `IntPoly` edit is additive** — adding methods, not changing
`from_coeffs`/`eval`/arithmetic; the frozen G.A contract is extended, not broken. (3) **`eval_mod`
must reduce at each Horner step**, not eval-then-reduce (toy p^k is small so either works, but the
reduce-each-step form is the correct general shape — annotate). (4) **Newton doubles precision** —
each step roughly squares the precision; mind the C-Padic precision bookkeeping so the lift lands at
exactly mod p^k.

**Deferred:** the p-adic log (E.D.3); multivariate/system Hensel (out of scope — E.E's point lift is
univariate per coordinate); non-simple-root lifting (principle-4 boundary).

### E.D.3 ◆ — p-adic logarithm + sub-track close (Sonnet, Cat B, `@plan`)

**Deliverable:** the p-adic logarithm E.E's SSA reduction consumes, the end-to-end KAT, and the
sub-track close. Lower-fidelity sketch (crisp after C-Padic + C-Hensel freeze):
- **The p-adic log** (`shared/padic/src/log.rs`): the formal-group logarithm series `log(1+x) = x −
  x²/2 + x³/3 − …` over C-Padic, convergent p-adically for `v_p(x) ≥ 1`, truncated at the precision
  the toy k supports. Whether E.D.3 ships the *general* formal-group log (the series, which E.E
  specialises to the elliptic formal group) or also the elliptic-curve specialisation is the ◆
  design call — **proposed: ship the general series in E.D; E.E supplies the elliptic formal-group
  parametrisation** (keeps E.D curve-free, item 5 of the intent).
- **End-to-end KAT** (`shared/padic/tests/log_kat.rs`): lift a known `1 + p·u` via E.D.2's Hensel (or
  construct it directly in C-Padic), apply the log, and check the result against the hand-computed
  p-adic value; verify the homomorphism property `log(ab) = log(a) + log(b)` on a sample (the
  property that makes the SSA escape work). **Optional PARI `Qp`/`log` `#[ignore]` cross-check** (the
  established dev-only oracle pattern).

Consumes C-Padic (frozen E.D.1, read) and C-Hensel (frozen E.D.2, read). **Freezes C-PadicLog.**

**KAT (primary correctness signal):** **end-to-end** — the log of a hand-computed `1 + p·u` matches
the known p-adic value to precision k; the homomorphism `log(ab) = log(a) + log(b)` holds; the
existing `shared` + `gnfs` KATs stay green. Optional PARI cross-check. **Verify gate:** `cargo test
--workspace` green.

**Subtlety (load-bearing):** (1) **Convergence requires `v_p(x) ≥ 1`** — the series converges
p-adically only on the kernel of reduction (`x ≡ 0 mod p`); the log must assert/require this, or it
silently diverges (returns a precision-limited garbage value). This is the log's analogue of the
unit-inversion guard. (2) **The `x^n / n` terms have a denominator** — `n` may be divisible by p,
which *lowers* p-adic precision (the `1/p` in `x^p/p` etc.); the truncation point must account for
this, or the log loses precision silently. **This is the subtle correctness point of E.D.3** — name
it and bound the series length to the precision the toy k actually supports. (3) **This is the E.D ◆
boundary** — re-read the Purpose intent and verify the p-adic substrate is coherent (Z_p arithmetic →
Hensel lift → log homomorphism) and genuinely the substrate E.E needs (general, curve-free) before
crossing. (4) **No MATHEMATICS chapter here** — the p-adic textbook chapter is T.E, paired with E.W
at the *Track-E* ◆ (ROADMAP per-track pairing); E.D.3 writes at most a PEDAGOGY code-tour delta.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.D.3 ◆
to confirm: (1) the p-adic substrate composes correctly (Z_p arithmetic → Hensel → log, the
homomorphism round-trip holds); (2) C-Padic's non-field guard (unit-only inversion) and C-PadicLog's
convergence guard (`v_p ≥ 1`) are both in place — the two silent-failure defenses; (3) the `IntPoly`
extension was **additive** (no frozen G.A signature changed; the `gnfs` NFS consumers stay green);
(4) E.D stayed **curve-free** (no SSA attack, no point-counting, no `rho → shared-padic` edge — those
are E.E); (5) the principle-4 boundaries (toy precision, simple-roots-only Hensel, general-series log
deferring the elliptic specialisation to E.E) are annotated, not silently presented as crypto-scale.
One-shot findings; does not implement. Held at **Opus** per the header (lever 3 on the E.D.1
substrate dominates the strong lever-5 KATs).

---

## Cross-session contracts

E.D **freezes three** contracts and **additively extends one frozen contract** (`IntPoly`). Per the
substrate-over-specify rule, C-Padic carries the unit-inversion guard and explicit precision now even
though a toy fixture would "work" with a looser type. All other composed substrates (`shared/bigint`)
are **read**, not amended.

### C-Padic — Z_p / Z/p^k arithmetic interface + non-field guard (compiler- + test-enforced) — *to be frozen at E.D.1*

**Defined in:** E.D.1 (`shared/padic/src/zp.rs`). **Consumed by:** E.D.2 (Hensel iterates over it),
E.D.3 (the log series computes over it); **E.E** (SSA lifts curve coordinates through it — the named
downstream consumer). Compiler-enforced (the type + method signatures) + test-enforced (hand-computed
arithmetic + the non-unit-inversion error). Exposes: the prime-power-modulus type carrying a `BigInt`
residue + precision `k` + the p-adic valuation; `add`/`sub`/`mul`; **unit inversion** (defined only
for `v_p = 0`, errors on non-units); `valuation`; `precision`; lift/truncate between precisions.
*Exact representation (valuation+unit vs. fixed-precision residue) and the p-threading convention
ratified at E.D.1 and re-ratified at the E.D.3 ◆.* **Z/p^k is not a field — only units invert** (the
silent-wrong-answer defense). **Toy precision only** (principle-4 boundary).

### C-Hensel — Hensel lift (Newton root-lifting over Z_p) (compiler- + test-enforced) — *to be frozen at E.D.2*

**Defined in:** E.D.2 (`shared/padic/src/hensel.rs`). **Consumed by:** E.D.3's KAT (constructs lifted
elements); **E.E** (lifts curve-point coordinates from F_p to Z_p). Compiler- + test-enforced.
Exposes the lift: `(f: &IntPoly, r_0 simple root mod p, target precision k) → unique root mod p^k`,
erroring on non-simple roots. **Depends on the additive `IntPoly` extension** (`derivative` +
`eval_mod`). *Exact signature ratified at E.D.2 and re-ratified at the E.D.3 ◆.* **Simple roots
only** (`f'(r_0) ≢ 0 mod p`; the uniqueness precondition).

### C-PadicLog — p-adic / formal-group logarithm + convergence guard (compiler- + test-enforced) — *to be frozen at E.D.3 ◆*

**Defined in:** E.D.3 (`shared/padic/src/log.rs`). **Consumed by:** **E.E** (the SSA reduction's
core homomorphism — maps the order-p subgroup to additive F_p); E.D's own end-to-end KAT now; **T.E**
(the p-adic textbook chapter at the Track-E ◆, the documentation consumer). Compiler- + test-enforced.
Exposes the log: the formal-group series over C-Padic, convergent for `v_p(x) ≥ 1`, truncated to the
precision toy k supports; the homomorphism `log(ab) = log(a) + log(b)`. *Exact signature (general
series vs. elliptic specialisation — proposed general, E.E specialises) ratified at the E.D.3 ◆.*
**Convergence requires `v_p(x) ≥ 1`** (the silent-divergence defense). **Ships the general series;
the elliptic formal-group parametrisation is E.E's** (keeps E.D curve-free).

### Additively-extended frozen contract

- **`IntPoly`** (`shared/numfield/src/poly.rs`, frozen G.A) — E.D.2 adds `derivative()` and
  `eval_mod(x, m)`. **Additive only** (new methods; no change to `from_coeffs`/`eval`/`degree`/
  arithmetic). The G.A NFS consumers in `gnfs` are untouched and must stay green. *(If E.D finds it
  must change an existing `IntPoly` signature — it should not — that is an **additive-reshard**
  surfaced at the ◆, never a silent patch.)*

### Frozen contracts read by E.D (composed, not amended)

- **`shared/bigint`** (`BigInt`, `gcd`, `isqrt`, `batch_invert`, frozen) — the integer substrate
  C-Padic computes over. Read, not changed.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.D.3 ◆ `@plan` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.D.1 | New `shared/padic` crate + Z_p / Z/p^k arithmetic substrate | done | 0a98148 | C-Padic (frozen) |
| E.D.2 | Hensel lifting + `IntPoly::derivative`/`eval_mod` | pending | | C-Hensel (+ `IntPoly` ext.) |
| E.D.3 ◆ | p-adic logarithm + sub-track close | pending | | C-PadicLog |

Contracts frozen before this sub-track (read/extended by E.D): `shared/bigint` (read); the G.A
`IntPoly` (`shared/numfield`, additively extended). This sub-track **freezes three new contracts**
(C-Padic, C-Hensel, C-PadicLog), all serving the downstream **E.E (Smart–Satoh–Araki)** consumer and
the **T.E** p-adic textbook chapter.

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.D is greenfield p-adic substrate — building the crate + arithmetic + lift + log is
  internal-continue (substrate finding, confirmed by survey).** No p-adic code exists; E.D writes it
  from scratch in `shared/padic`. A discovery that Z_p (without a Q_p field-of-fractions layer) is
  insufficient for the log's denominators is an **additive-reshard** surfaced at the E.D.3 ◆.

- **`Fp` must NOT be reused for the p-adic tower (the silent-wrong-answer guard).** `Fp::inv` uses
  Fermat (prime modulus); Z/p^k is not a field. A `@build` agent that builds the tower on `Fp`, or
  inverts a non-unit without erroring, is **internal-continue → corrected** (the C-Padic type +
  unit-only-inversion guard is mandatory). The silent failure — wrong inverse mod p^k, wrong lift, no
  error — is why the guard is non-negotiable.

- **The `IntPoly` extension must stay additive — changing an existing signature is a
  destructive-HALT (frozen-contract guard).** `IntPoly` is a G.A-frozen shared type consumed by the
  `gnfs` NFS pipeline. Adding `derivative()`/`eval_mod()` is additive (internal-continue). Changing
  `from_coeffs`/`eval`/arithmetic signatures, or breaking a `gnfs` `IntPoly` KAT, is a
  **destructive-HALT** — stop, surface it. `cargo test --workspace` failing on a `gnfs` `IntPoly`
  consumer is the loud signal.

- **The p-adic log converges only for `v_p(x) ≥ 1` (the silent-divergence guard).** The formal-group
  series diverges p-adically off the kernel of reduction; the log must require `v_p ≥ 1` (error or
  precondition), or it returns precision-limited garbage. A log that omits the convergence guard is
  **internal-continue → corrected**. The `x^n/n` denominator's p-divisibility lowering precision is
  the subtle correctness point — bound the series length to toy k.

- **E.D stays curve-free — the SSA attack, point-counting, and the `rho → shared-padic` edge are E.E,
  not E.D (defocus guard).** A `@build` agent that implements anomalous-curve detection, trace-of-
  Frobenius / point-counting, the curve-point lift, or adds a `rho → shared-padic` dependency during
  E.D is **defocus** — internal-continue only within the substrate scope. E.D delivers the general
  p-adic machinery; E.E consumes it against curves. Touching `rho` or `gnfs/src/curve` is out of E.D's
  scope.

- **No MATHEMATICS.md chapter in E.D (defocus / scope clarity).** The p-adic textbook chapter is
  **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing rule), not at the E.D
  sub-track ◆. A `@build` agent that writes a MATHEMATICS.md p-adic chapter during E.D is defocus;
  E.D.3 writes at most a PEDAGOGY code-tour delta. (The explorer survey's "chapter must be appended as
  part of E.D" inference is superseded by this rule.)

- **The end-to-end KAT must check against HAND-COMPUTED p-adic values + the homomorphism, not a spot
  value (correctness discipline).** A wrong log can return a plausible-but-wrong p-adic number; only
  the known-value check plus `log(ab) = log(a) + log(b)` catches it. A KAT that only spot-checks one
  log value has an under-specified contract — flag it.

- **No oracle dependency for correctness (principle-3 / E.C-consistent).** p-adic lifts and the log
  are exactly hand-computable; a PARI `Qp`/`log` cross-check is an **optional `#[ignore]` sidecar**
  (the established pattern). E.D introduces no new live oracle.

- **Toy precision only — the precision tower is small (scope clarity).** E.D fixes a small k
  (principle 4); a crypto-scale precision tower is a principle-4 boundary, not an E.D work item.
  Presenting toy precision as crypto-scale is a documentation defect — internal-continue → corrected.

- **Capture owed at the E.D ◆: the `Fp`-is-not-a-ring substrate fact is a durable cross-track note.**
  `Fp<L>` (`shared/field`) assumes a prime modulus (`inv` via Fermat) and cannot represent Z/p^k; any
  future prime-power-modulus work must use a distinct unit-only-inversion type (C-Padic). This is the
  p-adic analogue of the E.C modulus-consistency invariant the ROADMAP already records. Adding it to
  the ROADMAP Discoveries log alongside C-Padic is an **inflection-point Opus action at the E.D ◆**,
  not an E.D `@build` work item.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.D, "*Hensel lifting; p-adic logarithm. Sonnet.*"; the design
  statement's principle 4; the "On scale" mathematical-dimension-scale framing) and this PLAN before
  any session. **The ROADMAP is reconciled through the E.C ◆ boundary (2026-06-10)** — no static-frame
  debt is outstanding; the Progress table and Discoveries log are current.
- Read the **templates to mirror**: `shared/numfield/src/poly.rs` (`IntPoly` — the type E.D.2 extends;
  mirror its `eval` Horner idiom for `eval_mod`); `shared/field/src/lib.rs` (the `Fp<L>` trait — the
  *contrast* type, NOT to be reused for p^k; its pass-the-modulus convention may be mirrored for
  consistency); `shared/bigint/src/lib.rs` (`BigInt` + `gcd`); `shared/field/tests/sqrt_legendre_kat.rs`
  and `shared/numfield/tests/numfield_kat.rs` (the `shared`-crate KAT idiom E.D's tests mirror);
  `rho/tests/mov_kat.rs:253` (the `#[ignore = "PARI not installed…"]` dev-oracle pattern).
- **Register:** E.D is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New crate `shared/padic` with `src/{zp,hensel,log}.rs` and `tests/*_kat.rs`. The
  `IntPoly` extension goes in `shared/numfield/src/poly.rs` (additive methods).
- **Tier routing:** **all three E.D sessions are Sonnet** (`@build` on Sonnet) — the ROADMAP flags
  E.D Sonnet (well-understood material). E.D.3 carries the single `@plan` marker: a ◆-boundary
  juncture (page `@plan-juncture`) ratifying C-Padic / C-Hensel / C-PadicLog and confirming the
  substrate composition before the sub-track closes. juncture-tier (header) is **opus** — held by
  lever 3 (the E.D.1 Z_p representation is the design crux bounding the whole sub-track + E.E); the
  strong lever-5 exactly-checkable KATs would license `sonnet` in isolation but the user judged the
  substrate design-error cost decisive.
- **Invariants to preserve:** **Z/p^k is not a field** (unit-only inversion; the non-unit-inversion
  error is mandatory — C-Padic). **The `IntPoly` extension is additive** (no frozen G.A signature
  changes; the `gnfs` NFS consumers stay green). **The p-adic log requires `v_p(x) ≥ 1`** (the
  convergence guard — C-PadicLog). **E.D is curve-free** (no SSA attack, no point-counting, no `rho →
  shared-padic` edge — those are E.E). **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy
  precision only; Hensel lifts simple roots only; the log ships the general series (E.E specialises).
  No new live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional p-adic log / `Qp` cross-check follows
  the established `#[test] #[ignore = "PARI not installed…"]` pattern; never on the green path.
- **The new workspace member (load-bearing for E.D).** E.D.1 adds `shared/padic` to the root
  `Cargo.toml` `members` list. The crate depends only on `shared-bigint` (E.D.1) + `shared-numfield`
  (E.D.2 onward); `cargo check --workspace` must resolve with no cycle. The downstream `rho →
  shared-padic` edge is E.E's, not E.D's.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — E.D opens a new
  substrate crate (the Z_p representation is a Category-A design crux) and the shard pattern (a fresh
  `shared` crate + an additive frozen-type extension) is unproven for this sub-track; the conservative
  halt-at-◆ cadence is warranted, and it surfaces the E.D.1 substrate at its own commit for human eyes
  even though the only paged ◆/`@plan` juncture sits on E.D.3. *(Tradeoff vs default cadence: one
  extra halt-confirm on a 3-session Sonnet sub-track is cheap insurance on a new shared-crate
  substrate.)*
