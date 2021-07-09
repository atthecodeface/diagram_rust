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

@file    font.rs
@brief   Font and text handling
 */

//a Imports
#![allow(dead_code)]
mod traits;
mod parameter;
mod glyph_metrics;
mod font_metrics;
mod font;
pub use traits::*;
use parameter::Parameter;
use glyph_metrics::GlyphMetrics;
use font_metrics::Metrics as FontMetrics;

//a Font
//tp FontStyle
/// A font style as a size in points and flags for font styling options
#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    size : f64, // in points
    flags : usize, // italic, bold
}

//ip FontStyle
impl FontStyle {
    //fp new
    /// Create a new simple font style
    pub fn new(size:f64, weight:Option<&str>, _style:Option<&str>) -> Self {
        let weight_flags = {
            match weight {
                Some("bold") |
                Some("Bold") => { 1 },
                _ => 0,
            }
        };
        let style_flags = {
            match weight {
                Some("italic") |
                Some("Italic") => { 1 },
                _ => 0,
            }
        };
        let flags = weight_flags + style_flags;
        Self { size, flags }
    }
}

