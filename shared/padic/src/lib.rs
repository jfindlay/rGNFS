//! p-adic arithmetic substrate: Z/p^k ring arithmetic, p-adic valuation, unit inversion, and
//! the formal-group logarithm.
//!
//! This crate provides the prime-power-modulus arithmetic type the p-adic sub-track (E.D) stands
//! on. The central type is [`zp::Zp`] — an element of Z/p^k carrying a `BigInt` residue, an
//! explicit precision `k`, and the p-adic prime `p` stored on the element for ergonomics in a
//! precision tower.
//!
//! # Design
//!
//! Z/p^k is **not** a field for k > 1: it has zero divisors, and only elements coprime to p
//! (those with p-adic valuation 0) are invertible. The [`zp::Zp::inv`] method enforces this
//! guard — inversion of a non-unit (v_p > 0) returns an error rather than a silently wrong value.
//! This is the load-bearing non-field guard that distinguishes this type from `shared/field`'s
//! `Fp<L>` (which uses Fermat's little theorem and assumes a prime modulus).
//!
//! The [`log::padic_log`] function computes the formal-group logarithm series, convergent for
//! `v_p(z − 1) ≥ 1` (the kernel of reduction). The convergence guard mirrors the unit-inversion
//! guard: both defend against silent wrong answers.
//!
//! # Scope (principle-4 boundary)
//!
//! Toy precision only: `k` is small (demonstration fidelity). Crypto-scale precision towers,
//! Q_p (field of fractions), and the elliptic-curve formal-group parametrisation are out of scope
//! for this crate (E.E's concern).

pub mod hensel;
pub mod log;
pub mod zp;

pub use hensel::{HenselError, hensel_lift};
pub use log::{PadicLogError, padic_log};
pub use zp::{Zp, ZpError};
