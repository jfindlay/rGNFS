<!--
juncture-tier: opus
-->

# rGNFS — Current Plan: Phase β, sub-track G.F (Square root + assembly)

The rolling, current-sub-track view of the work, in `/run-plan`-executable form (session list +
contracts + ledger + digest). Rewritten at sub-track boundaries. For the project-lifetime view, see
`docs/ROADMAP.md`. For the planning philosophy, see
`~/.config/opencode/multisession/multi-session-planning.md`.

`juncture-tier: opus` (header above) — **holds the default; does not opt down.** Applying the
five-lever law to G.F: lever 2 (irreducible complexity) is **high** — the **algebraic square root via
Couveignes / Chinese remaindering** (reduce ℤ[α] elements mod split prime ideals → per-prime
Tonelli–Shanks → CRT lift → embedding-sign selection) is a genuine FLOOR that cannot be fractured
below one working algebraic-root computation; lever 3 (design-error cost) is **high** — G.F.1 adds
`NumberFieldElement::reduce_mod_ideal`, a **new cross-crate `shared/numfield` API** that D.B (NFS-DL
over F_ℓ) is the natural re-consumer of, exactly the C-LinAlg cross-track situation one stage on;
lever 4 (correctness-criticality) is **highest in the pipeline** — G.F is the **terminal seam**: a
wrong square root produces a *trivial* gcd (1 or N), a **silent non-factorization**, not a red test.
Levers 2/3/4 all high is the case the planning doc names as **holding the adjudicator at Opus**.
Lever 5 (inner-loop bandwidth) is strong (`cargo test --workspace` + deterministic
factor/root KATs) but does **not** license opting down: the real behavioural gate is the end-to-end
"factor an 80–100-bit challenge" KAT, whose oracle (CADO-NFS / msieve) may be absent — so lever 5
does **not** coincide with low correctness-criticality.

**Roadmap-frame flex (logged, additive).** The ROADMAP scoped G.F as **"2 sessions, Sonnet,"** with
**no Opus-flag** — written *before* the substrate survey revealed that G.F is not a thin wrapper. The
survey found **three substrate gaps** G.F must fill (`reduce_mod_ideal` in numfield; `isqrt` and an
exported big-integer `gcd` in bigint) and **two distinct square roots** (rational integer root;
algebraic root via Couveignes) plus assembly. This plan re-shards to **4 algorithmic sessions +
writeup** and adds **two Opus junctures at G.F.3** (the Couveignes FLOOR) — a *design* inflection
(freeze `reduce_mod_ideal` + the CRT/embedding-sign strategy) and a *correctness review* after it
lands (the terminal silent-failure seam). This mirrors the G.E two-juncture pattern, sited by
cost-of-wrong rather than by the boundary. The flex is **additive** (re-tier + split, no contract
break) and is surfaced for the ROADMAP Discoveries log at the G.F ◆ boundary (see Discoveries).

Last rewrite: G.E ◆ boundary crossed (G.E.W landed at `a985965`; G.E ledger still-on-intent
2026-06-07, `f8ca3f8`). G.E fully complete (G.E.1 → G.E.W); **C-LinAlg frozen** (`BlockVec`,
`MatrixOperator`, `KernelVector`, QC columns), C-Matrix / C-FactorBase / C-Relation stable. This plan
opens sub-track G.F — the **square-root and assembly** stage, the final stage of the GNFS factoring
pipeline. G.F takes a `KernelVector` (a congruence of squares), recovers the original (a, b) pairs
through the C-Matrix provenance map, computes the rational and algebraic square roots, and combines
them via integer GCD to extract a non-trivial factor of N.

---

## Purpose (design intent)

Per ROADMAP: a self-consistent, pedagogically clear Rust reference library for DLP/ECDLP/GNFS
algorithms. This sub-track (G.F) builds **GNFS square root + assembly** — the terminal stage that
turns a nullspace vector into a factor. Given a `KernelVector` from G.E (a subset S of relations
whose combined rational and algebraic norms are each perfect squares), compute:

1. **The rational square root.** X with $X^2 \equiv \prod_{i \in S}(a_i - b_i m) \pmod N$ — the
   product of rational norms is a perfect-square integer (the matrix nullspace + sign column
   guarantee it); extract its integer square root and reduce mod N.
2. **The algebraic square root (Couveignes' algorithm).** β ∈ K = ℚ(α) with
   $\beta^2 = \prod_{i \in S}(a_i - b_i \alpha)$; then Y = Norm(β) ∈ ℤ, and y = Y mod N. The QC
   columns (frozen in C-FactorBase by G.E.1) guarantee β exists *in K*. Couveignes computes β by
   **Chinese remaindering**: reduce the product element modulo many split prime ideals, take the
   square root in each residue field 𝔽_p (Tonelli–Shanks), and CRT-lift to a candidate in ℤ[α],
   resolving the global sign via the real embedding.
3. **Assembly.** Form x = X mod N, y = Y mod N; if x ≢ ±y (mod N), $\gcd(x - y, N)$ is a non-trivial
   factor of N. Wrap the whole pipeline (kernel → provenance → roots → gcd) in a driver and verify
   end-to-end.

It comprises four algorithmic sessions:

1. **Substrate gaps (G.F.1).** Fill the three capabilities the survey found absent:
   `NumberFieldElement::reduce_mod_ideal(p, r)` (ℤ[α] element → 𝔽_p, the Couveignes inner step),
   `isqrt` (exact integer square root for the rational side), and an **exported** big-integer `gcd`
   (the final assembly step). These reach into frozen `shared/numfield` and `shared/bigint` —
   additive-reshard, surfaced for sign-off.
2. **Rational square root (G.F.2).** Recover (a, b) pairs via `expand_provenance`, form the product
   ∏(a − bm), verify it is a perfect square, extract X via `isqrt`, reduce mod N.
3. **Algebraic square root via Couveignes (G.F.3, the FLOOR).** The CRT square root in ℤ[α], the
   lever-2/3/4 session: design-inflection `@plan` (freeze `reduce_mod_ideal` + the CRT/embedding-sign
   strategy) and a post-landing correctness-review `@plan`.
4. **Assembly + end-to-end driver (G.F.4).** Combine the roots, take gcd(x − y, N), and stand up the
   first top-level "factor N" driver chaining the whole pipeline. End-to-end factoring KAT.

This is the fifth and final stage of the GNFS factoring pipeline proper: it sits *on* the G.E
nullspace representation (C-LinAlg `KernelVector`), the C-Matrix provenance map, and the G.A
number-field substrate (C-NF), and it *produces* the factorization that closes the GNFS arc. G.W
(separate sub-track, Opus) is the integrative writeup that articulates the whole cross-phase pipeline.

Re-read this intent at every ◆ boundary to catch **defocus** (gold-plating beyond Couveignes at
demonstration fidelity — the full early-abort CRT prime budgeting, the lattice-based square-root
variant, and Montgomery's batch square root are out of scope, ROADMAP principle 3) and **rigidity**
(grinding through a CRT sign convention that the end-to-end factor KAT shows is wrong, rather than
surfacing it at the G.F.3 review juncture).

**Scoping discipline (ROADMAP three-way split, applied to G.F).** Algorithmic content complete:
rational integer square root, Couveignes' CRT algebraic square root with embedding-sign resolution,
the final gcd assembly, and the kernel → provenance → roots thread. **Scale-only** content (the
number of CRT primes needed to pin β, the bit-length growth of the lifted coefficients, early-abort
on a wrong sign) is present at **demonstration fidelity** (the CRT lift is real even where its prime
budget is trivial at toy scale — principle 2, disconnect annotated per principle 4). **Engineering
optimizations** (batch Tonelli–Shanks, lattice square root, p-adic Newton lift) are omitted.
**CADO-NFS / msieve are dev-only correctness oracles** for the end-to-end "recover the same factor"
cross-check — never on a build path.

---

## Current state

Phase α + sub-tracks G.A, G.B, G.C, G.D, G.E complete. Workspace crates: `shared/field`,
`shared/bigint`, `shared/numfield`, `shared/numth`, `rho`, `gnfs`. `cargo test --workspace` green at
`f8ca3f8`.

The `gnfs` crate carries `polyselect/` (G.B), `sieve/` (G.C), `filter/` (G.D), and `linalg/` (G.E).
G.F adds a sibling **`sqrt/`** module (no sqrt module exists yet — clean greenfield, the fourth
non-`polyselect`/`sieve` module after `filter/` and `linalg/`). The gnfs `lib.rs` module list is
`pub mod polyselect; pub mod sieve; pub mod filter; pub mod linalg;` — G.F.1 adds `pub mod sqrt;`.

Substrate G.F consumes (all frozen):
- **C-LinAlg** (`gnfs::linalg`): `KernelVector { row_indices: Vec<usize> }` with
  `expand_provenance(&self, &SparseMatrix) -> Vec<usize>` (symmetric difference of provenance →
  original relation indices) and `verify`. **This is the G.E → G.F seam, over-specified at G.E.1
  precisely for this consumer.** Frozen at G.E.1 (`416f6db`).
- **C-Matrix** (`gnfs::filter`): `SparseMatrix` with the `provenance: Vec<usize>` per row that
  `expand_provenance` threads. Frozen at G.D.1 (`a0e854b`).
- **C-Relation** (`gnfs::sieve`): `Relation { a: BigInt, b: BigInt, rational_exponents,
  algebraic_exponents, rational_sign: bool }`. G.F reads `a`, `b` (to form ∏(a − bm) and ∏(a − bα))
  and `rational_sign`. Frozen at G.C.1 (`c1dc0b6`).
- **C-PolyPair** (`gnfs::polyselect`): `PolyPair { f, g, m: BigInt, n: BigInt, degree, ... }` with
  `monic_f()` and `number_field() -> NumberField`. G.F reads `n` (target), `f`/`m` (the two sides),
  and constructs K via `number_field()`. Frozen at G.B.1 (`2f43f99`).
- **C-NF** (`shared::numfield`): `NumberField`, `NumberFieldElement` (`mul`, `square`, `pow`, `norm`,
  …), `Ideal { p, r }`. G.F **adds `reduce_mod_ideal`** to `NumberFieldElement` (the one sanctioned
  reach — see below). Frozen at G.A.3 (`7844773`).
- **C-Fp** (`shared::field`): the `Fp<L>` trait with **`legendre` and `sqrt` (Tonelli–Shanks) already
  present** as default methods (the ROADMAP-noted α.5 patch is *landed* — survey-confirmed; this
  gap is closed). G.F's per-prime square root calls `Fp::sqrt`. Frozen (α.5).

**Substrate gaps G.F fills itself (G.F.1, additive-reshard).** The survey found three capabilities
G.F needs that do not yet exist:
1. **`NumberFieldElement::reduce_mod_ideal(p, r)`** — reduce an ℤ[α] element mod the prime ideal
   `(p, α − r)` to an 𝔽_p element. The inner step of Couveignes' CRT. New method on a frozen C-NF
   type (additive — no existing signature changes). **D.B is the natural re-consumer** (F_ℓ residue
   fields), so freeze the signature deliberately, C-LinAlg-style.
2. **`isqrt(&BigInt) -> Option<BigInt>`** (exact integer square root, `None` if not a perfect
   square) — for the rational side. Absent everywhere; `shared/bigint` is the home.
3. **Exported big-integer `gcd`** — `shared/sieve` has a *private* `gcd_bigint`; lift/export a
   `gcd(&BigInt, &BigInt) -> BigInt` to `shared/bigint` for the final `gcd(x − y, N)`.

All three are **additive** (new functions / a new method; no signature breaks) and surfaced as an
**additive-reshard** at G.F.1 for sign-off, not silently grown.

---

## Verify gate

`VERIFY_TEST = cargo test --workspace`. `VERIFY_TYPES = cargo check --workspace` (Rust's compiler is
the type gate; `cargo test` subsumes it on a clean build, so one green `cargo test --workspace`
satisfies both). A red session is not a complete session. G.F adds a module to the *existing* `gnfs`
crate and methods to the *existing* `shared/numfield` + `shared/bigint` crates (all already in
`members`), so no workspace `Cargo.toml` change is required. The end-to-end factoring oracle KAT
follows the established `#[ignore = "CADO-NFS not installed; ..."]` pattern
(`lanczos_kat.rs:317`, `merge_kat.rs`, `line_sieve_kat.rs`) and skips cleanly when the oracle is
absent; the deterministic "factor this known N" KAT carries reproducibility without an external
oracle.

---

## Session list

One commit-shaped session per row (~150–400 LOC, 2–4 files — the current default). `Cat` = category
(A substrate / B algorithm / C optimization / I integrative). `◆` marks a sub-track-final session.
`@plan` marks an inflection or review point requiring a juncture fork + human sign-off before the next
session is dispatched.

| # | Session | Cat | Tier | Consumes | Expected files |
|---|---------|-----|------|----------|----------------|
| G.F.1 | Square-root substrate: `reduce_mod_ideal` + `isqrt` + exported BigInt `gcd` | A | Sonnet | C-NF, C-Fp | `shared/numfield/src/element.rs` (add `reduce_mod_ideal`), `shared/bigint/src/lib.rs` (+ new `isqrt.rs` / `gcd` export), `shared/numfield/tests/reduce_mod_ideal_kat.rs`, `shared/bigint/tests/isqrt_gcd_kat.rs` |
| G.F.2 | Rational square root from a kernel vector | B | Sonnet | C-LinAlg, C-Relation, C-PolyPair, G.F.1 | new `gnfs/src/sqrt/mod.rs`, `gnfs/src/sqrt/rational.rs`, `gnfs/src/lib.rs` (add `pub mod sqrt`), `gnfs/tests/sqrt_rational_kat.rs` |
| G.F.3 `@plan` | Algebraic square root via Couveignes / CRT | B | **Opus** (design fork) + Sonnet build + **Opus** review fork | C-NF (`reduce_mod_ideal`), C-Fp (`sqrt`), C-Relation, C-PolyPair | `gnfs/src/sqrt/algebraic.rs`, `gnfs/src/sqrt/mod.rs`, `gnfs/tests/sqrt_algebraic_kat.rs` |
| G.F.4 | Assembly + end-to-end factor driver (gcd(x−y, N)) | I | Sonnet | all G.F, C-PolyPair | `gnfs/src/sqrt/assembly.rs`, `gnfs/src/sqrt/mod.rs`, `gnfs/tests/factor_end_to_end_kat.rs` |
| G.F.W ◆ | G.F integrative writeup (square-root chapter) | I | Sonnet | all G.F | `gnfs/docs/PEDAGOGY.md` (append §42+), `docs/BENCHMARKS.md` (append) |

**Sequencing notes.** G.F carries **two `@plan` junctures, both at G.F.3** (the Couveignes FLOOR),
of different kinds. The first is a *design* inflection (runs **before** G.F.3 implements): page an
Opus `@plan-juncture` fork to freeze **C-AlgSqrt** — the `reduce_mod_ideal` signature (already
sketched at G.F.1 but the *Couveignes contract* — which split primes, how many, the CRT lift, the
embedding-sign convention — is resolved here), because a wrong sign convention or an under-budgeted
prime set is a silent terminal failure and the API is D.B-reused. The second is a *correctness-review*
juncture (runs **after** G.F.3 lands green, before G.F.4): page an Opus `@plan-juncture` fork to
review the landed CRT square root against the code + action-frame digest — because a Couveignes bug
is *silent* (a plausible β with the wrong sign → trivial gcd at G.F.4) and the end-to-end oracle may
be absent. Both forks return one-shot findings; neither implements. **G.F.1** (substrate gaps) and
**G.F.2** (rational root) are mutually orderable but both must precede G.F.3 (it consumes
`reduce_mod_ideal`); G.F.2 is independent of G.F.3 and could run after it, but the listed order keeps
the cheaper rational root first as a warm-up that exercises `expand_provenance`. **G.F.4** consumes
both roots; **G.F.W** is the ◆ boundary.

**Why G.F is 4 algorithmic sessions + writeup (ROADMAP said 2).** The one-line-commit-title corollary
drives the split: "square root" alone is *three* commit titles (substrate gaps; rational root;
Couveignes algebraic root) because the survey revealed three absent substrate capabilities and two
mathematically distinct roots — none of which the ROADMAP's "2 sessions" line anticipated (it
predates the survey). The Couveignes algebraic root (G.F.3) is the lever-2 FLOOR and **cannot** be
merged with the rational root (a different algorithm: integer isqrt vs CRT-in-ℤ[α]) or with the
substrate gaps (folding `reduce_mod_ideal` into it would make two titles and bury the FLOOR's design
surface). Assembly (G.F.4) is split from the roots because the end-to-end factor driver is the *first*
pipeline-chaining code in the project (no driver exists yet — survey item 7) and earns its own title +
its own end-to-end KAT. **G.F.W** is the integrative writeup, allocated its own session per the
under-scheduling guidance. The two G.F.3 junctures are **not** session rows: per the planning doc, a
juncture fork has no commit-shaped code deliverable, so they ride as `@plan` markers on G.F.3, not as
ledger lines.

---

## Session detail

Lower-fidelity rows (G.F.3, G.F.4, G.F.W) are sketched; per the planning philosophy, sessions inside a
sub-track are crisply specified only after the substrate (G.F.1) and the G.F.3 design juncture resolve
C-AlgSqrt. G.F.1 and G.F.2 are crisp; G.F.3's design surface is deliberately left for its `@plan` fork.

### G.F.1 — Square-root substrate: `reduce_mod_ideal` + `isqrt` + exported BigInt `gcd` (Sonnet, additive-reshard)

**Deliverable:**
- **`NumberFieldElement::reduce_mod_ideal(&self, p: &BigInt, r: &BigInt) -> /* Fp value */`** on the
  C-NF element type. Reduces the element's `RatPoly` representation mod the prime ideal `(p, α − r)`:
  evaluate the polynomial at α ≡ r (mod p), clear denominators mod p, return the 𝔽_p residue. The
  bridge from `BigInt`-valued coefficients into the `Fp<L>` world (survey item 4's missing bridge)
  lives here. **Additive** — new method, no change to any frozen `NumberFieldElement` signature.
- **`isqrt(&BigInt) -> Option<BigInt>`** in `shared/bigint`: exact integer square root, `Some(x)` iff
  the input is a perfect square `x²`, else `None`. Newton/bisection at demonstration fidelity.
- **Exported `gcd(&BigInt, &BigInt) -> BigInt`** in `shared/bigint`: lift the private `gcd_bigint`
  from `gnfs::sieve` (or delegate to `num_integer::gcd`) to a public, tested home. The final
  `gcd(x − y, N)` consumes it.

**Key design decisions (the additive-reshard surface):**
1. **`reduce_mod_ideal` return type and the `Fp` limb width.** Whether it returns a concrete
   `FpNaive<L>` / `Uint<L>` (and which `L`) or a `BigInt` residue. Bias toward returning the residue
   in a form G.F.3 can feed to `Fp::sqrt` directly. **Freeze the signature** — D.B (F_ℓ) re-consumes
   this; over-specify toward a scalar-residue shape, C-LinAlg-style.
2. **`isqrt` perfect-square contract.** `Option` (None = not a square) vs `(BigInt, bool)`. Bias
   toward `Option` — the rational-root caller wants exactly "is this a perfect square, and if so its
   root."
3. **`gcd` home and signedness.** `BigInt` (signed) gcd must return the non-negative gcd; document the
   sign convention. Confirm whether to lift-and-delete the `sieve` private copy or leave it (avoid two
   copies — prefer lift + re-export from `sieve` if needed).

**KAT (≥1 required):**
1. **`reduce_mod_ideal` correctness:** for a hand-built `NumberField` (e.g. f = x² − 2) and a split
   prime ideal `(p, r)`, the reduction of α, of a rational constant, and of a product matches the
   hand-computed 𝔽_p value; and `reduce_mod_ideal(α) = r mod p`.
2. **`isqrt`:** `isqrt(k²) = Some(k)` for several k; `isqrt(k² + 1) = None`; `isqrt(0) = Some(0)`.
3. **`gcd`:** matches known gcds; `gcd(0, n) = n`; non-negative result for mixed-sign inputs.

**Subtlety:** `reduce_mod_ideal` must clear rational denominators mod p — if a coefficient
denominator is divisible by p the reduction is undefined (the prime ideal is "bad" for this element);
decide and document the contract (panic? `Option`? — for split QC-style primes chosen > B_alg this
should not arise, but the contract must be explicit). **The additive-reshard must be surfaced** at
session start: it adds a method to frozen C-NF and functions to frozen `shared/bigint`; mechanical, but
sign-off per the additive-reshard rule.

**Deferred:** the rational root (G.F.2); Couveignes (G.F.3); assembly (G.F.4); the writeup (G.F.W).

### G.F.2 — Rational square root from a kernel vector (Sonnet)

**Deliverable:** the rational side of the congruence. Given a `KernelVector` and the original
`Vec<Relation>` + `PolyPair`: call `kv.expand_provenance(&matrix)` to get the relation index set S;
form the product $\prod_{i \in S}(a_i - b_i m)$ over ℤ (using `poly.m`); assert it is a perfect square
via `isqrt` (G.F.1) — the matrix nullspace + sign column guarantee it, so a `None` here is a
*detected* upstream bug, not a normal path; reduce X = isqrt(product) mod N. Returns X (mod N).
Establishes the `gnfs/src/sqrt/` module (`mod.rs` + `rational.rs`) and the `pub mod sqrt` in `lib.rs`.

Freezes the `sqrt` module entry surface (the kernel → product → root signature both sides share).

**KAT (≥1 required):** (a) for a hand-built relation set whose rational-norm product is a known
square, the recovered X satisfies X² ≡ product (mod N); (b) the product is computed over the correct
S (matches a hand-traced `expand_provenance`); (c) a deliberately non-square product triggers the
`isqrt = None` upstream-bug path (assert it is caught, not silently squared).

**Subtlety:** the sign column (frozen in C-FactorBase) guarantees an even count of negative rational
norms, so the product is positive — but G.F.2 must still handle the product sign defensively
(`isqrt` on a negative input is `None`); a negative product means the sign column was not honoured by
the kernel vector (an upstream G.E bug), which G.F.2 should surface, not paper over.

### G.F.3 — Algebraic square root via Couveignes / CRT (Opus design fork + Sonnet build + Opus review fork, the FLOOR, sketch)

**Deliverable:** Couveignes' algorithm for $\beta$ with $\beta^2 = \prod_{i \in S}(a_i - b_i \alpha)$
in K, then Y = Norm(β) ∈ ℤ and y = Y mod N. Sketch (the G.F.3 *design* `@plan` fork resolves and
freezes the specifics into C-AlgSqrt before implementation):
- Form the product element $\gamma = \prod_{i \in S}(a_i - b_i \alpha) \in \mathbb{Z}[\alpha]$ (using
  the C-NF element `mul`).
- Select a set of primes p that split completely in K (reuse the G.E.1 `select_qc_primes` machinery
  or a sibling); for each, `reduce_mod_ideal` γ to each 𝔽_p factor, take `Fp::sqrt` (Tonelli–Shanks,
  C-Fp), and assemble the per-prime square root of the element.
- **CRT-lift** the per-prime roots to a candidate β ∈ ℤ[α] with coefficients pinned once enough
  primes are used (the bit-length / prime-count budget is the principle-4 scale knob).
- **Resolve the global sign** of β via the real embedding of K (α ↦ a real root of f): β and −β both
  square to γ; the correct one makes Y = Norm(β) consistent with the rational side. **This sign
  resolution is the silent-failure locus** — getting it wrong yields a valid-looking Y that produces a
  trivial gcd at G.F.4.

Consumes C-NF (`reduce_mod_ideal`, `norm`), C-Fp (`sqrt`), C-Relation, C-PolyPair. **Freezes
C-AlgSqrt** (the algebraic-square-root contract: the Couveignes entry signature, the prime-selection
rule, the CRT lift, and the embedding-sign convention) — re-consumed by D.B.

**KAT (≥1 required):** (a) for a hand-built K and a γ that *is* a known square β², Couveignes recovers
β (up to sign) and Norm(β) matches the hand-computed Y; (b) the recovered Y, combined with a matching
rational X, satisfies X² ≡ Y² (mod N) for a toy N (the congruence holds before the full driver
exists); (c) determinism for a fixed γ and prime set; (d) **end-to-end via G.F.4's KAT** is the
behavioural gate (deferred to G.F.4, oracle-gated).

**Design juncture (`@plan`, T0/Opus, one-shot — runs BEFORE this session implements).** Page an Opus
`@plan-juncture` fork to design and freeze **C-AlgSqrt**: the number and selection of CRT primes
(enough to pin β's coefficients at toy scale — the principle-4 budget), the CRT lift representation,
and above all the **embedding-sign convention** (how β's global sign is fixed). The fork writes the
resolved interface into Cross-session contracts before implementation is dispatched. Its remit is the
load-bearing design call the ROADMAP's "Sonnet" line did not anticipate.

**Review juncture (`@plan`, T0/Opus, one-shot — runs AFTER this session lands green, before G.F.4).**
Because a Couveignes bug is *silent* (a wrong-sign β → trivial gcd, not a red test) and the end-to-end
oracle may be absent, page an Opus `@plan-juncture` fork to review the landed CRT square root against
the code + action-frame digest. Remit: (1) the **embedding-sign resolution** is correct (the silent
locus); (2) the prime budget genuinely pins β (not under-determined at toy scale); (3) the solver
honours C-NF `reduce_mod_ideal` and C-Fp `sqrt` as frozen; (4) the principle-4 annotations
(prime-count scale knob) are present and honest. The fork returns findings one-shot (accurate /
needs-fix / needs-human discussion) and does **not** implement — fixes are a follow-on `@build` turn.
This is the planning doc's review-juncture instrument, sited at the FLOOR by its cost-of-wrong.

**Subtlety (principle-4 annotation):** the *number* of CRT primes needed to reconstruct β grows with
the coefficient bit-length, which is tiny at toy scale (a handful of primes suffice) — annotate that
the prime budget is the scale knob, while the *algorithm* (reduce → Tonelli–Shanks → CRT → sign) is
the same at all scales. The embedding-sign resolution, by contrast, is **not** a scale artifact — it
is a correctness obligation present even at toy scale, and the primary target of the review juncture.

### G.F.4 — Assembly + end-to-end factor driver (Sonnet, sketch)

**Deliverable:** the terminal assembly. Given X (G.F.2) and Y (G.F.3): form x = X mod N, y = Y mod N;
if x ≡ ±y (mod N) the kernel vector yields the trivial factorization (report and signal "try another
kernel vector"); else compute $\gcd(x - y, N)$ (G.F.1's exported gcd) and return the non-trivial
factor. Then stand up the **first top-level "factor N" driver** (survey item 7: no end-to-end driver
exists yet) chaining kernel-vector → `expand_provenance` → rational root → algebraic root → gcd, with
the multi-kernel-vector retry loop (try kernel vectors until one gives a non-trivial gcd).

Consumes all G.F + C-PolyPair. Freezes nothing new (it is the integrative I-session that *uses* the
frozen roots).

**KAT (≥1 required):** (a) **deterministic end-to-end:** for a small hand-prepared N with a known
matrix + kernel vector, the driver recovers a known non-trivial factor of N (no external oracle —
carries reproducibility); (b) the trivial-gcd path (x ≡ ±y) is detected and the retry loop advances
to the next kernel vector; (c) **end-to-end oracle KAT (the ROADMAP G.F KAT):** factor a published
challenge in the **80–100-bit range** and confirm against CADO-NFS / msieve — dev-only,
`#[ignore = "CADO-NFS not installed; ..."]`d, deterministic KAT (a) carries reproducibility without
it.

**Subtlety:** the trivial-factorization outcome (x ≡ ±y) is *expected* for some kernel vectors — the
driver must loop over multiple nullspace vectors, not treat the first trivial gcd as failure. The
end-to-end KAT's 80–100-bit target may stress toy-scale assumptions elsewhere in the pipeline (norm
bit-widths, the `Uint<4>` limb width noted in ROADMAP α Discoveries) — if a width limit surfaces here,
that is an **additive-reshard** discovery (widen `Uint<L>`), surfaced not silently grown.

### G.F.W ◆ — Integrative writeup (square-root chapter) (Sonnet, sketch)

The square-root chapter (`gnfs/docs/PEDAGOGY.md`, append after §41 as a new
`# Square Root and Assembly: A Code-Tour Chapter`): the kernel vector as a congruence of squares, the
rational square root (perfect-square integer → isqrt), **Couveignes' algorithm** (the CRT square root
in ℤ[α], why reduce-mod-split-ideal + Tonelli–Shanks + CRT recovers β, and the embedding-sign
resolution), why the QC columns (frozen by G.E.1) guarantee β exists in K, and the final
$\gcd(x - y, N)$ that closes the GNFS pipeline. This is the chapter where the **whole factoring arc**
(polyselect → sieve → filter → linalg → square root → factor) is finally visible end-to-end. Append a
G.F benchmark row to `docs/BENCHMARKS.md` (N bit-length / kernel vectors tried / factor recovered /
timing). **Display-math chapter** — uses **MathJax `$$…$$`** (the CRT congruences, the
$\beta^2 = \prod(a - b\alpha)$ identity, the embedding map) per the doc-format recommendation (ROADMAP
Discoveries); `$$` is already established in PEDAGOGY.md (from §1429 and the G.E chapter §31+). Per
pacing guidance, integrative writeups are under-scheduled — allocate a full session. Its Track τ
maths-first sibling (T.G) pairs at the **G.W** ◆ boundary (the GNFS-wide writeup), not here — G.F.W is
a code-tour sub-chapter feeding the eventual G.W chapter. **Sonnet**, consistent with G.D.W / G.E.W
(exposition, not a designated payoff proof — the L-notation / Couveignes-correctness proof, if treated
as payoff, lands in G.W/T.G).

---

## Cross-session contracts

The scaffolding sessions compose through. The G.F.3 design juncture writes the resolved **C-AlgSqrt**
interface into this section before implementation is dispatched.

### C-LinAlg — GF(2) nullspace substrate: blocked vectors + matrix operator + kernel representation (compiler + KAT) — *frozen at G.E.1 (416f6db)*
**Defined:** G.E.1. **Consumed by (in G.F):** G.F.2 / G.F.3 (`KernelVector::expand_provenance` to get
the relation index set S), G.F.4 (the driver iterates kernel vectors). G.F **reads** the kernel
representation; it does **not** amend C-LinAlg. The `expand_provenance` seam was over-specified at
G.E.1 *for this consumer*; G.F is its first real client. Stable for G.F.

### C-Matrix — filtered sparse GF(2) matrix + relation-provenance map (compiler + KAT) — *frozen at G.D.1 (a0e854b)*
**Defined:** G.D.1. **Consumed by (in G.F):** G.F.2 / G.F.3 (the `provenance` map, via
`expand_provenance`). G.F **reads** provenance; it does not amend C-Matrix. Stable for G.F.

### C-Relation — relation / exponent-vector format (compiler + KAT) — *frozen at G.C.1 (c1dc0b6)*
**Defined:** G.C.1. **Consumed by (in G.F):** G.F.2 (`a`, `b`, `rational_sign` to form ∏(a − bm)),
G.F.3 (`a`, `b` to form ∏(a − bα)). G.F **reads** relations; it does not amend C-Relation. Stable for
G.F.

### C-PolyPair — polynomial pair + number-field constructor (compiler + KAT) — *frozen at G.B.1 (2f43f99)*
**Defined:** G.B.1. **Consumed by (in G.F):** G.F.2 (`m`, `n`), G.F.3 (`number_field()`, `f`), G.F.4
(`n` for the gcd). G.F **reads** the poly pair; it does not amend C-PolyPair. Stable for G.F.

### C-NF — number-field substrate: element arithmetic + ideals (compiler + KAT) — *frozen at G.A.3 (7844773); additively extended at G.F.1*
**Defined:** G.A.3. **Consumed by (in G.F):** G.F.1 (**adds `reduce_mod_ideal`**), G.F.3
(`reduce_mod_ideal`, `norm`, element `mul`). **G.F.1 is the first consumer to extend C-NF** — adding
`NumberFieldElement::reduce_mod_ideal(p, r)`. The extension is **additive** (a new method; no existing
`NumberFieldElement` signature changes), surfaced as an **additive-reshard** for sign-off at G.F.1,
not silently grown. The new method is **re-consumed by D.B** (F_ℓ residue fields), so freeze its
signature deliberately (over-specify toward a scalar-residue shape). *Watch:* if Couveignes turns out
to need *changing* an existing C-NF method's semantics (not adding), that is a **destructive-HALT**
(not expected — the survey confirms element arithmetic + ideals are sufficient as-is).

### C-Fp — prime-field substrate: `Fp<L>` trait with `legendre` + `sqrt` (compiler + KAT) — *frozen (α.5)*
**Defined:** α.5. **Consumed by (in G.F):** G.F.3 (`Fp::sqrt` per split prime, Tonelli–Shanks).
**Survey-confirmed present** — the ROADMAP-noted "Fp missing legendre/sqrt" gap is *closed*
(`legendre`/`sqrt` are landed default methods on the `Fp<L>` trait). G.F **reads** the trait; it does
not amend C-Fp. Stable for G.F.

### C-AlgSqrt — Couveignes algebraic-square-root contract (compiler + KAT) — *frozen at G.F.3 (design juncture)*

**Defined:** G.F.3. **Consumed by:** G.F.4 (Y for the assembly), **D.B (NFS-DL over F_ℓ generalises
the CRT square root — ROADMAP Phase γ)**. Like C-LinAlg, this is **cross-track-reused by D.B**, so
over-specified toward an F_ℓ-friendly shape (a general residue-field square-root view, not a
GF(2)-or-ℤ-hardcoded one) where the cost is low.

#### 1. Entry function signature

```rust
/// Compute the algebraic square root Y = Norm(β) mod N via Couveignes' CRT algorithm.
///
/// Given a kernel vector (a subset S of relations whose algebraic norm product is a
/// perfect square in K), computes β ∈ K with β² = γ = ∏_{i ∈ S}(a_i − b_i·α), then
/// returns Y = |Norm(β)| mod N.
///
/// # Algorithm (Couveignes)
///
/// 1. Form γ = ∏_{i ∈ S}(a_i − b_i·α) ∈ K via NumberFieldElement::mul.
/// 2. Select CRT primes: primes p that split completely in K, p > B_alg.
/// 3. For each split prime p with roots r_1, ..., r_d of f mod p:
///    - Reduce γ mod (p, α − r_j) to get γ_j ∈ 𝔽_p via reduce_mod_ideal.
///    - Compute β_j = sqrt(γ_j) in 𝔽_p via Fp::sqrt (Tonelli–Shanks).
///    - Combine the d roots β_1, ..., β_d into a single β mod p via Lagrange interpolation
///      (the unique polynomial of degree < d passing through (r_j, β_j)).
/// 4. CRT-lift the per-prime β mod p values to recover β's coefficients in ℤ[α].
/// 5. Resolve the global sign of β via the real embedding (see §4 below).
/// 6. Return Y = |Norm(β)| mod N.
///
/// # Panics
///
/// - If γ is not a quadratic residue mod any split prime (upstream kernel bug).
/// - If the CRT lift fails to converge (insufficient primes — scale bug).
///
/// # Parameters
///
/// - `kv`: The kernel vector (subset of filtered-matrix rows).
/// - `matrix`: The filtered sparse GF(2) matrix (carries provenance).
/// - `relations`: The original relation list.
/// - `poly`: The polynomial pair (provides f, m, n, and number_field()).
///
/// # Returns
///
/// Y = |Norm(β)| mod N as a `BigInt`.
pub fn algebraic_sqrt(
    kv: &KernelVector,
    matrix: &SparseMatrix,
    relations: &[Relation],
    poly: &PolyPair,
) -> BigInt
```

**D.B-friendly note:** The signature takes the kernel/relation/poly triple, not a pre-formed γ. D.B
may factor out a lower-level `couveignes_sqrt(gamma: &NumberFieldElement, nf: &NumberField, n:
&BigInt, num_primes: usize) -> BigInt` if the F_ℓ context provides γ differently. The G.F.3
implementation may internally use such a helper; the public entry is the above.

#### 2. Split-prime selection rule

**Reuse `select_qc_primes` from `gnfs::linalg::qc`.** The QC-prime machinery already finds primes
that split completely in K (i.e., f has d distinct roots mod p). The Couveignes primes are the same
class; the only difference is the count.

**Prime-count budget (principle-4 scale knob):**

- **Toy scale (demonstration fidelity):** 5–10 primes suffice. The coefficient bit-length of β is
  bounded by `|S| · max(log|a_i|, log|b_i|) · d`, which at toy scale (|S| ~ 50, coefficients ~ 20
  bits, d ~ 3–5) is ~3000 bits. Each 64-bit prime contributes ~64 bits of CRT information, so
  ~50 primes would be overkill; 5–10 is ample margin.
- **Scale annotation:** The prime count is the scale knob. The algorithm is identical at all scales;
  only the count changes. Annotate in code: `// Principle-4: prime_count is the scale knob; 10
  suffices at toy scale, O(coefficient_bits / 64) at NFS scale.`

**Selection:**

```rust
/// Select CRT primes for Couveignes' algorithm.
///
/// Returns `num_primes` primes p > b_alg that split completely in K (f has d distinct
/// roots mod p). Reuses the QC-prime selection logic.
fn select_couveignes_primes(f: &IntPoly, b_alg: u64, num_primes: usize) -> Vec<u64> {
    select_qc_primes(f, b_alg, num_primes)
}
```

**Default:** `const DEFAULT_COUVEIGNES_PRIMES: usize = 10;` (demonstration fidelity).

#### 3. CRT lift

**Per-prime square root step:**

For each split prime p with roots r_1, ..., r_d of f mod p:

1. **Reduce γ to 𝔽_p:** For each root r_j, call `gamma.reduce_mod_ideal(&p_big, &r_j_big)` to get
   `gamma_j: BigInt` in `[0, p)`.

2. **Convert BigInt → Uint<L>:** At toy scale, `L = 4` (256-bit) suffices for primes up to 64 bits.
   ```rust
   fn bigint_to_uint4(x: &BigInt, p: u64) -> Uint<4> {
       // x is in [0, p) where p < 2^64; extract the low 64 bits.
       use num_traits::ToPrimitive;
       let v = x.to_u64().expect("residue must fit in u64 at toy scale");
       Uint::<4>::from(v)
   }
   ```
   **D.B note:** At F_ℓ scale, L may need to be larger. The conversion is the limb-width seam;
   parameterise if D.B requires.

3. **Compute sqrt in 𝔽_p:** Call `FpNaive::<4>::from_uint(gamma_j_uint, &p_uint).sqrt(&p_uint)`.
   If `None`, panic — γ is not a QR mod p, indicating an upstream kernel bug (the QC columns should
   guarantee γ is a square in K, hence a QR mod every split prime).

4. **Lagrange interpolation mod p:** Given the d pairs `(r_j, beta_j)` where `beta_j = sqrt(gamma_j)`,
   reconstruct the unique polynomial `beta_poly` of degree < d such that `beta_poly(r_j) = beta_j`
   for all j. This is β mod p as a polynomial in α with coefficients in 𝔽_p.

   ```rust
   /// Lagrange interpolation mod p: given (r_j, beta_j) pairs, return the coefficients
   /// [c_0, c_1, ..., c_{d-1}] of the unique polynomial c_0 + c_1·x + ... + c_{d-1}·x^{d-1}
   /// passing through all points, reduced mod p.
   fn lagrange_interp_mod_p(points: &[(u64, u64)], p: u64) -> Vec<u64>
   ```

**CRT combination:**

After processing all primes, we have for each coefficient index k (0 ≤ k < d) a set of residues
`{c_k mod p_1, c_k mod p_2, ...}`. Apply the Chinese Remainder Theorem to recover `c_k` in ℤ.

**CRT algorithm:** Use Garner's algorithm (iterative, avoids large intermediate products):

```rust
/// CRT-lift a set of (residue, modulus) pairs to a single BigInt.
///
/// Given [(r_1, p_1), (r_2, p_2), ...], returns x such that x ≡ r_i (mod p_i) for all i,
/// with 0 ≤ x < ∏ p_i.
fn crt_lift(residues: &[(u64, u64)]) -> BigInt {
    // Garner's algorithm: iteratively lift.
    // x = r_1 + p_1 * (r_2 - r_1) * inv(p_1, p_2) + ...
}
```

**Representation of β:** After CRT, β is represented as a `Vec<BigInt>` of d coefficients
`[c_0, c_1, ..., c_{d-1}]` where β = c_0 + c_1·α + ... + c_{d-1}·α^{d-1}. Convert to
`NumberFieldElement` for norm computation:

```rust
let beta_poly = RatPoly::from_coeffs(coeffs.iter().map(|c| BigRational::from(c.clone())).collect());
let beta = NumberFieldElement { field: &nf, poly: beta_poly };
```

**Sign ambiguity:** The per-prime square roots have a ±1 ambiguity. Lagrange interpolation
propagates this: for each prime, we could have chosen −β_j instead of β_j. The CRT lift recovers
*some* β with β² = γ, but it may be −β (the other square root). This is resolved in §4.

#### 4. Embedding-sign convention (the silent-failure locus)

**The problem:** β and −β both satisfy β² = γ. The CRT lift recovers one of them, but we don't know
which. Choosing the wrong one gives Y = Norm(−β) = (−1)^d · Norm(β), which for odd d flips the sign
of Y. Even if |Y| is correct, the sign of Y mod N matters for the final gcd: we need X² ≡ Y² (mod N)
with X, Y having the same sign convention. A wrong sign yields a trivial gcd.

**The resolution:** Use the **real embedding** of K. Since f is the GNFS algebraic polynomial, it
has at least one real root θ (the polynomial arises from base-m expansion of N, which is positive,
so f has a real root near m^{1/d}). Evaluate β at θ to get a real number β(θ). Choose the sign of β
such that **β(θ) > 0**.

**Implementation:**

1. **Find a real root θ of f:** Use Newton's method starting from m^{1/d} (a good initial guess for
   base-m polynomials). At toy scale, f64 precision suffices; for robustness, use interval
   arithmetic or arbitrary-precision floats if needed.

   ```rust
   /// Find a real root of f using Newton's method.
   ///
   /// Starts from initial guess `x0` and iterates until |f(x)| < tol.
   /// Returns the root as an f64 (sufficient precision at toy scale).
   fn find_real_root(f: &IntPoly, x0: f64, tol: f64, max_iter: usize) -> Option<f64>
   ```

   **Initial guess:** `x0 = (poly.m as f64).powf(1.0 / d as f64)` where d = deg(f).

2. **Evaluate β at θ:** Given β = c_0 + c_1·α + ... + c_{d-1}·α^{d-1} and θ (a real root of f, so
   α ↦ θ is a valid embedding), compute β(θ) = c_0 + c_1·θ + ... + c_{d-1}·θ^{d-1}.

   ```rust
   /// Evaluate a polynomial (given as BigInt coefficients) at a real point.
   fn eval_at_real(coeffs: &[BigInt], theta: f64) -> f64 {
       coeffs.iter().enumerate().map(|(i, c)| {
           let c_f64 = c.to_f64().unwrap_or(0.0);
           c_f64 * theta.powi(i as i32)
       }).sum()
   }
   ```

3. **Sign correction:** If β(θ) < 0, negate β (i.e., negate all coefficients). This ensures the
   returned β has β(θ) > 0.

   ```rust
   let beta_at_theta = eval_at_real(&coeffs, theta);
   if beta_at_theta < 0.0 {
       for c in &mut coeffs {
           *c = -c.clone();
       }
   }
   ```

**Why this works:** The real embedding α ↦ θ is a ring homomorphism K → ℝ. For γ = β², we have
γ(θ) = β(θ)². Since γ is a product of (a_i − b_i·α) terms, γ(θ) = ∏(a_i − b_i·θ). The sign of γ(θ)
is determined by the count of negative factors, which the sign column in G.E tracks. By choosing
β(θ) > 0, we ensure β is the "positive" square root under the real embedding, consistent with the
rational side's sign convention (X > 0 from isqrt).

**Fallback (if Newton fails):** If f has no real root (possible for some polynomials, though rare
for GNFS base-m polynomials), fall back to the **norm-sign convention**: compute Norm(β) and
Norm(−β) = (−1)^d · Norm(β). Choose the one that is positive. This is less robust (it doesn't
guarantee consistency with the rational side) but is a fallback for edge cases.

**G.F.4 retry loop:** If the chosen sign is wrong, gcd(X − Y, N) will be trivial (1 or N). The G.F.4
assembly driver has a retry loop over kernel vectors; a trivial gcd from one vector is not fatal.
The sign convention here is a best-effort heuristic, not a guarantee. The retry loop is the safety
net.

#### 5. Per-prime square root step (BigInt → Fp bridge)

**The limb-width issue:** `reduce_mod_ideal` returns `BigInt` in `[0, p)`. `Fp::sqrt` requires
`Uint<L>`. At toy scale, primes are < 2^64, so `L = 4` (256-bit) is ample.

**Conversion:**

```rust
use crypto_bigint::Uint;
use num_traits::ToPrimitive;

/// Convert a BigInt residue in [0, p) to Uint<4> for Fp operations.
///
/// Panics if the residue does not fit in 64 bits (toy-scale assumption).
fn bigint_to_uint4(x: &BigInt) -> Uint<4> {
    let v = x.to_u64().expect("residue must fit in u64 at toy scale");
    Uint::<4>::from(v)
}

/// Convert a Uint<4> back to BigInt.
fn uint4_to_bigint(x: &Uint<4>) -> BigInt {
    // Extract the low 64 bits (sufficient at toy scale).
    BigInt::from(x.as_words()[0])
}
```

**D.B note:** At F_ℓ scale, primes may exceed 64 bits. The conversion would need to handle larger
`L` and multi-word extraction. Parameterise `L` if D.B requires; for G.F.3, hardcode `L = 4`.

**Sqrt call:**

```rust
use shared_field::{Fp, FpNaive};

let p_uint = Uint::<4>::from(p);
let gamma_j_uint = bigint_to_uint4(&gamma_j);
let gamma_j_fp = FpNaive::<4>::from_uint(gamma_j_uint, &p_uint);
let beta_j_fp = gamma_j_fp.sqrt(&p_uint).expect("gamma must be QR mod split prime");
let beta_j: u64 = beta_j_fp.to_uint().as_words()[0];
```

**Implementation note (harmless deviation from the above):** The G.F.3 build uses a standalone
`tonelli_shanks(n: u64, p: u64) -> Option<u64>` function instead of `FpNaive::<4>::sqrt`. The
algorithm is identical (Tonelli–Shanks); the interface is simpler: it operates directly on `u64`
values, avoiding the `BigInt → Uint<4>` conversion and the `FpNaive` wrapper entirely. This
sidesteps the `bigint_to_uint4` bridge and the `shared_field` import for the per-prime step. The
`FpNaive::<4>::sqrt` path above remains the D.B-friendly shape (it generalises to larger `L` and
to the `Fp<L>` trait); the standalone function is the pragmatic choice at G.F.3 toy scale. The
frozen C-AlgSqrt summary row below reflects the actual implementation.

#### Summary of frozen interface

| Component | Specification |
|-----------|---------------|
| **Entry signature** | `algebraic_sqrt(kv, matrix, relations, poly) -> BigInt` returning Y = \|Norm(β)\| mod N |
| **Prime selection** | Reuse `select_qc_primes`; default 10 primes (principle-4 scale knob) |
| **Per-prime step** | `reduce_mod_ideal` → standalone `tonelli_shanks(n: u64, p: u64)` → Lagrange interpolation (deviation from `FpNaive::<4>::sqrt` — same algorithm, simpler interface; see §5 note) |
| **CRT lift** | Garner's algorithm per coefficient; result is `Vec<BigInt>` → `NumberFieldElement` |
| **Sign convention** | Real embedding: find θ (real root of f), evaluate β(θ), negate if < 0 |
| **Norm computation** | `NumberFieldElement::norm()` → `BigRational` → extract numerator → reduce mod N |
| **Limb width** | `L = 4` (256-bit) at toy scale; parameterise for D.B if needed |

**Verification gate:** The G.F.3 KAT (a) tests that for a known-square γ, Couveignes recovers β with
Norm(β) matching the hand-computed Y. The G.F.4 end-to-end KAT is the behavioural gate.

---

## Progress ledger

`/run-plan` updates this table; status ∈ {pending, done}. Commit-hash recorded on completion.
"Froze" names contracts this session locked. The two G.F.3 junctures are not ledger rows (paged forks
with no commit-shaped deliverable); their outcomes are recorded in the Action-frame digest.

| # | Session | Status | Commit | Froze |
|---|---------|--------|--------|-------|
| G.F.1 | Square-root substrate: reduce_mod_ideal + isqrt + exported gcd | done | 2af8116 | C-NF additively extended (reduce_mod_ideal frozen); isqrt + gcd added to shared/bigint. Extra files: isqrt.rs (submodule), Cargo.toml updates, Cargo.lock (plainly part of unit). |
| G.F.2 | Rational square root from a kernel vector | done | 11f9065 | gnfs::sqrt module established (mod.rs + rational.rs). Extra file: gnfs/Cargo.toml (added shared-bigint dep, plainly part of unit). |
| G.F.3 | Algebraic square root via Couveignes / CRT | done | c80a855 + ec69a1f | C-AlgSqrt frozen. Extra files: gnfs/Cargo.toml (num-integer, num-rational deps), gnfs/src/lib.rs (re-export), Cargo.lock (plainly part of unit). Review fix: unconditional sign-consistency check (ec69a1f). |
| G.F.4 | Assembly + end-to-end factor driver (gcd(x−y, N)) | done | 7f80040 | First top-level factor driver. Extra file: gnfs/src/lib.rs (re-exports, plainly part of unit). |
| G.F.W | Integrative writeup (square-root chapter) | pending | — | — |

Contracts frozen before G.F: C-Fp (cf00ed5 / α.5), C-numth (α.2), C-NF (bdba6f5 / 7844773), C-Ideal
(05b27c8), C-Res (bcd63cd), C-Dedekind (7844773), C-PolyPair (2f43f99), C-Score (00aa32d),
C-FactorBase (c1dc0b6), C-Relation (c1dc0b6), C-Matrix (a0e854b), C-LinAlg (416f6db). G.F opens over
the frozen G.A substrate, G.B polynomial-selection layer, G.C sieve layer, G.D filter layer, and G.E
linear-algebra layer.

---

## Action-frame digest

The externalized action frame: appended on non-trivial iterations (discoveries, contract flexes,
notable texture) for the juncture forks to consume — including the **G.F.3 design-juncture outcome**
(the frozen C-AlgSqrt) and the **G.F.3 review-juncture outcome** (the T0 fork's findings on
Couveignes), which are recorded here rather than in the ledger.

### G.F.3 design juncture — 2026-06-07
Discovery/flex: G.F.3 design inflection fork returned `design-confident`; C-AlgSqrt frozen with: entry `algebraic_sqrt(kv, matrix, relations, poly) -> BigInt` (Y = |Norm(β)| mod N); prime selection reuses `select_qc_primes` (default 10, principle-4 scale knob); per-prime step: `reduce_mod_ideal` → `bigint_to_uint4` → `FpNaive::<4>::sqrt` → Lagrange interpolation; CRT lift via Garner's algorithm per coefficient; sign convention: real embedding (find θ via Newton from m^{1/d}, evaluate β(θ), negate if < 0); fallback: norm-sign convention if Newton fails.
Affected: C-AlgSqrt (frozen by design juncture, written into Cross-session contracts).
Deferred: no — all five design decisions resolved; D.B generalisation paths documented (L parameterisation, lower-level couveignes_sqrt helper).
Texture: The silent-failure locus (embedding-sign) is handled by real-embedding with G.F.4 retry loop as safety net. Lagrange interpolation per prime is the key step connecting reduce_mod_ideal output to CRT input.

### G.F.3 review juncture — 2026-06-07
Discovery/flex: T0 correctness-review juncture returned `review-needs-discussion` (no silent-failure bugs found). Two items: (1) per-prime sign consistency check was debug-only — fixed to unconditional assert (ec69a1f); (2) standalone `tonelli_shanks` used instead of `FpNaive::<4>::sqrt` — harmless deviation (same algorithm), C-AlgSqrt §5 updated with implementation note. Embedding-sign resolution confirmed correct. Prime budget confirmed sufficient at toy scale. Principle-4 annotations confirmed present and honest.
Affected: C-AlgSqrt §5 (implementation note added); gnfs/src/sqrt/algebraic.rs (unconditional check).
Deferred: no — both items resolved; G.F.4 may proceed.
Texture: The per-prime sign consistency is now enforced unconditionally (panics on upstream kernel bugs in both debug and release). The G.F.4 retry loop remains the safety net for wrong-sign β from the global sign resolution.

---

## Discoveries & risks

Phrased as `/run-plan` reads for discovery adjudication (internal-continue / additive-reshard /
destructive-HALT).

- **Roadmap-frame flex: G.F is 4 algorithmic sessions + Opus junctures, not "2 sessions Sonnet"
  (surface at the G.F ◆ boundary for the ROADMAP Discoveries log).** The ROADMAP scoped G.F before the
  substrate survey, which revealed three absent capabilities (`reduce_mod_ideal`, `isqrt`, exported
  `gcd`) and two distinct square roots. The one-line-commit-title corollary forces the split; lever
  4 (terminal silent-failure seam) + lever 3 (new D.B-reused cross-crate API) force the two Opus
  junctures at G.F.3. This is an **additive** roadmap flex (re-tier + split, no contract break) — log
  it at the boundary; do not treat the ROADMAP's "2 sessions Sonnet" as binding given the new
  evidence.

- **`reduce_mod_ideal` extends frozen C-NF — additive-reshard, surface don't grow silently (lever 3,
  cross-track into D.B).** G.F.1 adds a method to the frozen `NumberFieldElement` type. The C-NF
  element arithmetic + ideal machinery is sufficient as-is (survey-confirmed); the new method is
  purely additive (no existing signature changes). It is **re-consumed by D.B** (F_ℓ residue fields),
  so freeze the signature deliberately. **Additive-reshard** (surface at G.F.1). Only if Couveignes
  requires *changing* existing C-NF semantics is it a **destructive-HALT** (not expected).

- **The algebraic square root is the terminal silent-failure seam (lever 4) — why the G.F.3 review
  juncture is load-bearing.** Unlike G.D filtering (a bug surfaces as a wrong matrix dimension), a
  Couveignes bug — most acutely a wrong **embedding-sign** for β — returns a plausible Y that produces
  a *trivial* gcd (1 or N) at G.F.4: a silent non-factorization, not a red test. The deterministic
  end-to-end KAT (G.F.4 KAT a) is the behavioural gate; when the CADO/msieve oracle is absent, the
  **G.F.3 T0 review juncture** is the compensating control. This is why lever 4 stays high (holding
  juncture-tier at Opus) and why the FLOOR session earns *both* a design and a review juncture.

- **C-AlgSqrt is the second cross-track-reused Track-G contract (D.B generalises it, after C-LinAlg).**
  Like C-LinAlg, the Couveignes CRT square root generalises to D.B (NFS-DL over F_ℓ: the residue-field
  square root becomes a residue-field discrete-log-adjacent step). Re-forking it at D.B would be a
  destructive reshard one track over; designing the algebraic-square-root interface as a general
  residue-field view where the cost is low is the over-specify rule applied to a cross-track contract.
  If G.F.3 finds the ℤ-specific shortcuts too entangled to generalise cheaply, *surface it* (additive
  note for D.A/D.B), do not silently hardcode.

- **The end-to-end KAT's 80–100-bit target may stress the `Uint<4>` limb width (ROADMAP α Discovery).**
  ROADMAP α flagged C1/`shared::numth` as hardcoded to `Uint<4>` (256-bit), "sufficient for toy-scale
  NFS." G.F.4's published-challenge KAT (80–100 bits) is within 256 bits, so `Uint<4>` should suffice
  — but if norm products or CRT lifts overflow it, widening to `Uint<L>` is the **additive-reshard**
  the ROADMAP α Discovery already sanctioned ("mechanical changes that can be done in-place"). Surface
  if it surfaces; do not silently grow.

- **Reference-oracle availability (CADO-NFS / msieve for G.F).** The end-to-end "recover the same
  factor" KAT (G.F.4 KAT c) depends on CADO-NFS / msieve as dev oracles (ROADMAP reference-oracle
  entry names **msieve** specifically as a G.F square-root oracle; neither is yet present). Absent →
  gate behind the established `#[ignore = "CADO-NFS not installed; ..."]` pattern (already used in
  `lanczos_kat.rs`, `merge_kat.rs`, `line_sieve_kat.rs`); the **deterministic** factor KAT (G.F.4 KAT
  a) carries reproducibility without an oracle. The project-wide oracle-gating policy is still open
  (ROADMAP Discoveries, reference-oracle entry) — G.F inherits the per-test `#[ignore]` workaround; it
  does not resolve the policy. No new msieve dependency is introduced.

- **No end-to-end driver exists yet — G.F.4 stands up the first pipeline chain (lever 1).** The survey
  confirmed the pipeline stages are wired only in per-stage KATs, never end-to-end. G.F.4 builds the
  first "factor N" driver. Keep its surface minimal (a function that takes a `PolyPair` + matrix +
  relations and returns a factor, with the kernel-vector retry loop) — it is integrative glue, not a
  new frozen contract; G.W (the GNFS-wide writeup) may later articulate a cleaner pipeline API.

- **CRT-prime-budget and embedding-sign payoffs at toy scale (principle 4).** The number of CRT primes
  needed to pin β is tiny at toy scale (a handful), and the embedding-sign resolution is *correctness*,
  not scale — present even at toy scale. Implement at demonstration fidelity and annotate the
  prime-budget scale knob in code + G.F.W + Track τ. (Distinct from the sign resolution, which is a
  correctness obligation, not a scale artifact.)

- **`sqrt/` module is fresh ambient surface (lever 1).** Fourth non-`polyselect`/`sieve` module in
  `gnfs` (after `filter/`, `linalg/`); G.F.2 sets `src/sqrt/` layout. Keep the crate-internal module
  structure open rather than over-committing at G.F.2.

- **Documentation math-rendering format (G.F.W is heavy display math).** Recommended at the G.C ◆
  boundary: **Markdown + MathJax** (`$…$` / `$$…$$`), to be ratified at T.0 (ROADMAP Discoveries log).
  G.F.W (CRT congruences, the $\beta^2 = \prod(a - b\alpha)$ identity, the embedding map) uses MathJax
  `$$…$$`; `$$` is already established in PEDAGOGY.md (from §1429 and the G.E chapter). Not a code
  contract — gates no G.F implementation session, only G.F.W's prose.

---

## Notes for executors

- Read `docs/ROADMAP.md` (Phase β / Track G: G.F entry "Couveignes' algorithm via Chinese
  remaindering; final integer GCD; end-to-end KAT factor an 80–100-bit challenge"; the
  reference-oracle Discoveries entry naming msieve for G.F; the documentation-format entry) and the
  prior `docs/PLAN.md` G.E history before any G.F session. **Note the roadmap-frame flex** (G.F is 4
  sessions + Opus junctures, not the ROADMAP's "2 sessions Sonnet" — logged in Discoveries above).
- Read `gnfs/docs/PEDAGOGY.md` (the G.E linear-algebra chapter §31–§41, especially §32 "the congruence
  of squares" and §33 "why the sign and QC columns are needed" — they forward-reference G.F's two
  square roots; and the "What G.F consumes" passage on `expand_provenance`) and
  `shared/numfield/docs/PEDAGOGY.md` (the G.A.W chapter, for the number-field element + ideal register
  G.F.1's `reduce_mod_ideal` extends). New `gnfs::sqrt` and the C-NF extension match the genre.
- **Register: PEDAGOGY.** This is a reference library — code is teaching material. Match the G.D.W /
  G.E.W chapter genre and quality.
- **Tier routing:** G.F.1, G.F.2 are Sonnet (`@build`). **G.F.3 carries two `@plan` markers:** a
  T0/Opus **design** juncture *before* it implements (page `@plan-juncture` to freeze C-AlgSqrt — the
  Couveignes contract, the embedding-sign convention) and a T0/Opus **correctness-review** juncture
  *after* it lands (page `@plan-juncture` to review the CRT square root before G.F.4). G.F.3's *build*
  is Sonnet between the two forks. G.F.4 and G.F.W are Sonnet (`@build`). The two Opus touchpoints are
  the G.F.3 design fork and the G.F.3 review fork; the ROADMAP did not pre-flag them (the flex is
  logged in Discoveries).
- **Invariants to preserve:** the G.A substrate contracts (C-NF, C-Res, C-numth, C-Ideal, C-Dedekind),
  the G.B contracts (C-PolyPair, C-Score), the G.C contracts (C-Relation, C-FactorBase), the G.D
  contract (C-Matrix), and the **G.E contract (C-LinAlg)** are frozen — G.F consumes them. The
  sanctioned reaches are the **additive `reduce_mod_ideal` method on C-NF** and the **additive `isqrt`
  / exported `gcd` in `shared/bigint`** at G.F.1 (surface as additive-reshard). The `rho` crate,
  `gnfs::polyselect`, `gnfs::sieve`, `gnfs::filter`, and `gnfs::linalg` stay otherwise untouched (G.F
  *adds* `gnfs::sqrt` and extends `shared/numfield` + `shared/bigint` additively).
- **CADO-NFS / msieve are dev-only oracles**, never on a build path (ROADMAP scoping principle 3).
  Gate oracle KATs with the established `#[ignore]` pattern. msieve is named as the G.F square-root
  oracle but is not present — no new dependency is introduced; the deterministic factor KAT suffices.
- **Doc-format note (for G.F.W):** new display mathematics uses MathJax (`$$…$$`); inline-Unicode may
  stay. G.F.W is a display-math chapter (CRT, the square-root identity, the embedding). Recommendation
  pending T.0 ratification (ROADMAP Discoveries log).
- Suggested first invocation: **`/run-plan docs/PLAN.md halt-at-boundaries`** — the `sqrt/`-module
  shard pattern is unproven for this crate (first sqrt module; C-AlgSqrt is a new cross-track contract;
  G.F.1 is the first session to extend C-NF and `shared/bigint`; G.F.4 is the first end-to-end driver).
  With the two G.F.3 markers this halts **three times**: the **G.F.3 design inflection** (freeze
  C-AlgSqrt *before* implementation), the **G.F.3 correctness-review juncture** (review Couveignes
  *after* it lands, before G.F.4), and the **G.F.W ◆ boundary**.
