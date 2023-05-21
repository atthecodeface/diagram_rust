/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    path.rs
@brief   Diagram path element
 */

//a Imports
use geo_nd::Vector;
use indent_display::{IndentedDisplay, Indenter};
use vg_rs::layout::Layout;
use vg_rs::{BBox, Bezier, BezierPath, Point};

use super::super::IndentOptions;
use super::super::{
    DiagramDescriptor, DiagramElementContent, ElementError, ElementHeader, ElementScope,
};
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use crate::constants::attributes as at;
use crate::constants::elements as el;

//a Constants
const BEZIER_STRAIGHTNESS: f64 = 1E-2;

//a Path element
//tp Path - an Element that contains a path
#[derive(Debug)]
pub struct Path {
    // shape type - cubic, quadratic, linear
    // markers?
    pub center: Point,
    pub width: f64,
    pub height: f64,
    pub round: f64,
    pub closed: bool,
    pub coords: Vec<Point>, // relative to actual width and height
    pub fill: Option<(f64, f64, f64)>,
    pub stroke: Option<(f64, f64, f64)>,
    pub stroke_width: f64,
    pub markers: (Option<String>, Option<String>, Option<String>),
}

//ip DiagramElementContent for Path
impl<'a, 'b> DiagramElementContent<'a, 'b> for Path {
    //fp new
    fn new(_header: &ElementHeader, _name: el::Typ) -> Result<Self, ElementError> {
        Ok(Self {
            center: Point::zero(),
            width: 0.,
            height: 0.,
            round: 0.,
            closed: false,
            coords: Vec::new(),
            stroke_width: 0.,
            stroke: None,
            fill: None,
            markers: (None, None, None),
        })
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header: &ElementHeader, _scope: &ElementScope) -> Result<Self, ElementError> {
        let clone = Self::new(header, el::Typ::Clone)?;
        Ok(clone)
    }

    //fp get_style_names
    fn get_style_names<'z>(_name: &str) -> Vec<&'z str> {
        vec![
            at::FILL,
            at::STROKE,
            at::STROKEWIDTH,
            at::ROUND,
            at::MARKERS,
            at::WIDTH,
            at::HEIGHT,
            at::COORDS,
            at::FLAGS,
        ]
    }

    //mp style
    fn style(
        &mut self,
        _descriptor: &DiagramDescriptor,
        header: &ElementHeader,
    ) -> Result<(), ElementError> {
        if let Some(i) = header.get_style_of_name_int(at::FLAGS, None) {
            self.closed = (i & 1) == 1;
        }
        let mut floats = [0.; 4];
        if let Some(v) = header
            .get_style_value_of_name(at::FILL)
            .and_then(|x| x.as_floats(&mut floats))
        {
            self.fill = Some((v[0], v[1], v[2]));
        }
        if let Some(v) = header
            .get_style_value_of_name(at::STROKE)
            .and_then(|x| x.as_floats(&mut floats))
        {
            self.stroke = Some((v[0], v[1], v[2]));
        }
        if let Some(v) = header
            .get_style_value_of_name(at::COORDS)
            .and_then(|x| x.as_vec_float())
        {
            // v : Vec<f64>
            self.coords = Vec::new();
            for i in 0..v.len() / 2 {
                let x = v[i * 2];
                let y = v[i * 2 + 1];
                self.coords.push(Point::from_array([x, y]));
            }
        }
        let mut strs = [""; 4];
        if let Some(v) = header
            .get_style_value_of_name(at::MARKERS)
            .and_then(|x| x.as_strs(&mut strs))
        {
            match v.len() {
                0 => {}
                1 => {
                    self.markers.0 = Some(v[0].into());
                }
                2 => {
                    if v[0] != "none" {
                        self.markers.0 = Some(v[0].into());
                    }
                    self.markers.2 = Some(v[1].into());
                }
                _ => {
                    self.markers.0 = Some(v[0].into());
                    self.markers.1 = Some(v[1].into());
                    self.markers.2 = Some(v[2].into());
                }
            }
        }
        self.stroke_width = header
            .get_style_of_name_float(at::STROKEWIDTH, Some(0.))
            .unwrap();
        self.round = header.get_style_of_name_float(at::ROUND, Some(0.)).unwrap();
        self.width = header.get_style_of_name_float(at::WIDTH, Some(1.)).unwrap();
        self.height = header
            .get_style_of_name_float(at::HEIGHT, Some(self.width))
            .unwrap();
        let n = self.coords.len();
        if self.closed && n > 2 && self.coords[0].distance(&self.coords[n - 1]) > 1E-6 {
            self.coords.push(self.coords[0]);
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout: &mut Layout) -> BBox {
        BBox::of_cwh(self.center, self.width, self.height)
    }

    //fp apply_placement
    fn apply_placement(&mut self, _layout: &Layout, rect: &BBox) {
        // Policy decision not to reduce by stroke_width
        // let (c,w,h) = rect.clone().reduce(self.stroke_width).get_cwh();
        let (c, w, h) = rect.get_cwh();
        self.center = c;
        self.width = w;
        self.height = h;
    }

    //mp display
    /// Display - using indent_str + 2 indent, or an indent of indent spaces
    /// Content should be invoked with indent+4
    fn display(&self, _indent: usize, indent_str: &str) {
        println!(
            "{}  path {} {} {} {}",
            indent_str, self.center, self.width, self.height, self.round
        );
    }

    //zz All done
}
//ip Path
impl Path {}

//ip GenerateSvgElement for Path
impl GenerateSvgElement for Path {
    fn generate_svg(&self, svg: &mut Svg, header: &ElementHeader) -> Result<(), SvgError> {
        let mut ele = SvgElement::new("path");
        header.svg_add_transform(&mut ele);
        if self.coords.is_empty() {
            return Ok(());
        }
        match &self.stroke {
            None => {
                ele.add_attribute("stroke", "None");
            }
            Some(rgb) => {
                dbg!("Stroke", rgb);
                ele.add_color("stroke", rgb);
            }
        }
        match &self.fill {
            None => {
                ele.add_attribute("fill", "None");
            }
            Some(rgb) => {
                ele.add_color("fill", rgb);
            }
        }
        ele.add_markers(&self.markers);
        ele.add_size("stroke-width", self.stroke_width);
        let scale_xy = Point::from_array([self.width * 0.5, self.height * 0.5]);
        let mut coords = Vec::new();
        for c in &self.coords {
            coords.push((*c) * scale_xy + self.center);
        }
        let mut path = BezierPath::default();
        for i in 0..coords.len() - 1 {
            path.add_bezier(Bezier::line(&coords[i], &coords[i + 1]));
        }
        // apply marker relief of stroke-width * relief for start and end markers
        if let Some(m) = &self.markers.0 {
            if let Some((_, m)) = svg
                .diagram
                .find_marker(m)
                .map(|e| e.borrow_marker().unwrap())
            {
                let relief = m.get_relief(0);
                if relief > 0. {
                    path.apply_relief(0, BEZIER_STRAIGHTNESS, relief * self.stroke_width);
                }
            }
        }
        if let Some(m) = &self.markers.2 {
            if let Some((_, m)) = svg
                .diagram
                .find_marker(m)
                .map(|e| e.borrow_marker().unwrap())
            {
                let relief = m.get_relief(1);
                if relief > 0. {
                    path.apply_relief(1, BEZIER_STRAIGHTNESS, relief * self.stroke_width);
                }
            }
        }
        path.round(self.round, self.closed);
        ele.add_bezier_path(&path, self.closed);
        svg.add_subelement(ele);
        Ok(())
    }
}
//ti IndentedDisplay for Path
impl<'a> IndentedDisplay<'a, IndentOptions> for Path {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        use std::fmt::Write;
        writeln!(ind, "Path")?;
        let mut sub = ind.sub();
        writeln!(&mut sub, "center  : {}", self.center)?;
        writeln!(&mut sub, "width   : {}", self.width)?;
        writeln!(&mut sub, "height  : {}", self.height)?;
        writeln!(&mut sub, "round   : {}", self.round)?;
        writeln!(&mut sub, "closed  : {}", self.closed)?;
        writeln!(&mut sub, "fill    : {:?}", self.fill)?;
        writeln!(&mut sub, "stroke  : {:?}", self.stroke)?;
        writeln!(&mut sub, "strokewidth  : {}", self.stroke_width)?;
        writeln!(&mut sub, "markers  : {:?}", self.markers)?;
        // pub coords : Vec<Point>, // relative to actual width and height
        Ok(())
    }
}
