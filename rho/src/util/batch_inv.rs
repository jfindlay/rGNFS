//! Montgomery's batched inversion trick.
//!
//! Re-exports ``batch_invert`` from ``shared-bigint``, fixing the limb count
//! to ``L = 4`` for backward compatibility with the rest of ``rho``.

pub use shared_bigint::batch_invert;

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::field::{Fp, FpMonty};

    fn p_small() -> Uint<4> {
        // Use tiny_a's field prime.
        Uint::<4>::from(1_048_517u64)
    }

    /// batch_invert on a single element matches F::inv.
    #[test]
    fn batch_inv_single() {
        let p = p_small();
        let mut xs = vec![FpMonty::from_u64(7, &p)];
        let expected = xs[0].inv(&p);
        batch_invert(&mut xs, &p);
        assert_eq!(xs[0], expected, "batch_invert single element mismatch");
    }

    /// batch_invert on multiple elements matches individual F::inv calls.
    #[test]
    fn batch_inv_matches_individual() {
        let p = p_small();
        let vals: &[u64] = &[1, 2, 3, 5, 7, 11, 13, 100, 999, 1_048_516];
        let mut xs: Vec<FpMonty> = vals.iter().map(|&v| FpMonty::from_u64(v, &p)).collect();
        let expected: Vec<FpMonty> = xs.iter().map(|x| x.inv(&p)).collect();

        batch_invert(&mut xs, &p);

        for (i, (got, want)) in xs.iter().zip(expected.iter()).enumerate() {
            assert_eq!(got, want, "batch_invert mismatch at index {i} (val={})", vals[i]);
        }
    }

    /// batch_invert is a no-op on an empty slice.
    #[test]
    fn batch_inv_empty() {
        let p = p_small();
        let mut xs: Vec<FpMonty> = vec![];
        batch_invert(&mut xs, &p); // must not panic
    }
}
