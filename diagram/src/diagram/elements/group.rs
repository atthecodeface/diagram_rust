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
use super::super::super::layout::{GridData};
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
    minx  : Vec<GridData>,
    miny  : Vec<GridData>,
    growx : Vec<GridData>,
    growy : Vec<GridData>,
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
            minx  : Vec::new(),
            miny  : Vec::new(),
            growx : Vec::new(),
            growy : Vec::new(),
        } )
    }

    //fp clone
    /// Clone element given clone of header within scope
    fn clone(&self, _header:&ElementHeader<'a>, scope:&ElementScope<'a,'b> ) -> Result<Self,ElementError>{
        let layout = {if self.layout.is_some() {Some(Layout::new())} else {None}};
        let mut clone = Self {
            content:Vec::new(),
            layout,
            layout_record : None,
            minx  : Vec::new(),
            miny  : Vec::new(),
            growx : Vec::new(),
            growy : Vec::new(),
        };
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
    /// Layout supports minx/miny cell size descriptions
    fn get_style_names<'z> (name:&str) -> Vec<&'z str> {
        match name {
            "layout" => vec!["minx", "miny", "growx", "growy"],
            _ => vec![],
        }
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_floats_of_name("minx").as_floats(None) {
            self.minx = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name("miny").as_floats(None) {
            self.miny = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name("growx").as_floats(None) {
            self.growx = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name("growy").as_floats(None) {
            self.growy = self.read_cell_data(header, v)?;
        }
        if let Some(layout) = &mut self.layout {
            layout.grid_expand.0 = header.layout.expand.x;
            layout.grid_expand.1 = header.layout.expand.y;
        }
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
            layout.add_min_cell_data(&self.minx, &self.miny);
            layout.add_grow_cell_data(&self.growx, &self.growy);
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
            println!("{}    X grid {:?}",indent_str, layout.grid_placements.0 );
            println!("{}    Y grid {:?}",indent_str, layout.grid_placements.1 );
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
    //mp read_cell_data
    /// For styling, this uses an array of floats and attempts to produce an array of GridData,
    /// which provides the start/end/size for cells
    ///
    /// For the data to be valid it should be of the form
    /// int,(float,int)*, describing spacing between cell boundaries
    /// with increasing cell numbers.
    ///
    /// Hence the data must be an odd number, of elements, with even
    /// indices being integers, and the indices monotically
    /// increasing, and the floats all positive or zero.
    pub fn read_cell_data(&self, header:&ElementHeader, v:&Vec<f64>) -> Result<Vec<GridData>, ElementError> {
        if v.len() % 2 == 0 {
            Err(ElementError::of_string(header, &format!("grid minimums must be int,(float,int)* and hence and odd number of items, but got {} items", v.len())))
        } else {
            fn as_int(header:&ElementHeader, x:f64, n:usize) -> Result<isize, ElementError> {
                let x_i = x as isize;
                if x - (x_i as f64) == 0. {
                    Ok(x_i)
                } else {
                    Err(ElementError::of_string(header, &format!("grid boundaries must be integers, but got {} for cell boundary {}", x, n)))
                }
            }
            let mut n = 1;
            let mut start = as_int(header, v[0], 1)?;
            let mut result = Vec::new();
            while n*2 <= v.len() {
                let size = v[n*2-1];
                let end = as_int(header, v[n*2], n+1)?;
                result.push(GridData::new(start, end, size));
                if end <= start {
                    Err(ElementError::of_string(header, &format!("grid boundaries must increase left to right, but got {} followed by {}",start,end)))?;
                }
                start = end;
                n += 1;
            }
            Ok(result)
        }
    }
                                                                                 
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

