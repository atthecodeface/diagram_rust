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
const DEBUG_ELEMENT_HEADER: bool = 1 == 0;

//a Imports
use indent_display::{IndentedDisplay, Indenter};
use vg_rs::layout::{Layout, LayoutBox};
use vg_rs::BBox;

use super::types::*;
use super::ElementError;
use super::ElementScope;
use super::{ElementLayout, LayoutPlacement};
use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::diagram::{StylableNode, StyleTypeValue};
use crate::DiagramDescriptor;

//a ElementHeader
//tp ElementHeader
#[derive(Debug)]
pub struct ElementHeader<'a> {
    pub uid: usize,
    pub stylable: StylableNode<'a>,
    pub id_name: Option<String>, // replicated from stylable
    pub layout_box: LayoutBox,
    pub layout: ElementLayout,
}

//ti ElementHeader
impl<'a> ElementHeader<'a> {
    //fp new
    pub fn new(
        descriptor: &'a DiagramDescriptor,
        name: el::Typ,
        name_values: &mut dyn Iterator<Item = (String, &str)>,
    ) -> Result<Self, ElementError> {
        if let Some(styles) = descriptor.get(name) {
            // &RrcStyleDescriptor
            let uid = 0;
            let stylable = StylableNode::new(name.as_str(), styles);
            let id_name = None;
            let layout_box = LayoutBox::new();
            let layout = ElementLayout::new();
            let mut hdr = ElementHeader {
                uid,
                stylable,
                id_name,
                layout_box,
                layout,
            };
            for (name, value) in name_values {
                let result = hdr.stylable.add_name_value(&name, value);
                ElementError::of_result(&hdr, result)?;
            }
            let id_name = hdr.stylable.borrow_id().map(|s| s.to_string());
            hdr.id_name = id_name;
            Ok(hdr)
        } else {
            Err(ElementError::Error(
                "".to_string(),
                format!("Bug - unknown element descriptor {}", name),
            ))
        }
    }

    //fp clone
    pub fn clone(&self, scope: &ElementScope) -> ElementHeader<'a> {
        let mut id_name = scope.id_prefix.clone();
        id_name.push_str(".");
        id_name.push_str(self.borrow_id());
        // println!("Clone header with new id {}", id_name);
        let uid = 0;
        let stylable = self.stylable.clone(&id_name);
        let id_name = Some(id_name);
        let layout_box = LayoutBox::new();
        let layout = ElementLayout::new();
        ElementHeader {
            uid,
            stylable,
            id_name,
            layout_box,
            layout,
        }
    }

    //mp set_uid
    pub fn set_uid(&mut self, uid: usize) {
        self.uid = uid;
    }

    //mp get_uid
    pub fn get_uid(&self) -> usize {
        self.uid
    }

    //mp get_style_names
    pub fn get_style_names<'z>() -> Vec<&'z str> {
        vec![
            at::DEBUG,
            at::BBOX,
            at::GRID,
            at::GRIDX,
            at::GRIDY,
            at::PLACE,
            at::ANCHOR,
            at::EXPAND,
            at::ROTATE,
            at::SCALE,
            at::TRANSLATE,
            at::PAD,
            at::MARGIN,
            at::BG,
            at::BORDERWIDTH,
            at::BORDERCOLOR,
            at::BORDERROUND,
        ]
    }

    //mp override_values
    /// Override any values in the stylable that are set in 'other'
    /// This will be called before any stylesheet is invoked,
    /// basically at construction time
    ///
    /// This is invoked on the cloned element header, with 'other'
    /// being the header that may have overriding values. This may be
    /// the header for a 'use' element, for example.
    pub fn override_values<'z>(
        &mut self,
        other: &'z ElementHeader<'a>,
    ) -> Result<(), ElementError> {
        self.stylable.override_values(&other.stylable);
        Ok(())
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        match &self.id_name {
            None => self.stylable.borrow_id().unwrap_or(""),
            Some(s) => s,
        }
    }

    //mp get_style_value_of_name
    pub fn get_style_value_of_name(&self, name: &str) -> Option<&StyleTypeValue> {
        self.stylable.get_style_value_of_name(name)
    }

    //mp get_opt_style_value_of_name
    pub fn get_opt_style_value_of_name(&self, name: &str) -> Option<StyleTypeValue> {
        let r = self
            .stylable
            .get_style_value_of_name(name)
            .map(|a| a.clone());
        if DEBUG_ELEMENT_HEADER {
            println!("Debug {} {} {:?}", self.borrow_id(), name, r);
        }
        r
    }

    //mp get_style_rgb_of_name
    pub fn get_style_rgb_of_name(&self, name: &str) -> StyleTypeValue {
        match self.get_opt_style_value_of_name(name) {
            None => StyleTypeValue::rgb(None),
            Some(value) => value,
        }
    }

    //mp get_style_ints_of_name
    pub fn get_style_ints_of_name(&self, name: &str) -> StyleTypeValue {
        match self.get_opt_style_value_of_name(name) {
            None => StyleTypeValue::int_array(),
            Some(value) => value,
        }
    }

    //mp get_style_floats_of_name
    pub fn get_style_floats_of_name(&self, name: &str) -> StyleTypeValue {
        match self.get_opt_style_value_of_name(name) {
            None => StyleTypeValue::float_array(),
            Some(value) => value,
        }
    }

    //mp get_style_strings_of_name
    pub fn get_style_strings_of_name(&self, name: &str) -> StyleTypeValue {
        match self.get_opt_style_value_of_name(name) {
            None => StyleTypeValue::string_array("", true),
            Some(value) => value,
        }
    }

    //mp get_style_of_name_string
    pub fn get_style_of_name_string(&self, name: &str) -> Option<String> {
        match self.get_opt_style_value_of_name(name) {
            None => None,
            Some(value) => value.as_str().map(|s| s.into()),
        }
    }

    //mp get_style_of_name_float
    pub fn get_style_of_name_float(&self, name: &str, default: Option<f64>) -> Option<f64> {
        self.get_style_value_of_name(name)
            .and_then(|value| value.as_f64())
            .or(default)
    }

    //mp get_style_of_name_int
    pub fn get_style_of_name_int(&self, name: &str, default: Option<isize>) -> Option<isize> {
        self.get_style_value_of_name(name)
            .and_then(|value| value.as_isize())
            .or(default)
    }

    //mp style
    pub fn style(&mut self) -> Result<(), ElementError> {
        self.layout = ElementLayout::of_style(self)?;
        Ok(())
    }

    //mp set_layout_properties
    /// This method is invoked to set the layout of this element in
    /// its parent given a desired geometry of the content.
    ///
    /// It invokes the layout
    pub fn set_layout_properties(&mut self, layout: &mut Layout, content_desired: BBox) {
        let eref = if let Some(name) = &self.id_name {
            format!("{} : {}", self.uid, name)
        } else {
            format!("{}", self.uid)
        };
        self.layout
            .set_layout_box(&eref, &mut self.layout_box, content_desired);
        let bbox = self.layout_box.get_desired_bbox();
        self.layout.set_layout_properties(&eref, layout, bbox);
    }

    //mp apply_placement
    /// This method is invoked with a resolved [Layout] which knows
    /// its real geometry and hence can map from a place or grid
    /// layout to a render rectangle.
    ///
    /// If the element requires any further layout, that should be
    /// performed.
    ///
    /// After this has been invoked the content for the element will
    /// have its placement applied using the [BBox] that this
    /// method returns.
    pub fn apply_placement(&mut self, layout: &Layout) -> BBox {
        let rect = {
            match &self.layout.placement {
                LayoutPlacement::None => self.layout_box.get_desired_bbox(),
                LayoutPlacement::Grid(sx, sy, ex, ey) => {
                    let sx = layout.find_grid_id(true, &sx).unwrap();
                    let sy = layout.find_grid_id(false, &sy).unwrap();
                    let ex = layout.find_grid_id(true, &ex).unwrap();
                    let ey = layout.find_grid_id(false, &ey).unwrap();
                    layout.get_grid_rectangle((*sx, *sy), (*ex, *ey))
                }
                LayoutPlacement::Place(pt) => layout.get_placed_rectangle(&pt, &self.layout.ref_pt),
            }
        };
        //println!("Laying out {:?} => {}",self.layout,rect);
        self.layout_box.layout_within_rectangle(rect);
        self.layout_box.get_content_rectangle()
    }

    //mp display
    pub fn display(&self, indent_str: &str) {
        println!("{}{}: {:?}", indent_str, self.uid, self.id_name);
        self.layout.display(indent_str);
        self.layout_box.display(indent_str);
    }
    //zz All done
}

//ti IndentedDisplay for ElementHeader
impl<'diag, 'a> IndentedDisplay<'a, IndentOptions> for ElementHeader<'diag> {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(ind, "{} : {:?}\n", self.uid, self.id_name)?;
        self.layout.indent(ind)?;
        self.layout_box.indent(ind)?;
        Ok(())
    }
}
