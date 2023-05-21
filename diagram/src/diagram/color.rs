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

@file    value.rs
@brief   The basics of a style value/type
 */

//a Imports
use std::any::Any;

use stylesheet::ValueError;

use stylesheet::TypeValue;
use vg_rs::Color as VgColor;
use vg_rs::{Rgba, COLOR_DB_SVG};

//a Color
//tp Color
#[derive(Debug, Clone)]
pub struct Color(VgColor);

//ip Default for Color
impl std::default::Default for Color {
    fn default() -> Self {
        // Defaut needs to be transparent
        Self(VgColor::new(None, 0xff_00_00_00_u32))
    }
}

//ip TypeValue for Color
impl TypeValue for Color {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.0)
    }
    fn type_name(&self) -> String {
        "Color".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let s = Self::default();
        Box::new(s)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = self.clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        4
    }
    fn is_none(&self) -> bool {
        self.0.rgba().is_transparent()
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        if self.is_none() {
            None
        } else {
            let (r, g, b, a) = self.0.rgba().as_tuple_rgba_f32();
            let n = data.len().max(4);
            if n > 0 {
                data[0] = r as f64
            };
            if n > 1 {
                data[1] = g as f64
            };
            if n > 2 {
                data[2] = b as f64
            };
            if n > 3 {
                data[3] = a as f64
            };
            Some(&data[0..n])
        }
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        if self.is_none() {
            None
        } else {
            let (r, g, b, a) = self.0.rgba().as_tuple_rgba();
            let n = data.len().max(4);
            if n > 0 {
                data[0] = r as isize
            };
            if n > 1 {
                data[1] = g as isize
            };
            if n > 2 {
                data[2] = b as isize
            };
            if n > 3 {
                data[3] = a as isize
            };
            Some(&data[0..n])
        }
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        if let Some(c) = COLOR_DB_SVG.find_color(s) {
            self.0 = c;
            Ok(())
        } else if let Some(rgba) = Rgba::of_str(s) {
            self.0 = VgColor::new(None, rgba);
            Ok(())
        } else {
            Err(ValueError::bad_value(format!(
                "failed to parse color '{}'",
                s
            )))
        }
    }
}
