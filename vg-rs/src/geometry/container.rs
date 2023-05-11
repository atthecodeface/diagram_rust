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

@file    mbox.rs
@brief   Part of SVG library
 */
//a Imports
use crate::geometry::{BBox, MBox};

//a Container
//tp Edge
#[derive(Debug, Clone, Copy, PartialEq)]
/// [MBox] describes margins (or padding) for a box; it is a pair of
/// Margin
pub enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

//tt ContainerBorder
pub trait ContainerBorder:
    Copy + std::default::Default + std::fmt::Debug + std::fmt::Display
{
    #[inline]
    fn get_mbox(&self) -> MBox {
        MBox::none()
    }
    #[inline]
    fn get_width(&self, edge: Edge) -> f64 {
        let mbox = self.get_mbox();
        match edge {
            Edge::Left => mbox.x.lx(),
            Edge::Right => mbox.x.rx(),
            Edge::Bottom => mbox.y.by(),
            Edge::Top => mbox.y.ty(),
        }
    }
}

//tp Container
pub struct Container<B: ContainerBorder> {
    /// Content BBox - padding, border and margin go around it (in that order)
    content_bbox: BBox,
    /// The margin is applied just inside the container
    pub margin: MBox,
    /// Border - which provides the width of the border on each edge
    ///
    /// border is *inside* the margin of the container
    pub border: B,
    /// Padding applies *within* the border
    pub padding: MBox,
}

//ip Default for Container
impl<B> std::default::Default for Container<B>
where
    B: ContainerBorder,
{
    fn default() -> Self {
        let content_bbox = BBox::none();
        let margin = MBox::none();
        let border = B::default();
        let padding = MBox::none();
        Self {
            content_bbox,
            margin,
            border,
            padding,
        }
    }
}

//ti Display for Container
impl<B> std::fmt::Display for Container<B>
where
    B: ContainerBorder,
{
    //mp fmt - format for a human
    /// Display the Container
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cont[{}][m:{} b:{} p:{}]",
            self.content_bbox, self.margin, self.border, self.padding
        )
    }

    //zz All done
}

//ti Container
impl<B> Container<B>
where
    B: ContainerBorder,
{
    //cp with_margin
    /// Set the margin
    #[must_use]
    #[inline]
    pub fn with_margin(mut self, margin: MBox) -> Self {
        self.margin = margin;
        self
    }

    //cp with_border
    /// Set the border
    #[must_use]
    #[inline]
    pub fn with_border(mut self, border: B) -> Self {
        self.border = border;
        self
    }

    //cp with_padding
    /// Set the padding
    #[must_use]
    #[inline]
    pub fn with_padding(mut self, padding: MBox) -> Self {
        self.padding = padding;
        self
    }

    //cp with_content_bbox
    /// Set the content bbox
    #[must_use]
    #[inline]
    pub fn with_content_bbox(mut self, content_bbox: BBox) -> Self {
        self.content_bbox = content_bbox;
        self
    }

    //ap bbox
    /// Get the *Container* bbox
    ///
    /// The container BBox contains margin, border, padding and content
    #[inline]
    pub fn bbox(&self) -> BBox {
        self.content_bbox + self.padding + self.border.get_mbox() + self.margin
    }

    //mp derive_content_bbox
    /// Derive the content BBox from a given outer bbox and the
    /// current properties
    pub fn derive_content_bbox(&mut self, bbox: BBox) {
        self.content_bbox = bbox - self.margin - self.border.get_mbox() - self.padding;
    }

    //mp set_margin
    /// Set the margin
    #[inline]
    pub fn set_margin(&mut self, margin: MBox) {
        self.margin = margin;
    }

    //mp set_border
    /// Set the border
    #[inline]
    pub fn set_border(&mut self, border: B) {
        self.border = border;
    }

    //mp set_padding
    /// Set the padding
    #[inline]
    pub fn set_padding(&mut self, padding: MBox) {
        self.padding = padding;
    }

    //mp set_content_bbox
    /// Set the content bbox
    #[inline]
    pub fn set_content_bbox(&mut self, content_bbox: BBox) {
        self.content_bbox = content_bbox;
    }

    //zz All done
}
