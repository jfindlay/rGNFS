//! Short-Weierstrass group law over `F_{p^k}` (extension field).
//!
//! [`PairingPoint`] is an affine point on a short-Weierstrass curve
//! `y² = x³ + ax + b` whose coordinates live in [`FpExt`] rather than the
//! base prime field `F_p`.  It provides the full group law (`add`, `double`,
//! `scalar_mul`, `negate`, `is_on_curve`) and an explicit identity (point at
//! infinity).
//!
//! # Option-B duplication note
//!
//! This is the **deliberate option-B duplication** of the `rho::curve` group
//! law over the extension field `FpExt`.  The frozen `rho::curve` module uses
//! an `F: Fp<4>` bound that cannot represent extension-field points; rather
//! than amending that frozen contract (option A — declined at the E.B shard
//! adjudication), E.B builds a *separate* `E(F_{p^k})` arithmetic layer here,
//! composing the frozen `Curve` params (read-only) and the C-FpExt interface.
//! The cost is some group-law code duplication; the win is a bounded blast
//! radius on a widely-consumed frozen contract.
//!
//! # Coordinate representation
//!
//! Affine coordinates are used (no Jacobian lift).  For the toy embedding
//! degrees `k ≤ 6` at demonstration fidelity, the extra inversions in affine
//! addition are negligible.  A Jacobian lift is a principle-4 optimisation
//! annotation, not a work item.
//!
//! # Scalar multiplication
//!
//! `scalar_mul` uses the right-to-left binary method (double-and-add) and
//! handles the point at infinity correctly at every step.

use crypto_bigint::Uint;
use shared_field::Fp;

use crate::pairing::fpext::{FpExt, IrreducibleModulus};

// ── Point type ────────────────────────────────────────────────────────────────

/// An affine point on `y² = x³ + ax + b` over `F_{p^k}`.
///
/// The point at infinity is represented by the `Infinity` variant.  All finite
/// points satisfy the curve equation over `FpExt`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PairingPoint<F: Clone + PartialEq + Eq> {
    /// The group identity (point at infinity).
    Infinity,
    /// A finite affine point with coordinates in `F_{p^k}`.
    Finite {
        /// x-coordinate in `F_{p^k}`.
        x: FpExt<F>,
        /// y-coordinate in `F_{p^k}`.
        y: FpExt<F>,
    },
}

impl<F: Fp<4>> PairingPoint<F> {
    /// Construct a finite affine point.  Does not check the curve equation.
    pub fn new(x: FpExt<F>, y: FpExt<F>) -> Self {
        PairingPoint::Finite { x, y }
    }

    /// Return `true` if this is the point at infinity.
    #[inline]
    pub fn is_infinity(&self) -> bool {
        matches!(self, PairingPoint::Infinity)
    }

    /// Return a reference to the x-coordinate, or `None` for infinity.
    pub fn x(&self) -> Option<&FpExt<F>> {
        match self {
            PairingPoint::Infinity => None,
            PairingPoint::Finite { x, .. } => Some(x),
        }
    }

    /// Return a reference to the y-coordinate, or `None` for infinity.
    pub fn y(&self) -> Option<&FpExt<F>> {
        match self {
            PairingPoint::Infinity => None,
            PairingPoint::Finite { y, .. } => Some(y),
        }
    }

    /// Negate a point: `(x, y) → (x, −y)`; `∞ → ∞`.
    pub fn negate(&self, p: &Uint<4>) -> Self {
        match self {
            PairingPoint::Infinity => PairingPoint::Infinity,
            PairingPoint::Finite { x, y } => PairingPoint::Finite {
                x: x.clone(),
                y: y.neg(p),
            },
        }
    }

    /// Check whether this point satisfies `y² = x³ + ax + b` over `F_{p^k}`.
    ///
    /// The curve coefficients `a` and `b` are passed as `FpExt` elements (they
    /// are typically lifted from `F_p` via [`FpExt::from_base`]).
    pub fn is_on_curve(
        &self,
        a: &FpExt<F>,
        b: &FpExt<F>,
        modulus: &IrreducibleModulus<F>,
        p: &Uint<4>,
    ) -> bool {
        match self {
            PairingPoint::Infinity => true,
            PairingPoint::Finite { x, y } => {
                // lhs = y²
                let lhs = y.square(modulus, p);
                // rhs = x³ + a·x + b
                let x2 = x.square(modulus, p);
                let x3 = x2.mul(x, modulus, p);
                let ax = a.mul(x, modulus, p);
                let rhs = x3.add(&ax, p).add(b, p);
                lhs == rhs
            }
        }
    }

    /// Point doubling: `2P`.
    ///
    /// Uses the standard affine doubling formula:
    /// ```text
    /// λ = (3·x² + a) / (2·y)
    /// x' = λ² − 2·x
    /// y' = λ·(x − x') − y
    /// ```
    ///
    /// Returns `∞` if `P = ∞` or `y = 0` (i.e., `P = −P`).
    pub fn double(
        &self,
        a: &FpExt<F>,
        modulus: &IrreducibleModulus<F>,
        p: &Uint<4>,
    ) -> Self {
        match self {
            PairingPoint::Infinity => PairingPoint::Infinity,
            PairingPoint::Finite { x, y } => {
                // If y = 0, the tangent is vertical: 2P = ∞.
                if y.is_zero(p) {
                    return PairingPoint::Infinity;
                }
                let k = x.degree();
                // λ = (3·x² + a) / (2·y)
                let x2 = x.square(modulus, p);
                let three = FpExt::from_base(F::from_u64(3, p), k, p);
                let three_x2 = three.mul(&x2, modulus, p);
                let num = three_x2.add(a, p);
                let two = FpExt::from_base(F::from_u64(2, p), k, p);
                let den = two.mul(y, modulus, p);
                let lam = num.mul(&den.inv(modulus, p), modulus, p);

                // x' = λ² − 2·x
                let lam2 = lam.square(modulus, p);
                let two_x = x.add(x, p);
                let xp = lam2.sub(&two_x, p);

                // y' = λ·(x − x') − y
                let yp = lam.mul(&x.sub(&xp, p), modulus, p).sub(y, p);

                PairingPoint::Finite { x: xp, y: yp }
            }
        }
    }

    /// Point addition: `P + Q`.
    ///
    /// Uses the standard affine chord formula:
    /// ```text
    /// λ = (y₂ − y₁) / (x₂ − x₁)
    /// x' = λ² − x₁ − x₂
    /// y' = λ·(x₁ − x') − y₁
    /// ```
    ///
    /// Falls back to `double` when `P = Q`, and returns `∞` when `P = −Q`.
    pub fn add(
        &self,
        rhs: &Self,
        a: &FpExt<F>,
        modulus: &IrreducibleModulus<F>,
        p: &Uint<4>,
    ) -> Self {
        match (self, rhs) {
            (PairingPoint::Infinity, _) => rhs.clone(),
            (_, PairingPoint::Infinity) => self.clone(),
            (PairingPoint::Finite { x: x1, y: y1 }, PairingPoint::Finite { x: x2, y: y2 }) => {
                let x_diff = x2.sub(x1, p);
                if x_diff.is_zero(p) {
                    let y_sum = y1.add(y2, p);
                    if y_sum.is_zero(p) {
                        // P = −Q: P + Q = ∞
                        return PairingPoint::Infinity;
                    }
                    // P = Q: use doubling
                    return self.double(a, modulus, p);
                }

                // λ = (y₂ − y₁) / (x₂ − x₁)
                let y_diff = y2.sub(y1, p);
                let lam = y_diff.mul(&x_diff.inv(modulus, p), modulus, p);

                // x' = λ² − x₁ − x₂
                let lam2 = lam.square(modulus, p);
                let xp = lam2.sub(x1, p).sub(x2, p);

                // y' = λ·(x₁ − x') − y₁
                let yp = lam.mul(&x1.sub(&xp, p), modulus, p).sub(y1, p);

                PairingPoint::Finite { x: xp, y: yp }
            }
        }
    }

    /// Scalar multiplication: `k·P` using the right-to-left binary method.
    ///
    /// `scalar` is a `u64`; the low bits are consumed first (double-and-add).
    /// Handles the point at infinity correctly at every step.
    ///
    /// Returns `∞` for `scalar = 0` or `P = ∞`.
    pub fn scalar_mul(
        &self,
        scalar: u64,
        a: &FpExt<F>,
        modulus: &IrreducibleModulus<F>,
        p: &Uint<4>,
    ) -> Self {
        if scalar == 0 {
            return PairingPoint::Infinity;
        }
        if self.is_infinity() {
            return PairingPoint::Infinity;
        }

        let mut result = PairingPoint::Infinity;
        let mut base = self.clone();
        let mut k = scalar;

        while k != 0 {
            if k & 1 == 1 {
                result = result.add(&base, a, modulus, p);
            }
            base = base.double(a, modulus, p);
            k >>= 1;
        }

        result
    }

    /// Scalar multiplication by a `Uint<4>`: `k·P`.
    ///
    /// Same algorithm as [`scalar_mul`] but accepts a full 256-bit scalar.
    pub fn scalar_mul_uint(
        &self,
        scalar: &Uint<4>,
        a: &FpExt<F>,
        modulus: &IrreducibleModulus<F>,
        p: &Uint<4>,
    ) -> Self {
        if *scalar == Uint::<4>::ZERO {
            return PairingPoint::Infinity;
        }
        if self.is_infinity() {
            return PairingPoint::Infinity;
        }

        let mut result = PairingPoint::Infinity;
        let mut base = self.clone();
        let mut k = *scalar;

        while k != Uint::<4>::ZERO {
            if bool::from(k.bit(0)) {
                result = result.add(&base, a, modulus, p);
            }
            base = base.double(a, modulus, p);
            k >>= 1;
        }

        result
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_field::FpNaive;

    use crate::pairing::fpext::tests::{fp2, modulus_k2, p47};

    // ── Helpers ───────────────────────────────────────────────────────────────

    /// Curve coefficients for composite_toy lifted into F_{47^2}.
    ///
    /// `y² = x³ + x + 33 mod 47`; `a = 1`, `b = 33`.
    fn curve_a() -> FpExt<FpNaive<4>> {
        let p = p47();
        FpExt::from_base(FpNaive::<4>::from_u64(1, &p), 2, &p)
    }

    fn curve_b() -> FpExt<FpNaive<4>> {
        let p = p47();
        FpExt::from_base(FpNaive::<4>::from_u64(33, &p), 2, &p)
    }

    /// Construct a `PairingPoint` from two pairs of `u64` coefficients.
    fn pt(x0: u64, x1: u64, y0: u64, y1: u64) -> PairingPoint<FpNaive<4>> {
        PairingPoint::new(fp2(x0, x1), fp2(y0, y1))
    }

    // ── is_on_curve ───────────────────────────────────────────────────────────

    /// P = (8, 6) ∈ E(F_47) is on the curve (lifted to F_{47^2}).
    #[test]
    fn p_is_on_curve() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let b = curve_b();
        let pt_p = pt(8, 0, 6, 0);
        assert!(pt_p.is_on_curve(&a, &b, &m, &p), "P should be on curve");
    }

    /// Q = ((4,15), (22,34)) ∈ E(F_{47^2}) is on the curve.
    #[test]
    fn q_is_on_curve() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let b = curve_b();
        let pt_q = pt(4, 15, 22, 34);
        assert!(pt_q.is_on_curve(&a, &b, &m, &p), "Q should be on curve");
    }

    /// The point at infinity is on every curve.
    #[test]
    fn infinity_is_on_curve() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let b = curve_b();
        assert!(
            PairingPoint::<FpNaive<4>>::Infinity.is_on_curve(&a, &b, &m, &p),
            "∞ should be on curve"
        );
    }

    // ── Identity laws ─────────────────────────────────────────────────────────

    /// `P + ∞ = P`.
    #[test]
    fn add_infinity_right() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let result = pt_p.add(&PairingPoint::Infinity, &a, &m, &p);
        assert_eq!(result, pt_p, "P + ∞ should equal P");
    }

    /// `∞ + P = P`.
    #[test]
    fn add_infinity_left() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let result = PairingPoint::Infinity.add(&pt_p, &a, &m, &p);
        assert_eq!(result, pt_p, "∞ + P should equal P");
    }

    // ── Negation ──────────────────────────────────────────────────────────────

    /// `P + (−P) = ∞`.
    #[test]
    fn add_neg_is_infinity() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let neg_p = pt_p.negate(&p);
        let result = pt_p.add(&neg_p, &a, &m, &p);
        assert!(result.is_infinity(), "P + (−P) should be ∞");
    }

    /// `Q + (−Q) = ∞` for the extension-field point.
    #[test]
    fn add_neg_q_is_infinity() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_q = pt(4, 15, 22, 34);
        let neg_q = pt_q.negate(&p);
        let result = pt_q.add(&neg_q, &a, &m, &p);
        assert!(result.is_infinity(), "Q + (−Q) should be ∞");
    }

    // ── Doubling ──────────────────────────────────────────────────────────────

    /// `double(P) = add(P, P)`.
    #[test]
    fn double_matches_add_self() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let via_double = pt_p.double(&a, &m, &p);
        let via_add = pt_p.add(&pt_p, &a, &m, &p);
        assert_eq!(via_double, via_add, "double(P) should equal add(P, P)");
    }

    /// `2P = (8, 41)` — known value from offline computation.
    ///
    /// `2·(8, 6) = (8, 41)` on `y² = x³ + x + 33 mod 47`.
    #[test]
    fn double_p_known_value() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let two_p = pt_p.double(&a, &m, &p);
        assert_eq!(two_p, pt(8, 0, 41, 0), "2P should be (8, 41)");
    }

    /// `2Q = ((4,15), (25,13))` — known value from offline computation.
    #[test]
    fn double_q_known_value() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_q = pt(4, 15, 22, 34);
        let two_q = pt_q.double(&a, &m, &p);
        assert_eq!(two_q, pt(4, 15, 25, 13), "2Q should be ((4,15),(25,13))");
    }

    // ── Addition ──────────────────────────────────────────────────────────────

    /// `P + Q = ((4,32), (25,34))` — known value from offline computation.
    #[test]
    fn add_p_q_known_value() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let pt_q = pt(4, 15, 22, 34);
        let pq = pt_p.add(&pt_q, &a, &m, &p);
        assert_eq!(pq, pt(4, 32, 25, 34), "P + Q should be ((4,32),(25,34))");
    }

    /// Commutativity: `P + Q = Q + P`.
    #[test]
    fn add_commutative() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let pt_q = pt(4, 15, 22, 34);
        assert_eq!(
            pt_p.add(&pt_q, &a, &m, &p),
            pt_q.add(&pt_p, &a, &m, &p),
            "P + Q should equal Q + P"
        );
    }

    /// Associativity: `(P + Q) + Q = P + (Q + Q)`.
    #[test]
    fn add_associative() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let pt_q = pt(4, 15, 22, 34);
        let pq = pt_p.add(&pt_q, &a, &m, &p);
        let lhs = pq.add(&pt_q, &a, &m, &p);
        let qq = pt_q.add(&pt_q, &a, &m, &p);
        let rhs = pt_p.add(&qq, &a, &m, &p);
        assert_eq!(lhs, rhs, "(P+Q)+Q should equal P+(Q+Q)");
    }

    // ── Scalar multiplication ─────────────────────────────────────────────────

    /// `0·P = ∞`.
    #[test]
    fn scalar_mul_zero_is_infinity() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let result = pt_p.scalar_mul(0, &a, &m, &p);
        assert!(result.is_infinity(), "0·P should be ∞");
    }

    /// `1·P = P`.
    #[test]
    fn scalar_mul_one_is_identity() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let result = pt_p.scalar_mul(1, &a, &m, &p);
        assert_eq!(result, pt_p, "1·P should equal P");
    }

    /// `2·P = double(P)`.
    #[test]
    fn scalar_mul_two_matches_double() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let pt_p = pt(8, 0, 6, 0);
        let via_scalar = pt_p.scalar_mul(2, &a, &m, &p);
        let via_double = pt_p.double(&a, &m, &p);
        assert_eq!(via_scalar, via_double, "2·P via scalar_mul should match double");
    }

    /// `scalar_mul(0, …)` on ∞ returns ∞.
    #[test]
    fn scalar_mul_infinity_is_infinity() {
        let p = p47();
        let m = modulus_k2();
        let a = curve_a();
        let result = PairingPoint::<FpNaive<4>>::Infinity.scalar_mul(5, &a, &m, &p);
        assert!(result.is_infinity(), "k·∞ should be ∞");
    }
}
