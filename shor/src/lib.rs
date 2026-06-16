//! Classical state-vector quantum-circuit simulator: the substrate for Shor's algorithm.
//!
//! # Structure
//!
//! - [`statevec`] — dense `Vec<Complex<f64>>` register over n qubits, little-endian basis
//!   indexing (qubit 0 = LSB). Constructors, normalization invariant, ~25-qubit resource ceiling.
//! - [`gates`] — universal gate set applied by amplitude-pair iteration (O(2^n) per gate, never
//!   matrix materialization): single-qubit X, Y, Z, H, S, T, phase(θ), arbitrary unitary;
//!   two-qubit CNOT, controlled-phase, SWAP; multi-qubit Toffoli, multi-controlled.
//!
//! # Basis-indexing convention (FIXED — C-StateVec)
//!
//! **Little-endian:** qubit 0 is the least-significant bit of the basis index. For an n-qubit
//! register the basis state |q_{n-1} … q_1 q_0⟩ maps to index
//! `i = q_0 + 2·q_1 + 4·q_2 + … + 2^{n-1}·q_{n-1}`.
//!
//! This convention is fixed at S.A.1 and consumed by S.A.2 (QFT), S.B (modular exponentiation),
//! and S.C (Proos–Zalka ECDLP circuit). A silent flip is a wrong-answer bug.
//!
//! # Resource-scale ceiling (~25 qubits)
//!
//! The dense register holds `2^n` complex amplitudes. At n = 25 that is 2^25 ≈ 33 M entries
//! (≈ 512 MiB of f64 pairs). This is a **resource-scale wall** (principle 4): the mathematics
//! is identical at 25 or 250 qubits; only the exponential array makes 250 unreachable on a
//! laptop. The simulator demonstrates Shor's mathematics correctly at toy scale; it does not
//! claim quantum speedup, which requires real quantum hardware out of scope by construction.

pub mod gates;
pub mod statevec;
