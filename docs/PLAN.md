<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E (E.C — MOV/Frey–Rück reduction: ECDLP → F_{p^k} DLP)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error) AND lever 4
(correctness-criticality), with the ROADMAP's explicit both-sessions-Opus flag as a third
constraint.** E.C is **the cross-track bridge of the entire project** (ROADMAP Phase δ: "*the*
cross-track bridge… the pedagogical climax"): the MOV/Frey–Rück reduction transports an ECDLP on a
small-embedding-degree curve into a discrete log in `F_{p^k}*` via the E.B pairing, then solves that
DLP by calling the now-honest **C2-ext `solve_dl`** (made real at k=2 by D.E). It is the first
session-set to compose two tracks in code — adding the **first `rho → gnfs` crate dependency** — and
a wrong bridge or reduction silently produces a *wrong* discrete log. Lever 5 is strong and fast (the
`pairing_toy` end-to-end KAT recovers a *known* scalar and verifies `e(Q,R) = e(G,R)^k`, decisive on
any bridge/reduction error) and *would* license an opt-down in isolation — but lever 3 + lever 4 +
the ROADMAP flag dominate on the project's climax, so the ◆ that ratifies C-MovBridge / C-Mov runs at
**Opus**. *(Both E.C sessions are Opus per the ROADMAP. The juncture fork at the E.C.2 ◆ is Opus.)*

Last rewrite: **D.E ◆ boundary crossed** (D.E.1 `0d02b77`, D.E.2 `1a21a32`, D.E.3 ◆ `a804a7b`;
C-ExtTarget / C-ExtFactorBase / C2-ext frozen; the k>1 NFS-DL solver ships, proven correct at k=2 on
the `pairing_toy` field). The D.E.3 ◆ digest recorded **three E.C agenda items** this plan resolves:
**(A)** no single helper composes `FpExt::to_uint_vec`-coeffs into the base-p `BigInt` `solve_dl`
consumes — a thin bridge is owed (E.C.1); **(B)** the k=2 `solve_dl_ext` is a brute-force placeholder
over the ℓ=3 subgroup, *not* the D.E.2 NFS pipeline — on-intent per principle-4 toy scoping, so E.C
composes the brute-force solver and does **not** wire the NFS pipeline; **(C)** `find_irreducible_
degree2` re-derives the modulus inside `solve_dl_ext` independently of `ExtResidueMap.modulus` — for
p=47 they coincide (`u²+1` tried first), but E.C's wiring must ensure they coincide **by
construction** (the modulus-consistency invariant, E.C.1's guard).

---

## Purpose (design intent)

Per ROADMAP (Phase δ, E.C; Contract C2): E.C reduces an ECDLP on a curve `E/F_p` with small
embedding degree k to a discrete log in `F_{p^k}*`, and solves that DLP by **calling a real NFS-DL
solver** — "*the session where the MOV bridge first calls a real NFS-DL solver is the pedagogical
climax.*" ROADMAP C2 forbids the permanent PARI stub ("*merging E.C with a PARI stub permanently is
not [acceptable]*"); D.E removed that constraint by making C2-ext honest at k=2, so E.C can now land
against a *real* solver.

The structure-based-escape-from-search through-line reaches its cross-track payoff here. E.A built
the generic-and-Pohlig–Hellman ECDLP baseline; E.B built the **pairing** `e: E[ℓ] × E[ℓ] → μ_ℓ ⊂
F_{p^k}*` (the homomorphism that transports the ECDLP into a finite-field DLP); D.E built the
**finite-field DLP solver** in the extension the transported problem lands in. **E.C composes them
into the MOV reduction**: it is the moment the project's three substrates (curve, pairing,
NFS-DL) meet in one computation.

The MOV reduction's mathematical shape (the through-line the KAT exercises): given the ECDLP
`Q = k·G` in the order-ℓ subgroup `E[ℓ]`, pick a point `R ∈ E[ℓ]` with `e(G, R) ≠ 1` (a
μ_ℓ-generator). The pairing's bilinearity gives `e(Q, R) = e(k·G, R) = e(G, R)^k`. Writing
`g := e(G, R)` and `h := e(Q, R)`, both in `μ_ℓ ⊂ F_{p^k}*`, the ECDLP scalar `k` is exactly
`log_g(h)` — a *finite-field* discrete log. E.C transports `(G, Q, R)` through the pairing to
`(g, h)`, encodes `g, h` for `solve_dl`, recovers `k mod ℓ`, and (at toy scale) verifies
`e(Q, R) = e(G, R)^k`. **The escape:** an ECDLP with no exploitable curve structure becomes a
finite-field DLP where index-calculus/NFS-DL applies — the search bound is escaped by the pairing's
homomorphism.

The substrate survey established the shape precisely:

1. **E.C is a compose over frozen pieces, not new mathematics — the bridge is the design crux.** The
   pairing (`weil_pairing` / `reduced_tate` → `FpExt<F>` ∈ μ_ℓ) and the solver (`solve_dl(g, h, p, 2,
   ell)`) both exist and are KAT-proven. What does *not* exist is the **representational bridge** from
   the pairing output to the solver input. `FpExt::to_uint_vec() -> Vec<Uint<4>>` (coefficient form,
   crypto-bigint width); `solve_dl` takes `h: &num_bigint::BigInt` in **base-p encoding**
   (`c_0 + c_1·p`). The gap is two-stage: `Vec<Uint<4>> → Vec<BigInt>` (per-coefficient type
   conversion, toy-only single-limb) → base-p `BigInt` (scalar combine). E.C.1 builds this bridge.

2. **`solve_dl_ext` is brute-force at k=2 (digest item B) — E.C composes it as-is, does not touch
   the NFS pipeline.** The survey confirmed `solve_dl_ext` enumerates the ℓ=3 subgroup; the D.E.2
   `ExtFactorBase` / `augment_ext_relation` pipeline exists but is bypassed at toy scale. This is
   **on-intent** (principle 4: the toy implementation proves the interface; the NFS pipeline is the
   crypto-scale path). E.C calls `solve_dl` (which dispatches to `solve_dl_ext` at k=2) through the
   frozen C2 signature; it adds **no** NFS-pipeline wiring. *Whether E.C should exercise the NFS
   pipeline rather than brute force is a principle-4 annotation, not an E.C work item.*

3. **The modulus-consistency invariant (digest item C) is E.C.1's load-bearing guard.** The pairing
   carries its own `IrreducibleModulus` (`u²+1` for the toy fixture); `solve_dl_ext` re-derives a
   degree-2 irreducible internally via `find_irreducible_degree2` (which does **not** accept a
   caller modulus). For p=47 both pick `u²+1` (it is tried first and is irreducible), so they
   coincide — but **by coincidence, not by construction.** E.C.1 must make the coincidence explicit:
   the bridge asserts the pairing's modulus equals what the solver path uses, so a future p where
   `find_irreducible_degree2` picks a different polynomial fails *loudly* rather than computing a DL
   in the wrong F_{p²}. **This is the highest-cost-of-wrong silent-failure mode in E.C** — a
   different modulus gives a different field and a wrong log with no error.

4. **E.C adds the first `rho → gnfs` dependency (the cross-crate seam).** `rho` does not yet depend
   on `gnfs` (confirmed). E.C adds the edge. Per the bridge-location decision, **gnfs stays unaware
   of rho's `FpExt`**: gnfs grows an `FpExt`-shaped *input* helper (a coefficient `Vec<BigInt>` +
   modulus → base-p `BigInt`, composing the existing `ExtTarget::from_coeffs` + `ext_target_to_bigint`
   plus the modulus-consistency assertion); the rho side does only the `Uint<4> → BigInt` step and
   calls it. No `gnfs → rho` coupling; the new edge is one-directional.

5. **The end-to-end MOV KAT lives in `rho` and uses the real `pairing_toy` fixture.** Unlike D.E.3
   (which hand-built its F_{47²} target *independently in gnfs* because the rho→gnfs edge did not yet
   exist), E.C's KAT **is** the cross-track composition: `pairing_toy()` → `reduced_tate(Q, P, ell)`
   → bridge → `gnfs::solve_dl` → recovered `k mod ℓ` → verify `e(Q,R) = e(G,R)^k`. This is the
   project's first KAT that crosses the track boundary in a single test.

The work splits at the **bridge → reduction** seam, 2 sessions (matching the ROADMAP's 2-session
both-Opus estimate):

1. **E.C.1 — MOV bridge substrate: `rho → gnfs` edge + `FpExt → solve_dl`-target bridge (Opus, Cat
   A).** Add the `rho → gnfs` dependency; build the bridge that turns a pairing output (`FpExt<F>`,
   via `to_uint_vec`) into the base-p `BigInt` `solve_dl` consumes, with the **modulus-consistency
   guard** (digest item C). The gnfs-side helper (FpExt-shaped coefficient input) + the rho-side
   `Uint<4> → BigInt` step. **Freezes C-MovBridge** — the bridge interface E.C.2 reads pairing
   outputs through.

2. **E.C.2 ◆ — MOV reduction + end-to-end `pairing_toy` KAT (Opus, Cat I, `@plan`).** The reduction
   proper: transport `(G, Q, R)` through the pairing to `(g, h) ∈ μ_ℓ`, encode via C-MovBridge, call
   `gnfs::solve_dl(g, h, p, 2, ell)`, recover `log_g(h) = k mod ℓ`; the **end-to-end MOV KAT** at the
   `pairing_toy` parameters (recover the known ECDLP scalar, verify `e(Q,R) = e(G,R)^k` in F_{47²}*).
   **Freezes C-Mov** (the MOV-reduction entry). Crosses the **E.C ◆ boundary** (the cross-track
   climax exists; ECDLP reduces to NFS-DL through a real solver).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the *NFS pipeline* inside
the k=2 path — that is the principle-4 crypto-scale annotation, not E.C; E.C composes the brute-force
`solve_dl_ext` as-is. Or implementing other Track-E attacks — Smart–Satoh–Araki, GHS — those are
E.E/E.H. Or writing the MOV *textbook chapter* — that is T.E, paired with E.W at the Track-E ◆, not
E.C) and **rigidity** (re-deriving the base-p encoding in rho rather than calling the gnfs-side
bridge — the encoding convention is gnfs's, threaded through C-MovBridge; duplicating it in rho is
the two-crates-must-agree failure. Or skipping the modulus-consistency guard because "p=47 works" —
the guard is the silent-wrong-field defense, mandatory).

**Scoping discipline.** E.C reduces ECDLP → DLP at **toy embedding degree k=2** (the `pairing_toy`
fixture) at demonstration fidelity (principle 4). It introduces **no new live oracle** (the optional
PARI/`#[ignore]` cross-check is the established dev-only pattern). It **amends no frozen contract**:
C2-ext, C-ExtTarget, the pairing contracts (C-Pairing / C-FpExt / C-PairingCurve) and the E.A ECDLP
substrate are all *read*, not changed. The MOV reduction recovers `k mod ℓ` (the subgroup log); a
full composite-order lift via Pohlig–Hellman is a principle-4 annotation (the toy fixture's relevant
subgroup is order ℓ=3). A crypto-scale MOV against a real small-embedding-degree curve is a
principle-4 boundary, not a work item.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from D.E; oracle KATs are `#[ignore]`-gated only, no `oracle-tests` feature). `/run-plan`
re-discovers at preflight. E.C **adds a cross-crate dependency and a decisive end-to-end KAT**, so
the gate is a **correctness + integration gate**: the E.C.2 end-to-end MOV KAT (`pairing_toy` →
pairing → bridge → `solve_dl` → `k mod ℓ`, verify `e(Q,R) = e(G,R)^k`) is the primary correctness
signal — fast and decisive (lever 5). A wrong bridge encoding, a wrong modulus (item C), or a
wrong reduction breaks `e(Q,R) = e(G,R)^k` directly. **The frozen D.E k=2 KATs
(`gnfs/tests/dl_ext_kat.rs`) and the E.B pairing KATs (`rho/tests/pairing_kat.rs`) must stay green**
— the gate also guards the no-regression invariant on both composed substrates, and the
`cargo check --workspace` must confirm the new `rho → gnfs` edge resolves cleanly (no cycle).

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.C.1 | MOV bridge substrate: `rho → gnfs` edge + `FpExt → solve_dl`-target bridge + modulus-consistency guard | A | **Opus** | C2-ext `solve_dl` (frozen D.E.3, read), C-ExtTarget `ExtTarget`/`ext_target_to_bigint` (frozen D.E.1, read+compose), C-FpExt `FpExt`/`to_uint_vec` (frozen E.B.1, read), C-Pairing modulus (frozen E.B, read) | `rho/Cargo.toml` (add `gnfs` dep), `gnfs/src/dl/ext/target.rs` *or* `gnfs/src/dl/descent/solve.rs` (add the FpExt-shaped coeff-`Vec<BigInt>`→base-p-`BigInt` helper + modulus-consistency assertion), `rho/src/pairing/mov.rs` (new: rho-side `Uint<4>→BigInt` step + bridge entry), `rho/src/pairing/mod.rs` (add `pub mod mov;`) |
| E.C.2 ◆ `@plan` | MOV/Frey–Rück reduction + end-to-end `pairing_toy` MOV KAT | I | **Opus** | C-MovBridge (frozen E.C.1), C-Pairing `weil_pairing`/`reduced_tate` (frozen read), C-PairingCurve `pairing_toy`/`PairingPoint` (frozen read), E.A ECDLP substrate (frozen read), C2-ext `solve_dl` (frozen read) | `rho/src/pairing/mov.rs` (the reduction entry `mov_reduce` composing pairing→bridge→`solve_dl`), `rho/tests/mov_kat.rs` (new: end-to-end k=2 MOV KAT + optional PARI `#[ignore]` cross-check) |

**Sequencing notes.** Strictly serial: **E.C.1 → E.C.2.** E.C.1 lands the cross-crate edge and the
bridge the reduction stands on; E.C.2 writes the reduction and closes the sub-track with the
end-to-end KAT. The single `@plan` marker sits on **E.C.2 ◆** — the Opus boundary juncture ratifying
C-MovBridge / C-Mov (the cross-track composition is correct; the climax KAT recovers the right log)
before the sub-track closes. E.C.1, though Opus-tier, carries **no** `@plan` (it freezes a
compiler-/test-checkable bridge interface; C-MovBridge is re-ratified at the E.C.2 ◆ alongside C-Mov
— an inline juncture would double the boundary cost on a 2-row shard).

**Why 2 sessions (the ROADMAP's both-Opus estimate).** The split is taken at the bridge → reduction
seam:
- **One-line-commit-title corollary.** "MOV bridge substrate (`rho→gnfs` edge + FpExt→solver bridge)"
  and "MOV reduction + end-to-end KAT" are **two distinct commit titles** spanning two categories
  (A substrate, I integrative).
- **Contract-sharp boundary (legitimate, not LOC-driven).** E.C.1 **freezes** C-MovBridge; E.C.2
  **consumes** it and **freezes** C-Mov. One real produce/consume seam — and the bridge is the design
  call (the cross-crate edge + the modulus-consistency invariant) that bounds the reduction, so it
  earns its own session and its own freeze checkpoint.
- **Why the bridge is its own Opus session (lever 3 + lever 4).** The `FpExt → solver-target` bridge
  is the first cross-track code edge in the project and carries the silent-wrong-field failure mode
  (item C). Getting the modulus-consistency guard or the encoding wrong produces a wrong log with no
  error — cost-of-wrong (lever 3) and correctness-criticality (lever 4) are both maximal.
- **Irreducible unit kept whole (lever 2).** The reduction + the end-to-end KAT is one coherent unit
  (E.C.2) — the reduction has **no standalone KAT-able contract except as the working end-to-end MOV
  recovery** (a reduction with no end-to-end KAT has an undefined contract — the same rule that kept
  descent+`solve_dl`+KAT whole in D.E.3). Splitting it fractures an irreducible unit.

They are **not** further splittable: separating the reduction from its end-to-end KAT splits an
irreducible unit (per the corollary); merging E.C.2 into E.C.1 would put the cross-crate edge, the
bridge, the reduction, and the climax KAT in one >400-LOC two-title Opus session with **no
contract-freeze checkpoint between the bridge design and its consumption** — poor insurance on the
project's highest-stakes seam.

---

## Session detail

E.C.1 is specified at near-full fidelity (the bridge and the modulus-consistency guard are the design
crux). E.C.2 is a lower-fidelity sketch, correct per the substrate-first discipline: it is crisply
specified only after C-MovBridge freezes.

### E.C.1 — MOV bridge substrate (Opus, Cat A)

**Deliverable:** the `rho → gnfs` crate dependency and the bridge from a pairing output to the
`solve_dl` target, with the modulus-consistency guard. The design choices:
- **The cross-crate edge** (`rho/Cargo.toml`): add `gnfs` as a `rho` dependency. `cargo check
  --workspace` must confirm no dependency cycle (gnfs does not depend on rho — confirmed by survey).
- **The gnfs-side bridge helper** (per the bridge-location decision: FpExt-shaped *input*, gnfs
  unaware of rho's type). A function taking a coefficient `Vec<BigInt>` + `p` + the expected modulus
  (`Vec<BigInt>`) and returning the base-p `BigInt` `solve_dl` consumes — composing the existing
  `ExtTarget::from_coeffs(coeffs, p, modulus)` + `ext_target_to_bigint(&t)` (both frozen C-ExtTarget,
  read+compose). **The Opus design call:** where this lives (`target.rs` next to `ExtTarget`, or
  `solve.rs` next to `solve_dl`) and its exact signature — and crucially the **modulus-consistency
  assertion** (item C): the helper asserts the supplied modulus equals what the k=2 `solve_dl_ext`
  path will use (`find_irreducible_degree2(p)`), so a mismatch fails loudly. Decide whether to
  *assert* coincidence (cheapest, toy-scoped) or *thread* the modulus into `solve_dl_ext` (a larger
  change to a frozen file — likely deferred to a principle-4 annotation; surface at the ◆ if the
  assertion feels too fragile).
- **The rho-side step** (`rho/src/pairing/mov.rs`): the `Uint<4> → BigInt` per-coefficient conversion
  (toy-only single-limb: `BigInt::from(u.as_words()[0])`, with a `debug_assert` the higher limbs are
  zero) that turns `FpExt::to_uint_vec() -> Vec<Uint<4>>` into the `Vec<BigInt>` the gnfs helper
  accepts; and the rho-facing bridge entry that calls the gnfs helper.

Consumes the frozen C2-ext `solve_dl` (read — the bridge output must fit its `h: &BigInt` k=2 path),
C-ExtTarget (`ExtTarget`/`ext_target_to_bigint`, read+compose), C-FpExt (`FpExt`/`to_uint_vec`,
read), and the pairing's `IrreducibleModulus` (read — its coefficient form is the expected modulus
the guard checks). **Freezes C-MovBridge.**

**KAT:** the bridge round-trips a known μ_ℓ element (`FpExt` for, e.g., `(23, 6)` = ζ at the toy
params → base-p `BigInt` `23 + 6·47` matching the `dl_ext_kat.rs` encoding); the modulus-consistency
guard fires (a deliberately-wrong modulus is rejected — assert the panic/error); the `Uint<4> →
BigInt` step is correct on a sample. **Verify gate:** `cargo test --workspace` green; **D.E k=2 KATs
and E.B pairing KATs unchanged**; `cargo check --workspace` resolves the new edge with no cycle.

**Subtlety (load-bearing):** (1) **the modulus-consistency invariant is the silent-wrong-field
defense** — without it, a p where `find_irreducible_degree2` picks a different irreducible than the
pairing's modulus computes a DL in a *different* F_{p²} and returns a wrong `k` with no error. The
guard is mandatory, not a nice-to-have. (2) **The `Uint<4> → BigInt` step is toy-only** (assumes
p < 2^64, single limb) — annotate it as a principle-4 boundary (a crypto-scale p would need the full
limb vector; the mathematics is unchanged). (3) **gnfs must not gain a rho dependency** — the helper
takes a `Vec<BigInt>`, not an `FpExt`; the edge is strictly one-directional (`rho → gnfs`). (4) The
base-p encoding convention is **gnfs's** (`ext_target_to_bigint`) — do **not** re-derive it in rho;
the rho side produces only the coefficient `Vec<BigInt>` and lets gnfs encode (the rigidity guard).

**Deferred:** the reduction + end-to-end KAT (E.C.2); threading the modulus through `solve_dl_ext`
(principle-4 annotation unless the ◆ finds the assertion too fragile); the NFS-pipeline k=2 path
(principle-4, item B); composite-order lift beyond `k mod ℓ` (principle-4).

### E.C.2 ◆ — MOV/Frey–Rück reduction + end-to-end `pairing_toy` MOV KAT (Opus, Cat I, `@plan`)

**Deliverable:** the MOV reduction proper, the end-to-end KAT, and the sub-track close.
- **The reduction entry** (`rho/src/pairing/mov.rs`, `mov_reduce`-shaped): given the ECDLP
  `(curve, G, Q, R, ell)` (R a μ_ℓ-generator with `e(G,R) ≠ 1`), compute `g = e(G,R)` and
  `h = e(Q,R)` via `reduced_tate` (or `weil_pairing`) → `FpExt<F>`; bridge both to base-p `BigInt`
  via C-MovBridge; call `gnfs::solve_dl(g, h, p, 2, ell)`; return the recovered `log_g(h) = k mod ℓ`.
  The exact signature (what curve/point types, whether R is caller-supplied or discovered) ratified
  here.
- **End-to-end MOV KAT** (`rho/tests/mov_kat.rs`): use `pairing_toy()` (p=47, k=2, ℓ=3, modulus
  u²+1; P, Q the fixture points); construct an ECDLP with a *known* scalar `k` (`Q' = k·G` for a
  chosen `k ∈ {1, 2}` in the ℓ=3 subgroup); run `mov_reduce`; assert the recovered scalar equals `k`;
  verify the pairing identity `e(Q', R) = e(G, R)^k` in F_{47²}*. **Optional PARI `znlog`/`fflog`
  `#[ignore]` cross-check** (the established dev-only oracle pattern).

Consumes C-MovBridge (frozen E.C.1), C-Pairing (`weil_pairing`/`reduced_tate`, frozen read),
C-PairingCurve (`pairing_toy`/`PairingPoint`, frozen read), the E.A ECDLP substrate (frozen read),
and C2-ext `solve_dl` (frozen read). **Freezes C-Mov** (the MOV-reduction entry).

**KAT (primary correctness signal):** **end-to-end** — `mov_reduce` on the `pairing_toy` fixture
recovers the known scalar `k mod ℓ`; the pairing identity `e(Q,R) = e(G,R)^k` holds in F_{47²}*; the
**D.E k=2 KATs and E.B pairing KATs stay green** (the no-regression gate on both composed
substrates). Optional PARI cross-check. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) **the argument order to the pairing matters** — the survey found
non-degenerate Tate at the toy fixture is `reduced_tate(Q, P, ell)` (Q first); the reduction must use
the order that gives a non-trivial `e(G,R)`, or `g = 1` and the DL is undefined. Pick R (the second
pairing argument) so `e(G,R)` generates μ_ℓ; assert `g ≠ 1` before calling `solve_dl`. (2) **The
recovered log is `k mod ℓ`, not the full ECDLP scalar** — at the toy fixture the relevant subgroup is
order ℓ=3, so `k mod 3` *is* the answer in that subgroup; a full composite-order recovery (Pohlig–
Hellman lift over all subgroups) is a principle-4 annotation, not wired. State this in the KAT so the
"answer" is unambiguous. (3) **This is the E.C ◆ boundary** — re-read the Purpose intent and verify
the MOV reduction is coherent (curve → pairing → bridge → real solver → correct log) and that the
cross-track composition is genuinely the climax it claims (a *real* NFS-DL solver, not a stub) before
crossing.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.C.2 ◆
to confirm: (1) the MOV reduction composes the three substrates correctly (the end-to-end
`e(Q,R) = e(G,R)^k` round-trip recovers the right `k mod ℓ`); (2) C-MovBridge's modulus-consistency
guard (item C) is in place and the bridge is the right interface (gnfs stays rho-unaware; the edge is
one-directional); (3) `solve_dl` is called as a **real** solver (the ROADMAP's anti-stub constraint
satisfied — no permanent PARI stub); (4) no frozen contract was amended (C2-ext, C-ExtTarget, the
pairing contracts, the E.A ECDLP substrate are all read-only); (5) the principle-4 boundaries
(brute-force k=2, `k mod ℓ` not full composite lift, toy-only `Uint<4>→BigInt`) are annotated, not
silently presented as crypto-scale. One-shot findings; does not implement. Held at **Opus** per the
header (lever 3 + lever 4 + the ROADMAP both-Opus flag — the cross-track climax).

---

## Cross-session contracts

E.C **freezes two** contracts. Per the substrate-over-specify rule, C-MovBridge carries the
modulus-consistency guard now (item C) even though the toy fixture would "work" without it. All
composed substrates (C2-ext, C-ExtTarget, the pairing contracts, the E.A ECDLP substrate) are
**read**, not amended.

### C-MovBridge — `FpExt → solve_dl`-target bridge + modulus-consistency guard (compiler- + test-enforced) — *to be frozen at E.C.1*

**Defined in:** E.C.1 (gnfs-side helper in `gnfs/src/dl/ext/target.rs` *or* `descent/solve.rs`;
rho-side step in `rho/src/pairing/mov.rs`). **Consumed by:** E.C.2 (the reduction encodes pairing
outputs through it). Compiler-enforced (the helper signatures) + test-enforced (round-trip +
guard-fires). Exposes: the gnfs-side coefficient-`Vec<BigInt>` + `p` + modulus → base-p `BigInt`
helper (composing `ExtTarget::from_coeffs` + `ext_target_to_bigint` + the modulus-consistency
assertion against `find_irreducible_degree2(p)`); the rho-side `Uint<4> → BigInt` step + bridge
entry. *Exact helper location and signature ratified at E.C.1 and re-ratified at the E.C.2 ◆.*
**The `rho → gnfs` edge is one-directional** (gnfs takes a `Vec<BigInt>`, never an `FpExt`). **The
base-p encoding convention is gnfs's** (not re-derived in rho). **The modulus-consistency guard is
mandatory** (the silent-wrong-field defense).

### C-Mov — MOV/Frey–Rück reduction entry (compiler- + test-enforced) — *to be frozen at E.C.2 ◆*

**Defined in:** E.C.2 (`rho/src/pairing/mov.rs`, `mov_reduce`-shaped). **Consumed by:** E.C's own
end-to-end KAT now; **E.W / T.E** (the Track-E writeup + MOV textbook chapter, the named downstream
documentation consumer at the Track-E ◆); any later attack-comparison harness (E.W's "which attack
wins" table). Compiler- + test-enforced. Exposes the reduction entry: ECDLP `(curve, G, Q, R, ell)`
→ `k mod ℓ` via pairing + C-MovBridge + `gnfs::solve_dl`. *Exact signature (point types, whether R is
caller-supplied) ratified at the E.C.2 ◆.* **The reduction recovers `k mod ℓ`** (the subgroup log);
full composite-order lift is a principle-4 annotation. **`solve_dl` is called as a real solver** (the
ROADMAP anti-stub constraint).

### Frozen contracts read by E.C (composed, not amended)

E.C composes these; none is touched.
- **C2-ext `solve_dl`** (`gnfs/src/dl/descent/solve.rs`, frozen D.E.3) — the k>1 path; E.C calls
  `solve_dl(g, h, p, 2, ell)` and reads the returned `k mod ℓ`. The signature and `SolveDlError`
  taxonomy are unchanged. *(If E.C finds it must thread the modulus into `solve_dl_ext` — item C —
  that is an edit to a frozen file and an **additive-reshard** surfaced at the ◆, not a silent
  patch; the default is the bridge-side assertion, no `solve.rs` edit.)*
- **C-ExtTarget** (`gnfs/src/dl/ext/target.rs` + `descent.rs`, frozen D.E.1) — `ExtTarget::from_coeffs`
  (over-specified for E.C) and `ext_target_to_bigint`; composed by the gnfs-side bridge helper.
- **C-FpExt** (`rho/src/pairing/fpext.rs`, frozen E.B.1) — `FpExt` + `to_uint_vec() -> Vec<Uint<4>>`;
  read for the pairing output and its coefficient bridge.
- **C-Pairing** (`rho/src/pairing/{weil,tate,miller}.rs`, frozen E.B) — `weil_pairing` /
  `reduced_tate` → `FpExt<F>` ∈ μ_ℓ; the reduction's pairing step. The `IrreducibleModulus` carried
  here is the expected modulus the guard checks.
- **C-PairingCurve** (`rho/src/pairing/{ecext,test_curves}.rs`, frozen E.B) — `pairing_toy()`,
  `PairingPoint<F>`, the toy parameters (p=47, k=2, ℓ=3, u²+1); the KAT fixture.
- **E.A ECDLP substrate** (`rho/src/ecdlp/`, frozen E.A) — `Curve`, `AffinePoint`, the ECDLP problem
  shape; read for the reduction's input (the curve and points the MOV transports).

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The E.C.2 ◆ `@plan` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.C.1 | MOV bridge substrate (`rho→gnfs` edge + `FpExt→solve_dl` bridge + modulus guard) | done | 840608c | C-MovBridge (frozen) |
| E.C.2 ◆ | MOV/Frey–Rück reduction + end-to-end `pairing_toy` MOV KAT | done | 2e1edf8 | C-Mov (frozen) |

Contracts frozen before this sub-track (read by E.C): all Track-D NFS-DL contracts including
**C2-ext** (frozen D.E.3) and **C-ExtTarget** (frozen D.E.1); the E.B pairing contracts (**C-FpExt /
C-PairingCurve / C-Pairing**, frozen E.B); the E.A ECDLP substrate (frozen E.A). This sub-track
**freezes two new contracts** (C-MovBridge, C-Mov), both serving the cross-track climax and
forward-looking toward **E.W / T.E** (the Track-E writeup + MOV textbook chapter).

---

## Action-frame digest

### E.C.2 ◆ — 2026-06-10
Discovery/flex: E.C.2 ◆ boundary juncture returned still-on-intent on all six confirmation points. C-MovBridge and C-Mov are frozen. The cross-track climax composition is correct: `mov_reduce` composes pairing → C-MovBridge → `gnfs::solve_dl` and the end-to-end KAT recovers the known scalar with `e(Q,R) = e(G,R)^k` verified in F_{47²}*.
Affected: C-MovBridge (frozen E.C.1), C-Mov (frozen E.C.2)
Deferred: yes — ROADMAP Progress-table staleness + E.A/E.B/D.E closeout reconciliation is an inflection-point Opus action owed at the E.C ◆ boundary (PLAN Discoveries log, Notes-for-executors). Not an E.C `@build` work item.
Texture: Bridge-side modulus assertion (not modulus-threading) confirmed non-fragile at toy params. Degenerate-g guard reuses `DescentFailed { stuck_prime: 0 }` (frozen taxonomy, no new variant). E.C sub-track complete; the project's first cross-track ECDLP→DLP composition ships.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **E.C is a compose over frozen pieces — the bridge is the only new design surface (substrate
  finding, confirmed by survey).** The pairing and the solver both exist and are KAT-proven; E.C
  wires them. Writing the bridge + reduction is **internal-continue**. A discovery that the
  pairing output cannot be faithfully encoded for `solve_dl` (e.g. the coefficient form loses
  information) is an **additive-reshard** surfaced at the E.C.2 ◆.

- **The modulus-consistency invariant must hold by construction, not coincidence (item C —
  correctness guard).** `find_irreducible_degree2` (inside `solve_dl_ext`) re-derives the modulus
  independently of the pairing's `IrreducibleModulus`; they coincide for p=47 by luck (`u²+1` tried
  first). A bridge that **omits the consistency assertion** is **internal-continue → corrected** (add
  the guard). A discovery that the toy params *do not* coincide (they do) would be an
  **additive-reshard** (thread the modulus into `solve_dl_ext`). The silent failure mode — wrong
  field, wrong log, no error — is why the guard is mandatory.

- **`solve_dl_ext` is brute-force at k=2 — E.C must NOT wire the NFS pipeline (item B / defocus
  guard).** A `@build` agent that "improves" the k=2 path by wiring the D.E.2 `ExtFactorBase`
  pipeline into the live `solve_dl` call is **defocus** — internal-continue only within the compose
  scope. The brute-force k=2 path is on-intent (principle 4); the NFS pipeline is the crypto-scale
  annotation. Touching `solve_dl_ext`'s algorithm is out of E.C's scope.

- **The `rho → gnfs` edge is one-directional — a `gnfs → rho` coupling is a destructive-HALT
  (architecture guard).** gnfs must stay unaware of rho's `FpExt` (the bridge helper takes a
  `Vec<BigInt>`). A `@build` agent that adds a `rho` dependency to `gnfs/Cargo.toml`, or imports
  `FpExt` into gnfs, creates a dependency cycle and inverts the track layering — **destructive-HALT**:
  stop, surface it. `cargo check --workspace` failing to resolve (a cycle) is the loud signal.

- **No frozen contract is amended — the C2 signature, `SolveDlError`, and the pairing contracts are
  read-only (contract guard).** A `@build` agent that changes the `solve_dl` signature, adds a
  `SolveDlError` variant, or alters a pairing function's signature is a **destructive-HALT** (those
  are frozen, consumed by their own KATs and downstream). The default E.C touches **no** frozen file
  (the bridge composes existing helpers; the modulus guard is bridge-side). The one *possible* frozen
  edit — threading the modulus into `solve_dl_ext` — is an **additive-reshard** decision at the ◆,
  never a silent patch.

- **The end-to-end KAT must recover a KNOWN scalar and verify `e(Q,R)=e(G,R)^k` — not a spot value
  (correctness discipline).** A wrong bridge or reduction can return a plausible-but-wrong `k`; only
  the round-trip pairing identity catches it. The KAT constructs `Q = k·G` for a chosen `k` and
  asserts both `mov_reduce` recovers `k mod ℓ` *and* the pairing identity holds. A KAT that only
  spot-checks a single DL value has an under-specified contract — flag it.

- **The recovered log is `k mod ℓ`, not the full ECDLP scalar (scope clarity).** At the toy fixture
  the relevant subgroup is order ℓ=3; `k mod 3` is the answer *in that subgroup*. A full
  composite-order Pohlig–Hellman lift is a principle-4 annotation. Presenting `k mod ℓ` as "the full
  ECDLP scalar" without the subgroup qualifier is a documentation defect, not a contract break —
  internal-continue → corrected.

- **ROADMAP insertion owed: the E.A/E.B/D.E closeouts and the E.C predecessor state are not in the
  roadmap Discoveries log (static-frame debt — capture candidate).** The roadmap's Progress table
  still shows Track D and Track E "not started / 0 done" (reconciled at the T.G ◆, *before* D and
  E.A/E.B/D.E landed), and the roadmap Discoveries log has no E.A / E.B / D.E closeout entry — those
  live only in the prior PLAN ledgers and the D.E.3 digest. The roadmap's E.C entry (line ~298)
  predates D.E and assumes the k>1 solver folds into E.C; D.E formalised it as a Track-D extension.
  A ROADMAP reconciliation (E.A/E.B/D.E closeout entries + Progress-table update + the modulus-
  consistency invariant as a durable cross-track note) is an **inflection-point Opus action** owed at
  the E.C ◆ boundary — not by an E.C `@build` session.

- **No oracle dependency for correctness (principle-3 / D.E-consistent).** The end-to-end
  `e(Q,R) = e(G,R)^k` round-trip self-checks; a PARI `znlog`/`fflog` cross-check is an **optional
  `#[ignore]` sidecar** (the established pattern — `#[ignore]` gate only). E.C introduces no new live
  oracle.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — E.C, "*the* cross-track bridge… the pedagogical climax"; Contract
  C2 — the anti-stub constraint) and this PLAN before any session. **Note (non-blocking): the ROADMAP
  Progress table is stale** — it shows Track D and Track E "not started / 0 done" (reconciled at the
  T.G ◆, *before* D and E.A/E.B/D.E landed). Track D is complete (D.A→D.W, plus the D.E extension:
  D.E.1 `0d02b77`, D.E.2 `1a21a32`, D.E.3 ◆ `a804a7b`); E.A and E.B are done (per the prior PLAN
  ledgers / the D.E.3 digest). The prior PLAN ledgers + the D.E.3 digest are authoritative. The
  roadmap is rewritten only at sub-track boundaries; the E.A/E.B/D.E reconciliation + the E.C
  insertion is an inflection-point Opus action at the E.C ◆, not by E.C `@build`.
- Read the **templates to mirror**: `gnfs/src/dl/ext/target.rs` (`ExtTarget::from_coeffs` — the
  over-specified `Vec<BigInt>` constructor E.C composes) and `gnfs/src/dl/ext/descent.rs:59`
  (`ext_target_to_bigint` — the base-p encoder; `find_irreducible_degree2` at ~line 370 is the
  modulus the guard checks); `rho/src/pairing/fpext.rs` (`FpExt` + `to_uint_vec`); `rho/src/pairing/
  test_curves.rs` (`pairing_toy()` + the `PAIRING_TOY_*` constants); `rho/src/pairing/{weil,tate}.rs`
  (`weil_pairing` / `reduced_tate` — note the `reduced_tate(Q, P, ell)` argument order for
  non-degeneracy at the toy fixture); `rho/tests/pairing_kat.rs` (the pairing KAT idiom) and
  `gnfs/tests/dl_ext_kat.rs` (the k=2 DL KAT idiom + the `#[ignore]` PARI pattern — the two idioms
  E.C.2's end-to-end MOV KAT mirrors).
- **Register:** E.C is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module `rho/src/pairing/mov.rs`; KAT in `rho/tests/mov_kat.rs`. The gnfs-side
  bridge helper goes in `gnfs/src/dl/ext/target.rs` (next to `ExtTarget`, the default) or
  `descent/solve.rs` (next to `solve_dl`) — ratified at E.C.1. The new `rho → gnfs` edge in
  `rho/Cargo.toml`.
- **Tier routing:** **both E.C.1 and E.C.2 are Opus** (`@build` on Opus) — the ROADMAP flags both
  sessions Opus (the cross-track climax). E.C.2 carries the single `@plan` marker: a ◆-boundary
  juncture (page `@plan-juncture`) ratifying C-MovBridge / C-Mov and confirming the climax composition
  before the sub-track closes. juncture-tier (header) is **opus** — held by lever 3 (cost of design
  error) + lever 4 (correctness-criticality) + the ROADMAP both-Opus flag; the strong lever-5
  end-to-end round-trip KAT would license an opt-down in isolation but does not override the climax's
  cost-of-wrong.
- **Invariants to preserve:** **the `rho → gnfs` edge is one-directional** (no `gnfs → rho`; the
  bridge takes a `Vec<BigInt>`, never an `FpExt`). **No frozen contract is amended** (C2-ext,
  C-ExtTarget, the pairing contracts, the E.A ECDLP substrate are read-only; the default E.C touches
  no frozen file). **The modulus-consistency guard is mandatory** (item C — the silent-wrong-field
  defense). **The base-p encoding convention stays gnfs's** (not re-derived in rho). **`solve_dl` is
  called as a real solver** (the ROADMAP anti-stub constraint). The k=2 path stays **brute-force**
  (item B — the NFS pipeline is a principle-4 crypto-scale annotation, not wired). The reduction
  recovers **`k mod ℓ`** (full composite lift is principle-4). The `Uint<4> → BigInt` step is
  **toy-only** (single-limb p < 2^64; principle-4 boundary). No new live oracle (round-trip
  self-checks).
- **PARI remains a dev-only `#[ignore]` oracle** (Discoveries-log policy) — an optional k=2 DL
  cross-check follows the established `#[test] #[ignore = "PARI not installed…"]` pattern; never on
  the green path.
- **The cross-crate seam (load-bearing for E.C).** E.C adds the **first `rho → gnfs` dependency**.
  `solve_dl` lives in `gnfs`; the pairing + the MOV reduction + the end-to-end KAT live in `rho`. The
  edge is one-directional; `cargo check --workspace` must resolve with no cycle. This is the
  architectural inverse of D.E.3 (which built its target *independently in gnfs* precisely because the
  edge did not yet exist) — E.C is where the cross-track composition finally happens in code.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — E.C is the
  cross-track climax and the first real `rho → gnfs` composition; the conservative halt-at-◆ cadence
  is warranted. The single ◆/`@plan` on E.C.2 is the one that matters (the Opus C-Mov ratification +
  climax-composition check). *(Tradeoff vs default cadence: one extra halt-confirm on a 2-session
  sub-track is cheap insurance on the project's highest-stakes cross-track seam.)*
