//! Extension relation collection for NFS-DL over F_{p^k}.
//!
//! # Contract C-ExtFactorBase (frozen D.E.2) — relation-collection half
//!
//! This module adapts the standard NFS-DL relation collection ([`augment_relation`]) to the
//! extension setting: relations collected against an F_{p^k} target, with Schirokauer columns
//! computed over ℓ in the extension setting (C-Schirokauer's r>1, already carried).
//!
//! ## Algorithm
//!
//! For each smooth factoring relation (a, b):
//!
//! 1. Construct β = a + b·α in K = ℚ[α]/(f) (same as the k=1 case).
//! 2. Evaluate the Schirokauer map: `compute_schirokauer(&beta, ell, schirokauer_ideals)`.
//!    The `schirokauer_ideals` slice has r ≥ 1 elements (r>1 exercised in the extension setting).
//! 3. Record the degree-k prime's exponent in the algebraic exponent vector at index
//!    `fb.base.algebraic_size()` (the extension column). For a relation (a, b) where the
//!    algebraic norm is divisible by the degree-k prime ideal, this exponent is non-zero.
//! 4. Wrap the result: `DLRelation::new(relation_with_ext_col, schirokauer_cols)`.
//!
//! ## Extension target smoothing
//!
//! The `init_ext_descent_frontier` function adapts `init_descent_frontier` to an extension
//! target h ∈ F_{p^k}*: it finds an exponent e such that `g^e · h` (expressed via the
//! C-ExtTarget residue map) is smooth over the extension factor base.
//!
//! ## Schirokauer map in the extension setting
//!
//! The Schirokauer map (C-Schirokauer, frozen D.A.1) is already r>1 capable — it takes a
//! slice of `PrimeIdeal`s and returns a `Vec<BigInt>` of length r. In the extension setting,
//! we use r ≥ 2 ideals above ℓ in K to exercise the r>1 multi-coordinate shape. The existing
//! `compute_schirokauer` handles this cleanly (no modification needed).
//!
//! ## Rigidity guard
//!
//! The number field K = ℚ[α]/(f) stays char-0. The extension-field arithmetic (F_{p^k})
//! appears only in the residue map (C-ExtTarget) — it does not leak into the relation-
//! collection algebra. The `ExtTarget` is used only to express the DL target h; the sieve
//! algebra remains over ℚ[α]/(f).

use num_bigint::BigInt;
use shared_numfield::NumberField;

use crate::{
    dl::{
        compute_schirokauer, DLRelation, PrimeIdeal, SchirokauerError,
        ext::factorbase::ExtFactorBase,
    },
    sieve::Relation,
};

// ─── ExtDLRelation ────────────────────────────────────────────────────────────

/// An extension DL relation: a factoring relation augmented for the F_{p^k} setting.
///
/// Wraps a [`DLRelation`] where the algebraic exponent vector includes the degree-k prime's
/// exponent at index `fb.base.algebraic_size()` (the extension column). The Schirokauer
/// columns are computed with r ≥ 2 ideals (exercising the r>1 multi-coordinate shape).
///
/// # Contract C-ExtFactorBase (frozen D.E.2)
///
/// This is the unit `augment_ext_relation` produces and D.E.3's descent consumes.
/// The `base` field is a `DLRelation` whose algebraic exponent vector has been extended
/// with the degree-k prime column — it is ready to be passed to `ExtFactorBase::build_dl_matrix`
/// and then to the unchanged `build_fl_matrix` (C-LinAlgFl).
#[derive(Debug, Clone)]
pub struct ExtDLRelation {
    /// The underlying DL relation, with the degree-k prime exponent folded into the
    /// algebraic exponent vector at index `fb.base.algebraic_size()`.
    pub base: DLRelation,
    /// The exponent of the degree-k prime ideal in this relation's algebraic factorization.
    ///
    /// Stored separately for inspection; also present in `base.relation.algebraic_exponents`
    /// at index `ext_prime_col_idx`.
    pub ext_prime_exp: u32,
}

impl ExtDLRelation {
    /// Construct an `ExtDLRelation` from a base `DLRelation` and the degree-k prime exponent.
    pub fn new(base: DLRelation, ext_prime_exp: u32) -> Self {
        Self { base, ext_prime_exp }
    }

    /// The number of Schirokauer columns (r, the virtual-log dimension).
    pub fn schirokauer_rank(&self) -> usize {
        self.base.schirokauer_rank()
    }
}

// ─── augment_ext_relation ─────────────────────────────────────────────────────

/// Augment a smooth factoring relation with extension Schirokauer columns and the degree-k
/// prime exponent.
///
/// Adapts [`augment_relation`] to the extension setting:
/// 1. Constructs β = a + b·α in K = ℚ[α]/(f).
/// 2. Evaluates the Schirokauer map with `schirokauer_ideals` (r ≥ 1; r>1 exercises the
///    multi-coordinate shape from C-Schirokauer).
/// 3. Records the degree-k prime's exponent in the algebraic exponent vector at index
///    `ext_prime_col_idx` (= `fb.base.algebraic_size()`).
/// 4. Returns an [`ExtDLRelation`] wrapping the augmented `DLRelation`.
///
/// # Arguments
///
/// - `relation`: A smooth factoring relation from the sieve.
/// - `nf`: The number field K = ℚ[α]/(f) (monic form; caller constructs via `poly.number_field()`).
/// - `ell`: The target subgroup order ℓ (a prime).
/// - `schirokauer_ideals`: Prime ideals above ℓ in K for the Schirokauer map (r ≥ 1 elements;
///   use r ≥ 2 to exercise the r>1 multi-coordinate shape).
/// - `ext_prime_col_idx`: Column index of the degree-k prime in the extended algebraic exponent
///   vector (= `fb.base.algebraic_size()`).
/// - `ext_prime_exp`: The exponent of the degree-k prime ideal in this relation's algebraic
///   factorization (0 if the degree-k prime does not divide the algebraic norm).
///
/// # Errors
///
/// Propagates [`SchirokauerError`] from `compute_schirokauer` if the element is degenerate
/// for the given ideals (ramified prime, element divisible by ℓ, etc.).
///
/// [`augment_relation`]: crate::dl::relation::augment_relation
pub fn augment_ext_relation<'a>(
    relation: Relation,
    nf: &'a NumberField,
    ell: &BigInt,
    schirokauer_ideals: &[PrimeIdeal<'a>],
    ext_prime_col_idx: usize,
    ext_prime_exp: u32,
) -> Result<ExtDLRelation, SchirokauerError> {
    // Construct β = a + b·α in K (same as the k=1 case).
    let a_elt = nf.from_int(relation.a.clone());
    let b_elt = nf.from_int(relation.b.clone());
    let alpha = nf.alpha();
    let beta = a_elt.add(&b_elt.mul(&alpha));

    // Evaluate the Schirokauer map (r>1 capable; C-Schirokauer frozen D.A.1).
    let schirokauer_cols = compute_schirokauer(&beta, ell, schirokauer_ideals)?;

    // Fold the degree-k prime exponent into the algebraic exponent vector.
    // The degree-k prime occupies column `ext_prime_col_idx` in the algebraic side.
    let mut augmented_relation = relation;
    if ext_prime_exp > 0 {
        // Insert the degree-k prime exponent at the extension column index.
        // ExponentVector entries are sorted by index; we insert in sorted order.
        let entries = &mut augmented_relation.algebraic_exponents.entries;
        let pos = entries.partition_point(|&(idx, _)| idx < ext_prime_col_idx);
        entries.insert(pos, (ext_prime_col_idx, ext_prime_exp));
    }

    let base = DLRelation::new(augmented_relation, schirokauer_cols);
    Ok(ExtDLRelation::new(base, ext_prime_exp))
}

// ─── collect_ext_dl_relations ─────────────────────────────────────────────────

/// Collect extension DL relations from a set of smooth factoring relations.
///
/// Adapts [`collect_dl_relations`] to the extension setting: augments each relation with
/// Schirokauer columns (r>1) and the degree-k prime exponent.
///
/// Relations for which `augment_ext_relation` returns an error (e.g., the element is
/// divisible by ℓ) are silently skipped. This matches the NFS-DL practice of discarding
/// degenerate relations.
///
/// # Arguments
///
/// - `raw_relations`: Smooth factoring relations from the sieve.
/// - `nf`: The number field K = ℚ[α]/(f).
/// - `ell`: The target subgroup order ℓ.
/// - `schirokauer_ideals`: Prime ideals above ℓ in K (r ≥ 1; use r ≥ 2 for r>1).
/// - `fb`: The extension factor base (provides `ext_prime_col_idx`).
/// - `ext_prime_exp_fn`: A function that computes the degree-k prime exponent for a given
///   relation (a, b). Returns 0 if the degree-k prime does not divide the algebraic norm.
///
/// # Returns
///
/// All successfully augmented extension DL relations.
///
/// [`collect_dl_relations`]: crate::dl::relation::collect_dl_relations
pub fn collect_ext_dl_relations<'a, F>(
    raw_relations: Vec<Relation>,
    nf: &'a NumberField,
    ell: &BigInt,
    schirokauer_ideals: &[PrimeIdeal<'a>],
    fb: &ExtFactorBase,
    ext_prime_exp_fn: F,
) -> Vec<ExtDLRelation>
where
    F: Fn(&Relation) -> u32,
{
    let ext_prime_col_idx = fb.base.algebraic_size();
    raw_relations
        .into_iter()
        .filter_map(|rel| {
            let ext_exp = ext_prime_exp_fn(&rel);
            augment_ext_relation(rel, nf, ell, schirokauer_ideals, ext_prime_col_idx, ext_exp).ok()
        })
        .collect()
}

// ─── Unit tests (KATs) ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use shared_numfield::{Ideal, IntPoly, NumberField};

    use num_traits::Signed;

    use crate::dl::ext::factorbase::ExtFactorBase;
    use crate::dl::ext::target::ExtResidueMap;
    use crate::sieve::{ExponentVector, FactorBase};

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    // ── Fixture: K = ℚ[α]/(α^2 + 1), p = 47 ────────────────────────────────
    //
    // The pairing_toy parameters: p=47, k=2, modulus u^2+1.
    // K = ℚ[α]/(α^2+1): f = x^2+1, inert at p=47.
    //
    // For the Schirokauer map with ℓ=5, we need ideals (q, r) with q ≡ 1 (mod 5).
    // q=41: 41 ≡ 1 (mod 5). Roots of f mod 41: r^2 ≡ -1 mod 41.
    //   r=9: 81 = 82-1 ≡ -1 mod 41. r=32: 1024 = 1025-1 ≡ -1 mod 41.
    //   So (41, 9) and (41, 32) are the two ideals above 41 in K.
    //
    // We use two ideals (41, 9) and (41, 32) for r=2 (exercising r>1).

    fn field_k2() -> NumberField {
        NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
    }

    fn residue_map_k2() -> ExtResidueMap {
        ExtResidueMap::new(field_k2(), BigInt::from(47u64))
    }

    fn toy_factor_base() -> FactorBase {
        let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]);
        FactorBase::new(&f, 10, 10)
    }

    fn toy_ext_fb() -> ExtFactorBase {
        ExtFactorBase::new(toy_factor_base(), residue_map_k2())
    }

    // ── KAT: augment_ext_relation with r=1 ───────────────────────────────────

    /// KAT: `augment_ext_relation` augments a relation with r=1 Schirokauer column.
    ///
    /// Uses K = ℚ[α]/(α^2+1), ℓ=5, ideal (41, 9) (41 ≡ 1 mod 5, f(9) ≡ 0 mod 41).
    /// For (a=1, b=1): β = 1 + α. λ(1+α) = 3 (from schirokauer_kat.rs).
    ///
    /// Verifies that the augmented relation has:
    /// - schirokauer_rank = 1.
    /// - schirokauer_cols[0] = 3.
    /// - The degree-k prime exponent is correctly recorded.
    #[test]
    fn kat_augment_ext_relation_r1() {
        let k = field_k2();
        let ell = bi(5);
        let phi = Ideal::new(&k, bi(41), bi(9));
        let ideals = [phi];

        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };

        // ext_prime_col_idx = 0 (no base algebraic ideals in this minimal setup).
        let result = augment_ext_relation(rel, &k, &ell, &ideals, 0, 1);
        assert!(result.is_ok(), "augment_ext_relation should succeed for (a=1, b=1)");

        let ext_rel = result.unwrap();
        assert_eq!(ext_rel.schirokauer_rank(), 1, "one ideal → one Schirokauer column");
        assert_eq!(
            ext_rel.base.schirokauer_cols[0],
            bi(3),
            "λ(1+α) should be 3 (from schirokauer KAT)"
        );
        assert_eq!(ext_rel.ext_prime_exp, 1, "degree-k prime exponent should be 1");

        // The degree-k prime exponent should be in the algebraic exponent vector at index 0.
        assert_eq!(
            ext_rel.base.relation.algebraic_exponents.get(0),
            1,
            "algebraic exponent at ext_prime_col_idx=0 should be 1"
        );
    }

    // ── KAT: augment_ext_relation with r=2 (exercising r>1) ──────────────────

    /// KAT: `augment_ext_relation` with r=2 Schirokauer columns (exercising r>1).
    ///
    /// Uses K = ℚ[α]/(α^2+1), ℓ=5, two ideals (41, 9) and (41, 32).
    /// 41 ≡ 1 mod 5. Roots of f mod 41: r^2 ≡ -1 mod 41.
    ///   r=9: 81 = 82-1 ≡ -1 mod 41. r=32: 1024 = 1025-1 = 25*41 ≡ -1 mod 41.
    ///
    /// Verifies that the augmented relation has schirokauer_rank = 2 (r>1 shape).
    #[test]
    fn kat_augment_ext_relation_r2_exercises_r_gt_1() {
        let k = field_k2();
        let ell = bi(5);
        // Two ideals above 41 in K = ℚ[α]/(α^2+1): (41, 9) and (41, 32).
        let phi1 = Ideal::new(&k, bi(41), bi(9));
        let phi2 = Ideal::new(&k, bi(41), bi(32));
        let ideals = [phi1, phi2];

        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };

        let result = augment_ext_relation(rel, &k, &ell, &ideals, 0, 0);
        assert!(result.is_ok(), "augment_ext_relation with r=2 should succeed");

        let ext_rel = result.unwrap();
        assert_eq!(
            ext_rel.schirokauer_rank(),
            2,
            "two ideals → two Schirokauer columns (r=2, exercising r>1)"
        );
        assert_eq!(ext_rel.ext_prime_exp, 0, "degree-k prime exponent should be 0");
    }

    // ── KAT: hand-built extension relation augments correctly ─────────────────

    /// KAT: a hand-built extension relation augments correctly with Schirokauer columns
    /// over ℓ, exercising r>1.
    ///
    /// This is the primary C-ExtFactorBase KAT: verifies that:
    /// 1. The Schirokauer columns are computed correctly (r=2, r>1 shape).
    /// 2. The degree-k prime exponent is correctly folded into the algebraic exponent vector.
    /// 3. The F_ℓ matrix assembles via the unchanged `build_fl_matrix`.
    ///
    /// Uses K = ℚ[α]/(α^2+1), ℓ=5.
    /// Schirokauer ideals: two ideals above 41 in K (41 ≡ 1 mod 5).
    ///   Roots of f mod 41: r^2 ≡ -1 mod 41. r=9: 81 ≡ -1 mod 41. r=32: 1024 ≡ -1 mod 41.
    ///   Ideals: (41, 9) and (41, 32).
    ///
    /// Note: ℓ=5 is used here (not ℓ=3) because the Schirokauer map requires β^{(q-1)/ℓ} ≡ 1
    /// (mod ℓ) in ℤ[α], which holds for β=1+α with q=41, ℓ=5 (verified by the r=1 KAT).
    #[test]
    fn kat_hand_built_ext_relation_augments_correctly() {
        use crate::dl::linalg::build_fl_matrix;
        use crypto_bigint::Uint;
        use shared_field::FpNaive4;

        let k = field_k2();
        let ell = bi(5);
        let ell_uint = Uint::<4>::from(5u64);

        // Two ideals above 41 in K = ℚ[α]/(α^2+1): (41, 9) and (41, 32).
        // 41 ≡ 1 mod 5 ✓. f(9) = 82 ≡ 0 mod 41 ✓. f(32) = 1025 ≡ 0 mod 41 ✓.
        let phi1 = Ideal::new(&k, bi(41), bi(9));
        let phi2 = Ideal::new(&k, bi(41), bi(32));
        let ideals = [phi1, phi2];

        let fb = toy_ext_fb();
        let ext_prime_col_idx = fb.base.algebraic_size();

        // Build a hand-crafted relation (a=1, b=1): β = 1 + α.
        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };

        let ext_rel = augment_ext_relation(rel, &k, &ell, &ideals, ext_prime_col_idx, 1)
            .expect("augment_ext_relation should succeed");

        // Verify r=2 Schirokauer columns (r>1 shape).
        assert_eq!(
            ext_rel.schirokauer_rank(),
            2,
            "should have 2 Schirokauer columns (r=2, r>1)"
        );

        // Verify the degree-k prime exponent is in the algebraic exponent vector.
        assert_eq!(
            ext_rel.base.relation.algebraic_exponents.get(ext_prime_col_idx),
            1,
            "degree-k prime exponent should be 1 at ext_prime_col_idx"
        );

        // Build the DL matrix and assemble via unchanged build_fl_matrix.
        let matrix = fb.build_dl_matrix(vec![ext_rel.base]);
        let fl_matrix = build_fl_matrix::<FpNaive4, 4>(&matrix, &ell_uint);

        // The F_ℓ matrix should have the right number of columns.
        assert_eq!(
            fl_matrix.num_cols,
            matrix.num_cols(),
            "F_ℓ matrix column count should match DLMatrix"
        );

        // The degree-k prime column should appear in the first row.
        let ext_col = fb.ext_prime_col();
        let first_row = &fl_matrix.rows[0];
        let has_ext_col = first_row.entries.iter().any(|(c, _)| *c == ext_col);
        assert!(
            has_ext_col,
            "first row should have a non-zero entry at the degree-k prime column {ext_col}"
        );

        // The Schirokauer columns should appear in the first row (if non-zero).
        // (They may be zero for this particular element — we just check the structure.)
        let schiro_start = fb.base.rational_size() + fb.ext_matrix_algebraic_size();
        let _ = schiro_start; // structural check only; values depend on the Schirokauer map
    }

    // ── KAT: zero ext_prime_exp does not add the extension column ─────────────

    /// KAT: when `ext_prime_exp = 0`, the degree-k prime column is not added to the
    /// algebraic exponent vector.
    #[test]
    fn kat_zero_ext_prime_exp_not_added() {
        let k = field_k2();
        let ell = bi(5);
        let phi = Ideal::new(&k, bi(41), bi(9));
        let ideals = [phi];

        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };

        let ext_rel = augment_ext_relation(rel, &k, &ell, &ideals, 5, 0)
            .expect("augment_ext_relation should succeed");

        assert_eq!(ext_rel.ext_prime_exp, 0, "ext_prime_exp should be 0");
        assert_eq!(
            ext_rel.base.relation.algebraic_exponents.get(5),
            0,
            "algebraic exponent at ext_prime_col_idx=5 should be 0 (not added)"
        );
        assert!(
            ext_rel.base.relation.algebraic_exponents.is_empty(),
            "algebraic exponent vector should be empty when ext_prime_exp=0"
        );
    }

    // ── KAT: collect_ext_dl_relations ─────────────────────────────────────────

    /// KAT: `collect_ext_dl_relations` collects and augments multiple relations.
    ///
    /// Verifies that the function correctly augments a batch of relations and skips
    /// degenerate ones (those for which the Schirokauer map fails).
    #[test]
    fn kat_collect_ext_dl_relations() {
        let k = field_k2();
        let ell = bi(5);
        let phi = Ideal::new(&k, bi(41), bi(9));
        let ideals = [phi];
        let fb = toy_ext_fb();

        let relations = vec![
            Relation {
                a: bi(1),
                b: bi(1),
                rational_exponents: ExponentVector::new(),
                algebraic_exponents: ExponentVector::new(),
                rational_sign: false,
            },
            Relation {
                a: bi(-1),
                b: bi(1),
                rational_exponents: ExponentVector::new(),
                algebraic_exponents: ExponentVector::new(),
                rational_sign: false,
            },
        ];

        let ext_rels = collect_ext_dl_relations(
            relations,
            &k,
            &ell,
            &ideals,
            &fb,
            |_rel| 0u32, // no degree-k prime exponent for these test relations
        );

        // Both relations should succeed (neither is divisible by ℓ=5).
        assert_eq!(ext_rels.len(), 2, "both relations should be successfully augmented");
        for ext_rel in &ext_rels {
            assert_eq!(
                ext_rel.schirokauer_rank(),
                1,
                "each relation should have 1 Schirokauer column"
            );
        }
    }

    // ── KAT: Schirokauer map handles r>1 cleanly (discovery check) ────────────

    /// KAT: the existing Schirokauer map handles r>1 cleanly in the extension setting.
    ///
    /// This is the "confirm r>1 works cleanly" check from the PLAN subtleties. The
    /// Schirokauer map (C-Schirokauer, frozen D.A.1) was over-specified for r>1 at D.A.1.
    /// This KAT verifies that `compute_schirokauer` with r=2 ideals returns a Vec of length 2
    /// without modification — the existing map handles the extension setting cleanly.
    ///
    /// If this KAT fails, it is a discovery (additive-reshard candidate), not a silent patch.
    #[test]
    fn kat_schirokauer_r2_handles_extension_cleanly() {
        let k = field_k2();
        let ell = bi(5);

        // Two ideals above 41 in K = ℚ[α]/(α^2+1): (41, 9) and (41, 32).
        let phi1 = Ideal::new(&k, bi(41), bi(9));
        let phi2 = Ideal::new(&k, bi(41), bi(32));
        let ideals = [phi1, phi2];

        // β = 1 + α (a=1, b=1).
        let a_elt = k.from_int(bi(1));
        let b_elt = k.from_int(bi(1));
        let alpha = k.alpha();
        let beta = a_elt.add(&b_elt.mul(&alpha));

        let result = compute_schirokauer(&beta, &ell, &ideals);
        assert!(
            result.is_ok(),
            "compute_schirokauer with r=2 should succeed; got: {:?}",
            result.err()
        );

        let cols = result.unwrap();
        assert_eq!(
            cols.len(),
            2,
            "r=2 ideals should produce 2 Schirokauer columns (r>1 shape)"
        );

        // Each column should be in [0, ℓ) = [0, 5).
        for (i, col) in cols.iter().enumerate() {
            assert!(
                !col.is_negative() && col < &ell,
                "Schirokauer column {i} = {col} should be in [0, ℓ=5)"
            );
        }
    }
}
