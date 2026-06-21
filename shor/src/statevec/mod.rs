//! Dense quantum state-vector register over n qubits.
//!
//! # Basis-indexing convention (FIXED — C-StateVec)
//!
//! **Little-endian:** qubit 0 is the least-significant bit of the basis index.
//! For an n-qubit register the basis state |q_{n-1} … q_1 q_0⟩ maps to index
//! `i = q_0 + 2·q_1 + 4·q_2 + … + 2^{n-1}·q_{n-1}`.
//!
//! This convention is fixed in this module and consumed by the QFT ([`crate::qft`]), modular
//! exponentiation ([`crate::arith`]), and Proos–Zalka ECDLP circuit ([`crate::ecdlp`]).
//! A silent flip is a wrong-answer bug.
//!
//! # Resource-scale ceiling (~25 qubits)
//!
//! The register holds `2^n` complex amplitudes. At n = 25 that is 2^25 ≈ 33 M entries
//! (≈ 512 MiB of f64 pairs). This is a **resource-scale wall** (principle 4): the mathematics
//! is identical at 25 or 250 qubits; only the exponential array makes 250 unreachable on a
//! laptop. The simulator demonstrates Shor's mathematics correctly at toy scale.
//!
//! # Normalization invariant
//!
//! Every `StateVec` satisfies `Σ|aᵢ|² = 1`. Constructors enforce this. In debug builds,
//! `debug_assert!` guards check the invariant after construction. Gates (in `crate::gates`)
//! preserve normalization by applying unitary transformations.

use num_complex::Complex;

/// Floating-point tolerance for normalization and amplitude comparisons.
pub const EPS: f64 = 1e-10;

/// Dense quantum state-vector register over `n` qubits.
///
/// Holds `2^n` complex amplitudes in a `Vec<Complex<f64>>`. The basis-indexing convention is
/// **little-endian**: qubit 0 is the least-significant bit (LSB) of the basis index.
///
/// # Resource-scale ceiling
///
/// The `2^n` amplitude array is the resource wall: n = 25 requires ≈ 512 MiB. This is a
/// principle-4 engineering boundary, not a mathematical one. The simulator is correct at any
/// qubit count; only memory makes large n unreachable on a laptop.
///
/// # Normalization invariant
///
/// `Σ|aᵢ|² = 1` is maintained across all constructors and gate applications.
#[derive(Clone, Debug)]
pub struct StateVec {
    /// Number of qubits.
    n: usize,
    /// Amplitudes in little-endian basis order: index `i` encodes |q_{n-1}…q_1 q_0⟩
    /// where `q_k = (i >> k) & 1`.
    amplitudes: Vec<Complex<f64>>,
}

impl StateVec {
    /// Construct the all-zero state |0…0⟩ over `n` qubits.
    ///
    /// The amplitude of basis state 0 is 1; all others are 0.
    ///
    /// # Panics
    ///
    /// Panics if `n == 0`.
    #[must_use]
    pub fn zero(n: usize) -> Self {
        assert!(n > 0, "StateVec requires at least 1 qubit");
        let dim = 1usize << n;
        let mut amplitudes = vec![Complex::new(0.0, 0.0); dim];
        amplitudes[0] = Complex::new(1.0, 0.0);
        let sv = Self { n, amplitudes };
        debug_assert!(sv.is_normalized(), "StateVec::zero: normalization violated");
        sv
    }

    /// Construct the basis state |k⟩ over `n` qubits (little-endian index `k`).
    ///
    /// The amplitude of basis state `k` is 1; all others are 0.
    ///
    /// # Panics
    ///
    /// Panics if `n == 0` or `k >= 2^n`.
    #[must_use]
    pub fn basis(n: usize, k: usize) -> Self {
        assert!(n > 0, "StateVec requires at least 1 qubit");
        let dim = 1usize << n;
        assert!(k < dim, "basis index {k} out of range for {n}-qubit register (dim={dim})");
        let mut amplitudes = vec![Complex::new(0.0, 0.0); dim];
        amplitudes[k] = Complex::new(1.0, 0.0);
        let sv = Self { n, amplitudes };
        debug_assert!(sv.is_normalized(), "StateVec::basis: normalization violated");
        sv
    }

    /// Construct a state from an arbitrary amplitude vector.
    ///
    /// The vector must have length `2^n` and satisfy `Σ|aᵢ|² = 1` within [`EPS`].
    ///
    /// # Panics
    ///
    /// Panics if `n == 0`, `amplitudes.len() != 2^n`, or the vector is not normalized.
    #[must_use]
    pub fn from_amplitudes(n: usize, amplitudes: Vec<Complex<f64>>) -> Self {
        assert!(n > 0, "StateVec requires at least 1 qubit");
        let dim = 1usize << n;
        assert!(
            amplitudes.len() == dim,
            "amplitude vector length {} != 2^{n} = {dim}",
            amplitudes.len()
        );
        let sv = Self { n, amplitudes };
        assert!(
            sv.is_normalized(),
            "StateVec::from_amplitudes: normalization violated (Σ|aᵢ|² ≠ 1)"
        );
        sv
    }

    /// Number of qubits in this register.
    #[must_use]
    pub fn n_qubits(&self) -> usize {
        self.n
    }

    /// Dimension of the Hilbert space: `2^n`.
    #[must_use]
    pub fn dim(&self) -> usize {
        self.amplitudes.len()
    }

    /// Read-only view of the amplitude vector.
    #[must_use]
    pub fn amplitudes(&self) -> &[Complex<f64>] {
        &self.amplitudes
    }

    /// Mutable view of the amplitude vector (for in-place gate application).
    ///
    /// Gates in `crate::gates` use this to apply unitary transformations in-place.
    pub fn amplitudes_mut(&mut self) -> &mut Vec<Complex<f64>> {
        &mut self.amplitudes
    }

    /// Check whether the normalization invariant holds within [`EPS`].
    ///
    /// Returns `true` iff `|Σ|aᵢ|² − 1| < EPS`.
    #[must_use]
    pub fn is_normalized(&self) -> bool {
        let norm_sq: f64 = self.amplitudes.iter().map(|a| a.norm_sqr()).sum();
        (norm_sq - 1.0).abs() < EPS
    }

    /// Compute `Σ|aᵢ|²` (the total probability, should be 1.0 for a normalized state).
    #[must_use]
    pub fn norm_sq(&self) -> f64 {
        self.amplitudes.iter().map(|a| a.norm_sqr()).sum()
    }
}
