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

@file    element.rs
@brief   Diagram elements
 */

//a Imports
use crate::DiagramDescriptor;
use crate::{Layout, LayoutBox};
use crate::{Rectangle, Point};
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, RrcStylableNode};
use super::elements::{Group, Shape, Text, Use};
use super::types::*;
    
//a DiagramElement trait
pub trait DiagramElementContent <'a, 'b> : Sized+std::fmt::Debug {
    //fp new
    /// Create a new element of the given name
    fn new(header:&ElementHeader<'a>, name:&str ) -> Result<Self,ElementError>;

    //fp clone
    /// Clone element given clone of header within scope
    ///
    /// This method is only invoke prior to styling, so often is the same as `new`
    fn clone(&self, header:&ElementHeader<'a>, scope:&ElementScope<'a, 'b> ) -> Result<Self,ElementError>;
    
    //mp uniquify
    /// Sets internal self.content to a clone of a resolved definition
    ///
    /// The id_ref should identify an element in `scope`.
    /// The header may have to be cloned - it has layout information etc, and indeed any of its
    /// name/values override those of
    fn uniquify(&mut self, header:&ElementHeader<'a>, scope:&ElementScope<'a,'b>) -> Result<bool, ElementError> {
        Ok(false)
    }

    //fp get_descriptor
    /// Get the style descriptor for this element when referenced by the name
    fn get_descriptor(nts:&StyleSet, _name:&str) -> RrcStyleDescriptor;

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, _descriptor:&DiagramDescriptor, _header:&ElementHeader) -> Result<(),ElementError> {
        Ok(())
    }
    
    //mp get_desired_geometry
    /// Get the desired bounding box for the element; the layout is
    /// required if it is to be passed in to the contents (element
    /// header + element content) -- by setting their layout
    /// properties -- but does not effect the *content* of a single
    /// element
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        Rectangle::none()
    }

    //fp apply_placement
    /// Apply the layout to the element; this may cause contents to
    /// then get laid out, etc Nothing needs to be done - the layout
    /// is available when the element is visualized
    ///
    /// The rectangle supplied is the content-space rectangle derived
    /// for the content
    fn apply_placement(&mut self, _layout:&Layout, _rect:&Rectangle) {
        // No need to do anything
    }

    //zz All done
}

//a ElementError
//tp ElementError
pub enum ElementError {
    UnknownId(String,String),
    Error(String,String),
}

//ii ElementError
impl ElementError {
    //fp unknown_id
    pub fn unknown_id(hdr:&ElementHeader, name:&str) -> Self {
        Self::UnknownId(hdr.borrow_id().to_string(), name.to_string())
    }
    //fp of_string
    pub fn of_string(hdr:&ElementHeader, s:&str) -> Self {
        Self::Error(hdr.borrow_id().to_string(), s.to_string())
    }
    //mi of_result
    pub fn of_result<V,E:std::fmt::Display>(hdr:&ElementHeader, result:Result<V,E>) -> Result<V,ElementError> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(ElementError::Error(hdr.borrow_id().to_string(), e.to_string()))
        }
    }

    //zz All done
}

//ip Display for ElementError
impl std::fmt::Display for ElementError {
    //mp fmt - format error for display
    /// Display the error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementError::UnknownId(id,s) => write!(f, "Element id '{}': Unknown id reference '{}'", id, s),
            ElementError::Error(id,s) => write!(f, "Element id '{}': {}", id, s),
        }
    }

    //zz All done
}

//a ElementScope<'a> - 'a is the lifetime of the definition elements
//tp ElementScope
pub struct ElementScope<'a, 'b> {
    id_prefix   : String,
    definitions : &'b Vec<Element<'a>>,
}

//ip ElementScope
impl <'a, 'b> ElementScope<'a, 'b> {
    pub fn new(id_prefix:&str, definitions: &'b Vec<Element<'a>>) -> Self {
        let id_prefix = id_prefix.to_string();
        Self { id_prefix, definitions, }
    }
    pub fn new_subscope<'c>(&'c self, header:&ElementHeader<'a>, name:&str) -> Result<(ElementScope<'a, 'c>, &'c Element<'a>), ElementError> {
        let n = self.definitions.len();
        let mut index = None;
        for i in 0..n {
            if self.definitions[i].has_id(name) {
                index = Some(i);
            }
        }
        if let Some(index) = index {
            let mut id_prefix = self.id_prefix.clone();
            id_prefix.push_str(".");
            id_prefix.push_str(header.borrow_id());
            let definitions = self.definitions;
            let element     = &self.definitions[index];
            Ok((Self { id_prefix, definitions}, element))
        } else {
            Err(ElementError::unknown_id(header, name))
        }
            
    }
}
    
//a ElementContent - enumerated union of the above
//tp ElementContent 
#[derive(Debug)]
pub enum ElementContent<'a> {
    Group(Group<'a>),
    Text(Text),
    Shape(Shape),
    Use(Use<'a>), // use of a definition
}

//ti ElementContent
impl <'a> ElementContent<'a> {
    //fp new
    pub fn new(header:&ElementHeader<'a>, name:&str) -> Result<Self, ElementError> {
        match name {
            "group" => Ok(Self::Group(Group::new(&header, name)?)),
            "shape" => Ok(Self::Shape(Shape::new(&header, name)?)),
            "text"  => Ok(Self::Text(Text::new(&header, name)?)),
            "use"   => Ok(Self::Use(Use::new(&header, name)?)),
            _ => ElementError::of_result(&header,Err(format!("Bug - bad element name {}",name))),
        }
    }

    //mp uniquify
    /// Generates a *replacement* Content if required.
    ///
    /// This is for a 'use' content, which should have an id_ref that
    /// identifies and element in `scope`.  This will return Ok(true),
    /// if it uniquifies the use content reference; in doing so it
    /// must clone the relevant element and push its header
    /// name/values down in to the cloned content header.
    ///
    /// If the immediate element content, then recurse through any
    /// subcontent, and return Ok(false)
    pub fn uniquify<'b, 'c>(&'c mut self, header:&ElementHeader<'a>, scope:&ElementScope<'a, 'b>) -> Result<bool, ElementError> {
         match self {
            Self::Use(ref mut c)   => c.uniquify(header, scope),
            Self::Group(ref mut c) => c.uniquify(header, scope),
            _ => Ok(false),
        }
    }

    //mp clone
    pub fn clone<'b>(&self, header:&ElementHeader<'a>, scope:&ElementScope<'a,'b>) -> Result<Self, ElementError> {
        match self {
            Self::Group(ref c) => Ok(Self::Group(ElementError::of_result(&header,c.clone(header, scope))?)),
            Self::Shape(ref c) => Ok(Self::Shape(ElementError::of_result(&header,c.clone(header, scope))?)),
            Self::Text(ref c)  => Ok(Self::Text(ElementError::of_result(&header,c.clone(header, scope))?)),
            Self::Use(ref c)   => Ok(Self::Use(ElementError::of_result(&header,c.clone(header, scope))?)),
        }
    }

    //mp add_element
    pub fn add_element(&mut self, element:Element<'a>) {
        match self {
            Self::Group(ref mut c) => { c.add_element(element); },
            _ => (),
        }
    }

    //mp add_string
    pub fn add_string(&mut self, header:&ElementHeader, s:&str) -> Result<(),ElementError> {
        match self {
            Self::Text(ref mut c) => ElementError::of_result( header, c.add_string(s) ),
            Self::Use(ref mut c)  => ElementError::of_result( header, c.add_string(s) ),
            _ => Ok(()), // could error - bug in code
        }
    }

    //mp style
    pub fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        match self {
            Self::Shape(ref mut s) => { s.style(descriptor, header) },
            Self::Group(ref mut g) => { g.style(descriptor, header) },
            Self::Text(ref mut t)  => { t.style(descriptor, header) },
            Self::Use(ref mut t)   => { t.style(descriptor, header) },
            _ => Ok(())
        }
    }

    //mp get_desired_geometry
    pub fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        match self {
            Self::Shape(ref mut s) => { s.get_desired_geometry(layout) },
            Self::Group(ref mut g) => { g.get_desired_geometry(layout) },
            Self::Text(ref mut t)  => { t.get_desired_geometry(layout) },
            Self::Use(ref mut t)   => { t.get_desired_geometry(layout) },
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
    pub fn apply_placement(&mut self, layout:&Layout, rect:&Rectangle) {
        match self {
            Self::Group(ref mut g) => { g.apply_placement(layout, rect) },
            Self::Use(ref mut g)   => { g.apply_placement(layout, rect) },
            _ => (),
        }
    }
    
    //zz All done
}

//a ElementHeader and ElementLayout
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

//ip ElementLayout
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
    pub id_name      : Option<String>, // replicated from stylable
    pub layout_box   : LayoutBox,
    pub layout       : ElementLayout,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    //fp new
    pub fn new<'b> (descriptor:&DiagramDescriptor, name:&str, name_values:Vec<(String,String)>) -> Result<ElementHeader<'b>, ElementError> {
        if let Some(styles) = descriptor.get(name) { // &RrcStyleDescriptor
            let stylable = StylableNode::new(None, name, &styles, vec![]);
            for (name,value) in &name_values {
                stylable.borrow_mut().add_name_value(name, value);
            }
            let id_name = stylable.borrow().borrow_id().map(|s| s.to_string());
            let layout_box = LayoutBox::new();
            let layout = ElementLayout::new();
            let hdr = ElementHeader{ stylable, id_name, layout_box, layout };
            Ok(hdr)
        } else {
            Err(ElementError::Error("".to_string(),format!("Bug - unknown element descriptor {}",name)))
        }
    }

    //fp clone
    pub fn clone(&self, scope:&ElementScope) -> ElementHeader<'a> {
        let mut id_name = scope.id_prefix.clone();
        id_name.push_str(".");
        id_name.push_str(self.borrow_id());
        let id_name = Some(id_name);
        let stylable = self.stylable.clone(); // WRONG!!
        let layout_box = LayoutBox::new();
        let layout = ElementLayout::new();
        ElementHeader{ stylable, id_name, layout_box, layout }
    }

    //mp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = StyleDescriptor::new();
        desc.borrow_mut().add_styles(nts, vec!["bbox", "grid", "place", "rotate", "scale", "translate", "pad", "margin", "border", "bg", "bordercolor", "borderround"]);
        desc
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        match &self.id_name {
            None => "",
            Some(s) => s,
        }
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

    //mp get_style_of_name_string
    pub fn get_style_of_name_string(&self, name:&str) -> Option<String> {
        match self.get_opt_style_value_of_name(name) {
            None        => None,
            Some(value) => value.as_string(),
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
    pub fn style(&mut self) -> Result<(),ElementError> {
        if let Some(v) = self.get_style_of_name_float("border",None) {
            self.layout.border_width = v;
        }
        if let Some(v) = self.get_style_of_name_float("borderround",None) {
            self.layout.border_round = v;
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
        self.layout_box.set_border_round(self.layout.border_round);
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
                           
    //mp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout:&Layout) -> Rectangle {
        let rect = {
            match self.layout.placement {
                LayoutPlacement::None              => self.layout_box.get_desired_bbox(),
                LayoutPlacement::Grid(sx,sy,nx,ny) => layout.get_grid_rectangle( (sx,sy), (nx,ny) ),
                LayoutPlacement::Place(pt)         => layout.get_placed_rectangle( &pt, &self.layout.ref_pt ),
            }
        };
        println!("Laying out {:?} => {}",self.layout,rect);
        self.layout_box.layout_within_rectangle(rect);
        self.layout_box.get_content_rectangle()
    }
   
    //zz All done
}


//a Element
//tp Element
#[derive(Debug)]
pub struct Element<'a> {
    pub header  : ElementHeader<'a>,
    pub content : ElementContent<'a>,
}

//ip Element
impl <'a> Element <'a> {
    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        self.header.borrow_id()
    }

    //mp has_id
    pub fn has_id(&self, name:&str) -> bool {
        self.header.stylable.borrow().has_id(name)
    }

    //mp add_content_descriptors {
    pub fn add_content_descriptors(descriptor:&mut DiagramDescriptor) {
        descriptor.add("use",   |s,n| Use::get_descriptor(s,n) );
        descriptor.add("group", |s,n| Group::get_descriptor(s,n) );
        descriptor.add("text",  |s,n| Text::get_descriptor(s,n) );
        descriptor.add("shape", |s,n| Shape::get_descriptor(s,n) );
    }

    //fp new
    pub fn new(descriptor:&DiagramDescriptor, name:&str, name_values:Vec<(String,String)>) -> Result<Self, ElementError> {
        let header  = ElementHeader::new(descriptor, name, name_values)?;
        let content = ElementContent::new(&header,name)?;
        Ok( Self { header, content })
    }

    //mp uniquify
    /// Generates a *replacement* if the content requires it
    pub fn uniquify<'b>(&mut self, scope:&ElementScope<'a, 'b>) -> Result<(), ElementError> {
        if self.content.uniquify(&self.header, scope)? {
            // Updated the content, so uniquify again
            self.uniquify(scope)
        } else {
            Ok(())
        }
    }

    //mp clone
    pub fn clone<'b>(&self, scope:&ElementScope<'a, 'b>) -> Result<Element<'a>, ElementError> {
        let header = self.header.clone(scope);
        let content = self.content.clone(&header, scope)?;
        Ok( Self { header, content })
    }

    //fp add_string
    pub fn add_string(&mut self, s:&str) -> Result<(),ElementError> {
        self.content.add_string(&self.header, s)
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
    pub fn style(&mut self, descriptor:&DiagramDescriptor) -> Result<(),ElementError> {
        println!("Style  {} ", self.header.borrow_id());
        self.header.style()?;
        self.content.style(descriptor, &self.header)?;
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
        let content_rect = self.header.apply_placement(layout);
        self.content.apply_placement(layout, &content_rect);
    }

    //zz All done
}

