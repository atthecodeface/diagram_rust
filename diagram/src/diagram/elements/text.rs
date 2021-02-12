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

@file    group.rs
@brief   Diagram group element
 */

//a Imports
use super::super::{GenerateSvg, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, ElementHeader, ElementError};
use crate::{Layout};
use crate::{Rectangle};
use super::super::types::*;
use super::super::font::*;
use super::super::text::*;

//a TextError
//tp TextError
pub enum TextError {
    None
}

//ip Display for TextError
impl std::fmt::Display for TextError {
    //mp fmt - format error for display
    /// Display the error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TextError")
    }

    //zz All done
}

//a Text element
//tp Text - an Element that contains text
#[derive(Debug)]
pub struct Text {
    pub fill        : Option<(f64,f64,f64)>,
    pub font        : Option<String>,
    pub font_style  : Option<String>,
    pub font_weight : Option<String>,
    pub font_size   : f64,
    pub text        : Vec<String>,
    pub text_area   : TextArea<Font>,
}

//ip DiagramElementContent for Text
impl DiagramElementContent for Text {
    //fp new
    fn new(_header:&ElementHeader, _name:&str) -> Result<Self,ValueError> {
        Ok( Self {
            fill : None,
            text:Vec::new(),
            font : None,
            font_style : None,
            font_weight : None,
            font_size : 10.,
            text_area : TextArea::new(),
        } )
    }

    //fp get_descriptor
    fn get_descriptor(nts:&StyleSet, _name:&str) -> RrcStyleDescriptor {
        let desc = ElementHeader::get_descriptor(nts);
        // tab stops, bullets, alignment
        desc.borrow_mut().add_styles(nts, vec!["fill", "font", "fontsize", "fontweight", "fontstyle"]);
        desc
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_rgb_of_name("fill").as_floats(None) {
            self.fill = Some((v[0],v[1],v[2]));
        }
        self.font         = header.get_style_of_name_string("font");
        self.font_weight  = header.get_style_of_name_string("fontweight");
        self.font_style   = header.get_style_of_name_string("fontstyle");
        self.font_size = header.get_style_of_name_float("fontsize",Some(10.)).unwrap();
        // let height   = header.get_style_of_name_float("height",Some(width)).unwrap();
        let style = FontStyle::new(self.font_size, self.font_weight.as_ref(), self.font_style.as_ref());
        let font = descriptor.get_font();
        for t in &self.text {
            self.text_area.add_text(t, font.clone(), style);
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, _layout:&mut Layout) -> Rectangle {
        let (w,h) = self.text_area.get_bbox();
        Rectangle::new(0.,0.,w,h,)
    }

    //zz All done
}

//ip Text
impl Text {
    //mp add_string
    pub fn add_string(&mut self, s:&str) -> Result<(),TextError> {
        self.text.push(s.to_string());
        Ok(())
    }
}


//ip GenerateSvg for Text
impl GenerateSvg for Text {
    fn generate_svg(&self, svg:&mut Svg) -> Result<(), SvgError> {
        let font_size = self.font_size / 72.0 * 25.4;
        for t in self.text_area.iter_spans() {
            let mut ele = SvgElement::new("text");
            match &self.fill {
                None      => {ele.add_attribute("fill","None");},
                Some(rgb) => {ele.add_color("fill",rgb);},
            }
            ele.add_size("x",t.x);
            ele.add_size("y",t.y);
            ele.add_size("font-size",font_size);
            if let Some(f) = &self.font        { ele.add_attribute("font-family", f); }
            if let Some(f) = &self.font_style  { ele.add_attribute("font-style", f); }
            if let Some(f) = &self.font_weight { ele.add_attribute("font-weight", f); }
            ele.add_string(t.text);
            svg.add_subelement(ele);
        }
        Ok(())
    }
}

