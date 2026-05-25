//! r-adding walk and partition function (Teske 1998).
//!
//! The r-adding walk maintains the invariant that the current point `W` satisfies
//! `W = a·G + b·Q` for tracked scalars `a, b ∈ ℤ/nℤ`, where `G` is the curve generator
//! and `Q` is the target point.  A solution `k` to `Q = k·G` can be recovered when two
//! walk states collide at the same point: `a₁·G + b₁·Q = a₂·G + b₂·Q` implies
//! `k = (a₁ − a₂) / (b₂ − b₁) mod n` (when `b₂ ≠ b₁ mod n`).
//!
//! # Walk design
//!
//! The `r = 20` addend table `R[0..r]` is built once from random scalars:
//! each `R[i] = αᵢ·G + βᵢ·Q` with `αᵢ, βᵢ ← ℤ/nℤ`.  The partition function
//! is `i = x mod r` where `x` is the low 64 bits of the current point's x-coordinate.
//!
//! Brent's cycle detection is applied in [`super::solve_brent`]; this module provides
//! only the walk primitive used by the solver.

use crypto_bigint::Uint;
use rand::RngCore;

use crate::curve::{AffinePoint, Curve, JacobianPoint};
use crate::field::Fp;

// ── Constants ────────────────────────────────────────────────────────────────

/// Number of addends in the r-adding walk table (Teske recommends r ≈ 20).
pub const R: usize = 20;

// ── Scalar arithmetic mod n ──────────────────────────────────────────────────

/// Add two 64-bit scalars modulo `n` (64-bit modulus).
///
/// Both inputs and the modulus must be < 2^64.
#[inline]
fn add_mod_n(a: u64, b: u64, n: u64) -> u64 {
    // Use u128 to avoid overflow before the reduction.
    ((a as u128 + b as u128) % n as u128) as u64
}

// ── Addend table ─────────────────────────────────────────────────────────────

/// One entry in the r-adding table.
///
/// Stores the affine point `R = α·G + β·Q` together with the scalar pair `(α, β)`.
/// When a walk step selects entry `i`, the current point is updated as
/// `W ← W + R[i]`, and the tracked scalars as `a ← a + α mod n`, `b ← b + β mod n`.
#[derive(Clone, Debug)]
pub struct Addend<F: Fp> {
    /// Affine point `α·G + β·Q`.
    pub point: AffinePoint<F>,
    /// Scalar coefficient for G.
    pub alpha: u64,
    /// Scalar coefficient for Q.
    pub beta: u64,
}

/// Precomputed table of `R` random addend points.
///
/// Built once per DLP instance; shared (read-only) across all walk states.
#[derive(Clone, Debug)]
pub struct AddendTable<F: Fp> {
    /// The `R` addend entries.
    pub entries: Vec<Addend<F>>,
}

impl<F: Fp> AddendTable<F> {
    /// Build the addend table for the walk.
    ///
    /// Randomly samples `R` scalar pairs `(αᵢ, βᵢ)` from `[1, n)` and computes
    /// `R[i] = αᵢ·G + βᵢ·Q` for each.  The non-zero constraint on `βᵢ` ensures
    /// the table entries carry useful `Q`-weight.
    ///
    /// # Arguments
    ///
    /// * `curve` — the curve definition.
    /// * `g` — base point G in affine form.
    /// * `q` — target point Q in affine form.
    /// * `n` — prime group order.
    /// * `rng` — a cryptographic-quality RNG.
    pub fn new<R: RngCore>(
        curve: &Curve,
        g: &AffinePoint<F>,
        q: &AffinePoint<F>,
        n: u64,
        rng: &mut R,
    ) -> Self {
        let mut entries = Vec::with_capacity(R);

        for _ in 0..R {
            let alpha = random_nonzero_scalar(n, rng);
            let beta  = random_nonzero_scalar(n, rng);

            // R[i] = alpha*G + beta*Q  (scalar_mul + mixed add).
            // scalar_mul takes a Uint<4> scalar; the result is an AffinePoint<F>.
            let alpha_g = curve.scalar_mul(g, &Uint::<4>::from(alpha));
            let beta_q  = curve.scalar_mul(q, &Uint::<4>::from(beta));

            // Add alpha_g + beta_q in Jacobian, using the field prime as p.
            let alpha_gj = JacobianPoint::from_affine(&alpha_g, &curve.p);
            let pt_jac   = curve.add_mixed(&alpha_gj, &beta_q);
            let point    = pt_jac.to_affine(&curve.p);

            entries.push(Addend { point, alpha, beta });
        }

        AddendTable { entries }
    }

    /// Partition function: map an affine point's x-coordinate to table index `[0, R)`.
    ///
    /// Uses the low 64-bit word of the x-coordinate modulo `R`.  The x-coordinate
    /// is taken as a `Uint<4>` and the low limb extracted; this is consistent
    /// regardless of how the field element stores its internal representation.
    #[inline]
    pub fn partition<F2: Fp>(&self, pt: &AffinePoint<F2>) -> usize {
        match pt {
            AffinePoint::Infinity => 0,
            AffinePoint::Finite { x, .. } => {
                let x_uint = x.to_uint();
                let low_word = x_uint.as_words()[0];
                (low_word % R as u64) as usize
            }
        }
    }
}

// ── Walk state ───────────────────────────────────────────────────────────────

/// State of one r-adding walk instance.
///
/// Invariant: `point = a·G + b·Q` at all times.
#[derive(Clone, Debug)]
pub struct WalkState<F: Fp> {
    /// Current walk point in Jacobian coordinates.
    pub point_jac: JacobianPoint<F>,
    /// Scalar coefficient for G: `point = a·G + b·Q`.
    pub a: u64,
    /// Scalar coefficient for Q: `point = a·G + b·Q`.
    pub b: u64,
}

impl<F: Fp> WalkState<F> {
    /// Initialise a walk from random starting scalars `a₀, b₀`.
    ///
    /// Sets `point = a₀·G + b₀·Q` and records `(a₀, b₀)`.
    ///
    /// # Arguments
    ///
    /// * `curve` — the curve definition.
    /// * `g` — base point G.
    /// * `q` — target point Q.
    /// * `n` — prime group order.
    /// * `rng` — RNG for sampling `a₀, b₀`.
    pub fn new_random<Rng: RngCore>(
        curve: &Curve,
        g: &AffinePoint<F>,
        q: &AffinePoint<F>,
        n: u64,
        rng: &mut Rng,
    ) -> Self {
        let a0 = random_nonzero_scalar(n, rng);
        let b0 = random_nonzero_scalar(n, rng);

        let a0_g  = curve.scalar_mul(g, &Uint::<4>::from(a0));
        let b0_q  = curve.scalar_mul(q, &Uint::<4>::from(b0));
        let start = curve.add_mixed(&JacobianPoint::from_affine(&a0_g, &curve.p), &b0_q);

        WalkState { point_jac: start, a: a0, b: b0 }
    }

    /// Advance the walk by one step.
    ///
    /// Selects the addend `R[i]` based on the current point's affine x-coordinate,
    /// updates the Jacobian point via mixed addition, and updates `(a, b)` mod n.
    ///
    /// # Arguments
    ///
    /// * `curve` — the curve definition.
    /// * `table` — precomputed addend table.
    /// * `n` — prime group order (for scalar reduction).
    pub fn step(&mut self, curve: &Curve, table: &AddendTable<F>, n: u64) {
        // To partition, we need the affine x-coordinate.  Convert only the x-component
        // lazily by reading the Jacobian representation.  We need to fully convert to
        // affine to get the reduced x; this is necessary for correctness of the partition
        // function.  The cost is one inversion per step — expensive, but this is the
        // Phase 4 baseline.  Phase 7 (batched inversion) amortises this.
        let pt_affine = self.point_jac.to_affine(&curve.p);
        let idx = table.partition(&pt_affine);
        let addend = &table.entries[idx];

        // Update Jacobian point: W ← W + R[i]  (mixed Jacobian + affine)
        self.point_jac = curve.add_mixed(&self.point_jac, &addend.point);

        // Update scalar coefficients mod n
        self.a = add_mod_n(self.a, addend.alpha, n);
        self.b = add_mod_n(self.b, addend.beta, n);
    }

    /// Return the current walk point in affine coordinates.
    ///
    /// Performs one field inversion.  Use sparingly — the walk itself uses
    /// `step` which already does one inversion for partitioning.  This is
    /// a separate call for when the caller needs the affine form (e.g., Brent's
    /// tortoise/hare comparison or DP detection).
    #[inline]
    pub fn to_affine(&self, curve: &Curve) -> AffinePoint<F> {
        self.point_jac.to_affine(&curve.p)
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Sample a uniform random scalar from `[1, n)`.
///
/// Rejection-samples until the value is non-zero.
fn random_nonzero_scalar<R: RngCore>(n: u64, rng: &mut R) -> u64 {
    loop {
        // Use next_u64; reduce modulo n; reject zero.
        let v = rng.next_u64() % n;
        if v != 0 {
            return v;
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha20Rng;
    use rand::SeedableRng;
    use crate::curve::test_curves::{tiny_a, TINY_A_N};
    use crate::field::FpMonty;

    /// Check that the walk invariant `W = a·G + b·Q` holds after construction
    /// and after a sequence of steps.
    ///
    /// Uses the 20-bit test curve A for speed.
    #[test]
    fn walk_invariant_holds() {
        let curve = tiny_a();
        let g: AffinePoint<FpMonty> = curve.generator();
        let n = TINY_A_N;

        // Use k_target = 42 as the DLP: Q = 42·G
        let k_target: u64 = 42;
        let q = curve.scalar_mul(&g, &Uint::<4>::from(k_target));

        let mut rng = ChaCha20Rng::seed_from_u64(0xdeadbeef);
        let table = AddendTable::new(&curve, &g, &q, n, &mut rng);
        let mut walk = WalkState::<FpMonty>::new_random(&curve, &g, &q, n, &mut rng);

        for _ in 0..10 {
            // Reconstruct `a·G + b·Q` from current scalars.
            let ag  = curve.scalar_mul(&g, &Uint::<4>::from(walk.a));
            let bq  = curve.scalar_mul(&q, &Uint::<4>::from(walk.b));
            let reconstructed = curve.add_mixed(
                &JacobianPoint::from_affine(&ag, &curve.p),
                &bq,
            ).to_affine(&curve.p);
            let actual = walk.to_affine(&curve);
            assert_eq!(
                actual, reconstructed,
                "walk invariant broken at step"
            );
            walk.step(&curve, &table, n);
        }
    }

    /// Partition function returns indices in [0, R).
    ///
    /// Uses the 20-bit test curve A for speed.
    #[test]
    fn partition_in_range() {
        let curve = tiny_a();
        let g: AffinePoint<FpMonty> = curve.generator();
        let n = TINY_A_N;
        let q = curve.scalar_mul(&g, &Uint::<4>::from(7u64));

        let mut rng = ChaCha20Rng::seed_from_u64(1);
        let table = AddendTable::<FpMonty>::new(&curve, &g, &q, n, &mut rng);
        let mut walk = WalkState::<FpMonty>::new_random(&curve, &g, &q, n, &mut rng);

        for _ in 0..20 {
            let pt = walk.to_affine(&curve);
            let idx = table.partition(&pt);
            assert!(idx < R, "partition index {idx} out of range [0, {R})");
            walk.step(&curve, &table, n);
        }
    }
}
