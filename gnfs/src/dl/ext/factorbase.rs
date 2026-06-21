//! Extension factor base for NFS-DL over F_{p^k}.
//!
//! # Extension factor-base contract
//!
//! This module defines the extension factor base: the standard NFS-DL factor base augmented
//! with the degree-k prime ideal whose residue field is exactly F_{p^k}. It is consumed by
//! the k>1 descent + `solve_dl` wiring and the MOV bridge (indirectly, via `solve_dl`).
//!
//! ## Structure
//!
//! [`ExtFactorBase`] composes:
//! - The frozen [`FactorBase`] (degree-1 prime ideals, rational and algebraic sides).
//! - The frozen [`ExtResidueMap`] (the degree-k prime ideal's parameters: number field K,
//!   prime p, and irreducible modulus m(u) = f mod p of degree k).
//!
//! The degree-k prime ideal occupies a dedicated column in the extended DL matrix, placed
//! immediately after the algebraic factor-base columns and before the Schirokauer columns.
//! Its column index is `rational_size + algebraic_size` (the first "extra algebraic" column).
//!
//! ## Inert-prime condition
//!
//! The degree-k prime's residue field is exactly F_{p^k} — the prime is inert (f irreducible
//! mod p of degree k, not split). This is asserted by [`ExtResidueMap::new`] at construction
//! time. A split prime would give a smaller residue field and the lift would be vacuous.
//!
//! ## Rigidity guard
//!
//! The number field K = ℚ[α]/(f) stays char-0. The extension factor base does not introduce
//! extension-field arithmetic into the relation-collection algebra — the degree-k prime is
//! represented by its column index, not by its residue-field arithmetic.
//!
//! ## Column layout
//!
//! The extended DL matrix has columns:
//! - `0..rational_size`: rational factor-base primes (from `FactorBase`).
//! - `rational_size..rational_size+algebraic_size`: algebraic factor-base ideals (from `FactorBase`).
//! - `rational_size+algebraic_size`: the degree-k prime ideal (the extension column).
//! - `rational_size+algebraic_size+1..`: Schirokauer correction columns.
//!
//! This layout is transparent to [`build_fl_matrix`] because the degree-k prime column is
//! represented as an extra algebraic column in the [`DLMatrix`] (algebraic_size is incremented
//! by 1, and the degree-k prime's exponent is stored at index `algebraic_size` in the algebraic
//! exponent vector of each relation).

use num_bigint::BigInt;

use crate::dl::ext::target::ExtResidueMap;
use crate::dl::relation::DLMatrix;
use crate::sieve::FactorBase;

// ─── ExtFactorBase ────────────────────────────────────────────────────────────

/// Extension factor base: the standard NFS-DL factor base augmented with the degree-k prime.
///
/// Composes the frozen [`FactorBase`] (degree-1 ideals) and the frozen [`ExtResidueMap`]
/// (the degree-k prime ideal's parameters). The degree-k prime occupies a dedicated column
/// in the extended DL matrix at index `rational_size + algebraic_size`.
///
/// # Extension factor-base contract
///
/// Consumed by the k>1 descent + solver and the MOV bridge (indirectly, via `solve_dl`).
/// Exposes:
/// - [`ExtFactorBase::new`] — constructor (asserts the inert-prime condition via `ExtResidueMap`).
/// - [`ExtFactorBase::ext_prime_col`] — column index of the degree-k prime in the extended matrix.
/// - [`ExtFactorBase::ext_matrix_algebraic_size`] — algebraic column count for the extended matrix.
/// - [`ExtFactorBase::p`] — the prime p (the rational prime of the degree-k ideal).
/// - [`ExtFactorBase::k`] — the extension degree k.
///
/// # Inert-prime condition
///
/// The degree-k prime's residue field is exactly F_{p^k} — asserted by [`ExtResidueMap::new`].
#[derive(Debug)]
pub struct ExtFactorBase {
    /// The base NFS-DL factor base (degree-1 rational and algebraic ideals).
    pub base: FactorBase,
    /// The residue map for the degree-k prime ideal above p in K = ℚ[α]/(f).
    ///
    /// Carries: the number field K, the prime p, the extension degree k, and the
    /// irreducible modulus m(u) = f mod p. The inert-prime condition is asserted here.
    pub residue_map: ExtResidueMap,
}

impl ExtFactorBase {
    /// Construct an `ExtFactorBase` from a base factor base and a residue map.
    ///
    /// The `residue_map` must have been constructed with [`ExtResidueMap::new`], which
    /// asserts the inert-prime condition (f irreducible mod p of degree k).
    ///
    /// # Arguments
    ///
    /// - `base`: The NFS-DL factor base (degree-1 ideals, rational and algebraic sides).
    /// - `residue_map`: The degree-k prime ideal's residue map (carries p, k, modulus).
    ///   Must satisfy the inert-prime condition (already asserted by `ExtResidueMap::new`).
    pub fn new(base: FactorBase, residue_map: ExtResidueMap) -> Self {
        Self { base, residue_map }
    }

    /// The prime p (the rational prime of the degree-k ideal).
    pub fn p(&self) -> &BigInt {
        &self.residue_map.p
    }

    /// The extension degree k (residue degree of the degree-k prime ideal).
    pub fn k(&self) -> usize {
        self.residue_map.k
    }

    /// Column index of the degree-k prime ideal in the extended DL matrix.
    ///
    /// The degree-k prime occupies the column immediately after the algebraic factor-base
    /// columns: `rational_size + algebraic_size`.
    pub fn ext_prime_col(&self) -> usize {
        self.base.rational_size() + self.base.algebraic_size()
    }

    /// Algebraic column count for the extended DL matrix.
    ///
    /// The extended matrix has one extra algebraic column (the degree-k prime) beyond the
    /// base factor base's algebraic columns. This is the value to pass as `algebraic_size`
    /// when constructing a [`DLMatrix`] from extension relations.
    pub fn ext_matrix_algebraic_size(&self) -> usize {
        self.base.algebraic_size() + 1
    }

    /// Build a [`DLMatrix`] from a collection of extension DL relations.
    ///
    /// The matrix has columns:
    /// - `0..rational_size`: rational factor-base primes.
    /// - `rational_size..rational_size+algebraic_size+1`: algebraic ideals + degree-k prime.
    /// - `rational_size+algebraic_size+1..`: Schirokauer correction columns.
    ///
    /// The degree-k prime's exponent is stored at algebraic index `algebraic_size` in each
    /// relation's algebraic exponent vector, so [`build_fl_matrix`] handles it transparently.
    ///
    /// # Panics
    ///
    /// Panics if the relations have inconsistent Schirokauer ranks.
    ///
    /// [`build_fl_matrix`]: crate::dl::linalg::build_fl_matrix
    pub fn build_dl_matrix(&self, relations: Vec<crate::dl::DLRelation>) -> DLMatrix {
        // The degree-k prime column is folded into the algebraic columns (at index
        // algebraic_size). DLMatrix is constructed directly with the extended algebraic_size.
        let schirokauer_rank = relations.first().map(|r| r.schirokauer_rank()).unwrap_or(0);
        for (i, rel) in relations.iter().enumerate() {
            assert_eq!(
                rel.schirokauer_rank(),
                schirokauer_rank,
                "ExtFactorBase::build_dl_matrix: relation {i} has Schirokauer rank {} \
                 but expected {}",
                rel.schirokauer_rank(),
                schirokauer_rank,
            );
        }
        DLMatrix {
            relations,
            rational_size: self.base.rational_size(),
            algebraic_size: self.ext_matrix_algebraic_size(),
            schirokauer_rank,
        }
    }
}

// ─── Unit tests (KATs) ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use shared_numfield::{IntPoly, NumberField};

    use crate::dl::ext::target::ExtResidueMap;

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    // ── Fixture: F_{47^2} = F_47[u]/(u^2 + 1) ────────────────────────────────
    //
    // p = 47, k = 2, f = x^2 + 1 (inert at p=47: -1 is a QNR mod 47).
    // The number field K = ℚ[α]/(α^2 + 1).

    fn field_k2() -> NumberField {
        NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
    }

    fn residue_map_k2() -> ExtResidueMap {
        ExtResidueMap::new(field_k2(), BigInt::from(47u64))
    }

    fn toy_factor_base() -> FactorBase {
        // f = x^2 + 1, b_rat = 10, b_alg = 10.
        let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]);
        FactorBase::new(&f, 10, 10)
    }

    fn toy_ext_fb() -> ExtFactorBase {
        ExtFactorBase::new(toy_factor_base(), residue_map_k2())
    }

    // ── KAT: degree-k prime has the right residue degree ─────────────────────

    /// KAT: the extension factor base carries the degree-k prime with residue degree k=2.
    ///
    /// Verifies that the degree-k prime's residue field is exactly F_{47^2} (k=2), not a
    /// smaller field. This is the primary correctness KAT for the extension factor base.
    #[test]
    fn kat_ext_factor_base_residue_degree() {
        let fb = toy_ext_fb();
        assert_eq!(fb.k(), 2, "extension degree should be k=2");
        assert_eq!(*fb.p(), BigInt::from(47u64), "prime should be 47");
        assert_eq!(
            fb.residue_map.modulus,
            vec![bi(1), bi(0), bi(1)],
            "modulus should be [1, 0, 1] (u^2 + 1)"
        );
    }

    /// KAT: the degree-k prime is inert (not split) — the inert-prime condition holds.
    ///
    /// Verifies that `ExtResidueMap::new` does not panic for the inert prime p=47 in
    /// K = ℚ[α]/(α^2+1). This is the inert-prime assertion from the extension-target contract.
    #[test]
    fn kat_ext_factor_base_inert_prime() {
        // Should not panic: p=47 is inert in ℚ[α]/(α^2+1).
        let fb = toy_ext_fb();
        assert_eq!(fb.k(), 2, "inert prime p=47 should give residue degree k=2");
    }

    /// KAT: a split prime panics at construction (inert-prime guard).
    ///
    /// p=5 splits in ℚ[α]/(α^2+1) since f(2) = 5 ≡ 0 mod 5. Constructing an
    /// `ExtFactorBase` with a split prime must panic.
    #[test]
    fn kat_ext_factor_base_split_prime_panics() {
        let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]);
        let base = FactorBase::new(&f, 10, 10);
        let result = std::panic::catch_unwind(|| {
            let field = NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]));
            let map = ExtResidueMap::new(field, BigInt::from(5i64));
            ExtFactorBase::new(base, map)
        });
        assert!(
            result.is_err(),
            "ExtFactorBase with split prime p=5 should panic (inert-prime guard)"
        );
    }

    // ── KAT: column layout ────────────────────────────────────────────────────

    /// KAT: the degree-k prime column index is `rational_size + algebraic_size`.
    ///
    /// Verifies the column layout: the degree-k prime occupies the column immediately
    /// after the algebraic factor-base columns.
    #[test]
    fn kat_ext_prime_col_layout() {
        let fb = toy_ext_fb();
        let expected_col = fb.base.rational_size() + fb.base.algebraic_size();
        assert_eq!(
            fb.ext_prime_col(),
            expected_col,
            "degree-k prime column should be rational_size + algebraic_size"
        );
    }

    /// KAT: `ext_matrix_algebraic_size` is `algebraic_size + 1`.
    ///
    /// Verifies that the extended matrix has one extra algebraic column for the degree-k prime.
    #[test]
    fn kat_ext_matrix_algebraic_size() {
        let fb = toy_ext_fb();
        assert_eq!(
            fb.ext_matrix_algebraic_size(),
            fb.base.algebraic_size() + 1,
            "extended algebraic size should be algebraic_size + 1"
        );
    }

    // ── KAT: build_dl_matrix ──────────────────────────────────────────────────

    /// KAT: `build_dl_matrix` produces a DLMatrix with the correct column layout.
    ///
    /// Verifies that the DLMatrix built from extension relations has:
    /// - `rational_size` = base rational size.
    /// - `algebraic_size` = base algebraic size + 1 (the degree-k prime column).
    /// - `schirokauer_rank` = number of Schirokauer columns.
    #[test]
    fn kat_build_dl_matrix_column_layout() {
        use crate::dl::DLRelation;
        use crate::sieve::{ExponentVector, Relation};

        let fb = toy_ext_fb();

        // Build two minimal extension relations with schirokauer_rank = 2 (r>1).
        let make_rel = |ext_exp: u32| -> DLRelation {
            let mut alg_exp = ExponentVector::new();
            // The degree-k prime exponent at index algebraic_size (the extension column).
            if ext_exp > 0 {
                alg_exp.entries.push((fb.base.algebraic_size(), ext_exp));
            }
            let rel = Relation {
                a: bi(1),
                b: bi(1),
                rational_exponents: ExponentVector::new(),
                algebraic_exponents: alg_exp,
                rational_sign: false,
            };
            // Two Schirokauer columns (r=2, exercising r>1).
            DLRelation::new(rel, vec![bi(0), bi(1)])
        };

        let relations = vec![make_rel(1), make_rel(0)];
        let matrix = fb.build_dl_matrix(relations);

        assert_eq!(
            matrix.rational_size,
            fb.base.rational_size(),
            "rational_size should match base"
        );
        assert_eq!(
            matrix.algebraic_size,
            fb.base.algebraic_size() + 1,
            "algebraic_size should be base + 1 (degree-k prime column)"
        );
        assert_eq!(matrix.schirokauer_rank, 2, "schirokauer_rank should be 2 (r>1)");
        assert_eq!(matrix.num_rows(), 2, "should have 2 rows");
    }

    // ── KAT: F_ℓ matrix assembles via unchanged build_fl_matrix ──────────────

    /// KAT: the F_ℓ matrix assembles via the unchanged index-agnostic `build_fl_matrix`.
    ///
    /// Verifies that `build_fl_matrix` (F_ℓ linear-algebra substrate) handles the extended
    /// DLMatrix transparently — the degree-k prime column is just another algebraic column.
    /// This is the load-bearing KAT for the "reuse build_fl_matrix unchanged" requirement.
    #[test]
    fn kat_build_fl_matrix_via_unchanged_linalg() {
        use crate::dl::linalg::{build_fl_matrix, FL_BLOCK_WIDTH};
        use crate::dl::DLRelation;
        use crate::sieve::{ExponentVector, Relation};
        use crypto_bigint::Uint;
        use shared_field::FpNaive4;

        let fb = toy_ext_fb();
        let ell = Uint::<4>::from(3u64); // ℓ = 3 (the torsion prime for pairing_toy)

        // Build two extension relations with the degree-k prime column set.
        let make_rel = |ext_exp: u32, schiro: Vec<BigInt>| -> DLRelation {
            let mut alg_exp = ExponentVector::new();
            if ext_exp > 0 {
                alg_exp.entries.push((fb.base.algebraic_size(), ext_exp));
            }
            let rel = Relation {
                a: bi(1),
                b: bi(1),
                rational_exponents: ExponentVector::new(),
                algebraic_exponents: alg_exp,
                rational_sign: false,
            };
            DLRelation::new(rel, schiro)
        };

        // r=2 Schirokauer columns (exercising r>1).
        let relations = vec![
            make_rel(1, vec![bi(1), bi(2)]),
            make_rel(0, vec![bi(2), bi(0)]),
        ];
        let matrix = fb.build_dl_matrix(relations);

        // Call the unchanged build_fl_matrix from the F_ℓ linear-algebra substrate.
        let fl_matrix = build_fl_matrix::<FpNaive4, 4>(&matrix, &ell);

        // The F_ℓ matrix should have the right number of columns.
        let expected_cols = matrix.num_cols();
        assert_eq!(
            fl_matrix.num_cols, expected_cols,
            "F_ℓ matrix should have {} columns (rational + algebraic+1 + schirokauer)",
            expected_cols
        );
        assert_eq!(fl_matrix.rows.len(), 2, "F_ℓ matrix should have 2 rows");

        // Verify the degree-k prime column (ext_prime_col) appears in the first row.
        let ext_col = fb.ext_prime_col();
        let first_row = &fl_matrix.rows[0];
        let has_ext_col = first_row.entries.iter().any(|(c, _)| *c == ext_col);
        assert!(
            has_ext_col,
            "first row should have a non-zero entry at the degree-k prime column {ext_col}"
        );

        // Verify the Schirokauer columns appear in the matrix.
        let schiro_start = fb.base.rational_size() + fb.ext_matrix_algebraic_size();
        let has_schiro = first_row.entries.iter().any(|(c, _)| *c >= schiro_start);
        assert!(
            has_schiro,
            "first row should have Schirokauer column entries (starting at {schiro_start})"
        );

        // Verify the block width is FL_BLOCK_WIDTH (structural check).
        assert_eq!(FL_BLOCK_WIDTH, 32, "FL_BLOCK_WIDTH should be 32");
    }
}
