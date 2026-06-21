//! Universal gate set applied by amplitude-pair iteration.
//!
//! # Gate-application method (FIXED — C-StateVec)
//!
//! Every gate is applied **in-place** by iterating the pairs of amplitudes it couples, updating
//! them with the 2×2 unitary matrix of the gate. This is the standard state-vector method:
//! O(2^n) per gate, never O(4^n) matrix materialization.
//!
//! For a single-qubit gate on qubit `t` in an n-qubit register, the coupled pairs are all
//! index pairs `(i0, i1)` where `i0` has bit `t` clear and `i1 = i0 | (1 << t)`.
//!
//! For a controlled gate with control qubit `c` and target qubit `t`, only pairs where the
//! control bit is set are updated.
//!
//! # Basis-indexing convention
//!
//! Little-endian (qubit 0 = LSB), fixed in [`crate::statevec`]. See [`crate::statevec`] for the
//! full convention description.
//!
//! # Gate set (C-StateVec — frozen)
//!
//! - **Single-qubit:** X, Y, Z, H, S, T, phase(θ), arbitrary unitary U(a,b,c,d)
//! - **Two-qubit:** CNOT, controlled-phase(θ), SWAP
//! - **Multi-qubit:** Toffoli (CCX), multi-controlled-X (generalized Toffoli)
//!
//! The multi-controlled surface is over-specified for the gate-set KATs (which only exercise
//! single- and two-qubit gates) but is carried for modular exponentiation ([`crate::arith`]) and
//! the Proos–Zalka ECDLP circuit ([`crate::ecdlp`]) per the Category-A rule.

use std::f64::consts::PI;

use num_complex::Complex;

use crate::statevec::StateVec;

// ── internal helpers ──────────────────────────────────────────────────────────

/// Apply a 2×2 unitary [[a, b], [c, d]] to the amplitude pair (amp0, amp1) in-place.
///
/// The update rule is: amp0' = a·amp0 + b·amp1, amp1' = c·amp0 + d·amp1.
#[inline]
fn apply_2x2(
    amp0: &mut Complex<f64>,
    amp1: &mut Complex<f64>,
    a: Complex<f64>,
    b: Complex<f64>,
    c: Complex<f64>,
    d: Complex<f64>,
) {
    let v0 = *amp0;
    let v1 = *amp1;
    *amp0 = a * v0 + b * v1;
    *amp1 = c * v0 + d * v1;
}

/// Iterate all amplitude pairs coupled by a single-qubit gate on qubit `target`.
///
/// For each pair `(i0, i1)` where bit `target` is 0 in `i0` and `i1 = i0 | (1 << target)`,
/// calls `f(amp[i0], amp[i1])`.
fn apply_single_qubit<F>(sv: &mut StateVec, target: usize, mut f: F)
where
    F: FnMut(&mut Complex<f64>, &mut Complex<f64>),
{
    let n = sv.n_qubits();
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let dim = sv.dim();
    let mask = 1usize << target;
    let amps = sv.amplitudes_mut();
    // Iterate all indices with bit `target` = 0.
    // The standard pattern: outer loop over the upper bits, inner loop over the lower bits.
    // half = dim / 2 = number of pairs.
    for i in 0..dim {
        if i & mask == 0 {
            let i0 = i;
            let i1 = i | mask;
            // i0 < i1 always (bit `target` is 0 in i0, 1 in i1).
            let (lo, hi) = amps.split_at_mut(i1);
            f(&mut lo[i0], &mut hi[0]);
        }
    }
}

/// Iterate all amplitude pairs coupled by a controlled single-qubit gate.
///
/// Only pairs where the control qubit bit is set are updated.
fn apply_controlled_single_qubit<F>(sv: &mut StateVec, control: usize, target: usize, mut f: F)
where
    F: FnMut(&mut Complex<f64>, &mut Complex<f64>),
{
    let n = sv.n_qubits();
    assert!(control < n, "control qubit {control} out of range for {n}-qubit register");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    assert_ne!(control, target, "control and target qubits must differ");
    let dim = sv.dim();
    let ctrl_mask = 1usize << control;
    let tgt_mask = 1usize << target;
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        // Only process pairs where: control bit is set, target bit is 0.
        if (i & ctrl_mask != 0) && (i & tgt_mask == 0) {
            let i0 = i;
            let i1 = i | tgt_mask;
            let (lo, hi) = amps.split_at_mut(i1);
            f(&mut lo[i0], &mut hi[0]);
        }
    }
}

// ── single-qubit gates ────────────────────────────────────────────────────────

/// Apply the Pauli-X (NOT) gate to qubit `target`.
///
/// Matrix: [[0, 1], [1, 0]].
pub fn x(sv: &mut StateVec, target: usize) {
    apply_single_qubit(sv, target, |a0, a1| {
        std::mem::swap(a0, a1);
    });
}

/// Apply the Pauli-Y gate to qubit `target`.
///
/// Matrix: [[0, -i], [i, 0]].
pub fn y(sv: &mut StateVec, target: usize) {
    let mi = Complex::new(0.0, -1.0);
    let pi = Complex::new(0.0, 1.0);
    apply_single_qubit(sv, target, |a0, a1| {
        apply_2x2(
            a0,
            a1,
            Complex::new(0.0, 0.0),
            mi,
            pi,
            Complex::new(0.0, 0.0),
        );
    });
}

/// Apply the Pauli-Z gate to qubit `target`.
///
/// Matrix: [[1, 0], [0, -1]].
pub fn z(sv: &mut StateVec, target: usize) {
    apply_single_qubit(sv, target, |_a0, a1| {
        *a1 = -*a1;
    });
}

/// Apply the Hadamard gate to qubit `target`.
///
/// Matrix: (1/√2) · [[1, 1], [1, -1]].
pub fn h(sv: &mut StateVec, target: usize) {
    let inv_sqrt2 = Complex::new(std::f64::consts::FRAC_1_SQRT_2, 0.0);
    apply_single_qubit(sv, target, |a0, a1| {
        apply_2x2(a0, a1, inv_sqrt2, inv_sqrt2, inv_sqrt2, -inv_sqrt2);
    });
}

/// Apply the S (phase) gate to qubit `target`.
///
/// Matrix: [[1, 0], [0, i]]. Satisfies S² = Z.
pub fn s(sv: &mut StateVec, target: usize) {
    let i = Complex::new(0.0, 1.0);
    apply_single_qubit(sv, target, |_a0, a1| {
        *a1 = i * *a1;
    });
}

/// Apply the T gate to qubit `target`.
///
/// Matrix: [[1, 0], [0, e^{iπ/4}]]. Satisfies T² = S.
pub fn t(sv: &mut StateVec, target: usize) {
    let phase = Complex::new(0.0, PI / 4.0).exp();
    apply_single_qubit(sv, target, |_a0, a1| {
        *a1 = phase * *a1;
    });
}

/// Apply a phase gate with angle `theta` (radians) to qubit `target`.
///
/// Matrix: [[1, 0], [0, e^{iθ}]].
pub fn phase(sv: &mut StateVec, target: usize, theta: f64) {
    let p = Complex::new(0.0, theta).exp();
    apply_single_qubit(sv, target, |_a0, a1| {
        *a1 = p * *a1;
    });
}

/// Apply an arbitrary single-qubit unitary U to qubit `target`.
///
/// The unitary is specified by its four entries in row-major order:
/// `[[u00, u01], [u10, u11]]`.
///
/// # Panics
///
/// Does not check unitarity; the caller is responsible for providing a unitary matrix.
pub fn unitary(
    sv: &mut StateVec,
    target: usize,
    u00: Complex<f64>,
    u01: Complex<f64>,
    u10: Complex<f64>,
    u11: Complex<f64>,
) {
    apply_single_qubit(sv, target, |a0, a1| {
        apply_2x2(a0, a1, u00, u01, u10, u11);
    });
}

// ── two-qubit gates ───────────────────────────────────────────────────────────

/// Apply the CNOT (controlled-X) gate: control qubit `c`, target qubit `t`.
///
/// Flips the target qubit when the control qubit is |1⟩.
pub fn cnot(sv: &mut StateVec, control: usize, target: usize) {
    apply_controlled_single_qubit(sv, control, target, |a0, a1| {
        std::mem::swap(a0, a1);
    });
}

/// Apply a controlled-phase gate with angle `theta` (radians).
///
/// Applies e^{iθ} to the |11⟩ component: the amplitude of basis states where both
/// `control` and `target` are |1⟩ is multiplied by e^{iθ}.
pub fn controlled_phase(sv: &mut StateVec, control: usize, target: usize, theta: f64) {
    let n = sv.n_qubits();
    assert!(control < n, "control qubit {control} out of range for {n}-qubit register");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    assert_ne!(control, target, "control and target qubits must differ");
    let p = Complex::new(0.0, theta).exp();
    let ctrl_mask = 1usize << control;
    let tgt_mask = 1usize << target;
    let both_mask = ctrl_mask | tgt_mask;
    let dim = sv.dim();
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        if i & both_mask == both_mask {
            amps[i] *= p;
        }
    }
}

/// Apply the SWAP gate between qubits `a` and `b`.
///
/// Exchanges the amplitudes of all basis states that differ only in qubits `a` and `b`.
pub fn swap(sv: &mut StateVec, qa: usize, qb: usize) {
    let n = sv.n_qubits();
    assert!(qa < n, "qubit {qa} out of range for {n}-qubit register");
    assert!(qb < n, "qubit {qb} out of range for {n}-qubit register");
    assert_ne!(qa, qb, "SWAP qubits must differ");
    let mask_a = 1usize << qa;
    let mask_b = 1usize << qb;
    let dim = sv.dim();
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        // Only process pairs once: require bit a=0, bit b=1.
        if (i & mask_a == 0) && (i & mask_b != 0) {
            let j = (i | mask_a) & !mask_b; // flip a=1, b=0
            amps.swap(i, j);
        }
    }
}

// ── multi-qubit gates ─────────────────────────────────────────────────────────

/// Apply the Toffoli (CCX) gate: controls `c0` and `c1`, target `t`.
///
/// Flips the target qubit when both control qubits are |1⟩.
pub fn toffoli(sv: &mut StateVec, c0: usize, c1: usize, target: usize) {
    let n = sv.n_qubits();
    assert!(c0 < n, "control qubit c0={c0} out of range for {n}-qubit register");
    assert!(c1 < n, "control qubit c1={c1} out of range for {n}-qubit register");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    assert_ne!(c0, c1, "Toffoli control qubits must differ");
    assert_ne!(c0, target, "Toffoli c0 and target must differ");
    assert_ne!(c1, target, "Toffoli c1 and target must differ");
    let ctrl_mask = (1usize << c0) | (1usize << c1);
    let tgt_mask = 1usize << target;
    let dim = sv.dim();
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        if (i & ctrl_mask == ctrl_mask) && (i & tgt_mask == 0) {
            let j = i | tgt_mask;
            amps.swap(i, j);
        }
    }
}

/// Apply a multi-controlled-X gate: flip `target` when all `controls` are |1⟩.
///
/// Generalizes Toffoli to an arbitrary number of control qubits. This is the over-specified
/// surface carried for modular exponentiation ([`crate::arith`]) and the Proos–Zalka ECDLP
/// circuit ([`crate::ecdlp`]).
///
/// # Panics
///
/// Panics if `controls` is empty, if any qubit index is out of range, or if any two qubit
/// indices coincide.
pub fn multi_controlled_x(sv: &mut StateVec, controls: &[usize], target: usize) {
    let n = sv.n_qubits();
    assert!(!controls.is_empty(), "multi_controlled_x requires at least one control qubit");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    for (i, &c) in controls.iter().enumerate() {
        assert!(c < n, "control qubit {c} out of range for {n}-qubit register");
        assert_ne!(c, target, "control qubit {c} coincides with target");
        for &c2 in &controls[..i] {
            assert_ne!(c, c2, "duplicate control qubit {c}");
        }
    }
    let ctrl_mask: usize = controls.iter().map(|&c| 1usize << c).fold(0, |acc, m| acc | m);
    let tgt_mask = 1usize << target;
    let dim = sv.dim();
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        if (i & ctrl_mask == ctrl_mask) && (i & tgt_mask == 0) {
            let j = i | tgt_mask;
            amps.swap(i, j);
        }
    }
}

/// Apply a multi-controlled single-qubit unitary gate.
///
/// Applies the 2×2 unitary `[[u00, u01], [u10, u11]]` to `target` when all `controls` are |1⟩.
/// This is the most general multi-controlled gate; CNOT, Toffoli, and multi-controlled-X are
/// special cases.
///
/// # Panics
///
/// Panics if `controls` is empty, if any qubit index is out of range, or if any two qubit
/// indices coincide.
#[allow(clippy::too_many_arguments)]
pub fn multi_controlled_unitary(
    sv: &mut StateVec,
    controls: &[usize],
    target: usize,
    u00: Complex<f64>,
    u01: Complex<f64>,
    u10: Complex<f64>,
    u11: Complex<f64>,
) {
    let n = sv.n_qubits();
    assert!(!controls.is_empty(), "multi_controlled_unitary requires at least one control qubit");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    for (i, &c) in controls.iter().enumerate() {
        assert!(c < n, "control qubit {c} out of range for {n}-qubit register");
        assert_ne!(c, target, "control qubit {c} coincides with target");
        for &c2 in &controls[..i] {
            assert_ne!(c, c2, "duplicate control qubit {c}");
        }
    }
    let ctrl_mask: usize = controls.iter().map(|&c| 1usize << c).fold(0, |acc, m| acc | m);
    let tgt_mask = 1usize << target;
    let dim = sv.dim();
    let amps = sv.amplitudes_mut();
    for i in 0..dim {
        if (i & ctrl_mask == ctrl_mask) && (i & tgt_mask == 0) {
            let j = i | tgt_mask;
            let (lo, hi) = amps.split_at_mut(j);
            apply_2x2(&mut lo[i], &mut hi[0], u00, u01, u10, u11);
        }
    }
}

// ── re-export PI for use in KATs ──────────────────────────────────────────────

/// π (re-exported for use in tests and downstream crates).
pub use std::f64::consts::PI as PI_F64;
