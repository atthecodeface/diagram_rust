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
pub struct DiagramDescriptor<'a> {
    style_set   : &'a StyleSet,
    descriptors : HashMap<&'a str, StyleDescriptor<'a>>,
    fonts       : HashMap<&'a str, RrcFont>,
}

//ti DiagramDescriptor
impl <'a> DiagramDescriptor<'a> {
    pub fn create_style_set() -> StyleSet {
        let style_set = StyleSet::new()
            .add_type("bbox",        StyleValue::float_array(), false)            
            .add_type("grid",        StyleValue::int_array(),   false)
            .add_type("place",       StyleValue::float_array(), false)
            .add_type("pad",         StyleValue::floats(4),     false)
            .add_type("margin",      StyleValue::floats(4),     false)
            .add_type("border",      StyleValue::float(None),   false)
            .add_type("borderround", StyleValue::float(None),   false)
            .add_type("bordercolor", StyleValue::rgb(None),     false)
            .add_type("bg",          StyleValue::rgb(None),     false)
            .add_type("scale",       StyleValue::float(None),   false)
            .add_type("rotate",      StyleValue::float(None),   false)
            .add_type("translate",   StyleValue::floats(2),     false)
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
            .add_type("vertices",    StyleValue::int(None),     false)
            .add_type("ref",         StyleValue::string(None),  false)
            ;
        style_set
    }
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
    pub fn add_content_descriptor(&mut self, name:&'static str, styles:Vec<&str>) {
        let mut descriptor = StyleDescriptor::new(&self.style_set);
        descriptor.add_styles(ElementHeader::get_style_names());
        descriptor.add_styles(styles);
        self.descriptors.insert(name, descriptor);
    }

    pub fn get(&self, tag:&str) -> Option<&StyleDescriptor> {
        match self.descriptors.get(tag)
        { Some(rrc) => Some(rrc.clone()), None => None}
    }
    pub fn get_font(&self) -> RrcFont {
        self.fonts.get("default").unwrap().clone()
    }
}
