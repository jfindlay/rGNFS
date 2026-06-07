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

---

# Filtering: A Code-Tour Chapter

This chapter explains the `gnfs/src/filter` module — the filtering stage of the General Number
Field Sieve. It is organised by the implementation, not by mathematical abstraction: each section
introduces the mathematics, then shows how it is realised in code. A reader who has read the
sieving chapter (§12–§21 above) can read this one without consulting the source.

The chapter covers the transition from a relation corpus to a sparse GF(2) matrix, the graph view
of that matrix, singleton removal, excess and clique pruning, column merging, and the provenance
map that threads through to G.F.

---

## 22. The Relation Corpus as a Sparse Matrix

### From `Vec<Relation>` to a GF(2) matrix

The sieve (G.C) produces a `Vec<Relation>` — a corpus of smooth pairs (a, b). Each relation
carries two exponent vectors: one over the rational factor base, one over the algebraic factor
base. The filtering stage (G.D) converts this corpus into a sparse matrix over GF(2).

The conversion is straightforward:

- **One row per relation.** Relation i contributes row i.
- **One column per factor-base element.** Each rational prime p and each algebraic ideal (p, r)
  contributes one column.
- **The entry is the GF(2) parity of the exponent.** If prime p divides the rational norm to an
  odd exponent, the entry is 1; if even (or absent), it is 0.

The column layout is fixed by `FactorBase::matrix_width()`:

$$
\text{columns} = \underbrace{\text{rational\_size}}_{\text{rational primes}} + \underbrace{\text{algebraic\_size}}_{\text{algebraic ideals}} + \underbrace{\text{obstruction\_count}}_{\text{sign + quad. chars.}}
$$

Concretely, for the toy setup in `merge_kat.rs` (f(x) = x³ − x − 1, B_rat = B_alg = 13):

- Rational primes ≤ 13: {2, 3, 5, 7, 11, 13} → columns 0–5 (`rational_size = 6`).
- Algebraic ideals: (5, 2), (7, 5), (11, 6) → columns 6–8 (`algebraic_size = 3`).
- Obstruction column: sign bit → column 9 (`obstruction_count = 1`).
- Total: `matrix_width = 10`.

The `build_matrix` function in `gnfs/src/filter/mod.rs` performs this construction:

```rust
pub fn build_matrix(relations: &[Relation], fb: &FactorBase) -> SparseMatrix {
    // ...
    for (i, relation) in relations.iter().enumerate() {
        let rat_row = relation.rational_row_gf2(fb);  // [sign_bit, rat_col_0, ...]
        let alg_row = relation.algebraic_row_gf2(fb); // [alg_col_0, ..., zeros...]

        let mut cols: Vec<usize> = Vec::new();
        // Rational columns: local index 1+k → global column k.
        for k in 0..rat_size {
            if rat_row[1 + k] { cols.push(k); }
        }
        // Algebraic columns: local index k → global column rat_size + k.
        for k in 0..alg_size {
            if alg_row[k] { cols.push(rat_size + k); }
        }
        // Sign bit → global obstruction column.
        if rat_row[0] { cols.push(obstruction_col_start); }

        rows.push(MatrixRow { cols, provenance: vec![i] });
    }
    // ...
}
```

The `rational_row_gf2` and `algebraic_row_gf2` methods on `Relation` (defined in G.C) produce
the GF(2) parities. The sign bit is placed at the first obstruction column, not at column 0 of
the rational block — this is the re-mapping that `build_matrix` performs.

### Why GF(2): the parity condition

The goal of the linear algebra step (G.E) is to find a subset S of relations such that the
product of all rational norms is a perfect square in ℤ and the product of all algebraic norms
is a perfect square in the number field K = ℚ(α). A product of integers is a perfect square
iff every prime appears to an even total exponent. This is a **parity condition**: we need the
sum of all exponent vectors (mod 2) to be the zero vector. Working over GF(2) encodes exactly
this condition.

The sign column encodes the sign parity: the product of all selected rational norms must be
positive (an even number of negative norms). The quadratic-character columns (filled by G.E)
encode additional algebraic-side parity conditions needed to guarantee that the algebraic square
root is well-defined.

---

## 23. The Graph View

### Relations as edges, primes as nodes

The sparse GF(2) matrix has a natural graph interpretation that illuminates the filtering
algorithms:

- **Nodes** are the non-obstruction columns — the rational primes and algebraic ideals.
- **Edges** are the rows — the relations. A relation "connects" the primes it contains: if
  relation R has a 1 in columns c₁ and c₂, then R is an edge between nodes c₁ and c₂.

A **GF(2) dependency** is a subset of relations whose column-sum (mod 2) is the zero vector.
In graph terms, this is a **cycle**: a set of edges such that every node is incident to an even
number of edges in the set. The null space of the matrix over GF(2) is exactly the set of all
such cycles.

### Why singletons cannot appear in a dependency

A **singleton column** is a column of Hamming weight 1 — a prime or ideal that appears in
exactly one surviving relation. In graph terms, it is a node of degree 1 (a leaf).

A leaf cannot be part of any cycle. If a node has degree 1, the unique edge incident to it must
appear in any cycle that includes that node. But then the node is incident to exactly one edge
in the cycle — an odd count — which violates the cycle condition. Therefore, no cycle can
include a leaf node, and the relation (edge) containing the singleton column can never be part
of a GF(2) dependency.

This is the mathematical justification for singleton removal: the row containing a singleton
column contributes nothing to the null space and can be safely discarded.

---

## 24. Singleton Removal

### The fixpoint algorithm

Singleton removal is implemented in `gnfs/src/filter/singleton.rs` as a fixpoint iteration:

```rust
pub fn remove_singletons(mut matrix: SparseMatrix) -> SparseMatrix {
    loop {
        let mut rows_to_remove: Vec<usize> = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for col in 0..matrix.obstruction_col_start {
            if matrix.col_weights[col] == 1 {
                // Find the unique row containing this column.
                for (row_idx, row) in matrix.rows.iter().enumerate() {
                    if row.cols.binary_search(&col).is_ok() {
                        if seen.insert(row_idx) {
                            rows_to_remove.push(row_idx);
                        }
                        break;
                    }
                }
            }
        }

        if rows_to_remove.is_empty() { break; }  // fixpoint reached

        // Remove in descending order to preserve index validity.
        rows_to_remove.sort_unstable();
        rows_to_remove.dedup();
        for &row_idx in rows_to_remove.iter().rev() {
            matrix.remove_row(row_idx);
        }
    }
    matrix
}
```

The loop collects all rows that contain a weight-1 non-obstruction column, removes them, and
repeats. It terminates when no weight-1 non-obstruction column remains.

### Why a single pass is insufficient: cascading singletons

Removing a row decrements the weight of every column it contains. A column that had weight 2
becomes weight 1 after its partner row is removed — a new singleton. A single pass over the
column list would miss these newly created singletons. The fixpoint loop handles cascading
singletons correctly: after each batch of removals, the column weights are updated and the
scan restarts.

The KAT in `merge_kat.rs` (KAT d) demonstrates cascading removal: relation R4 contains the
only occurrence of ideal (7, 5) (a singleton), so R4 is removed. After R4's removal, ideal
(5, 2) drops to weight 1 (only R3 remains), so R3 is removed. After R3's removal, prime 7
drops to weight 2 (R0 and R1 remain) — the cascade stops.

### Obstruction columns are exempt

The obstruction columns (sign bit and quadratic-character columns) are structural: they encode
constraints that G.E needs, not factor-base primes. A weight-1 sign column does not mean that
the relation containing it is useless — it means that exactly one relation has a negative
rational norm. Treating it as a singleton would incorrectly discard that relation. The loop
scans only `col in 0..matrix.obstruction_col_start`.

### Provenance is unchanged by singleton removal

Singleton removal drops rows, never merges them. Each surviving row's provenance remains its
original singleton set `[i]` — the index of the original relation. This is the design decision
documented in the `remove_singletons` docstring:

> Provenance is preserved unchanged: singleton removal drops rows, never merges them, so each
> surviving row's provenance is its original singleton set.

The provenance map is the thread that G.F uses to recover original (a, b) pairs from a null-space
vector. Singleton removal does not disturb it.

---

## 25. Excess and the Clique/Pruning Step

### Defining excess

The **excess** of the matrix is:

$$
\text{excess} = \text{rows} - (\text{columns} - \text{obstruction\_count})
$$

implemented as:

```rust
pub fn excess(&self) -> isize {
    self.rows.len() as isize - (self.num_cols as isize - self.obstruction_count as isize)
}
```

The denominator `columns − obstruction_count` is the number of non-obstruction columns — the
"active" dimension of the matrix. For the null space to be non-trivial (i.e., for there to
exist at least one non-zero null-space vector), the matrix must have more rows than active
columns: excess must be positive.

The constant `EXCESS_FLOOR = 20` is the minimum excess that the pruning step must preserve:

```rust
/// Minimum excess G.D.2 pruning must preserve.
///
/// excess = rows − (columns − obstruction_count). At toy scale any positive excess
/// suffices for a non-trivial nullspace; 20 is a conservative floor that keeps the
/// matrix well-overdetermined.
pub const EXCESS_FLOOR: usize = 20;
```

**Principle-4 annotation (scale-dependent floor).** At toy scale, any positive excess suffices
for a non-trivial null space — the floor of 20 is conservative and is never approached in
practice because the toy corpus is small. At cryptographic scale (RSA-768+), the excess floor
is typically set to ~200 or a fraction of the column count, balancing the need for many
independent null-space vectors (each gives one congruence-of-squares candidate) against the
cost of carrying extra rows through the linear algebra step. The floor is the knob that trades
matrix density against null-space dimension; at toy scale this trade-off is invisible.

### The clique view and greedy pruning

In the graph view, a **clique** is a set of relations (edges) that all share a common prime
(node). A clique is dense: every pair of relations in the clique shares at least one column.
Dense cliques increase the total matrix weight (total number of set entries) without
proportionally increasing the null-space dimension.

The `prune_cliques` function in `gnfs/src/filter/merge.rs` approximates clique pruning by a
greedy heuristic: repeatedly remove the heaviest row (the relation with the most set columns)
as long as the excess exceeds `EXCESS_FLOOR`:

```rust
pub fn prune_cliques(mut matrix: SparseMatrix) -> SparseMatrix {
    loop {
        if matrix.excess() <= EXCESS_FLOOR as isize { break; }
        if matrix.rows.is_empty() { break; }

        // Find the row with the maximum weight (number of set columns).
        let max_weight = matrix.rows.iter().map(|r| r.cols.len()).max().unwrap_or(0);
        if max_weight == 0 { break; }

        let row_idx = matrix.rows.iter().enumerate()
            .find(|(_, r)| r.cols.len() == max_weight)
            .map(|(i, _)| i).unwrap();

        matrix.remove_row(row_idx);
    }
    matrix
}
```

The heaviest rows are the most "connected" relations — those that share columns with many other
relations. Removing them reduces the total matrix weight while preserving the excess floor.
Ties are broken by row index (lowest index first) for determinism.

The KAT `kat_prune_cliques_respects_excess_floor` in `merge_kat.rs` verifies that after
pruning, `excess() == EXCESS_FLOOR` exactly (when the initial excess is above the floor).

---

## 26. Merging

### Eliminating a column by XOR

A column of weight 2 appears in exactly two rows. XOR-merging those two rows eliminates the
column: the shared column cancels in the symmetric difference (GF(2) addition), and the merged
row contains only the columns that appeared in exactly one of the two source rows.

The `MatrixRow::xor_merge` method computes the symmetric difference of column sets and the
union of provenance sets in a single O(n + m) merge pass:

```rust
pub fn xor_merge(&self, other: &MatrixRow) -> MatrixRow {
    // Symmetric difference of two sorted vecs — O(n + m).
    let mut cols = Vec::new();
    let mut i = 0; let mut j = 0;
    while i < self.cols.len() && j < other.cols.len() {
        match self.cols[i].cmp(&other.cols[j]) {
            Ordering::Less    => { cols.push(self.cols[i]); i += 1; }
            Ordering::Greater => { cols.push(other.cols[j]); j += 1; }
            Ordering::Equal   => { /* XOR = 0, skip both */ i += 1; j += 1; }
        }
    }
    cols.extend_from_slice(&self.cols[i..]);
    cols.extend_from_slice(&other.cols[j..]);
    // ... union of provenance sets (same pattern) ...
    MatrixRow { cols, provenance }
}
```

### 2-way merge: two rows sharing a column

For a weight-2 column c with rows R₀ and R₁:

1. Compute `merged = R₀.xor_merge(R₁)`.
2. Remove R₀ and R₁ from the matrix.
3. Append `merged`.
4. Update `col_weights`: decrement for all columns in R₀ and R₁, increment for all columns
   in `merged`. The shared column c is decremented twice and not incremented → weight drops
   to 0 (eliminated).

The net effect on row count: two rows become one (net −1). The net effect on column count:
column c is eliminated (weight 0). The excess formula changes: both the numerator and the
effective denominator decrease, so the excess may increase or decrease depending on the
specific column weights.

### k-way merge for k = 3

For a weight-3 column c with rows R₀, R₁, R₂, the merge is:

```text
tmp = R₀ ⊕ R₁
merged = tmp ⊕ R₂
```

implemented in `merge_three_rows`. The three source rows are removed and `merged` is appended.
The shared column c is decremented three times and not incremented → weight drops to 0.

The `merge_columns` function performs two passes:

```rust
pub fn merge_columns(mut matrix: SparseMatrix) -> SparseMatrix {
    matrix = merge_pass(&mut matrix, 2);  // weight-2 pass
    matrix = merge_pass(&mut matrix, 3);  // weight-3 pass (after re-scan)
    matrix
}
```

The weight-2 pass is performed first; after it completes, some weight-3 columns may have
become weight-2 or weight-1 (because earlier merges in the weight-2 pass changed their
weights). The weight-3 pass re-scans `col_weights` and processes the remaining weight-3
columns.

### The Cavallar weight-cost heuristic

In production NFS, merges are ordered by a **weight-cost heuristic** (Cavallar 1996): prefer
merges that minimise the increase in total matrix weight. The total weight is the sum of all
`col_weights` — equivalently, the total number of set entries in the matrix. A merge of a
weight-k column eliminates that column (saving k entries) but the merged row may be heavier
than the sum of the source rows minus the shared column (adding entries). The Cavallar
heuristic picks the merge with the smallest net weight increase.

This implementation uses a simplified Cavallar ordering: process columns in order of increasing
weight (weight-2 first, then weight-3), breaking ties by column index. This is correct in
principle — lower-weight columns are cheaper to merge — but the full heuristic would also
consider the weight of the merged row.

**Principle-4 annotation (Cavallar weight-cost ordering).** At toy scale, the matrix is small
enough that any merge order gives a tractable matrix for G.E. The weight-saving the Cavallar
heuristic buys is under-exposed: the difference between the simplified ordering and the optimal
ordering is a few percent of total weight, which is invisible when the matrix has tens of rows.
At cryptographic scale (RSA-768+), the matrix has millions of rows and hundreds of thousands
of columns; the Cavallar heuristic can reduce the total weight by 30–50% compared to a naive
ordering, which directly reduces the cost of the structured Gaussian elimination in G.E. The
module docstring in `merge.rs` annotates this disconnect explicitly.

---

## 27. The Provenance Map as the Thread to G.F

### What provenance tracks

Every `MatrixRow` carries a `provenance: Vec<usize>` — a sorted, deduplicated list of original
relation indices (indices into the `Vec<Relation>` passed to `build_matrix`):

```rust
pub struct MatrixRow {
    /// Sorted column indices where this row has a 1 in GF(2).
    pub cols: Vec<usize>,
    /// Sorted, deduplicated original relation indices this row derives from.
    pub provenance: Vec<usize>,
}
```

For a freshly built row, `provenance = [i]` (the original relation index). Under merging,
`provenance` is combined by sorted union: if R₀ has provenance `[0, 2]` and R₁ has provenance
`[1, 3]`, then `R₀.xor_merge(R₁)` has provenance `[0, 1, 2, 3]`.

Singleton removal drops rows without merging, so provenance is unchanged. Clique pruning also
drops rows without merging. Only `merge_columns` grows provenance sets.

### The G.F expansion

G.E (linear algebra) finds a null-space vector — a subset of rows in the final matrix whose
GF(2) column-sum is the zero vector. G.F (square root) needs to expand this subset back to
the original (a, b) pairs to compute the congruence of squares.

The expansion is:

1. G.E selects a subset of rows in the final matrix (the null-space vector).
2. For each selected row, collect its `provenance` set.
3. Take the union of all collected provenance sets — this gives the set of original relation
   indices.
4. Look up the original `Relation` objects by index to recover the (a, b) pairs.

The KAT `kat_d_end_to_end_provenance` in `merge_kat.rs` verifies this property: for each row
in the final matrix, the XOR of the GF(2) column sets of all original relations indexed by its
provenance equals that row's column set.

### Why provenance stores original indices, not pre-reduced row sums

The provenance map stores original relation indices, not the GF(2) column set of the merged
row. This is the over-specification documented in the C-Matrix contract:

> Over-specify the provenance map now: store original relation indices, not pre-reduced row
> sums, so G.F can recover the actual (a, b) pairs.

The reason is that G.F needs the actual (a, b) pairs to compute the algebraic square root —
the product of (a − b·α) in the number field K = ℚ(α). The GF(2) column set of a merged row
is the XOR of the original exponent parities; it does not contain the original (a, b) values.
Storing the GF(2) column set instead of the original indices would make G.F impossible without
re-deriving the (a, b) pairs from the exponent vectors, which would require keeping the full
`Vec<Relation>` in scope anyway. The provenance map is the clean interface: G.F receives the
final matrix and the original `Vec<Relation>`, and uses the provenance map to bridge them.

---

## 28. What G.E Inherits

### The `SparseMatrix` contract (C-Matrix)

The output of the full filtering pipeline — `build_matrix` → `remove_singletons` →
`prune_cliques` → `merge_columns` — is a `SparseMatrix` that G.E consumes directly:

```rust
pub struct SparseMatrix {
    /// The rows of the matrix, each with its column set and provenance.
    pub rows: Vec<MatrixRow>,
    /// Total number of columns = FactorBase::matrix_width().
    pub num_cols: usize,
    /// Index of the first obstruction column = rational_size + algebraic_size.
    pub obstruction_col_start: usize,
    /// Number of obstruction columns = FactorBase::obstruction_count.
    pub obstruction_count: usize,
    /// col_weights[c] = number of rows with a 1 in column c.
    pub col_weights: Vec<u32>,
}
```

The key properties G.E inherits:

- **Dimensions.** `rows.len()` rows, `num_cols` columns. The excess `rows.len() − (num_cols −
  obstruction_count)` is ≥ `EXCESS_FLOOR` (guaranteed by `prune_cliques`), ensuring a
  non-trivial null space.

- **Obstruction columns.** The sign column (at `obstruction_col_start`) is populated by
  `build_matrix` from `relation.rational_sign`. The quadratic-character columns (at
  `obstruction_col_start + 1` through `num_cols − 1`) are carried as zeros by G.D — G.E fills
  them in before running the null-space computation.

- **Row-major store + `col_weights` side table.** The row-major representation (each row is a
  sorted `Vec<usize>` of set column indices) supports the row-XOR operations that structured
  Gaussian elimination performs. The `col_weights` side table supports column-weight queries
  without scanning all rows — G.E uses it to choose pivot columns.

- **Provenance map.** Each row carries its provenance set. G.E does not modify provenance; it
  passes the final matrix (with provenance intact) to G.F.

### The pipeline in full

The complete G.D pipeline, as exercised by the KATs in `merge_kat.rs`:

```rust
// 1. Build the initial matrix from the relation corpus.
let matrix = build_matrix(&relations, &fb);

// 2. Remove singleton columns to fixpoint.
let matrix = remove_singletons(matrix);

// 3. Prune heavy rows while excess > EXCESS_FLOOR.
let matrix = prune_cliques(matrix);

// 4. Eliminate weight-2 and weight-3 columns by XOR-merging.
let matrix = merge_columns(matrix);

// matrix is now ready for G.E (linear algebra over GF(2)).
```

Each step is a pure function (consumes and returns `SparseMatrix`), making the pipeline
composable and testable in isolation.

---

## 29. KAT Summary (G.D)

The following table lists the key known-answer tests across the filter module, with the
mathematical fact each one verifies. All tests are in `gnfs/tests/merge_kat.rs`.

| Test | Mathematical fact verified |
|------|---------------------------|
| `kat_a_two_way_merge_correctness` | Weight-2 column eliminated; merged row has correct cols (symmetric difference) and provenance (union) |
| `kat_b_determinism` | Full pipeline (build → singletons → prune → merge) is deterministic for a fixed corpus |
| `kat_c_cado_nfs_oracle` | (Ignored) CADO-NFS oracle for filtered matrix dimensions — gated when CADO absent |
| `kat_d_end_to_end_provenance` | For each final row, XOR of provenance relations' GF(2) column sets equals the row's cols |
| `kat_prune_cliques_respects_excess_floor` | After pruning, `excess() == EXCESS_FLOOR` exactly when initial excess > floor |

---

## 30. What Comes Next

Filtering is the fourth stage of the GNFS pipeline. The output — a `SparseMatrix` with
provenance map — is the input to the linear algebra and square-root stages.

**G.E (linear algebra over GF(2))** takes the filtered matrix and finds its null space. It
fills in the quadratic-character obstruction columns (currently zero in the G.D output), then
runs structured Gaussian elimination to find a basis for the null space. Each null-space vector
identifies a subset of rows whose GF(2) column-sum is zero — a candidate congruence of squares.

**G.F (square root)** takes a null-space vector from G.E and expands it through the provenance
map to recover the original (a, b) pairs. It then computes the product of all rational norms
(a perfect square in ℤ) and the product of all algebraic norms (a perfect square in K = ℚ(α)),
and extracts the square roots to form the congruence x² ≡ y² (mod N). A non-trivial GCD of
x − y and N yields a factor.

**D.A (NFS-DL)** adapts the same filtering infrastructure for discrete logarithm computation.
The `SparseMatrix` and provenance map are reused directly; the difference is in the linear
algebra step (GF(ℓ) instead of GF(2)) and the square-root step (Schirokauer maps instead of
the algebraic square root).

---

## Further Reading (G.D)

1. **Cavallar, S. (2002).** *Strategies in filtering in the number field sieve.* In: Bosma, W.
   (ed.) Algorithmic Number Theory (ANTS-IV), LNCS 1838. Springer. The original source for the
   weight-cost merge heuristic and the clique/excess balance.

2. **Buhler, J., Lenstra, H. W. Jr., and Pomerance, C. (1993).** *Factoring integers with the
   number field sieve.* In: Lenstra, A. K., and Lenstra, H. W. Jr. (eds.) The Development of
   the Number Field Sieve, LNM 1554. Springer. The original description of the filtering step
   and the relation matrix.

3. **Franke, J., and Kleinjung, T. (2006).** *Continued fractions and lattice sieving.* CADO
   workshop on integer factorization. Describes the filtering pipeline used in CADO-NFS,
   including the singleton removal and merge strategies.

4. **Pomerance, C. (1996).** *A tale of two sieves.* Notices of the AMS, 43(12), 1473–1485.
   An accessible introduction to the relation matrix and the linear algebra step, with the
   filtering step explained at a high level.

5. **Wiedemann, D. (1986).** *Solving sparse linear equations over finite fields.* IEEE
   Transactions on Information Theory, 32(1), 54–62. The Wiedemann algorithm used in G.E for
   the null-space computation — the downstream consumer of the filtered `SparseMatrix`.

---

# Linear Algebra: A Code-Tour Chapter

This chapter explains the `gnfs/src/linalg` module — the linear algebra stage of the General
Number Field Sieve. It is organised by the implementation, not by mathematical abstraction: each
section introduces the mathematics, then shows how it is realised in code. A reader who has read
the filtering chapter (§21–§30 above) can read this one without consulting the source.

The chapter covers the filtered matrix as a GF(2) linear system, the nullspace as a congruence
of squares, the role of sign and quadratic-character columns, block Lanczos, block Wiedemann,
the kernel-vector → provenance → original-relations thread, and the C-LinAlg contract.

---

## 31. The Filtered Matrix as a GF(2) Linear System

### From filtering to linear algebra

The output of G.D filtering is a `SparseMatrix` — a sparse GF(2) matrix A with m rows and n
columns. Each row corresponds to a filtered relation (or a merged aggregate of relations), and
each column corresponds to a prime or obstruction entry in the factor base. The entry A[i][j]
is 1 if the j-th prime appears to an odd total exponent in the i-th relation's norm product,
and 0 otherwise.

The goal of G.E is to find vectors in the **left nullspace** of A: subsets of rows whose GF(2)
column-sum is the zero vector. Formally, we seek non-zero vectors v ∈ GF(2)^m such that:

$$
v^T A = 0 \pmod{2}
$$

Equivalently, we seek vectors in the nullspace of A^T (the right nullspace of the transpose).

### Why the left nullspace

A left nullspace vector v is a 0/1 indicator over rows. The rows it selects are a subset S of
filtered relations. The GF(2) column-sum being zero means: for every prime p in the factor
base, the total exponent of p across all relations in S is even. This is exactly the condition
for the product of the corresponding norms to be a perfect square.

### The `SparseMatrix` contract (C-Matrix)

G.E inherits the `SparseMatrix` from G.D unchanged:

```rust
pub struct SparseMatrix {
    /// The rows of the matrix, each with its column set and provenance.
    pub rows: Vec<MatrixRow>,
    /// Total number of columns = FactorBase::matrix_width().
    pub num_cols: usize,
    /// Index of the first obstruction column = rational_size + algebraic_size.
    pub obstruction_col_start: usize,
    /// Number of obstruction columns = FactorBase::obstruction_count.
    pub obstruction_count: usize,
    /// col_weights[c] = number of rows with a 1 in column c.
    pub col_weights: Vec<u32>,
}
```

The column layout is:

```text
[  rational primes  |  algebraic ideals  |  sign  |  QC columns  ]
 0 .. rational_size   rational_size ..      obs_start  obs_start+1 ..
                       obs_start            (col 0)    (cols 1..num_qc)
```

G.D populates the rational and algebraic columns and the sign column. G.E fills in the
quadratic-character (QC) columns before running the nullspace solver.

### Toy-scale matrix dimensions

The KAT matrices in `gnfs/tests/lanczos_kat.rs` and `gnfs/tests/wiedemann_kat.rs` use a
hand-built 6 × 4 matrix:

```text
    col: 0 1 2 3
row 0:   1 1 0 0   ← provenance [0]
row 1:   0 1 1 0   ← provenance [1]
row 2:   1 0 1 0   ← provenance [2]
row 3:   1 1 0 0   ← provenance [3]  (duplicate of row 0)
row 4:   0 0 0 1   ← provenance [4]
row 5:   0 0 0 1   ← provenance [5]  (duplicate of row 4)
```

Known left nullspace vectors: `{0, 3}` (rows 0 and 3 are identical), `{4, 5}` (rows 4 and 5
are identical), and `{0, 1, 2}` (the XOR of rows 0, 1, 2 is zero). The kernel dimension is 3.

---

## 32. The Nullspace as a Congruence of Squares

### From nullspace vector to congruence

A left nullspace vector v — a subset S of filtered-matrix rows — has a precise algebraic
meaning. Let the rows in S correspond to relations with rational norms N_rat(aᵢ, bᵢ) and
algebraic norms N_alg(aᵢ, bᵢ). The GF(2) column-sum being zero means:

$$
\prod_{i \in S} N_{\mathrm{rat}}(a_i, b_i) = X^2 \quad \text{for some } X \in \mathbb{Z}
$$

$$
\prod_{i \in S} N_{\mathrm{alg}}(a_i, b_i) = Y^2 \quad \text{for some } Y \in K = \mathbb{Q}(\alpha)
$$

The first equation holds because the rational exponent parities all cancel (every prime appears
to an even total exponent). The second holds for the same reason on the algebraic side — with
the additional guarantee provided by the sign and QC columns (§33).

### The congruence of squares

The rational norm N_rat(a, b) = a − b·m is a linear form in (a, b). The product of the
rational norms over S is:

$$
\prod_{i \in S} (a_i - b_i \cdot m) = X^2 \pmod{N}
$$

The algebraic norm N_alg(a, b) = b^d · f(a/b) is the norm of the ideal (a − b·α) in ℤ[α].
The product of the algebraic norms over S is the norm of the product ideal:

$$
\prod_{i \in S} (a_i - b_i \cdot \alpha) = \beta^2 \quad \text{in } K = \mathbb{Q}(\alpha)
$$

Taking the norm of both sides: Norm(β²) = Norm(β)² = Y² where Y = Norm(β) ∈ ℤ. Setting
x = X mod N and y = Y mod N gives x² ≡ y² (mod N). If x ≢ ±y (mod N), then gcd(x − y, N)
is a non-trivial factor of N.

### Why the matrix encodes the right thing

Each row of the matrix is the GF(2) exponent vector of a relation: the parity of each prime's
exponent in the norm factorisation. The GF(2) column-sum of a subset S being zero is exactly
the condition that every prime appears to an even total exponent — i.e., the product of norms
is a perfect square. The linear algebra step is the bridge between the sieve output (individual
smooth pairs) and the factoring step (a congruence of squares).

---

## 33. Why the Sign and Quadratic-Character Columns Are Needed

### The sign column

The rational norm N_rat(a, b) = a − b·m can be negative (when a < b·m). A perfect square must
be positive, so the product of the rational norms over S must be positive. The sign column
(at `obstruction_col_start`) records the parity of the sign: it is 1 if the rational norm is
negative, 0 if positive. For the product to be a positive perfect square, the total number of
negative norms in S must be even — i.e., the sign column must sum to zero over S.

The `Relation` type records this directly:

```rust
pub struct Relation {
    // ...
    /// True if the rational norm a − b·m is negative.
    pub rational_sign: bool,
}
```

`build_matrix` populates the sign column from `relation.rational_sign`. A nullspace vector
that satisfies the sign column constraint has an even number of negative rational norms, so
their product is positive.

### The quadratic-character columns

The algebraic side requires more care. Even if the product of algebraic norms is a perfect
square integer, the algebraic square root β (satisfying Norm(β) = Y) might not exist in K.
The issue is that the product of the ideals (aᵢ − bᵢ·α) might be a square in the ideal group
of ℤ[α] without being a square of a principal ideal — i.e., the product might not be a
principal ideal generated by a square element.

The quadratic-character (QC) columns resolve this. For each auxiliary prime q that splits
completely in K = ℚ[x]/(f), the QC column records the Legendre-symbol parity of the algebraic
norm at q:

$$
\text{QC}_{q}(a, b) = \left(\frac{N_{\mathrm{alg}}(a, b)}{q}\right) \bmod 2
$$

where (·/q) is the Legendre symbol. A nullspace vector that satisfies all QC column constraints
guarantees that the product of the algebraic norms is a square in K, not merely a square
integer. Without QC columns, the algebraic square root might not exist in K even if the norm
is a square.

### The `populate_qc_columns` function

G.E fills in the QC columns before running the nullspace solver:

```rust
pub fn populate_qc_columns(
    matrix: &mut SparseMatrix,
    relations: &[Relation],
    fb: &FactorBase,
    poly: &PolyPair,
    qc_primes: &[u64],
) {
    for row in matrix.rows.iter_mut() {
        for (k, &q) in qc_primes.iter().enumerate() {
            let col = matrix.obstruction_col_start + 1 + k;
            let mut parity = false;
            for &rel_idx in &row.provenance {
                let rel = &relations[rel_idx];
                let norm = algebraic_norm_mod_q(&rel.a, &rel.b, &poly.f, q);
                parity ^= legendre_parity(norm, q);
            }
            if parity {
                // Insert col into row.cols (maintaining sorted order).
                match row.cols.binary_search(&col) {
                    Ok(_) => {}
                    Err(pos) => { row.cols.insert(pos, col); }
                }
            }
        }
    }
}
```

The QC primes are selected by `select_qc_primes`: the first `num_qc` primes q > B_alg such
that f splits completely mod q (i.e., f has deg(f) distinct roots mod q). Splitting completely
ensures the Legendre symbol is well-defined and non-degenerate.

The Legendre symbol is computed via Euler's criterion: (a/q) ≡ a^((q−1)/2) (mod q). The
parity is 1 iff the symbol is −1 (a is a quadratic non-residue mod q):

```rust
fn legendre_parity(norm: u64, q: u64) -> bool {
    if norm == 0 { return false; }
    let exp = (q - 1) / 2;
    let result = pow_mod(norm, exp, q);
    result == q - 1
}
```

**Principle-4 annotation.** At toy scale, `DEFAULT_NUM_QC = 10` QC columns suffice. At NFS
scale, 20–50 QC columns are typical. The number of QC columns is a scale knob: more columns
give a stronger guarantee that the algebraic square root exists, at the cost of a wider matrix.

---

## 34. The C-LinAlg Substrate

### The three frozen types

The C-LinAlg contract (frozen at G.E.1) defines three types that all downstream consumers
(G.E.2, G.E.3, G.F, D.B) use:

**`BlockVec`** — a block of `BLOCK_WIDTH = 64` GF(2) vectors, each of length `num_rows`,
packed into a `Vec<u64>`. The layout is "row of words": `data[i]` is a `u64` whose bit j is
the j-th vector's value at row i. This makes iterating over rows contiguous in memory and
keeps the 64 vectors interleaved bit-by-bit within each word.

```rust
pub struct BlockVec {
    /// Packed row data: `data[row]` bit `j` = vector `j`'s value at `row`.
    pub data: Vec<u64>,
    /// Number of rows (the vector dimension).
    pub num_rows: usize,
}
```

**`MatrixOperator`** — a view of a `SparseMatrix` as a linear operator over GF(2). Provides
`apply` (A·V) and `apply_transpose` (Aᵀ·V) for block vectors. Both solvers consume this
interface exclusively; they never read `SparseMatrix` fields directly.

```rust
pub struct MatrixOperator<'a> {
    matrix: &'a SparseMatrix,
}
```

**`KernelVector`** — a vector in the left nullspace of the matrix, represented as a sorted,
deduplicated `Vec<usize>` of filtered-matrix row indices.

```rust
pub struct KernelVector {
    /// Sorted, deduplicated row indices into the filtered matrix.
    pub row_indices: Vec<usize>,
}
```

### The `MatrixOperator` apply operations

The `apply` operation (A·V) computes the GF(2) matrix-vector product for all 64 block vectors
simultaneously. For each row i of the matrix, the result is the XOR of `v.data[c]` for each
column c in that row:

```rust
pub fn apply(&self, v: &BlockVec) -> BlockVec {
    let mut result = BlockVec::zeros(self.num_rows());
    for (i, row) in self.matrix.rows.iter().enumerate() {
        let mut word = 0u64;
        for &c in &row.cols {
            word ^= v.data[c];
        }
        result.data[i] = word;
    }
    result
}
```

The key insight: `v.data[c]` is a `u64` whose bit j is vector j's value at column c. XOR-ing
all `v.data[c]` for c in the row gives a `u64` whose bit j is the GF(2) dot product of the
row with vector j. All 64 dot products are computed in a single XOR loop.

The `apply_transpose` operation (Aᵀ·V) scatters contributions on-the-fly: for each row i,
for each column c in that row, XOR `v.data[i]` into `result.data[c]`:

```rust
pub fn apply_transpose(&self, v: &BlockVec) -> BlockVec {
    let mut result = BlockVec::zeros(self.num_cols());
    for (i, row) in self.matrix.rows.iter().enumerate() {
        let vi = v.data[i];
        if vi == 0 { continue; }
        for &c in &row.cols {
            result.data[c] ^= vi;
        }
    }
    result
}
```

### The `BLOCK_WIDTH` scale knob

```rust
/// Block width: 64 vectors packed into machine words.
///
/// Principle-4 annotation: at toy scale a single word suffices and the blocking overhead
/// is invisible; at NFS scale the word-wide block is the inner loop's cache-friendly unit.
/// The width is the scale knob — D.B may widen to 128 or parameterise over block width.
pub const BLOCK_WIDTH: usize = 64;
```

At toy scale (6 × 4 matrix), a single 64-bit word is more than enough to represent the entire
block. The blocking overhead is invisible. At NFS scale (millions of rows), the 64-wide block
is the cache-friendly inner loop unit: each `apply` call processes 64 vectors in a single pass
over the matrix, amortising the memory-bandwidth cost of loading each row once.

---

## 35. Block Lanczos: Krylov Subspaces over GF(2)

### The symmetric reduction

Both solvers work with the symmetric matrix B = A · Aᵀ (m × m), whose left nullspace equals
the left nullspace of A. This is because:

$$
A^T v = 0 \implies A A^T v = 0
$$

and conversely, if B·v = 0 then A·Aᵀ·v = 0, so Aᵀ·v is in the nullspace of A, and if A has
full column rank then Aᵀ·v = 0. Working with B avoids the need to handle the non-square
structure of A directly.

The B·v product is computed as two operator applications:

```rust
// w_cur = B * v_cur = A * (A^T * v_cur).
let at_v = op.apply_transpose(&v_cur);
let w_cur = op.apply(&at_v);
```

### The Krylov subspace

The block Lanczos algorithm builds a Krylov basis for B. Starting from a random block vector
V₀ ∈ GF(2)^{m × 64}, the Krylov sequence is:

$$
V_0,\; B V_0,\; B^2 V_0,\; \ldots
$$

The algorithm constructs an orthogonal basis for the Krylov subspace span{V₀, BV₀, B²V₀, …}
using a three-term recurrence. At each step, the recurrence advances the block vector using
the A-orthogonality condition.

### The A-orthogonality condition and the three-term recurrence

The block Lanczos recurrence is:

$$
V_{k+1} = B V_k - V_k \alpha_k - V_{k-1} \beta_k
$$

where the coefficient matrices α_k and β_k are chosen so that V_{k+1} is A-orthogonal to all
previous block vectors. The A-orthogonality condition is:

$$
V_i^T B V_j = 0 \quad \text{for } i \neq j
$$

The coefficients are:

$$
\alpha_k = S_k^{-1} (V_k^T B^2 V_k), \quad \beta_k = S_{k-1}^{-1} (V_{k-1}^T B V_k)
$$

where S_k = V_k^T B V_k is the BLOCK_WIDTH × BLOCK_WIDTH inner product matrix. The inverse
S_k^{-1} is computed by GF(2) Gaussian elimination.

In the implementation:

```rust
// alpha_raw = v_cur^T * B * w_cur (BLOCK_WIDTH × BLOCK_WIDTH).
let alpha_raw = v_cur.inner_product_matrix(&bw_cur);
// alpha = s_inv * alpha_raw (restricted to active columns).
let alpha = gf2_matmul_block(s_inv, alpha_raw, active_mask);

// beta_raw = v_prev^T * w_cur (BLOCK_WIDTH × BLOCK_WIDTH).
let beta_raw = v_prev.inner_product_matrix(&w_cur);
// beta = s_prev_inv * beta_raw (restricted to previously active columns).
let beta = gf2_matmul_block(s_prev_inv, beta_raw, active_mask_prev);

// v_next = w_cur - v_cur * alpha - v_prev * beta (over GF(2), - = +).
let mut v_next = w_cur.clone();
block_vec_sub_matmul(&mut v_next, &v_cur, alpha);
block_vec_sub_matmul(&mut v_next, &v_prev, beta);
```

### The GF(2) self-orthogonality problem and winnowing

Over ℝ, the Lanczos recurrence is guaranteed to produce orthogonal vectors because the inner
product is positive-definite. Over GF(2), this guarantee fails: a **nonzero** vector v can
satisfy v^T B v = 0. This is the self-orthogonality problem.

Self-orthogonality occurs exactly when B·v = 0, i.e., when v is already in the nullspace of B
(and hence in the left nullspace of A). When the inner product matrix S_k = V_k^T B V_k has
a zero column, the corresponding block column of V_k is self-orthogonal — it is already a
nullspace vector.

The **winnowing** step handles this: at each iteration, the algorithm identifies which block
columns are "active" (those where S_k has a non-zero pivot) and which are "inactive" (those
where S_k has a zero column). Inactive columns are checked for nullspace membership:

```rust
// Find active columns via GF(2) Gaussian elimination on s.
let (active_mask, s_inv) = gf2_block_pivot(s);

// Collect kernel candidates from inactive columns.
let inactive_mask = !active_mask;
let mut bits = inactive_mask;
while bits != 0 {
    let j = bits.trailing_zeros() as usize;
    bits &= bits - 1;

    let col = v_cur.column(j);
    if col.iter().any(|&b| b) {
        let col_bv = BlockVec::from_columns(&[col.clone()]);
        let at_col = op.apply_transpose(&col_bv);
        let is_zero = at_col.data.iter().all(|&x| x == 0);
        if is_zero {
            let kv = KernelVector::from_mask(&col);
            if !kv.is_empty() {
                results.push(kv);
            }
        }
    }
}
```

The `gf2_block_pivot` function performs GF(2) Gaussian elimination on the 64 × 64 inner
product matrix S, returning the pivot column mask (active columns) and the inverse of the
pivot submatrix (used in the recurrence):

```rust
fn gf2_block_pivot(s: [u64; BLOCK_WIDTH]) -> (u64, [u64; BLOCK_WIDTH]) {
    let mut s_work = s;
    let mut inv = [0u64; BLOCK_WIDTH];
    for i in 0..BLOCK_WIDTH { inv[i] = 1u64 << i; } // identity
    let mut pivot_mask = 0u64;
    for col in 0..BLOCK_WIDTH {
        // Find pivot row for this column.
        let mut found = BLOCK_WIDTH;
        for row in col..BLOCK_WIDTH {
            if (s_work[row] >> col) & 1 == 1 { found = row; break; }
        }
        if found == BLOCK_WIDTH { continue; } // no pivot — inactive column
        s_work.swap(col, found);
        inv.swap(col, found);
        pivot_mask |= 1u64 << col;
        // Full reduced row echelon form.
        for row in 0..BLOCK_WIDTH {
            if row != col && (s_work[row] >> col) & 1 == 1 {
                s_work[row] ^= s_work[col];
                inv[row] ^= inv[col];
            }
        }
    }
    // ...
    (pivot_mask, s_inv)
}
```

**Principle-4 annotation (self-orthogonality).** Self-orthogonality is a phenomenon that IS
exposed at toy scale. The KAT matrix in `lanczos_kat.rs` is deliberately constructed with
duplicate rows (rows 0 and 3 are identical; rows 4 and 5 are identical). Duplicate rows
produce self-orthogonal block columns in the starting vector, exercising the winnowing path.
The KAT comment explains this explicitly:

> To force this path in a KAT, we use a matrix with duplicate rows. If rows i and j are
> identical, then e_i XOR e_j is in the left nullspace of A. When the random starting block
> vector has a component along e_i XOR e_j, that component is self-orthogonal under B and
> triggers the winnowing.

**Principle-4 annotation (block width).** The block width `BLOCK_WIDTH = 64` is the scale
knob. At toy scale (6 × 4 matrix), the entire matrix fits in a handful of words and the
blocking overhead is invisible — a single-vector Lanczos would be equally fast. At NFS scale
(millions of rows), the 64-wide block amortises the cost of loading each matrix row once
across 64 simultaneous vector operations, giving a ~64× speedup over the single-vector variant.

---

## 36. Block Wiedemann: Krylov Sequences and Berlekamp-Massey

### The Wiedemann approach

Block Wiedemann takes a different route to the same destination. Instead of building an
orthogonal Krylov basis (Lanczos), it computes a scalar Krylov sequence and uses
Berlekamp-Massey to find the minimal polynomial of B restricted to a random direction.

For a random pair (x, y) ∈ GF(2)^m × GF(2)^m, the scalar Krylov sequence is:

$$
s_i = x^T B^i y, \quad i = 0, 1, 2, \ldots, 2m + 10
$$

The Berlekamp-Massey algorithm finds the shortest LFSR that generates this sequence — the
minimal polynomial f(z) of B in the direction (x, y). Then the kernel vector is extracted by
evaluating f(B)·y:

$$
w = f(B) \cdot y = \sum_{k=0}^{d} f_k \cdot B^k \cdot y
$$

If A^T·w = 0 and w ≠ 0, then w is a left nullspace vector of A.

### The Krylov sequence computation

```rust
fn krylov_sequence(
    op: &MatrixOperator<'_>,
    m: usize,
    x: &[bool],
    y: &[bool],
    seq_len: usize,
) -> Vec<bool> {
    let y_bv = bool_vec_to_blockvec(y, m);
    let mut v = y_bv;
    let mut sequence = Vec::with_capacity(seq_len);

    for _ in 0..seq_len {
        // s_i = x^T * v = XOR of v[j] for j where x[j] = true.
        let s = inner_product_scalar(x, &v);
        sequence.push(s);

        // v = B * v = apply(apply_transpose(v)).
        let at_v = op.apply_transpose(&v);
        v = op.apply(&at_v);
    }
    sequence
}
```

Each step applies B = A·Aᵀ to the current vector, then takes the scalar inner product with x.
The sequence length 2m + 10 is sufficient for Berlekamp-Massey to find the minimal polynomial
of degree at most m.

### The Berlekamp-Massey algorithm

Berlekamp-Massey finds the shortest LFSR (linear feedback shift register) that generates a
given GF(2) sequence. The output is the minimal polynomial f(z) = 1 + f₁z + … + f_d z^d such
that:

$$
\sum_{k=0}^{d} f_k \cdot s_{n-k} = 0 \quad \text{for all } n \geq d
$$

The implementation follows the standard GF(2) Berlekamp-Massey algorithm:

```rust
pub fn berlekamp_massey(s: &[bool]) -> Vec<bool> {
    let n = s.len();
    let mut c = vec![true]; // current LFSR polynomial C = 1
    let mut b = vec![true]; // previous polynomial B = 1
    let mut l: usize = 0;   // current LFSR length
    let mut m: usize = 1;   // steps since last length change

    for n_idx in 0..n {
        // Compute discrepancy d = s[n_idx] XOR sum_{i=1}^{L} C[i] * s[n_idx - i].
        let mut d = s[n_idx];
        for i in 1..=l {
            if n_idx >= i && c.len() > i && c[i] { d ^= s[n_idx - i]; }
        }

        if !d {
            m += 1; // no update needed
        } else if 2 * l <= n_idx {
            // Length must increase.
            let t = c.clone();
            xor_shifted(&mut c, &b, m); // C = C XOR z^m * B
            l = n_idx + 1 - l;
            b = t;
            m = 1;
        } else {
            xor_shifted(&mut c, &b, m); // C = C XOR z^m * B
            m += 1;
        }
    }
    c
}
```

The KAT in `wiedemann_kat.rs` verifies this on the Fibonacci sequence mod 2 (0, 1, 1, 0, 1,
1, …), which has minimal polynomial f(z) = 1 + z + z² (degree 2):

```rust
let s: Vec<bool> = vec![false, true, true, false, true, true, false, true, true, false];
let f = berlekamp_massey(&s);
assert_eq!(f.len(), 3); // degree 2
assert!(f[0] && f[1] && f[2]); // f(z) = 1 + z + z^2
```

### Kernel extraction via Horner's method

Once the minimal polynomial f(z) is known, the kernel vector is extracted by evaluating
f(B)·y via Horner's method:

$$
f(B) \cdot y = f_d \cdot y + B \cdot (f_{d-1} \cdot y + B \cdot (\ldots + B \cdot (f_1 \cdot y + B \cdot (f_0 \cdot y)) \ldots))
$$

```rust
fn eval_poly_on_krylov(op: &MatrixOperator<'_>, m: usize, y: &[bool], f: &[bool]) -> BlockVec {
    let d = f.len() - 1;
    let y_bv = bool_vec_to_blockvec(y, m);
    let mut w = if f[d] { y_bv.clone() } else { BlockVec::zeros(m) };

    for k in (0..d).rev() {
        // w = B * w.
        let at_w = op.apply_transpose(&w);
        w = op.apply(&at_w);
        // w = w XOR (f_k * y).
        if f[k] { w.xor_assign(&y_bv); }
    }
    w
}
```

The result w = f(B)·y satisfies B·w = 0 (since f is the minimal polynomial of B in the
direction (x, y)), and hence A^T·w = 0 — w is in the left nullspace of A.

### The parallelism payoff

The key architectural difference between Wiedemann and Lanczos is where the parallelism lives.
In block Lanczos, each iteration requires a global inner product (S_k = V_k^T B V_k) that
synchronises all 64 block columns. In block Wiedemann, the Krylov sequence computation for
each (x, y) pair is independent: the sequence {x^T B^i y} can be computed in parallel across
multiple (x, y) pairs with no global synchronisation per step.

At production NFS scale, the block Wiedemann algorithm uses `BLOCK_WIDTH` (x, y) pairs
simultaneously, distributing the Krylov sequence computation across multiple machines. Each
machine computes a subset of the sequences independently; the Berlekamp-Massey step is then
run on the combined sequences. This is the architecture used in the RSA-768 factorisation
(Kleinjung et al., 2010).

**Principle-4 annotation (Wiedemann parallelism).** At toy scale, the parallelism payoff is
invisible. The implementation uses a single (x, y) pair (the scalar Wiedemann variant) at
demonstration fidelity; the block variant would use `BLOCK_WIDTH` pairs simultaneously. The
module docstring annotates this explicitly:

> Block Wiedemann's payoff is distributed/parallel: the Krylov sequence {x^T A^i y} can be
> computed in parallel across multiple (x, y) pairs, with no global synchronisation per step
> (unlike block Lanczos, which requires a global inner product at each step). At toy scale,
> this parallelism is invisible — Lanczos is simpler and just as fast.

The implementation runs 4 independent (x, y) attempts to improve the probability of finding
a kernel vector:

```rust
let num_attempts = 4;
for attempt in 0..num_attempts {
    // ...
    let x = random_gf2_vec(m, rng_state);
    let y = random_gf2_vec(m, rng_state);
    if let Some(kv) = wiedemann_attempt(op, m, &x, &y) {
        if !results.iter().any(|r| r.row_indices == kv.row_indices) {
            results.push(kv);
        }
    }
}
```

---

## 37. The Kernel-Vector → Provenance → Original-Relations Thread

### What G.F consumes

G.F (the square-root step) needs the original (a, b) pairs — not the filtered-matrix row
indices. The bridge is the provenance map: each `MatrixRow` carries a sorted, deduplicated
list of original relation indices that merged into it during G.D filtering.

The `KernelVector` type provides the `expand_provenance` method that performs this expansion:

```rust
pub fn expand_provenance(&self, matrix: &SparseMatrix) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    for &i in &self.row_indices {
        let prov = &matrix.rows[i].provenance;
        // Symmetric difference of result and prov (both sorted).
        let mut merged = Vec::with_capacity(result.len() + prov.len());
        let mut ri = 0;
        let mut pi = 0;
        while ri < result.len() && pi < prov.len() {
            match result[ri].cmp(&prov[pi]) {
                std::cmp::Ordering::Less    => { merged.push(result[ri]); ri += 1; }
                std::cmp::Ordering::Greater => { merged.push(prov[pi]); pi += 1; }
                std::cmp::Ordering::Equal   => { ri += 1; pi += 1; } // cancel
            }
        }
        merged.extend_from_slice(&result[ri..]);
        merged.extend_from_slice(&prov[pi..]);
        result = merged;
    }
    result
}
```

The symmetric difference (XOR union) is the correct operation: if a relation index appears in
two provenance sets, it means the relation was merged into two different filtered rows, and
those two rows' contributions cancel in the GF(2) sum. The symmetric difference removes
cancelled indices, leaving only the original relations that contribute a net odd number of
times.

### The full thread

The complete kernel-vector → original-relations thread:

1. **G.E** runs `block_lanczos` or `block_wiedemann` on the filtered matrix, returning a
   `Vec<KernelVector>`. Each `KernelVector` is a sorted list of filtered-matrix row indices.

2. **G.F** calls `kv.expand_provenance(&matrix)` to get the set of original relation indices.
   This is the symmetric difference of the provenance sets of all selected rows.

3. **G.F** looks up the original `Relation` objects by index to recover the (a, b) pairs.

4. **G.F** computes the product of all rational norms (a perfect square in ℤ) and the product
   of all algebraic norms (a perfect square in K = ℚ(α)), and extracts the square roots to
   form the congruence x² ≡ y² (mod N).

The KAT `kat_3_round_trip_provenance` in `linalg_substrate_kat.rs` verifies this thread end-
to-end before G.F exists. For a matrix with hand-crafted provenance:

```rust
// Row 0: provenance = [0, 1], Row 1: provenance = [2], Row 2: provenance = [1, 3].
// Kernel vector {0, 2}: sym_diff([0,1], [1,3]) = [0, 3] (1 cancels).
let kv_02 = KernelVector::new(vec![0, 2]);
let expanded = kv_02.expand_provenance(&matrix);
assert_eq!(expanded, vec![0, 3]);

// Kernel vector {0, 1, 2} (valid nullspace vector):
// sym_diff([0,1], [2], [1,3]) = [0, 2, 3] (1 cancels from rows 0 and 2).
let kv_012 = KernelVector::new(vec![0, 1, 2]);
assert!(kv_012.verify(&matrix));
let expanded = kv_012.expand_provenance(&matrix);
assert_eq!(expanded, vec![0, 2, 3]);
```

### Why row indices, not a bit-mask

The `KernelVector` stores row indices rather than a bit-mask for three reasons:

1. G.F needs row indices to look up provenance; a bit-mask would require a scan.
2. Kernel vectors are sparse (typically a small fraction of rows); a bit-mask wastes space.
3. Solvers internally work with bit-packed block vectors, but convert to `KernelVector` on
   output — the conversion is O(rows) and happens once per kernel vector, not in the inner
   loop.

---

## 38. The C-LinAlg Contract and D.B Generalisation

### The frozen seam

The C-LinAlg contract (frozen at G.E.1) defines the interface that all downstream consumers
use. The three types — `BlockVec`, `MatrixOperator`, and `KernelVector` — are the frozen seam.
G.E.2 (block Lanczos), G.E.3 (block Wiedemann), G.F (square root), and D.B (NFS-DL linear
algebra over F_ℓ) all consume this interface directly.

The module docstring in `gnfs/src/linalg/mod.rs` states the contract explicitly:

```rust
//! # C-LinAlg contract
//!
//! The types and functions in this module implement the C-LinAlg contract frozen at G.E.1.
//! G.E.2 (block Lanczos), G.E.3 (Wiedemann), G.F (square root), and D.B (GF(ℓ) extension)
//! consume this interface directly.
```

### The D.B generalisation

D.B (NFS-DL linear algebra) generalises the G.E linear algebra from GF(2) to F_ℓ (the field
with ℓ elements, where ℓ is a prime). The generalisation touches three points:

**`BlockVec` generalisation.** The GF(2)-specific packing (64 scalars per `u64`) is the scale
knob. For F_ℓ, the natural generalisation is `data: Vec<[Scalar; BLOCK_WIDTH]>` where `Scalar`
is the field element type. The `BlockVec` docstring annotates this:

> For F_ℓ (ℓ > 2), the natural generalisation is `data: Vec<[Scalar; BLOCK_WIDTH]>` where
> `Scalar` is the field element type. The GF(2) specialisation packs 64 scalars into one
> `u64`. D.B may introduce a `BlockVec<S>` generic or a parallel `BlockVecFl` type; the
> *interface* (inner products, apply, apply_transpose) is the stable seam.

**`MatrixOperator` generalisation.** The operator interface is already abstract: `apply` and
`apply_transpose` take and return `BlockVec`. For F_ℓ, the same interface shape applies with
scalar multiplication replacing the GF(2) XOR. The `MatrixOperator` docstring annotates:

> For F_ℓ, the operator needs the same shape but with scalar multiplication. The natural
> generalisation is a trait `LinearOperator<V>` with `apply(&self, v: &V) -> V` and
> `apply_transpose(&self, v: &V) -> V`. G.E implements the concrete GF(2) version; D.B
> may introduce the trait and have `MatrixOperator` implement it.

**`KernelVector` generalisation.** For F_ℓ, a kernel vector is still a subset of rows (those
with nonzero coefficient in the nullspace vector). The row-index spine is stable; D.B may add
a `coefficients` field (`Vec<Scalar>`) for the non-GF(2) case.

### The interface shape is stable

The key design decision is that the GF(2)-specific packing is isolated in `BlockVec::data`
(the `Vec<u64>` representation) and the inner-loop operations (`xor_assign`,
`inner_product_matrix`). The interface shape — `apply`, `apply_transpose`, `KernelVector`,
`expand_provenance` — is stable across the GF(2) → F_ℓ generalisation. D.B can reuse the
`MatrixOperator` and `KernelVector` types directly, or introduce a thin generic wrapper, without
changing the solver structure.

---

## 39. Principle-4 Annotations Summary (G.E)

The following table collects all principle-4 annotations from the G.E module — phenomena that
are scale-dependent and either under-exposed or over-exposed at toy scale.

| Phenomenon | Toy-scale behaviour | NFS-scale behaviour | Scale knob |
|------------|--------------------|--------------------|------------|
| **Block width** | Single 64-bit word suffices; blocking overhead invisible | 64-wide block amortises memory bandwidth; ~64× speedup over single-vector | `BLOCK_WIDTH = 64` |
| **Self-orthogonality** | Exposed by duplicate rows in KAT matrices; winnowing path exercised | Same algorithm; more frequent at scale due to larger nullspace | — (algorithm feature) |
| **Wiedemann parallelism** | Invisible; 4 sequential attempts, no distributed computation | Krylov sequences computed in parallel across machines; no per-step sync | `num_attempts` / block width |
| **QC column count** | `DEFAULT_NUM_QC = 10` suffices | 20–50 QC columns typical | `DEFAULT_NUM_QC` |
| **Excess floor** | `EXCESS_FLOOR = 20` never approached in practice | Floor is a tuning parameter: too low → few null-space vectors; too high → denser matrix | `EXCESS_FLOOR` |

The self-orthogonality phenomenon is the one that IS fully exposed at toy scale: the KAT
matrices are deliberately constructed to trigger the winnowing path, and the algorithm handles
it correctly. The block-width and Wiedemann-parallelism phenomena are the ones most
under-exposed: at toy scale, both solvers are instantaneous and the performance difference
between them is invisible.

---

## 40. KAT Summary (G.E)

The following table lists the key known-answer tests across the linalg module, with the
mathematical fact each one verifies.

| Test file | Test | Mathematical fact verified |
|-----------|------|---------------------------|
| `linalg_substrate_kat.rs` | `kat_1_operator_correctness` | `A·V` and `Aᵀ·V` match hand-computed GF(2) products for a 3×4 matrix |
| `linalg_substrate_kat.rs` | `kat_2_qc_column_construction` | QC column parities match hand-computed Legendre symbols for toy relations |
| `linalg_substrate_kat.rs` | `kat_3_round_trip_provenance` | `expand_provenance` returns the correct symmetric difference of provenance sets |
| `linalg_substrate_kat.rs` | `kat_4_determinism` | Operator products and QC columns are deterministic for a fixed matrix |
| `lanczos_kat.rs` | `kat_a_correctness_with_self_orthogonality` | Block Lanczos finds valid kernel vectors for a 6×4 matrix with duplicate rows (self-orthogonality path exercised) |
| `lanczos_kat.rs` | `kat_a2_single_dependency` | Block Lanczos finds the single dependency `{0,3}` for a 4×3 matrix |
| `lanczos_kat.rs` | `kat_a3_full_rank_no_nullspace` | Block Lanczos returns only valid vectors for a full-rank 3×3 identity matrix |
| `lanczos_kat.rs` | `kat_b_determinism` | Block Lanczos is deterministic for a fixed matrix and seed |
| `lanczos_kat.rs` | `kat_c_cado_oracle_n35` | (Ignored) CADO oracle for N=35 — gated when CADO absent |
| `lanczos_kat.rs` | `kat_multiple_dependencies` | Block Lanczos finds at least one of three known dependencies in an 8×6 matrix |
| `wiedemann_kat.rs` | `kat_a_cross_validation_with_lanczos` | Wiedemann and Lanczos cross-validate: both find valid kernel vectors for the shared 6×4 matrix |
| `wiedemann_kat.rs` | `kat_a4_single_dependency` | Wiedemann finds the single dependency `{0,3}` for a 4×3 matrix |
| `wiedemann_kat.rs` | `kat_b_determinism` | Wiedemann is deterministic for a fixed matrix and seed |
| `wiedemann_kat.rs` | `kat_c_bm_fibonacci_degree` | Berlekamp-Massey returns degree-2 polynomial `1 + z + z²` for the Fibonacci mod 2 sequence |
| `wiedemann_kat.rs` | `kat_c2_bm_period4_sequence` | Berlekamp-Massey returns degree-4 polynomial `1 + z⁴` for a period-4 sequence |

---

## 41. What Comes Next

Linear algebra is the fifth stage of the GNFS pipeline. The output — a `Vec<KernelVector>`,
each with its provenance expansion thread back to the original relations — is the input to the
square-root stage.

**G.F (square root)** takes a `KernelVector` from G.E and expands it through the provenance
map to recover the original (a, b) pairs. It then computes the product of all rational norms
(a perfect square in ℤ) and the product of all algebraic norms (a perfect square in K = ℚ(α)),
and extracts the square roots to form the congruence x² ≡ y² (mod N). A non-trivial GCD of
x − y and N yields a factor.

**D.B (NFS-DL linear algebra)** generalises the G.E linear algebra from GF(2) to F_ℓ. The
`BlockVec`, `MatrixOperator`, and `KernelVector` types are the frozen seam; D.B may introduce
a generic wrapper or a parallel `BlockVecFl` type for the F_ℓ case. The solver structure
(block Lanczos or block Wiedemann) is unchanged; only the field arithmetic differs.

---

## Further Reading (G.E)

1. **Montgomery, P. L. (1995).** *A block Lanczos algorithm for finding dependencies over
   GF(2).* In: Guillou, L. C., and Quisquater, J.-J. (eds.) Advances in Cryptology —
   EUROCRYPT '95, LNCS 921. Springer. The original source for the block Lanczos algorithm
   used in G.E.2, including the self-orthogonality winnowing and the three-term recurrence.

2. **Coppersmith, D. (1994).** *Solving homogeneous linear equations over GF(2) via block
   Wiedemann algorithm.* Mathematics of Computation, 62(205), 333–350. The original source
   for the block Wiedemann algorithm used in G.E.3, including the Berlekamp-Massey step and
   the parallelism analysis.

3. **Wiedemann, D. (1986).** *Solving sparse linear equations over finite fields.* IEEE
   Transactions on Information Theory, 32(1), 54–62. The scalar Wiedemann algorithm that
   block Wiedemann generalises.

4. **Kleinjung, T., Aoki, K., Franke, J., Lenstra, A. K., Thomé, E., Bos, J. W., Gaudry, P.,
   Kruppa, A., Montgomery, P. L., Osvik, D. A., te Riele, H., Timofeev, A., and Zimmermann,
   P. (2010).** *Factorization of a 768-bit RSA modulus.* In: Rabin, T. (ed.) Advances in
   Cryptology — CRYPTO 2010, LNCS 6223. Springer. The RSA-768 factorisation, which used
   block Wiedemann for the linear algebra step across a distributed cluster.

5. **Berlekamp, E. R. (1968).** *Algebraic Coding Theory.* McGraw-Hill. The original source
   for the Berlekamp-Massey algorithm used in the Wiedemann step.

6. **Massey, J. L. (1969).** *Shift-register synthesis and BCH decoding.* IEEE Transactions
   on Information Theory, 15(1), 122–127. The Massey formulation of the LFSR synthesis
   problem, which is the GF(2) Berlekamp-Massey algorithm used in `wiedemann.rs`.
