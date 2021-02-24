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
use geometry::{Rectangle};
use stylesheet::{StylableNode, Tree};
use super::{DiagramDescriptor, StyleSheet};
use super::{Element, ElementScope, ElementError};
use crate::{Layout};
use super::types::*;

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
//tp DiagramContents
/// The contents of a diagram that are constructed; this is mutable
/// during construction, whereas other parts of a diagram are
/// immutable (such as its DiagramDescriptor).
pub struct DiagramContents<'a> {
    pub definitions       : Vec<Element<'a>>,
    pub markers           : Vec<Element<'a>>, // All these elements MUST be markers
    pub root_layout       : Option<Element<'a>>,
    pub content_bbox      : Rectangle,
}

//ip DiagramContents
impl <'a> DiagramContents<'a> {
    //fp new
    /// Create a new empty `DiagramContents`
    pub(self) fn new() -> Self {
        Self { definitions:Vec::new(),
               markers    :Vec::new(),
               root_layout:None,
               content_bbox : Rectangle::none(),
        }
    }

    //mp set_root_element
    pub fn set_root_element(&mut self, element:Element<'a>) {
        self.root_layout = Some(element);
    }
    
    //zz All done
}

//tp Diagram
/// This structure is used to construct and render a diagram.
///
/// It must be constructed using a DiagramDescriptor, which is
/// immutable once the Diagram is created.
///
/// Once constructed, contents are added to the diagram
pub struct Diagram<'a> {
    descriptor    : &'a DiagramDescriptor<'a>,
    pub(super) stylesheet    : StyleSheet<'a>,
    pub(super) contents      : DiagramContents<'a>,
}

//ti Diagram
impl <'a> Diagram <'a> {
    //fp new
    /// Create a new diagram using a `DiagramDescriptor` that has
    /// already been created.
    pub fn new(descriptor:&'a DiagramDescriptor) -> Self {
        let contents = DiagramContents::new();
        let stylesheet = StyleSheet::new(&descriptor.style_set);
        Self { descriptor,
               stylesheet,
               contents,
        }
    }

    //fp borrow_contents_descriptor
    /// Borrow the contents and descriptor to build the diagram contents
    ///
    /// The lifetime of the descriptor will be that of the diagram;
    /// the mutable borrow of the contents, required for building,
    /// will be for that of the caller, although the contents have a
    /// lifetime of the diagram.
    pub fn borrow_contents_descriptor<'z>(&'z mut self) -> (&'a DiagramDescriptor<'a>, &'z mut DiagramContents<'a>, &'z mut StyleSheet<'a>) {
        (&self.descriptor, &mut self.contents, &mut self.stylesheet)
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
        if let Some(element) = &mut self.contents.root_layout{
            element.uniquify(&scope)?;
        }
        for element in &mut self.contents.markers {
            element.uniquify(&scope)?;
        }
        Ok(())
    }

    //mp apply_stylesheet
    /// Apply the document's stylesheet
    ///
    /// This must be invoked after uniquify and before style.
    ///
    /// It updates the element's style attributes based on the
    /// stylesheet and its rules; the actually styling is then
    /// appllied in `style`.
    pub fn apply_stylesheet(&mut self) {
        let mut x = StylableNode::<'a, StyleValue>::new("diagram",self.descriptor.get("group").unwrap());
        let mut tree = Tree::new(&mut x);
        if let Some(element) = &mut self.contents.root_layout{
            tree = element.tree_add_element(tree);
        }
        for element in &mut self.contents.markers {
            tree = element.tree_add_element(tree);
        }
        tree.close_container();
        self.stylesheet.apply_rules_to_tree(&mut tree);
    }

    //mp style
    /// Style the contents of the diagram, using the stylesheet
    pub fn style(&mut self) -> Result<(),DiagramError> {
        if let Some(element) = &mut self.contents.root_layout{
            element.style(self.descriptor)?;
        }
        for element in &mut self.contents.markers {
            element.style(self.descriptor)?;
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
        if let Some(element) = &mut self.contents.root_layout {
            element.set_layout_properties(&mut layout);
        }
        // specify expansions
        let rect = {
            if within.is_none() {
                layout.get_desired_geometry()
            } else {
                *within
            }
        };

        self.contents.content_bbox = rect;
        if let Some(element) = &mut self.contents.root_layout {
            element.apply_placement(&layout);
        }

        for element in &mut self.contents.markers {
            let mut layout = Layout::new();
            element.set_layout_properties(&mut layout);
            element.apply_placement(&layout);
        }
        
        Ok(())
    }

    //mp geometry
    /// Resolve the geometry of the contents of the diagram based on
    /// how it has been laid out
    pub fn geometry(&mut self) -> Result<(),DiagramError> {
        Ok(())
    }

    //mp display
    /// Display the diagram in a human-parseable form, generally for debugging
    pub fn display(&self) {
        if let Some(element) = &self.contents.root_layout{
            element.display(0);
        }
        for element in &self.contents.markers {
            element.display(0);
        }
    }

    //zz All done
}

