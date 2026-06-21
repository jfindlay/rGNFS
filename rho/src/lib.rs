//! Pollard rho: integer factorization and ECDLP with all canonical optimizations.
//!
//! # Structure
//!
//! - [`curve`] — elliptic curve group law, affine/Jacobian points, two concrete curves.
//! - [`binary_curve`] — binary curve `y²+xy=x³+ax²+b` group law in López–Dahab
//!   projective coordinates; point decompression via the half-trace.
//! - [`binary_ecdlp`] — Pollard-rho ECDLP solver over binary curves (`F2m`+`BinaryCurve`):
//!   r-adding walk, distinguished-point collision, linear recovery of `k`.
//! - [`hyperelliptic`] — hyperelliptic curve `y²+h(x)y=f(x)` over GF(2^m), genus,
//!   Mumford divisor `[u,v]` representation, validity predicate, divisor-from-points.
//! - [`factor`] — integer factorization rho (Floyd → Brent → batched GCD → multi-c).
//! - [`ecdlp`] — ECDLP rho (r-adding walk → DPs → negmap → batched inv → GLV).
//! - [`pairing`] — `F_{p^k}` extension-field arithmetic and bilinear pairings (E.B).
//! - [`ssa`] — Smart–Satoh–Araki p-adic attack on anomalous curves (E.E).
//! - [`ghs`] — GHS Weil-descent attack on binary elliptic curves (E.H): Artin–Schreier
//!   extension, Weil restriction of scalars, hyperelliptic curve extraction.
//! - [`semaev`] — Semaev summation polynomials over a prime-field Weierstrass curve (E.J):
//!   `F_p[x]` univariate resultant, multivariate/symmetric-polynomial type `S_m`.
//! - [`index_calculus`] — Gaudry–Diem–Joux–Vitse index-calculus ECDLP solver over
//!   `E(F_p)` (E.K): factor base, prime-order subgroup, relation/matrix contract.

pub mod binary_curve;
pub mod binary_ecdlp;
pub mod curve;
pub mod ecdlp;
pub mod factor;
pub mod ghs;
pub mod hyperelliptic;
pub mod index_calculus;
pub mod pairing;
pub mod semaev;
pub mod ssa;
