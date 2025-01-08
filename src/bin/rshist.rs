use clap::Parser;
use patharg::InputArg;
use learn_rust::Histogram;

/// Make a histogram from input data
///
/// Makes a histogram from input data, in text form one number per line.
///
/// Fills a histogram with NBINS evenly spaced between XMIN and XMAX
#[derive(Parser)]
struct Arguments {
    /// Number of evenly spaced histogram bins
    #[arg(required=true)]
    nbins: u32,
    /// Histogram minimum
    #[arg(required=true)]
    min: f32,
    /// Histogram maximum
    #[arg(required=true)]
    max: f32,
    /// File to read input from
    #[arg(default_value_t)]
    infile: InputArg,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let mut hist = Histogram::new(args.nbins.try_into().unwrap(),args.min,args.max);
    for line in args.infile.lines()? {
        let s = line?;
        let val: f32 = s.parse().expect("Couldn't parse float");
        hist.fill(val);
    }
    println!("{:?}",hist);
    hist.print();
    Ok(())
}

