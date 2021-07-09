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

@file    svg.rs
@brief   Generate SVG output
 */

//a Imports
use geo_nd::Vector;
use geometry::{Polygon, Rectangle, BezierPath, Point, Transform};

//a Useful stuff
fn pt_as_str(pt:&Point) -> String {
    format!("{:.4},{:.4}", pt[0], pt[1])
}
const INDENT_STRING : &str="                                                            ";

//a SvgElement
//tp SvgElement
pub struct SvgElement {
    pub(super) name : String,
    pub(super) attributes : Vec<(String,String)>,
    pub(super) contents : Vec<SvgElement>,
    pub(super) characters : Option<String>,
}

//ip SvgElement
impl SvgElement {
    //fp new
    pub fn new(name:&str) -> Self {
        Self { name:name.to_string(),
               attributes : Vec::new(),
               contents : Vec::new(),
               characters : None,
        }
    }

    //fp add_attribute
    pub fn add_attribute(&mut self, name:&str, value:&str) {
        self.attributes.push( (name.to_string(), value.to_string() ) );
    }

    //fp add_string
    pub fn add_string(&mut self, s:&str) {
        self.characters = Some(s.to_string());
    }

    //fp add_transform
    pub fn add_transform(&mut self, transform:&Transform) {
        let mut r = String::new();
        if !transform.translation.is_zero() { r.push_str(&format!("translate({:.4} {:.4}) ",transform.translation[0], transform.translation[1])); }
        if transform.rotation != 0.           { r.push_str(&format!("rotate({:.4}) ",transform.rotation)); }
        if transform.scale != 1.              { r.push_str(&format!("scale({:.4}) ",transform.scale)); }
        if r.len() > 0 {
            self.add_attribute("transform", &r);
        }
    }

    //fp add_size
    pub fn add_size(&mut self, name:&str, value:f64) {
        self.add_attribute(name, &format!("{:.4}", value));
    }

    //fp add_color
    pub fn add_color(&mut self, name:&str, value:&(f64,f64,f64)) {
        let r = (value.0 * 255.).round() as u32;
        let g = (value.1 * 255.).round() as u32;
        let b = (value.2 * 255.).round() as u32;
        let rgb = (b<<0) | (g<<8) | (r<<16); // this order for SVG
        self.add_attribute(name, &format!("#{:06x}", rgb));
    }

    //fp add_markers
    pub fn add_markers(&mut self, markers:&(Option<String>, Option<String>, Option<String>)) {
        if let Some(ref s) = markers.0 {
            self.add_attribute("marker-start", &format!("url(#{})",s));
        }
        if let Some(ref s) = markers.1 {
            self.add_attribute("marker-mid", &format!("url(#{})",s));
        }
        if let Some(ref s) = markers.2 {
            self.add_attribute("marker-end", &format!("url(#{})",s));
        }
    }

    //fp add_bezier_path
    pub fn add_bezier_path(&mut self, bp:&BezierPath, closed:bool) {
        let mut r = String::new();
        r.push_str(&format!("M {}",pt_as_str(&bp.get_pt(0))));
        for b in bp.iter_beziers() {
            if b.is_line() {
                r.push_str( &format!(" L {}",
                                     pt_as_str(b.borrow_pt(1)) ));
            } else if b.is_quadratic() {
                r.push_str(&format!(" Q {} {}",
                                    pt_as_str(b.borrow_pt(2)),
                                    pt_as_str(b.borrow_pt(1)) ));
            } else {
                r.push_str(&format!(" C {} {} {}",
                                    pt_as_str(b.borrow_pt(2)),
                                    pt_as_str(b.borrow_pt(3)),
                                    pt_as_str(b.borrow_pt(1)) ));
            }
        }
        if closed { r.push_str(" z"); }
        self.add_attribute("d", &r);
    }

    //fp add_polygon_path
    pub fn add_polygon_path(&mut self, p:&Polygon, closed:bool) {
        self.add_bezier_path(&p.as_paths(), closed);
    }

    //fp new_grid
    /// Create a grid element with given region, spacing, line
    /// width and color
    pub(super) fn new_grid(rect:Rectangle, spacing:f64, line_width:f64, color:&str) -> Option<Self> {
        let xmin = ((rect.x0 / spacing) + 0. ).floor() as isize;
        let xmax = ((rect.x1 / spacing) + 1. ).floor() as isize;
        let xlen = xmax - xmin;

        let ymin = ((rect.y0 / spacing) + 0. ).floor() as isize;
        let ymax = ((rect.y1 / spacing) + 1. ).floor() as isize;
        let ylen = ymax - ymin;

        if xlen < 2 || ylen < 2 { return None; }

        let mut r = String::new();
        for i in xmin .. xmax+1 {
            let coord = (i as f64) * spacing;
            r.push_str(&format!("M {},{} v {} ",coord,rect.y0,rect.y1-rect.y0));
        }

        for i in ymin .. ymax+1 {
            let coord = (i as f64) * spacing;
            r.push_str(&format!("M {},{} h {} ",rect.x0,coord,rect.x1-rect.x0));
        }

        let mut grid = SvgElement::new("path");
        grid.add_attribute("fill","None");
        grid.add_attribute("stroke",color);
        grid.add_attribute("stroke-width",&format!("{}",line_width));
        grid.add_attribute("d",&r);
        Some(grid)
    }


    //fp display
    pub fn display(&self, indent:usize) {
        let indent_str = &INDENT_STRING[0..indent];
        println!("{}{}",indent_str, self.name);
        for (n,v) in &self.attributes {
            println!("{}      {}={}",indent_str, n,v);
        }
        for e in &self.contents {
            e.display(indent+2);
        }
    }

    //zz All done
}

