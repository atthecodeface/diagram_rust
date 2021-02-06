use stylesheet::{BaseValue, TypeValue};
use stylesheet::{Descriptor};
type StyleValue = BaseValue;
type StyleDescriptor = Descriptor<StyleValue>;

pub enum ElementStyle {
    Grid(StyleValue), // 2 or 4 ints
    Bbox(StyleValue), // 2 or 4 floats
    Transform(StyleValue), // 9 floats
    Fill(StyleValue),   // color
    Stroke(StyleValue), // color
    StrokeWidth(StyleValue), // float
    Markers(StyleValue), // 1-3 strings
    Font(StyleValue), // string
    FontSize(StyleValue), // float
    FontStyle(StyleValue), // string
    FontWeight(StyleValue), // string
}

pub struct ElementHeader {
    id           : StyleValue,
    classes      : StyleValue,
    styles       : Vec<ElementStyle>,
}

impl ElementHeader {
    pub fn new(name_values:Vec<(String,String)>) -> (ElementHeader, Vec<(String,String)>) {
        let mut unused_nv = Vec::new();
        let mut hdr = 
            ElementHeader{ id      : StyleValue::string(None),
                           classes : StyleValue::string_array(),
                           styles  : Vec::new(),
            };
        for (n,v) in name_values {
            if n=="id" {
                hdr.id.from_string(&v);
            }
            else if n=="class" {
                for s in v.split_whitespace() {
                    hdr.classes.add_string(s.to_string());
                }
            } else { // if in base elementstyle then do that else
                unused_nv.push((n,v));
            }
        }
        (hdr, unused_nv)
    }
    pub fn get_descriptor() -> StyleDescriptor {
        StyleDescriptor::new()
            .add_style("bbox",      &StyleValue::float_array(), false)
            .add_style("grid",      &StyleValue::int_array(),   false)
            .add_style("transform", &StyleValue::floats(9),     false)
    }
}

pub struct Group {
    // requires no styles
    header : ElementHeader,
    content : Vec<Element>
}
impl Group {
    pub fn get_descriptor() -> StyleDescriptor {
        ElementHeader::get_descriptor()
    }
}

pub struct Text {
    header  : ElementHeader,
    text    : Vec<String>,
}
impl Text {
    pub fn get_descriptor() -> StyleDescriptor {
        ElementHeader::get_descriptor()
            .add_style("fill",       &StyleValue::rgb(None),  true)
            .add_style("font",       &StyleValue::string(None), true)
            .add_style("fontsize",   &StyleValue::float(None),  true)
            .add_style("fontweight", &StyleValue::string(None),  true)
            .add_style("fontstyle",  &StyleValue::string(None),  true)
    }
}

pub struct Shape {
    // Possibly polygon
    // has Fill, Stroke, StrokeWidth, Markers
    header  : ElementHeader,
}

impl Shape {
    pub fn get_descriptor() -> StyleDescriptor {
        ElementHeader::get_descriptor()
            .add_style("fill",         &StyleValue::rgb(None),  true)
            .add_style("stroke",       &StyleValue::rgb(None),  true)
            .add_style("strokewidth",  &StyleValue::float(None),  true)
            .add_style("round",        &StyleValue::float(None),  true)
            .add_style("markers",      &StyleValue::string_array(),  true)
    }
}

pub struct Use {
    // has Transform - to put it somewhere!
    header  : ElementHeader,
    id_ref  : String,
}

pub enum Element {
    Group(Group),
    Text,
    Shape,
    Use(String), // use of a definition
}

pub struct Definition {
    name    : String,
    elements : Vec<Element>,
}

pub struct Diagram {
    definitions : Vec<Definition>,
    elements : Vec<Element>,
}

impl Diagram {
    pub fn new() -> Self {
        Self { definitions:Vec::new(),
               elements:Vec::new(),
        }
    }
}
    
    /*
defs
g
image
marker
text : x,y,dx,dy,rotate,lengthAdjust,textLength
path: d
[d has Moveto, Lineto, Cubicbezierto, Quadraticbezierto, ellipticalArcto, Zclosepath - upper case absolute, lower case relative
line
circle
ellipse
polygon: points
polyline: points
rect : x, y, width, height, rx, ry
     */
    
