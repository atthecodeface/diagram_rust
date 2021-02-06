use stylesheet::{BaseValue, TypeValue, NamedTypeSet};
use stylesheet::{Descriptor};
type StyleValue = BaseValue;
type StyleDescriptor = Descriptor<StyleValue>;
type StyleSet = NamedTypeSet<StyleValue>;

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
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        StyleDescriptor::new()
            .add_style(nts, "bbox")
            .add_style(nts, "grid")
            .add_style(nts, "transform")
    }
}

pub struct Group {
    // requires no styles
    header : ElementHeader,
    content : Vec<Element>
}
impl Group {
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }
}

pub struct Text {
    header  : ElementHeader,
    text    : Vec<String>,
}
impl Text {
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
            .add_style(nts, "fill")
            .add_style(nts, "font")
            .add_style(nts, "fontsize")
            .add_style(nts, "fontweight")
            .add_style(nts, "fontstyle")
    }
}

pub struct Shape {
    // Possibly polygon
    // has Fill, Stroke, StrokeWidth, Markers
    header  : ElementHeader,
}

impl Shape {
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
            .add_style(nts, "fill")
            .add_style(nts, "stroke")
            .add_style(nts, "strokewidth")
            .add_style(nts, "round")
            .add_style(nts, "markers")
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
    pub fn get_style_set() -> StyleSet {
        StyleSet::new()
            .add_type("bbox",       StyleValue::float_array(), false)            
            .add_type("grid",       StyleValue::int_array(),   false)
            .add_type("transform",  StyleValue::floats(9),     false)
            .add_type("fill",       StyleValue::rgb(None),  true)
            .add_type("stroke",     StyleValue::rgb(None),  true)
            .add_type("strokewidth",StyleValue::float(None),  true)
            .add_type("round",      StyleValue::float(None),  true)
            .add_type("markers",    StyleValue::string_array(),  true)
            .add_type("font",       StyleValue::string(None), true)
            .add_type("fontsize",   StyleValue::float(None),  true)
            .add_type("fontweight", StyleValue::string(None),  true)
            .add_type("fontstyle",  StyleValue::string(None),  true)
        
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
    
