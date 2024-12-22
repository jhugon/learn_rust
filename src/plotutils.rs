
const LEFTMARGIN: usize = 12;

/// Draws the bottom x-axis line with a few ticks
pub fn drawxaxis(xmin: f32,xmax: f32) -> () {
    let xhalf = (xmax+xmin)*0.5;
    let axiswidth = termwidth() - LEFTMARGIN-1;
    let halfaxiswidth = axiswidth / 2;
    println!("{:10} ├{}┐","",(0..axiswidth).map(|x| if x == halfaxiswidth {"┬"} else {"─"}).collect::<String>());
    println!("{:10}  {xmin:<10.3}{xhalf:>firsthalfwidth$.3}{xmax:>secondhalfwidth$.3}","",xmin=xmin,xhalf=xhalf,firsthalfwidth=halfaxiswidth-10,xmax=xmax,secondhalfwidth=halfaxiswidth);
}

/// Get the terminal x-width
fn termwidth() -> usize {
    usize::from(termsize::get().unwrap().cols)
}
