<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-E opens (E.A — Pohlig–Hellman)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by an explicit tier election, not by the default
levers.** On the five-lever law E.A reads as the textbook **opt-down-to-Sonnet** case: lever 5 (the
`cargo test --workspace` inner loop is fast and the composite-curve KAT makes a wrong CRT-combine or
prime-power lift directly test-catchable as `k·G ≠ Q`) coincides with lever 3 (low design-error
cost — E.A is a **leaf attack**, freezing no deep cross-track interface like C2) and lever 4
(moderate, KAT-bounded correctness-criticality). The default would therefore opt **down**. It does
**not**, for one reason: **E.A is the Phase-δ / Track-E *entry*** — the first algebraic-ECDLP attack
on a fresh sub-track scaffold — and the operator elected **E.A.2 at Opus** to hold a deliberate
design register at the phase-opening boundary (does the Track-E attack scaffolding start clean: the
composite-curve fixture, the order-factorization entry point, and the `solve_ecdlp_composite`
contract that later Track-E attacks may consume?). The juncture-tier follows that election: the ◆
juncture that ratifies C-Pohlig and the Track-E-entry design-statement note runs at **Opus**. *(The
hold is scoped to E.A.2's ◆ close; E.A.1 — the substrate fixture + factor loop — needs no Opus
juncture: it freezes test-/compiler-checkable objects and the default cadence lands it.)*

Last rewrite: **D.W ◆ boundary crossed** (Phase γ / Track D complete — D.A → D.B → D.C → D.W
coherent; NFS-DL landed end-to-end; C2 `solve_dl` frozen at 9d07c51; D.W.1 code-tour 8c92260, D.W.2
maths chapter + L-notation payoff 541be29; ledger reconciled 2026-06-09). This plan opens **Phase δ
(Algebraic ECDLP attacks, Track E)** at its dependency-clean entry, **E.A — Pohlig–Hellman**: the
reduction of a composite-order ECDLP to prime-order subgroups via CRT, calling the **existing**
Pollard-rho solvers (which assume a *prime* group order) once per prime-power subgroup. Predecessors
(`rho` crate ECDLP solvers, `shared::numth` order-factoring primitives) are landed and KAT-verified.

---

## Purpose (design intent)

Per ROADMAP (Phase δ): "**E.A — Pohlig–Hellman.** 2 sessions. Predecessor: existing `rho` crate,
S0.2 (factoring of group orders). The cleanest sub-rho attack: reduction to prime-order subgroups
via CRT. Sonnet." E.A is the first *algebraic* (structure-exploiting) ECDLP attack in the project:
where Track-rho solves a prime-order DLP by generic collision search, Pohlig–Hellman exploits the
**factorization of the group order** — it decomposes the DLP in a composite-order group into
independent DLPs in each prime-power subgroup, solves each (the prime-order ones by handing off to
the existing rho solvers), and reassembles the answer by CRT. The structure-based-escape-from-search
through-line: *a smooth group order is a structural weakness*; Pohlig–Hellman is the attack that
converts that structure into a √(largest-prime-factor) cost instead of √n.

The substrate survey established the shape precisely:

1. **The existing rho solvers assume a prime order.** `solve_brent` / `solve_dp` / `solve_dp_negmap`
   / `solve_dp_batch` / `solve_dp_glv` (`rho/src/ecdlp/mod.rs`) all take `n: u64` documented as a
   **prime** group order; `inv_mod_prime` (Fermat) silently misbehaves on composite `n`. E.A is
   exactly the missing composite→prime-subgroup layer **on top of** them — it does not modify them;
   it calls them with prime subgroup orders.
2. **No composite-order test curve exists.** All five fixtures (`tiny_a`, `tiny_b`, `tiny_glv`,
   `secp_k1_toy`, `generic_curve`) have **prime** order. E.A needs a curve whose generator has a
   **smooth composite** order to exercise the reduction (load-bearing for the KAT).
3. **No full-factorization entry point exists.** `shared::numth` gives `is_prime`, `ecm_factor`
   (one factor), `trial_smooth` (factor over a supplied base) — but no `factor_fully`. E.A writes
   the order-factorization loop (`lib.rs` already names this as the anticipated E.A use).
4. **`shared-numth` is not yet a `rho` dependency** — a one-line `Cargo.toml` addition.

The work splits at the **substrate→algorithm seam**:

1. **E.A.1 — composite-order substrate (Sonnet, Cat A).** The composite-order test-curve fixture
   (`test_curves.rs`), the order-factorization loop `factor_order(n) -> Vec<(u64,u32)>`
   (`rho/src/ecdlp/pohlig.rs`), the `shared-numth` Cargo wiring, and the prime-subgroup **projection**
   primitive (given prime power `pᵉ | n`, map `(G, Q)` to the order-`pᵉ` subgroup via
   `[n/pᵉ]`-multiplication). **Freezes** C-CompositeCurve and C-FactorOrder — the fixture and the
   factor-order interface E.A.2 consumes.

2. **E.A.2 ◆ — Pohlig–Hellman proper (Opus, Cat B, `@plan`).** The full reduction: the **prime-power
   lift** for `e>1` (the digit-by-digit base-`p` Pohlig–Hellman recursion, solving each digit by a
   rho call in the order-`p` subgroup) and the **CRT combine** across distinct primes, exposed as
   `solve_ecdlp_composite(curve, g, q, n) -> Option<u64>`; the **composite-curve end-to-end KAT**;
   and the **Track-E-entry design-statement note** (principles 1/3/4 for the attack scaffold).
   **Freezes** C-Pohlig. Crosses the **E.A ◆ boundary** (sub-track complete).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing *Baby-step/Giant-step* as a
"better" per-subgroup solver — that is a different attack, not E.A, whose per-subgroup solver is the
*existing rho*; or generalising the factor loop toward NFS-grade factoring — `trial_smooth` over a
√n base is sufficient and correct for u64 orders) and **rigidity** (re-implementing modular inverse /
CRT from scratch when the codebase already carries modular primitives; or modifying the rho solvers
to "handle composite order" — they must stay prime-order, E.A wraps them).

**Scoping discipline.** E.A solves the DLP on **toy composite-order curves** (smooth order, small
primes) at demonstration fidelity (principle 4). It introduces **no oracle dependency** (the
composite DLP is self-checking: `k·G == Q`). The per-subgroup solver is the **frozen rho substrate** —
E.A adds the reduction layer, nothing inside rho. The factor loop is **toy-order-scoped** (u64 group
orders, `trial_smooth`/`is_prime`); ECM (`ecm_factor`) is available as a fallback but is overkill at
toy scale — a principle-4 annotation, not a work item.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from D.C/D.W). `/run-plan` re-discovers at preflight. E.A **adds code and a KAT** (unlike
the D.W writeup), so the gate is a **correctness gate**, not merely a regression guard: the E.A.2
composite-curve KAT (`k·G == Q` on a curve whose order factors as ∏ pᵢ^{eᵢ}) is the primary
correctness signal, and lever 5 — that this signal is fast and decisive — is what made the Sonnet
opt-down *available* (declined at the ◆ tier election, taken for E.A.1).

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| E.A.1 | Composite-order test curve + `factor_order` decomposition + prime-subgroup projection | A | Sonnet | rho `Curve`/`scalar_mul`/`generator` (frozen, read), `shared::numth` `is_prime`/`trial_smooth` (frozen, read), existing prime-order curve fixtures (template) | `rho/src/curve/test_curves.rs` (add composite fixture), `rho/src/ecdlp/pohlig.rs` (new: `factor_order`, projection), `rho/src/ecdlp/mod.rs` (`pub mod pohlig;`), `rho/Cargo.toml` (+`shared-numth` dep) |
| E.A.2 ◆ `@plan` | Pohlig–Hellman prime-power lift + CRT combine + composite-order ECDLP KAT | B | **Opus** | C-CompositeCurve + C-FactorOrder (frozen E.A.1), rho solvers (`solve_brent`/`solve_dp`, frozen read), existing ecdlp KAT (template) | `rho/src/ecdlp/pohlig.rs` (`solve_ecdlp_composite`, lift, CRT), `rho/tests/ecdlp_kat.rs` (composite-order KAT) |

**Sequencing notes.** Strictly serial: **E.A.1 → E.A.2.** E.A.1 lands the substrate (fixture +
factor loop + projection) that E.A.2's reduction stands on; E.A.2 consumes both frozen contracts.
The single `@plan` marker sits on **E.A.2 ◆** — the Opus boundary juncture that ratifies C-Pohlig and
the Track-E-entry design-statement note before the sub-track is declared closed. E.A.1 carries **no**
`@plan` (it freezes test-/compiler-checkable objects; default cadence lands it).

**Why 2 sessions (matches the ROADMAP allotment — split taken at the contract-sharp seam).** The
ROADMAP allots E.A = 2 sessions. The split is the substrate→algorithm seam, taken up-front:
- **One-line-commit-title corollary.** "Composite-order test curve + `factor_order` + projection" and
  "Pohlig–Hellman lift + CRT + KAT" are **two distinct commit titles** — and two categories (A vs B):
  E.A.1 builds the *substrate the reduction consumes*, E.A.2 is the *reduction*.
- **Contract-sharp boundary (legitimate, not LOC-driven).** E.A.1 **freezes** C-CompositeCurve (the
  fixture) and C-FactorOrder (the decomposition interface); E.A.2 **consumes** both. A real
  produce/consume seam.
- **Irreducible unit kept whole (lever 2).** The prime-power lift and the CRT combine are the *same*
  coherent algorithm ("reassemble subgroup logs into the full log") — fracturing them into separate
  sessions would split an irreducible unit at a non-contract-sharp boundary, forbidden. E.A.2 holds
  the lift *and* the CRT *and* its KAT whole.
- **Tier seam.** E.A.1 is Sonnet (mechanical substrate against the established hand-computed-curve
  template); E.A.2 is **Opus** (operator-elected: the Phase-δ-entry design register on the freshly
  scaffolded attack — see header). The tier seam reinforces the session seam.

They are **not** further splittable: separating the composite fixture into its own pre-session
(3-session option) is unwarranted — the fixture follows the frozen hand-computed point-counting
template (`test_curves.rs` §1–§7: brute-force enumeration + `n·G=∞` verification), a bounded Sonnet
task, not the heavier offline search that would justify isolating it.

---

## Session detail

E.A.1 is crisply specified (it mirrors the frozen prime-order fixture template + a standard u64
factorization loop). E.A.2 is specified at near-full fidelity (Pohlig–Hellman is a textbook
algorithm), with the one open judgment (the composite-order parameter choice the KAT exercises, and
the design-statement depth) flagged for the ◆ juncture.

### E.A.1 — Composite-order substrate (Sonnet, Cat A)

**Deliverable:** the substrate E.A.2's reduction consumes, in three pieces.
- **Composite-order test curve** (`test_curves.rs`, mirroring the `tiny_a`/`tiny_b` pattern §1–§7):
  a short-Weierstrass curve over a small prime field whose generator has **full composite** order
  `n = ∏ pᵢ^{eᵢ}` with small primes (e.g. `n = 2³·3·5·7·…`, smooth, with at least one repeated prime
  to exercise the `e>1` lift). Verified by `n·G = ∞`, the generator being a **full-order** point
  (subtlety below), and the factorization recorded as a checkable constant. Computed offline by
  brute-force point-counting + `n·G=∞` (the established method, `test_curves.rs` header).
- **`factor_order(n: u64) -> Vec<(u64, u32)>`** (`rho/src/ecdlp/pohlig.rs`): the prime-power
  decomposition loop `shared::numth` lacks — trial-division via `trial_smooth` over
  `factor_base_up_to(isqrt(n))` with an `is_prime` short-circuit on the cofactor. Toy-order-scoped
  (u64); ECM-fallback noted, not wired.
- **Prime-subgroup projection** (`pohlig.rs`): given prime power `pᵉ | n`, map `(G, Q)` to the
  order-`pᵉ` subgroup by `[n/pᵉ]`-scalar-multiplication (`curve.scalar_mul`), returning the subgroup
  generator/target. The primitive the lift and CRT both build on.
- **Cargo wiring:** add `shared-numth = { path = "../shared/numth" }` to `rho/Cargo.toml`; declare
  `pub mod pohlig;` in `rho/src/ecdlp/mod.rs`.

Consumes the frozen rho curve interface (`Curve`, `scalar_mul`, `generator`) and `shared::numth`
(`is_prime`, `trial_smooth`, `factor_base_up_to`) read-only. **Freezes C-CompositeCurve and
C-FactorOrder.**

**KAT:** `factor_order` correctness (a unit KAT: `factor_order(n)` reproduces the recorded
factorization of the composite fixture's `n`, and `∏ pᵢ^{eᵢ} == n`); the fixture's `n·G = ∞` and
full-order checks (mirroring the existing `*_n_times_g_is_infinity` tests). **Verify gate:**
`cargo test --workspace` green.

**Subtlety (load-bearing):** the fixture generator must have order **exactly the full composite `n`**,
not a proper divisor — a subgroup generator would make the CRT reduction vacuous and the KAT pass
trivially. Verify `[n/pᵢ]·G ≠ ∞` for every prime `pᵢ | n` (the standard full-order check). This is
*the* fixture-correctness trap; the session must assert it explicitly.

**Deferred:** the lift + CRT + composite-DLP KAT (E.A.2); any non-rho per-subgroup solver (defocus);
NFS-grade factoring (the toy `trial_smooth` loop suffices).

### E.A.2 ◆ — Pohlig–Hellman proper (Opus, Cat B, `@plan`)

**Deliverable:** the composite-order ECDLP reduction, exposed as
`solve_ecdlp_composite(curve, g, q, n: u64) -> Option<u64>` (`pohlig.rs`), in two algorithmic pieces
over the E.A.1 substrate:
- **Prime-power lift (`e>1`).** For each `pᵉ ‖ n`: recover the order-`p` digit-by-digit base-`p`
  expansion of the subgroup log — at each digit, project to the order-`p` sub-subgroup and call a
  **frozen rho solver** (`solve_brent`/`solve_dp`) on that prime-order DLP, then lift. The `e=1` case
  is one rho call (no lift). This is where the *existing prime-order solver* is consumed.
- **CRT combine.** Reassemble the per-prime-power logs `{xᵢ mod pᵢ^{eᵢ}}` into `k mod n` by the
  Chinese Remainder Theorem.

**Track-E-entry design-statement note** (the phase-opening analogue of G.W §59 / the D.W
verification, at lighter fidelity — a sub-track entry, not a phase ◆): principle 1 (the attack is
the genuine structure-exploiting reduction, not a generic search relabelled); principle 3 (no
engineering optimization crept into the reduction — it composes the frozen rho substrate head-on);
principle 4 (toy composite-order scale; the smooth-order assumption and u64 factor loop annotated as
demonstration-scale, not mathematical, boundaries). Verdict recorded in the action-frame digest.

Consumes C-CompositeCurve + C-FactorOrder (frozen E.A.1), the rho solvers (frozen, read), and the
existing ecdlp KAT (template). **Freezes C-Pohlig.**

**KAT (primary correctness signal):** composite-order end-to-end — on the E.A.1 fixture, for several
target scalars `k_target` (including ones with nontrivial residue in the `e>1` subgroup), compute
`Q = k_target·G`, call `solve_ecdlp_composite`, assert `k·G == Q` (the `check_solver_on_curve`
style — any valid log accepted). At least one case must exercise the `e>1` lift and one the
multi-prime CRT. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) the **prime-power lift** is the subtle algorithmic step — an
off-by-one in the digit recursion or a wrong subgroup projection gives a silently-wrong log the KAT
must catch (hence a fixture with `e>1`); (2) **`inv_mod_prime` is prime-only** — every rho call must
receive a *prime* order, never `pᵉ`; the lift solves in the order-`p` group, not order-`pᵉ`; (3) the
**CRT moduli are the prime *powers*** `pᵢ^{eᵢ}`, pairwise coprime by construction. This is the
**E.A ◆ boundary** — re-read the Purpose intent and verify the Track-E attack scaffold (fixture,
factor loop, reduction, KAT) is coherent and that E.A is a clean leaf entry to Phase δ before
crossing.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the E.A.2 ◆
to confirm: (1) C-Pohlig's `solve_ecdlp_composite` signature is the right composite-order entry for
later Track-E consumers (E.G's rho-baseline re-run may call it); (2) the prime-power lift + CRT are
correct and the KAT exercises both `e>1` and multi-prime; (3) the per-subgroup solver is the *frozen
rho substrate*, unmodified (principle 3); (4) the Track-E-entry design-statement note passes 1/3/4;
(5) nothing in E.A presumes E.B/E.C structure (no premature pairing/MOV scaffolding). One-shot
findings; does not implement. Held at **Opus** per the header (operator-elected phase-entry register).

---

## Cross-session contracts

E.A **freezes three** contracts (substrate over-specifies per the rule — C-FactorOrder and
C-CompositeCurve carry interfaces later Track-E attacks may consume). All rho solver and curve
contracts are **read** (composed), not amended.

### C-CompositeCurve — composite-order test fixture (test-enforced) — *to be frozen at E.A.1*

**Defined in:** E.A.1 (`rho/src/curve/test_curves.rs`). **Consumed by:** E.A.2 (the composite-order
KAT) and any later Track-E attack needing a composite-order curve (E.G rho-baseline, potentially
E.A-adjacent tests). Test-enforced: the fixture carries `n·G = ∞`, full-order (`[n/pᵢ]·G ≠ ∞ ∀ pᵢ`),
and recorded-factorization KATs. **The generator has full composite order `n = ∏ pᵢ^{eᵢ}`** (smooth,
small primes, ≥1 repeated prime for the `e>1` path) — *not* a subgroup generator. *Interface frozen
at E.A.1: `composite_toy() -> Curve`, `COMPOSITE_TOY_N: u64`, `COMPOSITE_TOY_FACTORS: &[(u64,u32)]`
(exact names ratified at E.A.1).*

### C-FactorOrder — order-factorization entry point (compiler- + test-enforced) — *to be frozen at E.A.1*

**Defined in:** E.A.1 (`rho/src/ecdlp/pohlig.rs`). **Consumed by:** E.A.2 (drives the per-prime-power
loop) and potentially E.G (rho-baseline order analysis). Compiler-enforced (signature) + test-enforced
(reproduces the fixture factorization). Signature:
`factor_order(n: u64) -> Vec<(u64, u32)>` — the sorted prime-power decomposition `∏ pᵢ^{eᵢ} = n`.
Toy-order-scoped (u64); the ECM fallback for larger orders is a recorded principle-4 annotation, not
in the frozen surface. *Over-specify note:* returning `Vec<(u64,u32)>` (prime, exponent) rather than a
flat prime list carries the exponent the `e>1` lift needs — included now though a flat list would
suffice for a squarefree-only demo, because the `e>1` path is in E.A.2's scope.

### C-Pohlig — composite-order ECDLP entry (compiler- + test-enforced) — *to be frozen at E.A.2 ◆*

**Defined in:** E.A.2 (`rho/src/ecdlp/pohlig.rs`). **Consumed by:** E.A.2's KAT now; later Track-E
attacks that need a composite-order ECDLP entry (E.G rho-baseline re-run is the named candidate).
Compiler- + test-enforced. Signature:
`solve_ecdlp_composite(curve: &Curve, g: &AffinePoint<F>, q: &AffinePoint<F>, n: u64) -> Option<u64>`
— `Some(k)` with `k·G = Q`, `None` on solver failure. *Frozen at the ◆ juncture* (signature ratified
against the later-consumer fit before crossing).

### Frozen contracts read by E.A (composed, not amended)

E.A composes these; none is touched.
- **The rho ECDLP solvers** — `solve_brent` / `solve_dp` / `solve_dp_negmap` / `solve_dp_batch` /
  `solve_dp_glv` (`rho/src/ecdlp/mod.rs`), each `(curve, g, q, n: u64 [prime], …) -> Option<u64>`.
  E.A calls them with **prime** subgroup orders only. *`inv_mod_prime` is prime-only — a hard
  precondition E.A must honour.*
- **The rho curve interface** — `Curve { p, a, b, n, gx, gy }`, `generator`, `scalar_mul`, `negate`
  (`rho/src/curve/mod.rs`). `Curve.n` is `Uint<4>`; E.A's composite fixture sets it to the composite
  order.
- **`shared::numth`** — `is_prime`, `trial_smooth`, `factor_base_up_to` (`shared/numth/src/`). The
  factorization primitives `factor_order` composes. *(New `rho` → `shared-numth` dep added at E.A.1.)*

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The E.A.2 ◆ `@plan` confirmation is not a separate ledger row (a
paged fork with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| E.A.1 | Composite-order test curve + `factor_order` + prime-subgroup projection | done | 054df65 | C-CompositeCurve, C-FactorOrder (frozen) |
| E.A.2 ◆ | Pohlig–Hellman prime-power lift + CRT combine + composite-order KAT | done | 51fd477 | C-Pohlig (frozen) |

Contracts frozen before this sub-track (read by E.A): all Track-rho ECDLP/curve/field contracts
(existing crate), `shared::numth` C1-family primitives (`is_prime`, `trial_smooth`,
`factor_base_up_to`). This sub-track opens Phase δ over the existing rho substrate; it **freezes
three new contracts** (C-CompositeCurve, C-FactorOrder, C-Pohlig).

---

## Action-frame digest

### E.A.2 ◆ — 2026-06-09
Discovery/flex: E.A.2 ◆ boundary juncture returned still-on-intent; all five @plan confirmation points satisfied (C-Pohlig signature correct for later Track-E consumers, lift+CRT correct with KAT exercising e>1 and multi-prime, rho substrate frozen unmodified, design-statement note passes 1/3/4, no E.B/E.C scaffolding).
Affected: none — C-Pohlig frozen as specified; no contracts flexed.
Deferred: no — sub-track complete; E.A is a clean leaf entry to Phase δ. `solve_small_dlog` helper (brute-force for p≤64) noted as pragmatic guard against rho degeneration on tiny groups; reconciled against principle 3 (rho substrate itself unmodified).
Texture: Both E.A sessions were clean green runs. The inflection-design fork (pre-E.A.2 dispatch) also returned design-confident. Track-E attack scaffold (composite fixture n=60=2²·3·5, factor_order, project_to_subgroup, solve_ecdlp_composite, 6 composite KATs) is coherent and closed.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **No composite-order test curve exists — E.A.1 must construct one (substrate gap, not a blocker).**
  All five existing fixtures are prime-order. E.A.1 builds the composite fixture by the established
  offline point-counting method. If the offline search proves heavier than the prime-order template
  (it should not — smooth composite orders are *easier* to find than large primes), isolating the
  fixture into its own session is an **additive-reshard**, surfaced at E.A.1. A fixture whose
  generator turns out to have proper-divisor order (the subtlety) is **internal-continue** (fix the
  parameters), not a contract break.

- **No full-factorization entry point in `shared::numth` — E.A.1 writes the loop (anticipated, not a
  surprise).** `lib.rs` already names "factoring composite group orders in Pohlig–Hellman (E.A)" as
  the intended use of the ecm/smooth primitives. Writing `factor_order` on top of them is
  **internal-continue**. A discovery that `trial_smooth` is *insufficient* for a u64 order (it is not,
  with a √n base) would be a real finding — surface it; do not silently reach for ECM.

- **The rho solvers are prime-order-only — E.A composes, never modifies them (principle-3 guard).**
  `inv_mod_prime` (Fermat) is silently wrong on composite `n`. E.A must pass **prime** subgroup
  orders. A "discovery" that a solver fails on a composite order is **not** a solver bug — it is E.A
  calling it wrong (**internal-continue**: fix the projection). A *destructive* edit to a rho solver
  to "accept composite order" is a **destructive-HALT** — Pohlig–Hellman is the composite layer; the
  rho substrate stays prime-order.

- **`solve_ecdlp_composite` is a forward-looking contract (over-specify discipline).** C-Pohlig's
  signature is frozen with later Track-E consumers in mind (E.G's rho-baseline re-run is the named
  candidate). If E.A.2 finds the signature underserves a known consumer, widening it at the ◆ is
  **additive**; a consumer-driven *change* after freeze is an **additive-reshard** surfaced at the
  next inflection.

- **E.A is a leaf entry — it must not presume E.B/E.C structure (defocus guard).** No pairing
  (E.B) or MOV (E.C) scaffolding belongs in E.A. Writing toward the MOV bridge here is **defocus** —
  internal-continue only within the Pohlig–Hellman reduction scope. (The `@plan` juncture checks this
  explicitly.)

- **Phase-δ entry, not a phase ◆ — lighter design-statement register.** E.A.2 ◆ closes a *sub-track*,
  not a phase. The design-statement note is the *attack-scaffold* check (1/3/4 for the reduction),
  not the whole-phase verification a Track-E ◆ (E.W) will carry. The Opus juncture is operator-elected
  for the phase-*entry* register, not because E.A is correctness-critical at the C2 level.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase δ — "E.A — Pohlig–Hellman"; Contract C1 — the `shared::numth`
  smoothness family E.A's factor loop composes; the Sequencing note "γ before δ" — NFS-DL is now real,
  so Track E may proceed) and this PLAN before any session.
- Read the **templates to mirror**: `rho/src/curve/test_curves.rs` (the hand-computed prime-order
  fixture pattern — E.A.1's composite fixture model, incl. the `n·G=∞` / full-order verification
  idiom) and `rho/tests/ecdlp_kat.rs` (`check_solver_on_curve` — E.A.2's KAT model, `k·G==Q`
  assertion style). Read the **substrate E.A composes**: `rho/src/ecdlp/mod.rs` (the five prime-order
  solvers + `inv_mod_prime`'s prime precondition), `rho/src/curve/mod.rs` (`Curve`, `scalar_mul`,
  `generator`), `shared/numth/src/{prime,smooth,ecm}.rs` (`is_prime`, `trial_smooth`,
  `factor_base_up_to`, `ecm_factor`).
- **Register:** E.A is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module `rho/src/ecdlp/pohlig.rs`. KAT in `rho/tests/ecdlp_kat.rs`.
- **Tier routing:** **E.A.1 is Sonnet** (substrate against the frozen fixture template — mechanical).
  **E.A.2 is Opus** (`@build` on Opus) — operator-elected phase-δ-entry design register on the freshly
  scaffolded attack. E.A.2 carries the single `@plan` marker: a ◆-boundary juncture (page
  `@plan-juncture`) ratifying C-Pohlig and the Track-E-entry design-statement note before the
  sub-track is closed. juncture-tier (header) is **opus** on the same election.
- **Invariants to preserve:** **the rho solvers and curve interface are frozen — E.A composes them;
  it amends none.** The per-subgroup solver is the *existing rho* (a destructive edit to make it
  accept composite order is a destructive-HALT). Every rho call receives a **prime** order. The factor
  loop is toy-order-scoped (u64); ECM is a documented fallback, not wired. No oracle dependency (the
  composite DLP self-checks).
- **PARI / CADO remain dev-only oracles** (Discoveries-log policy) — E.A introduces no new oracle;
  the composite DLP needs none (`k·G==Q` is self-validating).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — this is the **first
  Track-E shard** (an unproven sub-track pattern for the new phase), so the conservative
  halt-at-every-◆ cadence is warranted for the entry; the single ◆/`@plan` on E.A.2 is the one that
  matters (the Opus C-Pohlig ratification + phase-entry design-statement note). *(Tradeoff vs default
  cadence: one extra halt-confirm on a 2-session sub-track is cheap insurance on the phase-opening
  shard; drop to default cadence for E.B once the Track-E pattern is proven.)*
