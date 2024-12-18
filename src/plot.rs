use std::iter::zip;
use std::error::Error;

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
    let yheight = ymax-ymin;

    let leftmargin: u32 = 12;
    let botmargin: u32 = 0;
    let termwidth = u32::from(termsize::get().unwrap().cols);
    let termheight = u32::from(termsize::get().unwrap().rows);

    let maxwidth = termwidth - leftmargin;
    let maxheight = termheight - botmargin;
    let _xtrans = |x: f32| maxwidth as f32/xwidth*x-xmin;
    let ytrans = |y: f32| maxheight as f32/yheight*y-ymin;
    for (_x,y) in &xsys {
        let yloc = ytrans(*y) as usize;
        println!("{:>10} |{:>y$}","","*",y=yloc);
    }
    for (x,y) in &xsys {
        println!("{:10} {:10}",*x,*y);
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
