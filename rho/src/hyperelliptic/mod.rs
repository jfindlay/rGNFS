//! Hyperelliptic curve `y²+h(x)y=f(x)` over GF(2^m) and Mumford divisor representation.
//!
//! This module defines:
//! - [`HyperellipticCurve`] — the curve `y²+h(x)y=f(x)` over GF(2^m), genus `g = ⌊(deg f − 1)/2⌋`.
//! - [`MumfordDivisor`] — a reduced divisor `[u(x), v(x)]` in Mumford representation.
//!
//! # Design
//!
//! `HyperellipticCurve` is a **parallel** to `rho::binary_curve::BinaryCurve`, not a reuse.
//! The curve stores `h` and `f` as coefficient vectors (`Vec<Uint<L>>`) and the field
//! irreducible `poly: Uint<L>`.  Group-law methods are generic over `F: F2m<L>` at the
//! method level, threading the field type per-call.  No `PhantomData<F>` is stored.
//!
//! # Characteristic-2 hyperelliptic model
//!
//! The curve equation is `y² + h(x)·y = f(x)` with `h ≠ 0`.  This is the correct
//! char-2 model (the odd-char model `y² = f(x)` has no group law in characteristic 2
//! — the same `2y` trap as the elliptic case, now on the Jacobian).
//!
//! # Mumford representation
//!
//! A reduced divisor is `[u(x), v(x)]` where:
//! - `u` is monic,
//! - `deg v < deg u ≤ g`,
//! - `u | (f − v·h − v²)` (the curve-compatibility invariant).
//!
//! The zero divisor (group identity) is `[1, 0]`.
//!
//! # Toy field sizes
//!
//! KATs use a genus-2 curve over GF(2^4) with irreducible `x⁴+x+1` (poly = 0x13).
//! The algorithms are arbitrary-genus-correct; only the parameters are toy
//! (principle-4 boundary).

use crypto_bigint::Uint;
use shared_gf2m::{F2m, Poly};

// ── Curve ─────────────────────────────────────────────────────────────────────

/// A hyperelliptic curve `y²+h(x)y=f(x)` over GF(2^m).
///
/// The curve is in the imaginary/ramified model: `deg f = 2g+1` (odd), `deg h ≤ g`.
/// Genus `g = ⌊(deg f − 1)/2⌋`.
///
/// Curve parameters `h` and `f` are stored as coefficient vectors of `Uint<L>` values
/// (raw GF(2^m) bit-vectors, unreduced).  Methods are generic over `F: F2m<L>` and
/// convert on demand.  The field irreducible `poly` is stored on the struct.
///
/// No `PhantomData<F>` — the field type is threaded per-call, mirroring `BinaryCurve`.
///
/// # Toy scale
///
/// Designed for genus-2 curves over GF(2^4) and GF(2^8).  The algorithms are
/// arbitrary-genus-correct; the parameters are toy (principle-4 boundary).
#[derive(Clone, Debug)]
pub struct HyperellipticCurve<const L: usize> {
    /// Irreducible polynomial defining GF(2^m).
    pub poly: Uint<L>,
    /// Coefficient vector of `h(x)`: `h_coeffs[i]` is the raw bit-vector of the
    /// coefficient of `x^i` in `h`.
    pub h_coeffs: Vec<Uint<L>>,
    /// Coefficient vector of `f(x)`: `f_coeffs[i]` is the raw bit-vector of the
    /// coefficient of `x^i` in `f`.
    pub f_coeffs: Vec<Uint<L>>,
}

impl<const L: usize> HyperellipticCurve<L> {
    /// Construct a hyperelliptic curve from coefficient vectors.
    ///
    /// `h_coeffs[i]` and `f_coeffs[i]` are the raw GF(2^m) bit-vectors of the
    /// coefficients of `x^i` in `h` and `f` respectively.
    ///
    /// # Panics
    ///
    /// Panics if `f_coeffs` is empty (the curve equation requires `f` to be nonzero).
    pub fn new(poly: Uint<L>, h_coeffs: Vec<Uint<L>>, f_coeffs: Vec<Uint<L>>) -> Self {
        assert!(!f_coeffs.is_empty(), "HyperellipticCurve: f must be nonzero");
        HyperellipticCurve { poly, h_coeffs, f_coeffs }
    }

    /// Return the genus `g = ⌊(deg f − 1)/2⌋`.
    ///
    /// For the imaginary/ramified model, `deg f = 2g+1`, so `g = (deg f − 1)/2`.
    pub fn genus(&self) -> usize {
        let deg_f = self.f_coeffs.len() - 1; // f is nonzero by construction
        (deg_f.saturating_sub(1)) / 2
    }

    /// Build the `h` polynomial as a `Poly<F, L>` over the field `F`.
    pub fn h<F: F2m<L>>(&self) -> Poly<F, L> {
        let coeffs = self.h_coeffs.iter().map(|&c| F::from_uint(c, &self.poly)).collect();
        Poly::from_coeffs(coeffs)
    }

    /// Build the `f` polynomial as a `Poly<F, L>` over the field `F`.
    pub fn f<F: F2m<L>>(&self) -> Poly<F, L> {
        let coeffs = self.f_coeffs.iter().map(|&c| F::from_uint(c, &self.poly)).collect();
        Poly::from_coeffs(coeffs)
    }

    /// Check whether the affine point `(x, y)` satisfies `y²+h(x)y = f(x)`.
    ///
    /// Returns `true` iff the point is on the curve.
    pub fn is_on_curve<F: F2m<L>>(&self, x: &F, y: &F) -> bool {
        let poly = &self.poly;
        let h = self.h::<F>();
        let f = self.f::<F>();

        // Evaluate h(x) and f(x) by Horner's method.
        let hx = eval_poly(&h, x, poly);
        let fx = eval_poly(&f, x, poly);

        // LHS = y² + h(x)·y
        let y_sq = y.square(poly);
        let hy = hx.mul(y, poly);
        let lhs = y_sq.add(&hy);

        lhs == fx
    }

    /// Check whether a `MumfordDivisor` is a valid reduced divisor for this curve.
    ///
    /// Checks:
    /// 1. `u` is monic.
    /// 2. `deg v < deg u ≤ g` (or the zero divisor `[1, 0]`).
    /// 3. `u | (f − v·h − v²)` (curve-compatibility invariant).
    pub fn is_valid<F: F2m<L>>(&self, div: &MumfordDivisor<F, L>) -> bool {
        let poly = &self.poly;
        let g = self.genus();
        let f = self.f::<F>();
        let h = self.h::<F>();

        // Check u is monic.
        match div.u.leading_coeff() {
            None => return false, // u = 0 is not valid
            Some(lc) => {
                if !lc.is_one() {
                    return false;
                }
            }
        }

        let deg_u = div.u.degree().unwrap_or(0);

        // deg u ≤ g.
        if deg_u > g {
            return false;
        }

        // deg v < deg u (or v = 0 when u = 1).
        if !div.v.is_zero() {
            let deg_v = div.v.degree().unwrap_or(0);
            if deg_v >= deg_u {
                return false;
            }
        }

        // Curve-compatibility: u | (f − v·h − v²).
        // In char 2: f − v·h − v² = f + v·h + v² (sub = add).
        let v_sq = div.v.mul(&div.v, poly);
        let vh = div.v.mul(&h, poly);
        // f + v·h + v²  (all additions are XOR in char 2)
        let rhs = f.add(&vh).add(&v_sq);
        let (_, rem) = rhs.divmod(&div.u, poly);
        rem.is_zero()
    }

    /// Construct the zero divisor `[1, 0]` (the group identity).
    pub fn zero_divisor<F: F2m<L>>(&self) -> MumfordDivisor<F, L> {
        MumfordDivisor { u: Poly::one(), v: Poly::zero() }
    }

    /// Build a `MumfordDivisor` from a list of affine points `(xᵢ, yᵢ)`.
    ///
    /// Constructs `u = Π(x − xᵢ)` (monic, degree = number of points) and
    /// `v` = the unique polynomial of degree < `len(points)` with `v(xᵢ) = yᵢ`
    /// (Lagrange interpolation).
    ///
    /// The resulting divisor satisfies the Mumford invariant provided the points
    /// are on the curve and `len(points) ≤ g`.
    ///
    /// # Panics
    ///
    /// Panics if any two points share the same x-coordinate (the divisor would
    /// not be reduced) or if `points` is empty.
    pub fn divisor_from_points<F: F2m<L>>(
        &self,
        points: &[(F, F)],
    ) -> MumfordDivisor<F, L> {
        assert!(!points.is_empty(), "divisor_from_points: points must be non-empty");
        let poly = &self.poly;

        // Build u = Π(x − xᵢ).  In char 2, x − xᵢ = x + xᵢ (sub = add).
        let mut u = Poly::one();
        for (xi, _yi) in points {
            // (x + xᵢ) = monomial x + constant xᵢ
            let linear = Poly::from_coeffs(vec![xi.clone(), F::one()]);
            u = u.mul(&linear, poly);
        }
        // u is already monic (product of monic linears).

        // Build v via Lagrange interpolation: v(xᵢ) = yᵢ, deg v < len(points).
        let v = lagrange_interpolate(points, poly);

        MumfordDivisor { u, v }
    }
}

// ── MumfordDivisor ────────────────────────────────────────────────────────────

/// A reduced divisor `[u(x), v(x)]` in Mumford representation.
///
/// Invariants (enforced by [`HyperellipticCurve::is_valid`]):
/// - `u` is monic.
/// - `deg v < deg u ≤ g` (or the zero divisor `u = 1`, `v = 0`).
/// - `u | (f − v·h − v²)` (curve-compatibility).
///
/// The zero divisor (group identity) is `[1, 0]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MumfordDivisor<F, const L: usize> {
    /// The `u(x)` polynomial (monic, `deg u ≤ g`).
    pub u: Poly<F, L>,
    /// The `v(x)` polynomial (`deg v < deg u`).
    pub v: Poly<F, L>,
}

impl<F: F2m<L>, const L: usize> MumfordDivisor<F, L> {
    /// Construct a Mumford divisor directly from `u` and `v`.
    ///
    /// Does not check the Mumford invariant — use
    /// [`HyperellipticCurve::is_valid`] to verify.
    pub fn new(u: Poly<F, L>, v: Poly<F, L>) -> Self {
        MumfordDivisor { u, v }
    }

    /// Return `true` if this is the zero divisor `[1, 0]`.
    pub fn is_zero(&self) -> bool {
        self.u == Poly::one() && self.v.is_zero()
    }

    /// Evaluate `u(x)` at a field element `x`.
    pub fn eval_u(&self, x: &F, poly: &Uint<L>) -> F {
        eval_poly(&self.u, x, poly)
    }

    /// Evaluate `v(x)` at a field element `x`.
    pub fn eval_v(&self, x: &F, poly: &Uint<L>) -> F {
        eval_poly(&self.v, x, poly)
    }
}

// ── Polynomial evaluation ─────────────────────────────────────────────────────

/// Evaluate a polynomial `p` at a field element `x` using Horner's method.
///
/// Returns `p(x)` in GF(2^m).
pub fn eval_poly<F: F2m<L>, const L: usize>(p: &Poly<F, L>, x: &F, poly: &Uint<L>) -> F {
    let coeffs = p.coeffs();
    if coeffs.is_empty() {
        return F::zero();
    }
    // Horner: p(x) = (...((cₙ·x + cₙ₋₁)·x + cₙ₋₂)·x + ... + c₀)
    let mut result = coeffs[coeffs.len() - 1].clone();
    for c in coeffs[..coeffs.len() - 1].iter().rev() {
        result = result.mul(x, poly).add(c);
    }
    result
}

// ── Lagrange interpolation ────────────────────────────────────────────────────

/// Lagrange interpolation over GF(2^m): find the unique polynomial `v` of
/// degree < `n` with `v(xᵢ) = yᵢ` for the given `n` points.
///
/// Uses the standard Lagrange formula:
///   `v(x) = Σᵢ yᵢ · Πⱼ≠ᵢ (x − xⱼ) / (xᵢ − xⱼ)`
///
/// In char 2, `x − xⱼ = x + xⱼ` and `xᵢ − xⱼ = xᵢ + xⱼ`.
///
/// # Panics
///
/// Panics if any two x-coordinates are equal (the interpolation is undefined).
fn lagrange_interpolate<F: F2m<L>, const L: usize>(
    points: &[(F, F)],
    poly: &Uint<L>,
) -> Poly<F, L> {
    let n = points.len();
    let mut result = Poly::zero();

    for i in 0..n {
        let (xi, yi) = &points[i];

        // Numerator: Πⱼ≠ᵢ (x + xⱼ)  [char-2: x − xⱼ = x + xⱼ]
        let mut num = Poly::one();
        for j in 0..n {
            if j == i {
                continue;
            }
            let (xj, _) = &points[j];
            // (x + xⱼ)
            let linear = Poly::from_coeffs(vec![xj.clone(), F::one()]);
            num = num.mul(&linear, poly);
        }

        // Denominator: Πⱼ≠ᵢ (xᵢ + xⱼ)  [char-2: xᵢ − xⱼ = xᵢ + xⱼ]
        let mut denom = F::one();
        for j in 0..n {
            if j == i {
                continue;
            }
            let (xj, _) = &points[j];
            let diff = xi.add(xj);
            assert!(!diff.is_zero(), "lagrange_interpolate: duplicate x-coordinates");
            denom = denom.mul(&diff, poly);
        }

        // Basis polynomial: num / denom = num · denom⁻¹
        let denom_inv = denom.inv(poly);
        let basis = num.scale(&denom_inv, poly);

        // Accumulate: result += yᵢ · basis
        let term = basis.scale(yi, poly);
        result = result.add(&term);
    }

    result
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_gf2m::F2mNaive;

    type F = F2mNaive<1>;

    /// GF(2^4) irreducible: x⁴+x+1 = 0x13.
    fn poly4() -> Uint<1> {
        Uint::<1>::from(0x13u64)
    }

    fn f(v: u64) -> F {
        F::from_u64(v, &poly4())
    }

    /// Toy genus-2 curve over GF(2^4).
    ///
    /// `y² + x·y = x⁵ + x³ + 1`
    ///
    /// h(x) = x  (h_coeffs = [0, 1])
    /// f(x) = x⁵ + x³ + 1  (f_coeffs = [1, 0, 0, 1, 0, 1])
    /// deg f = 5, genus = (5-1)/2 = 2.
    fn toy_curve() -> HyperellipticCurve<1> {
        let poly = poly4();
        HyperellipticCurve::new(
            poly,
            vec![Uint::<1>::ZERO, Uint::<1>::ONE], // h = x
            vec![
                Uint::<1>::ONE,  // constant 1
                Uint::<1>::ZERO, // x^1: 0
                Uint::<1>::ZERO, // x^2: 0
                Uint::<1>::ONE,  // x^3: 1
                Uint::<1>::ZERO, // x^4: 0
                Uint::<1>::ONE,  // x^5: 1
            ],
        )
    }

    #[test]
    fn genus_is_two() {
        let c = toy_curve();
        assert_eq!(c.genus(), 2, "genus should be 2 for deg f = 5");
    }

    #[test]
    fn zero_divisor_is_valid() {
        let c = toy_curve();
        let zero = c.zero_divisor::<F>();
        assert!(c.is_valid(&zero), "zero divisor [1, 0] must be valid");
        assert!(zero.is_zero(), "zero divisor must report is_zero()");
    }

    #[test]
    fn eval_poly_at_zero() {
        // f(0) = constant term = 1.
        let c = toy_curve();
        let f_poly = c.f::<F>();
        let result = eval_poly(&f_poly, &f(0), &poly4());
        assert_eq!(result, f(1), "f(0) should be the constant term = 1");
    }

    #[test]
    fn h_and_f_round_trip() {
        let c = toy_curve();
        let h = c.h::<F>();
        let fp = c.f::<F>();
        // h = x: degree 1, coeffs [0, 1].
        assert_eq!(h.degree(), Some(1));
        assert_eq!(h.coeff(0), f(0));
        assert_eq!(h.coeff(1), f(1));
        // f = x⁵ + x³ + 1: degree 5.
        assert_eq!(fp.degree(), Some(5));
        assert_eq!(fp.coeff(0), f(1));
        assert_eq!(fp.coeff(3), f(1));
        assert_eq!(fp.coeff(5), f(1));
    }

    #[test]
    fn divisor_from_two_points_is_valid() {
        // Build a degree-2 divisor from points (2, 8) and (3, 12).
        let c = toy_curve();
        let p1 = (f(2), f(8));
        let p2 = (f(3), f(12));
        assert!(c.is_on_curve(&p1.0, &p1.1), "p1 must be on curve");
        assert!(c.is_on_curve(&p2.0, &p2.1), "p2 must be on curve");
        let div = c.divisor_from_points::<F>(&[p1, p2]);
        assert!(c.is_valid(&div), "divisor from two points must be valid");
    }

    #[test]
    fn divisor_round_trip() {
        // Build divisor from (2, 8) and (3, 12), then recover points as roots of u.
        let c = toy_curve();
        let poly = poly4();
        let p1 = (f(2), f(8));
        let p2 = (f(3), f(12));
        let div = c.divisor_from_points::<F>(&[p1.clone(), p2.clone()]);

        // Recover points: roots of u are x-coords, y = v(x).
        let x1_recovered = p1.0.clone();
        let x2_recovered = p2.0.clone();
        let y1_recovered = div.eval_v(&x1_recovered, &poly);
        let y2_recovered = div.eval_v(&x2_recovered, &poly);

        assert_eq!(y1_recovered, p1.1, "v(x1) should equal y1");
        assert_eq!(y2_recovered, p2.1, "v(x2) should equal y2");

        // Also verify u(xi) = 0.
        let u_at_x1 = div.eval_u(&x1_recovered, &poly);
        let u_at_x2 = div.eval_u(&x2_recovered, &poly);
        assert!(u_at_x1.is_zero(), "u(x1) should be 0");
        assert!(u_at_x2.is_zero(), "u(x2) should be 0");
    }
}
