use clap::Parser;
use clap::ValueEnum;
use patharg::OutputArg;
use rand::prelude::*;
use std::io::Write;

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

    /// Type of distribution
    #[arg(value_enum)]
    distribution: Distribution,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Distribution {
    /// Standard normal distribution
    Normal,
    /// Gaussian (standard normal) distribution
    Gaussian,
    /// Uniform distribution on [0,1)
    Uniform,
    /// Exponential distribution with scale parameter 1
    Exponential,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let mut outfile = args.outfile.create()?;

    // Statistical distributions
    let normal = rand_distr::StandardNormal;
    let uniform = rand_distr::Uniform::new(0., 1.);
    let exponential = rand_distr::Exp1;

    for _ in 0..args.nsamples {
        let val: f64 = match args.distribution {
            Distribution::Normal => thread_rng().sample(normal),
            Distribution::Gaussian => thread_rng().sample(normal),
            Distribution::Uniform => thread_rng().sample(uniform),
            Distribution::Exponential => thread_rng().sample(exponential),
        };
        writeln!(&mut outfile, "{}", val)?;
    }
    Ok(())
}
