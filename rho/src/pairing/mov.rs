//! MOV bridge: rho-side entry for encoding pairing outputs as `solve_dl` inputs.
//!
//! # Contract C-MovBridge (E.C.1)
//!
//! This module is the rho-side half of the E.C MOV bridge. It converts a pairing output
//! [`FpExt<F>`] into the base-p [`BigInt`] encoding that [`gnfs::dl::solve_dl`] consumes
//! at k=2.
//!
//! The encoding convention belongs to gnfs (C2-ext, frozen D.E.3). The rho side produces
//! only the coefficient `Vec<BigInt>` and delegates the encoding to the gnfs-side helper
//! [`gnfs::dl::ext::target::fpext_coeffs_to_dl_target`]. The rho side does NOT re-derive
//! the base-p encoding itself.
//!
//! # Modulus-consistency guard
//!
//! The gnfs-side helper asserts that the pairing's irreducible modulus matches what
//! `find_irreducible_degree2(p)` returns. This guard is mandatory: without it, a prime p
//! where `find_irreducible_degree2` picks a different irreducible than the pairing's modulus
//! would compute a DL in a different F_{p²} and return a wrong discrete log with no error.
//!
//! # Principle-4 boundary
//!
//! The `Uint<4> → BigInt` conversion assumes p < 2^64 (single limb). This is a toy-scale
//! boundary: at crypto scale (p ~ 2^256), the full limb vector would be needed.

use crypto_bigint::Uint;
use num_bigint::BigInt;
use shared_field::Fp;

use crate::pairing::fpext::{FpExt, IrreducibleModulus};

/// Convert a `FpExt<F>` pairing output to the base-p `BigInt` encoding `solve_dl` consumes.
///
/// Extracts the coefficient vector from `elem` via `to_uint_vec`, converts each `Uint<4>` to
/// `BigInt` (toy-only: single-limb, p < 2^64), then delegates the base-p encoding and
/// modulus-consistency guard to the gnfs-side helper
/// [`gnfs::dl::ext::target::fpext_coeffs_to_dl_target`].
///
/// # Arguments
///
/// - `elem`: The pairing output element in F_{p^k}.
/// - `modulus`: The irreducible modulus the pairing used (carried by the pairing fixture).
/// - `p`: The base-field prime as `u64`.
///
/// # Panics
///
/// Panics if the pairing's modulus does not match what `find_irreducible_degree2(p)` returns
/// (forwarded from the gnfs-side modulus-consistency guard).
pub fn fpext_to_bigint<F: Fp<4>>(
    elem: &FpExt<F>,
    modulus: &IrreducibleModulus<F>,
    p: u64,
) -> BigInt {
    let p_big = BigInt::from(p);

    // Step 1: extract coefficient vector as Vec<Uint<4>>.
    let uint_vec: Vec<Uint<4>> = elem.to_uint_vec();

    // Step 2: convert each Uint<4> to BigInt.
    // Toy-only (principle-4 boundary): assumes p < 2^64 (single limb). A crypto-scale p
    // needs the full limb vector.
    let coeffs: Vec<BigInt> = uint_vec
        .iter()
        .map(|u| {
            let words = u.as_words();
            debug_assert!(
                words[1] == 0 && words[2] == 0 && words[3] == 0,
                "fpext_to_bigint: Uint<4> has non-zero upper limbs — p >= 2^64 is not supported \
                 at toy scale (principle-4 boundary)"
            );
            BigInt::from(words[0])
        })
        .collect();

    // Step 3: convert the pairing's IrreducibleModulus<F> coefficients to Vec<BigInt>.
    // The modulus is passed to the gnfs-side helper for the modulus-consistency guard.
    let modulus_bigint: Vec<BigInt> = modulus
        .coeffs
        .iter()
        .map(|c| {
            let uint_val = c.to_uint();
            let words = uint_val.as_words();
            debug_assert!(
                words[1] == 0 && words[2] == 0 && words[3] == 0,
                "fpext_to_bigint: modulus coefficient Uint<4> has non-zero upper limbs"
            );
            BigInt::from(words[0])
        })
        .collect();

    // Step 4: delegate to the gnfs-side helper (encoding convention + modulus-consistency guard).
    gnfs::dl::ext::target::fpext_coeffs_to_dl_target(coeffs, &p_big, &modulus_bigint)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use num_bigint::BigInt;
    use shared_field::FpNaive;

    use crate::pairing::fpext::IrreducibleModulus;
    use crate::pairing::test_curves::{PAIRING_TOY_P, pairing_toy};

    // ── Fixture helpers ───────────────────────────────────────────────────────

    fn p47() -> Uint<4> {
        Uint::<4>::from(PAIRING_TOY_P)
    }

    fn modulus_k2() -> IrreducibleModulus<FpNaive<4>> {
        let p = p47();
        IrreducibleModulus::new(
            vec![
                FpNaive::<4>::from_u64(1, &p),
                FpNaive::<4>::from_u64(0, &p),
                FpNaive::<4>::from_u64(1, &p),
            ],
            &p,
        )
    }

    fn fp2(c0: u64, c1: u64) -> FpExt<FpNaive<4>> {
        let p = p47();
        FpExt { coeffs: vec![FpNaive::<4>::from_u64(c0, &p), FpNaive::<4>::from_u64(c1, &p)] }
    }

    // ── KAT 1: Round-trip ─────────────────────────────────────────────────────

    /// KAT: a known μ_ℓ element (23, 6) at p=47 → bridge → base-p BigInt = 23 + 6*47 = 305.
    ///
    /// This matches the dl_ext_kat.rs encoding: ζ = (23, 6) → BigInt(305).
    #[test]
    fn kat_round_trip_zeta() {
        let elem = fp2(23, 6);
        let modulus = modulus_k2();
        let result = fpext_to_bigint(&elem, &modulus, PAIRING_TOY_P);
        assert_eq!(
            result,
            BigInt::from(23 + 6 * 47i64),
            "round-trip: (23, 6) at p=47 should encode to 23 + 6*47 = 305"
        );
    }

    /// KAT: ζ² = (23, 41) at p=47 → bridge → base-p BigInt = 23 + 41*47 = 1950.
    ///
    /// Matches dl_ext_kat.rs: zeta_sq() = BigInt(1950).
    #[test]
    fn kat_round_trip_zeta_sq() {
        let elem = fp2(23, 41);
        let modulus = modulus_k2();
        let result = fpext_to_bigint(&elem, &modulus, PAIRING_TOY_P);
        assert_eq!(
            result,
            BigInt::from(23 + 41 * 47i64),
            "round-trip: (23, 41) at p=47 should encode to 23 + 41*47 = 1950"
        );
    }

    /// KAT: identity element (1, 0) at p=47 → bridge → BigInt(1).
    #[test]
    fn kat_round_trip_identity() {
        let elem = fp2(1, 0);
        let modulus = modulus_k2();
        let result = fpext_to_bigint(&elem, &modulus, PAIRING_TOY_P);
        assert_eq!(result, BigInt::from(1i64), "identity (1, 0) should encode to 1");
    }

    // ── KAT 2: Modulus-consistency guard fires ────────────────────────────────

    /// KAT: a wrong modulus causes a panic (modulus-consistency guard).
    ///
    /// The correct modulus for p=47 is u²+1 = [1, 0, 1]. We supply [1, 1, 1] (u²+u+1),
    /// which differs from what find_irreducible_degree2(47) returns, so the guard must panic.
    #[test]
    fn kat_modulus_guard_fires_on_wrong_modulus() {
        let p = p47();
        // Wrong modulus: u²+u+1 = [1, 1, 1] (not what find_irreducible_degree2(47) returns).
        let wrong_modulus = IrreducibleModulus::new(
            vec![
                FpNaive::<4>::from_u64(1, &p),
                FpNaive::<4>::from_u64(1, &p),
                FpNaive::<4>::from_u64(1, &p),
            ],
            &p,
        );
        let elem = fp2(23, 6);
        let result = std::panic::catch_unwind(|| {
            fpext_to_bigint(&elem, &wrong_modulus, PAIRING_TOY_P)
        });
        assert!(
            result.is_err(),
            "fpext_to_bigint with wrong modulus should panic (modulus-consistency guard)"
        );
    }

    // ── KAT 3: Uint<4> → BigInt step ─────────────────────────────────────────

    /// KAT: Uint<4> with value 23 converts to BigInt::from(23).
    ///
    /// Verifies the single-limb extraction step used in fpext_to_bigint.
    #[test]
    fn kat_uint4_to_bigint_single_limb() {
        let u = Uint::<4>::from(23u64);
        let words = u.as_words();
        // Upper limbs must be zero for toy-scale p < 2^64.
        assert_eq!(words[1], 0, "upper limb 1 should be zero");
        assert_eq!(words[2], 0, "upper limb 2 should be zero");
        assert_eq!(words[3], 0, "upper limb 3 should be zero");
        let b = BigInt::from(words[0]);
        assert_eq!(b, BigInt::from(23i64), "Uint<4>(23) should convert to BigInt(23)");
    }

    /// KAT: Uint<4> with value 0 converts to BigInt::from(0).
    #[test]
    fn kat_uint4_to_bigint_zero() {
        let u = Uint::<4>::from(0u64);
        let words = u.as_words();
        let b = BigInt::from(words[0]);
        assert_eq!(b, BigInt::from(0i64), "Uint<4>(0) should convert to BigInt(0)");
    }

    /// KAT: Uint<4> with value 46 (= p-1 for p=47) converts to BigInt::from(46).
    #[test]
    fn kat_uint4_to_bigint_p_minus_1() {
        let u = Uint::<4>::from(46u64);
        let words = u.as_words();
        let b = BigInt::from(words[0]);
        assert_eq!(b, BigInt::from(46i64), "Uint<4>(46) should convert to BigInt(46)");
    }

    // ── KAT 4: pairing_toy fixture integration ────────────────────────────────

    /// KAT: the bridge is consistent with the pairing_toy fixture modulus.
    ///
    /// Uses the actual pairing_toy() modulus (not a hand-constructed one) to verify
    /// the bridge works end-to-end with the fixture.
    #[test]
    fn kat_pairing_toy_modulus_consistent() {
        let (_curve, modulus, _ell, _p_point, _q_point) = pairing_toy();
        let elem = fp2(23, 6);
        let result = fpext_to_bigint(&elem, &modulus, PAIRING_TOY_P);
        // ζ = (23, 6) → 23 + 6*47 = 305.
        assert_eq!(
            result,
            BigInt::from(305i64),
            "pairing_toy modulus: (23, 6) should encode to 305"
        );
    }
}
