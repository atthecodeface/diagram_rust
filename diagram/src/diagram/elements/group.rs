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
use geo_nd::Vector;
use indent_display::{IndentedDisplay, Indenter};
use stylesheet::{StylableNode, Tree};
use vg_rs::grid::GridData;
use vg_rs::layout::{Layout, LayoutRecord};
use vg_rs::{BBox, Point};

use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::diagram::{DiagramElementContent, Element, ElementError, ElementHeader, ElementScope};
use crate::diagram::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
use crate::{DiagramDescriptor, IndentOptions};

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
    group_type: GroupType,
    /// The elements that are part of this group
    pub content: Vec<Element<'a>>,
    layout: Option<Layout>,
    layout_record: Option<LayoutRecord>,
    x_cell_data: Vec<GridData<usize>>,
    y_cell_data: Vec<GridData<usize>>,
    bbox: BBox,

    // For markers ONLY
    // Reference point - where the 'end' of the marker is in its content
    pub ref_pt: Point,
    // Relief - amount of relief to apply to start and end in terms of stroke-width units
    pub relief: (f64, f64),
    /// Flags - other than default
    /// Default is orient auto, markerunits strokeWidth
    pub flags: usize,
    // Width - width in the parent to squish the viewbox into - if not given, uses 1
    pub width: f64,
    // Height - height in the parent to squish the viewbox into - if not given, uses width
    pub height: f64,
}

//ip DiagramElementContent for Group
impl<'a, 'b> DiagramElementContent<'a, 'b> for Group<'a> {
    //fp new
    /// Create a new group
    fn new(_header: &ElementHeader, name: el::Typ) -> Result<Self, ElementError> {
        let (group_type, layout) = {
            match name {
                el::Typ::Group => (GroupType::Group, None),
                el::Typ::Marker => (GroupType::Marker, Some(Layout::default())),
                _ => (GroupType::Layout, Some(Layout::default())),
            }
        };
        // println!("Group created using name '{}' layout {:?}",  name, layout);
        Ok(Self {
            group_type,
            content: Vec::new(),
            layout,
            layout_record: None,
            x_cell_data: Vec::new(),
            y_cell_data: Vec::new(),
            bbox: BBox::none(),
            ref_pt: Point::zero(), // for markers
            relief: (0., 0.),
            flags: 0,
            width: 0.,
            height: 0.,
        })
    }

    //fp clone
    /// Clone element given clone of header within scope
    /// This is called *before* the element is styled, so contents may be basically empty
    fn clone(
        &self,
        _header: &ElementHeader<'a>,
        scope: &ElementScope<'a, 'b>,
    ) -> Result<Self, ElementError> {
        let layout = {
            if self.layout.is_some() {
                Some(Layout::default())
            } else {
                None
            }
        };
        let mut clone = Self {
            group_type: self.group_type,
            content: Vec::new(),
            layout,
            layout_record: None,
            x_cell_data: Vec::new(),
            y_cell_data: Vec::new(),
            bbox: BBox::none(),
            ref_pt: Point::zero(), // for markers
            relief: (0., 0.),
            flags: 0,
            width: 0.,
            height: 0.,
        };
        for e in &self.content {
            clone.content.push(e.clone(scope)?);
        }
        Ok(clone)
    }

    //mp uniquify
    /// Uniquify any content - and since this itself is unmodifed, return false
    fn uniquify(
        &mut self,
        _header: &ElementHeader<'a>,
        scope: &ElementScope<'a, 'b>,
        mut uid: usize,
    ) -> Result<(bool, usize), ElementError> {
        for e in self.content.iter_mut() {
            uid = e.uniquify(scope, uid)?;
        }
        Ok((false, uid))
    }

    //fp get_style_names
    /// Get the style descriptor for this element when referenced by the name
    ///
    /// Layout supports minx/miny cell size descriptions
    fn get_style_names<'z>(name: &str) -> Vec<&'z str> {
        match name {
            el::GROUP => vec![],
            el::LAYOUT => vec![at::MINX, at::MINY],
            _ => vec![
                at::MINX,
                at::MINY,
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
    fn style(
        &mut self,
        descriptor: &DiagramDescriptor,
        header: &ElementHeader,
    ) -> Result<(), ElementError> {
        if let Some(v) = header
            .get_style_value_of_name(at::MINX)
            .and_then(|x| x.as_vec_str())
        {
            self.x_cell_data = self.read_cell_data(true, header, v)?;
        }
        if let Some(v) = header
            .get_style_value_of_name(at::MINY)
            .and_then(|x| x.as_vec_str())
        {
            self.y_cell_data = self.read_cell_data(false, header, v)?;
        }
        if let Some(layout) = &mut self.layout {
            layout.set_grid_expand(true, header.layout.expand[0]);
            layout.set_grid_expand(false, header.layout.expand[1]);
        } else {
            if header.layout.expand[0] != 0. {
                return Err(ElementError::of_string(header, &format!("X Expand {} specified for element id {} but it is not directly part of a Layout", header.layout.expand[0], header.borrow_id())));
            }
            if header.layout.expand[1] != 0. {
                return Err(ElementError::of_string(header, &format!("Y Expand {} specified for element id {} but it is not directly part of a Layout", header.layout.expand[1], header.borrow_id())));
            }
        }
        // width, height, flags, ref point are only used in markers
        let mut floats = [0.; 4];
        match header
            .get_style_value_of_name(at::POINT)
            .and_then(|x| x.as_floats(&mut floats))
        {
            Some(g) => match g.len() {
                0 => {}
                1 => {
                    self.ref_pt = Point::from_array([g[0], g[0]]);
                }
                _ => {
                    self.ref_pt = Point::from_array([g[0], g[1]]);
                }
            },
            _ => {}
        }
        match header
            .get_style_value_of_name(at::RELIEF)
            .and_then(|x| x.as_floats(&mut floats))
        {
            Some(g) => match g.len() {
                0 => {}
                1 => {
                    self.relief = (g[0], g[0]);
                }
                _ => {
                    self.relief = (g[0], g[1]);
                }
            },
            _ => {}
        }
        if let Some(i) = header.get_style_of_name_int(at::FLAGS, None) {
            self.flags = i as usize;
        }
        self.width = header.get_style_of_name_float(at::WIDTH, Some(1.)).unwrap();
        self.height = header
            .get_style_of_name_float(at::HEIGHT, Some(self.width))
            .unwrap();
        for e in self.content.iter_mut() {
            e.style(descriptor)?;
        }
        Ok(())
    }

    //mp get_desired_geometry
    fn get_desired_geometry(&mut self, layout: &mut Layout) -> BBox {
        if let Some(layout) = &mut self.layout {
            for e in self.content.iter_mut() {
                e.set_layout_properties(layout);
            }
            layout.add_cell_data(&self.x_cell_data, &self.y_cell_data);
            let rect = layout.get_desired_geometry();
            self.bbox = rect;
            // println!("Group layout desires rectangle of {}", rect);
            rect
        } else {
            dbg!("Should this get the content rectangle?");
            for e in self.content.iter_mut() {
                e.set_layout_properties(layout);
            }
            BBox::none()
        }
    }

    //fp apply_placement
    fn apply_placement(&mut self, layout: &Layout, rect: &BBox) {
        if let Some(layout) = &mut self.layout {
            // println!("Lay out group within {}", rect);
            layout.layout(rect);
            for e in self.content.iter_mut() {
                e.apply_placement(layout);
            }
            match self.layout_record {
                None => {
                    let mut layout_record = LayoutRecord::default();
                    match layout_record.capture_grid(&layout) {
                        Err(x) => {
                            eprintln!("Warning: failed to apply grid layout {}", x);
                        }
                        _ => (),
                    }
                    self.layout_record = Some(layout_record);
                }
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
    fn display(&self, indent: usize, indent_str: &str) {
        if let Some(layout) = &self.layout {
            println!("{}  layout", indent_str);
            println!("{}    X grid", indent_str);
            layout.grid_placements(true).display(indent_str);
            println!("{}    Y grid", indent_str);
            layout.grid_placements(false).display(indent_str);
        } else {
            println!("{}  group only", indent_str);
        }
        for e in self.content.iter() {
            e.display(indent + 4);
        }
    }

    //zz All done
}

//ip Group
fn parse_float<'a>(
    header: &ElementHeader,
    s: &'a str,
    ofs: usize,
) -> Result<(&'a str, f64), ElementError> {
    let s = if ofs > 0 {
        let (_, s) = s.split_at(ofs);
        s
    } else {
        s
    };
    let mut seen_point = false;
    let mut end_index = None;
    for (n, c) in s.char_indices() {
        if !(char::is_numeric(c) || (c == '.' && !seen_point)) {
            end_index = Some(n);
            break;
        }
        if c == '.' {
            seen_point = true;
        }
    }
    let (s, ns) = {
        if let Some(end_index) = end_index {
            s.split_at(end_index)
        } else {
            (s, "")
        }
    };
    match s.parse::<f64>() {
        Err(x) => Err(ElementError::of_string(
            header,
            &format!("Failed to parse float {}: {}", ns, x),
        )),
        Ok(v) => Ok((ns, v)),
    }
}

impl<'a> Group<'a> {
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
    pub fn read_cell_data(
        &mut self,
        x: bool,
        header: &ElementHeader,
        v: Vec<&str>,
    ) -> Result<Vec<GridData<usize>>, ElementError> {
        if self.layout.is_none() {
            return Err(ElementError::of_string(
                header,
                &format!(
                    "Cell data layout specified for id {} but it is not a Layout element",
                    header.borrow_id()
                ),
            ));
        }
        let layout = self.layout.as_mut().unwrap();
        let mut result = Vec::new();
        let mut last_element = 0;
        let mut expecting_data = false;
        let mut pending_min_size = None;
        let mut pending_growth = None;
        for s in v {
            let mut has_data = false;
            if expecting_data {
                let mut s = s.trim_start();
                while !s.is_empty() {
                    let opt_c = s.chars().next();
                    if opt_c == Some('+') {
                        let (ns, v) = parse_float(header, s, 1)?;
                        s = ns.trim_start();
                        pending_growth = Some(v);
                        has_data = true;
                    } else if opt_c == Some('=') {
                        let (ns, v) = parse_float(header, s, 1)?;
                        s = ns.trim_start();
                        result.push(GridData::new_place(last_element, v));
                        has_data = true;
                    } else {
                        match parse_float(header, s, 0) {
                            Err(e) if has_data => {
                                return Err(e);
                            }
                            Err(_) => {
                                break;
                            }
                            Ok((ns, v)) => {
                                s = ns.trim_start();
                                pending_min_size = Some(v);
                                has_data = true;
                            }
                        }
                    }
                }
            }
            if has_data {
                expecting_data = false;
            } else {
                if s.find(|c: char| (c == '+' || c == '.' || c == '='))
                    .is_some()
                {
                    return Err(ElementError::of_string(header, &format!("grid cell data hit an id of '{}' which contained +, = or ., which are illegal in a grid cell id", s)));
                }
                let e = layout.add_grid_id(x, s.trim());
                if let Some(size) = pending_min_size {
                    result.push(GridData::new_width(last_element, e, size));
                }
                if let Some(size) = pending_growth {
                    result.push(GridData::new_growth(last_element, e, size));
                }
                expecting_data = true;
                last_element = e;
                pending_min_size = None;
                pending_growth = None;
            }
        }
        Ok(result)
    }

    //mp add_element
    /// Add an element to the group; moves the element in to the content
    pub fn add_element(&mut self, element: Element<'a>) -> () {
        self.content.push(element);
    }
    //mp tree_add_element
    pub fn tree_add_element<'b>(
        &'b mut self,
        mut tree: Tree<'b, StylableNode<'a>>,
    ) -> Tree<'b, StylableNode<'a>> {
        for c in self.content.iter_mut() {
            tree = c.tree_add_element(tree);
        }
        tree
    }
    //mp get_relief
    pub fn get_relief(&self, index: usize) -> f64 {
        if index == 0 {
            self.relief.0
        } else {
            self.relief.1
        }
    }

    //zz All done
}

//ip GenerateSvgElement for Group
impl<'a> GenerateSvgElement for Group<'a> {
    fn generate_svg(&self, svg: &mut Svg, header: &ElementHeader) -> Result<(), SvgError> {
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
            ele.add_attribute(
                "viewBox",
                &format!(
                    "{} {} {} {}",
                    self.bbox.x.min(),
                    self.bbox.y.min(),
                    self.bbox.x.max() - self.bbox.x.min(),
                    self.bbox.y.max() - self.bbox.y.min(),
                ),
            );
            ele.add_size("refX", self.ref_pt[0]);
            ele.add_size("refY", self.ref_pt[1]);
            ele.add_size("markerWidth", self.width);
            ele.add_size("markerHeight", self.height);
            ele.add_attribute("markerUnits", "strokeWidth");
            ele.add_attribute("orient", "auto");
            svg.push_element(ele);
        }
        if let Some(_layout) = &self.layout {
            let mut ele = SvgElement::new("g");
            if !is_marker {
                header.svg_add_transform(&mut ele);
            }
            svg.push_element(ele);
            for e in &self.content {
                e.generate_svg(svg)?;
            }
            svg.generate_layout_recorded_svg(&self.layout_record)?;
            let ele = svg.pop_element();
            svg.add_subelement(ele);
        } else {
            for e in &self.content {
                e.generate_svg(svg)?;
            }
        }
        if self.group_type == GroupType::Marker {
            let ele = svg.pop_element();
            svg.add_subelement(ele);
        }

        Ok(())
    }
}

//ti IndentedDisplay for Group
impl<'a, 'diag> IndentedDisplay<'a, IndentOptions> for Group<'diag> {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        use std::fmt::Write;
        match self.group_type {
            GroupType::Marker => write!(ind, "Marker\n")?,
            GroupType::Group => write!(ind, "Group\n")?,
            GroupType::Layout => write!(ind, "Content\n")?,
        }

        let mut sub = ind.sub();
        write!(&mut sub, "bbox       : {}\n", self.bbox)?;
        write!(&mut sub, "ref_pt     : {}\n", self.ref_pt)?;
        write!(
            &mut sub,
            "relief     : {}, {}\n",
            self.relief.0, self.relief.1
        )?;
        write!(&mut sub, "flags      : {}\n", self.flags)?;
        write!(&mut sub, "width      : {}\n", self.width)?;
        write!(&mut sub, "height     : {}\n", self.height)?;
        write!(&mut sub, "x cell data\n")?;
        {
            let mut sub = sub.sub();
            for gd in &self.x_cell_data {
                write!(&mut sub, "{}\n", gd)?;
            }
        }
        write!(&mut sub, "y cell data\n")?;
        {
            let mut sub = sub.sub();
            for gd in &self.y_cell_data {
                write!(&mut sub, "{}\n", gd)?;
            }
        }

        for (i, e) in self.content.iter().enumerate() {
            write!(&mut sub, "Element {}\n", i + 1)?;
            let mut sub = sub.sub();
            e.indent(&mut sub)?;
        }
        // pub content : Vec<Element<'a>>,
        // layout : Option<Layout>,
        // layout_record : Option<LayoutRecord>,

        Ok(())
    }
}
