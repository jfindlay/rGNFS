//! Classical state-vector quantum-circuit simulator: the substrate for Shor's algorithm.
//!
//! # Structure
//!
//! - [`statevec`] — dense `Vec<Complex<f64>>` register over n qubits, little-endian basis
//!   indexing (qubit 0 = LSB). Constructors, normalization invariant, ~25-qubit resource ceiling.
//! - [`gates`] — universal gate set applied by amplitude-pair iteration (O(2^n) per gate, never
//!   matrix materialization): single-qubit X, Y, Z, H, S, T, phase(θ), arbitrary unitary;
//!   two-qubit CNOT, controlled-phase, SWAP; multi-qubit Toffoli, multi-controlled.
//! - [`sparse`] — sparse-state register: a `HashMap` of nonzero basis amplitudes. Same gate
//!   semantics as the dense register, iterating only nonzero entries. Dense↔sparse conversion.
//!   Sparsity is state-dependent (principle 4): a Hadamard on every qubit makes the state dense.
//! - [`measure`] — Born-rule measurement: sample a basis state with probability |aᵢ|², collapse
//!   the register. Single-qubit and full-register measurement. Deterministic seeded sampler
//!   (`rand_chacha::ChaCha8Rng`) for reproducible KATs.
//! - [`qft`] — Quantum Fourier Transform over the dense register: Hadamard + controlled-phase
//!   ladder (O(n²) gates), built from the frozen S.A.1 gate set. Inverse QFT. Bit-reversal
//!   convention documented and included (see [`qft`] module for the load-bearing note).
//! - [`arith`] — Reversible modular-arithmetic quantum circuit builders (C-ModExp, frozen S.B.1):
//!   controlled-add-mod (`|x⟩ → |x + c mod N⟩`), controlled-mult-mod (`|x⟩ → |c·x mod N⟩`),
//!   and controlled modular-exponentiation (`|x⟩|y⟩ → |x⟩|y·aˣ mod N⟩`). Assembled from the
//!   frozen S.A gate set; no new gate added to the simulator. Every ancilla is returned to |0⟩
//!   (ancilla-clean invariant). Register layout: exponent qubits `[0, t)`, work qubits `[t, t+n)`,
//!   where `N < 2^n` and `t` is the exponent register size. See [`arith::ModExpLayout`] for the
//!   frozen C-ModExp interface consumed by S.B.2's order-finding circuit.
//! - [`shor`] — Order-finding circuit orchestration, continued-fraction period extraction, and
//!   `factor(N)` driver (C-OrderFind + C-Factor, frozen S.B.2 ◆). Consumes C-ModExp and the
//!   frozen S.A.2 QFT/measure surfaces. Implements the complete Shor-factoring algorithm:
//!   exponent-register superposition → controlled-mod-exp → iQFT → measure → continued-fraction
//!   recovery of the order `r` → even-order factor extraction via `gcd(a^(r/2) ± 1, N)`.
//!   Factors 15 → {3,5}, 21 → {3,7}, 35 → {5,7}, 91 → {7,13} (the canonical toy targets).
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
//! # QFT bit-reversal convention (FIXED — C-QFT)
//!
//! The standard QFT circuit (N&C) is designed for big-endian. [`qft::qft`] adapts it for
//! little-endian with **two bit-reversal steps** (input + output), so the output is in natural
//! little-endian order. Without both steps, S.B/S.C read the period from the wrong qubit order.
//! See [`qft`] module documentation for the full load-bearing note.
//!
//! # Resource-scale ceiling (~25 qubits)
//!
//! The dense register holds `2^n` complex amplitudes. At n = 25 that is 2^25 ≈ 33 M entries
//! (≈ 512 MiB of f64 pairs). This is a **resource-scale wall** (principle 4): the mathematics
//! is identical at 25 or 250 qubits; only the exponential array makes 250 unreachable on a
//! laptop. The simulator demonstrates Shor's mathematics correctly at toy scale; it does not
//! claim quantum speedup, which requires real quantum hardware out of scope by construction.

pub mod arith;
pub mod gates;
pub mod measure;
pub mod qft;
pub mod shor;
pub mod sparse;
pub mod statevec;
