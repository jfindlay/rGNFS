//! Known-answer tests for the GF(2^l) ⊂ GF(2^m) subfield substrate.
//!
//! # Fields used
//!
//! - GF(2^6) with irreducible `x⁶ + x + 1` (poly = 0x43, degree 6).
//! - GF(2^2) with irreducible `x² + x + 1` (poly = 0x7, degree 2).
//! - GF(2^3) with irreducible `x³ + x + 1` (poly = 0xb, degree 3).
//!
//! # GHS fixture
//!
//! The canonical GHS fixture is `m = 6, l = 2, m/l = 3` (odd).  Odd `m/l` keeps the
//! descent in the imaginary/ramified hyperelliptic model the frozen C-HyperCurve handles.
//! The fixture `m = 6, l = 3, m/l = 2` (even) is also tested for completeness.
//!
//! # Coverage
//!
//! - **Trace landing**: `Tr_{m/l}(a) ∈ GF(2^l)` — `is_in_subfield(Tr_{m/l}(a), l)` is true.
//! - **Trace additivity**: `Tr_{m/l}(a + b) = Tr_{m/l}(a) + Tr_{m/l}(b)` (char-2: + is XOR).
//! - **Norm multiplicativity**: `N_{m/l}(a·b) = N_{m/l}(a)·N_{m/l}(b)`, landing in GF(2^l).
//! - **embed∘restrict round-trip**: `restrict(embed(c)) = Some(c)` for `c ∈ GF(2^l)`.
//! - **Subfield membership**: `is_in_subfield(a, l)` ⟺ `a^(2^l) = a`; GF(2^l) has exactly
//!   `2^l` elements.
//! - **Frobenius-by-subfield orbit**: length `m/l`, `a^(2^(il))` correct.
//! - **GHS fixture confirmation**: `m = 6, l = 2, m/l = 3` (odd) is non-degenerate.

use crypto_bigint::Uint;
use shared_gf2m::{
    F2m, F2mNaive, embed, frobenius_subfield_orbit, is_in_subfield, relative_norm, relative_trace,
    restrict, subfield_basis,
};

// ── Field parameters ──────────────────────────────────────────────────────────

/// GF(2^6) irreducible: x⁶ + x + 1 = 0x43.  Degree m = 6.
fn poly6() -> Uint<1> {
    Uint::<1>::from(0x43u64)
}

/// GF(2^2) irreducible: x² + x + 1 = 0x7.  Degree l = 2.
fn poly2() -> Uint<1> {
    Uint::<1>::from(0x7u64)
}

/// GF(2^3) irreducible: x³ + x + 1 = 0xb.  Degree l = 3.
fn poly3() -> Uint<1> {
    Uint::<1>::from(0xbu64)
}

type F = F2mNaive<1>;

fn f6(v: u64) -> F {
    F::from_u64(v, &poly6())
}

fn f2(v: u64) -> F {
    F::from_u64(v, &poly2())
}

fn f3(v: u64) -> F {
    F::from_u64(v, &poly3())
}

// ── GHS fixture confirmation ──────────────────────────────────────────────────

/// Confirm the GHS fixture: m=6, l=2, m/l=3 (odd) is non-degenerate.
///
/// Odd m/l is the canonical GHS target: it keeps the descent in the imaginary/ramified
/// hyperelliptic model the frozen C-HyperCurve handles.
#[test]
fn ghs_fixture_m6_l2_odd_ratio() {
    let m = 6usize;
    let l = 2usize;
    assert_eq!(m % l, 0, "l must divide m");
    let ratio = m / l;
    assert_eq!(ratio, 3, "m/l should be 3");
    assert_eq!(ratio % 2, 1, "m/l = 3 is odd — canonical GHS imaginary-model fixture");
}

// ── Subfield membership ───────────────────────────────────────────────────────

/// GF(2^2) inside GF(2^6) has exactly 4 elements.
#[test]
fn subfield_gf2_has_4_elements() {
    let p6 = poly6();
    let count = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 2, &p6)).count();
    assert_eq!(count, 4, "GF(2^2) ⊂ GF(2^6) should have exactly 4 elements");
}

/// GF(2^3) inside GF(2^6) has exactly 8 elements.
#[test]
fn subfield_gf3_has_8_elements() {
    let p6 = poly6();
    let count = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 3, &p6)).count();
    assert_eq!(count, 8, "GF(2^3) ⊂ GF(2^6) should have exactly 8 elements");
}

/// `is_in_subfield(a, l)` ⟺ `a^(2^l) = a`: membership is the Frobenius fixed-point condition.
#[test]
fn subfield_membership_iff_frobenius_fixed_point_l2() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        // Compute a^(2^l) by applying frobenius l=2 times.
        let a_frob = a.frobenius(&p6).frobenius(&p6);
        let fixed = a_frob == a;
        let member = is_in_subfield(&a, 2, &p6);
        assert_eq!(
            member, fixed,
            "is_in_subfield({v:#x}, 2) = {member} but a^(2^2) == a is {fixed}"
        );
    }
}

/// Same fixed-point check for l=3.
#[test]
fn subfield_membership_iff_frobenius_fixed_point_l3() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        // Compute a^(2^l) by applying frobenius l=3 times.
        let a_frob = a.frobenius(&p6).frobenius(&p6).frobenius(&p6);
        let fixed = a_frob == a;
        let member = is_in_subfield(&a, 3, &p6);
        assert_eq!(
            member, fixed,
            "is_in_subfield({v:#x}, 3) = {member} but a^(2^3) == a is {fixed}"
        );
    }
}

// ── Trace landing ─────────────────────────────────────────────────────────────

/// `Tr_{6/2}(a) ∈ GF(2^2)` for all `a ∈ GF(2^6)`.
#[test]
fn trace_lands_in_subfield_l2() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let tr = relative_trace(&a, 2, &p6);
        assert!(
            is_in_subfield(&tr, 2, &p6),
            "Tr_{{6/2}}({v:#x}) = {:?} not in GF(2^2)",
            tr.to_uint()
        );
    }
}

/// `Tr_{6/3}(a) ∈ GF(2^3)` for all `a ∈ GF(2^6)`.
#[test]
fn trace_lands_in_subfield_l3() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let tr = relative_trace(&a, 3, &p6);
        assert!(
            is_in_subfield(&tr, 3, &p6),
            "Tr_{{6/3}}({v:#x}) = {:?} not in GF(2^3)",
            tr.to_uint()
        );
    }
}

// ── Trace additivity ──────────────────────────────────────────────────────────

/// `Tr_{6/2}(a + b) = Tr_{6/2}(a) + Tr_{6/2}(b)` for all `a, b ∈ GF(2^6)`.
///
/// Additivity is the defining property of a trace map.  In char 2, `+` is XOR.
#[test]
fn trace_additivity_l2() {
    let p6 = poly6();
    // Exhaustive over all pairs — 64×64 = 4096 pairs, fast.
    for va in 0u64..64 {
        for vb in 0u64..64 {
            let a = f6(va);
            let b = f6(vb);
            let tr_ab = relative_trace(&a.add(&b), 2, &p6);
            let tr_a_plus_tr_b = relative_trace(&a, 2, &p6).add(&relative_trace(&b, 2, &p6));
            assert_eq!(
                tr_ab, tr_a_plus_tr_b,
                "Tr_{{6/2}}({va:#x} + {vb:#x}) ≠ Tr({va:#x}) + Tr({vb:#x})"
            );
        }
    }
}

/// `Tr_{6/3}(a + b) = Tr_{6/3}(a) + Tr_{6/3}(b)` for all `a, b ∈ GF(2^6)`.
#[test]
fn trace_additivity_l3() {
    let p6 = poly6();
    for va in 0u64..64 {
        for vb in 0u64..64 {
            let a = f6(va);
            let b = f6(vb);
            let tr_ab = relative_trace(&a.add(&b), 3, &p6);
            let tr_a_plus_tr_b = relative_trace(&a, 3, &p6).add(&relative_trace(&b, 3, &p6));
            assert_eq!(
                tr_ab, tr_a_plus_tr_b,
                "Tr_{{6/3}}({va:#x} + {vb:#x}) ≠ Tr({va:#x}) + Tr({vb:#x})"
            );
        }
    }
}

// ── Norm multiplicativity ─────────────────────────────────────────────────────

/// `N_{6/2}(a·b) = N_{6/2}(a)·N_{6/2}(b)` for all `a, b ∈ GF(2^6)`.
///
/// Multiplicativity is the defining property of a norm map.
#[test]
fn norm_multiplicativity_l2() {
    let p6 = poly6();
    for va in 0u64..64 {
        for vb in 0u64..64 {
            let a = f6(va);
            let b = f6(vb);
            let n_ab = relative_norm(&a.mul(&b, &p6), 2, &p6);
            let n_a_times_n_b = relative_norm(&a, 2, &p6).mul(&relative_norm(&b, 2, &p6), &p6);
            assert_eq!(
                n_ab, n_a_times_n_b,
                "N_{{6/2}}({va:#x} · {vb:#x}) ≠ N({va:#x}) · N({vb:#x})"
            );
        }
    }
}

/// `N_{6/3}(a·b) = N_{6/3}(a)·N_{6/3}(b)` for all `a, b ∈ GF(2^6)`.
#[test]
fn norm_multiplicativity_l3() {
    let p6 = poly6();
    for va in 0u64..64 {
        for vb in 0u64..64 {
            let a = f6(va);
            let b = f6(vb);
            let n_ab = relative_norm(&a.mul(&b, &p6), 3, &p6);
            let n_a_times_n_b = relative_norm(&a, 3, &p6).mul(&relative_norm(&b, 3, &p6), &p6);
            assert_eq!(
                n_ab, n_a_times_n_b,
                "N_{{6/3}}({va:#x} · {vb:#x}) ≠ N({va:#x}) · N({vb:#x})"
            );
        }
    }
}

/// `N_{6/2}(a) ∈ GF(2^2)` for all `a ∈ GF(2^6)`.
#[test]
fn norm_lands_in_subfield_l2() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let n = relative_norm(&a, 2, &p6);
        assert!(
            is_in_subfield(&n, 2, &p6),
            "N_{{6/2}}({v:#x}) = {:?} not in GF(2^2)",
            n.to_uint()
        );
    }
}

/// `N_{6/3}(a) ∈ GF(2^3)` for all `a ∈ GF(2^6)`.
#[test]
fn norm_lands_in_subfield_l3() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let n = relative_norm(&a, 3, &p6);
        assert!(
            is_in_subfield(&n, 3, &p6),
            "N_{{6/3}}({v:#x}) = {:?} not in GF(2^3)",
            n.to_uint()
        );
    }
}

// ── embed∘restrict round-trip ─────────────────────────────────────────────────

/// `restrict(embed(c), l, poly_m) = Some(c)` for all `c ∈ GF(2^2)`.
#[test]
fn embed_restrict_round_trip_l2() {
    let p6 = poly6();
    let p2 = poly2();
    // GF(2^2) has 4 elements: 0, 1, 2, 3 (as bit-vectors mod x²+x+1).
    for v in 0u64..4 {
        let c = f2(v);
        let embedded = embed(&c, &p2, &p6);
        // The embedded element must be in GF(2^2) ⊂ GF(2^6).
        assert!(
            is_in_subfield(&embedded, 2, &p6),
            "embed({v:#x}) = {:?} not in GF(2^2) ⊂ GF(2^6)",
            embedded.to_uint()
        );
        let restricted = restrict(&embedded, 2, &p6);
        assert!(restricted.is_some(), "restrict(embed({v:#x})) should be Some");
        assert_eq!(
            restricted.unwrap().to_uint(),
            c.to_uint(),
            "embed∘restrict round-trip failed for {v:#x}"
        );
    }
}

/// `restrict(embed(c), l, poly_m) = Some(c)` for all `c ∈ GF(2^3)`.
#[test]
fn embed_restrict_round_trip_l3() {
    let p6 = poly6();
    let p3 = poly3();
    // GF(2^3) has 8 elements: 0..7 (as bit-vectors mod x³+x+1).
    for v in 0u64..8 {
        let c = f3(v);
        let embedded = embed(&c, &p3, &p6);
        assert!(
            is_in_subfield(&embedded, 3, &p6),
            "embed({v:#x}) = {:?} not in GF(2^3) ⊂ GF(2^6)",
            embedded.to_uint()
        );
        let restricted = restrict(&embedded, 3, &p6);
        assert!(restricted.is_some(), "restrict(embed({v:#x})) should be Some");
        assert_eq!(
            restricted.unwrap().to_uint(),
            c.to_uint(),
            "embed∘restrict round-trip failed for {v:#x}"
        );
    }
}

/// `restrict` returns `None` for elements not in the subfield.
#[test]
fn restrict_none_for_non_subfield_elements() {
    let p6 = poly6();
    // Find an element not in GF(2^2) ⊂ GF(2^6).
    let non_member: Vec<u64> = (0u64..64).filter(|&v| !is_in_subfield(&f6(v), 2, &p6)).collect();
    assert!(!non_member.is_empty(), "there must be elements outside GF(2^2)");
    for v in non_member.iter().take(8) {
        let a = f6(*v);
        assert!(
            restrict(&a, 2, &p6).is_none(),
            "restrict({v:#x}) should be None (not in GF(2^2))"
        );
    }
}

// ── Frobenius-by-subfield orbit ───────────────────────────────────────────────

/// Orbit length is `m/l = 3` for `l = 2`.
#[test]
fn frobenius_orbit_length_l2() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let orbit = frobenius_subfield_orbit(&a, 2, &p6);
        assert_eq!(orbit.len(), 3, "orbit length should be m/l = 6/2 = 3 for v={v:#x}");
    }
}

/// Orbit length is `m/l = 2` for `l = 3`.
#[test]
fn frobenius_orbit_length_l3() {
    let p6 = poly6();
    for v in 0u64..64 {
        let a = f6(v);
        let orbit = frobenius_subfield_orbit(&a, 3, &p6);
        assert_eq!(orbit.len(), 2, "orbit length should be m/l = 6/3 = 2 for v={v:#x}");
    }
}

/// Each orbit entry `orbit[i]` equals `a^(2^(i·l))` computed by repeated Frobenius.
#[test]
fn frobenius_orbit_entries_correct_l2() {
    let p6 = poly6();
    // Check a few representative elements.
    for v in [0u64, 1, 5, 0x15, 0x3f] {
        let a = f6(v);
        let orbit = frobenius_subfield_orbit(&a, 2, &p6);
        // orbit[0] = a
        assert_eq!(orbit[0], a, "orbit[0] should be a for v={v:#x}");
        // orbit[1] = a^(2^2) = a.frob.frob
        let a_frob2 = a.frobenius(&p6).frobenius(&p6);
        assert_eq!(orbit[1], a_frob2, "orbit[1] should be a^(2^2) for v={v:#x}");
        // orbit[2] = a^(2^4) = a.frob.frob.frob.frob
        let a_frob4 = a_frob2.frobenius(&p6).frobenius(&p6);
        assert_eq!(orbit[2], a_frob4, "orbit[2] should be a^(2^4) for v={v:#x}");
    }
}

/// Orbit entries correct for `l = 3`.
#[test]
fn frobenius_orbit_entries_correct_l3() {
    let p6 = poly6();
    for v in [0u64, 1, 7, 0x2a, 0x3f] {
        let a = f6(v);
        let orbit = frobenius_subfield_orbit(&a, 3, &p6);
        assert_eq!(orbit[0], a, "orbit[0] should be a for v={v:#x}");
        // orbit[1] = a^(2^3) = a.frob.frob.frob
        let a_frob3 = a.frobenius(&p6).frobenius(&p6).frobenius(&p6);
        assert_eq!(orbit[1], a_frob3, "orbit[1] should be a^(2^3) for v={v:#x}");
    }
}

/// Subfield elements have a singleton orbit (orbit collapses to one distinct value).
///
/// For `a ∈ GF(2^l)`, `a^(2^l) = a`, so all orbit entries are equal.
#[test]
fn subfield_elements_have_constant_orbit_l2() {
    let p6 = poly6();
    // The 4 elements of GF(2^2) ⊂ GF(2^6).
    let subfield_elems: Vec<u64> = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 2, &p6)).collect();
    assert_eq!(subfield_elems.len(), 4);
    for v in subfield_elems {
        let a = f6(v);
        let orbit = frobenius_subfield_orbit(&a, 2, &p6);
        // All entries should equal a.
        for (i, entry) in orbit.iter().enumerate() {
            assert_eq!(
                *entry, a,
                "orbit[{i}] ≠ a for subfield element {v:#x}: Frobenius should fix it"
            );
        }
    }
}

// ── Subfield basis ────────────────────────────────────────────────────────────

/// The subfield basis for GF(2^2) inside GF(2^6) has exactly 2 elements.
#[test]
fn subfield_basis_length_l2() {
    let p6 = poly6();
    let basis = subfield_basis(2, &p6);
    assert_eq!(basis.len(), 2, "basis for GF(2^2) should have l=2 elements");
}

/// The subfield basis for GF(2^3) inside GF(2^6) has exactly 3 elements.
#[test]
fn subfield_basis_length_l3() {
    let p6 = poly6();
    let basis = subfield_basis(3, &p6);
    assert_eq!(basis.len(), 3, "basis for GF(2^3) should have l=3 elements");
}

/// All basis elements for GF(2^2) lie in GF(2^2) ⊂ GF(2^6).
#[test]
fn subfield_basis_elements_in_subfield_l2() {
    let p6 = poly6();
    let basis = subfield_basis(2, &p6);
    for (i, b) in basis.iter().enumerate() {
        assert!(
            is_in_subfield(b, 2, &p6),
            "basis[{i}] = {:?} not in GF(2^2)",
            b.to_uint()
        );
    }
}

/// All basis elements for GF(2^3) lie in GF(2^3) ⊂ GF(2^6).
#[test]
fn subfield_basis_elements_in_subfield_l3() {
    let p6 = poly6();
    let basis = subfield_basis(3, &p6);
    for (i, b) in basis.iter().enumerate() {
        assert!(
            is_in_subfield(b, 3, &p6),
            "basis[{i}] = {:?} not in GF(2^3)",
            b.to_uint()
        );
    }
}

/// The first basis element is always 1.
#[test]
fn subfield_basis_first_element_is_one() {
    let p6 = poly6();
    let basis2 = subfield_basis(2, &p6);
    let basis3 = subfield_basis(3, &p6);
    assert!(basis2[0].is_one(), "basis[0] should be 1 for l=2");
    assert!(basis3[0].is_one(), "basis[0] should be 1 for l=3");
}

/// The basis elements are linearly independent over GF(2) (span the subfield).
///
/// For GF(2^2): the 2 basis elements should span all 4 = 2^2 elements when
/// combined with GF(2) coefficients.
#[test]
fn subfield_basis_spans_gf2_l2() {
    let p6 = poly6();
    let basis = subfield_basis(2, &p6);
    // GF(2) combinations: 2^2 = 4 elements.
    let mut spanned = std::collections::HashSet::new();
    for c0 in 0u64..2 {
        for c1 in 0u64..2 {
            let mut elem = F::zero();
            if c0 == 1 {
                elem = elem.add(&basis[0]);
            }
            if c1 == 1 {
                elem = elem.add(&basis[1]);
            }
            spanned.insert(elem.to_uint());
        }
    }
    assert_eq!(spanned.len(), 4, "basis for GF(2^2) should span all 4 subfield elements");
    // All spanned elements should be in GF(2^2).
    for v in &spanned {
        let elem = F::from_uint(*v, &p6);
        assert!(is_in_subfield(&elem, 2, &p6), "spanned element {:?} not in GF(2^2)", v);
    }
}

/// For GF(2^3): the 3 basis elements span all 8 = 2^3 elements.
#[test]
fn subfield_basis_spans_gf2_l3() {
    let p6 = poly6();
    let basis = subfield_basis(3, &p6);
    let mut spanned = std::collections::HashSet::new();
    for c0 in 0u64..2 {
        for c1 in 0u64..2 {
            for c2 in 0u64..2 {
                let mut elem = F::zero();
                if c0 == 1 {
                    elem = elem.add(&basis[0]);
                }
                if c1 == 1 {
                    elem = elem.add(&basis[1]);
                }
                if c2 == 1 {
                    elem = elem.add(&basis[2]);
                }
                spanned.insert(elem.to_uint());
            }
        }
    }
    assert_eq!(spanned.len(), 8, "basis for GF(2^3) should span all 8 subfield elements");
    for v in &spanned {
        let elem = F::from_uint(*v, &p6);
        assert!(is_in_subfield(&elem, 3, &p6), "spanned element {:?} not in GF(2^3)", v);
    }
}

// ── Trace of subfield elements ────────────────────────────────────────────────

/// `Tr_{6/2}(a) = a` for `a ∈ GF(2^2)` (trace of a subfield element is itself × (m/l)).
///
/// For `a ∈ GF(2^l)`, `a^(2^(il)) = a` for all `i`, so `Tr_{m/l}(a) = (m/l) · a`.
/// In char 2, `(m/l) · a = a` if `m/l` is odd, `0` if `m/l` is even.
/// For `m=6, l=2, m/l=3` (odd): `Tr_{6/2}(a) = a` for `a ∈ GF(2^2)`.
#[test]
fn trace_of_subfield_element_l2_odd_ratio() {
    let p6 = poly6();
    // m/l = 3 is odd, so Tr_{6/2}(a) = 3·a = a (in char 2, 3 ≡ 1 mod 2).
    let subfield_elems: Vec<u64> = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 2, &p6)).collect();
    for v in subfield_elems {
        let a = f6(v);
        let tr = relative_trace(&a, 2, &p6);
        assert_eq!(
            tr, a,
            "Tr_{{6/2}}({v:#x}) should equal a (m/l=3 odd, so trace = identity on subfield)"
        );
    }
}

/// `Tr_{6/3}(a) = 0` for `a ∈ GF(2^3)` (trace of a subfield element is 0 when m/l is even).
///
/// For `m=6, l=3, m/l=2` (even): `Tr_{6/3}(a) = 2·a = 0` for `a ∈ GF(2^3)`.
#[test]
fn trace_of_subfield_element_l3_even_ratio() {
    let p6 = poly6();
    // m/l = 2 is even, so Tr_{6/3}(a) = 2·a = 0 (in char 2).
    let subfield_elems: Vec<u64> = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 3, &p6)).collect();
    for v in subfield_elems {
        let a = f6(v);
        let tr = relative_trace(&a, 3, &p6);
        assert!(
            tr.is_zero(),
            "Tr_{{6/3}}({v:#x}) should be 0 (m/l=2 even, so trace = 0 on subfield)"
        );
    }
}

// ── Norm of subfield elements ─────────────────────────────────────────────────

/// `N_{6/2}(a) = a^(m/l) = a^3` for `a ∈ GF(2^2)`.
///
/// For `a ∈ GF(2^l)`, `N_{m/l}(a) = a^(m/l)` since all orbit entries equal `a`.
#[test]
fn norm_of_subfield_element_l2() {
    let p6 = poly6();
    let subfield_elems: Vec<u64> = (0u64..64).filter(|&v| is_in_subfield(&f6(v), 2, &p6)).collect();
    for v in subfield_elems {
        let a = f6(v);
        let n = relative_norm(&a, 2, &p6);
        // N_{6/2}(a) = a^3 (m/l = 3 terms, each equal to a).
        let a_cubed = a.mul(&a, &p6).mul(&a, &p6);
        assert_eq!(
            n, a_cubed,
            "N_{{6/2}}({v:#x}) should equal a^3"
        );
    }
}
