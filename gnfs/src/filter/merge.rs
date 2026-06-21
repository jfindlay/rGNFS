//! Clique/excess pruning and column merging for the sparse GF(2) matrix.
//!
//! This module implements the two post-singleton-removal reduction steps (clique pruning
//! and column merging):
//!
//! 1. **Clique/excess pruning** ([`prune_cliques`]): greedily removes the heaviest rows
//!    (relations) while the matrix excess exceeds [`EXCESS_FLOOR`]. In the graph view,
//!    relations are edges and primes (non-obstruction columns) are nodes; a clique is a
//!    set of relations sharing a common prime. The greedy heuristic approximates clique
//!    pruning by targeting the highest-weight rows first.
//!
//! 2. **Column merging** ([`merge_columns`]): eliminates low-weight non-obstruction columns
//!    by XOR-merging the rows that contain them. A weight-2 column appears in exactly two
//!    rows; XOR-merging those rows eliminates the column (symmetric difference cancels it)
//!    and unions their provenance. After the weight-2 pass, a weight-3 pass is performed.
//!    Higher-weight columns are left for the linear algebra step (demonstration fidelity —
//!    Cavallar's full
//!    heuristic would continue to higher k, but at toy scale k=2 and k=3 are sufficient).
//!
//! # Cavallar weight-cost ordering (principle-4 annotation)
//!
//! In production NFS, merges are ordered by a weight-cost heuristic: prefer merges that
//! minimise the increase in total matrix weight (Cavallar 1996). At toy scale the matrix
//! is small enough that any merge order gives a tractable matrix, so the weight-saving the
//! heuristic buys is under-exposed. This implementation processes columns in order of
//! increasing weight (weight-2 first, then weight-3), breaking ties by column index —
//! a simplified Cavallar ordering that is correct in principle but whose benefit is
//! invisible at toy scale (principle-4: scale-dependent optimisation).
//!
//! # Provenance
//!
//! Every merge operation unions the provenance sets of the merged rows. After the full
//! pipeline (build_matrix → remove_singletons → prune_cliques → merge_columns), each
//! surviving row's provenance is the set of original relation indices whose GF(2) XOR
//! equals that row's column set. The square root step uses this to recover the original
//! (a, b) pairs.

use crate::filter::matrix::{SparseMatrix, EXCESS_FLOOR};

// ─── prune_cliques ────────────────────────────────────────────────────────────

/// Clique/excess pruning: greedily remove heavy rows while excess > EXCESS_FLOOR.
///
/// The "graph view": relations are edges, primes (non-obstruction columns) are nodes.
/// A clique is a set of relations sharing a common prime. This greedy heuristic
/// approximates clique pruning by removing the highest-weight row (most columns set)
/// as long as `matrix.excess() > EXCESS_FLOOR as isize`. Ties in weight are broken
/// by row index (lowest index removed first) for determinism.
///
/// Stops when `excess() == EXCESS_FLOOR` or no rows remain. After pruning,
/// `excess() >= EXCESS_FLOOR` is guaranteed (or the matrix is empty).
///
/// :param matrix: The singleton-removed matrix (consumed).
/// :returns: The pruned matrix with `excess() >= EXCESS_FLOOR`.
pub fn prune_cliques(mut matrix: SparseMatrix) -> SparseMatrix {
    loop {
        if matrix.excess() <= EXCESS_FLOOR as isize {
            break;
        }
        if matrix.rows.is_empty() {
            break;
        }

        // Find the row with the maximum weight (number of set columns).
        // Ties broken by lowest row index for determinism.
        let max_weight = matrix.rows.iter().map(|r| r.cols.len()).max().unwrap_or(0);
        if max_weight == 0 {
            // All rows are empty — nothing useful to remove.
            break;
        }

        // Find the first row (lowest index) with the maximum weight.
        let row_idx = matrix
            .rows
            .iter()
            .enumerate()
            .find(|(_, r)| r.cols.len() == max_weight)
            .map(|(i, _)| i)
            .unwrap();

        matrix.remove_row(row_idx);
    }

    matrix
}

// ─── merge_columns ────────────────────────────────────────────────────────────

/// Column merging: eliminate low-weight non-obstruction columns by XOR-merging rows.
///
/// Performs two passes over the non-obstruction columns:
///
/// **Pass 1 — weight-2 columns:** For each non-obstruction column with weight exactly 2,
/// find the two rows containing it, XOR-merge them (symmetric difference of cols, union
/// of provenance), remove the two source rows, and append the merged row. The shared
/// column cancels in the XOR and is eliminated. Columns are processed in order of
/// increasing weight then by column index (simplified Cavallar ordering — see module
/// docstring for the principle-4 annotation).
///
/// **Pass 2 — weight-3 columns:** After the weight-2 pass, re-scan col_weights and
/// process weight-3 columns similarly: find the three rows, merge r0⊕r1 then ⊕r2,
/// remove the three source rows, append the merged row.
///
/// Stops after k=3 (demonstration fidelity). Cavallar's full heuristic would continue
/// to higher k, but at toy scale k=2 and k=3 are sufficient for a tractable matrix.
///
/// **Principle-4 annotation (Cavallar weight-cost ordering):** In production NFS,
/// merges are ordered by a weight-cost heuristic that minimises the increase in total
/// matrix weight. At toy scale the matrix is small enough that any merge order gives a
/// tractable matrix, so the weight-saving the heuristic buys is under-exposed. This
/// implementation uses simplified Cavallar ordering (increasing weight, then column
/// index) — correct in principle, but the benefit is invisible at toy scale.
///
/// **EXCESS_FLOOR note:** `merge_columns` does not enforce `EXCESS_FLOOR`. Merging
/// reduces both row count and column count (eliminates columns), so the excess formula
/// changes; the floor is not meaningful in the same way as during pruning.
///
/// :param matrix: The pruned matrix (consumed).
/// :returns: The column-merged matrix.
pub fn merge_columns(mut matrix: SparseMatrix) -> SparseMatrix {
    // Pass 1: weight-2 columns.
    matrix = merge_pass(&mut matrix, 2);

    // Re-scan col_weights after pass 1 before doing pass 2 (some weight-3 columns may
    // have become weight-2 or weight-1 during the weight-2 pass).
    matrix = merge_pass(&mut matrix, 3);

    matrix
}

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Perform one merge pass for columns of the given target weight.
///
/// Collects all non-obstruction columns currently at `target_weight`, then processes
/// them in column-index order (deterministic). For each such column, re-checks the
/// current weight (it may have changed due to earlier merges in this pass), then
/// performs the merge if the weight still matches.
///
/// :param matrix: The matrix to reduce (consumed and returned).
/// :param target_weight: The column weight to target (2 or 3).
/// :returns: The reduced matrix after this pass.
fn merge_pass(matrix: &mut SparseMatrix, target_weight: u32) -> SparseMatrix {
    // We need to take ownership; use a swap trick.
    let mut m = std::mem::replace(
        matrix,
        SparseMatrix {
            rows: Vec::new(),
            num_cols: 0,
            obstruction_col_start: 0,
            obstruction_count: 0,
            col_weights: Vec::new(),
        },
    );

    // Collect candidate columns (non-obstruction, current weight == target_weight).
    // Process in column-index order for determinism (simplified Cavallar ordering).
    let candidates: Vec<usize> = (0..m.obstruction_col_start)
        .filter(|&c| m.col_weights[c] == target_weight)
        .collect();

    for col in candidates {
        // Re-check weight: earlier merges in this pass may have changed it.
        if m.col_weights[col] != target_weight {
            continue;
        }

        // Find the rows containing this column.
        let containing_rows: Vec<usize> = m
            .rows
            .iter()
            .enumerate()
            .filter(|(_, r)| r.cols.binary_search(&col).is_ok())
            .map(|(i, _)| i)
            .collect();

        if containing_rows.len() != target_weight as usize {
            // Inconsistency between col_weights and actual rows — skip defensively.
            continue;
        }

        match target_weight {
            2 => {
                let (i0, i1) = (containing_rows[0], containing_rows[1]);
                merge_two_rows(&mut m, i0, i1);
            }
            3 => {
                let (i0, i1, i2) = (containing_rows[0], containing_rows[1], containing_rows[2]);
                merge_three_rows(&mut m, i0, i1, i2);
            }
            _ => unreachable!("merge_pass called with unsupported target_weight"),
        }
    }

    m
}

/// Merge two rows at indices i0 and i1 (i0 < i1 assumed), updating col_weights.
///
/// Removes both rows, appends their XOR-merge, and updates col_weights:
/// - Decrement for all cols in r0 and r1.
/// - Increment for all cols in new_row.
/// Net effect for a column in both r0 and r1: decremented twice, not incremented → -2.
///
/// :param m: The matrix to mutate in place.
/// :param i0: Index of the first row (must be < i1).
/// :param i1: Index of the second row.
fn merge_two_rows(m: &mut SparseMatrix, i0: usize, i1: usize) {
    // Ensure i0 < i1 so that removing i1 first doesn't shift i0.
    let (lo, hi) = if i0 < i1 { (i0, i1) } else { (i1, i0) };

    // Clone the rows before removal (remove_row takes ownership of the row).
    let r_hi = m.rows[hi].clone();
    let r_lo = m.rows[lo].clone();

    // Decrement col_weights for both rows before removing them.
    // (remove_row also decrements, so we must NOT call remove_row here — we do it manually.)
    // Actually, remove_row decrements col_weights internally, so we just call it.
    // Remove hi first (higher index) so lo's index stays valid.
    m.rows.remove(hi);
    for &c in &r_hi.cols {
        m.col_weights[c] -= 1;
    }
    m.rows.remove(lo);
    for &c in &r_lo.cols {
        m.col_weights[c] -= 1;
    }

    // Compute the merged row.
    let new_row = r_lo.xor_merge(&r_hi);

    // Increment col_weights for the new row.
    for &c in &new_row.cols {
        m.col_weights[c] += 1;
    }

    // Append the merged row.
    m.rows.push(new_row);
}

/// Merge three rows at indices i0, i1, i2, updating col_weights.
///
/// Merges r0⊕r1 first, then ⊕r2. Removes all three source rows and appends the result.
///
/// :param m: The matrix to mutate in place.
/// :param i0: Index of the first row.
/// :param i1: Index of the second row.
/// :param i2: Index of the third row.
fn merge_three_rows(m: &mut SparseMatrix, i0: usize, i1: usize, i2: usize) {
    // Sort indices descending so we can remove from highest to lowest without shifting.
    let mut indices = [i0, i1, i2];
    indices.sort_unstable();
    let [lo, mid, hi] = indices;

    // Clone all three rows before removal.
    let r0 = m.rows[lo].clone();
    let r1 = m.rows[mid].clone();
    let r2 = m.rows[hi].clone();

    // Remove in descending index order to preserve validity.
    m.rows.remove(hi);
    for &c in &r2.cols {
        m.col_weights[c] -= 1;
    }
    m.rows.remove(mid);
    for &c in &r1.cols {
        m.col_weights[c] -= 1;
    }
    m.rows.remove(lo);
    for &c in &r0.cols {
        m.col_weights[c] -= 1;
    }

    // Merge: (r0 ⊕ r1) ⊕ r2.
    let tmp = r0.xor_merge(&r1);
    let new_row = tmp.xor_merge(&r2);

    // Increment col_weights for the new row.
    for &c in &new_row.cols {
        m.col_weights[c] += 1;
    }

    // Append the merged row.
    m.rows.push(new_row);
}
