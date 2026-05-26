# rGNFS — Current Plan: Phase α (Foundation)

The rolling, current-phase view of the work. Rewritten at sub-track boundaries.
For the project-lifetime view, see `docs/ROADMAP.md`.

---

## Current state

The `rho` crate is complete (Phases 0-9 in its own internal numbering, last commit `d0a7c04`).
Pollard rho for both integer factorization and ECDLP, with all canonical CPU optimizations, ships
as a single-crate artifact under the workspace root.

We are now at the boundary entering Phase α. **Nothing else exists yet.**

## Phase α — Foundation

Goal: restructure the workspace and build shared substrate so that Tracks G, D, E, S can begin.

Sessions in this phase: **3-5 total.** Estimated wall time at one session per week: **~1 month.**

### Session α.1 — Workspace restructure

**Type:** Category A (substrate). **Tier:** Sonnet. **Estimated LOC:** ~200 (mostly moves, plus
new Cargo.toml entries).

**Inputs:**
- Existing `rho/` crate, untouched.
- `Cargo.toml` workspace at repo root.

**Outputs (contracts established):**
- New workspace layout with `shared/` crates extracted from `rho/`.
- `shared::field` crate exposing the `Fp` trait + `FpNaive` + `FpMonty`, generic over `Uint<L>`
  rather than hard-coded to `Uint<4>`.
- `shared::bigint` crate carrying `batch_invert` and the `mp` helpers.
- `rho/` crate updated to depend on `shared::field` and `shared::bigint` instead of defining them.

**Subtleties:**
- The current `Fp` trait is hard-coded to `Uint<4>` (256-bit). Number-field arithmetic in G.A may
  want different limb counts; GF(2^m) in E.F will want a different field family entirely. This
  session must make `shared::field` generic over `L`, not just move the existing code.
- The `Fp` trait carries 10 methods (zero, one, from_u64, from_uint, to_uint, add, sub, neg, mul,
  square, pow, inv, is_zero). Per the over-specification rule, this session should review and add
  any obvious gaps before extraction — candidates: `double`, `triple`, `is_one`, scalar exponentiation
  with variable-width exponents, a `legendre`/`sqrt` for cases where the prime supports it.
- The decision on whether to extend the trait now or later is a design call worth flagging in the
  session prompt.

**Acceptance criterion:**
- `cargo test --workspace` passes with no regressions vs. the current rho test suite.
- `cargo bench --workspace` runs successfully (numbers may drift slightly due to crate-boundary
  inlining — acceptable as long as the qualitative speedups in `docs/BENCHMARKS.md` still hold).

**Deferred to later sessions:**
- `shared::numth` (smoothness, ECM) — that's α.2.
- Polynomial-coefficient arithmetic over rings — that's G.A.
- GF(2^m) — that's E.F.

### Session α.2 — `shared::numth` substrate (part 1: primality + smoothness scaffolding)

**Type:** Category A (substrate). **Tier:** Sonnet. **Estimated LOC:** ~400-600.

**Inputs:**
- α.1 complete (`shared::field` and `shared::bigint` available).

**Outputs (contracts established):**
- `shared::numth` crate.
- Miller–Rabin primality testing (probabilistic, with the deterministic witness set for `Uint<4>`).
- A `Smoothness` trait or set of free functions for testing B-smoothness of integers against a
  prime factor base, returning a factorisation witness.
- Trial-division based baseline. ECM is α.3.

**Acceptance criterion:**
- KAT: Miller–Rabin agrees with `primal-check` (or equivalent reference) on the first 100,000
  integers and on the published list of strong pseudoprime bases.
- KAT: smoothness witness multiplies back to the input integer (round-trip property).
- Property test: B-smoothness predicate matches a brute-force trial-division reference.

**Subtleties:**
- The smoothness witness representation needs to accommodate the consumer in E.K (smoothness of
  curve points via Semaev polynomials) — see Contract C1 in ROADMAP.md. This session designs the
  trait surface with all three consumers in mind even though only G.C and D.A are imminent.

**Deferred:**
- ECM — α.3.
- Witness representation specific to elliptic-curve smoothness — E.J/E.K will extend.

### Session α.3 — `shared::numth` (part 2: ECM)

**Type:** Category A/B (substrate-plus-algorithm — ECM is itself an algorithm, but used here as a
substrate primitive). **Tier:** Sonnet. **Estimated LOC:** ~500-700.

**Inputs:**
- α.2 complete.
- Curve group law from existing `rho` crate — ECM needs Montgomery-form curves and scalar
  multiplication, but reusing rho's curve machinery (lifted into shared::curve at this point)
  is correct.

**Outputs:**
- ECM (Lenstra elliptic-curve method) with stage 1 + stage 2.
- Used downstream as: a sub-step inside NFS large-prime variations (G.C); a fallback for factoring
  composite group orders in Pohlig–Hellman (E.A).

**Acceptance criterion:**
- KAT: factor a set of published ECM-friendly semiprimes (small factor + large cofactor) using
  stage 1 alone.
- KAT: stage 1 + stage 2 finds factors that stage 1 alone misses, on engineered instances.

**Subtleties:**
- This session may want to lift the existing `rho::curve` into `shared::curve` first, depending on
  how ECM's curve usage compares to rho's. Decide at session start. If the lift is non-trivial,
  split into α.3a (lift) and α.3b (ECM).

**Deferred:**
- Stage 2 optimizations (Brent–Suyama parameterization, FFT continuation) — demonstration fidelity
  only at this scale; full performance work is out of scope.

### Session α.4 (conditional) — Opus review at sub-track boundary

**Type:** Inflection-point Opus review. **Tier:** Opus. **Estimated LOC:** 0 (this is a planning
session, not an implementation session).

**Trigger condition:** α.1, α.2, α.3 all complete with green tests.

**Purpose:**
- Verify Phase α actually delivered the contracts ROADMAP.md says it would.
- Update Contract C1 (`is_smooth` interface) based on what was actually built, vs. what was
  specified.
- Decide whether to extend `shared::field`'s trait surface based on observations from α.1-α.3.
- Re-read the design statement from ROADMAP.md and confirm Phase β is the right next step (vs.
  diverting to ε or some other track).
- Rewrite this PLAN.md for Phase β (specifically G.A — number field substrate).

**Output:**
- Updated `docs/ROADMAP.md` (Discoveries log entries).
- New `docs/PLAN.md` for Phase β.
- Possibly `CAPTURE-CANDIDATE` items for `~/Documents/software/opencode-config/AGENTS-HINTS.md`.

### Session α.5 (conditional) — Patch-up session

**Trigger condition:** α.4 surfaces specific gaps in Phase α deliverables.

This is reserved capacity, not a planned session. If α.4 reveals that (e.g.) the `Fp` trait needs
another method or `shared::numth`'s smoothness interface needs adjustment, α.5 is the session
that lands those changes before Phase β begins.

---

## Cross-session contracts established in Phase α

| Contract | Established in | Consumers | Enforcement |
|----------|---------------|-----------|-------------|
| `shared::field::Fp` trait | α.1 | all subsequent tracks | Compiler |
| Field KAT corpus (lifted from rho) | α.1 | all subsequent tracks | `cargo test` |
| `shared::numth::is_prime`, `Smoothness` trait | α.2 | G.C, D.A, E.K | Compiler + tests |
| ECM as factoring sub-step | α.3 | G.C (large primes), E.A (subgroup orders) | API |

These are the substrates Phase β onwards consumes. Changes to any of them mid-project require an
Opus inflection-point session, not an ad-hoc edit.

---

## Risk register for Phase α

- **Risk: `Fp` extraction is harder than it looks.** The current trait is `Uint<4>`-specific in
  ways that may not be obvious from reading the trait alone. **Mitigation:** start α.1 with a
  spike to confirm the generic-over-`L` reshape compiles end-to-end before committing to the full
  extraction.
- **Risk: smoothness trait designed for three consumers when only two are imminent leads to
  premature abstraction.** **Mitigation:** at α.2, write a one-paragraph note describing how E.K
  would consume the trait, even though E.K is 30+ sessions away. If the note can't be written
  coherently, narrow the trait to G.C/D.A and accept that E.K may need an extension later.
- **Risk: ECM at α.3 turns out to need machinery that doesn't exist yet (e.g., FFT for stage 2).**
  **Mitigation:** stage 2 at demonstration fidelity only — Brent–Suyama parameterization without
  FFT. Performance is not the goal at this scale.

---

## Notes for `@build` sessions executing Phase α

- Read this file and `docs/ROADMAP.md` before starting any session in Phase α.
- Read `~/Documents/software/opencode-config/multi-session-planning.md` for the planning
  philosophy. In particular: this is the **action frame**; the roadmap is the **static frame**;
  the two require different review cadences.
- Read `rho/docs/PEDAGOGY.md` for the pedagogical register. New work in shared crates should match
  the register: rST docstrings, KATs per phase, narrative chapter at each sub-track boundary.
- The `rho` crate is **untouched** until α.1 explicitly modifies its `Cargo.toml`. Don't pre-edit.
- All sessions in Phase α use Sonnet 4.6 unless explicitly flagged otherwise. α.4 is the only
  pre-planned Opus session in this phase.
- End every session with green `cargo test --workspace`. A red session is not a complete session.
