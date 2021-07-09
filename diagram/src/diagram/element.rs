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
// const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use geometry::{Rectangle};
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, Tree};
use crate::constants::elements   as el;
use crate::DiagramDescriptor;
use crate::{Layout};
use super::elements::{Group, Shape, Path, Text, Use};
use super::types::*;
use super::DiagramElementContent;
use super::{ElementError};
use super::{ElementScope};
use super::{ElementContent};
use super::{ElementHeader};

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
        descriptor.add_content_descriptor(el::Typ::Use,      false, Use::get_style_names(el::USE));
        descriptor.add_content_descriptor(el::Typ::Diagram,  true,  Group::get_style_names(el::DIAGRAM));
        descriptor.add_content_descriptor(el::Typ::Group,    true,  Group::get_style_names(el::GROUP));
        descriptor.add_content_descriptor(el::Typ::Layout,   true,  Group::get_style_names(el::LAYOUT));
        descriptor.add_content_descriptor(el::Typ::Marker,   true,  Group::get_style_names(el::MARKER));
        descriptor.add_content_descriptor(el::Typ::Text,     true,  Text::get_style_names(el::TEXT));
        descriptor.add_content_descriptor(el::Typ::Polygon,  true,  Shape::get_style_names(el::POLYGON));
        descriptor.add_content_descriptor(el::Typ::Rect,     true,  Shape::get_style_names(el::RECT));
        descriptor.add_content_descriptor(el::Typ::Circle,   true,  Shape::get_style_names(el::CIRCLE));
        descriptor.add_content_descriptor(el::Typ::Path,     true,  Path::get_style_names(el::PATH));
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
    pub fn new(descriptor:&'a DiagramDescriptor, name:el::Typ, name_values:&mut dyn Iterator<Item = (String, &str)>) -> Result<Self, ElementError> {
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


