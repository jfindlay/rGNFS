//! Elliptic curve group law over GF(p).
//!
//! Phase 3 will fill in `EcPoint` (affine + Jacobian), the group law
//! (add, double, scalar-mult), and two concrete curves.

pub mod generic;
pub mod secp_k1_toy;
