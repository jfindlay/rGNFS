//! Rational square root from a kernel vector.
//!
//! Given a `KernelVector` and the original `Vec<Relation>` + `PolyPair`, computes X such that
//! X² ≡ ∏_{i ∈ S}(a_i − b_i·m) (mod N), where S is the relation index set recovered from the
//! kernel vector via `expand_provenance`.
//!
//! # Algorithm
//!
//! 1. Call `kv.expand_provenance(matrix)` to recover the relation index set S.
//! 2. Form the product P = ∏_{i ∈ S}(a_i − b_i·m) over ℤ.
//! 3. Assert P > 0 (the sign column in G.E guarantees an even count of negative rational norms;
//!    a negative P is an upstream kernel/sign-column bug, surfaced with a clear panic).
//! 4. Call `isqrt(&P)` (from `shared_bigint`). If `None`, panic: the product is not a perfect
//!    square, indicating an upstream kernel/sign-column bug.
//! 5. Reduce X = isqrt_result mod N and return X.
//!
//! # Sign invariant
//!
//! The `rational_sign` field on each `Relation` records whether `a − b·m < 0`. The G.E linear
//! algebra step (via the sign/obstruction column) selects only subsets S where the count of
//! negative rational norms is even, so P = ∏(a_i − b_i·m) is guaranteed positive. A negative P
//! means the sign column was not honoured — an upstream G.E bug, not a normal path.

use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use shared_bigint::isqrt;

use crate::filter::SparseMatrix;
use crate::linalg::KernelVector;
use crate::polyselect::PolyPair;
use crate::sieve::Relation;

/// Compute the rational square root X such that X² ≡ ∏_{i ∈ S}(a_i − b_i·m) (mod N).
///
/// Recovers the relation index set S from `kv` via `expand_provenance`, forms the integer
/// product P = ∏_{i ∈ S}(a_i − b_i·m), extracts its integer square root via `isqrt`, and
/// reduces the result mod N.
///
/// # Panics
///
/// - If P is negative: the sign column guarantees an even count of negative rational norms;
///   a negative product is an upstream kernel/sign-column bug.
/// - If `isqrt(&P)` returns `None`: P is not a perfect square, indicating an upstream
///   kernel/sign-column bug.
///
/// # Parameters
///
/// - `kv`: The kernel vector (a subset of filtered-matrix rows whose GF(2) sum is zero).
/// - `matrix`: The filtered sparse GF(2) matrix (carries the provenance map).
/// - `relations`: The original relation list (indexed by the provenance map).
/// - `poly`: The polynomial pair (provides `m` and `n`).
///
/// # Returns
///
/// X = isqrt(∏_{i ∈ S}(a_i − b_i·m)) mod N as a `BigInt`.
pub fn rational_sqrt(
    kv: &KernelVector,
    matrix: &SparseMatrix,
    relations: &[Relation],
    poly: &PolyPair,
) -> BigInt {
    // Step 1: recover the relation index set S.
    let s = kv.expand_provenance(matrix);

    // Step 2: form the product P = ∏_{i ∈ S}(a_i − b_i·m).
    let m = &poly.m;
    let mut product = BigInt::from(1i64);
    for &i in &s {
        let rel = &relations[i];
        // Rational norm factor: a_i − b_i·m.
        let factor = &rel.a - &rel.b * m;
        product *= factor;
    }

    // Step 3: assert P > 0. A negative product is an upstream G.E bug (sign column not honoured).
    if product.is_negative() {
        panic!(
            "rational_sqrt: rational norm product is negative — upstream kernel/sign-column bug \
             (the sign column in G.E must guarantee an even count of negative rational norms; \
             product = {product})"
        );
    }

    // Handle the degenerate case: empty S gives product = 1 (the empty product), which is fine.
    // isqrt(1) = Some(1), so this path is correct.

    // Step 4: extract the integer square root.
    let x = isqrt(&product).unwrap_or_else(|| {
        panic!(
            "rational_sqrt: rational norm product is not a perfect square — upstream \
             kernel/sign-column bug (product = {product})"
        )
    });

    // Step 5: reduce X mod N.
    let n = &poly.n;
    if n.is_zero() {
        // Degenerate: N = 0 means no modular reduction is meaningful; return x as-is.
        // This should not arise in practice but avoids a division-by-zero panic.
        return x;
    }
    let x_mod_n = x % n;
    // Ensure the result is in [0, N).
    if x_mod_n.is_negative() {
        x_mod_n + n
    } else {
        x_mod_n
    }
}
