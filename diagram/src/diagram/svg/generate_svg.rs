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

@file    generate_svg.rs
@brief   Trait for generating SVG
 */

//a Imports
use super::super::{Element, ElementContent, ElementHeader};
use super::{Svg, SvgElement, SvgError};
use crate::LayoutRecord;

//a GenerateSvg, GenerateSvgElement
//pt GenerateSvgElement
pub trait GenerateSvgElement {
    fn generate_svg(&self, svg: &mut Svg, header: &ElementHeader) -> Result<(), SvgError>;
}

//ip GenerateSvgElement for ElementContent
impl<'a> GenerateSvgElement for ElementContent<'a> {
    //mp generate_svg
    fn generate_svg(&self, svg: &mut Svg, header: &ElementHeader) -> Result<(), SvgError> {
        match self {
            ElementContent::Path(ref s) => s.generate_svg(svg, header),
            ElementContent::Shape(ref s) => s.generate_svg(svg, header),
            ElementContent::Text(ref t) => t.generate_svg(svg, header),
            ElementContent::Group(ref g) => g.generate_svg(svg, header),
            ElementContent::Use(ref g) => g.generate_svg(svg, header),
        }
    }
}

//pt GenerateSvg
/// This trait provdes a `Diagram` with the ability to render to an
/// SVG object, which may then be written to a file.
pub trait GenerateSvg {
    //mp generate_svg
    /// This method renders to the `Svg` instance any of the XML
    /// elements required for the object
    fn generate_svg(&self, _svg: &mut Svg) -> Result<(), SvgError> {
        Ok(())
    }
    //mp svg_add_transform
    /// This method is used internally
    fn svg_add_transform(&self, _ele: &mut SvgElement) {}
}

//ip GenerateSvg for ElementHeader
impl<'a> GenerateSvg for ElementHeader<'a> {
    fn svg_add_transform(&self, ele: &mut SvgElement) {
        if let Some(id) = self.id_name.as_ref() {
            ele.add_attribute("id", id);
        }
        match self.layout_box.borrow_content_transform() {
            Some(transform) => {
                ele.add_transform(transform);
            }
            _ => (),
        }
    }
}

//ip GenerateSvg for Element
impl<'a> GenerateSvg for Element<'a> {
    fn generate_svg(&self, svg: &mut Svg) -> Result<(), SvgError> {
        // println!("Generate svg with header layout {:?}", self.header.layout);
        if self.header.layout.bg.is_some() {
            let mut ele = SvgElement::new("path");
            ele.add_attribute("stroke", "None");
            ele.add_color("fill", &self.header.layout.bg.unwrap());
            ele.add_polygon_path(self.header.layout_box.get_border_shape().unwrap(), true);
            svg.add_subelement(ele);
        }

        if svg.show_content_rectangles {
            let rect = self.header.layout_box.get_content_rectangle();
            let (c, w, h) = rect.get_cwh();
            let mut ele = SvgElement::new("rect");
            // ele.add_attribute("id", &format!("{}.content_rect",self.header.borrow_id()));
            ele.add_attribute("fill", "#40ff8080");
            ele.add_size("x", c[0] - w / 2.);
            ele.add_size("y", c[1] - h / 2.);
            ele.add_size("width", w);
            ele.add_size("height", h);
            self.header.svg_add_transform(&mut ele);
            svg.add_subelement(ele);
        }

        self.content.generate_svg(svg, &self.header)?;

        if let Some((spacing, color)) = self.header.layout.debug_get_grid() {
            let r = self.header.layout_box.get_content_rectangle();
            if let Some(mut ele) = SvgElement::new_grid(r, spacing, 0.05, color) {
                self.header.svg_add_transform(&mut ele);
                svg.add_subelement(ele);
            }
        }

        if self.header.layout.border_color.is_some() {
            let mut ele = SvgElement::new("path");
            ele.add_color("stroke", &self.header.layout.border_color.unwrap());
            ele.add_size("stroke-width", self.header.layout.border_width);
            ele.add_attribute("fill", "None");
            ele.add_polygon_path(self.header.layout_box.get_border_shape().unwrap(), true);
            svg.add_subelement(ele);
        }
        Ok(())
    }
}

//ip GenerateSvg for LayoutRecord
impl GenerateSvg for LayoutRecord {
    fn generate_svg(&self, svg: &mut Svg) -> Result<(), SvgError> {
        match &self.grid_positions {
            Some((grid_x, grid_y)) => {
                if grid_x.len() < 2 || grid_y.len() < 2 {
                    ()
                } else {
                    let color = "lime";
                    let line_width = 0.25;
                    let mut rx = String::new();
                    let mut ry = String::new();
                    let xn = grid_x.len();
                    let yn = grid_y.len();
                    let x0 = grid_x[0].1;
                    let x1 = grid_x[xn - 1].1;
                    let y0 = grid_y[0].1;
                    let y1 = grid_y[yn - 1].1;
                    for (_, x) in grid_x {
                        rx.push_str(&format!("M {:.4},{:.4} v {:.4} ", x, y0, y1 - y0));
                    }
                    for (_, y) in grid_y {
                        ry.push_str(&format!("M {:.4},{:.4} h {:.4} ", x0, y, x1 - x0));
                    }
                    rx.push_str(&ry);
                    let mut grid = SvgElement::new("path");
                    grid.add_attribute("fill", "None");
                    grid.add_attribute("stroke", color);
                    grid.add_attribute("stroke-width", &format!("{:.4}", line_width));
                    grid.add_attribute("d", &rx);
                    svg.add_subelement(grid);
                    ()
                }
            }
            _ => (),
        }
        Ok(())
    }
}
