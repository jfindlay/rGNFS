<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.I — GF(2^m) hyperelliptic Jacobian: Cantor's algorithm, the second GF(2^m) consumer and E.H's missing predecessor)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) on the C-Jacobian
surface, against a strong lever 5 that would otherwise license an opt-down.** E.I freezes the
hyperelliptic-Jacobian surface (C-Jacobian) that **E.H — GHS/Weil descent, "the most mathematically
intricate single attack in the project" (ROADMAP:357-358) — descends *onto*.** GHS transfers the
ECDLP on a binary curve to a DLP on the Jacobian of a hyperelliptic curve over a subfield; that
Jacobian is exactly what E.I builds. The Mumford divisor representation and the Cantor group-law
interface bound what the descent reads and where it lands; getting the Jacobian surface wrong is an
expensive retrofit through the single hardest attack. Lever 5 is strong and fast (Jacobian arithmetic
is *exactly* checkable: the group axioms, `D + (−D) = 0`, associativity on a sample, the divisor-order
law `n·D = 0`, the reduced-divisor invariant `deg u ≤ g` with `u | f − v·h − v²`, and a PARI
`hyperellcharpoly` / Jacobian-arithmetic `#[ignore]` sidecar) and *would* license `juncture-tier:
sonnet` in isolation — **the user judged the C-Jacobian design-error cost (lever 3, E.H descends onto
it) decisive and held the ◆ juncture at Opus**, mirroring the E.G call exactly (a substrate-adjacent
juncture feeding the same high-stakes downstream consumer is held up despite exactly-checkable KATs).
*(E.I carries NO session-level Opus flag — the ROADMAP Opus-flagged-sessions table lists E.H.1 but no
E.I session; all three E.I sessions are **Sonnet `@build`**, exactly as E.G's were. The juncture-tier
governs only the paged `@plan-juncture` fork at the ◆, not the session tier.)*

**Sequencing correction — load-bearing, surfaced at shard time.** The ROADMAP "Remaining projected
sessions" table (ROADMAP:209-210) lists **E.H before E.I**. This is **stale / dependency-inverted**:
ROADMAP:356 names E.H's predecessor as "**E.G, E.I**", and the substrate survey (forked `@explore`,
2026-06-14) **confirmed E.I is entirely greenfield — zero hyperelliptic/Jacobian/divisor/Cantor code
exists anywhere in the tree** (the only `Jacobian` hits are the prime-field `JacobianPoint` projective
*coordinate* type in `rho::curve`, unrelated to the Jacobian *variety*). GHS/Weil descent transfers
the ECDLP *onto* a hyperelliptic Jacobian; that transfer target must exist first. **The dependency
graph and the predecessor annotation both require E.I before E.H; the table order is the staleness.**
The user adjudicated this at shard time: shard **E.I next**, reconcile the Remaining-table order at the
E.I ◆. *(Surfaced as a CAPTURE-CANDIDATE below.)*

**Static-frame debt outstanding (carried from the E.G PLAN, still owed — does NOT block E.I sharding).**
The ROADMAP **Progress** subsection (~line 168) still shows Track E "Done ~13 (E.A–E.E)" and the
**Remaining projected sessions** table (~line 207-208) still lists E.F as remaining and does not strike
E.G — both **E.F and E.G are now complete** (E.F `2ca3061`…`51c8c8d`; E.G `178c0aa`…`c7d5e0e`). The
E.G ◆ digest recorded this reconciliation as owed housekeeping; it is **still owed**, now compounded by
the E.F+E.G completions and the E.H/E.I order inversion above. This is a **roadmap-frame reconciliation
owed at the E.I ◆** (paired with this PLAN's close), not an E.I implementation concern — flagged here so
it is not lost. *(Surfaced as a CAPTURE-CANDIDATE below.)*

The substrate survey (forked `@explore`, 2026-06-14) established the shape and confirmed every planning
assumption:

1. **The hyperelliptic Jacobian is fully greenfield** (item 5, the load-bearing finding). No
   `HyperellipticCurve` / divisor / Mumford / Cantor code exists. E.I builds it from scratch in `rho`
   (parallel to `rho::binary_curve` — the same idiom, a struct holding curve params with per-call
   `F: F2m<1>` methods), consuming the frozen C-F2m field substrate.

2. **The free polynomial ring GF(2^m)[x] is greenfield** (item 6). `shared/gf2m` has field-element
   arithmetic *mod the irreducible* (`mul`/`square`/`pow`/`inv`) and `inv.rs::poly_divmod` over GF(2)[x]
   (internal to ext-Euclidean inversion), but **no free polynomial ring over GF(2^m)** — no
   `Poly<F: F2m>` type, no polynomial mul/divmod/gcd over GF(2^m). Cantor's algorithm *is* polynomial
   arithmetic over GF(2^m) (the Mumford `u(x)`/`v(x)` are GF(2^m)[x] polynomials; compose+reduce is
   resultant/gcd/divmod over GF(2^m)[x]). **E.I.1 builds this ring first** — it is the substrate the
   group law stands on, and a genuinely reusable one (E.H's function-field construction and E.K's
   Semaev work may consume it too — over-specify per the substrate rule).

3. **The C-F2m field surface is complete and Jacobian-ready** (item 3). `mul`/`square`/`frobenius`/
   `inv`/`div`/`pow`/`trace`/`solve_quadratic` are all live (E.F + E.G.1 filled the last two). E.I
   consumes C-F2m unchanged — **it amends no frozen contract** (unlike E.G.1, which filled the C-F2m
   stubs; E.I touches no `shared/gf2m` trait body). The only `shared/gf2m` change is **additive: a new
   `poly` module** (the GF(2^m)[x] ring), not a trait amendment.

4. **No new workspace edge** (item 1). `rho` already depends on `shared-gf2m` (the `rho → shared-gf2m`
   edge E.G.1 added) and on `gnfs`. The GF(2^m)[x] ring is an **additive module in `shared/gf2m`**
   (new `shared/gf2m/src/poly.rs`), and the Jacobian lives in `rho` (new `rho::hyperelliptic`). No
   `Cargo.toml` edge changes; `cargo check --workspace` confirms no new cycle (there is no new edge to
   cycle).

5. **`gnfs`'s NFS-DL pipeline is NOT the Jacobian DLP solver** (item 1). `gnfs::dl::solve_dl` is the
   *number-field* NFS-DL over `F_p`/`F_{p^k}` (BigInt-valued); it is **not directly reusable** for a
   DLP on a GF(2^m) hyperelliptic Jacobian. E.I builds the Jacobian *group law and arithmetic* only —
   **the index-calculus DLP solver on the Jacobian is E.H's job** (GHS descends onto the Jacobian and
   then runs index calculus over the subfield). E.I is **DLP-solver-free**: it exposes the group, not
   the attack on it. This is the central scope boundary (defocus guard).

The work splits at **three representation/contract-sharp seams**, **3 sessions** (matching the
ROADMAP's 3–4 estimate, mid-band), at the boundaries between the polynomial-ring substrate, the
curve+divisor representation, and the Cantor group law:

1. **E.I.1 — GF(2^m)[x] polynomial ring (Sonnet, Cat A).** A free polynomial ring over GF(2^m):
   `Poly<F: F2m<L>>` (coefficient vector), `add`/`mul`/`divmod`/`gcd`/`monic`/`derivative`, and the
   resultant/sub-resultant building blocks Cantor's reduce needs. **Freezes C-GF2mPoly** — the
   polynomial substrate the Jacobian arithmetic stands on. Additive `shared/gf2m` module; no edge change.

2. **E.I.2 — Hyperelliptic curve + Mumford divisor representation (Sonnet, Cat A).** The
   `HyperellipticCurve` type (`y² + h(x)·y = f(x)` over GF(2^m), genus `g` from `deg f`), the Mumford
   `[u(x), v(x)]` reduced-divisor representation (`u` monic, `deg v < deg u ≤ g`, `u | f − v·h − v²`),
   the validity predicate, and divisor construction from points. **Consumes C-GF2mPoly.** **Freezes
   C-HyperCurve.**

3. **E.I.3 ◆ — Cantor's algorithm: the Jacobian group law + sub-track close (Sonnet, Cat B,
   `@architect`).** Cantor compose + reduce (one irreducible unit — the Jacobian group law), `negate`,
   `scalar_mul`, the identity (the zero divisor), and the group-axiom + PARI-oracle KAT suite.
   **Consumes C-HyperCurve, C-GF2mPoly.** **Freezes C-Jacobian** — the surface E.H descends onto.
   Crosses the **E.I ◆ boundary** — the Jacobian ships, ready for E.H to descend.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing GHS/Weil descent or the
Jacobian *index-calculus DLP solver* — that is E.H; or the Koblitz/binary-curve work — that is E.G,
done; or writing the hyperelliptic *textbook chapter* in MATHEMATICS.md — that is **T.E**, paired with
E.W at the Track-E ◆; E.I writes at most a PEDAGOGY code-tour delta) and **rigidity** (forcing the
genus-2 char-2 curve into the elliptic `y²+xy=x³+ax²+b` form — hyperelliptic curves carry a general
`h(x)`; or re-deriving GF(2^m) field primitives rather than consuming the frozen C-F2m surface; or
trying to reuse `rho::binary_curve`'s elliptic group law for divisor arithmetic — the Jacobian group
law is Cantor's algorithm on Mumford pairs, structurally distinct from point addition).

**Scoping discipline.** E.I builds the hyperelliptic Jacobian at **demonstration fidelity** (principle
1 — algorithmic content complete: the polynomial ring, the curve, the Mumford representation, and
Cantor's compose+reduce all implemented head-on) and **toy field/curve/genus sizes** (small `m` — e.g.
GF(2^4), GF(2^8) — and genus 2, the canonical GHS target genus; toy-shaped curves enough to exercise
every code path and exhibit the algebra, not crypto-scale). It **amends no frozen contract** (C-F2m /
C-BinaryCurve / C-Koblitz / the p-adic / SSA / prime-field surfaces are all consumed-or-untouched; the
only `shared/gf2m` change is the additive `poly` module). It introduces **no new live oracle** on the
green path (the Jacobian group axioms + `D+(−D)=0` + the reduced-divisor invariant + `n·D=0` are
self-checking; an optional PARI `hyperellcharpoly`/Jacobian-arithmetic `#[ignore]` sidecar is the
established dev-only pattern). The **engineering-vs-mathematics disconnect** (ROADMAP principle 4) is
explicit: the toy genus/field sizes are a principle-4 boundary (Cantor's algorithm is crypto-scale- and
arbitrary-genus-correct; only the *parameters* are toy), annotated, never presented as crypto-scale.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.I): "*E.I — GF(2^m) hyperelliptic Jacobian. 3-4 sessions. Predecessor: E.F.
Cantor's algorithm, divisor representation, Jacobian group law. Sonnet — well-understood material.*"
E.I is the **second GF(2^m) consumer** (after E.G's binary curves) and **E.H's missing predecessor**:
where E.G built elliptic curves over GF(2^m), E.I builds the *hyperelliptic* curve `y²+h(x)y=f(x)` and
its **Jacobian** — the divisor class group, represented via Mumford coordinates, with the group law
computed by Cantor's algorithm. This Jacobian is exactly the structure **GHS/Weil descent (E.H)
transfers the ECDLP onto**: the descent maps the ECDLP on a binary elliptic curve to a DLP on the
Jacobian of a hyperelliptic curve over a subfield, and E.I builds that Jacobian's arithmetic.

E.I's structural predecessor is E.F (the GF(2^m) field substrate, complete); it consumes the frozen
C-F2m field trait directly (`add`/`mul`/`square`/`frobenius`/`inv`/`div`/`pow`), unchanged — E.I
amends no field contract. The central design tension is that **the Jacobian group law is NOT point
addition**: a hyperelliptic curve of genus `g ≥ 2` has no group structure on its *points*; the group
lives on *divisor classes*, represented as Mumford pairs `[u(x), v(x)]` of GF(2^m)[x] polynomials, and
the group law is **Cantor's algorithm** (compose two divisors via polynomial gcd/resultant, then
reduce to the canonical representative with `deg u ≤ g`). This is structurally distinct from
`rho::binary_curve`'s elliptic point addition — E.I shares the *idiom* (a const-generic-`F2m`-method
curve type) but the arithmetic is polynomial-over-GF(2^m), not coordinate.

The sub-track decomposes into three conceptual units, each a session:

1. **The GF(2^m)[x] polynomial ring (E.I.1).** A free polynomial ring over GF(2^m) — greenfield (the
   survey confirmed `shared/gf2m` has only field-element-mod-irreducible arithmetic and a private
   GF(2)[x] divmod). `Poly<F: F2m<L>>` with `add`/`mul`/`divmod`/`gcd`/`monic`/`derivative` and the
   sub-resultant/extended-gcd machinery Cantor's reduce step needs. **Freezes C-GF2mPoly.** **(E.I.1.)**

2. **The hyperelliptic curve + Mumford divisor representation (E.I.2).** `HyperellipticCurve`
   (`y²+h(x)y=f(x)` over GF(2^m), genus `g = ⌊(deg f − 1)/2⌋`); the Mumford `[u(x), v(x)]`
   representation of a reduced divisor (`u` monic, `deg v < deg u ≤ g`, the curve-compatibility
   `u | f − v·h − v²`); divisor construction from a set of points; the reduced-divisor validity
   predicate. **Freezes C-HyperCurve.** **(E.I.2.)**

3. **Cantor's algorithm + the Jacobian group law + close (E.I.3).** Cantor compose (the divisor
   addition via polynomial extended-gcd) + reduce (the canonical-representative reduction to
   `deg u ≤ g`) — **one irreducible unit**, the Jacobian group law. `negate` (the divisor `[u, h+v]`),
   `scalar_mul` (double-and-add over Cantor), the identity (the zero divisor `[1, 0]`). The
   sub-track-close KAT suite (group axioms, `D+(−D)=0`, associativity, `n·D=0`, the reduced-divisor
   invariant; optional PARI `hyperellcharpoly` cross-check). **Freezes C-Jacobian.** **(E.I.3 ◆.)**

E.I is **descent-free** (GHS/Weil descent is E.H — E.I builds the Jacobian the descent lands on, not
the descent), **DLP-solver-free** (the index-calculus DLP attack on the Jacobian is E.H's; E.I exposes
the *group*, not the *attack*), **field-arithmetic-frozen** (it consumes C-F2m unchanged — no field
amend), and **chapter-free** (the hyperelliptic textbook content is T.E, paired with E.W at the
Track-E ◆). Re-read this intent at the ◆ boundary to catch defocus (descent, the Jacobian DLP solver,
the MATHEMATICS chapter) and rigidity (the elliptic group law for divisors; the elliptic-curve form for
a genus-≥2 curve; re-deriving field primitives).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-14); raw
`cargo` is the only CI surface (unchanged from E.D…E.G). Oracle KATs are `#[ignore]`-gated only — the
exact form is `#[ignore = "PARI not installed; run manually when available"]`, used identically in
`rho/tests/ssa_kat.rs`, `rho/tests/mov_kat.rs`, and `shared/padic/tests/log_kat.rs`. `/run-plan`
re-discovers at preflight. E.I **adds no new workspace edge and no new crate** (`rho` already depends
on `shared-gf2m`; the GF(2^m)[x] ring is an additive `shared/gf2m` module, the Jacobian a new `rho`
module), so the gate is a **correctness + no-regression gate**:

- Each session's KATs (`shared/gf2m/tests/gf2m_poly_kat.rs` for E.I.1; `rho/tests/hyperelliptic_kat.rs`
  for E.I.2/E.I.3, plus inline unit tests mirroring the `rho::binary_curve` idiom) are the primary
  correctness signal — fast and *exactly* decisive (lever 5): polynomial-ring axioms and
  `divmod`/`gcd` round-trips for C-GF2mPoly; the Mumford reduced-divisor invariant
  (`deg v < deg u ≤ g`, `u | f − v·h − v²`) for C-HyperCurve; the Jacobian **group axioms**
  (`D + 0 = D`, `D + (−D) = 0`, associativity on a sample), the **divisor-order law** `n·D = 0`, and
  Cantor consistency (`2D` via doubling equals `D + D` via compose) for C-Jacobian — all self-checking
  with no external oracle.
- `cargo check --workspace` must stay green — **no edge change**, so no cycle risk (there is no new
  edge to introduce one). The additive `shared/gf2m::poly` module is a leaf addition.
- **The existing rho / gnfs / shared KATs must stay green** after the Jacobian code lands — E.I adds
  new modules (`shared/gf2m::poly`, `rho::hyperelliptic`) and changes no existing field / curve /
  pairing / rho path, so the no-regression invariant is structurally easy to hold; `cargo test
  --workspace` is the guard. *(One subtlety: the new `shared/gf2m::poly` module touches `shared/gf2m`'s
  public surface additively — the existing `gf2m_kat.rs` must stay green and the polynomial ring gets
  its own `gf2m_poly_kat.rs`.)*

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.I.1 | GF(2^m)[x] polynomial ring (add/mul/divmod/gcd/resultant over GF(2^m)) | A | Sonnet | C-F2m (frozen — `add`/`mul`/`square`/`inv`/`div`/`pow` read; no amend); `shared/gf2m::convert` (read — the per-call `poly: &Uint<L>` idiom to mirror) | `shared/gf2m/src/poly.rs` (new: `Poly<F: F2m<L>>` + ring ops + divmod/gcd/resultant), `shared/gf2m/src/lib.rs` (add `pub mod poly;` + re-export), `shared/gf2m/tests/gf2m_poly_kat.rs` (new: ring axioms, divmod round-trip, gcd, resultant KATs) |
| E.I.2 | Hyperelliptic curve `y²+h(x)y=f(x)` + Mumford divisor representation | A | Sonnet | C-GF2mPoly (frozen E.I.1); C-F2m (read); `rho::binary_curve::BinaryCurve` (read — the curve-struct idiom to mirror, not reuse) | `rho/src/hyperelliptic/mod.rs` (new: `HyperellipticCurve` + genus + `MumfordDivisor` `[u,v]` + validity predicate + divisor-from-points), `rho/src/lib.rs` (add `pub mod hyperelliptic;`), `rho/tests/hyperelliptic_kat.rs` (new: curve validity, Mumford reduced-divisor invariant, divisor-from-points KATs) |
| E.I.3 ◆ `@architect` | Cantor's algorithm: Jacobian group law (compose + reduce) + sub-track close | B | Sonnet | C-HyperCurve (frozen E.I.2, read), C-GF2mPoly (frozen E.I.1, read); `rho::binary_curve` scalar-mul idiom (read — double-and-add shape) | `rho/src/hyperelliptic/cantor.rs` (new: Cantor compose + reduce = the Jacobian group law; `negate`; `scalar_mul`; identity), `rho/src/hyperelliptic/mod.rs` (add `pub mod cantor;` + wire group ops), `rho/tests/hyperelliptic_kat.rs` (extend: group axioms, `D+(−D)=0`, associativity, `n·D=0`, `2D==D+D`, sub-track-close suite; optional PARI `hyperellcharpoly` `#[ignore]` sidecar) |

**Sequencing notes.** Strictly serial: **E.I.1 → E.I.2 → E.I.3.** E.I.1 lands the polynomial ring all
the divisor arithmetic stands on; E.I.2 adds the curve + Mumford representation (the divisor *data*);
E.I.3 adds Cantor's algorithm (the *group law* on that data) and closes the sub-track. **One
`@architect` marker** sits on the **E.I.3 ◆** (the boundary juncture ratifying C-GF2mPoly / C-HyperCurve
/ C-Jacobian and confirming the Jacobian is complete and descent-ready before the sub-track closes).
*(Tradeoff named: E.I freezes C-GF2mPoly at E.I.1 and C-HyperCurve at E.I.2 but does NOT page inline
junctures there — mirroring the E.G call. The in-crate orthogonality (E.I.2 and E.I.3 both consume the
earlier freezes immediately, where a wrong polynomial-ring or Mumford shape fails the E.I.3 group-axiom
KAT loudly at the next session) plus the single high-stakes consumer (E.H, which has its own
Opus-flagged E.H.1 to design its descent) makes the early-catch insurance less valuable than a separate
inline fork would cost. The ◆ juncture is held at Opus (juncture-tier) for the C-Jacobian → E.H
cost-of-wrong, but a separate inline E.I.1/E.I.2 fork is not bought. If E.I.3's group-axiom or `n·D=0`
KAT surfaces a representation concern, that is the loud signal that substitutes for an inline juncture.)*

**Why 3 sessions (the ROADMAP's 3–4 estimate, mid-band).** The split is taken at three
representation/contract-sharp seams:
- **One-line-commit-title corollary.** "GF(2^m)[x] polynomial ring", "Hyperelliptic curve + Mumford
  divisor representation", and "Cantor's algorithm: the Jacobian group law + close" are **three
  distinct commit titles** across two categories (A substrate ×2, B algorithm).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the polynomial ring
  (with the gcd/resultant Cantor needs), the curve+divisor *representation*, and Cantor's group law.
  **Cantor compose+reduce is the dominant floor and must not fracture** — a composed-but-unreduced
  divisor is *not* a canonical group element (it has `deg u > g`); "compose without reduce" is not a
  landable session. The G.C ◆ logged heuristic ("demonstration-fidelity sessions run 400–800 LOC, not
  <150") reinforces keeping Cantor whole rather than splitting it to hit a LOC number.
- **Contract-sharp boundary.** E.I.1 **freezes** C-GF2mPoly; E.I.2 consumes it (the divisor data is
  GF(2^m)[x] polynomials) and **freezes** C-HyperCurve; E.I.3 consumes both and adds the group law.
  The polynomial-ring↔divisor-representation↔group-law boundaries are genuine contract seams (each
  later session is meaningless without the earlier freeze), which is what licenses — and bounds — the
  3-way split.

They are **not** further splittable below 3 at contract-sharp seams (the 4-session option was
considered and rejected): splitting the polynomial ring's gcd/resultant from its add/mul would leave
the ring fill with no Cantor consumer to express its decisive KAT against (resultant's test is the
Cantor reduce it feeds). Splitting the Mumford representation from curve-validity would split one
substrate across two rows with no contract-sharp seam (the reduced-divisor invariant `u | f − v·h − v²`
*is* the curve-compatibility — they are one unit). Splitting Cantor compose from reduce would fracture
the irreducible group-law unit (lever 2 — an unreduced divisor is not a group element). The 2-session
option was also rejected: it bundles two greenfield substrates (the polynomial ring + the curve+Mumford
representation) into one top-of-band E.I.1, absorbing the real polynomial-ring↔representation contract
seam inside one row.

---

## Session detail

E.I.1 and E.I.2 are specified at near-full fidelity (the polynomial ring and Mumford representation are
the design crux Cantor — and downstream E.H — stand on). E.I.3 is a lower-fidelity sketch, correct per
the substrate-first discipline: it is crisply specified only after C-GF2mPoly and C-HyperCurve freeze.

### E.I.1 — GF(2^m)[x] polynomial ring (Sonnet, Cat A)

**Deliverable:** a free polynomial ring over GF(2^m) — the substrate Cantor's divisor arithmetic stands
on, greenfield (the survey confirmed `shared/gf2m` has only field-element-mod-irreducible arithmetic
and a private GF(2)[x] divmod inside ext-Euclidean inversion). The pieces:
- **The `Poly<F: F2m<L>>` type** (`shared/gf2m/src/poly.rs`): a polynomial over GF(2^m) stored as a
  coefficient `Vec<F>` (index `i` = coefficient of `x^i`, leading coefficient last, normalized to drop
  trailing zeros). Mirror the C-F2m **per-call `poly: &Uint<L>` idiom** — the *field* irreducible
  threads through every operation that touches field arithmetic (the polynomial ring is over GF(2^m),
  so `mul`/`divmod`/`gcd` all need the field modulus). The type owns its coefficient vector; the field
  modulus is passed per-call (not stored), exactly as `BinaryCurve`/`F2m` do.
- **Ring operations** (`shared/gf2m/src/poly.rs`): `add` (coefficient-wise field `add` = XOR),
  `mul` (schoolbook convolution with field `mul`), `divmod` (long division — needs leading-coefficient
  `inv`; the field is a field, so division is total for nonzero divisor), `monic` (scale by leading
  `inv`), `gcd` (Euclidean over GF(2^m)[x]), `derivative` (formal derivative — in char 2,
  `d/dx Σ aᵢxⁱ = Σ i·aᵢxⁱ⁻¹` where `i·aᵢ` is `aᵢ` for odd `i`, `0` for even — the char-2 trap: the
  derivative kills *even-degree* terms, a load-bearing subtlety for square-free / resultant work).
- **The resultant / extended-gcd machinery Cantor's reduce needs** (`shared/gf2m/src/poly.rs`):
  the extended Euclidean algorithm (`xgcd` returning `(g, s, t)` with `s·a + t·b = g`) — Cantor's
  *compose* step is an `xgcd` of the two divisors' `u`-polynomials, and the *reduce* step is repeated
  `divmod`. Over-specify (substrate rule): include `resultant` and `mod_pow`/`mod_inverse` (polynomial
  modular inverse for `gcd(a, m) = 1`) if confidence is reasonable — E.H's function-field construction
  and E.K's Semaev work may consume them, and adding later is costlier.

Consumes C-F2m (frozen — `add`/`mul`/`square`/`inv`/`div`/`pow` read; **no amend**), the C-F2m per-call
`poly` idiom (read — mirrored). **Freezes C-GF2mPoly.**

**KAT** (`shared/gf2m/tests/gf2m_poly_kat.rs` + inline unit tests): over toy fields (GF(2^4) with
`x⁴+x+1`, poly = 0x13, mirroring the binary-curve KATs): **ring axioms** (commutativity,
associativity, distributivity of `add`/`mul` on sample polynomials); **divmod round-trip**
(`a = q·b + r` with `deg r < deg b`); **gcd** (`gcd(a, b)` divides both; `gcd(a·c, b·c) = c·gcd(a,b)·unit`);
**xgcd** (`s·a + t·b = gcd`); **monic** (leading coefficient = 1); **derivative** (char-2 correctness:
`(x²)' = 0`, `(x³)' = x²` — the even-degree-killing trap); **resultant** (if included: `res(a,b)=0` iff
`gcd` nontrivial). **Verify gate:** `cargo test --workspace` green; `cargo check --workspace` green
(additive module, no edge change); the existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **The char-2 formal derivative kills even-degree terms** — `(xⁿ)' =
n·xⁿ⁻¹`, and `n` reduces mod 2 in char 2, so `(x²)' = 0`, `(x⁴)' = 0`, `(x³)' = x²`. A `@build` agent
porting an integer-coefficient derivative writes the wrong derivative; the `(x²)' = 0` KAT is the loud
signal. This matters because square-free factorization and resultant-based reduce lean on the
derivative. (2) **`divmod` needs the leading-coefficient inverse** — GF(2^m) is a field, so division is
total for any nonzero divisor; but the divisor's *leading coefficient* must be inverted (`inv` from
C-F2m), not assumed 1 — a non-monic divisor is the common case in Cantor. (3) **Coefficient
normalization** — trailing-zero coefficients must be dropped so `degree()` is correct; an off-by-one in
the leading-coefficient index is the classic polynomial-ring bug (the degree-tracking KAT catches it).
(4) **The field modulus threads per-call** — every `mul`/`divmod`/`gcd` needs `poly: &Uint<L>` (the
field irreducible); a stored-modulus design would diverge from the C-F2m idiom and the
`BinaryCurve`/`HyperellipticCurve` per-call convention. (5) **Module placement** — the ring lives in
`shared/gf2m` (not `rho`) because it is field-substrate-shaped and reusable (E.H/E.K may consume it);
this is the principled home, ratified at the ◆. *(If a reviewer prefers it in `rho::hyperelliptic`,
that is a ◆ ratification call — the survey's recommendation is `shared/gf2m::poly`.)*

**Deferred:** the hyperelliptic curve + Mumford representation (E.I.2); Cantor's algorithm (E.I.3);
GHS/Weil descent (E.H — the descent *onto* the Jacobian); the Jacobian index-calculus DLP solver (E.H);
the MATHEMATICS chapter (T.E at the Track-E ◆).

### E.I.2 — Hyperelliptic curve + Mumford divisor representation (Sonnet, Cat A)

**Deliverable:** the `HyperellipticCurve` type, its genus, and the Mumford `[u(x), v(x)]`
reduced-divisor representation — the divisor *data* Cantor's group law operates on. The pieces:
- **The `HyperellipticCurve` type** (`rho/src/hyperelliptic/mod.rs`): a **parallel** to
  `rho::binary_curve::BinaryCurve` (the same idiom — a struct holding curve params, per-call
  `F: F2m<L>` methods, no `PhantomData<F>`). The curve is `y² + h(x)·y = f(x)` over GF(2^m), holding
  `h: Poly<F>` and `f: Poly<F>` (or their coefficient representations) and the field `poly`. The
  **genus** `g = ⌊(deg f − 1)/2⌋` (for the imaginary/ramified model, `deg f = 2g+1`, `deg h ≤ g`).
  Toy target: **genus 2** (the canonical GHS descent genus), over GF(2^4)/GF(2^8).
- **The Mumford `MumfordDivisor` representation** (`rho/src/hyperelliptic/mod.rs`): a reduced divisor
  is `[u(x), v(x)]` with `u` monic, `deg v < deg u ≤ g`, and the **curve-compatibility invariant**
  `u | (f − v·h − v²)` (i.e. `v² + h·v − f ≡ 0 mod u`). The zero divisor (group identity) is `[1, 0]`.
  A `MumfordDivisor` carries `u: Poly<F>`, `v: Poly<F>`.
- **Divisor construction + validity** (`rho/src/hyperelliptic/mod.rs`): build a divisor from a set of
  affine points (`u = Π(x − xᵢ)`, `v` the interpolant with `v(xᵢ) = yᵢ`); the `is_reduced` /
  `is_valid` predicate checking the Mumford invariant; `is_on_curve` for a point.

Consumes C-GF2mPoly (frozen E.I.1), C-F2m (read), the `BinaryCurve` curve-struct idiom (read —
mirrored). **Freezes C-HyperCurve.**

**KAT** (`rho/tests/hyperelliptic_kat.rs` + inline unit tests): over a toy genus-2 curve over GF(2^4):
**point-on-curve** `y²+h(x)y = f(x)` for sample points; the **Mumford reduced-divisor invariant**
(`u` monic, `deg v < deg u ≤ g`, `u | f − v·h − v²`) for divisors built from points; the
**divisor-from-points round-trip** (build `[u,v]` from points, recover the points as roots of `u` with
`y = v(xᵢ)`); the **zero divisor** `[1, 0]` is valid. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **A hyperelliptic curve has a general `h(x)`** — the char-2
non-supersingular hyperelliptic model is `y² + h(x)y = f(x)` with `h ≠ 0` (the char-2 analogue of the
binary elliptic `xy` term generalized to `h(x)·y`; setting `h = x` and `g = 1` recovers the binary
elliptic curve). A `@build` agent forcing `y² = f(x)` (the odd-char model) writes a curve with no group
law in char 2 (the same `2y` trap as the elliptic case, now on the Jacobian). (2) **The Mumford
invariant `u | f − v·h − v²` is the load-bearing contract** — a divisor that violates it is not a valid
reduced divisor and Cantor will produce garbage; the validity KAT is the guard. (3) **Genus tracking**
— `g` is derived from `deg f`, and the reduced-divisor bound `deg u ≤ g` is what Cantor's reduce step
enforces; an off-by-one in `g` corrupts the whole group law. (4) **Toy genus + field sizes only** —
genus 2 over small `m`; the representation is arbitrary-genus-correct (principle-4 annotate: the genus
and field are toy, Cantor is not).

**Deferred:** Cantor's algorithm / the group law (E.I.3); the descent (E.H); the MATHEMATICS chapter
(T.E).

### E.I.3 ◆ — Cantor's algorithm: the Jacobian group law + sub-track close (Sonnet, Cat B, `@architect`)

**Deliverable:** Cantor's algorithm (compose + reduce — the Jacobian group law), the negation/identity,
scalar multiplication, and the sub-track close. Lower-fidelity sketch (crisp after C-HyperCurve freezes):
- **Cantor compose + reduce** (`rho/src/hyperelliptic/cantor.rs`): the divisor addition. *Compose*:
  given `D₁ = [u₁, v₁]`, `D₂ = [u₂, v₂]`, compute the (generally unreduced) sum via polynomial
  extended-gcd (`d = gcd(u₁, u₂, v₁+v₂+h)`, then the composed `u = u₁u₂/d²`, `v` via CRT) — **one
  irreducible step**. *Reduce*: repeatedly replace `[u, v]` with `[u', v']` where `u' = (f − vh − v²)/u`
  made monic and `v' = (−h − v) mod u'`, until `deg u ≤ g` — the canonical-representative reduction.
  **Compose+reduce is one unit** (an unreduced divisor is not a group element).
- **Negation, identity, scalar-mul** (`rho/src/hyperelliptic/cantor.rs`): `negate([u, v]) = [u, (h+v)
  mod u]` (the char-2 hyperelliptic negation — analogous to the elliptic `−P=(x,x+y)` trap, now on
  divisors); the identity is `[1, 0]`; `scalar_mul` is double-and-add over Cantor compose (mirroring
  `BinaryCurve::scalar_mul`'s shape).
- **Sub-track-close KAT suite** (`rho/tests/hyperelliptic_kat.rs`, extended): the **group axioms**, the
  **divisor-order law**, Cantor consistency, and (optional) the PARI `hyperellcharpoly` cross-check.

Consumes C-HyperCurve (frozen E.I.2, read), C-GF2mPoly (frozen E.I.1, read), the `BinaryCurve`
scalar-mul idiom (read). **Freezes C-Jacobian.**

**KAT (primary correctness signal):** over a toy genus-2 curve over GF(2^4)/GF(2^8): the **group
axioms** (`D + 0 = D`; `D + (−D) = 0` with `−D = [u, (h+v) mod u]`; **associativity**
`(D₁+D₂)+D₃ = D₁+(D₂+D₃)` on a sample); **Cantor consistency** (`2D` via doubling equals `D + D` via
compose); the **divisor-order law** `n·D = 0` for a divisor of known order `n` (the `#E_Jac` / group
order computed for the toy curve, or PARI-supplied); every result is a **valid reduced divisor**
(`deg u ≤ g`, the Mumford invariant holds — re-checked post-Cantor). Optional PARI cross-check
(`hyperellcharpoly` for the curve's L-polynomial / `#Jac`, and genus-2 Jacobian arithmetic for divisor
sums) `#[ignore]`-gated (`#[ignore = "PARI not installed; run manually when available"]`). **Verify
gate:** `cargo test --workspace` green; the existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **Cantor compose+reduce must not fracture** — an unreduced divisor
(`deg u > g`) is not a canonical group element; compose without reduce produces wrong group structure.
The `2D == D+D` consistency KAT and the post-Cantor reduced-divisor re-check are the guards. (2)
**`−D = [u, (h+v) mod u]`, NOT `[u, −v]`** — in char 2 `−v = v`, and the hyperelliptic involution sends
`(x, y) → (x, −y−h(x)) = (x, y+h(x))`, so the divisor negation reflects `v → h+v` (the divisor-level
analogue of the elliptic `−P=(x,x+y)` trap). The `D+(−D)=0` KAT is the loud signal. (3) **This is the
E.I ◆ boundary** — re-read the Purpose intent and verify the Jacobian is complete (curve + polynomial
ring + Mumford representation + Cantor group law all present and cross-checked) and **descent-ready**
(C-Jacobian exposes what E.H's GHS/Weil descent lands on — the Jacobian over GF(2^m), the Mumford
representation, the group law, the genus), and that E.I stayed descent-free / DLP-solver-free /
chapter-free. (4) **No GHS/Weil descent and no Jacobian DLP solver here** — the descent (E.H) lands
*onto* this Jacobian and then runs index calculus; E.I builds the group, not the attack. A `@build`
agent that implements index calculus or the descent map is defocus. (5) **No MATHEMATICS chapter** —
the hyperelliptic/Cantor textbook content is T.E, paired with E.W at the *Track-E* ◆; E.I.3 writes at
most a PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
E.I.3 ◆ to confirm: (1) the hyperelliptic Jacobian is complete and composes (polynomial ring + curve +
Mumford representation + Cantor group law all present and cross-checked — the group axioms, `D+(−D)=0`,
associativity, `n·D=0` all pass); (2) C-Jacobian exposes what E.H descends onto (the Jacobian over
GF(2^m), the Mumford `[u,v]` representation, the Cantor group law, the genus `g`) so E.H can build the
GHS/Weil descent map and the subsequent index-calculus DLP without amending the Jacobian surface — the
substrate-readiness defense; (3) E.I stayed in scope — no GHS/Weil descent (E.H), no Jacobian DLP solver
/ index calculus (E.H), no MATHEMATICS chapter (T.E), the Jacobian group law is Cantor-on-Mumford-pairs
not point addition; (4) the principle-4 boundary (toy genus + field sizes; Cantor is
arbitrary-genus-crypto-scale-correct) is recorded, not silently presented as crypto-scale. **Also:
reconcile the static-frame ROADMAP debt** — (a) the Progress table is stale by **two** completed
sub-tracks (E.F, E.G now done; table shows "Done ~13 (E.A–E.E)"); (b) the Remaining table still lists
the now-complete E.F and does not strike E.G; and (c) **the Remaining table lists E.H before E.I, which
is dependency-inverted** — E.H's predecessor is E.G+E.I (ROADMAP:356), and the E.I ◆ should record that
E.I now precedes E.H in execution order. One-shot findings; does not implement. Held at **Opus** per the
header (juncture-tier — C-Jacobian → E.H cost-of-wrong).

---

## Cross-session contracts

E.I **freezes three** contracts (C-GF2mPoly at E.I.1, C-HyperCurve at E.I.2, C-Jacobian at E.I.3) and
**amends no prior frozen contract** (C-F2m / C-BinaryCurve / C-Koblitz / the p-adic / SSA / prime-field
surfaces are all consumed-or-untouched). The only `shared/gf2m` change is the **additive `poly`
module** (a new public module, not a trait amendment — unlike E.G.1, which filled the C-F2m stubs).

### C-GF2mPoly — free polynomial ring GF(2^m)[x] (compiler- + test-enforced) — *to be frozen at E.I.1*

**Defined in:** E.I.1 (`shared/gf2m/src/poly.rs`). **Consumed by:** E.I.2 (the Mumford divisor `u`/`v`
are GF(2^m)[x] polynomials), E.I.3 (Cantor's compose+reduce is polynomial extended-gcd/divmod);
**downstream: E.H** (the GHS function-field / Weil-restriction construction is polynomial work over
GF(2^m) and a subfield), **potentially E.K** (Semaev summation polynomials). Compiler-enforced (the
`Poly<F: F2m<L>>` type + the ring/divmod/gcd/xgcd method surface) + test-enforced (ring axioms, divmod
round-trip, gcd/xgcd). Exposes: `Poly<F: F2m<L>>` (coefficient `Vec<F>`, normalized, per-call
`poly: &Uint<L>` field-modulus idiom); `add`/`mul`/`divmod`/`gcd`/`xgcd`/`monic`/`derivative`/`degree`;
**over-specified** `resultant` and polynomial modular inverse (carried for E.H/E.K if confidence is
reasonable). **Char-2 invariants:** the formal derivative kills even-degree terms (`(x²)' = 0`); `add`
is XOR; `divmod` inverts the divisor's leading coefficient via C-F2m `inv`. *Exact representation
(coefficient `Vec<F>` vs packed bit-vector; whether `resultant`/`mod_inverse` ship now or defer)
ratified at the E.I.3 ◆.*

### C-HyperCurve — hyperelliptic curve `y²+h(x)y=f(x)` + Mumford divisor representation (compiler- + test-enforced) — *to be frozen at E.I.2*

**Defined in:** E.I.2 (`rho/src/hyperelliptic/mod.rs`). **Consumed by:** E.I.3 (Cantor operates on
Mumford divisors of this curve); **downstream: E.H** (GHS/Weil descent lands a DLP on the Jacobian of
*this* curve — the single highest-stakes consumer, "the most mathematically intricate single attack";
the cost-of-wrong that holds the ◆ juncture at Opus), **E.W** (the cross-attack benchmark table).
Compiler-enforced (the `HyperellipticCurve` type + `MumfordDivisor` + the `<F: F2m<L>>` methods) +
test-enforced (point-on-curve, the Mumford reduced-divisor invariant, divisor-from-points round-trip).
Exposes: `HyperellipticCurve` (`y²+h(x)y=f(x)`, holding `h`/`f` as `Poly<F>`/coefficients + the field
`poly`, genus `g = ⌊(deg f − 1)/2⌋`); `MumfordDivisor` (`[u(x), v(x)]`, `u` monic, `deg v < deg u ≤ g`,
the invariant `u | f − v·h − v²`); the zero divisor `[1, 0]`; `is_on_curve`/`is_valid`/divisor-from-points.
**Char-2 hyperelliptic invariants:** the model carries a general `h(x)` (`h ≠ 0` for the
non-supersingular case); the Mumford invariant `u | f − v·h − v²` is the curve-compatibility contract.
**Toy genus 2 + toy field sizes only** (principle-4 boundary). *Exact curve-parameter representation
(`Poly<F>` vs coefficient `Uint`; the imaginary/ramified vs real model — imaginary is the GHS target)
ratified at the E.I.3 ◆.*

### C-Jacobian — Cantor's algorithm: the hyperelliptic Jacobian group law (compiler- + test-enforced) — *to be frozen at E.I.3 ◆*

**Defined in:** E.I.3 (`rho/src/hyperelliptic/cantor.rs`). **Consumed by:** **E.H** (GHS/Weil descent
transfers the ECDLP onto this Jacobian's DLP, then runs index calculus — the highest-stakes consumer),
**E.W** (the benchmark table). Compiler- + test-enforced. Exposes the Jacobian group law: `compose` +
`reduce` (Cantor's algorithm — the divisor addition), `negate` (`[u, (h+v) mod u]`), the identity
(`[1, 0]`), `scalar_mul` (double-and-add over Cantor). **The frozen invariants:** every group operation
returns a **valid reduced divisor** (`deg u ≤ g`, the Mumford invariant holds); `D + (−D) = 0`; the
group is associative; `n·D = 0` for a divisor of order `n`. **`−D = [u, (h+v) mod u]`, NOT `[u, −v]`**
(the char-2 divisor-negation trap). **The Jacobian group law is Cantor-on-Mumford-pairs, NOT point
addition** (a genus-≥2 curve has no group on its points). **E.I freezes the group; the index-calculus
DLP attack on it is E.H** (the scope boundary). *Exact Cantor variant (the NUCOMP optimization vs plain
compose+reduce — plain is the demonstration-fidelity default; NUCOMP is a scale-only optimization,
principle 2, likely deferred) ratified at the ◆.*

### Frozen contracts read by E.I (consumed, not amended)

- **C-F2m (frozen surface)** — `add`(XOR)/`mul`/`square`/`frobenius`/`inv`/`div`/`pow` consumed by the
  GF(2^m)[x] ring (`divmod` needs `inv`; `mul` needs field `mul`) and threaded per-call. Read.
  **Unchanged — E.I amends no field contract** (the contrast with E.G.1, which filled the C-F2m stubs).
- **`rho::binary_curve::BinaryCurve` + `BinaryAffinePoint`** — the *design idiom* E.I's
  `HyperellipticCurve` mirrors (a const-generic-method curve, per-call field threading, no
  `PhantomData<F>`). **Read for the pattern; NOT reused** (the elliptic group law is point addition,
  not Cantor-on-divisors). Untouched.
- **`shared/gf2m::convert`** — the per-call `poly: &Uint<L>` idiom and the Frobenius-orbit machinery
  (read for the polynomial-substrate convention). Untouched.

### Workspace edges (no new edge, no new crate)

- **No new edge.** `rho` already depends on `shared-gf2m` (the edge E.G.1 added) and on `gnfs`. The
  GF(2^m)[x] ring is an **additive module in `shared/gf2m`** (`shared/gf2m/src/poly.rs`); the Jacobian
  is a new module in the existing `rho` crate (`rho::hyperelliptic`). No `Cargo.toml` changes; `cargo
  check --workspace` stays green with no cycle risk (no new edge to introduce one). *(If E.I found it
  must change C-F2m's frozen trait surface — it should not, it only adds a sibling `poly` module — that
  would be a discovery surfaced at the ◆, never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.I.3 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.I.1 | GF(2^m)[x] polynomial ring | done | 84a1110 | C-GF2mPoly (frozen) |
| E.I.2 | Hyperelliptic curve + Mumford divisor representation | pending | | C-HyperCurve |
| E.I.3 ◆ | Cantor's algorithm: Jacobian group law + sub-track close | pending | | C-Jacobian |

Contracts frozen before this sub-track: the GF(2^m) field surface (C-F2m/C-F2mOpt — read by E.I,
unchanged), the binary-curve surface (C-BinaryCurve/C-BinaryRho/C-Koblitz from E.G — untouched, the
*elliptic* curve), the p-adic surface (C-Padic/C-Hensel/C-PadicLog), the SSA surface
(C-AnomalousLift/C-SSA), the prime-field rho curve + ECDLP surface, `Fp<4>`. This sub-track **freezes
three new contracts** (C-GF2mPoly, C-HyperCurve, C-Jacobian), serving the downstream **E.H** (GHS/Weil
descent — descends onto C-Jacobian), **E.W** (cross-attack benchmarks), and **completes E.H's missing
predecessor** so the descent has a Jacobian to land on.

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.I builds the hyperelliptic Jacobian on the frozen C-F2m substrate + a new GF(2^m)[x] ring —
  internal-continue (confirmed by survey).** All greenfield; no existing Jacobian/divisor/Cantor code.
  A discovery that Cantor or the Mumford representation needs a polynomial operation C-GF2mPoly did not
  over-specify is an **additive amend of C-GF2mPoly** (add the operation to the ring) surfaced at the ◆
  — not a silent patch. A discovery that the Jacobian needs a *field* primitive C-F2m lacks is an
  additive C-F2m amend (the established pattern), surfaced at the ◆.

- **The Jacobian group law is NOT point addition — forcing the elliptic group law is a rigidity
  failure (the central design guard).** A genus-≥2 curve has no group on its points; the group lives on
  divisor classes (Mumford pairs) and the law is Cantor's algorithm. A `@build` agent reusing
  `rho::binary_curve`'s point addition writes a nonsense group. Cantor-on-Mumford-pairs is mandatory;
  the `D+(−D)=0` / associativity / `2D==D+D` KATs are the defense — **internal-continue → corrected.**

- **`−D = [u, (h+v) mod u]`, not `[u, −v]` — the char-2 divisor-negation trap.** The hyperelliptic
  involution is `(x,y) → (x, y+h(x))` in char 2 (`−y = y`), so the divisor negation reflects `v → h+v`,
  not `v → −v`. The `D+(−D)=0` KAT is the loud signal (mirrors the elliptic `−P=(x,x+y)` trap from E.G).
  **Internal-continue → corrected.**

- **The char-2 formal derivative kills even-degree terms (`(x²)' = 0`) — the polynomial-ring trap.** A
  `@build` agent porting an integer-coefficient derivative gets the char-2 derivative wrong, corrupting
  square-free / resultant work. The `(x²)' = 0`, `(x³)' = x²` KATs are the guard. **Internal-continue →
  corrected.**

- **Cantor compose+reduce is one irreducible unit — fracturing it (compose without reduce) is a
  lever-2 violation.** An unreduced divisor (`deg u > g`) is not a canonical group element. A `@build`
  agent landing compose-without-reduce produces a wrong group; the post-Cantor reduced-divisor re-check
  and `2D==D+D` KAT are the guards. **Internal-continue → corrected** (the reduce must always run).

- **No GHS/Weil descent in E.I (defocus guard — the central scope boundary).** The descent machinery
  (the Weil-restriction / function-field map transferring the ECDLP onto the Jacobian, then the
  index-calculus DLP solver over the subfield) is **E.H** (a different sub-track, the consumer of
  C-Jacobian, Opus-flagged at E.H.1). A `@build` agent that implements the descent or the Jacobian DLP
  solver in E.I is defocus. *(E.I's job is to expose C-Jacobian so E.H can descend onto it without
  amending it — build the group, not the attack.)*

- **No Jacobian DLP solver / index calculus in E.I (defocus / scope clarity).** The DLP *attack* on the
  Jacobian (index calculus, the relation collection, the linear algebra) is E.H's, not E.I's. E.I
  exposes the group law and arithmetic; the attack consumes it. A `@build` agent implementing index
  calculus in E.I is defocus.

- **No MATHEMATICS.md chapter in E.I (defocus / scope clarity).** The hyperelliptic / Cantor / Jacobian
  textbook content is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing), not
  at the E.I sub-track ◆. E.I.3 writes at most a PEDAGOGY code-tour delta.

- **No oracle dependency for correctness (principle-3 / E.D…E.G-consistent).** Jacobian arithmetic is
  exactly self-checking (group axioms + `D+(−D)=0` + associativity + `n·D=0` + the reduced-divisor
  invariant); a PARI `hyperellcharpoly` / genus-2 Jacobian cross-check is an **optional `#[ignore]`
  sidecar** (the established `#[ignore = "PARI not installed; run manually when available"]` pattern).
  E.I introduces no new live oracle.

- **Toy genus + field sizes only (scope clarity).** E.I fixes genus 2 (the canonical GHS target) and
  small `m`. The toy sizes are a principle-4 boundary — Cantor's algorithm is arbitrary-genus- and
  crypto-scale-correct; only the *parameters* are toy. Presenting any as crypto-scale is a documentation
  defect (internal-continue → corrected).

- **The GF(2^m)[x] ring's home is `shared/gf2m::poly` (module-placement call, ratified at the ◆).** The
  ring is field-substrate-shaped and reusable (E.H/E.K may consume it), so `shared/gf2m` is the
  principled home over `rho`. If a reviewer prefers `rho::hyperelliptic`, that is a ◆ ratification
  decision, not a blocker (no edge consequence either way — `rho` already depends on `shared-gf2m`).

- **Sequencing correction (reconcile at the E.I ◆, does NOT block E.I) — E.I precedes E.H.** The
  ROADMAP Remaining table lists E.H before E.I, but the dependency graph requires E.I first (E.H
  descends onto C-Jacobian; the survey confirmed E.I is entirely greenfield). The E.I ◆ juncture should
  record that E.I now precedes E.H in execution order, and strike the inversion from the Remaining
  table. Not an implementation concern.

- **Static-frame ROADMAP debt (reconcile at the E.I ◆, does NOT block E.I).** The ROADMAP Progress
  subsection is stale by two completed sub-tracks (E.F, E.G; table shows "Done ~13 (E.A–E.E)") and the
  Remaining table still lists the now-complete E.F and does not strike E.G. The E.I ◆ juncture should
  update them (Progress: Track E Done → E.A–E.G; Remaining: strike E.F and E.G). Not an implementation
  concern.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.I, "*GF(2^m) hyperelliptic Jacobian … Cantor's algorithm, divisor
  representation, Jacobian group law. Sonnet — well-understood material.*"; the design statement's
  principles 1 + 3 + 4; the "On scale" mathematical-dimension framing — the hyperelliptic Jacobian's
  field GF(2^m) and genus are *mathematical-dimension scale*, orthogonal to operational scale) and this
  PLAN before any session. **NOTE: the ROADMAP Progress / Remaining tables are stale (E.F, E.G done;
  E.F still listed remaining) AND list E.H before E.I (dependency-inverted — E.I precedes E.H);
  reconcile at the E.I ◆.**
- Read the **templates to mirror**: `rho/src/binary_curve/mod.rs` (the `BinaryCurve` / per-call
  `F: F2m<1>` curve-struct idiom — the *pattern* E.I's `HyperellipticCurve` mirrors as a parallel type,
  NOT the type to reuse; the elliptic group law is point addition, not Cantor-on-divisors);
  `rho/src/binary_ecdlp/mod.rs` (`scalar_mul` / double-and-add shape E.I.3's Jacobian `scalar_mul`
  mirrors); `shared/gf2m/src/convert.rs` (the per-call `poly: &Uint<L>` idiom + Frobenius-orbit
  machinery the GF(2^m)[x] ring follows); `shared/gf2m/src/inv.rs` (the private GF(2)[x] `poly_divmod` —
  the closest existing polynomial-division code, a starting reference for the GF(2^m)[x] divmod);
  `rho/tests/binary_curve_kat.rs` (the group-axiom KAT idiom E.I.3 mirrors).
- **Register:** E.I is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module `shared/gf2m/src/poly.rs` (the polynomial ring), new modules
  `rho/src/hyperelliptic/{mod,cantor}.rs` (the curve + Jacobian), and new KATs in
  `shared/gf2m/tests/gf2m_poly_kat.rs` + `rho/tests/hyperelliptic_kat.rs`.
- **Tier routing:** **All three E.I sessions are Sonnet `@build`** — the ROADMAP Opus-flagged-sessions
  table lists E.H.1 but NO E.I session, and ROADMAP names E.I "Sonnet — well-understood material." E.I.3
  carries the **◆ `@architect` juncture** (page `@plan-juncture`) ratifying C-GF2mPoly/C-HyperCurve/
  C-Jacobian and confirming descent-readiness before the sub-track closes. juncture-tier (header) is
  **opus** — held by lever 3 (C-Jacobian is descended onto by E.H, "the most mathematically intricate
  single attack in the project"); the strong lever-5 exactly-checkable KATs (group axioms + `D+(−D)=0` +
  `n·D=0` + PARI `hyperellcharpoly` sidecar) would license `sonnet` in isolation, but the user judged
  the C-Jacobian → E.H design-error cost decisive, mirroring the E.D/E.E/E.F/E.G substrate-adjacent-
  juncture calls.
- **Invariants to preserve:** **The Jacobian group law is Cantor-on-Mumford-pairs, NOT point addition**
  (a genus-≥2 curve has no group on its points; the `D+(−D)=0`/associativity KATs are the guard).
  **`−D = [u, (h+v) mod u]`** in char 2 (NOT `[u, −v]`; the `D+(−D)=0` KAT is the guard). **Cantor
  compose+reduce always runs both steps** (an unreduced divisor is not a group element; the
  reduced-divisor re-check is the guard). **The char-2 formal derivative kills even-degree terms**
  (`(x²)'=0`; the derivative KAT is the guard). **The Mumford invariant `u | f − v·h − v²` holds for
  every valid divisor** (the validity KAT is the guard). **E.I consumes the frozen C-F2m surface
  unchanged** (no field amend; the only `shared/gf2m` change is the additive `poly` module). **No
  GHS/Weil descent and no Jacobian DLP solver / index calculus** (E.H). **No MATHEMATICS chapter** (T.E
  at the Track-E ◆). Toy genus 2 + toy field sizes only; no new live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional `hyperellcharpoly` / genus-2 Jacobian
  cross-check follows the established `#[test] #[ignore = "PARI not installed; run manually when
  available"]` pattern; never on the green path.
- **No new edge, no new crate (load-bearing for E.I).** `rho` already depends on `shared-gf2m`; the
  GF(2^m)[x] ring is an additive `shared/gf2m::poly` module and the Jacobian is a new `rho::hyperelliptic`
  module. `cargo check --workspace` stays green with no cycle risk. If E.I finds it must change C-F2m's
  frozen trait surface (it should not — it only adds a sibling `poly` module), that is a discovery
  surfaced at the ◆.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  polynomial-ring substrate + a divisor-representation curve + a Cantor group law) is **new to this
  project** (the prior GF(2^m) consumer E.G built *elliptic* curves with point addition; the Jacobian's
  Cantor-on-divisors group law is a structurally different shape with no in-repo precedent). Per the
  unproven-shard-pattern guidance, halt at each boundary for a human glance until the pattern proves
  out. *(Tradeoff vs autonomous: `halt-at-boundaries` trades velocity for a per-boundary check on a
  novel pattern — the polynomial ring (E.I.1) and the Mumford representation (E.I.2) are the
  design-crux freezes E.H descends onto, and a wrong shape there is expensive to retrofit. If E.I.1 and
  E.I.2 land cleanly and their KATs confirm the representation, fall back to autonomous for E.I.3.)*
