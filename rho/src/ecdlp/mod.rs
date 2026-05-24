//! ECDLP solver via Pollard rho.
//!
//! Optimization layers (per the plan):
//! 3. r-adding walk (Teske, r≈20) — baseline.
//! 4. Distinguished points + parallel collision search (van Oorschot–Wiener).
//! 5. Negation map + fruitless-cycle escape (BKNS).
//! 6. Batched field inversion + affine coordinates.
//! 7. GLV endomorphism (order-3, secp-toy curve only).

pub mod coordinator;
pub mod dp;
pub mod glv;
pub mod negmap;
pub mod walk;
