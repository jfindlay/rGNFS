//! Negation map and fruitless-cycle escape (BKNS).
//!
//! The negation map halves the effective group size by collapsing each pair
//! `{P, −P}` to a single canonical representative.  Because `P` and `−P`
//! always share the same x-coordinate, the canonical representative is chosen
//! by comparing y-coordinates: the one with the numerically smaller y (i.e.,
//! `y ≤ p/2`) is canonical.  This is equivalent to choosing the representative
//! with the smaller y in `[0, p)`.
//!
//! # Fruitless cycles
//!
//! A fruitless cycle of period 2 arises when the walk oscillates between a
//! point `W` and its negation `−W`.  After the negation map both states map to
//! the same canonical representative, so the canonical x-coordinate repeats
//! every 2 steps.  [`FruitlessCycleDetector`] watches for this pattern and
//! signals the caller to escape via the BKNS doubling perturbation.

use crypto_bigint::Uint;

use crate::curve::AffinePoint;
use crate::field::Fp;

// ── Canonical representative ──────────────────────────────────────────────────

/// Return the canonical representative of the equivalence class `{P, −P}`.
///
/// The canonical element is the one whose y-coordinate is numerically smaller
/// (comparing the canonical residues in `[0, p)`).  When `P` is the point at
/// infinity (which is its own negation), `P` is returned unchanged.
///
/// # Returns
///
/// `(canonical, negated)` where `negated` is `true` when `−P` was chosen.
/// The caller must flip the sign of the accumulated `(a, b)` scalars when
/// `negated` is `true`.
///
/// # Note on the comparison
///
/// `P` and `−P` always share the same x-coordinate.  The canonical
/// representative is determined by comparing y-coordinates: `min(y, p − y)`.
/// This is equivalent to choosing the representative with `y ≤ p/2`.
pub fn canonical_rep<F: Fp<4>>(pt: &AffinePoint<F>, p: &Uint<4>) -> (AffinePoint<F>, bool) {
    match pt {
        AffinePoint::Infinity => (AffinePoint::Infinity, false),
        AffinePoint::Finite { x, y } => {
            let y_uint = y.to_uint();
            // neg_y = p − y (the y-coordinate of −P).
            // Use wrapping subtraction via Uint arithmetic; p > 0 and y < p.
            let neg_y_uint = p.wrapping_sub(&y_uint);

            // Choose the representative with the smaller y.  When y == neg_y
            // (i.e., 2y ≡ 0 mod p, which means y = 0 or y = p/2 — the latter
            // impossible for odd p), P is its own negation; return P unchanged.
            if y_uint <= neg_y_uint {
                (pt.clone(), false)
            } else {
                // −P has the smaller y; return (x, neg_y).
                let neg_y = F::from_uint(neg_y_uint, p);
                (AffinePoint::Finite { x: x.clone(), y: neg_y }, true)
            }
        }
    }
}

// ── Scalar adjustment ─────────────────────────────────────────────────────────

/// Adjust accumulated scalar coefficients when the negation map fires.
///
/// If `negated == true`, the canonical representative is `−W = −(a·G + b·Q)`,
/// which equals `(n − a)·G + (n − b)·Q`.  Replace `a ← n − a`, `b ← n − b`.
///
/// # Arguments
///
/// * `a` — current scalar coefficient for G.
/// * `b` — current scalar coefficient for Q.
/// * `n` — prime group order.
#[inline]
pub fn negate_scalars(a: u64, b: u64, n: u64) -> (u64, u64) {
    // n − 0 would give n, which is ≡ 0 mod n; handle that edge case.
    let new_a = if a == 0 { 0 } else { n - a };
    let new_b = if b == 0 { 0 } else { n - b };
    (new_a, new_b)
}

// ── Fruitless-cycle detector ──────────────────────────────────────────────────

/// Number of canonical x-values retained in the sliding window.
pub const WINDOW: usize = 16;

/// Fruitless-cycle detector: a short sliding window over recent canonical
/// x-coordinates.
///
/// A fruitless cycle of length 2 occurs when the walk oscillates between `P`
/// and `−P`.  After the negation map both states have the same canonical
/// x-coordinate, so the sequence of canonical x-values shows a period-2
/// pattern: `[…, x, y, x, y, …]`.
///
/// The detector keeps the last [`WINDOW`] canonical x-coordinates in a ring
/// buffer.  [`is_fruitless`] returns `true` when the most-recently-pushed
/// value equals the value pushed two steps earlier (i.e., `buf[head−1] ==
/// buf[head−3]` in circular indexing), which is the minimal signature of a
/// period-2 loop.
///
/// [`is_fruitless`]: FruitlessCycleDetector::is_fruitless
pub struct FruitlessCycleDetector {
    buf: [u64; WINDOW],
    /// Index of the *next* write position (i.e., one past the last written).
    head: usize,
    /// Number of values pushed so far (saturates at WINDOW).
    count: usize,
}

impl FruitlessCycleDetector {
    /// Create a new detector with an empty window.
    pub fn new() -> Self {
        FruitlessCycleDetector { buf: [0u64; WINDOW], head: 0, count: 0 }
    }

    /// Push the low 64-bit word of the current canonical x-coordinate.
    pub fn push(&mut self, x_low: u64) {
        self.buf[self.head] = x_low;
        self.head = (self.head + 1) % WINDOW;
        if self.count < WINDOW {
            self.count += 1;
        }
    }

    /// Return `true` if the last 3 recorded x-values show a period-2 pattern.
    ///
    /// The pattern is: the value pushed most recently equals the value pushed
    /// two steps before it — i.e., `buf[head−1] == buf[head−3]` (mod WINDOW).
    /// This is the minimal signature of a 2-cycle in the canonical walk.
    ///
    /// Returns `false` when fewer than 3 values have been pushed.
    pub fn is_fruitless(&self) -> bool {
        if self.count < 3 {
            return false;
        }
        // head points to the *next* write slot, so:
        //   most recent  = buf[(head - 1 + WINDOW) % WINDOW]
        //   two steps ago = buf[(head - 3 + WINDOW) % WINDOW]
        let last  = (self.head + WINDOW - 1) % WINDOW;
        let older = (self.head + WINDOW - 3) % WINDOW;
        self.buf[last] == self.buf[older]
    }

    /// Reset the detector (called after an escape perturbation).
    pub fn reset(&mut self) {
        self.head = 0;
        self.count = 0;
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::Uint;
    use crate::curve::test_curves::tiny_a;
    use crate::field::FpMonty;

    /// canonical_rep is idempotent: applying it twice gives the same result.
    #[test]
    fn canonical_rep_idempotent() {
        let curve = tiny_a();
        let p = &curve.p;
        let g: AffinePoint<FpMonty> = curve.generator();
        let (canon, _) = canonical_rep(&g, p);
        let (canon2, negated2) = canonical_rep(&canon, p);
        assert_eq!(canon, canon2, "canonical_rep not idempotent");
        assert!(!negated2, "second application should not negate");
    }

    /// P and −P map to the same canonical representative.
    #[test]
    fn canonical_rep_same_for_negation() {
        let curve = tiny_a();
        let p = &curve.p;
        let g: AffinePoint<FpMonty> = curve.generator();
        let neg_g = curve.negate(&g);
        let (canon_g, _)     = canonical_rep(&g, p);
        let (canon_neg_g, _) = canonical_rep(&neg_g, p);
        assert_eq!(canon_g, canon_neg_g, "P and −P must share canonical rep");
    }

    /// negate_scalars gives (n−a, n−b) and the adjusted scalars still satisfy
    /// the walk invariant.
    #[test]
    fn negate_scalars_correct() {
        let n: u64 = 1_048_051; // TINY_A_N
        let (a, b) = (100u64, 200u64);
        let (na, nb) = negate_scalars(a, b, n);
        assert_eq!(na, n - a);
        assert_eq!(nb, n - b);
        // (n−a) + a ≡ 0 mod n
        assert_eq!((na + a) % n, 0);
        assert_eq!((nb + b) % n, 0);
    }

    /// negate_scalars handles zero inputs without wrapping to n.
    #[test]
    fn negate_scalars_zero() {
        let n: u64 = 1_048_051;
        let (na, nb) = negate_scalars(0, 0, n);
        assert_eq!(na, 0);
        assert_eq!(nb, 0);
    }

    /// FruitlessCycleDetector: fewer than 3 pushes never trigger.
    #[test]
    fn detector_needs_three_pushes() {
        let mut d = FruitlessCycleDetector::new();
        d.push(42);
        assert!(!d.is_fruitless());
        d.push(42);
        assert!(!d.is_fruitless());
    }

    /// FruitlessCycleDetector: period-2 pattern is detected.
    #[test]
    fn detector_catches_period_two() {
        let mut d = FruitlessCycleDetector::new();
        d.push(10);
        d.push(20);
        d.push(10); // buf[0]==buf[2] → fruitless
        assert!(d.is_fruitless(), "period-2 pattern not detected");
    }

    /// FruitlessCycleDetector: non-repeating sequence does not trigger.
    #[test]
    fn detector_no_false_positive() {
        let mut d = FruitlessCycleDetector::new();
        for i in 0..WINDOW {
            d.push(i as u64);
            assert!(!d.is_fruitless(), "false positive at push {i}");
        }
    }

    /// FruitlessCycleDetector: reset clears the window.
    #[test]
    fn detector_reset_clears() {
        let mut d = FruitlessCycleDetector::new();
        d.push(1);
        d.push(2);
        d.push(1);
        assert!(d.is_fruitless());
        d.reset();
        assert!(!d.is_fruitless(), "detector should be clear after reset");
    }

    /// canonical_rep on the point at infinity returns infinity unchanged.
    #[test]
    fn canonical_rep_infinity() {
        let p = Uint::<4>::from(1_048_517u64);
        let inf: AffinePoint<FpMonty> = AffinePoint::Infinity;
        let (canon, negated) = canonical_rep(&inf, &p);
        assert!(canon.is_infinity());
        assert!(!negated);
    }
}
