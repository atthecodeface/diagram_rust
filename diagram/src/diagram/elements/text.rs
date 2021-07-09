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
use crate::{Layout};
use geometry::{Rectangle};
use crate::constants::attributes as at;
use crate::constants::elements as el;
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, ElementScope, ElementHeader, ElementError};
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
impl <'a, 'b> DiagramElementContent <'a, 'b> for Text {
    //fp new
    fn new(_header:&ElementHeader, _name:el::Typ) -> Result<Self,ElementError> {
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

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header:&ElementHeader, _scope:&ElementScope ) -> Result<Self,ElementError>{
        let mut clone = Self::new(header, el::Typ::Clone)?;
        for s in &self.text {
            clone.text.push(s.clone());
        }
        Ok(clone)
    }

    //fp get_style_names
    fn get_style_names<'z> (_name:&str) -> Vec<&'z str> {
        vec![at::FILL,
             at::FONT,
             at::FONTSIZE,
             at::FONTWEIGHT,
             at::FONTSTYLE]
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_rgb_of_name(at::FILL).as_floats(None) {
            self.fill = Some((v[0],v[1],v[2]));
        }
        self.font         = header.get_style_of_name_string(at::FONT);
        self.font_weight  = header.get_style_of_name_string(at::FONTWEIGHT);
        self.font_style   = header.get_style_of_name_string(at::FONTSTYLE);
        self.font_size = header.get_style_of_name_float(at::FONTSIZE,Some(10.)).unwrap();
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

    //mp display
    /// Display - using indent_str + 2 indent, or an indent of indent spaces
    /// Content should be invoked with indent+4
    fn display(&self, _indent:usize, indent_str:&str) {
        println!("{}    font {}",indent_str, self.font.as_ref().unwrap_or(&"".to_string()));
        for t in &self.text {
            println!("{}     '{}'",indent_str, t);
        }
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


//ip GenerateSvgElement for Text
impl GenerateSvgElement for Text {
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError> {
        let font_size = self.font_size / 72.0 * 25.4;
        for t in self.text_area.iter_spans() {
            let mut ele = SvgElement::new("text");
            header.svg_add_transform(&mut ele);
            match &self.fill {
                None      => {ele.add_attribute("fill","None");},
                Some(rgb) => {ele.add_color("fill",rgb);},
            }
            ele.add_size("x",t.x);
            ele.add_size("y",t.y);
            ele.add_size("font-size",font_size);
            ele.add_attribute("stroke","None"); // ImageMagic will stroke it otherwise
            let mut style = String::new();
            if let Some(f) = &self.font        { style.push_str(&format!("font-family:{};",f)); }
            if let Some(f) = &self.font_style  { style.push_str(&format!("font-style:{};",f)); }
            if let Some(f) = &self.font_weight { style.push_str(&format!("font-weight:{};",f)); }
            if style != "" {
                ele.add_attribute("style", &style);
            }
            ele.add_string(t.text);
            svg.add_subelement(ele);
        }
        Ok(())
    }
}

