//! Cantor's algorithm: the Jacobian group law for hyperelliptic curves over GF(2^m).
//!
//! This module implements the Jacobian group law on the hyperelliptic curve
//! `y²+h(x)y=f(x)` over GF(2^m) via Cantor's algorithm (compose + reduce).
//!
//! # Group law
//!
//! The group elements are reduced Mumford divisors `[u(x), v(x)]` with:
//! - `u` monic, `deg u ≤ g`,
//! - `deg v < deg u`,
//! - `u | (f − v·h − v²)` (curve-compatibility).
//!
//! The identity is `[1, 0]`.  Addition is Cantor compose followed by Cantor reduce.
//!
//! # Characteristic-2 negation trap
//!
//! The negation of `[u, v]` is `[u, (h+v) mod u]`, **not** `[u, −v]`.
//! In char 2, `−v = v`, but the hyperelliptic involution sends
//! `(x, y) → (x, −y−h(x)) = (x, y+h(x))`, so the divisor-level negation
//! reflects `v → h+v` (reduced mod `u`).  The `D+(−D)=0` KAT guards this.
//!
//! # Cantor compose algorithm
//!
//! Given `D₁ = [u₁, v₁]`, `D₂ = [u₂, v₂]`:
//! 1. `(d₁, e₁, e₂) = xgcd(u₁, u₂)` so `e₁·u₁ + e₂·u₂ = d₁`.
//! 2. `(d, c₁, c₂) = xgcd(d₁, v₁+v₂+h)` so `c₁·d₁ + c₂·(v₁+v₂+h) = d`.
//! 3. `s₁ = c₁·e₁`, `s₂ = c₁·e₂`, `s₃ = c₂`.
//! 4. `u = u₁·u₂/d²` (exact), `v = (s₁·u₁·v₂ + s₂·u₂·v₁ + s₃·(v₁·v₂+f)) mod u`, monic.
//!
//! # Cantor reduce algorithm
//!
//! Repeat until `deg u ≤ g`:
//! - `u' = (f − v·h − v²) / u` (exact division; `u | f−v·h−v²` by the Mumford invariant).
//! - `v' = (h + v) mod u'` (char 2: `−h−v = h+v`).
//! - Make `u'` monic.
//!
//! # References
//!
//! Cantor, D.G. (1987). "Computing in the Jacobian of a hyperelliptic curve."
//! Mathematics of Computation 48(177), 95–101.

use crypto_bigint::Uint;

use shared_gf2m::F2m;

use crate::hyperelliptic::{HyperellipticCurve, MumfordDivisor};
use shared_gf2m::Poly;

// ── Cantor compose ────────────────────────────────────────────────────────────

/// Compose two Mumford divisors via Cantor's algorithm (step 1 of addition).
///
/// Returns an (unreduced) Mumford divisor `[u, v]` whose degree may exceed `g`.
/// Always follow with [`reduce`] to obtain a canonical group element.
///
/// # Arguments
///
/// * `curve` — the hyperelliptic curve (provides `h` and `f`).
/// * `d1`, `d2` — the two input divisors.
/// * `poly` — the GF(2^m) irreducible.
pub fn compose<F: F2m<L>, const L: usize>(
    curve: &HyperellipticCurve<L>,
    d1: &MumfordDivisor<F, L>,
    d2: &MumfordDivisor<F, L>,
    poly: &Uint<L>,
) -> MumfordDivisor<F, L> {
    // Identity short-circuits.
    if d1.is_zero() {
        return d2.clone();
    }
    if d2.is_zero() {
        return d1.clone();
    }

    let h = curve.h::<F>();
    let f = curve.f::<F>();

    let u1 = &d1.u;
    let v1 = &d1.v;
    let u2 = &d2.u;
    let v2 = &d2.v;

    // Step 1: d₁ = gcd(u₁, u₂), with Bézout e₁·u₁ + e₂·u₂ = d₁.
    let (d1_poly, e1, e2) = Poly::xgcd(u1, u2, poly);

    // Step 2: d = gcd(d₁, v₁+v₂+h), with Bézout c₁·d₁ + c₂·(v₁+v₂+h) = d.
    let v_sum_h = v1.add(v2).add(&h); // v₁ + v₂ + h  (char-2: add = XOR)
    let (d, c1, c2) = Poly::xgcd(&d1_poly, &v_sum_h, poly);

    // Step 3: s₁ = c₁·e₁, s₂ = c₁·e₂, s₃ = c₂.
    let s1 = c1.mul(&e1, poly);
    let s2 = c1.mul(&e2, poly);
    let s3 = c2;

    // Step 4: u = u₁·u₂/d².
    let u1u2 = u1.mul(u2, poly);
    let d_sq = d.mul(&d, poly);
    let (u_new, rem_u) = u1u2.divmod(&d_sq, poly);
    debug_assert!(rem_u.is_zero(), "compose: u₁·u₂ not divisible by d²");

    // v = (s₁·u₁·v₂ + s₂·u₂·v₁ + s₃·(v₁·v₂+f)) mod u_new.
    let s1_u1_v2 = s1.mul(u1, poly).mul(v2, poly);
    let s2_u2_v1 = s2.mul(u2, poly).mul(v1, poly);
    let v1v2_f = v1.mul(v2, poly).add(&f); // v₁·v₂ + f  (char-2: add = XOR)
    let s3_term = s3.mul(&v1v2_f, poly);
    let v_unreduced = s1_u1_v2.add(&s2_u2_v1).add(&s3_term);
    let (_, v_new) = v_unreduced.divmod(&u_new, poly);

    // Make u monic.
    let u_monic = u_new.monic(poly);
    // Scale v consistently: if u was scaled by lc_inv, v must be scaled too.
    // Since u_new.monic() scales by lc(u_new)⁻¹, we need to scale v the same way.
    // But v is already reduced mod u_new (not u_monic), so we just return as-is;
    // the degree bound deg v < deg u is preserved by the mod operation.
    // We do NOT scale v — the Mumford pair [u_monic, v_new] is valid because
    // making u monic does not change the divisibility condition (u_monic = c·u_new
    // for a scalar c, and c·u_new | f−v·h−v² iff u_new | f−v·h−v²).
    MumfordDivisor::new(u_monic, v_new)
}

// ── Cantor reduce ─────────────────────────────────────────────────────────────

/// Reduce a Mumford divisor to canonical form via Cantor's reduction step.
///
/// Repeatedly replaces `[u, v]` with `[u', v']` where:
/// - `u' = (f − v·h − v²) / u` (exact division),
/// - `v' = (h + v) mod u'` (char-2 negation: `−h−v = h+v`),
/// - `u'` made monic.
///
/// Terminates when `deg u ≤ g`.  The input need not be reduced.
///
/// # Arguments
///
/// * `curve` — the hyperelliptic curve (provides `h`, `f`, and genus `g`).
/// * `div` — the (possibly unreduced) divisor.
/// * `poly` — the GF(2^m) irreducible.
pub fn reduce<F: F2m<L>, const L: usize>(
    curve: &HyperellipticCurve<L>,
    div: &MumfordDivisor<F, L>,
    poly: &Uint<L>,
) -> MumfordDivisor<F, L> {
    let g = curve.genus();
    let h = curve.h::<F>();
    let f = curve.f::<F>();

    let mut u = div.u.clone();
    let mut v = div.v.clone();

    loop {
        let deg_u = match u.degree() {
            Some(d) => d,
            None => break, // u = 0 is degenerate; stop
        };
        if deg_u <= g {
            break;
        }

        // u' = (f − v·h − v²) / u.
        // In char 2: f − v·h − v² = f + v·h + v².
        let v_sq = v.mul(&v, poly);
        let vh = v.mul(&h, poly);
        let numerator = f.add(&vh).add(&v_sq); // f + v·h + v²
        let (u_new, rem) = numerator.divmod(&u, poly);
        debug_assert!(rem.is_zero(), "reduce: (f+v·h+v²) not divisible by u");

        // v' = (h + v) mod u'  (char-2: −h−v = h+v).
        let hv = h.add(&v);
        let (_, v_new) = hv.divmod(&u_new, poly);

        // Make u' monic.
        u = u_new.monic(poly);
        v = v_new;
    }

    MumfordDivisor::new(u, v)
}

// ── Public group-law API ──────────────────────────────────────────────────────

/// Add two Mumford divisors: Cantor compose followed by Cantor reduce.
///
/// Returns a valid reduced divisor `[u, v]` with `deg u ≤ g`.
///
/// This is the Jacobian group law.  The result is always a canonical group
/// element (reduced divisor).
pub fn add<F: F2m<L>, const L: usize>(
    curve: &HyperellipticCurve<L>,
    d1: &MumfordDivisor<F, L>,
    d2: &MumfordDivisor<F, L>,
    poly: &Uint<L>,
) -> MumfordDivisor<F, L> {
    let composed = compose(curve, d1, d2, poly);
    reduce(curve, &composed, poly)
}

/// Negate a Mumford divisor: `−[u, v] = [u, (h+v) mod u]`.
///
/// In char 2, the hyperelliptic involution sends `(x, y) → (x, y+h(x))`,
/// so the divisor-level negation is `v → h+v` (reduced mod `u`).
///
/// **Not** `[u, −v]` — in char 2 `−v = v`, which would give the identity
/// `D + (−D) = D + D ≠ 0` for most `D`.
pub fn negate<F: F2m<L>, const L: usize>(
    curve: &HyperellipticCurve<L>,
    div: &MumfordDivisor<F, L>,
    poly: &Uint<L>,
) -> MumfordDivisor<F, L> {
    if div.is_zero() {
        return div.clone();
    }
    let h = curve.h::<F>();
    let hv = h.add(&div.v); // h + v  (char-2: add = XOR)
    let (_, v_neg) = hv.divmod(&div.u, poly);
    MumfordDivisor::new(div.u.clone(), v_neg)
}

/// Scalar multiplication: `k · D` via double-and-add over Cantor compose.
///
/// Uses the left-to-right binary method, mirroring `BinaryCurve::scalar_mul`.
/// The scalar `k` is a `u64`; for the toy genus-2 / GF(2^4) setting the group
/// order fits comfortably in 64 bits.
///
/// Returns the identity `[1, 0]` for `k = 0`.
pub fn scalar_mul<F: F2m<L>, const L: usize>(
    curve: &HyperellipticCurve<L>,
    div: &MumfordDivisor<F, L>,
    k: u64,
    poly: &Uint<L>,
) -> MumfordDivisor<F, L> {
    if k == 0 {
        return curve.zero_divisor::<F>();
    }

    let mut result: MumfordDivisor<F, L> = curve.zero_divisor::<F>();

    // Left-to-right binary method: scan bits from MSB to LSB.
    for i in (0..64).rev() {
        result = add(curve, &result, &result, poly); // double
        if (k >> i) & 1 == 1 {
            result = add(curve, &result, div, poly); // add
        }
    }

    result
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_gf2m::F2mNaive;

    type F = F2mNaive<1>;

    fn poly4() -> Uint<1> {
        Uint::<1>::from(0x13u64)
    }

    fn f4(v: u64) -> F {
        F::from_u64(v, &poly4())
    }

    /// Toy genus-2 curve: `y² + x·y = x⁵ + x³ + 1` over GF(2^4).
    fn toy_curve() -> HyperellipticCurve<1> {
        HyperellipticCurve::new(
            poly4(),
            vec![Uint::<1>::ZERO, Uint::<1>::ONE],
            vec![
                Uint::<1>::ONE,
                Uint::<1>::ZERO,
                Uint::<1>::ZERO,
                Uint::<1>::ONE,
                Uint::<1>::ZERO,
                Uint::<1>::ONE,
            ],
        )
    }

    /// Build a degree-2 divisor from two known points on the toy curve.
    fn d1() -> MumfordDivisor<F, 1> {
        let c = toy_curve();
        c.divisor_from_points::<F>(&[(f4(2), f4(8)), (f4(3), f4(12))])
    }

    /// Build a second degree-2 divisor from different points.
    fn d2() -> MumfordDivisor<F, 1> {
        let c = toy_curve();
        c.divisor_from_points::<F>(&[(f4(1), f4(6)), (f4(7), f4(1))])
    }

    #[test]
    fn identity_is_valid() {
        let c = toy_curve();
        let zero = c.zero_divisor::<F>();
        assert!(c.is_valid(&zero), "identity [1,0] must be valid");
        assert!(zero.is_zero());
    }

    #[test]
    fn add_identity_left() {
        let c = toy_curve();
        let poly = poly4();
        let zero = c.zero_divisor::<F>();
        let d = d1();
        let result = add(&c, &zero, &d, &poly);
        assert_eq!(result, d, "0 + D must equal D");
    }

    #[test]
    fn add_identity_right() {
        let c = toy_curve();
        let poly = poly4();
        let zero = c.zero_divisor::<F>();
        let d = d1();
        let result = add(&c, &d, &zero, &poly);
        assert_eq!(result, d, "D + 0 must equal D");
    }

    #[test]
    fn negate_then_add_is_zero() {
        let c = toy_curve();
        let poly = poly4();
        let d = d1();
        let neg_d = negate(&c, &d, &poly);
        let result = add(&c, &d, &neg_d, &poly);
        assert!(result.is_zero(), "D + (−D) must be the identity");
    }

    #[test]
    fn double_via_add_matches_scalar_mul_2() {
        let c = toy_curve();
        let poly = poly4();
        let d = d1();
        let double_add = add(&c, &d, &d, &poly);
        let double_scalar = scalar_mul(&c, &d, 2, &poly);
        assert_eq!(double_add, double_scalar, "D+D must equal 2·D via scalar_mul");
    }

    #[test]
    fn result_is_valid_reduced_divisor() {
        let c = toy_curve();
        let poly = poly4();
        let d = d1();
        let d2 = d2();
        let sum = add(&c, &d, &d2, &poly);
        assert!(c.is_valid(&sum), "D₁+D₂ must be a valid reduced divisor");
    }

    #[test]
    fn scalar_mul_zero_is_identity() {
        let c = toy_curve();
        let poly = poly4();
        let d = d1();
        let result = scalar_mul(&c, &d, 0, &poly);
        assert!(result.is_zero(), "0·D must be the identity");
    }

    #[test]
    fn scalar_mul_one_is_identity_element() {
        let c = toy_curve();
        let poly = poly4();
        let d = d1();
        let result = scalar_mul(&c, &d, 1, &poly);
        assert_eq!(result, d, "1·D must equal D");
    }
}
