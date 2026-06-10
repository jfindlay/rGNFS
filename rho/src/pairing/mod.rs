//! Pairing arithmetic over extension fields `F_{p^k}`.
//!
//! This module builds the bilinear-pairing substrate (E.B sub-track) on top of
//! the frozen `shared::field` prime-field arithmetic.  It introduces no changes
//! to `rho::curve` or `shared::field` — it composes them (option-B decision).
//!
//! # Module tree
//!
//! - [`fpext`] — `F_{p^k}` extension-field arithmetic (E.B.1).
//!   Freezes contract C-FpExt.
//! - [`ecext`] — `E(F_{p^k})` point arithmetic (E.B.2).
//!   Freezes contract C-PairingCurve (together with `test_curves`).
//! - [`test_curves`] — pairing-friendly toy fixture: embedding degree `k = 2`,
//!   torsion prime `ℓ = 3`, base-field point `P` and extension-field point `Q`
//!   (E.B.2).  Freezes contract C-PairingCurve.
//! - [`miller`] — Miller's algorithm + line/vertical functions (E.B.3).
//! - [`weil`] — Weil pairing (E.B.3).
//! - `tate` — Tate/reduced-Tate pairing + final exponentiation (E.B.4).

pub mod ecext;
pub mod fpext;
pub mod miller;
pub mod test_curves;
pub mod weil;
