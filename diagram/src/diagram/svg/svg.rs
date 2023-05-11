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

@file    svg.rs
@brief   Generate SVG output
 */

//a Imports
use vg_rs::layout::LayoutRecord;
use vg_rs::BBox;

use super::super::Diagram;
use super::{ElementIter, GenerateSvg, SvgElement, SvgError};

//a Svg
//tp Svg
/// This structure is used to create SVG renderings of a `Diagram` It
/// should be constructed, and mutably borrowed by a diagram when it's
/// `generate_svg` method is invoked.
///
/// This method requires the `GenerateSvg` to be brought in to scope.
pub struct Svg<'a> {
    /// Diagram that the SVG is being built for
    pub diagram: &'a Diagram<'a>,
    /// version of SVG - 10, 11 or 20
    pub(super) version: usize,
    /// if asserted then show grid at the toplevel layout
    pub(super) show_grid: bool,
    /// if asserted then show layout of grids
    pub(super) show_layout: bool,
    /// if asserted then show content rectangles as translucent green rectangles
    pub(super) show_content_rectangles: bool,
    /// if asserted then display SVG elements to stdout
    pub(super) display: bool,
    /// Stack of elements being created
    stack: Vec<SvgElement>,
}

//ip Svg
impl<'a> Svg<'a> {
    //fp new
    /// Create a new `Svg` instance, to render a `Diagram` into
    pub fn new(diagram: &'a Diagram) -> Self {
        Self {
            diagram,
            version: 20,
            stack: Vec::new(),
            show_grid: false,
            show_layout: false,
            show_content_rectangles: false,
            display: false,
        }
    }

    //cp set_version
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of a version in to the SVG output
    pub fn set_version(mut self, version: usize) -> Self {
        self.version = version;
        self
    }

    //cp set_grid
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of a grid in to the SVG output
    pub fn set_grid(mut self, grid: bool) -> Self {
        self.show_grid = grid;
        self
    }

    //cp set_layout
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of lines indicating the `Layout`
    /// grids.
    pub fn set_layout(mut self, layout: bool) -> Self {
        self.show_layout = layout;
        self
    }

    //cp set_display
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the display to stdout of the Svg element hierarchy,
    /// once created from the diagram.
    pub fn set_display(mut self, display: bool) -> Self {
        self.display = display;
        self
    }

    //cp set_content_rectangles
    /// Used in a construction, to update the `Svg` instance to enable
    /// or disable the incorporation of a grid in to the SVG output
    pub fn set_content_rectangles(mut self, show: bool) -> Self {
        self.show_content_rectangles = show;
        self
    }

    //mp push_element
    pub(crate) fn push_element(&mut self, e: SvgElement) {
        self.stack.push(e);
    }
    //mp pop_element
    pub(crate) fn pop_element(&mut self) -> SvgElement {
        self.stack.pop().unwrap()
    }
    //mp add_subelement
    pub(crate) fn add_subelement(&mut self, e: SvgElement) {
        let n = self.stack.len();
        self.stack[n - 1].contents.push(e);
    }

    //mp generate_layout_recoredd_svg
    pub(crate) fn generate_layout_recorded_svg(
        &mut self,
        layout_record: &Option<LayoutRecord>,
    ) -> Result<(), SvgError> {
        if self.show_layout {
            if let Some(lr) = layout_record.as_ref() {
                lr.generate_svg(self)?;
            }
        }
        Ok(())
    }

    //mp generate_diagram
    /// Generate the SVG contents for the diagram
    pub fn generate_diagram(&mut self) -> Result<(), SvgError> {
        let contents = &self.diagram.contents;
        let mut ele = SvgElement::new("svg");
        ele.add_attribute("xmlns:svg", "http://www.w3.org/2000/svg");
        ele.add_attribute("xmlns", "http://www.w3.org/2000/svg");
        ele.add_attribute("version", &format!("{:.1}", (self.version as f64) / 10.));
        ele.add_attribute(
            "width",
            &format!(
                "{}mm",
                contents.content_bbox.x.max() - contents.content_bbox.x.min()
            ),
        );
        ele.add_attribute(
            "height",
            &format!(
                "{}mm",
                contents.content_bbox.y.max() - contents.content_bbox.y.min()
            ),
        );
        ele.add_attribute(
            "viewBox",
            &format!(
                "{} {} {} {}",
                contents.content_bbox.x.min(),
                contents.content_bbox.y.min(),
                contents.content_bbox.x.max() - contents.content_bbox.x.min(),
                contents.content_bbox.y.max() - contents.content_bbox.y.min(),
            ),
        );
        self.push_element(ele);

        let ele = SvgElement::new("defs");
        self.push_element(ele);

        for e in &contents.markers {
            e.generate_svg(self)?;
        }

        let ele = self.pop_element();
        self.add_subelement(ele);

        if self.show_grid {
            if let Some(ele) =
                SvgElement::new_grid(BBox::new(-100., -100., 100., 100.), 10., 0.1, "grey")
            {
                self.add_subelement(ele);
            }
        }

        if let Some(element) = &contents.root_layout {
            element.generate_svg(self)?;
        }

        if self.display {
            let ele = self.pop_element();
            ele.display(0);
            self.push_element(ele);
        }
        Ok(())
    }

    //mp iter_events
    /// Iterate over all the XML events the Svg would generate if it
    /// were an SVG file being read in by xml-rs
    ///
    /// This permits the SVG to be read by an XML reader, or written
    /// using xml-rs to convert reader XmlEvents to writer XmlEvents.
    pub fn iter_events<'z>(&'z self) -> ElementIter<'z> {
        ElementIter::new(&self.stack[0])
    }

    //zz All done
}
