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
use stylesheet::{StylableNode, Tree};
use geometry::{Rectangle, Point};

use crate::constants::attributes as at;
use crate::constants::elements   as el;
use super::super::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use super::super::{DiagramDescriptor, DiagramElementContent, Element, ElementScope, ElementHeader, ElementError};
use crate::{Layout, LayoutRecord};

use super::super::super::layout::{GridData};
use super::super::types::*;

//a Group element
//tp GroupType
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GroupType {
    Marker,
    Layout,
    Group,
}

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
    /// Group
    group_type : GroupType,
    /// The elements that are part of this group
    pub content : Vec<Element<'a>>,
    layout : Option<Layout>,
    layout_record : Option<LayoutRecord>,
    minx  : Vec<GridData>,
    miny  : Vec<GridData>,
    growx : Vec<GridData>,
    growy : Vec<GridData>,
    bbox : Rectangle,

    // For markers ONLY
    // Reference point - where the 'end' of the marker is in its content
    pub ref_pt:Point,
    // Relief - amount of relief to apply to start and end in terms of stroke-width units
    pub relief : (f64, f64),
    /// Flags - other than default
    /// Default is orient auto, markerunits strokeWidth
    pub flags:usize,
    // Width - width in the parent to squish the viewbox into - if not given, uses 1
    pub width :f64,
    // Height - height in the parent to squish the viewbox into - if not given, uses width
    pub height :f64,
}

//ip DiagramElementContent for Group
impl <'a, 'b> DiagramElementContent <'a, 'b> for Group<'a> {
    //fp new
    /// Create a new group
    fn new(_header:&ElementHeader, name:&str) -> Result<Self,ElementError> {
        let (group_type, layout) = {
            match name {
                el::GROUP  => (GroupType::Group,  None),
                el::MARKER => (GroupType::Marker, Some(Layout::new())),
                _ =>          (GroupType::Layout, Some(Layout::new())),
            }
        };
        // println!("Group created using name '{}' layout {:?}",  name, layout);
        Ok( Self {
            group_type,
            content:Vec::new(),
            layout,
            layout_record : None,
            minx  : Vec::new(),
            miny  : Vec::new(),
            growx : Vec::new(),
            growy : Vec::new(),
            bbox  : Rectangle::none(),
            ref_pt : Point::origin(), // for markers
            relief : (0.,0.),
            flags : 0,
            width : 0.,
            height : 0.,
        } )
    }

    //fp clone
    /// Clone element given clone of header within scope
    /// This is called *before* the element is styled, so contents may be basically empty
    fn clone(&self, _header:&ElementHeader<'a>, scope:&ElementScope<'a,'b> ) -> Result<Self,ElementError>{
        let layout = {if self.layout.is_some() {Some(Layout::new())} else {None}};
        let mut clone = Self {
            group_type : self.group_type,
            content:Vec::new(),
            layout,
            layout_record : None,
            minx  : Vec::new(),
            miny  : Vec::new(),
            growx : Vec::new(),
            growy : Vec::new(),
            bbox  : Rectangle::none(),
            ref_pt  : Point::origin(), // for markers
            relief : (0.,0.),
            flags : 0,
            width : 0.,
            height : 0.,
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
            el::GROUP => vec![],
            el::LAYOUT => vec![at::MINX,
                      at::MINY,
                      at::GROWX,
                      at::GROWY],
            _ => vec![at::MINX,
                      at::MINY,
                      at::GROWX,
                      at::GROWY,
                      at::POINT,
                      at::RELIEF,
                      at::WIDTH,
                      at::HEIGHT,
                      at::FLAGS,
                      ],
        }
    }

    //mp style
    /// Style the element within the Diagram's descriptor, using the
    /// header if required to extract styles
    fn style(&mut self, descriptor:&DiagramDescriptor, header:&ElementHeader) -> Result<(),ElementError> {
        if let Some(v) = header.get_style_floats_of_name(at::MINX).as_floats(None) {
            self.minx = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name(at::MINY).as_floats(None) {
            self.miny = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name(at::GROWX).as_floats(None) {
            self.growx = self.read_cell_data(header, v)?;
        }
        if let Some(v) = header.get_style_floats_of_name(at::GROWY).as_floats(None) {
            self.growy = self.read_cell_data(header, v)?;
        }
        if let Some(layout) = &mut self.layout {
            layout.grid_expand.0 = header.layout.expand.x;
            layout.grid_expand.1 = header.layout.expand.y;
        }
        // width, height, flags, ref point are only used in markers
        match header.get_style_floats_of_name(at::POINT).as_floats(None) {
            Some(g) => {
                match g.len() {
                    0 => {},
                    1 => { self.ref_pt = Point::new(g[0], g[0]); },
                    _ => { self.ref_pt = Point::new(g[0], g[1]); },
                }
            },
            _ => {},
        }
        match header.get_style_floats_of_name(at::RELIEF).as_floats(None) {
            Some(g) => {
                match g.len() {
                    0 => {},
                    1 => { self.relief = (g[0], g[0]); },
                    _ => { self.relief = (g[0], g[1]); },
                }
            },
            _ => {},
        }
        if let Some(i) = header.get_style_of_name_int(at::FLAGS, None) {
            self.flags = i as usize;
        }
        self.width    = header.get_style_of_name_float(at::WIDTH,Some(1.)).unwrap();
        self.height   = header.get_style_of_name_float(at::HEIGHT,Some(self.width)).unwrap();
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
            self.bbox = rect;
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
            println!("{}    X grid", indent_str);
            layout.grid_placements.0.display(indent_str);
            println!("{}    Y grid", indent_str);
            layout.grid_placements.1.display(indent_str);
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
    //mp tree_add_element
    pub fn tree_add_element<'b>(&'b mut self, mut tree:Tree<'b, StylableNode<'a, StyleValue>>) -> Tree<'b, StylableNode<'a, StyleValue>>{
        for c in self.content.iter_mut() {
            tree = c.tree_add_element(tree);
        }
        tree
    }
    //mp get_relief
    pub fn get_relief(&self, index:usize) -> f64 {
        if index == 0 { self.relief.0 } else { self.relief.1 }
    }
    
    //zz All done
}

//ip GenerateSvgElement for Group
impl <'a> GenerateSvgElement for Group <'a> {
    fn generate_svg(&self, svg:&mut Svg, header:&ElementHeader) -> Result<(), SvgError> {
        let is_marker = self.group_type == GroupType::Marker;
        // let marker_stroke = {
        //     match svg.version {
        //         20 => "context-stroke",
        //         _ => "black",
        //     }
        // };
        if is_marker {
            let mut ele = SvgElement::new("marker");
            header.svg_add_transform(&mut ele);
            ele.add_attribute("viewBox",
                              &format!("{} {} {} {}",
                                       self.bbox.x0,
                                       self.bbox.y0,
                                       self.bbox.x1-self.bbox.x0,
                                       self.bbox.y1-self.bbox.y0,));
            ele.add_size("refX", self.ref_pt.x);
            ele.add_size("refY", self.ref_pt.y);
            ele.add_size("markerWidth",  self.width  );
            ele.add_size("markerHeight", self.height );
            ele.add_attribute("markerUnits","strokeWidth");
            ele.add_attribute("orient", "auto");
            svg.push_element(ele);
        }
        if let Some(_layout) = &self.layout {
            let mut ele = SvgElement::new("g");
            if !is_marker { header.svg_add_transform(&mut ele); }
            svg.push_element(ele);
            for e in &self.content {
                e.generate_svg( svg )?;
            }
            svg.generate_layout_recorded_svg( &self.layout_record )?;
            let ele = svg.pop_element();
            svg.add_subelement(ele);
        } else {
            for e in &self.content {
                e.generate_svg( svg )?;
            }
        }
        if self.group_type == GroupType::Marker {
            let ele = svg.pop_element();
            svg.add_subelement(ele);
        }

        Ok(())
    }
}


