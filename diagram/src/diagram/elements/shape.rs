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

@file    group.rs
@brief   Diagram group element
 */

//a Imports
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, ElementScope, ElementHeader, ElementError};
use crate::{Layout};
use crate::{Rectangle, Polygon};

//a Group element
//tp Shape - an Element that contains a polygon (or path?)
#[derive(Debug)]
pub struct Shape {
    // Possibly polygon
    // has Fill, Stroke, StrokeWidth, Markers
    pub polygon : Polygon,
    pub fill   : Option<(f64,f64,f64)>,
    pub stroke : Option<(f64,f64,f64)>,
    pub stroke_width : f64,
}

//ip DiagramElementContent for Shape
impl <'a, 'b> DiagramElementContent <'a, 'b> for Shape {
    //fp new
    fn new(_header:&ElementHeader, _name:&str) -> Result<Self,ElementError> {
        let polygon = Polygon::new(0, 0.);
        Ok( Self {
            polygon,
            stroke_width:0.,
            stroke : None,
            fill : None,
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
        vec!["fill", "stroke", "strokewidth", "round", "markers", "vertices", "stellate", "width", "height"]
    }

    //mp style
    fn style(&mut self, _descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_rgb_of_name("fill").as_floats(None) {
            self.fill = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name("stroke").as_floats(None) {
            self.stroke = Some((v[0],v[1],v[2]));
        }
        self.stroke_width = header.get_style_of_name_float("strokewidth",Some(0.)).unwrap();
        let round    = header.get_style_of_name_float("round",Some(0.)).unwrap();
        let width    = header.get_style_of_name_float("width",Some(1.)).unwrap();
        let height   = header.get_style_of_name_float("height",Some(width)).unwrap();
        let stellate = header.get_style_of_name_float("stellate",Some(0.)).unwrap();
        let vertices = header.get_style_of_name_int("vertices",Some(4)).unwrap() as usize;
        self.polygon.set_vertices(vertices);
        self.polygon.set_size(height, width/height);
        self.polygon.set_rounding(round);
        if stellate != 0. { self.polygon.set_stellate_size(stellate); }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        let rect = self.polygon.get_bbox();
        rect.enlarge(self.stroke_width)
    }

    //zz All done
}
//ip Shape
impl Shape {
}

//ip GenerateSvgElement for Shape
impl GenerateSvgElement for Shape {
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
        ele.add_size("strokewidth",self.stroke_width);
        ele.add_polygon_path(&self.polygon);
        svg.add_subelement(ele);
        Ok(())
    }
}
