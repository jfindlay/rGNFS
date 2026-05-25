//! CLI binary `rho-dlog`.
//!
//! Usage: `rho-dlog --curve <NAME> --p <x,y> --q <x,y>`
//!
//! Solves the ECDLP `Q = k·P` on one of the built-in curves and prints `k`.
//!
//! # Supported curves
//!
//! - `secp-toy` — downsized GLV-friendly secp256k1-style curve (63-bit order).
//! - `generic`  — generic 63-bit Weierstrass curve `y² = x³ − 3x + 1 mod p`.
//!
//! # Point format
//!
//! Pass each point as `x,y` with no spaces (decimal integers):
//!
//! ```text
//! rho-dlog --curve secp-toy --p 2,3236101131256320111 --q 922337203685479039,132612412593110192
//! ```
//!
//! Output: `k = <decimal>` followed by a verification line.
//!
//! # Parallel mode
//!
//! Pass `--walkers N` (N ≥ 1) to use the distinguished-point parallel solver
//! ([`rho::ecdlp::solve_dp`]) instead of the single-threaded Brent solver.
//! `--theta T` controls the DP threshold (default: 10).

use clap::Parser;
use crypto_bigint::Uint;
use rho::curve::AffinePoint;
use rho::curve::generic::generic_curve;
use rho::curve::secp_k1_toy::{secp_k1_toy, N as SECP_N};
use rho::ecdlp::{solve_brent, solve_dp};
use rho::field::{Fp, FpMonty};

#[derive(Parser, Debug)]
#[command(name = "rho-dlog", about = "Pollard rho ECDLP solver")]
struct Args {
    /// Curve name: `secp-toy` or `generic`.
    #[arg(long, default_value = "secp-toy")]
    curve: String,

    /// Base point P as `x,y` (decimal, no spaces).  Defaults to the curve generator.
    #[arg(long)]
    p: Option<String>,

    /// Target point Q as `x,y` (decimal, no spaces).  Required.
    #[arg(long)]
    q: String,

    /// RNG seed for reproducibility (default: 0).
    #[arg(long, default_value_t = 0u64)]
    seed: u64,

    /// Maximum retry attempts on degenerate failures (default: 20).
    /// Only used by the single-threaded Brent solver.
    #[arg(long, default_value_t = 20usize)]
    retries: usize,

    /// Number of parallel walker threads for the DP solver (default: 0 = use Brent).
    ///
    /// When > 0, the distinguished-point parallel solver is used instead of Brent.
    #[arg(long, default_value_t = 0usize)]
    walkers: usize,

    /// Distinguished-point threshold: number of low-order zero bits required
    /// in the x-coordinate.  Only used when `--walkers > 0` (default: 10).
    #[arg(long, default_value_t = 10u32)]
    theta: u32,
}

/// Parse a `"x,y"` string into two `u64` values.
fn parse_point(s: &str) -> Result<(u64, u64), String> {
    let parts: Vec<&str> = s.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(format!("expected 'x,y', got: {s}"));
    }
    let x = parts[0].trim().parse::<u64>()
        .map_err(|e| format!("bad x in '{s}': {e}"))?;
    let y = parts[1].trim().parse::<u64>()
        .map_err(|e| format!("bad y in '{s}': {e}"))?;
    Ok((x, y))
}

fn main() {
    let args = Args::parse();

    // Select curve and group order.
    let (curve, n) = match args.curve.as_str() {
        "secp-toy" => (secp_k1_toy(), SECP_N),
        "generic" => {
            // The generic curve does not embed its group order.
            // Use the known order for `y² = x³ − 3x + 1 mod 2^63−25`.
            // Computed by Sage: E.order() — stored here as a constant.
            const GENERIC_N: u64 = 9_223_372_037_218_517_353;
            (generic_curve(), GENERIC_N)
        }
        other => {
            eprintln!("rho-dlog: unknown curve '{other}'; use 'secp-toy' or 'generic'");
            std::process::exit(1);
        }
    };

    // Parse base point P (or use generator).
    let g: AffinePoint<FpMonty> = match &args.p {
        None => curve.generator(),
        Some(s) => {
            let (x, y) = parse_point(s).unwrap_or_else(|e| {
                eprintln!("rho-dlog: --p parse error: {e}");
                std::process::exit(1);
            });
            AffinePoint::new(
                FpMonty::from_uint(Uint::<4>::from(x), &curve.p),
                FpMonty::from_uint(Uint::<4>::from(y), &curve.p),
            )
        }
    };

    // Parse target point Q.
    let (qx, qy) = parse_point(&args.q).unwrap_or_else(|e| {
        eprintln!("rho-dlog: --q parse error: {e}");
        std::process::exit(1);
    });
    let q: AffinePoint<FpMonty> = AffinePoint::new(
        FpMonty::from_uint(Uint::<4>::from(qx), &curve.p),
        FpMonty::from_uint(Uint::<4>::from(qy), &curve.p),
    );

    // Sanity checks.
    if !curve.is_on_curve(&g) {
        eprintln!("rho-dlog: base point P is not on the curve");
        std::process::exit(1);
    }
    if !curve.is_on_curve(&q) {
        eprintln!("rho-dlog: target point Q is not on the curve");
        std::process::exit(1);
    }

    // Solve: use parallel DP solver when --walkers > 0, otherwise Brent.
    let result = if args.walkers > 0 {
        solve_dp(&curve, &g, &q, n, args.walkers, args.theta, args.seed)
    } else {
        solve_brent(&curve, &g, &q, n, args.seed, args.retries)
    };

    match result {
        Some(k) => {
            println!("k = {k}");
            // Verify by computing k·G and comparing to Q.
            let check = curve.scalar_mul(&g, &Uint::<4>::from(k));
            if check == q {
                println!("verified: k·G = Q  ✓");
            } else {
                eprintln!("rho-dlog: WARNING: k·G ≠ Q (solver bug?)");
                std::process::exit(1);
            }
        }
        None => {
            if args.walkers > 0 {
                eprintln!("rho-dlog: failed to solve DLP (parallel DP solver gave up)");
            } else {
                eprintln!("rho-dlog: failed to solve DLP after {} retries", args.retries);
            }
            std::process::exit(1);
        }
    }
}
