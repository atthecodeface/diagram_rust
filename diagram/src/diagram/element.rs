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
use std::collections::HashMap;
use stylesheet::TypeValue; // For the trait, to get access to 'from_string'
use super::types::*;

//a Element types
//tp Group - an Element that contains just other Elements
pub struct Group {
    // requires no styles
    content : Vec<Element>
}

//ti Group
impl Group {
    //fp new
    pub fn new(name_values:Vec<(String,String)>) -> Result<Self,ValueError> {
        Ok( Self {
            content:Vec::new(),
        } )
    }

    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }

    //mp add_element
    pub fn add_element(&mut self, element:Element) -> () {
        self.content.push(element);
    }
    
    //zz All done
}

//tp Text - an Element that contains text
pub struct Text {
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

//tp Shape - an Element that contains a polygon (or path?)
pub struct Shape {
    // Possibly polygon
    // has Fill, Stroke, StrokeWidth, Markers
    vertices : usize, // 0 for circle?
}

//ti Shape
impl Shape {
    //fp new
    pub fn new(name_values:Vec<(String,String)>) -> Result<Self,ValueError> {
        let vertices = Element::value_of_name(name_values, "vertices", StyleValue::int(Some(4)))?.as_int(Some(4)).unwrap() as usize;
        Ok( Self {
            vertices,
        } )
    }

    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
            .add_style(nts, "fill")
            .add_style(nts, "stroke")
            .add_style(nts, "strokewidth")
            .add_style(nts, "round")
            .add_style(nts, "markers")
    }

    //zz All done
}

//tp Use - an Element that is a reference to a group or other element
pub struct Use {
    // has Transform - to put it somewhere!
    id_ref  : String,
}

//ti Use
impl Use {
    //fp get_descriptor
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }
}

//a ElementHeader and Element
//tp ElementStyle
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

//tp ElementHeader
pub struct ElementHeader {
    pub id           : StyleValue,
    pub classes      : StyleValue,
    styles       : Vec<ElementStyle>,
}

//ti ElementHeader
impl ElementHeader {
    pub fn new(_styles:&StyleDescriptor, name_values:Vec<(String,String)>) -> Result<(ElementHeader, Vec<(String,String)>), ValueError> {
        // let mut unused_nv = Vec::new();
        let unused_nv = Vec::new();
        let mut hdr = 
            ElementHeader{ id      : StyleValue::string(None),
                           classes : StyleValue::string_array(),
                           styles  : Vec::new(),
            };
        for (n,v) in name_values {
            if n=="id" {
                hdr.id.from_string(&v)?;
            } else if n=="class" {
                for s in v.split_whitespace() {
                    hdr.classes.add_string(s.to_string());
                }
            } else {
                //match styles.find_style_index(n) {
                //    None => unused_nv.push((n,v));
                //}
            }
        }
        Ok((hdr, unused_nv))
    }
    pub fn get_descriptor(nts:&StyleSet) -> StyleDescriptor {
        StyleDescriptor::new()
            .add_style(nts, "bbox")
            .add_style(nts, "grid")
            .add_style(nts, "transform")
            .add_style(nts, "pad")
            .add_style(nts, "margin")
            .add_style(nts, "border")
    }
}


//tp Element - the enumeration of the above
pub enum ElementContent {
    Group(Group),
    Text(Text),
    Shape(Shape),
    Use(Use), // use of a definition
}

pub struct Element {
    header  : ElementHeader,
    content : ElementContent,
}

//ti Element
impl Element {
    //mp has_id
    pub fn has_id(&self, name:&str) -> bool {
        self.header.id.eq_string(name)
    }

    //fp new_shape
    pub fn new_shape(styles:&StyleDescriptor, name_values:Vec<(String,String)>) -> Result<Element, ValueError> {
        let (header, name_values) = ElementHeader::new(styles, name_values)?;
        Ok(Self { header, content:ElementContent::Shape(Shape::new(name_values)?) })
    }

    //fp new_group
    pub fn new_group(styles:&StyleDescriptor, name_values:Vec<(String,String)>) -> Result<Element, ValueError> {
        let (header, name_values) = ElementHeader::new(styles, name_values)?;
        Ok(Self { header, content:ElementContent::Group(Group::new(name_values)?) })
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
}

