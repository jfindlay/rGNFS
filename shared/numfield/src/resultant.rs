//! Resultant and subresultant GCD over ℤ[x].
//!
//! Provides:
//! - [`resultant`] — Res(f, g) ∈ ℤ via the Sylvester matrix determinant.
//! - [`subresultant_gcd`] — primitive GCD of f and g via the pseudo-remainder sequence.

use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

use crate::poly::IntPoly;

// ─── helpers ─────────────────────────────────────────────────────────────────

/// Euclidean GCD of two `BigInt` values (always non-negative).
fn bigint_gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut x = a.abs();
    let mut y = b.abs();
    while !y.is_zero() {
        let r = &x % &y;
        x = y;
        y = r;
    }
    x
}

/// Content of a polynomial: gcd of all coefficients (always non-negative).
///
/// Returns 0 for the zero polynomial.
fn content(p: &IntPoly) -> BigInt {
    p.coeffs.iter().fold(BigInt::zero(), |acc, c| bigint_gcd(&acc, c))
}

/// Primitive part: divide all coefficients by the content.
///
/// The leading coefficient of the result is positive. Returns the zero polynomial
/// unchanged.
fn primitive_part(p: &IntPoly) -> IntPoly {
    let c = content(p);
    if c.is_zero() {
        return p.clone();
    }
    let mut coeffs: Vec<BigInt> = p.coeffs.iter().map(|a| a / &c).collect();
    // Normalise sign so that the leading coefficient is positive.
    if let Some(lc) = coeffs.last() {
        if lc.is_negative() {
            for a in &mut coeffs {
                *a = -a.clone();
            }
        }
    }
    IntPoly::from_coeffs(coeffs)
}

// ─── Sylvester matrix determinant ────────────────────────────────────────────

/// Build the Sylvester matrix of f and g as a flat row-major Vec<BigInt>.
///
/// The Sylvester matrix is (m+n) × (m+n) where m = deg(f), n = deg(g).
/// Rows 0..n are shifts of f; rows n..n+m are shifts of g.
fn sylvester_matrix(f: &IntPoly, g: &IntPoly) -> (Vec<BigInt>, usize) {
    let m = f.degree().unwrap_or(0);
    let n = g.degree().unwrap_or(0);
    let size = m + n;
    let mut mat = vec![BigInt::zero(); size * size];

    // Rows 0..n: shifts of f by 0..n-1 positions.
    for i in 0..n {
        for (j, c) in f.coeffs.iter().enumerate() {
            // f has degree m; coeffs[j] is coefficient of x^j.
            // In the Sylvester matrix, the coefficient of x^(m-k) goes in column (m-k)+i.
            // Equivalently, coeffs[j] (for x^j) goes in column (m - j) + i... but we
            // store coefficients least-significant first, so we need to reverse.
            // Column index: (m - j) + i  →  but columns run 0..size-1 left to right
            // for decreasing powers. Simpler: place f[m-k] at col k+i for k=0..m.
            let k = m - j; // power index from the top: x^(m-j) → position k = m-j from left
            let col = k + i; // shift by i
            if col < size {
                mat[i * size + col] = c.clone();
            }
        }
    }

    // Rows n..n+m: shifts of g by 0..m-1 positions.
    for i in 0..m {
        let row = n + i;
        for (j, c) in g.coeffs.iter().enumerate() {
            let k = n - j; // position from left for x^(n-j)
            let col = k + i;
            if col < size {
                mat[row * size + col] = c.clone();
            }
        }
    }

    (mat, size)
}

/// Compute the determinant of a square matrix over ℤ using fraction-free
/// (Bareiss) Gaussian elimination.
///
/// The Bareiss algorithm maintains exact integer arithmetic throughout by
/// dividing each pivot step by the previous pivot — a division that is always
/// exact over ℤ.
fn bareiss_det(mut mat: Vec<BigInt>, n: usize) -> BigInt {
    if n == 0 {
        return BigInt::one();
    }

    let mut sign = BigInt::one();
    let mut prev_pivot = BigInt::one();

    for col in 0..n {
        // Find a non-zero pivot in column `col` at or below row `col`.
        let pivot_row = (col..n).find(|&r| !mat[r * n + col].is_zero());

        let pivot_row = match pivot_row {
            None => return BigInt::zero(), // singular
            Some(r) => r,
        };

        // Swap rows if needed.
        if pivot_row != col {
            for j in 0..n {
                mat.swap(col * n + j, pivot_row * n + j);
            }
            sign = -sign;
        }

        let pivot = mat[col * n + col].clone();

        // Eliminate below the pivot using the Bareiss update:
        //   M[i][j] = (M[i][j] * pivot - M[i][col] * M[col][j]) / prev_pivot
        for i in (col + 1)..n {
            for j in (col + 1)..n {
                let new_val =
                    (&mat[i * n + j] * &pivot - &mat[i * n + col] * &mat[col * n + j])
                        / &prev_pivot;
                mat[i * n + j] = new_val;
            }
            mat[i * n + col] = BigInt::zero();
        }

        prev_pivot = pivot;
    }

    sign * prev_pivot
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Resultant of f and g over ℤ[x] via the Sylvester matrix determinant.
///
/// Returns Res(f, g) ∈ ℤ. Returns 0 if either polynomial is zero.
/// The Sylvester matrix is (deg(f)+deg(g)) × (deg(f)+deg(g)); its determinant
/// is computed with the Bareiss fraction-free algorithm to stay in ℤ throughout.
pub fn resultant(f: &IntPoly, g: &IntPoly) -> BigInt {
    // If either is zero, resultant is 0.
    if f.degree().is_none() || g.degree().is_none() {
        return BigInt::zero();
    }

    let (mat, size) = sylvester_matrix(f, g);
    if size == 0 {
        // Both are constants; Res(a, b) = 1 (empty product convention).
        return BigInt::one();
    }

    bareiss_det(mat, size)
}

/// Subresultant GCD of f and g over ℤ[x].
///
/// Returns a primitive polynomial proportional to gcd(f, g) over ℚ[x].
/// The result is the primitive part of the last non-zero remainder in the
/// pseudo-remainder sequence. Returns the zero polynomial if both inputs are zero.
pub fn subresultant_gcd(f: &IntPoly, g: &IntPoly) -> IntPoly {
    // Handle zero inputs.
    match (f.degree(), g.degree()) {
        (None, None) => return IntPoly::zero(),
        (None, Some(_)) => return primitive_part(g),
        (Some(_), None) => return primitive_part(f),
        _ => {}
    }

    // Work with primitive parts; the integer GCD of the contents is the
    // content of the result, but since we only need the primitive GCD here
    // (as required by C-Res), we can discard it.
    let mut a = primitive_part(f);
    let mut b = primitive_part(g);

    // Ensure deg(a) >= deg(b).
    if a.degree().unwrap() < b.degree().unwrap() {
        std::mem::swap(&mut a, &mut b);
    }

    // Pseudo-remainder sequence with content removal at each step.
    // This is the "subresultant-inspired" PRS: take the pseudo-remainder,
    // then reduce to its primitive part to keep coefficients small.
    loop {
        let (_, r) = a.pseudo_div_rem(&b);
        if r.degree().is_none() {
            // Remainder is zero → b is the GCD (up to primitive part).
            return primitive_part(&b);
        }
        a = b;
        b = primitive_part(&r);
    }
}

// ─── unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    fn ip(coeffs: Vec<i64>) -> IntPoly {
        IntPoly::from_coeffs(coeffs.into_iter().map(BigInt::from).collect())
    }

    // ── pseudo_div_rem ──

    #[test]
    fn pseudo_div_rem_exact_divisible() {
        // (x² − 1) pseudo-divided by (x − 1).
        // lc(g) = 1, so pseudo-div = ordinary div.
        // x² − 1 = (x + 1)(x − 1) + 0
        let f = ip(vec![-1, 0, 1]); // x² − 1
        let g = ip(vec![-1, 1]); // x − 1
        let (q, r) = f.pseudo_div_rem(&g);
        assert_eq!(r, IntPoly::zero());
        assert_eq!(q, ip(vec![1, 1])); // x + 1
    }

    #[test]
    fn pseudo_div_rem_non_monic_divisor() {
        // f = 2x² + 3x + 1, g = 2x + 1.
        // lc(g) = 2, e = 2 − 1 + 1 = 2, multiplier = 4.
        // 4f = 8x² + 12x + 4.
        // 8x² + 12x + 4 = (4x + 4)(2x + 1) + 0.
        let f = ip(vec![1, 3, 2]); // 2x² + 3x + 1
        let g = ip(vec![1, 2]); // 2x + 1
        let (q, r) = f.pseudo_div_rem(&g);
        assert_eq!(r, IntPoly::zero());
        // q should be 4x + 4
        assert_eq!(q, ip(vec![4, 4]));
    }

    #[test]
    fn pseudo_div_rem_with_remainder() {
        // f = x², g = x + 1.
        // lc(g) = 1, so pseudo-div = ordinary div.
        // x² = (x − 1)(x + 1) + 1
        let f = ip(vec![0, 0, 1]); // x²
        let g = ip(vec![1, 1]); // x + 1
        let (q, r) = f.pseudo_div_rem(&g);
        assert_eq!(q, ip(vec![-1, 1])); // x − 1
        assert_eq!(r, ip(vec![1])); // 1
    }

    #[test]
    fn pseudo_div_rem_f_degree_less_than_g() {
        // deg(f) < deg(g) → quotient = 0, remainder = f.
        let f = ip(vec![3, 1]); // x + 3
        let g = ip(vec![-1, 0, 1]); // x² − 1
        let (q, r) = f.pseudo_div_rem(&g);
        assert_eq!(q, IntPoly::zero());
        assert_eq!(r, f);
    }

    #[test]
    fn pseudo_div_rem_zero_dividend() {
        let f = IntPoly::zero();
        let g = ip(vec![-1, 1]); // x − 1
        let (q, r) = f.pseudo_div_rem(&g);
        assert_eq!(q, IntPoly::zero());
        assert_eq!(r, IntPoly::zero());
    }

    // ── resultant ──

    #[test]
    fn resultant_constant_polynomials() {
        // Res(a, b) for constants a, b: the Sylvester matrix is 0×0, so det = 1.
        // Actually for deg(f)=0, deg(g)=0: size = 0+0 = 0 → returns 1.
        let f = ip(vec![3]);
        let g = ip(vec![5]);
        // Res(3, 5) = 1 by the empty-product convention in our implementation.
        let r = resultant(&f, &g);
        assert_eq!(r, bi(1));
    }

    #[test]
    fn resultant_linear_linear() {
        // Res(x − a, x − b) = b − a.
        // Res(x − 2, x − 3): f = x − 2, g = x − 3.
        // Sylvester: 2×2 matrix [[1, -2], [1, -3]].
        // det = 1·(−3) − (−2)·1 = −3 + 2 = −1.
        let f = ip(vec![-2, 1]); // x − 2
        let g = ip(vec![-3, 1]); // x − 3
        let r = resultant(&f, &g);
        assert_eq!(r, bi(-1));
    }

    #[test]
    fn resultant_shared_root_is_zero() {
        // Res(x² − 1, x − 1) = 0 (they share root x = 1).
        let f = ip(vec![-1, 0, 1]); // x² − 1
        let g = ip(vec![-1, 1]); // x − 1
        let r = resultant(&f, &g);
        assert_eq!(r, bi(0));
    }
}
