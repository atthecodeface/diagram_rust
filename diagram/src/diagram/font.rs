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

//a FontMetrics trait
#[derive(Debug)]
pub struct TextMetrics {
    pub width : f64,
    pub descender : f64,
    pub ascender : f64,
}
pub trait FontMetrics {
    fn get_metrics(&self, text:&str, style:&FontStyle) -> TextMetrics;
}

//a Font
#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    size : f64, // in points
    flags : usize, // italic, bold
}
impl FontStyle {
    pub fn new(size:f64, _weight:Option<&String>, _style:Option<&String>) -> Self {
        let flags = 0;
        Self { size, flags }
    }
}

#[derive(Debug)]
pub struct Font {
}

impl Font {
    pub fn default() -> Self{
        Self {
        }
    }
}

impl FontMetrics for Font {
    fn get_metrics(&self, text:&str, style:&FontStyle) -> TextMetrics {
        let size = style.size * 25.4 / 72.0;
        let width = (text.len() as f64) * size * 0.5;
        let height = size;
        let ascender = height * 1.1;
        let descender = height * 0.3;
        TextMetrics { width, descender, ascender }
    }
}
