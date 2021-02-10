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

@file    mod.rs
@brief   Diagram module
 */

//a Imports
use super::types::*;
use super::DiagramDescriptor;
use super::Element;
use crate::Layout;
use crate::{Rectangle, Polygon, Point, Transform};

//a Diagram Definition
//tp Diagram
pub struct DiagramContents<'a> {
    pub definitions : Vec<Element<'a>>,
    pub elements    : Vec<Element<'a>>,
    pub content_transform : Transform,
}
pub struct Diagram<'a> {
    pub descriptor  : DiagramDescriptor<'a>,
    pub contents    : DiagramContents<'a>,
}

//ti Diagram
impl <'a> Diagram <'a> {
    pub fn new() -> Self {
        Self { descriptor: DiagramDescriptor::new(),
               contents: DiagramContents{ definitions:Vec::new(),
                                          elements:Vec::new(),
                                          content_transform:Transform::new(),
               },
        }
    }
    pub fn find_definition<'b>(&'b self, name:&str) -> Option<&'b Element<'a>> {
        for i in &self.contents.definitions {
            if i.has_id(name) {
                return Some(i);
            }
        }
        None
    }
    pub fn uniquify(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn style(&mut self) -> Result<(),()> {
        for e in self.contents.elements.iter_mut() {
            e.style();
        }
        Ok(())
    }

    //mp layout
    /// Lay out the diagram (within a bbox?)
    ///
    /// The diagram is a layout element by its nature,
    /// and so the process is as for a layout element.
    ///
    /// This is to create a `Layout`, and find the desired geometry
    /// and placement/layout properties of all of the contents.
    ///
    /// The `Layout` element can then be laid out within the required
    /// bbox, which will generate the positions of the grid elements,
    /// and so on
    ///
    pub fn layout(&mut self, within:&Rectangle) -> Result<(),()> {
        let mut layout = Layout::new();
        for e in self.contents.elements.iter_mut() {
            e.set_layout_properties(&mut layout);
        }
        // specify expansions
        layout.get_desired_geometry();
        layout.layout(within);
        self.contents.content_transform = layout.get_layout_transform();
        // apply expansions - lay it out in a rectangle, generate transform?
        for e in self.contents.elements.iter_mut() {
            e.apply_placement(&layout);
        }
        println!("{:?}", layout);
        Ok(())
    }
    pub fn geometry(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn iter_elements<'b> (&'b self) -> DiagramElements<'a,'b> {
        DiagramElements { contents:&self.contents, n: 0 }
    }
}
pub struct DiagramElements<'a, 'b> {
    contents : &'b DiagramContents<'a>,
    n : usize,
}
impl <'a, 'b> Iterator for DiagramElements<'a, 'b> {
    type Item = &'b Element<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n>=self.contents.elements.len() {
            None
        } else {
            let i=self.n;
            self.n = self.n + 1;
            Some(&self.contents.elements[i])
        }
    }
}
