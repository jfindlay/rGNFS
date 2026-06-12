//! Pollard rho: integer factorization and ECDLP with all canonical optimizations.
//!
//! # Structure
//!
//! - [`field`] — `Fp` trait, `FpNaive` (schoolbook), `FpMonty` (Montgomery form).
//! - [`curve`] — elliptic curve group law, affine/Jacobian points, two concrete curves.
//! - [`util`] — shared helpers: batched inversion, multi-precision utilities.
//! - [`factor`] — integer factorization rho (Floyd → Brent → batched GCD → multi-c).
//! - [`ecdlp`] — ECDLP rho (r-adding walk → DPs → negmap → batched inv → GLV).
//! - [`pairing`] — `F_{p^k}` extension-field arithmetic and bilinear pairings (E.B).
//! - [`ssa`] — Smart–Satoh–Araki p-adic attack on anomalous curves (E.E).

pub mod curve;
pub mod ecdlp;
pub mod factor;
pub mod field;
pub mod pairing;
pub mod ssa;
pub mod util;
