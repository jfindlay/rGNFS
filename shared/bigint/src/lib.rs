//! Multi-precision helpers for prime-field arithmetic and `BigInt` utilities.
//!
//! - [`batch_inv`]: Montgomery's batched inversion trick, generic over ``Fp<L>``.
//! - [`mp`]: multi-precision helpers beyond ``crypto-bigint`` if needed.
//! - [`isqrt`]: exact integer square root for `BigInt`.
//! - [`gcd`]: non-negative greatest common divisor for `BigInt`.

pub mod batch_inv;
pub mod isqrt;
pub mod mp;

pub use batch_inv::batch_invert;
pub use isqrt::isqrt;

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Signed;

/// Compute the greatest common divisor of `a` and `b`.
///
/// Returns the non-negative gcd. Sign convention: `gcd(a, b) ≥ 0` always, matching
/// the mathematical convention that gcd is a non-negative integer. In particular:
/// - `gcd(0, n) = |n|`
/// - `gcd(n, 0) = |n|`
/// - `gcd(0, 0) = 0`
/// - Mixed-sign inputs return the same result as their absolute values.
pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    // num_integer::gcd handles BigInt and returns a non-negative result.
    let g = a.gcd(b);
    // Ensure non-negative (num_integer::gcd already guarantees this for BigInt,
    // but we document and enforce it explicitly).
    if g.is_negative() { -g } else { g }
}
