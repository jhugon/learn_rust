use clap::Parser;
use patharg::InputArg;
use std::error::Error;
use anyhow::anyhow;
use learn_rust::plot;
use regex::Regex;

/// Plots data from input in form x y
///
#[derive(Parser)]
struct Arguments {
    #[arg(default_value_t)]
    infile: InputArg,
}

fn xyparseline(line: &str) -> Result<(f32,f32), Box<dyn Error>> {
    let re = Regex::new(r"^\s*(-??\d+(?:\.\d*)?)\s+(-??\d+(?:\.\d*)?)\s*$")?;
    let captureopt = re.captures(line);
    let (xnum, ynum) = if let Some(capture) = captureopt {
        let (_, [xstr,ystr]) = capture.extract();
        (xstr.parse()?, ystr.parse()?)
    } else {
        return Err(anyhow!("Couldn't parse line \"{}\", it should be two numbers seperated by whitespace",line).into());
    };
    println!("\"{}\" parsed to {} and {}",line,xnum,ynum);
    Ok((xnum,ynum))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    let infile = args.infile;
    for line in infile.lines()? {
        let (_x,_y) = xyparseline(&line?)?;
    }
    let xsys: Vec<(f32,f32)> = infile.lines()?.map(|line| xyparseline(&line?)).collect::<Result<Vec<(f32,f32)>,Box<dyn Error>>>()?;
    let xs: Vec<f32>  = xsys.iter().map(|(x,_)| *x).collect();
    let ys: Vec<f32> = xsys.iter().map(|(_,y)| *y).collect();
    println!("{:?}",xsys);
    println!("{:?}",xs);
    println!("{:?}",ys);
    plot(&xs,&ys)?;
    Ok(())
}
