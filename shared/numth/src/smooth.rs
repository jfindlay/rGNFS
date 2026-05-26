//! B-smoothness detection via trial division over a prime factor base.
//!
//! # Design for three consumers (Contract C1)
//!
//! This module is designed with three downstream consumers in mind:
//!
//! - **G.C** (NFS sieving): tests whether sieve values — integers of the form
//!   ``f(a, b)`` for a polynomial ``f`` — are smooth over a prime factor base
//!   of size ``B``.  The [`SmoothWitness`] carries the complete factorisation
//!   needed to build the exponent vector for the linear-algebra step.
//! - **D.A** (NFS-DL relation collection): structurally identical to G.C but
//!   operating over a different polynomial and factor base.  The same
//!   [`trial_smooth`] function and [`SmoothWitness`] type serve both.
//! - **E.K** (ECDLP index calculus via Semaev polynomials): evaluates a Semaev
//!   summation polynomial at candidate points to obtain an integer; tests that
//!   integer for smoothness over a factor base of small primes.  The
//!   [`SmoothWitness`] is then used to build a row in the relation matrix, with
//!   each ``(prime, exponent)`` pair mapping to a column in the matrix.  The
//!   interface is structurally identical to G.C/D.A: call [`trial_smooth`] with
//!   the evaluated integer and the factor base, inspect ``cofactor`` to decide
//!   whether to accept the relation (full smoothness) or discard it (partial
//!   smoothness with a large prime cofactor).  No extension to the
//!   [`SmoothWitness`] type is required for E.K; the curve-point provenance is
//!   tracked by the caller, not the witness.
//!
//! # Sieve for ``factor_base_up_to``
//!
//! The current implementation uses [`is_prime`] for primality testing, which
//! is ``O(B log B)`` in the number of candidates tested.  For large ``B``
//! (e.g., ``B > 10^6``), a sieve of Eratosthenes would be significantly faster.
//! This is noted as a TODO; the sieve will be added in a future session.

use crypto_bigint::Uint;

use crate::prime::is_prime;

// ── SmoothWitness ─────────────────────────────────────────────────────────────

/// A witness that an integer is B-smooth: its complete factorisation over the factor base.
///
/// ``factors`` is a sorted list of ``(prime, exponent)`` pairs.  The product of
/// ``p^e`` for all pairs, multiplied by ``cofactor``, equals the original
/// integer (verified by [`SmoothWitness::verify`]).  Primes outside the factor
/// base do not appear; if the integer is not fully factored, ``cofactor`` holds
/// the unfactored remainder (always 1 if the integer is truly B-smooth).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothWitness {
    /// Factored part: sorted ``(prime, exponent)`` pairs.
    pub factors: Vec<(u64, u32)>,
    /// Unfactored cofactor (1 if B-smooth, > 1 if partial).
    pub cofactor: Uint<4>,
}

impl SmoothWitness {
    /// Return ``true`` if the cofactor is 1 (fully B-smooth witness).
    pub fn is_smooth(&self) -> bool {
        self.cofactor == Uint::<4>::ONE
    }

    /// Reconstruct the original integer from the witness.
    ///
    /// Computes ``cofactor * prod(p^e for (p, e) in factors)``.
    pub fn product(&self) -> Uint<4> {
        let mut acc = self.cofactor;
        for &(p, e) in &self.factors {
            let p_uint = Uint::<4>::from(p);
            for _ in 0..e {
                acc = acc.wrapping_mul(&p_uint);
            }
        }
        acc
    }

    /// Verify that ``self.product() == n``.
    ///
    /// Returns ``true`` if the witness is consistent with ``n``.
    pub fn verify(&self, n: &Uint<4>) -> bool {
        self.product() == *n
    }
}

// ── trial_smooth ──────────────────────────────────────────────────────────────

/// Attempt to factor ``n`` over a prime factor base using trial division.
///
/// Returns a [`SmoothWitness`].  If ``n`` is B-smooth (all prime factors ≤ B),
/// the witness's ``cofactor`` is 1.  If ``n`` has a large prime factor,
/// ``cofactor > 1`` holds the unfactored remainder.  The caller decides whether
/// to accept partial smoothness.
///
/// ``factor_base`` must be a sorted slice of primes ≤ B.
///
/// # Special cases
///
/// - ``n == 0``: returns ``factors = []``, ``cofactor = 0``.
/// - ``n == 1``: returns ``factors = []``, ``cofactor = 1`` (trivially smooth).
pub fn trial_smooth(n: &Uint<4>, factor_base: &[u64]) -> SmoothWitness {
    if *n == Uint::<4>::ZERO {
        return SmoothWitness { factors: vec![], cofactor: Uint::<4>::ZERO };
    }

    let mut remainder = *n;
    let mut factors: Vec<(u64, u32)> = Vec::new();

    for &p in factor_base {
        if remainder == Uint::<4>::ONE {
            break;
        }

        let p_uint = Uint::<4>::from(p);
        // Check if p divides remainder.
        let (_, rem) = remainder.div_rem(&crypto_bigint::NonZero::new(p_uint).unwrap());
        if rem != Uint::<4>::ZERO {
            continue;
        }

        // p divides remainder; extract all powers of p.
        let mut exp = 0u32;
        loop {
            let (q, r) = remainder.div_rem(&crypto_bigint::NonZero::new(p_uint).unwrap());
            if r != Uint::<4>::ZERO {
                break;
            }
            remainder = q;
            exp += 1;
        }
        factors.push((p, exp));
    }

    SmoothWitness { factors, cofactor: remainder }
}

// ── factor_base_up_to ─────────────────────────────────────────────────────────

/// Generate a prime factor base: all primes ≤ B.
///
/// Uses [`is_prime`] for primality testing.  For large ``B`` this may be slow;
/// a sieve of Eratosthenes will be added in a future session (noted as a gap).
///
/// # TODO
///
/// Replace the inner loop with a sieve of Eratosthenes for ``B > 10^6``.
/// The current trial-division approach via ``is_prime`` is ``O(B * sqrt(B))``
/// in the worst case; a sieve is ``O(B log log B)``.
pub fn factor_base_up_to(bound: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    if bound < 2 {
        return primes;
    }
    for candidate in 2..=bound {
        if is_prime(&Uint::<4>::from(candidate)) {
            primes.push(candidate);
        }
    }
    primes
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn u(v: u64) -> Uint<4> {
        Uint::<4>::from(v)
    }

    // ── KAT: specific factorisation examples ──────────────────────────────────

    /// ``trial_smooth(12, [2,3,5])`` → ``factors=[(2,2),(3,1)]``, ``cofactor=1``.
    #[test]
    fn smooth_12_over_235() {
        let fb = vec![2u64, 3, 5];
        let w = trial_smooth(&u(12), &fb);
        assert_eq!(w.factors, vec![(2, 2), (3, 1)]);
        assert_eq!(w.cofactor, Uint::<4>::ONE);
        assert!(w.is_smooth());
        assert!(w.verify(&u(12)));
    }

    /// ``trial_smooth(60, [2,3,5])`` → ``factors=[(2,2),(3,1),(5,1)]``, ``cofactor=1``.
    #[test]
    fn smooth_60_over_235() {
        let fb = vec![2u64, 3, 5];
        let w = trial_smooth(&u(60), &fb);
        assert_eq!(w.factors, vec![(2, 2), (3, 1), (5, 1)]);
        assert_eq!(w.cofactor, Uint::<4>::ONE);
        assert!(w.is_smooth());
        assert!(w.verify(&u(60)));
    }

    /// ``trial_smooth(77, [2,3,5,7])`` → ``factors=[(7,1)]``, ``cofactor=11``.
    ///
    /// 77 = 7 * 11.  Factor base {2,3,5,7}: 7 is in the base, 11 is not.
    #[test]
    fn partial_77_over_2357() {
        let fb = vec![2u64, 3, 5, 7];
        let w = trial_smooth(&u(77), &fb);
        assert_eq!(w.factors, vec![(7, 1)]);
        assert_eq!(w.cofactor, u(11));
        assert!(!w.is_smooth());
        assert!(w.verify(&u(77)));
    }

    /// A prime ``p > B`` is not smooth: ``factors=[]``, ``cofactor=p``.
    #[test]
    fn prime_larger_than_base() {
        let fb = vec![2u64, 3, 5, 7];
        // 11 is prime and > 7 (the largest base element).
        let w = trial_smooth(&u(11), &fb);
        assert_eq!(w.factors, vec![]);
        assert_eq!(w.cofactor, u(11));
        assert!(!w.is_smooth());
        assert!(w.verify(&u(11)));
    }

    /// ``trial_smooth(1, ...)`` → trivially smooth with empty factors.
    #[test]
    fn smooth_one() {
        let fb = vec![2u64, 3, 5];
        let w = trial_smooth(&u(1), &fb);
        assert_eq!(w.factors, vec![]);
        assert_eq!(w.cofactor, Uint::<4>::ONE);
        assert!(w.is_smooth());
        assert!(w.verify(&u(1)));
    }

    // ── KAT: round-trip verify ────────────────────────────────────────────────

    /// Verify that ``witness.verify(n)`` holds for a variety of inputs.
    #[test]
    fn round_trip_verify() {
        let fb = factor_base_up_to(50);
        let test_cases: &[u64] = &[1, 2, 3, 4, 6, 8, 12, 24, 30, 60, 120, 360, 720, 1024, 2310];
        for &n in test_cases {
            let w = trial_smooth(&u(n), &fb);
            assert!(w.verify(&u(n)), "round-trip failed for n={n}");
        }
    }

    // ── Property: matches brute-force for n in 1..1000 ───────────────────────

    /// For ``n`` in 1..1000, ``trial_smooth(n, factor_base_up_to(50))`` matches
    /// a brute-force trial-division reference.
    #[test]
    fn matches_brute_force_up_to_1000() {
        let fb = factor_base_up_to(50);
        for n in 1u64..1000 {
            let w = trial_smooth(&u(n), &fb);

            // Brute-force: factor n over the same base.
            let (expected_factors, expected_cofactor) = brute_factor(n, &fb);

            assert_eq!(
                w.factors, expected_factors,
                "factors mismatch for n={n}: got {:?}, expected {:?}",
                w.factors, expected_factors
            );
            assert_eq!(
                w.cofactor,
                u(expected_cofactor),
                "cofactor mismatch for n={n}: got {:?}, expected {expected_cofactor}",
                w.cofactor
            );
            // Also verify the round-trip.
            assert!(w.verify(&u(n)), "verify failed for n={n}");
        }
    }

    /// Brute-force factorisation reference: trial-divide by each prime in the base.
    fn brute_factor(mut n: u64, factor_base: &[u64]) -> (Vec<(u64, u32)>, u64) {
        let mut factors = Vec::new();
        for &p in factor_base {
            if n == 1 {
                break;
            }
            if n % p == 0 {
                let mut exp = 0u32;
                while n % p == 0 {
                    n /= p;
                    exp += 1;
                }
                factors.push((p, exp));
            }
        }
        (factors, n)
    }

    // ── KAT: factor_base_up_to ────────────────────────────────────────────────

    /// Verify that ``factor_base_up_to(20)`` returns the correct primes.
    #[test]
    fn factor_base_up_to_20() {
        let fb = factor_base_up_to(20);
        assert_eq!(fb, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    /// Verify that ``factor_base_up_to(0)`` and ``factor_base_up_to(1)`` are empty.
    #[test]
    fn factor_base_empty_for_small_bound() {
        assert!(factor_base_up_to(0).is_empty());
        assert!(factor_base_up_to(1).is_empty());
    }

    /// Verify that ``factor_base_up_to(2)`` returns ``[2]``.
    #[test]
    fn factor_base_up_to_2() {
        assert_eq!(factor_base_up_to(2), vec![2u64]);
    }

    // ── KAT: SmoothWitness::product ──────────────────────────────────────────

    /// Verify that ``SmoothWitness::product`` reconstructs the original integer.
    #[test]
    fn product_reconstruction() {
        // 360 = 2^3 * 3^2 * 5
        let w = SmoothWitness {
            factors: vec![(2, 3), (3, 2), (5, 1)],
            cofactor: Uint::<4>::ONE,
        };
        assert_eq!(w.product(), u(360));
        assert!(w.verify(&u(360)));
    }

    /// Verify ``product`` with a non-unit cofactor.
    #[test]
    fn product_with_cofactor() {
        // 77 = 7 * 11; witness has factors=[(7,1)], cofactor=11.
        let w = SmoothWitness { factors: vec![(7, 1)], cofactor: u(11) };
        assert_eq!(w.product(), u(77));
        assert!(w.verify(&u(77)));
    }
}
