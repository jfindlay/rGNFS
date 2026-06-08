//! Known-answer tests (KATs) for the Schirokauer map (D.A.1).
//!
//! Three KATs are required by the D.A.1 session spec:
//!
//! (a) **Known-value KAT**: evaluate the Schirokauer map on hand-computed toy number-field
//!     elements with known ℓ-adic coordinates.
//!
//! (b) **Homomorphism KAT**: verify λ(xy) = λ(x) + λ(y) mod ℓ on a small sample of elements.
//!
//! (c) **Width KAT**: compute the toy-scale NFS-DL norm bound and assert it fits `Uint<4>`
//!     (256 bits). This is the confirm-record obligation per the ROADMAP C1 width policy.
//!
//! # Setup
//!
//! All KATs use:
//! - Number field K = ℚ(i) = ℚ[α]/(α² + 1), so α² = −1.
//! - ℓ = 5 (target subgroup order).
//! - Prime ideal φ = (41, α − 9): 41 ≡ 1 (mod 5), and 9² + 1 = 82 = 2·41 ≡ 0 (mod 41).
//! - ε = (41 − 1)/5 = 8.
//!
//! # Hand-computed reference values
//!
//! For β = 1 + α (i.e., 1 + i in ℚ(i)):
//! - β^8 = (1+i)^8 = 16 (since (1+i)^2 = 2i, (1+i)^4 = −4, (1+i)^8 = 16).
//! - 16 mod ℓ² = 16 mod 25 = 16.
//! - δ = 16 − 1 = 15.
//! - 15 / ℓ = 15 / 5 = 3.
//! - Evaluate at α ≡ 9 mod 5 = 4: constant 3 → λ(1+α) = 3.
//!
//! For β = 2 (rational integer):
//! - β^8 = 256.
//! - 256 mod 25 = 6.
//! - δ = 6 − 1 = 5.
//! - 5 / 5 = 1.
//! - λ(2) = 1.
//!
//! For β = α (the primitive element i):
//! - β^8 = α^8 = 1 (since α^4 = 1 in ℚ(i)).
//! - 1 mod 25 = 1. δ = 0. 0/5 = 0. λ(α) = 0.

use crypto_bigint::Uint;
use gnfs::compute_schirokauer;
use num_bigint::BigInt;
use num_integer::Integer;
use shared_numfield::{Ideal, IntPoly, NumberField};

// ─── Setup helpers ────────────────────────────────────────────────────────────

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// K = ℚ(i), f = x² + 1.
fn field_qi() -> NumberField {
    // coeffs: [1, 0, 1] → 1 + 0·x + 1·x² = x² + 1
    NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
}

// ─── KAT (a): Known-value KAT ─────────────────────────────────────────────────

/// KAT (a): Schirokauer map evaluates correctly on hand-computed toy elements.
///
/// Verifies:
/// - λ(1 + α) = 3 (hand-computed: (1+i)^8 = 16, (16−1)/5 = 3).
/// - λ(α) = 0 (hand-computed: α^8 = 1, (1−1)/5 = 0).
/// - λ(2) = 1 (hand-computed: 2^8 = 256, (256 mod 25 − 1)/5 = (6−1)/5 = 1).
#[test]
fn kat_a_known_value() {
    let k = field_qi();
    let ell = bi(5);

    // Prime ideal φ = (41, α − 9): 41 ≡ 1 (mod 5), 9² + 1 = 82 = 2·41 ≡ 0 (mod 41).
    let phi = Ideal::new(&k, bi(41), bi(9));
    let ideals = [phi];

    // β₁ = 1 + α: λ should be 3.
    let beta1 = k.from_int(bi(1)).add(&k.alpha());
    let lambda1 = compute_schirokauer(&beta1, &ell, &ideals).expect("Schirokauer map should succeed");
    assert_eq!(lambda1.len(), 1, "one ideal → one coordinate");
    assert_eq!(lambda1[0], bi(3), "λ(1+α) should be 3");

    // β₂ = α: λ should be 0.
    let beta2 = k.alpha();
    let lambda2 = compute_schirokauer(&beta2, &ell, &ideals).expect("Schirokauer map should succeed");
    assert_eq!(lambda2[0], bi(0), "λ(α) should be 0");

    // β₃ = 2: λ should be 1.
    let beta3 = k.from_int(bi(2));
    let lambda3 = compute_schirokauer(&beta3, &ell, &ideals).expect("Schirokauer map should succeed");
    assert_eq!(lambda3[0], bi(1), "λ(2) should be 1");
}

// ─── KAT (b): Homomorphism KAT ────────────────────────────────────────────────

/// KAT (b): Schirokauer map is a group homomorphism: λ(xy) = λ(x) + λ(y) mod ℓ.
///
/// Verifies the homomorphism property on three pairs:
/// - (1+α, 2): λ(2+2α) = λ(1+α) + λ(2) = 3 + 1 = 4.
/// - (α, 1+α): λ(−1+α) = λ(α) + λ(1+α) = 0 + 3 = 3.
/// - (2, 2): λ(4) = λ(2) + λ(2) = 1 + 1 = 2.
///
/// Hand-computed reference:
/// - (2+2α)^8 = 2^8 · (1+α)^8 = 256 · 16 = 4096. 4096 mod 25 = 21. (21−1)/5 = 4.
/// - (−1+α)^8 = 16 (same as (1+α)^8 since (−1+i)^8 = 16). (16−1)/5 = 3.
/// - 4^8 = 65536. 65536 mod 25 = 11. (11−1)/5 = 2.
#[test]
fn kat_b_homomorphism() {
    let k = field_qi();
    let ell = bi(5);
    let phi = Ideal::new(&k, bi(41), bi(9));
    let ideals = [phi];

    // Helper: compute λ for a single element.
    let lambda = |elt: &shared_numfield::NumberFieldElement<'_>| -> BigInt {
        compute_schirokauer(elt, &ell, &ideals).expect("Schirokauer map should succeed")[0].clone()
    };

    // β₁ = 1 + α, β₂ = 2, β₁·β₂ = 2 + 2α.
    let beta1 = k.from_int(bi(1)).add(&k.alpha());
    let beta2 = k.from_int(bi(2));
    let beta12 = beta1.mul(&beta2);
    let lam1 = lambda(&beta1);
    let lam2 = lambda(&beta2);
    let lam12 = lambda(&beta12);
    assert_eq!(
        lam12,
        (lam1 + lam2).mod_floor(&ell),
        "λ(2+2α) should equal λ(1+α) + λ(2) mod 5"
    );

    // β₁ = α, β₂ = 1 + α, β₁·β₂ = α + α² = α − 1 = −1 + α.
    let beta_a = k.alpha();
    let beta_b = k.from_int(bi(1)).add(&k.alpha());
    let beta_ab = beta_a.mul(&beta_b);
    // Verify β_ab = −1 + α.
    let expected_ab = k.from_int(bi(-1)).add(&k.alpha());
    assert_eq!(beta_ab, expected_ab, "α·(1+α) should equal −1+α");
    let lam_a = lambda(&beta_a);
    let lam_b = lambda(&beta_b);
    let lam_ab = lambda(&beta_ab);
    assert_eq!(
        lam_ab,
        (lam_a + lam_b).mod_floor(&ell),
        "λ(−1+α) should equal λ(α) + λ(1+α) mod 5"
    );

    // β₁ = 2, β₂ = 2, β₁·β₂ = 4.
    let lam_two_a = lambda(&k.from_int(bi(2)));
    let lam_two_b = lambda(&k.from_int(bi(2)));
    let lam_four = lambda(&k.from_int(bi(4)));
    assert_eq!(
        lam_four,
        (lam_two_a + lam_two_b).mod_floor(&ell),
        "λ(4) should equal λ(2) + λ(2) mod 5"
    );
}

// ─── KAT (c): Width KAT ───────────────────────────────────────────────────────

/// KAT (c): Toy-scale NFS-DL norm bound fits `Uint<4>` (256 bits).
///
/// Confirm-record obligation per ROADMAP C1 width policy: D.A.1 asserts toy F_p norms
/// fit 256 bits and consumes C1 as-is. No widening — the const-generic widening stays
/// behind the ROADMAP's prescriptive trigger.
///
/// # Setup
///
/// Toy prime p = 34_359_738_421 ≈ 2^35 (a 35-bit prime, p ≡ 1 mod 5).
/// Number field K = ℚ(i), f(x) = x² + 1.
/// Sieve bound B ≈ p^(1/3) ≈ 2^(35/3) ≈ 2^11.7 ≈ 3200.
///
/// The algebraic norm of a + bα for f = x² + 1 is N(a + bα) = a² + b².
/// For |a|, |b| ≤ B ≈ 3200, the norm is at most 2 · 3200² = 20_480_000 ≈ 2^24.3.
///
/// This is vastly smaller than 2^256 (the Uint<4> capacity), confirming the width is
/// sufficient for toy-scale NFS-DL with this polynomial.
///
/// # Width verdict
///
/// CONFIRMED: toy F_p norms (p ≈ 2^35, f = x²+1, sieve bound B ≈ p^(1/3)) fit comfortably
/// in Uint<4> (256 bits). The C1 width policy is satisfied. No widening required for D.A.
#[test]
fn kat_c_width_uint4() {
    // Toy prime p ≈ 2^35. We use p = 34_359_738_421 which is prime and ≡ 1 (mod 5).
    // Verify: 34_359_738_421 mod 5 = 1 (since 34_359_738_420 = 5 * 6_871_947_684).
    let p: u64 = 34_359_738_421;
    assert_eq!(p % 5, 1, "toy prime p should satisfy p ≡ 1 (mod 5)");

    // Sieve bound B ≈ p^(1/3). For p ≈ 2^35, B ≈ 2^(35/3) ≈ 2^11.7 ≈ 3200.
    // We use B = 3200 as a conservative upper bound.
    let sieve_bound: u64 = 3200;

    // Algebraic norm for f = x² + 1: N(a + bα) = a² + b².
    // Maximum norm: a = b = B → norm ≤ 2 * B².
    let max_norm: u64 = 2 * sieve_bound * sieve_bound; // = 20_480_000

    // Verify max_norm fits in Uint<4> (256 bits).
    // Uint<4> can hold values up to 2^256 − 1. Since max_norm ≈ 2^24, it trivially fits.
    let norm_uint = Uint::<4>::from(max_norm);
    assert!(
        norm_uint < Uint::<4>::MAX,
        "toy NFS-DL norm bound should fit in Uint<4>"
    );

    // Quantify the margin: max_norm ≈ 2^24.3, Uint<4> capacity is 2^256.
    // The margin is enormous (> 230 bits of headroom).
    let max_norm_bits = 64 - max_norm.leading_zeros();
    assert!(
        max_norm_bits <= 25,
        "toy NFS-DL norm bound should be at most 25 bits, got {max_norm_bits}"
    );
    assert!(
        max_norm_bits < 256,
        "toy NFS-DL norm bound must fit in 256 bits (Uint<4>)"
    );

    // Also verify a specific (a, b) pair: a = 1234, b = 5678.
    // N(1234 + 5678α) = 1234² + 5678² = 1_522_756 + 32_239_684 = 33_762_440.
    let a: u64 = 1234;
    let b: u64 = 5678;
    let specific_norm: u64 = a * a + b * b;
    assert_eq!(specific_norm, 33_762_440, "specific norm should match hand-computed value");
    let specific_norm_uint = Uint::<4>::from(specific_norm);
    assert!(
        specific_norm_uint < Uint::<4>::MAX,
        "specific NFS-DL norm should fit in Uint<4>"
    );

    // Width verdict: CONFIRMED. Toy F_p norms fit in Uint<4>. C1 width policy satisfied.
    // The const-generic widening (D.W) is deferred per ROADMAP prescriptive trigger.
}

// ─── Additional: error path KAT ───────────────────────────────────────────────

/// Verify that the Schirokauer map returns `RamifiedPrime` for p ≢ 1 (mod ℓ).
#[test]
fn kat_error_ramified_prime() {
    let k = field_qi();
    let ell = bi(5);

    // p = 7: 7 mod 5 = 2 ≠ 1, so the map should return RamifiedPrime.
    let phi = Ideal::new(&k, bi(7), bi(0));
    let ideals = [phi];
    let beta = k.from_int(bi(2));
    let result = compute_schirokauer(&beta, &ell, &ideals);
    assert!(
        matches!(result, Err(gnfs::SchirokauerError::RamifiedPrime { .. })),
        "should return RamifiedPrime for p=7, ℓ=5"
    );
}

/// Verify that the Schirokauer map returns an empty vector for an empty ideal list.
#[test]
fn kat_empty_ideals() {
    let k = field_qi();
    let ell = bi(5);
    let beta = k.from_int(bi(2));
    let result = compute_schirokauer(&beta, &ell, &[]).expect("empty ideals should succeed");
    assert!(result.is_empty(), "empty ideal list → empty result");
}

/// Verify the r > 1 multi-coordinate shape: two ideals → two coordinates.
///
/// Uses two prime ideals above different primes p₁ = 41 and p₂ = 11 (11 ≡ 1 mod 5).
/// For p₂ = 11: 11 ≡ 1 (mod 5). Root of x²+1 mod 11: need r with r²+1 ≡ 0 (mod 11),
/// i.e., r² ≡ −1 ≡ 10 (mod 11). Checking: 1,4,9,5,3,3,5,9,4,1 — none equal 10.
/// So x²+1 is irreducible mod 11 (11 is inert in ℚ(i)). Use p₂ = 61 instead:
/// 61 ≡ 1 (mod 5) and 61 ≡ 1 (mod 4) (so it splits in ℚ(i)).
/// Root of x²+1 mod 61: r² ≡ −1 ≡ 60 (mod 61). Try r=14: 196 = 3·61+13 ≡ 13. No.
/// r=25: 625 = 10·61+15 ≡ 15. No. r=11: 121 = 61+60 ≡ 60. Yes! r=11.
#[test]
fn kat_multi_coordinate_shape() {
    let k = field_qi();
    let ell = bi(5);

    // φ₁ = (41, α − 9): 41 ≡ 1 (mod 5), 9² + 1 = 82 ≡ 0 (mod 41).
    let phi1 = Ideal::new(&k, bi(41), bi(9));
    // φ₂ = (61, α − 11): 61 ≡ 1 (mod 5), 11² + 1 = 122 = 2·61 ≡ 0 (mod 61).
    let phi2 = Ideal::new(&k, bi(61), bi(11));
    let ideals = [phi1, phi2];

    let beta = k.from_int(bi(1)).add(&k.alpha()); // 1 + α
    let result = compute_schirokauer(&beta, &ell, &ideals).expect("multi-ideal map should succeed");

    assert_eq!(result.len(), 2, "two ideals → two coordinates (r=2 shape)");
    // Both coordinates should be in [0, ℓ).
    for (i, coord) in result.iter().enumerate() {
        assert!(
            *coord >= BigInt::from(0i64) && *coord < ell,
            "coordinate {i} = {coord} should be in [0, ℓ=5)"
        );
    }
}
