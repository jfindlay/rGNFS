# rGNFS — Survey Findings (Arc 2: A. SURVEY)

Findings-only ledger. Each entry names the item, its flow direction, and the consuming campaign.
No config, code, or doc content is written here — ALIGN/ORACLE/REFACTOR/CONSOLIDATE/EXTEND do that.

Sessions: A.1 (done) · A.2–A.7 (pending).

---

## A.1 — Template-formalities gap, both flow directions (D1)

**Charge:** audit every formalities item present in `~/Source/rust-template` but absent at the
rGNFS workspace root, assign a flow direction (template→rGNFS backport or rGNFS→template
forward-seed), and note scope (per-crate vs workspace-level). Also enumerate rGNFS-originated
discipline worth seeding back. Feeds C-TemplateSeed; consumed by ALIGN.

**Observed state of `~/Source/rust-template` root:**
`.cargo/config.toml`, `AGENTS.md`, `Cargo.lock`, `Cargo.toml`, `deny.toml`, `docs/development.md`,
`docs/NOTES.md`, `LICENSE`, `README.md`, `rustfmt.toml`, `src/`, `target/`, `tests/`.
No `rust-toolchain.toml` at template root.

**Observed state of rGNFS workspace root:**
`.gitignore`, `Cargo.lock`, `Cargo.toml`, `docs/`, `gnfs/`, `README.md`, `rho/`, `shared/`,
`shor/`, `target/`.
No `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, `LICENSE`, `development.md`, `.cargo/`.

---

### F-D1-01 · `deny.toml` absent at rGNFS root

**Direction:** template → rGNFS (backport).
**Scope:** workspace-level (one file at root; applies to all 9 crates).
**Template content:** advisories (`yanked = "deny"`), licenses allowlist
(`MIT`, `Apache-2.0`, `Unicode-3.0`, `GPL-3.0`, `BSD-3-Clause`), bans
(`multiple-versions = "warn"`), sources (`unknown-registry = "deny"`, `unknown-git = "deny"`).
**Finding:** `deny.toml` absent at rGNFS root → ALIGN adds it at workspace root.

---

### F-D1-02 · `rustfmt.toml` absent at rGNFS root

**Direction:** template → rGNFS (backport).
**Scope:** workspace-level (one file at root; `cargo fmt --all` picks it up for all crates).
**Template content:** `max_width = 100` (mirrors the 100-char wrap convention).
**Finding:** `rustfmt.toml` absent at rGNFS root → ALIGN adds it at workspace root.

---

### F-D1-03 · `rust-toolchain.toml` — stale per-crate artifact in `rho/`, absent at workspace root

**Direction:** neither a clean backport nor a forward-seed — the artifact is mis-scoped.
**Scope:** `rho/rust-toolchain.toml` exists (`[toolchain] channel = "stable"`, no version pin);
workspace root has no `rust-toolchain.toml`; template root also has no `rust-toolchain.toml`.
**Analysis:** a per-crate `rust-toolchain.toml` in a workspace member is unusual — Cargo applies
the toolchain file from the workspace root, not from a member directory, when running workspace
commands. The `rho/` file is therefore a stale artifact from before `rho` was part of the workspace
(or from an early arc-1 session), not a deliberate per-crate pin. It provides no effective toolchain
constraint for workspace-level `cargo` invocations.
**Finding:** `rho/rust-toolchain.toml` is a stale per-crate artifact with no effective workspace
scope → ALIGN decision: either promote to workspace root (with a concrete channel + version pin,
matching the template pattern if one is adopted) or remove it. The template itself carries no
`rust-toolchain.toml`, so promotion is not a strict backport — it is a workspace hygiene decision.
This is an open-Q for ALIGN to resolve; A.1 records the gap and the mis-scoping.

---

### F-D1-04 · `LICENSE` absent at rGNFS root

**Direction:** template → rGNFS (backport).
**Scope:** workspace-level (one file at root; covers all 9 crates).
**Template content:** GNU General Public License v3.0 (GPL-3.0-or-later). The template's
`Cargo.toml` declares `license = "GPL-3.0-or-later"` and `deny.toml` allows `GPL-3.0` in the
license allowlist.
**Finding:** `LICENSE` absent at rGNFS root → ALIGN adds GPL-3.0 `LICENSE` at workspace root.
Note: rGNFS crate `Cargo.toml` files do not carry a `license` field; ALIGN should also add
`license = "GPL-3.0-or-later"` to each crate manifest (or to the workspace root manifest via
`[workspace.package]` inheritance if the resolver supports it).

---

### F-D1-05 · `[lints]` table present in 3/9 crates only; absent from workspace root

**Direction:** template → rGNFS (backport), promoted to workspace-level.
**Scope:** workspace-level `[workspace.lints]` table (Rust 1.74+, resolver = "2" already set) is
the correct target — avoids per-crate repetition and ensures uniform lint policy across all 9 crates.
**Template content:** `[lints.rust]` (`unsafe_code = "forbid"`, `missing_docs = "warn"`) and
`[lints.clippy]` (`all = { level = "deny", priority = -1 }`, `pedantic = { level = "warn",
priority = -1 }`).
**Observed per-crate state:**

| Crate | `[lints]` present |
|---|---|
| `shor` | yes |
| `shared/padic` | yes |
| `shared/gf2m` | yes |
| `gnfs` | no |
| `rho` | no |
| `shared/field` | no |
| `shared/bigint` | no |
| `shared/numth` | no |
| `shared/numfield` | no |
| workspace root `Cargo.toml` | no |

**Finding:** `[lints]` present in `shor` + `shared/padic` + `shared/gf2m` only (3/9 crates) →
ALIGN promotes to `[workspace.lints]` in the workspace root `Cargo.toml` and removes the
per-crate duplicates from the 3 crates that already carry them, then adds `[lints] workspace = true`
to all 9 crate manifests.

---

### F-D1-06 · Coverage gate absent at rGNFS

**Direction:** template → rGNFS (backport), but gate value/config deferred to A.3.
**Scope:** workspace-level (one `cargo llvm-cov --workspace` invocation covers all 9 crates).
**Template content:** `cargo llvm-cov --all-targets --fail-under-lines 100` (100% line coverage
gate); `cargo-llvm-cov` listed as a required dev tool in `docs/development.md`; the `.cargo/
config.toml` alias `coverage = "llvm-cov --all-targets --fail-under-lines 100"` makes it
discoverable.
**Finding:** no coverage gate exists at rGNFS → ALIGN adds one, honoring the doctrine A.3 will
set. **The gate value (100% line vs math-behavior KAT threshold vs other) is A.3's decision;
A.1 records only that the gap exists and that ALIGN is the implementing campaign.** The
`cargo-llvm-cov` tool and the `.cargo/config.toml` alias are the implementation vehicles.

---

### F-D1-07 · `development.md` absent at rGNFS

**Direction:** template → rGNFS (backport, adapted).
**Scope:** workspace-level (one file, e.g. `docs/development.md`).
**Template content:** toolchain table (cargo, rustfmt, clippy, cargo test, cargo-llvm-cov,
cargo-deny), setup instructions, formatting commands, full check-suite four-command sequence,
versioning convention, code conventions summary, testing conventions summary, project layout.
**Finding:** `docs/development.md` absent at rGNFS → ALIGN adds it, adapted for the workspace
(9 crates, no binary entry point, workspace-level `cargo` invocations, the coverage doctrine
A.3 sets, and the CADO-NFS sidecar ORACLE builds).

---

### F-D1-08 · `.cargo/config.toml` (cargo aliases) absent at rGNFS

**Direction:** template → rGNFS (backport, adapted).
**Scope:** workspace-level (`.cargo/config.toml` at workspace root applies to all `cargo`
invocations in the workspace).
**Template content:** aliases `lint`, `fmt-check`, `format`, `test-all`, `coverage`, `audit` —
the "tox environments as pure-cargo aliases" pattern.
**Finding:** `.cargo/config.toml` absent at rGNFS root → ALIGN adds it, adapted for the workspace
(workspace-scoped `--workspace` flags, coverage alias honoring A.3's doctrine, `audit` alias for
`cargo deny check`).

---

### F-D1-09 · `AGENTS.md` absent at rGNFS root

**Direction:** template → rGNFS (backport, adapted).
**Scope:** workspace-level (one file at root).
**Template content:** agent guide with commands (`cargo build`, `cargo test --all-targets`,
`cargo clippy`, `cargo fmt`, `cargo llvm-cov`, `cargo deny check`) and code conventions summary.
**Finding:** `AGENTS.md` absent at rGNFS root → ALIGN adds it, adapted for the workspace
(workspace-level commands, the 9-crate structure, the CADO oracle policy, and the docs-layer
discipline CONSOLIDATE will freeze).

---

### F-D1-10 · rGNFS-originated discipline: multisession docs layering

**Direction:** rGNFS → template (forward-seed).
**Scope:** project-level docs convention (not a single file — a three-tier discipline).
**rGNFS content:** `docs/PLAN.md` (current sub-track, actionable session list + contracts +
ledger + digest) + `docs/ROADMAP.md` (project-lifetime view, updated only at sub-track
boundaries) + `docs/NOTES.md` (rolling-context durable framings, distinct from plan and roadmap).
The three-tier separation — rolling-context / sub-track / project-lifetime — prevents the
"one giant notes file" anti-pattern and makes the action-frame / static-frame distinction
concrete.
**Finding:** template carries only `docs/NOTES.md` (design notes and rationale); it lacks the
PLAN/ROADMAP tier separation → ALIGN seeds the three-tier discipline into the template, adapted
for single-crate projects (PLAN.md and ROADMAP.md at appropriate grain for a template project).

---

### F-D1-11 · rGNFS-originated discipline: dev-oracle policy

**Direction:** rGNFS → template (forward-seed).
**Scope:** testing convention (prose policy + `#[ignore]` gate pattern).
**rGNFS content:** all external-tool oracle KATs (PARI, msolve, CADO-NFS) are gated behind
`#[ignore = "PARI not installed; run manually when available"]` — the oracle is an opt-in
validation sidecar, never on the green test path, never a production dependency. The policy is
stated explicitly in `docs/PEDAGOGY.md` (Principle 3 verification) and is the arc-2 ORACLE
campaign's design anchor.
**Finding:** template has no dev-oracle policy (its testing conventions cover unit + integration
only) → ALIGN seeds the `#[ignore]`-gate pattern and the opt-in-oracle principle into the
template's `docs/development.md` testing conventions section.

---

### F-D1-12 · rGNFS-originated discipline: docs-register contracts (three-layer reference discipline)

**Direction:** rGNFS → template (forward-seed).
**Scope:** docs architecture convention (prose policy).
**rGNFS content:** a three-layer reference-direction discipline — (1) inline doc-comments
(`///`/`//!`) for API-adjacent exposition; (2) per-crate `PEDAGOGY.md` human-code-reference
tours (code-adjacent but human-facing, may reference code identifiers); (3) `docs/MATHEMATICS.md`
textbook layer (mathematical exposition, references code only where useful). Each layer has
prescribed reference directions and allowances. The discipline is implicit in the arc-1 corpus
and is the subject of A.4's audit; CONSOLIDATE will freeze it as C-DocsLayer.
**Finding:** template carries only `docs/NOTES.md` and `docs/development.md` — no layered
reference-direction discipline → ALIGN seeds the three-layer concept into the template at
appropriate grain for a single-crate project (inline / human-code-ref / agent-docs), adapted
from the rGNFS three-layer model.

---

### Summary table

| # | Item | Direction | Scope | Consuming campaign |
|---|---|---|---|---|
| F-D1-01 | `deny.toml` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-02 | `rustfmt.toml` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-03 | `rho/rust-toolchain.toml` stale per-crate artifact | mis-scoped (promote or remove) | workspace hygiene | ALIGN |
| F-D1-04 | `LICENSE` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-05 | `[lints]` in 3/9 crates only | template → rGNFS | workspace-level `[workspace.lints]` | ALIGN |
| F-D1-06 | coverage gate absent | template → rGNFS (gate value deferred to A.3) | workspace-level | ALIGN |
| F-D1-07 | `development.md` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-08 | `.cargo/config.toml` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-09 | `AGENTS.md` absent | template → rGNFS | workspace-level | ALIGN |
| F-D1-10 | multisession docs layering | rGNFS → template | project-level docs convention | ALIGN |
| F-D1-11 | dev-oracle policy | rGNFS → template | testing convention | ALIGN |
| F-D1-12 | docs-register contracts (three-layer) | rGNFS → template | docs architecture convention | ALIGN |

---

### Subtleties and deferrals

**`rho/rust-toolchain.toml` (F-D1-03).** The template carries no `rust-toolchain.toml` at its
root, so this is not a clean template→rGNFS backport. The `rho/` file (`channel = "stable"`,
no version pin) is a stale per-crate artifact — it has no effective scope for workspace-level
`cargo` invocations. ALIGN must decide: promote to workspace root with a concrete pin, or remove.
A.1 records the mis-scoping; ALIGN owns the decision.

**Coverage gate (F-D1-06).** The gap is recorded here; the gate value and doctrine are A.3's
charge. A.1 does not decide whether 100% line coverage (template default) or a math-behavior
KAT threshold is correct for rGNFS. ALIGN implements whatever doctrine A.3 sets.

**`[lints]` promotion (F-D1-05).** The three crates that already carry `[lints]` (`shor`,
`shared/padic`, `shared/gf2m`) use a slightly different form than the template (`all = "deny"`
vs `all = { level = "deny", priority = -1 }`). ALIGN should normalize to the template form
(with explicit `priority` fields) when promoting to `[workspace.lints]`.

**`license` field in crate manifests.** None of the 9 crate `Cargo.toml` files carry a `license`
field. Adding `LICENSE` at the workspace root (F-D1-04) is necessary but not sufficient for
`cargo deny` license checks — ALIGN should also add `license = "GPL-3.0-or-later"` to each
crate manifest or use `[workspace.package]` inheritance.

**rGNFS → template seeds (F-D1-10, F-D1-11, F-D1-12).** These are forward-seeds of discipline
the template currently lacks. They require adaptation (the template is a single-crate project;
rGNFS is a 9-crate workspace). ALIGN owns the adaptation; A.1 records the direction and the
content to seed.

---

## A.2 — Crate layout/dedup + code-depth provenance catalog (D5 + D9-code)

**Charge:** (a) audit the 9-crate workspace layout — is `rho` overloaded by hosting both Track ρ
and the large Track E attack suite? Is there duplicative code across `shared/*`? (b) catalog every
planning-frame token in code identifiers, module/file/dir names, benchmark labels, and test names;
classify each as **pure residue** (re-anchor on topic) or **grouping-coincides-with-topic** (keep
grouping, change label). Feeds C-Layout (sketch) and the code-half of C-Coherence. Consumed by
REFACTOR.

**Observed workspace members (9):**
`gnfs`, `rho`, `shor`, `shared/field`, `shared/bigint`, `shared/numth`, `shared/numfield`,
`shared/padic`, `shared/gf2m`.

---

### Part (a) — Layout audit

#### Observed `rho/src/` module tree

| Module | Belongs to |
|---|---|
| `factor/` | Track ρ — Pollard rho integer factorization (Floyd, Brent, batched-GCD, multi-c) |
| `ecdlp/` | Track ρ — Pollard rho ECDLP (r-adding walk, DPs, negmap, batched inv, GLV) |
| `curve/` | Track ρ — elliptic curve group law, affine/Jacobian points, concrete curves |
| `field/` | Track ρ — thin re-export wrapper over `shared/field` (fixes `L = 4`) |
| `util/` | Track ρ — thin re-export wrapper over `shared/bigint` (`batch_invert`) + empty `mp.rs` stub |
| `binary_curve/` | Track E — binary curve `y²+xy=x³+ax²+b` group law (López–Dahab projective) |
| `binary_ecdlp/` | Track E — Pollard-rho ECDLP over binary curves; Koblitz τ-orbit variant |
| `ghs/` | Track E — GHS Weil-descent attack (E.H): Artin–Schreier, Weil restriction, hyperelliptic extraction |
| `hyperelliptic/` | Track E — hyperelliptic curve `y²+h(x)y=f(x)` over GF(2^m), Mumford divisors, Cantor group law |
| `index_calculus/` | Track E — Gaudry–Diem–Joux–Vitse index-calculus ECDLP solver over `E(F_p)` (E.K) |
| `pairing/` | Track E — `F_{p^k}` extension-field arithmetic and bilinear pairings (E.B) |
| `semaev/` | Track E — Semaev summation polynomials over a prime-field Weierstrass curve (E.J) |
| `ssa/` | Track E — Smart–Satoh–Araki p-adic attack on anomalous curves (E.E) |

**Count:** 5 Track-ρ modules + 8 Track-E attack modules = 13 modules total in `rho/src/`.

---

### F-D5-01 · `rho` hosts 8 Track-E attack modules alongside the Track-ρ baseline

**Observed state:** `rho/src/` contains 5 modules that implement the Pollard rho baseline
(`factor`, `ecdlp`, `curve`, `field`, `util`) and 8 modules that implement algebraic ECDLP
attacks from Track E (`binary_curve`, `binary_ecdlp`, `ghs`, `hyperelliptic`, `index_calculus`,
`pairing`, `semaev`, `ssa`). The `rho` crate's `Cargo.toml` description reads "Pollard rho:
integer factorization and ECDLP with all canonical optimizations" — it does not mention the
Track-E attacks. The `rho/src/lib.rs` crate-level doc-comment lists all 13 modules, with the
Track-E modules annotated with their sub-track IDs (E.B, E.E, E.H, E.J, E.K).

**Cohesion argument for keeping together:** the Track-E attacks are benchmarked against the
Pollard-rho baseline in `rho/benches/attacks.rs` (the `E.W` cross-attack comparison bench). The
bench explicitly notes "Pollard rho is already benched in `rho/benches/ecdlp.rs` on `secp_k1_toy`
(63-bit). That bench is the generic-√n baseline column in the E.W table." The attacks and the
baseline share the `rho::curve` and `rho::field` types directly.

**Cohesion argument for peering out:** the 8 Track-E modules are algebraically distinct from
Pollard rho (pairings, p-adic arithmetic, GHS descent, index calculus, Semaev polynomials,
hyperelliptic curves, binary curves) and constitute a large body of code that is not "Pollard rho
with optimizations." A reader of the `rho` crate encounters a much larger surface than the crate
name suggests.

**Finding:** `rho` hosts 8 Track-E attack modules alongside the 5 Track-ρ baseline modules →
REFACTOR decision: peer Track-E attacks out into a separate `attacks` crate (or similar) vs keep
them co-located with the Pollard-rho baseline they are benchmarked against. **This is an open-Q
for the human, surfaced at A.7.** A.2 records the overload; REFACTOR owns the decision and its
compiler blast radius. *Feeds REFACTOR; seeds C-Layout (sketch).*

---

### F-D5-02 · `rho/src/field/` and `rho/src/util/batch_inv.rs` are thin re-export wrappers

**Observed state:**
- `rho/src/field/mod.rs` re-exports `shared_field::Fp`, `shared_field::FpNaive<4>` (as `FpNaive`),
  and `shared_field::FpMonty<4>` (as `FpMonty`). The module's sole purpose is to fix `L = 4` for
  backward compatibility with the rest of `rho`. No logic lives here.
- `rho/src/util/batch_inv.rs` re-exports `shared_bigint::batch_invert` with a doc-comment noting
  it fixes `L = 4`. The module adds inline tests that duplicate the tests already in
  `shared/bigint/src/batch_inv.rs` (same three test cases: single, multiple, empty).
- `rho/src/util/mp.rs` is an empty stub (4 lines of doc-comment, no code), mirroring
  `shared/bigint/src/mp.rs` (3 lines of doc-comment, no code).

**Finding:** `rho/src/field/` and `rho/src/util/` are thin re-export wrappers over `shared/field`
and `shared/bigint` respectively. The `batch_invert` tests in `rho/src/util/batch_inv.rs` are
duplicated from `shared/bigint/src/batch_inv.rs`. → REFACTOR decision: collapse the re-export
wrappers (have `rho` import `shared_field` and `shared_bigint` directly at call sites, or keep the
wrappers but remove the duplicated tests). *Feeds REFACTOR; seeds C-Layout (sketch).*

---

### F-D5-03 · No other duplicative code found across `shared/*`

**Observed state:** the six `shared/*` crates (`field`, `bigint`, `numth`, `numfield`, `padic`,
`gf2m`) have distinct, non-overlapping concerns:
- `shared/field` — prime-field `Fp<L>` trait + `FpNaive`/`FpMonty` implementations.
- `shared/bigint` — `batch_invert`, `isqrt`, `gcd`, `mp` stub.
- `shared/numth` — ECM, primality, smooth-number utilities.
- `shared/numfield` — number-field arithmetic (`IntPoly`, `NumberField`, `dedekind`, `ideal`,
  `resultant`).
- `shared/padic` — p-adic arithmetic (`Zp`, Hensel lifting, formal-group logarithm).
- `shared/gf2m` — GF(2^m) arithmetic (naive, normal-basis, optimized, subfield, polynomial).

No cross-crate duplication was observed among the six `shared/*` crates. Each crate's scope is
distinct and non-overlapping.

**Finding:** `shared/*` carries no duplicative code across the six crates — the dedup concern
(D5) is confined to the `rho/src/field/` and `rho/src/util/` re-export wrappers identified in
F-D5-02. *Feeds REFACTOR; seeds C-Layout (sketch).*

---

### Part (b) — Code-depth provenance catalog

**Scope of search:** all `*.rs` files in the workspace — `src/`, `tests/`, `benches/` directories
across all 9 crates. Searched for: `Phase N` (N = digit or letter), `Track [A-Z]`, `sub-track`,
`sub_track`, sub-track IDs (`E.X`, `G.X`, `D.X`, `S.X`), and planning-frame tokens in function
names, struct names, enum names, const names, module names, file names, and directory names.

**Headline finding (load-bearing for REFACTOR sizing):** planning-frame tokens are
**doc-comment-heavy and identifier-light**. Zero planning-frame tokens appear in code identifiers
(function names, struct names, enum names, const names, module names, file names, directory
names). All planning-frame tokens are in `//!` crate/module doc-comments, `///` item doc-comments,
and `//` inline comments. The identifier rename blast radius for REFACTOR is therefore zero for
this category; the doc-comment scrubbing is the bulk of the work, and doc-comments are a
REFACTOR/CONSOLIDATE boundary question (inline `//!`/`///` are code-adjacent → REFACTOR; human
prose docs → CONSOLIDATE).

---

### F-D9-01 · `Phase N` (N = 1–8) tokens in `rho/src/ecdlp/` and related files — pure residue

**Observed state:** `Phase 4` through `Phase 8` appear in doc-comments and inline comments
throughout the Pollard-rho ECDLP optimization stack. Specific locations:

| File | Phase tokens | Context |
|---|---|---|
| `rho/src/ecdlp/mod.rs` | Phase 4, 5, 6, 7, 8 | Module-level doc-comment (optimization layers list); section headers; function doc-comments |
| `rho/src/ecdlp/cli.rs` | Phase 5, 6, 7, 8 | CLI help text and inline comments labeling solver variants |
| `rho/src/ecdlp/walk.rs` | Phase 4, 7 | Module doc-comment; inline comments within `BatchedWalker::step_all` |
| `rho/benches/ecdlp.rs` | Phase 5, 6, 7, 8 | Bench module doc-comment; section headers; bench function doc-comments |
| `rho/benches/field.rs` | Phase 1 | Bench module doc-comment ("Phase 1 deliverable") |
| `rho/benches/factor.rs` | Phase 2 | Bench module doc-comment ("Phase 2 deliverable") |
| `rho/tests/ecdlp_kat.rs` | Phase 3, 4, E.A.2 | Test file doc-comment; section header comments |
| `rho/src/curve/secp_k1_toy.rs` | Phase 8 | Module doc-comment and function doc-comment |
| `rho/src/curve/generic.rs` | Phase 3–4 | Module doc-comment |
| `rho/src/curve/mod.rs` | Phase 8 | Module doc-comment |
| `rho/src/curve/test_curves.rs` | Phase 4–E | File doc-comment |
| `rho/src/field/monty.rs` | Phase 1 | Module doc-comment |

**Classification:** **pure residue.** The `Phase N` labels are arc-1 session-phase identifiers
(the optimization layers were built one per session: Phase 1 = field arithmetic, Phase 2 =
factorization, Phase 3 = curve arithmetic, Phase 4 = r-adding walk + Brent, Phase 5 = DPs,
Phase 6 = negmap, Phase 7 = batched inversion, Phase 8 = GLV). The mathematical content has
topic-native names: the optimization layers are named after their algorithms (r-adding walk,
distinguished-point parallel search, negation map, batched field inversion, GLV endomorphism).
The `Phase N` label adds no information beyond the planning provenance.

**Finding:** `Phase 4`–`Phase 8` tokens in `rho/src/ecdlp/mod.rs`, `rho/src/ecdlp/cli.rs`,
`rho/src/ecdlp/walk.rs`, `rho/benches/ecdlp.rs`, and related files → REFACTOR re-anchors on the
optimization-layer names (r-adding walk, distinguished-point search, negation map, batched
inversion, GLV endomorphism). `Phase 1`/`Phase 2` in bench doc-comments → REFACTOR removes the
"Phase N deliverable" framing and replaces with a topic description. *Feeds REFACTOR; seeds
C-Coherence (code half).*

**Note on `Phase 1/2/3` in `rho/src/ecdlp/walk.rs` (lines 321, 351, 354) and
`rho/src/index_calculus/solve.rs` (lines 15–16, 112, 129):** these are *algorithmic* phase labels
within a single function's multi-step algorithm (batched inversion steps; DLP recovery strategy
steps), not planning-frame tokens. They are NOT classified as provenance residue and are NOT in
scope for REFACTOR.

**Note on `Phase` in `shor/tests/factor_kat.rs`:** these refer to the *quantum phase* in quantum
phase estimation (e.g., "Phase 64/256 = 1/4"), a mathematical term. NOT a planning-frame token.

---

### F-D9-02 · `Track D` / `Track E` tokens in doc-comments — pure residue

**Observed state:** `Track D` appears in `gnfs/src/dl/mod.rs` (module doc-comment: "NFS-DL bridge
sub-track (Track D)") and `gnfs/src/dl/descent/node.rs` and `gnfs/src/dl/descent/mod.rs`
(inline comments: "not consumed outside Track D", "callers within Track D"). `Track E` does not
appear as a standalone token in `*.rs` files (the Track-E content is referenced via sub-track IDs
like `E.B`, `E.K`, etc., not as "Track E").

**Classification:** **pure residue.** "Track D" is a planning-frame label for the NFS-DL
sub-track. The mathematical topic is "NFS discrete-logarithm algorithms." The `gnfs/src/dl/`
module is already organized by topic (the `dl` directory name is topic-native); "Track D" adds
only planning provenance.

**Finding:** `Track D` tokens in `gnfs/src/dl/mod.rs`, `gnfs/src/dl/descent/node.rs`, and
`gnfs/src/dl/descent/mod.rs` → REFACTOR replaces with topic-native language ("NFS-DL substrate",
"this module", "callers within `gnfs::dl`"). *Feeds REFACTOR; seeds C-Coherence (code half).*

---

### F-D9-03 · Sub-track IDs (`E.X`, `G.X`, `D.X`) in doc-comments — pure residue

**Observed state:** sub-track IDs appear extensively in `///` and `//!` doc-comments and `//`
inline comments across `rho/src/`, `gnfs/src/`, `shared/padic/src/`, and in test files. Examples:

- `rho/src/lib.rs`: `(E.B)`, `(E.E)`, `(E.H)`, `(E.J)`, `(E.K)` in module listing
- `rho/src/pairing/mod.rs`: `(E.B sub-track)`, `(E.B.1)`, `(E.B.2)`, `(E.B.3)`, `(E.B.4)`
- `rho/src/ssa/mod.rs`, `rho/src/ssa/lift.rs`: `(E.E.1)`, `(E.E.2)`, `C-AnomalousLift, E.E.1`
- `rho/src/ghs/mod.rs`, `rho/src/ghs/curve.rs`, `rho/src/ghs/transfer.rs`, `rho/src/ghs/reduce.rs`,
  `rho/src/ghs/descent.rs`: `(E.H)`, `(E.H.2)`, `(E.H.3)`, `(E.H.4)`, `(E.H.5)`
- `rho/src/semaev/mod.rs`, `rho/src/semaev/poly.rs`, `rho/src/semaev/base.rs`,
  `rho/src/semaev/recursion.rs`: `(E.J)`, `(E.J.1)`, `(E.J.2)`, `(E.J.3)`
- `rho/src/index_calculus/mod.rs`, `rho/src/index_calculus/strategy.rs`,
  `rho/src/index_calculus/decompose.rs`, `rho/src/index_calculus/collect.rs`,
  `rho/src/index_calculus/linalg.rs`, `rho/src/index_calculus/solve.rs`: `(E.K)`, `(E.K.1)`
  through `(E.K.5 ◆)`
- `rho/src/binary_ecdlp/mod.rs`, `rho/src/binary_ecdlp/koblitz.rs`: `(E.G.2)`, `(E.G.3)`
- `rho/src/pairing/ecext.rs`, `rho/src/pairing/mov.rs`, `rho/src/pairing/fpext.rs`: `(E.B)`,
  `(E.C)`, `(E.C.1)`, `(E.C.2)`
- `gnfs/src/dl/mod.rs`, `gnfs/src/dl/linalg/mod.rs`, `gnfs/src/dl/descent/solve.rs`,
  `gnfs/src/dl/descent/node.rs`, `gnfs/src/dl/descent/mod.rs`, `gnfs/src/dl/descent/recurse.rs`:
  `(D.A.1)`, `(D.B.1)`, `(D.B.2)`, `(D.C.1)`, `(D.C.2)`, `(D.C.3)`, `(D.E.1)`, `(D.E.3)`
- `gnfs/src/polyselect/`, `gnfs/src/filter/`, `gnfs/src/sqrt/`, `gnfs/src/linalg/`: `(G.B.1)`
  through `(G.F.4)` extensively
- `shared/padic/src/lib.rs`: `(E.D)`, `(E.E)`
- `shared/gf2m/src/opt.rs`, `shared/gf2m/src/subfield.rs`: `(E.F.1)`, `(E.H.1)`, `(E.K)`

**Classification:** **pure residue.** The sub-track IDs (`E.B`, `G.C.3`, `D.C.1`, etc.) are
planning-frame session identifiers. Each module and function already has a topic-native name
(e.g., `pairing`, `weil_pairing`, `solve_dl`, `collect_relations`). The sub-track IDs add only
construction-history provenance; they do not convey mathematical content that the topic-native
names do not already convey.

**Finding:** sub-track IDs (`E.X.Y`, `G.X.Y`, `D.X.Y`, `S.X.Y`) in `///`/`//!`/`//` comments
across `rho/src/`, `gnfs/src/`, and `shared/*/src/` → REFACTOR replaces with topic-native
references (module paths, function names, contract names where those are already topic-native) or
removes where the ID is the only content. *Feeds REFACTOR; seeds C-Coherence (code half).*

---

### F-D9-04 · `sub-track` / `sub_track` tokens in doc-comments and test names — pure residue

**Observed state:**
- `sub-track` appears in doc-comments in `rho/src/pairing/mod.rs`, `rho/src/pairing/test_curves.rs`,
  `gnfs/src/dl/mod.rs`, `gnfs/src/dl/descent/node.rs`, `gnfs/src/dl/descent/mod.rs`.
- `sub-track` appears in test file doc-comments: `rho/tests/binary_curve_kat.rs`,
  `rho/tests/index_calculus_kat.rs`, `rho/tests/hyperelliptic_kat.rs`, `rho/tests/semaev_kat.rs`.
- `sub_track_close_curve_axioms_intact` is a **test function name** in
  `rho/tests/binary_curve_kat.rs` (line 582) — the only planning-frame token found in a code
  identifier (a test name).

**Classification of doc-comment occurrences:** **pure residue.** "Sub-track" is a planning-frame
organizational unit; the mathematical content is the algorithm or module being described.

**Classification of `sub_track_close_curve_axioms_intact` (test name):** **pure residue.** The
test name encodes the planning milestone ("sub-track close") rather than the mathematical property
being verified ("binary curve axioms intact"). The mathematical property is the correct anchor.

**Finding:** `sub_track_close_curve_axioms_intact` in `rho/tests/binary_curve_kat.rs` is the
single planning-frame token found in a code identifier (a test function name) → REFACTOR renames
to a topic-native name (e.g., `binary_curve_axioms_intact` or `curve_axioms_hold_after_full_suite`).
`sub-track` in doc-comments → REFACTOR replaces with topic-native language. *Feeds REFACTOR;
seeds C-Coherence (code half).*

---

### F-D9-05 · `PLAN.md` and session-contract references in test file doc-comments — pure residue

**Observed state:**
- `shared/padic/tests/hensel_kat.rs` line 6: `//! Hand-computed Newton iteration (from PLAN.md E.D.2):`
- `shared/numfield/tests/numfield_kat.rs` line 3: `//! Three required KATs from the G.A.1a session contract (C-NF):`
- `rho/tests/index_calculus_kat.rs` line 50: `//! # KAT coverage (E.K.5 ◆ — C-IndexCalc, sub-track close)`
- `rho/tests/index_calculus_kat.rs` line 911: `// ─── KAT 14: principle4_annotation (E.K.5 ◆ — sub-track close) ──`
- `rho/tests/index_calculus_kat.rs` line 916: `/// principle-4 boundary for the E.K sub-track close:`

**Classification:** **pure residue.** References to `PLAN.md`, session contracts (`G.A.1a session
contract`), and sub-track-close milestones (`E.K.5 ◆ — sub-track close`) are construction-history
provenance. The mathematical content (Newton iteration for Hensel lifting; KAT coverage for
number-field arithmetic; the principle-4 boundary for index calculus) has topic-native descriptions
that do not require the planning reference.

**Finding:** `PLAN.md E.D.2` reference in `shared/padic/tests/hensel_kat.rs`, `G.A.1a session
contract` reference in `shared/numfield/tests/numfield_kat.rs`, and `sub-track close` / `◆`
milestone references in `rho/tests/index_calculus_kat.rs` → REFACTOR replaces with topic-native
descriptions (e.g., "Hand-computed Newton iteration for `f(x) = x² − 2`, `p = 7`"; "KATs for
number-field arithmetic over ℤ[α]"; "principle-4 boundary: index calculus over `E(F_p)` at toy
scale"). *Feeds REFACTOR; seeds C-Coherence (code half).*

---

### F-D9-06 · `E.W.1` bench label and `Phase E.A.2` test section header — pure residue

**Observed state:**
- `rho/benches/attacks.rs` line 1: `//! Cross-attack ECDLP benchmark harness (E.W.1).` — the
  bench module doc-comment uses the sub-track ID `E.W.1` as the primary label.
- `rho/tests/ecdlp_kat.rs` line 352: `// ── Phase E.A.2 — Pohlig–Hellman composite-order ECDLP KATs ──` —
  a section header combining `Phase` and a sub-track ID.

**Classification:** **pure residue.** The mathematical content of `attacks.rs` is "cross-attack
ECDLP benchmark comparing Pollard rho, MOV/Frey–Rück, SSA, GHS, and index calculus." The
`E.W.1` label adds only planning provenance. Similarly, `Phase E.A.2` is a planning label for
the Pohlig–Hellman section; the topic-native label is "Pohlig–Hellman composite-order ECDLP KATs."

**Finding:** `E.W.1` in `rho/benches/attacks.rs` module doc-comment and `Phase E.A.2` in
`rho/tests/ecdlp_kat.rs` section header → REFACTOR replaces with topic-native descriptions.
*Feeds REFACTOR; seeds C-Coherence (code half).*

---

### Summary table — layout findings

| # | Finding | Classification | Consuming campaign |
|---|---|---|---|
| F-D5-01 | `rho` hosts 8 Track-E attack modules alongside 5 Track-ρ modules | Layout overload — open-Q for human at A.7 | REFACTOR |
| F-D5-02 | `rho/src/field/` and `rho/src/util/` are thin re-export wrappers; `batch_invert` tests duplicated | Dedup candidate | REFACTOR |
| F-D5-03 | No other duplicative code across `shared/*` | No action needed | — |

### Summary table — code-depth provenance findings

| # | Finding | Classification | Consuming campaign |
|---|---|---|---|
| F-D9-01 | `Phase 1`–`Phase 8` in `rho/src/ecdlp/`, `rho/benches/`, `rho/tests/ecdlp_kat.rs`, `rho/src/curve/`, `rho/src/field/monty.rs` | Pure residue | REFACTOR |
| F-D9-02 | `Track D` in `gnfs/src/dl/mod.rs` and descent sub-modules | Pure residue | REFACTOR |
| F-D9-03 | Sub-track IDs (`E.X.Y`, `G.X.Y`, `D.X.Y`) in doc-comments across `rho/src/`, `gnfs/src/`, `shared/*/src/` | Pure residue | REFACTOR |
| F-D9-04 | `sub_track_close_curve_axioms_intact` test name in `rho/tests/binary_curve_kat.rs` (the single planning-frame token in a code identifier); `sub-track` in doc-comments | Pure residue | REFACTOR |
| F-D9-05 | `PLAN.md E.D.2` and `G.A.1a session contract` references in test doc-comments; `◆ sub-track close` milestone labels | Pure residue | REFACTOR |
| F-D9-06 | `E.W.1` bench label in `rho/benches/attacks.rs`; `Phase E.A.2` section header in `rho/tests/ecdlp_kat.rs` | Pure residue | REFACTOR |

---

### Subtleties and deferrals

**De-provenancing blast radius (load-bearing for REFACTOR sizing).** The blast radius is
**doc-comment-heavy, identifier-light**: exactly one planning-frame token was found in a code
identifier (`sub_track_close_curve_axioms_intact` — a test function name). All other tokens are
in `//!`/`///`/`//` comments. Identifier renames are cheap (compiler-checked, one-shot); the bulk
of REFACTOR's de-provenancing work is doc-comment scrubbing across `rho/src/`, `gnfs/src/`, and
`shared/*/src/`. This is the headline sizing signal for REFACTOR.

**Inline `//!`/`///` vs human prose boundary.** Inline doc-comments (`//!` module-level,
`///` item-level) are code-adjacent and belong to REFACTOR's scope. Human prose docs
(`PEDAGOGY.md`, `MATHEMATICS.md`, `README.md`) belong to CONSOLIDATE's scope (A.4 audits those).
The `PLAN.md E.D.2` reference in `shared/padic/tests/hensel_kat.rs` is a test file doc-comment
(code-adjacent) → REFACTOR.

**`rho` overload decision (open-Q).** F-D5-01 records the overload; it does not decide the
layout. The cohesion argument (attacks bench against the rho baseline) and the organization
argument (8 algebraically-distinct attack modules under a "Pollard rho" crate name) are both
real. This is surfaced as an open-Q at A.7 for the human. REFACTOR owns the decision and its
compiler blast radius.

**`rho/src/field/` and `rho/src/util/` wrappers (F-D5-02).** These are thin re-export wrappers
that fix `L = 4` for backward compatibility. REFACTOR must decide whether to collapse them (have
`rho` import `shared_field`/`shared_bigint` directly) or keep them as convenience aliases. The
duplicated `batch_invert` tests are the cleaner dedup target — the wrapper itself may be worth
keeping for the `L = 4` fixation. A.2 records the state; REFACTOR decides.

**Contract-name tokens (`C-AnomalousLift`, `C-IndexCalc`, `C-Pairing`, etc.).** These appear
extensively in doc-comments alongside the sub-track IDs. They are planning-frame contract labels.
Their classification is the same as the sub-track IDs (pure residue) — the mathematical content
is the algorithm or interface, not the contract name. REFACTOR replaces with topic-native
descriptions. A.2 notes them here; the full catalog is in F-D9-03.
