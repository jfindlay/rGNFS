//! Known-answer tests for the GF(2^m)[x] polynomial ring (`Poly<F, L>`).
//!
//! All tests use GF(2^4) with irreducible `x⁴+x+1` (poly = 0x13), mirroring
//! the binary-curve KATs.  The algorithms are not toy — they are correct for
//! arbitrary `m` and arbitrary polynomial degree — but the parameters are small
//! for auditability (principle-4 boundary).
//!
//! # Coverage
//!
//! - **Ring axioms**: commutativity, associativity, distributivity of `add`/`mul`.
//! - **Char-2 `add` invariants**: `a + a = 0`; `add == sub`.
//! - **`divmod` round-trip**: `a = q·b + r` with `deg r < deg b`.
//! - **`monic`**: leading coefficient is 1 after `monic`.
//! - **`gcd`**: `gcd(a, b)` divides both `a` and `b`; `gcd(a·c, b·c) = c·gcd(a,b)·unit`.
//! - **`xgcd`**: Bézout identity `s·a + t·b = gcd(a, b)`.
//! - **`derivative`** (char-2 correctness): `(x²)' = 0`, `(x³)' = x²` — the
//!   even-degree-killing trap.
//! - **`resultant`**: `res(a, b) = 0` iff `gcd` is nontrivial.
//! - **`mod_inverse`**: `inv * a ≡ 1 (mod m)` when `gcd(a, m) = 1`.
//!
//! The derivative KATs are the loudest correctness signal for char-2: a
//! derivative ported from integer-coefficient polynomials writes `(x²)' = 2x`
//! instead of `0`.

use crypto_bigint::Uint;
use shared_gf2m::{F2m, F2mNaive, Poly};

// ── Field parameters ──────────────────────────────────────────────────────────

/// GF(2^4) irreducible: x⁴ + x + 1 = 0x13.  Degree m = 4.
fn fp4() -> Uint<1> {
    Uint::<1>::from(0x13u64)
}

// ── Helper constructors ───────────────────────────────────────────────────────

type F = F2mNaive<1>;
type P = Poly<F, 1>;

/// Construct a GF(2^4) field element from a small integer.
fn f(v: u64) -> F {
    F::from_u64(v, &fp4())
}

/// Construct a polynomial from a slice of coefficient values (index = degree).
fn poly(coeffs: &[u64]) -> P {
    P::from_coeffs(coeffs.iter().map(|&v| f(v)).collect())
}

/// All polynomials of degree ≤ 2 over GF(2^4) with coefficients in {0,1}.
///
/// Used for exhaustive ring-axiom checks.  Restricting to {0,1} coefficients
/// keeps the count manageable (2^3 = 8 polynomials) while covering the
/// structural cases.
fn small_polys() -> Vec<P> {
    let mut result = Vec::new();
    for c0 in 0u64..2 {
        for c1 in 0u64..2 {
            for c2 in 0u64..2 {
                result.push(poly(&[c0, c1, c2]));
            }
        }
    }
    result
}

/// A richer set of polynomials for ring-axiom checks, including non-binary
/// coefficients drawn from GF(2^4).
fn sample_polys() -> Vec<P> {
    vec![
        poly(&[]),          // 0
        poly(&[1]),         // 1
        poly(&[0, 1]),      // x
        poly(&[1, 1]),      // x + 1
        poly(&[0, 0, 1]),   // x²
        poly(&[1, 0, 1]),   // x² + 1
        poly(&[1, 1, 1]),   // x² + x + 1
        poly(&[0, 0, 0, 1]), // x³
        poly(&[1, 1, 0, 1]), // x³ + x + 1
        // Non-binary coefficients (field elements from GF(2^4)).
        poly(&[2, 3]),      // 3x + 2
        poly(&[5, 0, 7]),   // 7x² + 5
        poly(&[1, 2, 3, 4]), // 4x³ + 3x² + 2x + 1
    ]
}

// ── Char-2 add invariants ─────────────────────────────────────────────────────

/// `a + a = 0` for all sample polynomials.
///
/// In characteristic 2, every element is its own additive inverse.
#[test]
fn add_self_is_zero() {
    for a in sample_polys() {
        assert_eq!(
            a.add(&a),
            P::zero(),
            "a + a != 0 for a = {:?}",
            a
        );
    }
}

/// `add(a, b) == sub(a, b)` for all pairs in the small set.
///
/// In characteristic 2, subtraction equals addition (XOR).
#[test]
fn sub_equals_add() {
    for a in small_polys() {
        for b in small_polys() {
            assert_eq!(
                a.add(&b),
                a.sub(&b),
                "add != sub for a={:?} b={:?}",
                a, b
            );
        }
    }
}

// ── Ring axioms: commutativity ────────────────────────────────────────────────

/// `add(a, b) == add(b, a)` for all pairs in the small set.
#[test]
fn add_commutative() {
    for a in small_polys() {
        for b in small_polys() {
            assert_eq!(
                a.add(&b),
                b.add(&a),
                "add not commutative: a={:?} b={:?}",
                a, b
            );
        }
    }
}

/// `mul(a, b) == mul(b, a)` for all pairs in the small set.
#[test]
fn mul_commutative() {
    let p = fp4();
    for a in small_polys() {
        for b in small_polys() {
            assert_eq!(
                a.mul(&b, &p),
                b.mul(&a, &p),
                "mul not commutative: a={:?} b={:?}",
                a, b
            );
        }
    }
}

// ── Ring axioms: associativity ────────────────────────────────────────────────

/// `add(add(a,b),c) == add(a,add(b,c))` for all triples in the small set.
#[test]
fn add_associative() {
    for a in small_polys() {
        for b in small_polys() {
            for c in small_polys() {
                let lhs = a.add(&b).add(&c);
                let rhs = a.add(&b.add(&c));
                assert_eq!(
                    lhs, rhs,
                    "add not associative: a={:?} b={:?} c={:?}",
                    a, b, c
                );
            }
        }
    }
}

/// `mul(mul(a,b),c) == mul(a,mul(b,c))` for all triples in the small set.
#[test]
fn mul_associative() {
    let p = fp4();
    for a in small_polys() {
        for b in small_polys() {
            for c in small_polys() {
                let lhs = a.mul(&b, &p).mul(&c, &p);
                let rhs = a.mul(&b.mul(&c, &p), &p);
                assert_eq!(
                    lhs, rhs,
                    "mul not associative: a={:?} b={:?} c={:?}",
                    a, b, c
                );
            }
        }
    }
}

// ── Ring axioms: distributivity ───────────────────────────────────────────────

/// `mul(a, add(b,c)) == add(mul(a,b), mul(a,c))` for all triples in the small set.
#[test]
fn mul_distributive_over_add() {
    let p = fp4();
    for a in small_polys() {
        for b in small_polys() {
            for c in small_polys() {
                let lhs = a.mul(&b.add(&c), &p);
                let rhs = a.mul(&b, &p).add(&a.mul(&c, &p));
                assert_eq!(
                    lhs, rhs,
                    "distributivity failed: a={:?} b={:?} c={:?}",
                    a, b, c
                );
            }
        }
    }
}

// ── Ring axioms: identity elements ────────────────────────────────────────────

/// `mul(a, 1) == a` for all sample polynomials.
#[test]
fn mul_by_one_is_identity() {
    let p = fp4();
    let one = P::one();
    for a in sample_polys() {
        assert_eq!(
            a.mul(&one, &p),
            a,
            "a * 1 != a for a={:?}",
            a
        );
    }
}

/// `add(a, 0) == a` for all sample polynomials.
#[test]
fn add_zero_is_identity() {
    let zero = P::zero();
    for a in sample_polys() {
        assert_eq!(
            a.add(&zero),
            a,
            "a + 0 != a for a={:?}",
            a
        );
    }
}

/// `mul(a, 0) == 0` for all sample polynomials.
#[test]
fn mul_by_zero_is_zero() {
    let p = fp4();
    let zero = P::zero();
    for a in sample_polys() {
        assert_eq!(
            a.mul(&zero, &p),
            zero,
            "a * 0 != 0 for a={:?}",
            a
        );
    }
}

// ── divmod round-trip ─────────────────────────────────────────────────────────

/// `a = q·b + r` with `deg r < deg b` for a representative set of pairs.
///
/// This is the primary `divmod` correctness check.
#[test]
fn divmod_round_trip() {
    let p = fp4();
    // Dividend / divisor pairs: (a, b) with b ≠ 0.
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 0, 1], &[1, 1]),         // x² / (x+1)
        (&[1, 0, 0, 1], &[1, 1]),      // x³+1 / (x+1)
        (&[1, 1, 1, 1], &[1, 0, 1]),   // x³+x²+x+1 / (x²+1)
        (&[0, 0, 0, 0, 1], &[1, 1]),   // x⁴ / (x+1)
        (&[1, 2, 3, 4], &[1, 1]),      // 4x³+3x²+2x+1 / (x+1)
        (&[5, 0, 7], &[2, 3]),         // 7x²+5 / (3x+2)
        (&[1, 0, 0, 1], &[1]),         // x³+1 / 1 (constant divisor)
        (&[1, 1, 1], &[1, 1, 1]),      // a / a = 1, remainder 0
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let (q, r) = a.divmod(&b, &p);
        // Round-trip: q * b + r = a.
        let reconstructed = q.mul(&b, &p).add(&r);
        assert_eq!(
            reconstructed, a,
            "divmod round-trip failed: a={:?} b={:?} q={:?} r={:?}",
            a, b, q, r
        );
        // Degree bound: deg(r) < deg(b), or r = 0.
        if !r.is_zero() {
            assert!(
                r.degree().unwrap() < b.degree().unwrap(),
                "divmod degree bound violated: deg(r)={:?} >= deg(b)={:?}",
                r.degree(),
                b.degree()
            );
        }
    }
}

/// `divmod` with a non-monic divisor: leading coefficient is inverted correctly.
#[test]
fn divmod_non_monic_divisor() {
    let p = fp4();
    // Divisor with leading coefficient 3 (non-monic).
    let a = poly(&[1, 0, 0, 1]); // x³ + 1
    let b = poly(&[1, 0, 3]); // 3x² + 1
    let (q, r) = a.divmod(&b, &p);
    let reconstructed = q.mul(&b, &p).add(&r);
    assert_eq!(
        reconstructed, a,
        "divmod non-monic divisor: round-trip failed"
    );
    if !r.is_zero() {
        assert!(r.degree().unwrap() < b.degree().unwrap());
    }
}

// ── monic ─────────────────────────────────────────────────────────────────────

/// `monic(a).leading_coeff() == 1` for all nonzero sample polynomials.
#[test]
fn monic_leading_coeff_is_one() {
    let p = fp4();
    let one = f(1);
    for a in sample_polys() {
        if a.is_zero() {
            continue;
        }
        let m = a.monic(&p);
        assert_eq!(
            m.leading_coeff().unwrap(),
            &one,
            "monic: leading coeff != 1 for a={:?}",
            a
        );
    }
}

/// `monic(a)` and `a` are proportional: `monic(a) * lc(a) = a`.
#[test]
fn monic_proportional_to_original() {
    let p = fp4();
    for a in sample_polys() {
        if a.is_zero() {
            continue;
        }
        let lc = a.leading_coeff().unwrap().clone();
        let m = a.monic(&p);
        // m * lc = a.
        assert_eq!(
            m.scale(&lc, &p),
            a,
            "monic: m * lc != a for a={:?}",
            a
        );
    }
}

// ── gcd ───────────────────────────────────────────────────────────────────────

/// `gcd(a, b)` divides both `a` and `b`.
#[test]
fn gcd_divides_both() {
    let p = fp4();
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),         // gcd(x, x+1) = 1
        (&[1, 0, 1], &[1, 1]),      // gcd(x²+1, x+1) = x+1
        (&[0, 1, 1], &[1, 0, 1]),   // gcd(x²+x, x²+1)
        (&[1, 1, 0, 1], &[1, 0, 1]), // gcd(x³+x+1, x²+1)
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let g = P::gcd(&a, &b, &p);
        // g divides a: a mod g = 0.
        let (_, ra) = a.divmod(&g, &p);
        assert_eq!(
            ra,
            P::zero(),
            "gcd does not divide a: a={:?} b={:?} g={:?}",
            a, b, g
        );
        // g divides b: b mod g = 0.
        let (_, rb) = b.divmod(&g, &p);
        assert_eq!(
            rb,
            P::zero(),
            "gcd does not divide b: a={:?} b={:?} g={:?}",
            a, b, g
        );
    }
}

/// `gcd(a·c, b·c) = c · gcd(a, b)` (up to unit / monic normalization).
///
/// Specifically, `gcd(a·c, b·c)` should be `monic(c) * gcd(a, b)` when both
/// sides are made monic.
#[test]
fn gcd_common_factor() {
    let p = fp4();
    let a = poly(&[0, 1]); // x
    let b = poly(&[1, 1]); // x + 1
    let c = poly(&[1, 0, 1]); // x² + 1
    let ac = a.mul(&c, &p);
    let bc = b.mul(&c, &p);
    let g_ab = P::gcd(&a, &b, &p); // gcd(x, x+1) = 1
    let g_acbc = P::gcd(&ac, &bc, &p); // gcd(x(x²+1), (x+1)(x²+1)) = x²+1
    // g_acbc should equal c * g_ab = (x²+1) * 1 = x²+1 (monic).
    let expected = c.mul(&g_ab, &p).monic(&p);
    assert_eq!(
        g_acbc, expected,
        "gcd(a*c, b*c) != c * gcd(a,b): a={:?} b={:?} c={:?}",
        a, b, c
    );
}

/// `gcd(a, a) = monic(a)`.
#[test]
fn gcd_self() {
    let p = fp4();
    for a in sample_polys() {
        if a.is_zero() {
            continue;
        }
        let g = P::gcd(&a, &a, &p);
        assert_eq!(
            g,
            a.monic(&p),
            "gcd(a, a) != monic(a) for a={:?}",
            a
        );
    }
}

/// `gcd(a, 0) = monic(a)` and `gcd(0, a) = monic(a)`.
#[test]
fn gcd_with_zero() {
    let p = fp4();
    let a = poly(&[1, 1, 1]); // x² + x + 1
    assert_eq!(P::gcd(&a, &P::zero(), &p), a.monic(&p));
    assert_eq!(P::gcd(&P::zero(), &a, &p), a.monic(&p));
}

// ── xgcd ─────────────────────────────────────────────────────────────────────

/// Bézout identity: `s·a + t·b = gcd(a, b)` for a representative set.
#[test]
fn xgcd_bezout_identity() {
    let p = fp4();
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),         // x, x+1
        (&[1, 0, 1], &[1, 1]),      // x²+1, x+1
        (&[0, 0, 0, 1], &[1, 1]),   // x³, x+1
        (&[1, 1, 1], &[1, 0, 1]),   // x²+x+1, x²+1
        (&[1, 2, 3, 4], &[1, 1]),   // 4x³+3x²+2x+1, x+1
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let (g, s, t) = P::xgcd(&a, &b, &p);
        let lhs = s.mul(&a, &p).add(&t.mul(&b, &p));
        assert_eq!(
            lhs, g,
            "Bézout identity s*a + t*b = gcd failed: a={:?} b={:?} g={:?} s={:?} t={:?}",
            a, b, g, s, t
        );
    }
}

/// `xgcd` returns a monic GCD.
#[test]
fn xgcd_gcd_is_monic() {
    let p = fp4();
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),
        (&[1, 0, 1], &[1, 1]),
        (&[1, 2, 3, 4], &[1, 1]),
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let (g, _, _) = P::xgcd(&a, &b, &p);
        if !g.is_zero() {
            assert_eq!(
                g.leading_coeff().unwrap(),
                &f(1),
                "xgcd: gcd is not monic for a={:?} b={:?}",
                a, b
            );
        }
    }
}

// ── derivative (char-2 correctness) ──────────────────────────────────────────

/// `(x²)' = 0` — the char-2 trap.
///
/// In characteristic 2, the formal derivative kills even-degree terms.
/// A derivative ported from integer-coefficient polynomials writes `(x²)' = 2x`
/// instead of `0`.  This is the loudest signal.
#[test]
fn derivative_x_squared_is_zero() {
    let x2 = poly(&[0, 0, 1]); // x²
    assert_eq!(
        x2.derivative(),
        P::zero(),
        "(x²)' should be 0 in characteristic 2"
    );
}

/// `(x⁴)' = 0` — even degree.
#[test]
fn derivative_x4_is_zero() {
    let x4 = poly(&[0, 0, 0, 0, 1]); // x⁴
    assert_eq!(
        x4.derivative(),
        P::zero(),
        "(x⁴)' should be 0 in characteristic 2"
    );
}

/// `(x³)' = x²` — odd degree survives.
#[test]
fn derivative_x_cubed_is_x_squared() {
    let x3 = poly(&[0, 0, 0, 1]); // x³
    let x2 = poly(&[0, 0, 1]); // x²
    assert_eq!(
        x3.derivative(),
        x2,
        "(x³)' should be x² in characteristic 2"
    );
}

/// `(x)' = 1` — degree-1 term survives.
#[test]
fn derivative_x_is_one() {
    let x = poly(&[0, 1]); // x
    assert_eq!(
        x.derivative(),
        P::one(),
        "(x)' should be 1"
    );
}

/// `(c)' = 0` for any constant `c`.
#[test]
fn derivative_constant_is_zero() {
    for v in [1u64, 2, 5, 7, 15] {
        let c = poly(&[v]);
        assert_eq!(
            c.derivative(),
            P::zero(),
            "({v})' should be 0 (constant)"
        );
    }
}

/// `(x³ + x² + x + 1)' = x² + 1`.
///
/// Odd terms: x³ → x², x → 1.  Even terms: x² → 0, 1 → 0.
/// Result: x² + 1.
#[test]
fn derivative_mixed_polynomial() {
    let a = poly(&[1, 1, 1, 1]); // x³ + x² + x + 1
    let expected = poly(&[1, 0, 1]); // x² + 1
    assert_eq!(
        a.derivative(),
        expected,
        "(x³+x²+x+1)' should be x²+1 in characteristic 2"
    );
}

/// Linearity of the derivative: `(a + b)' = a' + b'`.
#[test]
fn derivative_is_linear() {
    for a in small_polys() {
        for b in small_polys() {
            let lhs = a.add(&b).derivative();
            let rhs = a.derivative().add(&b.derivative());
            assert_eq!(
                lhs, rhs,
                "derivative not linear: a={:?} b={:?}",
                a, b
            );
        }
    }
}

/// Product rule: `(a·b)' = a'·b + a·b'` for sample pairs.
///
/// This is the Leibniz rule, which holds in any commutative ring.
#[test]
fn derivative_product_rule() {
    let p = fp4();
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),       // x, x+1
        (&[1, 0, 1], &[0, 1]),    // x²+1, x
        (&[1, 1, 1], &[1, 1]),    // x²+x+1, x+1
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let ab = a.mul(&b, &p);
        let lhs = ab.derivative();
        let rhs = a.derivative().mul(&b, &p).add(&a.mul(&b.derivative(), &p));
        assert_eq!(
            lhs, rhs,
            "product rule failed: a={:?} b={:?}",
            a, b
        );
    }
}

// ── resultant ─────────────────────────────────────────────────────────────────

/// `res(a, b) = 0` iff `gcd(a, b)` is nontrivial (degree ≥ 1).
#[test]
fn resultant_zero_iff_gcd_nontrivial() {
    let p = fp4();
    // Pairs with nontrivial GCD.
    let nontrivial: &[(&[u64], &[u64])] = &[
        (&[0, 1, 1], &[1, 0, 1]),   // gcd(x²+x, x²+1) = x+1
        (&[1, 0, 1], &[1, 1]),      // gcd(x²+1, x+1) = x+1
        (&[0, 0, 1, 1], &[0, 1, 1]), // gcd(x³+x², x²+x) = x²+x (or x)
    ];
    for &(av, bv) in nontrivial {
        let a = poly(av);
        let b = poly(bv);
        let r = P::resultant(&a, &b, &p);
        assert!(
            r.is_zero(),
            "res(a,b) should be 0 when gcd is nontrivial: a={:?} b={:?} res={:?}",
            a, b, r
        );
    }
    // Pairs with trivial GCD (coprime).
    let coprime: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),         // gcd(x, x+1) = 1
        (&[1, 1, 1], &[1, 1]),      // gcd(x²+x+1, x+1) — check
        (&[0, 0, 0, 1], &[1, 1]),   // gcd(x³, x+1) = 1
    ];
    for &(av, bv) in coprime {
        let a = poly(av);
        let b = poly(bv);
        let g = P::gcd(&a, &b, &p);
        // Only assert nonzero resultant when gcd is actually 1.
        if g == P::one() {
            let r = P::resultant(&a, &b, &p);
            assert!(
                !r.is_zero(),
                "res(a,b) should be nonzero when gcd=1: a={:?} b={:?}",
                a, b
            );
        }
    }
}

/// Known-answer: `res(x, x+1)` is nonzero (they are coprime).
#[test]
fn resultant_x_and_xp1_nonzero() {
    let p = fp4();
    let x = poly(&[0, 1]);
    let xp1 = poly(&[1, 1]);
    let r = P::resultant(&x, &xp1, &p);
    assert!(!r.is_zero(), "res(x, x+1) should be nonzero (coprime)");
}

/// Known-answer: `res(x, x) = 0` (they share a common factor).
#[test]
fn resultant_x_and_x_is_zero() {
    let p = fp4();
    let x = poly(&[0, 1]);
    let r = P::resultant(&x, &x, &p);
    assert!(r.is_zero(), "res(x, x) should be 0 (common factor)");
}

/// `res(a, b) = res(b, a)` (symmetry, up to sign — in char 2, sign = 1).
#[test]
fn resultant_symmetric() {
    let p = fp4();
    let pairs: &[(&[u64], &[u64])] = &[
        (&[0, 1], &[1, 1]),
        (&[1, 0, 1], &[1, 1]),
        (&[0, 0, 0, 1], &[1, 1]),
    ];
    for &(av, bv) in pairs {
        let a = poly(av);
        let b = poly(bv);
        let rab = P::resultant(&a, &b, &p);
        let rba = P::resultant(&b, &a, &p);
        // In char 2, res(a,b) = res(b,a) (sign factor (-1)^(deg_a*deg_b) = 1).
        assert_eq!(
            rab, rba,
            "resultant not symmetric: a={:?} b={:?}",
            a, b
        );
    }
}

// ── mod_inverse ───────────────────────────────────────────────────────────────

/// `inv * a ≡ 1 (mod m)` when `gcd(a, m) = 1`.
#[test]
fn mod_inverse_round_trip() {
    let p = fp4();
    // Modulus: x² + x + 1 (irreducible over GF(2)).
    let m = poly(&[1, 1, 1]);
    // Elements coprime to m.
    let elements: &[&[u64]] = &[
        &[0, 1],    // x
        &[1, 1],    // x + 1
        &[1],       // 1
    ];
    for &av in elements {
        let a = poly(av);
        let inv = a.mod_inverse(&m, &p);
        assert!(
            inv.is_some(),
            "mod_inverse should exist for a={:?} mod m={:?}",
            a, m
        );
        let inv = inv.unwrap();
        // Verify: inv * a mod m = 1.
        let prod = inv.mul(&a, &p);
        let (_, r) = prod.divmod(&m, &p);
        assert_eq!(
            r,
            P::one(),
            "inv * a mod m != 1: a={:?} inv={:?} m={:?}",
            a, inv, m
        );
    }
}

/// `mod_inverse` returns `None` when `gcd(a, m) ≠ 1`.
#[test]
fn mod_inverse_none_when_not_coprime() {
    let p = fp4();
    // m = x² + 1 = (x+1)² in char 2.
    let m = poly(&[1, 0, 1]);
    // a = x + 1 shares a factor with m.
    let a = poly(&[1, 1]);
    let inv = a.mod_inverse(&m, &p);
    assert!(
        inv.is_none(),
        "mod_inverse should be None when gcd(a,m) != 1: a={:?} m={:?}",
        a, m
    );
}

// ── degree tracking ───────────────────────────────────────────────────────────

/// `degree()` returns `None` for the zero polynomial.
#[test]
fn degree_zero_poly_is_none() {
    assert_eq!(P::zero().degree(), None);
}

/// `degree()` is correct after `add` (no spurious trailing zeros).
#[test]
fn degree_after_add_normalized() {
    // (x² + x) + (x² + 1) = x + 1 (degree 1, not 2).
    let a = poly(&[0, 1, 1]); // x² + x
    let b = poly(&[1, 0, 1]); // x² + 1
    let c = a.add(&b);
    assert_eq!(c.degree(), Some(1), "degree after add should be 1, not 2");
}

/// `degree()` is correct after `mul`.
#[test]
fn degree_after_mul() {
    let p = fp4();
    let a = poly(&[0, 1]); // x, degree 1
    let b = poly(&[1, 1]); // x+1, degree 1
    let c = a.mul(&b, &p); // x²+x, degree 2
    assert_eq!(c.degree(), Some(2));
}
