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

@file    text.rs
@brief   Text handling
 */

//a Imports
use super::font::*;
use std::cell::RefCell;
use std::rc::Rc;

//a Text span
#[derive(Debug)]
pub enum Bullet {
    Char(char),
    Numeral(usize),
    Alphabet(usize),
}
#[derive(Debug)]
pub struct TextSpan<F: FontMetrics> {
    font: Rc<RefCell<F>>,
    style: FontStyle,
    text: String, // single line
}

impl<F: FontMetrics> TextSpan<F> {
    pub fn new(text: &str, font: Rc<RefCell<F>>, style: FontStyle) -> Self {
        Self {
            font,
            style,
            text: text.to_string(),
        }
    }
    pub fn get_metrics(&self) -> TextMetrics {
        self.font.borrow().get_metrics(&self.text, &self.style)
    }
}

//a Text line
#[derive(Debug)]
pub struct TextLine<F: FontMetrics> {
    indent: usize,
    bullet: Option<Bullet>,
    spans: Vec<TextSpan<F>>,
}
impl<F: FontMetrics> TextLine<F> {
    pub fn new(indent: usize, bullet: Option<Bullet>) -> Self {
        Self {
            indent,
            bullet,
            spans: Vec::new(),
        }
    }
    pub fn add_span(&mut self, span: TextSpan<F>) {
        self.spans.push(span);
    }
    pub fn line_metrics(&self) -> f64 {
        (self.indent as f64) * 5.
    }
    pub fn get_metrics(&self) -> TextMetrics {
        let mut max_asc = 0.;
        let mut max_desc = 0.;
        let mut width = self.line_metrics();
        if self.bullet.is_some() {
            width = width + 7.;
        }
        for s in &self.spans {
            let tm = s.get_metrics();
            width += tm.width;
            if max_asc < tm.ascender {
                max_asc = tm.ascender;
            }
            if max_desc < tm.descender {
                max_desc = tm.descender;
            }
        }
        TextMetrics {
            width,
            ascender: max_asc,
            descender: max_desc,
        }
    }
}

//a Text area
//ip TextArea
#[derive(Debug)]
pub struct TextArea<F: FontMetrics> {
    lines: Vec<TextLine<F>>,
}

//ip TextArea
impl<F: FontMetrics> TextArea<F> {
    //fp new
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    //mp add_line
    pub fn add_line(&mut self, line: TextLine<F>) {
        self.lines.push(line);
    }

    //mp add_text
    pub fn add_text(&mut self, text: &str, font: Rc<RefCell<F>>, style: FontStyle) {
        for l in text.lines() {
            let mut bullet = None;
            let mut sl = &l[..];
            let mut indent = 0;
            while sl.starts_with("  ") {
                indent += 1;
                sl = &sl[2..];
            }
            if sl.starts_with("* ") {
                bullet = Some(Bullet::Char('*'));
                sl = &sl[2..];
            } else if sl.starts_with("+ ") {
                bullet = Some(Bullet::Char('+'));
                sl = &sl[2..];
            } else if sl.starts_with("# ") {
                bullet = Some(Bullet::Numeral(0));
                sl = &sl[2..];
            }
            let mut line = TextLine::new(indent, bullet);
            line.add_span(TextSpan::new(sl, font.clone(), style));
            self.add_line(line);
        }
    }

    //mp get_bbox
    pub fn get_bbox(&self) -> (f64, f64) {
        let mut width = 0.;
        let mut height = 0.;
        for l in &self.lines {
            let tm = l.get_metrics();
            if width < tm.width {
                width = tm.width;
            }
            height += tm.ascender + tm.descender;
        }
        (width, height)
    }

    //mp iter_spans
    pub fn iter_spans(&self) -> TextSpanIter<F> {
        TextSpanIter::new(self)
    }

    //zz All done
}

//a TextSpan iterator
//tp TextSpanIter
pub struct TextSpanIter<'a, F: FontMetrics> {
    area: &'a TextArea<F>,
    top_y: f64,
    x: f64,
    ascender: f64,
    descender: f64,
    line: usize,
    span: usize,
}

//ip TextSpanIter
impl<'a, F: FontMetrics> TextSpanIter<'a, F> {
    pub fn new(area: &'a TextArea<F>) -> Self {
        Self {
            area,
            top_y: 0.,
            x: 0.,
            line: 0,
            span: 0,
            ascender: 0.,
            descender: 0.,
        }
    }
}

#[derive(Debug)]
pub struct TextSpanElement<'a> {
    pub x: f64,
    pub y: f64,
    pub text: &'a str,
}

//ip Iterator for TextSpanIter
impl<'a, F: FontMetrics> Iterator for TextSpanIter<'a, F> {
    type Item = TextSpanElement<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.line >= self.area.lines.len() {
            None
        } else if self.span >= self.area.lines[self.line].spans.len() {
            self.span = 0;
            self.line += 1;
            self.top_y += self.ascender + self.descender;
            self.next()
        } else {
            if self.span == 0 {
                let lm = self.area.lines[self.line].line_metrics();
                let tm = self.area.lines[self.line].get_metrics();
                self.ascender = tm.ascender;
                self.descender = tm.descender;
                self.x = lm;
            }
            let i = self.span;
            self.span += 1;
            let span = &self.area.lines[self.line].spans[i];
            let tm = span.get_metrics();
            let y = self.top_y + self.ascender; // not sure where the text should go - tm.ascender;
            let x = self.x;
            self.x += tm.width;
            let text_span_element = TextSpanElement {
                x,
                y,
                text: &span.text,
            };
            Some(text_span_element)
        }
    }
}
