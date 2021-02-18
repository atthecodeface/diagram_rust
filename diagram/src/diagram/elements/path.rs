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
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, ElementScope, ElementHeader, ElementError};
use crate::{Layout};
use crate::{Rectangle, Bezier, Point};

//a Path element
//tp Path - an Element that contains a path
#[derive(Debug)]
pub struct Path {
    pub closed : bool,
    // shape type - cubic, quadratic, linear
    // markers?
    pub center : Point,
    pub width : f64,
    pub height : f64,
    pub coords : Vec<Point>, // relative to actual width and height
    pub fill   : Option<(f64,f64,f64)>,
    pub stroke : Option<(f64,f64,f64)>,
    pub stroke_width : f64,
    pub markers : (Option<String>, Option<String>, Option<String>),
}

//ip DiagramElementContent for Path
impl <'a, 'b> DiagramElementContent <'a, 'b> for Path {
    //fp new
    fn new(_header:&ElementHeader, name:&str) -> Result<Self,ElementError> {
        let closed=false;
        Ok( Self {
            closed,
            center:Point::origin(),
            width : 0.,
            height : 0.,
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
        vec!["fill", "stroke", "strokewidth", "round", "markers", "width", "height", "coords"]
    }

    //mp style
    fn style(&mut self, _descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_rgb_of_name("fill").as_floats(None) {
            self.fill = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name("stroke").as_floats(None) {
            self.stroke = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_floats_of_name("coords").as_floats(None) {
            // v : Vec<f64>
            self.coords = Vec::new();
            for i in 0..v.len()/2 {
                let x = v[i*2];
                let y = v[i*2+1];
                self.coords.push(Point::new(x,y));
            }
        }
        if let Some(v) = header.get_style_strings_of_name("markers").as_strings(None) {
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
        self.stroke_width = header.get_style_of_name_float("strokewidth",Some(0.)).unwrap();
        let round    = header.get_style_of_name_float("round",Some(0.)).unwrap();
        self.width    = header.get_style_of_name_float("width",Some(1.)).unwrap();
        self.height   = header.get_style_of_name_float("height",Some(self.width)).unwrap();
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        Rectangle::of_cwh(self.center, self.width, self.height)
            .enlarge(self.stroke_width/2.)
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
    fn display(&self, indent:usize, indent_str:&str) {
        println!("{}  path {} {} {}",indent_str, self.center, self.width, self.height);
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
        let mut coords = Vec::new();
        let bl = self.center.clone().add(&Point::new(self.width*-0.5, self.height*-0.5), 1.);
        for c in &self.coords {
            coords.push( c.scale_xy(self.width, self.height).add(&bl, 1.) );
        }
        let mut path = Vec::new();
        for i in 0..coords.len()-1 {
            path.push( Bezier::line(&coords[i], &coords[i+1]) );
        }
        ele.add_path(&path, self.closed);
        svg.add_subelement(ele);
        Ok(())
    }
}