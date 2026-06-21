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
