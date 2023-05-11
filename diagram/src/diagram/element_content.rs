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

@file    element.rs
@brief   Diagram elements
 */

//a Constants
// const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use indent_display::{IndentedDisplay, Indenter};
use stylesheet::{StylableNode, Tree};
use vg_rs::layout::Layout;
use vg_rs::BBox;

pub use super::elements::{Group, Path, Shape, Text, Use};
use super::types::*;
use super::DiagramElementContent;
use super::Element;
use super::ElementError;
use super::ElementHeader;
use super::ElementScope;
use super::IndentOptions;
use crate::constants::elements as el;
use crate::DiagramDescriptor;

//a ElementContent - enumerated union of the above
//tp ElementContent
#[derive(Debug)]
pub enum ElementContent<'a> {
    /// Group is used for Marker, Layout and Group
    Group(Group<'a>),
    /// Text is used for all text boxes
    Text(Text),
    /// Shape is used for circles, polygons, rectangles
    Shape(Shape),
    /// Path is used for custom shapes
    Path(Path),
    /// Use describes a reference to a defined element
    Use(Use<'a>), // use of a definition
}

//ti ElementContent
impl<'a> ElementContent<'a> {
    //fp new
    pub fn new(header: &ElementHeader<'a>, name: el::Typ) -> Result<Self, ElementError> {
        match name {
            el::Typ::Diagram => Ok(Self::Group(Group::new(&header, name)?)),
            el::Typ::Group => Ok(Self::Group(Group::new(&header, name)?)),
            el::Typ::Layout => Ok(Self::Group(Group::new(&header, name)?)),
            el::Typ::Marker => Ok(Self::Group(Group::new(&header, name)?)),
            el::Typ::Path => Ok(Self::Path(Path::new(&header, name)?)),
            el::Typ::Rect => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::Typ::Circle => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::Typ::Polygon => Ok(Self::Shape(Shape::new(&header, name)?)),
            el::Typ::Text => Ok(Self::Text(Text::new(&header, name)?)),
            el::Typ::Use => Ok(Self::Use(Use::new(&header, name)?)),
            _ => ElementError::of_result(
                &header,
                Err(format!("Bug - bad element name {}", name.as_str())),
            ),
        }
    }

    //mp uniquify
    /// Generates a *replacement* Content if required.
    ///
    /// This is for a 'use' content, which should have an id_ref that
    /// identifies and element in `scope`.  This will return Ok(true),
    /// if it uniquifies the use content reference; in doing so it
    /// must clone the relevant element and push its header
    /// name/values down in to the cloned content header.
    ///
    /// If the immediate element content, then recurse through any
    /// subcontent, and return Ok(false)
    pub fn uniquify<'b, 'c>(
        &'c mut self,
        header: &ElementHeader<'a>,
        scope: &ElementScope<'a, 'b>,
        uid: usize,
    ) -> Result<(bool, usize), ElementError> {
        match self {
            Self::Use(ref mut c) => c.uniquify(header, scope, uid),
            Self::Group(ref mut c) => c.uniquify(header, scope, uid),
            _ => Ok((false, uid)),
        }
    }

    //mp clone
    pub fn clone<'b>(
        &self,
        header: &ElementHeader<'a>,
        scope: &ElementScope<'a, 'b>,
    ) -> Result<Self, ElementError> {
        match self {
            Self::Group(ref c) => Ok(Self::Group(ElementError::of_result(
                &header,
                c.clone(header, scope),
            )?)),
            Self::Shape(ref c) => Ok(Self::Shape(ElementError::of_result(
                &header,
                c.clone(header, scope),
            )?)),
            Self::Path(ref c) => Ok(Self::Path(ElementError::of_result(
                &header,
                c.clone(header, scope),
            )?)),
            Self::Text(ref c) => Ok(Self::Text(ElementError::of_result(
                &header,
                c.clone(header, scope),
            )?)),
            Self::Use(ref c) => Ok(Self::Use(ElementError::of_result(
                &header,
                c.clone(header, scope),
            )?)),
        }
    }

    //mp add_element
    pub fn add_element(&mut self, element: Element<'a>) {
        match self {
            Self::Group(ref mut c) => {
                c.add_element(element);
            }
            _ => (),
        }
    }

    //mp add_string
    pub fn add_string(&mut self, header: &ElementHeader, s: &str) -> Result<(), ElementError> {
        match self {
            Self::Text(ref mut c) => ElementError::of_result(header, c.add_string(s)),
            Self::Use(ref mut c) => ElementError::of_result(header, c.add_string(s)),
            _ => Ok(()), // could error - bug in code
        }
    }

    //mp borrow_group
    pub fn borrow_group<'z>(&'z self) -> Option<&'z Group<'a>> {
        match self {
            Self::Group(ref g) => Some(g),
            _ => None,
        }
    }

    //fp tree_add_element
    pub fn tree_add_element<'b>(
        &'b mut self,
        tree: Tree<'b, StylableNode<'a, StyleValue>>,
    ) -> Tree<'b, StylableNode<'a, StyleValue>> {
        match self {
            Self::Group(ref mut g) => g.tree_add_element(tree),
            Self::Use(ref mut g) => g.tree_add_element(tree),
            _ => tree,
        }
    }

    //mp style
    pub fn style(
        &mut self,
        descriptor: &DiagramDescriptor,
        header: &ElementHeader,
    ) -> Result<(), ElementError> {
        match self {
            Self::Shape(ref mut s) => s.style(descriptor, header),
            Self::Path(ref mut s) => s.style(descriptor, header),
            Self::Group(ref mut g) => g.style(descriptor, header),
            Self::Text(ref mut t) => t.style(descriptor, header),
            Self::Use(ref mut t) => t.style(descriptor, header),
        }
    }

    //mp get_desired_geometry
    pub fn get_desired_geometry(&mut self, layout: &mut Layout) -> BBox {
        match self {
            Self::Shape(ref mut s) => s.get_desired_geometry(layout),
            Self::Path(ref mut s) => s.get_desired_geometry(layout),
            Self::Group(ref mut g) => g.get_desired_geometry(layout),
            Self::Text(ref mut t) => t.get_desired_geometry(layout),
            Self::Use(ref mut t) => t.get_desired_geometry(layout),
        }
    }

    //fp apply_placement
    /// The layout contains the data required to map a grid or placement layout of the element
    ///
    /// Note that `layout` is that of the parent layout (not the group this is part of, for example)
    ///
    /// If the element requires any further layout, that should be performed; certainly its
    /// transformation should be determined
    pub fn apply_placement(&mut self, layout: &Layout, rect: &BBox) {
        match self {
            Self::Path(ref mut g) => g.apply_placement(layout, rect),
            Self::Group(ref mut g) => g.apply_placement(layout, rect),
            Self::Use(ref mut g) => g.apply_placement(layout, rect),
            _ => (),
        }
    }

    //mp display
    pub fn display(&self, indent: usize, indent_str: &str) {
        match self {
            Self::Shape(ref s) => {
                println!("{}  Shape", indent_str);
                s.display(indent, indent_str);
            }
            Self::Path(ref s) => {
                println!("{}  Path", indent_str);
                s.display(indent, indent_str);
            }
            Self::Group(ref g) => {
                println!("{}  Group", indent_str);
                g.display(indent, indent_str);
            }
            Self::Text(ref t) => {
                println!("{}  Text", indent_str);
                t.display(indent, indent_str);
            }
            Self::Use(ref t) => {
                println!("{}  Use", indent_str);
                t.display(indent, indent_str);
            }
        }
    }

    //zz All done
}

//ti IndentedDisplay for ElementContent
impl<'a, 'diag> IndentedDisplay<'a, IndentOptions> for ElementContent<'diag> {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        match self {
            Self::Shape(s) => s.indent(ind),
            Self::Path(s) => s.indent(ind),
            Self::Group(g) => g.indent(ind),
            Self::Text(t) => t.indent(ind),
            Self::Use(t) => t.indent(ind),
        }
    }
}
