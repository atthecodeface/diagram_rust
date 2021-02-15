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
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::types::*;
use super::element::{Element, ElementHeader};
use super::font::*;

//a Diagram Descriptor - covers
//tp DiagramDescriptor - contains the StyleSet and StyleDescriptor's for each element type
/// A DiagramDescriptor contains the names and types of the styles
/// that elements may have within a `Diagram`, and the fonts that the
/// `Diagram` knows about.
///
/// A `DiagramDescriptor` must be created so that it can be used by a
/// `Diagram`, and once constructed it should not be further mutated.
/// The `Diagram` can borrow references to it, and hence it must
/// outlive the Diagram.
pub struct DiagramDescriptor<'a> {
    style_set   : &'a StyleSet,
    descriptors : HashMap<&'a str, StyleDescriptor<'a>>,
    fonts       : HashMap<&'a str, RrcFont>,
}

//ti DiagramDescriptor
impl <'a> DiagramDescriptor<'a> {
    //fp create_style_set
    /// Create the `StyleSet` required by the `DiagramDescriptor`
    /// The `StyleSet` must have a lifetime that exceeds the descriptor,
    /// and it should be treated as immutable
    pub fn create_style_set() -> StyleSet {
        let style_set = StyleSet::new()
            .add_type("bbox",        StyleValue::float_array(), false)            
            .add_type("grid",        StyleValue::int_array(),   false)
            .add_type("gridx",       StyleValue::int_array(),   false)
            .add_type("gridy",       StyleValue::int_array(),   false)
            .add_type("minx",        StyleValue::float_array(), false)
            .add_type("miny",        StyleValue::float_array(), false)
            .add_type("growx",       StyleValue::float_array(), false)
            .add_type("growy",       StyleValue::float_array(), false)
            .add_type("place",       StyleValue::float_array(), false)
            .add_type("anchor",      StyleValue::floats(2),     true)
            .add_type("expand",      StyleValue::floats(2),     true)
            .add_type("pad",         StyleValue::floats(4),     false)
            .add_type("margin",      StyleValue::floats(4),     true)
            .add_type("border",      StyleValue::float(None),   true)
            .add_type("borderround", StyleValue::float(None),   true)
            .add_type("bordercolor", StyleValue::rgb(None),     true)
            .add_type("bg",          StyleValue::rgb(None),     true)
            .add_type("scale",       StyleValue::float(None),   true)
            .add_type("rotate",      StyleValue::float(None),   true)
            .add_type("translate",   StyleValue::floats(2),     true)
            .add_type("fill",        StyleValue::rgb(None),     true)
            .add_type("stroke",      StyleValue::rgb(None),     true)
            .add_type("strokewidth", StyleValue::float(None),   true)
            .add_type("width",       StyleValue::float(None),   true)
            .add_type("height",      StyleValue::float(None),   true)
            .add_type("round",       StyleValue::float(None),   true)
            .add_type("stellate",    StyleValue::float(None),   true)
            .add_type("markers",     StyleValue::string_array(),  true)
            .add_type("font",        StyleValue::string(None),  true)
            .add_type("fontsize",    StyleValue::float(None),   true)
            .add_type("fontweight",  StyleValue::string(None),  true)
            .add_type("fontstyle",   StyleValue::string(None),  true)
            .add_type("vertices",    StyleValue::int(None),     true)
            .add_type("ref",         StyleValue::string(None),  false)
            ;
        style_set
    }

    //fp new
    /// Create a new `DiagramDescriptor` using a StyleSet The
    /// `DiagramDescriptor` can be used to create diagrams, and read
    /// them from markup files.  It must have a lifetime that is at
    /// least as long as any `Diagram`s it is used for.
    ///
    /// It is immutable after it has been created.
    ///
    /// # Example
    ///
    /// ```
    /// extern crate diagram;
    /// let style_set          = diagram::DiagramDescriptor::create_style_set();
    /// let diagram_descriptor = diagram::DiagramDescriptor::new(&style_set);
    /// let mut diagram        = diagram::Diagram::new(&diagram_descriptor);
    /// let mut dml            = diagram::DiagramML::new(&mut diagram);
    /// dml.read_file("#diagram ##shape id=circle vertices=0".as_bytes())?;
    /// # Ok::<(), diagram::MLErrorList>(())
    /// ```
    pub fn new(style_set:&'a StyleSet) -> Self {
        let descriptors = HashMap::new();
        let fonts       = HashMap::new();
        let mut descriptor = Self {
            style_set,
            descriptors,
            fonts,
        };
        Element::add_content_descriptors(&mut descriptor);
        descriptor.fonts.insert("default", Rc::new(RefCell::new(Font::default())) );
        descriptor
    }

    //fp add_content_descriptors
    /// Invoked by element types to add descriptors to the set
    /// dependent on those descriptor values.
    ///
    /// This is only invoked by the `Element` type to add the required
    /// descriptors for the element types for styling.
    pub(super) fn add_content_descriptor(&mut self, name:&'static str, include_hdr:bool, styles:Vec<&str>) {
        let mut descriptor = StyleDescriptor::new(&self.style_set);
        if include_hdr {
            descriptor.add_styles(ElementHeader::get_style_names());
        }
        descriptor.add_styles(styles);
        self.descriptors.insert(name, descriptor);
    }

    //mp get
    /// Get the descriptor belonging to a tag name
    pub(crate) fn get(&self, tag:&str) -> Option<&StyleDescriptor> {
        match self.descriptors.get(tag)
        { Some(rrc) => Some(rrc), None => None}
    }

    //mp get_font
    /// Get a font
    pub(crate) fn get_font(&self) -> RrcFont {
        self.fonts.get("default").unwrap().clone()
    }

    //zz All done
}
