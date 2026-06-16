//! Quantum Fourier Transform (QFT) over the dense state-vector register.
//!
//! # Algorithm
//!
//! The QFT is implemented as the standard Hadamard + controlled-phase ladder, O(n²) gates,
//! built entirely from the frozen S.A.1 gate set ([`crate::gates::h`] and
//! [`crate::gates::controlled_phase`]).
//!
//! For an n-qubit register the QFT maps basis state |j⟩ to
//!
//! ```text
//! QFT|j⟩ = (1/√N) Σ_{k=0}^{N-1} ω^{jk} |k⟩,   N = 2^n,  ω = e^{2πi/N}
//! ```
//!
//! # Bit-reversal convention (LOAD-BEARING — read before consuming in S.B/S.C)
//!
//! **The standard QFT circuit (Nielsen & Chuang) is designed for big-endian qubit ordering.**
//! In the little-endian convention (qubit 0 = LSB), the H + controlled-phase ladder (processing
//! qubits 0 to n-1) maps |j⟩ → QFT|j_reversed⟩ instead of QFT|j⟩. This is a silent
//! wrong-answer bug for S.B/S.C's period extraction.
//!
//! The fix: **one input bit-reversal step** (swap qubit i with qubit n-1-i) before the ladder.
//! After the bit-reversal, the circuit sees |j_reversed⟩ and outputs QFT|j⟩ directly in
//! natural little-endian order — no output bit-reversal is needed.
//!
//! This implementation includes the input bit-reversal in [`qft`]. The net result:
//!
//! - After `qft(sv)`, basis index `k` of `sv` corresponds to Fourier mode `k` in the
//!   little-endian convention (qubit 0 = LSB). No further reordering is needed.
//! - `iqft(qft(sv)) == sv` (the identity KAT).
//!
//! **If you remove the bit-reversal step**, the circuit computes QFT|j_reversed⟩ instead of
//! QFT|j⟩ — S.B/S.C will read the period out of the wrong qubit order. The bit-reversal is
//! the silent-wrong-answer guard.
//!
//! # Inverse QFT
//!
//! [`iqft`] applies the inverse QFT: the inverse controlled-phase + Hadamard ladder (phases
//! negated, ladder reversed), then the output bit-reversal (to undo the forward QFT's input
//! bit-reversal). Satisfies `iqft(qft(sv)) == sv` within floating-point tolerance.
//!
//! # Resource-scale ceiling
//!
//! The QFT applies O(n²) gates, each O(2^n) in the dense register. Total cost: O(n² · 2^n).
//! At n = 25 this is ~800 M gate operations — feasible but slow. The ~25-qubit ceiling is the
//! same resource-scale wall as the register itself (principle 4).

use std::f64::consts::PI;

use crate::gates::{controlled_phase, h, swap};
use crate::statevec::StateVec;

/// Apply the Quantum Fourier Transform to `sv` in-place.
///
/// After this call, the register holds the QFT of the input state in natural little-endian
/// order. See the module-level documentation for the bit-reversal convention.
///
/// Built from the frozen S.A.1 gate set: H + controlled-phase ladder + SWAP for bit-reversal.
///
/// # Circuit structure
///
/// 1. **Input bit-reversal**: swap qubit i with qubit n-1-i. Required to adapt the standard
///    big-endian QFT circuit for the little-endian convention (see module docs).
/// 2. **H + controlled-phase ladder**: for each qubit j from 0 to n-1, apply H then
///    controlled-phase(2π/2^{k-j+1}) for each k from j+1 to n-1.
///
/// After the input bit-reversal, the ladder correctly computes QFT|j⟩ in natural little-endian
/// order — no output bit-reversal is needed.
///
/// # Panics
///
/// Panics if `sv.n_qubits() == 0` (impossible by construction) or if any gate assertion fires.
pub fn qft(sv: &mut StateVec) {
    let n = sv.n_qubits();
    // Step 1: Input bit-reversal — converts little-endian input to big-endian order.
    //
    // The standard QFT circuit (H + controlled-phase ladder, processing qubits 0 to n-1)
    // maps |j⟩ → QFT|j_reversed⟩ in the little-endian basis. To compute QFT|j⟩, we first
    // bit-reverse the input so the circuit sees |j_reversed⟩ and outputs QFT|j⟩.
    //
    // Without this step, the circuit computes QFT|j_reversed⟩ instead of QFT|j⟩ — a silent
    // wrong-answer bug for S.B/S.C's period extraction.
    for i in 0..(n / 2) {
        swap(sv, i, n - 1 - i);
    }
    // Step 2: Standard QFT circuit (big-endian): for each qubit j from 0 to n-1:
    //   1. Apply H to qubit j.
    //   2. For each k from j+1 to n-1: apply controlled-phase(2π/2^{k-j+1}) with control k,
    //      target j.
    // After the input bit-reversal, this correctly computes QFT|j⟩ in natural little-endian
    // order — no output bit-reversal is needed.
    for j in 0..n {
        h(sv, j);
        for k in (j + 1)..n {
            let theta = 2.0 * PI / (1usize << (k - j + 1)) as f64;
            controlled_phase(sv, k, j, theta);
        }
    }
}

/// Apply the inverse Quantum Fourier Transform to `sv` in-place.
///
/// Satisfies `iqft(qft(sv)) == sv` within floating-point tolerance (the identity KAT).
///
/// The inverse circuit reverses the forward QFT steps:
/// 1. Inverse controlled-phase + Hadamard ladder (phases negated, ladder reversed).
/// 2. Output bit-reversal (to undo the forward QFT's input bit-reversal).
///
/// # Panics
///
/// Panics if any gate assertion fires.
pub fn iqft(sv: &mut StateVec) {
    let n = sv.n_qubits();
    // The inverse QFT reverses the forward QFT steps:
    //   Forward: (1) input bit-reversal, (2) H + controlled-phase ladder.
    //   Inverse: (1) inverse controlled-phase + Hadamard ladder (reversed), (2) output bit-reversal.
    //
    // Step 1: Inverse of the Hadamard + controlled-phase ladder (reversed order, negated phases).
    for j in (0..n).rev() {
        for k in ((j + 1)..n).rev() {
            let theta = -2.0 * PI / (1usize << (k - j + 1)) as f64;
            controlled_phase(sv, k, j, theta);
        }
        h(sv, j);
    }
    // Step 2: Undo the forward QFT's input bit-reversal.
    for i in 0..(n / 2) {
        swap(sv, i, n - 1 - i);
    }
}
