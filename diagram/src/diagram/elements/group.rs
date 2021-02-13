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
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, Element, ElementScope, ElementHeader, ElementError};
use crate::{Layout, LayoutRecord};
use crate::{Rectangle};

//a Group element
//tp Group - an Element that contains just other Elements
/// The Group supplies simple grouping of elements
///
/// This element could have its own layout blob, if it is defined to be a layout entity
///
/// The elements that are part of this group must be created and moved
/// in to this group; the lifetime of the elements is the same as that
/// of the group.
#[derive(Debug)]
pub struct Group<'a> {
    /// The elements that are part of this group
    pub content : Vec<Element<'a>>,
    layout : Option<Layout>,
    layout_record : Option<LayoutRecord>,
}

//ip DiagramElementContent for Group
impl <'a, 'b> DiagramElementContent <'a, 'b> for Group<'a> {
    //fp new
    /// Create a new group
    fn new(_header:&ElementHeader, name:&str) -> Result<Self,ElementError> {
        let layout = {
            match name {
                "layout" => Some(Layout::new()),
                _ => None,
            }
        };
        // println!("Group created using name '{}' layout {:?}",  name, layout);
        Ok( Self {
            content:Vec::new(),
            layout,
            layout_record : None,
        } )
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, header:&ElementHeader<'a>, scope:&ElementScope<'a,'b> ) -> Result<Self,ElementError>{
        let mut clone = Self::new(header, "")?;
        if self.layout.is_some() {
            clone.layout = Some(Layout::new());
        }
        for e in &self.content {
            clone.content.push(e.clone(scope)?);
        }
        Ok(clone)
    }

    //mp uniquify
    /// Uniquify any content - and since this itself is unmodifed, return false
    fn uniquify(&mut self, _header:&ElementHeader<'a>, scope:&ElementScope<'a,'b>) -> Result<bool, ElementError> {
        for e in self.content.iter_mut() {
            e.uniquify(scope)?;
        }
        Ok(false)
    }

    //fp get_style_names
    /// Get the style descriptor for this element when referenced by the name
    ///
    /// Same descriptor is returned for 'layout' or for 'group'
    fn get_style_names<'z> (_name:&str) -> Vec<&'z str> {
        vec![]
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, _header:&ElementHeader) -> Result<(),ElementError> {
        for e in self.content.iter_mut() {
            e.style(descriptor)?;
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, layout:&mut Layout) -> Rectangle {
        if let Some(layout) = &mut self.layout {
            for e in self.content.iter_mut() {
                e.set_layout_properties(layout);
            }
            let rect = layout.get_desired_geometry();
            // println!("Group layout desires rectangle of {}", rect);
            rect
        } else {
            let mut rect = Rectangle::none();
            for e in self.content.iter_mut() {
                rect = rect.union(&e.set_layout_properties(layout));
            }
            rect
        }
    }

    //fp apply_placement
    fn apply_placement(&mut self, layout:&Layout, rect:&Rectangle) {
        if let Some(layout) = &mut self.layout {
            // println!("Lay out group within {}", rect);
            layout.layout(rect);
            for e in self.content.iter_mut() {
                e.apply_placement(layout);
            }
            match self.layout_record {
                None => {
                    let mut layout_record = LayoutRecord::new();
                    layout_record.capture_grid(&layout);
                    self.layout_record = Some(layout_record);
                },
                _ => (),
            }
        } else {
            for e in self.content.iter_mut() {
                e.apply_placement(layout);
            }
        }
    }
    
    //mp display
    /// Display - using indent_str + 2 indent, or an indent of indent spaces
    /// Content should be invoked with indent+4
    fn display(&self, indent:usize, indent_str:&str) {
        if let Some(layout) = &self.layout {
            println!("{}  layout",indent_str);
            println!("{}    {} : {} : {}",indent_str, layout.desired_grid, layout.desired_placement, layout.desired_geometry);
        } else {
            println!("{}  group only",indent_str);
        }
        for e in self.content.iter() {
            e.display(indent+4);
        }
    }

    //zz All done
}

//ip Group
impl <'a> Group<'a> {
    //mp add_element
    /// Add an element to the group; moves the element in to the content
    pub fn add_element(&mut self, element:Element<'a>) -> () {
        self.content.push(element);
    }
}

//ip GenerateSvgElement for Group
impl <'a> GenerateSvgElement for Group <'a> {
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError> {
        if let Some(_layout) = &self.layout {
            let mut ele = SvgElement::new("g");
            header.svg_add_transform(&mut ele);
            svg.push_element(ele);
            for e in &self.content {
                e.generate_svg( svg )?;
            }
            svg.generate_layout_recoved_svg( &self.layout_record )?;
            let ele = svg.pop_element();
            svg.add_subelement(ele);
        } else {
            for e in &self.content {
                e.generate_svg( svg )?;
            }
        }

        Ok(())
    }
}

