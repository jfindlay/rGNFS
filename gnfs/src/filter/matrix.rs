//! Sparse GF(2) matrix with relation-provenance map for GNFS filtering.
//!
//! This module defines the core data structures consumed by singleton removal, clique pruning
//! and merging, and the linear algebra step. The representation is row-major (each row =
//! sorted ``Vec<usize>`` of set-column indices) with a maintained column-weight side table,
//! balancing the needs of row-XOR operations (merging) and column-weight queries (singleton
//! removal).
//!
//! # Column layout
//!
//! Total columns = ``FactorBase::matrix_width()`` = rational_size + algebraic_size + obstruction_count.
//!
//! - ``[0, rational_size)``: rational factor-base columns.
//! - ``[rational_size, rational_size + algebraic_size)``: algebraic factor-base columns.
//! - ``[rational_size + algebraic_size, matrix_width)``: obstruction columns (sign bit first,
//!   then quadratic-character columns filled by the linear algebra step).
//!
//! # Provenance
//!
//! Each row carries a sorted, deduplicated ``Vec<usize>`` of *original* relation indices
//! (indices into the ``Vec<Relation>`` passed to ``build_matrix``). For a freshly built row,
//! provenance = ``[original_index]``. Under merging, provenance is combined by sorted union.
//! The square root step expands a nullspace vector by collecting ``row.provenance`` for each
//! selected row and recovering the original ``(a, b)`` pairs.

// ─── EXCESS_FLOOR ─────────────────────────────────────────────────────────────

/// Minimum excess the clique pruning step must preserve.
///
/// excess = rows − (columns − obstruction_count). At toy scale any positive excess
/// suffices for a non-trivial nullspace; 20 is a conservative floor that keeps the
/// matrix well-overdetermined. Annotated as scale-dependent (principle-4): at
/// cryptographic scale the floor is typically set to ~200 or a fraction of the column
/// count. Singleton removal defines the constant; clique pruning enforces it.
pub const EXCESS_FLOOR: usize = 20;

// ─── MatrixRow ────────────────────────────────────────────────────────────────

/// One row of the sparse GF(2) matrix, with its relation-provenance record.
///
/// ``cols`` is a sorted ``Vec<usize>`` of column indices where this row has a 1 (GF(2)).
/// Column layout matches ``FactorBase::matrix_width()``:
///
/// - ``[0, rational_size)``: rational factor-base columns.
/// - ``[rational_size, rational_size + algebraic_size)``: algebraic columns.
/// - ``[rational_size + algebraic_size, matrix_width)``: obstruction columns
///   (sign at ``rational_size + algebraic_size``; quadratic-character columns follow —
///   the linear algebra step fills those; the filtering step carries them as zeros).
///
/// ``provenance`` is a sorted, deduplicated ``Vec<usize>`` of *original* relation indices
/// (indices into the ``Vec<Relation>`` passed to ``build_matrix``). For a freshly built
/// row, provenance = ``[original_index]``. Under merge (clique pruning and merging),
/// provenance is combined by sorted union. The square root step expands a nullspace vector
/// by collecting ``row.provenance`` for each selected row and recovering the original
/// ``(a, b)`` pairs.
///
/// Provenance stores original indices, not pre-reduced row sums — over-specified per
/// the provenance design so the square root step can recover actual ``(a, b)`` pairs without
/// re-deriving anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixRow {
    /// Sorted column indices where this row has a 1 in GF(2).
    pub cols: Vec<usize>,
    /// Sorted, deduplicated original relation indices this row derives from.
    pub provenance: Vec<usize>,
}

impl MatrixRow {
    /// XOR this row with ``other`` in GF(2), unioning their provenance sets.
    ///
    /// Used by clique pruning and merging: combining two rows that share a column eliminates
    /// that column (symmetric difference of ``cols``) and unions their provenance.
    ///
    /// :param other: The row to XOR with.
    /// :returns: A new row with the symmetric difference of columns and union of provenance.
    pub fn xor_merge(&self, other: &MatrixRow) -> MatrixRow {
        // Symmetric difference of two sorted vecs — O(n + m).
        let mut cols = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < self.cols.len() && j < other.cols.len() {
            match self.cols[i].cmp(&other.cols[j]) {
                std::cmp::Ordering::Less => {
                    cols.push(self.cols[i]);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    cols.push(other.cols[j]);
                    j += 1;
                }
                std::cmp::Ordering::Equal => {
                    // Both have a 1 here: XOR = 0, skip both.
                    i += 1;
                    j += 1;
                }
            }
        }
        cols.extend_from_slice(&self.cols[i..]);
        cols.extend_from_slice(&other.cols[j..]);

        // Sorted union of provenance sets — O(n + m).
        let mut provenance = Vec::new();
        let mut pi = 0;
        let mut pj = 0;
        while pi < self.provenance.len() && pj < other.provenance.len() {
            match self.provenance[pi].cmp(&other.provenance[pj]) {
                std::cmp::Ordering::Less => {
                    provenance.push(self.provenance[pi]);
                    pi += 1;
                }
                std::cmp::Ordering::Greater => {
                    provenance.push(other.provenance[pj]);
                    pj += 1;
                }
                std::cmp::Ordering::Equal => {
                    // Deduplicate: include once.
                    provenance.push(self.provenance[pi]);
                    pi += 1;
                    pj += 1;
                }
            }
        }
        provenance.extend_from_slice(&self.provenance[pi..]);
        provenance.extend_from_slice(&other.provenance[pj..]);

        MatrixRow { cols, provenance }
    }
}

// ─── SparseMatrix ─────────────────────────────────────────────────────────────

/// Sparse GF(2) matrix over a factor base, with relation-provenance map.
///
/// Total columns = ``FactorBase::matrix_width()`` = rational_size + algebraic_size + obstruction_count.
/// The obstruction block starts at ``obstruction_col_start`` = rational_size + algebraic_size.
/// Singleton removal and merging skip any column >= ``obstruction_col_start``.
///
/// ``col_weights[c]`` = number of rows with a 1 in column ``c``. Maintained in sync with
/// ``rows`` by all mutating operations (``remove_row``, ``xor_merge_rows``).
///
/// ``excess()`` = rows.len() − (num_cols − obstruction_count). Singleton removal reports it;
/// clique pruning enforces >= ``EXCESS_FLOOR``.
#[derive(Debug, Clone)]
pub struct SparseMatrix {
    /// The rows of the matrix, each with its column set and provenance.
    pub rows: Vec<MatrixRow>,
    /// Total number of columns = FactorBase::matrix_width().
    pub num_cols: usize,
    /// Index of the first obstruction column = rational_size + algebraic_size.
    pub obstruction_col_start: usize,
    /// Number of obstruction columns = FactorBase::obstruction_count.
    pub obstruction_count: usize,
    /// col_weights[c] = number of rows with a 1 in column c.
    pub col_weights: Vec<u32>,
}

impl SparseMatrix {
    /// Current excess: rows.len() − (num_cols − obstruction_count).
    ///
    /// Positive excess means the matrix is overdetermined (more rows than non-obstruction
    /// columns), which is required for a non-trivial nullspace. Clique pruning enforces
    /// excess >= ``EXCESS_FLOOR`` during pruning.
    pub fn excess(&self) -> isize {
        self.rows.len() as isize - (self.num_cols as isize - self.obstruction_count as isize)
    }

    /// Remove a row by index, updating ``col_weights``.
    ///
    /// Used by singleton removal and clique pruning.
    ///
    /// IMPORTANT: uses ordered removal (``Vec::remove``, not ``swap_remove``) to preserve
    /// row-index stability during singleton fixpoint iteration.
    ///
    /// :param row_idx: Index of the row to remove.
    /// :panics: If ``row_idx`` is out of bounds.
    pub fn remove_row(&mut self, row_idx: usize) {
        let row = self.rows.remove(row_idx);
        for &col in &row.cols {
            self.col_weights[col] -= 1;
        }
    }
}
