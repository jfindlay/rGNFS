<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-D extension (D.E — k>1 NFS-DL: F_{p^k} discrete log)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **held up by lever 3 (cost of design error), the binding
constraint.** D.E is the **hard predecessor of E.C** (the MOV/Frey–Rück climax): E.C reduces an
ECDLP on a small-embedding-degree curve to a discrete log in `F_{p^k}*` and calls `solve_dl` (C2).
But C2 was frozen at D.C.3 with the **k>1 path returning `SolveDlError::Unsupported`** — and the MOV
fixture is genuinely `k=2` (μ_ℓ ⊂ F_{p²}*, μ_ℓ ⊄ F_p*), so **no honest k=1 projection exists**. D.E
lifts NFS-DL from F_p to F_{p^k}, making C2 honest at k=2. The D.E.1 **target-embedding substrate**
(how an `FpExt` element — the pairing output — is expressed in the NFS-DL number field and fed to the
solver) **bounds D.E.2/3 and E.C**: a wrong representation freeze is the highest-cost-of-wrong
interface remaining before the climax. Lever 5 is strong (the end-to-end `g^x = h` round-trip in
F_{p²} plus a PARI `znlog`/`fflog` cross-check is decisive and fast) and *would* license an opt-down
in isolation, but lever 3 dominates, so the ◆ that ratifies C-ExtTarget / C-ExtFactorBase / C2-ext
runs at **Opus**. *(Scoped to the D.E.3 ◆ close. D.E.1 — the target-embedding substrate design — is
Opus-tier; D.E.2–3 are Sonnet against the frozen target/factor-base representation.)*

Last rewrite: **E.B ◆ boundary crossed** (E.B.1 `74b2ff3`, E.B.2 `dceaff1`, E.B.3 `7e28962`, E.B.4 ◆
`6c082bb`; C-FpExt / C-PairingCurve / C-Pairing frozen; the Weil + Tate pairing substrate ships,
proven bilinear/non-degenerate). The E.B.4 ◆ juncture recorded the **load-bearing assumption** this
plan resolves: *"E.C calls solve_dl with k=1 (prime subfield projection) OR an E.C-prep session
widens C2 to k>1 before E.C lands."* The substrate survey confirmed there is **no honest k=1 path**
for the k=2 MOV fixture, so the second branch is mandatory. This plan shards that branch as **D.E**,
a Track-D extension sub-track (a one-line ROADMAP insertion is owed as a capture candidate — see
Discoveries & risks).

---

## Purpose (design intent)

Per ROADMAP (Phase γ, Contract C2 + the D.W closeout Discoveries entry): C2 `solve_dl(g, h, p, k,
ell)` is frozen, **live for k=1, returning `Unsupported` for k>1**; "*the F_{p^k} NFS-DL extension is
genuine new mathematics, deferred to a dedicated E.C-prep ROADMAP-then-shard session (never
spontaneous in-flight scope growth during E.C).*" D.E **is** that session-set. It is the substrate
that makes the project's pedagogical climax (E.C, the MOV bridge first calling a *real* NFS-DL
solver) possible: without it, E.C can only call a permanent PARI stub, which ROADMAP C2 explicitly
forbids ("*merging E.C with a PARI stub permanently is not [acceptable]*").

The structure-based-escape-from-search through-line continues: E.B built the **pairing** (the
homomorphism `e: E[ℓ] × E[ℓ] → μ_ℓ ⊂ F_{p^k}*` that transports the ECDLP into a finite-field DLP);
D.E builds the **finite-field DLP solver in the extension** that the transported problem lands in;
E.C composes them into the MOV reduction. D.E ships no attack — it ships the k>1 NFS-DL solver and
proves it recovers the correct discrete log in `F_{p²}*` on the `pairing_toy` fixture's target field.

The substrate survey established the shape precisely:

1. **The k=1 NFS-DL pipeline is ~3,500 LOC of frozen, index-agnostic substrate that mostly
   transfers.** `gnfs/src/dl/`: the F_ℓ linear algebra (`build_fl_matrix`, `block_lanczos_fl`,
   `recover_virtual_logs`) is keyed on factor-base *indices*, not integer magnitudes — it is
   **already extension-ready**. The Schirokauer map (`compute_schirokauer`) was **over-specified at
   D.A.1 to carry r>1** (`gnfs/src/dl/mod.rs:32`: *"the r>1 multi-coordinate shape is carried even
   when toy instances use r=1, since D.C's descent and E.C's solver will need it"*). The `DLRelation`
   wrapper composes cleanly. **So D.E is a bounded lift, not a rewrite** — what changes is the
   *target embedding*, the *factor base*, and the *individual-log descent*.

2. **The number-field machinery stays char-0; only the residue field is the extension.** NFS-DL over
   F_{p^k} does **not** mean "redo NFS with F_{p^k} coefficients." The number field K = ℚ[α]/(f) is
   char-0 as before; what changes is that the prime p now has a **degree-k prime ideal** above it
   whose residue field is F_{p^k}, and the DL target h lives in that residue field. This is the
   rigidity guard restated from E.B: do **not** reach for the char-p `FpExt` as the relation-algebra
   coefficient field — `FpExt` is the *target representation* (D.E.1's job: the residue map), not the
   sieve algebra.

3. **C2's signature and error taxonomy are FROZEN — D.E fills in the k>1 path behind them, it does
   not change them.** `gnfs/src/dl/descent/solve.rs:22–23, 100`: *"No further variants will be
   added… E.C consumes exactly these three variants."* The `k` parameter and `Unsupported` variant
   already exist. D.E removes the early `if k != 1 { return Unsupported }` return and implements the
   real path — a **contract-respecting extension, not a contract break.** The `Unsupported` variant
   stays (for k beyond the toy ceiling).

4. **The cross-crate KAT seam: gnfs holds solve_dl; rho holds the pairing fixture.** `rho` will
   depend on `gnfs` (added at E.C, not D.E — D.E adds no rho dependency). D.E's end-to-end k=2 KAT
   lives **in `gnfs/`** and constructs its F_{47²} DL target **independently** (a hand-built
   extension-field instance matching the `pairing_toy` parameters: p=47, k=2, ℓ=3, modulus u²+1).
   The full MOV-to-pairing round-trip (pairing output → solve_dl) belongs to **E.C in `rho`**, not
   D.E. D.E proves the solver; E.C wires it to the pairing.

5. **The FpExt → solve_dl-input representation mismatch is D.E.1's central design call.** The survey
   found `solve_dl` takes `h: &BigInt` (a prime-field integer); `FpExt::to_uint_vec() -> Vec<Uint<4>>`
   exists as a coefficient bridge but **no single-step `FpExt → solver-target` encoding exists**. The
   solver's k>1 path needs a target type richer than `BigInt` (an element of F_{p^k} is a coefficient
   vector). D.E.1 designs that representation (C-ExtTarget) — the interface E.C reads pairing outputs
   through.

The work splits at the **target-embedding → factor-base → descent** seams, 3 sessions:

1. **D.E.1 — F_{p^k} target-embedding substrate (Opus, Cat A).** The representation of a DL target
   in `F_{p^k}*` for the NFS-DL solver, and the residue map between `FpExt` (E.B's char-p extension)
   and the NFS-DL number field's degree-k residue field. **The Opus design call** (the reason this
   session is Opus and bounds E.C): the *solver-target type* the k>1 `solve_dl` path consumes — does
   it take a coefficient `Vec<BigInt>`, a thin residue-map wrapper, or accept `FpExt` directly? —
   and how `g, h ∈ F_{p^k}*` are encoded. **Freezes C-ExtTarget** — the extension-target interface
   D.E.2/3 and E.C consume.

2. **D.E.2 — Extension factor base + relation collection over the degree-k prime (Sonnet, Cat A).**
   The factor base augmented with the **degree-k prime ideal** whose residue field is F_{p^k}, and
   relation collection adapted to the extension target (the Schirokauer ideals over ℓ in the extension
   setting; the `init_descent_frontier` smoothing of an extension target). Composes the frozen
   C-Schirokauer (r>1, already carried) and C-LinAlgFl (index-agnostic, already extension-ready).
   **Freezes C-ExtFactorBase.**

3. **D.E.3 ◆ — k>1 individual-log descent + `solve_dl` k>1 wiring + end-to-end k=2 KAT (Sonnet, Cat
   B, `@plan`).** The individual-log descent for an extension target, **removing the `solve_dl`/
   `solve_dl_full` `Unsupported` early-return** and threading the k>1 path through the frozen C2
   signature; the **end-to-end k=2 KAT** (recover `log_g(h)` in F_{47²}*, verify `g^x = h`) and a
   **PARI `znlog`/`fflog` `#[ignore]` cross-check**. **Freezes C2-ext** (the k>1 `solve_dl` contract
   E.C consumes — C2 made honest at k=2). Crosses the **D.E ◆ boundary** (sub-track complete; E.C's
   hard predecessor exists).

Re-read this intent at the ◆ boundary to catch **defocus** (implementing the MOV reduction itself —
that is **E.C**, not D.E; D.E ships the k>1 NFS-DL solver and stops at proving it recovers the right
log. Or implementing a general crypto-scale F_{p^{12}} NFS-DL with k=12 towers — toy k=2 suffices)
and **rigidity** (reaching for E.B's char-p `FpExt` as the *sieve algebra* coefficient field — wrong;
the number field stays char-0 ℚ[α]/(f), `FpExt` is the *target representation* only. Or adding a
fourth `SolveDlError` variant — the taxonomy is frozen; the k>1 path uses the existing three).

**Scoping discipline.** D.E lifts NFS-DL to **toy extension degree k=2** (the `pairing_toy` fixture's
embedding degree) at demonstration fidelity (principle 4). It introduces **no new live oracle** (PARI
`znlog`/`fflog` is the established `#[ignore]` dev-only cross-check, matching D.B's pattern). It
**amends nothing** in the frozen k=1 path — the k=1 KATs must stay green (`solve_dl` with k=1 is
unchanged); the k>1 path is purely additive behind the frozen C2 signature. A general crypto-scale
F_{p^{12}} NFS-DL is a principle-4 annotation, not a work item.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Discovered, not
assumed: no Makefile / justfile / xtask wrapper; raw `cargo` is the only CI surface (confirmed
unchanged from E.B; oracle KATs are `#[ignore]`-gated only, no `oracle-tests` feature). `/run-plan`
re-discovers at preflight. D.E **adds substantial code and a decisive end-to-end KAT**, so the gate
is a **correctness gate**: the D.E.3 end-to-end k=2 KAT (`solve_dl(g, h, p, k=2, ell)` returns `x`
with `g^x = h` in F_{47²}*) is the primary correctness signal — fast and decisive (lever 5). A wrong
residue embedding, a wrong extension factor base, or a descent bug breaks the round-trip directly.
**The frozen k=1 KATs (`gnfs/tests/dl_descent_kat.rs`) must stay green** — the gate also guards the
no-regression invariant on the unchanged k=1 path.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| D.E.1 | `F_{p^k}` target-embedding substrate (residue map, k>1 solver-target representation) | A | **Opus** | C2 `solve_dl` (frozen signature, read), C-Schirokauer (frozen, read), `shared::numfield` `NumberField`/residue (frozen, read), C-FpExt `FpExt` (frozen, read — for the residue-map cross-check) | `gnfs/src/dl/ext/mod.rs` (new: `pub mod target;`), `gnfs/src/dl/ext/target.rs` (new: extension-target type + residue map), `gnfs/src/dl/mod.rs` (add `pub mod ext;` + re-exports) |
| D.E.2 | Extension factor base + relation collection over the degree-k prime | A | Sonnet | C-ExtTarget (frozen D.E.1), C-Schirokauer (r>1, frozen read), C-LinAlgFl (index-agnostic, frozen read), `FactorBase` (frozen read) | `gnfs/src/dl/ext/factorbase.rs` (new: degree-k prime + extension factor base), `gnfs/src/dl/ext/relation.rs` (new: extension relation collection), `gnfs/src/dl/ext/mod.rs` (add `pub mod factorbase; pub mod relation;`) |
| D.E.3 ◆ `@plan` | k>1 individual-log descent + `solve_dl` k>1 wiring + end-to-end k=2 KAT | B | Sonnet | C-ExtTarget + C-ExtFactorBase (frozen D.E.1/2), C-Descent (frozen read), C2 signature (frozen) | `gnfs/src/dl/ext/descent.rs` (new: k>1 descent), `gnfs/src/dl/descent/solve.rs` (remove `Unsupported` early-return for k=2, wire k>1 path), `gnfs/tests/dl_ext_kat.rs` (new: end-to-end k=2 KAT + PARI `#[ignore]` cross-check) |

**Sequencing notes.** Strictly serial: **D.E.1 → D.E.2 → D.E.3.** D.E.1 lands the target
representation the whole sub-track stands on; D.E.2 builds the factor base + relations over it;
D.E.3 writes the descent, wires the k>1 `solve_dl` path, and closes the sub-track with the
end-to-end KAT. The single `@plan` marker sits on **D.E.3 ◆** — the Opus boundary juncture ratifying
C-ExtTarget / C-ExtFactorBase / C2-ext against E.C's needs before the sub-track closes. D.E.1,
though Opus-tier, carries **no** `@plan` (it freezes a compiler-/test-checkable target interface;
C-ExtTarget is re-ratified at the D.E.3 ◆ alongside the others — an inline juncture would double the
boundary cost on a 3-row shard).

**Why 3 sessions (top of the roadmap 3-4 band for Track-D sub-tracks).** The split is taken at the
target-embedding → factor-base → descent seams:
- **One-line-commit-title corollary.** "k>1 target-embedding substrate", "extension factor base +
  relation collection", "k>1 descent + solve_dl wiring + e2e KAT" are **three distinct commit
  titles** spanning two categories (A substrate ×2, B algorithm ×1).
- **Contract-sharp boundaries (legitimate, not LOC-driven).** D.E.1 **freezes** C-ExtTarget; D.E.2
  **consumes** it and **freezes** C-ExtFactorBase; D.E.3 **consumes** both and **freezes** C2-ext.
  Two real produce/consume seams.
- **Why the target-embedding is its own Opus session (lever 1 + lever 3).** The F_{p^k}-target
  representation and residue map is the deepest consumed surface (E.C reads pairing outputs through
  C-ExtTarget into `solve_dl`), and the FpExt→solver-target mismatch is an open design call the
  substrate survey flagged explicitly — ambient complexity (lever 1, ~3,500 LOC frozen substrate) and
  cost-of-wrong (lever 3, bounds E.C) are both high.
- **Irreducible units kept whole (lever 2).** The descent + the `solve_dl` k>1 wiring + the
  end-to-end KAT is one coherent unit (D.E.3) — the descent has **no standalone KAT-able contract
  except as the working `solve_dl`** (a descent with no solver deliverable has an undefined contract,
  same rule that kept Miller+Weil whole in E.B.3). Splitting it fractures an irreducible unit.

They are **not** further splittable: separating the k>1 descent from the `solve_dl` wiring splits
an irreducible unit (per the corollary); merging D.E.2 into D.E.1 risks a >400-LOC two-title Opus
session against high ambient complexity (lever 1).

---

## Session detail

D.E.1 is specified at near-full fidelity (the target representation and residue map are the design
crux, with one open Opus design call flagged). D.E.2–3 are lower-fidelity sketches, correct per the
substrate-first discipline: they are crisply specified only after C-ExtTarget and C-ExtFactorBase
freeze.

### D.E.1 — F_{p^k} target-embedding substrate (Opus, Cat A)

**Deliverable:** the representation of a DL target in `F_{p^k}*` for the NFS-DL solver, and the
residue map between the char-p extension and the NFS-DL number field. **The Opus design call** (the
reason this session is Opus and bounds E.C): the *solver-target type* the k>1 `solve_dl` path
consumes. The survey found `solve_dl` takes `h: &BigInt` (prime-field integer); an F_{p^k} target is
a coefficient vector `[c_0, …, c_{k−1}]` over F_p. The design choices:
- **The extension-target type** (`ext/target.rs`): the representation of `g, h ∈ F_{p^k}*` the k>1
  solver path reads. Candidates (the Opus call): (a) a coefficient `Vec<BigInt>` over the prime
  field; (b) a thin wrapper that the residue map produces from a number-field element; (c) accept
  E.B's `FpExt` directly (cleanest for E.C, but couples gnfs to rho's type — rejected unless the
  survey finds a clean way). Decision criteria: what E.C can produce from a pairing output (`FpExt`
  via `to_uint_vec` → `Vec<Uint<4>>` → `Vec<BigInt>`) and what the descent/smoothing needs to
  factor an extension target. Recorded as C-ExtTarget; re-ratified at the D.E.3 ◆ against E.C.
- **The residue map** F_{p^k} ↔ residue field of the degree-k prime ideal in K = ℚ[α]/(f). The map
  that takes an abstract F_{p^k} element (in coefficient form) to the NFS-DL representation and back
  — the bridge a pairing output crosses when E.C calls `solve_dl`. Cross-checked against E.B's
  `FpExt` arithmetic (read-only): the same field, two representations — assert they agree on a sample
  (a + b, a · b, a^p / Frobenius).
- **The g, h encoding** for k=2 at the `pairing_toy` parameters (p=47, modulus u²+1).

Consumes the frozen C2 `solve_dl` signature (read — the target must fit it or extend it
compatibly), C-Schirokauer (read), `shared::numfield` residue machinery (read), and C-FpExt `FpExt`
(read — for the residue-map cross-check; **no `rho` dependency added** — the cross-check is
conceptual, not a code import). **Freezes C-ExtTarget.**

**KAT:** residue-map round-trip (an `FpExt`-shaped coefficient vector → extension-target → back is
identity on a sample); residue-map homomorphism (the map respects +, ×, Frobenius — cross-checked
against E.B's `FpExt` arithmetic at the `pairing_toy` parameters); the encoding of a k=2 target is
well-formed. **Verify gate:** `cargo test --workspace` green; **k=1 KATs unchanged.**

**Subtlety (load-bearing):** (1) the number field stays **char-0** — the residue map is the only
place F_{p^k} (char p) meets the sieve algebra; do not let extension-field arithmetic leak into the
relation-collection coefficient field (the rigidity guard). (2) **Over-specify** the target type for
E.C: carry a constructor that accepts the `Vec<BigInt>` coefficient form `FpExt::to_uint_vec` already
produces, even if D.E's own KATs build targets by other means — E.C produces targets from pairing
outputs and adding the constructor later is costlier. (3) The degree-k prime ideal's residue field
must be **exactly F_{p^k}** (the prime must be inert/degree-k over p, not split) — a split prime
gives a smaller residue field and the lift is vacuous (the E.B minimal-embedding-degree trap
analogue). Assert the residue degree in the fixture.

**Deferred:** the extension factor base + relation collection (D.E.2); the k>1 descent + `solve_dl`
wiring (D.E.3); crypto-scale F_{p^{12}} NFS-DL (principle-4 annotation, not a work item).

### D.E.2 — Extension factor base + relation collection over the degree-k prime (Sonnet, Cat A)

**Deliverable:** the factor base and relation collection adapted to the extension target.
- **Extension factor base** (`ext/factorbase.rs`): the factor base augmented with the degree-k prime
  ideal (and its residue structure) so relations can be collected against an F_{p^k} target. Composes
  the frozen `FactorBase` (read) and C-ExtTarget.
- **Extension relation collection** (`ext/relation.rs`): adapt `augment_relation` /
  `init_descent_frontier` smoothing to an extension target — the Schirokauer ideals over ℓ in the
  extension setting (C-Schirokauer's r>1, already carried), the smoothing of `g^e · h` where h is an
  extension target expressed via the C-ExtTarget residue map.

Consumes C-ExtTarget (frozen D.E.1), C-Schirokauer (r>1, frozen read), C-LinAlgFl (index-agnostic,
frozen read), `FactorBase` (frozen read). **Freezes C-ExtFactorBase.**

**KAT:** the extension factor base contains the degree-k prime with the right residue degree; a
hand-built extension relation augments correctly (Schirokauer columns over ℓ, exercising r>1); the
F_ℓ matrix assembles via the unchanged index-agnostic `build_fl_matrix`. **Verify gate:** green;
k=1 KATs unchanged.

**Subtlety (load-bearing):** the F_ℓ linalg is **already extension-ready** (index-keyed) — D.E.2
must **not** re-implement it; reuse `build_fl_matrix` / `block_lanczos_fl` / `recover_virtual_logs`
unchanged. The Schirokauer map was over-specified for r>1 at D.A.1 — confirm the extension setting
exercises r>1 and that the existing map handles it cleanly; if it does not, that is an
**additive-reshard** (surfaced at the D.E.3 ◆), not a silent patch.

**Deferred:** the k>1 descent + `solve_dl` wiring + end-to-end KAT (D.E.3).

### D.E.3 ◆ — k>1 individual-log descent + solve_dl k>1 wiring + end-to-end k=2 KAT (Sonnet, Cat B, `@plan`)

**Deliverable:** the individual-log descent for an extension target, the k>1 `solve_dl` wiring, the
end-to-end k=2 KAT, and the sub-track close.
- **k>1 descent** (`ext/descent.rs`): the individual-log descent (special-q, composing the frozen
  C-Descent node/frontier types) adapted to an extension target — descend the extension target's
  factorization to factor-base leaves with known virtual logs from the C-ExtFactorBase setup.
- **`solve_dl` k>1 wiring** (`descent/solve.rs`): **remove the `if k != 1 { return Unsupported }`
  early-return** in `solve_dl` (line 436) and `solve_dl_full` (line 517); thread the k>1 path (D.E.1
  target + D.E.2 factor base + this descent) through the **frozen C2 signature**. The `Unsupported`
  variant **stays** (now returned only for k beyond the toy ceiling, e.g. k>2 if the substrate is
  k=2-scoped) — the taxonomy is unchanged. *(This is the only edit to a frozen file in this
  sub-track; the C2 signature and error variants are not touched.)*
- **End-to-end k=2 KAT** (`tests/dl_ext_kat.rs`): construct a k=2 instance at the `pairing_toy`
  parameters (p=47, k=2, ℓ=3, modulus u²+1) **independently in gnfs** — a hand-built F_{47²} DL
  target, *not* the rho fixture (the cross-crate seam: rho→gnfs dependency is E.C's, not D.E's);
  call `solve_dl(g, h, p, k=2, ell)`; assert the recovered `x` satisfies `g^x = h` in F_{47²}*.
  **PARI `znlog`/`fflog` `#[ignore]` cross-check** (the established dev-only oracle pattern,
  matching `gnfs/tests/dl_descent_kat.rs`).

Consumes C-ExtTarget + C-ExtFactorBase (frozen D.E.1/2), C-Descent (frozen read), the C2 signature
(frozen). **Freezes C2-ext** (the k>1 `solve_dl` contract E.C consumes).

**KAT (primary correctness signal):** **end-to-end** `solve_dl(g, h, 47, 2, 3)` returns `x` with
`g^x = h` in F_{47²}* for several (g, h) pairs; the k>1 path no longer returns `Unsupported` for
k=2; **all k=1 KATs stay green** (`gnfs/tests/dl_descent_kat.rs` — the no-regression gate). Optional
PARI cross-check. **Verify gate:** `cargo test --workspace` green.

**Subtlety (load-bearing):** (1) the descent is the subtle step — a wrong residue embedding or a
descent that loses a virtual log gives a `solve_dl` that returns a *wrong* x silently (hence the
end-to-end `g^x = h` round-trip is the KAT, not a spot value). (2) **The k=1 path must be untouched
in behaviour** — the wiring threads k>1 *alongside* k=1, it does not refactor the k=1 logic; the k=1
KATs green is a hard gate, not a nice-to-have. (3) This is the **D.E ◆ boundary** — re-read the
Purpose intent and verify the k>1 NFS-DL solver (target embedding, extension factor base, descent) is
coherent and that **C2-ext is genuinely E.C-ready** (the MOV bridge can hand a pairing output to
`solve_dl` at k=2 and get the right log) before crossing.

**`@plan` confirmation (post-landing, Opus, one-shot).** Page a `@plan-juncture` fork at the D.E.3 ◆
to confirm: (1) C-ExtTarget / C-ExtFactorBase / C2-ext are the right inputs for **E.C** (the MOV
bridge: does the extension-target representation accept what E.C produces from a pairing output —
`FpExt::to_uint_vec` → coefficient `Vec<BigInt>` → C-ExtTarget — and does `solve_dl` at k=2 return
the log E.C needs?); (2) the k=1 path is behaviourally unchanged (no-regression); (3) the descent +
end-to-end KAT recover correct logs with the round-trip KAT exercising `g^x = h`; (4) the frozen C2
signature and `SolveDlError` taxonomy are **unchanged** (the k>1 path lives behind them); (5) the
number field stayed char-0 (rigidity guard — `FpExt` did not leak into the sieve algebra). One-shot
findings; does not implement. Held at **Opus** per the header (lever-3 binding constraint — the
E.C-bounding freeze).

---

## Cross-session contracts

D.E **freezes three** contracts. Per the substrate-over-specify rule, C-ExtTarget carries the
`FpExt`-aligned constructor E.C will consume even where D.E's own KATs build targets directly. The
frozen k=1 C2 signature and `SolveDlError` taxonomy are **read** (the k>1 path fills in behind them),
not amended.

### C-ExtTarget — F_{p^k} solver-target representation + residue map (compiler- + test-enforced) — *to be frozen at D.E.1*

**Defined in:** D.E.1 (`gnfs/src/dl/ext/target.rs`). **Consumed by:** D.E.2 (factor base reads the
target structure), D.E.3 (the solver path), and **E.C** (the MOV bridge produces a pairing output and
encodes it as a C-ExtTarget for `solve_dl`). Compiler-enforced (the target type + residue-map
signatures) + test-enforced (round-trip + homomorphism). Exposes the extension-target type, the
residue map F_{p^k} ↔ degree-k residue field, and (over-specify) a constructor that accepts the
coefficient-`Vec<BigInt>` form `FpExt::to_uint_vec` already produces. *Exact type name and target
representation (coeff `Vec<BigInt>` vs wrapper vs other) ratified at D.E.1 and re-ratified at the
D.E.3 ◆ against E.C.* *Over-specify note:* the `Vec<BigInt>`-accepting constructor is carried now
though D.E's own KATs may build targets by other means — E.C needs it centrally.

### C-ExtFactorBase — extension factor base + relation collection (test-enforced) — *to be frozen at D.E.2*

**Defined in:** D.E.2 (`gnfs/src/dl/ext/{factorbase,relation}.rs`). **Consumed by:** D.E.3 (the
descent + solver) and **E.C** (indirectly, via `solve_dl`). Test-enforced: the factor base carries
the degree-k prime with the right residue degree; extension relations augment with r-coordinate
Schirokauer columns; the F_ℓ matrix assembles via the unchanged index-agnostic `build_fl_matrix`.
Exposes the extension factor base constructor and the extension relation-collection entry. *Exact
names ratified at D.E.2.* **The degree-k prime's residue field is exactly F_{p^k}** (inert/degree-k,
not split).

### C2-ext — k>1 `solve_dl` (compiler- + test-enforced) — *to be frozen at D.E.3 ◆*

**Defined in:** D.E.3 (`gnfs/src/dl/descent/solve.rs` — edit; descent in `ext/descent.rs` — new).
**Consumed by:** D.E's own end-to-end KAT now; **E.C** (the MOV/Frey–Rück reduction — the named,
climactic consumer). Compiler- + test-enforced. **The C2 signature is unchanged** —
`solve_dl(g, h, p, k, ell) -> Result<BigInt, SolveDlError>`; what changes is that **k=2 now returns
a real `Ok(x)`** instead of `Unsupported`. The exact parameter shape (whether k>1 uses the existing
`g, h: &BigInt` with an agreed encoding, or `solve_dl` gains a separate extension overload) is
ratified at the D.E.1 design call and finalized here. *Frozen at the ◆ juncture (signature/target
ratified against the E.C MOV-bridge fit before crossing — the lever-3 reason the juncture is Opus).*
**The `SolveDlError` taxonomy is unchanged (frozen D.C.3); `Unsupported` now fires only for k beyond
the toy ceiling.**

### Frozen contracts read by D.E (composed, not amended)

D.E composes these; none is touched.
- **C2 `solve_dl` signature + `SolveDlError` taxonomy** (`gnfs/src/dl/descent/solve.rs`, frozen
  D.C.3) — the k>1 path fills in behind the frozen signature; the early-return is removed but the
  signature and error variants are unchanged.
- **C-Schirokauer** (`gnfs/src/dl/schirokauer.rs`, frozen D.A.1) — the r>1 multi-coordinate shape,
  over-specified at D.A.1 for exactly this consumer; read, not amended.
- **C-LinAlgFl** (`gnfs/src/dl/linalg/`, frozen D.B.1/2) — `build_fl_matrix` / `block_lanczos_fl` /
  `recover_virtual_logs`, index-keyed and already extension-ready; reused unchanged.
- **C-Descent** (`gnfs/src/dl/descent/`, frozen D.C.1) — the node/frontier types; the k>1 descent
  composes them.
- **C-FpExt** (`rho/src/pairing/fpext.rs`, frozen E.B.1) — read at D.E.1 *conceptually only* for the
  residue-map cross-check (same field, two representations). D.E adds **no `rho` dependency**; the
  cross-check is a test-time arithmetic check using the same parameters (p=47, k=2, u²+1), not a code
  import. The rho→gnfs dependency is E.C's.
- **`shared::numfield`** `NumberField` / residue machinery — the char-0 number field stays the sieve
  algebra; D.E reads its residue structure for the degree-k prime.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The D.E.3 ◆ `@plan` confirmation is not a separate
ledger row (a paged fork with no commit-shaped deliverable); its outcome is recorded in the
Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| D.E.1 | `F_{p^k}` target-embedding substrate | pending | — | C-ExtTarget (pending) |
| D.E.2 | Extension factor base + relation collection | pending | — | C-ExtFactorBase (pending) |
| D.E.3 ◆ | k>1 descent + solve_dl wiring + end-to-end k=2 KAT | pending | — | C2-ext (pending) |

Contracts frozen before this sub-track (read by D.E): all Track-D NFS-DL contracts (C2,
C-Schirokauer, C-LinAlgFl, C-Descent — existing, frozen D.A–D.C); E.B's C-FpExt / C-PairingCurve /
C-Pairing (Track-E, read only conceptually for the residue-map cross-check). This sub-track
**freezes three new contracts** (C-ExtTarget, C-ExtFactorBase, C2-ext), all forward-looking toward
**E.C** (the MOV bridge).

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **The k=1 NFS-DL pipeline mostly transfers — D.E is a bounded lift, not a rewrite (substrate
  finding, confirmed by survey).** The F_ℓ linalg is index-agnostic; Schirokauer carries r>1; the
  DLRelation wrapper composes. Writing the lift is **internal-continue**. A discovery that the
  Schirokauer map does *not* handle the extension r>1 setting cleanly is an **additive-reshard**
  surfaced at the D.E.3 ◆.

- **The C2 signature and `SolveDlError` taxonomy are FROZEN — the k>1 path fills in behind them
  (contract guard).** A `@build` agent that **adds a fourth `SolveDlError` variant** or **changes
  the `solve_dl` signature** is a **destructive-HALT** — the taxonomy is frozen (D.C.3, "no further
  variants will be added"), consumed by E.C. The k>1 path uses the existing three variants and the
  existing signature; the only edit to `solve.rs` is removing the `Unsupported` early-return for k=2
  and threading the real path. Surface any felt need to change the surface as a juncture discovery.

- **The number field stays char-0 — `FpExt` is the target representation, not the sieve algebra
  (rigidity guard).** Reaching for E.B's char-p `FpExt` as the relation-collection coefficient field
  is **internal-continue → corrected** (the sieve algebra is char-0 ℚ[α]/(f); `FpExt` meets it only
  at the residue map in D.E.1). Do not let `FpExt` or extension-field arithmetic leak into the factor
  base / relation algebra.

- **The k=1 path must stay behaviourally unchanged — no-regression invariant (hard gate).** A wiring
  that refactors the k=1 logic (rather than threading k>1 alongside) risks breaking the frozen k=1
  KATs. A k=1 regression is a **destructive-HALT**: stop, do not proceed to D.E.3 ◆, surface it.

- **The degree-k prime's residue field must be exactly F_{p^k} — minimality trap.** If the prime
  splits (residue field smaller than F_{p^k}) the lift is vacuous (the E.B minimal-embedding-degree
  trap analogue). A non-degree-k prime is **internal-continue** (fix the prime choice in D.E.1), not
  a contract break. D.E.1 must assert the residue degree in the fixture.

- **C2-ext is a forward-looking contract bounding E.C (over-specify discipline).** The
  extension-target representation is frozen with the MOV bridge (E.C) in mind — E.C produces targets
  from pairing outputs (`FpExt::to_uint_vec`). If D.E.3 finds the surface underserves E.C,
  **widening** it at the ◆ is additive; a consumer-driven *change* after freeze is an
  **additive-reshard** at the E.C inflection. This is the lever-3 reason the ◆ juncture is Opus.

- **D.E is the NFS-DL *extension*, not the MOV attack — it must not presume E.C (defocus guard).**
  No pairing call, no curve, no MOV reduction belongs in D.E. The end-to-end KAT builds its F_{47²}
  target **independently in gnfs** (not via the rho pairing fixture — the rho→gnfs dependency is
  E.C's). Writing toward the MOV bridge here is **defocus** — internal-continue only within the
  k>1-NFS-DL scope.

- **ROADMAP insertion owed: D.E is a new Track-D sub-track not yet in the roadmap (static-frame
  debt — capture candidate).** The roadmap's E.C entry (line 300) assumes the k>1 solver folded into
  E.C; the D.W closeout deferred it to "an E.C-prep ROADMAP-then-shard session." This shard
  formalises it as **D.E**. A one-line ROADMAP insertion (inserting D.E between Track D and E.C, and
  reconciling the stale Progress table that shows D/E "not started") is an **inflection-point Opus
  action** owed at the D.E ◆ boundary — not by a D.E `@build` session.

- **No oracle dependency for correctness (principle-3 / D.B-consistent).** The end-to-end `g^x = h`
  round-trip self-checks; a PARI `znlog`/`fflog` cross-check is an **optional `#[ignore]` sidecar**
  (the established pattern — no `oracle-tests` feature, `#[ignore]` gate only). D.E introduces no
  new live oracle.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase γ — Contract C2 `solve_dl`, and the D.W closeout Discoveries entry
  deferring the F_{p^k} extension to "an E.C-prep ROADMAP-then-shard session"; Phase δ — E.C, the
  MOV bridge that consumes C2-ext) and this PLAN before any session.
- **Note (non-blocking): the ROADMAP Progress table is stale** — it shows Track D and Track E "not
  started / 0 done" (reconciled at the T.G ◆ boundary, *before* Track D and E.A/E.B landed). Track
  D is complete (D.A→D.W, `8c92260`…`541be29`); E.A and E.B are done (E.A: `054df65` + `51fd477`;
  E.B: `74b2ff3`…`6c082bb`). The Discoveries log and the prior PLAN ledgers are authoritative. The
  roadmap is rewritten only at sub-track boundaries; the D.E insertion + progress reconciliation is
  an inflection-point Opus action at the D.E ◆, not by D.E `@build`.
- Read the **templates to mirror**: `gnfs/src/dl/relation.rs` (the `augment_relation` /
  `collect_dl_relations` idiom D.E.2 parallels); `gnfs/src/dl/descent/solve.rs` (the `solve_dl` /
  `solve_dl_full` structure D.E.3 threads the k>1 path through — the early-returns at lines 436 and
  517 are the edit sites); `gnfs/src/dl/schirokauer.rs` (the r>1-carrying map); `gnfs/tests/
  dl_descent_kat.rs` (the KAT idiom — esp. KAT (h) end-to-end shape, the template for the D.E.3
  end-to-end k=2 KAT); `rho/src/pairing/fpext.rs` (`FpExt` + `to_uint_vec`, read conceptually at
  D.E.1 for the residue-map cross-check only — no rho dependency added by D.E).
- **Register:** D.E is **Rust code** (`STYLE-CODE.md` → `STYLE-CODE-RUST.md`; 100-char wrap, rustdoc
  thin-by-default). New module tree `gnfs/src/dl/ext/{mod,target,factorbase,relation,descent}.rs`.
  KATs in `gnfs/tests/dl_ext_kat.rs`. One edit to an existing frozen file: `gnfs/src/dl/descent/
  solve.rs` (remove the k>1 `Unsupported` early-return, thread the k>1 path — signature unchanged).
- **Tier routing:** **D.E.1 is Opus** (`@build` on Opus) — the target-embedding substrate that
  bounds E.C. **D.E.2–3 are Sonnet** (mechanical against the frozen target/factor-base
  representation). D.E.3 carries the single `@plan` marker: a ◆-boundary juncture (page
  `@plan-juncture`) ratifying C-ExtTarget / C-ExtFactorBase / C2-ext against E.C before the
  sub-track closes. juncture-tier (header) is **opus** — held by lever 3 (the E.C-bounding freeze),
  the binding constraint; the strong lever-5 end-to-end round-trip KAT would license an opt-down in
  isolation but does not override lever 3.
- **Invariants to preserve:** **the frozen C2 signature + `SolveDlError` taxonomy are unchanged** —
  D.E fills in the k>1 path behind them (the only `solve.rs` edit is removing the two early-returns).
  **The k=1 path is behaviourally unchanged** (no-regression; k=1 KATs green is a hard gate). **The
  number field stays char-0** — `FpExt` is the target representation, not the sieve algebra (rigidity
  guard). The F_ℓ linalg is reused unchanged (index-agnostic). The extension is **toy k=2-scoped**;
  crypto-scale F_{p^{12}} NFS-DL is a documented principle-4 boundary, not wired. No new live oracle
  (round-trip self-checks).
- **PARI remains a dev-only `#[ignore]` oracle** (Discoveries-log policy) — a k>1 DL cross-check
  (`znlog`/`fflog`) follows the established `#[test] #[ignore = "PARI not installed…"]` pattern; it
  is optional, never on the green path.
- **The cross-crate seam (load-bearing for D.E.3).** `solve_dl` lives in `gnfs`; the `pairing_toy`
  fixture lives in `rho`. `rho` will depend on `gnfs` **at E.C, not D.E** — D.E adds no rho
  dependency. D.E.3's end-to-end KAT therefore builds its F_{47²} target **independently in gnfs**
  (hand-built, matching p=47/k=2/ℓ=3/u²+1). The full pairing-output → `solve_dl` round-trip KAT is
  **E.C's in rho**, not D.E's.
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — D.E is the
  highest-stakes interface remaining before the cross-track climax; the conservative halt-at-◆
  cadence is warranted. The single ◆/`@plan` on D.E.3 is the one that matters (the Opus C2-ext
  ratification + E.C-readiness check). *(Tradeoff vs default cadence: one extra halt-confirm on a
  3-session sub-track is cheap insurance on an E.C-bounding shard.)*
