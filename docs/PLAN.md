<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.H — GHS/Weil descent: ECDLP → hyperelliptic-Jacobian DLP transfer, "the most mathematically intricate single attack in the project")

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by levers 2 (irreducible complexity) and 3 (cost of
design error), with lever 5 too weak to license an opt-down.** E.H is "**the most mathematically
intricate single attack in the project**" (ROADMAP:357-358): GHS/Weil descent transfers the ECDLP on a
binary elliptic curve `E/GF(2^m)` to a DLP on the Jacobian of a hyperelliptic curve `C/GF(2^l)` over a
subfield (`l | m`), via the Artin–Schreier / function-field Weil-restriction construction. The descent
*algebra* (the function-field setup, the curve extraction) is the FLOOR (lever 2) — it cannot be
fractured below its irreducible mathematical content. The cost-of-design-error (lever 3) is high: a
wrong subfield-substrate or descent-algebra shape is the **most expensive retrofit in the project** (the
construction is intricate and there is no in-repo precedent). Lever 5 is **weaker than E.I's**: descent
correctness is checkable (PARI `hyperellcharpoly` on `#Jac` + the logarithm-preservation relation
`h = k·g ⟹ D_h = k·D_g`), but it is *not* exactly self-checking the way group axioms are — the
transfer-correctness KAT leans on a correct PARI oracle and a correct genus/order relationship, so lever
5 does not buy the `sonnet` opt-down. **This is a cleaner Opus call than E.I's** (where lever 5 was
strong and the user overrode it on lever 3); here levers 2+3 hold the juncture up directly and lever 5
is too weak to contest. *(E.H carries a session-level Opus flag on **E.H.1** — the subfield-substrate
design session — matching the ROADMAP Opus-flagged-sessions table, which lists E.H.1. The juncture-tier
governs the paged `@plan-juncture` fork at the ◆; the session tier governs the `@build` model per row.)*

**Scope boundary — load-bearing, surfaced and adjudicated at shard time (the central decision).** E.H
is the **transfer/reduction**, NOT the solve. The ROADMAP separates three sub-tracks: **E.H** (GHS/Weil
descent — transfer ECDLP → Jacobian DLP), **E.J** (Semaev summation polynomials — the combinatorial
heart of index calculus), and **E.K** (Gaudry–Diem–Joux–Vitse index calculus — the Jacobian
index-calculus DLP *solver*, consuming E.J). The substrate survey (forked `@explore`, 2026-06-15)
confirmed the Jacobian index calculus (relation collection, the divisor factor base, linear algebra over
`Z/ℓZ`) is **entirely greenfield and structurally belongs to E.K** — the existing `gnfs` index-calculus
machinery (`block_lanczos_fl`, the NFS-DL relation format) is number-field-coupled and not reusable for
a Jacobian DLP. **The user adjudicated transfer-only, verified structurally** (mirroring E.I, which
built the Jacobian group and left the attack to its consumer; and the MOV/SSA reduction idiom — transfer
verified by relationship-preservation, the solve delegated). E.H's sub-track-close KAT verifies the
transfer is **correct** (right genus, `#Jac` matches PARI, the logarithm relation `D_h = k·D_g` holds
via Cantor `scalar_mul` for known small `k`), **not** that an ECDLP is solved end-to-end — that closes
when E.K lands. *(Tradeoff named: no single end-to-end "ECDLP broken" KAT until E.K; bought to keep the
delicate descent design decoupled from the equally-delicate index-calculus design, the coupling the
inflection-juncture discipline warns against.)*

The substrate survey (forked `@explore`, 2026-06-15) established the shape and confirmed the planning
assumptions:

1. **E.H descends ONTO the frozen E.I Jacobian surface** (C-Jacobian / C-HyperCurve / C-GF2mPoly, all
   frozen at E.I, commits `84a1110`/`cb13d4a`/`06bd257`). The survey verified the public API:
   `HyperellipticCurve<L>` (fields `poly`/`h_coeffs`/`f_coeffs`, `genus()`, on-demand `h::<F>()`/`f::<F>()`),
   `MumfordDivisor<F,L>` (`u`/`v`, `zero_divisor`, `divisor_from_points`), and the free-function group
   law (`compose`/`reduce`/`add`/`negate`/`scalar_mul`/identity in `cantor.rs`). `Poly<F,L>` ships the
   **full** ring surface including `resultant` and `mod_inverse` (nothing deferred) — the descent algebra
   (function-field / Artin–Schreier work) consumes it. **E.H amends no E.I contract** (it *consumes*
   C-Jacobian, builds its descended curve as a `HyperellipticCurve`, and reads the group law).

2. **The subfield infrastructure GF(2^l) ⊂ GF(2^m) is entirely greenfield** (the load-bearing finding,
   item 4). The existing Frobenius machinery (`convert.rs::frobenius_orbit`, `is_normal_element`) is
   **GF(2^m)-over-GF(2) only** — the orbit spans the *whole* field. E.H needs **relative** machinery:
   subfield embedding, relative trace `Tr_{m/l}` and norm `N_{m/l}`, Frobenius-by-subfield orbit
   (iterate `frobenius` `l` times). **E.H.1 builds this first** — the substrate the Weil restriction
   stands on, and a genuinely reusable one (E.K's index calculus over the subfield may consume it —
   over-specify per the substrate rule). This is why **E.H.1 is the Opus-flagged session.**

3. **The GHS construction is fully greenfield** (item 6 — zero `ghs`/`weil-descent`/`subfield`/
   `weil_restrict` code anywhere; the only `weil_pairing` hits are the MOV Weil pairing over `F_p`,
   unrelated). E.H builds the Artin–Schreier / function-field Weil-restriction construction and extracts
   the hyperelliptic curve `C/GF(2^l)` from scratch in `rho` (a new `rho::ghs` module).

4. **The descended curve is the imaginary/ramified model — resolves the E.I.3 ◆ conditional.** The E.I.3
   ◆ digest flagged: "if E.H targets the real/split model rather than imaginary/ramified, the genus
   formula and `[1,0]` reduction would need amendment — evidence points to imaginary; confirm at E.H
   kickoff." The survey **confirmed** the frozen C-HyperCurve/C-Jacobian code assumes the imaginary model
   exclusively (`genus() = (deg f − 1)/2`, doc "imaginary/ramified model: deg f = 2g+1", single point at
   infinity; `cantor.rs` reduce has no split-model branch). **GHS with odd extension degree `m/l`
   produces imaginary-model curves** (the standard case), so the frozen surface is compatible and **no
   C-HyperCurve amend is needed for E.H's standard target.** *(E.H.1 confirms-and-records this; if E.H
   ever needs the real/split model — even `m/l` — that is an additive C-HyperCurve amend surfaced at the
   ◆, never a silent patch. The fixture is chosen with odd `m/l` to stay in-model — surfaced below.)*

5. **The `BinaryCurve` source surface is hardcoded `Uint<1>`** (item 3 — GF(2^4)/GF(2^8) toy scale).
   GHS's mathematically interesting cases are over composite-degree fields, but the existing toy
   `BinaryCurve` is `L=1`. **E.H targets a tiny composite `m` (e.g. `m = 4 = 2·2`, `l = 2`, or
   `m = 6 = 2·3`, `l = 2`/`l = 3`) that fits `Uint<1>`** — keeping E.H from amending the frozen
   C-BinaryCurve surface. *(Design tension to resolve at E.H.1: if no in-`Uint<1>` composite `m` exhibits
   a non-degenerate descent, generalizing `BinaryCurve` to `L>1` is a **surfaced discovery** at the ◆,
   not silent in-flight scope growth — the established C1-widening discipline. The survey's read is that
   a toy composite `m ≤ 8` suffices for demonstration fidelity.)*

6. **`rho` already depends on every needed crate; no new edge, no new crate** (item 1). `rho → gnfs`,
   `rho → shared-gf2m`, `rho → shared-field`, `rho → shared-padic` all exist. The subfield substrate is
   an **additive `shared/gf2m` module** (new `shared/gf2m/src/subfield.rs`) and the GHS descent lives in
   `rho` (new `rho::ghs`). No `Cargo.toml` edge changes; `cargo check --workspace` stays green (no new
   edge to cycle). *(One asymmetry vs E.I: `gnfs` does NOT depend on `shared-gf2m`, confirming the
   Jacobian DLP solver cannot live in `gnfs` and reuse NFS-DL — reinforcing the E.H/E.K boundary.)*

The work splits at **five representation/contract-sharp seams**, **5 sessions** (matching the ROADMAP's
5-session estimate, confirmed by the project's documented ceiling-bias), at the boundaries between the
subfield substrate, the descent algebra, the curve extraction, the transfer map, and the reduction
wiring:

1. **E.H.1 — Subfield substrate GF(2^l) ⊂ GF(2^m) (Opus, Cat A).** Subfield embedding, relative trace
   `Tr_{m/l}` / norm `N_{m/l}`, Frobenius-by-subfield orbit, subfield-membership test. **Freezes
   C-Subfield** — the substrate the Weil restriction stands on. Additive `shared/gf2m` module. **Opus**
   per the ROADMAP flag (substrate design with downstream consumption — E.H.2–5 and potentially E.K).
   Confirms-and-records the imaginary-model target and the `Uint<1>` composite-`m` fixture choice.

2. **E.H.2 — Artin–Schreier / function-field Weil-restriction algebra (Sonnet, Cat B).** The descent
   algebra: the Artin–Schreier extension of the function field, the Weil restriction of scalars from
   GF(2^m) to GF(2^l), the polynomial-over-GF(2^m)-and-subfield machinery the curve extraction consumes.
   **Consumes C-Subfield, C-GF2mPoly.** **Freezes C-DescentAlgebra.**

3. **E.H.3 — GHS hyperelliptic-curve extraction (Sonnet, Cat B).** Extract the hyperelliptic curve
   `C/GF(2^l)` (`y² + h(x)y = f(x)`, the GHS image) from the descent algebra; verify it as a frozen
   `HyperellipticCurve`; compute/relate its genus to the extension degree. **Consumes C-DescentAlgebra,
   C-HyperCurve (frozen E.I, read).** **Freezes C-GHSCurve.**

4. **E.H.4 — The transfer map E(GF(2^m)) → Jac(C)(GF(2^l)) (Sonnet, Cat B).** Map a point `P ∈ E(GF(2^m))`
   to a reduced Mumford divisor `D_P ∈ Jac(C)(GF(2^l))`; the homomorphism property
   (`D_{P+Q} = D_P + D_Q` via Cantor). **Consumes C-GHSCurve, C-Jacobian (frozen E.I, read).** **Freezes
   C-DescentMap.**

5. **E.H.5 ◆ — ECDLP → Jacobian-DLP reduction + sub-track close (Sonnet, Cat I, `@architect`).** Wire the
   reduction: given an ECDLP instance `(E, g, h)` find the Jacobian-DLP instance `(C, D_g, D_h)` such that
   `log_g h = log_{D_g} D_h`; the logarithm-preservation KAT + PARI `#Jac` oracle; the sub-track close.
   **Consumes C-DescentMap, C-GHSCurve, C-Jacobian.** **Freezes C-GHSDescent** — the surface E.K's index
   calculus and E.W's benchmark table consume. Crosses the **E.H ◆ boundary** — the descent ships,
   transfer-correct and verified, ready for E.K to solve the resulting Jacobian DLP.

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the Jacobian *index-calculus DLP
solver* / relation collection / linear algebra over `Z/ℓZ` — that is **E.K**, consuming **E.J**'s Semaev
polynomials; or building Semaev polynomials — that is E.J; or re-deriving the Jacobian group law — that
is E.I, frozen; or writing the GHS *textbook chapter* in MATHEMATICS.md — that is **T.E**, paired with
E.W at the Track-E ◆; E.H.5 writes at most a PEDAGOGY code-tour delta) and **rigidity** (forcing the
real/split hyperelliptic model when GHS-with-odd-`m/l` gives the imaginary model the frozen C-HyperCurve
already handles; or re-deriving the GF(2^m)[x] ring / the Cantor group law rather than consuming the
frozen E.I surfaces; or amending the frozen C-BinaryCurve `Uint<1>` surface in-flight rather than
choosing a composite `m` that fits — generalizing to `L>1` is a surfaced discovery, not a silent edit).

**Scoping discipline.** E.H builds the GHS/Weil descent at **demonstration fidelity** (principle 1 —
algorithmic content complete: the subfield substrate, the Artin–Schreier/function-field algebra, the
curve extraction, and the transfer map all implemented head-on) and **toy field/extension sizes** (small
composite `m ≤ 8`, `l | m` with odd `m/l`; genus follows the extension degree; the descended curve is
the canonical GHS imaginary-model target). It **amends no frozen contract** (C-Jacobian / C-HyperCurve /
C-GF2mPoly / C-F2m / C-BinaryCurve / C-Koblitz are all consumed-or-untouched; the only `shared/gf2m`
change is the additive `subfield` module). It introduces **no Jacobian DLP solver** (transfer-only,
verified structurally — the index-calculus solve is E.K). The correctness signal is the
**logarithm-preservation relation** (`D_h = k·D_g` for known small `k`, via the frozen Cantor
`scalar_mul`) + the genus/order checks; an optional PARI `hyperellcharpoly` `#[ignore]` sidecar on `#Jac`
is the established dev-only oracle. The **engineering-vs-mathematics disconnect** (ROADMAP principle 4)
is explicit: the toy `m`/`l`/genus are a principle-4 boundary (GHS is crypto-scale-correct over composite
binary fields; only the *parameters* are toy), annotated, never presented as crypto-scale.

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.H): "*E.H — GHS/Weil descent. 4-5 sessions. Predecessor: E.G, E.I. Transfer
ECDLP on a binary curve to DLP on a hyperelliptic Jacobian over a subfield. First session is Opus-tier —
the descent machinery is the most mathematically intricate single attack in the project.*" E.H is the
**consumer of E.I's frozen Jacobian** (C-Jacobian) and the **structurally hardest attack in the
project**: where E.I built the hyperelliptic curve and its Jacobian group law (Cantor on Mumford pairs),
E.H builds the **GHS/Weil-descent transfer** that maps an ECDLP on a binary elliptic curve `E/GF(2^m)`
onto a DLP on the Jacobian `Jac(C)/GF(2^l)` of a hyperelliptic curve over a subfield. The descent is the
function-field / Artin–Schreier Weil-restriction construction: it builds `C/GF(2^l)` from `E/GF(2^m)`
and transports the discrete-log problem onto `Jac(C)`, where (in E.K) index calculus solves it faster
than generic search on `E`.

E.H's structural predecessors are **E.G** (binary curves `E/GF(2^m)`, the descent source — C-BinaryCurve
/ C-Koblitz, frozen, read) and **E.I** (the hyperelliptic Jacobian, the descent target — C-Jacobian /
C-HyperCurve / C-GF2mPoly, frozen, read). E.H **amends neither**: it consumes the binary curve as the
ECDLP source and the Jacobian as the DLP destination, and builds the *map between them*. The central
design tension is that **the descent is a transfer, not a solve**: GHS does not break the ECDLP directly;
it *relocates* it to a group (the Jacobian) where index calculus (E.K) is asymptotically faster. E.H's
job is to build the relocation correctly and prove it preserves the logarithm; the speed-up is realised
only when E.K's index calculus runs on the relocated problem.

The sub-track decomposes into five conceptual units, each a session:

1. **The subfield substrate GF(2^l) ⊂ GF(2^m) (E.H.1, Opus).** Subfield embedding, relative trace
   `Tr_{m/l}` / norm `N_{m/l}`, Frobenius-by-subfield orbit, subfield-membership — greenfield (the survey
   confirmed the existing Frobenius machinery is GF(2^m)-over-GF(2) only). The substrate the Weil
   restriction stands on; reusable (E.K may consume). **Freezes C-Subfield.** **(E.H.1.)**

2. **The Artin–Schreier / function-field Weil-restriction algebra (E.H.2).** The descent algebra — the
   Artin–Schreier extension of the function field of `E`, the Weil restriction of scalars to GF(2^l), the
   polynomial machinery over GF(2^m) and the subfield. **Freezes C-DescentAlgebra.** **(E.H.2.)**

3. **The GHS hyperelliptic-curve extraction (E.H.3).** Extract `C/GF(2^l)` (`y²+h(x)y=f(x)`, imaginary
   model) from the descent algebra, verify it as a frozen `HyperellipticCurve`, relate its genus to the
   extension degree. **Freezes C-GHSCurve.** **(E.H.3.)**

4. **The transfer map E(GF(2^m)) → Jac(C)(GF(2^l)) (E.H.4).** Map points to reduced Mumford divisors; the
   homomorphism property via Cantor. **Freezes C-DescentMap.** **(E.H.4.)**

5. **The ECDLP → Jacobian-DLP reduction + close (E.H.5 ◆).** The reduction wiring `(E,g,h) → (C,D_g,D_h)`;
   the logarithm-preservation KAT (`D_h = k·D_g`) + PARI `#Jac` oracle; the sub-track close. **Freezes
   C-GHSDescent.** **(E.H.5 ◆.)**

E.H is **solver-free** (the Jacobian index-calculus DLP solver is E.K — E.H builds the transfer the
solver consumes, not the solver), **Semaev-free** (the summation polynomials are E.J), **Jacobian-group-
frozen** (it consumes C-Jacobian unchanged — no group-law amend), and **chapter-free** (the GHS textbook
content is T.E, paired with E.W at the Track-E ◆). Re-read this intent at the ◆ boundary to catch defocus
(the Jacobian DLP solver / index calculus / Semaev polynomials, the MATHEMATICS chapter) and rigidity
(the real/split model when GHS gives imaginary; re-deriving the frozen E.I surfaces; amending the
C-BinaryCurve `Uint<1>` surface in-flight).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (survey re-confirmed zero hits, 2026-06-15); raw `cargo`
is the only CI surface (unchanged from E.D…E.I). Oracle KATs are `#[ignore]`-gated only — the exact form
is `#[ignore = "PARI not installed; run manually when available"]`, used identically in
`rho/tests/ssa_kat.rs`, `rho/tests/mov_kat.rs`, `rho/tests/hyperelliptic_kat.rs`, and
`shared/padic/tests/log_kat.rs`. `/run-plan` re-discovers at preflight. E.H **adds no new workspace edge
and no new crate** (`rho` already depends on `gnfs`/`shared-gf2m`/`shared-field`/`shared-padic`; the
subfield substrate is an additive `shared/gf2m` module, the descent a new `rho` module), so the gate is a
**correctness + no-regression gate**:

- Each session's KATs are the primary correctness signal — fast and decisive (lever 5): for E.H.1,
  the relative trace/norm identities (`Tr_{m/l}(a) ∈ GF(2^l)`, `N_{m/l}` multiplicative, the
  embed∘restrict round-trip, Frobenius-by-subfield fixed-point = subfield membership) for C-Subfield;
  for E.H.2/E.H.3, the descent-algebra round-trips and the extracted curve's validity (it *is* a valid
  `HyperellipticCurve`, the genus matches the extension degree) for C-DescentAlgebra/C-GHSCurve; for
  E.H.4, the transfer-map **homomorphism** (`D_{P+Q} = D_P + D_Q` via Cantor) for C-DescentMap; for
  E.H.5, the **logarithm-preservation** relation (`D_h = k·D_g` for known small `k` via the frozen
  Cantor `scalar_mul`) + the `#Jac`/genus checks for C-GHSDescent. The logarithm-preservation KAT is the
  decisive transfer-correctness signal (the descent is correct iff it preserves the log).
- `cargo check --workspace` must stay green — **no edge change**, so no cycle risk. The additive
  `shared/gf2m::subfield` module is a leaf addition.
- **The existing rho / gnfs / shared KATs must stay green** after the descent code lands — E.H adds new
  modules (`shared/gf2m::subfield`, `rho::ghs`) and changes no existing field / curve / Jacobian / rho
  path, so the no-regression invariant is structurally easy to hold; `cargo test --workspace` is the
  guard. *(One subtlety: the new `shared/gf2m::subfield` module touches `shared/gf2m`'s public surface
  additively — the existing `gf2m_kat.rs` and `gf2m_poly_kat.rs` must stay green and the subfield
  substrate gets its own `gf2m_subfield_kat.rs`.)*
- **PARI oracle:** the `hyperellcharpoly` `#[ignore]` sidecar (cross-checking the descended curve's
  `#Jac` / L-polynomial against the binary curve's group order via the descent relationship) follows the
  established `#[ignore = "PARI not installed; run manually when available"]` pattern; never on the green
  path. The green-path transfer-correctness signal is the self-checking logarithm-preservation relation.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.H.1 | Subfield substrate GF(2^l) ⊂ GF(2^m): embedding + relative trace/norm + Frobenius-by-subfield | A | **Opus** | C-F2m (frozen — `mul`/`square`/`frobenius`/`pow`/`trace` read; no amend); `shared/gf2m::convert` (read — the `frobenius_orbit` / per-call `poly` idiom to extend) | `shared/gf2m/src/subfield.rs` (new: subfield embed/restrict + `Tr_{m/l}` + `N_{m/l}` + Frobenius-by-subfield orbit + membership), `shared/gf2m/src/lib.rs` (add `pub mod subfield;` + re-export), `shared/gf2m/tests/gf2m_subfield_kat.rs` (new: trace/norm identities, embed∘restrict round-trip, subfield-membership KATs) |
| E.H.2 | Artin–Schreier / function-field Weil-restriction algebra | B | Sonnet | C-Subfield (frozen E.H.1); C-GF2mPoly (frozen E.I, read — the descent algebra is polynomial-over-GF(2^m) work); C-F2m (read) | `rho/src/ghs/mod.rs` (new: `GhsError` enum + the toy binary-curve fixture + the Artin–Schreier extension setup), `rho/src/ghs/descent.rs` (new: the Weil-restriction algebra over GF(2^m)/GF(2^l)), `rho/src/lib.rs` (add `pub mod ghs;`), `rho/tests/ghs_kat.rs` (new: descent-algebra round-trip KATs) |
| E.H.3 | GHS hyperelliptic-curve extraction `C/GF(2^l)` (imaginary model) | B | Sonnet | C-DescentAlgebra (frozen E.H.2); C-HyperCurve (frozen E.I, read — the extracted curve *is* a `HyperellipticCurve`); C-Subfield (read) | `rho/src/ghs/curve.rs` (new: extract `C/GF(2^l)` = `HyperellipticCurve`, genus↔extension-degree relation, imaginary-model validity), `rho/src/ghs/mod.rs` (add `pub mod curve;`), `rho/tests/ghs_kat.rs` (extend: extracted-curve validity, genus = expected from `m/l`, the curve is a valid frozen `HyperellipticCurve`) |
| E.H.4 | Transfer map E(GF(2^m)) → Jac(C)(GF(2^l)): point → Mumford divisor + homomorphism | B | Sonnet | C-GHSCurve (frozen E.H.3); C-Jacobian (frozen E.I, read — `compose`/`add`/`divisor_from_points`/`scalar_mul`); C-BinaryCurve (frozen E.G, read — the source point type) | `rho/src/ghs/transfer.rs` (new: `P ∈ E(GF(2^m)) ↦ D_P ∈ Jac(C)(GF(2^l))` + the homomorphism property), `rho/src/ghs/mod.rs` (add `pub mod transfer;`), `rho/tests/ghs_kat.rs` (extend: `D_{P+Q} = D_P + D_Q` via Cantor, transfer of the base point and a sample point) |
| E.H.5 ◆ `@architect` | GHS/Weil descent: ECDLP → Jacobian-DLP reduction + sub-track close | I | Sonnet | C-DescentMap (frozen E.H.4); C-GHSCurve (frozen E.H.3, read); C-Jacobian (frozen E.I, read — `scalar_mul` for log-preservation); `rho::ssa` reduction idiom (read — the verify→transfer→reduce shape) | `rho/src/ghs/reduce.rs` (new: `ghs_descend((E,g,h)) → (C, D_g, D_h)` reduction + logarithm-preservation), `rho/src/ghs/mod.rs` (add `pub mod reduce;` + `pub use`), `rho/tests/ghs_kat.rs` (extend: `D_h = k·D_g` for known `k`, `#Jac`/genus checks, sub-track-close suite; optional PARI `hyperellcharpoly` `#[ignore]` sidecar) |

**Sequencing notes.** Strictly serial: **E.H.1 → E.H.2 → E.H.3 → E.H.4 → E.H.5.** E.H.1 lands the
subfield substrate the descent stands on; E.H.2 the Weil-restriction algebra; E.H.3 extracts the curve
(consuming the algebra); E.H.4 the transfer map (consuming the curve + the frozen Jacobian); E.H.5 wires
the reduction and closes. **One `@architect` marker** sits on the **E.H.5 ◆** (the boundary juncture
ratifying the five frozen contracts and confirming the descent is transfer-correct and E.K-ready before
the sub-track closes). *(Tradeoff named: E.H freezes four contracts before the ◆ — C-Subfield (E.H.1),
C-DescentAlgebra (E.H.2), C-GHSCurve (E.H.3), C-DescentMap (E.H.4) — but pages NO inline juncture there,
mirroring the E.G/E.I calls. The in-crate orthogonality (each later session consumes the earlier freeze
immediately, where a wrong shape fails the next session's KAT loudly) plus the single primary downstream
consumer (E.K, which has its own Opus-flagged E.K.1 to design the index calculus) makes the early-catch
insurance less valuable than a separate inline fork would cost. The ◆ juncture is held at Opus
(juncture-tier) for the descent-algebra cost-of-wrong, but separate inline forks are not bought. If a
later session's KAT surfaces a substrate concern — e.g. the transfer-map homomorphism fails because
C-DescentAlgebra has the wrong shape — that is the loud signal that substitutes for an inline juncture.)*

**Why 5 sessions (the ROADMAP's 5-session estimate, confirmed by ceiling-bias).** The split is taken at
five representation/contract-sharp seams:
- **One-line-commit-title corollary.** "Subfield substrate", "Artin–Schreier Weil-restriction algebra",
  "GHS curve extraction", "Transfer map E → Jac", and "ECDLP → Jacobian-DLP reduction + close" are **five
  distinct commit titles** across three categories (A substrate ×1, B algorithm ×3, I integrative ×1).
- **Irreducible units kept whole (lever 2).** Each session is one conceptual unit: the subfield
  substrate, the descent algebra, the curve extraction, the transfer map, the reduction. The descent
  **algebra↔curve seam** (E.H.2↔E.H.3) is the one deliberated split (see the shard-time decision below):
  the Artin–Schreier / Weil-restriction *algebra* (the function-field setup) is contract-distinct from
  the *curve extraction* it feeds — freezing C-DescentAlgebra at E.H.2 buys an early contract on the
  hardest piece, lowering the cost-of-wrong on "the most mathematically intricate single attack."
- **Contract-sharp boundary.** E.H.1 **freezes** C-Subfield; E.H.2 consumes it and **freezes**
  C-DescentAlgebra; E.H.3 consumes that and **freezes** C-GHSCurve; E.H.4 consumes the curve + the frozen
  Jacobian and **freezes** C-DescentMap; E.H.5 consumes the map and **freezes** C-GHSDescent. Each later
  session is meaningless without the earlier freeze — which is what licenses and bounds the 5-way split.

**The deliberated algebra↔curve split (shard-time decision, user-adjudicated).** The GHS construction
*could* be one session (algebra + extraction as one irreducible unit, giving 4 sessions). The user chose
**5: split the GHS construction at the algebra↔curve seam**, matching the ROADMAP estimate and the
project's documented ceiling-bias (G/D both landed at or above their upper bands). The split buys an
early contract freeze (C-DescentAlgebra) on the most expensive-to-retrofit piece. **If E.H.2 and E.H.3
prove tightly coupled** (the Artin–Schreier setup and the curve extraction have no clean seam — the
extraction is a thin wrapper over the algebra), the split is artificial and **E.H.2/E.H.3 should
re-merge** — a judgment E.H.2 can surface once the algebra shape is concrete (an additive-reshard
discovery, not a silent merge). This is the one place the 5-vs-4 sizing is genuinely uncertain until the
algebra lands.

---

## Session detail

E.H.1 and E.H.2 are specified at near-full fidelity (the subfield substrate and the descent algebra are
the design crux the whole descent — and downstream E.K — stand on). E.H.3–E.H.5 are lower-fidelity
sketches, correct per the substrate-first discipline: they are crisply specified only after C-Subfield
and C-DescentAlgebra freeze.

### E.H.1 — Subfield substrate GF(2^l) ⊂ GF(2^m) (Opus, Cat A)

**Deliverable:** the relative-field substrate the Weil restriction stands on — greenfield (the survey
confirmed the existing Frobenius machinery in `convert.rs` is GF(2^m)-over-GF(2) only: `frobenius_orbit`
spans the *whole* field, there is no relative trace/norm, no subfield embedding). The pieces:
- **Subfield embedding / restriction** (`shared/gf2m/src/subfield.rs`): the embedding `GF(2^l) ↪ GF(2^m)`
  (`l | m`) and the partial restriction `GF(2^m) ⊃ GF(2^l)` (defined on subfield elements). The subfield
  is `{a ∈ GF(2^m) : a^(2^l) = a}` (the fixed field of the `l`-th Frobenius power). Threads the field
  modulus per-call (`poly: &Uint<L>`), the C-F2m idiom.
- **Relative trace and norm** (`shared/gf2m/src/subfield.rs`): `Tr_{m/l}(a) = Σ_{i=0}^{m/l−1} a^(2^(il))`
  (the relative trace, landing in GF(2^l)) and `N_{m/l}(a) = Π_{i=0}^{m/l−1} a^(2^(il))` (the relative
  norm, multiplicative, landing in GF(2^l)). These iterate the `l`-th Frobenius power, distinct from the
  absolute `trace` (to GF(2)) already on the C-F2m trait. The **char-2 trap**: the relative trace is over
  the *subfield* tower, not the absolute tower — iterate `frobenius` in steps of `l`, not `1`.
- **Frobenius-by-subfield orbit + membership** (`shared/gf2m/src/subfield.rs`): the orbit
  `[a, a^(2^l), a^(2^(2l)), …]` (length `m/l`), and `is_in_subfield(a, l) = (a^(2^l) == a)`. Over-specify
  (substrate rule): include the subfield *basis* (a normal/polynomial basis for GF(2^l) inside GF(2^m))
  if confidence is reasonable — E.K's index calculus over the subfield may consume it.
- **Confirm-and-record (the load-bearing E.H.1 acts):** (a) **the imaginary-model target** — record that
  GHS with odd `m/l` produces the imaginary/ramified hyperelliptic model the frozen C-HyperCurve handles
  (resolving the E.I.3 ◆ conditional); (b) **the `Uint<1>` composite-`m` fixture** — choose a toy
  composite `m ≤ 8` with `l | m`, odd `m/l` (e.g. `m=4, l=2, m/l=2` is *even* — degenerate; `m=6, l=2,
  m/l=3` odd, or `m=6, l=3, m/l=2` even; the survey read is a small composite exists; **resolving the
  exact fixture is an E.H.1 design act** and a KAT asserts the descent is non-degenerate at it). If no
  in-`Uint<1>` composite `m` gives a non-degenerate odd-`m/l` descent, **generalizing `BinaryCurve` to
  `L>1` is a surfaced discovery at the ◆** (the C1-widening discipline), never silent in-flight growth.

Consumes C-F2m (frozen — `mul`/`square`/`frobenius`/`pow`/`trace` read; **no amend**), the
`convert.rs::frobenius_orbit` idiom (read — extended to relative orbits). **Freezes C-Subfield.**

**KAT** (`shared/gf2m/tests/gf2m_subfield_kat.rs` + inline unit tests): over toy fields (GF(2^6) over
GF(2^2) and GF(2^3), with explicit irreducibles): **trace landing** (`Tr_{m/l}(a) ∈ GF(2^l)` —
`is_in_subfield(Tr_{m/l}(a), l)`); **trace additivity** (`Tr_{m/l}(a+b) = Tr_{m/l}(a) + Tr_{m/l}(b)`);
**norm multiplicativity** (`N_{m/l}(a·b) = N_{m/l}(a)·N_{m/l}(b)`, landing in GF(2^l)); **embed∘restrict
round-trip** (`restrict(embed(c)) = c` for `c ∈ GF(2^l)`); **subfield membership** (`is_in_subfield(a, l)
⟺ a^(2^l) = a`, and GF(2^l) has exactly `2^l` elements); **Frobenius-by-subfield orbit** (length `m/l`,
`a^(2^(il))` correct). **Verify gate:** `cargo test --workspace` green; `cargo check --workspace` green
(additive module, no edge change); existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **The relative trace/norm iterate the `l`-th Frobenius power, not the
1st** — `Tr_{m/l}(a) = Σ a^(2^(il))` sums `m/l` terms stepping by `l`, NOT the `m` terms of the absolute
trace. A `@build` agent reusing the absolute `trace` (already on C-F2m) writes the wrong map; the
"trace lands in GF(2^l)" KAT is the loud signal (the absolute trace lands in GF(2), a *different*
subfield). (2) **The subfield is the Frobenius fixed field** — `GF(2^l) = {a : a^(2^l) = a}`, characterised
by membership, not by a stored basis. (3) **`l | m` is mandatory** — GF(2^l) ⊆ GF(2^m) iff `l | m`; a
fixture with `l ∤ m` has no subfield and the whole descent is undefined (assert `l | m` loudly). (4)
**Odd `m/l` keeps the descent in the imaginary model** — the genus of the GHS curve scales with the
extension degree, and odd `m/l` lands the imaginary/ramified model the frozen C-HyperCurve handles; even
`m/l` risks the real/split model (an additive C-HyperCurve amend). The fixture choice (act (b) above) is
constrained by this. (5) **Module placement** — the substrate lives in `shared/gf2m` (not `rho`) because
it is field-substrate-shaped and reusable (E.K may consume it); the principled home, ratified at the ◆.

**Deferred:** the descent algebra (E.H.2); the curve extraction (E.H.3); the transfer map (E.H.4); the
reduction (E.H.5); the Jacobian index-calculus DLP solver (E.K); Semaev polynomials (E.J); the
MATHEMATICS chapter (T.E at the Track-E ◆).

### E.H.2 — Artin–Schreier / function-field Weil-restriction algebra (Sonnet, Cat B)

**Deliverable:** the descent algebra — the Artin–Schreier / function-field Weil-restriction machinery
that the GHS curve extraction (E.H.3) consumes. Near-full fidelity (the algebra is the descent crux).
The pieces:
- **The `rho::ghs` module skeleton** (`rho/src/ghs/mod.rs`): a `GhsError` enum (the established
  reduction-attack idiom — cf. `rho::ssa::SsaError`), the toy binary-curve fixture (the composite-`m`
  `BinaryCurve` chosen at E.H.1), and the precondition verifier (`l | m`, the curve admits the descent).
- **The Artin–Schreier extension** (`rho/src/ghs/descent.rs`): the char-2 Artin–Schreier extension of the
  function field of `E/GF(2^m)` (the GHS construction builds the hyperelliptic function field as an
  Artin–Schreier extension `y² + y = ...` lifted through the Weil restriction). The char-2 Artin–Schreier
  form `y^2 - y = f` is *the* char-2 separable-extension primitive — the descent's algebraic core.
- **The Weil restriction of scalars** (`rho/src/ghs/descent.rs`): restrict the function-field data from
  GF(2^m) to GF(2^l) using the C-Subfield relative trace/norm and the Frobenius-by-subfield orbit (the
  Weil restriction `Res_{GF(2^m)/GF(2^l)}` expresses GF(2^m)-objects as GF(2^l)-objects of dimension
  `m/l`). This is the polynomial-over-GF(2^m)-and-subfield work consuming C-GF2mPoly.

Consumes C-Subfield (frozen E.H.1), C-GF2mPoly (frozen E.I, read), C-F2m (read), the `rho::ssa` module
idiom (read — the error-enum + fixture + verifier shape). **Freezes C-DescentAlgebra.**

**KAT** (`rho/tests/ghs_kat.rs` + inline unit tests): over the toy composite-field fixture: the
**Artin–Schreier round-trip** (the extension is well-formed — `y² + y = f` has the expected structure);
the **Weil-restriction dimension** (a GF(2^m)-object restricts to an `m/l`-dimensional GF(2^l)-object);
the **precondition verifier** (`l | m` accepted, `l ∤ m` rejected with `GhsError`). **Verify gate:**
`cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The char-2 Artin–Schreier form is `y² + y = f`, NOT `y² = f`** — in
char 2 the separable degree-2 extensions are Artin–Schreier (`℘(y) = y² + y`), not Kummer (`y² = f`,
which is inseparable in char 2). A `@build` agent porting an odd-char Kummer construction writes an
inseparable extension with no GHS curve. (2) **The Weil restriction lowers the field and raises the
dimension** — `Res_{m/l}` of a 1-dimensional GF(2^m)-object is an `m/l`-dimensional GF(2^l)-object; the
genus of the GHS curve grows with `m/l` (the source of the descent's leverage and its toy-scale genus).
(3) **The descent algebra consumes C-Subfield's relative trace/norm, not the absolute trace** — the Weil
restriction is *relative* to GF(2^l) (the subtlety E.H.1's KAT guards). (4) **The algebra↔curve seam** —
if the curve extraction (E.H.3) turns out to be a thin wrapper over this algebra (no clean seam), surface
an additive-reshard merge of E.H.2/E.H.3 at the ◆; if the seam is genuine (the algebra is a reusable
function-field layer the extraction reads), the split holds.

**Deferred:** the curve extraction (E.H.3); the transfer map (E.H.4); the reduction (E.H.5); the
Jacobian DLP solver / Semaev (E.K/E.J); the MATHEMATICS chapter (T.E).

### E.H.3 — GHS hyperelliptic-curve extraction `C/GF(2^l)` (Sonnet, Cat B)

**Deliverable:** extract the hyperelliptic curve `C/GF(2^l)` (the GHS image) from the descent algebra,
and verify it as a frozen `HyperellipticCurve`. Lower-fidelity sketch (crisp after C-DescentAlgebra
freezes):
- **Curve extraction** (`rho/src/ghs/curve.rs`): from the Artin–Schreier / Weil-restriction data, extract
  the hyperelliptic curve `C: y² + h(x)y = f(x)` over GF(2^l) — populate a frozen
  `HyperellipticCurve<L>` (its `poly`/`h_coeffs`/`f_coeffs`). The extracted curve is the **imaginary
  model** (deg f = 2g+1, odd `m/l`).
- **Genus ↔ extension-degree relation** (`rho/src/ghs/curve.rs`): the genus `g` of the GHS curve is
  determined by the extension degree `m/l` (and the curve's structure); record/verify
  `g = HyperellipticCurve::genus()` matches the expected value from the descent.
- **Imaginary-model validity** (`rho/src/ghs/curve.rs`): confirm the extracted curve is a *valid* frozen
  `HyperellipticCurve` (the C-HyperCurve invariants hold — `is_on_curve` for sample points, the model is
  imaginary).

Consumes C-DescentAlgebra (frozen E.H.2), C-HyperCurve (frozen E.I, read — the extracted curve *is* a
`HyperellipticCurve`), C-Subfield (read). **Freezes C-GHSCurve.**

**KAT** (`rho/tests/ghs_kat.rs`, extended): the **extracted curve is a valid `HyperellipticCurve`** (the
C-HyperCurve `is_on_curve` / model checks pass); the **genus matches** the expected value from `m/l`; the
curve is the **imaginary model** (deg f = 2g+1). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The extracted curve must satisfy the frozen C-HyperCurve contract** —
it is consumed by E.H.4/E.H.5 and E.K as a `HyperellipticCurve`, so the extraction must produce
imaginary-model `h_coeffs`/`f_coeffs` the frozen `genus()`/`is_valid` accept. (2) **The genus grows with
`m/l`** — at toy `m/l` the genus is small (2–4); the principle-4 annotation records that the genus and
field are toy, GHS is crypto-scale-correct. (3) **Even `m/l` would break the model** — if the extraction
yields deg f = 2g+2 (real/split), the frozen C-HyperCurve does not handle it (an additive amend); the
fixture's odd `m/l` (E.H.1) keeps it imaginary. (4) **The algebra↔curve seam check** — if the extraction
is a thin wrapper (no genuine seam over E.H.2), this is the loud signal to surface the E.H.2/E.H.3 merge.

**Deferred:** the transfer map (E.H.4); the reduction (E.H.5); the Jacobian DLP solver / Semaev
(E.K/E.J); the MATHEMATICS chapter (T.E).

### E.H.4 — Transfer map E(GF(2^m)) → Jac(C)(GF(2^l)) (Sonnet, Cat B)

**Deliverable:** the transfer map carrying a point on the binary elliptic curve to a reduced Mumford
divisor on the descended Jacobian, and its homomorphism property. Lower-fidelity sketch:
- **The point → divisor map** (`rho/src/ghs/transfer.rs`): `P ∈ E(GF(2^m)) ↦ D_P ∈ Jac(C)(GF(2^l))` —
  the GHS conorm/transfer map, built using the C-Subfield relative machinery and the frozen
  `divisor_from_points` / `compose`. Maps the base point `G` and a sample point.
- **The homomorphism property** (`rho/src/ghs/transfer.rs`): the transfer is a group homomorphism —
  `D_{P+Q} = D_P + D_Q` (where `P+Q` is binary-curve point addition and `D_P + D_Q` is Cantor compose).
  This is the property that makes the descent preserve discrete logs.

Consumes C-GHSCurve (frozen E.H.3), C-Jacobian (frozen E.I, read — `compose`/`add`/`divisor_from_points`/
`scalar_mul`), C-BinaryCurve (frozen E.G, read — the source point type `BinaryAffinePoint`). **Freezes
C-DescentMap.**

**KAT** (`rho/tests/ghs_kat.rs`, extended): the **homomorphism** `D_{P+Q} = D_P + D_Q` (transfer the
base point `G`, a sample `P`, and `G+P`; check `D_{G+P} = compose(D_G, D_P)` via Cantor); the **identity
maps to identity** (`D_∞ = [1,0]`, the zero divisor); transferred divisors are **valid reduced divisors**
(C-HyperCurve invariant holds). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **The homomorphism is the descent's whole point** — if `D_{P+Q} ≠ D_P +
D_Q`, the transfer does not preserve discrete logs and the descent is wrong; the homomorphism KAT is the
decisive guard (it is the E.H.4 analogue of E.I's group-axiom KATs). (2) **The transfer consumes the
frozen Cantor group law unchanged** — `D_P + D_Q` is `cantor::compose` + `reduce`; E.H builds the *map*,
not a new group law (rigidity guard — do not re-derive Cantor). (3) **The identity must map to the
identity** — `∞ ↦ [1,0]`; a transfer that mishandles the point at infinity breaks the homomorphism at the
identity. (4) **The map uses C-Subfield's relative trace/norm** (the GHS conorm), not the absolute trace.

**Deferred:** the reduction (E.H.5); the Jacobian DLP solver / Semaev (E.K/E.J); the MATHEMATICS chapter
(T.E).

### E.H.5 ◆ — GHS/Weil descent: ECDLP → Jacobian-DLP reduction + sub-track close (Sonnet, Cat I, `@architect`)

**Deliverable:** the reduction wiring that turns an ECDLP instance into a Jacobian-DLP instance, the
logarithm-preservation verification, and the sub-track close. Lower-fidelity sketch (crisp after
C-DescentMap freezes):
- **The reduction** (`rho/src/ghs/reduce.rs`): `ghs_descend((E, g, h)) → (C, D_g, D_h)` — given an ECDLP
  instance (find `k` with `h = k·g` on `E`), produce the Jacobian-DLP instance (`D_g`, `D_h` on `Jac(C)`)
  via the E.H.4 transfer map, such that `log_g h = log_{D_g} D_h`. Mirrors the `rho::ssa` reduction idiom
  (verify precondition → transfer → produce the relocated problem). **E.H produces the relocated problem;
  E.K solves it** (transfer-only — the scope boundary).
- **Logarithm preservation** (`rho/src/ghs/reduce.rs` + KAT): the decisive correctness property — for a
  known small `k`, `D_h = k·D_g` (via the frozen Cantor `scalar_mul`), confirming the transfer preserves
  the discrete log.
- **Sub-track-close KAT suite** (`rho/tests/ghs_kat.rs`, extended): logarithm preservation, the `#Jac` /
  genus checks, and (optional) the PARI `hyperellcharpoly` cross-check on the descended curve's order.

Consumes C-DescentMap (frozen E.H.4), C-GHSCurve (frozen E.H.3, read), C-Jacobian (frozen E.I, read —
`scalar_mul` for log-preservation), the `rho::ssa` reduction idiom (read). **Freezes C-GHSDescent.**

**KAT (primary correctness signal):** over the toy composite-field fixture: **logarithm preservation**
(`D_h = k·D_g` for a known small `k`, via `cantor::scalar_mul` — the decisive transfer-correctness
signal); the **`#Jac` / genus relationship** (the descended Jacobian's order relates to `#E(GF(2^m))`
via the descent; genus matches `m/l`); every transferred divisor is a **valid reduced divisor**. Optional
PARI cross-check (`hyperellcharpoly` for `#Jac` / the L-polynomial of the descended curve)
`#[ignore]`-gated (`#[ignore = "PARI not installed; run manually when available"]`). **Verify gate:**
`cargo test --workspace` green; existing rho/gnfs/shared KATs unchanged.

**Subtlety (load-bearing):** (1) **Transfer-only — NO Jacobian DLP solver here** — E.H produces
`(C, D_g, D_h)` and verifies `D_h = k·D_g`; it does NOT solve for `k` via index calculus (that is E.K,
consuming E.J's Semaev polynomials). A `@build` agent implementing relation collection / a divisor factor
base / linear algebra over `Z/ℓZ` is defocus (the central scope boundary). (2) **Logarithm preservation
is the correctness signal** — the descent is correct iff `log_g h = log_{D_g} D_h`; the `D_h = k·D_g`
KAT (known `k`) is the green-path guard, the PARI `#Jac` oracle the optional sidecar. (3) **This is the
E.H ◆ boundary** — re-read the Purpose intent and verify the descent is complete (subfield substrate +
descent algebra + curve extraction + transfer map + reduction all present and cross-checked) and
**E.K-ready** (C-GHSDescent exposes what E.K's index calculus consumes — the descended `Jac(C)/GF(2^l)`,
the transferred DLP instance `(D_g, D_h)`, the genus, the subfield), and that E.H stayed solver-free /
Semaev-free / chapter-free. (4) **No Semaev polynomials and no index calculus** — E.J builds Semaev; E.K
the index calculus; E.H the transfer. (5) **No MATHEMATICS chapter** — the GHS textbook content is T.E,
paired with E.W at the *Track-E* ◆; E.H.5 writes at most a PEDAGOGY code-tour delta.

**`@architect` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.H.5 ◆
to confirm: (1) the GHS/Weil descent is complete and composes (subfield substrate → descent algebra →
curve extraction → transfer map → reduction all present and cross-checked — the trace/norm identities,
the homomorphism `D_{P+Q}=D_P+D_Q`, and logarithm preservation `D_h=k·D_g` all pass); (2) C-GHSDescent
exposes what E.K descends into (the descended `Jac(C)/GF(2^l)`, the transferred DLP instance, the genus,
the subfield infrastructure) so E.K can build the index-calculus solver without amending the descent
surface — the substrate-readiness defense; (3) E.H stayed in scope — no Jacobian DLP solver / index
calculus (E.K), no Semaev polynomials (E.J), no MATHEMATICS chapter (T.E), the descent is a transfer
verified by logarithm-preservation, not a solve; (4) the principle-4 boundary (toy composite `m`, genus,
field sizes; GHS is crypto-scale-correct over composite binary fields) is recorded, not silently
presented as crypto-scale; (5) **the imaginary-model / `Uint<1>`-fixture / algebra-curve-seam
resolutions** — confirm the descended curve stayed imaginary (no C-HyperCurve amend), the fixture stayed
in `Uint<1>` (no C-BinaryCurve widening — or, if widened, that it was a surfaced discovery not a silent
patch), and the E.H.2/E.H.3 split held (or was merged via surfaced additive-reshard). **Also: reconcile
the outstanding static-frame ROADMAP debt** carried from the E.I ◆ — (a) the Progress table is stale by
**three** completed sub-tracks (E.F, E.G, E.I; table still shows "Done ~13 (E.A–E.E)"); (b) the Remaining
table lists the now-complete E.F/E.G/E.I and the now-complete-or-reordered rows; (c) **the E.I-before-E.H
sequencing correction** (the Remaining table listed E.H before E.I — dependency-inverted; E.I shipped
first and E.H now follows it) should be recorded; and (d) **strike E.H** from Remaining on completion.
One-shot findings; does not implement. Held at **Opus** per the header (juncture-tier — descent-algebra
cost-of-wrong on the hardest attack).

---

## Cross-session contracts

E.H **freezes five** contracts (C-Subfield at E.H.1, C-DescentAlgebra at E.H.2, C-GHSCurve at E.H.3,
C-DescentMap at E.H.4, C-GHSDescent at E.H.5) and **amends no prior frozen contract** (C-Jacobian /
C-HyperCurve / C-GF2mPoly / C-F2m / C-BinaryCurve / C-Koblitz / the p-adic / SSA / prime-field surfaces
are all consumed-or-untouched). The only `shared/gf2m` change is the **additive `subfield` module** (a
new public module, not a trait amendment).

### C-Subfield — subfield substrate GF(2^l) ⊂ GF(2^m): embedding + relative trace/norm + Frobenius-by-subfield (compiler- + test-enforced) — *to be frozen at E.H.1*

**Defined in:** E.H.1 (`shared/gf2m/src/subfield.rs`). **Consumed by:** E.H.2 (the Weil restriction is
relative to GF(2^l)), E.H.4 (the transfer map's conorm uses the relative trace/norm); **downstream:
E.K** (index calculus over the subfield GF(2^l) — the relative trace/norm and subfield basis). Compiler-
+ test-enforced. Exposes: `embed`/`restrict` (GF(2^l) ↔ GF(2^m), `l | m`); `relative_trace`
(`Tr_{m/l}(a) = Σ_{i=0}^{m/l−1} a^(2^(il))`, lands in GF(2^l)); `relative_norm` (`N_{m/l}`,
multiplicative, lands in GF(2^l)); `frobenius_subfield_orbit` (length `m/l`); `is_in_subfield(a, l)`
(`a^(2^l) == a`); **over-specified** subfield basis (carried for E.K if confidence is reasonable). All
threaded per-call (`poly: &Uint<L>`, the C-F2m idiom). **Char-2 invariants:** the relative trace/norm
iterate the `l`-th Frobenius power (steps of `l`, NOT the absolute-trace steps of 1); the subfield is the
Frobenius fixed field; `l | m` is mandatory. *Exact representation (whether the subfield basis ships now
or defers to E.K) ratified at the E.H.5 ◆.*

### C-DescentAlgebra — Artin–Schreier / function-field Weil-restriction algebra (compiler- + test-enforced) — *to be frozen at E.H.2*

**Defined in:** E.H.2 (`rho/src/ghs/descent.rs` + `rho/src/ghs/mod.rs`). **Consumed by:** E.H.3 (the curve
extraction reads the descent algebra). Compiler- + test-enforced. Exposes: the `GhsError` enum; the toy
binary-curve fixture + precondition verifier (`l | m`); the Artin–Schreier extension (char-2 `y²+y=f`);
the Weil restriction of scalars `Res_{GF(2^m)/GF(2^l)}` (consuming C-Subfield + C-GF2mPoly).
**Char-2 invariants:** the separable degree-2 extension is Artin–Schreier (`y²+y=f`), NOT Kummer
(`y²=f`, inseparable in char 2); the Weil restriction lowers the field and raises the dimension to `m/l`.
*Exact algebra↔curve seam (whether E.H.2/E.H.3 stay split or merge — depends on whether the curve
extraction is a thin wrapper) ratified at the E.H.5 ◆; the algebra surface E.H.3 consumes is frozen
here.*

### C-GHSCurve — the extracted GHS hyperelliptic curve `C/GF(2^l)` (compiler- + test-enforced) — *to be frozen at E.H.3*

**Defined in:** E.H.3 (`rho/src/ghs/curve.rs`). **Consumed by:** E.H.4 (the transfer map lands on
`Jac(C)`), E.H.5 (the reduction's DLP instance lives on `Jac(C)`); **downstream: E.K** (index calculus on
`Jac(C)/GF(2^l)`). Compiler- + test-enforced. Exposes: the extracted `HyperellipticCurve<L>` over GF(2^l)
(a frozen C-HyperCurve instance — populated `poly`/`h_coeffs`/`f_coeffs`); the genus↔extension-degree
relation (`g` from `m/l`); imaginary-model validity. **Invariant:** the extracted curve satisfies the
frozen C-HyperCurve contract (imaginary model, deg f = 2g+1, the `is_valid`/`is_on_curve` checks pass) —
**E.H amends no C-HyperCurve.** **Toy composite `m`, toy genus** (principle-4 boundary). *Exact
genus-vs-`m/l` formula ratified at the E.H.5 ◆.*

### C-DescentMap — the transfer map E(GF(2^m)) → Jac(C)(GF(2^l)) (compiler- + test-enforced) — *to be frozen at E.H.4*

**Defined in:** E.H.4 (`rho/src/ghs/transfer.rs`). **Consumed by:** E.H.5 (the reduction transfers the
ECDLP instance via this map). Compiler- + test-enforced. Exposes: `transfer(P: BinaryAffinePoint) →
MumfordDivisor` (the GHS conorm, consuming C-Subfield + the frozen `divisor_from_points`/`compose`); the
homomorphism property. **Invariants:** the transfer is a **group homomorphism** (`D_{P+Q} = D_P + D_Q`
via Cantor compose); the identity maps to the identity (`∞ ↦ [1,0]`); transferred divisors are valid
reduced divisors. **The transfer consumes the frozen Cantor group law unchanged** (no group-law amend).
*Exact map representation (conorm vs explicit divisor construction) ratified at the E.H.5 ◆.*

### C-GHSDescent — the ECDLP → Jacobian-DLP reduction (compiler- + test-enforced) — *to be frozen at E.H.5 ◆*

**Defined in:** E.H.5 (`rho/src/ghs/reduce.rs`). **Consumed by:** **E.K** (the index-calculus DLP solver
consumes the relocated problem — the highest-stakes consumer), **E.W** (the cross-attack benchmark
table). Compiler- + test-enforced. Exposes the reduction: `ghs_descend((E, g, h)) → (C, D_g, D_h)` such
that `log_g h = log_{D_g} D_h`. **The frozen invariant:** **logarithm preservation** — for known `k`,
`D_h = k·D_g` (via the frozen Cantor `scalar_mul`); the descended `#Jac` / genus relate to
`#E(GF(2^m))` via the descent. **E.H freezes the transfer; the index-calculus DLP solve on `Jac(C)` is
E.K** (the scope boundary). **The descent is a transfer verified by logarithm-preservation, NOT a solve.**
*Exact reduction signature (error cases for non-descendable curves) ratified at the ◆.*

### Frozen contracts read by E.H (consumed, not amended)

- **C-Jacobian / C-HyperCurve / C-GF2mPoly (frozen E.I surface)** — `compose`/`reduce`/`add`/`negate`/
  `scalar_mul`/identity (the Jacobian group law), `HyperellipticCurve`/`MumfordDivisor`/`divisor_from_points`/
  `genus`/`is_valid` (the curve + divisor), `Poly<F,L>` ring + `resultant`/`mod_inverse` (the polynomial
  ring). Consumed by the descent algebra (polynomial work), the curve extraction (the extracted curve is
  a `HyperellipticCurve`), the transfer map (divisors via Cantor), and the reduction (`scalar_mul` for
  log-preservation). **Unchanged — E.H amends no E.I contract.**
- **C-BinaryCurve / C-Koblitz (frozen E.G surface)** — `BinaryCurve` + `BinaryAffinePoint` + point
  addition (`add`/`scalar_mul`) — the ECDLP *source*. **Read for the transfer-map source point type; NOT
  amended.** *(Open: `BinaryCurve` is hardcoded `Uint<1>`; E.H targets a composite `m` that fits — if no
  in-`Uint<1>` fixture works, generalizing to `L>1` is a surfaced discovery at the ◆, never silent.)*
- **C-F2m (frozen surface)** — `mul`/`square`/`frobenius`/`pow`/`trace` (the absolute trace, distinct
  from C-Subfield's relative trace) consumed by the subfield substrate and the descent algebra, threaded
  per-call. Read. **Unchanged.**
- **`rho::ssa` reduction idiom** — the `SsaError` enum + fixture + verify→lift→reduce shape (the
  reduction-attack template E.H mirrors). Read for the pattern; untouched.

### Workspace edges (no new edge, no new crate)

- **No new edge.** `rho` already depends on `gnfs`, `shared-gf2m`, `shared-field`, `shared-padic`,
  `shared-numfield`, `shared-numth`, `shared-bigint`. The subfield substrate is an **additive module in
  `shared/gf2m`** (`shared/gf2m/src/subfield.rs`); the descent is a new module in the existing `rho`
  crate (`rho::ghs`). No `Cargo.toml` changes; `cargo check --workspace` stays green with no cycle risk.
  *(Asymmetry confirmed by the survey: `gnfs` does NOT depend on `shared-gf2m`, so the Jacobian DLP solver
  cannot live in `gnfs` and reuse NFS-DL — reinforcing the E.H/E.K scope boundary. If E.H found it must
  change a frozen trait surface — it should not, it only adds the `subfield` module and a new `ghs`
  module — that would be a discovery surfaced at the ◆, never a silent patch.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The E.H.5 ◆ `@architect` confirmation is not a separate ledger row
(a paged fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.H.1 | Subfield substrate GF(2^l) ⊂ GF(2^m) | done | d37e2e9 | C-Subfield (frozen) |
| E.H.2 | Artin–Schreier / function-field Weil-restriction algebra | done | 7a4a72f | C-DescentAlgebra (frozen) |
| E.H.3 | GHS hyperelliptic-curve extraction C/GF(2^l) | done | 2cc5205 | C-GHSCurve (frozen) |
| E.H.4 | Transfer map E(GF(2^m)) → Jac(C)(GF(2^l)) | pending | — | C-DescentMap |
| E.H.5 ◆ | GHS/Weil descent: ECDLP → Jacobian-DLP reduction + close | pending | — | C-GHSDescent |

Contracts frozen before this sub-track: the GF(2^m) field surface (C-F2m/C-F2mOpt — read by E.H,
unchanged), the binary-curve surface (C-BinaryCurve/C-BinaryRho/C-Koblitz from E.G — read as the ECDLP
source, unchanged), the hyperelliptic-Jacobian surface (C-GF2mPoly/C-HyperCurve/C-Jacobian from E.I —
read as the descent target, unchanged), the p-adic surface (C-Padic/C-Hensel/C-PadicLog), the SSA surface
(C-AnomalousLift/C-SSA — read as the reduction-attack idiom), the prime-field rho curve + ECDLP surface,
`Fp<4>`. This sub-track **freezes five new contracts** (C-Subfield, C-DescentAlgebra, C-GHSCurve,
C-DescentMap, C-GHSDescent), serving the downstream **E.K** (Gaudry–Diem–Joux–Vitse index calculus —
solves the relocated DLP on C-GHSDescent's Jacobian, consuming E.J's Semaev polynomials), **E.W**
(cross-attack benchmarks), and completing the **ECDLP-transfer** half of the small-characteristic
index-calculus attack (E.H transfers; E.J+E.K solve).

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.H builds the GHS/Weil descent on the frozen E.I Jacobian + a new subfield substrate —
  internal-continue (confirmed by survey).** All greenfield; no existing GHS/descent/subfield/
  Weil-restriction code. A discovery that the descent needs a Jacobian or polynomial operation the frozen
  E.I surface lacks is an **additive amend** (C-Jacobian/C-GF2mPoly gains the operation) surfaced at the
  ◆ — not a silent patch. A discovery that the descent needs a *field* primitive C-F2m lacks is an
  additive C-F2m amend, surfaced at the ◆.

- **The descent is a TRANSFER, not a solve — implementing the Jacobian DLP solver is a defocus failure
  (the central scope boundary).** E.H produces the relocated problem `(C, D_g, D_h)` and verifies
  `log_g h = log_{D_g} D_h`; the index-calculus *solve* (relation collection, the divisor factor base,
  linear algebra over `Z/ℓZ`) is **E.K**, consuming **E.J**'s Semaev polynomials. A `@build` agent
  implementing index calculus or Semaev polynomials in E.H is defocus. **Internal-continue → corrected**
  (the logarithm-preservation KAT is the green-path terminus; the solve is delegated).

- **The relative trace/norm iterate the `l`-th Frobenius power, NOT the absolute trace — the subfield
  trap.** `Tr_{m/l}(a) = Σ a^(2^(il))` (steps of `l`, lands in GF(2^l)), distinct from the absolute
  `trace` (steps of 1, lands in GF(2)) already on C-F2m. A `@build` agent reusing the absolute trace
  writes the wrong Weil restriction; the "trace lands in GF(2^l)" KAT is the loud signal.
  **Internal-continue → corrected.**

- **The char-2 separable degree-2 extension is Artin–Schreier `y²+y=f`, NOT Kummer `y²=f` — the char-2
  extension trap.** In char 2 `y²=f` is inseparable (no GHS curve); the descent's extension is
  Artin–Schreier (`℘(y)=y²+y`). A `@build` agent porting an odd-char Kummer construction writes a
  degenerate extension. The descent-algebra round-trip KAT is the guard. **Internal-continue →
  corrected.**

- **The transfer must be a group homomorphism — a non-homomorphic map breaks log-preservation (the
  central correctness guard).** `D_{P+Q} = D_P + D_Q` (binary-curve addition ↦ Cantor compose) is the
  property that makes the descent preserve discrete logs. A transfer that fails this relocates the
  *points* but not the *problem*. The `D_{P+Q}=D_P+D_Q` and `D_h=k·D_g` KATs are the defense.
  **Internal-continue → corrected.**

- **The descended curve is the imaginary model when `m/l` is odd — resolves the E.I.3 ◆ conditional;
  even `m/l` would need a C-HyperCurve amend.** The frozen C-HyperCurve handles only the imaginary/
  ramified model (deg f = 2g+1). GHS with odd `m/l` lands the imaginary model (compatible — no amend).
  The fixture (E.H.1) is chosen with odd `m/l`. If E.H ever needs even `m/l` (real/split model), that is
  an **additive C-HyperCurve amend surfaced at the ◆** — not a silent patch. **Internal-continue (no
  amend for the standard target).**

- **`BinaryCurve` is hardcoded `Uint<1>` — E.H targets a composite `m` that fits; widening is a surfaced
  discovery, not silent growth.** GHS's interesting cases are over composite fields, but the toy
  `BinaryCurve` is `L=1`. E.H chooses a toy composite `m ≤ 8` with odd `m/l` that fits `Uint<1>` (E.H.1's
  fixture act). **If no in-`Uint<1>` fixture gives a non-degenerate descent, generalizing `BinaryCurve`
  to `L>1` is a surfaced discovery at the ◆** (the established C1-widening discipline — a deliberate,
  boundary-respecting amend, never spontaneous in-flight scope growth). **Internal-continue (target
  in-`Uint<1>`); additive-reshard only if the fixture forces widening.**

- **The algebra↔curve seam (E.H.2↔E.H.3) may be artificial — surface a merge if the extraction is a thin
  wrapper.** The 5-vs-4 sizing splits the GHS construction at the algebra↔curve seam (buying an early
  C-DescentAlgebra freeze). **If E.H.2's algebra and E.H.3's curve extraction prove tightly coupled (the
  extraction is a thin wrapper, no genuine reusable function-field layer), the split is artificial and
  E.H.2/E.H.3 should merge** — surfaced as an additive-reshard at the ◆ (or by E.H.2 once the algebra
  shape is concrete), never a silent merge. **Additive-reshard if the seam proves false.**

- **No Jacobian DLP solver / index calculus / Semaev polynomials in E.H (defocus / scope clarity).** The
  DLP *attack* on the descended Jacobian (index calculus, relation collection, linear algebra over
  `Z/ℓZ`) is **E.K**; the Semaev summation polynomials are **E.J**. E.H exposes the transfer; the attack
  consumes it. A `@build` agent implementing any of these in E.H is defocus.

- **No MATHEMATICS.md chapter in E.H (defocus / scope clarity).** The GHS / Weil-descent textbook content
  is **T.E, paired with E.W at the Track-E ◆** (ROADMAP per-track-chapter pairing), not at the E.H
  sub-track ◆. E.H.5 writes at most a PEDAGOGY code-tour delta.

- **No oracle dependency for correctness (principle-3 / E.D…E.I-consistent).** The descent's correctness
  is checkable on the green path (the trace/norm identities, the transfer homomorphism, logarithm
  preservation `D_h=k·D_g` via the frozen Cantor `scalar_mul`); a PARI `hyperellcharpoly` `#Jac`
  cross-check is an **optional `#[ignore]` sidecar** (the established `#[ignore = "PARI not installed;
  run manually when available"]` pattern). E.H introduces no new live oracle. *(Lever-5 note: this signal
  is weaker than E.I's exactly-self-checking group axioms — log-preservation leans on a correct genus/
  order relationship and, for the order cross-check, the PARI oracle — which is why juncture-tier stays
  opus.)*

- **Toy composite `m` + field/genus sizes only (scope clarity).** E.H fixes a small composite `m ≤ 8`,
  `l | m` (odd `m/l`), and the genus follows the extension degree. The toy sizes are a principle-4
  boundary — GHS is crypto-scale-correct over composite binary fields; only the *parameters* are toy.
  Presenting any as crypto-scale is a documentation defect (internal-continue → corrected).

- **The subfield substrate's home is `shared/gf2m::subfield` (module-placement call, ratified at the ◆).**
  The substrate is field-substrate-shaped and reusable (E.K may consume it), so `shared/gf2m` is the
  principled home over `rho`. If a reviewer prefers `rho::ghs`, that is a ◆ ratification decision, not a
  blocker (no edge consequence either way — `rho` already depends on `shared-gf2m`).

- **Static-frame ROADMAP debt (reconcile at the E.H ◆, does NOT block E.H) — carried + compounded from
  the E.I ◆.** The ROADMAP Progress subsection is stale by **three** completed sub-tracks (E.F, E.G, E.I;
  table shows "Done ~13 (E.A–E.E)"); the Remaining table lists the now-complete E.F/E.G/E.I; and the
  Remaining table listed **E.H before E.I** (dependency-inverted — E.I shipped first, E.H now follows).
  The E.I ◆ digest recorded this as owed but did not write it into the ROADMAP. The E.H ◆ should: update
  Progress (Track E Done → E.A–E.I, ~22), strike E.F/E.G/E.I from Remaining, record the E.I-before-E.H
  correction, and strike E.H on completion. Not an implementation concern.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.H, "*GHS/Weil descent … Transfer ECDLP on a binary curve to DLP on
  a hyperelliptic Jacobian over a subfield. First session is Opus-tier — the descent machinery is the
  most mathematically intricate single attack in the project.*"; the design statement's principles 1 + 3
  + 4; the "On scale" mathematical-dimension framing — the descended curve's subfield GF(2^l) and genus
  are *mathematical-dimension scale*, orthogonal to operational scale) and this PLAN before any session.
  **NOTE: the ROADMAP Progress / Remaining tables are stale (E.F, E.G, E.I done; E.F/E.G/E.I still listed)
  AND listed E.H before E.I (dependency-inverted — E.I shipped first); reconcile at the E.H ◆.**
- Read the **templates to mirror**: `rho/src/ssa/mod.rs` + `rho/src/ssa/reduce.rs` (the reduction-attack
  idiom — `SsaError` enum + toy fixture + verify→lift→reduce shape — E.H's `ghs` module mirrors this);
  `rho/src/hyperelliptic/{mod,cantor}.rs` (the **frozen E.I Jacobian** E.H descends onto — read for the
  `HyperellipticCurve`/`MumfordDivisor`/`compose`/`scalar_mul`/`divisor_from_points` surface, NOT to
  amend); `rho/src/binary_curve/mod.rs` (the **frozen E.G binary curve** — the ECDLP source point type
  `BinaryAffinePoint` + point addition, read NOT amend); `shared/gf2m/src/convert.rs` (the
  `frobenius_orbit` + per-call `poly` idiom the subfield substrate extends to *relative* orbits);
  `shared/gf2m/src/poly.rs` (the frozen C-GF2mPoly ring the descent algebra consumes — `resultant`/
  `mod_inverse` shipped); `rho/tests/{ssa_kat,hyperelliptic_kat,mov_kat}.rs` (the reduction-KAT + PARI
  `#[ignore]` oracle idioms E.H.5 mirrors).
- **Register:** E.H is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module `shared/gf2m/src/subfield.rs` (the subfield substrate), new modules
  `rho/src/ghs/{mod,descent,curve,transfer,reduce}.rs` (the descent), and new KATs in
  `shared/gf2m/tests/gf2m_subfield_kat.rs` + `rho/tests/ghs_kat.rs`.
- **Tier routing:** **E.H.1 is Opus `@build`** (the ROADMAP Opus-flagged substrate-design session —
  C-Subfield has downstream consumption by E.H.2–5 and potentially E.K, and the subfield-tower design is
  the foundation the whole descent stands on). **E.H.2–E.H.5 are Sonnet `@build`.** E.H.5 carries the **◆
  `@architect` juncture** (page `@plan-juncture`) ratifying C-Subfield/C-DescentAlgebra/C-GHSCurve/
  C-DescentMap/C-GHSDescent and confirming E.K-readiness before the sub-track closes. juncture-tier
  (header) is **opus** — held by levers 2 (irreducible complexity of "the most mathematically intricate
  single attack") + 3 (descent-algebra design-error cost, the most expensive retrofit in the project);
  lever 5 is too weak to license `sonnet` (log-preservation is checkable but not exactly self-checking —
  it leans on a correct genus/order relationship and the PARI `#Jac` oracle).
- **Invariants to preserve:** **The descent is a TRANSFER, not a solve** (no Jacobian DLP solver / index
  calculus / Semaev polynomials — those are E.K/E.J; the log-preservation KAT is the terminus).
  **The transfer is a group homomorphism** (`D_{P+Q}=D_P+D_Q` via Cantor; the homomorphism + `D_h=k·D_g`
  KATs are the guard). **The relative trace/norm iterate the `l`-th Frobenius power** (NOT the absolute
  trace; the "lands in GF(2^l)" KAT is the guard). **The char-2 extension is Artin–Schreier `y²+y=f`**
  (NOT Kummer `y²=f`). **The descended curve is the imaginary model** (odd `m/l`; consumes the frozen
  C-HyperCurve unchanged — no amend for the standard target). **E.H consumes the frozen E.I Jacobian + E.G
  binary curve + C-F2m unchanged** (the only `shared/gf2m` change is the additive `subfield` module).
  **`BinaryCurve` stays `Uint<1>`** (target a composite `m` that fits; widening is a surfaced discovery).
  **No MATHEMATICS chapter** (T.E at the Track-E ◆). Toy composite `m` + toy field/genus sizes only; no
  new live oracle.
- **PARI remains a dev-only `#[ignore]` oracle** — an optional `hyperellcharpoly` `#Jac` cross-check on
  the descended curve follows the established `#[test] #[ignore = "PARI not installed; run manually when
  available"]` pattern; never on the green path.
- **No new edge, no new crate (load-bearing for E.H).** `rho` already depends on `gnfs`/`shared-gf2m`/
  `shared-field`/`shared-padic`; the subfield substrate is an additive `shared/gf2m::subfield` module and
  the descent is a new `rho::ghs` module. `cargo check --workspace` stays green with no cycle risk. If E.H
  finds it must change a frozen trait surface (it should not — it only adds the `subfield` and `ghs`
  modules), that is a discovery surfaced at the ◆.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  subfield-tower substrate + a function-field/Artin–Schreier descent algebra + a curve extraction + a
  transfer map + a reduction) is **new to this project** (the prior reduction attacks — MOV, SSA — were
  prime-field; the GHS function-field/Weil-restriction shape over a binary subfield tower has no in-repo
  precedent, and it is "the most mathematically intricate single attack"). Per the unproven-shard-pattern
  guidance, halt at each boundary for a human glance until the pattern proves out. *(Tradeoff vs
  autonomous: `halt-at-boundaries` trades velocity for a per-boundary check on a novel, high-stakes
  pattern — the subfield substrate (E.H.1, Opus) and the descent algebra (E.H.2) are the design-crux
  freezes E.K consumes, and a wrong shape there is the most expensive retrofit in the project. If E.H.1
  and E.H.2 land cleanly and their KATs confirm the substrate + algebra shape, fall back to autonomous
  for E.H.3–E.H.5. The algebra↔curve seam uncertainty (E.H.2↔E.H.3) is itself a reason to halt at the
  E.H.2 boundary — that is where a merge-back would be surfaced.)*
