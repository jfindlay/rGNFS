//! Sparse-state quantum register: a hashmap of nonzero basis amplitudes.
//!
//! # When to use sparse vs dense
//!
//! The sparse representation is efficient **only while the state is sparse** — i.e., while few
//! basis states have nonzero amplitude. A single Hadamard applied to every qubit of |0…0⟩
//! produces a uniform superposition with all `2^n` amplitudes nonzero, at which point the sparse
//! register degenerates to the same cost as the dense register (O(2^n) entries). This is a
//! **principle-4 annotation**: sparsity is a state-dependent property, not a universal speedup.
//!
//! # Gate semantics (C-Sparse)
//!
//! Gate application iterates only the nonzero entries in the map, applying the same unitary
//! transformations as the dense gate set. The sparse path is a no-regression mirror of the dense
//! path: a gate sequence on the sparse register yields the same amplitudes as on the dense
//! register (the sparse-dense agreement KAT).
//!
//! # Dense↔sparse conversion
//!
//! [`SparseStateVec::from_dense`] converts a [`StateVec`] to sparse form by collecting nonzero
//! amplitudes. [`SparseStateVec::to_dense`] reconstructs the dense register. Amplitudes with
//! absolute value below [`SPARSE_EPS`] are treated as zero and dropped.
//!
//! # Basis-indexing convention
//!
//! Same as the dense register (C-StateVec): **little-endian**, qubit 0 = LSB. The basis index
//! `i` encodes |q_{n-1}…q_1 q_0⟩ where `q_k = (i >> k) & 1`.

use std::collections::HashMap;
use std::f64::consts::PI;

use num_complex::Complex;

use crate::statevec::{StateVec, EPS};

/// Amplitude threshold below which a basis-state amplitude is treated as zero and dropped.
///
/// Set to [`EPS`] (the same tolerance used by the dense register's normalization check).
pub const SPARSE_EPS: f64 = EPS;

/// Sparse quantum state-vector register.
///
/// Stores only the nonzero amplitudes as a `HashMap<basis_index, Complex<f64>>`. Basis states
/// with amplitude below [`SPARSE_EPS`] are absent from the map.
///
/// # Invariant (C-Sparse)
///
/// A gate sequence on the sparse register yields the same amplitudes as on the dense register
/// (the sparse-dense agreement KAT). Sparsity helps only while the state is sparse; a fully-
/// superposed state (e.g., after H on every qubit) is dense and the sparse path matches the
/// dense cost.
#[derive(Clone, Debug)]
pub struct SparseStateVec {
    /// Number of qubits.
    n: usize,
    /// Nonzero amplitudes: maps basis index → complex amplitude.
    ///
    /// Invariant: every stored amplitude has `|a| >= SPARSE_EPS`.
    amplitudes: HashMap<usize, Complex<f64>>,
}

impl SparseStateVec {
    /// Construct the all-zero state |0…0⟩ over `n` qubits.
    ///
    /// # Panics
    ///
    /// Panics if `n == 0`.
    #[must_use]
    pub fn zero(n: usize) -> Self {
        assert!(n > 0, "SparseStateVec requires at least 1 qubit");
        let mut amplitudes = HashMap::new();
        amplitudes.insert(0, Complex::new(1.0, 0.0));
        Self { n, amplitudes }
    }

    /// Construct the basis state |k⟩ over `n` qubits (little-endian index `k`).
    ///
    /// # Panics
    ///
    /// Panics if `n == 0` or `k >= 2^n`.
    #[must_use]
    pub fn basis(n: usize, k: usize) -> Self {
        assert!(n > 0, "SparseStateVec requires at least 1 qubit");
        let dim = 1usize << n;
        assert!(k < dim, "basis index {k} out of range for {n}-qubit register (dim={dim})");
        let mut amplitudes = HashMap::new();
        amplitudes.insert(k, Complex::new(1.0, 0.0));
        Self { n, amplitudes }
    }

    /// Convert a dense [`StateVec`] to sparse form.
    ///
    /// Amplitudes with `|a| < SPARSE_EPS` are dropped. The resulting sparse register is
    /// equivalent to the dense register for all gate applications and measurements.
    #[must_use]
    pub fn from_dense(sv: &StateVec) -> Self {
        let n = sv.n_qubits();
        let amplitudes = sv
            .amplitudes()
            .iter()
            .enumerate()
            .filter(|(_, a)| a.norm() >= SPARSE_EPS)
            .map(|(i, &a)| (i, a))
            .collect();
        Self { n, amplitudes }
    }

    /// Convert this sparse register to a dense [`StateVec`].
    ///
    /// Basis states absent from the map are assigned amplitude 0.
    ///
    /// # Panics
    ///
    /// Panics if the resulting amplitude vector is not normalized (within [`EPS`]).
    #[must_use]
    pub fn to_dense(&self) -> StateVec {
        let dim = 1usize << self.n;
        let mut amps = vec![Complex::new(0.0, 0.0); dim];
        for (&idx, &a) in &self.amplitudes {
            amps[idx] = a;
        }
        StateVec::from_amplitudes(self.n, amps)
    }

    /// Number of qubits in this register.
    #[must_use]
    pub fn n_qubits(&self) -> usize {
        self.n
    }

    /// Number of nonzero basis states currently stored.
    #[must_use]
    pub fn nnz(&self) -> usize {
        self.amplitudes.len()
    }

    /// Read-only view of the nonzero amplitudes.
    #[must_use]
    pub fn amplitudes(&self) -> &HashMap<usize, Complex<f64>> {
        &self.amplitudes
    }

    /// Retrieve the amplitude of a specific basis state (0 if absent).
    #[must_use]
    pub fn amplitude(&self, idx: usize) -> Complex<f64> {
        self.amplitudes.get(&idx).copied().unwrap_or(Complex::new(0.0, 0.0))
    }

    /// Drop entries whose amplitude has fallen below [`SPARSE_EPS`].
    ///
    /// Called after gate application to maintain the sparsity invariant.
    fn prune(&mut self) {
        self.amplitudes.retain(|_, a| a.norm() >= SPARSE_EPS);
    }
}

// ── gate application on sparse registers ─────────────────────────────────────
//
// Each gate iterates only the nonzero entries. For a single-qubit gate on qubit `t`, each
// nonzero entry at index `i` is coupled with the entry at `i ^ (1 << t)` (its partner with
// bit `t` flipped). We collect the updates into a new map to avoid aliasing, then replace.

/// Apply the Pauli-X (NOT) gate to qubit `target` on a sparse register.
///
/// Swaps amplitudes between each basis state and its partner with bit `target` flipped.
pub fn x(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let mut new_amps: HashMap<usize, Complex<f64>> = HashMap::new();
    for (&idx, &a) in &sv.amplitudes {
        // Move amplitude from idx to idx ^ mask.
        let partner = idx ^ mask;
        *new_amps.entry(partner).or_insert(Complex::new(0.0, 0.0)) += a;
    }
    sv.amplitudes = new_amps;
    sv.prune();
}

/// Apply the Pauli-Y gate to qubit `target` on a sparse register.
///
/// Matrix: [[0, -i], [i, 0]].
pub fn y(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let mi = Complex::new(0.0, -1.0);
    let pi = Complex::new(0.0, 1.0);
    let mut new_amps: HashMap<usize, Complex<f64>> = HashMap::new();
    for (&idx, &a) in &sv.amplitudes {
        let partner = idx ^ mask;
        if idx & mask == 0 {
            // |0⟩ component: Y maps |0⟩ → i|1⟩ (matrix row 1: [i, 0])
            *new_amps.entry(partner).or_insert(Complex::new(0.0, 0.0)) += pi * a;
        } else {
            // |1⟩ component: Y maps |1⟩ → -i|0⟩ (matrix row 0: [0, -i])
            *new_amps.entry(partner).or_insert(Complex::new(0.0, 0.0)) += mi * a;
        }
    }
    sv.amplitudes = new_amps;
    sv.prune();
}

/// Apply the Pauli-Z gate to qubit `target` on a sparse register.
///
/// Negates amplitudes of basis states where qubit `target` is |1⟩.
pub fn z(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    for (&idx, a) in &mut sv.amplitudes {
        if idx & mask != 0 {
            *a = -*a;
        }
    }
}

/// Apply the Hadamard gate to qubit `target` on a sparse register.
///
/// Matrix: (1/√2) · [[1, 1], [1, -1]].
///
/// # Principle-4 note
///
/// Applying H to every qubit of |0…0⟩ produces a uniform superposition with all `2^n`
/// amplitudes nonzero. After this operation the sparse register is fully dense and the sparse
/// path matches the dense cost. Sparsity is a state-dependent property, not a universal speedup.
pub fn h(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let inv_sqrt2 = std::f64::consts::FRAC_1_SQRT_2;
    // Collect all indices that appear (both as-is and as partners).
    let indices: Vec<usize> = sv.amplitudes.keys().copied().collect();
    let mut new_amps: HashMap<usize, Complex<f64>> = HashMap::new();
    let mut processed = std::collections::HashSet::new();
    for idx in indices {
        if processed.contains(&idx) {
            continue;
        }
        let partner = idx ^ mask;
        processed.insert(idx);
        processed.insert(partner);
        let a0 = sv.amplitudes.get(&(idx & !mask)).copied().unwrap_or(Complex::new(0.0, 0.0));
        let a1 = sv.amplitudes.get(&(idx | mask)).copied().unwrap_or(Complex::new(0.0, 0.0));
        let i0 = idx & !mask;
        let i1 = idx | mask;
        let new0 = Complex::new(inv_sqrt2, 0.0) * (a0 + a1);
        let new1 = Complex::new(inv_sqrt2, 0.0) * (a0 - a1);
        if new0.norm() >= SPARSE_EPS {
            new_amps.insert(i0, new0);
        }
        if new1.norm() >= SPARSE_EPS {
            new_amps.insert(i1, new1);
        }
    }
    sv.amplitudes = new_amps;
}

/// Apply the S (phase) gate to qubit `target` on a sparse register.
///
/// Multiplies amplitudes of basis states where qubit `target` is |1⟩ by i.
pub fn s(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let i = Complex::new(0.0, 1.0);
    for (&idx, a) in &mut sv.amplitudes {
        if idx & mask != 0 {
            *a = i * *a;
        }
    }
}

/// Apply the T gate to qubit `target` on a sparse register.
///
/// Multiplies amplitudes of basis states where qubit `target` is |1⟩ by e^{iπ/4}.
pub fn t(sv: &mut SparseStateVec, target: usize) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let phase = Complex::new(0.0, PI / 4.0).exp();
    for (&idx, a) in &mut sv.amplitudes {
        if idx & mask != 0 {
            *a = phase * *a;
        }
    }
}

/// Apply a phase gate with angle `theta` (radians) to qubit `target` on a sparse register.
///
/// Multiplies amplitudes of basis states where qubit `target` is |1⟩ by e^{iθ}.
pub fn phase(sv: &mut SparseStateVec, target: usize, theta: f64) {
    let n = sv.n;
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    let mask = 1usize << target;
    let p = Complex::new(0.0, theta).exp();
    for (&idx, a) in &mut sv.amplitudes {
        if idx & mask != 0 {
            *a = p * *a;
        }
    }
}

/// Apply the CNOT gate (control `c`, target `t`) on a sparse register.
///
/// Flips the target qubit when the control qubit is |1⟩.
pub fn cnot(sv: &mut SparseStateVec, control: usize, target: usize) {
    let n = sv.n;
    assert!(control < n, "control qubit {control} out of range for {n}-qubit register");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    assert_ne!(control, target, "control and target qubits must differ");
    let ctrl_mask = 1usize << control;
    let tgt_mask = 1usize << target;
    let mut new_amps: HashMap<usize, Complex<f64>> = HashMap::new();
    for (&idx, &a) in &sv.amplitudes {
        let new_idx = if idx & ctrl_mask != 0 { idx ^ tgt_mask } else { idx };
        *new_amps.entry(new_idx).or_insert(Complex::new(0.0, 0.0)) += a;
    }
    sv.amplitudes = new_amps;
    sv.prune();
}

/// Apply a controlled-phase gate with angle `theta` on a sparse register.
///
/// Multiplies amplitudes of basis states where both `control` and `target` are |1⟩ by e^{iθ}.
pub fn controlled_phase(sv: &mut SparseStateVec, control: usize, target: usize, theta: f64) {
    let n = sv.n;
    assert!(control < n, "control qubit {control} out of range for {n}-qubit register");
    assert!(target < n, "target qubit {target} out of range for {n}-qubit register");
    assert_ne!(control, target, "control and target qubits must differ");
    let ctrl_mask = 1usize << control;
    let tgt_mask = 1usize << target;
    let both_mask = ctrl_mask | tgt_mask;
    let p = Complex::new(0.0, theta).exp();
    for (&idx, a) in &mut sv.amplitudes {
        if idx & both_mask == both_mask {
            *a = p * *a;
        }
    }
}
