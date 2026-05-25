//! CLI binary `rho-factor`.
//!
//! Usage: `rho-factor <N> [--threads T] [--batch-size B]`
//!
//! Factors a composite integer N using parallel Pollard rho (Brent + batched GCD).
//! Prints the smaller factor and its cofactor on success.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rho-factor", about = "Pollard rho integer factorization")]
struct Args {
    /// Composite integer to factor (decimal, up to 128 bits).
    n: u128,

    /// Number of c-values to try in parallel (defaults to logical CPU count).
    #[arg(long, default_value_t = 0)]
    threads: u64,

    /// GCD batch size for Montgomery's batched-GCD trick (default: 128).
    #[arg(long, default_value_t = 128)]
    batch_size: usize,
}

fn main() {
    let args = Args::parse();
    let n = args.n;

    if n <= 1 {
        eprintln!("error: N must be > 1");
        std::process::exit(1);
    }
    if n == 2 || n == 3 {
        println!("{n} is prime");
        return;
    }

    // Default threads: number of available logical CPUs.
    let max_c = if args.threads == 0 {
        rayon::current_num_threads() as u64
    } else {
        args.threads
    };

    match rho::factor::factor(n, max_c, args.batch_size) {
        Some(p) => {
            let q = n / p;
            let (lo, hi) = if p <= q { (p, q) } else { (q, p) };
            println!("{n} = {lo} × {hi}");
        }
        None => {
            eprintln!("rho-factor: failed to find a factor of {n} with {max_c} c-values");
            std::process::exit(1);
        }
    }
}
