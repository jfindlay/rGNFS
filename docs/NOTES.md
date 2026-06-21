# rGNFS — Notes

Rolling-context durable framings, decisions, and mental models. Distinct from `docs/PLAN.md`
(current sub-track) and `docs/ROADMAP.md` (project-lifetime view). Captured framings that prove
durable graduate to the ROADMAP Discoveries log at a sub-track boundary.

---

## Transfer-style attacks shard into transfer / structure / solve (2026-06-15, E.H shard)

The small-characteristic index-calculus attack on binary-curve ECDLP decomposes into three
sub-tracks at clean contract seams: **E.H** (GHS/Weil descent) *transfers* an ECDLP on `E/GF(2^m)`
to a DLP on `Jac(C)/GF(2^l)`; **E.J** builds the Semaev summation polynomials; **E.K** runs the
index-calculus *solve*.

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

### The GHS dangling-transfer pattern

The GHS chapter (Ch. 10.3) reduces ECDLP to hyperelliptic Jacobian DLP but does not develop the
downstream solve. This is the only genuine spectrum gap found (F-D7-01). The pattern — "build the
transfer, delegate the solve" — is the same idiom seen across the project (MOV/SSA transfer to a
different DLP; E.H transfers to hyperelliptic Jacobian DLP). The substrate for the downstream
solve already exists (`rho/src/hyperelliptic/`: Mumford divisors, Cantor group law); the missing
layer is the index calculus on the Jacobian (Gaudry 2000). Whether to fill this gap in arc-2 or
defer to arc-3 is open-Q 3 for the human.
