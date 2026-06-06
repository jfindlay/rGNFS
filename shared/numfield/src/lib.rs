//! Number-field substrate: ℚ(α), element arithmetic, norm via resultant, and ideal arithmetic.
//!
//! This crate provides:
//! - [`IntPoly`] — polynomials over ℤ with `BigInt` coefficients.
//! - [`RatPoly`] — polynomials over ℚ with `BigRational` coefficients.
//! - [`NumberField`] — a number field K = ℚ(α) defined by a monic irreducible f ∈ ℤ[x].
//! - [`NumberFieldElement`] — an element of K, represented as a reduced polynomial in α.
//! - [`Ideal`] — a fractional ideal of ℤ[α] in two-element primary representation `(p, α − r)`.
//!
//! # Design
//!
//! Coefficient arithmetic delegates to `num_bigint::BigInt` and `num_rational::BigRational`
//! (correctness-oracle dependencies, analogous to CADO-NFS for the sieve). The number-field
//! abstraction and all algorithms above it are first-party.
//!
//! # Invariants
//!
//! - Polynomial trailing zeros are always trimmed.
//! - `NumberFieldElement::poly` always has degree strictly less than `field.degree()`.
//!   Multiplication eagerly reduces mod f to maintain this invariant.
//! - `Ideal::p` is always positive.

pub mod element;
pub mod ideal;
pub mod poly;
pub mod resultant;

pub use element::{NumberField, NumberFieldElement};
pub use ideal::Ideal;
pub use poly::{IntPoly, RatPoly};
pub use resultant::{resultant, subresultant_gcd};
