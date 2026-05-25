//! Coordinator: receives distinguished points from walker threads and detects collisions.
//!
//! The coordinator owns a hash table keyed by `x_low` (the low 64-bit word of the
//! distinguished point's x-coordinate).  When two records with the same key but
//! different `(a, b)` pairs arrive, they represent two walk trajectories that have
//! passed through the same point — a collision that lets us recover `k`.
//!
//! # Collision recovery
//!
//! If `W = a₁·G + b₁·Q = a₂·G + b₂·Q`, then
//! `(b₂ − b₁)·Q = (a₁ − a₂)·G`, so `k = (a₁ − a₂) / (b₂ − b₁) mod n`
//! (provided `b₂ ≠ b₁ mod n`).
//!
//! A degenerate collision (`b₁ = b₂ mod n`) cannot yield `k`; the coordinator
//! discards it and waits for the next collision.

use std::collections::HashMap;

use crate::ecdlp::dp::DpRecord;

// ── Coordinator ───────────────────────────────────────────────────────────────

/// Coordinator for the van Oorschot–Wiener parallel rho algorithm.
///
/// Maintains a hash table of seen distinguished points and detects collisions
/// that allow the discrete logarithm to be recovered.
pub struct Coordinator {
    /// Distinguished-point table, keyed by the low word of the x-coordinate.
    table: HashMap<u64, DpRecord>,
    /// Prime group order.
    n: u64,
}

impl Coordinator {
    /// Create a new coordinator for a group of order `n`.
    pub fn new(n: u64) -> Self {
        Coordinator { table: HashMap::new(), n }
    }

    /// Insert a distinguished point and attempt to recover `k` on collision.
    ///
    /// Returns `Some(k)` if the incoming record collides with a stored one and
    /// `k` is recoverable (i.e., the `b` coefficients differ mod `n`).
    ///
    /// A *collision* is two records with the same `x_low` but different `(a, b)`
    /// pairs.  Records from the same walker (same `walk_id`) with the same `x_low`
    /// are a *fruitless cycle* — the walk looped back to a DP it already visited —
    /// and are discarded without updating the table.
    ///
    /// # Arguments
    ///
    /// * `dp` — the incoming distinguished-point record.
    pub fn insert(&mut self, dp: DpRecord) -> Option<u64> {
        use std::collections::hash_map::Entry;

        match self.table.entry(dp.x_low) {
            Entry::Vacant(e) => {
                // First time we see this x_low; store and wait.
                e.insert(dp);
                None
            }
            Entry::Occupied(e) => {
                let stored = e.get();

                // Fruitless cycle: same walker revisiting its own DP.
                // This happens when a walk loops without hitting a new DP.
                // Discard without updating the table so the stored record
                // (from a potentially different walker) remains available.
                if stored.walk_id == dp.walk_id && stored.a == dp.a && stored.b == dp.b {
                    return None;
                }

                // Genuine collision: two distinct (a, b) pairs at the same point.
                let a1 = stored.a;
                let b1 = stored.b;
                let a2 = dp.a;
                let b2 = dp.b;

                // k = (a1 - a2) / (b2 - b1) mod n
                recover_k(a1, b1, a2, b2, self.n)
            }
        }
    }

    /// Return the number of distinct distinguished points currently stored.
    #[inline]
    pub fn table_len(&self) -> usize {
        self.table.len()
    }
}

// ── k recovery ────────────────────────────────────────────────────────────────

/// Attempt to recover `k` from a collision between two walk states.
///
/// Given `a₁·G + b₁·Q = a₂·G + b₂·Q`, solves for `k = Q/G`:
/// `k = (a₁ − a₂) / (b₂ − b₁) mod n`.
///
/// Returns `None` if `b₂ = b₁ mod n` (degenerate; no information about `k`).
fn recover_k(a1: u64, b1: u64, a2: u64, b2: u64, n: u64) -> Option<u64> {
    let db = submod64(b2, b1, n); // b2 - b1 mod n
    if db == 0 {
        return None; // degenerate collision
    }
    let da = submod64(a1, a2, n); // a1 - a2 mod n
    let db_inv = inv_mod_prime(db, n);
    Some(mulmod64(da, db_inv, n))
}

// ── Modular arithmetic helpers ────────────────────────────────────────────────

#[inline]
fn mulmod64(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

#[inline]
fn submod64(a: u64, b: u64, n: u64) -> u64 {
    if a >= b { a - b } else { a + n - b }
}

/// Modular inverse via Fermat's little theorem: `a^(n-2) mod n`.
///
/// Requires `n` prime and `a ≠ 0 mod n`.
fn inv_mod_prime(a: u64, n: u64) -> u64 {
    debug_assert!(a != 0, "inv_mod_prime: zero has no inverse");
    let mut result: u64 = 1;
    let mut base: u64 = a % n;
    let mut exp: u64 = n - 2;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mulmod64(result, base, n);
        }
        base = mulmod64(base, base, n);
        exp >>= 1;
    }
    result
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Small prime for arithmetic tests.
    const N: u64 = 1_048_051; // TINY_A_N

    #[test]
    fn no_collision_on_first_insert() {
        let mut coord = Coordinator::new(N);
        let dp = DpRecord { x_low: 0x100, a: 1, b: 2, walk_id: 0 };
        assert!(coord.insert(dp).is_none());
    }

    #[test]
    fn collision_recovers_k() {
        // Construct a synthetic collision: two DPs at the same x_low with
        // known (a, b) pairs.  Verify that the recovered k satisfies the
        // linear equation.
        let mut coord = Coordinator::new(N);

        // Choose a1, b1, k such that a1 + k*b1 = a2 + k*b2 (mod N).
        // Equivalently: k = (a1 - a2) / (b2 - b1) mod N.
        let k_expected: u64 = 42;
        let a1: u64 = 100;
        let b1: u64 = 7;
        // a2 = a1 - k*(b2-b1) mod N; pick b2 = 11.
        let b2: u64 = 11;
        let db = submod64(b2, b1, N); // b2 - b1 = 4
        let a2 = submod64(a1, mulmod64(k_expected, db, N), N);

        let dp1 = DpRecord { x_low: 0xABCD, a: a1, b: b1, walk_id: 0 };
        let dp2 = DpRecord { x_low: 0xABCD, a: a2, b: b2, walk_id: 1 };

        assert!(coord.insert(dp1).is_none(), "first insert should not collide");
        let k = coord.insert(dp2).expect("second insert should collide");
        assert_eq!(k, k_expected, "recovered k mismatch");
    }

    #[test]
    fn degenerate_collision_returns_none() {
        // b1 = b2 mod N → no k can be recovered.
        let mut coord = Coordinator::new(N);
        let dp1 = DpRecord { x_low: 0x200, a: 5, b: 3, walk_id: 0 };
        let dp2 = DpRecord { x_low: 0x200, a: 9, b: 3, walk_id: 1 };
        coord.insert(dp1);
        assert!(coord.insert(dp2).is_none(), "degenerate collision should return None");
    }

    #[test]
    fn fruitless_cycle_ignored() {
        // Same walker, same (a, b) — fruitless cycle; table unchanged.
        let mut coord = Coordinator::new(N);
        let dp = DpRecord { x_low: 0x300, a: 1, b: 2, walk_id: 0 };
        coord.insert(dp.clone());
        assert_eq!(coord.table_len(), 1);
        // Re-inserting the identical record should not collide.
        assert!(coord.insert(dp).is_none());
        assert_eq!(coord.table_len(), 1);
    }
}
