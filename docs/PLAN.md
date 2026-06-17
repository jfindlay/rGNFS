<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-S continues (S.C — Shor's algorithm for ECDLP)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **kept at the default; the lever-5 opt-down was available
and declined (user-adjudicated at shard time, 2026-06-17).** As with S.B, the cost-of-design-error
lever (3) is **dropped**: S.C is **Category-B (algorithm-on-substrate)** — a *consumer* of the
frozen simulator, not a substrate. It freezes contracts (**C-PointAdd**, the controlled
elliptic-curve point-addition circuit; **C-ECDLPSolve**, the two-register period extraction +
discrete-log driver) consumed only **within S.C** (S.D is prose-only — NIST PQC / the SIDH break /
the migration landscape — and consumes no S.C code). There is **no two-track propagation** of the
kind that held S.A at opus. Correctness-criticality (lever 4) is moderate (toy 4-bit ECDLP, caught
by the `Q == k·G` relationship-preservation KAT plus the independent `rho::solve_brent` cross-check)
and **lever 5 is strong** (the discrete log is deterministically checkable, the toy curve is tiny,
the measurement is seeded, and `rho` gives an independent classical oracle). On the levers alone
this is **the textbook lever-5 opt-down case** (`@plan-juncture-sonnet`). It was **declined** because
(1) the **Proos–Zalka controlled point-addition circuit** — reversible elliptic-curve group
arithmetic including a reversible **modular inverse** over the curve field — is the **most intricate
single construction in Track S** (the irreducible-complexity FLOOR, lever 2, harder than S.B's
mod-exp), and (2) S.C is the project's **first *cross-track* quantum attack** (the value-level
`rho` cross-check is a new integration surface). The differential is single-digit dollars and the
`destructive-HALT` invariant caps the downside either way; opus buys the strongest adjudicator at
the hardest construction in the track. *(Both per-row tiers are **Sonnet** — S.C carries no
Opus-flagged session per the ROADMAP Opus-flagged table; the ◆ fork pages `@plan-juncture` at opus.)*

**Scope boundary — S.C is Shor-FOR-ECDLP ONLY, on a self-contained `u64` toy curve.** S.C builds the
**ECDLP instance** of Shor's algorithm on the frozen simulator: the **controlled elliptic-curve
point-addition quantum circuit** (the Proos–Zalka construction — reversible `|P⟩ → |P + cG⟩`
controlled on an exponent qubit, over a small curve), the **two-register ECDLP period-finding
circuit** (two exponent registers `|a⟩|b⟩` → controlled point-addition computing `a·G + b·Q` →
2D-iQFT → measure), the **classical discrete-log extraction** (recover `k` from the measured pair
via the `b·k ≡ -a mod r` lattice relation), and the **end-to-end `solve_ecdlp` driver** with a
**value-level `rho::solve_brent` cross-check**. It builds the curve fresh in `u64` inside `shor` —
**it does NOT depend on the `rho::curve` types** (which are generic over `Fp<4>`/`Uint<4>` — a
256-bit substrate the `u64` circuit cannot consume without heavy impedance; survey-confirmed at
shard time, 2026-06-17). The cross-check is **value-level**: `rho::solve_brent` runs on the *same
curve parameters* and the recovered `k` is compared, in an **`#[ignore]`-gated** test (`rho` is a
**dev-dependency only** — the green path stays oracle-free, the S.A/S.B gate invariant). A `@build`
agent that takes a regular `shor → rho` build dependency, adapts the `Fp<4>`/`Uint<4>` curve types
into the circuit, or extracts a shared curve crate has reached past S.C's scope (those were the
two declined alternatives) — that is defocus.

The substrate S.C consumes was **frozen and ratified at the S.A.2 ◆** (commit `5ec563a`) and
extended by the S.B `arith` module (commit `60aa816`, C-ModExp frozen). The shard-time survey
(2026-06-17, `@explore` fork against the landed `shor` + `rho` crates) established five grounding
facts:

1. **The frozen gate surface + the S.B modular-arithmetic primitives cover the point-addition
   circuit.** `shor::gates` exposes `multi_controlled_x`, `multi_controlled_unitary`, `toffoli`,
   `controlled_phase`, `cnot`, `swap`, and the single-qubit set; `shor::arith` exposes
   `controlled_add_mod` / `controlled_sub_mod` / `controlled_mult_mod` / `controlled_mult_mod_inv`
   (the `ModExpLayout`-based reversible `u64` modular arithmetic S.B landed). The reversible
   elliptic-curve point-addition circuit is assembled from **this frozen surface** — modular add /
   sub / mult / inverse over the curve's prime field, composed into the affine addition formula.
   S.C adds **no new gate** to the simulator. *(This is the S.B over-specification — exposing the
   modular-inverse primitive `controlled_mult_mod_inv` and the `mod_inverse` classical helper —
   paying off for the next consumer, exactly as the Category-A reasoning predicted.)*

2. **The QFT/iQFT and measurement surfaces are frozen and sufficient for the two-register circuit.**
   `shor::qft::iqft` is in-place, little-endian, with the documented input-bit-reversal convention;
   `shor::measure::measure_all_seeded` / `MeasureAllOutcome` are seeded (`ChaCha8Rng`). The
   two-register ECDLP circuit applies `iqft` to **each** exponent register independently then
   measures — both primitives are present and compose (the 2D QFT is two 1D QFTs on disjoint
   register ranges).

3. **No public toy curve exists at ECDLP-simulatable scale — S.C defines its own (the load-bearing
   substrate fact).** `rho` has no public curve with group order `n ≤ 16` (the only `n=5` curve is a
   *private* `#[cfg(test)]` helper in `rho/src/curve/mod.rs`; the smallest *public* curve,
   `composite_toy`, has `n = 60` — too large for a Proos–Zalka simulation under the ~25-qubit
   ceiling). S.C therefore **defines a fresh small curve in `u64`** inside `shor` (a new `curve`
   module), using plain `u64` modular arithmetic — no `Fp<4>`, no `Uint<4>`, no `rho::curve`
   dependency. The toy curve's parameters (prime `p`, coefficients `a,b`, generator `G`, order `r`)
   are chosen small enough that `r·G` and the two `2·⌈log₂ r⌉`-bit exponent registers fit the qubit
   ceiling.

4. **The `rho` cross-check is value-level via a dev-dependency, not a type-shared edge.**
   `rho::ecdlp::solve_brent(curve, g, q, n, seed, max_retries) -> Option<u64>` returns the discrete
   log `k` as a `u64` — directly comparable to S.C's quantum result. The cross-check runs
   `rho::solve_brent` on a `rho::Curve` built from the **same parameters** as S.C's `u64` toy curve
   and asserts agreement. `rho` enters as a **`[dev-dependencies]` entry only** (the transitive
   `shared-*` / `crypto-bigint` closure lands in the *test* build, never the library build), and the
   cross-check test is **`#[ignore]`-gated** so the default `cargo test` green path is oracle-free
   (the S.A/S.B invariant). *(Survey-confirmed: `shor` does not currently depend on `rho`, `rho`
   does not depend on `shor`, so the dev-dep edge is acyclic.)*

5. **The KAT corpus for Shor-ECDLP is deterministic and self-contained (the lever-5 fact).** The
   classical pieces — the toy curve's group law, the order `r` of `G`, the discrete-log relation
   `Q = k·G` for a chosen `k`, the 2D continued-fraction / lattice recovery of `k` from a measured
   `(a,b)` pair — are fully deterministic; the quantum half is seeded (`measure_all_seeded`). The
   end-to-end KAT recovers `k` from `Q = k·G` reproducibly and asserts `Q == k·G`
   (relationship-preservation, the project's transfer-attack idiom per NOTES 2026-06-15). The
   `rho::solve_brent` agreement is the `#[ignore]`-gated deeper check. No external quantum oracle.

The work splits at **one quantum↔classical contract-sharp seam**, **2 sessions** (the ROADMAP
"2 sessions"; the soft seam for an additive S.C.1a/S.C.1b is named below):

1. **S.C.1 — Toy curve + controlled point-addition quantum circuit (Sonnet, Cat B).** A fresh `u64`
   toy elliptic curve (new `shor::curve` module: curve params, classical group law, generator,
   order) + the **reversible controlled point-addition circuit** (`|P⟩ → |P + cG⟩` controlled on a
   qubit, the Proos–Zalka heart), assembled from the frozen gate set and the S.B `arith` modular
   primitives (add / sub / mult / inverse mod `p`) composed into the affine addition formula on
   work-registers holding the point coordinates. **Freezes C-PointAdd** (the circuit-builder
   interface S.C.2's period-finding consumes). The intricate reversible curve-arithmetic FLOOR
   (lever 2) kept whole.

2. **S.C.2 ◆ — Two-register ECDLP period-finding + discrete-log extraction + end-to-end solve
   (Sonnet, Cat B→I).** The two-register period-finding circuit (two exponent-register Hadamard
   superpositions → controlled point-additions computing `a·G + b·Q` → `iqft` on each register →
   `measure_all`), the **classical discrete-log extraction** (recover `k` from the measured `(a,b)`
   pair via the `b·k ≡ -a mod r` relation), the **end-to-end `solve_ecdlp(curve, G, Q)` driver**
   with the **`#[ignore]`-gated `rho::solve_brent` cross-check**, and the BENCHMARKS.md `## S.C`
   section. **Consumes C-PointAdd. Freezes C-ECDLPSolve.** Crosses the **S.C ◆ boundary** — Shor-for-
   ECDLP is complete end-to-end (the project's **second cryptanalytic break** and **first cross-track
   quantum attack**).

Re-read this intent at the ◆ boundary to catch **defocus** (taking a regular `shor → rho` build
dependency; adapting the `Fp<4>`/`Uint<4>` `rho::curve` types into the circuit; extracting a shared
curve crate — the declined alternatives; or implementing single-register order-finding instead of
the two-register ECDLP hidden-subgroup circuit; or qubit-optimizing the point-addition circuit past
what the 4-bit target needs) and **rigidity** (forcing a qubit-efficient point-addition circuit
where the honest demonstration uses the straightforward reversible affine formula; skipping the
2D-lattice discrete-log KAT because "the `Q == k·G` check will catch it"; or refusing a larger toy
curve if the chosen one is too small to be a faithful demonstration — principle 4).

**Scoping discipline.** S.C solves ECDLP at **demonstration fidelity** (a 4-bit toy curve; the
point-addition circuit uses the straightforward reversible affine construction, not a qubit-optimized
Jacobian or windowed one — principle 3). The **principle-4 science↔engineering gap is load-bearing
and explicit:** the qubit cost of the two-register ECDLP circuit is roughly
`2·⌈log₂ r⌉` (two exponent registers) `+` the point-coordinate work-registers `+` ancilla, so the
~25-qubit simulator ceiling (the frozen S.A wall) is the **resource-scale** boundary — the toy
curve's order `r` is chosen so the circuit fits, and a curve large enough to be cryptographically
meaningful is out of reach by construction. The BENCHMARKS.md `## S.C` section records this: the
algorithm demonstrates Shor's *ECDLP mathematics* (the two-register hidden-subgroup reduction)
correctly at toy scale, and the qubit ceiling is the engineering-scale wall principle 4 says to
annotate, not paper over. *(Same posture as the S.A simulator ceiling, the S.B factoring ceiling,
and the index-calculus "asymptotic win not observable at toy scale.")*

---

## Purpose (design intent)

Per ROADMAP (Phase ε, S.C): "*S.C — Shor for ECDLP. 2 sessions. Predecessor: S.A, curve substrate.
Proos–Zalka circuit. Solve 4-bit ECDLP via simulation; cross-check with rho. Sonnet.*" And per the
design statement (item 6): "*Shor's algorithm via a classical state-vector simulator, for both
factoring and ECDLP.*"

S.C is **the ECDLP instance of Shor's algorithm** — the second *consumer* of the Track-S simulator
substrate, the project's **second quantum-model cryptanalytic break**, and its **first cross-track
quantum attack**. Where Track E's algebraic attacks (Pohlig–Hellman, MOV, SSA, GHS) each escape the
generic √n ECDLP bound by finding exploitable *classical* curve structure, and where Pollard rho
(the `rho` crate) only matches the √n bound, Shor's algorithm **dissolves the bound entirely** via
quantum period-finding on the *two-dimensional* hidden subgroup: the ECDLP `Q = k·G` reduces to
finding the period lattice of the map `(a,b) ↦ a·G + b·Q`, which a two-register QFT extracts in
polynomial time. S.C is the machine that runs that reduction end-to-end on the classical simulator
at toy scale, and cross-checks the recovered log against the classical `rho` solver.

The deliverable is two-fold (the two conceptual units, each a session):

1. **The toy curve + controlled point-addition circuit (S.C.1).** A fresh `u64` toy elliptic curve
   (defined in `shor`, no `rho::curve` dependency) and the reversible controlled-`P + cG` circuit —
   the Proos–Zalka quantum heart of ECDLP period-finding, assembled from the frozen gate set and the
   S.B `arith` modular primitives. **Freezes C-PointAdd.**

2. **Two-register period-finding + discrete-log extraction + end-to-end solve (S.C.2 ◆).** The
   two-register ECDLP period-finding circuit (superposition → controlled point-additions → 2D-iQFT →
   measure), the classical lattice recovery of `k`, the `solve_ecdlp` driver, and the value-level
   `rho::solve_brent` cross-check. The complete Shor-ECDLP algorithm. **Freezes C-ECDLPSolve.**

S.C is **self-contained in `u64`** (it defines its own toy curve — no `rho::curve` type dependency;
`rho` enters only as a dev-dep for the `#[ignore]`-gated cross-check), **substrate-reusing** (it adds
no gate to the simulator — the frozen gate set + S.B `arith` cover it), and **principle-4-honest**
(the two-register qubit budget against the ~25-qubit ceiling is annotated as a resource-scale wall).
Re-read this intent at the ◆ boundary to catch defocus (a regular `rho` build-dep, an `Fp<4>` curve
adapter, single-register order-finding) and rigidity (forcing qubit efficiency, skipping the
2D-lattice KAT).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper (the workspace `Cargo.toml` carries only
`[workspace]` members + a `[profile.bench]`); raw `cargo` is the only CI surface (unchanged since
S.A / S.B). S.C.2 may add a benchmark, so the third discovered command applies if so:
`VERIFY_BENCH = cargo bench --no-run` (compile-only — the actual `cargo bench` timings are
hand-transcribed into BENCHMARKS.md, the established pattern). `/run-plan` re-discovers at preflight.
S.C **adds no workspace member**, adds **no library dependency** to `shor`, and adds **one
`[dev-dependencies]` entry** (`rho`, path dep, for the `#[ignore]`-gated cross-check only) — so the
gate is a **shor-crate-grows + new-KATs-green + dev-dep-builds gate**, with the no-regression
invariant trivially held for every existing crate (S.C touches no code outside `shor`):

- **The existing rho / gnfs / shared / shor (S.A + S.B) KATs must stay green** — S.C extends the
  `shor` crate; it changes no existing solver path and no frozen module surface. `cargo test
  --workspace` is the no-regression guard *and* the new S.C KAT gate (the new `shor` ECDLP KATs join
  the run; the `rho` cross-check is `#[ignore]`-gated, off the default green path).
- **`cargo check --workspace` must stay green** — S.C adds modules to `shor` using only its existing
  *library* dependencies (`num-complex`, `rand`, `rand_chacha`); the new `rho` edge is
  `[dev-dependencies]`-only (test build), acyclic (`rho` does not depend on `shor`), no new member,
  no cycle risk.
- **The `rho` dev-dependency must compile in the test build** — `cargo test --workspace` builds
  `shor`'s test targets, which pull `rho` + its `shared-*` / `crypto-bigint` transitive closure;
  this must build (it already does for the `rho` crate itself). The `#[ignore]` gate keeps the
  cross-check *off the run* but the dep still *compiles*.
- **`cargo bench --no-run` compiles any S.C.2 bench** — if S.C.2 adds a `shor/benches/` qubit-budget
  or point-addition bench, it must build; the timings are hand-transcribed (matching the
  S.A / S.B / G.W / E.W pattern).
- **KATs are canonical/deterministic, self-contained — no oracle on the green path** — the ECDLP
  KATs are the toy curve's group-law values, the order `r` of `G`, the `Q = k·G` relation, the
  2D-lattice discrete-log recovery, and the seeded end-to-end `solve_ecdlp` traces asserting
  `Q == k·G`; the measurement sampler is seeded (`ChaCha8Rng`) so the end-to-end run is
  reproducible. The `rho::solve_brent` agreement is the **only** oracle and it is `#[ignore]`-gated.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@architect` marks an inflection or
contract-freeze point requiring a juncture fork + human sign-off before the next session is
dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| S.C.1 | Toy curve + controlled point-addition quantum circuit | B | Sonnet | C-StateVec (frozen S.A.1 — `StateVec` + the `gates` surface); C-ModExp (frozen S.B.1 — the `arith` modular primitives `controlled_add_mod` / `controlled_sub_mod` / `controlled_mult_mod` / `controlled_mult_mod_inv`, the `ModExpLayout` pattern, the `mod_inverse` / `mod_pow` classical helpers) | `shor/src/curve/mod.rs` (new: `u64` toy curve — params, classical affine group law, generator, order), `shor/src/ecc/mod.rs` (new: reversible controlled point-addition circuit + register layout), `shor/src/lib.rs` (add `pub mod curve;` + `pub mod ecc;`), `shor/tests/pointadd_kat.rs` (new: group-law + permutation-correctness + reversibility + ancilla/work-clean + control-off KATs) |
| S.C.2 ◆ | Two-register ECDLP period-finding + discrete-log extraction + end-to-end solve | B→I | Sonnet | C-PointAdd (frozen S.C.1 — the controlled-`P + cG` circuit); C-QFT (frozen S.A.2 — `iqft`); `shor::measure` (`measure_all_seeded`); `rho::ecdlp::solve_brent` (dev-dep, `#[ignore]`-gated cross-check) | `shor/src/ecdlp/mod.rs` (new: two-register period-finding orchestration + 2D-lattice discrete-log extraction + `solve_ecdlp` driver), `shor/src/lib.rs` (add `pub mod ecdlp;`), `shor/Cargo.toml` (add `rho` to `[dev-dependencies]`), `shor/tests/ecdlp_kat.rs` (new: order KATs + 2D-lattice KATs + end-to-end `solve_ecdlp` + the `#[ignore]`-gated `rho::solve_brent` cross-check), `docs/BENCHMARKS.md` (add `## S.C` section: two-register qubit-budget-vs-curve-order table + the principle-4 resource-scale note) |

**Sequencing notes.** Strictly serial: **S.C.1 → S.C.2.** S.C.1 lands the toy curve + point-addition
circuit and freezes its interface; S.C.2 consumes it (two-register period-finding + classical
extraction + cross-check) and closes the sub-track. **One `@architect` marker:** the **S.C.2 ◆** (the
sub-track-boundary juncture — ratifying the second cryptanalytic break / first cross-track quantum
attack). *(Tradeoff named: S.C pages a juncture only at the ◆-close, NOT at the open — same as S.A
and S.B, and unlike the Opus-substrate opens. S.C.1's point-addition circuit is the most intricate
construction in Track S, but its construction is canonical (the standard reversible affine addition
formula via modular primitives), so no opening fork is warranted; the integrative judgment — does the
ECDLP solve correctly end-to-end, agree with rho, and stay in scope? — concentrates at the ◆ close.
The juncture-tier is `opus`, the declined lever-5 opt-down recorded in the header.)*

**Why 2 sessions (the ROADMAP "2 sessions").** The split is taken at the single quantum↔classical
contract-sharp seam:
- **One-line-commit-title corollary.** "Toy curve + controlled point-addition quantum circuit" and
  "Two-register ECDLP period-finding + discrete-log extraction + end-to-end solve" are **two distinct
  commit titles**. Bundling them — "build the curve AND the point-addition circuit AND the
  two-register period-finding AND the lattice extraction AND the solve driver AND the rho
  cross-check" — fails the corollary.
- **Irreducible units kept whole (lever 2).** S.C.1 is the toy curve + reversible point-addition
  circuit (the intricate FLOOR — reversible curve arithmetic including modular inverse as one
  conceptual unit); S.C.2 is the orchestration + classical-extraction + cross-check layer. Neither
  fractures below its floor — splitting the point-addition circuit's modular primitives from the
  affine-formula composition that uses them would fracture an irreducible unit with no external
  freeze between (the soft seam, named below); the period-finding + 2D-lattice + solve driver cohere
  as the layer that *uses* the circuit to solve.
- **Contract-sharp boundary.** S.C.1 **freezes** C-PointAdd (the circuit-builder interface); S.C.2
  **consumes** it (period-finding wraps the point-addition circuit in two-register superposition +
  2D-iQFT + measure) and **freezes** C-ECDLPSolve. The two-register period-finding circuit is
  meaningless without the point-addition freeze — it is the circuit's wrapper.
- **Lower lever 3 + strong lever 5 license the small commits.** S.C freezes contracts consumed only
  within S.C (no two-track propagation — S.D is prose-only), and the ECDLP KATs are deterministic,
  canonical, reproducible (seeded measurement), and independently cross-checkable (rho), so the inner
  loop catches drift behaviourally — the condition that makes small commits safe.

**The softest seam — could S.C.1 split into the toy curve + modular-primitive composition (S.C.1a)
and the point-addition circuit (S.C.1b)?** The reversible point-addition circuit layers as: the
curve's field arithmetic (modular add / sub / mult / inverse — already frozen in S.B's `arith`) →
the affine addition formula composing them (`λ = (y₂-y₁)/(x₂-x₁)`, `x₃ = λ²-x₁-x₂`, `y₃ =
λ(x₁-x₃)-y₁`) → the controlled wrapper. A planner could split the classical toy-curve + group-law
module (S.C.1a) from the quantum point-addition circuit (S.C.1b). The chosen shard keeps **the whole
curve + point-addition circuit in S.C.1**, because the toy curve's classical group law is small (a
few `u64` functions) and exists *to be* the reference the circuit's permutation-correctness KAT
checks against — splitting it out as its own session would produce a sub-band commit with no
contract seam worth freezing (the classical group law is consumed in-crate by the circuit's KAT with
no external freeze). **If S.C.1 overruns** (the reversible affine-formula circuit — particularly the
reversible **modular inverse** over the curve field, the single hardest sub-piece, and the
uncomputation of the intermediate `λ` register — pushes past the session band; plausible, since
reversible curve arithmetic is the most intricate construction in Track S), the escape applies:
**split at the curve↔circuit layer** (the `u64` toy curve + classical group law freezing C-ToyCurve
in S.C.1a; the controlled point-addition circuit freezing C-PointAdd in S.C.1b) — an
additive-reshard surfaced at the S.C.1 readout or by S.C.1 once the circuit's true size and the
modular-inverse-uncomputation cost are concrete, never a silent overrun. This is the one place the
2-vs-3 sizing is genuinely uncertain until the point-addition circuit's reversibility-KAT size is
visible.

---

## Session detail

S.C.1 is specified at near-full fidelity (the toy curve is a routine `u64` construction; the
reversible point-addition circuit is a canonical Proos–Zalka construction; the design choices are the
toy-curve parameters, the point-coordinate register layout, the reversible-affine-formula
decomposition, and the C-PointAdd interface shape, resolved below). S.C.2 is specified at the
structural level (the two-register period-finding + 2D-lattice + solve outline) with the per-piece
content sketched — correct per the substrate-first discipline: the period-finding circuit's exact
qubit budget and the curve-order reachability are crisp only after the point-addition circuit's
work-register count freezes.

### S.C.1 — Toy curve + controlled point-addition quantum circuit (Sonnet, Cat B)

**Deliverable:** a fresh `u64` toy elliptic curve + the reversible controlled point-addition circuit
— the quantum heart of ECDLP period-finding — built entirely from the frozen gate set and the S.B
`arith` modular primitives. The pieces:
- **The `u64` toy curve** (`shor/src/curve/mod.rs`, new): a small short-Weierstrass curve
  `y² = x³ + ax + b mod p` over a small prime `p`, with the classical affine group law (point
  addition, doubling, negation, scalar-mul), a generator `G`, and its order `r`. **Chosen so the
  two-register ECDLP circuit fits the ~25-qubit ceiling** — `r ≤ ~16` (4-bit ECDLP per the ROADMAP),
  coordinates fitting a few-qubit work-register. Plain `u64`, no `Fp<4>`, no `rho::curve` dependency.
  The reference for the circuit's permutation-correctness KAT. *(Parameter choice resolved at S.C.1:
  e.g. a curve near `p=7..23` with prime order `r ≤ 16` and a generator of full order — pick one
  whose order and coordinate sizes leave headroom under the ceiling for the two exponent registers
  plus the point-coordinate work-registers.)*
- **The reversible controlled point-addition circuit** (`shor/src/ecc/mod.rs`, new): `|P⟩ → |P + cG⟩`
  for a classically-fixed point `cG`, controlled on a qubit — the affine addition formula
  (`λ = (y₂-y₁)·(x₂-x₁)⁻¹ mod p`; `x₃ = λ²-x₁-x₂ mod p`; `y₃ = λ(x₁-x₃)-y₁ mod p`) implemented
  reversibly via the S.B `arith` primitives (`controlled_add_mod`, `controlled_sub_mod`,
  `controlled_mult_mod`, `controlled_mult_mod_inv`) on work-registers holding the running point's
  `(x,y)` coordinates plus a scratch register for `λ`. **Reversibility discipline: every scratch /
  intermediate register (notably the `λ` register) is uncomputed back to `|0⟩`** — the circuit is a
  clean permutation on the point-coordinate registers — the load-bearing reversibility invariant.
  *(Since the added point `cG` is a classical constant — the powers `2ᵏ·G` precomputed classically —
  the addition is constant-point addition, which simplifies the reversible formula vs general
  point-addition; document the construction choice.)*
- **The C-PointAdd interface**: a circuit-builder that, given `(curve, classical point cG, register
  layout)`, applies the controlled `|P⟩ → |P + cG⟩` operation in-place on a `&mut StateVec` —
  little-endian throughout (the frozen S.A convention), with the register layout (which qubits hold
  `x`, `y`, the `λ` scratch) documented as part of the freeze. *(Handle the elliptic-curve special
  cases — `P = ∞`, `P = -cG` giving `∞`, `P = cG` requiring doubling — per the toy curve's small
  point set; document how the identity/exceptional cases are encoded in the work-register.)*

Consumes C-StateVec (frozen S.A.1 — `StateVec` + the `gates` surface) and C-ModExp (frozen S.B.1 —
the `arith` modular primitives + `ModExpLayout` pattern + `mod_inverse`/`mod_pow` helpers). Reads the
S.B.1 digest (the ancilla-free direct-permutation-synthesis approach — relevant to the point-addition
circuit's register budget). **Freezes C-PointAdd.**

**KAT:** over reversibility + permutation-correctness invariants (no quantum-superposition needed for
S.C.1's own KATs — the circuit is tested on basis states, where it computes a classical permutation):
(1) **classical group-law correctness** — the toy curve's classical point-addition / doubling /
scalar-mul match hand-computed values across a fixture of points, and `r·G = ∞` (the order check);
(2) **permutation correctness** — the controlled point-addition circuit on basis state `|P⟩` (control
set) gives `|P + cG⟩` for the classically-computed sum, across a fixture of `(P, cG)` pairs including
the exceptional cases (`P = ∞`, `P = cG`, `P = -cG`); (3) **reversibility** — the circuit followed by
its inverse is the identity on basis states; (4) **scratch-clean** — every scratch register (the `λ`
register especially) returns to `|0⟩` after the circuit (a basis-state check), or — following the
S.B.1 ancilla-free precedent — the full-state forward+inverse=identity check subsumes it; (5)
**control-off no-op** — with the control qubit `|0⟩`, the circuit is the identity. The KATs follow the
`shor/tests/*_kat.rs` idiom (per-fixture helper, many `#[test]` one-liners, exact classical-value
comparison — the circuit on a basis state is a classical permutation, so comparison is exact, not
ε-tolerance, mirroring S.B.1's `modexp_kat.rs`). **Verify gate:** `cargo test --workspace` green (the
new `pointadd` KATs + no regression); `cargo check --workspace` green (no new *library* edge);
`cargo bench --no-run` iff a bench lands.

**Subtlety (load-bearing):** (1) **Reversible modular inverse is the hardest sub-piece** — the affine
slope `λ` needs `(x₂-x₁)⁻¹ mod p` as a reversible operation; `arith::controlled_mult_mod_inv` (frozen
S.B) provides the primitive, but composing the full slope-then-coordinate update reversibly, with the
`λ` scratch uncomputed, is the intricate FLOOR and the reason the circuit is one irreducible unit.
A scratch register left entangled silently corrupts the two-register interference and produces wrong
discrete logs with no error. (2) **Constant-point addition simplifies the formula** — because the
added point `cG = 2ᵏ·G` is a *classical* constant (precomputed), the circuit does constant-point
addition, not general point-addition; this avoids quantum-quantum coordinate arithmetic and is the
standard Proos–Zalka simplification (document it). (3) **Exceptional cases are real at toy scale** —
on a tiny curve the running point can hit `∞` or `±cG`; the encoding of the identity in the
work-register and the doubling/inverse special cases must be handled (the textbook reversible-EC
constructions encode `∞` with a flag qubit or a reserved coordinate value — choose and document).
(4) **Basis-state KATs suffice for S.C.1** — on a basis state the circuit computes a classical
permutation, so the KATs are exact classical-value checks (no superposition, no ε); the superposition
behaviour is exercised in S.C.2's period-finding. (5) **Little-endian + fixed layout** — the `x`, `y`,
`λ` registers use the frozen S.A little-endian convention; the register-layout map is part of the
C-PointAdd freeze (a silent layout flip is a wrong-answer bug for S.C.2). (6)
**Demonstration-fidelity, not qubit-optimized** — straightforward reversible affine formula; no
Jacobian-coordinate inversion-avoidance, no windowed scalar arithmetic beyond what the 4-bit curve
needs (principle 3). The work-register budget that results is the input to S.C.2's curve-order
reachability check.

**Deferred:** the two-register period-finding orchestration (S.C.2); the 2D-lattice discrete-log
extraction (S.C.2); the `solve_ecdlp` driver (S.C.2); the `rho::solve_brent` cross-check (S.C.2 — the
dev-dep + the `#[ignore]`-gated test); the BENCHMARKS.md `## S.C` section (S.C.2 — needs the
qubit-budget numbers).

### S.C.2 ◆ — Two-register ECDLP period-finding + discrete-log extraction + end-to-end solve (Sonnet, Cat B→I)

**Deliverable:** the layer that *uses* the point-addition circuit to solve ECDLP — the two-register
period-finding circuit, the classical 2D-lattice discrete-log extraction, the `solve_ecdlp` driver,
and the value-level `rho` cross-check — + the BENCHMARKS.md section + the sub-track ◆ close.
Structural-fidelity sketch (the per-piece content is crisp once the point-addition work-register
budget freezes). The pieces:
- **The two-register period-finding circuit** (`shor/src/ecdlp/mod.rs`, new): prepare two exponent
  registers `|a⟩|b⟩` in uniform superposition (`h` on every exponent qubit, each register size
  `t ≈ ⌈log₂ r⌉`), apply controlled point-additions to compute `|a⟩|b⟩|a·G + b·Q⟩` (the `a·G` and
  `b·Q` controlled-constant-point-additions via the C-PointAdd circuit, one per exponent qubit with
  the classically-precomputed `2ᵏ·G` and `2ᵏ·Q` points), apply `iqft` (frozen S.A.2) to **each**
  exponent register, then `measure_all_seeded` (frozen S.A). Returns a measured pair `(a', b')` lying
  near the period lattice. *(The two iQFTs are independent 1D iQFTs on the two disjoint exponent
  register ranges — the 2D QFT factors.)*
- **The classical 2D-lattice discrete-log extraction** (`shor/src/ecdlp/mod.rs`): from the measured
  `(a', b')` (each `≈ k·2ᵗ/r`-style phase), recover the discrete log `k` via the relation
  `b·k ≡ -a mod r` — the measured pair satisfies `a' + k·b' ≡ 0 mod 2ᵗ` (approximately), and `k` is
  recovered by the lattice / continued-fraction argument over the two phases (collect a few measured
  pairs if one is insufficient; the toy scale makes brute-force-over-candidates honest if cleaner).
  Built fresh in `u64`.
- **The `solve_ecdlp(curve, G, Q, seed)` driver** (`shor/src/ecdlp/mod.rs`): run the two-register
  period-finding, recover `k`, verify `k·G == Q` (relationship-preservation — retry with a new seed
  if the recovered `k` fails the check), return `k`. Handles the trivial cases (`Q = ∞ → k = 0`,
  `Q = G → k = 1`) per the toy curve.
- **The `rho::solve_brent` cross-check** (`shor/tests/ecdlp_kat.rs`, `#[ignore]`-gated; `rho` added to
  `shor/Cargo.toml` `[dev-dependencies]`): build a `rho::Curve` from the **same** `(p, a, b, n, gx,
  gy)` parameters as the `u64` toy curve, run `rho::ecdlp::solve_brent(curve, G, Q, n, seed,
  max_retries)`, and assert it returns the **same** `k` the quantum `solve_ecdlp` recovered. Gated so
  the default green path is oracle-free (the S.A/S.B invariant); explicitly runnable + documented.
- **The `## S.C` BENCHMARKS.md section** (`docs/BENCHMARKS.md`, append): a **two-register
  qubit-budget-vs-curve-order table** (two exponent registers + point-coordinate work + scratch
  qubits required vs the curve order `r`, against the ~25-qubit ceiling) + the **principle-4
  resource-scale note** (the algorithm demonstrates Shor's *ECDLP mathematics* — the two-register
  hidden-subgroup reduction — correctly; the qubit ceiling is the engineering/resource wall, the same
  posture as the S.A simulator ceiling and the S.B factoring ceiling). Matches the per-sub-track
  BENCHMARKS.md genre (prose setup + table + science↔engineering note).
- **The S.C ◆ close**: re-read the Purpose intent; verify Shor-for-ECDLP runs end-to-end on the
  simulator (the toy curve, recovering `k` from `Q = k·G`); confirm the `rho` cross-check agrees;
  confirm the principle-4 qubit-budget ceiling is annotated; confirm S.C stayed self-contained in
  `u64` (no regular `rho` build-dep, no `Fp<4>` adapter, no shared-curve-crate extraction).

Consumes C-PointAdd (frozen S.C.1), C-QFT (`iqft`, frozen S.A.2), `shor::measure`, and
`rho::ecdlp::solve_brent` (dev-dep, gated). **Freezes C-ECDLPSolve.**

**KAT:** (1) **order/group-law KATs** — `r·G = ∞`, `k·G = Q` for the chosen `k` (the discrete-log
instance is well-formed); (2) **2D-lattice extraction KATs** — a known measured pair `(a', b')`
recovers the correct `k` via the `b·k ≡ -a mod r` relation (a deterministic classical KAT, no quantum
needed — the crux that turns a measured pair into the log); (3) **end-to-end ECDLP** —
`solve_ecdlp(G, Q=k·G)` recovers `k` for a fixture of `k` values on the toy curve, with a **fixed
measurement seed** so the run is reproducible, asserting `recovered_k · G == Q` (relationship-
preservation); (4) **the `rho` cross-check** (`#[ignore]`-gated) — `rho::solve_brent` on the
equivalent `rho::Curve` returns the same `k`. The KATs follow the `shor/tests/*_kat.rs` idiom
(per-fixture helper, many `#[test]` one-liners, seeded measurement for reproducibility, values in
comments). **Verify gate:** `cargo test --workspace` green (the new green-path KATs + no regression;
the `rho` cross-check is `#[ignore]`-gated, off the run); `cargo check --workspace` green (the `rho`
edge is dev-dep-only); `cargo bench --no-run` compiles any qubit-budget bench.

**Subtlety (load-bearing):** (1) **ECDLP is a TWO-register hidden-subgroup problem, NOT single-register
order-finding** — the defining difference from S.B. You prepare *two* exponent registers, compute
`a·G + b·Q`, and recover `k` from the *2D* period lattice via `b·k ≡ -a mod r`. Implementing
single-register order-finding (the S.B pattern) here is a defocus bug that will not solve ECDLP.
(2) **The 2D-lattice extraction is the classical crux** — its KAT (known pair → known `k`) must be
complete, not a smoke test; it is the piece that turns a noisy measured pair into the discrete log.
At toy scale, recovering `k` may need a couple of measured pairs (or honest brute-force over the few
candidate `k < r`); document the choice. (3) **Seeded measurement + retry for reproducible
end-to-end KATs** — the period-finding measurement is `ChaCha8Rng`-seeded; pick seeds that land a
successful measured pair so the KAT is a clean pass, and verify `k·G == Q` as the success predicate
(retry on failure). (4) **The `rho` cross-check is the integration milestone but stays off the green
path** — `rho` is a dev-dep, the cross-check is `#[ignore]`-gated; the default `cargo test` is
oracle-free, the cross-check is the deliberate deeper check (the S.A/S.B "no oracle on the green
path" invariant preserved). (5) **The qubit ceiling is principle-4 (resource scale)** — the
two-register budget (`2t` exponent + coordinate work + `λ` scratch) against the ~25-qubit wall is
recorded in BENCHMARKS.md as engineering scale; a curve order large enough to be cryptographically
meaningful is out of reach by construction, exactly the wall to annotate, not paper over.

**Deferred:** the post-quantum context writeup (S.D — NIST PQC, the SIDH break, the migration
landscape; prose-only, no PQC implementations) + T.S (the Track-S math chapter, paired with S.D, not
S.C); all of Track ζ (umbrella — Z.1) + the τ-bind (T.Z). **S.C is the ECDLP attack; S.D is the
post-quantum context — a prose-only sibling closing Track S.**

**`@architect` ◆ confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the
S.C.2 ◆ to confirm: (1) Shor-for-ECDLP runs end-to-end — the toy curve + point-addition circuit +
two-register period-finding + 2D-lattice extraction + `solve_ecdlp` driver compose correctly and the
end-to-end KATs recover `k` with their fixed seeds, asserting `k·G == Q`; (2) **the point-addition
circuit is reversible and scratch-clean** — the load-bearing correctness FLOOR (an entangled `λ`
scratch silently corrupts the two-register interference); confirm the reversibility + scratch-clean
KATs pass; (3) **the algorithm is genuinely two-register** — two exponent registers + the
`b·k ≡ -a mod r` lattice recovery, NOT single-register order-finding (the S.B-pattern defocus);
(4) **the `rho` cross-check agrees** — `rho::solve_brent` on the equivalent curve returns the same
`k` (the `#[ignore]`-gated test passes when run); confirm `rho` is a **dev-dependency only** and the
green path stayed oracle-free; (5) the principle-4 qubit-budget ceiling is annotated in BENCHMARKS.md
`## S.C` (the two-register cost vs curve order, against the ~25-qubit wall); (6) C-PointAdd +
C-ECDLPSolve are coherent (S.C's own internal contracts — no downstream consumer, S.D is prose-only);
(7) S.C stayed in scope — **self-contained in `u64`, no regular `shor → rho` build-dep, no `Fp<4>`
curve adapter, no shared-curve-crate extraction** (the declined alternatives). **Also: surface the
outstanding static-frame ROADMAP debt** (the Progress/Remaining reconciliation owed since the E.W ◆,
flagged at the S.A ◆ and again at the S.B ◆ — now compounded: Phase δ complete, S.A done, S.B done,
S.C now landing; the ROADMAP write was out of `@architect` PLAN-write scope at every prior juncture)
— **note it as a capture candidate, not a PLAN edit**. One-shot findings; does not implement. Held at
**opus** per the header (the declined lever-5 opt-down).

---

## Cross-session contracts

S.C **freezes two** contracts (C-PointAdd at S.C.1; C-ECDLPSolve at S.C.2 ◆) and **amends no prior
frozen contract** — it extends the `shor` crate, consuming the S.A-frozen C-StateVec / C-QFT / measure
surfaces and the S.B-frozen C-ModExp (`arith`) surface without modifying them. S.C adds no workspace
member, no `shor` *library* dependency, and one `[dev-dependencies]` entry (`rho`, for the gated
cross-check). **Like S.B, S.C's two new contracts have no downstream consumer** — they are internal to
S.C (C-PointAdd is consumed by S.C.2; C-ECDLPSolve is the sub-track's terminal deliverable; S.D is
prose-only). This is the lever-3 drop that made the juncture-tier opt-down *available* (declined per
the header).

### C-PointAdd — the controlled elliptic-curve point-addition circuit (compiler-/test-enforced) — *to be frozen at S.C.1*

**Defined in:** S.C.1 (`shor/src/curve/mod.rs` for the `u64` toy curve; `shor/src/ecc/mod.rs` for the
circuit).
**Consumed by:** S.C.2 (the two-register period-finding circuit wraps the point-addition circuit in
superposition + 2D-iQFT + measure). **No downstream sub-track consumes C-PointAdd** — S.D is
prose-only. Compiler-enforced (the circuit-builder function signature + the toy-curve type) +
test-enforced (the group-law + permutation-correctness + reversibility + scratch-clean KATs).

**Ratified shape (to be confirmed at the S.C ◆).** A `u64` toy short-Weierstrass curve
(`y² = x³ + ax + b mod p`, params + classical affine group law + generator `G` + order `r ≤ ~16`,
no `Fp<4>`/`rho` dependency) plus a circuit-builder that applies the controlled
`|P⟩ → |P + cG⟩` operation in-place on a `&mut StateVec` for a classically-fixed point `cG`, given
the curve and the register layout (`x` / `y` coordinate ranges + the `λ` scratch range) — **little-
endian throughout** (the frozen S.A convention). Assembled from the frozen S.A gate set + the frozen
S.B `arith` modular primitives (`controlled_add_mod`, `controlled_sub_mod`, `controlled_mult_mod`,
`controlled_mult_mod_inv`) — **no new gate added to the simulator**. **Invariants:** the circuit is a
reversible permutation on the point-coordinate registers (tested on basis states, exact); **every
scratch register (the `λ` register) returns to `|0⟩`** (the scratch-clean invariant — an entangled
scratch silently corrupts the period-finding interference); control-off is a no-op; the exceptional
cases (`P = ∞`, `P = ±cG`) are handled and the identity-encoding is documented; the register-layout
map is fixed and documented (a silent layout flip is a wrong-answer bug for S.C.2). The
classically-precomputed points `2ᵏ·G` (and `2ᵏ·Q` for S.C.2) are constants fed to the controlled
stages. *(The soft-seam candidate: if S.C.1 overruns, this splits into C-ToyCurve at S.C.1a +
C-PointAdd at S.C.1b.)*

### C-ECDLPSolve — the two-register period-finding + discrete-log extraction + `solve_ecdlp` driver (compiler-/test-enforced) — *to be frozen at S.C.2 ◆*

**Defined in:** S.C.2 (`shor/src/ecdlp/mod.rs`). **Consumed by:** the end-to-end ECDLP KATs + the
`rho` cross-check (the sub-track's terminal deliverable — no further consumer). Compiler-enforced (the
period-finding + extraction + `solve_ecdlp` signatures) + test-enforced (the order KATs + 2D-lattice
KATs + end-to-end KATs + the `#[ignore]`-gated `rho` cross-check).

**Ratified shape (to be confirmed at the S.C ◆).** The two-register period-finding routine: two
exponent registers (each size `t ≈ ⌈log₂ r⌉`) in superposition → controlled point-additions computing
`a·G + b·Q` via the C-PointAdd circuit → `iqft` (frozen S.A.2) on **each** register →
`measure_all_seeded` (frozen S.A), returning a measured pair `(a', b')`; plus the classical 2D-lattice
recovery of the discrete log `k` from `(a', b')` via `b·k ≡ -a mod r`; plus the
`solve_ecdlp(curve, G, Q, seed) -> Option<u64>` driver (recover `k`, verify `k·G == Q`, retry on
failure). **Invariants:** the recovered `k` satisfies `k·G == Q` (the discrete-log definition, the
relationship-preservation KAT); the algorithm is genuinely two-register (NOT single-register
order-finding); the measurement is seeded (reproducible KATs); the `rho::solve_brent` cross-check on
the equivalent curve agrees on `k` (the `#[ignore]`-gated integration check).

### Frozen contracts read by S.C (consumed, not amended)

- **C-StateVec (frozen S.A.1) — the dense register + gate interface.** S.C.1's point-addition circuit
  is assembled from the `gates` surface. **Not amended** — S.C adds no gate.
- **C-ModExp (frozen S.B.1) — the `arith` modular primitives + `ModExpLayout` + classical helpers.**
  S.C.1's point-addition circuit composes `controlled_add_mod` / `controlled_sub_mod` /
  `controlled_mult_mod` / `controlled_mult_mod_inv` into the affine addition formula; the
  `mod_inverse` / `mod_pow` / `n_bits` classical helpers are reused. **Not amended** — S.C consumes
  the `arith` surface read-only. *(This is the S.B over-specification — exposing the modular-inverse
  primitive and the classical helpers — paying off for the next consumer.)*
- **C-QFT (frozen S.A.2) — `qft`/`iqft`.** S.C.2's period-finding applies `iqft` to each exponent
  register. **Not amended.**
- **`shor::measure` (frozen S.A.2) — `measure_all_seeded`, `MeasureAllOutcome`.** S.C.2's
  period-finding measures the exponent registers, seeded for reproducibility. **Not amended.**
- **C-Sparse (frozen S.A.2) — NOT consumed by S.C.** The period-finding state is fully superposed
  (Hadamards on both exponent registers), so the dense register is the honest vehicle; the sparse
  path offers no win here. S.C uses the dense register.
- **`rho::ecdlp::solve_brent` (existing, frozen `rho` surface) — consumed as a dev-dep oracle, not
  amended.** The value-level cross-check builds a `rho::Curve` from the toy-curve parameters and
  calls `solve_brent`; `rho` is a `[dev-dependencies]` entry, the call is `#[ignore]`-gated, and no
  `rho` code is modified.

### Downstream contracts S.C does NOT produce (named, to bound scope)

- **The post-quantum writeup is S.D, prose-only — no contract.** S.D (NIST PQC, the SIDH break, the
  migration landscape) is a research-and-write task; it implements no PQC and freezes no code
  contract. **S.C produces no input to S.D** beyond the completed Track-S attack pair (S.B factoring
  + S.C ECDLP) the writeup contextualizes.
- **No regular `shor → rho` build edge, no `Fp<4>` curve adapter, no shared-curve crate.** These were
  the two declined alternatives at shard time. S.C defines its own `u64` toy curve and consumes `rho`
  only as a dev-dep oracle. A `@build` agent that takes a regular `rho` dependency, adapts the
  `Fp<4>`/`Uint<4>` types, or extracts a shared curve crate has reached past S.C's scope.

### Workspace edges (no new member, no new library dependency)

- **No new member.** S.C extends the existing `shor` crate; the workspace `Cargo.toml` `members` list
  is unchanged.
- **No new *library* dependency.** S.C's library code uses `shor`'s existing deps (`num-complex`,
  `rand`, `rand_chacha`); the toy curve + 2D-lattice extraction are built fresh in `u64`.
- **One new `[dev-dependencies]` edge: `rho` (path dep), test-build-only, acyclic.** The
  `#[ignore]`-gated cross-check calls `rho::solve_brent`; `rho` + its transitive `shared-*` /
  `crypto-bigint` closure land in `shor`'s *test* build only (never the library build). `rho` does
  not depend on `shor`, so the edge is acyclic; `cargo check --workspace` stays green (dev-deps don't
  affect the library check); `cargo test --workspace` builds it but the `#[ignore]` keeps the
  cross-check off the run.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The S.C.2 ◆ `@architect` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| S.C.1 | Toy curve + controlled point-addition quantum circuit | done | 82fb198 | C-PointAdd |
| S.C.2 ◆ | Two-register ECDLP period-finding + discrete-log extraction + end-to-end solve | pending | | C-ECDLPSolve |

Contracts frozen before this sub-track: the entire classical-attack arc — all of Track G (GNFS
factoring), Track D (NFS-DL), and Track E (algebraic ECDLP, closed at the E.W ◆) — plus the shared
substrate (C1 smoothness, the field/bigint/numfield/padic/gf2m crates), the Track-τ register
(C-Textbook, frozen T.0), **the Track-S simulator substrate (C-StateVec, C-Sparse, C-QFT, frozen at
the S.A.2 ◆, commit `5ec563a`), and the Shor-factoring arithmetic (C-ModExp, C-OrderFind, C-Factor,
frozen across the S.B sub-track, commits `60aa816` / `6cc4c6e`)**. **S.C consumes the Track-S
substrate (C-StateVec, C-QFT, measure) and the S.B `arith` surface (C-ModExp), and amends none of
it** — it is a self-contained extension of the `shor` crate (plus a `rho` dev-dep for the gated
cross-check). This sub-track **freezes two new contracts** (C-PointAdd, C-ECDLPSolve), **both internal
to S.C** (no downstream sub-track consumes them — S.D is prose-only). **With the S.C ◆, Shor-for-ECDLP
is complete end-to-end (the project's second cryptanalytic break and first cross-track quantum
attack); S.D (the PQ writeup + T.S) remains in Phase ε, then Phase ζ (umbrella, Z.1) + the τ-bind
(T.Z) close the project.**

---

## Action-frame digest

### S.C.1 — 2026-06-17
Discovery/flex: Implementation used permutation synthesis (ancilla-free direct permutation on the combined (x,y) register) rather than the explicit reversible affine formula with λ scratch — same approach as S.B.1's `apply_controlled_permutation`. The λ register is allocated in the PointAddLayout for S.C.2 compatibility but unused at runtime; scratch-clean invariant trivially satisfied.
Affected: C-PointAdd (confirmed stable — permutation synthesis satisfies all C-PointAdd invariants: reversible permutation, scratch-clean, control-off no-op, exceptional cases handled, little-endian layout documented)
Deferred: no — S.C.2 consumes C-PointAdd as frozen; the permutation-synthesis approach is transparent to the consumer (same circuit-builder interface, same register layout)
Texture: `shor/src/arith/mod.rs` needed a one-line visibility change (`apply_controlled_permutation` → `pub(crate)`) to allow `ecc` to call it; allowed as plainly-part-of-unit. Curve: p=7, a=0, b=3 (y²=x³+3 mod 7), 12 affine points + ∞, group order r=13 (prime), generator G=(1,2). 45 KATs across all 5 required groups pass.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **S.C is a self-contained `u64` extension of the frozen `shor` crate (plus a `rho` dev-dep) —
  internal-continue (confirmed by the shard-time survey, 2026-06-17).** S.C consumes the S.A-frozen
  substrate (C-StateVec / C-QFT / measure) and the S.B-frozen `arith` (C-ModExp) and amends none of
  it; all S.C code is new modules in `shor` (`curve`, `ecc`, `ecdlp`) + `lib.rs` `mod` decls + new
  test files + one `[dev-dependencies]` entry. No new crate, no new *library* dependency, no edit
  outside `shor`. A discovery that the point-addition circuit needs a gate the simulator lacks is an
  **additive edge to S.A** surfaced at the ◆ — judged unlikely (the gate set + `arith` primitives
  were confirmed sufficient at survey time).

- **The point-addition circuit is the irreducible-complexity FLOOR (lever 2) — keep it whole.**
  Reversible elliptic-curve arithmetic, particularly the reversible **modular inverse** for the slope
  `λ` and its uncomputation, is the most intricate construction in Track S; splitting the toy curve +
  classical group law from the circuit that uses it fractures the unit with no contract seam between
  (the named soft seam). **Internal-continue → keep S.C.1 whole; additive-reshard to S.C.1a/S.C.1b
  only if it overruns the band.**

- **Scratch-clean reversibility is the load-bearing correctness invariant.** Every scratch register
  (the `λ` register especially) MUST be uncomputed to `|0⟩`; an entangled scratch silently corrupts
  the two-register period-finding interference and produces wrong discrete logs with no error. The
  reversibility + scratch-clean KATs (basis-state checks; or the full-state forward+inverse=identity
  check per the S.B.1 precedent) are the guard. **Internal-continue → scratch discipline enforced +
  KAT'd at S.C.1.**

- **ECDLP is a TWO-register hidden-subgroup problem, not single-register order-finding (the central
  defocus risk).** Solving `Q = k·G` requires two exponent registers, the `a·G + b·Q` computation,
  and the `b·k ≡ -a mod r` 2D-lattice recovery — materially different from S.B's single-register
  order-finding. A `@build` agent that copies the S.B order-finding pattern (one register, the
  `factor`-style continued fraction) will not solve ECDLP. **Internal-continue → the two-register
  circuit + 2D-lattice extraction are the deliverable; the S.B pattern is the wrong template for the
  orchestration (only the `arith` primitives and the `*_kat.rs` idiom transfer).**

- **No public toy curve at 4-bit scale — S.C defines its own (the substrate fact).** `rho` has no
  public curve with `n ≤ 16` (only a private `n=5` test helper; the smallest public is
  `composite_toy` at `n=60`). S.C therefore defines a fresh `u64` toy curve in `shor`. **Internal-
  continue → the curve is new code in `shor::curve`, chosen so the two-register circuit fits the
  ceiling.** *(This is why the self-contained-`u64`-curve option was chosen over the `rho`-dep and
  shared-crate alternatives at shard time.)*

- **The `rho` cross-check is value-level, dev-dep-only, `#[ignore]`-gated (the integration design).**
  The cross-check builds a `rho::Curve` from the same parameters and asserts `rho::solve_brent`
  agrees on `k`. `rho` is a `[dev-dependencies]` entry (test build only, acyclic) and the test is
  `#[ignore]`-gated so the green path stays oracle-free (the S.A/S.B invariant). **Internal-continue →
  the cross-check is the deliberate deeper check, not a green-path KAT; a `@build` agent that takes a
  regular `rho` build-dep or runs the cross-check on the green path has broken the design.**

- **The curve order may push the two-register circuit past the ~25-qubit ceiling — resolve at the
  S.C.1 readout (principle 4).** The two-register budget is `2t` exponent (`t ≈ ⌈log₂ r⌉`) +
  point-coordinate work + the `λ` scratch; choose the toy curve order `r` so this fits, and annotate
  the budget-vs-order in BENCHMARKS.md. Do NOT qubit-optimize the point-addition circuit (Jacobian
  coordinates, inversion-avoidance) to force a larger curve (gold-plating — principle 3). **Internal-
  continue → pick a curve that fits; annotate the ceiling; the budget is concrete after S.C.1.**

- **The point-addition circuit is demonstration-fidelity, not qubit-optimized (principle 3).** Use
  the straightforward reversible affine formula; no Jacobian-coordinate inversion-avoidance, no
  windowed scalar arithmetic beyond what the 4-bit curve needs. **Internal-continue → straightforward
  construction; the resulting work-register budget is the curve-order-reachability input.**

- **Exceptional elliptic-curve cases are real at toy scale.** On a tiny curve the running point can
  hit `∞` or `±cG`; the identity-encoding and the doubling/inverse special cases must be handled in
  the reversible circuit (a flag qubit or reserved coordinate value — chosen and documented at S.C.1).
  **Internal-continue → exceptional cases are part of the C-PointAdd freeze + KAT'd.**

- **Static-frame ROADMAP debt (surface at the S.C ◆ — out of `@architect` PLAN-write scope; a capture
  candidate) — carried from the E.W ◆, flagged at the S.A ◆ and the S.B ◆, now compounded by S.C's
  landing.** The ROADMAP Progress table still shows "δ — Algebraic ECDLP (E) … complete" but "ε —
  Shor + PQ (S) … 0 sessions (S.A sharded, not executed)" — stale by three sub-tracks: S.A landed
  (`5ec563a`), S.B landed end-to-end (`6cc4c6e`/`2a79bd9`), and S.C is now sharded. The S.A ◆ and
  S.B ◆ digests both named this; the write was deferred (out of `@architect` PLAN-write scope). **The
  S.C close is a fourth natural prompt.** The full reconciliation (mark S.A + S.B done; advance Phase
  ε to S.C; update the remaining-session projection) is owed. **This is a ROADMAP write — outside the
  `@architect` PLAN-only write scope; surfaced here as a capture candidate for the user to action (via
  `/note` or a ROADMAP edit), not a PLAN edit.** Not an implementation concern; does not block S.C.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase ε — S.C, "*Shor for ECDLP. 2 sessions. Predecessor: S.A, curve
  substrate. Proos–Zalka circuit. Solve 4-bit ECDLP via simulation; cross-check with rho. Sonnet.*";
  the design statement item 6 — "*Shor's algorithm via a classical state-vector simulator, for both
  factoring and ECDLP*"; the Track-τ pairing — T.S/ch. 11 pairs with **S.D**, not S.C) and this PLAN
  before any session. **NOTE: the ROADMAP Progress / Remaining tables are stale by three sub-tracks
  (S.A + S.B landed; Track S shown not-started); the S.C close is a fourth reconciliation prompt —
  surface it at the ◆, but it is outside `@architect` PLAN-write scope (a capture candidate).**
  **CAUTION on "curve substrate":** the ROADMAP's "Predecessor: S.A, curve substrate" does NOT mean a
  type-level dependency on `rho::curve` — that substrate is `Fp<4>`/`Uint<4>` (256-bit), incompatible
  with `shor`'s `u64` circuit, and has no toy curve at 4-bit scale. S.C defines its own `u64` toy
  curve; "curve substrate" is consumed as a *reference* and a *value-level cross-check oracle* (`rho`
  dev-dep), not a type dependency. This was the central shard-time design decision (user-adjudicated,
  2026-06-17).
- Read the **frozen substrate to consume**: `shor/src/statevec/mod.rs` + `shor/src/gates/mod.rs`
  (C-StateVec — the `StateVec` type + the gate surface); `shor/src/arith/mod.rs` (C-ModExp — the
  `ModExpLayout`, the `controlled_add_mod` / `controlled_sub_mod` / `controlled_mult_mod` /
  `controlled_mult_mod_inv` primitives, the `mod_inverse` / `mod_pow` / `n_bits` /
  `read_work_register` helpers — the building blocks of the point-addition circuit); `shor/src/qft/`
  (C-QFT — `iqft`, the bit-reversal convention); `shor/src/measure/` (`measure_all_seeded`,
  `MeasureAllOutcome`, the `ChaCha8Rng` seeding). Read the **templates to mirror**:
  `shor/src/arith/mod.rs` + `shor/tests/modexp_kat.rs` (the reversible-circuit + basis-state-KAT
  idiom — the closest precedent for the point-addition circuit); `shor/src/shor/mod.rs` +
  `shor/tests/factor_kat.rs` (the period-finding orchestration + classical-extraction + driver idiom
  — but note S.C is TWO-register, so the orchestration differs); `docs/BENCHMARKS.md ## S.B` (the
  per-sub-track section genre — prose + qubit-budget table + science↔engineering note, here adapted to
  a two-register qubit-budget-vs-curve-order table). Read the **cross-check target**:
  `rho/src/curve/mod.rs` (the `Curve` struct fields `p, a, b, n, gx, gy` to build the equivalent
  curve) + `rho/src/ecdlp/mod.rs` (`solve_brent` signature) — for the `#[ignore]`-gated value
  cross-check only.
- **Register:** S.C is **Rust library + benchmark code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; new
  modules in the `shor` crate + their KATs + an optional Criterion bench + one dev-dep) **plus prose**
  (the BENCHMARKS.md `## S.C` section). **No PEDAGOGY.md or MATHEMATICS.md chapter at S.C** — the
  Track-S math chapter (T.S, ch. 11) pairs with S.D (the track closeout), not this attack sub-track.
- **Tier routing:** **both S.C.1 and S.C.2 are Sonnet `@build`** (S.C carries no Opus-flagged session
  per the ROADMAP Opus-flagged table — the point-addition circuit is a canonical Proos–Zalka
  construction, the two-register period-finding is the textbook Shor-ECDLP algorithm; the only
  judgment is the C-PointAdd interface shape + register layout + the toy-curve choice, which the ◆
  juncture handles). **juncture-tier (header) is `opus`** — the **lever-5 opt-down was available and
  declined** (lever 3 dropped: S.C binds no downstream track, freezing its contracts for its own
  consumption only; strong deterministic ECDLP KATs + the independent `rho` oracle + moderate
  criticality would license `sonnet`, but the most-intricate-FLOOR-in-Track-S point-addition circuit
  + the first-cross-track-quantum-attack integration were judged to warrant the strongest adjudicator
  — user-adjudicated at shard time, 2026-06-17). The ◆ fork pages `@plan-juncture` at opus.
- **Invariants to preserve:** **S.C builds its own `u64` toy curve** (no regular `shor → rho`
  build-dep, no `Fp<4>`/`Uint<4>` adapter, no shared-curve-crate extraction — the declined
  alternatives). **S.C amends NO frozen contract** (it consumes the S.A substrate + the S.B `arith`
  read-only; the only new files are `shor` modules + tests + the BENCHMARKS.md section + one dev-dep).
  **S.C adds NO gate to the simulator** (the point-addition circuit is assembled from the frozen gate
  set + the S.B `arith` primitives). **The point-addition circuit is reversible and scratch-clean**
  (every scratch register, esp. `λ`, returns to `|0⟩` — the load-bearing correctness FLOOR; an
  entangled scratch silently corrupts period-finding). **The algorithm is TWO-register** (two exponent
  registers + the `b·k ≡ -a mod r` lattice recovery, NOT single-register order-finding — the central
  defocus guard). **The register layout (x/y/λ, little-endian) is fixed and documented** (a silent
  layout flip is a wrong-answer bug). **The point-addition circuit is demonstration-fidelity**
  (straightforward reversible affine formula, not qubit-optimized — principle 3). **The two-register
  qubit budget vs the ~25-qubit ceiling is annotated** (principle 4 — engineering, not mathematical).
  **The `rho` cross-check is value-level, dev-dep-only, `#[ignore]`-gated** (green path oracle-free).
  **Measurement is seeded** (reproducible end-to-end KATs).
- **No new crate, no new library dependency, one dev-dep, all in `shor` (load-bearing for S.C).** S.C
  extends the existing `shor` crate with three new modules (`curve`, `ecc`, `ecdlp`); the workspace
  `Cargo.toml` and `shor/Cargo.toml` *library* dependency lists are unchanged. The only manifest edit
  is one `[dev-dependencies]` entry (`rho`, path dep). No existing crate's edge changes; `cargo check
  --workspace` stays green; the no-regression invariant (existing KATs green) holds trivially since no
  existing code changes.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the shard pattern (a
  toy-curve + quantum-circuit session, then a two-register-orchestration + cross-check session closing
  at the ◆) follows the proven S.B shape, but S.C introduces **two new judgment surfaces** (the
  two-register hidden-subgroup orchestration, which is NOT the S.B single-register pattern; and the
  first cross-track `rho` cross-check), and the S.C.2 ◆ confirms the second cryptanalytic break, so
  the conservative default is to halt at the S.C.2 ◆ for the human glance + the opus juncture fork.
  Both sessions are Sonnet and the constructions are canonical (textbook reversible point-addition,
  textbook two-register Shor-ECDLP), so S.C.1 could run autonomously, but the new-orchestration-pattern
  + second-break milestone argues for `halt-at-boundaries` on the first invocation; the S.C.2 ◆ fork
  is itself a halt. *(Tradeoff vs autonomous: `halt-at-boundaries` trades a little velocity on the
  mechanical S.C.1 for a guaranteed human check at the second break + the curve-order-ceiling
  resolution + the rho-cross-check integration — the right trade. If S.C.1 lands clean and its
  reversibility/scratch-clean KATs pass, S.C.2 can be dispatched immediately after the ◆ glance.)*
