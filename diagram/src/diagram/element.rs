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
use crate::GridLayout;
use crate::Diagram;
use crate::DiagramDescriptor;
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, RrcStylableNode};
use super::types::*;

//a Element types
//tp Group - an Element that contains just other Elements
#[derive(Debug)]
pub struct Group<'a> {
    // requires no styles
    content : Vec<Element<'a>>
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
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = ElementHeader::get_descriptor(nts);
        desc.borrow_mut().add_styles(nts, vec!["fill", "stroke", "strokewidth", "round", "markers"]);
        desc
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

//a ElementHeader and Element
//tp ElementStyle
#[derive(Debug)]
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
#[derive(Debug)]
pub struct ElementHeader<'a> {
    pub id           : StyleValue,
    pub classes      : StyleValue,
    styles           : Vec<ElementStyle>,
    stylable         : RrcStylableNode<'a, StyleValue>,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    pub fn new<'b> (styles:&RrcStyleDescriptor, name_values:Vec<(String,String)>) -> Result<(ElementHeader<'b>, Vec<(String,String)>), ValueError> {
        // let mut unused_nv = Vec::new();
        let unused_nv = Vec::new();
        let mut hdr = 
            ElementHeader{ id      : StyleValue::string(None),
                           classes : StyleValue::string_array(),
                           styles  : Vec::new(),
                           stylable: StylableNode::new(None, "node_type", styles, Vec::new()/*name_values*/),
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
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = StyleDescriptor::new();
        desc.borrow_mut().add_styles(nts, vec!["bbox", "grid", "transform", "pad", "margin", "border"]);
        desc
    }
}


//tp Element - the enumeration of the above
#[derive(Debug)]
pub enum ElementContent<'a> {
    Group(Group<'a>),
    Text(Text),
    Shape(Shape),
    Use(Use), // use of a definition
}

#[derive(Debug)]
pub struct Element<'a> {
    header  : ElementHeader<'a>,
    content : ElementContent<'a>,
}

//ti Element
impl <'a> Element <'a> {
    //mp has_id
    pub fn has_id(&self, name:&str) -> bool {
        self.header.id.eq_string(name)
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

    //fp value_of_name
    pub fn value_of_name(name_values:Vec<(String,String)>, name:&str, mut value:StyleValue) -> Result<StyleValue,ValueError> {
        for (n,v) in name_values {
            if n==name {
                value.from_string(&v)?;
            }
        }
        Ok(value)
    }

    //fp set_grid_layout
    pub fn set_grid_layout(&self, grid:&mut GridLayout) {
    }
                           
}

