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
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, Element, ElementScope, ElementHeader, ElementError};
use super::super::types::*;
use crate::{Layout};
use crate::{Rectangle};

//a Use element
//tp Use - an Element that is a reference to a group or other element
#[derive(Debug)]
pub struct Use<'a> {
    /// The ID that this is a usage of
    id_ref  : String,
    /// Any strings provided with the element, which should be used
    /// within the uniqified element content post-uniqification
    strings : Vec<String>,
    /// The element that the use is bound to, once uniqified
    content : Vec<Element<'a>>,
    /// Depth of recursion
    depth : usize,
}

//ti DiagramElementContent for Use<'a>
impl <'a, 'b> DiagramElementContent <'a, 'b> for Use<'a> {
    //fp new
    /// Create a new element of the given name
    fn new(header:&ElementHeader, _name:&str) -> Result<Self,ElementError> {
        if let Some(id_ref) = header.get_style_of_name_string("ref") {
            Ok(Self { id_ref, strings:Vec::new(), content:Vec::new(), depth:0 })
        } else {
            Err(ElementError::of_string(header, "No 'ref' attribute found in use element"))
        }
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, _header:&ElementHeader, scope:&ElementScope ) -> Result<Self,ElementError> {
        let id_ref = self.id_ref.clone();
        let strings = self.strings.iter().map(|s| s.clone()).collect();
        let content = Vec::new();
        let depth = scope.depth + 1;
        Ok(Self { id_ref, strings, content, depth })
    }

    //mp uniquify
    /// Sets internal self.content to a clone of a resolved definition
    ///
    /// The id_ref should identify an element in `scope`.
    /// The header may have to be cloned - it has layout information etc, and indeed any of its
    /// name/values override those of
    fn uniquify(&mut self, header:&ElementHeader<'a>, scope:&ElementScope<'a,'b>) -> Result<bool, ElementError> {
        match self.content.len() {
            0 => {
                let (scope, element) = scope.new_subscope(header, &self.id_ref, self.depth+1)?;
                self.content.push(element.clone(&scope)?);
                Ok(true)
            }
            _ => {
                self.content[0].uniquify(scope)?;
                Ok(false)
            }
        }
    }

    //fp get_descriptor
    fn get_descriptor(nts:&StyleSet, _name:&str) -> RrcStyleDescriptor {
        let desc = ElementHeader::get_descriptor(nts);
        // tab stops, bullets, alignment
        desc.borrow_mut().add_styles(nts, vec!["ref"]);
        desc
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
    fn apply_placement(&mut self, layout:&Layout, _rect:&Rectangle) {
        for e in self.content.iter_mut() {
            e.apply_placement(layout);
        }
    }
    
    //zz All done
}

//ti Use
impl <'a> Use<'a> {
    //mp add_string
    pub fn add_string(&mut self, s:&str) -> Result<(),String> {
        self.strings.push(s.to_string());
        Ok(())
    }

}

//ip GenerateSvg format Use
impl <'a> GenerateSvgElement for Use <'a> {
    fn generate_svg(&self, svg:&mut Svg, _header:&ElementHeader) -> Result<(), SvgError> {
        for e in &self.content {
            e.generate_svg( svg )?;
        }

        Ok(())
    }
}

