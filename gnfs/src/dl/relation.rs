//! DL relation collection: augmenting sieve relations with Schirokauer columns.
//!
//! This module implements the D.A.2 deliverable: taking smooth factoring relations from the
//! sieve and augmenting each with Schirokauer virtual-log columns to produce the DL relation
//! matrix that D.B's F_ℓ linear algebra will consume.
//!
//! # Algorithm
//!
//! For each smooth relation (a, b) from the sieve:
//!
//! 1. Construct the number-field element β = a + b·α in K = ℚ(α).
//! 2. Evaluate the Schirokauer map: `compute_schirokauer(&beta, ell, ideals)`.
//! 3. Wrap the result: `DLRelation::new(relation, schirokauer_cols)`.
//!
//! The assembled `DLMatrix` has columns:
//! - Rational exponent columns (one per rational factor-base prime).
//! - Algebraic exponent columns (one per algebraic factor-base ideal).
//! - Schirokauer columns (one per prime ideal in the Schirokauer ideal set).
//!
//! D.B will reduce the exponent columns mod ℓ and the Schirokauer columns are already in ℤ/ℓ.
//!
//! # Scope
//!
//! D.A.2 stops at producing the augmented DL relation matrix. The F_ℓ linear algebra
//! (solving the system to recover discrete logarithms) is D.B's responsibility.

use num_bigint::BigInt;
use shared_numfield::NumberField;

use crate::{
    dl::{compute_schirokauer, DLRelation, PrimeIdeal, SchirokauerError},
    polyselect::PolyPair,
    sieve::{FactorBase, LineSieveConfig, Relation, line_sieve},
};

// ─── augment_relation ─────────────────────────────────────────────────────────

/// Augment a smooth factoring relation with Schirokauer virtual-log columns.
///
/// Constructs the number-field element β = a + b·α from the relation's (a, b) pair,
/// evaluates the Schirokauer map, and returns a `DLRelation` wrapping the original
/// relation plus the virtual-log coordinates.
///
/// # Arguments
///
/// - `relation`: A smooth factoring relation from the sieve.
/// - `nf`: The number field K = ℚ(α) (must be the monic form of the algebraic polynomial).
/// - `ell`: The target subgroup order ℓ (a prime).
/// - `schirokauer_ideals`: Prime ideals above ℓ in K for the Schirokauer map.
///
/// # Errors
///
/// Propagates [`SchirokauerError`] from `compute_schirokauer` if the element is
/// degenerate for the given ideals (ramified prime, element divisible by ℓ, etc.).
pub fn augment_relation<'a>(
    relation: Relation,
    nf: &'a NumberField,
    ell: &BigInt,
    schirokauer_ideals: &[PrimeIdeal<'a>],
) -> Result<DLRelation, SchirokauerError> {
    // Construct β = a + b·α in K.
    // a is a BigInt (possibly negative); b is a BigInt (always positive).
    let a_elt = nf.from_int(relation.a.clone());
    let b_elt = nf.from_int(relation.b.clone());
    let alpha = nf.alpha();
    // β = a·1 + b·α
    let beta = a_elt.add(&b_elt.mul(&alpha));

    // Evaluate the Schirokauer map.
    let schirokauer_cols = compute_schirokauer(&beta, ell, schirokauer_ideals)?;

    Ok(DLRelation::new(relation, schirokauer_cols))
}

// ─── DLMatrix ─────────────────────────────────────────────────────────────────

/// The assembled DL relation matrix: a collection of `DLRelation` rows with metadata.
///
/// Each row is a `DLRelation` (factoring exponent vectors + Schirokauer columns).
/// The matrix has columns:
/// - `rational_size` rational exponent columns (one per rational factor-base prime).
/// - `algebraic_size` algebraic exponent columns (one per algebraic factor-base ideal).
/// - `schirokauer_rank` Schirokauer columns (one per prime ideal in the Schirokauer set).
///
/// D.B consumes this matrix to solve the F_ℓ linear system for discrete logarithms.
/// The raw integer values are stored here; D.B reduces the exponent columns mod ℓ.
#[derive(Debug, Clone)]
pub struct DLMatrix {
    /// The DL relation rows.
    pub relations: Vec<DLRelation>,
    /// Number of rational factor-base primes (rational exponent columns).
    pub rational_size: usize,
    /// Number of algebraic factor-base ideals (algebraic exponent columns).
    pub algebraic_size: usize,
    /// Number of Schirokauer columns (virtual-log dimension r).
    pub schirokauer_rank: usize,
}

impl DLMatrix {
    /// Assemble a `DLMatrix` from a collection of `DLRelation` rows and the factor base.
    ///
    /// All relations must have the same Schirokauer rank (number of virtual-log columns).
    /// Panics if the relations have inconsistent Schirokauer ranks.
    ///
    /// :param relations: The DL relation rows (from `augment_relation`).
    /// :param fb: The factor base (provides rational and algebraic sizes).
    /// :returns: The assembled DL matrix.
    pub fn from_relations(relations: Vec<DLRelation>, fb: &FactorBase) -> Self {
        // Determine the Schirokauer rank from the first relation (all must agree).
        let schirokauer_rank = relations.first().map(|r| r.schirokauer_rank()).unwrap_or(0);

        // Verify all relations have the same Schirokauer rank.
        for (i, rel) in relations.iter().enumerate() {
            assert_eq!(
                rel.schirokauer_rank(),
                schirokauer_rank,
                "DLMatrix::from_relations: relation {i} has Schirokauer rank {} but expected {}",
                rel.schirokauer_rank(),
                schirokauer_rank
            );
        }

        DLMatrix {
            relations,
            rational_size: fb.rational_size(),
            algebraic_size: fb.algebraic_size(),
            schirokauer_rank,
        }
    }

    /// Number of rows in the matrix (number of DL relations).
    pub fn num_rows(&self) -> usize {
        self.relations.len()
    }

    /// Number of columns in the matrix.
    ///
    /// Columns = rational exponents + algebraic exponents + Schirokauer columns.
    pub fn num_cols(&self) -> usize {
        self.rational_size + self.algebraic_size + self.schirokauer_rank
    }

    /// Matrix dimensions: (rows, cols).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.num_rows(), self.num_cols())
    }
}

// ─── collect_dl_relations ─────────────────────────────────────────────────────

/// Collect DL relations by running the line sieve and augmenting each relation.
///
/// This is a thin wrapper that:
/// 1. Runs `line_sieve` to collect smooth factoring relations.
/// 2. Augments each relation with Schirokauer columns via `augment_relation`.
/// 3. Returns the successfully augmented relations (skipping any that fail the map).
///
/// The caller is responsible for constructing the `NumberField` (via `poly.number_field()`)
/// and the Schirokauer ideals referencing it, so that lifetimes are managed at the call site.
///
/// Relations for which `augment_relation` returns an error (e.g., the element is
/// divisible by ℓ) are silently skipped. This matches the NFS-DL practice of
/// discarding degenerate relations.
///
/// :param poly: The NFS polynomial pair (provides f, m for norm computation).
/// :param fb: The two-sided factor base.
/// :param sieve_config: Sieve region and threshold parameters.
/// :param nf: The number field K = ℚ(α) (monic form; caller constructs via `poly.number_field()`).
/// :param ell: The target subgroup order ℓ.
/// :param schirokauer_ideals: Prime ideals above ℓ in `nf` for the Schirokauer map.
/// :returns: All successfully augmented DL relations.
pub fn collect_dl_relations<'a>(
    poly: &PolyPair,
    fb: &FactorBase,
    sieve_config: &LineSieveConfig,
    nf: &'a NumberField,
    ell: &BigInt,
    schirokauer_ideals: &[PrimeIdeal<'a>],
) -> Vec<DLRelation> {
    // Run the sieve to collect smooth factoring relations.
    let raw_relations = line_sieve(poly, fb, sieve_config);

    // Augment each relation with Schirokauer columns.
    // Relations that fail the Schirokauer map (degenerate elements) are silently skipped.
    raw_relations
        .into_iter()
        .filter_map(|rel| augment_relation(rel, nf, ell, schirokauer_ideals).ok())
        .collect()
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use shared_numfield::{Ideal, IntPoly, NumberField};

    fn bi(n: i64) -> BigInt {
        BigInt::from(n)
    }

    /// K = ℚ(i), f = x² + 1.
    fn field_qi() -> NumberField {
        NumberField::new(IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]))
    }

    #[test]
    fn augment_relation_constructs_element_correctly() {
        // Verify that augment_relation correctly constructs a + b·α and evaluates the map.
        // Use K = ℚ(i), ℓ = 5, φ = (41, α − 9).
        // For (a=1, b=1): β = 1 + α = 1 + i. λ(1+i) = 3 (from schirokauer_kat.rs).
        let k = field_qi();
        let ell = bi(5);
        let phi = Ideal::new(&k, bi(41), bi(9));
        let ideals = [phi];

        // Build a minimal Relation with a=1, b=1.
        use crate::sieve::ExponentVector;
        let rel = Relation {
            a: bi(1),
            b: bi(1),
            rational_exponents: ExponentVector::new(),
            algebraic_exponents: ExponentVector::new(),
            rational_sign: false,
        };

        let dl_rel = augment_relation(rel, &k, &ell, &ideals)
            .expect("augment_relation should succeed for (a=1, b=1)");

        assert_eq!(dl_rel.schirokauer_rank(), 1, "one ideal → one Schirokauer column");
        assert_eq!(dl_rel.schirokauer_cols[0], bi(3), "λ(1+α) should be 3");
        assert_eq!(dl_rel.relation.a, bi(1));
        assert_eq!(dl_rel.relation.b, bi(1));
    }

    #[test]
    fn dl_matrix_dimensions() {
        // Build a minimal DLMatrix and verify dimensions.
        // Use (a=1, b=1) and (a=-1, b=1): both have norm N(a+bi) = a²+b² = 2, coprime to ℓ=5.
        use crate::sieve::ExponentVector;
        let k = field_qi();
        let ell = bi(5);
        let phi = Ideal::new(&k, bi(41), bi(9));
        let ideals = [phi];

        // Two relations: (a=1, b=1) and (a=-1, b=1).
        // N(1+i) = 2, N(-1+i) = 2 — both coprime to ℓ=5, so the Schirokauer map is defined.
        let make_rel = |a: i64, b: i64| -> DLRelation {
            let rel = Relation {
                a: bi(a),
                b: bi(b),
                rational_exponents: ExponentVector::new(),
                algebraic_exponents: ExponentVector::new(),
                rational_sign: false,
            };
            augment_relation(rel, &k, &ell, &ideals).expect("augment should succeed")
        };

        let relations = vec![make_rel(1, 1), make_rel(-1, 1)];

        // Build a minimal FactorBase for f = x² + 1, B_rat = 5, B_alg = 5.
        let f = IntPoly::from_coeffs(vec![bi(1), bi(0), bi(1)]);
        let fb = FactorBase::new(&f, 5, 5);

        let matrix = DLMatrix::from_relations(relations, &fb);

        // Dimensions: 2 rows, rational_size + algebraic_size + 1 Schirokauer column.
        assert_eq!(matrix.num_rows(), 2);
        assert_eq!(matrix.schirokauer_rank, 1);
        assert_eq!(matrix.num_cols(), fb.rational_size() + fb.algebraic_size() + 1);
        let (rows, cols) = matrix.dimensions();
        assert_eq!(rows, 2);
        assert_eq!(cols, matrix.num_cols());
    }
}
