//! Descent-tree node and frontier types for NFS-DL individual-logarithm descent.
//!
//! This module defines the data structures of the descent tree:
//!
//! - [`DescentTarget`] — a prime or ideal being descended (rational or algebraic side).
//! - [`DescentNode`] — a node in the descent tree: the target, its rewriting relation,
//!   child nodes, and the known virtual log (for leaf nodes).
//! - [`DescentFrontier`] — a max-heap of pending descent targets, ordered by prime descending.
//!
//! # Termination invariant
//!
//! The frontier is a max-heap ordered by `target.prime()` descending. Each descent step pops
//! the largest prime, rewrites it as strictly smaller primes, and pushes those back. Since each
//! step strictly reduces the largest prime, the descent terminates when all frontier elements
//! are factor-base leaves.
//!
//! # Contract C-Descent (frozen D.C.1)
//!
//! These types are the frozen internal substrate consumed by D.C.2 (recursion) and D.C.3
//! (assembly). They are sub-track-internal — not consumed outside Track D.

use std::collections::BinaryHeap;

use crate::dl::DLRelation;

// ─── DescentTarget ────────────────────────────────────────────────────────────

/// A prime or ideal being descended.
///
/// For the rational side, this is a prime `p` (represented as `u64`).
/// For the algebraic side, this is a degree-1 prime ideal `(p, r)` where `f(r) ≡ 0 (mod p)`.
///
/// The `prime()` method returns `p` in both cases — the value that must strictly decrease at
/// each descent step (the termination invariant).
///
/// # Ordering
///
/// `DescentTarget` implements `Ord` by `prime()` ascending. Combined with `BinaryHeap` (a
/// max-heap), this means `pop()` yields the target with the **largest** prime first — the
/// ordering required by the termination invariant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DescentTarget {
    /// A rational prime `p`.
    Rational(u64),
    /// An algebraic prime ideal `(p, r)` with `f(r) ≡ 0 (mod p)`.
    Algebraic {
        /// The rational prime.
        p: u64,
        /// The root: `f(r) ≡ 0 (mod p)`.
        r: u64,
    },
}

impl DescentTarget {
    /// The prime `p` — the value that must strictly decrease at each descent step.
    ///
    /// Returns `p` for both `Rational(p)` and `Algebraic { p, r }`.
    pub fn prime(&self) -> u64 {
        match self {
            Self::Rational(p) => *p,
            Self::Algebraic { p, .. } => *p,
        }
    }
}

// Ord by prime() ascending so BinaryHeap (max-heap) pops the largest prime first.
impl PartialOrd for DescentTarget {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DescentTarget {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Primary: compare by prime() ascending (largest prime → max-heap pops it first).
        // Secondary: break ties by variant (Rational < Algebraic) then by r for Algebraic.
        // The tie-breaking is arbitrary but deterministic.
        self.prime()
            .cmp(&other.prime())
            .then_with(|| self.variant_ord().cmp(&other.variant_ord()))
            .then_with(|| self.r_ord().cmp(&other.r_ord()))
    }
}

impl DescentTarget {
    /// Variant ordinal for tie-breaking: Rational = 0, Algebraic = 1.
    fn variant_ord(&self) -> u8 {
        match self {
            Self::Rational(_) => 0,
            Self::Algebraic { .. } => 1,
        }
    }

    /// Secondary key for Algebraic tie-breaking: the root `r` (0 for Rational).
    fn r_ord(&self) -> u64 {
        match self {
            Self::Rational(_) => 0,
            Self::Algebraic { r, .. } => *r,
        }
    }
}

// ─── DescentNode ──────────────────────────────────────────────────────────────

/// A node in the descent tree: a prime/ideal being descended, its rewriting relation,
/// and references to child nodes.
///
/// The descent tree is built top-down: the root is the target `h` (after initialization-
/// smoothing), interior nodes are medium primes rewritten via special-q descent, and leaves
/// are factor-base elements with known virtual logs.
///
/// # Invariants
///
/// - `target.prime() > child.target.prime()` for all children (strict descent — termination).
/// - `known_log.is_some()` iff `target` is a factor-base element (leaf node).
/// - `rewriting_relation.is_some()` iff this is an interior node (not a leaf).
///
/// # Type parameter
///
/// `F` is the field element type for virtual logs (e.g., `FpNaive4`). Leaf nodes store
/// `known_log: Some(F)` from the `VirtualLogTable`; interior nodes store `None` until
/// D.C.3's assembly fills them in.
#[derive(Debug, Clone)]
pub struct DescentNode<F> {
    /// The prime/ideal being descended.
    pub target: DescentTarget,

    /// The DL relation that rewrites `log(target)` as a combination of smaller primes' logs.
    ///
    /// `None` for leaf nodes (factor-base elements with known logs).
    /// `Some(rel)` for interior nodes: the relation from special-q descent.
    pub rewriting_relation: Option<DLRelation>,

    /// Child nodes: the smaller primes from the rewriting relation.
    ///
    /// Empty for leaf nodes. For interior nodes, each child's `target.prime()` is strictly
    /// less than `self.target.prime()` (the termination invariant).
    pub children: Vec<DescentNode<F>>,

    /// The known virtual log of this target, if it is a factor-base element.
    ///
    /// `Some(log)` for leaves (looked up from `VirtualLogTable`).
    /// `None` for interior nodes (computed during assembly by combining children's logs).
    pub known_log: Option<F>,
}

// ─── DescentFrontier ──────────────────────────────────────────────────────────

/// A heap entry wrapping a prime key and payload for `BinaryHeap` ordering.
///
/// `BinaryHeap` requires `Ord` on its elements. `DescentNode<F>` is generic over `F` which
/// may not implement `Ord`. This wrapper implements `Ord` based solely on the prime key,
/// ignoring the payload — so the heap orders entries by prime without requiring `F: Ord`.
struct HeapEntry<F> {
    /// The ordering key: the prime value.
    prime: u64,
    /// The descent target (payload).
    target: DescentTarget,
    /// The descent node (payload).
    node: DescentNode<F>,
}

impl<F> PartialEq for HeapEntry<F> {
    fn eq(&self, other: &Self) -> bool {
        // Order by prime only; payload is not compared.
        self.prime == other.prime
    }
}

impl<F> Eq for HeapEntry<F> {}

impl<F> PartialOrd for HeapEntry<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> Ord for HeapEntry<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Ascending order by prime: BinaryHeap (max-heap) pops the largest prime first.
        self.prime.cmp(&other.prime)
    }
}

/// The frontier of medium primes awaiting descent: a max-heap ordered by prime (largest-first).
///
/// The largest-first ordering is the termination invariant: each descent step pops the largest
/// prime `q` from the frontier, finds a relation rewriting `q` as smaller primes, and pushes
/// those smaller primes back onto the frontier. Since each step strictly reduces the largest
/// prime, the descent terminates when all frontier elements are factor-base leaves.
///
/// # Type parameter
///
/// `F` is the field element type for virtual logs (e.g., `FpNaive4`).
pub struct DescentFrontier<F> {
    /// Max-heap of `HeapEntry<F>` values, ordered by `prime` descending.
    ///
    /// `HeapEntry` implements `Ord` based solely on the prime key, so the heap orders entries
    /// by prime without requiring `F: Ord`.
    heap: BinaryHeap<HeapEntry<F>>,
}

impl<F> DescentFrontier<F> {
    /// Create an empty frontier.
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new() }
    }

    /// Push a target onto the frontier with an initial node.
    ///
    /// The target's prime is used as the ordering key.
    pub fn push(&mut self, target: DescentTarget, node: DescentNode<F>) {
        let prime = target.prime();
        self.heap.push(HeapEntry { prime, target, node });
    }

    /// Pop the largest-prime target from the frontier.
    ///
    /// Returns `None` if the frontier is empty.
    pub fn pop_largest(&mut self) -> Option<(DescentTarget, DescentNode<F>)> {
        self.heap.pop().map(|entry| (entry.target, entry.node))
    }

    /// Returns `true` if the frontier is empty (all targets descended to leaves).
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// The number of targets remaining on the frontier.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

impl<F> Default for DescentFrontier<F> {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Unit tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descent_target_prime_rational() {
        let t = DescentTarget::Rational(17);
        assert_eq!(t.prime(), 17);
    }

    #[test]
    fn descent_target_prime_algebraic() {
        let t = DescentTarget::Algebraic { p: 13, r: 5 };
        assert_eq!(t.prime(), 13);
    }

    #[test]
    fn frontier_ordering_largest_first() {
        // Push primes in arbitrary order; pop should yield largest-first.
        let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
        for p in [7u64, 3, 11, 5, 13, 2] {
            let target = DescentTarget::Rational(p);
            let node = DescentNode {
                target: target.clone(),
                rewriting_relation: None,
                children: vec![],
                known_log: None,
            };
            frontier.push(target, node);
        }
        let mut popped = vec![];
        while let Some((t, _)) = frontier.pop_largest() {
            popped.push(t.prime());
        }
        assert_eq!(popped, vec![13, 11, 7, 5, 3, 2]);
    }

    #[test]
    fn frontier_is_empty_and_len() {
        let mut frontier: DescentFrontier<u64> = DescentFrontier::new();
        assert!(frontier.is_empty());
        assert_eq!(frontier.len(), 0);

        let target = DescentTarget::Rational(7);
        let node = DescentNode {
            target: target.clone(),
            rewriting_relation: None,
            children: vec![],
            known_log: None,
        };
        frontier.push(target, node);
        assert!(!frontier.is_empty());
        assert_eq!(frontier.len(), 1);

        frontier.pop_largest();
        assert!(frontier.is_empty());
    }
}
