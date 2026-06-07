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

---

# Sieving: A Code-Tour Chapter

This chapter explains the `gnfs/src/sieve` module — the sieving stage of the General Number Field
Sieve. It is organised by the implementation, not by mathematical abstraction: each section
introduces the mathematics, then shows how it is realised in code. A reader who has read the
polynomial selection chapter (§1–§11 above) can read this one without consulting the source.

The chapter covers the relation as the unit of NFS data, the two-sided factor base, the norm
bridge, the line sieve, the special-q strategy, lattice sieving, the exponent vector and relation
matrix, and the downstream reuse in G.D, G.E, and D.A.

---

## 12. The Relation as the Unit of NFS Data

### What a relation is

A **relation** in GNFS is a coprime integer pair (a, b) with b ≥ 1 for which both the rational
norm and the algebraic norm are smooth over their respective factor bases:

```text
N_rat(a, b) = a − b·m          (rational side: the value of g(x) = x − m at a/b, cleared)
N_alg(a, b) = b^d · f(a/b)     (algebraic side: the homogeneous form of f at (a, b))
```

Both norms must be smooth — divisible only by primes up to their respective bounds B_rat and B_alg.
A pair that is smooth on only one side is not a relation; it contributes nothing to the factoring
computation.

The `Relation` type in `gnfs/src/sieve/mod.rs` carries the pair and both smoothness witnesses:

```rust
pub struct Relation {
    /// The a-coordinate (can be negative).
    pub a: BigInt,
    /// The b-coordinate (always positive: b ≥ 1).
    pub b: BigInt,
    /// Exponent vector over the rational factor base.
    pub rational_exponents: ExponentVector,
    /// Exponent vector over the algebraic factor base.
    pub algebraic_exponents: ExponentVector,
    /// True if the rational norm a − b·m is negative.
    pub rational_sign: bool,
}
```

The `verify()` predicate checks all four invariants:

1. gcd(a, b) = 1 (coprimality — non-coprime pairs are degenerate).
2. The rational exponent vector reconstructs |N_rat(a, b)| (product of p^e matches the norm).
3. The algebraic exponent vector reconstructs |N_alg(a, b)| (same check on the algebraic side).
4. The sign matches: `rational_sign == (a − b·m < 0)`.

```rust
rel.verify(&poly, &fb).expect("relation must be valid");
```

### Why smoothness on both sides is required

The factoring computation (G.F) requires finding a subset S of relations such that the product
of all rational norms is a perfect square in ℤ and the product of all algebraic norms is a
perfect square in the number field K = ℚ(α). This is the congruence of squares that yields a
non-trivial factor of N.

For the product to be a perfect square, every prime must appear to an even total exponent across
all selected relations. This is a linear algebra problem over GF(2): find a subset S such that
the sum of all exponent vectors (mod 2) is the zero vector. The relation matrix has one row per
relation and one column per prime (plus sign and quadratic-character columns). The null space of
this matrix over GF(2) gives the subsets S.

A relation that is smooth on only one side has no algebraic (or rational) exponent vector, so it
cannot participate in the square-root computation. Both sides must be smooth for the relation to
contribute a complete row to the matrix.

---

## 13. The Two-Sided Factor Base

### The rational factor base

The rational factor base is the set of primes p ≤ B_rat. A rational norm N_rat(a, b) = a − b·m
is smooth over this base if all its prime factors are ≤ B_rat.

### The algebraic factor base: degree-1 prime ideals

The algebraic factor base is not simply the set of primes p ≤ B_alg. It is the set of
**degree-1 prime ideals** (p, α − r) in ℤ[α], where α is a root of f and r ∈ [0, p) satisfies
f(r) ≡ 0 (mod p). Each prime p contributes as many ideals as f has roots mod p (up to deg(f)).

The `AlgebraicPrime` type represents one such ideal:

```rust
pub struct AlgebraicPrime {
    /// The rational prime p.
    pub p: u64,
    /// The root r ∈ [0, p) with f(r) ≡ 0 (mod p).
    pub r: u64,
    /// Index of this ideal in the algebraic factor base (for column mapping).
    pub index: usize,
    /// True if p | disc(f) (bad prime).
    pub is_bad_prime: bool,
}
```

The key sieve condition: the ideal (p, α − r) divides N_alg(a, b) if and only if a ≡ r·b (mod p).
This is the algebraic analogue of the rational condition a ≡ b·m (mod p) for the rational side.

### Why degree-1 ideals

At toy scale, the degree-1 ideal (p, r) representation suffices because:

1. The algebraic norm N_alg(a, b) = b^d · f(a/b) factors in ℤ[α] as a product of prime ideals.
2. For a prime p with f(r) ≡ 0 (mod p), the ideal (p, α − r) divides the principal ideal (a − b·α)
   in ℤ[α], and hence divides N_alg(a, b) = Norm(a − b·α).
3. The sieve condition a ≡ r·b (mod p) is the efficient way to enumerate which (a, b) pairs have
   the ideal (p, r) dividing their algebraic norm.

### The `FactorBase` type

The `FactorBase` type in `gnfs/src/sieve/factor_base.rs` holds both sides:

```rust
pub struct FactorBase {
    /// Rational factor base: primes p ≤ B_rat, sorted ascending.
    pub rational_primes: Vec<u64>,
    /// Algebraic factor base: degree-1 prime ideals (p, r), sorted by (p, r).
    pub algebraic_ideals: Vec<AlgebraicPrime>,
    /// Rational smoothness bound B_rat.
    pub b_rat: u64,
    /// Algebraic smoothness bound B_alg.
    pub b_alg: u64,
    /// Number of obstruction columns reserved for G.E (sign + quadratic chars).
    pub obstruction_count: usize,
}
```

Construction:

```rust
let fb = FactorBase::new(&poly.f, b_rat, b_alg);
```

`FactorBase::new` uses `factor_base_up_to` (from `shared-numth`) for the rational side, and
brute-force root-finding (checking f(r) ≡ 0 mod p for each r ∈ 0..p) for the algebraic side.

### Bad primes and the principle-4 annotation

A prime p is **bad** if p | disc(f). At bad primes, Dedekind's theorem does not apply directly:
ℤ[α] may not be the full ring of integers at p, and the ideal factorisation above p may be more
complex than the root structure of f mod p suggests.

At toy scale, bad primes are prominent. For f(x) = x³ − x − 1, the discriminant is −23, so p = 23
is a bad prime. The implementation includes bad primes in the algebraic factor base with the
`is_bad_prime: true` flag, using direct root-finding (correct for linear factors even at bad
primes). The flag documents the principle-4 over-exposure:

**Principle-4 annotation.** At cryptographic scale (RSA-768+), bad primes are marginal: the
discriminant has few prime factors, and those primes are a negligible fraction of the algebraic
factor base. At toy scale, bad primes can be a significant fraction of the factor base and are
unavoidable for hand-picked polynomials. The `is_bad_prime` flag makes this visible without
changing the algorithm.

### Column indexing for G.D and G.E

The `AlgebraicPrime::index` field and the lookup methods `rational_index(p)` and
`algebraic_index(p, r)` provide the mapping from factor-base elements to matrix columns. G.D
(filtering) and G.E (linear algebra) use these indices to build and manipulate the relation
matrix without re-scanning the factor base.

The `matrix_width()` method returns the total number of columns:

```rust
pub fn matrix_width(&self) -> usize {
    self.rational_size() + self.algebraic_size() + self.obstruction_count
}
```

The `obstruction_count` is initialised to 1 (the sign/−1 column). G.E will add quadratic-character
columns; the slot exists so the matrix-width calculation is stable across stages.

---

## 14. The Norm Bridge

### Why norms are signed BigInt but trial_smooth takes Uint<4>

The rational norm N_rat(a, b) = a − b·m can be negative (when a < b·m). The algebraic norm
N_alg(a, b) = b^d · f(a/b) can also be negative depending on f and (a, b). Both are computed as
signed `BigInt` values.

The smoothness predicate `trial_smooth` (from `shared-numth`) operates on `Uint<4>` — unsigned
256-bit integers. The **norm bridge** in `gnfs/src/sieve/norms.rs` converts a signed norm to its
absolute value as `Uint<4>`:

```rust
pub fn norm_to_uint(norm: &BigInt) -> Result<Uint<4>, NormBridgeError> {
    let abs_norm = norm.abs();
    let bits = abs_norm.bits() as usize;
    if bits > 256 {
        return Err(NormBridgeError::Overflow { bits_required: bits });
    }
    // ... convert to Uint<4> via big-endian byte array ...
}
```

The bridge rejects out-of-range norms with `NormBridgeError::Overflow` rather than silently
truncating. Silent truncation would produce incorrect smoothness witnesses — a correctness hazard.

### The sign column in the relation matrix

The sign of the rational norm is tracked separately in the `rational_sign: bool` field of
`Relation`. This is the "−1 column" for G.E's linear algebra: the product of all selected
relations' rational norms must be a perfect square, which requires the sign product to be +1
(an even number of negative norms). The sign column encodes this constraint.

The algebraic norm's sign is not stored separately because the algebraic square root computation
(G.F) handles sign via the real embedding of K, not via a matrix column.

### The algebraic norm as a resultant

The algebraic norm N_alg(a, b) = b^d · f(a/b) is computed as the homogeneous form:

```text
N_alg(a, b) = Σ_{i=0}^{d} f.coeffs[i] · a^i · b^{d−i}
```

This equals Res(f, a − b·x) up to sign and leading-coefficient factors. The resultant
relationship is the algebraic hook: N_alg(a, b) is the norm of the ideal (a − b·α) in ℤ[α],
which equals the resultant of f and a − b·x. The implementation computes the homogeneous form
directly (avoiding rational arithmetic) rather than via the resultant, which would be slower.

---

## 15. The Line Sieve

### The log-p mark-then-confirm pattern

The line sieve in `gnfs/src/sieve/line.rs` is the baseline relation-collection algorithm. For
each b in 1..=B, it sieves the range a ∈ −A..=A on both sides simultaneously:

**Step 1.** Initialise a sieve array of size 2A + 1 with zeros.

**Step 2 (rational side).** For each prime p in the rational factor base, find the starting
a ≡ b·m (mod p) and mark all a in range with += log₂(p). The rational norm N_rat(a, b) = a − b·m
is divisible by p iff a ≡ b·m (mod p).

**Step 3 (algebraic side).** For each ideal (p, r) in the algebraic factor base, find the
starting a ≡ r·b (mod p) and mark all a in range with += log₂(p). The algebraic norm N_alg(a, b)
is divisible by the ideal (p, r) iff a ≡ r·b (mod p).

**Step 4 (threshold filter).** Collect candidates where the sieve value exceeds a smoothness
threshold (default: 0.8 × log₂(B_alg)). The threshold accepts candidates where the algebraic
norm has accumulated enough log contributions to plausibly be smooth.

**Step 5 (confirm).** For each candidate (a, b): check gcd(a, b) = 1; compute both norms; call
`trial_smooth` on both; if both are fully smooth, construct a `Relation`.

```rust
pub fn line_sieve(
    poly: &PolyPair,
    fb: &FactorBase,
    config: &LineSieveConfig,
) -> Vec<Relation> {
    // ...
    for b in 1u64..=config.b_bound {
        let mut sieve: Vec<f32> = vec![0.0f32; sieve_len];
        // Step 2: rational side sieve
        for (pi, &p) in fb.rational_primes.iter().enumerate() {
            let bm_mod_p = mod_u64_bigint(&(&b_big * &poly.m), p);
            let start_a = first_a_in_range(bm_mod_p, p, -a_bound);
            let mut a = start_a;
            while a <= a_bound { sieve[(a + a_bound) as usize] += rat_logs[pi]; a += p as i64; }
        }
        // Step 3: algebraic side sieve
        for (ai, ap) in fb.algebraic_ideals.iter().enumerate() {
            let rb_mod_p = (ap.r as u128 * b as u128 % ap.p as u128) as u64;
            let start_a = first_a_in_range(rb_mod_p, ap.p, -a_bound);
            let mut a = start_a;
            while a <= a_bound { sieve[(a + a_bound) as usize] += alg_logs[ai]; a += ap.p as i64; }
        }
        // Steps 4–5: threshold filter and confirm
        // ...
    }
}
```

### Why this is the engineering heart of NFS

The log-p mark-then-confirm pattern is the key engineering insight of NFS sieving. The sieve
array accumulates approximate log contributions cheaply (one addition per prime per sieve
position). The threshold filter eliminates most candidates before the expensive `trial_smooth`
call. At cryptographic scale (B ≈ 10⁷, A ≈ 10⁷, B_rat/B_alg ≈ 10⁶), the threshold filters
~99% of candidates, making the sieve dramatically faster than brute-force trial division of
every (a, b) pair.

### Principle-4 annotation: asymptotic win under-exposed at toy scale

**At toy scale (A = 10, B = 3, B_rat = B_alg = 30), the log-sieve barely beats direct trial
division.** The sieve array has only 21 entries; the factor bases have ~10 primes each; the
threshold filters almost nothing (every candidate passes). The asymptotic win — avoiding trial
division for most pairs — is not visible because there are so few pairs to begin with.

The KAT uses a conservative lower bound (≥ 5 relations) rather than an exact count, because the
threshold heuristic may miss some smooth pairs at toy scale. The exact count is pinned by the
determinism KAT (KAT b3 in `line_sieve_kat.rs`).

At cryptographic scale, the sieve array has ~10⁷ entries, the factor bases have ~10⁵ elements,
and the threshold filters ~99% of candidates. The log-sieve is then 100–1000× faster than
brute-force trial division.

---

## 16. The Special-q Strategy

### The yield multiplier

The special-q strategy in `gnfs/src/sieve/special_q.rs` is an optimization layer over the line
sieve. For each **special prime** q in a chosen range [q_min, q_max], the sieve is restricted to
pairs (a, b) with q | N_alg(a, b). The restriction is enforced by the sieve condition:

```text
q | N_alg(a, b)  iff  a ≡ r_q·b (mod q)
```

for some root r_q of f mod q. Every candidate in the restricted set already has q as a known
algebraic factor. The remaining cofactor N_alg(a, b) / q is smaller and therefore more likely
to be smooth over the algebraic factor base.

The `SpecialQResult` type records the per-q output:

```rust
pub struct SpecialQResult {
    /// The special prime q.
    pub q: u64,
    /// The root r_q ∈ [0, q) of f mod q used for this sieve run.
    pub r_q: u64,
    /// Relations collected in this sieve run.
    pub relations: Vec<Relation>,
    /// The sieve area covered: ⌈(2A+1)/q⌉ × B pairs.
    pub restricted_area: u64,
}
```

Every relation in `relations` carries q in its algebraic exponent vector — this is the structural
invariant of the special-q strategy, checked by the KAT.

### The restricted sieve loop

The implementation runs the same log-sieve as the line sieve, but only visits a values satisfying
a ≡ r_q·b (mod q) for each b. This is implemented by stepping a in increments of q:

```rust
let rb_q = (r_q as u128 * b as u128 % q as u128) as u64;
let start_a_q = first_a_in_range(rb_q, q, -a_bound);
let mut a_q = start_a_q;
while a_q <= a_bound {
    // apply threshold filter and trial-divide
    a_q += q as i64;
}
```

The log-sieve array is still computed over the full range (for all primes in the factor bases),
but only the q-restricted positions are trial-divided. This is the correct approach: the sieve
contributions from other primes are still needed to filter candidates.

### Principle-4 annotation: yield advantage under-exposed at toy scale

**At toy scale, the yield advantage of the special-q strategy over the plain line sieve is
under-exposed.** The yield multiplier is a scale phenomenon:

- At cryptographic scale (B_alg ≈ 10⁶, A ≈ 10⁷), the algebraic norm N_alg(a, b) is large
  (hundreds of bits) and the probability of smoothness is low. The pre-guaranteed factor q
  significantly reduces the cofactor, making smoothness much more likely. The special-q strategy
  yields 5–10× more relations per sieve area than the plain line sieve.

- At toy scale (B_alg = 30, A = 10, B = 3), the norms are already small (tens of bits) and
  smooth with high probability. The pre-guaranteed factor q does not significantly improve the
  smoothness probability. The yield advantage is not observable.

The KAT (`kat_b_yield_comparison_principle4_annotated` in `special_q_kat.rs`) checks the
structural property (q in the algebraic exponent vector) and annotates the yield comparison as
under-exposed at toy scale, rather than asserting a yield improvement that would fail.

---

## 17. Lattice Sieving (Demonstration Fidelity)

### The lattice L_q and its natural basis

For a special prime q with root r_q, the set of (a, b) pairs satisfying the sieve restriction is
the **lattice**:

```text
L_q = { (a, b) ∈ ℤ² : a ≡ r_q·b (mod q) }
```

A natural basis for L_q is:

```text
v1 = (q, 0)    (check: q ≡ r_q·0 (mod q) ✓)
v2 = (r_q, 1)  (check: r_q ≡ r_q·1 (mod q) ✓)
```

Every lattice point s·v1 + t·v2 = (s·q + t·r_q, t) satisfies a ≡ r_q·b (mod q) since
s·q + t·r_q ≡ t·r_q ≡ r_q·t (mod q).

### Gauss 2D lattice reduction

The natural basis (v1, v2) is not efficient for enumeration: v1 = (q, 0) has length q, which
is large for large special primes. The **Gauss lattice reduction** algorithm finds a shorter,
more orthogonal basis (V1, V2):

```text
while |V1| > |V2|: swap V1, V2
V1 = V1 - round(dot(V1, V2) / dot(V2, V2)) * V2
```

Repeat until convergence. The reduced basis satisfies |V1| ≤ |V2| and
|dot(V1, V2)| ≤ |V2|² / 2 (the Gauss-reduced condition). The algorithm preserves the lattice
because each step is a unimodular transformation (determinant ±1).

The `LatticeBasis` type in `gnfs/src/sieve/lattice.rs` implements this:

```rust
pub struct LatticeBasis {
    pub v1: (i64, i64),
    pub v2: (i64, i64),
    pub q: u64,
    pub r_q: u64,
}

impl LatticeBasis {
    pub fn initial(q: u64, r_q: u64) -> Self { /* v1=(q,0), v2=(r_q,1) */ }
    pub fn gauss_reduce(&self) -> Self { /* Gauss reduction loop */ }
    pub fn in_lattice(&self, a: i64, b: i64) -> bool { /* a ≡ r_q·b (mod q) */ }
}
```

### Enumeration via the reduced basis

The lattice sieve enumerates (a, b) = s·V1 + t·V2 for integer (s, t) in a bounded region,
rather than stepping through a in increments of q for each b. The reduced basis vectors are
shorter than the original basis, so the enumeration visits fewer lattice points outside the
sieve region |a| ≤ A, 1 ≤ b ≤ B.

For each enumerated (a, b), the sieve applies the same log-threshold filter and trial-division
as the line sieve. The `LatticeSieveResult` records the per-(q, r_q) output:

```rust
pub struct LatticeSieveResult {
    pub q: u64,
    pub r_q: u64,
    pub basis: LatticeBasis,  // exposed for KAT inspection
    pub relations: Vec<Relation>,
    pub enumerated_points: u64,
}
```

### Principle-4 annotation: yield-per-area improvement is a scale phenomenon

**At toy scale, the lattice sieve produces the same (a, b) pairs as the special-q line sieve
for the same (q, r_q).** The two algorithms are mathematically equivalent: both enumerate L_q
in the region |a| ≤ A, 1 ≤ b ≤ B. The efficiency difference — the reduced basis visits fewer
wasted candidates outside the sieve region — is a constant factor that is swamped by the
overhead of the reduction and enumeration at small q.

At cryptographic scale, the reduced basis has vectors of length ≈ √q, so the enumeration covers
≈ A·B / q lattice points with minimal waste. For q ≈ 10⁶ and A, B ≈ 10⁷, this is a significant
efficiency gain over stepping by q.

The KAT (`kat_b_lattice_sieve_subset_of_special_q_sieve` in `lattice_kat.rs`) verifies that the
lattice sieve produces a subset of the special-q sieve relations for the same (q, r_q), and
annotates the yield-per-area comparison as under-exposed at toy scale.

---

## 18. The Exponent Vector and the Relation Matrix

### The `ExponentVector` type

The `ExponentVector` type in `gnfs/src/sieve/mod.rs` stores the sparse exponent representation
for one side of a relation:

```rust
pub struct ExponentVector {
    /// Sparse: (factor-base index, exponent) pairs, sorted by index.
    /// Exponents are always > 0 (zeros are not stored).
    pub entries: Vec<(usize, u32)>,
}
```

The exponent type is `u32`, not `u8` or `bool`. This accommodates:

- **NFS-factoring (G.E):** exponents are small (typically 1–3), reduced mod 2 for the GF(2)
  nullspace computation.
- **NFS-DL (D.A):** exponents are reduced mod ℓ where ℓ is the target group order; ℓ can be
  large, but exponents before reduction are still small integers.

The `u32` type is the smallest that accommodates both without overflow risk. This is the
over-specification for D.A: storing integer exponents rather than pre-reduced GF(2) parities
means D.A can read the exponents directly without resharding the relation format.

### The GF(2) row methods

The `Relation` type provides two methods for G.E:

```rust
/// GF(2) row for the rational side: sign column prepended, then exponent parities.
pub fn rational_row_gf2(&self, fb: &FactorBase) -> Vec<bool> {
    let mut row = vec![false; 1 + fb.rational_size()];
    row[0] = self.rational_sign;  // sign/−1 column
    for (idx, exp) in self.rational_exponents.iter() {
        row[1 + idx] = (exp % 2) == 1;
    }
    row
}

/// GF(2) row for the algebraic side: exponent parities, obstruction columns appended as zeros.
pub fn algebraic_row_gf2(&self, fb: &FactorBase) -> Vec<bool> {
    let mut row = vec![false; fb.algebraic_size() + fb.obstruction_count];
    for (idx, exp) in self.algebraic_exponents.iter() {
        if idx < fb.algebraic_size() { row[idx] = (exp % 2) == 1; }
    }
    // Obstruction columns remain false; G.E fills them in.
    row
}
```

The sign column is prepended to the rational row: bit 0 is 1 iff `rational_sign` is true. The
obstruction columns (quadratic characters) are appended to the algebraic row as zeros; G.E fills
them in when it constructs the full matrix.

### Why integer exponents, not GF(2) parities

The design decision to store full integer exponents (not pre-reduced GF(2) parities) is the
load-bearing cross-track call in the C-Relation contract. Three consumers need different views:

- **G.D (filtering):** reads the exponent vectors to detect duplicate or linearly dependent
  relations. Integer exponents are needed to identify exact duplicates.
- **G.E (linear algebra):** reduces exponents mod 2 via `rational_row_gf2` / `algebraic_row_gf2`
  to build the GF(2) matrix. The `u32` exponents are reduced on demand.
- **D.A (NFS-DL):** reads the integer exponents directly for GF(ℓ) linear algebra, where ℓ is
  the target group order. Pre-reducing to GF(2) would destroy the information D.A needs.

Re-narrowing C-Relation after G.E or D.A consumes it would be a destructive reshard. The
over-specification (integer exponents) is the correct design for a cross-track contract.

---

## 19. Downstream Reuse: G.D, G.E, and D.A

### G.D (filtering)

The relation corpus from the sieve feeds G.D (filtering), which removes:

- **Duplicate relations:** the same (a, b) pair found by multiple sieve runs (e.g., the line
  sieve and the special-q sieve may both find the same relation).
- **Linearly dependent relations:** relations whose exponent vectors are linearly dependent over
  GF(2), which would not contribute to the null space.

G.D reads the `ExponentVector` entries and the `FactorBase` column indices. The `AlgebraicPrime::index`
field and the `rational_index` / `algebraic_index` lookup methods provide the column mapping.

### G.E (linear algebra)

G.E (linear algebra over GF(2)) takes the filtered relation corpus and finds a subset S such that
the sum of all exponent vectors (mod 2) is the zero vector. This is the null space computation:

1. Build the relation matrix: one row per relation, one column per factor-base prime (plus sign
   and quadratic-character columns). Each row is `rational_row_gf2` concatenated with
   `algebraic_row_gf2`.
2. Find the null space of this matrix over GF(2) (e.g., via structured Gaussian elimination).
3. Each null-space vector identifies a subset S of relations whose exponent product is a perfect
   square on both sides.

The `matrix_width()` method on `FactorBase` gives the total column count:
rational_size + algebraic_size + obstruction_count.

### D.A (NFS-DL)

The NFS discrete logarithm algorithm (D.A) uses the same relation structure as NFS factoring,
but interprets exponents mod ℓ (the target group order) instead of mod 2. The integer exponents
stored in `ExponentVector` support this interpretation directly.

D.A may add **Schirokauer map** columns to the algebraic exponent vector. The sparse
`(index, exponent)` representation accommodates additional entries without resharding the
`Relation` type. This is the cross-track over-specification: the `Relation` type is designed so
D.A can read it without modification.

---

## 20. KAT Summary (G.C)

The following table lists the key known-answer tests across the sieve module, with the
mathematical fact each one verifies. All tests are in `gnfs/tests/`.

| Test | File | Mathematical fact verified |
|------|------|---------------------------|
| `kat1_algebraic_factor_base_matches_brute_force` | `factor_base_kat.rs` | Algebraic FB = brute-force root enumeration mod each p ≤ B_alg |
| `kat1b_index_lookups_consistent` | `factor_base_kat.rs` | `rational_index` / `algebraic_index` round-trip correctly |
| `kat2_norm_reconstruction_and_relation_verify` | `factor_base_kat.rs` | `Relation::verify()` holds for hand-constructed smooth relation |
| `kat2b_verify_fails_on_perturbed_exponent` | `factor_base_kat.rs` | `verify()` fails when an exponent is perturbed |
| `kat2c_relation_new_rejects_non_coprime` | `factor_base_kat.rs` | `Relation::new` returns None for non-coprime (a, b) |
| `kat2d_relation_new_rejects_partial_smoothness` | `factor_base_kat.rs` | `Relation::new` returns None when cofactor > 1 |
| `kat3_norm_bridge_range` | `factor_base_kat.rs` | Toy-scale norm fits Uint<4>; 2^257 overflows with error |
| `kat3b_norm_sign` | `factor_base_kat.rs` | `norm_sign` returns true iff norm < 0 |
| `kat3c_norm_bridge_round_trip_with_trial_smooth` | `factor_base_kat.rs` | `norm_to_uint` + `trial_smooth` round-trip for known smooth norm |
| `kat_a_sieve_produces_relations_all_verify` | `line_sieve_kat.rs` | Sieve finds ≥ 5 relations; all satisfy `verify()` |
| `kat_a2_all_relations_are_coprime_and_smooth` | `line_sieve_kat.rs` | All returned relations are coprime and fully smooth |
| `kat_b_deterministic_relation_count` | `line_sieve_kat.rs` | Same parameters → same relations (determinism) |
| `kat_b2_lower_threshold_finds_at_least_as_many_relations` | `line_sieve_kat.rs` | Lower threshold ≥ higher threshold relation count |
| `kat_b3_exact_count_is_stable` | `line_sieve_kat.rs` | Exact count is stable across runs (pins the implementation) |
| `kat_a_relations_verify_and_carry_q` | `special_q_kat.rs` | All special-q relations verify and carry q in algebraic EV |
| `kat_a2_sieve_restriction_enforced` | `special_q_kat.rs` | All (a, b) satisfy a ≡ r_q·b (mod q) |
| `kat_a3_spot_check_known_relation_q7` | `special_q_kat.rs` | (a=5, b=1) found for q=7, r_q=5; carries q=7 in algebraic EV |
| `kat_b_yield_comparison_principle4_annotated` | `special_q_kat.rs` | Yield advantage annotated as under-exposed at toy scale |
| `kat_b2_special_q_relations_subset_of_line_sieve` | `special_q_kat.rs` | Special-q relations ⊆ line sieve relations for same q |
| `kat_a_all_relations_lie_in_lattice` | `lattice_kat.rs` | All lattice-sieve (a, b) satisfy a ≡ r_q·b (mod q) |
| `kat_a2_all_relations_verify` | `lattice_kat.rs` | All lattice-sieve relations satisfy `verify()` |
| `kat_a3_relations_carry_q_in_algebraic_exponents` | `lattice_kat.rs` | All lattice-sieve relations carry q in algebraic EV |
| `kat_a4_reduced_basis_q7_r5_is_correct` | `lattice_kat.rs` | Gauss-reduced basis for q=7, r_q=5 is correct |
| `kat_b_lattice_sieve_subset_of_special_q_sieve` | `lattice_kat.rs` | Lattice-sieve relations ⊆ special-q sieve relations |
| `kat_c_yield_comparison_principle4_annotated` | `lattice_kat.rs` | Yield-per-area improvement annotated as under-exposed |

---

## 21. What Comes Next

Sieving is the third stage of the GNFS pipeline. The output — a corpus of `Relation` objects,
each with both exponent vectors and the sign bit — is the input to the filtering and linear
algebra stages.

**G.D (filtering)** takes the raw relation corpus and removes duplicates and linearly dependent
relations. The `ExponentVector` entries and the `FactorBase` column indices are the interface
between G.C and G.D.

**G.E (linear algebra over GF(2))** takes the filtered corpus and finds a subset S of relations
whose exponent product is a perfect square on both sides. The `rational_row_gf2` and
`algebraic_row_gf2` methods on `Relation` produce the GF(2) matrix rows. The `matrix_width()`
method on `FactorBase` gives the column count.

**G.F (square root)** takes the subset S from G.E and computes the congruence of squares
mod N that yields the non-trivial factor. This stage uses the number-field arithmetic from
`shared-numfield` and the algebraic structure of K = ℚ(α).

**D.A (NFS-DL)** adapts the same relation structure for discrete logarithm computation. The
integer exponents in `ExponentVector` are read directly (not reduced mod 2) and used in GF(ℓ)
linear algebra. The `Relation` type is designed to support this adaptation without resharding.

---

## Further Reading (G.C)

1. **Pomerance, C. (1996).** *A tale of two sieves.* Notices of the AMS, 43(12), 1473–1485.
   An accessible introduction to the quadratic sieve and the number field sieve, with the
   sieving step explained at a high level.

2. **Lenstra, A. K., and Lenstra, H. W. Jr. (eds.) (1993).** *The Development of the Number
   Field Sieve.* Springer LNM 1554. The original papers on GNFS, including the sieving step
   and the algebraic side.

3. **Briggs, M. E. (1998).** *An introduction to the general number field sieve.* Master's
   thesis, Virginia Tech. A self-contained exposition of the GNFS pipeline including sieving,
   with worked examples at toy scale.

4. **Kleinjung, T., Aoki, K., Franke, J., et al. (2010).** *Factorization of a 768-bit RSA
   modulus.* CRYPTO 2010. The RSA-768 factorisation, which used the special-q lattice sieve
   as the primary relation-collection strategy.

5. **Schirokauer, O. (1993).** *Discrete logarithms and local units.* Philosophical Transactions
   of the Royal Society A, 345(1676), 409–423. The Schirokauer maps used in D.A (NFS-DL) to
   handle the units in the number field — the D.A extension of the C-Relation contract.
