use clap::Parser;
use patharg::OutputArg;
use std::io::Write;
use rand::prelude::*;
use rand_distr::StandardNormal;

/// Generate random numbers
///
/// Generates random samples (numbers) and outputs them in text form, one number per line.
#[derive(Parser)]
struct Arguments {
    /// File to write the output to
    #[arg(short = 'o', long, default_value_t)]
    outfile: OutputArg,

    /// Number of samples to generate
    #[arg()]
    nsamples: u64,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let mut outfile = args.outfile.create()?;
    let dist = StandardNormal;
    for _ in 0..args.nsamples {
        let val: f64 = thread_rng().sample(dist);
        writeln!(&mut outfile,"{}",val)?;
    }
    Ok(())
}
