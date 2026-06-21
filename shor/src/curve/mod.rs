//! Toy short-Weierstrass elliptic curve over a small prime field.
//!
//! Defines the curve `y² = x³ + ax + b mod p` with the classical affine group law
//! (point addition, doubling, negation, scalar multiplication), a generator `G`, and
//! its prime order `r`. All arithmetic is plain `u64`; no `Fp<4>` or `rho::curve`
//! dependency.
//!
//! # Curve parameters (C-PointAdd freeze)
//!
//! ```text
//! p = 7   (prime field modulus)
//! a = 0   (short-Weierstrass coefficient)
//! b = 3   (short-Weierstrass coefficient)
//! Curve: y² = x³ + 3 mod 7
//! ```
//!
//! The 13 affine points (verified by exhaustive enumeration):
//!
//! ```text
//! ∞, (1,2), (1,5), (2,2), (2,5), (3,3), (3,4),
//!    (4,2), (4,5), (5,3), (5,4), (6,3), (6,4)
//! ```
//!
//! Group elements in scalar-multiple order from G = (1,2):
//!
//! ```text
//! 0·G = ∞,      1·G = (1,2),  2·G = (6,3),  3·G = (2,2),
//! 4·G = (4,5),  5·G = (3,3),  6·G = (5,3),  7·G = (5,4),
//! 8·G = (3,4),  9·G = (4,2),  10·G = (2,5), 11·G = (6,4),
//! 12·G = (1,5), 13·G = ∞
//! ```
//!
//! Group order `r = 13` (prime). Generator `G = (1, 2)` of full order 13.
//!
//! # Why these parameters
//!
//! - `p = 7` is the smallest prime giving a curve with prime order `r ≤ 16`.
//! - `r = 13` is prime, so the ECDLP is well-defined on the full group.
//! - Coordinates fit in 3 bits (values 0–6), so the two-register ECDLP circuit
//!   uses 3-qubit x/y registers — well within the ~25-qubit ceiling.
//! - The 4-bit exponent register (for `r = 13`) + 3-bit x + 3-bit y + 3-bit λ scratch
//!   = 13 qubits for the point-addition sub-circuit (plus the exponent register for
//!   the full ECDLP circuit in [`crate::ecdlp`]).
//!
//! # Identity encoding
//!
//! The identity point `∞` is represented as `Point::Infinity`. In the quantum circuit
//! (see `shor::ecc`), `∞` is encoded as the coordinate pair `(0, 0)` — a reserved
//! value that does not appear on the curve (since x=0 gives y²=3 mod 7, which has no
//! solution). The circuit handles the `P = ∞` case by leaving the register unchanged
//! when the input encodes `∞`.
//!
//! # References
//!
//! - Silverman, J.H. (2009). "The Arithmetic of Elliptic Curves." Springer.
//! - Proos, J., Zalka, C. (2003). "Shor's discrete logarithm quantum algorithm for
//!   elliptic curves." QIC 3(4).

// ── curve parameters ──────────────────────────────────────────────────────────

/// Field prime `p = 7`.
pub const P: u64 = 7;

/// Short-Weierstrass coefficient `a = 0` (curve: `y² = x³ + ax + b mod p`).
pub const A: u64 = 0;

/// Short-Weierstrass coefficient `b = 3` (curve: `y² = x³ + 3 mod 7`).
pub const B: u64 = 3;

/// Generator point `G = (1, 2)` of full order `r = 13`.
pub const G: Point = Point::Affine { x: 1, y: 2 };

/// Group order `r = 13` (prime). Every non-identity point has order 13.
pub const R: u64 = 13;

// ── point type ────────────────────────────────────────────────────────────────

/// An affine point on the toy curve, or the identity (point at infinity).
///
/// Coordinates are elements of `GF(p)` stored as `u64` in `[0, p)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Point {
    /// The identity element (point at infinity).
    Infinity,
    /// An affine point `(x, y)` on the curve.
    Affine {
        /// x-coordinate in `[0, p)`.
        x: u64,
        /// y-coordinate in `[0, p)`.
        y: u64,
    },
}

impl Point {
    /// Return `true` if this is the identity point.
    #[must_use]
    pub fn is_infinity(&self) -> bool {
        matches!(self, Point::Infinity)
    }

    /// Return the x-coordinate, or `None` for the identity.
    #[must_use]
    pub fn x(&self) -> Option<u64> {
        match self {
            Point::Affine { x, .. } => Some(*x),
            Point::Infinity => None,
        }
    }

    /// Return the y-coordinate, or `None` for the identity.
    #[must_use]
    pub fn y(&self) -> Option<u64> {
        match self {
            Point::Affine { y, .. } => Some(*y),
            Point::Infinity => None,
        }
    }
}

// ── field arithmetic helpers ──────────────────────────────────────────────────

/// Compute `(a + b) mod p`.
#[inline]
#[must_use]
pub fn field_add(a: u64, b: u64) -> u64 {
    (a + b) % P
}

/// Compute `(a - b) mod p` (always non-negative result in `[0, p)`).
#[inline]
#[must_use]
pub fn field_sub(a: u64, b: u64) -> u64 {
    (a + P - b % P) % P
}

/// Compute `(a * b) mod p`.
#[inline]
#[must_use]
pub fn field_mul(a: u64, b: u64) -> u64 {
    (a * b) % P
}

/// Compute the modular inverse of `a` modulo `p` using Fermat's little theorem.
///
/// Returns `a^{p-2} mod p`. Panics if `a ≡ 0 (mod p)`.
///
/// # Panics
///
/// Panics if `a % p == 0` (zero has no inverse).
#[must_use]
pub fn field_inv(a: u64) -> u64 {
    let a = a % P;
    assert!(a != 0, "field_inv: cannot invert zero mod {P}");
    // Fermat: a^{p-1} = 1 mod p, so a^{-1} = a^{p-2} mod p.
    // p = 7, p-2 = 5.
    field_pow(a, P - 2)
}

/// Compute `a^exp mod p` by repeated squaring.
#[must_use]
pub fn field_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= P;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % P;
        }
        exp >>= 1;
        base = base * base % P;
    }
    result
}

// ── group law ─────────────────────────────────────────────────────────────────

/// Negate a point: `−P = (x, −y mod p)`.
///
/// The identity maps to itself.
#[must_use]
pub fn negate(p: Point) -> Point {
    match p {
        Point::Infinity => Point::Infinity,
        Point::Affine { x, y } => Point::Affine { x, y: field_sub(0, y) },
    }
}

/// Add two affine points on the toy curve.
///
/// Implements the standard short-Weierstrass affine addition formula:
/// - `P + ∞ = P`, `∞ + Q = Q`
/// - `P + (−P) = ∞`
/// - `P + P` uses the doubling formula (slope `λ = (3x² + a) / (2y)`)
/// - `P + Q` (P ≠ ±Q) uses the addition formula (slope `λ = (y₂ − y₁) / (x₂ − x₁)`)
///
/// # Panics
///
/// Panics if either point is not on the curve (debug builds only).
#[must_use]
pub fn add(p: Point, q: Point) -> Point {
    match (p, q) {
        (Point::Infinity, q) => q,
        (p, Point::Infinity) => p,
        (Point::Affine { x: x1, y: y1 }, Point::Affine { x: x2, y: y2 }) => {
            if x1 == x2 {
                if y1 == y2 {
                    // P = Q: use doubling formula.
                    double(Point::Affine { x: x1, y: y1 })
                } else {
                    // P = −Q (same x, different y): P + Q = ∞.
                    // (Since p is odd and y1 ≠ y2 with x1 = x2, we have y2 = p - y1.)
                    Point::Infinity
                }
            } else {
                // General addition: λ = (y2 - y1) / (x2 - x1) mod p.
                let dy = field_sub(y2, y1);
                let dx = field_sub(x2, x1);
                let lambda = field_mul(dy, field_inv(dx));
                let x3 = field_sub(field_sub(field_mul(lambda, lambda), x1), x2);
                let y3 = field_sub(field_mul(lambda, field_sub(x1, x3)), y1);
                Point::Affine { x: x3, y: y3 }
            }
        }
    }
}

/// Double a point: `2P`.
///
/// Uses the short-Weierstrass doubling formula:
/// `λ = (3x² + a) / (2y) mod p`, then `x₃ = λ² − 2x`, `y₃ = λ(x − x₃) − y`.
///
/// Returns `∞` if `P = ∞` or `y = 0` (point of order 2, not present on this curve).
#[must_use]
pub fn double(p: Point) -> Point {
    match p {
        Point::Infinity => Point::Infinity,
        Point::Affine { x, y } => {
            if y == 0 {
                // Point of order 2: 2P = ∞.
                return Point::Infinity;
            }
            // λ = (3x² + a) / (2y) mod p.
            let x2 = field_mul(x, x);
            let numerator = field_add(field_mul(3, x2), A);
            let denominator = field_mul(2, y);
            let lambda = field_mul(numerator, field_inv(denominator));
            let x3 = field_sub(field_sub(field_mul(lambda, lambda), x), x);
            let y3 = field_sub(field_mul(lambda, field_sub(x, x3)), y);
            Point::Affine { x: x3, y: y3 }
        }
    }
}

/// Compute the scalar multiple `k·P` using double-and-add.
///
/// Returns `∞` for `k = 0`.
#[must_use]
pub fn scalar_mul(mut k: u64, p: Point) -> Point {
    let mut result = Point::Infinity;
    let mut addend = p;
    while k > 0 {
        if k & 1 == 1 {
            result = add(result, addend);
        }
        addend = double(addend);
        k >>= 1;
    }
    result
}

/// Check whether a point lies on the curve `y² = x³ + ax + b mod p`.
///
/// Always returns `true` for `∞`.
#[must_use]
pub fn on_curve(p: Point) -> bool {
    match p {
        Point::Infinity => true,
        Point::Affine { x, y } => {
            let lhs = field_mul(y, y);
            let rhs = field_add(field_add(field_mul(field_mul(x, x), x), field_mul(A, x)), B);
            lhs == rhs
        }
    }
}

/// Return all affine points on the curve (excluding ∞), in lexicographic order.
///
/// Enumerates all `(x, y)` pairs with `x, y ∈ [0, p)` satisfying `y² = x³ + ax + b mod p`.
#[must_use]
pub fn all_affine_points() -> Vec<Point> {
    let mut pts = Vec::new();
    for x in 0..P {
        for y in 0..P {
            let pt = Point::Affine { x, y };
            if on_curve(pt) {
                pts.push(pt);
            }
        }
    }
    pts
}

// ── C-PointAdd layout ─────────────────────────────────────────────────────────

/// Register layout descriptor for the controlled point-addition circuit (C-PointAdd freeze).
///
/// Documents the qubit layout consumed by `shor::ecc::controlled_point_add`.
///
/// # Layout (little-endian throughout — C-StateVec convention)
///
/// ```text
/// qubit  ctrl_qubit                    — control qubit (1 = apply addition, 0 = no-op)
/// qubits [x_start, x_start + x_len)   — x-coordinate of running point (little-endian LSB)
/// qubits [y_start, y_start + y_len)   — y-coordinate of running point (little-endian LSB)
/// qubits [lam_start, lam_start + lam_len) — λ scratch register (must start at |0⟩)
/// ```
///
/// # Identity encoding
///
/// The identity point `∞` is encoded as the coordinate pair `(0, 0)`. This value does not
/// appear on the curve (x=0 gives y²=3 mod 7, which has no solution), so it is a safe
/// sentinel. The circuit detects `(0, 0)` and leaves the register unchanged (identity
/// is a fixed point of point-addition).
///
/// # λ scratch register
///
/// The λ scratch register is part of the layout for documentation and ECDLP circuit
/// compatibility. In the current permutation-synthesis implementation (see `shor::ecc`), the
/// λ register is not used — the permutation synthesis is ancilla-free. The register is
/// allocated in the layout so the ECDLP circuit can extend to a formula-based implementation
/// if needed.
#[derive(Clone, Debug)]
pub struct PointAddLayout {
    /// Control qubit index.
    pub ctrl_qubit: usize,
    /// Index of the first x-coordinate qubit (little-endian LSB).
    pub x_start: usize,
    /// Number of x-coordinate qubits (`⌈log₂ p⌉`).
    pub x_len: usize,
    /// Index of the first y-coordinate qubit (little-endian LSB).
    pub y_start: usize,
    /// Number of y-coordinate qubits (`⌈log₂ p⌉`).
    pub y_len: usize,
    /// Index of the first λ scratch qubit (little-endian LSB).
    pub lam_start: usize,
    /// Number of λ scratch qubits (`⌈log₂ p⌉`).
    pub lam_len: usize,
}

impl PointAddLayout {
    /// Construct a standard `PointAddLayout` for the toy curve.
    ///
    /// Standard layout (little-endian, all registers contiguous):
    /// - Control qubit: qubit 0
    /// - x register: qubits `[1, 1 + coord_bits)`
    /// - y register: qubits `[1 + coord_bits, 1 + 2·coord_bits)`
    /// - λ scratch: qubits `[1 + 2·coord_bits, 1 + 3·coord_bits)`
    ///
    /// where `coord_bits = ⌈log₂ p⌉`.
    ///
    /// Total qubits: `1 + 3·coord_bits`.
    ///
    /// For the toy curve (`p = 7`): `coord_bits = 3`, total = 10 qubits.
    #[must_use]
    pub fn standard() -> Self {
        let coord_bits = coord_qubits();
        Self {
            ctrl_qubit: 0,
            x_start: 1,
            x_len: coord_bits,
            y_start: 1 + coord_bits,
            y_len: coord_bits,
            lam_start: 1 + 2 * coord_bits,
            lam_len: coord_bits,
        }
    }

    /// Total number of qubits required for this layout.
    #[must_use]
    pub fn total_qubits(&self) -> usize {
        let ends = [
            self.ctrl_qubit + 1,
            self.x_start + self.x_len,
            self.y_start + self.y_len,
            self.lam_start + self.lam_len,
        ];
        *ends.iter().max().unwrap()
    }

    /// Qubit indices of the x-coordinate register (little-endian, LSB first).
    #[must_use]
    pub fn x_qubits(&self) -> Vec<usize> {
        (self.x_start..self.x_start + self.x_len).collect()
    }

    /// Qubit indices of the y-coordinate register (little-endian, LSB first).
    #[must_use]
    pub fn y_qubits(&self) -> Vec<usize> {
        (self.y_start..self.y_start + self.y_len).collect()
    }

    /// Qubit indices of the λ scratch register (little-endian, LSB first).
    #[must_use]
    pub fn lam_qubits(&self) -> Vec<usize> {
        (self.lam_start..self.lam_start + self.lam_len).collect()
    }
}

/// Number of qubits needed to hold a field element in `[0, p)`.
///
/// Returns `⌈log₂ p⌉`. For `p = 7`, this is 3.
#[must_use]
pub fn coord_qubits() -> usize {
    // ⌈log₂ 7⌉ = 3 (since 2² = 4 < 7 ≤ 8 = 2³)
    let bits = u64::BITS - (P - 1).leading_zeros();
    bits as usize
}

// ── encoding helpers ──────────────────────────────────────────────────────────

/// Encode a `Point` as a `(x_val, y_val)` pair for the quantum register.
///
/// `∞` is encoded as `(0, 0)` (the reserved sentinel — not on the curve).
/// Affine points are encoded as their coordinates.
#[must_use]
pub fn encode_point(p: Point) -> (u64, u64) {
    match p {
        Point::Infinity => (0, 0),
        Point::Affine { x, y } => (x, y),
    }
}

/// Decode a `(x_val, y_val)` pair from the quantum register to a `Point`.
///
/// `(0, 0)` decodes to `∞`. All other pairs decode to `Point::Affine { x, y }`.
#[must_use]
pub fn decode_point(x_val: u64, y_val: u64) -> Point {
    if x_val == 0 && y_val == 0 {
        Point::Infinity
    } else {
        Point::Affine { x: x_val, y: y_val }
    }
}
