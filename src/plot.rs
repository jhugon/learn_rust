use std::iter::zip;

/// Plots in the terminal
///
/// Removes all xs and ys that contain nan or infinity
pub fn plot(xs: Vec<f32>, ys: Vec<f32>) {
    let xsys_incnan = zip(xs,ys);
    let xsys = xsys_incnan.filter(|(x,y)| x.is_finite() && y.is_finite());

    let (xmin,ymin) = xsys.clone().fold((0.,0.),|(xmin,ymin): (f32,f32),(x,y)| (xmin.min(x),ymin.min(y)));
    let (xmax,ymax) = xsys.clone().fold((0.,0.),|(xmax,ymax): (f32,f32),(x,y)| (xmax.max(x),ymax.max(y)));
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
    for (_x,y) in xsys {
        let yloc = ytrans(y) as usize;
        println!("{:>10} |{:>y$}","","*",y=yloc);
    }
}
