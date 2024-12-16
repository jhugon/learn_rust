use clap::Parser;
use patharg::InputArg;

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

#[derive(Debug)]
struct Histogram {
    nbins: usize,
    bincontent: Vec<u64>,
    binedges: Vec<f32>,
}

/// A 1D Histogram
///
/// Bin edges include lower bound but not upper bound of each bin
impl Histogram {
    //pub fn new_arb_bins(binedges: Vec<f32>) -> Self {
    //    let nbins = binedges.len() - 1;
    //    let bincontent = vec![0;nbins];
    //    Self{nbins,bincontent,binedges}
    //}
    pub fn new(nbins: usize,xmin: f32, xmax: f32) -> Self {
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
        let leftmargin: usize = 12;
        let termwidth = usize::from(termsize::get().unwrap().cols);
        let histwidth = termwidth - leftmargin;
        let maxbincontent_u64ref: &u64 = self.bincontent.iter().max().unwrap();
        let maxbincontent: usize = (*maxbincontent_u64ref).try_into().unwrap();
        let bcscalefactor: f32 = if maxbincontent > histwidth {
            histwidth as f32 / maxbincontent as f32
        } else {
            1.
        };
        for i in 0..self.nbins {
            let count: usize = self.bincontent[i].try_into().unwrap();
            let scaledcount = (count as f32 * bcscalefactor) as usize;
            let bar: String = std::iter::repeat("X").take(scaledcount).collect();
            println!("{:>10} |{}",self.binedges[i],bar);
        }
        println!("{:>10}",self.binedges[self.nbins]);
        println!("Each X is {} entries; right edge is {} entries",1./bcscalefactor,(histwidth as f32) * bcscalefactor)
    }
}
