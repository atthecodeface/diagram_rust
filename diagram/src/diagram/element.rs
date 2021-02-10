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

@file    mod.rs
@brief   Diagram module
 */

//a Imports
use crate::Diagram;
use crate::DiagramDescriptor;
use crate::{Layout, LayoutBox};
use crate::{Rectangle, Polygon, Point};
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, RrcStylableNode};
use super::types::*;
    
//a Element types
//tp Group - an Element that contains just other Elements
#[derive(Debug)]
pub struct Group<'a> {
    // requires no styles
    pub content : Vec<Element<'a>>
}

//ti Group
impl <'a> Group<'a> {
    //fp new
    pub fn new(name_values:Vec<(String,String)>) -> Result<Self,ValueError> {
        Ok( Self {
            content:Vec::new(),
        } )
    }

    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }

    //mp add_element
    pub fn add_element(&mut self, element:Element<'a>) -> () {
        self.content.push(element);
    }
    
    //mp style
    pub fn style(&mut self, header:&ElementHeader) -> Result<(),()> {
        for e in self.content.iter_mut() {
            e.style()?;
        }
        Ok(())
    }

    //mp get_desired_geometry
    pub fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        let mut rect = Rectangle::none();
        for e in self.content.iter_mut() {
            rect = rect.union(&e.set_layout_properties(layout));
        }
        rect
    }

    //fp apply_placement
    pub fn apply_placement(&mut self, layout:&Layout) {
        for e in self.content.iter_mut() {
            e.apply_placement(layout);
        }
    }
    
    //zz All done
}

//tp Text - an Element that contains text
#[derive(Debug)]
pub struct Text {
    text    : Vec<String>,
}
impl Text {
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = ElementHeader::get_descriptor(nts);
        desc.borrow_mut().add_styles(nts, vec!["fill", "font", "fontsize", "fontweight", "fontstyle"]);
        desc
    }
}

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

//ti Shape
impl Shape {
    //fp new
    pub fn new(name_values:Vec<(String,String)>) -> Result<Self,ValueError> {
        let polygon = Polygon::new(0, 0.);
        Ok( Self {
            polygon,
            stroke_width:0.,
            stroke : None,
            fill : None,
        } )
    }

    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = ElementHeader::get_descriptor(nts);
        desc.borrow_mut().add_styles(nts, vec!["fill", "stroke", "strokewidth", "round", "markers", "vertices", "stellate", "width", "height"]);
        desc
    }

    //mp style
    pub fn style(&mut self, header:&ElementHeader) -> Result<(),()> {
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
    pub fn get_desired_geometry(&mut self) -> Rectangle {
        let rect = self.polygon.get_bbox();
        rect.enlarge(self.stroke_width)
    }

    //zz All done
}

//tp Use - an Element that is a reference to a group or other element
#[derive(Debug)]
pub struct Use {
    // has Transform - to put it somewhere!
    id_ref  : String,
}

//ti Use
impl Use {
    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }
}

//a ElementContent - enumerated union of the above
//tp ElementContent 
#[derive(Debug)]
pub enum ElementContent<'a> {
    Group(Group<'a>),
    Text(Text),
    Shape(Shape),
    Use(Use), // use of a definition
}

//ti ElementContent
impl <'a> ElementContent<'a> {
    //fp add_element
    pub fn add_element(&mut self, element:Element<'a>) {
        match self {
            ElementContent::Group(ref mut g) => { g.add_element(element); },
            _ => (),
        }
    }

    //mp style
    pub fn style(&mut self, header:&ElementHeader) -> Result<(),()> {
        match self {
            ElementContent::Shape(ref mut s) => { s.style(header) },
            ElementContent::Group(ref mut g) => { g.style(header) },
            _ => Ok(())
        }
    }

    //mp get_desired_geometry
    pub fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        match self {
            ElementContent::Shape(ref mut s) => { s.get_desired_geometry() },
            ElementContent::Group(ref mut g) => { g.get_desired_geometry(layout) },
            _ => Rectangle::new(0.,0.,10.,10.),
        }
    }

    //fp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout:&Layout) {
        match self {
            ElementContent::Group(ref mut g) => { g.apply_placement(layout) },
            _ => (),
        }
    }
    
    //zz All done
}

//a ElementHeader and Element
//tp LayoutPlacement
#[derive(Debug)]
enum LayoutPlacement {
    None,
    Place(Point),
    Grid(isize,isize,usize,usize),
}

//tp ElementLayout
#[derive(Debug)]
pub struct ElementLayout {
    placement : LayoutPlacement,
    ref_pt    : Option<Point>,
    pub scale     : f64,
    pub rotation  : f64,
    pub translate : Point,
    pub border_width : f64,
    pub border_round : f64,
    pub border_color : Option<(f64,f64,f64)>,
    pub bg           : Option<(f64,f64,f64)>,
    pub pad          : Option<(f64,f64,f64,f64)>,
    pub margin       : Option<(f64,f64,f64,f64)>,
}
impl ElementLayout {
    //fp new
    pub fn new() -> Self {
        Self { placement:LayoutPlacement::None,
               ref_pt : None,
               scale:1.,
               rotation:0.,
               translate : Point::origin(),
               border_width : 0.,
               border_round : 0.,
               border_color : None,
               bg : None,
               pad : None,
               margin : None,
        }
    }
    //fp set_grid
    pub fn set_grid(&mut self, sx:isize, sy:isize, nx:usize, ny:usize) {
        self.placement = LayoutPlacement::Grid(sx,sy,nx,ny);
    }
    //fp set_place
    pub fn set_place(&mut self, x:f64, y:f64) {
        self.placement = LayoutPlacement::Place(Point::new(x,y));
    }
    //zz All done
}

//tp ElementHeader
#[derive(Debug)]
pub struct ElementHeader<'a> {
    stylable         : RrcStylableNode<'a, StyleValue>,
    pub layout_box   : LayoutBox,
    pub layout       : ElementLayout,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    //fp new
    pub fn new<'b> (styles:&RrcStyleDescriptor, name_values:Vec<(String,String)>) -> Result<(ElementHeader<'b>, Vec<(String,String)>), ValueError> {
        // let mut unused_nv = Vec::new();
        let unused_nv = Vec::new();
        let stylable = StylableNode::new(None, "node_type", styles, vec![]);
        for (name,value) in &name_values {
            stylable.borrow_mut().add_name_value(name, value);
        }
        let layout_box = LayoutBox::new();
        let hdr = ElementHeader{ stylable, layout_box, layout:ElementLayout::new() };
        Ok((hdr, unused_nv))
    }

    //mp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = StyleDescriptor::new();
        desc.borrow_mut().add_styles(nts, vec!["bbox", "grid", "place", "rotate", "scale", "translate", "pad", "margin", "border", "bg", "bordercolor", "borderround"]);
        desc
    }

    //mp get_opt_style_value_of_name
    pub fn get_opt_style_value_of_name(&self, name:&str) -> Option<StyleValue> {
        let stylable = self.stylable.borrow();
        stylable.get_style_value_of_name(name).map(|a| a.clone())
    }

    //mp get_style_rgb_of_name
    pub fn get_style_rgb_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::rgb(None),
            Some(value) => value,
        }
    }

    //mp get_style_ints_of_name
    pub fn get_style_ints_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::int_array(),
            Some(value) => value,
        }
    }

    //mp get_style_floats_of_name
    pub fn get_style_floats_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::float_array(),
            Some(value) => value,
        }
    }

    //mp get_style_of_name_float
    pub fn get_style_of_name_float(&self, name:&str, default:Option<f64>) -> Option<f64> {
        match self.get_opt_style_value_of_name(name) {
            None => default,
            Some(value) => value.as_float(default),
        }
    }

    //mp get_style_of_name_int
    pub fn get_style_of_name_int(&self, name:&str, default:Option<isize>) -> Option<isize> {
        match self.get_opt_style_value_of_name(name) {
            None => default,
            Some(value) => value.as_int(default),
        }
    }

    //mp style
    pub fn style(&mut self) -> Result<(),()> {
        if let Some(v) = self.get_style_of_name_float("border",None) {
            self.layout.border_width = v;
        }
        if let Some(v) = self.get_style_of_name_float("scale",None) {
            self.layout.scale = v;
        }
        if let Some(v) = self.get_style_of_name_float("rotate",None) {
            self.layout.rotation = v;
        }
        if let Some(v) = self.get_style_rgb_of_name("bordercolor").as_floats(None) {
            self.layout.border_color = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = self.get_style_rgb_of_name("bg").as_floats(None) {
            self.layout.bg = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = self.get_style_floats_of_name("margin").as_floats(None) {
            self.layout.margin = Some( (v[0], v[1], v[2], v[3]) );
        }
        if let Some(v) = self.get_style_floats_of_name("pad").as_floats(None) {
            self.layout.pad = Some( (v[0], v[1], v[2], v[3]) );
        }
        /*
        if let Some(v) = stylable.get_style_value_of_name("translate").unwrap().as_floats(None) {
        }
         */
        if let Some( (sx,sy,nx,ny) ) = {
            match self.get_style_ints_of_name("grid").as_ints(None) {
                Some(g) => {
                    match g.len() {
                        0 => None,
                        1 => Some( (g[0],g[0],1,1) ),
                        2 => Some( (g[0],g[1],1,1) ),
                        3 => Some( (g[0],g[1],g[2],1) ),
                        _ => Some( (g[0],g[1],g[2],g[3]) ),
                    }
                },
                _ => None,
            }
        } {
            self.layout.set_grid(sx,sy,nx as usize, ny as usize);
        }
        if let Some( (x,y) ) = {
            match self.get_style_ints_of_name("place").as_floats(None) {
                Some(g) => {
                    match g.len() {
                        0 => None,
                        1 => Some( (g[0],g[0]) ),
                        _ => Some( (g[0], g[1]) ),
                    }
                },
                _ => None,
            }
        } {
            self.layout.set_place(x,y);
        }
        Ok(())
    }
    
    //mp set_layout_properties
    /// By this point layout_box has had its desired_geometry set
    pub fn set_layout_properties(&mut self, layout:&mut Layout, content_desired:Rectangle) -> Rectangle{
        self.layout_box.set_content_geometry(content_desired, Point::origin(), self.layout.scale, self.layout.rotation);
        self.layout_box.set_border_width(self.layout.border_width);
        self.layout_box.set_margin(&self.layout.margin);
        self.layout_box.set_padding(&self.layout.pad);
        let bbox = self.layout_box.get_desired_bbox();
        match self.layout.placement {
            LayoutPlacement::None => bbox,
            LayoutPlacement::Grid(sx,sy,nx,ny) => {
                layout.add_grid_element( (sx,sy), (nx,ny), (bbox.width(), bbox.height() ));
                Rectangle::none()
            },
            LayoutPlacement::Place(pt) => {
                layout.add_placed_element( &pt, &self.layout.ref_pt, &bbox );
                Rectangle::none()
            },
        }
    }
                           
    //fp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout:&Layout) {
        let rect = {
            match self.layout.placement {
                LayoutPlacement::None              => self.layout_box.get_desired_bbox(),
                LayoutPlacement::Grid(sx,sy,nx,ny) => layout.get_grid_rectangle( (sx,sy), (nx,ny) ),
                LayoutPlacement::Place(pt)         => layout.get_placed_rectangle( &pt, &self.layout.ref_pt ),
            }
        };
        self.layout_box.layout_within_rectangle(rect);
    }
   
    //zz All done
}


//tp Element
#[derive(Debug)]
pub struct Element<'a> {
    pub header  : ElementHeader<'a>,
    pub content : ElementContent<'a>,
}

//ti Element
impl <'a> Element <'a> {
    //mp has_id
    pub fn has_id(&self, name:&str) -> bool {
        self.header.stylable.borrow().has_id(name)
    }

    //fp new_shape
    pub fn new_shape(descriptor:&DiagramDescriptor, name:&str, name_values:Vec<(String,String)>) -> Result<Self, ValueError> {
        let styles = descriptor.get("shape").unwrap();
        let (header, name_values) = ElementHeader::new(&styles, name_values)?;
        Ok(Self { header, content:ElementContent::Shape(Shape::new(name_values)?) })
    }

    //fp new_group
    pub fn new_group(descriptor:&DiagramDescriptor, name:&str, name_values:Vec<(String,String)>) -> Result<Self, ValueError> {
        let styles = descriptor.get("group").unwrap();
        let (header, name_values) = ElementHeader::new(&styles, name_values)?;
        Ok(Self { header, content:ElementContent::Group(Group::new(name_values)?) })
    }

    //fp add_element
    pub fn add_element(&mut self, element:Element<'a>) {
        self.content.add_element(element);
    }

    //fp value_of_name
    pub fn value_of_name(name_values:Vec<(String,String)>, name:&str, mut value:StyleValue) -> Result<StyleValue,ValueError> {
        for (n,v) in name_values {
            if n==name {
                value.from_string(&v)?;
            }
        }
        Ok(value)
    }

    //mp style
    pub fn style(&mut self) -> Result<(),()> {
        self.header.style()?;
        self.content.style(&self.header)?;
        Ok(())
    }
        
    //mp set_layout_properties
    /// This method is invoked to set the `Layout` of this element, by
    /// finding its desired geometry and any placement or grid
    /// constraints
    ///
    /// If the element has a specified layout then it should have a 'none' desired geometry
    /// if it is unplaced then its geometry should be its bounding box
    ///
    /// For normal elements (such as a shape) this requires finding
    /// the desired geometry, reporting this to the `LayoutBox`, and
    /// using the `LayoutBox` data to generate the boxed desired
    /// geometry, which is then added to the `Layout` element as a
    /// place or grid desire.
    pub fn set_layout_properties(&mut self, layout:&mut Layout) -> Rectangle {
        let content_rect = self.content.get_desired_geometry(layout);
        self.header.set_layout_properties(layout, content_rect)
    }

    //fp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout:&Layout) {
        self.header.apply_placement(layout);
        self.content.apply_placement(layout);
    }

    //zz All done
}

