<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Track-D continues (D.C — individual logarithm + special-q descent)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default; does not opt down**, on a joint
**lever-3 + lever-4** call (recorded here; contrast D.B, which held on lever 4 *alone*). Applying the
five-lever law to D.C: lever 3 (design-error cost) is **back up** — D.C freezes **C2, the cross-track
NFS-DL solver interface** (`solve_dl`), consumed by **E.C** (the MOV bridge, the project's
pedagogical climax). A wrong C2 shape propagates into a different track and is the most expensive
freeze to get wrong in Track D — the same cross-track weight D.A.1 carried, which D.B explicitly did
not. Lever 4 (correctness-criticality) is **also high** — an individual logarithm that returns a
plausible-but-wrong value (the silent failure mode) is the worst outcome, and the descent's
correctness rests on the virtual-log table being combined correctly. Lever 2 (irreducible
complexity) is **high** — special-q descent is the part of NFS-DL with *no factoring analogue* and is
"mathematically delicate" (ROADMAP); it is the FLOOR that holds D.C.2 whole. Lever 5 (inner-loop
bandwidth) is **strong** — mature `cargo test --workspace` gate, the D.B end-to-end DL KAT to extend,
a PARI discrete-log oracle, and the special-q sieve machinery (`special_q_sieve`, `lattice_sieve`)
already KAT-verified in Track G. Levers 3+4 jointly **hold the Opus register** at the D.C.1
C-Descent+C2-shape freeze and the D.C.3 ◆ C2 final freeze; the strong inner loop (lever 5) does
**not** license opting down when a cross-track interface freezes. *(Contrast D.B: lever-4-only hold,
lever 3 relaxed because C-LinAlgFl was sub-track-internal. Here lever 3 is restored by C2's
cross-track reach.)*

Last rewrite: D.B.2 ◆ boundary crossed (Track-D linear-algebra arc D.B.1 → D.B.2 coherent; F_ℓ
block Lanczos/Wiedemann + virtual-log recovery landed, ledger reconciled 2026-06-08, commits
652cfa6 / a569049). This plan opens **Track D's third sub-track, D.C — individual logarithm +
special-q descent**: take the virtual-log table D.B.2 recovered (factor-base element → log mod ℓ)
and compute the discrete logarithm of an *arbitrary* target `h`, by smoothing `h` over progressively
smaller primes (special-q descent) until it reduces to factor-base elements with known logs — then
freeze the cross-track **C2 `solve_dl`** interface E.C consumes.

---

## Purpose (design intent)

Per ROADMAP: D.C is "Individual logarithm + special-q descent. The part with no factoring analogue.
Special-q descent is mathematically delicate. First session is Opus-tier." It is the step that turns
NFS-DL from "logs of the factor-base elements" (D.B's `VirtualLogTable`) into "log of *any* target".
The mathematics has three stages, which are the three sessions:

1. **Descent substrate + target initialization-smoothing + the C2 interface shape (D.C.1, Opus).**
   The data structures of the descent (a descent-tree node; the frontier of medium primes not yet in
   the factor base), the **target initialization step** (given `h ∈ F_p*`, randomized search for an
   exponent `e` such that the number-field lift of `g^e·h` is *B'-smooth* into medium primes — the
   first descent step, reusing the `trial_smooth` / `Relation::new` factor-over-the-factor-base
   pattern), and the **C2 `solve_dl` interface shape**: signature `solve_dl(g, h, p, k) -> integer`
   + the error type frozen, the **k = 1 (prime-field F_p) path live**, k > 1 returning a clean
   `Unsupported` error. **Freezes C-Descent** (the internal descent substrate, sub-track-internal)
   and **opens C2** (the cross-track public interface — shape frozen, error taxonomy settled at
   D.C.3). This is the substrate session whose interface binds D.C.2 and D.C.3. Over-specify the
   descent-node and C2 shape deliberately within reason.

2. **Special-q descent recursion (D.C.2, Sonnet, algorithm).** The recursive heart: each medium
   prime `q` on the frontier (not in the factor base) is the root of a **special-q lattice sieve**
   (reusing Track-G `special_q_sieve` / `lattice_sieve`) that produces a relation rewriting `log q`
   as a combination of *smaller* primes' logs; recurse down the descent tree until every leaf is a
   factor-base element with a known virtual log. Consumes C-Descent. The delicacy (lever 2): descent
   must **terminate** (each step strictly reduces the largest prime), handle the **degree-of-freedom
   / relation-selection** at each node, and not loop. Stays at demonstration fidelity (principle 2 —
   the descent-tree breadth is an NFS-scale phenomenon, annotated, not engineered).

3. **Assembly + C2 final freeze + end-to-end individual-log KAT (D.C.3 ◆, Sonnet, integrative).**
   Combine the virtual logs along the descent tree to recover `log_g(h)` mod ℓ; handle subgroup
   recovery (if ℓ is a proper factor of `p−1`, the Pohlig–Hellman / CRT lift to the full order is
   in-scope at demonstration fidelity, or annotated as deferred — decided at the boundary). **Freeze
   C2** (the full error taxonomy now that descent reality is known). The **end-to-end toy-F_p
   individual-log KAT**: recover a *known* discrete log of an arbitrary `h` through the full path
   (relation collection → F_ℓ solve → virtual-log table → initialization → descent → assembly),
   cross-checked against a hand-computed reference and (stub-gated) PARI. This session crosses the
   **D.C ◆ boundary** (Track-D's algorithmic content complete; D.W writeup remains).

Re-read this intent at the ◆ boundary to catch **defocus** (building the **F_{p^k} extension-field**
NFS-DL solver here — that is genuine new mathematics D.B's F_p pipeline does not support; C2's k > 1
path is a deliberately-deferred later ROADMAP-then-shard session, see Discoveries; and **building
E.C / the MOV bridge** — that is Track E) and **rigidity** (forcing the descent to terminate by an
artificial bound when a relation-selection fix is the real issue; or freezing C2's error taxonomy at
D.C.1 before the descent in D.C.2 reveals the real failure modes — the shape freezes at D.C.1, the
error taxonomy at D.C.3).

**Scoping discipline (ROADMAP three-way split, applied here).** Special-q descent and individual
logarithm are **algorithmic content included in full** (principle 1) — the descent recursion, the
initialization-smoothing search, and the log-assembly are implemented head-on; this is the
no-factoring-analogue core of NFS-DL. The descent stays at **demonstration fidelity** (principle 2):
the descent-tree breadth and the medium-prime-bound tuning are NFS-scale phenomena, annotated, not
engineered. No engineering optimisations (principle 3). PARI remains a dev-only oracle (D.C.3
cross-check, stub-gated `#[ignore]`), never on a build path. **C2 is scoped to the prime field F_p
(k = 1) now** — the F_{p^k} extension is genuine new mathematics deferred to an E.C-prep session
(Discoveries); C2's *signature* is frozen in its full F_{p^k} shape so E.C's call site is stable,
but only the k = 1 path is live. C1 `Uint<4>` stays as-is per the ROADMAP width policy (D.C touches
factor-base indices, F_ℓ logs, and special-q ideals — not the smoothness width).

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace`. Confirmed by the
D.B survey and unchanged: no Makefile / justfile / xtask wrapper exists in the workspace; raw
`cargo` is the only CI surface. Rust's compiler is the type gate; `cargo test` subsumes it on a
clean build, so one green `cargo test --workspace` satisfies both. D.C is **code** — the gate is a
real inner loop (lever 5, strong), which is why the juncture-tier hold at Opus rests on levers 3+4,
not on a weak inner loop. `/run-plan` re-discovers these at preflight.

---

## Session list

One commit-shaped session per row. `Cat` = category (A substrate / B algorithm / C optimization /
I integrative). `◆` marks a sub-track-final session. `@plan` marks an inflection or contract-freeze
point requiring a juncture fork + human sign-off before the next session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| D.C.1 `@plan` | Descent substrate + target initialization-smoothing + C2 `solve_dl` interface shape (F_p path live, F_{p^k} stubbed); freeze C-Descent, open C2 | A | **Opus** | C-LinAlgFl (VirtualLogTable), C-DLRelation (DLMatrix, collect_dl_relations), C-Schirokauer, C-FactorBase, C1 (trial_smooth), Fp (`shared-field`) | `gnfs/src/dl/descent/mod.rs` (new), `gnfs/src/dl/descent/node.rs` (new), `gnfs/src/dl/descent/solve.rs` (new: C2 `solve_dl` + init-smoothing), `gnfs/src/dl/mod.rs` (re-export), `gnfs/tests/dl_descent_kat.rs` (new) |
| D.C.2 | Special-q descent recursion: rewrite each medium prime via reused special_q_sieve, recurse to factor-base leaves | B | Sonnet | C-Descent, C-FactorBase, special_q_sieve / lattice_sieve (read), C-LinAlgFl (VirtualLogTable) | `gnfs/src/dl/descent/recurse.rs` (new), `gnfs/src/dl/descent/mod.rs` (extend), `gnfs/tests/dl_descent_kat.rs` (extend) |
| D.C.3 ◆ | Individual-log assembly + subgroup recovery + C2 final freeze + end-to-end toy-F_p DL KAT (PARI cross-check) | I | Sonnet | C-Descent, C2 (finalize), C-LinAlgFl, C-DLRelation, (PARI oracle) | `gnfs/src/dl/descent/solve.rs` (extend: assembly, C2 freeze), `gnfs/tests/dl_descent_kat.rs` (extend), `gnfs/tests/dl_individual_log_kat.rs` (new) |

**Sequencing notes.** Strictly serial: **D.C.1 → D.C.2 → D.C.3.** D.C.2's recursion consumes the
C-Descent node/frontier types and the C2 init-smoothing entry that D.C.1 froze; D.C.3's assembly
reads the descent tree D.C.2 produces and finalizes C2. The single `@plan` marker sits on **D.C.1** —
a post-landing freeze confirmation for **C-Descent + the C2 shape**, weightier than D.B.1's
(C2 is cross-track, consumed by E.C); confirm the descent substrate, the C2 signature/error shape,
and the initialization-smoothing correctness before D.C.2 is dispatched. **D.C.3 ◆** is the D.C
sub-track boundary *and* the C2 final freeze (the second halt under `@plan`-marker cadence).

**Why 3 sessions (matches ROADMAP allotment).** The one-line-commit-title corollary: D.C.1 ("descent
substrate + init-smoothing + C2 shape"), D.C.2 ("special-q descent recursion"), D.C.3 ("individual-log
assembly + C2 freeze + end-to-end KAT") are three distinct commit titles, split on **contract-sharp
boundaries** (D.C.1 freezes C-Descent and opens C2; D.C.2 consumes C-Descent; D.C.3 finalizes C2).
The split follows the **three mathematical stages of NFS-DL individual logarithm** (initialization /
descent / assembly). They are **not** mergeable — D.C.1 freezes the descent substrate D.C.2 consumes,
and the C2 error taxonomy genuinely settles only after D.C.2's descent runs (so the shape-open /
taxonomy-freeze split across D.C.1 → D.C.3 is load-bearing, not cosmetic). They are **not** further
splittable below the floor — special-q descent (D.C.2) is the irreducible "mathematically delicate"
unit (lever 2); fracturing it would split the recursion at a non-contract-sharp boundary just to hit
a LOC number (forbidden). The **initialization-smoothing step stays in D.C.1** (not merged into
D.C.2): it is the substrate's first exercise of the descent frontier and the natural KAT vehicle for
the C-Descent node type — merging it into D.C.2 would fracture "rewrite the target over smaller
primes" across the substrate/algorithm boundary.

---

## Session detail

D.C.1 is crisp (its design surface is the C-Descent freeze + the C2 interface shape, resolved
in-session as the substrate's own work). D.C.2 and D.C.3 are sketched at post-substrate fidelity —
correct to leave their precise shape open until D.C.1 freezes the descent-node types and the C2
shape.

### D.C.1 — Descent substrate + target initialization + C2 interface shape (Opus, substrate, `@plan`)

**Deliverable:** the descent data structures, the target initialization-smoothing step, and the
cross-track C2 interface shape.
- **Descent-tree substrate.** A `DescentNode` (the prime/ideal being descended, its provenance, the
  relation that rewrote it, child frontier) and the descent frontier (the set of medium primes not
  yet in the factor base, awaiting recursion). New module `gnfs/src/dl/descent/`. The
  `gnfs/src/dl/mod.rs:85` anticipatory hook ("reusing line_sieve / special_q_sieve") names this seam.
- **Target initialization-smoothing.** Given a target `h ∈ F_p*` and generator `g`: randomized
  search over exponents `e` for one where the number-field lift of `g^e·h` is *B'-smooth* (smooth
  into medium primes, B' > factor-base bound) — the first descent step. Reuses the `trial_smooth`
  (`shared::numth`) / `Relation::new` (`gnfs/src/sieve/mod.rs:259`) factor-over-the-factor-base
  pattern. Produces the initial descent frontier (the medium primes of `g^e·h`).
- **C2 `solve_dl` interface shape.** Freeze the signature `solve_dl(g, h, p, k) -> Result<BigInt,
  SolveDlError>` (E.C's call site) and the `SolveDlError` type, with the **k = 1 (prime-field)
  path live** and **k > 1 returning `SolveDlError::Unsupported`** (the F_{p^k} extension deferred —
  Discoveries). At D.C.1 the body wires init-smoothing → (descent stub) → (assembly stub); the
  descent and assembly fill in at D.C.2 / D.C.3.
- **Freeze C-Descent** (the descent substrate interface) and **open C2** (shape frozen, taxonomy at
  D.C.3) — see Cross-session contracts.

**Key design decisions (the C-Descent + C2-shape freeze surface — the `@plan` confirmation):**
1. **DescentNode shape & frontier representation:** what a node carries (the descended prime/ideal,
   the rewriting relation, the child primes, the known-log flag once a leaf is a factor-base
   element); how the frontier is ordered (largest-prime-first, to guarantee the descent strictly
   reduces — the termination argument). Over-specify lightly: carry what D.C.2's recursion and
   D.C.3's assembly will read.
2. **C2 signature & error taxonomy (the cross-track surface):** `solve_dl(g, h, p, k)` — element
   representation for `g`,`h` (prime-field: `Uint<L>` / `BigInt` mod p; the F_{p^k} element type is
   the deferred shape), the `ell` / subgroup-order threading, and `SolveDlError` variants
   (`Unsupported` for k > 1; the descent-failure / size variants are *opened* here, *finalized* at
   D.C.3 once descent reveals them). This is the freeze E.C consumes — design it for stability.
3. **Initialization-smoothing parameters:** the medium-prime bound B', the exponent-search strategy
   and bound, the `threshold_scale` reuse (the G.C toy-scale log-sieve calibration discovery applies).
4. **k = 1 / k > 1 boundary confirmation:** confirm the prime-field path is complete and the k > 1
   `Unsupported` stub is clean (no panic, no silent wrong answer) — the deferred-F_{p^k} decision
   held in implementation.

**KAT (≥1 required, in `gnfs/tests/dl_descent_kat.rs`):** (a) DescentNode construction / frontier
ordering KAT (largest-prime-first invariant); (b) **initialization-smoothing KAT** — for a toy `h`,
the search finds an `e` with `g^e·h` B'-smooth, and the recovered frontier primes are correct
(hand-checked); (c) C2-shape KAT — `solve_dl` with k > 1 returns `SolveDlError::Unsupported`; the
k = 1 path is wired (may return a partial/stub result pending D.C.2/D.C.3, gated by a KAT that
asserts the *shape*, not yet the full answer). `cargo test --workspace` green.

**Subtlety:** the load-bearing judgments are the **C2 cross-track shape** (a wrong signature/error
taxonomy propagates into E.C — the Opus lever-3 case) and the **descent termination invariant**
(the frontier ordering that guarantees each descent step strictly reduces the largest prime — set up
here, exercised in D.C.2). The initialization-smoothing reuses verified machinery, so its risk is
parameter calibration (B', threshold_scale), not new algebra.

**Deferred:** the special-q descent recursion (D.C.2); log-assembly + subgroup recovery + C2 final
freeze (D.C.3); the **F_{p^k} extension-field NFS-DL** (k > 1 — its own E.C-prep ROADMAP-then-shard
session, Discoveries); E.C / the MOV bridge (Track E).

**`@plan` confirmation (post-landing, T0/Opus, one-shot).** Page a `@plan-juncture` fork to confirm
the **C-Descent + C2-shape freeze** before D.C.2 is dispatched: (1) the descent substrate is complete
and consistent (DescentNode, frontier, ordering invariant); (2) the C2 signature + error taxonomy is
stable and E.C-consumable, with k > 1 cleanly `Unsupported` and the F_{p^k} deferral recorded; (3)
initialization-smoothing recovers a correct frontier for a known toy `h` (KAT-confirmed); (4) the
k = 1 / k > 1 boundary held in implementation. One-shot findings; does not implement. Held at **Opus**
on levers 3+4 (cross-track C2 + correctness), per the header.

### D.C.2 — Special-q descent recursion (Sonnet, algorithm, sketch)

**Deliverable:** the recursive descent that drives every frontier prime down to factor-base leaves.
Sketch (crisp shape resolved once D.C.1's C-Descent + C2-shape freeze):
- **Per-node special-q descent.** For each medium prime `q` on the frontier: run a **special-q
  lattice sieve** rooted at `q` (reuse `special_q_sieve` / `lattice_sieve`,
  `gnfs/src/sieve/special_q.rs` / `lattice.rs`) to find a relation in which `q` appears alongside
  *strictly smaller* primes; that relation rewrites `log q` as a combination of the smaller primes'
  logs. Add the smaller non-factor-base primes as child nodes; recurse.
- **Termination.** Each step strictly reduces the largest prime in the frontier (the D.C.1 ordering
  invariant); recursion bottoms out when every leaf is a factor-base element (`fb.rational_index` /
  `fb.algebraic_index` hits) with a known `VirtualLogTable` entry. Guard against non-termination
  (a node that fails to descend after a bounded search → `SolveDlError` surfaced, not a loop).
- Stays at demonstration fidelity (principle-2 annotation: descent-tree breadth and medium-prime
  bound tuning are NFS-scale).

Consumes C-Descent (node/frontier types + the descent entry), C-FactorBase (leaf detection),
C-LinAlgFl (`VirtualLogTable` for leaf logs), and the Track-G special-q sieve (read-only reuse).
Freezes nothing new (it is the algorithm session consuming D.C.1's substrate).

**KAT (≥1 required):** (a) **single-node descent KAT** — one medium prime `q` descends to a relation
over smaller primes (hand-checked); (b) **multi-level descent KAT** — a frontier prime descends
through ≥2 levels to factor-base leaves; (c) **termination KAT** — a deliberately-undescendable input
surfaces a `SolveDlError` rather than looping. `cargo test --workspace` green.

**Subtlety:** the delicacy (lever 2) is **descent termination + relation selection** (which sieve
relation to pick at each node so the descent strictly reduces and does not re-introduce a prime
already descended) and the **special-q reuse seam** (the Track-G sieve was built for relation
*collection*, not *descent of a specific q* — confirm it can be driven with a fixed target `q` rather
than scanning a `[q_min, q_max]` range; if it cannot without modification, that is a **contract
discovery** on the sieve surface, internal-continue if additive, surfaced at the ◆ boundary).

### D.C.3 ◆ — Individual-log assembly + C2 freeze + end-to-end KAT (Sonnet, integrative, sketch)

**Deliverable:** combine the descent tree into `log_g(h)`, finalize C2, close the D.C arc. Sketch:
- **Log assembly.** Walk the descent tree from leaves (known virtual logs) up to the root,
  accumulating `log q = Σ (exponent · log child)` mod ℓ at each node, until `log_g(g^e·h)` is known;
  then `log_g(h) = log_g(g^e·h) − e` mod ℓ (back out the initialization exponent).
- **Subgroup recovery.** If ℓ is a proper prime factor of the group order (`p−1`), the recovered log
  is mod ℓ only; the full `log_g(h)` mod (p−1) requires Pohlig–Hellman / CRT across the order's
  factors. **In-scope at demonstration fidelity** (the toy KAT picks ℓ = p−1 or a single prime
  factor to keep this clean) **or annotated as deferred** — decided at the boundary. Surface the call.
- **C2 final freeze.** Finalize `SolveDlError` (the descent-failure / size variants now known from
  D.C.2) and confirm `solve_dl` end-to-end for k = 1. C2 is now the frozen cross-track interface E.C
  will consume.
- **End-to-end toy-F_p individual-log KAT** (◆ vehicle). Recover a *known* `log_g(h)` for an
  arbitrary toy `h` through the full path: relation collection (D.A) → F_ℓ solve (D.B) →
  `VirtualLogTable` → initialization (D.C.1) → descent (D.C.2) → assembly, cross-checked against a
  hand-computed reference and (stub-gated) PARI.

Consumes C-Descent, C-LinAlgFl, C-DLRelation, (PARI oracle). **Freezes C2.** Note: D.C delivers C2
for the **prime field (k = 1)**; the F_{p^k} extension path is deferred (Discoveries) — C2's k > 1
returns `Unsupported`, a known debt E.C-prep resolves.

**KAT (≥1 required):** (a) **assembly KAT** — a small hand-built descent tree assembles to the
correct `log_g(h)` mod ℓ; (b) **end-to-end individual-log KAT** — recover a known toy discrete log
end-to-end (the ◆ vehicle), hand-checked; (c) PARI cross-check —
`#[ignore = "PARI not installed; run manually when available"]` stub (matching the established
`kat_h_pari_oracle` / `kat_pari_dl_oracle` pattern; no feature flag, no subprocess in CI). The
deterministic non-PARI KATs carry the reproducibility burden. `cargo test --workspace` green.

**Subtlety:** the load-bearing judgments are the **log-assembly correctness** (sign/exponent
bookkeeping along the descent tree — a sign error gives a plausible-but-wrong log, the silent
lever-4 failure) and the **subgroup-recovery scope call** (whether toy ℓ = p−1 sidesteps
Pohlig–Hellman or it is implemented at demonstration fidelity). If assembly reveals the C-Descent
node shape can't carry what C2 needs, that is a **contract discovery** (additive-reshard) surfaced
at the ◆ boundary. This is the **D.C ◆ boundary** — re-read the Purpose intent and verify the D.C
arc (D.C.1 → D.C.2 → D.C.3) is coherent and that Track-D's algorithmic content is complete (D.W
writeup remains) before crossing toward D.W.

---

## Cross-session contracts

D.C freezes one new internal contract (C-Descent, at D.C.1) and the cross-track **C2** (shape at
D.C.1, finalized at D.C.3), and reads the frozen Track-G / Track-D contracts. **C2 is cross-track**
(consumed by E.C) — this is what restores lever 3 and holds the juncture tier at Opus.

### C-Descent — individual-log descent substrate (compiler + KAT) — *frozen D.C.1*

**Defined:** D.C.1. **Consumed by:** D.C.2 (recursion drives the frontier), D.C.3 (assembly walks the
tree). Compiler-enforced (DescentNode / frontier / descent-entry signatures) + KAT-enforced (frontier
ordering invariant + single-node descent). Sub-track-internal (not consumed outside Track D).

**Frozen interface (`gnfs/src/dl/descent/`):** *(to be frozen at D.C.1 — the inflection-juncture
fork writes the resolved DescentNode / frontier / descent-entry signatures here at execution time.)*
Anticipated surface (over-specify lightly): a `DescentNode` carrying the descended prime/ideal, the
rewriting relation, child references, and a known-log flag; a frontier type ordered largest-prime-
first (termination invariant); a `descend_node(...)` entry D.C.2 implements and a frontier-init entry
the initialization-smoothing feeds.

### C2 — NFS-DL solver interface (compiler + KAT) — *shape opened D.C.1, frozen D.C.3*

**Defined:** D.C (shape D.C.1, finalized D.C.3). **Consumed by:** E.C (MOV bridge — Track E). This is
the project's most-visible cross-track contract (ROADMAP Contract C2). Compiler-enforced (the
`solve_dl` signature + `SolveDlError`) + KAT-enforced (end-to-end individual log, k = 1).

**Frozen interface (`gnfs/src/dl/descent/solve.rs`):** *(shape to be frozen at D.C.1; error taxonomy
finalized at D.C.3.)* Anticipated surface, per ROADMAP Contract C2:

```rust
/// Compute the discrete logarithm log_g(h) in F_{p^k} via NFS-DL.
///
/// k = 1 (prime field F_p) is implemented; k > 1 (extension field) returns
/// SolveDlError::Unsupported — the F_{p^k} extension is a deferred E.C-prep session.
pub fn solve_dl(
    g: /* group element */,
    h: /* group element */,
    p: &BigInt,
    k: usize,
) -> Result<BigInt, SolveDlError>;

pub enum SolveDlError {
    /// k > 1 (extension field F_{p^k}) not yet supported (deferred to E.C-prep).
    Unsupported,
    /// Descent failed to terminate within bounds for this target/size.
    DescentFailed,
    // … further size/failure variants finalized at D.C.3 once descent reality is known.
}
```

**Scope at freeze:** prime field (k = 1) live; F_{p^k} (k > 1) `Unsupported` — a recorded debt
(Discoveries), not a silent gap. E.C must not be implemented against a PARI stub permanently
(ROADMAP); the k > 1 path is built in a deliberate E.C-prep session before the MOV climax.

### Frozen contracts read by D.C (not amended)

- **C-LinAlgFl** — F_ℓ block-solver substrate; provides `VirtualLogTable { rational_logs,
  algebraic_logs }` (`gnfs/src/dl/linalg/blockvec_fl.rs`) and `recover_virtual_logs` — *frozen D.B.1
  (652cfa6)*. D.C reads the virtual-log table for descent leaves.
- **C-DLRelation** — `DLRelation` + `DLMatrix` (`collect_dl_relations`, `from_relations`, `num_cols`,
  rational|algebraic|Schirokauer column layout) — *frozen D.A.1 (f2dbf0a), assembled D.A.2
  (651c17e)*. D.C reuses relation collection for descent relations.
- **C-Schirokauer** — Schirokauer map (`schirokauer` / `compute_schirokauer`, `PrimeIdeal`) —
  *frozen D.A.1 (f2dbf0a)*. The descent's medium-prime relations carry Schirokauer columns; assembly
  accounts for them.
- **C-FactorBase** — `FactorBase` (`rational_primes`, `algebraic_ideals`, `rational_index`,
  `algebraic_index`, `AlgebraicPrime`) — *frozen G.C.1 (c1dc0b6)*. Leaf detection (is this prime in
  the factor base?) keys on the index lookups.
- **Special-q / lattice sieve** — `special_q_sieve`, `lattice_sieve`, `SpecialQConfig`,
  `SpecialQResult` (`gnfs/src/sieve/special_q.rs`, `lattice.rs`) — *frozen G.C (toy-scale)*. **Read /
  reuse** for per-node descent. D.C.2 confirms the sieve can be driven at a fixed target `q` (a
  possible additive contract discovery on the sieve surface).
- **C1** — `shared::numth` smoothness (`trial_smooth`, `SmoothWitness`, `Uint<4>`,
  `norm_to_uint`) — *frozen α.2 / width-policy D.A*. The initialization-smoothing and per-node
  descent reuse `trial_smooth`. **Width not in D.C's surface** (D.C touches FB indices, F_ℓ logs,
  special-q ideals); the ROADMAP width policy is untouched.
- **`Fp<L>` (`shared-field`)** — prime-field trait (`FpNaive4`); the F_ℓ logs live here — *Phase α
  substrate*. `bigint_to_fp` (D.B.1) reused where a `BigInt` log re-enters `Fp`.

(Plus the remaining frozen Track-G / Track-D contracts — C-NF, C-Ideal, C-Res, C-Dedekind, C-Score,
C-Matrix, C-LinAlg, C-AlgSqrt — read where relevant but not foregrounded in D.C.)

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion. "Froze"
names contracts this session locked. The D.C.1 `@plan` confirmation is not a ledger row (a paged fork
with no commit-shaped deliverable); its outcome is recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| D.C.1 | Descent substrate + init-smoothing + C2 shape; freeze C-Descent, open C2 | pending | — | — |
| D.C.2 | Special-q descent recursion | pending | — | — |
| D.C.3 | Individual-log assembly + C2 freeze + end-to-end DL KAT | pending | — | — |

Contracts frozen before this sub-track (read by D.C): C-NF (bdba6f5 / extended 20cd263), C-Ideal
(05b27c8), C-Res (bcd63cd), C-Dedekind (7844773), C-Relation (c1dc0b6), C-FactorBase (c1dc0b6),
C-Score (00aa32d), C-Matrix (a0e854b), C-LinAlg (416f6db), C-AlgSqrt (c80a855 + ec69a1f), C1 (α.2),
C-Textbook (5c9b783), C-Schirokauer (f2dbf0a), C-DLRelation (f2dbf0a), C-LinAlgFl (652cfa6). This
sub-track continues Phase γ over the frozen GNFS pipeline and the D.A/D.B DL substrate, and freezes
one new internal contract (C-Descent, D.C.1) plus the cross-track **C2** (shape D.C.1, frozen D.C.3).

---

## Action-frame digest

*(none yet)*

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **C2 scoped to the prime field F_p (k = 1); F_{p^k} extension deferred (decided at D.C sharding,
  2026-06-08).** D.B's virtual-log pipeline solves DL in **F_p** (the D.B.2 end-to-end KAT is p = 11,
  prime field). The ROADMAP C2 signature is `solve_dl(g, h: F_pk_element, p, k)` — DL in **F_{p^k}**
  (E.C's MOV bridge lands ECDLP in an extension field). Extending NFS-DL from F_p to F_{p^k} is
  *genuine new mathematics* (extension-field / tower number-field setup) D.B's substrate does not
  support. *Decision:* D.C freezes the **C2 signature in its full F_{p^k} shape** (so E.C's call site
  is stable) but implements only the **k = 1 (prime-field) path**; k > 1 returns
  `SolveDlError::Unsupported`. The F_{p^k} extension is its own deliberate **E.C-prep
  ROADMAP-then-shard session** before the MOV climax — the same "freeze the interface/trigger, defer
  the generality" discipline as the C1 width policy. *Tradeoff named:* E.C will need the k > 1 path
  built before the MOV bridge can call a *real* solver (the ROADMAP forbids a permanent PARI stub at
  E.C) — a **known, surfaced debt**, not a silent gap. **internal-continue** within D.C scope;
  building the F_{p^k} path here is **defocus** (additive-reshard at most, never inline). A genuine
  discovery that the C2 *signature* itself can't accommodate F_{p^k} cleanly (not just the body) is
  an **additive-reshard** at the D.C.1 `@plan` freeze, surfaced to the human.

- **C2 shape opens at D.C.1, error taxonomy freezes at D.C.3 (not all at D.C.1).** The cross-track
  C2 *signature* freezes at D.C.1 (E.C's call site stability), but the `SolveDlError` *taxonomy*
  (descent-failure / size variants) is genuinely known only after D.C.2's descent runs. Freezing the
  full taxonomy at D.C.1 would risk a freeze D.C.2/D.C.3 must reopen. **internal-continue:** the
  shape/taxonomy split is deliberate; a taxonomy addition at D.C.3 is **additive** (C2 is D.C's own
  until E.C consumes it), not a destructive reopen.

- **Special-q sieve reuse seam (resolve at D.C.2).** The Track-G `special_q_sieve` was built for
  relation *collection* (scan `[q_min, q_max]`), not *descent of a specific `q`*. D.C.2's per-node
  descent needs to drive the sieve at a **fixed target `q`**. If the existing entry supports this
  (or a thin wrapper does), **internal-continue**. If it needs a genuine signature change to the
  Track-G sieve surface, that is an **additive** contract discovery surfaced at the ◆ boundary —
  never a destructive edit to the frozen G.C sieve contract (an inline change that breaks the G.C
  sieve KATs is a **destructive-HALT**).

- **Descent termination is the delicacy (lever 2, D.C.2).** Special-q descent must strictly reduce
  the largest prime at each step and not loop. The frontier-ordering invariant (largest-first, set
  at D.C.1) is the termination argument; a node that fails to descend within a bounded search
  surfaces `SolveDlError::DescentFailed`, never an infinite loop. A toy instance where descent
  legitimately cannot terminate (no relation found) is **internal-continue** (a clean error), not a
  contract break.

- **Subgroup recovery scope (decide at D.C.3 ◆).** The descent recovers `log_g(h)` **mod ℓ**. If ℓ
  is a proper factor of `p−1`, the full log mod (p−1) needs Pohlig–Hellman / CRT. **In-scope at
  demonstration fidelity** (or the toy KAT picks ℓ = p−1 to sidestep it) — decided at the boundary.
  Deferring full-order recovery with a principle-4 annotation is **internal-continue**; it is not a
  contract break (C2 returns the log in the target subgroup, documented).

- **No F_{p^k}, no E.C in D.C (defocus guard).** D.C stops at a prime-field individual logarithm and
  the C2 shape. The F_{p^k} extension-field solver and the E.C MOV bridge are **out of D.C**.
  Implementing either here is **defocus** — internal-continue only within D.C scope.

- **PARI oracle gating (resolved policy, D.A boundary — apply uniformly).** D.C.3's individual-log
  KAT cross-checks against PARI's discrete log. Per the resolved project-wide policy: oracles are
  **absent-by-default, opt-in, skip cleanly** — the PARI KAT is `#[ignore = "PARI not installed;
  run manually when available"]` (matching the `kat_h_pari_oracle` / `kat_pari_dl_oracle` stubs: no
  feature flag, no subprocess in CI), and a deterministic non-PARI KAT carries the reproducibility
  burden. No new policy decision is owed.

---

## Notes for executors

- Read `docs/ROADMAP.md` (the D.C spec under Phase γ — "Individual logarithm + special-q descent";
  **Contract C2** — the NFS-DL solver interface this sub-track freezes, consumed by E.C; **Contract
  C1 → Width policy** to confirm D.C does not touch the smoothness width) and this PLAN before any
  session.
- Read the substrate D.C consumes: `gnfs/src/dl/linalg/blockvec_fl.rs` (`VirtualLogTable`,
  `recover_virtual_logs`, `FlSolution`), `gnfs/src/dl/relation.rs` (`DLMatrix`, `collect_dl_relations`,
  `num_cols`), `gnfs/src/dl/schirokauer.rs` (the Schirokauer map, `PrimeIdeal`), `gnfs/src/dl/mod.rs`
  (`DLRelation`, the `mod.rs:85` descent hook), `gnfs/src/sieve/factor_base.rs` (`FactorBase`,
  `rational_index` / `algebraic_index`, `AlgebraicPrime`), `gnfs/src/sieve/special_q.rs` +
  `gnfs/src/sieve/lattice.rs` (the special-q / lattice sieve to reuse for descent),
  `gnfs/src/sieve/mod.rs:259` (`Relation::new` — the factor-over-the-factor-base pattern),
  `shared/numth/src/smooth.rs` (`trial_smooth`, `SmoothWitness`). The G.C sieving and D.B linalg
  PEDAGOGY/MATHEMATICS sections give the mathematical background; the NFS-DL writeup is D.W (later,
  paired with T.D) — **no PEDAGOGY chapter in D.C**.
- **Register:** D.C is **code** (Rust, `STYLE-CODE-RUST.md`), with KATs in `gnfs/tests/*_kat.rs`
  following the existing naming convention (`dl_descent_kat.rs`, `dl_individual_log_kat.rs`).
- **Tier routing:** **D.C.1 is Opus** (`@build` on Opus, per the ROADMAP Opus-flagged table —
  "D.C.1 special-q descent design"); **D.C.2 and D.C.3 are Sonnet**. D.C.1 carries one `@plan`
  marker: a T0/Opus post-landing C-Descent + C2-shape freeze confirmation (page `@plan-juncture`)
  before D.C.2 is dispatched — held at Opus on **levers 3+4** (cross-track C2 + correctness). The
  juncture-tier (header) is **opus** on the same lever-3+4 call.
- **Invariants to preserve:** all Track-G code contracts and the D.A/D.B DL contracts (C-Schirokauer,
  C-DLRelation, C-LinAlgFl, C-FactorBase, the special-q sieve) are **frozen** — D.C reads and reuses
  them; it amends none. **The Track-G special-q sieve stays untouched** (reuse, not modify — a
  needed signature change is an additive discovery, not an inline edit; breaking the G.C sieve KATs
  is a destructive-HALT). **C1 `Uint<4>` stays as-is** (not in D.C's surface). New contracts: C-Descent
  (D.C.1, internal) and C2 (D.C.1 shape → D.C.3 freeze, cross-track).
- **PARI remains a dev-only oracle**, never on a build path; the project-wide gating policy is
  resolved (D.A boundary) — apply the `#[ignore]` stub pattern uniformly.
- Suggested first invocation: **`/run-plan docs/PLAN.md`** (default cadence). The two `@plan`/◆
  junctures already force the halts that matter — the **D.C.1 C-Descent + C2-shape freeze
  confirmation** (before D.C.2 consumes it) and the **D.C.3 ◆ C2 final freeze + sub-track close** —
  so no additional boundary halts beyond those preconditioned ones are needed. *(Tradeoff: this is
  one notch less conservative than D.B's `halt-at-boundaries`; it is justified because the two
  cross-track-critical freezes are already `@plan`-gated, and the descent pattern, while new, is
  exercised against a mature test suite — lever 5 strong.)*
