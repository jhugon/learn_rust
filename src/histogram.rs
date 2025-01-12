use std::iter::zip;
use crate::plotutils::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Histogram {
    nbins: usize,
    bincontent: Vec<usize>,
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
        let binedges = (0..nbins+1)
                        .map(|ibin| xmin + xbinwidth*ibin as f32)
                        .collect();
        let bincontent = vec![0;nbins];
        Self{nbins,bincontent,binedges}
    }
    pub fn fill(& mut self, x: f32) -> () {
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
    pub fn print(&self) -> () {
        let leftmargin: usize = 12;
        let botmargin: usize = 3;
        let termwidth = usize::from(termsize::get().unwrap().cols);
        let termheight = usize::from(termsize::get().unwrap().rows);

        let axes = AxesMeta {
            dataminmax: DataMinMax::fromhistogram(&self.binedges,&self.bincontent),
            termwidth: termwidth,
            termheight: termheight,
            leftmargin: leftmargin,
            botmargin: botmargin,
        };

        let yaxistext = drawyaxis(&axes);
        let plotteddatatext = self.drawdata(&axes);
        let xaxistext = drawxaxis(&axes);
        let resultexceptxaxis: Vec<String> = zip(yaxistext,plotteddatatext)
                    .map(
                        |(t_ax,t_data)| format!("{t_ax}{t_data}")
                        )
                    .collect();
        let result = resultexceptxaxis.iter().chain(&xaxistext);
        for line in result {
            assert!(line.graphemes(true).count() == termwidth);
            println!("{}",line);
        }
    }
    fn drawdata(&self,axes: &AxesMeta) -> Vec<String> {
        let maxbincontent = *self.bincontent.iter().max().unwrap();
        let histwidth = axes.axeswidth();
        let scalefactor: f32 = if maxbincontent > histwidth {
            histwidth as f32 / maxbincontent as f32
        } else {
            1.
        };

        let counts = self.bincontent.iter();
        let scaledcount = counts
                            .map(|count| (*count as f32 * scalefactor) as usize);
        let bars = scaledcount
                        .map(|count| std::iter::repeat("█")
                        .take(count)
                        .collect());
        bars.collect()
    }
}
