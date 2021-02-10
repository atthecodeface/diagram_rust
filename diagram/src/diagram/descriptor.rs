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
use super::types::*;
use super::element;

//a Diagram Descriptor - covers
//tp DiagramDescriptor - contains the StyleSet and StyleDescriptor's for each element type
pub struct DiagramDescriptor<'a> {
    style_set   : StyleSet,
    descriptors : HashMap<&'a str, RrcStyleDescriptor>,
}

//ti DiagramDescriptor
impl <'a> DiagramDescriptor<'a> {
    pub fn new() -> Self {
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
            .add_type("round",       StyleValue::float(None),   true)
            .add_type("markers",     StyleValue::string_array(),  true)
            .add_type("font",        StyleValue::string(None),  true)
            .add_type("fontsize",    StyleValue::float(None),   true)
            .add_type("fontweight",  StyleValue::string(None),  true)
            .add_type("fontstyle",   StyleValue::string(None),  true)
            .add_type("vertices",    StyleValue::int(None),     false)
            ;
        let mut descriptors = HashMap::new();
        descriptors.insert("use",   element::Use::get_descriptor(&style_set));
        descriptors.insert("group", element::Group::get_descriptor(&style_set));
        descriptors.insert("text",  element::Text::get_descriptor(&style_set));
        descriptors.insert("shape", element::Shape::get_descriptor(&style_set));
        Self {
            style_set,
            descriptors
        }
    }
    pub fn get(&self, tag:&str) -> Option<RrcStyleDescriptor> {
        match self.descriptors.get(tag)
        { Some(rrc) => Some(rrc.clone()), None => None}
    }
}
