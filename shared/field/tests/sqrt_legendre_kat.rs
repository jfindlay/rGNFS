//! Known-answer tests and property tests for ``Fp::legendre`` and ``Fp::sqrt``.
//!
//! Coverage:
//! - KAT: legendre values for hand-picked (a, p) pairs spanning QR/QNR/zero,
//!   over primes {5, 7, 13, 17, 1009, 1048517}.
//! - KAT: sqrt round-trip ``r^2 ≡ a (mod p)`` for residues; ``None`` for
//!   non-residues.
//! - Property: ``legendre(a, p) ∈ {-1, 0, 1}`` for all a in a small range.
//! - Property: multiplicativity ``legendre(a·b) == legendre(a)·legendre(b)``.
//! - Cross-check: ``FpNaive`` and ``FpMonty`` agree on all results.
//!
//! Primes exercised:
//! - p = 5   (≡ 1 mod 4, Tonelli–Shanks loop)
//! - p = 7   (≡ 3 mod 4, shortcut a^((p+1)/4))
//! - p = 13  (≡ 1 mod 4, Tonelli–Shanks loop)
//! - p = 17  (≡ 1 mod 4, Tonelli–Shanks loop)
//! - p = 1009        (≡ 1 mod 4, Tonelli–Shanks loop)
//! - p = 1048517     (≡ 1 mod 4, Tonelli–Shanks loop)

use crypto_bigint::Uint;
use proptest::prelude::*;
use shared_field::{Fp, FpMonty, FpNaive};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn p4(v: u64) -> Uint<4> {
    Uint::<4>::from(v)
}

/// Verify the Legendre symbol for one (a, p, expected) triple on both impls.
fn check_legendre(a: u64, p: u64, expected: i8) {
    let pu = p4(p);
    let an = FpNaive::<4>::from_u64(a, &pu);
    let am = FpMonty::<4>::from_u64(a, &pu);
    assert_eq!(
        an.legendre(&pu),
        expected,
        "FpNaive legendre({a}, {p}) expected {expected}"
    );
    assert_eq!(
        am.legendre(&pu),
        expected,
        "FpMonty legendre({a}, {p}) expected {expected}"
    );
}

/// Verify sqrt round-trip for a quadratic residue on both impls.
///
/// Checks that ``sqrt(a, p)`` returns ``Some(r)`` with ``r^2 ≡ a (mod p)``.
fn check_sqrt_residue(a: u64, p: u64) {
    let pu = p4(p);
    let au = p4(a);

    // FpNaive
    {
        let an = FpNaive::<4>::from_u64(a, &pu);
        let r = an.sqrt(&pu).unwrap_or_else(|| {
            panic!("FpNaive sqrt({a}, {p}) returned None for a QR")
        });
        let r2 = r.square(&pu);
        assert_eq!(
            r2.to_uint(),
            au.rem(&crypto_bigint::NonZero::new(pu).unwrap()),
            "FpNaive sqrt({a}, {p})^2 ≠ a"
        );
    }

    // FpMonty
    {
        let am = FpMonty::<4>::from_u64(a, &pu);
        let r = am.sqrt(&pu).unwrap_or_else(|| {
            panic!("FpMonty sqrt({a}, {p}) returned None for a QR")
        });
        let r2 = r.square(&pu);
        assert_eq!(
            r2.to_uint(),
            au.rem(&crypto_bigint::NonZero::new(pu).unwrap()),
            "FpMonty sqrt({a}, {p})^2 ≠ a"
        );
    }
}

/// Verify sqrt returns None for a quadratic non-residue on both impls.
fn check_sqrt_nonresidue(a: u64, p: u64) {
    let pu = p4(p);
    let an = FpNaive::<4>::from_u64(a, &pu);
    let am = FpMonty::<4>::from_u64(a, &pu);
    assert!(
        an.sqrt(&pu).is_none(),
        "FpNaive sqrt({a}, {p}) should be None for a QNR"
    );
    assert!(
        am.sqrt(&pu).is_none(),
        "FpMonty sqrt({a}, {p}) should be None for a QNR"
    );
}

// ── Legendre KATs ─────────────────────────────────────────────────────────────

#[test]
fn legendre_zero_is_zero() {
    // legendre(0, p) = 0 for any prime p.
    for &p in &[5u64, 7, 13, 17, 1009, 1048517] {
        check_legendre(0, p, 0);
    }
}

#[test]
fn legendre_p5() {
    // p = 5 ≡ 1 (mod 4).  QRs: {1, 4}.  QNRs: {2, 3}.
    let p = 5;
    check_legendre(1, p, 1);
    check_legendre(4, p, 1);
    check_legendre(2, p, -1);
    check_legendre(3, p, -1);
}

#[test]
fn legendre_p7() {
    // p = 7 ≡ 3 (mod 4).  QRs: {1, 2, 4}.  QNRs: {3, 5, 6}.
    let p = 7;
    check_legendre(1, p, 1);
    check_legendre(2, p, 1);
    check_legendre(4, p, 1);
    check_legendre(3, p, -1);
    check_legendre(5, p, -1);
    check_legendre(6, p, -1);
}

#[test]
fn legendre_p13() {
    // p = 13 ≡ 1 (mod 4).  QRs: {1, 3, 4, 9, 10, 12}.  QNRs: {2, 5, 6, 7, 8, 11}.
    let p = 13;
    for &a in &[1u64, 3, 4, 9, 10, 12] {
        check_legendre(a, p, 1);
    }
    for &a in &[2u64, 5, 6, 7, 8, 11] {
        check_legendre(a, p, -1);
    }
}

#[test]
fn legendre_p17() {
    // p = 17 ≡ 1 (mod 4).  QRs: {1, 2, 4, 8, 9, 13, 15, 16}.  QNRs: {3, 5, 6, 7, 10, 11, 12, 14}.
    let p = 17;
    for &a in &[1u64, 2, 4, 8, 9, 13, 15, 16] {
        check_legendre(a, p, 1);
    }
    for &a in &[3u64, 5, 6, 7, 10, 11, 12, 14] {
        check_legendre(a, p, -1);
    }
}

#[test]
fn legendre_p1009() {
    // p = 1009 ≡ 1 (mod 4).  Spot-check a few known values.
    // QRs mod 1009: 1^2=1, 2^2=4, 3^2=9, 31^2=961, 32^2=1024≡15.
    let p = 1009u64;
    for &a in &[1u64, 4, 9, 961, 15] {
        check_legendre(a, p, 1);
    }
    // p ≡ 1 (mod 8) so 2 is a QR.
    check_legendre(2, p, 1);
    // 11 is a QNR mod 1009: by quadratic reciprocity, legendre(11,1009) = legendre(1009,11)
    // = legendre(8,11) = legendre(2,11)^3.  Since 11 ≡ 3 (mod 8), legendre(2,11) = -1,
    // so legendre(11,1009) = (-1)^3 = -1.
    check_legendre(11, p, -1);
}

#[test]
fn legendre_p1048517() {
    // p = 1048517 ≡ 1 (mod 4).  Spot-check.
    // 1^2 = 1 is always a QR.
    let p = 1048517u64;
    check_legendre(1, p, 1);
    // 4 = 2^2 is always a QR.
    check_legendre(4, p, 1);
    // 9 = 3^2 is always a QR.
    check_legendre(9, p, 1);
    // 2 is a QR iff p ≡ ±1 (mod 8).  1048517 mod 8 = 5, so 2 is a QNR.
    check_legendre(2, p, -1);
}

// ── Sqrt KATs ─────────────────────────────────────────────────────────────────

#[test]
fn sqrt_zero() {
    // sqrt(0, p) = Some(0) for any prime p.
    for &p in &[5u64, 7, 13, 17, 1009, 1048517] {
        let pu = p4(p);
        let zn = FpNaive::<4>::zero(&pu);
        let zm = FpMonty::<4>::zero(&pu);
        assert_eq!(
            zn.sqrt(&pu).map(|r| r.to_uint()),
            Some(Uint::<4>::ZERO),
            "FpNaive sqrt(0, {p})"
        );
        assert_eq!(
            zm.sqrt(&pu).map(|r| r.to_uint()),
            Some(Uint::<4>::ZERO),
            "FpMonty sqrt(0, {p})"
        );
    }
}

#[test]
fn sqrt_residues_p5() {
    // p = 5 ≡ 1 (mod 4).  QRs: {1, 4}.
    check_sqrt_residue(1, 5);
    check_sqrt_residue(4, 5);
    check_sqrt_nonresidue(2, 5);
    check_sqrt_nonresidue(3, 5);
}

#[test]
fn sqrt_residues_p7() {
    // p = 7 ≡ 3 (mod 4), uses shortcut a^((p+1)/4).  QRs: {1, 2, 4}.
    check_sqrt_residue(1, 7);
    check_sqrt_residue(2, 7);
    check_sqrt_residue(4, 7);
    check_sqrt_nonresidue(3, 7);
    check_sqrt_nonresidue(5, 7);
    check_sqrt_nonresidue(6, 7);
}

#[test]
fn sqrt_residues_p13() {
    // p = 13 ≡ 1 (mod 4).  QRs: {1, 3, 4, 9, 10, 12}.
    for &a in &[1u64, 3, 4, 9, 10, 12] {
        check_sqrt_residue(a, 13);
    }
    for &a in &[2u64, 5, 6, 7, 8, 11] {
        check_sqrt_nonresidue(a, 13);
    }
}

#[test]
fn sqrt_residues_p17() {
    // p = 17 ≡ 1 (mod 4).  QRs: {1, 2, 4, 8, 9, 13, 15, 16}.
    for &a in &[1u64, 2, 4, 8, 9, 13, 15, 16] {
        check_sqrt_residue(a, 17);
    }
    for &a in &[3u64, 5, 6, 7, 10, 11, 12, 14] {
        check_sqrt_nonresidue(a, 17);
    }
}

#[test]
fn sqrt_residues_p1009() {
    // p = 1009 ≡ 1 (mod 4).  Test all perfect squares mod 1009.
    let p = 1009u64;

    // Collect all QRs by squaring 1..=(p-1)/2.
    let mut qrs: Vec<u64> = (1u64..=(p - 1) / 2)
        .map(|x| (x * x) % p)
        .collect();
    qrs.sort_unstable();
    qrs.dedup();

    for &a in &qrs {
        check_sqrt_residue(a, p);
    }

    // 11 is a QNR mod 1009 (see legendre_p1009 for derivation).
    check_sqrt_nonresidue(11, p);
}

#[test]
fn sqrt_residues_p1048517() {
    // p = 1048517 ≡ 1 (mod 4).  Spot-check a handful of residues.
    let p = 1048517u64;
    // Perfect squares: 1, 4, 9, 25, 100, 1024.
    for &a in &[1u64, 4, 9, 25, 100, 1024] {
        check_sqrt_residue(a, p);
    }
    // 2 is a QNR mod 1048517 (p ≡ 5 mod 8).
    check_sqrt_nonresidue(2, p);
}

// ── Property tests ────────────────────────────────────────────────────────────

/// Legendre symbol is always in {-1, 0, 1}.
#[test]
fn prop_legendre_range() {
    // Test over p = 101 (a small prime) for all a in 0..p.
    let p = 101u64;
    let pu = p4(p);
    for a in 0..p {
        let an = FpNaive::<4>::from_u64(a, &pu);
        let sym = an.legendre(&pu);
        assert!(
            sym == -1 || sym == 0 || sym == 1,
            "legendre({a}, {p}) = {sym} ∉ {{-1, 0, 1}}"
        );
    }
}

/// Multiplicativity: legendre(a·b, p) == legendre(a, p) · legendre(b, p).
#[test]
fn prop_legendre_multiplicative() {
    // Test over p = 17 for all pairs (a, b) — exhaustive at O(p^2) = 289 pairs.
    // p = 17 is large enough to cover QR×QR, QR×QNR, QNR×QNR, and zero cases.
    let p = 17u64;
    let pu = p4(p);

    for a in 0u64..p {
        for b in 0u64..p {
            let an = FpNaive::<4>::from_u64(a, &pu);
            let bn = FpNaive::<4>::from_u64(b, &pu);
            let ab = an.mul(&bn, &pu);

            let la = an.legendre(&pu) as i32;
            let lb = bn.legendre(&pu) as i32;
            let lab = ab.legendre(&pu) as i32;

            assert_eq!(
                lab,
                la * lb,
                "multiplicativity failed: legendre({a}·{b}, {p}) = {lab}, \
                 legendre({a},{p})·legendre({b},{p}) = {la}·{lb} = {}",
                la * lb
            );
        }
    }
}

// proptest: legendre range over random (a, p) pairs.
proptest! {
    #[test]
    fn proptest_legendre_range_random(a in 0u64..1009u64) {
        let p = 1009u64;
        let pu = p4(p);
        let an = FpNaive::<4>::from_u64(a, &pu);
        let sym = an.legendre(&pu);
        prop_assert!(sym == -1 || sym == 0 || sym == 1);
    }
}

// proptest: sqrt round-trip for quadratic residues.
proptest! {
    #[test]
    fn proptest_sqrt_roundtrip(x in 1u64..1009u64) {
        let p = 1009u64;
        let pu = p4(p);
        // a = x^2 mod p is always a QR.
        let xn = FpNaive::<4>::from_u64(x, &pu);
        let a = xn.square(&pu);
        let r = a.sqrt(&pu).expect("x^2 is always a QR");
        let r2 = r.square(&pu);
        prop_assert_eq!(r2.to_uint(), a.to_uint());
    }
}

// proptest: sqrt returns None for QNRs (verified by legendre).
proptest! {
    #[test]
    fn proptest_sqrt_none_for_qnr(a in 1u64..1009u64) {
        let p = 1009u64;
        let pu = p4(p);
        let an = FpNaive::<4>::from_u64(a, &pu);
        if an.legendre(&pu) == -1 {
            prop_assert!(an.sqrt(&pu).is_none());
        }
    }
}

// ── Cross-impl consistency ────────────────────────────────────────────────────

/// FpNaive and FpMonty agree on legendre and sqrt for all a in 0..p, p=17.
#[test]
fn naive_monty_agree_p17() {
    let p = 17u64;
    let pu = p4(p);
    for a in 0..p {
        let an = FpNaive::<4>::from_u64(a, &pu);
        let am = FpMonty::<4>::from_u64(a, &pu);

        let ln = an.legendre(&pu);
        let lm = am.legendre(&pu);
        assert_eq!(ln, lm, "legendre({a}, {p}): naive={ln} monty={lm}");

        let sn = an.sqrt(&pu).map(|r| r.to_uint());
        let sm = am.sqrt(&pu).map(|r| r.to_uint());

        // Both should agree on Some vs None.
        assert_eq!(
            sn.is_some(),
            sm.is_some(),
            "sqrt({a}, {p}): naive={sn:?} monty={sm:?}"
        );

        // If both Some, verify r^2 ≡ a for each (they may differ by sign).
        if let (Some(rn), Some(rm)) = (sn, sm) {
            let a_reduced = p4(a % p);
            let rn_sq = FpNaive::<4>::from_uint(rn, &pu).square(&pu).to_uint();
            let rm_sq = FpMonty::<4>::from_uint(rm, &pu).square(&pu).to_uint();
            assert_eq!(rn_sq, a_reduced, "FpNaive sqrt({a},{p})^2 ≠ a");
            assert_eq!(rm_sq, a_reduced, "FpMonty sqrt({a},{p})^2 ≠ a");
        }
    }
}
