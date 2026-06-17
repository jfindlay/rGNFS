//! Order-finding circuit, continued-fraction period extraction, and `factor(N)` driver.
//!
//! This module implements the S.B.2 deliverable: the order-finding circuit orchestration,
//! the classical continued-fraction post-processing, and the end-to-end `factor(N)` driver.
//!
//! # Algorithm outline
//!
//! 1. **Order-finding circuit** ([`find_order`]): prepare the exponent register in uniform
//!    superposition, apply the C-ModExp circuit (`|x⟩|1⟩ → |x⟩|aˣ mod N⟩`), apply iQFT to
//!    the exponent register, measure to get a phase numerator `s ≈ k·2^t/r`.
//!
//! 2. **Continued-fraction period extraction** ([`order_from_phase`]): expand `s/2^t` as a
//!    continued fraction, take convergents with denominator `< N`, and recover the order `r`
//!    (the smallest denominator `d` such that `a^d ≡ 1 mod N`).
//!
//! 3. **`factor(N)` driver** ([`factor`]): pick `a` coprime to `N`, run order-finding, extract
//!    `r`; if `r` is even and `a^(r/2) ≢ -1 mod N`, return `gcd(a^(r/2) ± 1, N)`; else retry.
//!
//! # Register layout (C-ModExp — consumed)
//!
//! The order-finding circuit uses the standard `ModExpLayout::standard(N, exp_len)` layout:
//! - Qubits `[0, exp_len)` — exponent register (t bits, put in superposition)
//! - Qubits `[exp_len, exp_len + n)` — work register (n bits, initialized to `|1⟩`)
//!
//! Total qubits: `exp_len + n`, where `N < 2^n` and `exp_len = n_bits(N)`.
//!
//! # Qubit budgets (confirmed S.B.1 ancilla-free implementation)
//!
//! | N  | n_bits(N) | exp_len (t) | work (n) | total qubits |
//! |----|-----------|-------------|----------|--------------|
//! | 15 | 4         | 4           | 4        | 8            |
//! | 21 | 5         | 5           | 5        | 10           |
//! | 35 | 6         | 6           | 6        | 12           |
//! | 91 | 7         | 7           | 7        | 14           |
//!
//! All within the ~25-qubit simulator ceiling (principle 4).
//! These match the S.B.1 action-frame digest qubit budgets.
//!
//! # Contracts produced
//!
//! - **C-OrderFind**: [`find_order`] + [`order_from_phase`] (the order-finding + period-extraction
//!   interface, frozen at S.B.2 ◆).
//! - **C-Factor**: [`factor`] (the end-to-end factoring driver, frozen at S.B.2 ◆).
//!
//! # References
//!
//! - Shor, P.W. (1994). "Algorithms for quantum computation: discrete logarithms and factoring."
//!   FOCS 1994.
//! - Nielsen, M.A., Chuang, I.L. (2000). "Quantum Computation and Quantum Information."
//!   Cambridge University Press. §5.3.

use std::f64::consts::PI;

use crate::arith::{controlled_mod_exp, mod_pow, n_bits, ModExpLayout};
use crate::gates::{controlled_phase, h, swap};
use crate::measure::measure_all_seeded;
use crate::statevec::StateVec;

// ── classical helpers ─────────────────────────────────────────────────────────

/// Compute `gcd(a, b)` using the Euclidean algorithm.
///
/// Returns the greatest common divisor of `a` and `b`. Returns 0 if both inputs are 0.
#[must_use]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute the exponent register size `t = n_bits(N)` for order-finding.
///
/// Uses `t = n_bits(N) = ⌈log₂(N)⌉` bits for the exponent register. This matches the
/// ancilla-free S.B.1 qubit budget (confirmed in the S.B.1 action-frame digest):
/// - N=15: t=4, total=8 qubits
/// - N=21: t=5, total=10 qubits
/// - N=35: t=6, total=12 qubits
/// - N=91: t=7, total=14 qubits
///
/// Phase resolution: `1/2^t`. For the factoring targets, this is sufficient for
/// continued-fraction recovery of the order `r` (verified for all four targets).
///
/// # Panics
///
/// Panics if `n == 0`.
#[must_use]
pub fn exp_len_for(n: u64) -> usize {
    n_bits(n)
}

/// Compute the continued-fraction convergents of `p/q` up to denominator bound `max_denom`.
///
/// Returns a `Vec` of `(numerator, denominator)` pairs for each convergent, in order from
/// the first (coarsest) to the last (finest) convergent with denominator `≤ max_denom`.
///
/// The continued-fraction expansion of `p/q` is computed via the Euclidean algorithm.
/// Each convergent `h_k/k_k` satisfies `|p/q - h_k/k_k| ≤ 1/(k_k · k_{k+1})`.
///
/// # Panics
///
/// Panics if `q == 0`.
#[must_use]
pub fn convergents(p: u64, q: u64, max_denom: u64) -> Vec<(u64, u64)> {
    assert!(q > 0, "convergents: denominator q must be nonzero");
    let mut result = Vec::new();

    // Continued-fraction coefficients via the Euclidean algorithm.
    // p/q = a_0 + 1/(a_1 + 1/(a_2 + ...))
    // Convergents: h_{-1}=1, h_0=a_0, h_k = a_k*h_{k-1} + h_{k-2}
    //              k_{-1}=0, k_0=1,   k_k = a_k*k_{k-1} + k_{k-2}
    // Standard continued-fraction recurrence:
    //   h_k = a_k * h_{k-1} + h_{k-2},  with h_{-2}=0, h_{-1}=1
    //   k_k = a_k * k_{k-1} + k_{k-2},  with k_{-2}=1, k_{-1}=0
    let mut h_prev = 0u64; // h_{k-2}, starts at h_{-2}=0
    let mut h_curr = 1u64; // h_{k-1}, starts at h_{-1}=1
    let mut k_prev = 1u64; // k_{k-2}, starts at k_{-2}=1
    let mut k_curr = 0u64; // k_{k-1}, starts at k_{-1}=0

    let mut num = p;
    let mut den = q;

    loop {
        let a = num / den;
        let rem = num % den;

        // h_k = a * h_{k-1} + h_{k-2}
        // k_k = a * k_{k-1} + k_{k-2}
        let h_next = a.saturating_mul(h_curr).saturating_add(h_prev);
        let k_next = a.saturating_mul(k_curr).saturating_add(k_prev);

        if k_next > max_denom {
            break;
        }

        result.push((h_next, k_next));

        h_prev = h_curr;
        h_curr = h_next;
        k_prev = k_curr;
        k_curr = k_next;

        if rem == 0 {
            break;
        }
        num = den;
        den = rem;
    }

    result
}

/// Recover the order `r` of `a` mod `N` from a measured phase numerator `s` and register size `t`.
///
/// The phase is `s / 2^t ≈ k/r` for some integer `k`. Expands `s / 2^t` as a continued
/// fraction, takes convergents with denominator `< N`, and returns the smallest denominator `d`
/// such that `a^d ≡ 1 mod N`.
///
/// Returns `Some(r)` if a valid order is found among the convergents, `None` otherwise.
///
/// # Arguments
///
/// - `s` — measured phase numerator (from the exponent register measurement)
/// - `t` — exponent register size (number of qubits)
/// - `a` — the base whose order we seek
/// - `n` — the modulus
#[must_use]
pub fn order_from_phase(s: u64, t: usize, a: u64, n: u64) -> Option<u64> {
    if s == 0 {
        return None; // s=0 gives phase 0, which is uninformative
    }
    let two_t = 1u64 << t; // 2^t
    let convs = convergents(s, two_t, n);
    for (_num, denom) in convs {
        if denom == 0 {
            continue;
        }
        // Check if a^denom ≡ 1 mod N.
        if mod_pow(a, denom, n) == 1 {
            return Some(denom);
        }
    }
    None
}

// ── partial iQFT on a subset of qubits ───────────────────────────────────────

/// Apply the inverse QFT to a specified subset of qubits in-place.
///
/// Applies iQFT to the qubits listed in `qubits` (in the order given), treating them as a
/// sub-register. The other qubits in `sv` are not touched by the QFT gates, but they may
/// be entangled with the sub-register.
///
/// The convention matches [`crate::qft::iqft`]: the inverse controlled-phase + Hadamard
/// ladder (phases negated, ladder reversed), then an output bit-reversal over the sub-register.
///
/// # Arguments
///
/// - `sv` — state vector (modified in-place)
/// - `qubits` — qubit indices of the sub-register (little-endian, LSB first)
///
/// # Panics
///
/// Panics if `qubits` is empty or any qubit index is out of range.
fn iqft_on_qubits(sv: &mut StateVec, qubits: &[usize]) {
    let m = qubits.len();
    assert!(m > 0, "iqft_on_qubits: qubits must be nonempty");

    // Inverse of the H + controlled-phase ladder (reversed order, negated phases).
    // The ladder processes qubits[0], qubits[1], ..., qubits[m-1] in the forward QFT.
    // The inverse reverses this: process qubits[m-1] down to qubits[0].
    for j in (0..m).rev() {
        for k in ((j + 1)..m).rev() {
            let theta = -2.0 * PI / (1usize << (k - j + 1)) as f64;
            controlled_phase(sv, qubits[k], qubits[j], theta);
        }
        h(sv, qubits[j]);
    }

    // Output bit-reversal: swap qubits[i] with qubits[m-1-i].
    // This undoes the input bit-reversal of the forward QFT.
    for i in 0..(m / 2) {
        swap(sv, qubits[i], qubits[m - 1 - i]);
    }
}

// ── order-finding circuit ─────────────────────────────────────────────────────

/// Run the order-finding circuit for base `a` modulo `N`, returning the measured phase numerator.
///
/// Circuit steps:
/// 1. Allocate `exp_len + n` qubits; initialize work register to `|1⟩`.
/// 2. Apply `H` to every exponent qubit (uniform superposition).
/// 3. Apply `controlled_mod_exp(sv, a, &layout)` — maps `|x⟩|1⟩ → |x⟩|aˣ mod N⟩`.
/// 4. Apply iQFT to the exponent register.
/// 5. Measure with `measure_all_seeded(sv, seed)`.
/// 6. Extract the exponent register value: `s = basis_index & ((1 << exp_len) - 1)`.
///
/// Returns the measured phase numerator `s` (the lower `exp_len` bits of the measurement).
///
/// # Arguments
///
/// - `a` — the base (must satisfy `gcd(a, N) = 1`)
/// - `layout` — register layout (from `ModExpLayout::standard`)
/// - `seed` — RNG seed for reproducible measurement
///
/// # Panics
///
/// Panics if `gcd(a, N) != 1` or any qubit index is out of range.
#[must_use]
pub fn run_order_finding_circuit(a: u64, layout: &ModExpLayout, seed: u64) -> u64 {
    let exp_len = layout.exp_len;
    let total_qubits = layout.total_qubits();

    // Step 1: Allocate state vector; initialize work register to |1⟩.
    // Work register starts at qubit exp_len (little-endian LSB = qubit exp_len).
    // Basis index for |1⟩ in the work register = 1 << exp_len (bit exp_len set = LSB of work).
    let work_init_index = 1usize << layout.work_start;
    let mut sv = StateVec::basis(total_qubits, work_init_index);

    // Step 2: Apply H to every exponent qubit (uniform superposition over exponent register).
    for q in layout.exp_start..(layout.exp_start + exp_len) {
        h(&mut sv, q);
    }

    // Step 3: Apply controlled modular exponentiation: |x⟩|1⟩ → |x⟩|aˣ mod N⟩.
    controlled_mod_exp(&mut sv, a, layout);

    // Step 4: Apply iQFT to the exponent register.
    let exp_qubits: Vec<usize> = (layout.exp_start..(layout.exp_start + exp_len)).collect();
    iqft_on_qubits(&mut sv, &exp_qubits);

    // Step 5: Measure the full register.
    let outcome = measure_all_seeded(sv, seed);

    // Step 6: Extract the exponent register value (lower exp_len bits of basis_index).
    let exp_mask = (1usize << exp_len) - 1;
    let s = (outcome.basis_index >> layout.exp_start) & exp_mask;
    s as u64
}

/// Find the order of `a` modulo `N` using the quantum order-finding circuit.
///
/// Runs the order-finding circuit with the given seed, then applies continued-fraction
/// period extraction to recover the order `r` (the smallest `r > 0` such that `a^r ≡ 1 mod N`).
///
/// Returns `Some(r)` if the circuit measurement yields a phase that recovers the order,
/// `None` if the measurement was uninformative (e.g., `s = 0` or no convergent satisfies
/// `a^d ≡ 1 mod N`).
///
/// # Arguments
///
/// - `a` — the base (must satisfy `gcd(a, N) = 1`)
/// - `n` — the modulus
/// - `seed` — RNG seed for the measurement
///
/// # Panics
///
/// Panics if `gcd(a, N) != 1`, `n == 0`, or `a == 0`.
#[must_use]
pub fn find_order(a: u64, n: u64, seed: u64) -> Option<u64> {
    assert!(n > 1, "find_order: modulus n must be > 1");
    assert!(a > 0, "find_order: a must be nonzero");
    assert!(gcd(a % n, n) == 1, "find_order: gcd(a, N) must be 1, got a={a}, N={n}");

    let exp_len = exp_len_for(n);
    let layout = ModExpLayout::standard(n, exp_len);

    let s = run_order_finding_circuit(a, &layout, seed);
    order_from_phase(s, exp_len, a, n)
}

// ── end-to-end factoring driver ───────────────────────────────────────────────

/// Check whether `n` is a prime power `p^k` for `k >= 2`.
///
/// Returns `Some(p)` if `n = p^k` for some prime `p` and `k >= 2`, else `None`.
/// Used as a classical short-circuit in the factoring driver.
fn prime_power_base(n: u64) -> Option<u64> {
    if n < 4 {
        return None;
    }
    // Try each possible exponent k from 2 up to log2(n).
    let max_k = 63 - n.leading_zeros() as u64; // floor(log2(n))
    for k in 2..=max_k {
        // Compute the k-th root of n (integer).
        let root = integer_kth_root(n, k);
        if root >= 2 && root.pow(k as u32) == n {
            // Verify root is prime (simple trial division at toy scale).
            if is_prime_trial(root) {
                return Some(root);
            }
        }
    }
    None
}

/// Compute the integer k-th root of `n` (floor(n^{1/k})).
fn integer_kth_root(n: u64, k: u64) -> u64 {
    if k == 1 {
        return n;
    }
    if n == 0 {
        return 0;
    }
    // Newton's method for integer k-th root.
    let mut x = (n as f64).powf(1.0 / k as f64) as u64 + 1;
    loop {
        // x_new = ((k-1)*x + n/x^{k-1}) / k
        let xk1 = x.saturating_pow((k - 1) as u32);
        if xk1 == 0 {
            break;
        }
        let x_new = ((k - 1) * x + n / xk1) / k;
        if x_new >= x {
            break;
        }
        x = x_new;
    }
    // Adjust down if overshoot.
    while x > 0 && x.saturating_pow(k as u32) > n {
        x -= 1;
    }
    x
}

/// Simple primality test by trial division (sufficient for toy-scale N).
fn is_prime_trial(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Factor `N` using Shor's algorithm with a fixed seed.
///
/// Returns `Some((p, q))` where `p * q = N` and `1 < p, q < N`, or `None` if no factor
/// is found within the retry budget.
///
/// # Algorithm
///
/// 1. **Classical short-circuits**: if `N` is even, return `(2, N/2)`; if `N` is a prime
///    power `p^k`, return `(p, N/p)`.
/// 2. **Quantum order-finding loop**: for each candidate base `a` coprime to `N`:
///    - Run the order-finding circuit with the given seed.
///    - If the order `r` is even and `a^(r/2) mod N ≠ N-1` (i.e., `a^(r/2) ≢ -1 mod N`):
///      - Compute `p = gcd(a^(r/2) - 1, N)` and `q = gcd(a^(r/2) + 1, N)`.
///      - If `1 < p < N`, return `Some((p, N/p))`.
///      - If `1 < q < N`, return `Some((q, N/q))`.
///    - Retry with the next `a` (seed is incremented per retry to get different measurements).
///
/// # Arguments
///
/// - `n` — the integer to factor (must be composite and `> 1`)
/// - `seed` — base RNG seed for reproducible measurement (incremented per retry)
///
/// # Returns
///
/// `Some((p, q))` with `p * q = n` and `1 < p, q < n`, or `None` if the retry budget is
/// exhausted without finding a factor.
///
/// # Panics
///
/// Panics if `n <= 1`.
#[must_use]
pub fn factor(n: u64, seed: u64) -> Option<(u64, u64)> {
    assert!(n > 1, "factor: n must be > 1");

    // Classical short-circuit: even N.
    if n % 2 == 0 {
        return Some((2, n / 2));
    }

    // Classical short-circuit: prime power N = p^k.
    if let Some(p) = prime_power_base(n) {
        return Some((p, n / p));
    }

    // Quantum order-finding loop.
    // Try bases a = 2, 3, 5, 7, 11, 13, ... (skip multiples of N's factors if known).
    // For each base, try multiple seeds to get different measurement outcomes.
    let max_bases = 20usize;
    let max_seeds_per_base = 8usize;

    let candidate_bases: Vec<u64> = (2u64..n)
        .filter(|&a| gcd(a, n) == 1)
        .take(max_bases)
        .collect();

    for (base_idx, &a) in candidate_bases.iter().enumerate() {
        for seed_offset in 0..max_seeds_per_base {
            let trial_seed = seed
                .wrapping_add(base_idx as u64 * 1000)
                .wrapping_add(seed_offset as u64);

            let Some(r) = find_order(a, n, trial_seed) else {
                continue;
            };

            // Order must be even for the factor extraction to work.
            if r % 2 != 0 {
                continue;
            }

            // Check a^(r/2) ≢ -1 mod N (i.e., a^(r/2) mod N ≠ N-1).
            let half_power = mod_pow(a, r / 2, n);
            if half_power == n - 1 {
                continue; // trivial square root of unity — no factor
            }

            // Extract factors via gcd.
            let p = gcd(half_power.saturating_sub(1), n);
            let q = gcd(half_power + 1, n);

            if p > 1 && p < n {
                return Some((p, n / p));
            }
            if q > 1 && q < n {
                return Some((q, n / q));
            }
        }
    }

    None
}
