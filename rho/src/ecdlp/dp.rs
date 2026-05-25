//! Distinguished-point predicate and record type.
//!
//! A *distinguished point* (DP) is a walk state whose x-coordinate has at least
//! `theta` low-order zero bits.  The expected number of steps between DPs is
//! `2^theta`, so `theta` trades off memory (fewer DPs stored) against the
//! expected number of steps per walker before a DP is emitted.
//!
//! # van Oorschot–Wiener parallel rho
//!
//! Each walker thread emits a [`DpRecord`] whenever it lands on a distinguished
//! point.  The coordinator collects these records and detects collisions: two
//! records with the same `x_low` but different `(a, b)` pairs imply that two
//! walk trajectories have merged, allowing `k` to be recovered.

// ── Distinguished-point predicate ────────────────────────────────────────────

/// Return `true` when the low `theta` bits of `x_low_word` are all zero.
///
/// A point is "distinguished" when its x-coordinate has at least `theta`
/// trailing zero bits.  The expected density of DPs in the walk is `2^{-theta}`.
///
/// # Arguments
///
/// * `x_low_word` — low 64-bit word of the point's x-coordinate.
/// * `theta` — number of required low-order zero bits (0 ≤ theta ≤ 63).
#[inline]
pub fn is_distinguished(x_low_word: u64, theta: u32) -> bool {
    if theta == 0 {
        // Every point is distinguished when theta = 0.
        return true;
    }
    // Mask the low `theta` bits; a DP has all of them zero.
    let mask = (1u64 << theta).wrapping_sub(1);
    (x_low_word & mask) == 0
}

// ── Distinguished-point record ────────────────────────────────────────────────

/// A distinguished point emitted by a walker thread.
///
/// Carries the walk's accumulated scalar coefficients `(a, b)` at the moment
/// the DP was detected, plus the low word of the x-coordinate used as the
/// hash-table key in the coordinator.
///
/// Invariant: the walk point satisfies `W = a·G + b·Q` at the time of emission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DpRecord {
    /// Low 64-bit word of the x-coordinate (hash-table key).
    pub x_low: u64,
    /// Accumulated scalar coefficient for G: `W = a·G + b·Q`.
    pub a: u64,
    /// Accumulated scalar coefficient for Q: `W = a·G + b·Q`.
    pub b: u64,
    /// Index of the walker thread that produced this record.
    pub walk_id: usize,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theta_zero_always_distinguished() {
        // Every x value is a DP when theta = 0.
        for x in [0u64, 1, 2, 3, 0xFFFF_FFFF_FFFF_FFFF] {
            assert!(is_distinguished(x, 0), "theta=0: x={x} should be distinguished");
        }
    }

    #[test]
    fn theta_one_even_only() {
        assert!(is_distinguished(0, 1));
        assert!(!is_distinguished(1, 1));
        assert!(is_distinguished(2, 1));
        assert!(!is_distinguished(3, 1));
    }

    #[test]
    fn theta_eight_low_byte_zero() {
        assert!(is_distinguished(0x0000_0000_0000_0100, 8));
        assert!(!is_distinguished(0x0000_0000_0000_0101, 8));
        assert!(!is_distinguished(0x0000_0000_0000_00FF, 8));
        assert!(is_distinguished(0x0000_0000_0000_0000, 8));
    }

    #[test]
    fn theta_63_only_zero() {
        assert!(is_distinguished(0, 63));
        assert!(!is_distinguished(1 << 62, 63));
    }
}
