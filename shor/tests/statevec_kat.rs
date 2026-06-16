//! Known-answer tests for the `StateVec` register and universal gate set.
//!
//! # What is tested
//!
//! 1. **Unitarity** — each gate U satisfies U U† = I: applying U then U† to a basis state
//!    returns the original state within tolerance.
//! 2. **Bell state** — H on qubit 0 then CNOT(0→1) on |00⟩ gives (|00⟩ + |11⟩)/√2.
//!    Published amplitudes: a[0] = 1/√2, a[1] = 0, a[2] = 0, a[3] = 1/√2.
//! 3. **GHZ state** — n-qubit GHZ: H on qubit 0, then CNOT(0→k) for k=1..n-1.
//!    Amplitudes: a[0] = 1/√2, a[2^n - 1] = 1/√2, all others 0.
//! 4. **Normalization** — Σ|aᵢ|² = 1 preserved across arbitrary gate sequences.
//! 5. **Gate identities** — HH = I, XX = I, S² = Z, T² = S (published single-qubit relations).
//!
//! # Basis-indexing convention
//!
//! Little-endian: qubit 0 = LSB. For a 2-qubit register:
//!   index 0 = |00⟩, index 1 = |10⟩ (qubit 0 = 1, qubit 1 = 0),
//!   index 2 = |01⟩ (qubit 0 = 0, qubit 1 = 1), index 3 = |11⟩.
//!
//! Wait — let us be precise. With little-endian (qubit 0 = LSB):
//!   index i encodes |q_{n-1}…q_1 q_0⟩ where q_k = (i >> k) & 1.
//!   index 0 = |q1=0, q0=0⟩ = |00⟩
//!   index 1 = |q1=0, q0=1⟩ = |01⟩  (qubit 0 is the rightmost in ket notation)
//!   index 2 = |q1=1, q0=0⟩ = |10⟩
//!   index 3 = |q1=1, q0=1⟩ = |11⟩
//!
//! Bell state (|00⟩ + |11⟩)/√2: index 0 (|00⟩) and index 3 (|11⟩) each have amplitude 1/√2.

use std::f64::consts::PI;

use num_complex::Complex;
use shor::gates;
use shor::statevec::{StateVec, EPS};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Assert that two complex amplitudes agree within tolerance.
fn assert_amp_eq(got: Complex<f64>, expected: Complex<f64>, label: &str) {
    let diff = (got - expected).norm();
    assert!(
        diff < EPS * 1e4, // 1e-6 tolerance for accumulated f64 error
        "{label}: amplitude mismatch: got {got:.6e}, expected {expected:.6e}, diff={diff:.2e}"
    );
}

/// Assert that two amplitude slices agree element-wise within tolerance.
fn assert_amps_eq(got: &[Complex<f64>], expected: &[Complex<f64>], label: &str) {
    assert_eq!(got.len(), expected.len(), "{label}: amplitude vector length mismatch");
    for (i, (&g, &e)) in got.iter().zip(expected.iter()).enumerate() {
        assert_amp_eq(g, e, &format!("{label}[{i}]"));
    }
}

/// Assert that a state vector is normalized within tolerance.
fn assert_normalized(sv: &StateVec, label: &str) {
    let norm_sq = sv.norm_sq();
    assert!(
        (norm_sq - 1.0).abs() < EPS * 1e4,
        "{label}: normalization violated: Σ|aᵢ|² = {norm_sq:.10}"
    );
}

/// Real complex number shorthand.
fn re(x: f64) -> Complex<f64> {
    Complex::new(x, 0.0)
}

/// Imaginary complex number shorthand.
fn im(x: f64) -> Complex<f64> {
    Complex::new(0.0, x)
}

// ── unitarity KATs ────────────────────────────────────────────────────────────
//
// For each gate U, verify U U† = I by:
//   1. Start with a basis state |k⟩.
//   2. Apply U.
//   3. Apply U† (the conjugate transpose / inverse).
//   4. Assert the result equals |k⟩.
//
// Single-qubit gate inverses:
//   X† = X, Y† = Y, Z† = Z, H† = H (all Hermitian / self-inverse)
//   S† = phase(-π/2), T† = phase(-π/4)
//   phase(θ)† = phase(-θ)

/// Unitarity of X: X·X = I.
#[test]
fn unitarity_x() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::x(&mut sv, 0);
        gates::x(&mut sv, 0);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("X·X=I on |{k}⟩"));
    }
}

/// Unitarity of Y: Y·Y = I.
#[test]
fn unitarity_y() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::y(&mut sv, 0);
        gates::y(&mut sv, 0);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("Y·Y=I on |{k}⟩"));
    }
}

/// Unitarity of Z: Z·Z = I.
#[test]
fn unitarity_z() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::z(&mut sv, 0);
        gates::z(&mut sv, 0);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("Z·Z=I on |{k}⟩"));
    }
}

/// Unitarity of H: H·H = I.
#[test]
fn unitarity_h() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::h(&mut sv, 0);
        gates::h(&mut sv, 0);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("H·H=I on |{k}⟩"));
    }
}

/// Unitarity of S: S·S† = I (S† = phase(-π/2)).
#[test]
fn unitarity_s() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::s(&mut sv, 0);
        gates::phase(&mut sv, 0, -PI / 2.0); // S†
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("S·S†=I on |{k}⟩"));
    }
}

/// Unitarity of T: T·T† = I (T† = phase(-π/4)).
#[test]
fn unitarity_t() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::t(&mut sv, 0);
        gates::phase(&mut sv, 0, -PI / 4.0); // T†
        let expected = StateVec::basis(2, k);
        assert_amps_eq(sv.amplitudes(), expected.amplitudes(), &format!("T·T†=I on |{k}⟩"));
    }
}

/// Unitarity of CNOT: CNOT·CNOT = I (CNOT is self-inverse).
#[test]
fn unitarity_cnot() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::cnot(&mut sv, 0, 1);
        gates::cnot(&mut sv, 0, 1);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(
            sv.amplitudes(),
            expected.amplitudes(),
            &format!("CNOT·CNOT=I on |{k}⟩"),
        );
    }
}

/// Unitarity of SWAP: SWAP·SWAP = I (SWAP is self-inverse).
#[test]
fn unitarity_swap() {
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::swap(&mut sv, 0, 1);
        gates::swap(&mut sv, 0, 1);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(
            sv.amplitudes(),
            expected.amplitudes(),
            &format!("SWAP·SWAP=I on |{k}⟩"),
        );
    }
}

/// Unitarity of Toffoli: Toffoli·Toffoli = I (Toffoli is self-inverse).
#[test]
fn unitarity_toffoli() {
    for k in 0..8usize {
        let mut sv = StateVec::basis(3, k);
        gates::toffoli(&mut sv, 0, 1, 2);
        gates::toffoli(&mut sv, 0, 1, 2);
        let expected = StateVec::basis(3, k);
        assert_amps_eq(
            sv.amplitudes(),
            expected.amplitudes(),
            &format!("Toffoli·Toffoli=I on |{k}⟩"),
        );
    }
}

/// Unitarity of controlled-phase: CP(θ)·CP(-θ) = I.
#[test]
fn unitarity_controlled_phase() {
    let theta = PI / 3.0;
    for k in 0..4usize {
        let mut sv = StateVec::basis(2, k);
        gates::controlled_phase(&mut sv, 0, 1, theta);
        gates::controlled_phase(&mut sv, 0, 1, -theta);
        let expected = StateVec::basis(2, k);
        assert_amps_eq(
            sv.amplitudes(),
            expected.amplitudes(),
            &format!("CP(θ)·CP(-θ)=I on |{k}⟩"),
        );
    }
}

// ── Bell state KAT ────────────────────────────────────────────────────────────
//
// Circuit: H(q0) then CNOT(q0→q1) on |00⟩.
// Result: (|00⟩ + |11⟩)/√2.
//
// Little-endian index mapping for 2 qubits:
//   index 0 = |q1=0, q0=0⟩ = |00⟩  → amplitude 1/√2
//   index 1 = |q1=0, q0=1⟩ = |01⟩  → amplitude 0
//   index 2 = |q1=1, q0=0⟩ = |10⟩  → amplitude 0
//   index 3 = |q1=1, q0=1⟩ = |11⟩  → amplitude 1/√2
//
// Reference: Nielsen & Chuang, "Quantum Computation and Quantum Information", §1.3.6.

/// Bell state: H(q0) then CNOT(q0→q1) on |00⟩ gives (|00⟩ + |11⟩)/√2.
#[test]
fn bell_state() {
    let mut sv = StateVec::zero(2); // |00⟩
    gates::h(&mut sv, 0); // H on qubit 0 → (|00⟩ + |01⟩)/√2
    gates::cnot(&mut sv, 0, 1); // CNOT(0→1) → (|00⟩ + |11⟩)/√2

    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    // index 0 = |00⟩: amplitude 1/√2
    assert_amp_eq(sv.amplitudes()[0], re(inv_sqrt2), "Bell[0]=1/√2");
    // index 1 = |01⟩: amplitude 0
    assert_amp_eq(sv.amplitudes()[1], re(0.0), "Bell[1]=0");
    // index 2 = |10⟩: amplitude 0
    assert_amp_eq(sv.amplitudes()[2], re(0.0), "Bell[2]=0");
    // index 3 = |11⟩: amplitude 1/√2
    assert_amp_eq(sv.amplitudes()[3], re(inv_sqrt2), "Bell[3]=1/√2");

    assert_normalized(&sv, "Bell state");
}

// ── GHZ state KAT ─────────────────────────────────────────────────────────────
//
// n-qubit GHZ state: H(q0), then CNOT(q0→q1), CNOT(q0→q2), …, CNOT(q0→q_{n-1}).
// Result: (|0…0⟩ + |1…1⟩)/√2.
//
// Little-endian: |0…0⟩ = index 0, |1…1⟩ = index 2^n - 1.
// All other amplitudes are 0.
//
// Reference: Greenberger, Horne, Zeilinger (1989); standard quantum information textbook result.

/// GHZ state for n=3: (|000⟩ + |111⟩)/√2.
#[test]
fn ghz_state_3() {
    let n = 3;
    let mut sv = StateVec::zero(n);
    gates::h(&mut sv, 0);
    gates::cnot(&mut sv, 0, 1);
    gates::cnot(&mut sv, 0, 2);

    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    let dim = 1usize << n;
    for i in 0..dim {
        let expected = if i == 0 || i == dim - 1 { re(inv_sqrt2) } else { re(0.0) };
        assert_amp_eq(sv.amplitudes()[i], expected, &format!("GHZ3[{i}]"));
    }
    assert_normalized(&sv, "GHZ-3 state");
}

/// GHZ state for n=4: (|0000⟩ + |1111⟩)/√2.
#[test]
fn ghz_state_4() {
    let n = 4;
    let mut sv = StateVec::zero(n);
    gates::h(&mut sv, 0);
    for k in 1..n {
        gates::cnot(&mut sv, 0, k);
    }

    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    let dim = 1usize << n;
    for i in 0..dim {
        let expected = if i == 0 || i == dim - 1 { re(inv_sqrt2) } else { re(0.0) };
        assert_amp_eq(sv.amplitudes()[i], expected, &format!("GHZ4[{i}]"));
    }
    assert_normalized(&sv, "GHZ-4 state");
}

/// GHZ state for n=5: (|00000⟩ + |11111⟩)/√2.
#[test]
fn ghz_state_5() {
    let n = 5;
    let mut sv = StateVec::zero(n);
    gates::h(&mut sv, 0);
    for k in 1..n {
        gates::cnot(&mut sv, 0, k);
    }

    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    let dim = 1usize << n;
    for i in 0..dim {
        let expected = if i == 0 || i == dim - 1 { re(inv_sqrt2) } else { re(0.0) };
        assert_amp_eq(sv.amplitudes()[i], expected, &format!("GHZ5[{i}]"));
    }
    assert_normalized(&sv, "GHZ-5 state");
}

// ── normalization KATs ────────────────────────────────────────────────────────
//
// Verify Σ|aᵢ|² = 1 is preserved across arbitrary gate sequences.

/// Normalization preserved across a sequence of single-qubit gates.
#[test]
fn normalization_single_qubit_sequence() {
    let mut sv = StateVec::zero(3);
    gates::h(&mut sv, 0);
    gates::h(&mut sv, 1);
    gates::h(&mut sv, 2);
    assert_normalized(&sv, "after H⊗H⊗H");
    gates::x(&mut sv, 0);
    gates::y(&mut sv, 1);
    gates::z(&mut sv, 2);
    assert_normalized(&sv, "after X⊗Y⊗Z");
    gates::s(&mut sv, 0);
    gates::t(&mut sv, 1);
    gates::phase(&mut sv, 2, PI / 7.0);
    assert_normalized(&sv, "after S⊗T⊗phase");
}

/// Normalization preserved across a sequence of two-qubit gates.
#[test]
fn normalization_two_qubit_sequence() {
    let mut sv = StateVec::zero(3);
    gates::h(&mut sv, 0);
    gates::cnot(&mut sv, 0, 1);
    assert_normalized(&sv, "after H+CNOT");
    gates::cnot(&mut sv, 1, 2);
    assert_normalized(&sv, "after second CNOT");
    gates::swap(&mut sv, 0, 2);
    assert_normalized(&sv, "after SWAP");
    gates::controlled_phase(&mut sv, 0, 1, PI / 4.0);
    assert_normalized(&sv, "after controlled-phase");
}

/// Normalization preserved across a Toffoli gate sequence.
#[test]
fn normalization_toffoli_sequence() {
    let mut sv = StateVec::zero(3);
    gates::h(&mut sv, 0);
    gates::h(&mut sv, 1);
    gates::toffoli(&mut sv, 0, 1, 2);
    assert_normalized(&sv, "after H⊗H+Toffoli");
    gates::toffoli(&mut sv, 0, 1, 2);
    assert_normalized(&sv, "after second Toffoli");
}

// ── gate identity KATs ────────────────────────────────────────────────────────
//
// Published single-qubit relations:
//   HH = I        (H is Hermitian / self-inverse)
//   XX = I        (X is Hermitian / self-inverse)
//   S² = Z        (S² = diag(1, i²) = diag(1, -1) = Z)
//   T² = S        (T² = diag(1, e^{iπ/4·2}) = diag(1, e^{iπ/2}) = diag(1, i) = S)
//
// Reference: standard quantum gate algebra; see e.g. Nielsen & Chuang Appendix.

/// Gate identity HH = I: applying H twice returns the original state.
#[test]
fn identity_hh() {
    for k in 0..4usize {
        let original = StateVec::basis(2, k);
        let mut sv = StateVec::basis(2, k);
        gates::h(&mut sv, 0);
        gates::h(&mut sv, 0);
        assert_amps_eq(sv.amplitudes(), original.amplitudes(), &format!("HH=I on |{k}⟩"));
    }
}

/// Gate identity XX = I: applying X twice returns the original state.
#[test]
fn identity_xx() {
    for k in 0..4usize {
        let original = StateVec::basis(2, k);
        let mut sv = StateVec::basis(2, k);
        gates::x(&mut sv, 0);
        gates::x(&mut sv, 0);
        assert_amps_eq(sv.amplitudes(), original.amplitudes(), &format!("XX=I on |{k}⟩"));
    }
}

/// Gate identity S² = Z: applying S twice equals applying Z.
///
/// S = diag(1, i), so S² = diag(1, i²) = diag(1, -1) = Z.
#[test]
fn identity_s_squared_eq_z() {
    for k in 0..4usize {
        // Compute S²|k⟩
        let mut sv_s2 = StateVec::basis(2, k);
        gates::s(&mut sv_s2, 0);
        gates::s(&mut sv_s2, 0);

        // Compute Z|k⟩
        let mut sv_z = StateVec::basis(2, k);
        gates::z(&mut sv_z, 0);

        assert_amps_eq(sv_s2.amplitudes(), sv_z.amplitudes(), &format!("S²=Z on |{k}⟩"));
    }
}

/// Gate identity T² = S: applying T twice equals applying S.
///
/// T = diag(1, e^{iπ/4}), so T² = diag(1, e^{iπ/2}) = diag(1, i) = S.
#[test]
fn identity_t_squared_eq_s() {
    for k in 0..4usize {
        // Compute T²|k⟩
        let mut sv_t2 = StateVec::basis(2, k);
        gates::t(&mut sv_t2, 0);
        gates::t(&mut sv_t2, 0);

        // Compute S|k⟩
        let mut sv_s = StateVec::basis(2, k);
        gates::s(&mut sv_s, 0);

        assert_amps_eq(sv_t2.amplitudes(), sv_s.amplitudes(), &format!("T²=S on |{k}⟩"));
    }
}

// ── additional gate correctness KATs ─────────────────────────────────────────

/// X gate flips qubit 0: X|0⟩ = |1⟩, X|1⟩ = |0⟩.
#[test]
fn x_gate_flips_qubit() {
    // Single qubit: X|0⟩ = |1⟩
    let mut sv = StateVec::zero(1);
    gates::x(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(0.0), "X|0⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(1.0), "X|0⟩[1]");

    // Single qubit: X|1⟩ = |0⟩
    let mut sv = StateVec::basis(1, 1);
    gates::x(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(1.0), "X|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(0.0), "X|1⟩[1]");
}

/// H gate on |0⟩ gives (|0⟩ + |1⟩)/√2.
#[test]
fn h_gate_superposition() {
    let mut sv = StateVec::zero(1);
    gates::h(&mut sv, 0);
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    assert_amp_eq(sv.amplitudes()[0], re(inv_sqrt2), "H|0⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(inv_sqrt2), "H|0⟩[1]");
    assert_normalized(&sv, "H|0⟩");
}

/// H gate on |1⟩ gives (|0⟩ - |1⟩)/√2.
#[test]
fn h_gate_minus_state() {
    let mut sv = StateVec::basis(1, 1);
    gates::h(&mut sv, 0);
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    assert_amp_eq(sv.amplitudes()[0], re(inv_sqrt2), "H|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(-inv_sqrt2), "H|1⟩[1]");
    assert_normalized(&sv, "H|1⟩");
}

/// Z gate: Z|0⟩ = |0⟩, Z|1⟩ = -|1⟩.
#[test]
fn z_gate_phase_flip() {
    // Z|0⟩ = |0⟩ (no change)
    let mut sv = StateVec::zero(1);
    gates::z(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(1.0), "Z|0⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(0.0), "Z|0⟩[1]");

    // Z|1⟩ = -|1⟩
    let mut sv = StateVec::basis(1, 1);
    gates::z(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(0.0), "Z|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(-1.0), "Z|1⟩[1]");
}

/// Y gate: Y|0⟩ = i|1⟩, Y|1⟩ = -i|0⟩.
#[test]
fn y_gate_action() {
    // Y|0⟩ = i|1⟩
    let mut sv = StateVec::zero(1);
    gates::y(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(0.0), "Y|0⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], im(1.0), "Y|0⟩[1]");

    // Y|1⟩ = -i|0⟩
    let mut sv = StateVec::basis(1, 1);
    gates::y(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], im(-1.0), "Y|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], re(0.0), "Y|1⟩[1]");
}

/// S gate: S|0⟩ = |0⟩, S|1⟩ = i|1⟩.
#[test]
fn s_gate_action() {
    let mut sv = StateVec::basis(1, 1);
    gates::s(&mut sv, 0);
    assert_amp_eq(sv.amplitudes()[0], re(0.0), "S|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], im(1.0), "S|1⟩[1]");
}

/// T gate: T|1⟩ = e^{iπ/4}|1⟩.
#[test]
fn t_gate_action() {
    let mut sv = StateVec::basis(1, 1);
    gates::t(&mut sv, 0);
    let expected = Complex::new(0.0, PI / 4.0).exp();
    assert_amp_eq(sv.amplitudes()[0], re(0.0), "T|1⟩[0]");
    assert_amp_eq(sv.amplitudes()[1], expected, "T|1⟩[1]");
}

/// CNOT: control=0, target=1. CNOT|11⟩ = |10⟩ (flips target when control=1).
///
/// Little-endian: |11⟩ = index 3 (q0=1, q1=1), |10⟩ = index 1 (q0=1, q1=0).
#[test]
fn cnot_flips_target() {
    // CNOT on |11⟩ (index 3): control q0=1, target q1 gets flipped → |10⟩ (index 1)
    // Wait: with control=0 (q0) and target=1 (q1):
    //   |q1 q0⟩ = |11⟩ means q0=1, q1=1 → index = q0 + 2*q1 = 1 + 2 = 3
    //   After CNOT: q1 flips → q0=1, q1=0 → index = 1 + 0 = 1 → |01⟩ in ket notation
    let mut sv = StateVec::basis(2, 3); // |11⟩
    gates::cnot(&mut sv, 0, 1);
    // Result should be |01⟩ = index 1 (q0=1, q1=0)
    assert_amp_eq(sv.amplitudes()[1], re(1.0), "CNOT|11⟩ → |01⟩");
    assert_amp_eq(sv.amplitudes()[3], re(0.0), "CNOT|11⟩ → |01⟩ (index 3 = 0)");

    // CNOT on |01⟩ (index 1): control q0=1, target q1=0 → q1 flips → |11⟩ (index 3)
    let mut sv = StateVec::basis(2, 1); // |01⟩ (q0=1, q1=0)
    gates::cnot(&mut sv, 0, 1);
    assert_amp_eq(sv.amplitudes()[3], re(1.0), "CNOT|01⟩ → |11⟩");

    // CNOT on |00⟩ (index 0): control q0=0, no flip → |00⟩
    let mut sv = StateVec::zero(2);
    gates::cnot(&mut sv, 0, 1);
    assert_amp_eq(sv.amplitudes()[0], re(1.0), "CNOT|00⟩ → |00⟩");

    // CNOT on |10⟩ (index 2): control q0=0, no flip → |10⟩
    let mut sv = StateVec::basis(2, 2); // q0=0, q1=1
    gates::cnot(&mut sv, 0, 1);
    assert_amp_eq(sv.amplitudes()[2], re(1.0), "CNOT|10⟩ → |10⟩");
}

/// SWAP: SWAP|01⟩ = |10⟩ (little-endian: index 1 ↔ index 2).
#[test]
fn swap_exchanges_qubits() {
    // |01⟩ = index 1 (q0=1, q1=0) → SWAP → |10⟩ = index 2 (q0=0, q1=1)
    let mut sv = StateVec::basis(2, 1);
    gates::swap(&mut sv, 0, 1);
    assert_amp_eq(sv.amplitudes()[2], re(1.0), "SWAP|01⟩ → |10⟩");
    assert_amp_eq(sv.amplitudes()[1], re(0.0), "SWAP|01⟩ → |10⟩ (index 1 = 0)");
}

/// Toffoli: flips target when both controls are |1⟩.
///
/// Toffoli on |111⟩ (index 7, 3-qubit) with controls=0,1 and target=2:
/// q0=1, q1=1 → target q2 flips: q2=1→0 → |011⟩ = index 3.
#[test]
fn toffoli_flips_target() {
    // |111⟩ = index 7 (q0=1, q1=1, q2=1), controls=0,1, target=2
    // After Toffoli: q2 flips → q0=1, q1=1, q2=0 → index = 1+2+0 = 3 → |011⟩
    let mut sv = StateVec::basis(3, 7);
    gates::toffoli(&mut sv, 0, 1, 2);
    assert_amp_eq(sv.amplitudes()[3], re(1.0), "Toffoli|111⟩ → |011⟩");
    assert_amp_eq(sv.amplitudes()[7], re(0.0), "Toffoli|111⟩ → |011⟩ (index 7 = 0)");

    // |011⟩ = index 3 (q0=1, q1=1, q2=0): both controls set, target=2 flips → |111⟩
    let mut sv = StateVec::basis(3, 3);
    gates::toffoli(&mut sv, 0, 1, 2);
    assert_amp_eq(sv.amplitudes()[7], re(1.0), "Toffoli|011⟩ → |111⟩");

    // |101⟩ = index 5 (q0=1, q1=0, q2=1): only one control set → no flip
    let mut sv = StateVec::basis(3, 5);
    gates::toffoli(&mut sv, 0, 1, 2);
    assert_amp_eq(sv.amplitudes()[5], re(1.0), "Toffoli|101⟩ → |101⟩ (no flip)");
}

/// Multi-controlled-X with 3 controls: flips target when all 3 controls are |1⟩.
#[test]
fn multi_controlled_x_4qubit() {
    // 4-qubit register: controls = [0,1,2], target = 3.
    // |1111⟩ = index 15 (all bits set): all controls set → target flips → |0111⟩ = index 7.
    let mut sv = StateVec::basis(4, 15);
    gates::multi_controlled_x(&mut sv, &[0, 1, 2], 3);
    assert_amp_eq(sv.amplitudes()[7], re(1.0), "MCX|1111⟩ → |0111⟩");
    assert_amp_eq(sv.amplitudes()[15], re(0.0), "MCX|1111⟩ → |0111⟩ (index 15 = 0)");

    // |0111⟩ = index 7 (q0=1,q1=1,q2=1,q3=0): all controls set → target flips → |1111⟩
    let mut sv = StateVec::basis(4, 7);
    gates::multi_controlled_x(&mut sv, &[0, 1, 2], 3);
    assert_amp_eq(sv.amplitudes()[15], re(1.0), "MCX|0111⟩ → |1111⟩");

    // |1011⟩ = index 13 (q0=1,q1=1,q2=0,q3=1): control q2=0 → no flip
    let mut sv = StateVec::basis(4, 13);
    gates::multi_controlled_x(&mut sv, &[0, 1, 2], 3);
    assert_amp_eq(sv.amplitudes()[13], re(1.0), "MCX|1011⟩ → no flip");
}
