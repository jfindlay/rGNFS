//! Reversible controlled point-addition quantum circuit.
//!
//! Implements the controlled `|P⟩ → |P + cG⟩` operation for a classically-fixed point
//! `cG`, controlled on a qubit. This is the quantum heart of the ECDLP period-finding
//! circuit in [`crate::ecdlp`].
//!
//! # Construction choice: constant-point addition via permutation synthesis
//!
//! Since `cG` is a **classical constant** (precomputed), this circuit performs
//! *constant-point addition* — adding a fixed classical point to a quantum register
//! holding the running point `P`. This is the standard Proos–Zalka simplification:
//! the added point `cG` is never in superposition, so the circuit reduces to a
//! classical permutation on the `(x, y)` coordinate register.
//!
//! The permutation is synthesized directly using the same ancilla-free approach as
//! the `arith` module (`apply_controlled_permutation`): the map
//! `(x_P, y_P) → (x_{P+cG}, y_{P+cG})` is computed classically for all 13 group
//! elements (including `∞`), then synthesized as a product of transpositions on the
//! combined `(x, y)` register. No λ scratch register is consumed at runtime — the
//! permutation synthesis is ancilla-free.
//!
//! This approach:
//! - Is correct: the permutation exactly implements the group law.
//! - Is reversible: every permutation is its own inverse composed with itself.
//! - Is ancilla-free: no scratch register is borrowed or left entangled.
//! - Uses only the gate set in [`crate::gates`] (via `arith::apply_controlled_permutation`).
//!   - Mirrors the `arith` permutation-synthesis precedent exactly.
//!
//! # Register layout (C-PointAdd freeze)
//!
//! See [`crate::curve::PointAddLayout`] for the full layout documentation.
//!
//! ```text
//! qubit  0                — control qubit
//! qubits [1, 4)          — x-coordinate (3 bits, little-endian LSB)
//! qubits [4, 7)          — y-coordinate (3 bits, little-endian LSB)
//! qubits [7, 10)         — λ scratch (3 bits, allocated but unused in this impl)
//! Total: 10 qubits
//! ```
//!
//! # Identity encoding
//!
//! `∞` is encoded as `(x=0, y=0)` in the coordinate registers. The circuit maps
//! `(0, 0) → (0, 0)` (identity is a fixed point: `∞ + cG` is handled by returning
//! `cG`, but since `∞` is the additive identity, `∞ + cG = cG`). Wait — actually
//! `∞ + cG = cG`, so the circuit maps `(0,0) → encode(cG)`. This is correct: the
//! circuit applies the group law faithfully, including the `∞` case.
//!
//! # Exceptional cases
//!
//! - `P = ∞`: `∞ + cG = cG`. The circuit maps `(0,0) → encode(cG)`.
//! - `P = cG`: `cG + cG = 2·cG`. Handled by the group law (doubling).
//! - `P = −cG`: `−cG + cG = ∞`. The circuit maps `encode(−cG) → (0,0)`.
//! - All other cases: standard affine addition formula.
//!
//! All exceptional cases are handled automatically by the permutation synthesis —
//! the classical group law is computed for every input, so no special-case branching
//! is needed in the circuit.
//!
//! # References
//!
//! - Proos, J., Zalka, C. (2003). "Shor's discrete logarithm quantum algorithm for
//!   elliptic curves." QIC 3(4).
//! - Roetteler, M., Naehrig, M., Svore, K.M., Lauter, K. (2017). "Quantum resource
//!   estimates for computing elliptic curve discrete logarithms." ASIACRYPT 2017.

use crate::arith::apply_controlled_permutation;
use crate::curve::{self, Point, PointAddLayout};
use crate::statevec::StateVec;

// ── permutation builder ───────────────────────────────────────────────────────

/// Compute the combined (x, y) register index for a point.
///
/// The combined register encodes `x` in the low `coord_bits` bits and `y` in the
/// next `coord_bits` bits: `index = x | (y << coord_bits)`.
///
/// `∞` is encoded as index 0 (since `encode_point(∞) = (0, 0)`).
fn point_to_xy_index(p: Point, coord_bits: usize) -> usize {
    let (x, y) = curve::encode_point(p);
    (x as usize) | ((y as usize) << coord_bits)
}

/// Build the permutation for `(x, y) → (x', y')` under `P → P + cg` on the combined
/// `(x, y)` register.
///
/// The combined register has `2 * coord_bits` bits, encoding `x` in the low half and
/// `y` in the high half. The permutation maps each index encoding a valid group element
/// `P` to the index encoding `P + cg`. Indices that do not correspond to any group
/// element are left as fixed points (identity).
///
/// Returns a `Vec<usize>` of length `2^(2 * coord_bits)` where `perm[i]` is the image
/// of index `i`.
fn build_point_add_permutation(cg: Point, coord_bits: usize) -> Vec<usize> {
    let dim = 1usize << (2 * coord_bits);
    // Start with identity permutation.
    let mut perm: Vec<usize> = (0..dim).collect();

    // Enumerate all group elements (∞ + all affine points).
    let mut group_elements = vec![Point::Infinity];
    group_elements.extend(curve::all_affine_points());

    for p in group_elements {
        let src = point_to_xy_index(p, coord_bits);
        let dst = point_to_xy_index(curve::add(p, cg), coord_bits);
        perm[src] = dst;
    }

    perm
}

// ── public circuit builder ────────────────────────────────────────────────────

/// Apply the controlled point-addition circuit: `|P⟩ → |P + cG⟩` when `ctrl` is |1⟩.
///
/// Adds the classically-fixed point `cg` to the quantum register holding point `P`,
/// controlled on the layout's control qubit. When the control qubit is |0⟩, the
/// register is unchanged (identity).
///
/// # Circuit structure
///
/// The operation is a controlled permutation on the combined `(x, y)` register:
/// each basis state `|x_P, y_P⟩` encoding a group element `P` is mapped to
/// `|x_{P+cG}, y_{P+cG}⟩`. The permutation is synthesized ancilla-free using the
/// same transposition-based approach as the `arith` module.
///
/// # Arguments
///
/// - `sv` — state vector (modified in-place); must have at least `layout.total_qubits()` qubits
/// - `cg` — the classical point to add (precomputed; must not be `∞`)
/// - `layout` — register layout descriptor (C-PointAdd freeze)
///
/// # Panics
///
/// Panics if `cg` is `∞` (adding the identity is a no-op; use the identity circuit),
/// if `cg` is not on the curve, or if any qubit index is out of range.
pub fn controlled_point_add(sv: &mut StateVec, cg: Point, layout: &PointAddLayout) {
    assert!(
        !cg.is_infinity(),
        "controlled_point_add: cg must not be ∞ (adding identity is a no-op)"
    );
    assert!(curve::on_curve(cg), "controlled_point_add: cg is not on the curve");

    let coord_bits = layout.x_len;
    assert_eq!(
        layout.y_len, coord_bits,
        "controlled_point_add: x_len and y_len must be equal"
    );

    // Build the permutation on the combined (x, y) register.
    let perm = build_point_add_permutation(cg, coord_bits);

    // The combined (x, y) register: x qubits first (LSB), then y qubits.
    // Little-endian: x_qubits[0] is the LSB of x, y_qubits[0] is the LSB of y.
    let mut xy_qubits: Vec<usize> = layout.x_qubits();
    xy_qubits.extend(layout.y_qubits());

    // Apply the controlled permutation on the combined (x, y) register.
    apply_controlled_permutation(sv, layout.ctrl_qubit, &xy_qubits, &perm);
}

/// Apply the inverse controlled point-addition circuit: `|P⟩ → |P − cG⟩` when `ctrl` is |1⟩.
///
/// This is the inverse of [`controlled_point_add`]: adds `−cG` instead of `cG`.
/// Used for reversibility verification and for the inverse circuit in [`crate::ecdlp`].
///
/// # Panics
///
/// Panics if `cg` is `∞` or not on the curve.
pub fn controlled_point_add_inv(sv: &mut StateVec, cg: Point, layout: &PointAddLayout) {
    assert!(
        !cg.is_infinity(),
        "controlled_point_add_inv: cg must not be ∞"
    );
    assert!(curve::on_curve(cg), "controlled_point_add_inv: cg is not on the curve");

    // The inverse of adding cG is adding −cG.
    let neg_cg = curve::negate(cg);
    controlled_point_add(sv, neg_cg, layout);
}

// ── register read/write helpers ───────────────────────────────────────────────

/// Read the point encoded in the `(x, y)` registers of a basis state.
///
/// Extracts the x and y coordinate values from the layout's x and y registers,
/// then decodes them to a `Point` (using `(0, 0)` → `∞`).
///
/// # Panics
///
/// Panics if `sv` is not a basis state.
#[must_use]
pub fn read_point(sv: &StateVec, layout: &PointAddLayout) -> Point {
    let basis_idx = crate::arith::find_basis_index(sv);
    let x_val = read_register(basis_idx, &layout.x_qubits());
    let y_val = read_register(basis_idx, &layout.y_qubits());
    curve::decode_point(x_val, y_val)
}

/// Read the value of a register from a basis index.
///
/// Extracts the integer value `Σ bit_k · 2^k` (little-endian) from the given qubit indices.
fn read_register(basis_idx: usize, qubits: &[usize]) -> u64 {
    let mut val = 0u64;
    for (k, &q) in qubits.iter().enumerate() {
        if (basis_idx >> q) & 1 == 1 {
            val |= 1 << k;
        }
    }
    val
}

/// Read the value of the λ scratch register from a basis state.
///
/// Returns the integer value encoded in the λ register. Should be 0 after the circuit
/// (ancilla-clean invariant).
///
/// # Panics
///
/// Panics if `sv` is not a basis state.
#[must_use]
pub fn read_lambda(sv: &StateVec, layout: &PointAddLayout) -> u64 {
    let basis_idx = crate::arith::find_basis_index(sv);
    read_register(basis_idx, &layout.lam_qubits())
}

/// Build a basis state with the control qubit set to `ctrl_bit` and the point registers
/// encoding `p`.
///
/// The λ scratch register is initialized to 0.
///
/// # Panics
///
/// Panics if `n_total < layout.total_qubits()`.
#[must_use]
pub fn make_point_state(ctrl_bit: u8, p: Point, layout: &PointAddLayout) -> StateVec {
    let n_total = layout.total_qubits();
    let (x_val, y_val) = curve::encode_point(p);
    let mut basis_idx = 0usize;
    // Set control qubit.
    if ctrl_bit != 0 {
        basis_idx |= 1 << layout.ctrl_qubit;
    }
    // Set x register (little-endian).
    for (k, &q) in layout.x_qubits().iter().enumerate() {
        if (x_val >> k) & 1 == 1 {
            basis_idx |= 1 << q;
        }
    }
    // Set y register (little-endian).
    for (k, &q) in layout.y_qubits().iter().enumerate() {
        if (y_val >> k) & 1 == 1 {
            basis_idx |= 1 << q;
        }
    }
    // λ register starts at 0 (no bits set).
    StateVec::basis(n_total, basis_idx)
}
