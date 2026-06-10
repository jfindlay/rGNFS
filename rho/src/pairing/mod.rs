//! Pairing arithmetic over extension fields `F_{p^k}`.
//!
//! This module builds the bilinear-pairing substrate (E.B sub-track) on top of
//! the frozen `shared::field` prime-field arithmetic.  It introduces no changes
//! to `rho::curve` or `shared::field` — it composes them (option-B decision).
//!
//! # Track-E pairing design statement
//!
//! **Principle 1 — genuine bilinear pairing.**  The Weil and Tate pairings are
//! implemented head-on via Miller's algorithm and the Weil ratio / Tate final
//! exponentiation.  No stubbed map, no shortcut.  Bilinearity `e(aP, bQ) =
//! e(P, Q)^{ab}` is the primary correctness signal and is KAT-verified.
//!
//! **Principle 3 — no engineering optimisation.**  The extension field `F_{p^k}`
//! is a schoolbook polynomial quotient `F_p[u]/(m(u))` (direct degree-`k`
//! quotient, not a tower).  No optimal-ate pairing, no fast-tower path, no
//! Montgomery tricks beyond what the base `Fp` already provides.
//!
//! **Principle 4 — toy embedding degree.**  The fixture uses `k = 2` (the
//! minimal embedding degree for the chosen torsion prime `ℓ = 3` over `F_47`).
//! The crypto-scale `F_{p^{12}}` gap (BN/BLS curves) is a demonstration-scale
//! boundary, not a mathematical one: the same Miller + final-exp algorithm
//! applies at `k = 12`; the schoolbook `F_{p^k}` arithmetic would simply be
//! slower.  No `k = 12` tower is wired — it is a principle-4 annotation.
//!
//! **E.C-readiness check (C-Pairing contract).**  [`weil::weil_pairing`] and
//! [`tate::reduced_tate`] both return [`fpext::FpExt`] values — elements of
//! `F_{p^k}` represented as coefficient vectors over `F_p`.  E.C's MOV bridge
//! reads these as `F_{p^k}` elements (via `to_uint_vec`-style canonicalisation)
//! and maps them toward `solve_dl`.  The [`fpext::FpExt::frobenius`] map is
//! over-specified here (E.B's own pairings barely use it) because E.C's MOV
//! reduction needs it centrally.
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
//! - [`weil`] — Weil pairing: two Miller calls, no final exponentiation (E.B.3).
//! - [`tate`] — Tate/reduced-Tate pairing: one Miller call + final
//!   exponentiation `^{(p^k−1)/ℓ}` (E.B.4).  Freezes contract C-Pairing.

pub mod ecext;
pub mod fpext;
pub mod miller;
pub mod mov;
pub mod tate;
pub mod test_curves;
pub mod weil;
