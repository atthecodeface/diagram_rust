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
use super::{Element, ElementHeader, ElementContent, Diagram};
use crate::{LayoutRecord, Transform};
use crate::{Polygon, Bezier, Point};
use xml::attribute::{Attribute};
use xml::name::{Name};
use xml::namespace::{Namespace};
use xml::reader::XmlEvent;
use xml::common::XmlVersion;

//a Useful stuff
fn pt_as_str(pt:&Point) -> String {
    format!("{:.4},{:.4}", pt.x, pt.y)
}
const INDENT_STRING : &str="                                                            ";

//a SvgError
//tp SvgError
pub enum SvgError {
    None
}

//ip Display for SvgError
impl std::fmt::Display for SvgError {
    //mp fmt - format error for display
    /// Display the error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SvgError")
    }

    //zz All done
}

//a SvgElement
//tp SvgElement
pub struct SvgElement {
    name : String,
    attributes : Vec<(String,String)>,
    contents : Vec<SvgElement>,
    characters : Option<String>,
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
        if !transform.translation.is_origin() { r.push_str(&format!("translate({:.4} {:.4}) ",transform.translation.x, transform.translation.y)); }
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

    //fp add_path
    pub fn add_path(&mut self, v:&Vec<Bezier>, closed:bool) {
        let mut r = String::new();
        r.push_str(&format!("M {}",pt_as_str(v[0].get_pt(0))));
        for b in v {
            match b {
                Bezier::Linear(_,p1) => r.push_str(&format!(" L {}",pt_as_str(p1))),
                Bezier::Quadratic(_,c,p1) => r.push_str(&format!(" Q {} {}",pt_as_str(c),pt_as_str(p1))),
                Bezier::Cubic(_,c0,c1,p1) => r.push_str(&format!(" C {} {} {}",pt_as_str(c0),pt_as_str(c1),pt_as_str(p1))),
            }
        }
        if closed { r.push_str(" z"); }
        self.add_attribute("d", &r);
    }

    //fp add_polygon_path
    pub fn add_polygon_path(&mut self, p:&Polygon, closed:bool) {
        let v = p.as_paths(Vec::new());
        self.add_path(&v, closed);
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

//a SvgElement iterator
//ti IterState
#[derive(Debug)]
enum IterState {
    PreDocument,
    PreElement,
    PreString,
    PreContent,
    PostContent,
    FindNextElement,
    DocumentEnd,
    Completed,
}

//tp ElementIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct ElementIter<'a> {
    state: IterState,
    elements: Vec<(&'a SvgElement, usize)>
}

//ip ElementIter
impl <'a> ElementIter<'a> {
    //fp new
    /// Create a new Svg element iterator
    pub fn new(e:&'a SvgElement) -> Self {
        let mut elements = Vec::new();
        elements.push((e,0));
        Self { state:IterState::PreDocument,
               elements,
        }
    }
}

//ip Iterator for ElementIter
const DEBUG_SVG_ITERATOR : bool = false;
impl <'a> Iterator for ElementIter<'a> {
    type Item = XmlEvent;
    fn next(&mut self) -> Option<Self::Item> {
        // Track the state for debugging
        if DEBUG_SVG_ITERATOR {
            let (ele,n) = self.elements.pop().unwrap();
            println!("State {:?} {}:{} [{}]",self.state,ele.name,n,ele.contents.len());
            self.elements.push((ele,n));
        }
        match self.state {
            IterState::PreDocument => {
                self.state = IterState::PreElement;
                Some(XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None })
            },
            IterState::PreElement => {
                let (ele,n) = self.elements.pop().unwrap();
                self.state = IterState::PreString;
                let name = Name::local(&ele.name).to_owned();
                let namespace = Namespace::empty();
                let mut attributes = Vec::new();
                for (n,v) in &ele.attributes {
                    let name = Name::local(n);
                    let attribute = Attribute::new(name, v).to_owned();
                    attributes.push(attribute);
                }
                self.elements.push((ele,n));
                Some(XmlEvent::StartElement{name, attributes, namespace})
            },
            IterState::PreString => {
                let (ele,n) = self.elements.pop().unwrap();
                self.state = IterState::PreContent;
                if let Some(s) = &ele.characters {
                    self.elements.push((ele,n));
                    Some(XmlEvent::Characters(s.to_string()))
                } else {
                    self.elements.push((ele,n));
                    self.next()
                }
            },
            IterState::PreContent => {
                let (ele,n) = self.elements.pop().unwrap();
                if n<ele.contents.len() {
                    let next_ele = &ele.contents[n];
                    self.elements.push((ele,n));
                    self.elements.push((next_ele,0));
                    self.state = IterState::PreElement;
                } else {
                    self.state = IterState::PostContent;
                    self.elements.push((ele,n));
                }
                self.next()
            },
            IterState::PostContent => {
                let (ele,n) = self.elements.pop().unwrap();
                self.state = IterState::FindNextElement;
                let name = Name::local(&ele.name).to_owned();
                self.elements.push((ele,n));
                Some(XmlEvent::EndElement{name})
            },
            IterState::FindNextElement => {
                if self.elements.len() > 1 {
                    let (_ele,_n) = self.elements.pop().unwrap();
                    let (ele,n) = self.elements.pop().unwrap();
                    if n+1<ele.contents.len() {
                        let next_ele = &ele.contents[n+1];
                        self.elements.push((ele,n+1));
                        self.elements.push((next_ele,0));
                        self.state = IterState::PreElement;
                    } else {
                        self.elements.push((ele,n+1));
                        self.state = IterState::PostContent;
                    }
                } else {
                    self.state = IterState::DocumentEnd;
                }
                self.next()
            },
            IterState::DocumentEnd => {
                self.state = IterState::Completed;
                Some(XmlEvent::EndDocument)
            },
            IterState::Completed => None,
        }
    }
}

//a Svg
//tp Svg
/// This structure is used to create SVG renderings of a `Diagram` It
/// should be constructed, and mutably borrowed by a diagram when it's
/// `generate_svg` method is invoked.
///
/// This method requires the `GenerateSvg` to be brought in to scope.
pub struct Svg {
    pub(super) width  : f64, // in mm
    pub(super) height : f64, // in mm
    pub(super) show_grid : bool,
    pub(super) show_layout : bool,
    pub(super) show_content_rectangles : bool,
    pub(super) display : bool,
    stack : Vec<SvgElement>,
}

//ip Svg
impl Svg {
    //fp new
    /// Create a new `Svg` instance, to render a `Diagram` into
    pub fn new() -> Self  {
        Self {
            width  : 297.,
            height : 210.,
            stack : Vec::new(),
            show_grid : false,
            show_layout : false,
            show_content_rectangles : false,
            display : false,
        }
    }

    //cp set_grid
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of a grid in to the SVG output
    pub fn set_grid(mut self, grid:bool) -> Self {
        self.show_grid = grid;
        self
    }
    
    //cp set_layout
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of lines indicating the `Layout`
    /// grids.
    pub fn set_layout(mut self, layout:bool) -> Self {
        self.show_layout = layout;
        self
    }
    
    //cp set_display
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the display to stdout of the Svg element hierarchy,
    /// once created from the diagram.
    pub fn set_display(mut self, display:bool) -> Self {
        self.display = display;
        self
    }
    
    //cp set_content_rectangles
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of a grid in to the SVG output
    pub fn set_content_rectangles(mut self, show:bool) -> Self {
        self.show_content_rectangles = show;
        self
    }
    
    //mp push_element
    pub(super) fn push_element(&mut self, e:SvgElement) {
        self.stack.push(e);
    }
    //mp pop_element
    pub(super) fn pop_element(&mut self) -> SvgElement {
        self.stack.pop().unwrap()
    }
    //mp add_subelement
    pub(super) fn add_subelement(&mut self, e:SvgElement) {
        let n = self.stack.len();
        self.stack[n-1].contents.push(e);
    }

    //mi add_grid
    /// Add a grid to the SVG file with given region, spacing, line
    /// width and color
    fn add_grid(&mut self, min:f64, max:f64, spacing:f64, line_width:f64, color:&str) {
        let length = max-min;
        let mut rx = String::new();
        let mut ry = String::new();
        let mut coord = min;
        while coord <= max {
            rx.push_str(&format!("M {},{} v {} ",coord,min,length));
            ry.push_str(&format!("M {},{} h {} ",min,coord,length));
            coord += spacing;
        }
        rx.push_str(&ry);
        let mut grid = SvgElement::new("path");
        grid.add_attribute("fill","None");
        grid.add_attribute("stroke",color);
        grid.add_attribute("stroke-width",&format!("{}",line_width));
        grid.add_attribute("d",&rx);
        self.add_subelement(grid);
    }
    
    //mp generate_layout_recoved_svg
    pub(super) fn generate_layout_recoved_svg(&mut self, layout_record:&Option<LayoutRecord> ) -> Result<(), SvgError> {
        if self.show_layout {
            if let Some(lr) = layout_record.as_ref() {
                lr.generate_svg( self )?;
            }
        }
        Ok(())
    }
    
    //mp iter_events
    /// Iterate over all the XML events the Svg would generate if it
    /// were an SVG file being read in by xml-rs
    ///
    /// This permits the SVG to be read by an XML reader, or written
    /// using xml-rs to convert reader XmlEvents to writer XmlEvents.
    pub fn iter_events<'a>(&'a self) -> ElementIter<'a> {
        ElementIter::new(&self.stack[0])
    }
    
    //zz All done
}

//a GenerateSvg, GenerateSvgElement
//pt GenerateSvgElement
pub trait GenerateSvgElement {
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError>;
}

//ip GenerateSvgElement for ElementContent
impl <'a> GenerateSvgElement for ElementContent<'a> {
    //mp generate_svg
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError> {
        match self {
            ElementContent::Path(ref s)  => { s.generate_svg(svg, header) },
            ElementContent::Shape(ref s) => { s.generate_svg(svg, header) },
            ElementContent::Text(ref t)  => { t.generate_svg(svg, header) },
            ElementContent::Group(ref g) => { g.generate_svg(svg, header) },
            ElementContent::Use(ref g)   => { g.generate_svg(svg, header) },
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
    fn generate_svg(&self, _svg:&mut Svg) -> Result<(), SvgError> { Ok(()) }
    //mp svg_add_transform
    /// This method is used internally
    fn svg_add_transform(&self, _ele:&mut SvgElement) {}
}

//ip GenerateSvg for ElementHeader
impl <'a> GenerateSvg for ElementHeader<'a> {
    fn svg_add_transform(&self, ele:&mut SvgElement) {
        if let Some(id) = self.id_name.as_ref() {
            ele.add_attribute("id", id);
        }
        match self.layout_box.borrow_content_transform() {
            Some(transform) => { ele.add_transform(transform); },
            _ => (),
        }
    }
}

//ip GenerateSvg for Element
impl <'a> GenerateSvg for Element<'a> {
    fn generate_svg(&self, svg:&mut Svg) -> Result<(), SvgError> {
        if self.header.layout.bg.is_some() {
            let mut ele = SvgElement::new("path");
            ele.add_attribute("stroke","None");
            ele.add_color("fill",&self.header.layout.bg.unwrap());
            ele.add_polygon_path(self.header.layout_box.get_border_shape().unwrap(), true);
            svg.add_subelement(ele);
        }
        if svg.show_content_rectangles {
            let rect = self.header.layout_box.get_content_rectangle();
            let (c,w,h) = rect.get_cwh();
            let mut ele = SvgElement::new("rect");
            // ele.add_attribute("id", &format!("{}.content_rect",self.header.borrow_id()));
            ele.add_attribute("fill", "#40ff8080");
            ele.add_size("x", c.x-w/2.);
            ele.add_size("y", c.y-h/2.);
            ele.add_size("width", w);
            ele.add_size("height", h);
            self.header.svg_add_transform(&mut ele);
            svg.add_subelement(ele);
        }
        self.content.generate_svg(svg, &self.header)?;
        if self.header.layout.border_color.is_some() {
            let mut ele = SvgElement::new("path");
            ele.add_color("stroke",&self.header.layout.border_color.unwrap());
            ele.add_size("stroke-width",self.header.layout.border_width);
            ele.add_attribute("fill","None");
            ele.add_polygon_path(self.header.layout_box.get_border_shape().unwrap(), true);
            svg.add_subelement(ele);
        }
        Ok(())
    }
}

//ip GenerateSvg for LayoutRecord
impl GenerateSvg for LayoutRecord {
    fn generate_svg(&self, svg:&mut Svg) -> Result<(), SvgError> {
        match &self.grid_positions {
            Some( (grid_x, grid_y) ) => {
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
                    let x1 = grid_x[xn-1].1;
                    let y0 = grid_y[0].1;
                    let y1 = grid_y[yn-1].1;
                    for (_,x) in grid_x {
                        rx.push_str(&format!("M {:.4},{:.4} v {:.4} ",x,y0,y1-y0));
                    }
                    for (_,y) in grid_y {
                        ry.push_str(&format!("M {:.4},{:.4} h {:.4} ",x0,y,x1-x0));
                    }
                    rx.push_str(&ry);
                    let mut grid = SvgElement::new("path");
                    grid.add_attribute("fill","None");
                    grid.add_attribute("stroke",color);
                    grid.add_attribute("stroke-width",&format!("{:.4}",line_width));
                    grid.add_attribute("d",&rx);
                    svg.add_subelement(grid);
                    ()
                }
            },
            _ => (),
        }
        Ok(())
    }
}

//ip GenerateSvg for Diagram
impl <'a> GenerateSvg for Diagram<'a> {
    fn generate_svg(&self, svg:&mut Svg) -> Result<(), SvgError> {
        let mut ele = SvgElement::new("svg");
        ele.add_attribute("xmlns:svg","http://www.w3.org/2000/svg");
        ele.add_attribute("xmlns","http://www.w3.org/2000/svg");
        ele.add_attribute("version","1.1");
        ele.add_attribute("width" ,&format!("{}mm", svg.width));
        ele.add_attribute("height",&format!("{}mm", svg.height));
        ele.add_attribute("viewBox",
                          &format!("{} {} {} {}",
                                   self.contents.content_bbox.x0,
                                   self.contents.content_bbox.y0,
                                   self.contents.content_bbox.x1-self.contents.content_bbox.x0,
                                   self.contents.content_bbox.y1-self.contents.content_bbox.y0,
                                   ) );
        svg.push_element(ele);
        let mut ele = SvgElement::new("g");
        ele.add_transform(&self.contents.content_transform);
        svg.push_element(ele);

        if svg.show_grid {
            svg.add_grid(-200.,200.,10.,0.5,"grey");
            svg.add_grid(-100.,100.,10.,1.0,"blue");
        }

        for e in self.iter_elements() {
            e.generate_svg( svg )?;
        }

        if svg.show_layout {
            if let Some(lr) = &self.layout_record {
                lr.generate_svg( svg )?;
            }
        }

        let ele = svg.pop_element();
        if svg.display {
            ele.display(0);
        }
        svg.add_subelement(ele);
        Ok(())
    }
}
