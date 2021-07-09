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

//a Constants
const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use geo_nd::Vector;
use geometry::{Rectangle, Point};
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, Tree};
use crate::constants::attributes as at;
use crate::constants::elements   as el;
use crate::DiagramDescriptor;
use crate::{Layout, LayoutBox};
pub use super::elements::{Group, Shape, Path, Text, Use};
use super::types::*;
use super::DiagramElementContent;

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
#[derive(Debug)]
pub struct ElementScope<'a, 'b> {
    id_prefix   : String,
    definitions : &'b Vec<Element<'a>>,
    pub depth   : usize,
}

//ip ElementScope
impl <'a, 'b> ElementScope<'a, 'b> {
    //fp new
    pub fn new(id_prefix:&str, definitions: &'b Vec<Element<'a>>) -> Self {
        let id_prefix = id_prefix.to_string();
        Self { id_prefix, definitions, depth:0, }
    }
    //mp new_subscope
    pub fn new_subscope<'c>(&'c self, header:&ElementHeader<'a>, name:&str, depth:usize) -> Result<(ElementScope<'a, 'c>, &'c Element<'a>), ElementError> {
        if depth > 50 {
            Err(ElementError::of_string(header, &format!("Maximum scope depth of {} reached - recursive Use?", depth)))
        } else {
            let n = self.definitions.len();
            let mut index = None;
            for i in 0..n {
                if self.definitions[i].has_id(name) {
                    index = Some(i);
                }
            }
            if let Some(index) = index {
                let mut id_prefix = self.id_prefix.clone();
                id_prefix.push_str(header.borrow_id());
                id_prefix.push_str(".");
                id_prefix.push_str(name);
                // println!("New scope with prefix {}", id_prefix);
                let definitions = self.definitions;
                let element     = &self.definitions[index];
                Ok((Self { id_prefix, definitions, depth}, element))
            } else {
                Err(ElementError::unknown_id(header, name))
            }
        }
    }
}

//a ElementContent - enumerated union of the above
//tp ElementContent
#[derive(Debug)]
pub enum ElementContent<'a> {
    /// Group is used for Marker, Layout and Group
    Group(Group<'a>),
    /// Text is used for all text boxes
    Text(Text),
    /// Shape is used for circles, polygons, rectangles
    Shape(Shape),
    /// Path is used for custom shapes
    Path(Path),
    /// Use describes a reference to a defined element
    Use(Use<'a>), // use of a definition
}

//ti ElementContent
impl <'a> ElementContent<'a> {
    //fp new
    pub fn new(header:&ElementHeader<'a>, name:&str) -> Result<Self, ElementError> {
        match name {
            el::DIAGRAM => Ok(Self::Group(Group::new(&header, name)?)),
            el::GROUP   => Ok(Self::Group(Group::new(&header, name)?)),
            el::LAYOUT  => Ok(Self::Group(Group::new(&header, name)?)),
            el::MARKER  => Ok(Self::Group(Group::new(&header, name)?)),
            el::PATH    => Ok(Self::Path(Path::new(&header, name)?)),
            el::RECT    => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::CIRCLE  => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::POLYGON => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::TEXT    => Ok(Self::Text(Text::new(&header, name)?)),
            el::USE     => Ok(Self::Use(Use::new(&header, name)?)),
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
            Self::Path(ref c)  => Ok(Self::Path(ElementError::of_result(&header,c.clone(header, scope))?)),
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

    //mp borrow_group
    pub fn borrow_group<'z>(&'z self) -> Option<&'z Group<'a>> {
        match self {
            Self::Group(ref g) => { Some(g) },
            _ => None,
        }
    }

    //fp tree_add_element
    pub fn tree_add_element<'b>(&'b mut self, tree:Tree<'b, StylableNode<'a, StyleValue>>) -> Tree<'b, StylableNode<'a, StyleValue>>{
        match self {
            Self::Group(ref mut g) => { g.tree_add_element(tree) },
            Self::Use(ref mut g)   => { g.tree_add_element(tree) },
            _ => tree
        }
    }

    //mp style
    pub fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        match self {
            Self::Shape(ref mut s) => { s.style(descriptor, header) },
            Self::Path(ref mut s)  => { s.style(descriptor, header) },
            Self::Group(ref mut g) => { g.style(descriptor, header) },
            Self::Text(ref mut t)  => { t.style(descriptor, header) },
            Self::Use(ref mut t)   => { t.style(descriptor, header) },
        }
    }

    //mp get_desired_geometry
    pub fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        match self {
            Self::Shape(ref mut s) => { s.get_desired_geometry(layout) },
            Self::Path(ref mut s)  => { s.get_desired_geometry(layout) },
            Self::Group(ref mut g) => { g.get_desired_geometry(layout) },
            Self::Text(ref mut t)  => { t.get_desired_geometry(layout) },
            Self::Use(ref mut t)   => { t.get_desired_geometry(layout) },
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
            Self::Path(ref mut g)  => { g.apply_placement(layout, rect) },
            Self::Group(ref mut g) => { g.apply_placement(layout, rect) },
            Self::Use(ref mut g)   => { g.apply_placement(layout, rect) },
            _ => (),
        }
    }

    //mp display
    pub fn display(&self, indent:usize, indent_str:&str) {
        match self {
            Self::Shape(ref s) => { println!("{}  Shape",indent_str); s.display(indent, indent_str);},
            Self::Path(ref s)  => { println!("{}  Path",indent_str);  s.display(indent, indent_str);},
            Self::Group(ref g) => { println!("{}  Group",indent_str); g.display(indent, indent_str);},
            Self::Text(ref t)  => { println!("{}  Text",indent_str);  t.display(indent, indent_str);},
            Self::Use(ref t)   => { println!("{}  Use",indent_str);   t.display(indent, indent_str);},
        }
    }

    //zz All done
}

//a ElementLayout
//tp LayoutPlacement
#[derive(Debug)]
enum LayoutPlacement {
    None,
    Place(Point),
    Grid(isize,isize,isize,isize),
}

//ip Display for LayoutPlacement
impl std::fmt::Display for LayoutPlacement {
    //mp fmt - format for display
    /// Display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Place(p) => write!(f, "PlaceAt{}", p),
            Self::Grid(x0,y0,x1,y1) => write!(f, "Grid[({},{}) -> ({},{})]", x0,y0,x1,y1),
            _ => write!(f, "Not placed or gridded"),
        }
    }

    //zz All done
}

//tp ElementLayout
#[derive(Debug)]
pub struct ElementLayout {
    placement : LayoutPlacement,
    debug     : String,
    ref_pt    : Option<Point>,
    bbox      : Rectangle,
    pub anchor    : Point,
    pub expand    : Point,
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
               debug  : "".to_string(),
               ref_pt : None,
               bbox   : Rectangle::none(),
               anchor : Point::zero(),
               expand : Point::zero(),
               scale:1.,
               rotation:0.,
               translate : Point::zero(),
               border_width : 0.,
               border_round : 0.,
               border_color : None,
               bg : None,
               pad : None,
               margin : None,
        }
    }

    //fp of_style
    fn of_style(header:&ElementHeader) -> Result<Self,ElementError> {
        let mut layout = Self::new();
        if let Some(d) = header.get_style_of_name_string(at::DEBUG) {
            layout.debug = d;
        }
        match header.get_style_floats_of_name(at::BBOX).as_floats(None) {
            Some(g) => {
                match g.len() {
                    0 => (),
                    1 =>  { layout.bbox = Rectangle::of_cwh(Point::zero(), g[0], g[0]); },
                    2 =>  { layout.bbox = Rectangle::of_cwh(Point::zero(), g[0], g[1]); },
                    3 =>  { layout.bbox = Rectangle::of_cwh(Point::from_array([g[0], g[1]]), g[2], g[2]); },
                    _ =>  { layout.bbox = Rectangle::new(g[0], g[1], g[2], g[3]); },
                }
            }
            _ => (),
        };
        if let Some(v) = header.get_style_floats_of_name(at::ANCHOR).as_floats(None) {
            layout.anchor = Point::from_array([v[0],v[1]]);
        }
        if let Some(v) = header.get_style_floats_of_name(at::EXPAND).as_floats(None) {
            layout.expand = Point::from_array([v[0],v[1]]);
        }
        if let Some(v) = header.get_style_of_name_float(at::BORDERWIDTH,None) {
            layout.border_width = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::BORDERROUND,None) {
            layout.border_round = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::SCALE,None) {
            layout.scale = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::ROTATE,None) {
            layout.rotation = v;
        }
        if let Some(v) = header.get_style_rgb_of_name(at::BORDERCOLOR).as_floats(None) {
            layout.border_color = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name(at::BG).as_floats(None) {
            layout.bg = Some((v[0],v[1],v[2]));
        }
        if let Some(v) = header.get_style_floats_of_name(at::MARGIN).as_floats(None) {
            layout.margin = Some( (v[0], v[1], v[2], v[3]) );
        }
        if let Some(v) = header.get_style_floats_of_name(at::PAD).as_floats(None) {
            layout.pad = Some( (v[0], v[1], v[2], v[3]) );
        }
        if let Some(v) = header.get_style_floats_of_name(at::TRANSLATE).as_floats(None) {
            layout.translate = Point::from_array([v[0],v[1]]);
        }
        if let Some( (sx,sy,ex,ey) ) = {
            let opt_gx = {
                match header.get_style_ints_of_name(at::GRIDX).as_ints(None) {
                    Some(g) => {
                        match g.len() {
                            0 => None,
                            1 => Some( (g[0], g[0]+1) ),
                            _ => Some( (g[0], g[1]) ),
                        }
                    },
                    _ => None,
                }
            };
            let opt_gy = {
                match header.get_style_ints_of_name(at::GRIDY).as_ints(None) {
                    Some(g) => {
                        match g.len() {
                            0 => None,
                            1 => Some( (g[0], g[0]+1) ),
                            _ => Some( (g[0], g[1]) ),
                        }
                    },
                    _ => None,
                }
            };
            let opt_grid = {
                match header.get_style_ints_of_name(at::GRID).as_ints(None) {
                    Some(g) => {
                        match g.len() {
                            0 => None,
                            1 => Some( (g[0],g[0],g[0]+1,g[0]+1) ),
                            2 => Some( (g[0],g[1],g[0]+1,g[1]+1) ),
                            3 => Some( (g[0],g[1],g[2],g[1]+1) ),
                            _ => Some( (g[0],g[1],g[2],g[3]) ),
                        }
                    },
                    _ => None,
                }
            };
            if let Some( (gx0, gx1) ) = opt_gx {
                if let Some( (gy0, gy1) ) = opt_gy {
                    Some( (gx0, gy0, gx1, gy1) )
                } else if let Some( (_,gy0,_,gy1) ) = opt_grid {
                    Some( (gx0, gy0, gx1, gy1) )
                } else  {
                    Some( (gx0, 1, gx1, 2) )
                }
            } else if let Some( (gy0, gy1) ) = opt_gy {
                if let Some( (gx0,_,gx1,_) ) = opt_grid {
                    Some( (gx0, gy0, gx1, gy1) )
                } else  {
                    Some( (1,gy0,2,gy1) )
                }
            } else {
                opt_grid
            }
        } {
            layout.set_grid(sx,sy,ex,ey);
        }
        if let Some( (x,y) ) = {
            match header.get_style_ints_of_name(at::PLACE).as_floats(None) {
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
            layout.set_place(x,y);
        }
        Ok(layout)
    }

    //mp debug_get_grid
    pub fn debug_get_grid(&self) -> Option<(f64, &str)> {
        if self.debug != "" {
            Some((1.,"cyan"))
        } else {
            None
        }
    }

    //mp set_grid
    pub fn set_grid(&mut self, sx:isize, sy:isize, ex:isize, ey:isize) {
        self.placement = LayoutPlacement::Grid(sx,sy,ex,ey);
    }

    //mp set_place
    pub fn set_place(&mut self, x:f64, y:f64) {
        self.placement = LayoutPlacement::Place(Point::from_array([x,y]));
    }

    //mp set_layout_box
    fn set_layout_box(&self, layout_box:&mut LayoutBox, content_desired:Rectangle) {
        let bbox = {
            if self.bbox.is_none() {
                content_desired
            } else {
                self.bbox
            }
        };
        layout_box.set_content_geometry(bbox, Point::zero(), self.scale, self.rotation);
        layout_box.set_border_width(self.border_width);
        layout_box.set_border_round(self.border_round);
        layout_box.set_margin(&self.margin);
        layout_box.set_padding(&self.pad);
        layout_box.set_anchor_expand(self.anchor.clone(), self.expand.clone());
    }

    //mp set_layout_properties
    fn set_layout_properties(&self, layout:&mut Layout, bbox:Rectangle) -> Rectangle {
        match self.placement {
            LayoutPlacement::None => {
                layout.add_placed_element( &Point::zero(), &None, &bbox );
                Rectangle::none()
            }
            LayoutPlacement::Grid(sx,sy,ex,ey) => {
                layout.add_grid_element( (sx,sy), (ex,ey), (bbox.width(), bbox.height() ));
                Rectangle::none()
            },
            LayoutPlacement::Place(pt) => {
                layout.add_placed_element( &pt, &self.ref_pt, &bbox );
                Rectangle::none()
            },
        }
    }

    //zz All done
}

//a ElementHeader
//tp ElementHeader
#[derive(Debug)]
pub struct ElementHeader<'a> {
    pub stylable     : StylableNode<'a, StyleValue>,
    pub id_name      : Option<String>, // replicated from stylable
    pub layout_box   : LayoutBox,
    pub layout       : ElementLayout,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    //fp new
    pub fn new(descriptor:&'a DiagramDescriptor, name:&str, name_values:&mut dyn Iterator<Item = (String, &str)>) -> Result<Self, ElementError> {
        if let Some(styles) = descriptor.get(name) { // &RrcStyleDescriptor
            let stylable = StylableNode::new(name, styles);
            let id_name = None;
            let layout_box = LayoutBox::new();
            let layout     = ElementLayout::new();
            let mut hdr    = ElementHeader{ stylable, id_name, layout_box, layout };
            for (name,value) in name_values {
                let result = hdr.stylable.add_name_value(&name, value);
                ElementError::of_result(&hdr, result)?;
            }
            let id_name = hdr.stylable.borrow_id().map(|s| s.to_string());
            hdr.id_name = id_name;
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
        // println!("Clone header with new id {}", id_name);
        let stylable = self.stylable.clone(&id_name);
        let id_name = Some(id_name);
        let layout_box = LayoutBox::new();
        let layout = ElementLayout::new();
        ElementHeader{ stylable, id_name, layout_box, layout }
    }

    //mp get_style_names
    pub fn get_style_names<'z> () -> Vec<&'z str> {
        vec![at::DEBUG,
             at::BBOX,
             at::GRID,
             at::GRIDX,
             at::GRIDY,
             at::PLACE,
             at::ANCHOR,
             at::EXPAND,
             at::ROTATE,
             at::SCALE,
             at::TRANSLATE,
             at::PAD,
             at::MARGIN,
             at::BG,
             at::BORDERWIDTH,
             at::BORDERCOLOR,
             at::BORDERROUND]
    }

    //mp override_values
    /// Override any values in the stylable that are set in 'other'
    /// This will be called before any stylesheet is invoked,
    /// basically at construction time
    ///
    /// This is invoked on the cloned element header, with 'other'
    /// being the header that may have overriding values. This may be
    /// the header for a 'use' element, for example.
    pub fn override_values<'z>(&mut self, other:&'z ElementHeader<'a>) -> Result<(),ElementError> {
        self.stylable.override_values( &other.stylable );
        Ok(())
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        match &self.id_name {
            None => self.stylable.borrow_id().unwrap_or(""),
            Some(s) => s,
        }
    }

    //mp get_opt_style_value_of_name
    pub fn get_opt_style_value_of_name(&self, name:&str) -> Option<StyleValue> {
        let r = self.stylable.get_style_value_of_name(name).map(|a| a.clone());
        if DEBUG_ELEMENT_HEADER {println!("Debug {} {} {:?}", self.borrow_id(), name, r);}
        r
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

    //mp get_style_strings_of_name
    pub fn get_style_strings_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::string_array(),
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
        self.layout = ElementLayout::of_style(self)?;
        Ok(())
    }

    //mp set_layout_properties
    /// By this point layout_box has had its desired_geometry set
    pub fn set_layout_properties(&mut self, layout:&mut Layout, content_desired:Rectangle) -> Rectangle {
        self.layout.set_layout_box(&mut self.layout_box, content_desired);
        let bbox = self.layout_box.get_desired_bbox();
        self.layout.set_layout_properties(layout, bbox)
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
                LayoutPlacement::Grid(sx,sy,ex,ey) => layout.get_grid_rectangle( (sx,sy), (ex,ey) ),
                LayoutPlacement::Place(pt)         => layout.get_placed_rectangle( &pt, &self.layout.ref_pt ),
            }
        };
        //println!("Laying out {:?} => {}",self.layout,rect);
        self.layout_box.layout_within_rectangle(rect);
        self.layout_box.get_content_rectangle()
    }

    //mp display
    pub fn display(&self, indent_str:&str) {
        println!("{}{:?} {}",indent_str, self.id_name, self.layout.placement);
        self.layout_box.display(indent_str);
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
    //fp add_content_descriptors {
    pub fn add_content_descriptors(descriptor:&mut DiagramDescriptor) {
        descriptor.add_content_descriptor(el::USE,      false, Use::get_style_names(el::USE));
        descriptor.add_content_descriptor(el::DIAGRAM,  true,  Group::get_style_names(el::DIAGRAM));
        descriptor.add_content_descriptor(el::GROUP,    true,  Group::get_style_names(el::GROUP));
        descriptor.add_content_descriptor(el::LAYOUT,   true,  Group::get_style_names(el::LAYOUT));
        descriptor.add_content_descriptor(el::MARKER,   true,  Group::get_style_names(el::MARKER));
        descriptor.add_content_descriptor(el::TEXT,     true,  Text::get_style_names(el::TEXT));
        descriptor.add_content_descriptor(el::POLYGON,  true,  Shape::get_style_names(el::POLYGON));
        descriptor.add_content_descriptor(el::RECT,     true,  Shape::get_style_names(el::RECT));
        descriptor.add_content_descriptor(el::CIRCLE,   true,  Shape::get_style_names(el::CIRCLE));
        descriptor.add_content_descriptor(el::PATH,     true,  Path::get_style_names(el::PATH));
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        self.header.borrow_id()
    }

    //mp has_id
    pub fn has_id(&self, name:&str) -> bool {
        self.header.stylable.has_id(name)
    }

    //fp new
    pub fn new(descriptor:&'a DiagramDescriptor, name:&str, name_values:&mut dyn Iterator<Item = (String, &str)>) -> Result<Self, ElementError> {
        // println!("New element name '{}'", name);
        let header  = ElementHeader::new(descriptor, name, name_values)?;
        let content = ElementContent::new(&header, name)?;
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

    //mp borrow_marker
    pub fn borrow_marker<'z> (&'z self) -> Option<(&'z ElementHeader<'a>, &'z Group<'a>)> {
        match self.content.borrow_group() {
            None => None,
            Some(x) => Some((&self.header, x))
        }
    }

    //fp tree_add_element
    pub fn tree_add_element<'b>(&'b mut self, mut tree:Tree<'b, StylableNode<'a, StyleValue>>) -> Tree<'b, StylableNode<'a, StyleValue>>{
        tree.open_container(&mut self.header.stylable);
        tree = self.content.tree_add_element(tree);
        tree.close_container();
        tree
        }

    //mp style
    pub fn style(&mut self, descriptor:&DiagramDescriptor) -> Result<(),ElementError> {
        // println!("Style  {} ", self.header.borrow_id());
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

    //fp display
    pub fn display(&self, indent:usize) {
        const INDENT_STRING : &str="                                                            ";
        let indent_str = &INDENT_STRING[0..indent];
        self.header.display(indent_str);
        self.content.display(indent, indent_str);
    }

    //zz All done
}


