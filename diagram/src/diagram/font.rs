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

//a FontMetrics trait
//tp TextMetrics
/// A simple metrics definition: it is not meant to cover complex
/// typography, as the diagram is not performing the full font
/// rendering. Rather it is a good approximation to the bounding box
/// for the text given it is placed at a baseline Y of 0.
///
/// This is used to describe the bounding box for a particular text span.
#[derive(Debug)]
pub struct TextMetrics {
    /// Width in mm of the text
    pub width: f64,
    /// Descent below the baseline for the font
    pub descender: f64,
    /// Ascent above the baseline for the font
    pub ascender: f64,
}

//tt FontMetrics
/// A trait for a Font type class that provides the font metrics for a given text given a font style
pub trait FontMetrics {
    fn get_metrics(&self, text: &str, style: &FontStyle) -> TextMetrics;
}

//a Font
//tp FontStyle
/// A font style as a size in points and flags for font styling options
#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    size: f64,    // in points
    flags: usize, // italic, bold
}

//ip FontStyle
impl FontStyle {
    //fp new
    /// Create a new simple font style
    pub fn new(size: f64, _weight: Option<&String>, _style: Option<&String>) -> Self {
        let flags = 0;
        Self { size, flags }
    }
}

//tp Metrics
pub trait Value:
    Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
{
    fn zero() -> Self;
}

/// This structure provides simple metrics for a font or a region of
/// characters in a font. It is based on the TeX Font Metrics.
#[derive(Debug, Clone, Copy)]
pub struct CharIndices(u32);
impl std::fmt::Display for CharIndices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
impl CharIndices {
    fn of_indices(
        width: usize,
        height: usize,
        depth: usize,
        italic: usize,
        options: usize,
    ) -> Self {
        assert!(width < 256);
        assert!(height < 16);
        assert!(depth < 16);
        assert!(italic < 64);
        assert!(options < 1024);
        let v = (options << 22) | (italic << 16) | (depth << 12) | (height << 8) | width;
        Self(v as u32)
    }
    pub fn width_index(&self) -> usize {
        (self.0 & 0xff) as usize
    }
    pub fn height_index(&self) -> usize {
        ((self.0 >> 8) & 0xf) as usize
    }
    pub fn depth_index(&self) -> usize {
        ((self.0 >> 12) & 0xf) as usize
    }
    pub fn italic_index(&self) -> usize {
        ((self.0 >> 16) & 0x3f) as usize
    }
    pub fn options(&self) -> usize {
        ((self.0 >> 22) & 0x3ff) as usize
    }
    pub fn width<V: Value>(&self, metrics: &Metrics<V>) -> V {
        metrics.get_width(self.width_index())
    }
    pub fn height<V: Value>(&self, metrics: &Metrics<V>) -> V {
        metrics.get_height(self.height_index())
    }
    pub fn depth<V: Value>(&self, metrics: &Metrics<V>) -> V {
        metrics.get_depth(self.depth_index())
    }
    pub fn italic<V: Value>(&self, metrics: &Metrics<V>) -> V {
        metrics.get_italic(self.italic_index())
    }
}

//tp Parameter
/// Font metric parameters
#[derive(Debug, Copy, Clone)]
pub enum Parameter<V: Value> {
    /// Size of a space in the font (standard gap between words)
    Space(V),
    /// Size of an 'em' in the font (length of an em-dash, not necessarily the width of 'M')
    Em(V),
    /// Space after a period at the end of a sentence
    PunctSpace(V),
    // x height
    // cap height
    // ascent
    // descent
}
#[derive(Debug)]
pub struct Metrics<V: Value> {
    /// First Unicode Scalar Value represented by these metrics (inclusive)
    first_char: char,
    /// Last Unicode Scalar Value represented by these metrics (inclusive)
    last_char: char,
    /// Widths - at most 256 long, with zeroth element of 0
    widths: Vec<V>,
    /// Heights - at most 16 long, with zeroth element of 0
    heights: Vec<V>,
    /// Depths - at most 16 long, with zeroth element of 0
    depths: Vec<V>,
    /// Italic - at most 64 long, with zeroth element of 0
    italics: Vec<V>,
    /// Character metrics - as indices in to the above vectors, for characters from first_char to last_char
    char_metrics: Vec<CharIndices>,
    /// parameters, sorted by the parameter order for faster indexing
    parameters: Vec<Parameter<V>>,
    /// Exceptions to the metrics provided here - allowing for more than 16 heights, 256 widths, etc.
    exceptions: Vec<Metrics<V>>,
}

pub struct GlyphMetrics<V: Value> {
    width: V,
    height: V,
    depth: V,
    italic: V,
    options: usize,
}
impl<V: Value> GlyphMetrics<V> {
    pub fn zero() -> Self {
        Self {
            width: V::zero(),
            height: V::zero(),
            depth: V::zero(),
            italic: V::zero(),
            options: 0,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            width: self.width + other.width,
            height: if self.height > other.height {
                self.height
            } else {
                other.height
            },
            depth: if self.depth > other.depth {
                self.depth
            } else {
                other.depth
            },
            italic: other.italic,
            options: other.options,
        }
    }
}
impl<V: Value> Metrics<V> {
    pub fn new_monospace(width: V, height: V, depth: V, italic: V) -> Self {
        let first_char = '\0';
        let last_char = '\0';
        let widths = vec![width];
        let heights = vec![height];
        let depths = vec![depth];
        let italics = vec![italic];
        let char_metrics = vec![CharIndices::of_indices(0, 0, 0, 0, 0)];
        let parameters = Vec::new();
        let exceptions = Vec::new();
        Self {
            first_char,
            last_char,
            widths,
            heights,
            depths,
            italics,
            char_metrics,
            parameters,
            exceptions,
        }
    }
    pub fn get_width(&self, index: usize) -> V {
        assert!(index < self.widths.len());
        self.widths[index]
    }
    pub fn get_height(&self, index: usize) -> V {
        assert!(index < self.heights.len());
        self.heights[index]
    }
    pub fn get_depth(&self, index: usize) -> V {
        assert!(index < self.depths.len());
        self.depths[index]
    }
    pub fn get_italic(&self, index: usize) -> V {
        assert!(index < self.italics.len());
        self.italics[index]
    }
    pub fn get_glyph_metrics(&self, index: usize) -> GlyphMetrics<V> {
        let ci = self.char_metrics[index];
        let width = ci.width(self);
        let height = ci.height(self);
        let depth = ci.depth(self);
        let italic = ci.italic(self);
        let options = ci.options();
        GlyphMetrics {
            width,
            height,
            depth,
            italic,
            options,
        }
    }
    pub fn borrow_metrics_of_char(&self, c: char) -> Option<(&Metrics<V>, usize)> {
        if c < self.first_char || c > self.last_char {
            None
        } else {
            for e in &self.exceptions {
                if let Some(m) = e.borrow_metrics_of_char(c) {
                    return Some(m);
                }
            }
            Some((self, ((c as u32) - (self.first_char as u32)) as usize))
        }
    }
    pub fn glyph_metrics(&self, c: char) -> GlyphMetrics<V> {
        if let Some((m, i)) = self.borrow_metrics_of_char(c) {
            m.get_glyph_metrics(i)
        } else {
            self.get_glyph_metrics(0)
        }
    }
}

//tp Font
impl Value for f32 {
    fn zero() -> Self {
        0.0
    }
}
impl Value for f64 {
    fn zero() -> Self {
        0.0
    }
}
/// This structure provides simple metric storage for
#[derive(Debug)]
pub struct Font {
    metrics: Metrics<f64>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            metrics: Metrics::new_monospace(0.5, 1.1, 0.3, 0.),
        }
    }
}

impl FontMetrics for Font {
    fn get_metrics(&self, text: &str, style: &FontStyle) -> TextMetrics {
        let mut gm = GlyphMetrics::zero();
        for c in text.chars() {
            // if a space, add metrics.space?
            gm = gm.add(&self.metrics.glyph_metrics(c));
        }
        let size = style.size * 25.4 / 72.0;
        let width = gm.width * size;
        let ascender = gm.height * size;
        let descender = gm.depth * size;
        TextMetrics {
            width,
            descender,
            ascender,
        }
    }
}
