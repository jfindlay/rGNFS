//! Pollard rho: integer factorization and ECDLP with all canonical optimizations.
//!
//! # Structure
//!
//! - [`field`] — `Fp` trait, `FpNaive` (schoolbook), `FpMonty` (Montgomery form).
//! - [`curve`] — elliptic curve group law, affine/Jacobian points, two concrete curves.
//! - [`binary_curve`] — binary curve `y²+xy=x³+ax²+b` group law in López–Dahab
//!   projective coordinates; point decompression via the half-trace.
//! - [`binary_ecdlp`] — Pollard-rho ECDLP solver over binary curves (`F2m`+`BinaryCurve`):
//!   r-adding walk, distinguished-point collision, linear recovery of `k`.
//! - [`hyperelliptic`] — hyperelliptic curve `y²+h(x)y=f(x)` over GF(2^m), genus,
//!   Mumford divisor `[u,v]` representation, validity predicate, divisor-from-points.
//! - [`util`] — shared helpers: batched inversion, multi-precision utilities.
//! - [`factor`] — integer factorization rho (Floyd → Brent → batched GCD → multi-c).
//! - [`ecdlp`] — ECDLP rho (r-adding walk → DPs → negmap → batched inv → GLV).
//! - [`pairing`] — `F_{p^k}` extension-field arithmetic and bilinear pairings (E.B).
//! - [`ssa`] — Smart–Satoh–Araki p-adic attack on anomalous curves (E.E).
//! - [`ghs`] — GHS Weil-descent attack on binary elliptic curves (E.H): Artin–Schreier
//!   extension, Weil restriction of scalars, hyperelliptic curve extraction.

pub mod binary_curve;
pub mod binary_ecdlp;
pub mod curve;
pub mod ecdlp;
pub mod factor;
pub mod field;
pub mod ghs;
pub mod hyperelliptic;
pub mod pairing;
pub mod ssa;
pub mod util;
