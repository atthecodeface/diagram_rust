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

@file    group.rs
@brief   Diagram group element
 */

//a Imports
use super::super::{GenerateSvg, Svg, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, Element, ElementScope, ElementHeader, ElementError};
use super::super::types::*;
use crate::{Layout};
use crate::{Rectangle};

//a Group element
//tp Group - an Element that contains just other Elements
/// The Group supplies simple grouping of elements
///
/// This element could have its own layout blob, if it is defined to be a layout entity
///
/// The elements that are part of this group must be created and moved
/// in to this group; the lifetime of the elements is the same as that
/// of the group.
#[derive(Debug)]
pub struct Group<'a> {
    /// The elements that are part of this group
    pub content : Vec<Element<'a>>
}

//ip DiagramElementContent for Group
impl <'a> DiagramElementContent for Group<'a> {
    //fp new
    /// Create a new group
    fn new(_header:&ElementHeader, _name:&str) -> Result<Self,ElementError> {
        Ok( Self {
            content:Vec::new(),
        } )
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header:&ElementHeader, scope:&ElementScope ) -> Result<Self,ElementError>{
        ElementError::of_result(header, Err("nyi:group"))
    }

    //fp get_descriptor
    /// Get the style descriptor for this element when referenced by the name
    fn get_descriptor(nts:&StyleSet, _name:&str) -> RrcStyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, _header:&ElementHeader) -> Result<(),ElementError> {
        for e in self.content.iter_mut() {
            e.style(descriptor)?;
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        let mut rect = Rectangle::none();
        for e in self.content.iter_mut() {
            rect = rect.union(&e.set_layout_properties(layout));
        }
        rect
    }

    //fp apply_placement
    fn apply_placement(&mut self, layout:&Layout) {
        for e in self.content.iter_mut() {
            e.apply_placement(layout);
        }
    }
    
    //zz All done
}

//ip Group
impl <'a> Group<'a> {
    //mp add_element
    /// Add an element to the group; moves the element in to the content
    pub fn add_element(&mut self, element:Element<'a>) -> () {
        self.content.push(element);
    }
}

//ip GenerateSvg for Group
impl <'a> GenerateSvg for Group <'a> {
    fn generate_svg(&self, svg:&mut Svg) -> Result<(), SvgError> {
        for e in &self.content {
            e.generate_svg( svg )?;
        }

        Ok(())
    }
}

