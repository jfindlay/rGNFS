//! Montgomery's batched inversion trick, generic over ``Fp<L>``.
//!
//! Computes ``n`` modular inverses using a single inversion and ``3(n-1)``
//! multiplications.

use crypto_bigint::Uint;
use shared_field::Fp;

/// Compute modular inverses of all elements in ``xs`` in-place.
///
/// Uses Montgomery's batched inversion trick: 1 inversion and 3(n−1)
/// multiplications for a batch of n elements.  All elements in ``xs`` must be
/// non-zero.
///
/// Algorithm:
///
/// 1. Forward pass: compute prefix products ``p[i] = x[0]·x[1]·…·x[i]``.
/// 2. Invert the last prefix product: ``inv_all = 1 / p[n-1]``.
/// 3. Backward pass: working from the right,
///    ``x[i]_inv = inv_all · p[i-1];  inv_all = inv_all · x[i]``.
///
/// # Panics
///
/// Panics if ``xs`` is empty or any element is zero.
pub fn batch_invert<const L: usize, F: Fp<L>>(xs: &mut [F], p: &Uint<L>) {
    let n = xs.len();
    if n == 0 {
        return;
    }

    // Forward pass: build prefix products.
    // prefix[i] = x[0] * x[1] * ... * x[i]
    let mut prefix = Vec::with_capacity(n);
    prefix.push(xs[0].clone());
    for i in 1..n {
        let prev = prefix[i - 1].clone();
        prefix.push(prev.mul(&xs[i], p));
    }

    // Invert the total product (one field inversion).
    let mut inv_acc = prefix[n - 1].inv(p);

    // Backward pass: recover each individual inverse.
    for i in (1..n).rev() {
        // x[i]^{-1} = inv_acc * prefix[i-1]
        let xi_inv = inv_acc.mul(&prefix[i - 1], p);
        // Advance accumulator: inv_acc = inv_acc * x[i]  (so it becomes prefix[i-1]^{-1})
        inv_acc = inv_acc.mul(&xs[i], p);
        xs[i] = xi_inv;
    }
    // x[0]^{-1} = inv_acc (which is now prefix[0]^{-1} = x[0]^{-1})
    xs[0] = inv_acc;
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_field::FpMonty;

    fn p_small() -> Uint<4> {
        // Use tiny_a's field prime.
        Uint::<4>::from(1_048_517u64)
    }

    /// batch_invert on a single element matches F::inv.
    #[test]
    fn batch_inv_single() {
        let p = p_small();
        let mut xs = vec![FpMonty::<4>::from_u64(7, &p)];
        let expected = xs[0].inv(&p);
        batch_invert(&mut xs, &p);
        assert_eq!(xs[0], expected, "batch_invert single element mismatch");
    }

    /// batch_invert on multiple elements matches individual F::inv calls.
    #[test]
    fn batch_inv_matches_individual() {
        let p = p_small();
        let vals: &[u64] = &[1, 2, 3, 5, 7, 11, 13, 100, 999, 1_048_516];
        let mut xs: Vec<FpMonty<4>> = vals.iter().map(|&v| FpMonty::<4>::from_u64(v, &p)).collect();
        let expected: Vec<FpMonty<4>> = xs.iter().map(|x| x.inv(&p)).collect();

        batch_invert(&mut xs, &p);

        for (i, (got, want)) in xs.iter().zip(expected.iter()).enumerate() {
            assert_eq!(got, want, "batch_invert mismatch at index {i} (val={})", vals[i]);
        }
    }

    /// batch_invert is a no-op on an empty slice.
    #[test]
    fn batch_inv_empty() {
        let p = p_small();
        let mut xs: Vec<FpMonty<4>> = vec![];
        batch_invert(&mut xs, &p); // must not panic
    }
}
