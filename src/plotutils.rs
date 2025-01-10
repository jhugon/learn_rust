use unicode_segmentation::UnicodeSegmentation;

/// Draws the bottom x-axis line with a few labeled ticks
pub fn drawxaxis(axesmeta: &AxesMeta) -> Vec<String> {
    let xmin = axesmeta.dataminmax.xmin;
    let xmax = axesmeta.dataminmax.xmax;
    let xhalf = (xmax+xmin)*0.5;
    let axiswidth = axesmeta.axeswidth();
    let halfaxiswidth = axiswidth / 2;
    let leftmargin = axesmeta.leftmargin;
    let linetext: String = (0..axiswidth-1)
                            .map(|x| if x == halfaxiswidth {"┬"} else {"─"})
                            .collect();
    let ylabelwidth = leftmargin-2;
    let result = vec![
        format!("{:>ylabelwidth$} ├{linetext}┐",""),
        format!(
            "{:>ylabelwidth$}  {xmin:<10.3}{xhalf:>firsthalfwidth$.3}{xmax:>secondhalfwidth$.3}",
            "",
            ylabelwidth=ylabelwidth,
            xmin=xmin,
            xhalf=xhalf,
            firsthalfwidth=halfaxiswidth-10,
            xmax=xmax,
            secondhalfwidth=halfaxiswidth
        )
    ];
    result
}

/// Draws the left y-axis scale and bar
pub fn drawyaxis(axesmeta: &AxesMeta) -> Vec<String> {
    let leftmargin = axesmeta.leftmargin;
    let numwidth = leftmargin - 2;
    let ybincenters = axesmeta.ybincenters();
    let result: Vec<String> = ybincenters.iter().rev()
                    .map(|y| format!("{:>numwidth$.3} │",y,numwidth=numwidth))
                    .collect();
    for line in &result {
        assert!(line.graphemes(true).count() == leftmargin);
    }
    result
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
        let resultfloat = (self.axeswidth() as f32-1.)/self.dataminmax.xwidth()*(x-self.dataminmax.xmin);
        resultfloat.floor() as usize
    }
    pub fn ydatatoaxes(&self, y: &f32) -> usize {
        let resultfloat = (self.axesheight() as f32-1.)/self.dataminmax.ywidth()*(y-self.dataminmax.ymin);
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
        (0..self.axeswidth())
            .map(|ibin| self.dataminmax.xmin + (self.xbinwidth()*(ibin as f32+0.5)))
            .collect()
    }
    pub fn ybincenters(&self) -> Vec<f32> {
        (0..self.axesheight())
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

