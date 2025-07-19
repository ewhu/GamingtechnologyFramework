// src/main.rs
/*
 * Main executable for GamingtechnologyFramework
 */

use clap::Parser;
use gamingtechnologyframework::{Result, run};

#[derive(Parser)]
#[command(version, about = GamingtechnologyFramework - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
