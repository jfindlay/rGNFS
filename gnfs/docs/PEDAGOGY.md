# Polynomial Selection: A Code-Tour Chapter

This chapter explains the `gnfs/src/polyselect` module — the polynomial-selection stage of the
General Number Field Sieve. It is organised by the implementation, not by mathematical abstraction:
each section introduces the mathematics, then shows how it is realised in code. A reader who has
read the `shared/numfield` PEDAGOGY chapter can read this one without consulting the source.

The chapter covers the base-m construction, degree selection, Murphy-E scoring, the root sieve,
Coppersmith multi-poly, and the public contracts that downstream stages consume.

---

## 1. Introduction: Why Polynomial Selection Matters

The General Number Field Sieve factors a large integer N by finding many pairs (a, b) ∈ ℤ² such
that both the rational norm G(a, b) = a − bm and the algebraic norm F(a, b) = b^d · f(a/b) are
B-smooth — divisible only by primes up to a bound B. When enough such pairs are collected, linear
algebra over 𝔽₂ produces a congruence of squares mod N, and hence a non-trivial factor.

The key insight is that the smoothness probability of F(a, b) depends heavily on the polynomial f.
A polynomial with small coefficients, many real roots, and many roots modulo small primes produces
smaller norms on average, which are more likely to be smooth. Polynomial selection is the stage
that finds such an f before the sieve begins.

The output of polynomial selection is a **polynomial pair** (f, g): the algebraic-side polynomial
f ∈ ℤ[x] of degree d ≥ 2, and the rational-side polynomial g(x) = x − m, where m is a shared
root mod N:

```text
f(m) ≡ 0 (mod N)
g(m) = 0,  g(x) = x − m
```

This shared-root invariant is the load-bearing contract that the sieve consumes. The sieve does not
care how f was found — only that f(m) ≡ 0 (mod N) and that f has small norms over the sieve region.

---

## 2. The `PolyPair` Contract (C-PolyPair)

The central type in this module is `PolyPair`, defined in `gnfs/src/polyselect/mod.rs`. It carries
the polynomial pair and all metadata that downstream stages need:

```rust
pub struct PolyPair {
    /// Algebraic-side polynomial f ∈ ℤ[x]. Generally non-monic for base-m; stored as-is.
    pub f: IntPoly,
    /// Rational-side polynomial g = x − m ∈ ℤ[x].
    pub g: IntPoly,
    /// The shared root: f(m) ≡ 0 (mod n) and g(m) = 0.
    pub m: BigInt,
    /// The integer to factor.
    pub n: BigInt,
    /// Polynomial degree (redundant with f.degree() but convenient for pattern matching).
    pub degree: usize,
    /// Skew parameter s: balances algebraic and rational norm sizes.
    /// None until Murphy-E scoring (G.B.2) computes it.
    pub skew: Option<f64>,
    /// Factor-base bounds (rational_bound, algebraic_bound).
    /// None until sieving (G.C) sets them.
    pub factor_base_bounds: Option<(u64, u64)>,
}
```

The invariants are checked by `PolyPair::verify()`:

1. `f` has degree ≥ 1 (not zero or constant).
2. `f.degree() == Some(self.degree)` — the `degree` field is consistent with the polynomial.
3. `g = x − m` — the rational polynomial is always the linear form `x − m`.
4. `f(m) ≡ 0 (mod n)` — the algebraic polynomial has m as a root mod n.

```rust
let pair = select_base_m(&n, 3);
assert_eq!(pair.verify(), Ok(()));
```

The `skew` and `factor_base_bounds` fields are `None` at construction and populated by later
pipeline stages. This is the substrate-over-specifies pattern: adding them now is cheaper than
amending the contract after G.C consumes it.

### The non-monic seam

Base-m expansion (§3) produces a polynomial f whose leading coefficient a_d satisfies 1 ≤ a_d < m,
so f is generally non-monic. The `shared-numfield` crate's `NumberField::new` requires a monic
defining polynomial. `PolyPair` resolves this seam with two methods:

```rust
// Return the monic form of f via the standard NFS homogenisation:
// f_monic(x) = a_d^{d−1} · f(x / a_d)
pub fn monic_f(&self) -> IntPoly;

// Construct the number field K = ℚ(α) where α is a root of the monic form of f.
pub fn number_field(&self) -> NumberField;
```

The homogenisation `f(x) → a_d^{d−1} f(x/a_d)` is monic and has roots a_d · αᵢ where αᵢ are the
roots of f. The coefficient of x^k in f_monic is a_k · a_d^{d−1−k}, which is always an integer.

The design decision is to store the original non-monic f and expose the monic form on demand:

- **Sieving uses the original f.** Norm computation in G.C uses the original polynomial
  coefficients; the monic transformation would change the norms.
- **The transformation is internal to `number_field()`.** Consumers that need `NumberField`
  element arithmetic call `poly_pair.number_field()`; consumers that need the original f
  (sieving, scoring) access `poly_pair.f` directly.

---

## 3. Base-m Construction

### The algorithm

The simplest NFS polynomial generator is **base-m expansion**, implemented in
`gnfs/src/polyselect/base_m.rs`. Given N and degree d:

1. Choose m = ⌊N^{1/(d+1)}⌋ + 1 — the smallest integer with m^{d+1} > N.
2. Write N in base m: N = a_0 + a_1·m + a_2·m² + … + a_d·m^d.
3. Define f(x) = Σ aᵢ xⁱ.

The key invariant is immediate: f(m) = N ≡ 0 (mod N). The rational side is g(x) = x − m.

```rust
pub fn select_base_m(n: &BigInt, degree: usize) -> PolyPair {
    let m = base_m_for_degree(n, degree);
    select_base_m_with_m(n, &m, degree)
}
```

The `base_m_for_degree` function computes m using Newton's method in `BigInt` arithmetic to find
⌊N^{1/(d+1)}⌋, then adds 1. The `base_m_digits` function performs the base-m expansion by
repeated division:

```rust
fn base_m_digits(n: &BigInt, m: &BigInt, d: usize) -> Vec<BigInt> {
    let mut coeffs = Vec::with_capacity(d + 1);
    let mut remainder = n.clone();
    for _ in 0..=d {
        let digit = &remainder % m;
        coeffs.push(digit.clone());
        remainder = (remainder - digit) / m;
    }
    coeffs
}
```

The resulting coefficient vector has exactly d + 1 entries (indices 0 through d), with the
constant term first (least-significant coefficient first, matching `IntPoly`'s storage convention).

### Why f(m) = N exactly

The base-m construction gives f(m) = N exactly, not just f(m) ≡ 0 (mod N). This is stronger than
required — the sieve only needs the congruence — but it is the natural consequence of writing N in
base m. The KAT confirms this:

```rust
let pair = select_base_m(&n, 3);
assert_eq!(pair.f.eval(&pair.m), n);  // exact equality, not just mod n
```

### The non-monic leading coefficient

The leading digit a_d satisfies 1 ≤ a_d < m (since N < m^{d+1} by construction of m). So f is
non-monic in general. For example, for N = 1022117 = 1009 × 1013 and d = 3, the base-m expansion
gives m = 101 and f(x) = c₀ + c₁x + c₂x² + c₃x³ with c₃ < 101. The `monic_f()` method
(§2) handles the conversion when needed.

---

## 4. Degree Selection

### The L-notation balance

The optimal degree d for NFS polynomial selection is determined by balancing the smoothness bounds
on the algebraic and rational sides. The classical heuristic derives from the L-notation complexity
of NFS:

> The NFS running time is L_N[1/3, c] = exp((c + o(1)) · (ln N)^{1/3} · (ln ln N)^{2/3})

Balancing the algebraic and rational sides gives the condition:

> d ≈ (3 ln N / ln ln N)^{1/3}

This is implemented in `optimal_degree`:

```rust
pub fn optimal_degree(n: &BigInt) -> usize {
    let bits = n.bits() as f64;
    let ln_n = bits * std::f64::consts::LN_2;
    let ln_ln_n = ln_n.ln();
    let d_float = (3.0 * ln_n / ln_ln_n).cbrt();
    let d = d_float.round() as usize;
    d.clamp(3, 6)
}
```

The bit length approximates ln N ≈ bits · ln 2. The result is clamped to [3, 6] for toy-scale N.

### Why degree 5 for RSA-100

For RSA-100 (330-bit, 100 decimal digits), the heuristic gives d ≈ 5. The KAT confirms this:

```rust
let n_rsa100 = BigInt::from_str(RSA_100_STR).unwrap();
assert_eq!(optimal_degree(&n_rsa100), 5);
```

For toy N (60–100 bit), the heuristic gives d = 3 or 4. The clamping to [3, 6] ensures that
even for very small N (where the formula might suggest d = 2), the degree is at least 3 — the
minimum that makes the NFS algebraic side non-trivial.

### The degree-smoothness tradeoff

Higher degree means:
- Larger algebraic norms (the coefficients of f grow as N^{1/d}, so larger d means smaller
  coefficients, but the norm F(a, b) = b^d · f(a/b) grows with d for fixed sieve region).
- More roots mod small primes (a degree-d polynomial can have up to d roots mod p, giving more
  factor-base elements).

The L-notation balance is the point where these effects cancel. At toy scale, the balance is
approximate and the degree choice matters less than at cryptographic scale.

---

## 5. Murphy-E Scoring

### What Murphy-E predicts

Murphy-E is the standard heuristic score for NFS polynomial pairs, implemented in
`gnfs/src/polyselect/murphy.rs`. It predicts the density of smooth relations produced by the
sieve. A higher score means more smooth pairs (a, b) in the sieve region, which means fewer
sieve pairs needed to collect enough relations for the linear algebra step.

The score is defined as the average over a sample of (a, b) pairs in the sieve region of the
product of Dickman-ρ values for the algebraic and rational norms:

```text
E(f, g) ≈ (1/|S|) Σ_{(a,b) ∈ S} ρ(log|F(a,b)| / log B_f) · ρ(log|G(a,b)| / log B_g)
```

where:
- F(a, b) = b^d · f(a/b) is the algebraic norm (homogeneous form of f).
- G(a, b) = a − bm is the rational norm.
- B_f, B_g are the algebraic and rational factor-base bounds.
- ρ is the Dickman rho function.

The implementation samples a 50×50 grid over the sieve region [-M, M] × [1, M] with M = 1000:

```rust
pub fn score(pair: &PolyPair) -> f64 {
    const M: f64 = 1000.0;
    const GRID: usize = 50;
    let log_bf = (1_000_000.0_f64).ln();
    let log_bg = (1_000_000.0_f64).ln();
    // ... sample grid, compute rho products, average ...
}
```

### The Dickman ρ function

The Dickman rho function ρ(u) is the probability that a random integer near x is x^{1/u}-smooth
(has no prime factor larger than x^{1/u}). It satisfies:

- ρ(u) = 1 for u ≤ 1.
- ρ(u) = 1 − ln u for 1 < u ≤ 2.
- For u > 2: the delay-differential equation u·ρ'(u) = −ρ(u−1).

The implementation uses the exact formula for u ≤ 2 and 4th-order Runge-Kutta numerical
integration of the recurrence for 2 < u ≤ 25, with ρ(u) = 0 for u > 25:

```rust
pub fn dickman_rho(u: f64) -> f64 {
    if u <= 1.0 { return 1.0; }
    if u <= 2.0 { return 1.0 - u.ln(); }
    if u > 25.0 { return 0.0; }
    dickman_rho_numerical(u)
}
```

The numerical integrator builds a table of 2301 entries (step h = 0.01) from u = 2 to u = 25,
using the one-unit lookback ρ(t−1) at each step. The full table is allocated as a `Vec` to avoid
the correctness hazard of a rolling window that would overwrite values still needed for the
lookback.

Key known values (verified by KATs):
- ρ(1.5) = 1 − ln(1.5) ≈ 0.5945
- ρ(2.0) = 1 − ln(2) ≈ 0.3069
- ρ(3.0) ≈ 0.04861 (from tables)
- ρ(10.0) ≈ 2.77 × 10⁻¹⁰ (essentially zero)

### The skew parameter

The optimal skew s balances algebraic and rational norm sizes. The score function applies the
skew by sampling (a, b·s) instead of (a, b), which rescales the algebraic norm relative to the
rational norm. The skew stored in `pair.skew` is used if present; otherwise it defaults to 1.0.

### Science↔engineering disconnect at toy scale (principle-4 annotation)

**Murphy-E's predictive value — that higher E implies more relations — only manifests at sieve
scale (N ≳ 2^100, sieve region ≳ 10^6 pairs).** At toy scale (N < 2^60), the sieve region is
tiny and the smoothness probabilities are dominated by the factor-base bounds rather than the
polynomial shape. At toy scale, Murphy-E is a *ranking heuristic* whose payoff is under-exposed:
the ordering it induces is correct in expectation, but the absolute values are not meaningful.

This is why the KATs test ordering and self-consistency rather than absolute values:

```rust
// KAT 1: smaller coefficients → higher score (ordering is correct)
assert!(score_better > score_worse);

// KAT 2: scaling f by 10 decreases the score (monotonicity)
assert!(score_1x > score_10x);

// KAT 3: score is always positive (positivity)
assert!(score(&pair) > 0.0);
```

Downstream consumers (root sieve, Coppersmith, D.A NFS-DL poly selection) should treat the score
as an ordinal ranking, not a cardinal measure.

---

## 6. Root Sieve

### The Kleinjung rotation

The root sieve, implemented in `gnfs/src/polyselect/root_sieve.rs`, searches for a better
polynomial by applying **Kleinjung rotation** to a seed pair. The rotation is:

```text
f'(x) = f(x) + (j·x + k)·g(x)
g'(x) = g(x)   (unchanged)
```

This preserves the shared root m mod N because g(m) = 0:

```text
f'(m) = f(m) + (j·m + k)·g(m) = 0 + (j·m + k)·0 = 0  (mod N)
```

So `PolyPair::verify()` holds for every rotated pair. The rotation is implemented in `rotate`:

```rust
pub fn rotate(seed: &PolyPair, j: i64, k: i64) -> PolyPair {
    let m = &seed.m;
    let mut coeffs: Vec<BigInt> = seed.f.coeffs.clone();
    while coeffs.len() < 3 { coeffs.push(BigInt::from(0i64)); }

    let j_big = BigInt::from(j);
    let k_big = BigInt::from(k);

    // (j·x + k)·(x − m) = j·x² + (k − j·m)·x − k·m
    coeffs[0] -= &k_big * m;       // f'[0] += −k·m
    coeffs[1] += &k_big - &j_big * m;  // f'[1] += k − j·m
    coeffs[2] += &j_big;           // f'[2] += j

    let f_prime = IntPoly::from_coeffs(coeffs);
    let g = IntPoly::from_coeffs(vec![-m.clone(), BigInt::one()]);
    PolyPair::new(f_prime, g, m.clone(), n.clone())
}
```

For g(x) = x − m, the rotation adds a degree-2 perturbation to f. Since base-m always gives
d ≥ 3, the degree of f is unchanged by the rotation (the j contribution lands at index 2, below
the leading term at index d ≥ 3).

### The search strategy

`root_sieve` searches over a grid of (j, k) values, scoring each candidate with Murphy-E and
returning the best:

```rust
pub fn root_sieve(seed: &PolyPair, config: &RootSieveConfig) -> PolyPair {
    let mut best_pair = rotate(seed, 0, 0);  // seed itself
    let mut best_score = score(&best_pair);

    for j in -config.j_range..=config.j_range {
        for k in -config.k_range..=config.k_range {
            if j == 0 && k == 0 { continue; }
            let candidate = rotate(seed, j, k);
            let s = score(&candidate);
            if s > best_score {
                best_score = s;
                best_pair = candidate;
            }
        }
    }
    best_pair
}
```

The default `RootSieveConfig` uses `j_range = k_range = 10`, giving 441 candidates. The seed
itself (at j = 0, k = 0) is always included, so the returned pair is always at least as good as
the seed. The search is deterministic: the grid is traversed in row-major order and ties are
broken by keeping the first maximum found.

### The `RootSieveGenerator` trait implementation

`RootSieveGenerator` implements the `PolyGenerator` trait, yielding all candidates in the grid
rather than just the best:

```rust
pub struct RootSieveGenerator {
    pub seed: PolyPair,
    pub config: RootSieveConfig,
}

impl PolyGenerator for RootSieveGenerator {
    fn generate(&self) -> impl Iterator<Item = PolyPair> {
        // Yields all (2·j_range + 1) × (2·k_range + 1) candidates in row-major order.
        // ...
    }
}
```

This fits into the common score-and-rank pipeline alongside `BaseMGenerator` and
`CoppersmithGenerator`. Callers can use `.max_by(...)` to recover the best, or `.take(limit)` for
early termination.

### Why rotation preserves the root: the algebraic argument

The root-preservation property is not just a numerical coincidence — it is an algebraic identity.
For any polynomial h(x) ∈ ℤ[x]:

```text
f'(m) = f(m) + h(m)·g(m) = 0 + h(m)·0 = 0  (mod N)
```

The Kleinjung rotation uses h(x) = j·x + k, but any h would work. The rotation is a change of
basis in the space of polynomials with root m mod N: the set {f + h·g : h ∈ ℤ[x]} is the coset
of g in ℤ[x] that contains f. The root sieve searches this coset for the element with the best
Murphy-E score.

### Science↔engineering note

At toy scale (N < 2^60), the Murphy-E improvement from rotation is small and may not manifest as
a strict improvement for every seed. The KAT uses `≥` rather than `>`:

```rust
// KAT 1: root_sieve returns score ≥ seed score (not necessarily strictly better)
assert!(best_score >= seed_score);
```

At cryptographic scale (RSA-768+), rotation search is essential and produces measurable
improvements in relation yield.

---

## 7. Coppersmith Multi-Poly

### The mathematical construction

Coppersmith's multiple-polynomial method, implemented in `gnfs/src/polyselect/coppersmith.rs`,
generates several algebraic-side polynomials f₁, f₂, …, f_k that all share the same rational-side
polynomial g = x − m. Each variant is produced by a Kleinjung rotation:

```text
f_i(x) = f_0(x) + (j_i · x + k_i) · g(x)
```

The rotation parameters (j_i, k_i) are chosen by a symmetric spiral schedule:

```text
i = 0: (j=0,  k=0)          — the seed itself
i = 1: (j=0,  k=+step)      — translate by +step
i = 2: (j=0,  k=-step)      — translate by -step
i = 3: (j=+step, k=0)       — rotate by +step
i = 4: (j=-step, k=0)       — rotate by -step
i = 5: (j=0,  k=+2*step)    — translate by +2*step
...
```

The spiral keeps the parameters small and symmetric, exploring the neighbourhood of the seed in
a balanced way. The implementation:

```rust
pub fn coppersmith_polys(seed: &PolyPair, config: &CoppersmithConfig) -> Vec<PolyPair> {
    (0..config.num_polys)
        .map(|i| {
            let (j, k) = rotation_params(i, config.step);
            rotate(seed, j, k)
        })
        .collect()
}

pub fn coppersmith_best(seed: &PolyPair, config: &CoppersmithConfig) -> PolyPair {
    coppersmith_polys(seed, config)
        .into_iter()
        .max_by(|a, b| score(a).partial_cmp(&score(b)).expect("Murphy-E score is finite"))
        .expect("coppersmith_polys returns at least one pair when num_polys >= 1")
}
```

### Demonstration-fidelity role (principle-4 annotation)

**In production NFS (e.g., RSA-768), using different polynomials for different sieve regions
measurably improves the relation yield.** The idea is that different parts of the sieve region
may be better served by different polynomials — a polynomial that is good on average may be poor
in some sub-region, and a different rotation may be better there.

**At toy scale (60–100 bit N), the sieve region is too small for this effect to manifest.** The
Murphy-E improvement from multi-poly is typically small (< 2×), and the absolute score values are
not meaningful. This is a **demonstration-fidelity** implementation: the mathematical content is
complete, but the engineering payoff requires cryptographic-scale N to observe.

The KAT documents this explicitly:

```rust
// KAT 5: principle-4 annotation — improvement is < 2× at toy scale
let improvement = best_score / seed_score;
assert!(improvement >= 1.0);  // always at least as good as seed
assert!(improvement < 2.0);   // but not dramatically better at toy scale
```

This is not a failure condition — it is the expected behaviour at toy scale, and the assertion
documents the science↔engineering disconnect.

### The `CoppersmithGenerator` trait implementation

`CoppersmithGenerator` implements `PolyGenerator`, yielding all variants in spiral order:

```rust
pub struct CoppersmithGenerator {
    pub seed: PolyPair,
    pub config: CoppersmithConfig,
}
```

The default config generates 5 variants with step = 1. Callers can increase `num_polys` and
`step` to explore a larger neighbourhood.

---

## 8. The `PolyGenerator` Trait: A Common Pipeline

All three generators — base-m, root sieve, and Coppersmith — implement the `PolyGenerator` trait:

```rust
pub trait PolyGenerator {
    fn generate(&self) -> impl Iterator<Item = PolyPair>;
}
```

This trait is the common interface for the score-and-rank pipeline. The generator produces
candidates; the scorer (`murphy::score`) ranks them. The three generators differ in how many
candidates they produce:

| Generator | Candidates | Use case |
|-----------|-----------|----------|
| `BaseMGenerator` | 1 (exactly) | Starting point; always valid |
| `RootSieveGenerator` | (2·j+1)×(2·k+1) | Grid search for better polynomial |
| `CoppersmithGenerator` | `num_polys` | Multi-poly set for sieve region partitioning |

A typical pipeline:

```rust
// 1. Start with base-m.
let seed = select_base_m(&n, optimal_degree(&n));

// 2. Improve with root sieve.
let config = RootSieveConfig::default();
let best = root_sieve(&seed, &config);

// 3. Optionally generate a multi-poly set.
let copper_config = CoppersmithConfig::default();
let polys = coppersmith_polys(&best, &copper_config);
```

---

## 9. C-PolyPair and C-Score: Public Contracts

### C-PolyPair

The `PolyPair` type is the **C-PolyPair contract** — the interface between polynomial selection
and all downstream stages. It is frozen at G.B.1 and consumed by:

- **G.B.2 (Murphy-E scoring):** reads `pair.f`, `pair.m`, `pair.skew`; writes `pair.skew`.
- **G.B.3 (root sieve):** reads `pair.f`, `pair.g`, `pair.m`, `pair.n`; produces new `PolyPair`.
- **G.B.4 (Coppersmith):** same as root sieve.
- **G.C (sieving):** reads `pair.f`, `pair.m`, `pair.n`, `pair.degree`; writes
  `pair.factor_base_bounds`. The sieve evaluates the algebraic norm F(a, b) = b^d · f(a/b) for
  each sieve pair (a, b) using the original non-monic f.
- **D.A (NFS-DL poly selection):** adapts `PolyPair` for the discrete logarithm setting. The
  polynomial-pair structure is identical; the difference is in the linear algebra step.

The `verify()` method is the contract enforcement mechanism: any code that produces a `PolyPair`
should call `verify()` before passing it downstream. The KATs enforce this:

```rust
// Every rotation must satisfy verify().
for j in -3i64..=3 {
    for k in -3i64..=3 {
        let rotated = rotate(&seed, j, k);
        assert_eq!(rotated.verify(), Ok(()));
    }
}
```

### C-Score

The `score` function is the **C-Score contract** — the Murphy-E scoring interface consumed by:

- **G.B.3 (root sieve):** scores each rotation candidate to find the best.
- **G.B.4 (Coppersmith):** scores each variant to select `coppersmith_best`.
- **D.A (NFS-DL):** adapts the scoring for the DL polynomial selection problem.
- **E.K (factor-base balancing):** uses the score to balance algebraic and rational factor-base
  sizes. The Murphy-E score's sensitivity to the skew parameter is the signal that E.K uses to
  choose the optimal factor-base bounds.

The C-Score interface is:

```rust
pub fn score(pair: &PolyPair) -> f64;
```

It takes `&PolyPair` (not `&mut`), so it is safe to call from multiple contexts without
mutation. The score is deterministic for a fixed pair (no hidden mutable state).

### Cross-track C3 reuse (D.A and E.K)

The C3 cross-track reuse is the observation that the same polynomial-selection infrastructure
serves multiple downstream tracks:

**D.A (NFS-DL poly selection):** The discrete logarithm variant of NFS uses the same polynomial
pair structure. The base-m construction, Murphy-E scoring, and root sieve are all reused directly.
The only difference is the target: instead of factoring N, the goal is to compute discrete
logarithms in a finite field, which requires a polynomial pair adapted to the field's structure.

**E.K (factor-base balancing):** The factor-base balancing stage uses the Murphy-E score's
sensitivity to the skew parameter to choose the optimal split between algebraic and rational
factor-base sizes. The `pair.skew` field (set by G.B.2 scoring) is the signal that E.K reads.
The `pair.factor_base_bounds` field (set by G.C sieving) is the output that E.K produces.

Both reuses are enabled by the `PolyPair` struct's over-specification: the `skew` and
`factor_base_bounds` fields are `None` at construction and populated by later stages, so the
same type serves all three tracks without modification.

---

## 10. KAT Summary

The following table lists the key known-answer tests across the module, with the mathematical
fact each one verifies. All tests are in `gnfs/tests/`.

| Test | File | Mathematical fact verified |
|------|------|---------------------------|
| `kat1_toy_base_m_round_trip` | `base_m_kat.rs` | f(m) = N exactly for toy N; g(m) = 0; verify() holds |
| `kat2_rsa100_base_m_deterministic` | `base_m_kat.rs` | Same (N, d) → same f; f(m) = N for RSA-100 |
| `kat3_optimal_degree` | `base_m_kat.rs` | d = 3–4 for toy N; d = 5 for RSA-100 (330-bit) |
| `monic_f_and_number_field_from_base_m` | `base_m_kat.rs` | monic_f() is monic; number_field() does not panic |
| `kat1_ordering_better_scores_higher` | `murphy_kat.rs` | Smaller coefficients → higher Murphy-E score |
| `kat2_monotonicity_scaling_decreases_score` | `murphy_kat.rs` | Scaling f by 10 decreases score (monotonicity) |
| `kat3_positivity` | `murphy_kat.rs` | score(pair) > 0 for any valid PolyPair |
| `kat4a–4e_dickman_rho_*` | `murphy_kat.rs` | ρ(0.5) = 1; ρ(1.5) ≈ 0.5945; ρ(2) ≈ 0.3069; ρ(10) < 1e-6 |
| `kat5_score_is_deterministic` | `murphy_kat.rs` | score() is deterministic (no hidden mutable state) |
| `kat1_improvement_score_ge_seed` | `root_sieve_kat.rs` | root_sieve returns score ≥ seed score |
| `kat2_determinism` | `root_sieve_kat.rs` | root_sieve is deterministic for fixed seed and config |
| `kat3_verify` | `root_sieve_kat.rs` | root_sieve result satisfies PolyPair::verify() |
| `kat3b_all_candidates_verify` | `root_sieve_kat.rs` | All 121 candidates in 5×5 grid satisfy verify() |
| `kat4_generator_candidate_count` | `root_sieve_kat.rs` | Generator yields exactly (2j+1)×(2k+1) candidates |
| `kat5_generator_best_matches_root_sieve` | `root_sieve_kat.rs` | Generator best score agrees with root_sieve |
| `kat1_all_polys_verify` | `coppersmith_kat.rs` | All coppersmith_polys variants satisfy verify() |
| `kat2_exact_count` | `coppersmith_kat.rs` | coppersmith_polys returns exactly num_polys pairs |
| `kat2b_single_poly_is_seed` | `coppersmith_kat.rs` | num_polys=1 returns the seed (identity rotation) |
| `kat3_best_score_ge_seed` | `coppersmith_kat.rs` | coppersmith_best returns score ≥ seed score |
| `kat3c_best_matches_max_of_polys` | `coppersmith_kat.rs` | coppersmith_best agrees with max of coppersmith_polys |
| `kat5_principle4_improvement_small_at_toy_scale` | `coppersmith_kat.rs` | Multi-poly improvement < 2× at toy scale (principle-4) |

---

## 11. What Comes Next

Polynomial selection is the first stage of the GNFS pipeline. The output — a `PolyPair` with a
good Murphy-E score — is the input to the sieve.

**G.C (sieving)** consumes `C-PolyPair` directly. For each sieve pair (a, b) in the sieve region,
the sieve evaluates:
- The algebraic norm F(a, b) = b^d · f(a/b) using the original non-monic f.
- The rational norm G(a, b) = a − bm.

Both norms are trial-divided over their respective factor bases. A pair (a, b) is **smooth** if
both norms factor completely over their factor bases. The factor-base bounds (stored in
`pair.factor_base_bounds`) are set by G.C based on the polynomial's Murphy-E score and the
target number of relations.

The `number_field()` method on `PolyPair` is the seam between polynomial selection and the
algebraic side of the sieve: it constructs the number field K = ℚ(α) where α is a root of the
monic form of f, which the sieve uses for ideal-theoretic computations (Dedekind factorisation,
factor-base construction).

The polynomial pair is also the input to the linear algebra step (G.D): the smooth pairs collected
by the sieve form a matrix over 𝔽₂, and the null space of this matrix gives the congruence of
squares that factors N.

---

## Further Reading

1. **Murphy, B. (1999).** *Polynomial selection for the number field sieve integer factorisation
   algorithm.* PhD thesis, Australian National University. The original source for Murphy-E scoring,
   the Dickman-ρ integral, and the skew parameter.

2. **Kleinjung, T. (2006).** *Polynomial selection.* CADO workshop on integer factorization. The
   rotation `f' = f + (j·x + k)·g` and the root-sieve search strategy.

3. **Bai, S., Bouvier, C., Kruppa, A., Zimmermann, P. (2014).** *Better polynomials for GNFS.*
   Mathematics of Computation. The state of the art in polynomial selection, including the
   alpha-value (root property) term in Murphy-E and the size-optimisation step.

4. **Coppersmith, D. (1993).** *Modifications to the number field sieve.* Journal of Cryptology,
   6(3), 169–180. The original multi-polynomial method.

5. **Crandall, R., and Pomerance, C. (2005).** *Prime Numbers: A Computational Perspective.*
   2nd ed. Springer. Chapter 6 covers the Number Field Sieve, including polynomial selection and
   the role of the factor base.

6. **Lenstra, A. K., and Lenstra, H. W. Jr. (eds.) (1993).** *The Development of the Number Field
   Sieve.* Springer LNM 1554. The original papers on GNFS, including the algebraic side and the
   role of the defining polynomial.
