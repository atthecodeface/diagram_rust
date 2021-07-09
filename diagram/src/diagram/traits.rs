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
use crate::DiagramDescriptor;
use crate::{Layout};
pub use super::elements::{Group, Shape, Path, Text, Use};
use super::{ElementHeader, ElementScope, ElementError};

//a DiagramElementContent trait
//tp DiagramElementContent
/// 'a is the lifetime of the diagram
/// 'b is the lifetime of a scope while uniqifying/cloning contents of the diagram
pub trait DiagramElementContent <'a, 'b> : Sized+std::fmt::Debug {
    //fp new
    /// Create a new element of the given name
    fn new(header:&ElementHeader<'a>, name:&str ) -> Result<Self,ElementError>;

    //fp clone
    /// Clone element given clone of header within scope
    ///
    /// This method is only invoke prior to styling, so often is the same as `new`
    fn clone(&self, header:&ElementHeader<'a>, scope:&ElementScope<'a, 'b> ) -> Result<Self,ElementError>;

    //mp uniquify
    /// Sets internal self.content to a clone of a resolved definition
    ///
    /// The id_ref should identify an element in `scope`.
    /// The header may have to be cloned - it has layout information etc, and indeed any of its
    /// name/values override those of
    fn uniquify(&mut self, _header:&ElementHeader<'a>, _scope:&ElementScope<'a,'b>) -> Result<bool, ElementError> {
        Ok(false)
    }

    //fp get_style_names
    /// Get the style descriptor for this element when referenced by the name
    fn get_style_names<'c>(_name:&str) -> Vec<&'c str>;

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, _descriptor:&DiagramDescriptor, _header:&ElementHeader) -> Result<(),ElementError> {
        Ok(())
    }

    //mp get_desired_geometry
    /// Get the desired bounding box for the element; the layout is
    /// required if it is to be passed in to the contents (element
    /// header + element content) -- by setting their layout
    /// properties -- but does not effect the *content* of a single
    /// element
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        Rectangle::none()
    }

    //fp apply_placement
    /// Apply the layout to the element; this may cause contents to
    /// then get laid out, etc Nothing needs to be done - the layout
    /// is available when the element is visualized
    ///
    /// The rectangle supplied is the content-space rectangle derived
    /// for the content
    fn apply_placement(&mut self, _layout:&Layout, _rect:&Rectangle) {
        // No need to do anything
    }

    //mp display
    /// Display - using indent_str + 2 indent, or an indent of indent spaces
    /// Content should be invoked with indent+4
    fn display(&self, _indent:usize, _indent_str:&str) {
    }

    //zz All done
}

