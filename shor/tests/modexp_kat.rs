//! Known-answer tests for the reversible modular-exponentiation quantum circuit.
//!
//! # What is tested
//!
//! 1. **Permutation correctness** — `controlled_mult_mod` by `c` on basis state `|x⟩`
//!    (control set) gives `|c·x mod N⟩` for a fixture of `(c, x, N)` triples.
//!
//! 2. **Modular exponentiation correctness** — the controlled-`aˣ mod N` circuit on
//!    `|x⟩|1⟩` (control set) gives `|x⟩|aˣ mod N⟩` for a fixture of `(a, x, N)` triples.
//!
//! 3. **Reversibility** — the circuit followed by its inverse is the identity on basis states.
//!
//! 4. **Ancilla-clean** — every ancilla returns to `|0⟩` after the circuit (basis-state check).
//!    (In this implementation, no ancilla qubits are used beyond the work register — the
//!    permutation synthesis is ancilla-free — so this KAT verifies the work register is clean
//!    after the inverse circuit restores it.)
//!
//! 5. **Control-off no-op** — with the control qubit `|0⟩`, the circuit is the identity.
//!
//! # Basis-indexing convention
//!
//! Little-endian (qubit 0 = LSB), matching the C-StateVec convention.
//! The work register occupies qubits `[t, t+n)` in the full state vector.
//!
//! # Published reference values
//!
//! Modular multiplication:
//!   - 2 · 3 mod 15 = 6
//!   - 7 · 4 mod 15 = 13
//!   - 2 · 1 mod 15 = 2
//!   - 4 · 7 mod 21 = 7  (4·7 = 28 = 21 + 7)
//!   - 2 · 3 mod 35 = 6
//!
//! Modular exponentiation:
//!   - 2^3 mod 15 = 8
//!   - 7^2 mod 15 = 4  (49 mod 15 = 4)
//!   - 2^4 mod 15 = 1  (ord₂(15) = 4)
//!   - 2^6 mod 21 = 1  (ord₂(21) = 6)
//!   - 4^3 mod 15 = 4  (64 mod 15 = 4)
//!
//! Reference: Shor (1994); Nielsen & Chuang §5.3.

use shor::arith::{
    controlled_add_mod, controlled_mod_exp, controlled_mod_exp_inv, controlled_mult_mod,
    controlled_mult_mod_inv, find_basis_index, mod_pow, n_bits, read_work_register, ModExpLayout,
};
use shor::statevec::StateVec;

// ── helpers ───────────────────────────────────────────────────────────────────

/// Amplitude tolerance for basis-state KATs (exact classical permutation, no ε needed).
const EPS: f64 = 1e-9;

/// Assert that `sv` is in basis state `expected_idx` (amplitude 1 at that index, 0 elsewhere).
fn assert_basis_state(sv: &StateVec, expected_idx: usize, label: &str) {
    let got_idx = find_basis_index(sv);
    assert_eq!(
        got_idx, expected_idx,
        "{label}: expected basis state |{expected_idx}⟩, got |{got_idx}⟩"
    );
}

/// Assert that the work register encodes `expected_val` in a basis state.
fn assert_work_val(sv: &StateVec, work_qubits: &[usize], expected_val: u64, label: &str) {
    let got = read_work_register(sv, work_qubits);
    assert_eq!(
        got, expected_val,
        "{label}: work register = {got}, expected {expected_val}"
    );
}

/// Build a state vector with the control qubit set to `ctrl_bit` and the work register
/// encoding `work_val`.
///
/// Layout: qubit 0 = control, qubits [1, 1+n) = work register (n bits).
fn make_ctrl_work_state(ctrl_bit: u8, work_val: u64, n_work: usize) -> StateVec {
    let n_total = 1 + n_work;
    // Basis index: ctrl_bit in qubit 0 (LSB), work_val in qubits [1, 1+n).
    let basis_idx = (ctrl_bit as usize) | ((work_val as usize) << 1);
    StateVec::basis(n_total, basis_idx)
}

/// Work qubit indices for the ctrl+work layout: qubits [1, 1+n).
fn work_qubits(n_work: usize) -> Vec<usize> {
    (1..1 + n_work).collect()
}

/// Build a state vector for the mod-exp circuit with the given layout.
///
/// Sets the exponent register to `exp_val` and the work register to `work_val`.
fn make_modexp_state(layout: &ModExpLayout, exp_val: u64, work_val: u64) -> StateVec {
    let n_total = layout.total_qubits();
    // Encode exp_val in exponent register and work_val in work register.
    let mut basis_idx = 0usize;
    for (k, &q) in layout.exp_qubits().iter().enumerate() {
        if (exp_val >> k) & 1 == 1 {
            basis_idx |= 1 << q;
        }
    }
    for (k, &q) in layout.work_qubits().iter().enumerate() {
        if (work_val >> k) & 1 == 1 {
            basis_idx |= 1 << q;
        }
    }
    StateVec::basis(n_total, basis_idx)
}

// ── KAT group 1: permutation correctness (controlled_mult_mod) ────────────────
//
// For each (c, x, N) triple, verify that controlled_mult_mod by c on basis state |x⟩
// (control set) gives |c·x mod N⟩.
//
// Reference values:
//   (c=2, x=3, N=15): 2·3 mod 15 = 6
//   (c=7, x=4, N=15): 7·4 mod 15 = 13  (28 mod 15 = 13)
//   (c=2, x=1, N=15): 2·1 mod 15 = 2
//   (c=2, x=7, N=15): 2·7 mod 15 = 14
//   (c=4, x=7, N=21): 4·7 mod 21 = 7   (28 mod 21 = 7)
//   (c=2, x=3, N=35): 2·3 mod 35 = 6
//   (c=11, x=5, N=35): 11·5 mod 35 = 20 (55 mod 35 = 20)
//   (c=2, x=0, N=15): 2·0 mod 15 = 0   (identity on 0)

fn check_mult_mod(c: u64, x: u64, modulus: u64) {
    let n_work = n_bits(modulus + 1);
    let mut sv = make_ctrl_work_state(1, x, n_work);
    let wq = work_qubits(n_work);
    let expected = c * x % modulus;
    controlled_mult_mod(&mut sv, 0, c, modulus, &wq);
    assert_work_val(
        &sv,
        &wq,
        expected,
        &format!("mult_mod(c={c}, x={x}, N={modulus}): expected {expected}"),
    );
}

/// Permutation correctness: 2·3 mod 15 = 6.
#[test]
fn mult_mod_2x3_mod15() {
    check_mult_mod(2, 3, 15);
}

/// Permutation correctness: 7·4 mod 15 = 13.
#[test]
fn mult_mod_7x4_mod15() {
    check_mult_mod(7, 4, 15);
}

/// Permutation correctness: 2·1 mod 15 = 2.
#[test]
fn mult_mod_2x1_mod15() {
    check_mult_mod(2, 1, 15);
}

/// Permutation correctness: 2·7 mod 15 = 14.
#[test]
fn mult_mod_2x7_mod15() {
    check_mult_mod(2, 7, 15);
}

/// Permutation correctness: 4·7 mod 21 = 7 (28 mod 21 = 7).
#[test]
fn mult_mod_4x7_mod21() {
    check_mult_mod(4, 7, 21);
}

/// Permutation correctness: 2·3 mod 35 = 6.
#[test]
fn mult_mod_2x3_mod35() {
    check_mult_mod(2, 3, 35);
}

/// Permutation correctness: 11·5 mod 35 = 20 (55 mod 35 = 20).
#[test]
fn mult_mod_11x5_mod35() {
    check_mult_mod(11, 5, 35);
}

/// Permutation correctness: 2·0 mod 15 = 0 (identity on 0).
#[test]
fn mult_mod_2x0_mod15() {
    check_mult_mod(2, 0, 15);
}

/// Permutation correctness: full sweep of x in [0, 15) for c=2, N=15.
///
/// Verifies 2·x mod 15 for all x in [0, 15). Published: ord₂(15) = 4.
#[test]
fn mult_mod_sweep_c2_mod15() {
    for x in 0..15u64 {
        check_mult_mod(2, x, 15);
    }
}

/// Permutation correctness: full sweep of x in [0, 21) for c=2, N=21.
///
/// Verifies 2·x mod 21 for all x in [0, 21). Published: ord₂(21) = 6.
#[test]
fn mult_mod_sweep_c2_mod21() {
    for x in 0..21u64 {
        check_mult_mod(2, x, 21);
    }
}

// ── KAT group 2: modular exponentiation correctness ───────────────────────────
//
// For each (a, x, N) triple, verify that the controlled-aˣ mod N circuit on |x⟩|1⟩
// (control set) gives |x⟩|aˣ mod N⟩.
//
// Reference values (published orders):
//   ord₂(15) = 4: 2^0=1, 2^1=2, 2^2=4, 2^3=8, 2^4=1 (cycle)
//   ord₇(15) = 4: 7^0=1, 7^1=7, 7^2=4, 7^3=13, 7^4=1 (cycle)
//   ord₂(21) = 6: 2^0=1, 2^1=2, 2^2=4, 2^3=8, 2^4=16, 2^5=11, 2^6=1 (cycle)
//   ord₂(35) = 12: 2^0=1, 2^1=2, ..., 2^12=1 (cycle)

fn check_mod_exp(a: u64, exp_val: u64, modulus: u64) {
    let exp_len = n_bits(exp_val + 1).max(1);
    let layout = ModExpLayout::standard(modulus, exp_len);
    let mut sv = make_modexp_state(&layout, exp_val, 1); // work starts at |1⟩
    let expected = mod_pow(a, exp_val, modulus);
    controlled_mod_exp(&mut sv, a, &layout);
    let wq = layout.work_qubits();
    assert_work_val(
        &sv,
        &wq,
        expected,
        &format!("mod_exp(a={a}, x={exp_val}, N={modulus}): expected {expected}"),
    );
    // Also verify exponent register is unchanged.
    let eq = layout.exp_qubits();
    let exp_got = read_work_register(&sv, &eq);
    assert_eq!(
        exp_got, exp_val,
        "mod_exp(a={a}, x={exp_val}, N={modulus}): exponent register changed from {exp_val} to {exp_got}"
    );
}

/// Mod-exp correctness: 2^3 mod 15 = 8.
#[test]
fn mod_exp_2_pow3_mod15() {
    check_mod_exp(2, 3, 15);
}

/// Mod-exp correctness: 7^2 mod 15 = 4 (49 mod 15 = 4).
#[test]
fn mod_exp_7_pow2_mod15() {
    check_mod_exp(7, 2, 15);
}

/// Mod-exp correctness: 2^4 mod 15 = 1 (ord₂(15) = 4).
#[test]
fn mod_exp_2_pow4_mod15() {
    check_mod_exp(2, 4, 15);
}

/// Mod-exp correctness: 2^1 mod 15 = 2.
#[test]
fn mod_exp_2_pow1_mod15() {
    check_mod_exp(2, 1, 15);
}

/// Mod-exp correctness: 2^0 mod 15 = 1 (identity exponent).
#[test]
fn mod_exp_2_pow0_mod15() {
    // exp_val=0 means exponent register is |0⟩ — control-off, work unchanged.
    let modulus = 15u64;
    let exp_len = 1;
    let layout = ModExpLayout::standard(modulus, exp_len);
    let mut sv = make_modexp_state(&layout, 0, 1);
    controlled_mod_exp(&mut sv, 2, &layout);
    let wq = layout.work_qubits();
    assert_work_val(&sv, &wq, 1, "mod_exp(a=2, x=0, N=15): work should stay 1");
}

/// Mod-exp correctness: 2^6 mod 21 = 1 (ord₂(21) = 6).
#[test]
fn mod_exp_2_pow6_mod21() {
    check_mod_exp(2, 6, 21);
}

/// Mod-exp correctness: 2^3 mod 21 = 8.
#[test]
fn mod_exp_2_pow3_mod21() {
    check_mod_exp(2, 3, 21);
}

/// Mod-exp correctness: full sweep of exponents for a=2, N=15.
///
/// Verifies 2^x mod 15 for x in [0, 8). Published cycle: 1,2,4,8,1,2,4,8,...
#[test]
fn mod_exp_sweep_a2_mod15() {
    for x in 0..8u64 {
        check_mod_exp(2, x, 15);
    }
}

/// Mod-exp correctness: full sweep of exponents for a=7, N=15.
///
/// Verifies 7^x mod 15 for x in [0, 8). Published cycle: 1,7,4,13,1,7,4,13,...
#[test]
fn mod_exp_sweep_a7_mod15() {
    for x in 0..8u64 {
        check_mod_exp(7, x, 15);
    }
}

/// Mod-exp correctness: a=2, N=35, x=12 (ord₂(35) = 12, so 2^12 mod 35 = 1).
#[test]
fn mod_exp_2_pow12_mod35() {
    check_mod_exp(2, 12, 35);
}

// ── KAT group 3: reversibility ────────────────────────────────────────────────
//
// The circuit followed by its inverse is the identity on basis states.
// Verified for controlled_mult_mod, controlled_add_mod, and controlled_mod_exp.

fn check_mult_mod_reversibility(c: u64, x: u64, modulus: u64) {
    let n_work = n_bits(modulus + 1);
    let original = make_ctrl_work_state(1, x, n_work);
    let mut sv = original.clone();
    let wq = work_qubits(n_work);
    // Forward: |x⟩ → |c·x mod N⟩
    controlled_mult_mod(&mut sv, 0, c, modulus, &wq);
    // Inverse: |c·x mod N⟩ → |x⟩
    controlled_mult_mod_inv(&mut sv, 0, c, modulus, &wq);
    // Should recover original.
    let orig_idx = find_basis_index(&original);
    assert_basis_state(
        &sv,
        orig_idx,
        &format!("mult_mod reversibility (c={c}, x={x}, N={modulus})"),
    );
}

/// Reversibility: controlled_mult_mod followed by inverse is identity (c=2, N=15).
#[test]
fn mult_mod_reversibility_c2_mod15() {
    for x in 0..15u64 {
        check_mult_mod_reversibility(2, x, 15);
    }
}

/// Reversibility: controlled_mult_mod followed by inverse is identity (c=7, N=15).
#[test]
fn mult_mod_reversibility_c7_mod15() {
    for x in 0..15u64 {
        check_mult_mod_reversibility(7, x, 15);
    }
}

/// Reversibility: controlled_mult_mod followed by inverse is identity (c=4, N=21).
#[test]
fn mult_mod_reversibility_c4_mod21() {
    for x in 0..21u64 {
        check_mult_mod_reversibility(4, x, 21);
    }
}

fn check_add_mod_reversibility(c: u64, x: u64, modulus: u64) {
    let n_work = n_bits(modulus + 1);
    let original = make_ctrl_work_state(1, x, n_work);
    let mut sv = original.clone();
    let wq = work_qubits(n_work);
    controlled_add_mod(&mut sv, 0, c, modulus, &wq);
    // Inverse: subtract c mod N.
    let c_inv = modulus - c; // (x + c) - c = x
    controlled_add_mod(&mut sv, 0, c_inv, modulus, &wq);
    let orig_idx = find_basis_index(&original);
    assert_basis_state(
        &sv,
        orig_idx,
        &format!("add_mod reversibility (c={c}, x={x}, N={modulus})"),
    );
}

/// Reversibility: controlled_add_mod followed by inverse is identity (c=3, N=15).
#[test]
fn add_mod_reversibility_c3_mod15() {
    for x in 0..15u64 {
        check_add_mod_reversibility(3, x, 15);
    }
}

/// Reversibility: controlled_add_mod followed by inverse is identity (c=5, N=21).
#[test]
fn add_mod_reversibility_c5_mod21() {
    for x in 0..21u64 {
        check_add_mod_reversibility(5, x, 21);
    }
}

fn check_mod_exp_reversibility(a: u64, exp_val: u64, modulus: u64) {
    let exp_len = n_bits(exp_val + 1).max(1);
    let layout = ModExpLayout::standard(modulus, exp_len);
    let original = make_modexp_state(&layout, exp_val, 1);
    let mut sv = original.clone();
    controlled_mod_exp(&mut sv, a, &layout);
    controlled_mod_exp_inv(&mut sv, a, &layout);
    let orig_idx = find_basis_index(&original);
    assert_basis_state(
        &sv,
        orig_idx,
        &format!("mod_exp reversibility (a={a}, x={exp_val}, N={modulus})"),
    );
}

/// Reversibility: controlled_mod_exp followed by inverse is identity (a=2, N=15).
#[test]
fn mod_exp_reversibility_a2_mod15() {
    for x in 1..8u64 {
        check_mod_exp_reversibility(2, x, 15);
    }
}

/// Reversibility: controlled_mod_exp followed by inverse is identity (a=7, N=15).
#[test]
fn mod_exp_reversibility_a7_mod15() {
    for x in 1..8u64 {
        check_mod_exp_reversibility(7, x, 15);
    }
}

/// Reversibility: controlled_mod_exp followed by inverse is identity (a=2, N=21).
#[test]
fn mod_exp_reversibility_a2_mod21() {
    for x in 1..8u64 {
        check_mod_exp_reversibility(2, x, 21);
    }
}

// ── KAT group 4: ancilla-clean ────────────────────────────────────────────────
//
// Every ancilla returns to |0⟩ after the circuit.
//
// In this implementation, the permutation synthesis is ancilla-free (no ancilla qubits
// beyond the work register are used). The "ancilla-clean" invariant is verified by
// checking that after the forward circuit + inverse circuit, the full state vector
// returns to the original basis state (no qubit is left in a non-|0⟩ state).
//
// For the mod-exp circuit, the work register starts at |1⟩ and returns to |1⟩ after
// forward + inverse. All other qubits (exponent register) are unchanged.

/// Ancilla-clean: after forward + inverse mod-exp, full state vector is restored (a=2, N=15).
///
/// Verifies that no qubit is left entangled or in a non-original state.
#[test]
fn mod_exp_ancilla_clean_a2_mod15() {
    let modulus = 15u64;
    let a = 2u64;
    for exp_val in 1..8u64 {
        let exp_len = n_bits(exp_val + 1).max(1);
        let layout = ModExpLayout::standard(modulus, exp_len);
        let original = make_modexp_state(&layout, exp_val, 1);
        let mut sv = original.clone();
        controlled_mod_exp(&mut sv, a, &layout);
        controlled_mod_exp_inv(&mut sv, a, &layout);
        // Full state vector must match original (all qubits clean).
        let orig_idx = find_basis_index(&original);
        let got_idx = find_basis_index(&sv);
        assert_eq!(
            got_idx, orig_idx,
            "ancilla-clean (a={a}, x={exp_val}, N={modulus}): full state index {got_idx} != {orig_idx}"
        );
        // Verify all amplitudes match.
        for (i, (&got, &exp)) in sv.amplitudes().iter().zip(original.amplitudes().iter()).enumerate() {
            let diff = (got - exp).norm();
            assert!(
                diff < EPS,
                "ancilla-clean (a={a}, x={exp_val}, N={modulus}): amplitude[{i}] diff={diff:.2e}"
            );
        }
    }
}

/// Ancilla-clean: after forward + inverse mult-mod, full state vector is restored (c=2, N=15).
#[test]
fn mult_mod_ancilla_clean_c2_mod15() {
    let modulus = 15u64;
    let c = 2u64;
    let n_work = n_bits(modulus + 1);
    for x in 0..modulus {
        let original = make_ctrl_work_state(1, x, n_work);
        let mut sv = original.clone();
        let wq = work_qubits(n_work);
        controlled_mult_mod(&mut sv, 0, c, modulus, &wq);
        controlled_mult_mod_inv(&mut sv, 0, c, modulus, &wq);
        let orig_idx = find_basis_index(&original);
        let got_idx = find_basis_index(&sv);
        assert_eq!(
            got_idx, orig_idx,
            "ancilla-clean mult_mod (c={c}, x={x}, N={modulus}): state {got_idx} != {orig_idx}"
        );
    }
}

/// Ancilla-clean: after forward + inverse add-mod, full state vector is restored (c=3, N=15).
#[test]
fn add_mod_ancilla_clean_c3_mod15() {
    let modulus = 15u64;
    let c = 3u64;
    let n_work = n_bits(modulus + 1);
    for x in 0..modulus {
        let original = make_ctrl_work_state(1, x, n_work);
        let mut sv = original.clone();
        let wq = work_qubits(n_work);
        controlled_add_mod(&mut sv, 0, c, modulus, &wq);
        let c_inv = modulus - c;
        controlled_add_mod(&mut sv, 0, c_inv, modulus, &wq);
        let orig_idx = find_basis_index(&original);
        let got_idx = find_basis_index(&sv);
        assert_eq!(
            got_idx, orig_idx,
            "ancilla-clean add_mod (c={c}, x={x}, N={modulus}): state {got_idx} != {orig_idx}"
        );
    }
}

// ── KAT group 5: control-off no-op ───────────────────────────────────────────
//
// With the control qubit |0⟩, the circuit is the identity.
// Verified for controlled_mult_mod, controlled_add_mod, and controlled_mod_exp.

fn check_mult_mod_control_off(c: u64, x: u64, modulus: u64) {
    let n_work = n_bits(modulus + 1);
    let original = make_ctrl_work_state(0, x, n_work); // ctrl = |0⟩
    let mut sv = original.clone();
    let wq = work_qubits(n_work);
    controlled_mult_mod(&mut sv, 0, c, modulus, &wq);
    // Work register must be unchanged.
    assert_work_val(
        &sv,
        &wq,
        x,
        &format!("control-off mult_mod (c={c}, x={x}, N={modulus}): work should be unchanged"),
    );
    // Full state must be unchanged.
    let orig_idx = find_basis_index(&original);
    assert_basis_state(
        &sv,
        orig_idx,
        &format!("control-off mult_mod (c={c}, x={x}, N={modulus}): state should be unchanged"),
    );
}

/// Control-off no-op: controlled_mult_mod with ctrl=|0⟩ is identity (c=2, N=15).
#[test]
fn mult_mod_control_off_c2_mod15() {
    for x in 0..15u64 {
        check_mult_mod_control_off(2, x, 15);
    }
}

/// Control-off no-op: controlled_mult_mod with ctrl=|0⟩ is identity (c=7, N=21).
#[test]
fn mult_mod_control_off_c7_mod21() {
    for x in 0..21u64 {
        check_mult_mod_control_off(7, x, 21);
    }
}

fn check_add_mod_control_off(c: u64, x: u64, modulus: u64) {
    let n_work = n_bits(modulus + 1);
    let original = make_ctrl_work_state(0, x, n_work); // ctrl = |0⟩
    let mut sv = original.clone();
    let wq = work_qubits(n_work);
    controlled_add_mod(&mut sv, 0, c, modulus, &wq);
    assert_work_val(
        &sv,
        &wq,
        x,
        &format!("control-off add_mod (c={c}, x={x}, N={modulus}): work should be unchanged"),
    );
}

/// Control-off no-op: controlled_add_mod with ctrl=|0⟩ is identity (c=3, N=15).
#[test]
fn add_mod_control_off_c3_mod15() {
    for x in 0..15u64 {
        check_add_mod_control_off(3, x, 15);
    }
}

/// Control-off no-op: controlled_mod_exp with ctrl=|0⟩ (all exponent qubits) is identity.
///
/// When the exponent register is |0⟩, the work register must be unchanged.
#[test]
fn mod_exp_control_off_a2_mod15() {
    let modulus = 15u64;
    let a = 2u64;
    let exp_len = 4; // enough for ord₂(15) = 4
    let layout = ModExpLayout::standard(modulus, exp_len);
    // Exponent = 0 means all exponent qubits are |0⟩ — control-off.
    let mut sv = make_modexp_state(&layout, 0, 1);
    controlled_mod_exp(&mut sv, a, &layout);
    let wq = layout.work_qubits();
    assert_work_val(&sv, &wq, 1, "control-off mod_exp: work should stay 1");
}

/// Control-off no-op: controlled_mod_exp with ctrl=|0⟩ is identity for various work values.
#[test]
fn mod_exp_control_off_various_work() {
    let modulus = 15u64;
    let a = 2u64;
    let exp_len = 4;
    let layout = ModExpLayout::standard(modulus, exp_len);
    // Test with various initial work values (not just 1).
    for work_val in 1..modulus {
        let original = make_modexp_state(&layout, 0, work_val);
        let mut sv = original.clone();
        controlled_mod_exp(&mut sv, a, &layout);
        let wq = layout.work_qubits();
        assert_work_val(
            &sv,
            &wq,
            work_val,
            &format!("control-off mod_exp: work={work_val} should be unchanged"),
        );
    }
}

// ── additional correctness KATs ───────────────────────────────────────────────

/// Add-mod correctness: full sweep for c=3, N=15.
///
/// Verifies (x + 3) mod 15 for all x in [0, 15).
#[test]
fn add_mod_sweep_c3_mod15() {
    let modulus = 15u64;
    let c = 3u64;
    let n_work = n_bits(modulus + 1);
    for x in 0..modulus {
        let mut sv = make_ctrl_work_state(1, x, n_work);
        let wq = work_qubits(n_work);
        let expected = (x + c) % modulus;
        controlled_add_mod(&mut sv, 0, c, modulus, &wq);
        assert_work_val(
            &sv,
            &wq,
            expected,
            &format!("add_mod(c={c}, x={x}, N={modulus}): expected {expected}"),
        );
    }
}

/// Add-mod correctness: full sweep for c=7, N=21.
#[test]
fn add_mod_sweep_c7_mod21() {
    let modulus = 21u64;
    let c = 7u64;
    let n_work = n_bits(modulus + 1);
    for x in 0..modulus {
        let mut sv = make_ctrl_work_state(1, x, n_work);
        let wq = work_qubits(n_work);
        let expected = (x + c) % modulus;
        controlled_add_mod(&mut sv, 0, c, modulus, &wq);
        assert_work_val(
            &sv,
            &wq,
            expected,
            &format!("add_mod(c={c}, x={x}, N={modulus}): expected {expected}"),
        );
    }
}

/// Mod-exp correctness: a=2, N=35, full sweep of exponents [0, 12].
///
/// Published: ord₂(35) = 12, so 2^12 mod 35 = 1.
#[test]
fn mod_exp_sweep_a2_mod35() {
    for x in 0..13u64 {
        check_mod_exp(2, x, 35);
    }
}

/// Classical helper: mod_pow correctness.
///
/// Verifies the classical mod_pow helper against published values.
#[test]
fn mod_pow_correctness() {
    // 2^4 mod 15 = 1 (ord₂(15) = 4)
    assert_eq!(mod_pow(2, 4, 15), 1, "2^4 mod 15");
    // 7^4 mod 15 = 1 (ord₇(15) = 4)
    assert_eq!(mod_pow(7, 4, 15), 1, "7^4 mod 15");
    // 2^6 mod 21 = 1 (ord₂(21) = 6)
    assert_eq!(mod_pow(2, 6, 21), 1, "2^6 mod 21");
    // 2^12 mod 35 = 1 (ord₂(35) = 12)
    assert_eq!(mod_pow(2, 12, 35), 1, "2^12 mod 35");
    // 2^3 mod 15 = 8
    assert_eq!(mod_pow(2, 3, 15), 8, "2^3 mod 15");
    // 7^2 mod 15 = 4
    assert_eq!(mod_pow(7, 2, 15), 4, "7^2 mod 15");
}

/// ModExpLayout: total_qubits is correct for standard layout.
#[test]
fn layout_total_qubits() {
    // N=15 (4-bit), t=4: total = 4 + 4 = 8
    let layout = ModExpLayout::standard(15, 4);
    assert_eq!(layout.work_len, 4, "N=15 needs 4 work bits");
    assert_eq!(layout.total_qubits(), 8, "N=15, t=4: total = 8");

    // N=21 (5-bit), t=4: total = 4 + 5 = 9
    let layout = ModExpLayout::standard(21, 4);
    assert_eq!(layout.work_len, 5, "N=21 needs 5 work bits");
    assert_eq!(layout.total_qubits(), 9, "N=21, t=4: total = 9");

    // N=35 (6-bit), t=6: total = 6 + 6 = 12
    let layout = ModExpLayout::standard(35, 6);
    assert_eq!(layout.work_len, 6, "N=35 needs 6 work bits");
    assert_eq!(layout.total_qubits(), 12, "N=35, t=6: total = 12");
}
