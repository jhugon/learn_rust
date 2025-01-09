
const LEFTMARGIN: usize = 12;

/// Draws the bottom x-axis line with a few ticks
pub fn drawxaxis(axesmeta: &AxesMeta) -> () {
    let xmin = axesmeta.dataminmax.xmin;
    let xmax = axesmeta.dataminmax.xmax;
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

//////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct AxesMeta {
    pub dataminmax: DataMinMax,
    pub termwidth: usize,
    pub termheight: usize,
    pub leftmargin: usize,
    pub botmargin: usize,
}

impl AxesMeta {
    pub fn xdatatoaxes(&self, x: &f32) -> usize {
        let resultfloat = self.axeswidth() as f32/self.dataminmax.xwidth()*(x-self.dataminmax.xmin);
        resultfloat.floor() as usize
    }
    pub fn ydatatoaxes(&self, y: &f32) -> usize {
        let resultfloat = self.axesheight() as f32/self.dataminmax.ywidth()*(y-self.dataminmax.ymin);
        resultfloat.floor() as usize
    }
    pub fn axeswidth(&self) -> usize {
        self.termwidth-self.leftmargin
    }
    pub fn axesheight(&self) -> usize {
        self.termheight-self.botmargin
    }
    pub fn xbinwidth(&self) -> f32 {
        self.dataminmax.xwidth()/self.axeswidth() as f32
    }
    pub fn ybinwidth(&self) -> f32 {
        self.dataminmax.ywidth()/self.axesheight() as f32
    }
    pub fn xbincenters(&self) -> Vec<f32> {
        (0..self.axeswidth()+1)
            .map(|ibin| self.dataminmax.xmin + (self.xbinwidth()*(ibin as f32+0.5)))
            .collect()
    }
    pub fn ybincenters(&self) -> Vec<f32> {
        (0..self.axesheight()+1)
            .map(|ibin| self.dataminmax.ymin + (self.ybinwidth()*(ibin as f32+0.5)))
            .collect()
    }
}

//////////////////////////////////////////////////////////////


#[derive(Debug)]
pub struct DataMinMax {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl DataMinMax {
    pub fn find(xsys: &[(f32,f32)]) -> DataMinMax {
        let (xmin,ymin): (f32,f32) = xsys.iter()
                            .fold((0.,0.),|(xmin,ymin): (f32,f32),(x,y)| (xmin.min(*x),ymin.min(*y)));
        let (xmax,ymax): (f32,f32) = xsys.iter()
                            .fold((0.,0.),|(xmax,ymax): (f32,f32),(x,y)| (xmax.max(*x),ymax.max(*y)));
        DataMinMax { xmin: xmin, xmax: xmax, ymin: ymin, ymax: ymax }
    }

    pub fn xwidth(&self) -> f32 {
        self.xmax-self.xmin
    }

    pub fn ywidth(&self) -> f32 {
        self.ymax-self.ymin
    }
}

