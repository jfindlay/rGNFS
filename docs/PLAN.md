<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E continues (E.B — Pairing arithmetic: Weil, Tate)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error), the binding
constraint.** Unlike E.A — where the default *opted down* and an operator election held it up — E.B
holds at Opus on the law itself: the E.B substrate (the `F_{p^k}` extension field and the
`E(F_{p^k})` point representation) **bounds E.C**, the MOV/Frey–Rück bridge that is *the* cross-track
pedagogical climax (it calls the real NFS-DL solver, C2). A wrong representation freeze here is the
**highest-cost-of-wrong interface in Track E so far** — E.C must consume whatever E.B freezes. Lever 5
is strong (bilinearity `e(aP,bQ) = e(P,Q)^{ab}` is a decisive, fast, self-checking KAT) and *would*
license an opt-down in isolation, but lever 3 dominates and is the binding constraint, so the ◆
juncture that ratifies C-FpExt / C-PairingCurve / C-Pairing runs at **Opus**. *(Scoped to the E.B.4 ◆
close. The roadmap-flagged Opus session is E.B.1 — the `F_{p^k}` substrate design; E.B.2–4 are Sonnet
against the frozen extension/point representation.)*

Last rewrite: **E.A ◆ boundary crossed** (E.A.1 substrate `054df65`, E.A.2 ◆ Pohlig–Hellman
`51fd477`; C-CompositeCurve, C-FactorOrder, C-Pohlig frozen; sub-track closed, Track-E attack scaffold
coherent — composite fixture `n=60=2²·3·5`, `factor_order`, `project_to_subgroup`,
`solve_ecdlp_composite`, 6 composite KATs). This plan continues **Phase δ (Algebraic ECDLP, Track E)**
with **E.B — Pairing arithmetic (Weil, Tate)**: the bilinear-pairing substrate on which the MOV/Frey–
Rück reduction (E.C) stands. Predecessor per ROADMAP: G.A (number-field machinery, *as conceptual
reference for divisor arithmetic — not as the char-p extension-field substrate; see the rigidity guard
below*).

---

## Purpose (design intent)

Per ROADMAP (Phase δ): "**E.B — Pairing arithmetic (Weil, Tate).** 3-4 sessions. Predecessor: G.A.
**First session is Opus-tier** — pairings are mathematically delicate and the divisor-arithmetic
representation choice bounds E.C. Subsequent Sonnet." E.B is the second Track-E attack-substrate and
the first to leave the prime field: where E.A reduced a composite-order DLP to prime subgroups by CRT,
E.B builds the machinery that lets a *different* structural escape exist — the **bilinear pairing**
`e: E[ℓ] × E[ℓ] → μ_ℓ ⊂ F_{p^k}*`, which transports the ECDLP into the multiplicative group of a
finite extension field, where index calculus / NFS-DL is subexponential. The
structure-based-escape-from-search through-line: *a low embedding degree is a structural weakness* —
the pairing is the homomorphism that converts an elliptic-curve discrete log into a finite-field one.
E.B builds the pairing; **E.C is the attack that exploits it** (calling NFS-DL via C2). E.B itself
ships no attack — it ships the bilinear map and proves it bilinear and non-degenerate.

The substrate survey established the shape precisely, and it is **larger than the roadmap sketch**
implies in one decisive respect:

1. **No extension-field arithmetic exists.** There is no `Fp2` / `FpK` / tower / polynomial-quotient
   field anywhere in the workspace — only the base prime field `Fp<L>` (`shared/field/src/lib.rs`,
   `FpNaive` / `FpMonty`). But Weil/Tate pairings **land in `F_{p^k}`** (the embedding field, μ_ℓ ⊂
   F_{p^k}*). So E.B must **build `F_{p^k}` arithmetic from scratch** — this is the substrate session
   the roadmap's "3-4 sessions, first Opus" folds in but does not decompose. E.B.1 is that session.
2. **The curve group law is `F: Fp<4>`-bound.** `Curve` stores its coefficients as `Uint<4>`
   (base-prime-field), and `double_jacobian` / `add_jacobian` / `add_mixed` / `scalar_mul`
   (`rho/src/curve/mod.rs`) all require `F: Fp<4>` — a **prime** field. Miller's algorithm must
   evaluate line functions at points of `E(F_{p^k})`, which the existing law **cannot represent**.
   This is exactly the "**divisor-arithmetic representation choice bounds E.C**" the roadmap flags, and
   the α-boundary discovery anticipated ("`rho::curve` was NOT lifted to `shared::curve`… Revisit at
   E.B.1 when pairings need divisor-arithmetic curves").
3. **The representation decision was adjudicated at shard time: option (B), the standalone pairing
   layer.** Rather than generalising the frozen `rho::curve` field bound (option A — cleanest
   long-term, but a destructive edit to a contract consumed by all of Track-rho + E.A), E.B builds a
   **separate** `E(F_{p^k})` arithmetic inside a new `rho/src/pairing/` module, **composing** the
   frozen curve params (read-only) and the new `F_{p^k}` field, amending nothing in `rho::curve`. This
   matches the E.A principle-3 discipline (compose the frozen substrate; the attack layer wraps it).
   The cost is some group-law duplication over the extension field; the win is a bounded blast radius.
   *(The E.B.4 ◆ juncture re-ratifies this against E.C's needs at execution time.)*
4. **No Miller / divisor / line-function machinery exists.** No rational-function-on-a-curve, line
   function, vertical function, or divisor type anywhere. E.B writes Miller's algorithm from scratch
   (E.B.3). The G.A `NumberField`/`NumberFieldElement` machinery is **char-0** number-field arithmetic
   — *not* the char-p `F_{p^k}` E.B needs (a `numfield` PEDAGOGY note claiming "E.D (pairings) uses
   `NumberField` for extension arithmetic" is a forward-looking mis-mapping: it conflates char-0
   number fields with char-p finite extensions, and names the wrong sub-track — see the rigidity
   guard, and the corrective work item folded into E.B.4). G.A is a *conceptual* reference for divisor
   arithmetic, not a code dependency here.

The work splits at the **field-substrate → curve-substrate → algorithm** seams, 4 sessions:

1. **E.B.1 — `F_{p^k}` extension-field arithmetic (Opus, Cat A).** A finite extension field over the
   base prime field, as a polynomial quotient `F_p[u]/(m(u))` for an irreducible modulus `m` of degree
   `k` (toy embedding degrees `k = 2`, possibly `k = 3`/`6`). Element = coefficient vector over
   `Fp<4>`; add/sub/mul (with reduction mod `m`)/inv (extended-Euclid or Frobenius-norm route)/pow/
   Frobenius/equality/`one`/`zero`. **Freezes C-FpExt** — the extension-field interface E.B.2–4 and
   E.C consume.

2. **E.B.2 — `E(F_{p^k})` point arithmetic + pairing-friendly fixture (Sonnet, Cat A).** A standalone
   short-Weierstrass group law over `F_{p^k}` (`PairingPoint`, add/double/scalar-mul/negate), composing
   the frozen `Curve` params (read-only) and C-FpExt; and a **pairing-friendly toy fixture**: a curve
   with a *small known embedding degree* `k` w.r.t. a small prime ℓ | #E(F_p), with the ℓ-torsion
   structure recorded (a base-field point `P ∈ E(F_p)[ℓ]` and a linearly-independent
   `Q ∈ E(F_{p^k})[ℓ]`). **Freezes C-PairingCurve** — the fixture + point representation E.B.3/E.B.4
   and E.C consume.

3. **E.B.3 — Miller's algorithm + Weil pairing (Sonnet, Cat B).** Miller's algorithm (the
   double-and-add accumulation of the line/vertical-function quotient `f_{ℓ,P}`), and the **Weil
   pairing** `w_ℓ(P,Q) = f_{ℓ,P}(Q) / f_{ℓ,Q}(P)` (the ratio form, no final exponentiation). KAT:
   **bilinearity** `w(aP,bQ) = w(P,Q)^{ab}` and **non-degeneracy** `w(P,Q) ≠ 1` for independent P,Q.

4. **E.B.4 ◆ — Tate / reduced-Tate pairing + Track-E pairing design note (Sonnet, Cat B, `@plan`).**
   The **Tate pairing** `t_ℓ(P,Q) = f_{ℓ,P}(Q)` followed by **final exponentiation**
   `^{(p^k−1)/ℓ}` (the reduced Tate pairing, a well-defined element of μ_ℓ), reusing the E.B.3 Miller
   loop; bilinearity KAT for Tate; the **Track-E pairing design-statement note** (principles 1/3/4 for
   the pairing substrate, and the explicit E.C-readiness check: is C-Pairing the right input to the MOV
   bridge?); and the **numfield PEDAGOGY correction** (the corrective work item, see below). **Freezes
   C-Pairing.** Crosses the **E.B ◆ boundary** (sub-track complete).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the MOV reduction itself —
that is **E.C**, not E.B; E.B ships the pairing and stops at proving it bilinear/non-degenerate. Or
implementing BN/BLS *cryptographic* pairing curves / optimal-ate / a full tower `F_{p^{12}}` — toy
embedding degree `k ≤ 6` suffices for the demonstration; pairing-based-crypto engineering is out of
scope) and **rigidity** (reaching for G.A's char-0 `NumberFieldElement` to represent `F_{p^k}` — wrong
characteristic; E.B builds a char-p extension field. Or amending the frozen `rho::curve` group law to
"accept extension fields" — option A was declined; E.B composes a *separate* `E(F_{p^k})` layer).

**Scoping discipline.** E.B builds pairings on **toy pairing-friendly curves** (small `p`, small
embedding degree `k`, small torsion prime ℓ) at demonstration fidelity (principle 4). It introduces
**no oracle dependency** for correctness (bilinearity self-checks; a PARI `ellweilpairing`/
`elltatepairing` cross-check is an *optional* `#[ignore]` sidecar, matching the established pattern).
The base-curve params are the **frozen `rho::curve`** read read-only; E.B adds the extension-field
and pairing layers, nothing inside `rho::curve` or `shared::field`. The `F_{p^k}` arithmetic is
**toy-embedding-degree-scoped** (`k ≤ 6`); a general tower-field construction for crypto-scale `k=12`
is a principle-4 annotation, not a work item.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from E.A; no `oracle-tests` feature exists — oracle KATs are `#[ignore]`-gated only).
`/run-plan` re-discovers at preflight. E.B **adds substantial code and KATs**, so the gate is a
**correctness gate**: the E.B.3 Weil-bilinearity KAT (`w(aP,bQ) = w(P,Q)^{ab}`) and the E.B.4 Tate
counterpart are the primary correctness signals — fast and decisive (lever 5). A wrong `F_{p^k}` mul/
inv, a Miller off-by-one, or a wrong final exponentiation breaks bilinearity directly.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.B.1 | `F_{p^k}` extension-field arithmetic (`F_p[u]/(m)` quotient, add/mul/inv/pow/Frobenius) | A | **Opus** | `shared::field` `Fp<4>` (frozen, read), `crypto-bigint` `Uint<4>` | `rho/src/pairing/mod.rs` (new: `pub mod fpext;`), `rho/src/pairing/fpext.rs` (new: `FpExt`, irreducible modulus), `rho/src/lib.rs` (`pub mod pairing;`) |
| E.B.2 | `E(F_{p^k})` point arithmetic + pairing-friendly fixture (embedding degree `k`, ℓ-torsion) | A | Sonnet | C-FpExt (frozen E.B.1), `rho::curve::Curve` params (frozen, read), `composite_toy`/`tiny_*` fixtures (template) | `rho/src/pairing/ecext.rs` (new: `PairingPoint`, group law over `FpExt`), `rho/src/pairing/test_curves.rs` (new: pairing-friendly fixture + ℓ-torsion P, Q), `rho/src/pairing/mod.rs` (`pub mod ecext; pub mod test_curves;`) |
| E.B.3 | Miller's algorithm + Weil pairing + bilinearity/non-degeneracy KAT | B | Sonnet | C-FpExt + C-PairingCurve (frozen E.B.1/E.B.2), existing ecdlp KAT (template) | `rho/src/pairing/miller.rs` (new: Miller loop, line/vertical functions), `rho/src/pairing/weil.rs` (new: `weil_pairing`), `rho/tests/pairing_kat.rs` (new: Weil bilinearity + non-degeneracy) |
| E.B.4 ◆ `@plan` | Tate/reduced-Tate pairing + final exponentiation + design note + numfield-doc correction | B | Sonnet | C-FpExt + C-PairingCurve + Miller (frozen E.B.1–3) | `rho/src/pairing/tate.rs` (new: `tate_pairing`, `reduced_tate`, final exp), `rho/tests/pairing_kat.rs` (Tate bilinearity), `rho/src/pairing/mod.rs` (design-note module docstring), `shared/numfield/docs/PEDAGOGY.md` (correct the E.D/E.B + char-0/char-p mis-mapping) |

**Sequencing notes.** Strictly serial: **E.B.1 → E.B.2 → E.B.3 → E.B.4.** E.B.1 lands the field the
whole sub-track stands on; E.B.2 builds the curve arithmetic over it and the fixture; E.B.3 writes
Miller + Weil over both; E.B.4 reuses E.B.3's Miller loop for Tate and closes the sub-track. The single
`@plan` marker sits on **E.B.4 ◆** — the Opus boundary juncture ratifying C-FpExt / C-PairingCurve /
C-Pairing against E.C's needs before the sub-track closes. E.B.1, though Opus-tier, carries **no**
`@plan` (it freezes a compiler-/test-checkable field interface; the C-FpExt design is re-ratified at
the E.B.4 ◆ alongside the others — an inline juncture would double the boundary cost on a 4-row shard).

**Why 4 sessions (top of the ROADMAP 3-4 band — the field substrate is its own session).** The split
is taken at the field→curve→algorithm seams:
- **One-line-commit-title corollary.** "`F_{p^k}` arithmetic", "`E(F_{p^k})` + fixture", "Miller +
  Weil", "Tate + final exp" are **four distinct commit titles** spanning two categories (A substrate ×2,
  B algorithm ×2).
- **Contract-sharp boundaries (legitimate, not LOC-driven).** E.B.1 **freezes** C-FpExt; E.B.2
  **consumes** it and **freezes** C-PairingCurve; E.B.3 **consumes** both; E.B.4 **consumes** all and
  **freezes** C-Pairing. Three real produce/consume seams.
- **Why the field is its own session (lever 1 + lever 3).** `F_{p^k}` arithmetic from scratch
  (quotient construction, irreducible modulus selection, extension inverse, Frobenius) is a full
  substrate unit, and the survey confirmed *nothing* exists to build on past the base prime field —
  ambient complexity (lever 1) is real, and the field interface is the deepest consumed surface (E.C
  reads `F_{p^k}` elements through C-FpExt). Merging it into E.B.2 risks a >400-LOC two-title session.
- **Irreducible units kept whole (lever 2).** Miller's algorithm + the Weil ratio is one coherent unit
  (E.B.3); the Tate pairing + final exponentiation is one coherent unit reusing the same Miller loop
  (E.B.4). Neither is fractured.

They are **not** further splittable: separating Miller's algorithm from the Weil pairing (a 5-session
option) would split an irreducible unit — Miller's loop has no standalone KAT-able contract except *as*
the pairing it computes (a Miller loop with no pairing deliverable has an undefined contract). Tate
genuinely reuses E.B.3's Miller, so it is the natural ◆-closing session, not a separate substrate.

---

## Session detail

E.B.1 is specified at near-full fidelity (finite-extension-field arithmetic is textbook, with the one
open design call — the tower-vs-direct-quotient construction and the embedding degrees the fixture
exercises — flagged for the E.B.1 design register and re-ratified at the ◆). E.B.2–4 are
lower-fidelity sketches, correct per the substrate-first discipline: they are crisply specified only
after C-FpExt and C-PairingCurve freeze.

### E.B.1 — `F_{p^k}` extension-field arithmetic (Opus, Cat A)

**Deliverable:** a finite extension field `F_{p^k}` over the base prime field, as a polynomial quotient
`F_p[u]/(m(u))` with `m` irreducible of degree `k`. **The Opus design call** (the reason this session is
Opus and bounds E.C): the *construction shape* — a **direct degree-`k` quotient** vs a **tower**
(`F_{p^2}` then `F_{(p^2)^{k/2}}`), and which embedding degrees the substrate must support (`k = 2`
minimum for the toy Weil/Tate demo; `k = 3`/`6` if a richer fixture is wanted). The choice bounds how
E.C reads pairing outputs and feeds them to NFS-DL (C2). Pieces:
- **`FpExt`** (`fpext.rs`): element = coefficient vector `[Fp<4>; k]` (or `Vec<Fp<4>>`) mod the
  irreducible `m`; `zero`/`one`/`from_base`/`add`/`sub`/`neg`/`mul` (schoolbook poly mult + reduction
  mod `m`)/`square`/`pow`/`inv` (extended-Euclid in `F_p[u]`, or the norm-to-base-field route)/
  `frobenius` (the `x ↦ x^p` map, needed for the trace/norm and for E.C)/`eq`.
- **Irreducible modulus selection** for the chosen `k` over the fixture's `p` (recorded as a checkable
  constant; irreducibility verified offline + asserted).
- **Base-field embedding** `F_p ↪ F_{p^k}` (constant term), so curve coefficients `a, b ∈ F_p` lift.

Consumes `shared::field` `Fp<4>` (frozen, read) and `crypto-bigint` `Uint<4>` only. **Freezes C-FpExt.**

**KAT:** field-axiom KATs — `a·a^{-1} = 1`, distributivity, `(a+b)^p = a^p + b^p` (Frobenius / freshman's
dream in char p), `frobenius` applied `k` times is the identity on `F_{p^k}`, and `m` is irreducible
(no root in `F_p`, recorded factorisation-free check). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) `inv` in `F_{p^k}` is *not* Fermat-`a^{p^k−2}` by default cost — the
extended-Euclid route over `F_p[u]` is the clean one; document which is used and why. (2) The
irreducible modulus must be **genuinely irreducible over `F_p`** — a reducible `m` makes
`F_p[u]/(m)` a ring with zero divisors and `inv` silently wrong; assert irreducibility in the fixture.
(3) `frobenius` (`x^p`, not `x^{p^k}`) is the load-bearing map E.C's MOV reduction needs — over-specify
it now (the over-specify rule: carry the method E.C will need even if E.B's own KATs barely use it).

**Deferred:** the curve-over-`F_{p^k}` group law and fixture (E.B.2); Miller/Weil/Tate (E.B.3/4); any
crypto-scale tower `F_{p^{12}}` / optimal-ate machinery (principle-4 annotation, not a work item).

### E.B.2 — `E(F_{p^k})` point arithmetic + pairing-friendly fixture (Sonnet, Cat A)

**Deliverable:** the curve arithmetic over the extension field and the fixture the pairings exercise.
- **`PairingPoint`** (`ecext.rs`): short-Weierstrass affine (or Jacobian) point over `FpExt`, with
  `add`/`double`/`scalar_mul`/`negate`/`is_on_curve`/identity — a standalone group law composing the
  frozen `Curve` params (read `p, a, b` read-only, lifted into `F_{p^k}` via C-FpExt's base embedding)
  and C-FpExt. *(This is the deliberate duplication of `rho::curve`'s law over `FpExt`, the cost of
  option B; annotate it as such — the frozen prime-field law stays untouched.)*
- **Pairing-friendly fixture** (`test_curves.rs`): a toy curve with a *small known embedding degree* `k`
  with respect to a small torsion prime `ℓ | #E(F_p)` (i.e. `ℓ | p^k − 1` but `ℓ ∤ p^i − 1` for
  `i < k`), with **recorded** `ℓ`, `k`, a base-field ℓ-torsion point `P ∈ E(F_p)[ℓ]`, and a
  linearly-independent `Q ∈ E(F_{p^k})[ℓ]` (the second pairing argument). Verified by `ℓ·P = ∞`,
  `ℓ·Q = ∞`, and P, Q independent (`Q ∉ ⟨P⟩`).

Consumes C-FpExt (frozen E.B.1), `rho::curve::Curve` params (frozen, read), and the existing fixtures
as template. **Freezes C-PairingCurve.**

**KAT:** `PairingPoint` group-law KATs (`ℓ·P = ∞`, associativity sample, `is_on_curve` over `F_{p^k}`);
the fixture's torsion + independence checks. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** the **embedding degree must be exactly `k`** — if `ℓ | p^i − 1` for some
`i < k` the pairing degenerates into a smaller field and the demo is vacuous (the analogue of E.A's
full-order trap). Assert the *minimality* of `k` explicitly. Finding `Q` independent of `P` in the
ℓ-torsion is the fixture's real work (the distortion-map / random-point search), done offline and
recorded.

**Deferred:** Miller/Weil (E.B.3), Tate (E.B.4).

### E.B.3 — Miller's algorithm + Weil pairing (Sonnet, Cat B)

**Deliverable:** Miller's algorithm and the Weil pairing over the E.B.2 substrate.
- **Miller's algorithm** (`miller.rs`): `f_{ℓ,P}(Q)` — the double-and-add accumulation over the bits of
  `ℓ`, multiplying in the line function `g_{T,T}` / `g_{T,P}` and dividing by the vertical `v` at each
  step, evaluated at `Q`. Line and vertical functions as helpers over `FpExt`.
- **Weil pairing** (`weil.rs`): `w_ℓ(P,Q) = (−1)^ℓ · f_{ℓ,P}(Q) / f_{ℓ,Q}(P)` (the ratio form — **no
  final exponentiation**, which is the Weil-vs-Tate distinction; document it).

Consumes C-FpExt + C-PairingCurve (frozen). *(Does not freeze a new contract; C-Pairing is frozen at
E.B.4 once both pairings share the surface.)*

**KAT (primary correctness signal):** **bilinearity** `w(aP, Q) = w(P,Q)^a`, `w(P, bQ) = w(P,Q)^b`,
`w(aP, bQ) = w(P,Q)^{ab}` for several `a, b`; **non-degeneracy** `w(P,Q) ≠ 1` for the independent P,Q;
**alternation** `w(P,P) = 1`. Optional `#[ignore]` PARI `ellweilpairing` cross-check (the established
oracle pattern). **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) Miller's loop is the subtle step — an off-by-one in the bit iteration
or a wrong line/vertical function gives a value that *fails bilinearity* (hence bilinearity is the KAT,
not a single spot-value). (2) Division by a vertical function that vanishes at `Q` (when `Q` shares an
x-coordinate with an intermediate `T`) needs the standard care — choose `Q` (or a shifted evaluation
point) to avoid it, or handle the degenerate divisor. (3) The Weil pairing needs **two** Miller calls
(`f_{ℓ,P}(Q)` and `f_{ℓ,Q}(P)`); Tate (E.B.4) needs **one** — keep the Miller core shared.

**Deferred:** Tate + final exponentiation (E.B.4).

### E.B.4 ◆ — Tate/reduced-Tate pairing + Track-E pairing design note + numfield correction (Sonnet, Cat B, `@plan`)

**Deliverable:** the Tate pairing (reusing E.B.3's Miller core), the sub-track-closing design note, and
a scheduled documentation correction.
- **Tate pairing** (`tate.rs`): `t_ℓ(P,Q) = f_{ℓ,P}(Q)` (one Miller call), then **final exponentiation**
  `^{(p^k − 1)/ℓ}` to land in μ_ℓ (the **reduced** Tate pairing, a well-defined coset representative).
  `tate_pairing` (raw) and `reduced_tate` (with final exp) both exposed.
- **Track-E pairing design-statement note** (the sub-track-entry analogue, lighter than a phase ◆):
  principle 1 (genuine bilinear pairing — Miller + Weil/Tate implemented head-on, not a stubbed map);
  principle 3 (no engineering optimization crept in — schoolbook `F_{p^k}`, no optimal-ate / no tower
  fast path); principle 4 (toy embedding degree, the `k ≤ 6` ceiling and the crypto-scale `F_{p^{12}}`
  gap annotated as demonstration-scale, not mathematical, boundaries). **Plus the E.C-readiness check:**
  is C-Pairing's surface the right input to the MOV bridge — does it expose what E.C needs to map an
  ECDLP into `F_{p^k}*` and hand off to `solve_dl` (C2)? Verdict recorded in the action-frame digest.
- **numfield PEDAGOGY correction (scheduled corrective work item).** `shared/numfield/docs/PEDAGOGY.md`
  (≈ lines 665–668) carries a doubly-wrong forward-looking note: "**E.D (pairings)** uses `NumberField`
  and `NumberFieldElement` for extension field arithmetic… degree-12/degree-6 extensions… which are
  number fields in the sense of this crate. The element arithmetic and inversion via extended Euclidean
  are directly reused." Both claims are false and E.B is the session with the authority to correct them
  (it builds the *actual* substrate): (1) **sub-track mis-name** — pairings are **E.B**, not E.D (E.D is
  p-adic arithmetic); (2) **char-0/char-p conflation** — pairing target fields are `F_{p^k}`
  (characteristic p), *not* char-0 `ℚ[x]/(f)` "number fields in the sense of this crate"; the BN/BLS
  "degree-12 extension" is `F_{p^{12}}`, and the char-0 `NumberFieldElement` arithmetic is **not**
  directly reusable (different coefficient field). Correct the note to point at the real char-p
  `rho/src/pairing/fpext.rs` substrate. Small, doc-only; folded here (effort-neutral) rather than given
  its own sub-LOC-band session.

Consumes C-FpExt + C-PairingCurve + Miller (frozen). **Freezes C-Pairing.**

**KAT:** **Tate bilinearity** `t(aP,bQ) = t(P,Q)^{ab}` (reduced form, in μ_ℓ); reduced-Tate lands in
μ_ℓ (`result^ℓ = 1`); Weil/Tate consistency where the theory predicts a fixed-power relation. **Verify
gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** the **final exponentiation** `(p^k − 1)/ℓ` is the Tate-specific subtle step
— a wrong exponent gives a non-μ_ℓ result that fails `result^ℓ = 1`. This is the **E.B ◆ boundary** —
re-read the Purpose intent and verify the pairing substrate (extension field, curve-over-extension,
Miller, Weil, Tate) is coherent and that **C-Pairing is genuinely E.C-ready** before crossing.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.B.4 ◆ to
confirm: (1) C-FpExt / C-PairingCurve / C-Pairing are the right inputs for **E.C** (the MOV bridge: does
the `F_{p^k}` element representation feed `solve_dl`'s `BigInt`-in-`[1,p^k)` interface, and is the
embedding-degree / torsion-fixture shape what E.C needs?); (2) the standalone-pairing-layer decision
(option B) still holds against E.C's needs, or whether E.C will want the generalised field bound
(option A) after all — surface as a discovery if so; (3) Miller + Weil + Tate are correct with KATs
exercising bilinearity and non-degeneracy; (4) the pairing substrate composes frozen `rho::curve` /
`shared::field` unmodified (principle 3); (5) the design-statement note passes 1/3/4 and nothing
presumes E.D (p-adic) / E.E structure. One-shot findings; does not implement. Held at **Opus** per the
header (lever-3 binding constraint — the E.C-bounding freeze).

---

## Cross-session contracts

E.B **freezes three** contracts. Per the substrate-over-specify rule, C-FpExt and C-PairingCurve carry
interfaces E.C (and possibly E.D) will consume even where E.B's own KATs barely exercise them
(`frobenius`, the torsion-point pair). All `rho::curve` and `shared::field` contracts are **read**
(composed), not amended — the option-B decision.

### C-FpExt — finite extension field `F_{p^k}` (compiler- + test-enforced) — *to be frozen at E.B.1*

**Defined in:** E.B.1 (`rho/src/pairing/fpext.rs`). **Consumed by:** E.B.2 (curve coefficients lift),
E.B.3/E.B.4 (pairing values live here), and **E.C** (the MOV bridge reads pairing outputs as `F_{p^k}`
elements and maps them toward `solve_dl`). Compiler-enforced (the `FpExt` type + method signatures) +
test-enforced (field axioms, Frobenius). **`FpExt` represents `F_p[u]/(m)`** for irreducible `m` of
degree `k`; exposes `add`/`sub`/`neg`/`mul`/`square`/`inv`/`pow`/`frobenius`/`from_base`/`zero`/`one`/
`eq` and `to_uint_vec`-style canonicalisation (the bridge to `solve_dl`'s `BigInt` encoding). *Exact
type name, `k` representation (const-generic vs runtime), and the construction shape (direct quotient
vs tower) ratified at E.B.1 and re-ratified at the E.B.4 ◆ against E.C.* *Over-specify note:* `frobenius`
is carried now though E.B's own pairings need it only incidentally — E.C's MOV reduction needs it
centrally.

### C-PairingCurve — pairing-friendly fixture + `E(F_{p^k})` arithmetic (test-enforced) — *to be frozen at E.B.2*

**Defined in:** E.B.2 (`rho/src/pairing/{ecext,test_curves}.rs`). **Consumed by:** E.B.3/E.B.4 (the
pairing arguments) and **E.C** (the MOV bridge needs a curve with small embedding degree and an
ℓ-torsion basis). Test-enforced: the fixture carries `ℓ·P = ∞`, `ℓ·Q = ∞`, P/Q independence, and
**minimal embedding degree `k`** KATs. Exposes `PairingPoint` (group law over `FpExt`) and the fixture
accessors (`pairing_toy() -> (Curve, FpExt-modulus, ℓ, P, Q)` shape — exact names ratified at E.B.2).
**The embedding degree is exactly `k`** (minimal) and **P, Q are independent ℓ-torsion** — not a
degenerate/collinear pair.

### C-Pairing — Weil + Tate pairing entry (compiler- + test-enforced) — *to be frozen at E.B.4 ◆*

**Defined in:** E.B.4 (`rho/src/pairing/{weil,tate}.rs`, Miller in `miller.rs`). **Consumed by:** E.B's
own KATs now; **E.C** (the MOV/Frey–Rück reduction — the named, climactic consumer). Compiler- +
test-enforced. Signatures (shape; exact types ratified at the ◆):
`weil_pairing(curve, &P, &Q, ℓ) -> FpExt` and
`tate_pairing(curve, &P, &Q, ℓ) -> FpExt` / `reduced_tate(curve, &P, &Q, ℓ) -> FpExt` (in μ_ℓ).
*Frozen at the ◆ juncture* (signature ratified against the E.C MOV-bridge fit before crossing — the
lever-3 reason the juncture is Opus).

### Frozen contracts read by E.B (composed, not amended — the option-B decision)

E.B composes these; none is touched.
- **`shared::field` `Fp<4>`** — the base prime field (`FpNaive`/`FpMonty`, `add`/`mul`/`inv`/`pow`/
  `sqrt`/`legendre`, `shared/field/src/lib.rs`). `FpExt` is built *over* it as coefficient vectors.
- **`rho::curve`** — `Curve { p, a, b, n, gx, gy }` (`Uint<4>` params), the `Fp<4>`-bound group law
  (`scalar_mul`/`add_jacobian`/…). E.B reads the params and **re-implements** the group law over `FpExt`
  in `ecext.rs` (the deliberate option-B duplication); it **amends nothing** in `rho::curve`. *A
  destructive edit to generalise the `Fp<4>` bound is option A — declined; resurfacing it is a juncture
  discovery, not an in-flight edit.*
- **`crypto-bigint` `Uint<4>`** — the limb-backed integer the base field and `solve_dl` encoding share.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The E.B.4 ◆ `@plan` confirmation is not a separate ledger row (a
paged fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.B.1 | `F_{p^k}` extension-field arithmetic | done | 74b2ff3 | C-FpExt (frozen) |
| E.B.2 | `E(F_{p^k})` point arithmetic + pairing-friendly fixture | done | dceaff1 | C-PairingCurve (frozen) |
| E.B.3 | Miller's algorithm + Weil pairing + bilinearity KAT | done | 7e28962 | — |
| E.B.4 ◆ | Tate/reduced-Tate pairing + final exp + design note + numfield correction | done | 6c082bb | C-Pairing (frozen) |

Contracts frozen before this sub-track (read by E.B): all Track-rho curve/field contracts (existing
crate); E.A's C-CompositeCurve / C-FactorOrder / C-Pohlig (Track-E, not consumed by E.B but
sibling-frozen). This sub-track **freezes three new contracts** (C-FpExt, C-PairingCurve, C-Pairing),
all forward-looking toward **E.C** (C2 / the MOV bridge).

---

## Action-frame digest

### E.B.4 ◆ — 2026-06-09
Discovery/flex: Tate non-degeneracy requires argument-order reversal (t_ℓ(Q,P) not t_ℓ(P,Q)) due to eigenvalue structure of E[ℓ] under Frobenius — P∈E(F_p)[ℓ] is in the eigenvalue-1 eigenspace (Z/3 factor, no [3]-preimage in E(F_{47^2})); final exponentiation kills the non-trivial part for t_ℓ(P,Q). Documented in test_curves.rs.
Affected: C-Pairing surface unchanged (both argument orders are valid inputs; KAT chooses the non-degenerate direction).
Deferred: yes — E.C implementers must use t_ℓ(Q,P) for the non-degenerate direction on this fixture; the eigenvalue-1/(-1) eigenspace structure of E[ℓ] is the group-theoretic reason. Documented in test_curves.rs lines 42-58.
Texture: E.B ◆ boundary juncture returned still-on-intent on all five confirmation points. C-FpExt/C-PairingCurve/C-Pairing are E.C-ready. Option B holds. Load-bearing assumption: E.C calls solve_dl with k=1 (prime subfield projection) or E.C-prep session widens C2 to k>1 before E.C lands — anticipated and documented in ROADMAP.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **No extension-field arithmetic exists — E.B.1 builds `F_{p^k}` from scratch (substrate gap, larger
  than the ROADMAP sketch).** The survey confirmed zero `Fp2`/`FpK`/tower types. This is **the** reason
  E.B is 4 sessions, not 3, and why the field is its own Opus session. Writing it is
  **internal-continue**. A discovery that the chosen construction (direct quotient) is awkward for E.C's
  needs is an **additive-reshard** surfaced at the E.B.4 ◆ (or a tower refactor as its own session).

- **The curve group law is `Fp<4>`-bound — E.B builds a *separate* `E(F_{p^k})` layer (option-B
  guard).** `rho::curve`'s law cannot represent extension-field points. The shard adjudicated **option
  B** (compose, don't amend). A `@build` agent that starts **editing `rho::curve` to accept extension
  fields** (option A) is a **destructive-HALT** — that is a frozen, widely-consumed contract and a
  separate deliberate decision; surface it as a juncture discovery, do not edit opportunistically.

- **G.A's `NumberFieldElement` is char-0 — not the `F_{p^k}` E.B needs (rigidity guard, with a
  scheduled doc fix).** A `numfield` PEDAGOGY note (≈ lines 665–668) claims "E.D (pairings) uses
  `NumberField` for extension arithmetic" — **doubly wrong** (char-0 number fields ≠ char-p finite
  extensions; and pairings are E.B, not E.D). Reaching for `NumberFieldElement` to represent `F_{p^k}`
  is **internal-continue → corrected** (build the char-p `FpExt`); do not silently adopt the char-0
  type. **The note correction is scheduled as a corrective work item in E.B.4** (E.B has the authority
  to fix it — it builds the real substrate).

- **The pairing fixture's embedding degree must be *minimal* `k` — E.B.2's fixture trap.** If `ℓ | p^i−1`
  for `i < k` the pairing degenerates into a smaller field and the demo is vacuous (the E.A full-order
  trap analogue). A fixture that turns out non-minimal is **internal-continue** (fix the parameters),
  not a contract break. The session must assert minimality.

- **C-Pairing is a forward-looking contract bounding E.C (over-specify discipline).** The pairing
  signatures and `FpExt` surface are frozen with the MOV bridge (E.C) in mind — the highest-stakes
  forward contract in Track E. If E.B.4 finds the surface underserves E.C, **widening** it at the ◆ is
  additive; a consumer-driven *change* after freeze is an **additive-reshard** at the E.C inflection.
  This is the lever-3 reason the ◆ juncture is Opus.

- **E.B is the pairing *substrate*, not the MOV attack — it must not presume E.C (defocus guard).** No
  MOV/Frey–Rück reduction, no `solve_dl` call, no embedding-into-NFS-DL belongs in E.B. Writing toward
  the MOV bridge here is **defocus** — internal-continue only within the pairing-construction scope.
  (The `@plan` juncture checks this and the *converse*: that C-Pairing is nonetheless E.C-*ready*.)

- **No oracle dependency for correctness (principle-3 / E.A-consistent).** Bilinearity self-checks; a
  PARI `ellweilpairing`/`elltatepairing` cross-check is an **optional `#[ignore]` sidecar** (the
  established pattern — no `oracle-tests` feature, `#[ignore]` gate only). E.B introduces no new live
  oracle.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — "E.B — Pairing arithmetic (Weil, Tate)"; Contract C2 — the
  `solve_dl` interface E.C will consume, *for context on what E.B's representation must eventually feed*;
  the α-boundary discovery "`rho::curve` was NOT lifted to `shared::curve`… Revisit at E.B.1 when
  pairings need divisor-arithmetic curves" — now resolved as option B) and this PLAN before any session.
- **Note (non-blocking): the ROADMAP Progress table is stale** — it shows Track E "not started / 0 done"
  (reconciled at the T.G boundary, *before* E.A landed). E.A is done (`054df65`, `51fd477`); the
  Discoveries log and the prior PLAN ledger are authoritative. The roadmap is rewritten only at
  sub-track boundaries; this lag is expected and is reconciled at the next Track-E ◆ that touches the
  roadmap, not by E.B.
- Read the **templates to mirror**: `shared/field/src/lib.rs` (the `Fp` trait E.B.1's `FpExt` parallels
  and composes); `rho/src/curve/mod.rs` (the `Fp<4>`-bound group law E.B.2 re-implements over `FpExt` —
  the `double_jacobian`/`add_jacobian` formulae transfer); `rho/src/curve/test_curves.rs` (the
  hand-computed fixture idiom — `n·G=∞` / order verification — E.B.2's fixture model); `rho/tests/
  ecdlp_kat.rs` (`check_solver_on_curve` — the KAT-helper style for E.B.3/4 bilinearity assertions).
- **Register:** E.B is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module tree `rho/src/pairing/{mod,fpext,ecext,test_curves,miller,weil,tate}.rs`.
  KATs in `rho/tests/pairing_kat.rs`. One doc edit at E.B.4: `shared/numfield/docs/PEDAGOGY.md`.
- **Tier routing:** **E.B.1 is Opus** (`@build` on Opus) — the roadmap-flagged `F_{p^k}` substrate
  design that bounds E.C. **E.B.2–4 are Sonnet** (mechanical against the frozen extension/point
  representation). E.B.4 carries the single `@plan` marker: a ◆-boundary juncture (page `@plan-juncture`)
  ratifying C-FpExt / C-PairingCurve / C-Pairing against E.C before the sub-track closes. juncture-tier
  (header) is **opus** — held by lever 3 (the E.C-bounding freeze), the binding constraint; the strong
  lever-5 bilinearity KAT would license an opt-down in isolation but does not override lever 3.
- **Invariants to preserve:** **`rho::curve` and `shared::field` are frozen — E.B composes them; it
  amends neither.** The `E(F_{p^k})` group law is a *separate* layer in `rho/src/pairing/` (option B); a
  destructive edit generalising `rho::curve`'s `Fp<4>` bound (option A) is a **destructive-HALT**. The
  `F_{p^k}` arithmetic is toy-embedding-degree-scoped (`k ≤ 6`); a crypto-scale `F_{p^{12}}` tower is a
  documented principle-4 boundary, not wired. No oracle dependency (bilinearity self-checks).
- **PARI remains a dev-only `#[ignore]` oracle** (Discoveries-log policy) — a pairing cross-check
  (`ellweilpairing`/`elltatepairing`) follows the established `#[test] #[ignore = "PARI not installed…"]`
  pattern; it is optional, never on the green path.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — E.B is still an early
  Track-E shard and its substrate (a from-scratch extension field bounding the cross-track climax) is the
  highest-stakes interface in the track so far, so the conservative halt-at-every-◆ cadence is warranted.
  The single ◆/`@plan` on E.B.4 is the one that matters (the Opus C-Pairing ratification + E.C-readiness
  check). *(Tradeoff vs default cadence: one extra halt-confirm on a 4-session sub-track is cheap
  insurance on an E.C-bounding shard; revisit cadence for E.D once the pairing layer is proven.)*
