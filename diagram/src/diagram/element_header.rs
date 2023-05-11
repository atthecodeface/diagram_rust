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
const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use geometry::{Rectangle};
use stylesheet::{StylableNode};
use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::DiagramDescriptor;
use crate::{Layout, LayoutBox};
use super::types::*;
use super::{ElementLayout, LayoutPlacement};
use super::{ElementError};
use super::{ElementScope};

//a ElementHeader
//tp ElementHeader
#[derive(Debug)]
pub struct ElementHeader<'a> {
    pub stylable     : StylableNode<'a, StyleValue>,
    pub id_name      : Option<String>, // replicated from stylable
    pub layout_box   : LayoutBox,
    pub layout       : ElementLayout,
}

//ti ElementHeader
impl <'a> ElementHeader <'a> {
    //fp new
    pub fn new(descriptor:&'a DiagramDescriptor, name:el::Typ, name_values:&mut dyn Iterator<Item = (String, &str)>) -> Result<Self, ElementError> {
        if let Some(styles) = descriptor.get(name) { // &RrcStyleDescriptor
            let stylable = StylableNode::new(name.as_str(), styles);
            let id_name = None;
            let layout_box = LayoutBox::new();
            let layout     = ElementLayout::new();
            let mut hdr    = ElementHeader{ stylable, id_name, layout_box, layout };
            for (name,value) in name_values {
                let result = hdr.stylable.add_name_value(&name, value);
                ElementError::of_result(&hdr, result)?;
            }
            let id_name = hdr.stylable.borrow_id().map(|s| s.to_string());
            hdr.id_name = id_name;
            Ok(hdr)
        } else {
            Err(ElementError::Error("".to_string(),format!("Bug - unknown element descriptor {}",name)))
        }
    }

    //fp clone
    pub fn clone(&self, scope:&ElementScope) -> ElementHeader<'a> {
        let mut id_name = scope.id_prefix.clone();
        id_name.push_str(".");
        id_name.push_str(self.borrow_id());
        // println!("Clone header with new id {}", id_name);
        let stylable = self.stylable.clone(&id_name);
        let id_name = Some(id_name);
        let layout_box = LayoutBox::new();
        let layout = ElementLayout::new();
        ElementHeader{ stylable, id_name, layout_box, layout }
    }

    //mp get_style_names
    pub fn get_style_names<'z> () -> Vec<&'z str> {
        vec![at::DEBUG,
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
             at::BORDERROUND]
    }

    //mp override_values
    /// Override any values in the stylable that are set in 'other'
    /// This will be called before any stylesheet is invoked,
    /// basically at construction time
    ///
    /// This is invoked on the cloned element header, with 'other'
    /// being the header that may have overriding values. This may be
    /// the header for a 'use' element, for example.
    pub fn override_values<'z>(&mut self, other:&'z ElementHeader<'a>) -> Result<(),ElementError> {
        self.stylable.override_values( &other.stylable );
        Ok(())
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> &str {
        match &self.id_name {
            None => self.stylable.borrow_id().unwrap_or(""),
            Some(s) => s,
        }
    }

    //mp get_opt_style_value_of_name
    pub fn get_opt_style_value_of_name(&self, name:&str) -> Option<StyleValue> {
        let r = self.stylable.get_style_value_of_name(name).map(|a| a.clone());
        if DEBUG_ELEMENT_HEADER {println!("Debug {} {} {:?}", self.borrow_id(), name, r);}
        r
    }

    //mp get_style_rgb_of_name
    pub fn get_style_rgb_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::rgb(None),
            Some(value) => value,
        }
    }

    //mp get_style_ints_of_name
    pub fn get_style_ints_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::int_array(),
            Some(value) => value,
        }
    }

    //mp get_style_floats_of_name
    pub fn get_style_floats_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::float_array(),
            Some(value) => value,
        }
    }

    //mp get_style_strings_of_name
    pub fn get_style_strings_of_name(&self, name:&str) -> StyleValue {
        match self.get_opt_style_value_of_name(name) {
            None        => StyleValue::string_array("",true),
            Some(value) => value,
        }
    }

    //mp get_style_of_name_string
    pub fn get_style_of_name_string(&self, name:&str) -> Option<String> {
        match self.get_opt_style_value_of_name(name) {
            None        => None,
            Some(value) => value.as_string(),
        }
    }

    //mp get_style_of_name_float
    pub fn get_style_of_name_float(&self, name:&str, default:Option<f64>) -> Option<f64> {
        match self.get_opt_style_value_of_name(name) {
            None => default,
            Some(value) => value.as_float(default),
        }
    }

    //mp get_style_of_name_int
    pub fn get_style_of_name_int(&self, name:&str, default:Option<isize>) -> Option<isize> {
        match self.get_opt_style_value_of_name(name) {
            None => default,
            Some(value) => value.as_int(default),
        }
    }

    //mp style
    pub fn style(&mut self) -> Result<(),ElementError> {
        self.layout = ElementLayout::of_style(self)?;
        Ok(())
    }

    //mp set_layout_properties
    /// By this point layout_box has had its desired_geometry set
    pub fn set_layout_properties(&mut self, layout:&mut Layout, content_desired:Rectangle) -> Rectangle {
        self.layout.set_layout_box(&mut self.layout_box, content_desired);
        let bbox = self.layout_box.get_desired_bbox();
        self.layout.set_layout_properties(layout, bbox)
    }

    //mp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout:&Layout) -> Rectangle {
        let rect = {
            match self.layout.placement {
                LayoutPlacement::None              => self.layout_box.get_desired_bbox(),
                LayoutPlacement::Grid(sx,sy,ex,ey) => layout.get_grid_rectangle( (sx,sy), (ex,ey) ),
                LayoutPlacement::Place(pt)         => layout.get_placed_rectangle( &pt, &self.layout.ref_pt ),
            }
        };
        //println!("Laying out {:?} => {}",self.layout,rect);
        self.layout_box.layout_within_rectangle(rect);
        self.layout_box.get_content_rectangle()
    }

    //mp display
    pub fn display(&self, indent_str:&str) {
        println!("{}{:?} {}",indent_str, self.id_name, self.layout.placement);
        self.layout_box.display(indent_str);
    }
    //zz All done
}


