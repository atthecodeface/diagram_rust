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
use super::DiagramDescriptor;
use super::{Element, ElementScope, ElementError};
use crate::{Layout, LayoutRecord};
use crate::{Rectangle, Transform};

//a DiagramError
//tp DiagramError
pub enum DiagramError {
    Error(String),
}

//ip Display for DiagramError
impl std::fmt::Display for DiagramError {
    //mp fmt - format error for display
    /// Display the error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Error(s) => write!(f, "DiagramError {}", s),
        }
    }

    //zz All done
}

//ip From std::fmt::Display for DiagramError
/// Provides an implicit conversion from 
impl From<ElementError> for DiagramError {
    fn from(e: ElementError) -> DiagramError {
        DiagramError::Error(e.to_string())
    }
}


//a Diagram Definition
//tp Diagram
pub struct DiagramContents<'a> {
    pub definitions : Vec<Element<'a>>,
    pub elements    : Vec<Element<'a>>,
    pub content_transform : Transform,
    pub content_bbox      : Rectangle,
}
pub struct Diagram<'a> {
    pub descriptor    : &'a DiagramDescriptor<'a>,
    pub contents      : DiagramContents<'a>,
    pub layout_record : Option<LayoutRecord>, 
}

//ti Diagram
impl <'a> Diagram <'a> {
    //fp new
    pub fn new(descriptor:&'a DiagramDescriptor) -> Self {
        Self { descriptor,
               contents: DiagramContents{ definitions:Vec::new(),
                                          elements:Vec::new(),
                                          content_transform:Transform::new(),
                                          content_bbox : Rectangle::none(),
               },
               layout_record : None,
        }
    }

    //mp record_layout
    /// If there is no layout set for the diagram, then create one
    pub fn record_layout(&mut self) {
        match self.layout_record {
            None => { self.layout_record = Some(LayoutRecord::new()); },
            _ => (),
        }
    }

    //mp find_definition
    /// Find the definition of an id, if it exists in the contents
    /// 'definitions' section
    pub fn find_definition<'b>(&'b self, name:&str) -> Option<&'b Element<'a>> {
        for i in &self.contents.definitions {
            if i.has_id(name) {
                return Some(i);
            }
        }
        None
    }

    //mp uniquify
    /// Convert all 'use <id_ref>'s in to copies of the definition
    /// that has id==<id_ref>, uniquifying the contents within that
    /// definition too along with the ids therein
    pub fn uniquify(&mut self) -> Result<(),DiagramError> {
        let scope = ElementScope::new("", &self.contents.definitions);
        let n = self.contents.elements.len();
        for i in 0..n {
            self.contents.elements[i].uniquify(&scope)?
        }
        Ok(())
    }

    //mp style
    /// Style the contents of the diagram, using the stylesheet
    pub fn style(&mut self) -> Result<(),DiagramError> {
        for e in self.contents.elements.iter_mut() {
            e.style(self.descriptor)?;
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
    pub fn layout(&mut self, within:&Rectangle) -> Result<(),DiagramError> {
        let mut layout = Layout::new();
        for e in self.contents.elements.iter_mut() {
            e.set_layout_properties(&mut layout);
        }
        // specify expansions
        let mut rect = layout.get_desired_geometry();
        if !within.is_none() {
            rect = within.clone();
        }
        layout.layout(&rect);
        self.contents.content_transform = layout.get_layout_transform();
        self.contents.content_bbox = rect;
        // apply expansions - lay it out in a rectangle, generate transform?
        for e in self.contents.elements.iter_mut() {
            e.apply_placement(&layout);
        }
        if let Some(ref mut lr) = &mut self.layout_record {
            lr.capture_grid(&layout);
        }
        Ok(())
    }

    //mp geometry
    /// Resolve the geometry of the contents of the diagram based on
    /// how it has been laid out
    pub fn geometry(&mut self) -> Result<(),DiagramError> {
        Ok(())
    }

    //mp iter_elements
    /// Iterate over all the elements in the contents
    pub fn iter_elements<'b> (&'b self) -> DiagramElements<'a,'b> {
        DiagramElements { contents:&self.contents, n: 0 }
    }
    
    //mp display
    pub fn display(&self) {
        for e in self.iter_elements() {
            e.display(2);
        }
    }

    //zz All done
}

//tp DiagramElements
pub struct DiagramElements<'a, 'b> {
    contents : &'b DiagramContents<'a>,
    n : usize,
}

//ip Iterator for DiagramElements
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
