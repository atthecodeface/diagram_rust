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
use super::super::IndentOptions;
use super::super::{
    DiagramDescriptor, DiagramElementContent, ElementError, ElementHeader, ElementScope,
};
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::Layout;
use geometry::{Polygon, Rectangle};
use indent_display::{IndentedDisplay, Indenter};

//a Shape element
//tp Shape - an Element that contains a polygon (or path?)
#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Rect,
    Circle,
    Polygon,
}
#[derive(Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub polygon: Polygon,
    pub fill: Option<(f64, f64, f64)>,
    pub stroke: Option<(f64, f64, f64)>,
    pub stroke_width: f64,
    pub markers: (Option<String>, Option<String>, Option<String>),
}

//ip DiagramElementContent for Shape
impl<'a, 'b> DiagramElementContent<'a, 'b> for Shape {
    //fp new
    fn new(_header: &ElementHeader, name: el::Typ) -> Result<Self, ElementError> {
        let shape_type = {
            match name {
                el::Typ::Circle => ShapeType::Circle,
                el::Typ::Rect => ShapeType::Rect,
                _ => ShapeType::Polygon,
            }
        };
        let polygon = Polygon::new(0, 0.);
        Ok(Self {
            shape_type,
            polygon,
            stroke_width: 0.,
            stroke: None,
            fill: None,
            markers: (None, None, None),
        })
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header: &ElementHeader, _scope: &ElementScope) -> Result<Self, ElementError> {
        let mut clone = Self::new(header, el::Typ::Clone)?;
        clone.shape_type = self.shape_type;
        Ok(clone)
    }

    //fp get_style_names
    fn get_style_names<'z>(name: &str) -> Vec<&'z str> {
        match name {
            el::CIRCLE => vec![
                at::FILL,
                at::STROKE,
                at::STROKEWIDTH,
                at::ROUND,
                at::WIDTH,
                at::HEIGHT,
            ],
            el::RECT => vec![
                at::FILL,
                at::STROKE,
                at::STROKEWIDTH,
                at::ROUND,
                at::MARKERS,
                at::WIDTH,
                at::HEIGHT,
            ],
            _ => vec![
                at::FILL,
                at::STROKE,
                at::STROKEWIDTH,
                at::ROUND,
                at::MARKERS,
                at::VERTICES,
                at::STELLATE,
                at::WIDTH,
                at::HEIGHT,
            ],
        }
    }

    //mp style
    fn style(
        &mut self,
        _descriptor: &DiagramDescriptor,
        header: &ElementHeader,
    ) -> Result<(), ElementError> {
        if let Some(v) = header.get_style_rgb_of_name(at::FILL).as_floats(None) {
            self.fill = Some((v[0], v[1], v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name(at::STROKE).as_floats(None) {
            self.stroke = Some((v[0], v[1], v[2]));
        }
        self.stroke_width = header
            .get_style_of_name_float(at::STROKEWIDTH, Some(0.))
            .unwrap();
        let round = header.get_style_of_name_float(at::ROUND, Some(0.)).unwrap();
        let width = header.get_style_of_name_float(at::WIDTH, Some(1.)).unwrap();
        let height = header
            .get_style_of_name_float(at::HEIGHT, Some(width))
            .unwrap();
        let stellate = header
            .get_style_of_name_float(at::STELLATE, Some(0.))
            .unwrap();
        match self.shape_type {
            ShapeType::Polygon => {
                let vertices =
                    header.get_style_of_name_int(at::VERTICES, Some(4)).unwrap() as usize;
                self.polygon.set_vertices(vertices);
                self.polygon.set_size(height / 2., width / height);
            }
            ShapeType::Circle => {
                self.polygon.set_vertices(0);
                self.polygon.set_size(height / 2., width / height);
            }
            ShapeType::Rect => {
                self.polygon.set_vertices(4);
                let eccentricity = width / height;
                let height = height / (2.0_f64.sqrt());
                self.polygon.set_size(height, eccentricity);
            }
        };

        self.polygon.set_rounding(round);
        if stellate != 0. {
            self.polygon.set_stellate_size(stellate);
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout: &mut Layout) -> Rectangle {
        let rect = self.polygon.get_bbox();
        rect.enlarge(self.stroke_width / 2.)
    }

    //zz All done
}
//ip Shape
impl Shape {}

//ip GenerateSvgElement for Shape
impl GenerateSvgElement for Shape {
    fn generate_svg(&self, svg: &mut Svg, header: &ElementHeader) -> Result<(), SvgError> {
        let mut ele = SvgElement::new("path");
        header.svg_add_transform(&mut ele);
        match &self.stroke {
            None => {
                ele.add_attribute("stroke", "None");
            }
            Some(rgb) => {
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
        ele.add_polygon_path(&self.polygon, true);
        svg.add_subelement(ele);
        Ok(())
    }
}
//ti IndentedDisplay for Shape
impl<'a> IndentedDisplay<'a, IndentOptions> for Shape {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        Ok(())
    }
}
