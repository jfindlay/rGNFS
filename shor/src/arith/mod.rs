//! Reversible modular-arithmetic quantum circuit builders.
//!
//! Implements the controlled modular-exponentiation circuit
//! `|x⟩|y⟩ → |x⟩|y · aˣ mod N⟩` from the frozen S.A gate set, for use by S.B.2's
//! order-finding circuit.
//!
//! # Circuit hierarchy
//!
//! 1. [`controlled_add_mod`] — `|x⟩ → |x + c mod N⟩` controlled on a qubit.
//! 2. [`controlled_mult_mod`] — `|x⟩ → |c · x mod N⟩` controlled on a qubit, built from
//!    repeated [`controlled_add_mod`].
//! 3. [`controlled_mod_exp`] — `|x⟩|y⟩ → |x⟩|y · aˣ mod N⟩` controlled on the exponent
//!    register `x`, built from repeated [`controlled_mult_mod`] by classically-precomputed
//!    powers `a^{2^k} mod N`.
//!
//! # Register layout (C-ModExp — frozen)
//!
//! The full mod-exp circuit uses the following qubit layout (little-endian throughout,
//! matching the frozen S.A.1 convention):
//!
//! ```text
//! qubits [0, t)         — exponent register (t bits, controls the exponentiation)
//! qubits [t, t+n)       — work register (n bits, holds the value being multiplied)
//! ```
//!
//! Total qubits: `t + n`, where `N < 2^n` and `t` is the exponent register size
//! (typically `t = 2·⌈log₂ N⌉` for order-finding).
//!
//! The work register starts at `|1⟩` for the mod-exp circuit (so `y · aˣ mod N` with `y=1`
//! gives `aˣ mod N`). No ancilla qubits are required: the permutation synthesis is
//! ancilla-free (the ancilla-clean invariant is trivially satisfied).
//!
//! # Ancilla discipline
//!
//! This implementation uses a **direct permutation synthesis** approach: each
//! modular-arithmetic operation is a classical permutation on basis states, synthesized as
//! a sequence of multi-controlled-X gates. No ancilla qubits are borrowed, so the
//! ancilla-clean invariant is trivially satisfied.
//!
//! # Implementation approach
//!
//! Each modular-arithmetic operation is a classical permutation on basis states. The
//! permutation is synthesized as a product of transpositions (selection-sort order). Each
//! transposition `(a, b)` is implemented via a Gray code path: a sequence of single-bit-
//! difference transpositions, each implemented as a multi-controlled-X gate conditioned on
//! all work qubits EXCEPT the target bit (plus the external control qubit). This is correct
//! for all inputs (basis states and superpositions), reversible, and ancilla-free.
//!
//! This is a demonstration-fidelity circuit (not gate-count-optimized) per the S.B scoping
//! discipline.
//!
//! # References
//!
//! - Shor, P.W. (1994). "Algorithms for quantum computation: discrete logarithms and
//!   factoring." FOCS 1994.
//! - Vedral, V., Barenco, Ekert (1996). "Quantum networks for elementary arithmetic
//!   operations." PRA 54(1).
//! - Nielsen, M.A., Chuang, I.L. (2000). "Quantum Computation and Quantum Information."
//!   Cambridge University Press.

use crate::gates;
use crate::statevec::StateVec;

// ── classical helpers ─────────────────────────────────────────────────────────

/// Compute `a^exp mod n` using fast modular exponentiation.
///
/// Returns `a^exp mod n`. Panics if `n == 0`.
///
/// # Panics
///
/// Panics if `n == 0`.
#[must_use]
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    assert!(modulus > 0, "mod_pow: modulus must be nonzero");
    if modulus == 1 {
        return 0;
    }
    let mut result = 1u64;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

/// Compute the number of bits needed to represent values in `[0, n)`.
///
/// Returns `⌈log₂(n)⌉`, the minimum number of qubits to hold values `0..n-1`.
/// Returns 1 for `n ≤ 2`.
///
/// # Panics
///
/// Panics if `n == 0`.
#[must_use]
pub fn n_bits(n: u64) -> usize {
    assert!(n > 0, "n_bits: n must be nonzero");
    if n <= 2 {
        return 1;
    }
    // ⌈log₂(n)⌉ = number of bits to represent n-1
    let bits = u64::BITS - (n - 1).leading_zeros();
    bits as usize
}

// ── permutation synthesis helpers ─────────────────────────────────────────────

/// Apply a controlled permutation on `work_qubits` when `ctrl` is |1⟩.
///
/// The permutation is given as a slice `perm` where `perm[x]` is the image of `x`.
/// Only values `x < perm.len()` are permuted; values outside this range are unchanged.
///
/// The permutation is synthesized as a product of transpositions (selection-sort order),
/// applied in reverse order so that their composition equals `perm`. Each transposition is
/// implemented via a Gray code path of single-bit-difference transpositions.
///
/// # Ordering note
///
/// The selection sort produces transpositions `τ_1, ..., τ_k` such that applying them in
/// REVERSE order (τ_k first, then τ_{k-1}, ..., then τ_1) gives the permutation `perm`.
/// This is because the selection sort decomposes `perm` as `perm = τ_1 ∘ τ_2 ∘ ... ∘ τ_k`
/// (right-to-left composition), so applying τ_k first gives the correct left-to-right
/// action on basis states.
///
/// # Panics
///
/// Panics if `perm` is not a valid permutation (contains duplicates or out-of-range values).
pub(crate) fn apply_controlled_permutation(
    sv: &mut StateVec,
    ctrl: usize,
    work_qubits: &[usize],
    perm: &[usize],
) {
    let n = work_qubits.len();
    let dim = 1usize << n;
    assert!(
        perm.len() <= dim,
        "permutation length {} exceeds 2^{n} = {dim}",
        perm.len()
    );

    // Build a mutable copy of the permutation to track where values currently are.
    // `pos[v]` = current position of value `v` in the permutation array.
    let mut current: Vec<usize> = (0..perm.len()).collect();
    let mut pos: Vec<usize> = (0..perm.len()).collect();

    // Collect transpositions from selection sort.
    // The selection sort produces τ_1, ..., τ_k such that perm = τ_1 ∘ τ_2 ∘ ... ∘ τ_k.
    // To apply perm to the quantum state, we apply τ_k first, then τ_{k-1}, ..., then τ_1.
    let mut transpositions: Vec<(usize, usize)> = Vec::new();

    for i in 0..perm.len() {
        let target_val = perm[i];
        let cur_pos = pos[target_val];
        if cur_pos == i {
            continue; // already in place
        }
        transpositions.push((i, cur_pos));
        // Update tracking.
        let displaced_val = current[i];
        current[cur_pos] = displaced_val;
        current[i] = target_val;
        pos[displaced_val] = cur_pos;
        pos[target_val] = i;
    }

    // Apply transpositions in REVERSE order to implement perm on the quantum state.
    for &(a, b) in transpositions.iter().rev() {
        apply_controlled_transposition(sv, ctrl, work_qubits, a, b);
    }
}

/// Apply a controlled transposition that swaps basis states `|a⟩ ↔ |b⟩` on `work_qubits`
/// when `ctrl` is |1⟩.
///
/// Implemented via a Gray code path: a sequence of single-bit-difference transpositions,
/// each implemented as a multi-controlled-X gate. The path from `a` to `b` flips one
/// differing bit at a time; the conjugation identity `(a,b) = (a,s)·(s,b)·(a,s)` ensures
/// only `a` and `b` are swapped, leaving all other basis states unchanged.
///
/// Each single-bit-difference transposition `(s, t)` (differing only on bit `k`) is
/// implemented as: condition on all work qubits EXCEPT bit `k` matching `s` (= `t` on
/// those bits), then flip bit `k`. The target qubit `work_qubits[k]` is NOT in the control
/// list, which is required by the gate interface.
fn apply_controlled_transposition(
    sv: &mut StateVec,
    ctrl: usize,
    work_qubits: &[usize],
    a: usize,
    b: usize,
) {
    if a == b {
        return;
    }
    let n_work = work_qubits.len();
    let diff = a ^ b;

    // Collect differing bit positions (from high to low).
    // We process them in this order to build the Gray code path a → s_1 → ... → b.
    let diff_bits: Vec<usize> = (0..n_work).filter(|&k| (diff >> k) & 1 == 1).collect();

    if diff_bits.len() == 1 {
        // Single-bit difference: direct implementation.
        apply_single_bit_transposition(sv, ctrl, work_qubits, a, diff_bits[0]);
        return;
    }

    // Multi-bit difference: use the Gray code path conjugation.
    //
    // Path: a = s_0, s_1, ..., s_m = b, where s_{i+1} = s_i XOR (1 << diff_bits[i]).
    // Conjugation identity: (a, b) = (a, s_1) · (s_1, b) · (a, s_1)
    //   where (s_1, b) is a transposition with one fewer differing bit.
    //
    // Recursively: apply (a, s_1), recurse on (s_1, b), apply (a, s_1) again.
    //
    // This correctly swaps a ↔ b while leaving all other states unchanged.
    let k0 = diff_bits[0]; // first differing bit
    let s1 = a ^ (1 << k0); // intermediate state: a with bit k0 flipped

    // Apply (a, s_1): single-bit transposition
    apply_single_bit_transposition(sv, ctrl, work_qubits, a, k0);
    // Apply (s_1, b): transposition with one fewer differing bit (recurse)
    apply_controlled_transposition(sv, ctrl, work_qubits, s1, b);
    // Apply (a, s_1) again: undo the first step
    apply_single_bit_transposition(sv, ctrl, work_qubits, a, k0);
}

/// Apply a single-bit-difference transposition: swap `|a⟩ ↔ |a XOR (1<<k)⟩` when `ctrl=1`.
///
/// `a` and `a XOR (1<<k)` differ only on bit `k`. The transposition is implemented as:
/// - Condition on all work qubits EXCEPT bit `k` matching `a` (= `a XOR (1<<k)` on those bits)
/// - Flip bit `k` (target = `work_qubits[k]`)
///
/// The target qubit `work_qubits[k]` is NOT in the control list.
fn apply_single_bit_transposition(
    sv: &mut StateVec,
    ctrl: usize,
    work_qubits: &[usize],
    a: usize,
    k: usize,
) {
    let n_work = work_qubits.len();
    let target = work_qubits[k];

    // Build control list: ctrl + all work qubits EXCEPT work_qubits[k].
    // Condition: ctrl=1 AND all work qubits except k match a (= a XOR (1<<k) on those bits).
    let mut controls: Vec<usize> = Vec::with_capacity(n_work); // ctrl + (n_work - 1) others
    controls.push(ctrl);
    for j in 0..n_work {
        if j != k {
            controls.push(work_qubits[j]);
        }
    }

    // Invert work qubits (except target) where a has bit 0 (to make all controls positive).
    for j in 0..n_work {
        if j != k && (a >> j) & 1 == 0 {
            gates::x(sv, work_qubits[j]);
        }
    }

    // Apply multi-controlled-X: fires when ctrl=1 AND all non-target work qubits match a.
    gates::multi_controlled_x(sv, &controls, target);

    // Undo inversions.
    for j in 0..n_work {
        if j != k && (a >> j) & 1 == 0 {
            gates::x(sv, work_qubits[j]);
        }
    }
}

// ── public circuit builders ───────────────────────────────────────────────────

/// Apply the controlled-add-mod circuit: `|x⟩ → |x + c mod N⟩` when `ctrl` is |1⟩.
///
/// Adds the classical constant `c` to the quantum register `work_qubits` modulo `N`,
/// controlled on qubit `ctrl`. When `ctrl` is |0⟩, the register is unchanged.
///
/// The operation is a reversible permutation on the work register: values in `[0, N)` are
/// mapped to `(x + c) mod N`, and values in `[N, 2^n)` are left unchanged (they are
/// outside the modular domain).
///
/// # Arguments
///
/// - `sv` — state vector (modified in-place)
/// - `ctrl` — control qubit index
/// - `c` — classical constant to add (must satisfy `c < N`)
/// - `modulus` — the modulus `N`
/// - `work_qubits` — qubit indices of the work register (little-endian, LSB first)
///
/// # Panics
///
/// Panics if `c >= modulus`, `modulus == 0`, or any qubit index is out of range.
pub fn controlled_add_mod(
    sv: &mut StateVec,
    ctrl: usize,
    c: u64,
    modulus: u64,
    work_qubits: &[usize],
) {
    assert!(modulus > 0, "controlled_add_mod: modulus must be nonzero");
    assert!(c < modulus, "controlled_add_mod: c={c} must be < modulus={modulus}");

    if c == 0 {
        return; // adding 0 is a no-op
    }

    let n = work_qubits.len();
    let dim = 1usize << n;
    assert!(
        modulus <= dim as u64,
        "controlled_add_mod: modulus={modulus} > 2^{n}={dim}"
    );

    // Build the permutation: x → (x + c) mod N for x in [0, N), identity for x in [N, 2^n).
    let perm: Vec<usize> = (0..modulus as usize)
        .map(|x| (x + c as usize) % modulus as usize)
        .collect();

    apply_controlled_permutation(sv, ctrl, work_qubits, &perm);
}

/// Apply the inverse controlled-add-mod circuit: `|x⟩ → |x - c mod N⟩` when `ctrl` is |1⟩.
///
/// Subtracts the classical constant `c` from the quantum register `work_qubits` modulo `N`,
/// controlled on qubit `ctrl`. This is the inverse of [`controlled_add_mod`].
///
/// # Panics
///
/// Panics if `c >= modulus`, `modulus == 0`, or any qubit index is out of range.
pub fn controlled_sub_mod(
    sv: &mut StateVec,
    ctrl: usize,
    c: u64,
    modulus: u64,
    work_qubits: &[usize],
) {
    assert!(modulus > 0, "controlled_sub_mod: modulus must be nonzero");
    assert!(c < modulus, "controlled_sub_mod: c={c} must be < modulus={modulus}");

    if c == 0 {
        return;
    }

    // Subtracting c mod N = adding (N - c) mod N.
    let c_inv = modulus - c;
    controlled_add_mod(sv, ctrl, c_inv, modulus, work_qubits);
}

/// Apply the controlled-mult-mod circuit: `|x⟩ → |c · x mod N⟩` when `ctrl` is |1⟩.
///
/// Multiplies the quantum register `work_qubits` by the classical constant `c` modulo `N`,
/// controlled on qubit `ctrl`. When `ctrl` is |0⟩, the register is unchanged.
///
/// The operation is a reversible permutation on the work register (valid when `gcd(c, N) = 1`).
/// Values in `[0, N)` are mapped to `(c · x) mod N`; values in `[N, 2^n)` are left unchanged.
///
/// # Arguments
///
/// - `sv` — state vector (modified in-place)
/// - `ctrl` — control qubit index
/// - `c` — classical multiplier (must satisfy `gcd(c, N) = 1` for reversibility)
/// - `modulus` — the modulus `N`
/// - `work_qubits` — qubit indices of the work register (little-endian, LSB first)
///
/// # Panics
///
/// Panics if `modulus == 0`, `c == 0`, or any qubit index is out of range.
pub fn controlled_mult_mod(
    sv: &mut StateVec,
    ctrl: usize,
    c: u64,
    modulus: u64,
    work_qubits: &[usize],
) {
    assert!(modulus > 0, "controlled_mult_mod: modulus must be nonzero");
    assert!(c > 0, "controlled_mult_mod: c must be nonzero");

    let c_mod = c % modulus;
    if c_mod == 0 {
        // c is a multiple of N — not a valid multiplier (not invertible).
        // For well-formed inputs (gcd(c, N) = 1), this should not occur.
        return;
    }

    let n = work_qubits.len();
    let dim = 1usize << n;
    assert!(
        modulus <= dim as u64,
        "controlled_mult_mod: modulus={modulus} > 2^{n}={dim}"
    );

    // Build the permutation: x → (c * x) mod N for x in [0, N), identity for x in [N, 2^n).
    let perm: Vec<usize> = (0..modulus as usize)
        .map(|x| (c_mod as usize * x) % modulus as usize)
        .collect();

    apply_controlled_permutation(sv, ctrl, work_qubits, &perm);
}

/// Apply the inverse controlled-mult-mod circuit: `|x⟩ → |c⁻¹ · x mod N⟩` when `ctrl` is |1⟩.
///
/// Multiplies the quantum register by the modular inverse of `c` modulo `N`, controlled on
/// `ctrl`. This is the inverse of [`controlled_mult_mod`].
///
/// # Panics
///
/// Panics if `gcd(c, N) != 1` (c is not invertible mod N), `modulus == 0`, or `c == 0`.
pub fn controlled_mult_mod_inv(
    sv: &mut StateVec,
    ctrl: usize,
    c: u64,
    modulus: u64,
    work_qubits: &[usize],
) {
    assert!(modulus > 0, "controlled_mult_mod_inv: modulus must be nonzero");
    assert!(c > 0, "controlled_mult_mod_inv: c must be nonzero");

    let c_inv = mod_inverse(c % modulus, modulus)
        .expect("controlled_mult_mod_inv: c has no modular inverse (gcd(c, N) != 1)");
    controlled_mult_mod(sv, ctrl, c_inv, modulus, work_qubits);
}

/// Compute the modular inverse of `a` modulo `m` using the extended Euclidean algorithm.
///
/// Returns `Some(x)` where `a * x ≡ 1 (mod m)`, or `None` if `gcd(a, m) != 1`.
#[must_use]
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    let (g, x, _) = extended_gcd(a as i64, m as i64);
    if g != 1 {
        return None;
    }
    Some(((x % m as i64 + m as i64) % m as i64) as u64)
}

/// Extended Euclidean algorithm: returns `(gcd, x, y)` such that `a*x + b*y = gcd(a, b)`.
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    (g, y1 - (b / a) * x1, x1)
}

/// Register layout descriptor for the controlled modular-exponentiation circuit.
///
/// Documents the qubit layout (C-ModExp freeze) consumed by S.B.2's order-finding circuit.
///
/// # Layout
///
/// ```text
/// qubits [exp_start, exp_start + exp_len)    — exponent register (t bits)
/// qubits [work_start, work_start + work_len) — work register (n bits)
/// ```
///
/// The total qubit count of the `StateVec` must be at least `exp_start + exp_len` and
/// `work_start + work_len`.
#[derive(Clone, Debug)]
pub struct ModExpLayout {
    /// Index of the first exponent qubit (little-endian LSB).
    pub exp_start: usize,
    /// Number of exponent qubits (`t`).
    pub exp_len: usize,
    /// Index of the first work qubit (little-endian LSB).
    pub work_start: usize,
    /// Number of work qubits (`n`, where `N < 2^n`).
    pub work_len: usize,
    /// The modulus `N`.
    pub modulus: u64,
}

impl ModExpLayout {
    /// Construct a `ModExpLayout` for modulus `N` with the standard qubit assignment.
    ///
    /// Standard layout (little-endian, all registers contiguous):
    /// - Exponent register: qubits `[0, t)` where `t = exp_len`
    /// - Work register: qubits `[t, t + n)` where `n = ⌈log₂(N+1)⌉`
    ///
    /// Total qubits required: `t + n`.
    ///
    /// # Panics
    ///
    /// Panics if `modulus == 0` or `exp_len == 0`.
    #[must_use]
    pub fn standard(modulus: u64, exp_len: usize) -> Self {
        assert!(modulus > 0, "ModExpLayout::standard: modulus must be nonzero");
        assert!(exp_len > 0, "ModExpLayout::standard: exp_len must be nonzero");
        let work_len = n_bits(modulus + 1);
        Self {
            exp_start: 0,
            exp_len,
            work_start: exp_len,
            work_len,
            modulus,
        }
    }

    /// Total number of qubits required for this layout.
    #[must_use]
    pub fn total_qubits(&self) -> usize {
        // Maximum of the end of each register.
        let exp_end = self.exp_start + self.exp_len;
        let work_end = self.work_start + self.work_len;
        exp_end.max(work_end)
    }

    /// Qubit indices of the exponent register (little-endian, LSB first).
    #[must_use]
    pub fn exp_qubits(&self) -> Vec<usize> {
        (self.exp_start..self.exp_start + self.exp_len).collect()
    }

    /// Qubit indices of the work register (little-endian, LSB first).
    #[must_use]
    pub fn work_qubits(&self) -> Vec<usize> {
        (self.work_start..self.work_start + self.work_len).collect()
    }
}

/// Apply the controlled modular-exponentiation circuit: `|x⟩|y⟩ → |x⟩|y · aˣ mod N⟩`.
///
/// Applies the controlled-`aˣ mod N` operation in-place on `sv`. The exponent register `x`
/// controls the operation; the work register `y` is multiplied by `aˣ mod N`. When all
/// exponent qubits are |0⟩, the work register is unchanged (control-off no-op).
///
/// # Circuit structure
///
/// For each exponent qubit `k` (from LSB to MSB), applies a controlled-mult-mod by
/// `a^{2^k} mod N` on the work register, controlled on exponent qubit `k`. The powers
/// `a^{2^k} mod N` are classically precomputed.
///
/// # Arguments
///
/// - `sv` — state vector (modified in-place)
/// - `a` — the base of the exponentiation (must satisfy `gcd(a, N) = 1`)
/// - `layout` — register layout descriptor (C-ModExp freeze)
///
/// # Panics
///
/// Panics if `gcd(a, N) != 1`, `a == 0`, or any qubit index is out of range.
pub fn controlled_mod_exp(sv: &mut StateVec, a: u64, layout: &ModExpLayout) {
    let modulus = layout.modulus;
    assert!(a > 0, "controlled_mod_exp: a must be nonzero");
    assert!(
        mod_inverse(a % modulus, modulus).is_some(),
        "controlled_mod_exp: gcd(a, N) != 1 — a={a}, N={modulus}"
    );

    let work_qubits = layout.work_qubits();
    let exp_qubits = layout.exp_qubits();

    // For each exponent qubit k (LSB = qubit 0 of exponent register),
    // apply controlled-mult-mod by a^{2^k} mod N, controlled on exp_qubits[k].
    //
    // The exponent register encodes x = Σ x_k · 2^k (little-endian).
    // aˣ = Π_{k: x_k=1} a^{2^k} mod N.
    // Each factor is applied as a controlled-mult-mod stage.
    let mut power = a % modulus; // a^{2^0} mod N
    for &exp_qubit in &exp_qubits {
        // Apply controlled-mult-mod by `power` on the work register, controlled on exp_qubit.
        controlled_mult_mod(sv, exp_qubit, power, modulus, &work_qubits);
        // Advance to next power: a^{2^{k+1}} = (a^{2^k})^2 mod N.
        power = power * power % modulus;
    }
}

/// Apply the inverse controlled modular-exponentiation circuit.
///
/// This is the inverse of [`controlled_mod_exp`]: applies the stages in reverse order,
/// each using the inverse multiplier. Used for reversibility verification.
///
/// # Panics
///
/// Panics if `gcd(a, N) != 1`, `a == 0`, or any qubit index is out of range.
pub fn controlled_mod_exp_inv(sv: &mut StateVec, a: u64, layout: &ModExpLayout) {
    let modulus = layout.modulus;
    assert!(a > 0, "controlled_mod_exp_inv: a must be nonzero");
    assert!(
        mod_inverse(a % modulus, modulus).is_some(),
        "controlled_mod_exp_inv: gcd(a, N) != 1 — a={a}, N={modulus}"
    );

    let work_qubits = layout.work_qubits();
    let exp_qubits = layout.exp_qubits();

    // Precompute all powers a^{2^k} mod N.
    let mut powers = Vec::with_capacity(exp_qubits.len());
    let mut power = a % modulus;
    for _ in &exp_qubits {
        powers.push(power);
        power = power * power % modulus;
    }

    // Apply inverse stages in reverse order.
    for (&exp_qubit, &pw) in exp_qubits.iter().zip(powers.iter()).rev() {
        controlled_mult_mod_inv(sv, exp_qubit, pw, modulus, &work_qubits);
    }
}

// ── re-exports for tests ──────────────────────────────────────────────────────

/// Read the basis-state index of the work register from a state vector.
///
/// Extracts the classical value encoded in `work_qubits` from a basis state `sv`.
/// Returns the integer value `Σ bit_k · 2^k` (little-endian).
///
/// # Panics
///
/// Panics if `sv` is not a basis state (more than one nonzero amplitude).
#[must_use]
pub fn read_work_register(sv: &StateVec, work_qubits: &[usize]) -> u64 {
    // Find the unique nonzero amplitude index.
    let basis_idx = find_basis_index(sv);
    // Extract the work register bits.
    let mut val = 0u64;
    for (k, &q) in work_qubits.iter().enumerate() {
        if (basis_idx >> q) & 1 == 1 {
            val |= 1 << k;
        }
    }
    val
}

/// Find the unique nonzero amplitude index in a basis state.
///
/// # Panics
///
/// Panics if `sv` is not a basis state.
#[must_use]
pub fn find_basis_index(sv: &StateVec) -> usize {
    let eps = 1e-9;
    let mut found = None;
    for (i, a) in sv.amplitudes().iter().enumerate() {
        if a.norm() > eps {
            assert!(
                found.is_none(),
                "find_basis_index: state is not a basis state (multiple nonzero amplitudes)"
            );
            found = Some(i);
        }
    }
    found.expect("find_basis_index: state is all-zero (not a valid quantum state)")
}
