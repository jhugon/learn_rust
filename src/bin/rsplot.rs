use clap::Parser;
use patharg::InputArg;
use regex::Regex;
use std::error::Error;

/// Plots data from input in form x y
///
#[derive(Parser)]
struct Arguments {
    #[arg(default_value_t)]
    infile: InputArg,
}

fn parsexyinput(infile: InputArg) -> Result<(Vec<f32>, Vec<f32>)> {
    let re = Regex::new(r"^\s*(-??\d+(?:\.\d*))\s+(-??\d+(?:\.\d*))\s*$").unwrap();
    for lineiter in infile.lines()? {
        let line: String = lineiter?;
        println!("{}",&line);
        let capture = re.captures(&line);
        let xnum: f32 = if let Some(xmatch) = capture.get(1) {
            xmatch.as_str().parse()
        } else {
            return Error(format!("Couldn't parse x and y in {}",line));
        };
    }
    Ok((vec!(),vec!()))
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let (xs, ys) = parsexyinput(args.infile)?;
    learn_rust::plot(xs,ys);
    Ok(())
}
