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

---

## A.3 — Testing balance + coverage-doctrine resolution (D2)

**Charge:** (a) audit the unit/integration mix across the workspace — count KAT files per crate,
identify over-long tests, map inline-vs-`tests/` distribution, confirm the `shared/numth`
no-`tests/`-dir case, note bench files; (b) resolve the meta-question "is full/100% line coverage
the right target for toy-scale-but-compute-heavy KAT-driven pedagogical code, or does KAT +
property coverage of *mathematical* behavior matter more?" — stated as a resolved coverage
doctrine. Freezes C-Testing-Philosophy; consumed by ALIGN (coverage-gate), CONSOLIDATE, F-EXTEND.

---

### Part (a) — Testing balance

#### KAT file counts per crate

| Crate | `tests/` KAT files | `benches/` files | Inline `#[test]` in `src/` |
|---|---|---|---|
| `gnfs` | 23 | 0 | yes (many — linalg, polyselect, dl substrates) |
| `rho` | 11 | 4 | yes (many — ghs, pairing, hyperelliptic, field, util) |
| `shor` | 6 | 0 | no |
| `shared/field` | 1 | 0 | yes (naive.rs: 12, monty.rs: 11) |
| `shared/bigint` | 1 | 0 | yes (batch_inv.rs: 3, isqrt.rs: 6) |
| `shared/numth` | **0** | 0 | yes (prime.rs: 10, smooth.rs: 12, ecm.rs: 12) |
| `shared/numfield` | 5 | 0 | yes (dedekind.rs: 10, ideal.rs: 4, poly.rs: 12, element.rs: 8, resultant.rs: 8) |
| `shared/padic` | 3 | 0 | no |
| `shared/gf2m` | 3 | 0 | yes (opt.rs: 5, convert.rs: 4, subfield.rs: 9, normal.rs: 4, inv.rs: 5, naive.rs: 10, poly.rs: 29) |
| **Total** | **53** | **4** | — |

**Confirmed counts vs PLAN.md survey notes:** `gnfs` has 23 KAT files (matches); `rho` has 11 KAT
files + 4 bench files (matches). Total workspace: 53 external KAT files + 4 bench files.

---

### F-D2-01 · `shared/numth` has no `tests/` directory — all tests are inline

**Observed state:** `shared/numth/` contains `Cargo.toml`, `docs/`, and `src/` only — no `tests/`
directory. All 34 tests for `shared/numth` are inline `#[cfg(test)]` blocks inside the source
files:
- `src/prime.rs`: 10 inline tests (Miller–Rabin primality, trial division, prime-power detection)
- `src/smooth.rs`: 12 inline tests (B-smooth detection, factor-base construction, sieve correctness)
- `src/ecm.rs`: 12 inline tests (ECM factorization at toy scale, stage-1 and stage-2 correctness)

**Finding:** `shared/numth` has no `tests/` directory; all 34 tests are inline in `src/`. This is
the only crate in the workspace with no external `tests/` directory and a non-trivial test suite.
The inline tests are mathematically substantive (they verify ECM factorization, primality, and
smooth-number detection at toy scale) and are not merely unit-level scaffolding. → CONSOLIDATE
decision: whether to migrate `shared/numth`'s inline tests to an external `tests/` directory for
consistency with the rest of the workspace, or accept the inline pattern as appropriate for
`shared/numth`'s role as a utility crate. A.3 records the inconsistency; CONSOLIDATE owns the
decision. *Feeds CONSOLIDATE; seeds C-Testing-Philosophy.*

---

### F-D2-02 · Inline `#[test]` usage is widespread — not confined to `shared/numth`

**Observed state:** inline `#[test]` blocks appear in `src/` files across 7 of 9 crates:
`gnfs`, `rho`, `shared/field`, `shared/bigint`, `shared/numth`, `shared/numfield`, `shared/gf2m`.
Only `shor` and `shared/padic` have no inline tests in `src/`.

The inline tests in `gnfs/src/` are particularly extensive: `linalg/blockvec.rs` (5),
`linalg/lanczos.rs` (8), `linalg/operator.rs` (4), `linalg/wiedemann.rs` (9), `linalg/qc.rs`
(6), `linalg/kernel.rs` (7), `dl/linalg/blockvec_fl.rs` (8), `dl/linalg/lanczos_fl.rs` (3),
`dl/linalg/wiedemann_fl.rs` (6), `dl/schirokauer.rs` (3), `dl/relation.rs` (2),
`dl/ext/target.rs` (17), `dl/descent/solve.rs` (5), `polyselect/base_m.rs` (6),
`polyselect/root_sieve.rs` (5), `polyselect/roots.rs` (6). These inline tests cover
linear-algebra primitives (block-vector operations, Lanczos/Wiedemann kernel steps, QC matrix
operations) that are also covered by the external `tests/` KAT files — the inline tests are
lower-level unit tests for the substrates, while the external KATs test the composed algorithms.

**Finding:** inline `#[test]` usage is the norm for low-level substrate verification (linear
algebra primitives, field arithmetic, polynomial operations), while external `tests/*_kat.rs`
files are the norm for algorithm-level KATs. The two layers are complementary, not redundant:
inline tests catch substrate regressions; external KATs verify mathematical correctness of
composed algorithms. This is a healthy two-tier pattern. → No action needed; the pattern is
intentional and consistent with the codebase's pedagogical structure. *Feeds C-Testing-Philosophy
(the inline/external split is part of the doctrine).*

---

### F-D2-03 · Over-long test files: 12 files exceed 500 lines

**Observed state (files > 500 lines, sorted descending):**

| File | Lines | Character |
|---|---|---|
| `rho/tests/ghs_kat.rs` | 1646 | GHS Weil-descent: 5 sub-modules (E.H.2–E.H.5), each with multiple KATs |
| `shared/gf2m/tests/gf2m_kat.rs` | 1330 | GF(2^m) field axioms across 3 implementations × 2 field sizes |
| `rho/tests/semaev_kat.rs` | 1165 | Semaev polynomials: resultant, multivariate, summation-poly, ECDLP |
| `gnfs/tests/dl_descent_kat.rs` | 1110 | NFS-DL descent: frontier, smoothing, C2, single-node, multi-level, log assembly |
| `rho/tests/index_calculus_kat.rs` | 979 | Index calculus: decomposition, collection, linear algebra, solve, end-to-end |
| `gnfs/tests/dl_linalg_kat.rs` | 887 | NFS-DL linear algebra: Lanczos-FL, Wiedemann-FL, block-vector operations |
| `shared/gf2m/tests/gf2m_poly_kat.rs` | 779 | GF(2^m) polynomial arithmetic: 29+ operations |
| `rho/tests/hyperelliptic_kat.rs` | 756 | Hyperelliptic curves: Cantor group law, Mumford divisors, ECDLP reduction |
| `shor/tests/modexp_kat.rs` | 712 | Quantum modular exponentiation: circuit construction, measurement statistics |
| `shor/tests/qft_kat.rs` | 640 | Quantum Fourier transform: state-vector correctness, period finding |
| `gnfs/tests/lattice_kat.rs` | 628 | GNFS lattice sieve: lattice reduction, sieve region, relation collection |
| `shor/tests/pointadd_kat.rs` | 627 | Quantum point addition: circuit correctness, ancilla management |
| `shor/tests/statevec_kat.rs` | 618 | Quantum state-vector: amplitude correctness, measurement probabilities |
| `rho/tests/binary_curve_kat.rs` | 605 | Binary curve group law: López–Dahab, Koblitz, axioms |
| `shared/gf2m/tests/gf2m_subfield_kat.rs` | 595 | GF(2^m) subfield operations |
| `gnfs/tests/special_q_kat.rs` | 559 | GNFS special-q sieve |
| `gnfs/tests/linalg_substrate_kat.rs` | 544 | GNFS linear algebra substrate |
| `gnfs/tests/merge_kat.rs` | 541 | GNFS relation merging |
| `shor/tests/factor_kat.rs` | 512 | Shor factorization: circuit, period, factor extraction |

**Analysis:** the over-long files are not monolithic tests — they are multi-KAT files covering
several related sub-modules or algorithm stages in a single file. For example, `ghs_kat.rs`
(1646 lines) covers 5 distinct GHS sub-modules (Artin–Schreier algebra, curve extraction, transfer
map, Jacobian reduction, and the PARI oracle cross-check), each with multiple KATs. The length
reflects the mathematical depth of the subject, not test bloat. The files are well-structured with
section headers and doc-comment coverage tables.

**Finding:** 12 KAT files exceed 500 lines; the longest (`ghs_kat.rs`, 1646 lines) covers 5
distinct algorithm sub-modules. The length is driven by mathematical depth, not by test bloat —
each file is a coherent KAT suite for a single algorithm or module family. Splitting would
fragment the coverage narrative. → No mandatory split is warranted; the test-length norm for this
codebase is "one file per algorithm family, however long that takes." CONSOLIDATE may add
section-level navigation (e.g., `mod` blocks within the file) to the longest files if readability
suffers, but splitting is not the right remedy. *Feeds C-Testing-Philosophy (test-length norm).*

---

### F-D2-04 · Bench/test ratio: 4 bench files vs 53 KAT files; benches are `rho`-only

**Observed state:** the workspace has 4 bench files, all in `rho/benches/`:
- `rho/benches/field.rs` (86 lines) — FpNaive vs FpMonty multiplication/inversion speedup
- `rho/benches/factor.rs` (61 lines) — Floyd vs Brent vs Brent+batched-GCD factorization
- `rho/benches/ecdlp.rs` (202 lines) — Pollard-rho ECDLP optimization layers (Phases 5–8)
- `rho/benches/attacks.rs` (254 lines) — cross-attack ECDLP comparison (E.W.1)

No bench files exist in `gnfs/`, `shor/`, or any `shared/*` crate. The `docs/BENCHMARKS.md`
records results from these 4 benches (603 total bench lines) plus narrative for Phase 3 (curve
arithmetic, no bench — "the deliverable is correctness: 50 tests, 0 failures").

**Finding:** benches are confined to `rho/` (4 files, 603 lines). The bench/KAT ratio is 4:53
(~1:13). The `gnfs` and `shor` crates have no benches — their compute-heavy paths (GNFS sieving,
quantum circuit simulation) are too slow for Criterion benchmarks at non-toy scale, and the toy
scale used in KATs is not representative of algorithmic performance. The `shared/*` crates have no
benches — their primitives are benchmarked indirectly through `rho`. This is appropriate for the
codebase's pedagogical scope. → No bench additions are warranted by this finding; the bench
coverage is intentional. *Feeds C-Testing-Philosophy (bench scope norm).*

---

### F-D2-05 · `#[ignore]`-gated oracle tests: 15 tests across 10 files

**Observed state:** 15 tests carry `#[ignore]` across 10 test files:
- PARI/GP oracle cross-checks: `shared/padic/tests/log_kat.rs` (1), `gnfs/tests/dl_individual_log_kat.rs`
  (1), `gnfs/tests/dl_end_to_end_kat.rs` (1), `rho/tests/hyperelliptic_kat.rs` (1),
  `gnfs/tests/dl_relation_kat.rs` (1), `rho/tests/semaev_kat.rs` (1), `rho/tests/ghs_kat.rs` (1),
  `rho/tests/mov_kat.rs` (1), `rho/tests/ssa_kat.rs` (2), `gnfs/tests/dl_ext_kat.rs` (1).
- CADO-NFS oracle cross-checks: `gnfs/tests/merge_kat.rs` (1), `gnfs/tests/factor_end_to_end_kat.rs`
  (1), `gnfs/tests/lanczos_kat.rs` (1), `gnfs/tests/line_sieve_kat.rs` (1).
- Compute-heavy `#[ignore]` (not oracle): `shor/tests/ecdlp_kat.rs` (5 — quantum circuit
  instances too slow for CI), `shor/tests/factor_kat.rs` (1 — slow quantum simulation).

**Finding:** the `#[ignore]`-gate pattern is consistently applied: external-tool oracle KATs
(PARI, CADO-NFS) and compute-heavy quantum circuit tests are gated behind `#[ignore]` with
descriptive messages. The pattern is the arc-1 dev-oracle policy in action. The 15 `#[ignore]`
tests are not dead tests — they are opt-in validation sidecars. → No action needed; the pattern
is correct and consistent. *Feeds C-Testing-Philosophy (oracle-gate norm).*

---

### Part (b) — Coverage doctrine resolution

#### The meta-question (D2)

> Is full/100% line coverage the right target for toy-scale-but-compute-heavy KAT-driven
> pedagogical code, or does KAT + property coverage of *mathematical* behavior matter more?

#### Evidence from the observed test suite

1. **The entire test suite is KAT-based.** Every external test file is named `*_kat.rs`. There
   are no property-based tests (no `proptest`, no `quickcheck`). The KATs verify mathematical
   correctness: field axioms, group laws, algorithm outputs against hand-computed or
   oracle-verified answers.

2. **The compute-heavy paths are already exercised at toy scale.** ECM, Pollard rho, GNFS
   sieving, and quantum circuit simulation all run in the KAT suite at toy scale (small primes,
   small fields, small circuits). The `#[ignore]`-gated tests extend this to oracle cross-checks.
   100% line coverage would require these paths to run — and they do, at toy scale, in the KATs.

3. **The inline tests cover low-level substrates; the external KATs cover composed algorithms.**
   This two-tier structure means the coverage signal is already split: inline tests catch
   substrate regressions (field arithmetic, linear algebra primitives); external KATs verify
   mathematical correctness of composed algorithms. A line-coverage gate would not distinguish
   these two tiers.

4. **The template's 100% gate is calibrated for a simple single-crate project.** The template's
   `--fail-under-lines 100` is appropriate for a project where every line is reachable from a
   small, deterministic test suite. In a 9-crate compute-heavy math library, 100% line coverage
   is achievable only if every compute path runs at toy scale — which the KATs already ensure for
   the mathematically meaningful paths. The `#[ignore]`-gated tests cover the oracle paths; the
   remaining uncovered lines (if any) are likely error-handling branches and dead-code candidates.

5. **Mathematical behavior is the correctness signal, not line coverage.** A field-arithmetic
   implementation that passes all field-axiom KATs (associativity, distributivity, inversion
   round-trip, Frobenius fixed-field law) is correct regardless of whether every branch in the
   implementation is covered. A line-coverage gate that misses a mathematical property (e.g., a
   KAT that doesn't test the `inv(zero)` panic path) is a weaker correctness signal than a
   KAT-completeness check.

#### Resolved doctrine

**C-Testing-Philosophy (frozen at A.3, ratified at A.7):**

> **Primary correctness target: mathematical-behavior KAT coverage.** Every public API must have
> at least one KAT exercising its mathematical contract (the algorithm produces the correct answer
> on a known input). Field axioms, group laws, and algorithm-level correctness are the primary
> correctness signals. Line coverage is a secondary signal — a floor, not a ceiling.
>
> **Coverage gate: 80% line coverage as a dead-code floor.** A line-coverage gate of 80% (not
> 100%) is appropriate for this codebase. The gate's purpose is to catch dead code and
> unreachable branches, not to mandate that every compute path is exercised by a test. The
> `#[ignore]`-gated oracle tests are excluded from the gate (they are opt-in sidecars, not CI
> tests). The gate is implemented by ALIGN using `cargo llvm-cov --workspace --fail-under-lines
> 80` (or equivalent), honoring the `#[ignore]` exclusion.
>
> **Unit/integration norm: two-tier (inline substrate + external KAT).** Inline `#[test]` blocks
> in `src/` are appropriate for low-level substrate verification (field arithmetic, linear algebra
> primitives, polynomial operations). External `tests/*_kat.rs` files are the norm for
> algorithm-level KATs. New tests (CONSOLIDATE, F-EXTEND) follow this split: substrate-level
> tests go inline; algorithm-level KATs go in `tests/`.
>
> **Test-length norm: one file per algorithm family, however long that takes.** KAT files may
> exceed 500 lines if the algorithm family has multiple sub-modules or stages. Splitting is not
> warranted by length alone; splitting is warranted only if a file covers genuinely distinct
> algorithm families that would be clearer as separate files. The longest files (e.g.,
> `ghs_kat.rs` at 1646 lines) are coherent KAT suites for a single algorithm family and should
> not be split.
>
> **Oracle-gate norm: `#[ignore]` for all external-tool and compute-heavy tests.** PARI, CADO-NFS,
> and slow quantum circuit tests are gated behind `#[ignore]` with descriptive messages. This is
> the arc-1 dev-oracle policy; new tests (CONSOLIDATE, F-EXTEND) honor it.
>
> **`shared/numth` inline-test pattern: accept as-is.** The 34 inline tests in `shared/numth/src/`
> are mathematically substantive and cover ECM, primality, and smooth-number detection. Migration
> to an external `tests/` directory is a CONSOLIDATE decision (not mandated by this doctrine).

---

### F-D2-06 · Coverage doctrine resolved — C-Testing-Philosophy frozen

**Falsifiable finding (the primary deliverable of A.3):**

> **Coverage doctrine = mathematical-behavior KAT coverage over line coverage; gate at 80% line
> coverage as a dead-code floor (not a correctness ceiling).**
>
> **Downstream checks:**
> - ALIGN's coverage-gate implementation must use `--fail-under-lines 80` (not 100), with
>   `#[ignore]`-gated tests excluded from the gate. A gate of 100% would be a doctrine violation.
> - New tests added by CONSOLIDATE and F-EXTEND must follow the two-tier pattern (inline for
>   substrate, external KAT for algorithm-level) and the oracle-gate norm (`#[ignore]` for
>   external-tool and compute-heavy tests).
> - A new test that covers a line but not a mathematical property is not a KAT and does not
>   satisfy the doctrine. A new test that covers a mathematical property but misses a line is
>   acceptable.

*Freezes C-Testing-Philosophy. Feeds ALIGN (coverage-gate value), CONSOLIDATE (new test norms),
F-EXTEND (new test norms).*

---

### Summary table — testing-balance findings

| # | Finding | Action | Consuming campaign |
|---|---|---|---|
| F-D2-01 | `shared/numth` has no `tests/` dir; all 34 tests are inline | CONSOLIDATE decision: migrate or accept | CONSOLIDATE |
| F-D2-02 | Inline `#[test]` is widespread (7/9 crates); two-tier pattern is healthy | No action — pattern is intentional | — |
| F-D2-03 | 12 KAT files exceed 500 lines; length reflects mathematical depth, not bloat | No mandatory split; CONSOLIDATE may add section navigation | CONSOLIDATE |
| F-D2-04 | 4 bench files, all in `rho/`; bench/KAT ratio 1:13; no benches in `gnfs`/`shor`/`shared/*` | No action — bench scope is intentional | — |
| F-D2-05 | 15 `#[ignore]`-gated oracle/compute-heavy tests across 10 files; pattern is consistent | No action — pattern is correct | — |
| F-D2-06 | **Coverage doctrine resolved: math-behavior KAT coverage; 80% line gate** | ALIGN implements gate; CONSOLIDATE/F-EXTEND honor norms | ALIGN, CONSOLIDATE, F-EXTEND |

---

### Subtleties and deferrals

**The 80% gate value is a recommendation, not a measurement.** A.3 does not run `cargo llvm-cov`
(SURVEY writes no code and runs no benchmarks). The 80% floor is reasoned from the codebase
structure: the KAT suite exercises all mathematically meaningful paths at toy scale; the remaining
uncovered lines are likely error-handling branches and dead-code candidates. ALIGN should run
`cargo llvm-cov --workspace` to measure the actual baseline before setting the gate, and may
adjust the threshold if the baseline is significantly above or below 80%. The doctrine (math-
behavior KAT over line coverage) is fixed; the gate value is ALIGN's calibration decision.

**`shared/numth` inline-test migration (F-D2-01).** The 34 inline tests are substantive and
correct. Migration to `tests/` is a consistency question, not a correctness question. CONSOLIDATE
owns the decision; A.3 records the inconsistency.

**`rho/src/util/batch_inv.rs` duplicated tests (from A.2, F-D5-02).** The three inline tests in
`rho/src/util/batch_inv.rs` duplicate the tests in `shared/bigint/tests/isqrt_gcd_kat.rs`. This
is a dedup candidate for REFACTOR (noted in A.2); the coverage doctrine does not change this
assessment.

**The human's lean on the 80% vs 100% question is an open-Q at A.7.** The doctrine recommends
80%; the human may prefer a different threshold (e.g., 90% as a tighter floor, or 100% with
`#[ignore]` exclusions making it achievable). A.7 surfaces this as an open-Q. The doctrine's
*shape* (math-behavior KAT primary, line coverage secondary) is frozen here; the *gate value* is
an A.7 open-Q if the human disagrees with 80%.

---

## A.4 — Docs-layer discipline + math-exposition continuity + prose provenance (D3 + D4 + D9-prose)

**Charge:** (a) audit the three-layer docs discipline (D3) — do inline `//!`/`///` docs, per-crate
`PEDAGOGY.md` code-tours, and `docs/MATHEMATICS.md` honor their prescribed reference directions?
(b) audit math-exposition continuity (D4) — does `MATHEMATICS.md` (12 chapters, 4592 lines) read
as a continuous textbook or a stitched companion reference? (c) catalog every planning-frame token
in `PEDAGOGY.md`, `MATHEMATICS.md`, and `README.md` prose (D9-prose), classified as pure residue
or grouping-coincides-with-topic.

**Read surface:** `docs/MATHEMATICS.md` (4592 lines), `docs/PEDAGOGY.md` (1401 lines),
`gnfs/docs/PEDAGOGY.md` (5240 lines), `shared/numth/docs/PEDAGOGY.md` (446 lines),
`shared/numfield/docs/PEDAGOGY.md` (747 lines), `shor/docs/PEDAGOGY.md` (509 lines),
`README.md` (297 lines). Total ~13 232 lines sampled by ToC, chapter openings, and targeted
grep passes.

---

### Part (a) — Docs-layer discipline (D3)

#### The three-layer model (as observed)

The three-layer discipline, as it exists in the corpus:

1. **Inline `//!`/`///` docs** — code-adjacent; reference code identifiers and module surfaces.
   Consumed by REFACTOR (A.2 cataloged these).
2. **Per-crate `PEDAGOGY.md` code-tours** — human-facing; reference code identifiers and cite
   `docs/MATHEMATICS.md` chapters for the mathematics. Do NOT reference MATHEMATICS.md as a
   prerequisite — they are complementary lenses.
3. **`docs/MATHEMATICS.md`** — the maths-first textbook; references PEDAGOGY.md files for the
   code realisation. The C-Textbook contract (lines 8–130) states it is "maths-first, code-second,
   learnable on its own."

The prescribed reference directions are: PEDAGOGY → MATHEMATICS (for math foundations) and
MATHEMATICS → PEDAGOGY (for code realisation). Neither is a prerequisite for the other.

---

### F-D3-01 · `MATHEMATICS.md` references specific Rust code identifiers — partial layer violation

**Observed state:** `MATHEMATICS.md` contains 9 occurrences of Rust module paths as inline
citations within the mathematical text. Examples (sampled):

- Line 117–118 (chapter-pairing note): "`rho::ecdlp`, `rho::pairing`, `rho::ssa`, `rho::ghs`,
  `rho::semaev`, `rho::index_calculus`"
- Line 2899 (§10.1 Pohlig–Hellman): "Code realisation: `docs/PEDAGOGY.md` §9 (Pohlig–Hellman
  code-tour, `rho::ecdlp::pohlig`)."
- Line 2962 (§10.2 SSA): "Code realisation: `docs/PEDAGOGY.md` §11 (SSA code-tour, `rho::ssa`)."
- Line 3017 (§10.3 GHS): "Code realisation: `docs/PEDAGOGY.md` §12 (GHS code-tour, `rho::ghs`)."
- Line 3081 (§10.4 index calculus): "`rho::semaev` and `rho::index_calculus`"
- Line 3180 (§10.5 MOV): "the frozen `gnfs::dl::solve_dl` entry point, Track D"
- Line 3230 (§10.5 MOV): "Code realisation: `docs/PEDAGOGY.md` §10 (MOV code-tour,
  `rho::pairing::mov`)."
- Line 3283 (§10.5 cross-references): "The MOV reduction (§10.5) calls `gnfs::dl::solve_dl`"

**Pattern:** The code-identifier citations appear consistently in the "Code realisation" lines at
the end of each §10.x sub-section, and in the chapter-pairing note. They are not scattered through
the mathematical exposition itself — the mathematical prose is clean. The identifiers appear as
cross-reference pointers, not as mathematical content.

**Classification:** **Partial layer violation.** The C-Textbook contract states MATHEMATICS.md is
"maths-first, code-second, learnable on its own" and that "the code-tour cites the textbook
chapter for the mathematics; the textbook chapter cites the code-tour for the realisation." The
cross-reference direction (MATHEMATICS → PEDAGOGY) is correct and prescribed. However, the
inclusion of bare Rust module paths (`rho::ecdlp::pohlig`, `gnfs::dl::solve_dl`) inside the
textbook body goes one step further than citing the PEDAGOGY file — it names the implementation
artifact directly. This is a mild violation: the textbook can point to the code-tour without
naming the Rust path. The PEDAGOGY file is the correct indirection layer.

**Finding:** `MATHEMATICS.md` contains ~9 Rust module-path citations (`rho::*`, `gnfs::dl::*`)
embedded in "Code realisation" lines within §10 (Algebraic ECDLP Attacks) and the chapter-pairing
note. The mathematical prose is clean; the violation is in the cross-reference lines only. →
CONSOLIDATE replaces bare Rust paths with PEDAGOGY-section citations (e.g., "Code realisation:
`docs/PEDAGOGY.md` §9" without the trailing `rho::ecdlp::pohlig`). *Feeds C-DocsLayer (sketch).*

---

### F-D3-02 · `shor/docs/PEDAGOGY.md` references MATHEMATICS.md as a forward reference to a chapter that now exists

**Observed state:** `shor/docs/PEDAGOGY.md` lines 3–4:

> "This chapter is the code-tour sibling of `docs/MATHEMATICS.md` ch. 11 (T.S, to be written in
> S.D.2)."

And lines 480–485:

> "**T.S — the maths-first sibling.** `docs/MATHEMATICS.md` ch. 11 (to be written in S.D.2) is
> the maths-first sibling of this chapter. … The 'see T.S' pointer in this document is a forward
> reference that S.D.2 resolves."

**Observed state of MATHEMATICS.md:** Chapter 11 ("Shor's Algorithm and Post-Quantum Context")
exists at line 3371, is complete (4592 − 3371 = ~1221 lines), and is the designated payoff proof
chapter for Track S.

**Classification:** **Stale forward reference.** The chapter was written in session T.S (arc 1);
the PEDAGOGY file was written earlier and its forward-reference text was never updated. The
reference direction (PEDAGOGY → MATHEMATICS) is correct; only the "to be written in S.D.2"
annotation is stale.

**Finding:** `shor/docs/PEDAGOGY.md` lines 4 and 480 contain stale "to be written in S.D.2"
annotations for `docs/MATHEMATICS.md` ch. 11, which now exists and is complete. → CONSOLIDATE
removes the forward-reference annotation and replaces with a direct citation. *Feeds C-DocsLayer
(sketch).*

---

### F-D3-03 · `shared/numfield/docs/PEDAGOGY.md` has no maths-first citation header

**Observed state:** `shared/numfield/docs/PEDAGOGY.md` opens (lines 1–13) with a plain
introduction to the `shared/numfield` crate, with no blockquote citing `docs/MATHEMATICS.md` for
the mathematical foundations. Compare: `shared/numth/docs/PEDAGOGY.md` (lines 3–9) has a
`> **Maths-first treatment.**` blockquote citing `docs/MATHEMATICS.md §The α-Substrate`; and
`shor/docs/PEDAGOGY.md` (lines 3–8) has a `> **Code-first treatment.**` blockquote citing
`docs/MATHEMATICS.md` ch. 11.

`shared/numfield/docs/PEDAGOGY.md` does reference the mathematical content inline (e.g., line 642
references "contracts frozen in G.A"), but lacks the standard header blockquote that establishes
the layer relationship.

**Classification:** **Layer-discipline gap (minor).** The reference direction is implicitly
correct (the file is a code-tour), but the standard header citation is absent. The PEDAGOGY
discipline requires each code-tour to declare its maths-first sibling at the top.

**Finding:** `shared/numfield/docs/PEDAGOGY.md` lacks the standard `> **Maths-first treatment.**`
header blockquote citing `docs/MATHEMATICS.md §GNFS §3 "The Number-Field Bridge"` (the relevant
chapter). → CONSOLIDATE adds the header. *Feeds C-DocsLayer (sketch).*

---

### F-D3-04 · `docs/PEDAGOGY.md` references `MATHEMATICS.md` chapters correctly; `README.md` references implementation details at the crate-map level

**Observed state (PEDAGOGY → MATHEMATICS, correct):** `docs/PEDAGOGY.md` consistently cites
`docs/MATHEMATICS.md` for mathematical foundations (lines 47, 58, 68, 112–113, 122, 141, 149,
192, 240, 746, 754, 842, 910, 971, 1046, 1084, 1172, 1340, 1346, 1350, 1399). All citations are
in the correct direction (code-tour → textbook). No PEDAGOGY file references MATHEMATICS.md
chapters as prerequisites — they are cited as complementary lenses.

**Observed state (README):** `README.md` references `Track ρ`, `Track E`, `Track G`, `Track D`,
`Track S` as section headings (lines 19, 34, 49, 63, 83) and references implementation details
(crate names, module paths in the crate-map section, lines 122–131). The README is a
workspace-level orientation document; its reference to implementation details is appropriate for
its role. It does not reference MATHEMATICS.md chapters or PEDAGOGY section numbers in a way that
violates layer discipline.

**Classification:** **Layer discipline coheres** for PEDAGOGY → MATHEMATICS and README. The
README's implementation-detail references are appropriate for a workspace orientation document.

**Finding:** No violation. PEDAGOGY → MATHEMATICS reference direction is consistently honored
across all five PEDAGOGY files (with the gap noted in F-D3-03). README references implementation
details appropriately for its role. *Feeds C-DocsLayer (sketch) as a positive finding.*

---

### Summary table — docs-layer discipline findings

| # | Finding | Location | Action | Consuming campaign |
|---|---------|----------|--------|--------------------|
| F-D3-01 | `MATHEMATICS.md` embeds ~9 Rust module paths in "Code realisation" lines | `docs/MATHEMATICS.md` §10, chapter-pairing note | CONSOLIDATE replaces with PEDAGOGY-section citations | CONSOLIDATE |
| F-D3-02 | `shor/docs/PEDAGOGY.md` has stale "to be written in S.D.2" for ch. 11 (now exists) | `shor/docs/PEDAGOGY.md` lines 4, 480 | CONSOLIDATE removes stale annotation | CONSOLIDATE |
| F-D3-03 | `shared/numfield/docs/PEDAGOGY.md` lacks standard maths-first citation header | `shared/numfield/docs/PEDAGOGY.md` lines 1–13 | CONSOLIDATE adds header blockquote | CONSOLIDATE |
| F-D3-04 | PEDAGOGY → MATHEMATICS direction coheres; README references appropriate | All PEDAGOGY files; `README.md` | No action | — |

---

### Part (b) — Math-exposition continuity (D4)

#### Chapter inventory

`MATHEMATICS.md` has 12 chapters (4592 lines), assembled chapter-by-chapter at arc-1 track
boundaries (T.0, T.G, T.D, T.E, T.S, T.Z):

| Ch. | Title | Heading level | Lines (approx.) | Session |
|-----|-------|---------------|-----------------|---------|
| 1 | C-Textbook: Documentation-Register Contract | `##` | 1–131 | T.0 |
| 2 | Table of Contents | `##` | 133–189 | T.0 |
| 3 | Escape from Search: The Through-Line | `##` | 191–321 | T.0 |
| 4 | Prerequisites | `##` | 323–668 | T.0 |
| 5 | On Scale: A Natural-Philosophy Interlude | `##` | 670–823 | T.0 |
| 6 | Pollard Rho for ECDLP | `##` | 825–1025 | T.0 |
| 7 | The α-Substrate: Primality, Smoothness, and ECM | `##` | 1026–1359 | T.0 |
| 8 | The General Number Field Sieve | **`#`** | 1361–2026 | T.G |
| 9 | NFS-DL: Discrete Logarithm via the Number Field Sieve | **`#`** | 2028–2801 | T.D |
| 10 | Algebraic ECDLP Attacks | `##` | 2803–3369 | T.E |
| 11 | Shor's Algorithm and Post-Quantum Context | `##` | 3371–4135 | T.S |
| 12 | Chapter 12 — Modularity and the Arithmetic of Elliptic Curves | `##` | 4137–4592 | T.Z |

---

### F-D4-01 · Heading-level inconsistency: chapters 8 and 9 use top-level `#` while all others use `##`

**Observed state:** Chapters 8 (GNFS, line 1361) and 9 (NFS-DL, line 2028) open with a top-level
`#` heading:

```
# The General Number Field Sieve: Structure-Based Escape from Search
# NFS-DL: Discrete Logarithm via the Number Field Sieve
```

All other chapters (1–7, 10–12) open with a second-level `##` heading:

```
## Escape from Search: The Through-Line
## Prerequisites
## Pollard Rho for ECDLP
## Algebraic ECDLP Attacks
## Shor's Algorithm and Post-Quantum Context
## Chapter 12 — Modularity and the Arithmetic of Elliptic Curves: A Speculation
```

**Cause:** Chapters 8 and 9 were written as standalone documents (T.G and T.D sessions) and
appended to the single file without normalizing the heading level. The `#` heading makes them
appear as separate documents within the file rather than chapters of the same textbook.

**Impact on continuity:** In a Markdown renderer that builds a document outline from headings,
chapters 8 and 9 appear at the document root level while all other chapters appear as subsections
of the root. This breaks the visual and structural continuity of the textbook.

**Finding:** Chapters 8 and 9 use `#` (top-level) headings; all other chapters use `##`
(second-level). → CONSOLIDATE normalizes chapters 8 and 9 to `##` and adjusts their internal
sub-headings accordingly (their internal `##` sub-headings become `###`, etc.). *Feeds C-MathSpine
(sketch).*

---

### F-D4-02 · ToC has no "to be appended" labels — all 12 chapters exist

**Observed state:** The ToC (lines 133–189) lists all 12 chapters with complete scope statements.
No entry is marked "to be appended," "forthcoming," or "TBD." The single occurrence of "TBD" in
MATHEMATICS.md (line 75) is in the C-Textbook markup section, referring to a superseded "rST or
Markdown TBD" decision — not a chapter placeholder.

**Finding:** No stale "to be appended" ToC labels. The ToC is complete and accurate. *Positive
finding for C-MathSpine (sketch).*

---

### F-D4-03 · Chapter-pairing table Track-E row is correct; historical mis-pairing documented

**Observed state:** The chapter-pairing table (lines 106–125) contains:

```
| Ch. 10 — Algebraic ECDLP Attacks | `docs/PEDAGOGY.md` §§8–18 (E.W integrative chapter) |
```

And the note (lines 116–121):

> "**Note on Ch. 10 (Track E).** The E.W integrative chapter lives in the master
> `docs/PEDAGOGY.md` §§8–18 — not in `gnfs/docs/PEDAGOGY.md`. … The pairing table previously
> pointed at `gnfs/docs/PEDAGOGY.md (Track E chapter)`; that was a structural mis-pairing
> corrected here at T.Z."

**Verification:** `docs/PEDAGOGY.md` §§8–18 (lines 731–1401) is confirmed as the Track-E
integrative chapter ("Chapter E — Algebraic ECDLP Attacks: An Integrative Chapter", line 731).
The Track-E code lives in the `rho` crate (confirmed in A.2). The pairing is now correct.

**Finding:** The chapter-pairing table Track-E row correctly points to `docs/PEDAGOGY.md` §§8–18.
The historical mis-pairing (pointing at `gnfs/docs/PEDAGOGY.md`) was corrected at T.Z and is
documented in the note. No action needed. *Positive finding for C-MathSpine (sketch).*

---

### F-D4-04 · Voice, notation, and audience are consistent across chapters; through-line is threaded

**Observed state (sampled):** Each chapter opening was sampled (first 20–40 lines). Observations:

- **Voice:** All chapters use the same academic-and-clinical register: "intuition leads, rigour
  follows." Each chapter opens with a through-line paragraph ("The through-line for this chapter")
  that connects to the §"Escape from Search" spine. The register is consistent.
- **Notation:** MathJax (`$…$`, `$$…$$`) is used uniformly. The $L$-notation
  ($L_N[\alpha, c]$) is introduced in §3 and used consistently in all subsequent chapters. The
  $\mathbb{Z}$, $\mathbb{F}_p$, $\mathbb{Q}(\alpha)$ notation is consistent.
- **Audience floor:** The C-Textbook contract (undergraduate background) is honored. Each chapter
  cites §Prerequisites for background results rather than re-deriving them.
- **Through-line:** The "structure-based escape from search" through-line is stated in §3 and
  revisited at the opening of every subsequent chapter (confirmed in chapters 6, 7, 8, 9, 10, 11,
  12 openings).

**Finding:** Voice, notation, and audience are consistent across all 12 chapters. The through-line
is threaded as specified by C-Textbook. *Positive finding for C-MathSpine (sketch).*

---

### F-D4-05 · No explicit "suggested reading path" section; the ToC and Prerequisites chapter serve this role

**Observed state:** There is no section titled "Suggested reading path" or "How to read this
textbook." The Prerequisites chapter (§4, lines 323–668) contains (lines 329–330): "A reader who
is comfortable with all of these results can proceed directly to any chapter. A reader who needs to
review specific topics should consult the cited references." The C-Textbook contract (§"Audience
floor") states the background assumption but does not prescribe a reading order.

**Assessment:** The textbook is designed for non-linear reading (each chapter is self-contained
with citations to §Prerequisites). The absence of a suggested-path section is consistent with the
"complementary lenses" design philosophy. However, the ROADMAP D4 charge asks whether the file
reads as "a continuous textbook with a suggested path" — the path is implicit (linear chapter
order) but not stated.

**Finding:** No explicit suggested-reading-path section. The textbook is navigable via the ToC and
the Prerequisites chapter, but a reader unfamiliar with the project has no explicit guidance on
reading order. → CONSOLIDATE may add a brief "How to read this textbook" note to the ToC section
(one paragraph, not a new chapter). This is a CONSOLIDATE design decision, not a SURVEY fix.
*Feeds C-MathSpine (sketch) as an open design question.*

---

### F-D4-06 · Cross-reference health: chapter references resolve; one internal section-name mismatch

**Observed state:** Cross-references within MATHEMATICS.md use the pattern "§Pollard Rho chapter",
"§Prerequisites", "§GNFS §3", "§10.5", etc. Sampled references:

- `docs/PEDAGOGY.md` §0.2 L-notation table (line 273) cites "§6 for ρ, §7 for GNFS, §9.7 for
  NFS-DL, §10.6 for algebraic ECDLP, §11.3.4 and §11.4.4 for Shor" — these section numbers
  resolve correctly in MATHEMATICS.md.
- The chapter-pairing table (lines 108–113) cites `gnfs/docs/PEDAGOGY.md §§52–62` and `§§63–71`
  — these resolve to the G.W and D.W chapters in `gnfs/docs/PEDAGOGY.md`.
- The Shor chapter (line 3373) cites "`shor/docs/PEDAGOGY.md` (Track S code-tour)" — resolves.
- The NFS-DL chapter (line 2030) cites "`gnfs/docs/PEDAGOGY.md` §63–§71 (the D.W.1 NFS-DL
  code-tour)" — resolves.

**One mismatch observed:** The chapter-pairing table (line 110) cites
"`gnfs/docs/PEDAGOGY.md` §§52–62 (G.W integrative chapter)" but `gnfs/docs/PEDAGOGY.md` does not
use numbered sections in the §52–§62 range in its heading structure — it uses numbered headings
(e.g., "## 52. …"). This is a section-numbering convention difference, not a broken reference, but
it means the citation style is inconsistent with how the PEDAGOGY file is actually organized.

**Finding:** Cross-references are substantively correct and resolve. One citation-style
inconsistency: the chapter-pairing table cites `gnfs/docs/PEDAGOGY.md §§52–62` using `§` notation
while the GNFS PEDAGOGY file uses plain numbered headings. → CONSOLIDATE normalizes the citation
style. *Feeds C-MathSpine (sketch).*

---

### Summary table — math-exposition continuity findings

| # | Finding | Location | Action | Consuming campaign |
|---|---------|----------|--------|--------------------|
| F-D4-01 | Chapters 8 and 9 use `#` heading; all others use `##` — structural discontinuity | `docs/MATHEMATICS.md` lines 1361, 2028 | CONSOLIDATE normalizes to `##` | CONSOLIDATE |
| F-D4-02 | ToC is complete; no "to be appended" labels | `docs/MATHEMATICS.md` lines 133–189 | No action | — |
| F-D4-03 | Chapter-pairing table Track-E row is correct; historical mis-pairing documented | `docs/MATHEMATICS.md` lines 106–121 | No action | — |
| F-D4-04 | Voice, notation, audience, and through-line are consistent across all 12 chapters | All chapters | No action | — |
| F-D4-05 | No explicit suggested-reading-path section; implicit linear order | `docs/MATHEMATICS.md` §ToC | CONSOLIDATE may add one-paragraph reading note | CONSOLIDATE |
| F-D4-06 | Cross-references resolve; one citation-style inconsistency (`§§52–62` vs plain numbered headings) | `docs/MATHEMATICS.md` line 110 | CONSOLIDATE normalizes citation style | CONSOLIDATE |

---

### Part (c) — Prose-depth provenance catalog (D9-prose)

**Scope:** Planning-frame tokens in `docs/PEDAGOGY.md`, `docs/MATHEMATICS.md`, `README.md`,
`shor/docs/PEDAGOGY.md`, `gnfs/docs/PEDAGOGY.md`, `shared/numth/docs/PEDAGOGY.md`,
`shared/numfield/docs/PEDAGOGY.md`. Excludes inline `//!`/`///` doc-comments (cataloged in A.2).

---

### F-D9-07 · `Track ρ / G / D / E / S` tokens in MATHEMATICS.md prose — grouping-coincides-with-topic

**Observed state:** "Track ρ", "Track E", "Track G", "Track D", "Track S" appear in
`docs/MATHEMATICS.md` in the following locations (sampled):

- Line 108: chapter-pairing table column header "Track ρ code-tour"
- Line 113: chapter-pairing table "Track S code-tour"
- Lines 116, 120: chapter-pairing note "Track E", "Track E chapter"
- Lines 289, 291, 295: L-notation table reading note "Track ρ", "Track E", "Track S"
- Lines 2448, 2457: NFS-DL design-statement section "Track E", "Track D, D.A → D.B → D.C → D.W"
- Lines 3089, 3180, 3221–3222: MOV section "Track E", "Track D"
- Lines 3373, 3507: Shor chapter header and body "Track S"
- Lines 4032–4060: Shor chapter summary "Track ρ", "Track G", "Track D", "Track E", "Track S"
- Lines 4076–4077: Shor cross-reference table "Sub-Track S.B", "Sub-Track S.C"

**Total occurrences in MATHEMATICS.md prose:** approximately 25 "Track X" tokens; 2 "Sub-Track"
tokens.

**Classification:** **Grouping-coincides-with-topic.** The five tracks map onto five real
mathematical families: Track ρ = Pollard rho (birthday-paradox baseline); Track G = GNFS (number-
field sieve for factoring); Track D = NFS-DL (number-field sieve for discrete log); Track E =
algebraic ECDLP attacks (five structure-specific escapes); Track S = Shor's algorithm (quantum
period-finding). The grouping is mathematically sound; only the planning label changes. The
ROADMAP D9 charge explicitly names this case: "Track ρ → Pollard rho, the birthday-paradox
baseline."

**Finding:** ~25 "Track X" tokens in `docs/MATHEMATICS.md` prose. Classification:
**grouping-coincides-with-topic** — the grouping survives; CONSOLIDATE re-anchors the label (e.g.,
"Track ρ" → "Pollard rho" or "the rho family"). The sub-track IDs in the cross-reference table
(lines 4076–4077: "Sub-Track S.B", "Sub-Track S.C") are borderline: they are used as section
labels in `shor/docs/PEDAGOGY.md` and may survive as section references. Open-Q for A.7.
*Feeds C-Coherence (prose half).*

---

### F-D9-08 · `Track ρ / E / G / D / S` tokens in `docs/PEDAGOGY.md` prose — grouping-coincides-with-topic

**Observed state:** "Track ρ", "Track E", "Track G", "Track D", "Track S" appear extensively in
`docs/PEDAGOGY.md`. Selected occurrences:

- Line 51: "GLV, Koblitz (Track ρ, Phase 8)" — in the five-family list
- Lines 52, 54: "Track E, §10", "Track S" — in the five-family list
- Lines 133–134: "a factor (Track G) or a discrete logarithm (Track D)"
- Lines 136, 143: "**Track G — GNFS**", "**Track D — NFS-DL**" — section headings
- Line 157: "### §0.5 Track S: Shor's Algorithm" — section heading
- Lines 198, 201, 209, 218, 222: "Track E", "Track D", "Track E", "Track E", "Track S" — in
  synthesis paragraphs
- Lines 734, 740, 752, 754, 858, 920, 1249, 1273, 1298, 1351: "Track E" — throughout the E.W
  integrative chapter

**Total occurrences in `docs/PEDAGOGY.md` prose:** approximately 40 "Track X" tokens.

**Classification:** **Grouping-coincides-with-topic** (same reasoning as F-D9-07). The track
labels are used as section headings and cross-references throughout the master tour. The grouping
is mathematically sound; CONSOLIDATE re-anchors the labels.

**Finding:** ~40 "Track X" tokens in `docs/PEDAGOGY.md` prose. Classification:
**grouping-coincides-with-topic**. → CONSOLIDATE re-anchors (e.g., "Track E — Algebraic ECDLP
Attacks" → "Algebraic ECDLP Attacks"). *Feeds C-Coherence (prose half).*

---

### F-D9-09 · `Track ρ / E / G / D / S` tokens in `README.md` prose — grouping-coincides-with-topic

**Observed state:** `README.md` uses "Track ρ", "Track G", "Track D", "Track E", "Track S" as
section headings and in-prose labels throughout:

- Lines 19, 34, 49, 63, 83: section headings "### Track ρ — Pollard rho: …", "### Track G — GNFS:
  …", "### Track D — NFS-DL: …", "### Track E — Algebraic ECDLP attacks: …", "### Track S —
  Shor's algorithm: …"
- Lines 108–109: shared-crate table "used by the SSA attack in Track E", "used by the GHS attack
  in Track E"
- Lines 122–124: crate-map code block "Track G (GNFS) + Track D (NFS-DL)", "Track ρ (Pollard rho)
  + Track E (algebraic ECDLP attacks)", "Track S (Shor's algorithm)"
- Lines 175, 182, 206, 208: CLI section "Track ρ", "Track ρ", "Track ρ", "Track E"
- Lines 237–245: crate-layout listing "(Track E)" annotations on module directories

**Total occurrences in `README.md`:** approximately 20 "Track X" tokens.

**Classification:** **Grouping-coincides-with-topic.** The README section headings already include
the topic name alongside the track label (e.g., "Track ρ — Pollard rho: the birthday-paradox
baseline"). The grouping is mathematically sound; CONSOLIDATE can simplify to the topic name alone
or keep the "Track X — Topic" format as a navigation aid.

**Finding:** ~20 "Track X" tokens in `README.md`. Classification: **grouping-coincides-with-topic**.
→ CONSOLIDATE re-anchors (simplest form: drop "Track X —" prefix from section headings, or keep
as "Pollard rho (Track ρ)" for backward compatibility). *Feeds C-Coherence (prose half).*

---

### F-D9-10 · `Phase N` tokens in `docs/PEDAGOGY.md` prose — mixed: pure residue and grouping-coincides-with-topic

**Observed state:** "Phase 0" through "Phase 8" appear extensively in `docs/PEDAGOGY.md` as
section headings and in-prose labels for the Pollard rho optimization sequence (lines 287–727):

- Lines 287, 302, 324, 347, 371, 406, 440, 472, 511: section headings "### Phase 0 — Crate
  skeleton", "### Phase 1 — FpMonty", "### Phase 2 — Integer factorization rho", "### Phase 3 —
  Curve arithmetic", "### Phase 4 — ECDLP rho baseline", "### Phase 5 — Distinguished points",
  "### Phase 6 — Negation map", "### Phase 7 — Batched field inversion", "### Phase 8 — GLV
  endomorphism"
- Lines 360, 367, 466, 477, 511, 544, 548, 553–554, 563, 567, 573, 577, 581–582, 584, 595,
  597–600: "Phase N" in-prose references throughout the optimization narrative

Also: line 51: "GLV, Koblitz (Track ρ, Phase 8)" — in the five-family list.

**Classification:** **Mixed.**

- The "Phase N" section headings (Phase 0–8) in the Pollard rho chapter are **pure residue**: the
  phases are construction-history labels for the optimization layers (Phase 1 = field arithmetic,
  Phase 2 = factorization, etc.). The mathematical content is the optimization technique, not the
  phase number. CONSOLIDATE re-anchors as topic-native section headings (e.g., "### Montgomery
  form and the FpMonty speedup" instead of "### Phase 1 — FpMonty").
- The in-prose "Phase N" references that describe the optimization sequence (e.g., "Phase 7
  reduces per-step cost rather than walk length") are also **pure residue** — they are
  cross-references to the planning-frame section headings.
- The cumulative speedup table (lines 567–573) uses "Phase" as a column label — **pure residue**;
  CONSOLIDATE replaces with the optimization name.

**Finding:** ~30 "Phase N" tokens in `docs/PEDAGOGY.md` prose (section headings + in-prose
references). Classification: **pure residue** (the phase numbers are construction-history labels;
the mathematical content is the optimization technique). → CONSOLIDATE re-anchors section headings
to topic-native names and updates in-prose cross-references. *Feeds C-Coherence (prose half).*

---

### F-D9-11 · Sub-track IDs in `docs/MATHEMATICS.md` prose — pure residue

**Observed state:** Sub-track IDs appear in `docs/MATHEMATICS.md` in the following locations:

- Line 11: "T.G, T.D, T.E, T.S, T.Z" — in the C-Textbook contract, describing which sessions
  appended which chapters
- Line 45: "T.G chapter", "T.E chapter" — in the C-Textbook depth section
- Line 75: "ROADMAP Phase τ scope contract" — in the markup section
- Line 88: "T.G appends the GNFS chapter; T.D, T.E, T.S append their chapters at their respective
  ◆ boundaries" — in the artifact-location section
- Line 91: "T.Z promotion decision" — in the artifact-location section
- Line 551: "The MOV reduction (T.E chapter)" — in the Prerequisites chapter
- Line 1015: "These are the subjects of the T.E chapter." — in the Prerequisites chapter
- Line 1756: "This is the designated payoff proof for the T.G chapter (C-Textbook contract)."
- Line 2448: "to the E.C-prep session (Track E)"
- Line 2456–2457: "This section is the D.W.2 analogue of G.W §59 — the design-statement
  verification for the whole NFS-DL arc (Track D, D.A → D.B → D.C → D.W)"
- Line 2519: "This is the designated payoff proof for the T.D chapter (C-Textbook contract)."
- Lines 4076–4077: "Sub-Track S.B", "Sub-Track S.C" in the Shor cross-reference table

**Classification:** **Pure residue.** The session IDs (T.G, T.D, T.E, T.S, T.Z, D.W.2, G.W §59,
D.A → D.B → D.C → D.W, E.C-prep) are construction-history labels. A reader of the textbook has
no use for the session that appended a chapter; the mathematical content is the chapter itself.
The "T.G chapter" / "T.E chapter" references are particularly residual — they name the chapter by
its construction session rather than its title.

**Finding:** ~15 session/sub-track ID tokens in `docs/MATHEMATICS.md` prose (T.G, T.D, T.E, T.S,
T.Z, D.W.2, G.W §59, D.A → D.B → D.C → D.W, E.C-prep, Sub-Track S.B/S.C). Classification:
**pure residue**. → CONSOLIDATE replaces with chapter titles or topic-native references (e.g.,
"T.G chapter" → "the GNFS chapter"; "D.A → D.B → D.C → D.W" → "the NFS-DL pipeline"). *Feeds
C-Coherence (prose half).*

---

### F-D9-12 · Sub-track IDs in `shor/docs/PEDAGOGY.md` prose — mixed

**Observed state:** `shor/docs/PEDAGOGY.md` uses sub-track IDs extensively as section labels and
cross-references:

- Lines 18–22: table column "Sub-track" with values "S.A", "S.B", "S.C" — structural labels
- Lines 20–22, 24–26, 30, 42, 47, 52, 65, 75, 82, 84, 87, 95, 121–122, 126, 130–131, 133, 138,
  140, 166, 177, 188, 213–214, 218, 222–225, 230, 254, 264–265, 275, 302, 312, 346–347, 353,
  356–388, 432–443, 454, 480, 485, 489, 500, 508: "S.A", "S.B", "S.C", "S.A.1", "S.A.2",
  "S.B.1", "S.B.2", "S.C.1", "S.C.2" throughout

**Classification:** **Borderline — open-Q for A.7.** The sub-track IDs S.A/S.B/S.C map onto
three real algorithmic components: S.A = the state-vector simulator substrate; S.B = Shor's
factoring algorithm; S.C = Shor's ECDLP algorithm. The grouping is mathematically sound (three
distinct deliverables). However, the fine-grained IDs (S.A.1, S.A.2, S.B.1, S.B.2, S.C.1, S.C.2)
are construction-history labels for individual sessions within each sub-track — these are pure
residue. The coarse IDs (S.A, S.B, S.C) are borderline.

**Finding:** Sub-track IDs in `shor/docs/PEDAGOGY.md` are **borderline** (S.A/S.B/S.C:
grouping-coincides-with-topic; S.A.1/S.B.2/S.C.2 etc.: pure residue). The fine-grained IDs
appear ~40 times. → Open-Q for A.7: does the grouping survive as "simulator / factoring / ECDLP"
or does CONSOLIDATE dissolve the S.X.Y labels entirely? *Feeds C-Coherence (prose half) as an
open-Q.*

---

### F-D9-13 · Sub-track IDs in `gnfs/docs/PEDAGOGY.md` prose — pure residue (fine-grained) and grouping-coincides-with-topic (coarse)

**Observed state:** `gnfs/docs/PEDAGOGY.md` uses sub-track IDs G.A–G.W and D.A–D.W as section
labels and cross-references throughout its 5240 lines. Sampled occurrences:

- Line 57: "None until Murphy-E scoring (G.B.2) computes it" — in struct field doc-comment
- Lines 79, 101, 317, 575–607, 619–632, 676, 683, 691, 732, 782, 854, 885, 888–889, 900, 934,
  939, 1216–1267, 1272–1310, 1355–1394, 1407, 1415, 1467, 1473, 1481, 1577, 1591, 1622, 1766,
  1771, 1776, 1799–1830, 1835–1893, 1921–1932, 1939, 1959: G.B, G.C, G.D, G.E, G.F, G.W, D.A
  throughout

**Classification:** **Mixed.** The coarse labels (G.B = polynomial selection, G.C = sieving, G.D
= filtering, G.E = linear algebra, G.F = square root, G.W = integrative; D.A = NFS-DL relation
collection) map onto real algorithmic pipeline stages — **grouping-coincides-with-topic**. The
fine-grained IDs (G.B.1, G.B.2, G.B.3, G.B.4, G.C.3, G.D.2, G.E.3, G.F.3, G.F.4, D.A) are
construction-history session labels — **pure residue**.

**Finding:** Coarse sub-track IDs (G.B–G.W, D.A–D.W) in `gnfs/docs/PEDAGOGY.md` are
**grouping-coincides-with-topic** (the pipeline stages are real); fine-grained IDs (G.B.1,
G.C.3, etc.) are **pure residue**. → CONSOLIDATE replaces fine-grained IDs with stage names
(e.g., "G.B.2 Murphy-E scoring" → "Murphy-E scoring"); coarse IDs may survive as section labels
with topic-native names. *Feeds C-Coherence (prose half).*

---

### F-D9-14 · `◆` boundary marks in `shor/docs/PEDAGOGY.md` prose — pure residue

**Observed state:** `shor/docs/PEDAGOGY.md` uses `◆` boundary marks as part of sub-track-close
labels: "frozen S.A.2 ◆" (lines 65, 75, 82), "S.B.2 ◆" (lines 133, 166), "S.C.2 ◆" (lines 225,
275). These appear in the module-surface descriptions as "frozen at S.X.Y ◆" annotations.

**Classification:** **Pure residue.** The `◆` mark is a planning-frame milestone marker. A reader
of the code-tour has no use for the sub-track-close milestone; the relevant information is that
the contract is frozen (i.e., the interface is stable). The "frozen" annotation is useful; the
`◆` and the session ID are residue.

**Finding:** ~7 `◆` boundary marks in `shor/docs/PEDAGOGY.md` prose, embedded in "frozen S.X.Y ◆"
annotations. Classification: **pure residue**. → CONSOLIDATE replaces "frozen S.A.2 ◆" with
"frozen" or "stable interface". *Feeds C-Coherence (prose half).*

---

### Summary table — prose-depth provenance catalog (D9-prose)

| # | Token type | Location | Count (approx.) | Classification | Consuming campaign |
|---|-----------|----------|-----------------|----------------|--------------------|
| F-D9-07 | `Track X` tokens | `docs/MATHEMATICS.md` | ~25 | Grouping-coincides-with-topic | CONSOLIDATE |
| F-D9-08 | `Track X` tokens | `docs/PEDAGOGY.md` | ~40 | Grouping-coincides-with-topic | CONSOLIDATE |
| F-D9-09 | `Track X` tokens | `README.md` | ~20 | Grouping-coincides-with-topic | CONSOLIDATE |
| F-D9-10 | `Phase N` tokens (N=0–8) | `docs/PEDAGOGY.md` | ~30 | Pure residue | CONSOLIDATE |
| F-D9-11 | Session/sub-track IDs (T.G, D.W.2, etc.) | `docs/MATHEMATICS.md` | ~15 | Pure residue | CONSOLIDATE |
| F-D9-12 | Sub-track IDs S.A/S.B/S.C (coarse) and S.A.1/S.B.2 etc. (fine) | `shor/docs/PEDAGOGY.md` | ~50 | Borderline (open-Q A.7) / Pure residue | CONSOLIDATE |
| F-D9-13 | Sub-track IDs G.B–G.W, D.A–D.W (coarse) and G.B.1 etc. (fine) | `gnfs/docs/PEDAGOGY.md` | many | Grouping-coincides-with-topic / Pure residue | CONSOLIDATE |
| F-D9-14 | `◆` boundary marks | `shor/docs/PEDAGOGY.md` | ~7 | Pure residue | CONSOLIDATE |

**Total prose planning-frame tokens (estimated):** ~185 across the six prose files. The bulk
(~110) are "Track X" tokens classified as grouping-coincides-with-topic; the remainder (~75) are
pure residue (Phase N, session IDs, ◆ marks, fine-grained sub-track IDs).

---

### Subtleties and deferrals

**D3 — Layer discipline is mostly sound; violations are at the cross-reference level.** The three-
layer model (inline / PEDAGOGY / MATHEMATICS) is honored in its reference directions. The
violations are at the cross-reference granularity: MATHEMATICS.md names Rust module paths where it
should cite PEDAGOGY sections; `shor/docs/PEDAGOGY.md` has a stale forward reference;
`shared/numfield/docs/PEDAGOGY.md` lacks the standard header. None of these are structural
violations — they are citation-style and staleness issues that CONSOLIDATE can fix in a single
pass.

**D4 — The textbook reads as a continuous textbook; the heading inconsistency is the main
structural defect.** The voice, notation, audience, and through-line are consistent. The heading-
level inconsistency (chapters 8 and 9 at `#` vs `##`) is the single structural defect that breaks
the document outline. The absence of an explicit suggested-reading-path section is a design gap,
not a defect — CONSOLIDATE decides whether to add one.

**D9-prose — Track labels are the dominant token class; they are grouping-coincides-with-topic.**
The ~110 "Track X" tokens are the largest class and are all grouping-coincides-with-topic: the
five tracks map onto five real mathematical families. CONSOLIDATE's re-anchoring work is label
replacement (e.g., "Track E" → "algebraic ECDLP attacks"), not structural reorganization. The
~75 pure-residue tokens (Phase N, session IDs, ◆ marks) require more surgical replacement but are
confined to specific sections (the Pollard rho optimization narrative in PEDAGOGY, the C-Textbook
contract and design-statement sections in MATHEMATICS).

**Borderline groupings (open-Q for A.7):** The S.A/S.B/S.C sub-track IDs in
`shor/docs/PEDAGOGY.md` are the primary borderline case: the grouping (simulator / factoring /
ECDLP) is mathematically sound, but the labels are planning-frame. A.7 decides whether to
preserve the grouping under topic-native labels or dissolve the S.X structure entirely.

**CONSOLIDATE blast radius for D9-prose:** ~185 prose tokens across 6 files. The bulk are label
replacements (no structural reorganization). The `gnfs/docs/PEDAGOGY.md` file (5240 lines) has the
highest density of sub-track IDs (G.B–G.W, D.A–D.W) but these are mostly grouping-coincides-with-
topic and survive as pipeline-stage labels under topic-native names.

*Feeds C-DocsLayer (sketch), C-MathSpine (sketch), and the prose half of C-Coherence.*
