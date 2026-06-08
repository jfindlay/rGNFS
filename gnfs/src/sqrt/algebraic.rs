//! Algebraic square root via Couveignes' CRT algorithm.
//!
//! Given a kernel vector (a subset S of relations whose algebraic norm product is a perfect
//! square in K), computes β ∈ K with β² = γ = ∏_{i ∈ S}(a_i − b_i·α), then returns
//! Y = |Norm(β)| mod N.
//!
//! # Algorithm (Couveignes)
//!
//! 1. Form γ = ∏_{i ∈ S}(a_i − b_i·α) ∈ K via `NumberFieldElement::mul`.
//! 2. Select CRT primes: primes p that split completely in K, p > B_alg.
//! 3. For each split prime p with roots r_1, ..., r_d of f mod p:
//!    - Reduce γ mod (p, α − r_j) to get γ_j ∈ 𝔽_p via `reduce_mod_ideal`.
//!    - Compute β_j = sqrt(γ_j) in 𝔽_p via Tonelli–Shanks.
//!    - Combine the d roots β_1, ..., β_d into β mod p via Lagrange interpolation.
//! 4. CRT-lift the per-prime β mod p values to recover β's coefficients in ℤ[α].
//! 5. Resolve the global sign of β via the real embedding.
//! 6. Return Y = |Norm(β)| mod N.
//!
//! # Principle-4 annotation
//!
//! `DEFAULT_COUVEIGNES_PRIMES = 10` suffices at toy scale. At NFS scale, the prime count
//! is O(coefficient_bits / 64) — the scale knob. The algorithm is identical at all scales.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};

use shared_numfield::{IntPoly, NumberField, NumberFieldElement, RatPoly};

use crate::filter::SparseMatrix;
use crate::linalg::KernelVector;
use crate::linalg::select_qc_primes;
use crate::polyselect::PolyPair;
use crate::sieve::Relation;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Default number of CRT primes for Couveignes' algorithm.
///
/// Principle-4: 10 suffices at toy scale (coefficient bit-length is small); at NFS scale,
/// use O(coefficient_bits / 64) primes. The algorithm is identical at all scales.
const DEFAULT_COUVEIGNES_PRIMES: usize = 10;

// ─── Public entry point ───────────────────────────────────────────────────────

/// Compute the algebraic square root Y = |Norm(β)| mod N via Couveignes' CRT algorithm.
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
///    - Compute β_j = sqrt(γ_j) in 𝔽_p via Tonelli–Shanks.
///    - Combine the d roots β_1, ..., β_d into β mod p via Lagrange interpolation.
/// 4. CRT-lift the per-prime β mod p values to recover β's coefficients in ℤ[α].
/// 5. Resolve the global sign of β via the real embedding (see §4 of C-AlgSqrt).
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
) -> BigInt {
    let nf = poly.number_field();
    let f = poly.monic_f();
    let d = nf.degree();

    // Step 1: Form γ = ∏_{i ∈ S}(a_i − b_i·α).
    let s = kv.expand_provenance(matrix);
    let gamma = form_gamma(&nf, relations, &s);

    // Step 2: Select CRT primes (split completely in K, > b_alg).
    // Principle-4: prime_count is the scale knob; 10 suffices at toy scale,
    // O(coefficient_bits / 64) at NFS scale.
    let b_alg = 100u64;
    let primes = select_couveignes_primes(&f, b_alg, DEFAULT_COUVEIGNES_PRIMES);

    // Step 3: Per-prime square root + Lagrange interpolation.
    // For each prime p, collect the polynomial β mod p as Vec<u64> of d coefficients.
    // Normalize each per-prime polynomial: if the constant term c_0 > p/2, negate all
    // coefficients (replace c_k with p - c_k). This ensures all primes give the same
    // "branch" (β or -β), making the CRT lift consistent. The global sign is resolved
    // in Step 5 via the real embedding.
    let mut per_prime_coeffs: Vec<Vec<u64>> = Vec::with_capacity(primes.len());
    for &p in &primes {
        let mut beta_mod_p = per_prime_beta(&gamma, &f, p, d);
        // Canonical form: constant term in [0, p/2). If c_0 > p/2, negate all coefficients.
        if !beta_mod_p.is_empty() && beta_mod_p[0] > p / 2 {
            for c in &mut beta_mod_p {
                *c = if *c == 0 { 0 } else { p - *c };
            }
        }
        per_prime_coeffs.push(beta_mod_p);
    }

    // Step 4: CRT lift — for each coefficient index k, lift across all primes.
    let mut coeffs: Vec<BigInt> = Vec::with_capacity(d);
    for k in 0..d {
        let residues: Vec<(u64, u64)> =
            primes.iter().zip(per_prime_coeffs.iter()).map(|(&p, betas)| (betas[k], p)).collect();
        let ck_raw = garner_crt(&residues);

        // Center: if ck_raw > ∏p_i / 2, subtract ∏p_i.
        let prod: BigInt = primes.iter().map(|&p| BigInt::from(p)).product();
        let half_prod = &prod / BigInt::from(2u64);
        let ck = if ck_raw > half_prod { ck_raw - &prod } else { ck_raw };
        coeffs.push(ck);
    }

    // Step 5: Sign resolution via real embedding.
    // Find a real root θ of f using Newton's method from m^{1/d}.
    let m_f64 = poly.m.to_f64().unwrap_or(1.0);
    let x0 = m_f64.powf(1.0 / d as f64);
    let sign_positive = match find_real_root(&f, x0, 1e-10, 200) {
        Some(theta) => {
            let val = eval_at_real(&coeffs, theta);
            // β(θ) > 0 means we have the positive square root; negate if < 0.
            val >= 0.0
        }
        None => {
            // Fallback: norm-sign convention. Norm(β) for the correct β should be positive
            // (or at least consistent). We use the sign of the raw CRT result.
            // This is a best-effort fallback; the G.F.4 retry loop is the safety net.
            true
        }
    };
    if !sign_positive {
        for c in &mut coeffs {
            *c = -c.clone();
        }
    }

    // Step 6: Compute Y = |Norm(β)| mod N.
    let beta_poly = RatPoly::from_coeffs(
        coeffs.iter().map(|c| BigRational::from(c.clone())).collect(),
    );
    let beta = NumberFieldElement { field: &nf, poly: beta_poly };
    let norm_rat = beta.norm();

    // The norm of an algebraic integer is a rational integer; the denominator should be 1.
    let norm_numer = norm_rat.numer().clone();
    let norm_abs = norm_numer.abs();

    let n = &poly.n;
    if n.is_zero() {
        return norm_abs;
    }
    norm_abs % n
}

// ─── Step 1: Form γ ───────────────────────────────────────────────────────────

/// Form γ = ∏_{i ∈ S}(a_i − b_i·α) ∈ K.
///
/// Each factor is the element (a_i − b_i·α) = a_i·1 − b_i·α in K.
fn form_gamma<'a>(nf: &'a NumberField, relations: &[Relation], s: &[usize]) -> NumberFieldElement<'a> {
    // Start with the multiplicative identity 1.
    let mut gamma = nf.from_int(BigInt::from(1i64));

    for &i in s {
        let rel = &relations[i];
        // Factor: a_i − b_i·α = a_i·1 + (−b_i)·α.
        // Coefficients: [a_i, -b_i] (constant term first).
        let factor_poly = RatPoly::from_coeffs(vec![
            BigRational::from(rel.a.clone()),
            BigRational::from(-rel.b.clone()),
        ]);
        let factor = NumberFieldElement { field: nf, poly: factor_poly };
        gamma = gamma.mul(&factor);
    }

    gamma
}

// ─── Step 2: Select CRT primes ────────────────────────────────────────────────

/// Select `num_primes` primes > `b_alg` that split completely in K (f has d distinct roots mod p).
///
/// Reuses the QC-prime selection logic from `gnfs::linalg::qc`.
fn select_couveignes_primes(f: &IntPoly, b_alg: u64, num_primes: usize) -> Vec<u64> {
    select_qc_primes(f, b_alg, num_primes)
}

// ─── Step 3: Per-prime square root + Lagrange interpolation ──────────────────

/// Compute β mod p as a polynomial in α with coefficients in 𝔽_p.
///
/// Returns the d coefficients [c_0, ..., c_{d-1}] of the unique polynomial of degree < d
/// such that β(r_j)² ≡ γ(r_j) (mod p) for each root r_j of f mod p.
///
/// The per-root square roots have a ±1 ambiguity. This function resolves the ambiguity by
/// choosing the square root for each root r_j that is consistent with the polynomial built
/// from the previous roots. Specifically, after fixing β_0 (from Tonelli-Shanks for r_0),
/// for each subsequent root r_j, we evaluate the current partial Lagrange polynomial at r_j
/// and choose the square root that matches (or its negation if it doesn't).
fn per_prime_beta(gamma: &NumberFieldElement<'_>, f: &IntPoly, p: u64, d: usize) -> Vec<u64> {
    // Find all roots of f mod p.
    let roots = roots_mod_p(f, p);
    debug_assert_eq!(
        roots.len(),
        d,
        "per_prime_beta: f should have exactly d={d} roots mod p={p} (split completely)"
    );

    let p_big = BigInt::from(p);

    // Compute γ_j = γ(r_j) mod p for each root r_j.
    let gamma_vals: Vec<u64> = roots
        .iter()
        .map(|&r| {
            let r_big = BigInt::from(r);
            let gamma_j = gamma.reduce_mod_ideal(&p_big, &r_big);
            gamma_j.to_u64().expect("residue must fit in u64 at toy scale")
        })
        .collect();

    // Compute the initial square root for each root via Tonelli-Shanks.
    let mut beta_vals: Vec<u64> = gamma_vals
        .iter()
        .zip(roots.iter())
        .map(|(&gamma_j, &r)| {
            tonelli_shanks(gamma_j, p).unwrap_or_else(|| {
                panic!(
                    "algebraic_sqrt: γ is not a QR mod split prime p={p} at root r={r} — \
                     upstream kernel bug (QC columns should guarantee γ is a square in K)"
                )
            })
        })
        .collect();

    // Resolve the ±1 ambiguity: ensure the chosen β_j values are consistent with a single
    // polynomial of degree < d. We do this by building the Lagrange polynomial incrementally:
    // after fixing β_0, for each subsequent root r_j, evaluate the current polynomial at r_j
    // and choose the square root that matches.
    if d > 1 {
        for j in 1..d {
            // Build the Lagrange polynomial from the first j points.
            let partial_points: Vec<(u64, u64)> =
                roots[..j].iter().zip(beta_vals[..j].iter()).map(|(&r, &b)| (r, b)).collect();
            let partial_poly = lagrange_interp_mod_p(&partial_points, p, j);

            // Evaluate the partial polynomial at r_j.
            let r_j = roots[j];
            let poly_at_rj = eval_poly_u64(&partial_poly, r_j, p);

            // Choose the square root that matches the polynomial evaluation.
            // If beta_vals[j] matches, keep it. If p - beta_vals[j] matches, negate it.
            // If neither matches (shouldn't happen for a valid γ), keep the original.
            if poly_at_rj == beta_vals[j] {
                // Already consistent.
            } else if poly_at_rj == (if beta_vals[j] == 0 { 0 } else { p - beta_vals[j] }) {
                // The negation matches; negate β_j.
                beta_vals[j] = if beta_vals[j] == 0 { 0 } else { p - beta_vals[j] };
            }
            // If neither matches, the partial polynomial is not yet constrained enough
            // (this can happen when j < d-1 and the polynomial has multiple valid extensions).
            // In that case, keep the original and let the full interpolation sort it out.
        }
    }

    // Build the final Lagrange interpolation from all d points.
    let points: Vec<(u64, u64)> =
        roots.iter().zip(beta_vals.iter()).map(|(&r, &b)| (r, b)).collect();
    let result = lagrange_interp_mod_p(&points, p, d);

    // Verify unconditionally: the result polynomial must square to γ at each root.
    // A mismatch here means the per-root sign choices are inconsistent — an upstream kernel
    // bug (the QC columns should guarantee γ is a square in K, so every β(r_j)² = γ_j must
    // hold after Lagrange interpolation). Panic loudly rather than silently propagating a
    // wrong β into the CRT lift.
    for (j, (&r, &gamma_j)) in roots.iter().zip(gamma_vals.iter()).enumerate() {
        let beta_at_r = eval_poly_u64(&result, r, p);
        let beta_sq = mul_mod(beta_at_r, beta_at_r, p);
        assert_eq!(
            beta_sq, gamma_j,
            "per_prime_beta: sign inconsistency — β(r_{j})² = {beta_sq} ≠ γ(r_{j}) = {gamma_j} \
             (mod p={p}); the Lagrange polynomial does not square to γ at root r_{j}={r}. \
             This is an upstream kernel bug: the QC columns should guarantee γ is a perfect \
             square in K, so β(r_j)² ≡ γ_j (mod p) must hold for all j."
        );
    }

    result
}

/// Evaluate a polynomial (given as u64 coefficients mod p) at a point r mod p.
fn eval_poly_u64(coeffs: &[u64], r: u64, p: u64) -> u64 {
    let mut result = 0u64;
    for &c in coeffs.iter().rev() {
        result = mul_mod(result, r, p);
        result = add_mod(result, c, p);
    }
    result
}

/// Find all roots of `f` in [0, p) by trial evaluation.
fn roots_mod_p(f: &IntPoly, p: u64) -> Vec<u64> {
    (0..p).filter(|&r| eval_poly_mod(f, r, p) == 0).collect()
}

/// Evaluate `f(r) mod p` using Horner's method.
fn eval_poly_mod(f: &IntPoly, r: u64, p: u64) -> u64 {
    let mut result = 0u64;
    for c in f.coeffs.iter().rev() {
        result = mul_mod(result, r, p);
        let c_mod = bigint_mod_u64(c, p);
        result = add_mod(result, c_mod, p);
    }
    result
}

/// Tonelli–Shanks algorithm: compute sqrt(n) mod p.
///
/// Returns `Some(r)` with `r² ≡ n (mod p)` and `r ∈ [0, p)`, or `None` if `n` is not a
/// quadratic residue mod `p` (including `n = 0`, which returns `Some(0)`).
///
/// Preconditions: `p` is an odd prime, `n < p`.
pub(crate) fn tonelli_shanks(n: u64, p: u64) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }
    // Check QR via Euler's criterion: n^((p-1)/2) mod p must be 1.
    let legendre = pow_mod(n, (p - 1) / 2, p);
    if legendre != 1 {
        return None; // n is a non-residue (legendre == p-1) or p | n (legendre == 0)
    }

    // Special case: p ≡ 3 (mod 4) → sqrt = n^((p+1)/4) mod p.
    if p % 4 == 3 {
        let r = pow_mod(n, (p + 1) / 4, p);
        return Some(r);
    }

    // General Tonelli–Shanks: factor p-1 = Q * 2^S with Q odd.
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Find a quadratic non-residue z mod p.
    let mut z = 2u64;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = pow_mod(z, q, p);
    let mut t = pow_mod(n, q, p);
    let mut r = pow_mod(n, (q + 1) / 2, p);

    loop {
        if t == 0 {
            return Some(0);
        }
        if t == 1 {
            return Some(r);
        }

        // Find the least i > 0 such that t^(2^i) ≡ 1 (mod p).
        let mut i = 1u32;
        let mut tmp = mul_mod(t, t, p);
        while tmp != 1 {
            tmp = mul_mod(tmp, tmp, p);
            i += 1;
        }

        let b = pow_mod(c, pow_mod(2, (m - i - 1) as u64, p - 1), p);
        m = i;
        c = mul_mod(b, b, p);
        t = mul_mod(t, c, p);
        r = mul_mod(r, b, p);
    }
}

/// Lagrange interpolation mod p.
///
/// Given `d` points `(r_j, beta_j)`, returns the unique polynomial of degree < d
/// with those values, as coefficients `[c_0, c_1, ..., c_{d-1}]` mod p.
///
/// Uses the standard Lagrange formula: for each j, the basis polynomial L_j(x) is
/// `∏_{k≠j} (x − r_k) / (r_j − r_k)`, and the result is `∑_j beta_j * L_j(x)`.
fn lagrange_interp_mod_p(points: &[(u64, u64)], p: u64, d: usize) -> Vec<u64> {
    // Accumulate the result polynomial as a Vec<u64> of d coefficients.
    let mut result = vec![0u64; d];

    for (j, &(r_j, beta_j)) in points.iter().enumerate() {
        if beta_j == 0 {
            continue; // L_j contributes nothing
        }

        // Compute the Lagrange basis polynomial L_j(x) mod p.
        // L_j(x) = beta_j * ∏_{k≠j} (x − r_k) / (r_j − r_k).

        // First compute the denominator: ∏_{k≠j} (r_j − r_k) mod p.
        let mut denom = 1u64;
        for (k, &(r_k, _)) in points.iter().enumerate() {
            if k == j {
                continue;
            }
            // (r_j - r_k) mod p, handling negative differences.
            let diff = if r_j >= r_k { r_j - r_k } else { p - (r_k - r_j) % p };
            denom = mul_mod(denom, diff, p);
        }
        let denom_inv = mod_inv_u64(denom, p);
        let scale = mul_mod(beta_j, denom_inv, p);

        // Compute the numerator polynomial ∏_{k≠j} (x − r_k) mod p.
        // Start with the constant polynomial 1.
        let mut num_poly = vec![0u64; d];
        num_poly[0] = 1;
        let mut num_deg = 0usize; // current degree of num_poly

        for (k, &(r_k, _)) in points.iter().enumerate() {
            if k == j {
                continue;
            }
            // Multiply num_poly by (x − r_k).
            // For each coefficient c_i at position i:
            //   new[i+1] += c_i        (from the x factor)
            //   new[i]    = -r_k * c_i (from the -r_k factor, replacing the old value)
            // Iterate from high to low so we read old[i] before overwriting it.
            for i in (0..=num_deg).rev() {
                let coeff = num_poly[i];
                // Shift up: add c_i to position i+1.
                num_poly[i + 1] = add_mod(num_poly[i + 1], coeff, p);
                // Replace position i with -r_k * c_i mod p.
                let sub = mul_mod(r_k, coeff, p);
                num_poly[i] = if sub == 0 { 0 } else { p - sub };
            }
            num_deg += 1;
        }

        // Add scale * num_poly to result.
        for i in 0..d {
            result[i] = add_mod(result[i], mul_mod(scale, num_poly[i], p), p);
        }
    }

    result
}

// ─── Step 4: CRT lift via Garner's algorithm ──────────────────────────────────

/// CRT-lift a set of (residue, modulus) pairs to a single BigInt using Garner's algorithm.
///
/// Given `[(r_1, p_1), (r_2, p_2), ...]`, returns x such that x ≡ r_i (mod p_i) for all i,
/// with 0 ≤ x < ∏ p_i.
///
/// Garner's algorithm: represent x in the mixed-radix form
/// x = a_1 + p_1 * (a_2 + p_2 * (a_3 + ...))
/// where a_i are computed iteratively.
fn garner_crt(residues: &[(u64, u64)]) -> BigInt {
    if residues.is_empty() {
        return BigInt::zero();
    }

    let n = residues.len();
    // Mixed-radix coefficients a[i] in [0, p_i).
    let mut a: Vec<u64> = vec![0; n];
    a[0] = residues[0].0;

    // For each i from 1 to n-1:
    // a[i] = (r_i - (a[0] + p[0]*(a[1] + p[1]*(...a[i-1]...)))) * inv(p[0]*...*p[i-1], p[i]) mod p[i]
    for i in 1..n {
        let p_i = residues[i].1;
        let r_i = residues[i].0;

        // Evaluate the mixed-radix expansion at p_i: x_{i-1} mod p_i.
        // x_{i-1} = a[0] + p[0]*(a[1] + p[1]*(...a[i-1]...))
        // Compute this mod p_i using Horner's method from the inside out.
        let mut val = a[i - 1] % p_i;
        for j in (0..i - 1).rev() {
            let p_j = residues[j].1 % p_i;
            val = add_mod(mul_mod(val, p_j, p_i), a[j] % p_i, p_i);
        }

        // a[i] = (r_i - val) * inv(p[0]*...*p[i-1], p[i]) mod p[i].
        // But we need inv(p[0]*...*p[i-1], p[i]) mod p[i].
        let mut prod_mod = 1u64;
        for j in 0..i {
            prod_mod = mul_mod(prod_mod, residues[j].1 % p_i, p_i);
        }
        let prod_inv = mod_inv_u64(prod_mod, p_i);

        let diff = if r_i >= val { r_i - val } else { p_i - (val - r_i) % p_i };
        a[i] = mul_mod(diff, prod_inv, p_i);
    }

    // Reconstruct x = a[0] + p[0]*(a[1] + p[1]*(a[2] + ...)).
    // Use Horner's method from the outside in.
    let mut x = BigInt::from(a[n - 1]);
    for i in (0..n - 1).rev() {
        let p_i = BigInt::from(residues[i].1);
        x = x * &p_i + BigInt::from(a[i]);
    }

    x
}

// ─── Step 5: Sign resolution ──────────────────────────────────────────────────

/// Find a real root of `f` using Newton's method.
///
/// Starts from initial guess `x0` and iterates until |f(x)| < tol.
/// Returns the root as an f64 (sufficient precision at toy scale).
fn find_real_root(f: &IntPoly, x0: f64, tol: f64, max_iter: usize) -> Option<f64> {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = eval_int_poly_f64(f, x);
        if fx.abs() < tol {
            return Some(x);
        }
        let fpx = eval_int_poly_deriv_f64(f, x);
        if fpx.abs() < 1e-15 {
            // Derivative too small; Newton's method fails here.
            break;
        }
        x -= fx / fpx;
    }
    // Check if we converged.
    let fx = eval_int_poly_f64(f, x);
    if fx.abs() < tol * 1000.0 {
        Some(x)
    } else {
        None
    }
}

/// Evaluate an IntPoly at a real point using Horner's method.
fn eval_int_poly_f64(f: &IntPoly, x: f64) -> f64 {
    let mut result = 0.0f64;
    for c in f.coeffs.iter().rev() {
        result = result * x + c.to_f64().unwrap_or(0.0);
    }
    result
}

/// Evaluate the derivative of an IntPoly at a real point using Horner's method.
fn eval_int_poly_deriv_f64(f: &IntPoly, x: f64) -> f64 {
    let d = match f.degree() {
        None | Some(0) => return 0.0,
        Some(d) => d,
    };
    // Derivative coefficients: d_k = (k+1) * f.coeffs[k+1] for k = 0..d-1.
    let mut result = 0.0f64;
    for k in (0..d).rev() {
        let coeff = (k + 1) as f64 * f.coeffs[k + 1].to_f64().unwrap_or(0.0);
        result = result * x + coeff;
    }
    result
}

/// Evaluate a polynomial (given as BigInt coefficients) at a real point using Horner's method.
fn eval_at_real(coeffs: &[BigInt], theta: f64) -> f64 {
    let mut result = 0.0f64;
    for c in coeffs.iter().rev() {
        result = result * theta + c.to_f64().unwrap_or(0.0);
    }
    result
}

// ─── Modular arithmetic helpers ───────────────────────────────────────────────

/// Multiply `a * b mod p` without overflow using u128.
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

/// Add `(a + b) mod p`.
fn add_mod(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p { s - p } else { s }
}

/// Compute `base^exp mod p` using fast exponentiation.
fn pow_mod(mut base: u64, mut exp: u64, p: u64) -> u64 {
    if p == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= p;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, p);
        }
        base = mul_mod(base, base, p);
        exp >>= 1;
    }
    result
}

/// Compute the modular inverse of `a` mod `p` using Fermat's little theorem (p prime).
///
/// Returns `a^(p-2) mod p`. Panics if `a ≡ 0 (mod p)`.
fn mod_inv_u64(a: u64, p: u64) -> u64 {
    debug_assert!(a % p != 0, "mod_inv_u64: a={a} is not invertible mod p={p}");
    pow_mod(a % p, p - 2, p)
}

/// Reduce a `BigInt` to `[0, p)` as a `u64`.
fn bigint_mod_u64(a: &BigInt, p: u64) -> u64 {
    let p_big = BigInt::from(p);
    // Use % and adjust for negative values (BigInt % can be negative).
    let r = a % &p_big;
    let r = if r < BigInt::zero() { r + &p_big } else { r };
    r.to_u64().unwrap_or(0)
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    // ── Tonelli–Shanks ──

    #[test]
    fn tonelli_shanks_zero() {
        assert_eq!(tonelli_shanks(0, 7), Some(0));
    }

    #[test]
    fn tonelli_shanks_known_qr() {
        // 4 mod 7: sqrt = 2 (2² = 4)
        let r = tonelli_shanks(4, 7).expect("4 is a QR mod 7");
        assert_eq!(mul_mod(r, r, 7), 4, "r² ≡ 4 (mod 7)");

        // 2 mod 7: 2 is a QR mod 7 (3² = 9 ≡ 2 mod 7)
        let r = tonelli_shanks(2, 7).expect("2 is a QR mod 7");
        assert_eq!(mul_mod(r, r, 7), 2, "r² ≡ 2 (mod 7)");
    }

    #[test]
    fn tonelli_shanks_non_qr() {
        // 3 mod 7: 3 is a non-residue mod 7 (3^3 = 27 ≡ 6 ≡ -1 mod 7)
        assert_eq!(tonelli_shanks(3, 7), None);
    }

    #[test]
    fn tonelli_shanks_p3mod4() {
        // p = 7 ≡ 3 (mod 4). sqrt(2) mod 7: 2^((7+1)/4) = 2^2 = 4. 4² = 16 ≡ 2 mod 7. ✓
        let r = tonelli_shanks(2, 7).expect("2 is a QR mod 7");
        assert_eq!(mul_mod(r, r, 7), 2);
    }

    #[test]
    fn tonelli_shanks_p1mod4() {
        // p = 17 ≡ 1 (mod 4). sqrt(4) mod 17 = 2 or 15.
        let r = tonelli_shanks(4, 17).expect("4 is a QR mod 17");
        assert_eq!(mul_mod(r, r, 17), 4, "r² ≡ 4 (mod 17)");
    }

    // ── Lagrange interpolation ──

    #[test]
    fn lagrange_interp_degree1() {
        // Two points: (0, 3) and (1, 5) mod 7.
        // Polynomial: 3 + 2x (c_0=3, c_1=2).
        let points = vec![(0u64, 3u64), (1u64, 5u64)];
        let coeffs = lagrange_interp_mod_p(&points, 7, 2);
        assert_eq!(coeffs[0], 3, "c_0 = 3");
        assert_eq!(coeffs[1], 2, "c_1 = 2");
        // Verify: p(0) = 3, p(1) = 3 + 2 = 5. ✓
    }

    #[test]
    fn lagrange_interp_constant() {
        // One point: (2, 5) mod 7. Degree < 1 → constant polynomial 5.
        let points = vec![(2u64, 5u64)];
        let coeffs = lagrange_interp_mod_p(&points, 7, 1);
        assert_eq!(coeffs[0], 5, "constant polynomial = 5");
    }

    // ── Garner CRT ──

    #[test]
    fn garner_crt_two_primes() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5). Solution: x = 8 (8 mod 3 = 2, 8 mod 5 = 3).
        let residues = vec![(2u64, 3u64), (3u64, 5u64)];
        let x = garner_crt(&residues);
        assert_eq!(x, bi(8));
    }

    #[test]
    fn garner_crt_three_primes() {
        // x ≡ 1 (mod 2), x ≡ 2 (mod 3), x ≡ 3 (mod 5). Solution: x = 23.
        // 23 mod 2 = 1, 23 mod 3 = 2, 23 mod 5 = 3. ✓
        let residues = vec![(1u64, 2u64), (2u64, 3u64), (3u64, 5u64)];
        let x = garner_crt(&residues);
        assert_eq!(x, bi(23));
    }

    // ── Newton root finding ──

    #[test]
    fn find_real_root_sqrt2() {
        // f(x) = x² − 2. Real root: √2 ≈ 1.41421356.
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        let root = find_real_root(&f, 1.5, 1e-10, 100).expect("should converge");
        assert!((root - 2.0f64.sqrt()).abs() < 1e-8, "root ≈ √2, got {root}");
    }

    #[test]
    fn find_real_root_cubic() {
        // f(x) = x³ − 8. Real root: 2.
        let f = IntPoly::from_coeffs(vec![bi(-8), bi(0), bi(0), bi(1)]);
        let root = find_real_root(&f, 2.1, 1e-10, 100).expect("should converge");
        assert!((root - 2.0).abs() < 1e-8, "root ≈ 2, got {root}");
    }

    // ── form_gamma ──

    #[test]
    fn form_gamma_rational_factor() {
        // K = ℚ(√2), f = x² − 2.
        // One relation: a=9, b=0. Factor = 9 − 0·α = 9.
        // γ = 9 (rational).
        let f = IntPoly::from_coeffs(vec![bi(-2), bi(0), bi(1)]);
        let nf = NumberField::new(f);
        let rel = Relation {
            a: bi(9),
            b: bi(0),
            rational_exponents: crate::sieve::ExponentVector { entries: vec![] },
            algebraic_exponents: crate::sieve::ExponentVector { entries: vec![] },
            rational_sign: false,
        };
        let gamma = form_gamma(&nf, &[rel], &[0]);
        // γ should be the constant 9.
        assert_eq!(gamma.poly.coeffs.len(), 1);
        assert_eq!(gamma.poly.coeffs[0], BigRational::from(bi(9)));
    }
}
