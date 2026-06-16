//! Known-answer tests for the QFT, iQFT, measurement, and sparse-state modules.
//!
//! # What is tested
//!
//! 1. **QFT on |0…0⟩** — gives uniform superposition: amplitude 1/√N for all N = 2^n basis
//!    states. Verified for n = 1, 2, 3, 4.
//!
//! 2. **QFT on a basis state** — gives the published Fourier amplitudes:
//!    `QFT|j⟩ = (1/√N) Σ_{k=0}^{N-1} ω^{jk} |k⟩`, ω = e^{2πi/N}.
//!    Verified for n = 3 (N = 8), j = 1, 2, 3 against Python-computed reference values.
//!
//! 3. **QFT∘iQFT = identity** — applying QFT then iQFT recovers the original state within
//!    floating-point tolerance. Verified on several input states.
//!
//! 4. **Measurement distribution** — seeded measurement of a known superposition recovers
//!    Born-rule frequencies within tolerance over many samples. Deterministic-seed statistical KAT.
//!
//! 5. **Sparse-dense agreement** — a gate sequence on the sparse representation yields the same
//!    amplitudes as on the dense register (the C-Sparse no-regression mirror).
//!
//! # Reference values
//!
//! QFT amplitudes are computed from the formula `(1/√N) · e^{2πi·j·k/N}` for each (j, k) pair.
//! Python reference (numpy):
//!   ```python
//!   import numpy as np
//!   N = 8; j = 1
//!   [(1/np.sqrt(N)) * np.exp(2j * np.pi * j * k / N) for k in range(N)]
//!   ```
//! The bit-reversal convention is included in `qft()` (see `shor::qft` module documentation),
//! so the output index `k` directly corresponds to Fourier mode `k` in natural order.

use std::f64::consts::PI;

use num_complex::Complex;
use shor::gates;
use shor::measure::{sample_counts, seeded_rng, measure_all, measure_qubit};
use shor::qft::{iqft, qft};
use shor::sparse::{self, SparseStateVec};
use shor::statevec::{StateVec, EPS};

// ── helpers ───────────────────────────────────────────────────────────────────

/// Amplitude tolerance for QFT KATs (slightly looser than EPS to account for O(n²) gate
/// accumulation of floating-point error).
const QFT_EPS: f64 = 1e-9;

/// Assert that two complex amplitudes agree within `QFT_EPS`.
fn assert_amp_eq(got: Complex<f64>, expected: Complex<f64>, label: &str) {
    let diff = (got - expected).norm();
    assert!(
        diff < QFT_EPS,
        "{label}: amplitude mismatch\n  got      = {got:.10}\n  expected = {expected:.10}\n  |diff|   = {diff:.2e}"
    );
}

/// Assert that all amplitudes of `sv` agree with `expected` within `QFT_EPS`.
fn assert_state_eq(sv: &StateVec, expected: &[Complex<f64>], label: &str) {
    assert_eq!(
        sv.amplitudes().len(),
        expected.len(),
        "{label}: dimension mismatch"
    );
    for (k, (&got, &exp)) in sv.amplitudes().iter().zip(expected.iter()).enumerate() {
        assert_amp_eq(got, exp, &format!("{label} [k={k}]"));
    }
}

/// Compute the expected QFT amplitude for basis state |j⟩ at output index k, N = 2^n.
///
/// `QFT|j⟩[k] = (1/√N) · e^{2πi·j·k/N}`
fn qft_amp(j: usize, k: usize, n: usize) -> Complex<f64> {
    let big_n = 1usize << n;
    let theta = 2.0 * PI * (j * k) as f64 / big_n as f64;
    Complex::new(theta.cos(), theta.sin()) / (big_n as f64).sqrt()
}

// ── KAT group 1: QFT on |0…0⟩ gives uniform superposition ───────────────────

/// QFT on |0⟩ (1 qubit, N=2): both amplitudes = 1/√2.
///
/// QFT|0⟩ = (1/√2)(|0⟩ + |1⟩) — the Hadamard output.
#[test]
fn qft_zero_n1_uniform() {
    let mut sv = StateVec::zero(1);
    qft(&mut sv);
    let inv_sqrt2 = 1.0_f64 / 2.0_f64.sqrt();
    let expected = vec![
        Complex::new(inv_sqrt2, 0.0), // k=0: (1/√2)·e^0 = 1/√2
        Complex::new(inv_sqrt2, 0.0), // k=1: (1/√2)·e^0 = 1/√2 (j=0 → all phases = 1)
    ];
    assert_state_eq(&sv, &expected, "qft_zero_n1");
}

/// QFT on |00⟩ (2 qubits, N=4): all amplitudes = 1/2.
#[test]
fn qft_zero_n2_uniform() {
    let mut sv = StateVec::zero(2);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..4).map(|_| Complex::new(0.5, 0.0)).collect();
    assert_state_eq(&sv, &expected, "qft_zero_n2");
}

/// QFT on |000⟩ (3 qubits, N=8): all amplitudes = 1/√8.
#[test]
fn qft_zero_n3_uniform() {
    let mut sv = StateVec::zero(3);
    qft(&mut sv);
    let inv_sqrt8 = 1.0_f64 / 8.0_f64.sqrt();
    let expected: Vec<Complex<f64>> = (0..8).map(|_| Complex::new(inv_sqrt8, 0.0)).collect();
    assert_state_eq(&sv, &expected, "qft_zero_n3");
}

/// QFT on |0000⟩ (4 qubits, N=16): all amplitudes = 1/4.
#[test]
fn qft_zero_n4_uniform() {
    let mut sv = StateVec::zero(4);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..16).map(|_| Complex::new(0.25, 0.0)).collect();
    assert_state_eq(&sv, &expected, "qft_zero_n4");
}

// ── KAT group 2: QFT on basis state gives published Fourier amplitudes ────────
//
// Reference: QFT|j⟩[k] = (1/√N) · e^{2πi·j·k/N}
// Python: [(1/np.sqrt(8)) * np.exp(2j*np.pi*j*k/8) for k in range(8)]

/// QFT on |1⟩ (n=3, j=1): amplitude at k is (1/√8)·e^{2πi·k/8}.
///
/// Python reference values (j=1, N=8):
///   k=0: 0.3536+0j, k=1: 0.2500+0.2500j, k=2: 0+0.3536j, k=3: -0.2500+0.2500j,
///   k=4: -0.3536+0j, k=5: -0.2500-0.2500j, k=6: 0-0.3536j, k=7: 0.2500-0.2500j
#[test]
fn qft_basis_n3_j1() {
    let mut sv = StateVec::basis(3, 1);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..8).map(|k| qft_amp(1, k, 3)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n3_j1");
}

/// QFT on |2⟩ (n=3, j=2): amplitude at k is (1/√8)·e^{2πi·2k/8} = (1/√8)·e^{πi·k/2}.
///
/// Python reference values (j=2, N=8):
///   k=0: 0.3536+0j, k=1: 0+0.3536j, k=2: -0.3536+0j, k=3: 0-0.3536j,
///   k=4: 0.3536+0j, k=5: 0+0.3536j, k=6: -0.3536+0j, k=7: 0-0.3536j
#[test]
fn qft_basis_n3_j2() {
    let mut sv = StateVec::basis(3, 2);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..8).map(|k| qft_amp(2, k, 3)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n3_j2");
}

/// QFT on |3⟩ (n=3, j=3): amplitude at k is (1/√8)·e^{2πi·3k/8}.
#[test]
fn qft_basis_n3_j3() {
    let mut sv = StateVec::basis(3, 3);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..8).map(|k| qft_amp(3, k, 3)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n3_j3");
}

/// QFT on |5⟩ (n=3, j=5): amplitude at k is (1/√8)·e^{2πi·5k/8}.
#[test]
fn qft_basis_n3_j5() {
    let mut sv = StateVec::basis(3, 5);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..8).map(|k| qft_amp(5, k, 3)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n3_j5");
}

/// QFT on |7⟩ (n=3, j=7, the maximum basis index for n=3).
#[test]
fn qft_basis_n3_j7() {
    let mut sv = StateVec::basis(3, 7);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..8).map(|k| qft_amp(7, k, 3)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n3_j7");
}

/// QFT on |6⟩ (n=4, j=6, N=16): amplitude at k is (1/4)·e^{2πi·6k/16}.
#[test]
fn qft_basis_n4_j6() {
    let mut sv = StateVec::basis(4, 6);
    qft(&mut sv);
    let expected: Vec<Complex<f64>> = (0..16).map(|k| qft_amp(6, k, 4)).collect();
    assert_state_eq(&sv, &expected, "qft_basis_n4_j6");
}

// ── KAT group 3: QFT∘iQFT = identity ─────────────────────────────────────────

/// QFT then iQFT on |0…0⟩ recovers |0…0⟩.
#[test]
fn qft_iqft_identity_zero_n3() {
    let original = StateVec::zero(3);
    let mut sv = original.clone();
    qft(&mut sv);
    iqft(&mut sv);
    assert_state_eq(&sv, original.amplitudes(), "qft_iqft_identity_zero_n3");
}

/// QFT then iQFT on |5⟩ (n=3) recovers |5⟩.
#[test]
fn qft_iqft_identity_basis_n3_j5() {
    let original = StateVec::basis(3, 5);
    let mut sv = original.clone();
    qft(&mut sv);
    iqft(&mut sv);
    assert_state_eq(&sv, original.amplitudes(), "qft_iqft_identity_basis_n3_j5");
}

/// QFT then iQFT on |3⟩ (n=4) recovers |3⟩.
#[test]
fn qft_iqft_identity_basis_n4_j3() {
    let original = StateVec::basis(4, 3);
    let mut sv = original.clone();
    qft(&mut sv);
    iqft(&mut sv);
    assert_state_eq(&sv, original.amplitudes(), "qft_iqft_identity_basis_n4_j3");
}

/// QFT then iQFT on a Bell state (|00⟩ + |11⟩)/√2 recovers the Bell state.
#[test]
fn qft_iqft_identity_bell_state() {
    // Construct (|00⟩ + |11⟩)/√2 via H on qubit 0 then CNOT(0→1).
    let mut sv = StateVec::zero(2);
    gates::h(&mut sv, 0);
    gates::cnot(&mut sv, 0, 1);
    let original = sv.clone();
    qft(&mut sv);
    iqft(&mut sv);
    assert_state_eq(&sv, original.amplitudes(), "qft_iqft_identity_bell");
}

/// iQFT then QFT on |2⟩ (n=3) recovers |2⟩ (commutativity of inverse).
#[test]
fn iqft_qft_identity_basis_n3_j2() {
    let original = StateVec::basis(3, 2);
    let mut sv = original.clone();
    iqft(&mut sv);
    qft(&mut sv);
    assert_state_eq(&sv, original.amplitudes(), "iqft_qft_identity_basis_n3_j2");
}

/// QFT preserves normalization: Σ|aᵢ|² = 1 after QFT on any normalized input.
#[test]
fn qft_preserves_normalization() {
    for j in 0..8usize {
        let mut sv = StateVec::basis(3, j);
        qft(&mut sv);
        assert!(
            sv.is_normalized(),
            "QFT on |{j}⟩ (n=3) violated normalization: Σ|aᵢ|² = {:.10}",
            sv.norm_sq()
        );
    }
}

// ── KAT group 4: measurement distribution ────────────────────────────────────
//
// Seeded measurement of a known superposition recovers Born-rule frequencies within tolerance
// over many samples. The sampler is deterministic: same seed + same state → same counts.

/// Measurement of |+⟩ = (|0⟩ + |1⟩)/√2 recovers ~50/50 distribution over many shots.
///
/// Born-rule: P(0) = P(1) = 0.5. With 10000 shots and seed 42, the empirical frequencies
/// should be within 3% of 0.5 (a 3-sigma tolerance for a binomial with p=0.5, n=10000).
#[test]
fn measure_plus_state_distribution() {
    let mut sv = StateVec::zero(1);
    gates::h(&mut sv, 0);
    let n_shots = 10_000;
    let counts = sample_counts(&sv, n_shots, 42);
    let p0 = counts[0] as f64 / n_shots as f64;
    let p1 = counts[1] as f64 / n_shots as f64;
    assert!(
        (p0 - 0.5).abs() < 0.03,
        "measure |+⟩: P(0) = {p0:.4} deviates from 0.5 by more than 3%"
    );
    assert!(
        (p1 - 0.5).abs() < 0.03,
        "measure |+⟩: P(1) = {p1:.4} deviates from 0.5 by more than 3%"
    );
}

/// Measurement of a 4-state uniform superposition recovers ~25% per state.
///
/// State: (|0⟩ + |1⟩ + |2⟩ + |3⟩)/2 (2-qubit uniform superposition via H⊗H).
/// Born-rule: P(k) = 0.25 for k ∈ {0,1,2,3}. Tolerance: 3% (3-sigma for n=10000, p=0.25).
#[test]
fn measure_uniform_n2_distribution() {
    let mut sv = StateVec::zero(2);
    gates::h(&mut sv, 0);
    gates::h(&mut sv, 1);
    let n_shots = 10_000;
    let counts = sample_counts(&sv, n_shots, 123);
    for k in 0..4 {
        let p = counts[k] as f64 / n_shots as f64;
        assert!(
            (p - 0.25).abs() < 0.03,
            "measure uniform n=2: P({k}) = {p:.4} deviates from 0.25 by more than 3%"
        );
    }
}

/// Measurement of |0⟩ always returns basis index 0 (deterministic state).
#[test]
fn measure_zero_state_always_zero() {
    let sv = StateVec::zero(3);
    let counts = sample_counts(&sv, 100, 0);
    assert_eq!(counts[0], 100, "measure |000⟩: should always return 0");
    for k in 1..8 {
        assert_eq!(counts[k], 0, "measure |000⟩: basis {k} should have count 0");
    }
}

/// Measurement of |5⟩ always returns basis index 5 (deterministic basis state).
#[test]
fn measure_basis_state_deterministic() {
    let sv = StateVec::basis(3, 5);
    let counts = sample_counts(&sv, 100, 7);
    assert_eq!(counts[5], 100, "measure |5⟩: should always return 5");
    for k in 0..8 {
        if k != 5 {
            assert_eq!(counts[k], 0, "measure |5⟩: basis {k} should have count 0");
        }
    }
}

/// Seeded measurement is reproducible: same seed + same state → same outcome.
#[test]
fn measure_seeded_reproducible() {
    let mut sv = StateVec::zero(3);
    gates::h(&mut sv, 0);
    gates::h(&mut sv, 1);
    gates::h(&mut sv, 2);
    let counts1 = sample_counts(&sv, 1000, 999);
    let counts2 = sample_counts(&sv, 1000, 999);
    assert_eq!(counts1, counts2, "seeded measurement: different results for same seed");
}

/// Measurement of a weighted superposition recovers Born-rule probabilities.
///
/// State: (√(3/4)|0⟩ + √(1/4)|1⟩) on 1 qubit. P(0) = 3/4, P(1) = 1/4.
/// Tolerance: 3% (3-sigma for n=10000).
#[test]
fn measure_weighted_superposition() {
    let amps = vec![
        Complex::new((3.0_f64 / 4.0).sqrt(), 0.0), // |0⟩: amplitude √(3/4)
        Complex::new((1.0_f64 / 4.0).sqrt(), 0.0), // |1⟩: amplitude √(1/4)
    ];
    let sv = StateVec::from_amplitudes(1, amps);
    let n_shots = 10_000;
    let counts = sample_counts(&sv, n_shots, 31415);
    let p0 = counts[0] as f64 / n_shots as f64;
    let p1 = counts[1] as f64 / n_shots as f64;
    assert!(
        (p0 - 0.75).abs() < 0.03,
        "weighted superposition: P(0) = {p0:.4}, expected ~0.75"
    );
    assert!(
        (p1 - 0.25).abs() < 0.03,
        "weighted superposition: P(1) = {p1:.4}, expected ~0.25"
    );
}

/// Full-register measurement collapses to the measured basis state.
#[test]
fn measure_all_collapses_correctly() {
    let mut sv = StateVec::zero(3);
    gates::h(&mut sv, 0);
    gates::h(&mut sv, 1);
    gates::h(&mut sv, 2);
    let mut rng = seeded_rng(42);
    let outcome = measure_all(sv, &mut rng);
    // The collapsed state should be a pure basis state.
    let k = outcome.basis_index;
    let collapsed = outcome.collapsed;
    for (i, &a) in collapsed.amplitudes().iter().enumerate() {
        if i == k {
            assert!(
                (a.norm() - 1.0).abs() < EPS,
                "collapsed state: amplitude at measured index {k} should be 1, got {a}"
            );
        } else {
            assert!(
                a.norm() < EPS,
                "collapsed state: amplitude at index {i} should be 0, got {a}"
            );
        }
    }
}

/// Single-qubit measurement collapses only the measured qubit.
#[test]
fn measure_qubit_collapses_correctly() {
    // Bell state: (|00⟩ + |11⟩)/√2. Measuring qubit 0:
    // - If bit=0: collapse to |00⟩ (amplitude 1 at index 0).
    // - If bit=1: collapse to |11⟩ (amplitude 1 at index 3).
    let mut sv = StateVec::zero(2);
    gates::h(&mut sv, 0);
    gates::cnot(&mut sv, 0, 1);
    let mut rng = seeded_rng(77);
    let outcome = measure_qubit(sv, 0, &mut rng);
    let bit = outcome.bit;
    let collapsed = outcome.collapsed;
    assert!(bit == 0 || bit == 1, "measured bit must be 0 or 1");
    // After measuring qubit 0 = bit, the state should be |bit, bit⟩.
    let expected_idx = if bit == 0 { 0 } else { 3 }; // |00⟩ = index 0, |11⟩ = index 3
    for (i, &a) in collapsed.amplitudes().iter().enumerate() {
        if i == expected_idx {
            assert!(
                (a.norm() - 1.0).abs() < EPS,
                "collapsed Bell: amplitude at {i} should be 1, got {a} (bit={bit})"
            );
        } else {
            assert!(
                a.norm() < EPS,
                "collapsed Bell: amplitude at {i} should be 0, got {a} (bit={bit})"
            );
        }
    }
}

// ── KAT group 5: sparse-dense agreement ──────────────────────────────────────
//
// A gate sequence on the sparse representation yields the same amplitudes as on the dense
// register. This is the C-Sparse no-regression mirror KAT.

/// Helper: assert that a sparse register agrees with a dense register within QFT_EPS.
fn assert_sparse_dense_eq(sparse: &SparseStateVec, dense: &StateVec, label: &str) {
    let n = dense.n_qubits();
    let dim = 1usize << n;
    for k in 0..dim {
        let sparse_amp = sparse.amplitude(k);
        let dense_amp = dense.amplitudes()[k];
        assert_amp_eq(sparse_amp, dense_amp, &format!("{label} [k={k}]"));
    }
}

/// Sparse-dense agreement: X gate on |0⟩.
///
/// Both dense and sparse should give |1⟩ after X on qubit 0.
#[test]
fn sparse_dense_x_gate() {
    let mut dense = StateVec::zero(2);
    gates::x(&mut dense, 0);

    let mut sp = SparseStateVec::zero(2);
    sparse::x(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_x_gate");
}

/// Sparse-dense agreement: H gate on |0⟩ (creates superposition).
///
/// After H on qubit 0 of |00⟩, both dense and sparse should give (|00⟩ + |10⟩)/√2.
/// (Little-endian: qubit 0 is LSB, so |0⟩ on qubit 0 = index 0, |1⟩ on qubit 0 = index 1.)
#[test]
fn sparse_dense_h_gate() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_h_gate");
}

/// Sparse-dense agreement: CNOT after H (Bell state preparation).
///
/// H on qubit 0, then CNOT(0→1) on |00⟩ gives (|00⟩ + |11⟩)/√2.
#[test]
fn sparse_dense_bell_state() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);
    gates::cnot(&mut dense, 0, 1);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);
    sparse::cnot(&mut sp, 0, 1);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_bell_state");
}

/// Sparse-dense agreement: controlled-phase gate.
///
/// Apply controlled-phase(π/4) on a Bell state. Both paths should agree.
#[test]
fn sparse_dense_controlled_phase() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);
    gates::cnot(&mut dense, 0, 1);
    gates::controlled_phase(&mut dense, 0, 1, PI / 4.0);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);
    sparse::cnot(&mut sp, 0, 1);
    sparse::controlled_phase(&mut sp, 0, 1, PI / 4.0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_controlled_phase");
}

/// Sparse-dense agreement: Z gate on a superposition.
#[test]
fn sparse_dense_z_gate() {
    let mut dense = StateVec::zero(3);
    gates::h(&mut dense, 0);
    gates::h(&mut dense, 1);
    gates::z(&mut dense, 0);

    let mut sp = SparseStateVec::zero(3);
    sparse::h(&mut sp, 0);
    sparse::h(&mut sp, 1);
    sparse::z(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_z_gate");
}

/// Sparse-dense agreement: S gate on a superposition.
#[test]
fn sparse_dense_s_gate() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);
    gates::s(&mut dense, 0);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);
    sparse::s(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_s_gate");
}

/// Sparse-dense agreement: T gate on a superposition.
#[test]
fn sparse_dense_t_gate() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);
    gates::t(&mut dense, 0);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);
    sparse::t(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_t_gate");
}

/// Sparse-dense agreement: Y gate on a basis state.
#[test]
fn sparse_dense_y_gate() {
    let mut dense = StateVec::basis(2, 0);
    gates::y(&mut dense, 0);

    let mut sp = SparseStateVec::basis(2, 0);
    sparse::y(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_y_gate");
}

/// Sparse-dense agreement: phase gate on a superposition.
#[test]
fn sparse_dense_phase_gate() {
    let mut dense = StateVec::zero(2);
    gates::h(&mut dense, 0);
    gates::phase(&mut dense, 0, PI / 3.0);

    let mut sp = SparseStateVec::zero(2);
    sparse::h(&mut sp, 0);
    sparse::phase(&mut sp, 0, PI / 3.0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_phase_gate");
}

/// Sparse-dense agreement: dense↔sparse round-trip conversion.
///
/// Converting a dense state to sparse and back should recover the original amplitudes.
#[test]
fn sparse_dense_round_trip() {
    let mut dense = StateVec::zero(3);
    gates::h(&mut dense, 0);
    gates::h(&mut dense, 1);
    gates::cnot(&mut dense, 0, 2);

    let sp = SparseStateVec::from_dense(&dense);
    let recovered = sp.to_dense();

    assert_state_eq(&recovered, dense.amplitudes(), "sparse_dense_round_trip");
}

/// Sparse-dense agreement: a longer gate sequence on 3 qubits.
///
/// Sequence: H(0), H(1), CNOT(0→2), controlled_phase(1,2, π/2), Z(0).
/// Both dense and sparse paths should agree on all amplitudes.
#[test]
fn sparse_dense_longer_sequence_n3() {
    let mut dense = StateVec::zero(3);
    gates::h(&mut dense, 0);
    gates::h(&mut dense, 1);
    gates::cnot(&mut dense, 0, 2);
    gates::controlled_phase(&mut dense, 1, 2, PI / 2.0);
    gates::z(&mut dense, 0);

    let mut sp = SparseStateVec::zero(3);
    sparse::h(&mut sp, 0);
    sparse::h(&mut sp, 1);
    sparse::cnot(&mut sp, 0, 2);
    sparse::controlled_phase(&mut sp, 1, 2, PI / 2.0);
    sparse::z(&mut sp, 0);

    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_longer_sequence_n3");
}

/// Principle-4 annotation: after H on every qubit, the sparse register is fully dense.
///
/// This test verifies that the sparse register correctly handles the fully-superposed case
/// (all 2^n amplitudes nonzero) and still agrees with the dense register. It also documents
/// that sparsity is a state-dependent property: the sparse path is not a universal speedup.
#[test]
fn sparse_dense_fully_superposed_principle4() {
    // Apply H to every qubit of |000⟩ → uniform superposition (all 8 amplitudes nonzero).
    let mut dense = StateVec::zero(3);
    for q in 0..3 {
        gates::h(&mut dense, q);
    }

    let mut sp = SparseStateVec::zero(3);
    for q in 0..3 {
        sparse::h(&mut sp, q);
    }

    // After H on every qubit, the sparse register has 2^3 = 8 nonzero entries — fully dense.
    // Principle 4: sparsity helps only while the state is sparse. A Hadamard on every qubit
    // makes the state dense; the sparse path then matches the dense cost.
    assert_eq!(
        sp.nnz(),
        8,
        "after H on all 3 qubits, sparse register should have 8 nonzero entries (fully dense)"
    );
    assert_sparse_dense_eq(&sp, &dense, "sparse_dense_fully_superposed");
}
