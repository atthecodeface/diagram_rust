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
use crate::{Layout, LayoutBox, Rectangle};
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
//tp ElementHeader
#[derive(Debug)]
pub struct ElementHeader<'a> {
    stylable         : RrcStylableNode<'a, StyleValue>,
    layout_box       : LayoutBox,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    pub fn new<'b> (styles:&RrcStyleDescriptor, name_values:Vec<(String,String)>) -> Result<(ElementHeader<'b>, Vec<(String,String)>), ValueError> {
        // let mut unused_nv = Vec::new();
        let unused_nv = Vec::new();
        let stylable = StylableNode::new(None, "node_type", styles, vec![]);
        for (name,value) in &name_values {
            stylable.borrow_mut().add_name_value(name, value);
        }
        let layout_box = LayoutBox::new();
        let hdr = ElementHeader{ stylable, layout_box };
        Ok((hdr, unused_nv))
    }
    pub fn get_descriptor(nts:&StyleSet) -> RrcStyleDescriptor {
        let desc = StyleDescriptor::new();
        desc.borrow_mut().add_styles(nts, vec!["bbox", "grid", "transform", "pad", "margin", "border", "bg", "bordercolor"]);
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

    //fp value_of_name
    pub fn value_of_name(name_values:Vec<(String,String)>, name:&str, mut value:StyleValue) -> Result<StyleValue,ValueError> {
        for (n,v) in name_values {
            if n==name {
                value.from_string(&v)?;
            }
        }
        Ok(value)
    }

    //fp get_desired_geometry
    pub fn get_desired_geometry(&mut self) -> Rectangle {
        Rectangle::new(0.,0.,10.,10.)
    }
    
    //fp set_layout
    pub fn set_layout(&mut self, layout:&mut Layout) {
        let bbox = self.get_desired_geometry();
        let stylable = self.header.stylable.borrow();
        match stylable.get_style_value_of_name("grid").unwrap().as_ints(None) {
            None => {return;},
            Some(g) => {
                let (sx,sy,nx,ny):(isize,isize,isize,isize) = {
                    match g.len() {
                        0 => {return;},
                        1 => (g[0],g[0],1,1),
                        2 => (g[0],g[1],1,1),
                        3 => (g[0],g[1],g[2],1),
                        _ => (g[0],g[1],g[3],g[4]),
                    }
                };
                layout.add_element( (sx,sy), (nx as usize,ny as usize), (bbox.width(), bbox.height()) );
            },
        }
    }
                           
    //fp apply_placement
    pub fn apply_placement(&mut self, layout:&Layout) {
    }
                           
}

