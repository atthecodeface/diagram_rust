pub struct Transform {
    matrix : [f64;9],
}

pub struct Color {
    rgb : [f64;3],
}

pub enum ElementStyle {
    Transform(Transform),
    Fill(Option<Color>),
    Stroke(Option<Color>),
    StrokeWidth(f64),
}

pub struct ElementHeader {
    id           : Option<String>,
    classes      : Vec<String>,
    styles       : Vec<ElementStyle>,
}

impl ElementHeader {
    pub fn new(name_values:Vec<(String,String)>) -> (ElementHeader, Vec<(String,String)>) {
        let mut unused_nv = Vec::new();
        let mut hdr = 
            ElementHeader{ id:None,
                           classes:Vec::new(),
                           styles:Vec::new(),
            };
        for (n,v) in name_values {
            if n=="id" { hdr.id=Some(v); }
            else if n=="class" {
                for s in v.split_whitespace() {
                    hdr.classes.push(s.to_string());
                }
            } else {
                unused_nv.push((n,v));
            }
        }
        (hdr, unused_nv)
    }
}

pub struct Group {
    header : ElementHeader,
    content : Vec<Element>
}

pub struct Text {
    header  : ElementHeader,
    text    : Vec<String>,
}

pub struct Shape {
    header  : ElementHeader,
}

pub struct Use {
    header  : ElementHeader,
    id      : String,
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
    
