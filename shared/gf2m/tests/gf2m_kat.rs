//! Known-answer tests and field-axiom tests for `F2mNaive`.
//!
//! Coverage:
//! - Field axioms over GF(2^4) with `x⁴+x+1` (poly = 0x13).
//! - Field axioms over GF(2^8) AES with `x⁸+x⁴+x³+x+1` (poly = 0x11b).
//! - Char-2 invariants: `sub(a,b) == add(a,b)`, `neg(a) == a`.
//! - Frobenius fixed-field law: `a^(2^m) == a` for all `a` in the field.
//! - `square(a) == mul(a, a)`.
//! - Associativity of `mul`.
//! - Distributivity of `mul` over `add`.
//! - `mul` by `one` is identity.
//! - `add` is its own inverse: `add(a, a) == zero`.
//! - Inversion round-trip: `mul(a, inv(a)) == one` for all non-zero `a`.
//! - Extended-Euclidean ↔ Itoh–Tsujii agreement.
//! - `inv(one) == one`.
//! - `inv(zero)` panics.
//! - `div(a, b) == mul(a, inv(b))`.
//!
//! The Frobenius fixed-field law `a^(2^m) = a` is the loudest correctness
//! signal: it fails immediately if modular reduction is wrong (wrong irreducible,
//! off-by-one in the shift, etc.).

use crypto_bigint::Uint;
use shared_gf2m::{ext_euclid_inv, itoh_tsujii_inv, F2m, F2mNaive};

// ── Field parameters ──────────────────────────────────────────────────────────

/// GF(2^4) irreducible: x⁴ + x + 1 = 0b1_0011 = 0x13.  Degree m = 4.
fn poly4() -> Uint<1> {
    Uint::<1>::from(0x13u64)
}

/// GF(2^8) AES irreducible: x⁸ + x⁴ + x³ + x + 1 = 0x11b.  Degree m = 8.
fn poly8() -> Uint<1> {
    Uint::<1>::from(0x11bu64)
}

/// All non-zero elements of GF(2^4): 1 through 15.
fn gf4_elements() -> Vec<F2mNaive<1>> {
    let p = poly4();
    (0u64..16).map(|v| F2mNaive::<1>::from_u64(v, &p)).collect()
}

/// All non-zero elements of GF(2^8): 1 through 255.
fn gf8_elements() -> Vec<F2mNaive<1>> {
    let p = poly8();
    (0u64..256).map(|v| F2mNaive::<1>::from_u64(v, &p)).collect()
}

// ── Char-2 invariant KATs ─────────────────────────────────────────────────────

/// `sub(a, b) == add(a, b)` for all pairs in GF(2^4).
///
/// This is the load-bearing char-2 invariant: subtraction is XOR, identical
/// to addition.  A wrong `sub` (porting `Fp`'s `p − self`) would fail here.
#[test]
fn sub_equals_add_gf4() {
    for a in gf4_elements() {
        for b in gf4_elements() {
            assert_eq!(
                a.sub(&b),
                a.add(&b),
                "sub != add for a={:#x} b={:#x} in GF(2^4)",
                a.to_uint(),
                b.to_uint()
            );
        }
    }
}

/// `sub(a, b) == add(a, b)` for all pairs in GF(2^8).
#[test]
fn sub_equals_add_gf8() {
    // Exhaustive over all 256×256 pairs is 65536 — fast enough.
    for a in gf8_elements() {
        for b in gf8_elements() {
            assert_eq!(
                a.sub(&b),
                a.add(&b),
                "sub != add for a={:#x} b={:#x} in GF(2^8)",
                a.to_uint(),
                b.to_uint()
            );
        }
    }
}

/// `neg(a) == a` for all elements in GF(2^4).
///
/// In characteristic 2, negation is the identity.  A wrong `neg` (porting
/// `Fp`'s `p − self`) would fail here.
#[test]
fn neg_is_identity_gf4() {
    for a in gf4_elements() {
        assert_eq!(
            a.neg(),
            a,
            "neg(a) != a for a={:#x} in GF(2^4)",
            a.to_uint()
        );
    }
}

/// `neg(a) == a` for all elements in GF(2^8).
#[test]
fn neg_is_identity_gf8() {
    for a in gf8_elements() {
        assert_eq!(
            a.neg(),
            a,
            "neg(a) != a for a={:#x} in GF(2^8)",
            a.to_uint()
        );
    }
}

// ── Additive axioms ───────────────────────────────────────────────────────────

/// `add(a, a) == zero` for all elements in GF(2^4).
///
/// Every element is its own additive inverse in char 2.
#[test]
fn add_self_is_zero_gf4() {
    let zero = F2mNaive::<1>::zero();
    for a in gf4_elements() {
        assert_eq!(
            a.add(&a),
            zero,
            "a + a != 0 for a={:#x} in GF(2^4)",
            a.to_uint()
        );
    }
}

/// `add(a, a) == zero` for all elements in GF(2^8).
#[test]
fn add_self_is_zero_gf8() {
    let zero = F2mNaive::<1>::zero();
    for a in gf8_elements() {
        assert_eq!(
            a.add(&a),
            zero,
            "a + a != 0 for a={:#x} in GF(2^8)",
            a.to_uint()
        );
    }
}

// ── Multiplicative axioms ─────────────────────────────────────────────────────

/// `mul(a, one) == a` for all elements in GF(2^4).
#[test]
fn mul_by_one_is_identity_gf4() {
    let p = poly4();
    let one = F2mNaive::<1>::one();
    for a in gf4_elements() {
        assert_eq!(
            a.mul(&one, &p),
            a,
            "a * 1 != a for a={:#x} in GF(2^4)",
            a.to_uint()
        );
    }
}

/// `mul(a, one) == a` for all elements in GF(2^8).
#[test]
fn mul_by_one_is_identity_gf8() {
    let p = poly8();
    let one = F2mNaive::<1>::one();
    for a in gf8_elements() {
        assert_eq!(
            a.mul(&one, &p),
            a,
            "a * 1 != a for a={:#x} in GF(2^8)",
            a.to_uint()
        );
    }
}

/// Associativity of `mul` over GF(2^4): `mul(mul(a,b),c) == mul(a,mul(b,c))`.
#[test]
fn mul_associative_gf4() {
    let p = poly4();
    // Exhaustive over all 16^3 = 4096 triples.
    for a in gf4_elements() {
        for b in gf4_elements() {
            for c in gf4_elements() {
                let lhs = a.mul(&b, &p).mul(&c, &p);
                let rhs = a.mul(&b.mul(&c, &p), &p);
                assert_eq!(
                    lhs, rhs,
                    "mul not associative: a={:#x} b={:#x} c={:#x} in GF(2^4)",
                    a.to_uint(), b.to_uint(), c.to_uint()
                );
            }
        }
    }
}

/// Associativity of `mul` over GF(2^8): spot-check a representative set.
///
/// Exhaustive (256^3 = 16M) is too slow; we check a representative subset.
#[test]
fn mul_associative_gf8() {
    let p = poly8();
    // Representative elements: 0, 1, x, x²-1, AES S-box inputs, max.
    let elems: Vec<u64> = vec![0, 1, 2, 3, 0x53, 0xca, 0x8d, 0xf6, 0x01, 0xff, 0x1b, 0xe5];
    for &av in &elems {
        for &bv in &elems {
            for &cv in &elems {
                let a = F2mNaive::<1>::from_u64(av, &p);
                let b = F2mNaive::<1>::from_u64(bv, &p);
                let c = F2mNaive::<1>::from_u64(cv, &p);
                let lhs = a.mul(&b, &p).mul(&c, &p);
                let rhs = a.mul(&b.mul(&c, &p), &p);
                assert_eq!(
                    lhs, rhs,
                    "mul not associative: a={av:#x} b={bv:#x} c={cv:#x} in GF(2^8)"
                );
            }
        }
    }
}

/// Distributivity of `mul` over `add` in GF(2^4):
/// `mul(a, add(b, c)) == add(mul(a,b), mul(a,c))`.
#[test]
fn mul_distributive_gf4() {
    let p = poly4();
    for a in gf4_elements() {
        for b in gf4_elements() {
            for c in gf4_elements() {
                let lhs = a.mul(&b.add(&c), &p);
                let rhs = a.mul(&b, &p).add(&a.mul(&c, &p));
                assert_eq!(
                    lhs, rhs,
                    "distributivity failed: a={:#x} b={:#x} c={:#x} in GF(2^4)",
                    a.to_uint(), b.to_uint(), c.to_uint()
                );
            }
        }
    }
}

/// Distributivity of `mul` over `add` in GF(2^8): spot-check.
#[test]
fn mul_distributive_gf8() {
    let p = poly8();
    let elems: Vec<u64> = vec![0, 1, 2, 3, 0x53, 0xca, 0x8d, 0xf6, 0x01, 0xff, 0x1b, 0xe5];
    for &av in &elems {
        for &bv in &elems {
            for &cv in &elems {
                let a = F2mNaive::<1>::from_u64(av, &p);
                let b = F2mNaive::<1>::from_u64(bv, &p);
                let c = F2mNaive::<1>::from_u64(cv, &p);
                let lhs = a.mul(&b.add(&c), &p);
                let rhs = a.mul(&b, &p).add(&a.mul(&c, &p));
                assert_eq!(
                    lhs, rhs,
                    "distributivity failed: a={av:#x} b={bv:#x} c={cv:#x} in GF(2^8)"
                );
            }
        }
    }
}

// ── Squaring / Frobenius ──────────────────────────────────────────────────────

/// `square(a) == mul(a, a)` for all elements in GF(2^4).
#[test]
fn square_equals_mul_self_gf4() {
    let p = poly4();
    for a in gf4_elements() {
        assert_eq!(
            a.square(&p),
            a.mul(&a, &p),
            "square != mul(a,a) for a={:#x} in GF(2^4)",
            a.to_uint()
        );
    }
}

/// `square(a) == mul(a, a)` for all elements in GF(2^8).
#[test]
fn square_equals_mul_self_gf8() {
    let p = poly8();
    for a in gf8_elements() {
        assert_eq!(
            a.square(&p),
            a.mul(&a, &p),
            "square != mul(a,a) for a={:#x} in GF(2^8)",
            a.to_uint()
        );
    }
}

// ── Frobenius fixed-field law: a^(2^m) == a ───────────────────────────────────
//
// This is the loudest correctness signal for the implementation.  If modular
// reduction is wrong (wrong irreducible, off-by-one in the shift, etc.), this
// test fails immediately.  Every element of GF(2^m) satisfies a^(2^m) = a
// (the Frobenius endomorphism has order m, so a^(2^m) = a).

/// `a^(2^4) == a` for all elements in GF(2^4).
///
/// Computed as 4 successive squarings (the Frobenius tower).
#[test]
fn frobenius_fixed_field_gf4() {
    let p = poly4();
    // 2^4 = 16 as a Uint<1>.
    let exp = Uint::<1>::from(16u64);
    for a in gf4_elements() {
        let a_pow = a.pow(&exp, &p);
        assert_eq!(
            a_pow, a,
            "a^(2^4) != a for a={:#x} in GF(2^4) — reduction is wrong",
            a.to_uint()
        );
    }
}

/// `a^(2^8) == a` for all elements in GF(2^8).
///
/// Computed via `pow` with exponent 256.
#[test]
fn frobenius_fixed_field_gf8() {
    let p = poly8();
    // 2^8 = 256 as a Uint<1>.
    let exp = Uint::<1>::from(256u64);
    for a in gf8_elements() {
        let a_pow = a.pow(&exp, &p);
        assert_eq!(
            a_pow, a,
            "a^(2^8) != a for a={:#x} in GF(2^8) — reduction is wrong",
            a.to_uint()
        );
    }
}

/// Frobenius tower: 4 successive squarings give identity in GF(2^4).
///
/// Verifies that `square` iterated m times returns the original element,
/// which is the direct Frobenius-tower check (distinct from `pow`).
#[test]
fn frobenius_tower_gf4() {
    let p = poly4();
    for a in gf4_elements() {
        let mut x = a.clone();
        for _ in 0..4 {
            x = x.square(&p);
        }
        assert_eq!(
            x, a,
            "Frobenius tower (4 squarings) != identity for a={:#x} in GF(2^4)",
            a.to_uint()
        );
    }
}

/// Frobenius tower: 8 successive squarings give identity in GF(2^8).
#[test]
fn frobenius_tower_gf8() {
    let p = poly8();
    for a in gf8_elements() {
        let mut x = a.clone();
        for _ in 0..8 {
            x = x.square(&p);
        }
        assert_eq!(
            x, a,
            "Frobenius tower (8 squarings) != identity for a={:#x} in GF(2^8)",
            a.to_uint()
        );
    }
}

// ── Known-answer tests ────────────────────────────────────────────────────────

/// AES GF(2^8) multiplication: 0x53 * 0xca = 0x01.
///
/// These are multiplicative inverses in the AES field.
#[test]
fn aes_mul_inverse_pair() {
    let p = poly8();
    let a = F2mNaive::<1>::from_u64(0x53, &p);
    let b = F2mNaive::<1>::from_u64(0xca, &p);
    let prod = a.mul(&b, &p);
    assert_eq!(
        prod.to_uint(),
        Uint::<1>::ONE,
        "0x53 * 0xca should be 1 in AES GF(2^8)"
    );
}

/// AES GF(2^8) multiplication: 0x57 * 0x83 = 0xc1.
///
/// Known AES MixColumns test vector.
#[test]
fn aes_mul_known_vector() {
    let p = poly8();
    let a = F2mNaive::<1>::from_u64(0x57, &p);
    let b = F2mNaive::<1>::from_u64(0x83, &p);
    let prod = a.mul(&b, &p);
    assert_eq!(
        prod.to_uint(),
        Uint::<1>::from(0xc1u64),
        "0x57 * 0x83 should be 0xc1 in AES GF(2^8)"
    );
}

/// GF(2^4) multiplication: x² * x² = x⁴ ≡ x + 1 (mod x⁴+x+1).
#[test]
fn gf4_mul_x2_x2() {
    let p = poly4();
    let x2 = F2mNaive::<1>::from_u64(0b0100, &p); // x²
    let prod = x2.mul(&x2, &p);
    // x⁴ mod (x⁴+x+1) = x+1 = 0b0011
    assert_eq!(prod.to_uint(), Uint::<1>::from(0b0011u64));
}

/// GF(2^4) squaring: square(x²) = x⁴ ≡ x + 1 (mod x⁴+x+1).
#[test]
fn gf4_square_x2() {
    let p = poly4();
    let x2 = F2mNaive::<1>::from_u64(0b0100, &p); // x²
    let sq = x2.square(&p);
    assert_eq!(sq.to_uint(), Uint::<1>::from(0b0011u64));
}

/// `frobenius` delegates to `square` and gives the same result.
#[test]
fn frobenius_equals_square() {
    let p = poly8();
    for a in gf8_elements() {
        assert_eq!(
            a.frobenius(&p),
            a.square(&p),
            "frobenius != square for a={:#x}",
            a.to_uint()
        );
    }
}

// ── Inversion round-trip ──────────────────────────────────────────────────────

/// `mul(a, inv(a)) == one` for all non-zero `a` in GF(2^4).
///
/// This is the primary inversion correctness check: every non-zero element
/// must have a multiplicative inverse.
#[test]
fn inv_round_trip_gf4() {
    let p = poly4();
    let one = F2mNaive::<1>::one();
    for v in 1u64..16 {
        let a = F2mNaive::<1>::from_u64(v, &p);
        let a_inv = a.inv(&p);
        let prod = a.mul(&a_inv, &p);
        assert_eq!(
            prod, one,
            "a * inv(a) != 1 for a={v:#x} in GF(2^4)"
        );
    }
}

/// `mul(a, inv(a)) == one` for all non-zero `a` in GF(2^8).
///
/// Exhaustive over all 255 non-zero elements of GF(2^8).
#[test]
fn inv_round_trip_gf8() {
    let p = poly8();
    let one = F2mNaive::<1>::one();
    for v in 1u64..256 {
        let a = F2mNaive::<1>::from_u64(v, &p);
        let a_inv = a.inv(&p);
        let prod = a.mul(&a_inv, &p);
        assert_eq!(
            prod, one,
            "a * inv(a) != 1 for a={v:#x} in GF(2^8)"
        );
    }
}

/// `inv(one) == one` in GF(2^4).
#[test]
fn inv_one_is_one_gf4() {
    let p = poly4();
    let one = F2mNaive::<1>::one();
    assert_eq!(one.inv(&p), one, "inv(1) != 1 in GF(2^4)");
}

/// `inv(one) == one` in GF(2^8).
#[test]
fn inv_one_is_one_gf8() {
    let p = poly8();
    let one = F2mNaive::<1>::one();
    assert_eq!(one.inv(&p), one, "inv(1) != 1 in GF(2^8)");
}

/// `inv(zero)` panics — zero has no multiplicative inverse.
#[test]
#[should_panic]
fn inv_zero_panics() {
    let p = poly4();
    let zero = F2mNaive::<1>::zero();
    let _ = zero.inv(&p);
}

// ── Extended-Euclidean ↔ Itoh–Tsujii agreement ───────────────────────────────

/// Both inversion algorithms agree for all non-zero elements of GF(2^4).
///
/// This is the cross-check: a bug in `square`/`frobenius` shows up in
/// Itoh–Tsujii but not in extended-Euclidean, and vice versa for a bug in
/// polynomial division.
#[test]
fn ext_euclid_itoh_tsujii_agree_gf4() {
    let p = poly4();
    for v in 1u64..16 {
        let a = F2mNaive::<1>::from_u64(v, &p);
        let ee = ext_euclid_inv(&a, &p);
        let it = itoh_tsujii_inv(&a, &p);
        assert_eq!(
            ee, it,
            "ext_euclid_inv != itoh_tsujii_inv for a={v:#x} in GF(2^4)"
        );
    }
}

/// Both inversion algorithms agree for all non-zero elements of GF(2^8).
///
/// Exhaustive over all 255 non-zero elements.
#[test]
fn ext_euclid_itoh_tsujii_agree_gf8() {
    let p = poly8();
    for v in 1u64..256 {
        let a = F2mNaive::<1>::from_u64(v, &p);
        let ee = ext_euclid_inv(&a, &p);
        let it = itoh_tsujii_inv(&a, &p);
        assert_eq!(
            ee, it,
            "ext_euclid_inv != itoh_tsujii_inv for a={v:#x} in GF(2^8)"
        );
    }
}

// ── Division ──────────────────────────────────────────────────────────────────

/// `div(a, b) == mul(a, inv(b))` for all `a` and non-zero `b` in GF(2^4).
#[test]
fn div_equals_mul_inv_gf4() {
    let p = poly4();
    for av in 0u64..16 {
        for bv in 1u64..16 {
            let a = F2mNaive::<1>::from_u64(av, &p);
            let b = F2mNaive::<1>::from_u64(bv, &p);
            let lhs = a.div(&b, &p);
            let rhs = a.mul(&b.inv(&p), &p);
            assert_eq!(
                lhs, rhs,
                "div(a,b) != mul(a,inv(b)) for a={av:#x} b={bv:#x} in GF(2^4)"
            );
        }
    }
}

/// `div(a, b) == mul(a, inv(b))` for a representative set in GF(2^8).
#[test]
fn div_equals_mul_inv_gf8() {
    let p = poly8();
    let elems: Vec<u64> = vec![0, 1, 2, 3, 0x53, 0xca, 0x8d, 0xf6, 0x01, 0xff, 0x1b, 0xe5];
    for &av in &elems {
        for &bv in &elems {
            if bv == 0 {
                continue; // skip zero divisor
            }
            let a = F2mNaive::<1>::from_u64(av, &p);
            let b = F2mNaive::<1>::from_u64(bv, &p);
            let lhs = a.div(&b, &p);
            let rhs = a.mul(&b.inv(&p), &p);
            assert_eq!(
                lhs, rhs,
                "div(a,b) != mul(a,inv(b)) for a={av:#x} b={bv:#x} in GF(2^8)"
            );
        }
    }
}

/// `div(zero, b) == zero` for non-zero `b` in GF(2^4).
#[test]
fn div_zero_numerator_is_zero() {
    let p = poly4();
    let zero = F2mNaive::<1>::zero();
    for bv in 1u64..16 {
        let b = F2mNaive::<1>::from_u64(bv, &p);
        assert_eq!(
            zero.div(&b, &p),
            zero,
            "0 / b != 0 for b={bv:#x} in GF(2^4)"
        );
    }
}

/// `div(zero, b)` panics when `b` is zero.
#[test]
#[should_panic]
fn div_zero_denominator_panics() {
    let p = poly4();
    let a = F2mNaive::<1>::from_u64(0x5, &p);
    let zero = F2mNaive::<1>::zero();
    let _ = a.div(&zero, &p);
}

// ── AES inversion KATs ────────────────────────────────────────────────────────

/// AES GF(2^8): `inv(0x53) == 0xca` (known AES S-box inverse pair).
#[test]
fn aes_inv_known_pair() {
    let p = poly8();
    let a = F2mNaive::<1>::from_u64(0x53, &p);
    let b = F2mNaive::<1>::from_u64(0xca, &p);
    assert_eq!(a.inv(&p), b, "inv(0x53) != 0xca in AES GF(2^8)");
    assert_eq!(b.inv(&p), a, "inv(0xca) != 0x53 in AES GF(2^8)");
}
