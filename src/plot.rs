use std::iter::zip;
use std::error::Error;
use crate::plotutils::drawxaxis;

/// Plots in the terminal
///
/// Removes all xs and ys that contain nan or infinity
pub fn plot(xs: &[f32], ys: &[f32]) -> Result<(), Box<dyn Error>> {
    let xsys = validate_input(xs,ys);

    let (xmin,ymin): (f32,f32) = xsys.iter()
                        .fold((0.,0.),|(xmin,ymin): (f32,f32),(x,y)| (xmin.min(*x),ymin.min(*y)));
    let (xmax,ymax): (f32,f32) = xsys.iter()
                        .fold((0.,0.),|(xmax,ymax): (f32,f32),(x,y)| (xmax.max(*x),ymax.max(*y)));
    let xwidth = xmax-xmin;
    let ywidth = ymax-ymin;
    println!("xmin,xmax: {},{} ymin,ymax: {},{} xwidth,ywidth: {},{}",
                        xmin,xmax,ymin,ymax,xwidth,ywidth);

    let leftmargin: usize = 12;
    let botmargin: usize = 3;
    let termwidth = usize::from(termsize::get().unwrap().cols);
    let termheight = usize::from(termsize::get().unwrap().rows);

    let maxwidth = termwidth - leftmargin -1;
    let maxheight = termheight - botmargin -1;
    println!("maxwidth,maxheight: {},{}",maxwidth,maxheight);

    let xtrans = |x: f32| maxwidth as f32/xwidth*(x-xmin);
    let ytrans = |y: f32| maxheight as f32/ywidth*(y-ymin);
    println!("{:>10} {:>10} {:>10} {:>10}","x","y","xtrans","ytrans");
    for (x,y) in &xsys {
        println!("{:10} {:10} {:10} {:10}",*x,*y,xtrans(*x),ytrans(*y));
    }

    let ybinwidth: f32 = ywidth/maxheight as f32;
    let ybincenters: Vec<f32> = (0..maxheight+1)
                        .map(|ibin| ymin + (ybinwidth*(ibin as f32+0.5)))
                        .collect();
    let data_ybins: Vec<Vec<usize>> = { // put mut var in its own scope to keep it here
        let mut data_mut = vec![vec![];usize::try_from(maxheight+1).expect("n ybins fits in usize")];

        for (x,y) in &xsys {
            let xloc = xtrans(*x) as usize;
            let yloc = ytrans(*y) as usize;
            //println!("x,y: {},{} xloc,yloc: {},{}",*x,*y,xloc,yloc);
            data_mut[yloc].push(xloc);
        }
        data_mut
    };
    //for (ibin, yvals) in data_xbins.iter().enumerate() {
    //    println!("{:10} {:?}",ibin,yvals);
    //}
    for (y,xvals) in zip(&ybincenters,&data_ybins).rev() {
        let line: String = (0..maxwidth+1)
                                .map(
                                    |x| if xvals.iter().any(|matchx| *matchx == x) {
                                            '●'
                                        } else {
                                            ' '
                                        }
                                )
                                .collect();
        println!("{:>10.3} │{}",y,line);
    }
    //println!("{:10} ├{}┐","",(0..maxwidth).map(|x| if x == maxwidth / 2 {"┬"} else {"─"}).collect::<String>());
    //println!("{:10}  {:<10.3}{:>firsthalfwidth$.3}{xmax:>secondhalfwidth$.3}","",xmin,xmin + xwidth*0.5,firsthalfwidth=(maxwidth / 2)-10,xmax=xmax,secondhalfwidth=(maxwidth/2));
    drawxaxis(xmin,xmax);
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
