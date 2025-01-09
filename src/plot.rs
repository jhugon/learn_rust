use std::iter::zip;
use std::error::Error;
use crate::plotutils::*;

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

    let plotteddata = drawdata(&xsys,&axes);
    //println!("{:10} ├{}┐","",(0..maxwidth).map(|x| if x == maxwidth / 2 {"┬"} else {"─"}).collect::<String>());
    //println!("{:10}  {:<10.3}{:>firsthalfwidth$.3}{xmax:>secondhalfwidth$.3}","",xmin,xmin + xwidth*0.5,firsthalfwidth=(maxwidth / 2)-10,xmax=xmax,secondhalfwidth=(maxwidth/2));
    drawxaxis(&axes);
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

    let ybincenters = axes.ybincenters();
    let data_ybins: Vec<Vec<usize>> = { // put mut var in its own scope to keep it here
        let mut data_mut = vec![vec![];axes.axesheight()];

        for (x,y) in xsys {
            let xloc = axes.xdatatoaxes(x);
            let yloc = axes.ydatatoaxes(y);
            data_mut[yloc].push(xloc);
        }
        data_mut
    };
    for (y,xvals) in zip(&ybincenters,&data_ybins).rev() {
        let line: String = (0..axes.axeswidth()+1)
                                .map(
                                    |x| if xvals.iter().any(|matchx| *matchx == x) {
                                            '●'
                                        } else {
                                            ' '
                                        }
                                )
                                .collect();
        result.push(format!("{:>10.3} │{}",y,line));
    }
    result
}
