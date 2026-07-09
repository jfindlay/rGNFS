# rDLP — Survey Findings (Arc 2: A. SURVEY)

Findings-only ledger. Each entry names the item, its flow direction, and the consuming campaign.
No config, code, or doc content is written here — ALIGN/ORACLE/REFACTOR/CONSOLIDATE/EXTEND do that.

Sessions: A.1 (done) · A.2–A.7 (pending).

---

## A.1 — Template-formalities gap, both flow directions (D1)

**Charge:** audit every formalities item present in `~/Source/rust-template` but absent at the
rDLP workspace root, assign a flow direction (template→rDLP backport or rDLP→template
forward-seed), and note scope (per-crate vs workspace-level). Also enumerate rDLP-originated
discipline worth seeding back. Feeds C-TemplateSeed; consumed by ALIGN.

**Observed state of `~/Source/rust-template` root:**
`.cargo/config.toml`, `AGENTS.md`, `Cargo.lock`, `Cargo.toml`, `deny.toml`, `docs/development.md`,
`docs/NOTES.md`, `LICENSE`, `README.md`, `rustfmt.toml`, `src/`, `target/`, `tests/`.
No `rust-toolchain.toml` at template root.

**Observed state of rDLP workspace root:**
`.gitignore`, `Cargo.lock`, `Cargo.toml`, `docs/`, `gnfs/`, `README.md`, `rho/`, `shared/`,
`shor/`, `target/`.
No `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, `LICENSE`, `development.md`, `.cargo/`.

---

### F-D1-01 · `deny.toml` absent at rDLP root

**Direction:** template → rDLP (backport).
**Scope:** workspace-level (one file at root; applies to all 9 crates).
**Template content:** advisories (`yanked = "deny"`), licenses allowlist
(`MIT`, `Apache-2.0`, `Unicode-3.0`, `GPL-3.0`, `BSD-3-Clause`), bans
(`multiple-versions = "warn"`), sources (`unknown-registry = "deny"`, `unknown-git = "deny"`).
**Finding:** `deny.toml` absent at rDLP root → ALIGN adds it at workspace root.

---

### F-D1-02 · `rustfmt.toml` absent at rDLP root

**Direction:** template → rDLP (backport).
**Scope:** workspace-level (one file at root; `cargo fmt --all` picks it up for all crates).
**Template content:** `max_width = 100` (mirrors the 100-char wrap convention).
**Finding:** `rustfmt.toml` absent at rDLP root → ALIGN adds it at workspace root.

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

### F-D1-04 · `LICENSE` absent at rDLP root

**Direction:** template → rDLP (backport).
**Scope:** workspace-level (one file at root; covers all 9 crates).
**Template content:** GNU General Public License v3.0 (GPL-3.0-or-later). The template's
`Cargo.toml` declares `license = "GPL-3.0-or-later"` and `deny.toml` allows `GPL-3.0` in the
license allowlist.
**Finding:** `LICENSE` absent at rDLP root → ALIGN adds GPL-3.0 `LICENSE` at workspace root.
Note: rDLP crate `Cargo.toml` files do not carry a `license` field; ALIGN should also add
`license = "GPL-3.0-or-later"` to each crate manifest (or to the workspace root manifest via
`[workspace.package]` inheritance if the resolver supports it).

---

### F-D1-05 · `[lints]` table present in 3/9 crates only; absent from workspace root

**Direction:** template → rDLP (backport), promoted to workspace-level.
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

### F-D1-06 · Coverage gate absent at rDLP

**Direction:** template → rDLP (backport), but gate value/config deferred to A.3.
**Scope:** workspace-level (one `cargo llvm-cov --workspace` invocation covers all 9 crates).
**Template content:** `cargo llvm-cov --all-targets --fail-under-lines 100` (100% line coverage
gate); `cargo-llvm-cov` listed as a required dev tool in `docs/development.md`; the `.cargo/
config.toml` alias `coverage = "llvm-cov --all-targets --fail-under-lines 100"` makes it
discoverable.
**Finding:** no coverage gate exists at rDLP → ALIGN adds one, honoring the doctrine A.3 will
set. **The gate value (100% line vs math-behavior KAT threshold vs other) is A.3's decision;
A.1 records only that the gap exists and that ALIGN is the implementing campaign.** The
`cargo-llvm-cov` tool and the `.cargo/config.toml` alias are the implementation vehicles.

---

### F-D1-07 · `development.md` absent at rDLP

**Direction:** template → rDLP (backport, adapted).
**Scope:** workspace-level (one file, e.g. `docs/development.md`).
**Template content:** toolchain table (cargo, rustfmt, clippy, cargo test, cargo-llvm-cov,
cargo-deny), setup instructions, formatting commands, full check-suite four-command sequence,
versioning convention, code conventions summary, testing conventions summary, project layout.
**Finding:** `docs/development.md` absent at rDLP → ALIGN adds it, adapted for the workspace
(9 crates, no binary entry point, workspace-level `cargo` invocations, the coverage doctrine
A.3 sets, and the CADO-NFS sidecar ORACLE builds).

---

### F-D1-08 · `.cargo/config.toml` (cargo aliases) absent at rDLP

**Direction:** template → rDLP (backport, adapted).
**Scope:** workspace-level (`.cargo/config.toml` at workspace root applies to all `cargo`
invocations in the workspace).
**Template content:** aliases `lint`, `fmt-check`, `format`, `test-all`, `coverage`, `audit` —
the "tox environments as pure-cargo aliases" pattern.
**Finding:** `.cargo/config.toml` absent at rDLP root → ALIGN adds it, adapted for the workspace
(workspace-scoped `--workspace` flags, coverage alias honoring A.3's doctrine, `audit` alias for
`cargo deny check`).

---

### F-D1-09 · `AGENTS.md` absent at rDLP root

**Direction:** template → rDLP (backport, adapted).
**Scope:** workspace-level (one file at root).
**Template content:** agent guide with commands (`cargo build`, `cargo test --all-targets`,
`cargo clippy`, `cargo fmt`, `cargo llvm-cov`, `cargo deny check`) and code conventions summary.
**Finding:** `AGENTS.md` absent at rDLP root → ALIGN adds it, adapted for the workspace
(workspace-level commands, the 9-crate structure, the CADO oracle policy, and the docs-layer
discipline CONSOLIDATE will freeze).

---

### F-D1-10 · rDLP-originated discipline: multisession docs layering

**Direction:** rDLP → template (forward-seed).
**Scope:** project-level docs convention (not a single file — a three-tier discipline).
**rDLP content:** `docs/PLAN.md` (current sub-track, actionable session list + contracts +
ledger + digest) + `docs/ROADMAP.md` (project-lifetime view, updated only at sub-track
boundaries) + `docs/NOTES.md` (rolling-context durable framings, distinct from plan and roadmap).
The three-tier separation — rolling-context / sub-track / project-lifetime — prevents the
"one giant notes file" anti-pattern and makes the action-frame / static-frame distinction
concrete.
**Finding:** template carries only `docs/NOTES.md` (design notes and rationale); it lacks the
PLAN/ROADMAP tier separation → ALIGN seeds the three-tier discipline into the template, adapted
for single-crate projects (PLAN.md and ROADMAP.md at appropriate grain for a template project).

---

### F-D1-11 · rDLP-originated discipline: dev-oracle policy

**Direction:** rDLP → template (forward-seed).
**Scope:** testing convention (prose policy + `#[ignore]` gate pattern).
**rDLP content:** all external-tool oracle KATs (PARI, msolve, CADO-NFS) are gated behind
`#[ignore = "PARI not installed; run manually when available"]` — the oracle is an opt-in
validation sidecar, never on the green test path, never a production dependency. The policy is
stated explicitly in `docs/PEDAGOGY.md` (Principle 3 verification) and is the arc-2 ORACLE
campaign's design anchor.
**Finding:** template has no dev-oracle policy (its testing conventions cover unit + integration
only) → ALIGN seeds the `#[ignore]`-gate pattern and the opt-in-oracle principle into the
template's `docs/development.md` testing conventions section.

---

### F-D1-12 · rDLP-originated discipline: docs-register contracts (three-layer reference discipline)

**Direction:** rDLP → template (forward-seed).
**Scope:** docs architecture convention (prose policy).
**rDLP content:** a three-layer reference-direction discipline — (1) inline doc-comments
(`///`/`//!`) for API-adjacent exposition; (2) per-crate `PEDAGOGY.md` human-code-reference
tours (code-adjacent but human-facing, may reference code identifiers); (3) `docs/MATHEMATICS.md`
textbook layer (mathematical exposition, references code only where useful). Each layer has
prescribed reference directions and allowances. The discipline is implicit in the arc-1 corpus
and is the subject of A.4's audit; CONSOLIDATE will freeze it as C-DocsLayer.
**Finding:** template carries only `docs/NOTES.md` and `docs/development.md` — no layered
reference-direction discipline → ALIGN seeds the three-layer concept into the template at
appropriate grain for a single-crate project (inline / human-code-ref / agent-docs), adapted
from the rDLP three-layer model.

---

### Summary table

| # | Item | Direction | Scope | Consuming campaign |
|---|---|---|---|---|
| F-D1-01 | `deny.toml` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-02 | `rustfmt.toml` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-03 | `rho/rust-toolchain.toml` stale per-crate artifact | mis-scoped (promote or remove) | workspace hygiene | ALIGN |
| F-D1-04 | `LICENSE` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-05 | `[lints]` in 3/9 crates only | template → rDLP | workspace-level `[workspace.lints]` | ALIGN |
| F-D1-06 | coverage gate absent | template → rDLP (gate value deferred to A.3) | workspace-level | ALIGN |
| F-D1-07 | `development.md` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-08 | `.cargo/config.toml` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-09 | `AGENTS.md` absent | template → rDLP | workspace-level | ALIGN |
| F-D1-10 | multisession docs layering | rDLP → template | project-level docs convention | ALIGN |
| F-D1-11 | dev-oracle policy | rDLP → template | testing convention | ALIGN |
| F-D1-12 | docs-register contracts (three-layer) | rDLP → template | docs architecture convention | ALIGN |

---

### Subtleties and deferrals

**`rho/rust-toolchain.toml` (F-D1-03).** The template carries no `rust-toolchain.toml` at its
root, so this is not a clean template→rDLP backport. The `rho/` file (`channel = "stable"`,
no version pin) is a stale per-crate artifact — it has no effective scope for workspace-level
`cargo` invocations. ALIGN must decide: promote to workspace root with a concrete pin, or remove.
A.1 records the mis-scoping; ALIGN owns the decision.

**Coverage gate (F-D1-06).** The gap is recorded here; the gate value and doctrine are A.3's
charge. A.1 does not decide whether 100% line coverage (template default) or a math-behavior
KAT threshold is correct for rDLP. ALIGN implements whatever doctrine A.3 sets.

**`[lints]` promotion (F-D1-05).** The three crates that already carry `[lints]` (`shor`,
`shared/padic`, `shared/gf2m`) use a slightly different form than the template (`all = "deny"`
vs `all = { level = "deny", priority = -1 }`). ALIGN should normalize to the template form
(with explicit `priority` fields) when promoting to `[workspace.lints]`.

**`license` field in crate manifests.** None of the 9 crate `Cargo.toml` files carry a `license`
field. Adding `LICENSE` at the workspace root (F-D1-04) is necessary but not sufficient for
`cargo deny` license checks — ALIGN should also add `license = "GPL-3.0-or-later"` to each
crate manifest or use `[workspace.package]` inheritance.

**rDLP → template seeds (F-D1-10, F-D1-11, F-D1-12).** These are forward-seeds of discipline
the template currently lacks. They require adaptation (the template is a single-crate project;
rDLP is a 9-crate workspace). ALIGN owns the adaptation; A.1 records the direction and the
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

---

## A.5 — CADO-NFS sidecar build/trigger design needs (D6)

**Charge:** audit the design space for the dynamic CADO-NFS sidecar — build-trigger model,
version pinning strategy, what regression/comparison tests it would gate, and how it honors the
arc-1 dev-oracle policy (CADO as opt-in validation sidecar, never part of how rDLP computes).
Feeds C-Oracle (sketch); consumed by ORACLE.

**Observed state of CADO-NFS oracle tests in the codebase:**

Four `#[ignore]`-gated CADO-NFS oracle tests exist across the `gnfs` crate:

| File | Test name | Oracle role |
|---|---|---|
| `gnfs/tests/line_sieve_kat.rs` | `kat_c_cado_nfs_oracle` | Compare relation count from rDLP line sieve against CADO-NFS at matched parameters; tolerance ±3× (CADO uses large-prime relations) |
| `gnfs/tests/merge_kat.rs` | `kat_c_cado_nfs_oracle` | Compare filtered matrix dimensions (row count, column count, Hamming weight) against CADO output at matched parameters; tolerance ±10% on row count |
| `gnfs/tests/lanczos_kat.rs` | `kat_c_cado_oracle_n35` | Expand Lanczos kernel vector through provenance to a congruence of squares; verify the factor matches what CADO-NFS finds for N=35 |
| `gnfs/tests/factor_end_to_end_kat.rs` | `kat_c_oracle_80_100_bit_challenge` | Factor an 80–100-bit semiprime end-to-end; verify against CADO-NFS / msieve |

All four tests are stubs: the CADO invocation logic is documented in comments but not implemented
(`unimplemented!` or `todo!` macros). The tests are gated with
`#[ignore = "CADO-NFS not installed; run manually when available"]`. The existing test code
references `cado-nfs.py` as the invocation entry point (the Python orchestration script).

**Observed CADO-NFS project structure (from GitHub mirror `cado-nfs/cado-nfs`):**

- **Authoritative source:** `https://gitlab.inria.fr/cado-nfs/cado-nfs` (mirrored to GitHub at
  `https://github.com/cado-nfs/cado-nfs`; the README states the GitHub mirror is kept up-to-date
  with the master branch).
- **Build system:** CMake + GNU make (C/C++ project). Build produces binaries in
  `build/$(hostname)/`; the Python orchestration script `cado-nfs.py` calls these binaries.
- **Build requirements:** GCC ≥ 10 (or Clang ≥ 12, Apple Clang ≥ 16, Intel ICX ≥ 2023),
  CMake ≥ 3.18, Python ≥ 3.8, GMP ≥ 5 (with `--enable-shared`). As of `cado-nfs-3.0.0`,
  C++20 is required.
- **Invocation entry point:** `./cado-nfs.py <N>` for factoring; `./cado-nfs.py <param_file>`
  for a pre-configured run. The existing test stubs reference `cado-nfs.py` as the entry point.
- **Version tags (GitHub mirror):** `git-2.0.1` (Oct 2020, commit `88c4751`) is the most recent
  tagged version on the GitHub mirror. The README references `cado-nfs-3.0.0` as requiring C++20,
  indicating a 3.0.0 release exists on the Inria GitLab (the authoritative source). The Inria
  GitLab is bot-protected and could not be directly browsed; the GitHub mirror is the accessible
  pinning target.
- **Docker containers:** pre-compiled containers are available at
  `registry.gitlab.inria.fr/cado-nfs/cado-nfs/factoring-full` (x86_64, Haswell or later).

---

### Part (a) — Build-trigger model

#### Design space

Two trigger models are possible:

1. **CI-eager:** CADO-NFS is always built in CI (every push triggers a CADO build). The sidecar
   is always present; the `#[ignore]`-gated oracle tests can be promoted to the standard CI run.
   The oracle is no longer opt-in — it becomes part of the green path.

2. **Lazy/on-demand + opt-in CI flag:** CADO-NFS is built only when explicitly requested — either
   by the developer running `cargo test -- --ignored` locally, or by a CI job triggered with an
   explicit flag (e.g., an environment variable `CADO_ORACLE=1` or a CI job variant). The
   `#[ignore]`-gated tests remain gated; the sidecar is available but not mandatory.

#### Recommendation

**Lazy/on-demand + opt-in CI flag.** Rationale:

- **Arc-1 dev-oracle policy fidelity.** The policy is explicit: CADO-NFS is an opt-in validation
  sidecar, never part of how rDLP computes. CI-eager violates this policy by making the oracle
  mandatory — a CI failure due to CADO unavailability or a CADO version change would block the
  green path. The `#[ignore]` gate is the policy's mechanical expression; the trigger model must
  honor it.
- **Build cost.** CADO-NFS is a large C/C++ project (CMake + make, GCC ≥ 10, GMP). Building it
  in every CI run adds significant wall-clock time and dependency surface to a project whose CI
  currently runs `cargo test --workspace` only. The pedagogical library's CI should remain fast
  and dependency-light.
- **Dependency surface.** CI-eager requires GCC/Clang, CMake, GMP, and Python in the CI
  environment. The current rDLP CI surface is pure Rust (`cargo`). Adding a C/C++ build
  dependency to the mandatory CI path is a scope expansion that ORACLE must justify — and the
  arc-1 policy already answers: it is not justified for the mandatory path.
- **Opt-in CI flag is sufficient.** A CI job variant (e.g., a separate GitHub Actions job or a
  Makefile target `make oracle-check`) that builds CADO and runs the `--ignored` tests provides
  the regression/comparison signal without polluting the mandatory green path. This is the
  standard pattern for optional integration tests in Rust projects.

---

### F-D6-01 · Sidecar trigger = lazy on-demand + opt-in CI flag

**Falsifiable finding:**

> **The CADO-NFS sidecar trigger model is lazy/on-demand + opt-in CI flag.** The `#[ignore]`-gated
> oracle tests (`kat_c_cado_nfs_oracle` in `line_sieve_kat.rs` and `merge_kat.rs`,
> `kat_c_cado_oracle_n35` in `lanczos_kat.rs`, `kat_c_oracle_80_100_bit_challenge` in
> `factor_end_to_end_kat.rs`) remain gated behind `#[ignore]` in the standard test run. They are
> exercised only when the developer explicitly passes `-- --ignored` or when a CI job variant
> (opt-in flag) is triggered. The sidecar is never on the mandatory green path.
>
> **Downstream checks:**
> - ORACLE must NOT promote the `#[ignore]`-gated tests to the standard `cargo test --workspace`
>   run. Doing so is a policy violation.
> - ORACLE's CI integration (if any) must be a separate, opt-in job — not a required check on
>   every push.
> - The `docs/development.md` that ALIGN adds (F-D1-07) must document the opt-in invocation
>   (`cargo test -- --ignored` or the CI flag) as the way to run oracle tests.

*Feeds ORACLE; seeds C-Oracle (sketch).*

---

### Part (b) — Version pinning strategy

#### Design space

Three pinning strategies are possible:

1. **Git tag pin:** pin to a specific git tag (e.g., `git-2.0.1` or a future `3.0.0` tag). Tags
   are human-readable and stable; the GitHub mirror carries them. Reproducible via
   `git clone --branch git-2.0.1 --depth 1`.

2. **Commit hash pin:** pin to a specific commit hash (e.g., `88c4751` for `git-2.0.1`). More
   precise than a tag (tags can be moved, though rarely); requires knowing the hash in advance.
   Reproducible via `git checkout 88c4751`.

3. **Release tarball:** download a versioned tarball from the Inria GitLab or GitHub mirror
   (e.g., `git-2.0.1.tar.gz`). No git dependency at build time; reproducible via checksum.
   Requires storing the tarball URL and a SHA-256 checksum.

#### Recommendation

**Git tag pin, with the GitHub mirror as the source.** Rationale:

- **Human-readable and auditable.** A tag like `git-2.0.1` is self-documenting; a raw commit
  hash is opaque. The tag is the right unit of "this is the version we validated against."
- **GitHub mirror is accessible.** The authoritative Inria GitLab is bot-protected and may be
  inaccessible in automated environments. The GitHub mirror (`github.com/cado-nfs/cado-nfs`) is
  publicly accessible and kept in sync with the master branch.
- **Tag + commit hash together.** The pin should record both the tag name and its commit hash
  (e.g., `git-2.0.1` → `88c4751ca1fe4677d6b83efa348d4a7b4d15d1fa`) so that ORACLE can verify
  the checkout is correct even if the tag were to be moved.
- **Tarball is an alternative if git is unavailable.** The GitHub mirror provides `.tar.gz`
  downloads for each tag. If the CI environment lacks git, ORACLE may use the tarball + SHA-256
  checksum as a fallback. This is a ORACLE implementation decision; SURVEY records the option.

**Version to pin:** The most recent tagged version on the GitHub mirror is `git-2.0.1` (Oct 2020,
commit `88c4751`). The README references `cado-nfs-3.0.0` as requiring C++20 — if a 3.0.0 tag
exists on the Inria GitLab and is mirrored to GitHub, ORACLE should prefer it (C++20 is the
current standard; GCC ≥ 10 is widely available). ORACLE must verify the tag resolves before
committing to a pin. If only `git-2.0.1` is accessible, pin to that.

---

### F-D6-02 · Version pin = git tag on GitHub mirror, tag + commit hash recorded

**Falsifiable finding:**

> **The CADO-NFS sidecar pins to a specific git tag on the GitHub mirror
> (`github.com/cado-nfs/cado-nfs`), with the commit hash recorded alongside the tag name.**
> The pin is recorded in a prose file (e.g., `docs/development.md` or a dedicated
> `docs/oracle.md`) — not in a `Cargo.toml` or build script (those are ORACLE scope). The
> minimum pin is `git-2.0.1` (commit `88c4751`); ORACLE should prefer the most recent tagged
> version that resolves on the GitHub mirror at ORACLE build time.
>
> **Downstream checks:**
> - ORACLE's build script must clone or download the pinned tag, not `master` or `HEAD`.
> - The pin must be recorded as both a tag name and a full commit hash (40 hex characters).
> - If ORACLE upgrades the pin, the commit hash must be updated in the same change.

*Feeds ORACLE; seeds C-Oracle (sketch).*

---

### Part (c) — Regression/comparison tests gated by the sidecar

#### What the sidecar gates

The four existing `#[ignore]`-gated stubs define the comparison surface ORACLE must implement:

| Test | rDLP output | CADO-NFS output | Comparison |
|---|---|---|---|
| `line_sieve_kat.rs` `kat_c_cado_nfs_oracle` | Relation count from `line_sieve` at matched polynomial + factor-base bounds | Relation count from `cado-nfs.py` at same parameters | rDLP count ≥ CADO count / 3.0 (CADO finds more via large-prime relations) |
| `merge_kat.rs` `kat_c_cado_nfs_oracle` | Filtered matrix dimensions (rows, cols, Hamming weight) from rDLP filtering pipeline | Filtered matrix dimensions from CADO's `.mat` output at matched parameters | Dimensions within ±10% of CADO's output |
| `lanczos_kat.rs` `kat_c_cado_oracle_n35` | Lanczos kernel vector expanded through provenance to a factor of N=35 | CADO-NFS factor of N=35 | Both recover the same nontrivial factor (5 or 7) |
| `factor_end_to_end_kat.rs` `kat_c_oracle_80_100_bit_challenge` | Full GNFS pipeline factor of an 80–100-bit semiprime | CADO-NFS factor of the same semiprime | Both recover the same nontrivial factor |

**What the sidecar does NOT gate:** rDLP's correctness on the green path. The green-path KATs
(the 53 external `*_kat.rs` files and the inline `#[test]` blocks) are self-contained — they do
not require CADO. The sidecar gates only the cross-validation comparisons, which are opt-in.

**What the sidecar enables:** the comparisons above validate that rDLP's GNFS pipeline stages
(sieving, filtering, linear algebra, end-to-end) produce outputs that are quantitatively
consistent with CADO-NFS at matched parameters. This is the "live correctness oracle" the ROADMAP
charges ORACLE to build. The comparisons are not equality checks — they are tolerance-bounded
consistency checks, because rDLP is a demonstration-fidelity implementation (no large-prime
relations, no FFT sieve) while CADO is a production implementation.

---

### F-D6-03 · Sidecar gates four tolerance-bounded consistency comparisons, not equality checks

**Falsifiable finding:**

> **The CADO-NFS sidecar gates exactly four comparison tests, each with a documented tolerance:**
> (1) relation count from line sieve: rDLP ≥ CADO / 3.0; (2) filtered matrix dimensions: within
> ±10% of CADO; (3) Lanczos factor of N=35: same nontrivial factor as CADO; (4) end-to-end factor
> of an 80–100-bit semiprime: same nontrivial factor as CADO. The comparisons are
> tolerance-bounded, not equality checks, because rDLP is a demonstration-fidelity
> implementation. ORACLE implements these four tests by filling in the `unimplemented!` / `todo!`
> stubs in the existing test files.
>
> **Downstream checks:**
> - ORACLE must not change the tolerance values without a documented rationale.
> - ORACLE must not add new oracle tests to the mandatory green path.
> - The four stub tests are the complete CADO oracle surface; ORACLE does not need to add new
>   `#[ignore]`-gated tests (it implements the existing stubs).

*Feeds ORACLE; seeds C-Oracle (sketch).*

---

### Part (d) — Dev-oracle policy fidelity

#### The arc-1 dev-oracle policy (observed)

The policy is stated in `docs/PEDAGOGY.md` (Principle 3 verification, line 1242–1266) and
`gnfs/docs/PEDAGOGY.md` (Principle 3, lines 4100–4123):

> CADO-NFS / msieve remain dev-only oracles, gated behind `#[ignore]` KATs. No production
> dependency was added. The oracle KATs are ignored in the standard test run.

The policy has three components:
1. **Never on the green path.** Oracle tests are `#[ignore]`-gated; `cargo test --workspace`
   never runs them.
2. **Never a production dependency.** CADO-NFS is not a `[dev-dependencies]` entry in any
   `Cargo.toml`; it is an external tool invoked by a test stub.
3. **Opt-in validation only.** The oracle validates rDLP outputs; it does not compute them.
   rDLP's GNFS pipeline is self-contained; CADO is a cross-check, not a component.

#### How the sidecar design honors the policy

The lazy/on-demand + opt-in CI flag trigger model (F-D6-01) directly honors all three components:

- **Never on the green path:** the `#[ignore]` gate is preserved; the sidecar is not built in
  the standard CI run.
- **Never a production dependency:** CADO-NFS is built as an external binary sidecar, not linked
  into the rDLP workspace. No `Cargo.toml` entry is added.
- **Opt-in validation only:** the sidecar's role is comparison (rDLP output vs CADO output at
  matched parameters). The comparison is always rDLP-first: rDLP computes the output; CADO
  validates it. The direction is never reversed.

The tolerance-bounded comparison design (F-D6-03) also honors the policy: the tolerances
acknowledge that rDLP is a demonstration-fidelity implementation, not a CADO clone. A strict
equality check would conflate "rDLP is correct" with "rDLP matches CADO exactly" — the latter
is false by design (rDLP has no large-prime relations, no FFT sieve). The tolerance-bounded
check is the honest oracle: it validates that rDLP is in the right ballpark, not that it
replicates CADO's engineering optimizations.

---

### F-D6-04 · Dev-oracle policy fidelity: sidecar is validation-only, never on the green path

**Falsifiable finding:**

> **The CADO-NFS sidecar honors the arc-1 dev-oracle policy on all three axes:**
> (1) never on the green path — `#[ignore]` gate preserved, `cargo test --workspace` never runs
> oracle tests; (2) never a production dependency — CADO-NFS is an external binary, no
> `Cargo.toml` entry; (3) opt-in validation only — rDLP computes, CADO validates, never the
> reverse. The tolerance-bounded comparison design (F-D6-03) is the honest oracle: it validates
> correctness-in-ballpark, not engineering-optimization parity.
>
> **Downstream checks:**
> - ORACLE must not add CADO-NFS to any `[dev-dependencies]` or `[build-dependencies]` in any
>   `Cargo.toml`. A `Cargo.toml` delta touching CADO is a policy violation.
> - ORACLE must not change the `#[ignore]` attribute on any of the four oracle tests. Removing
>   `#[ignore]` is a policy violation.
> - The comparison direction is always rDLP-first: rDLP computes the output, CADO validates.
>   A test that calls CADO to compute a result and then checks rDLP against it is a policy
>   violation.

*Feeds ORACLE; seeds C-Oracle (sketch).*

---

### Summary table — CADO-NFS sidecar design findings

| # | Finding | Recommendation | Consuming campaign |
|---|---|---|---|
| F-D6-01 | Sidecar trigger model | Lazy/on-demand + opt-in CI flag; `#[ignore]` gate preserved | ORACLE |
| F-D6-02 | Version pinning strategy | Git tag on GitHub mirror, tag + commit hash recorded; minimum `git-2.0.1` (`88c4751`) | ORACLE |
| F-D6-03 | Gated comparisons | Four tolerance-bounded consistency checks (relation count, matrix dims, N=35 factor, 80–100-bit factor); ORACLE fills existing stubs | ORACLE |
| F-D6-04 | Dev-oracle policy fidelity | Sidecar is validation-only, never on green path, never a `Cargo.toml` dependency | ORACLE |

---

### Subtleties and deferrals

**Build automation intricacy (ROADMAP flag).** The ROADMAP notes "opus only if build automation
proves intricate" for ORACLE. The CADO-NFS build is a CMake + make C/C++ project with non-trivial
dependencies (GMP, Python, GCC ≥ 10). The build automation is not trivial, but it is also not
unprecedented — the CADO README documents a two-step build (`make` then `./cado-nfs.py`), and
Docker containers are available for x86_64. A.5 records the build requirements and the invocation
entry point; ORACLE decides whether the automation warrants an opus juncture. **This is not an
A.7 open-Q** — the ROADMAP already flags it as an ORACLE-internal decision. A.5's finding is that
the design (trigger model, pinning, gated comparisons) can be settled as findings; the build
automation complexity is ORACLE's to assess.

**`git-2.0.1` vs `3.0.0` pin.** The GitHub mirror's most recent tag is `git-2.0.1` (Oct 2020).
The README references `cado-nfs-3.0.0` as requiring C++20, suggesting a newer release exists on
the Inria GitLab. ORACLE must verify which tags are accessible on the GitHub mirror at ORACLE
build time and pin to the most recent accessible tag. If the Inria GitLab becomes accessible,
ORACLE may prefer it as the authoritative source. A.5 records the uncertainty; ORACLE resolves it.

**Tolerance values.** The tolerances in F-D6-03 (3× for relation count, ±10% for matrix dims)
are taken from the existing test stub comments. They are design intent, not measured values.
ORACLE must validate these tolerances against actual CADO runs at the pinned version and adjust
if the stubs' intent does not match observed behavior. A.5 records the intent; ORACLE calibrates.

**`factor_end_to_end_kat.rs` stub is a `todo!`.** Unlike the other three stubs (which have
partial implementation logic), `kat_c_oracle_80_100_bit_challenge` is a bare `todo!` — the full
GNFS pipeline (polyselect → sieve → filter → linalg → sqrt) is not yet wired end-to-end in the
test. ORACLE must wire the pipeline before the oracle comparison is meaningful. This is the
highest-complexity stub; ORACLE may defer it to a later ORACLE session if the pipeline wiring
proves intricate.

**msieve as an alternative oracle.** `factor_end_to_end_kat.rs` references "CADO-NFS or msieve"
as the oracle. msieve is a simpler build (single C binary, no Python, no CMake). ORACLE may use
msieve as a fallback oracle for the end-to-end test if CADO proves difficult to build. A.5 notes
the option; ORACLE decides.

*Feeds ORACLE; seeds C-Oracle (sketch).*

---

## A.6 — Spectrum completeness + 4-reference distillation (D7 + D8) — **Opus**

**Charge:** (a) **Spectrum-completeness audit (D7)** — map the current matrix
(`MATHEMATICS.md`'s 12 chapters + the 5 tracks) against the intent's discriminant (full spectrum
of mathematically-necessary algorithms for integer factorization and DLP, classical and quantum,
excluding hardware/distributed specifics); identify omitted subject areas and classify each as a
genuine gap or a deliberate non-extension. (b) **4-reference distillation (D8)** — verify that
the four external quantum-DLP references resolve, then distil what each adds to the spectrum.
Feeds F-EXTEND; consumed by F.

**Discriminant (from ROADMAP D7):** full spectrum from integer DLP → finite-field DLP →
EC-over-finite-field DLP, classical and quantum, constrained to mathematically-necessary
algorithms (excluding hardware/distributed specifics). The discriminant is about the
*mathematical* algorithm families, not about engineering optimizations (hardware-specific gate
sets, distributed sieving, etc.).

---

### Part (a) — Spectrum-completeness audit (D7)

#### The algorithm matrix

The matrix axes are:

- **Problem type:** integer factorization, finite-field DLP (in $\mathbb{F}_p^*$ or
  $\mathbb{F}_{p^n}^*$), elliptic-curve DLP (generic and structured), quantum DLP.
- **Algorithm family:** the mathematically-necessary algorithms at each cell.

The current library covers 5 tracks across 12 chapters of `MATHEMATICS.md` and 9 workspace
crates. The chapter inventory (from A.4):

| Ch. | Title | Problem type covered |
|-----|-------|---------------------|
| 6 | Pollard Rho for ECDLP | EC-DLP (generic), integer factorization (Pollard rho factor) |
| 7 | The α-Substrate: Primality, Smoothness, ECM | Integer factorization (ECM, Miller–Rabin) |
| 8 | GNFS | Integer factorization (NFS) |
| 9 | NFS-DL | Finite-field DLP (NFS-DL) |
| 10 | Algebraic ECDLP Attacks | EC-DLP (structured: Pohlig–Hellman, SSA, GHS, index calculus, MOV) |
| 11 | Shor's Algorithm | Integer factorization (quantum), EC-DLP (quantum), finite-field DLP (quantum) |
| 12 | Modularity (speculation) | No algorithm — mathematical survey only |

#### Cell-by-cell matrix

**Integer factorization:**

| Algorithm | Present? | Classification |
|-----------|----------|----------------|
| Trial division | No | **Deliberate non-extension.** Trial division is a preprocessing step subsumed by ECM and Pollard rho at any scale the library targets. It is not a "mathematically necessary" algorithm at the level of the discriminant — it is a trivial special case of smooth-number detection. |
| Pollard rho (factoring) | **Yes** | Ch. 6 (theory), `rho/src/factor/` (code). Floyd's cycle detection, Brent's improvement, batched-GCD variant. |
| ECM (Lenstra) | **Yes** | Ch. 7 (theory), `shared/numth/src/ecm.rs` (code). Stage-1 and stage-2 ECM. |
| Quadratic sieve (QS) | No | **Deliberate non-extension.** QS is the historically important predecessor to GNFS, but GNFS strictly dominates QS for $N > 10^{100}$ (the crossover point). The library's stated scope is the best-known classical algorithms; QS adds historical context but not a new mathematical family. The discriminant ("mathematically-necessary algorithms") does not require QS if GNFS is present. |
| GNFS | **Yes** | Ch. 8 (theory, full payoff proof), `gnfs/` (code, complete pipeline). |
| Shor's algorithm (quantum) | **Yes** | Ch. 11.3 (full proof), `shor/src/shor.rs` (code). |

**Finite-field DLP (in $\mathbb{F}_p^*$):**

| Algorithm | Present? | Classification |
|-----------|----------|----------------|
| Baby-step giant-step (BSGS) | No | **Deliberate non-extension.** BSGS is the deterministic $O(\sqrt{n})$ generic DLP algorithm. The library uses Pollard rho as the generic baseline throughout (Ch. 6 establishes the $\sqrt{n}$ bound via Pollard rho; the same bound applies to BSGS). BSGS is mentioned in the prerequisites but not separately developed. Since Pollard rho and BSGS are in the same complexity class ($O(\sqrt{n})$, same L-notation cell), BSGS does not add a new mathematical family — it is the deterministic variant of the same generic birthday-paradox algorithm. Not a gap under the discriminant. |
| Pohlig–Hellman | **Yes** | Ch. 10.1 (theory), `rho/src/ecdlp/pohlig.rs` (code). Applies to $\mathbb{F}_p^*$ and EC groups. |
| Index calculus (classical, $\mathbb{F}_p^*$) | Partial | The classical index calculus for $\mathbb{F}_p^*$ (smooth-number sieving, linear algebra) is the mathematical predecessor to NFS-DL. The library covers NFS-DL (Ch. 9), which is the best-known algorithm and subsumes classical index calculus. The classical index calculus is not separately developed as a chapter or crate. **Classification: deliberate non-extension** — NFS-DL subsumes it; a separate classical-index-calculus chapter would be redundant. |
| NFS-DL | **Yes** | Ch. 9 (theory), `gnfs/src/dl/` (code, complete pipeline including Schirokauer maps and individual-log descent). |
| Shor's algorithm (quantum, DLP) | **Yes** | Ch. 11 (theory — the factoring reduction implies DLP via the order-finding reduction; the ECDLP variant in §11.4 applies to any group). |

**Elliptic-curve DLP (generic):**

| Algorithm | Present? | Classification |
|-----------|----------|----------------|
| Pollard rho (ECDLP) | **Yes** | Ch. 6 (theory, full treatment), `rho/src/ecdlp/` (code, all optimizations: r-adding walk, DPs, negmap, batched inversion, GLV). |
| Baby-step giant-step (BSGS, ECDLP) | No | **Deliberate non-extension.** Same reasoning as for finite-field BSGS: same $O(\sqrt{n})$ complexity class as Pollard rho; the library uses Pollard rho as the generic baseline. Not a gap under the discriminant. |
| Pohlig–Hellman (ECDLP) | **Yes** | Ch. 10.1 (theory), `rho/src/ecdlp/pohlig.rs` (code). |

**Elliptic-curve DLP (structured attacks):**

| Algorithm | Present? | Classification |
|-----------|----------|----------------|
| MOV/Frey–Rück (small embedding degree) | **Yes** | Ch. 10.5 (theory, full payoff proof), `rho/src/pairing/mov.rs` (code). |
| Smart–Satoh–Araki (anomalous curves) | **Yes** | Ch. 10.2 (theory), `rho/src/ssa/` (code). |
| GHS/Weil descent (binary field tower) | **Yes** (transfer only) | Ch. 10.3 (theory, descent reduction), `rho/src/ghs/` (code). The transfer from ECDLP to hyperelliptic Jacobian DLP is present; the downstream solve is explicitly deferred (see below). |
| Index calculus over $E(\mathbb{F}_{p^n})$ (Gaudry–Diem–Joux–Vitse) | **Yes** | Ch. 10.4 (theory), `rho/src/index_calculus/` + `rho/src/semaev/` (code). |
| Hyperelliptic Jacobian DLP (downstream of GHS) | **No** | **Genuine gap — see F-D7-01 below.** |
| Shor's algorithm (quantum, ECDLP) | **Yes** | Ch. 11.4 (full proof, two-register hidden-subgroup), `shor/src/ecdlp.rs` (code). |

---

### F-D7-01 · Hyperelliptic Jacobian DLP: genuine gap within the discriminant

**Observed state:** Chapter 10.3 (GHS/Weil descent) explicitly states:

> "**This section represents GHS honestly as a transfer.** The GHS construction reduces the ECDLP
> on $E/\mathbb{F}_{2^m}$ to a DLP on the Jacobian $\mathrm{Jac}(C)/\mathbb{F}_{2^l}$. The
> downstream solve — index calculus on the hyperelliptic Jacobian — is a separate step (a deferred
> re-shard in this project). The chapter covers the descent reduction and log-preservation
> verification; the downstream solve is not developed here."

The downstream solve — index calculus on the Jacobian of a hyperelliptic curve — is the
algorithm that actually extracts the discrete logarithm after the GHS transfer. Without it, the
GHS chapter demonstrates the transfer mechanism but leaves the DLP unsolved. The downstream
algorithm is Gaudry's index calculus for hyperelliptic Jacobians (2000), which achieves
subexponential complexity for genus $g \geq 3$ curves.

**Why this is within the discriminant:** The discriminant requires the "full spectrum" of
mathematically-necessary algorithms for DLP. The GHS descent is only half of the attack: the
transfer reduces ECDLP to hyperelliptic Jacobian DLP, but the hyperelliptic Jacobian DLP is
itself a distinct problem requiring its own algorithm. The library currently has a "dangling
transfer" — the GHS chapter reduces to a problem it does not solve. A complete treatment of the
GHS attack requires the downstream solve.

**The algorithm:** Gaudry's index calculus for hyperelliptic Jacobians (Gaudry 2000, "An
algorithm for solving the discrete log problem on hyperelliptic curves"). For a genus-$g$
hyperelliptic curve $C/\mathbb{F}_q$, the algorithm achieves complexity
$O(q^{2-2/g})$ — subexponential for $g \geq 3$. The factor base consists of degree-1 divisors
(points on $C$); the relation collection uses the Mumford representation of divisors. The linear
algebra step is the same block-Lanczos/Wiedemann infrastructure already present in `gnfs/`.

**Matrix cell:** EC-DLP (via GHS transfer) → hyperelliptic Jacobian DLP → Gaudry index calculus.
The cell is: *hyperelliptic Jacobian DLP, index calculus, classical*.

**Finding:**

> **F-D7-01 (genuine gap): Hyperelliptic Jacobian DLP (Gaudry index calculus) is absent from the
> spectrum.** The GHS chapter (Ch. 10.3) reduces ECDLP to hyperelliptic Jacobian DLP but does not
> develop the downstream solve. The downstream algorithm (Gaudry 2000) is within the discriminant:
> it is a mathematically-necessary algorithm for completing the GHS attack, and it is a distinct
> mathematical family (index calculus on a Jacobian variety, not on $\mathbb{F}_p^*$ or an
> elliptic curve). → **F-EXTEND candidate.** EXTEND adds a treatment of Gaudry's index calculus
> for hyperelliptic Jacobians, completing the GHS attack chain. The `rho/src/hyperelliptic/`
> module (which implements the Mumford divisor group law and Cantor's algorithm) is the substrate
> for this extension — the group law is already present; the index calculus layer is what is
> missing.

*Feeds F-EXTEND. The EXTEND scope-ceiling question (arc-2 vs arc-3) is a human open-Q at A.7.*

---

### F-D7-02 · Spectrum otherwise complete for the stated discriminant

**Observed state:** Every other cell in the algorithm matrix is either present or a deliberate
non-extension justified by the discriminant:

- **Trial division:** subsumed by ECM/Pollard rho; not a distinct mathematical family at the
  library's level.
- **Quadratic sieve:** GNFS strictly dominates; QS is a historical predecessor, not a
  mathematically-necessary algorithm if GNFS is present.
- **Baby-step giant-step (BSGS):** same $O(\sqrt{n})$ complexity class as Pollard rho; the
  library uses Pollard rho as the generic baseline. Not a distinct mathematical family.
- **Classical index calculus for $\mathbb{F}_p^*$:** subsumed by NFS-DL; the library covers the
  best-known algorithm.

**Finding:**

> **F-D7-02 (deliberate non-extension): The spectrum is complete for the stated discriminant
> except for the hyperelliptic Jacobian DLP gap (F-D7-01).** The absent algorithms (trial
> division, QS, BSGS, classical index calculus for $\mathbb{F}_p^*$) are all deliberate
> non-extensions: each is either subsumed by a present algorithm in the same complexity class, or
> is a historical predecessor to a present algorithm. The discriminant ("mathematically-necessary
> algorithms, excluding hardware/distributed specifics") does not require them. F-EXTEND's scope
> is therefore narrow: one genuine gap (hyperelliptic Jacobian DLP) plus any additions the
> reference distillation (D8) justifies.

*Feeds F-EXTEND; seeds C-Findings.*

---

### Summary table — spectrum-completeness findings

| # | Algorithm | Problem type | Present? | Classification | Consuming campaign |
|---|-----------|-------------|----------|----------------|--------------------|
| F-D7-01 | Hyperelliptic Jacobian DLP (Gaudry index calculus) | EC-DLP (via GHS) | No | **Genuine gap** — downstream of GHS transfer; within discriminant | F-EXTEND |
| F-D7-02 | Trial division | Integer factorization | No | Deliberate non-extension (subsumed by ECM/Pollard rho) | — |
| F-D7-02 | Quadratic sieve | Integer factorization | No | Deliberate non-extension (GNFS dominates) | — |
| F-D7-02 | Baby-step giant-step | Generic DLP / EC-DLP | No | Deliberate non-extension (same class as Pollard rho) | — |
| F-D7-02 | Classical index calculus ($\mathbb{F}_p^*$) | Finite-field DLP | Partial | Deliberate non-extension (NFS-DL subsumes) | — |

---

### Part (b) — 4-reference distillation (D8)

**Reference resolution status (verified at audit time, 2026-06-21):**

The ROADMAP D8 charge lists four references. The PLAN flags that arxiv IDs `2603.28627` and
`2606.02235` are dated beyond construction time (2026-06) and may not resolve. All four were
fetched and verified:

| # | Reference | Resolved? |
|---|-----------|-----------|
| 1 | Google QuantumAI cryptocurrency whitepaper — `quantumai.google/static/site-assets/downloads/cryptocurrency-whitepaper.pdf` | **Yes** (PDF binary; content readable) |
| 2 | `arxiv.org/abs/2603.28627` (Cain et al. 2026) | **Yes** — resolves |
| 3 | `arxiv.org/abs/2606.02235` (Schrottenloher 2026) | **Yes** — resolves |
| 4 | `github.com/ecdsafail/ecdsafail-challenge` | **Yes** — resolves |

**Note on Shor's original paper:** The PLAN's A.6 session detail lists "Shor's original paper
(1994/1997)" as a reference to distil. `arXiv:quant-ph/9508027` (Shor 1995/1996, journal
version 1997) was also fetched and verified as resolving. It is distilled below as F-D8-01.

---

### F-D8-01 · Shor (1994/1995/1997) — `arXiv:quant-ph/9508027`

**Resolved:** Yes. Peter W. Shor, "Polynomial-Time Algorithms for Prime Factorization and
Discrete Logarithms on a Quantum Computer," arXiv:quant-ph/9508027 (submitted 1995, revised
1996). Journal reference: SIAM J. Sci. Statist. Comput. 26 (1997) 1484.

**What it covers:** The original paper giving polynomial-time quantum algorithms for both integer
factorization and discrete logarithms. The factoring algorithm uses quantum order-finding (QFT +
modular exponentiation) and the continued-fraction period extraction. The DLP algorithm is a
direct adaptation: the function $f(x) = g^x \bmod p$ is periodic with period equal to the
multiplicative order of $g$, and the same QFT machinery finds the period.

**What it adds to the spectrum:**
- The foundational reference for Shor's algorithm in both the factoring and DLP settings.
- The DLP variant in the original paper is for $\mathbb{F}_p^*$ (multiplicative group), not for
  elliptic curves. The ECDLP variant (Proos–Zalka 2003) is a later extension.
- The complexity result: $O((\log N)^3)$ for factoring, $O((\log p)^3)$ for DLP in
  $\mathbb{F}_p^*$.

**Relationship to current library:** The library (Ch. 11) cites Shor's 1994 conference paper
(ref. 49 in §11.9: "Shor, P. W. (1994). Algorithms for quantum computation: discrete logarithms
and factoring. FOCS 1994.") and the Proos–Zalka ECDLP extension (ref. 43). The arXiv version
`quant-ph/9508027` is the expanded journal version (28 pages, LaTeX) — the same work, more
complete. The library already covers the mathematical content of this reference fully in Ch. 11.

**Distillation for F-EXTEND:** No new algorithmic content for F-EXTEND. The library's Ch. 11
treatment is complete and cites the original paper. The arXiv version adds no new algorithm
beyond what is already present. **This reference confirms the library's Shor coverage is
correct and complete; it does not scope new F-EXTEND work.**

---

### F-D8-02 · Google QuantumAI cryptocurrency whitepaper (2026)

**Resolved:** Yes. "Securing Elliptic Curve Cryptocurrencies against Quantum Vulnerabilities:
Resource Estimates and Mitigations," Google QuantumAI (2026). PDF available at
`quantumai.google/static/site-assets/downloads/cryptocurrency-whitepaper.pdf`.

**What it covers (from the ecdsafail-challenge README, which credits this paper):** Resource
estimates for running Shor's algorithm on elliptic-curve cryptography at cryptographically
relevant scales. The paper provides Pareto-frontier estimates for the Toffoli count and qubit
count needed to break secp256k1 (the Bitcoin/Ethereum curve) using Shor's ECDLP algorithm. The
paper is the source of the "Google's private low-qubit Pareto point" and "Google's private
low-gate Pareto point" benchmarks cited in the ecdsafail-challenge README (2,700,000 Toffoli ×
1,175 qubits and 2,100,000 Toffoli × 1,425 qubits respectively).

**What it adds to the spectrum:**
- Concrete resource estimates for Shor's ECDLP algorithm on secp256k1 at cryptographic scale.
- The paper's focus is on the *quantum circuit cost* of point addition (the inner primitive of
  Shor's ECDLP algorithm), not on the mathematical algorithm itself.
- The resource estimates are hardware-specific (fault-tolerant quantum computer with a specific
  gate set) and therefore **outside the discriminant** (the discriminant excludes
  hardware/distributed specifics).

**Relationship to current library:** The library's Ch. 11 (§11.4.4) cites Proos–Zalka [PZ03]
for the qubit-budget analysis ("approximately 768 qubits for secp256k1"). The Google whitepaper
provides more recent and lower estimates (1,175–1,425 qubits with optimized circuits). The
mathematical algorithm (Shor's two-register ECDLP) is the same; the improvement is in the
quantum circuit implementation.

**Distillation for F-EXTEND:** The resource-estimate improvement (768 → ~1,175 qubits) is a
hardware/circuit-optimization result, not a new mathematical algorithm. It is outside the
discriminant. **This reference does not scope new F-EXTEND algorithmic work.** However, it is
relevant to the §11.4.4 "Principle-4 annotation" in Ch. 11: the library currently cites the
Proos–Zalka estimate of ~768 qubits; CONSOLIDATE may update this annotation to cite the more
recent Google estimate (~1,175 qubits at the low-qubit Pareto point). This is a CONSOLIDATE
citation-update, not an F-EXTEND algorithmic addition.

**Finding (CONSOLIDATE note):** The library's §11.4.4 Principle-4 annotation cites Proos–Zalka
[PZ03] for the ~768-qubit estimate for secp256k1. The Google QuantumAI whitepaper (2026)
provides a more recent estimate of ~1,175 qubits (low-qubit Pareto point). → CONSOLIDATE may
update the citation in §11.4.4 to reference the Google whitepaper alongside Proos–Zalka.
*Feeds CONSOLIDATE (citation update); does not feed F-EXTEND.*

---

### F-D8-03 · Cain et al. (2026) — `arXiv:2603.28627`

**Resolved:** Yes. Madelyn Cain, Qian Xu, Robbie King, Lewis R. B. Picard, Harry Levine,
Manuel Endres, John Preskill, Hsin-Yuan Huang, Dolev Bluvstein, "Shor's algorithm is possible
with as few as 10,000 reconfigurable atomic qubits," arXiv:2603.28627 (submitted 30 March 2026).

**What it covers:** A resource-estimate paper showing that Shor's algorithm for integer
factorization and discrete logarithms can be executed at cryptographically relevant scales with
as few as 10,000 physical qubits (using high-rate quantum error-correcting codes and a
neutral-atom architecture). Key results:
- RSA-2048 factoring: feasible with ~10,000 physical qubits (runtime: one to two orders of
  magnitude longer than the ECDLP case).
- P-256 ECDLP (discrete logarithm on the NIST P-256 elliptic curve): feasible with ~26,000
  physical qubits, runtime of a few days under plausible assumptions.
- The improvement over prior estimates (millions of physical qubits) comes from high-rate
  quantum error-correcting codes (not the surface code), efficient logical instruction sets,
  and optimized circuit design.

**What it adds to the spectrum:**
- A significantly lower physical-qubit estimate for cryptographically relevant Shor's algorithm
  runs. Prior estimates (e.g., Proos–Zalka 2003, Roetteler et al. 2017) required millions of
  physical qubits; this paper reduces the estimate to tens of thousands.
- The mathematical algorithm (Shor's) is unchanged; the improvement is in the fault-tolerant
  implementation (error-correcting codes, physical architecture).
- The paper is hardware-specific (neutral-atom architecture) and therefore **outside the
  discriminant** for the algorithmic spectrum.

**Relationship to current library:** The library's §11.3.4 and §11.4.4 Principle-4 annotations
state that factoring RSA-2048 requires "approximately 4100 qubits" (logical qubits) and that
secp256k1 ECDLP requires "approximately 768 qubits" (logical qubits, Proos–Zalka). The Cain
et al. paper works in *physical* qubits (a different unit), so the comparison is not direct.
However, the paper's headline result (10,000 physical qubits for RSA-2048) is a significant
update to the migration-timeline narrative in §11.5.

**Distillation for F-EXTEND:** No new mathematical algorithm. The paper is a hardware/circuit
resource-estimate paper. **Outside the discriminant; does not scope F-EXTEND algorithmic work.**
However, it is relevant to the §11.5 post-quantum migration narrative: the library's §11.5.1
states "the question is when, and the answer is uncertain" — the Cain et al. result (10,000
physical qubits, potentially achievable with near-term neutral-atom hardware) updates the
urgency of the migration timeline. → CONSOLIDATE may add a citation to Cain et al. in §11.5.1
to update the migration-timeline narrative.

**Finding (CONSOLIDATE note):** The library's §11.5.1 migration narrative does not cite recent
resource-estimate results. Cain et al. (2026) provides a significantly lower physical-qubit
estimate (10,000 qubits for RSA-2048) than prior work. → CONSOLIDATE may add a citation in
§11.5.1. *Feeds CONSOLIDATE (citation update); does not feed F-EXTEND.*

---

### F-D8-04 · Schrottenloher (2026) — `arXiv:2606.02235`

**Resolved:** Yes. André Schrottenloher, "Optimized Point Addition Circuits for Elliptic Curve
Discrete Logarithms," arXiv:2606.02235 (submitted 1 June 2026).

**What it covers:** A quantum circuit optimization paper for the point-addition primitive in
Shor's ECDLP algorithm. Key results:
- Provides explicit quantum logical circuit architecture for point addition on elliptic curves
  over prime fields, achieving similar results to Babbush et al. (arXiv 2026) with a slightly
  higher qubit count (~1.5% increase) and a slightly smaller Toffoli gate count (6.5–10%
  reduction) for secp256k1.
- The paper fills a gap left by Babbush et al., who improved the cost of computing elliptic
  curve discrete logarithms but "did not reveal their logical quantum circuits, relying instead
  on a zero-knowledge proof."
- The circuit is valid for any prime field (not just secp256k1).
- The paper builds on prior work by Chevignard et al. (CRYPTO 2024) and Gidney (arXiv 2025)
  for RSA factoring, and Litinski (arXiv 2023) for ECDLP.

**What it adds to the spectrum:**
- Explicit quantum circuit architecture for elliptic-curve point addition, the inner primitive
  of Shor's ECDLP algorithm.
- The mathematical algorithm (Shor's two-register ECDLP) is unchanged; the improvement is in
  the circuit implementation of the point-addition subroutine.
- The paper is a circuit-optimization paper, not a new algorithm. It is **outside the
  discriminant** (hardware/circuit specifics).

**Relationship to current library:** The library's `shor/src/` implements the point-addition
circuit for Shor's ECDLP algorithm at toy scale (17 qubits for a curve with $r = 13$). The
Schrottenloher paper provides an optimized circuit for secp256k1 at cryptographic scale. The
mathematical content (the two-register hidden-subgroup formulation, the QFT, the 2D-lattice
extraction) is already fully covered in Ch. 11.4.

**Distillation for F-EXTEND:** No new mathematical algorithm. The paper is a circuit-level
optimization. **Outside the discriminant; does not scope F-EXTEND algorithmic work.** The paper
is relevant to the §11.4.4 Principle-4 annotation as a more recent qubit-count reference.

**Finding (CONSOLIDATE note):** Schrottenloher (2026) provides explicit circuit counts for
secp256k1 ECDLP that are more recent than Proos–Zalka [PZ03]. → CONSOLIDATE may add a citation
in §11.4.4 alongside Proos–Zalka. *Feeds CONSOLIDATE (citation update); does not feed F-EXTEND.*

---

### F-D8-05 · ecdsafail/ecdsafail-challenge (GitHub)

**Resolved:** Yes. `github.com/ecdsafail/ecdsafail-challenge` — "A collaborative effort to build
the leanest circuit that breaks ECDSA." A Rust benchmark harness for optimizing the reversible
quantum circuit for secp256k1 point addition, scored by Toffoli count × peak qubit width.

**What it covers:** A competitive benchmark for quantum circuit optimization of the secp256k1
point-addition primitive. The harness validates circuits against 9024 random test points and
scores by Toffoli × qubits. The README credits the Google QuantumAI whitepaper as the source
of the benchmark harness. Reference numbers: the challenge initial circuit scores 1.07 × 10¹⁰
(3,942,753 Toffoli × 2,715 qubits); Google's private Pareto points are ~3.0–3.2 × 10⁹.

**What it adds to the spectrum:**
- A practical benchmark for quantum circuit optimization of the ECDLP inner primitive.
- The mathematical algorithm (Shor's ECDLP) is unchanged; this is a circuit-engineering
  challenge.
- **Outside the discriminant** (hardware/circuit specifics, not a mathematical algorithm).

**Distillation for F-EXTEND:** No new mathematical algorithm. **Outside the discriminant; does
not scope F-EXTEND algorithmic work.** The ecdsafail challenge is relevant as a pointer to the
state of the art in quantum circuit optimization for ECDLP, but it does not add to the
mathematical spectrum the library covers.

---

### Summary table — reference distillation findings

| # | Reference | Resolves? | Adds to mathematical spectrum? | F-EXTEND scope? | CONSOLIDATE note? |
|---|-----------|-----------|-------------------------------|-----------------|-------------------|
| F-D8-01 | Shor (1994/1995/1997) `quant-ph/9508027` | Yes | No (already fully covered in Ch. 11) | No | No |
| F-D8-02 | Google QuantumAI whitepaper (2026) | Yes | No (hardware/circuit resource estimates) | No | Yes — update §11.4.4 qubit estimate |
| F-D8-03 | Cain et al. (2026) `2603.28627` | Yes | No (hardware/circuit resource estimates) | No | Yes — update §11.5.1 migration narrative |
| F-D8-04 | Schrottenloher (2026) `2606.02235` | Yes | No (circuit-level optimization) | No | Yes — update §11.4.4 circuit reference |
| F-D8-05 | ecdsafail/ecdsafail-challenge | Yes | No (circuit engineering benchmark) | No | No |

**Headline finding (D8):** All four ROADMAP-listed references resolve (plus Shor's arXiv paper).
None adds a new mathematical algorithm to the spectrum. All three 2026 references (Google
whitepaper, Cain et al., Schrottenloher) are hardware/circuit resource-estimate papers that
update the migration-timeline narrative and the qubit-count annotations in Ch. 11, but do not
scope new F-EXTEND algorithmic work. The reference distillation confirms the library's
quantum-DLP coverage is mathematically complete; the updates are citation-level and belong to
CONSOLIDATE.

---

### Subtleties and deferrals

**The hyperelliptic Jacobian DLP gap (F-D7-01) is the only genuine F-EXTEND candidate.** The
gap is within the discriminant: the GHS chapter explicitly defers the downstream solve, and the
downstream algorithm (Gaudry index calculus) is a distinct mathematical family. The
`rho/src/hyperelliptic/` module (Mumford divisors, Cantor group law) is the substrate; the
index calculus layer is what is missing. F-EXTEND's scope for this gap is: (a) a mathematical
treatment of Gaudry's index calculus for hyperelliptic Jacobians in `MATHEMATICS.md`; (b) an
implementation in `rho/src/hyperelliptic/` or a new module; (c) KATs. The EXTEND scope-ceiling
question (arc-2 vs arc-3) is a human open-Q at A.7.

**The 2026 references all resolve.** The PLAN's risk flag ("arxiv IDs `2603.28627` and
`2606.02235` are dated beyond construction time — may not resolve") is resolved: both IDs
resolve as of 2026-06-21. The internal-continue protocol (note as unverifiable if they 404) was
not triggered.

**The reference distillation does not scope new F-EXTEND algorithmic work.** All four references
are either (a) already covered (Shor), or (b) hardware/circuit resource-estimate papers outside
the discriminant. The discriminant ("mathematically-necessary algorithms, excluding
hardware/distributed specifics") correctly excludes the qubit-count and circuit-optimization
content of the 2026 papers. F-EXTEND's scope is therefore determined by the spectrum audit
(F-D7-01), not by the reference distillation.

**CONSOLIDATE citation updates (not F-EXTEND).** The three 2026 references (Google whitepaper,
Cain et al., Schrottenloher) are relevant to the §11.4.4 and §11.5.1 annotations in Ch. 11.
These are citation-level updates (adding references to more recent qubit-count estimates and
migration-timeline data) that belong to CONSOLIDATE, not F-EXTEND. A.6 records them as
CONSOLIDATE notes; they are not falsifiable F-EXTEND findings.

*Feeds F-EXTEND (F-D7-01 only); feeds CONSOLIDATE (F-D8-02, F-D8-03, F-D8-04 citation notes);
seeds C-Findings.*

---

## A.7 — Findings Ledger (authoritative scope source for all five campaigns) ◆

**Charge:** consolidate all findings from A.1–A.6 into the authoritative findings ledger; freeze
C-Findings, C-Testing-Philosophy, and C-Coherence; record sketches of C-Layout, C-DocsLayer,
C-MathSpine, and C-Oracle; surface the four human open-Qs with adjudicator recommendations.

**Governing rule — scope-routing invariant (load-bearing).** SURVEY split execution by artifact
register, and the routing is binding for all five campaigns:

> **Inline `//!`/`///`/`//` doc-comments + identifier/file/dir/bench/test renames → REFACTOR
> (compiler blast radius); human prose (PEDAGOGY/MATHEMATICS/README) → CONSOLIDATE; config/
> manifest/CI → ALIGN; CADO sidecar build → ORACLE; new algorithm/chapter → EXTEND.**

A finding routed to the wrong campaign is the primary cross-campaign defocus mode. The ledger
below enforces this routing for every entry.

**Falsifiability law (binds the ledger).** Every entry below states a checkable boundary. A
campaign that acts beyond its findings is defocus; a campaign that ignores a finding is rigidity.
The boundary is the falsifiable contract: a downstream session is checked against it.

---

### Ledger by consuming campaign

#### ALIGN (B) — formalities reconciliation (A.1, F-D1-01…12)

*In scope:* add the 8 absent root formalities items as backports and seed the 3 rDLP→template
forward-seeds.

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D1-01 | A.1 | Add `deny.toml` at workspace root | Do not redesign the formalities set |
| F-D1-02 | A.1 | Add `rustfmt.toml` at workspace root | — |
| F-D1-03 | A.1 | Resolve `rho/rust-toolchain.toml` mis-scoping: promote to root or remove | ALIGN decides; A.1 records only the gap |
| F-D1-04 | A.1 | Add GPL-3.0 `LICENSE` at workspace root; add `license` field to crate manifests | — |
| F-D1-05 | A.1 | Promote `[lints]` to `[workspace.lints]`; add `[lints] workspace = true` to all 9 crates | — |
| F-D1-06 | A.1 | Add coverage gate honoring C-Testing-Philosophy doctrine (gate value: open-Q below) | Gate value calibrated against measured baseline; doctrine shape is frozen |
| F-D1-07 | A.1 | Add `docs/development.md` adapted for the workspace | — |
| F-D1-08 | A.1 | Add `.cargo/config.toml` with workspace-scoped aliases | — |
| F-D1-09 | A.1 | Add `AGENTS.md` at workspace root | — |
| F-D1-10 | A.1 | Seed three-tier docs layering into `rust-template` | — |
| F-D1-11 | A.1 | Seed dev-oracle policy into `rust-template` | — |
| F-D1-12 | A.1 | Seed docs-register contracts into `rust-template` | — |

*Defocus boundary:* ALIGN implements the recorded gap only — it does not redesign the formalities
set, and it does not set the coverage doctrine (that is C-Testing-Philosophy, frozen). The
coverage gate value is an open-Q (below); ALIGN calibrates the number against a measured
`cargo llvm-cov` baseline, honoring the frozen doctrine shape.

---

#### ORACLE (C) — CADO-NFS sidecar (A.5, F-D6-01…04)

*In scope:* build the sidecar to the recorded design.

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D6-01 | A.5 | Lazy/on-demand + opt-in CI flag trigger; `#[ignore]` gate preserved | Must NOT promote oracle tests to the green path |
| F-D6-02 | A.5 | Pin to git tag on GitHub mirror, tag + commit hash recorded; minimum `git-2.0.1` (`88c4751`) | Must clone/download pinned tag, not `master` or `HEAD` |
| F-D6-03 | A.5 | Implement the four existing `#[ignore]`-gated stubs as tolerance-bounded consistency checks (relation count ≥ CADO/3.0; matrix dims ±10%; N=35 factor; 80–100-bit factor) | Must not add new oracle tests beyond the four stubs; must not change tolerances without documented rationale |
| F-D6-04 | A.5 | Sidecar is validation-only, never on green path, never a `Cargo.toml` dependency | Must not add CADO to any `[dev-dependencies]`; must not remove `#[ignore]`; comparison direction is always rDLP-first |

*Defocus boundary:* ORACLE must NOT promote any oracle test to the green path, add CADO to any
`Cargo.toml`, reverse the rDLP-first comparison direction, or add new oracle tests beyond the
four stubs. Build-automation-intricacy (opus-ORACLE) is an ORACLE-internal call.

---

#### REFACTOR (D) — layout + code-depth de-provenancing (A.2, F-D5-01…03, F-D9-01…06)

*In scope:* collapse/dedup the re-export wrappers and scrub code-adjacent provenance.

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D5-01 | A.2 | `rho` overload: peer Track-E attacks out or keep co-located — **gated on human open-Q below** | Must not pre-empt the human decision; owns the decision + compiler blast radius once answered |
| F-D5-02 | A.2 | Collapse `rho/src/field/` and `rho/src/util/` re-export wrappers; remove duplicated `batch_invert` tests | Wrapper collapse is the dedup target; the `L = 4` fixation may warrant keeping the wrapper — REFACTOR decides |
| F-D5-03 | A.2 | No action — `shared/*` carries no dedup beyond the two `rho` wrappers | Inventing further dedup is defocus |
| F-D9-01 | A.2 | Re-anchor `Phase 4`–`Phase 8` tokens in `rho/src/ecdlp/`, `rho/benches/`, `rho/tests/ecdlp_kat.rs`, `rho/src/curve/`, `rho/src/field/monty.rs` on optimization-layer names | Algorithmic phase labels within a single function (not planning-frame) are NOT in scope |
| F-D9-02 | A.2 | Replace `Track D` tokens in `gnfs/src/dl/mod.rs` and descent sub-modules with topic-native language | — |
| F-D9-03 | A.2 | Replace sub-track IDs (`E.X.Y`, `G.X.Y`, `D.X.Y`, `S.X.Y`) in `///`/`//!`/`//` comments across `rho/src/`, `gnfs/src/`, `shared/*/src/` | Human prose is CONSOLIDATE's |
| F-D9-04 | A.2 | Rename `sub_track_close_curve_axioms_intact` (the single planning-frame token in a code identifier) to a topic-native name; replace `sub-track` in doc-comments | — |
| F-D9-05 | A.2 | Replace `PLAN.md E.D.2` and `G.A.1a session contract` references in test doc-comments; replace `◆ sub-track close` milestone labels | — |
| F-D9-06 | A.2 | Replace `E.W.1` bench label in `rho/benches/attacks.rs`; replace `Phase E.A.2` section header in `rho/tests/ecdlp_kat.rs` | — |

**Headline sizing signal (load-bearing for REFACTOR):** the de-provenancing blast radius is
**doc-comment-heavy, identifier-light** — exactly one planning-frame token in a code identifier
(`sub_track_close_curve_axioms_intact`); all other tokens are in `//!`/`///`/`//` comments.
Identifier renames are cheap (compiler-checked); the bulk of REFACTOR's de-provenancing work is
doc-comment scrubbing.

*Defocus boundary:* REFACTOR touches code-adjacent artifacts only — human prose is CONSOLIDATE's.
The `rho`-overload layout decision is gated on the human open-Q (below). `shared/*` carries no
dedup beyond the two `rho` wrappers — REFACTOR inventing further dedup is defocus.

---

#### CONSOLIDATE (E) — docs-layer + math-spine + prose de-provenancing + citation updates
(A.4, F-D3-01…04, F-D4-01…06, F-D9-07…14; A.6, F-D8-02/03/04)

*In scope:* fix the 3 docs-layer defects, normalize the math-spine, re-anchor ~185 prose
provenance tokens, update 3 citation annotations.

**Docs-layer defects (3):**

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D3-01 | A.4 | Replace ~9 bare Rust module paths in `MATHEMATICS.md` "Code realisation" lines with PEDAGOGY-section citations | Mathematical prose is clean; only the cross-reference lines need fixing |
| F-D3-02 | A.4 | Remove stale "to be written in S.D.2" annotations in `shor/docs/PEDAGOGY.md` lines 4 and 480 | — |
| F-D3-03 | A.4 | Add standard `> **Maths-first treatment.**` header blockquote to `shared/numfield/docs/PEDAGOGY.md` | — |

**Math-spine defects:**

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D4-01 | A.4 | Normalize chapters 8 and 9 from `#` to `##` headings; adjust internal sub-headings accordingly | — |
| F-D4-05 | A.4 | May add a brief "How to read this textbook" note to the ToC section (one paragraph) | CONSOLIDATE design decision, not mandated |
| F-D4-06 | A.4 | Normalize citation style: `§§52–62` vs plain numbered headings in `gnfs/docs/PEDAGOGY.md` | — |

**Prose de-provenancing (~185 tokens across 6 files):**

| Finding | Source | Token type | Count | Classification | Action |
|---------|--------|-----------|-------|----------------|--------|
| F-D9-07 | A.4 | `Track X` in `docs/MATHEMATICS.md` | ~25 | Grouping-coincides-with-topic | Re-anchor label (e.g., "Track ρ" → "Pollard rho") |
| F-D9-08 | A.4 | `Track X` in `docs/PEDAGOGY.md` | ~40 | Grouping-coincides-with-topic | Re-anchor label |
| F-D9-09 | A.4 | `Track X` in `README.md` | ~20 | Grouping-coincides-with-topic | Re-anchor label |
| F-D9-10 | A.4 | `Phase N` (N=0–8) in `docs/PEDAGOGY.md` | ~30 | Pure residue | Re-anchor on optimization-technique names |
| F-D9-11 | A.4 | Session/sub-track IDs (T.G, D.W.2, etc.) in `docs/MATHEMATICS.md` | ~15 | Pure residue | Replace with chapter titles or topic-native references |
| F-D9-12 | A.4 | S.A/S.B/S.C (coarse) and S.A.1/S.B.2 etc. (fine) in `shor/docs/PEDAGOGY.md` | ~50 | Borderline (open-Q below) / Pure residue | Fine-grained IDs dissolved; coarse IDs per open-Q |
| F-D9-13 | A.4 | G.B–G.W, D.A–D.W (coarse) and G.B.1 etc. (fine) in `gnfs/docs/PEDAGOGY.md` | many | Grouping-coincides-with-topic / Pure residue | Fine-grained IDs dissolved; coarse IDs survive as pipeline-stage labels |
| F-D9-14 | A.4 | `◆` boundary marks in `shor/docs/PEDAGOGY.md` | ~7 | Pure residue | Replace "frozen S.X.Y ◆" with "frozen" or "stable interface" |

**Citation updates (3 CONSOLIDATE notes from A.6):**

| Finding | Source | In-scope action |
|---------|--------|-----------------|
| F-D8-02 | A.6 | Update §11.4.4 qubit estimate to cite Google QuantumAI whitepaper (2026) alongside Proos–Zalka |
| F-D8-03 | A.6 | Add citation to Cain et al. (2026) in §11.5.1 migration-timeline narrative |
| F-D8-04 | A.6 | Add citation to Schrottenloher (2026) in §11.4.4 alongside Proos–Zalka |

*Defocus boundary:* CONSOLIDATE re-anchors and reconciles existing prose — it does NOT author new
mathematical content (that is EXTEND), and it does not restructure beyond the recorded defects.
The C-Coherence borderline-aggressiveness default is a human open-Q (below) that sets the cut
depth. Code-adjacent artifacts (inline `//!`/`///`/`//` doc-comments) are REFACTOR's.

---

#### EXTEND (F) — spectrum completion (A.6, F-D7-01; bounded by F-D7-02, F-D8-01…05)

*In scope (if the human admits it to arc-2 — see scope-ceiling open-Q):* exactly one genuine gap.

| Finding | Source | In-scope action | Defocus boundary |
|---------|--------|-----------------|------------------|
| F-D7-01 | A.6 | Hyperelliptic Jacobian DLP (Gaudry index calculus): add mathematical treatment in `MATHEMATICS.md`; implement index calculus layer in `rho/src/hyperelliptic/` or a new module; add KATs | Gated on scope-ceiling open-Q below; substrate exists (`rho/src/hyperelliptic/`: Mumford divisors, Cantor group law) |
| F-D7-02 | A.6 | No action — trial division, QS, BSGS, classical index calculus for 𝔽ₚ* are deliberate non-extensions | Adding any of these is defocus |
| F-D8-01…05 | A.6 | No action — all four references add no new mathematical algorithm to the spectrum | The 2026 papers are citation updates for CONSOLIDATE, not F-EXTEND algorithmic scope |

*Defocus boundary (sharp):* the spectrum is otherwise complete for the discriminant. EXTEND
adding anything beyond the one Gaudry gap is defocus; a deliberate non-extension is a successful
finding, not a gap to fill.

---

### Four human open-Qs (resolved by preference/scope, not by audit)

These ride to the step-3 review as flagged recommendations; they do not block the freeze. The
audit resolved every factual/doctrinal question; these four are preference/scope calls.

**Open-Q 1 — `rho`-overload split (D5, feeds REFACTOR).**
Peer Track-E's 8 attack modules into a separate crate, or keep them co-located with the
Pollard-rho baseline they are benchmarked against?

*Cohesion argument for keeping:* the E.W cross-attack bench measures the attacks against the rho
baseline; they share `rho::curve`/`rho::field` types directly; the pedagogical "attacks live with
the baseline they're measured against" framing is load-bearing.
*Organisation argument for splitting:* 8 algebraically-distinct attack modules under a "Pollard
rho" crate name; a reader encounters a much larger surface than the crate name suggests.
*Adjudicator recommendation: keep-with-baseline.* The measured-against cohesion is load-bearing;
a split buys crate-name honesty at the cost of that cohesion. *(Worse at: a reader still meets a
larger surface than "Pollard rho" advertises.)*

**Open-Q 2 — Coverage gate value (D2, feeds ALIGN).**
The doctrine shape is frozen (math-behavior KAT primary; line coverage a secondary dead-code
floor). The number (80% recommended) is unmeasured. ALIGN calibrates against a real `cargo
llvm-cov` baseline; the human's lean (80 / 90 / 100-with-`#[ignore]`-exclusions) sets the target.
*Adjudicator recommendation: 80% floor, ALIGN re-tunes to the measured baseline.* *(Worse at:
80% is a reasoned guess, not a measurement — a baseline far from 80% should move it.)*

**Open-Q 3 — F-EXTEND scope ceiling (D7, feeds EXTEND).**
Is arc-2 the place to fill the one genuine gap (hyperelliptic Jacobian DLP), or does it become
an arc-3 roadmap, keeping arc-2 consolidation-focused?
*Adjudicator recommendation: defer to arc-3.* The gap is real but self-contained; arc-2's stated
spine is consolidation/alignment; EXTEND is the one campaign with no upstream pressure; filling
it in arc-2 widens scope against the survey-first/consolidate-first discipline. *(Worse at:
leaves the GHS chapter a "dangling transfer" one cycle longer.)*

**Open-Q 4 — C-Coherence borderline default (D9, feeds REFACTOR + CONSOLIDATE).**
For groupings where track ≈ topic (the S.A/S.B/S.C Shor sub-tracks; coarse G.*/D.* pipeline
stages), preserve the grouping under a topic-native label, or dissolve into a purer topical
organization?
*Adjudicator recommendation: preserve-under-topic-label.* The coarse groupings map onto real
mathematical families (the ROADMAP's own D9 example), so dissolving them discards real structure;
only the fine-grained S.X.Y / G.X.Y session IDs are pure residue and dissolve. *(Worse at: keeps
a faint planning-shaped silhouette in the section structure.)*

---

### Findings ledger summary table

| Finding | Source | Consuming campaign | Classification |
|---------|--------|--------------------|----------------|
| F-D1-01 | A.1 | ALIGN | `deny.toml` absent |
| F-D1-02 | A.1 | ALIGN | `rustfmt.toml` absent |
| F-D1-03 | A.1 | ALIGN | `rho/rust-toolchain.toml` mis-scoped |
| F-D1-04 | A.1 | ALIGN | `LICENSE` absent |
| F-D1-05 | A.1 | ALIGN | `[lints]` in 3/9 crates only |
| F-D1-06 | A.1 | ALIGN | coverage gate absent (gate value open-Q 2) |
| F-D1-07 | A.1 | ALIGN | `development.md` absent |
| F-D1-08 | A.1 | ALIGN | `.cargo/config.toml` absent |
| F-D1-09 | A.1 | ALIGN | `AGENTS.md` absent |
| F-D1-10 | A.1 | ALIGN | multisession docs layering (rDLP → template) |
| F-D1-11 | A.1 | ALIGN | dev-oracle policy (rDLP → template) |
| F-D1-12 | A.1 | ALIGN | docs-register contracts (rDLP → template) |
| F-D5-01 | A.2 | REFACTOR | `rho` hosts 8 Track-E modules (open-Q 1) |
| F-D5-02 | A.2 | REFACTOR | `rho/src/field/` + `rho/src/util/` re-export wrappers; `batch_invert` tests duplicated |
| F-D5-03 | A.2 | — | No dedup needed in `shared/*` |
| F-D9-01 | A.2 | REFACTOR | `Phase N` tokens in `rho/src/ecdlp/`, benches, tests — pure residue |
| F-D9-02 | A.2 | REFACTOR | `Track D` tokens in `gnfs/src/dl/` — pure residue |
| F-D9-03 | A.2 | REFACTOR | Sub-track IDs (`E.X.Y`, `G.X.Y`, `D.X.Y`) in doc-comments — pure residue |
| F-D9-04 | A.2 | REFACTOR | `sub_track_close_curve_axioms_intact` test name (single identifier hit); `sub-track` in doc-comments — pure residue |
| F-D9-05 | A.2 | REFACTOR | `PLAN.md E.D.2`, `G.A.1a session contract`, `◆ sub-track close` in test doc-comments — pure residue |
| F-D9-06 | A.2 | REFACTOR | `E.W.1` bench label; `Phase E.A.2` section header — pure residue |
| F-D2-01 | A.3 | CONSOLIDATE | `shared/numth` has no `tests/` dir; all 34 tests inline |
| F-D2-06 | A.3 | ALIGN, CONSOLIDATE, F-EXTEND | Coverage doctrine resolved: math-behavior KAT; 80% line gate |
| F-D3-01 | A.4 | CONSOLIDATE | `MATHEMATICS.md` ~9 bare Rust paths in "Code realisation" lines |
| F-D3-02 | A.4 | CONSOLIDATE | `shor/docs/PEDAGOGY.md` stale "to be written in S.D.2" |
| F-D3-03 | A.4 | CONSOLIDATE | `shared/numfield/docs/PEDAGOGY.md` missing maths-first header |
| F-D4-01 | A.4 | CONSOLIDATE | Chapters 8 and 9 use `#` heading; all others use `##` |
| F-D4-05 | A.4 | CONSOLIDATE | No explicit suggested-reading-path section (optional add) |
| F-D4-06 | A.4 | CONSOLIDATE | Citation-style inconsistency (`§§52–62` vs plain numbered headings) |
| F-D9-07 | A.4 | CONSOLIDATE | ~25 `Track X` tokens in `docs/MATHEMATICS.md` — grouping-coincides-with-topic |
| F-D9-08 | A.4 | CONSOLIDATE | ~40 `Track X` tokens in `docs/PEDAGOGY.md` — grouping-coincides-with-topic |
| F-D9-09 | A.4 | CONSOLIDATE | ~20 `Track X` tokens in `README.md` — grouping-coincides-with-topic |
| F-D9-10 | A.4 | CONSOLIDATE | ~30 `Phase N` tokens in `docs/PEDAGOGY.md` — pure residue |
| F-D9-11 | A.4 | CONSOLIDATE | ~15 session/sub-track IDs in `docs/MATHEMATICS.md` — pure residue |
| F-D9-12 | A.4 | CONSOLIDATE | ~50 S.A/S.B/S.C tokens in `shor/docs/PEDAGOGY.md` — borderline (open-Q 4) |
| F-D9-13 | A.4 | CONSOLIDATE | G.B–G.W, D.A–D.W tokens in `gnfs/docs/PEDAGOGY.md` — mixed |
| F-D9-14 | A.4 | CONSOLIDATE | ~7 `◆` marks in `shor/docs/PEDAGOGY.md` — pure residue |
| F-D6-01 | A.5 | ORACLE | Sidecar trigger = lazy/on-demand + opt-in CI flag |
| F-D6-02 | A.5 | ORACLE | Version pin = git tag + commit hash; minimum `git-2.0.1` (`88c4751`) |
| F-D6-03 | A.5 | ORACLE | Four tolerance-bounded consistency comparisons (existing stubs) |
| F-D6-04 | A.5 | ORACLE | Dev-oracle policy fidelity: validation-only, never on green path |
| F-D7-01 | A.6 | F-EXTEND | Hyperelliptic Jacobian DLP (Gaudry index calculus) — genuine gap |
| F-D7-02 | A.6 | — | Trial division, QS, BSGS, classical index calculus — deliberate non-extensions |
| F-D8-01 | A.6 | — | Shor (1994/1997) — already fully covered; no new scope |
| F-D8-02 | A.6 | CONSOLIDATE | Google QuantumAI whitepaper — update §11.4.4 qubit estimate |
| F-D8-03 | A.6 | CONSOLIDATE | Cain et al. (2026) — update §11.5.1 migration narrative |
| F-D8-04 | A.6 | CONSOLIDATE | Schrottenloher (2026) — update §11.4.4 circuit reference |
| F-D8-05 | A.6 | — | ecdsafail-challenge — outside discriminant; no scope |

*This table is the authoritative scope source for all five campaigns. C-Findings is frozen.*
