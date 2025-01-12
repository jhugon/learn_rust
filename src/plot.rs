use std::iter::zip;
use std::error::Error;
use crate::plotutils::*;
use unicode_segmentation::UnicodeSegmentation;

/// Plots in the terminal
///
/// Removes all xs and ys that contain nan or infinity
pub fn plot(xs: &[f32], ys: &[f32]) -> Result<(), Box<dyn Error>> {
    let leftmargin: usize = 12;
    let botmargin: usize = 3;
    let termwidth = usize::from(termsize::get().unwrap().cols);
    let termheight = usize::from(termsize::get().unwrap().rows);

    let xsys = validate_input(xs,ys);
    let axes = AxesMeta {
        dataminmax: DataMinMax::find(&xsys),
        termwidth: termwidth,
        termheight: termheight,
        leftmargin: leftmargin,
        botmargin: botmargin,
    };

    let yaxistext = drawyaxis(&axes);
    let plotteddatatext = drawdata(&xsys,&axes);
    let xaxistext = drawxaxis(&axes);
    let resultexceptxaxis: Vec<String> = zip(yaxistext,plotteddatatext).map(|(t_ax,t_data)| format!("{t_ax}{t_data}")).collect();
    let result = resultexceptxaxis.iter().chain(&xaxistext);
    for line in result {
        assert!(line.graphemes(true).count() == termwidth);
        println!("{}",line);
    }
    Ok(())
}

fn validate_input(xs: &[f32], ys: &[f32]) -> Vec<(f32,f32)> {
    let mut xsys_buffer: Vec<(f32,f32)> = zip(xs,ys)
                                .filter(|(x,y)| x.is_finite() && y.is_finite())
                                .map(|(x,y)| (*x,*y))
                                .collect();
    xsys_buffer.sort_by(|(x1,_),(x2,_)| x1.partial_cmp(x2).unwrap());
    xsys_buffer
}

//////////////////////////////////////////////////////////////

fn drawdata(xsys: &[(f32,f32)], axes: &AxesMeta) -> Vec<String> {
    let mut result = vec!();

    let data_ybins: Vec<Vec<usize>> = { // put mut var in its own scope to keep it here
        let mut data_mut = vec![vec![];axes.axesheight()];

        for (x,y) in xsys {
            let xloc = axes.xdatatoaxes(x);
            let yloc = axes.ydatatoaxes(y);
            data_mut[yloc].push(xloc);
        }
        data_mut
    };
    for xvals in data_ybins.iter().rev() {
        let line: String = (0..axes.axeswidth())
                                .map(
                                    |x| if xvals.iter().any(|matchx| *matchx == x) {
                                            '●'
                                        } else {
                                            ' '
                                        }
                                )
                                .collect();
        assert!(line.graphemes(true).count() == axes.axeswidth());
        result.push(line);
    }
    result
}
