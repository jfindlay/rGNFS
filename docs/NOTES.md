# rDLP — Notes

Rolling-context durable framings, decisions, and mental models. Distinct from `docs/PLAN.md`
(current sub-track) and `docs/ROADMAP.md` (project-lifetime view). Captured framings that prove
durable graduate to the ROADMAP Discoveries log at a sub-track boundary.

---

## Transfer-style attacks shard into transfer / structure / solve (2026-06-15, E.H shard)

The small-characteristic index-calculus attack on binary-curve ECDLP decomposes into three
sub-tracks at clean contract seams: **E.H** (GHS/Weil descent) *transfers* an ECDLP on `E/GF(2^m)`
to a DLP on `Jac(C)/GF(2^l)`; **E.J** builds the Semaev summation polynomials; **E.K** runs the
index-calculus *solve*.

## D. REFACTOR freeze record (2026-06-21, D.6 ◆)

Captured at the D.6 ◆ freeze. These are the durable structural facts that bind CONSOLIDATE and
EXTEND.

### C-Layout freeze (compiler-enforced, FROZEN at D.6 ◆)

The final crate layout after D.1's wrapper collapse:

- **Removed:** `rho/src/field/mod.rs`, `rho/src/field/monty.rs` (dead code — never declared in
  `mod.rs`), `rho/src/field/naive.rs` (dead code — never declared in `mod.rs`),
  `rho/src/util/batch_inv.rs` (re-export shim + duplicated `batch_invert` tests),
  `rho/src/util/mp.rs` (empty stub), `rho/src/util/mod.rs` (collapsed away).
- **`rho` now imports directly:** `shared_field::{Fp, FpNaive4, FpMonty4}` and
  `shared_bigint::batch_invert` at call-sites (no intermediate re-export layer).
- **Preserved:** `F: Fp<4>` trait bounds throughout `rho` (the trait is genuinely generic; only the
  local aliases were contrivance). No crate split (open-Q 1 ratified: Track-E attack modules stay
  co-located with the Pollard-rho baseline). No other `shared/*` dedup (F-D5-03).
- **Proof:** `cargo check --workspace` and `cargo test --workspace` green at D.5 terminus
  (`3ef9aca`). The compiler-enforced contract is the proof.
- **Binds:** CONSOLIDATE (docs reference the final layout) and EXTEND (new code sits in it).

### `shor/Cargo.toml` dev-dep addition (structural necessity of D.1)

Before the collapse, `shor/tests/ecdlp_kat.rs` imported `rho::field::FpMonty` (the re-export
wrapper). Once `rho::field` was removed, the compiler forced a direct `shared-field` dev-dep in
`shor/Cargo.toml`. This is a mechanical consequence of the collapse, not a REFACTOR-scope
violation. Recorded here because it is a structural fact about the workspace's dependency graph.

### C-Coherence code-half freeze (FROZEN at D.6 ◆, with ~1% residue for CONSOLIDATE)

The code-depth de-provenancing catalog (F-D9-01…06, ~524 tokens) is consumed:

- **Scrubbed across D.2–D.5** (`89f1e22`, `84b993c`, `f2302a4`, `3ef9aca`): `Phase N` planning
  labels → optimization-layer names; `Track D` → topic-native language; sub-track IDs + contract-
  label tokens (`C-AnomalousLift`, `C-IndexCalc`, `C-Pairing`) → topic-native references or removal.
- **Identifier renames (D.3):** `sub_track_close_curve_axioms_intact` → `binary_curve_axioms_intact`
  (in `rho/tests/binary_curve_kat.rs`); `c_ek_relation_round_trip` → `relation_round_trip` (in
  `rho/tests/index_calculus_kat.rs`, contract-label prefix removal).
- **Three load-bearing carve-outs held:** algorithmic `Phase 1/2/3` step-indices in
  `rho/src/ecdlp/walk.rs` and `rho/src/index_calculus/solve.rs` preserved (not planning residue);
  quantum-phase-estimation `Phase N/M` tokens in `shor/tests/factor_kat.rs` preserved (math term).
- **Interface-contract vocabulary preserved:** `C-StateVec`, `C-PointAdd`, `C-ModExp`, `C-QFT`,
  `C-Factor`, `C-OrderFind`, `C-Sparse`, `C-Relation`, `C-Ideal`, `C-Res`, `C-NF`, `C-AlgSqrt`,
  `C-Padic`, `C-Hensel`, `C-PadicLog`, `C-HyperCurve`, `C-MovBridge` — these are arc-1 design
  vocabulary naming living interfaces, not planning-frame residue.
- **~1% residue for CONSOLIDATE:** five tokens escaped the sweep — `rho/src/curve/test_curves.rs:36`
  (`Pohlig–Hellman (E.A.2)`), `:139` (`Pohlig–Hellman (E.A)`), `rho/benches/attacks.rs:20,26`
  (`E.W table` / `E.W.2 chapter`), `rho/tests/hyperelliptic_kat.rs:6` (`## E.I.2`). All
  behaviourally inert; recommended for CONSOLIDATE (which will be in these files' neighbourhood).
- **KAT suite identically green throughout** (inertness invariant held across all four sessions).

---

## Transfer-style attacks shard into transfer / structure / solve (2026-06-15, E.H shard)

The load-bearing observation: E.H's correctness signal is the **homomorphism + logarithm-preservation
relation** (`D_{P+Q} = D_P + D_Q`, and `D_h = k·D_g` for known `k` via the frozen Cantor
`scalar_mul`) — the transfer is verified by *relationship-preservation*, with the solve delegated
downstream. This is the same "build the structure, delegate the attack" idiom seen across the
project:

- **E.I** built the Jacobian group law (Cantor on Mumford pairs) and left the DLP solver to its
  consumer.
- **MOV / SSA** (E.C / E.E) transferred a prime-field ECDLP to a different DLP and verified the
  transfer by the preserved log, calling a solver (NFS-DL / p-adic log) rather than re-deriving one.

Consequence for sharding: a transfer attack freezes the *relocated problem* (here C-GHSDescent)
without ever solving it; its sub-track-close KAT is relationship-preservation, not an end-to-end
break. Bundling the solve into the transfer sub-track couples two independently-delicate designs —
the coupling the inflection-juncture discipline warns against. This framing should inform E.J/E.K
sharding and the E.W cross-attack synthesis.

---

## SURVEY durable framings (2026-06-21, A.7 ◆)

Captured at the A.7 ◆ findings-ledger freeze. These framings proved durable across all six audit
sessions and are load-bearing for the five downstream campaigns.

### Scope-routing invariant

The SURVEY split execution by artifact register, and the routing is binding for all five campaigns:

> **Inline `//!`/`///`/`//` doc-comments + identifier/file/dir/bench/test renames → REFACTOR
> (compiler blast radius); human prose (PEDAGOGY/MATHEMATICS/README) → CONSOLIDATE; config/
> manifest/CI → ALIGN; CADO sidecar build → ORACLE; new algorithm/chapter → EXTEND.**

A finding routed to the wrong campaign is the primary cross-campaign defocus mode. The boundary
is sharp: inline doc-comments are code-adjacent (REFACTOR); human prose docs are CONSOLIDATE's.
The `PLAN.md E.D.2` reference in `shared/padic/tests/hensel_kat.rs` is a test file doc-comment
(code-adjacent) → REFACTOR, not CONSOLIDATE.

### The findings-vs-fixes line (the central SURVEY defocus risk)

A session that edits a `.rs`, manifest, config, or doc *content* (vs the SURVEY findings file)
has broken scope. **SURVEY records what should change; the campaigns change it.** Any code/config/
refactor edit is a HALT-and-surface defocus signal — the audit-substrate analogue of "Z.1/T.Z
must not write code." This line held across all six audit sessions.

### C-Testing-Philosophy doctrine (frozen at A.3, ratified at A.7)

The resolved doctrine for "what is the right testing target for toy-scale compute-heavy KAT-driven
pedagogical code":

- **Primary target:** mathematical-behavior KAT coverage. Field axioms, group laws, and
  algorithm-level correctness are the primary signals. Line coverage is secondary — a floor, not
  a ceiling.
- **Coverage gate:** 80% line coverage as a dead-code floor (not a correctness ceiling). Gate value
  is a recommendation; ALIGN calibrates against a measured baseline. The `#[ignore]`-gated oracle
  tests are excluded from the gate.
- **Two-tier pattern:** inline `#[test]` for substrate verification; external `tests/*_kat.rs` for
  algorithm-level KATs. New tests (CONSOLIDATE, F-EXTEND) follow this split.
- **Test-length norm:** one file per algorithm family, however long. Splitting is not warranted by
  length alone.
- **Oracle-gate norm:** `#[ignore]` for all external-tool and compute-heavy tests. The arc-1
  dev-oracle policy; new tests honor it.

The key insight: the entire test suite is KAT-based (no proptest/quickcheck); the compute-heavy
paths are already exercised at toy scale; the two-tier structure (inline substrate + external KAT)
is healthy and intentional. A line-coverage gate that misses a mathematical property is a weaker
correctness signal than a KAT-completeness check.

### C-Coherence classification defaults (frozen at A.7)

The artifact-stands-on-its-own-terms principle, applied to the SURVEY findings:

- **Code-half:** doc-comment-heavy, identifier-light. Exactly one planning-frame token in a code
  identifier (`sub_track_close_curve_axioms_intact`). All other tokens are in `//!`/`///`/`//`
  comments. Identifier renames are cheap (compiler-checked); doc-comment scrubbing is the bulk of
  REFACTOR's de-provenancing work.
- **Prose-half:** ~185 tokens across 6 files. ~110 are "Track X" tokens (grouping-coincides-with-
  topic — the five tracks map onto five real mathematical families). ~75 are pure residue (Phase N,
  session IDs, ◆ marks, fine-grained sub-track IDs).
- **Borderline default (open-Q 4):** for coarse groupings where track ≈ topic (S.A/S.B/S.C Shor
  sub-tracks; G.B–G.W GNFS pipeline stages), the adjudicator recommendation is
  **preserve-under-topic-label** — the coarse groupings map onto real mathematical families; only
  the fine-grained S.X.Y / G.X.Y session IDs are pure residue and dissolve.
- **The key distinction:** "Track ρ" → "Pollard rho" is a label change (grouping survives);
  "Phase 5" → "distinguished-point search" is a re-anchor (planning label replaced by topic name);
  "E.K.3 ◆" → removed (pure residue with no topic-native replacement).
## D. REFACTOR freeze record (2026-06-21, D.6 ◆)

Captured at the D.6 ◆ C-Layout + C-Coherence code-half freeze. These are the durable structural
facts that bind CONSOLIDATE and EXTEND.

### C-Layout freeze (compiler-enforced, FROZEN at D.6 ◆)

Final crate layout after the D.1 wrapper collapse:

- **Removed:** `rho/src/field/mod.rs`, `rho/src/field/monty.rs` (dead code — never declared in
  `mod.rs`), `rho/src/field/naive.rs` (dead code — same), `rho/src/util/batch_inv.rs` (re-export
  shim + duplicated `batch_invert` tests), `rho/src/util/mp.rs` (empty stub),
  `rho/src/util/mod.rs`.
- **`rho` now imports directly:** `shared_field::{Fp, FpNaive4, FpMonty4}` and
  `shared_bigint::batch_invert` at call-sites. The `F: Fp<4>` trait bounds are preserved (the trait
  is genuinely generic; only the local aliases and duplicated tests were contrivance).
- **No crate split** (open-Q 1 ratified): the 8 Track-E attack modules stay co-located with the
  Pollard-rho baseline in `rho`. The E.W cross-attack bench measures attacks against the rho baseline
  and they share `rho::curve` types directly — "attacks live with the baseline they're measured
  against" is load-bearing pedagogy.
- **No other `shared/*` dedup** (F-D5-03): the six `shared/*` crates have distinct, non-overlapping
  concerns.
- **Proof:** `cargo check --workspace` and `cargo test --workspace` green at D.5 terminus (`3ef9aca`).

**Structural necessity of the D.1 collapse:** `shor/tests/ecdlp_kat.rs` imported `rho::field::FpMonty`
(the re-export wrapper). Once `rho::field` was removed, the compiler forced a direct `shared-field`
dev-dep in `shor/Cargo.toml`. This is a mechanical consequence of the collapse, not a REFACTOR-scope
violation.

### C-Coherence code-half freeze (≈99% completeness, ~1% residue for CONSOLIDATE)

All ~524 cataloged F-D9-01…06 code-depth tokens re-anchored across D.2–D.5 (`89f1e22`, `84b993c`,
`f2302a4`, `3ef9aca`). The KAT suite stayed identically green throughout (inertness invariant held).

**Three load-bearing carve-outs held (verified):**
- Algorithmic `Phase 1/2/3` step-indices in `rho/src/ecdlp/walk.rs` (batched-inversion loop steps)
  and `Phase 1/2` in `rho/src/index_calculus/solve.rs` (Gaussian-elim steps) — these are numbered
  steps of one algorithm, not planning-frame session labels. Preserved.
- Quantum-phase-estimation `Phase N/M` tokens in `shor/tests/factor_kat.rs` — a mathematical term
  ("Phase 64/256 = 1/4"), not planning residue. Preserved.

**Codebase interface-contract vocabulary preserved:** `C-StateVec`, `C-PointAdd`, `C-ModExp`,
`C-QFT`, `C-Factor`, `C-OrderFind`, `C-Sparse`, `C-Relation`, `C-Ideal`, `C-Res`, `C-NF`,
`C-AlgSqrt`, `C-Padic`, `C-Hensel`, `C-PadicLog`, `C-HyperCurve`, `C-MovBridge` — these are arc-1
design vocabulary naming living interfaces, not planning-frame residue. Scrubbing them would be the
defocus failure mode.

**~1% residue for CONSOLIDATE (not a blocker):** Five pure-residue sub-track-ID tokens escaped the
D.2–D.5 sweep, all behaviourally inert doc-comment text in lower-density peripheral files:
- `rho/src/curve/test_curves.rs:36` — `Pohlig–Hellman (E.A.2)`
- `rho/src/curve/test_curves.rs:139` — `Pohlig–Hellman (E.A)`
- `rho/benches/attacks.rs:20,26` — `E.W table` / `E.W.2 chapter` (the line-1 `E.W.1` was scrubbed
  at D.3; interior refs were missed)
- `rho/tests/hyperelliptic_kat.rs:6` — `## E.I.2` section header

Recommended for CONSOLIDATE (which owns the prose-half coherence sweep and will be in these files'
neighbourhood). The code-half freeze stands with this residue logged.

### Identifier renames in D. REFACTOR

Two identifier renames were made (both compiler- and test-verified):
- `sub_track_close_curve_axioms_intact` → `binary_curve_axioms_intact` in
  `rho/tests/binary_curve_kat.rs:582` (D.3 scope — sub-track-ID removal from test name).
- `c_ek_relation_round_trip` → `relation_round_trip` in `rho/tests/index_calculus_kat.rs` (D.3
  scope — contract-label prefix `c_ek_` removed).
