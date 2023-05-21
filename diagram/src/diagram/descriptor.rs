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
use super::font::*;
use super::types::*;
use super::{Element, ElementHeader};
use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::diagram::Color;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
    pub(super) style_set: &'a StyleSet,
    descriptors: HashMap<el::Typ, StyleDescriptor<'a>>,
    fonts: HashMap<&'a str, RrcFont>,
}

//ti DiagramDescriptor
impl<'a> DiagramDescriptor<'a> {
    //fp create_style_set
    /// Create the `StyleSet` required by the `DiagramDescriptor`
    /// The `StyleSet` must have a lifetime that exceeds the descriptor,
    /// and it should be treated as immutable
    pub fn create_style_set() -> StyleSet {
        let color_type = StyleTypeValue::new(Color::default());
        let string_type = StyleTypeValue::new(Option::<String>::None);
        let int_type = StyleTypeValue::new(Option::<isize>::None);
        let int_list_type = StyleTypeValue::new(Option::<Vec<isize>>::None);
        let float_type = StyleTypeValue::new(Option::<f64>::None);
        let float_list_type = StyleTypeValue::new(Option::<Vec<f64>>::None);
        let point_type = StyleTypeValue::new(Option::<[f64; 2]>::None);
        let box_type = StyleTypeValue::new(Option::<[f64; 4]>::None);
        let string_list_type = StyleTypeValue::new(("", true, Vec::<String>::new()));
        let string_comma_list_type = StyleTypeValue::new((",", true, Vec::<String>::new()));
        StyleSet::default()
            .add_type(at::DEBUG, string_type.clone(), false)
            .add_type(at::BBOX, box_type.clone(), false)
            .add_type(at::GRID, int_list_type.clone(), false)
            .add_type(at::GRIDX, int_list_type.clone(), false)
            .add_type(at::GRIDY, int_list_type, false)
            .add_type(at::MINX, string_comma_list_type.clone(), false)
            .add_type(at::MINY, string_comma_list_type, false)
            .add_type(at::PLACE, float_list_type.clone(), false)
            .add_type(at::ANCHOR, point_type.clone(), true)
            .add_type(at::EXPAND, point_type.clone(), true)
            .add_type(at::PAD, box_type.clone(), false)
            .add_type(at::MARGIN, box_type, true)
            .add_type(at::BORDERWIDTH, float_type.clone(), true)
            .add_type(at::BORDERROUND, float_type.clone(), true)
            .add_type(at::BORDERCOLOR, color_type.as_type(), true)
            .add_type(at::BG, color_type.as_type(), true)
            .add_type(at::SCALE, float_type.clone(), true)
            .add_type(at::ROTATE, float_type.clone(), true)
            .add_type(at::TRANSLATE, point_type.clone(), true)
            .add_type(at::POINT, point_type.clone(), true)
            .add_type(at::RELIEF, point_type, true)
            .add_type(at::FILL, color_type.as_type(), true)
            .add_type(at::STROKE, color_type.as_type(), true)
            .add_type(at::STROKEWIDTH, float_type.clone(), true)
            .add_type(at::WIDTH, float_type.clone(), true)
            .add_type(at::HEIGHT, float_type.clone(), true)
            .add_type(at::COORDS, float_list_type, false)
            .add_type(at::ROUND, float_type.clone(), true)
            .add_type(at::STELLATE, float_type.clone(), true)
            .add_type(at::MARKERS, string_list_type, true)
            .add_type(at::FONT, string_type.clone(), true)
            .add_type(at::FONTSIZE, float_type, true)
            .add_type(at::FONTWEIGHT, string_type.clone(), true)
            .add_type(at::FONTSTYLE, string_type.clone(), true)
            .add_type(at::VERTICES, int_type.clone(), true)
            .add_type(at::FLAGS, int_type, true)
            .add_type(at::REF, string_type, false)
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
    /// dml.read_file("#diagram ##circle id=circle vertices=0".as_bytes(), false).unwrap();
    /// # Ok::<(), diagram::MLErrorList<hml_rs::string::Position, hml_rs::string::Error>>(())
    /// ```
    pub fn new(style_set: &'a StyleSet) -> Self {
        let descriptors = HashMap::new();
        let fonts = HashMap::new();
        let mut descriptor = Self {
            style_set,
            descriptors,
            fonts,
        };
        Element::add_content_descriptors(&mut descriptor);
        descriptor
            .fonts
            .insert("default", Rc::new(RefCell::new(Font::default())));
        descriptor
    }

    //fp add_content_descriptors
    /// Invoked by element types to add descriptors to the set
    /// dependent on those descriptor values.
    ///
    /// This is only invoked by the `Element` type to add the required
    /// descriptors for the element types for styling.
    pub(super) fn add_content_descriptor(
        &mut self,
        name: el::Typ,
        include_hdr: bool,
        styles: Vec<&str>,
    ) {
        let mut descriptor = StyleDescriptor::new(self.style_set);
        if include_hdr {
            descriptor.add_styles(ElementHeader::get_style_names());
        }
        descriptor.add_styles(styles);
        self.descriptors.insert(name, descriptor);
    }

    //mp get
    /// Get the descriptor belonging to a tag name
    pub(crate) fn get(&self, tag: el::Typ) -> Option<&StyleDescriptor> {
        match self.descriptors.get(&tag) {
            Some(rrc) => Some(rrc),
            None => None,
        }
    }

    //mp get_font
    /// Get a font
    pub(crate) fn get_font(&self) -> RrcFont {
        self.fonts.get("default").unwrap().clone()
    }

    //zz All done
}
