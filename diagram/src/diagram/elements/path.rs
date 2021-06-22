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
use geometry::{Rectangle, Bezier, BezierPath, Point};
use crate::constants::attributes as at;
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, ElementScope, ElementHeader, ElementError};
use crate::{Layout};

//a Constants
const BEZIER_STRAIGHTNESS : f64 = 1E-2;

//a Path element
//tp Path - an Element that contains a path
#[derive(Debug)]
pub struct Path {
    // shape type - cubic, quadratic, linear
    // markers?
    pub center : Point,
    pub width : f64,
    pub height : f64,
    pub round  : f64,
    pub closed : bool,
    pub coords : Vec<Point>, // relative to actual width and height
    pub fill   : Option<(f64,f64,f64)>,
    pub stroke : Option<(f64,f64,f64)>,
    pub stroke_width : f64,
    pub markers : (Option<String>, Option<String>, Option<String>),
}

//ip DiagramElementContent for Path
impl <'a, 'b> DiagramElementContent <'a, 'b> for Path {
    //fp new
    fn new(_header:&ElementHeader, _name:&str) -> Result<Self,ElementError> {
        Ok( Self {
            center:Point::zero(),
            width : 0.,
            height : 0.,
            round : 0.,
            closed : false,
            coords : Vec::new(),
            stroke_width:0.,
            stroke : None,
            fill : None,
            markers : (None, None, None),
        } )
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header:&ElementHeader, _scope:&ElementScope ) -> Result<Self,ElementError>{
        let clone = Self::new(header, "")?;
        Ok( clone )
    }

    //fp get_style_names
    fn get_style_names<'z> (_name:&str) -> Vec<&'z str> {
        vec![at::FILL,
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
    fn style(&mut self, _descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(i) = header.get_style_of_name_int(at::FLAGS, None) {
            self.closed = (i & 1) == 1;
        }
        if let Some(v) = header.get_style_rgb_of_name(at::FILL).as_floats(None) {
            self.fill = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name(at::STROKE).as_floats(None) {
            self.stroke = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_floats_of_name(at::COORDS).as_floats(None) {
            // v : Vec<f64>
            self.coords = Vec::new();
            for i in 0..v.len()/2 {
                let x = v[i*2];
                let y = v[i*2+1];
                self.coords.push(Point::from_array([x,y]));
            }
        }
        if let Some(v) = header.get_style_strings_of_name(at::MARKERS).as_strings(None) {
            match v.len() {
                1 => {
                    self.markers.0 = Some(v[0].clone());
                },
                2 => {
                    if v[0] != "none" {
                        self.markers.0 = Some(v[0].clone());
                    }
                    self.markers.2 = Some(v[1].clone());
                },
                _ => {
                    self.markers.0 = Some(v[0].clone());
                    self.markers.1 = Some(v[1].clone());
                    self.markers.2 = Some(v[2].clone());
                },
            }
        }
        self.stroke_width = header.get_style_of_name_float(at::STROKEWIDTH,Some(0.)).unwrap();
        self.round    = header.get_style_of_name_float(at::ROUND,Some(0.)).unwrap();
        self.width    = header.get_style_of_name_float(at::WIDTH,Some(1.)).unwrap();
        self.height   = header.get_style_of_name_float(at::HEIGHT,Some(self.width)).unwrap();
        if self.closed && self.coords.len() > 2 {
            if self.coords[0].distance(&self.coords[self.coords.len()-1]) > 1E-6 {
                self.coords.push(self.coords[0].clone());
            }
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        Rectangle::of_cwh(self.center, self.width, self.height)
    }

    //fp apply_placement
    fn apply_placement(&mut self, _layout:&Layout, rect:&Rectangle) {
        // Policy decision not to reduce by stroke_width
        // let (c,w,h) = rect.clone().reduce(self.stroke_width).get_cwh();
        let (c,w,h) = rect.get_cwh();
        self.center = c;
        self.width  = w;
        self.height = h;
    }

    //mp display
    /// Display - using indent_str + 2 indent, or an indent of indent spaces
    /// Content should be invoked with indent+4
    fn display(&self, _indent:usize, indent_str:&str) {
        println!("{}  path {} {} {} {}",indent_str, self.center, self.width, self.height, self.round);
    }

    //zz All done
}
//ip Path
impl Path {
}

//ip GenerateSvgElement for Path
impl GenerateSvgElement for Path {
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError> {
        let mut ele = SvgElement::new("path");
        header.svg_add_transform(&mut ele);
        if self.coords.is_empty() {
            return Ok(());
        }
        match &self.stroke {
            None      => {ele.add_attribute("stroke","None");},
            Some(rgb) => {ele.add_color("stroke",rgb);},
        }
        match &self.fill {
            None      => {ele.add_attribute("fill","None");},
            Some(rgb) => {ele.add_color("fill",rgb);},
        }
        ele.add_markers(&self.markers);
        ele.add_size("stroke-width",self.stroke_width);
        let scale_xy = Point::from_array([self.width*0.5, self.height*0.5]);
        let mut coords = Vec::new();
        for c in &self.coords {
            coords.push( (*c)*scale_xy + self.center );
        }
        let mut path = BezierPath::new();
        for i in 0..coords.len()-1 {
            path.add_bezier( Bezier::line(&coords[i], &coords[i+1]) );
        }
        // apply marker relief of stroke-width * relief for start and end markers
        if let Some(m) = &self.markers.0 {
            if let Some((_,m)) = svg.diagram.borrow_marker(&m).map(|e| e.borrow_marker().unwrap()) {
                let relief = m.get_relief(0);
                if relief > 0. {
                    path.apply_relief(0, BEZIER_STRAIGHTNESS, relief*self.stroke_width);
                }
            }
        }
        if let Some(m) = &self.markers.2 {
            if let Some((_,m)) = svg.diagram.borrow_marker(&m).map(|e| e.borrow_marker().unwrap()) {
                let relief = m.get_relief(1);
                if relief > 0. {
                    path.apply_relief(1, BEZIER_STRAIGHTNESS, relief*self.stroke_width);
                }
            }
        }
        path.round(self.round, self.closed);
        ele.add_bezier_path(&path, self.closed);
        svg.add_subelement(ele);
        Ok(())
    }
}
