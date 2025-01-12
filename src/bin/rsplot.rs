use clap::Parser;
use learn_rust::plot;
use patharg::InputArg;
use std::error::Error;
//use regex::Regex;

/// Plots data from input in form x y
///
#[derive(Parser)]
struct Arguments {
    #[arg(default_value_t)]
    infile: InputArg,
}

//fn xyinputparse(infile: InputArg) -> Result<(Vec<f32>, Vec<f32>)> {
//    let re = Regex::new(r"^\s*(-??\d+(?:\.\d*))\s+(-??\d+(?:\.\d*))\s*$").unwrap();
//    for lineiter in infile.lines()? {
//        let line: String = lineiter?;
//        println!("{}",&line);
//        let capture = re.captures(&line);
//        let xnum: f32 = if let Some(xmatch) = capture.get(1) {
//            xmatch.as_str().parse()
//        } else {
//            return Error(format!("Couldn't parse x and y in {}",line));
//        };
//    }
//    Ok((vec!(),vec!()))
//}

fn main() -> Result<(), Box<dyn Error>> {
    let _args = Arguments::parse();
    //let (xs, ys) = xyinputparse(args.infile)?;
    let xs = vec![0., 1., 2., 2., 7., 4., -2.];
    let ys = vec![5., 8., 1., 6., 0., -4., 10.];
    plot(&xs, &ys)?;
    Ok(())
}
