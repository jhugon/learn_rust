use clap::Parser;
use patharg::InputArg;

/// Make a histogram from input data
///
/// Makes a histogram from input data, in text form one number per line.
#[derive(Parser)]
struct Arguments {
    /// File to write the output to
    #[arg(required=true,default_value_t)]
    infile: InputArg,
}

fn main() -> std::io::Result<()> {
    //let mut hist = Histogram::new(vec![0.,0.25,0.5,0.75,1.]);
    let mut hist = Histogram::new_even_bins(20,0.,1.);

    let args = Arguments::parse();
    for line in args.infile.lines()? {
        let s = line?;
        let val: f32 = s.parse().expect("Couldn't parse float");
        hist.fill(val);
    }
    println!("{:?}",hist);
    hist.print();
    Ok(())
}

#[derive(Debug)]
struct Histogram {
    nbins: usize,
    bincontent: Vec<u64>,
    binedges: Vec<f32>,
}

/// A 1D Histogram
///
/// Bin edges include lower bound but not upper bound
impl Histogram {
    //pub fn new_arb_bins(binedges: Vec<f32>) -> Self {
    //    let nbins = binedges.len() - 1;
    //    let bincontent = vec![0;nbins];
    //    Self{nbins,bincontent,binedges}
    //}
    pub fn new_even_bins(nbins: usize,xmin: f32, xmax: f32) -> Self {
        let xwidth = xmax - xmin;
        let xbinwidth: f32 = xwidth/nbins as f32;
        let binedges = (0..nbins+1).map(|ibin| xmin + xbinwidth*ibin as f32).collect();
        let bincontent = vec![0;nbins];
        Self{nbins,bincontent,binedges}
    }
    fn fill(& mut self, x: f32) -> () {
        if x < self.binedges[0] {
            return
        }
        for i in 0..self.nbins {
            if x < self.binedges[i+1] {
                self.bincontent[i] += 1;
                return
            }
        }
    }
    fn print(&self) -> () {
        for i in 0..self.nbins {
            let count: usize = self.bincontent[i].try_into().unwrap();
            let bar: String = std::iter::repeat("X").take(count).collect();
            println!("{:>10} |{}",self.binedges[i],bar);
        }
        println!("{:>10}",self.binedges[self.nbins]);
    }
}
