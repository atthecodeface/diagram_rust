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
use super::ElementError;
use super::ElementHeader;
use super::IndentOptions;
use crate::constants::attributes as at;
use crate::{Layout, LayoutBox};
use geo_nd::Vector;
use geometry::{Point, Rectangle};
use indent_display::{IndentedDisplay, Indenter};

//a ElementLayout
//tp LayoutPlacement
#[derive(Debug)]
pub enum LayoutPlacement {
    None,
    Place(Point),
    Grid(String, String, String, String),
}

//ip Display for LayoutPlacement
impl std::fmt::Display for LayoutPlacement {
    //mp fmt - format for display
    /// Display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Place(p) => write!(f, "PlaceAt{}", p),
            Self::Grid(x0, y0, x1, y1) => write!(f, "Grid[({},{}) -> ({},{})]", x0, y0, x1, y1),
            _ => write!(f, "Not placed or gridded"),
        }
    }

    //zz All done
}

//tp ElementLayout
#[derive(Debug)]
pub struct ElementLayout {
    pub placement: LayoutPlacement,
    debug: String,
    pub ref_pt: Option<Point>,
    bbox: Rectangle,
    pub anchor: Point,
    pub expand: Point,
    pub scale: f64,
    pub rotation: f64,
    pub translate: Point,
    pub border_width: f64,
    pub border_round: f64,
    pub border_color: Option<(f64, f64, f64)>,
    pub bg: Option<(f64, f64, f64)>,
    pub pad: Option<(f64, f64, f64, f64)>,
    pub margin: Option<(f64, f64, f64, f64)>,
}

//ip ElementLayout
impl ElementLayout {
    //fp new
    pub fn new() -> Self {
        Self {
            placement: LayoutPlacement::None,
            debug: "".to_string(),
            ref_pt: None,
            bbox: Rectangle::none(),
            anchor: Point::zero(),
            expand: Point::zero(),
            scale: 1.,
            rotation: 0.,
            translate: Point::zero(),
            border_width: 0.,
            border_round: 0.,
            border_color: None,
            bg: None,
            pad: None,
            margin: None,
        }
    }

    //fp of_style
    pub fn of_style(header: &ElementHeader) -> Result<Self, ElementError> {
        let mut layout = Self::new();
        if let Some(d) = header.get_style_of_name_string(at::DEBUG) {
            layout.debug = d;
        }
        match header.get_style_floats_of_name(at::BBOX).as_floats(None) {
            Some(g) => match g.len() {
                0 => (),
                1 => {
                    layout.bbox = Rectangle::of_cwh(Point::zero(), g[0], g[0]);
                }
                2 => {
                    layout.bbox = Rectangle::of_cwh(Point::zero(), g[0], g[1]);
                }
                3 => {
                    layout.bbox = Rectangle::of_cwh(Point::from_array([g[0], g[1]]), g[2], g[2]);
                }
                _ => {
                    layout.bbox = Rectangle::new(g[0], g[1], g[2], g[3]);
                }
            },
            _ => (),
        };
        if let Some(v) = header.get_style_floats_of_name(at::ANCHOR).as_floats(None) {
            layout.anchor = Point::from_array([v[0], v[1]]);
        }
        if let Some(v) = header.get_style_floats_of_name(at::EXPAND).as_floats(None) {
            layout.expand = Point::from_array([v[0], v[1]]);
        }
        if let Some(v) = header.get_style_of_name_float(at::BORDERWIDTH, None) {
            layout.border_width = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::BORDERROUND, None) {
            layout.border_round = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::SCALE, None) {
            layout.scale = v;
        }
        if let Some(v) = header.get_style_of_name_float(at::ROTATE, None) {
            layout.rotation = v;
        }
        if let Some(v) = header
            .get_style_rgb_of_name(at::BORDERCOLOR)
            .as_floats(None)
        {
            layout.border_color = Some((v[0], v[1], v[2]));
        }
        if let Some(v) = header.get_style_rgb_of_name(at::BG).as_floats(None) {
            layout.bg = Some((v[0], v[1], v[2]));
        }
        if let Some(v) = header.get_style_floats_of_name(at::MARGIN).as_floats(None) {
            layout.margin = Some((v[0], v[1], v[2], v[3]));
        }
        if let Some(v) = header.get_style_floats_of_name(at::PAD).as_floats(None) {
            layout.pad = Some((v[0], v[1], v[2], v[3]));
        }
        if let Some(v) = header
            .get_style_floats_of_name(at::TRANSLATE)
            .as_floats(None)
        {
            layout.translate = Point::from_array([v[0], v[1]]);
        }
        if let Some((sx, sy, ex, ey)) = {
            let opt_gx = {
                match header.get_style_ints_of_name(at::GRIDX).as_ints(None) {
                    Some(g) => match g.len() {
                        0 => None,
                        1 => Some((g[0], g[0] + 1)),
                        _ => Some((g[0], g[1])),
                    },
                    _ => None,
                }
            };
            let opt_gy = {
                match header.get_style_ints_of_name(at::GRIDY).as_ints(None) {
                    Some(g) => match g.len() {
                        0 => None,
                        1 => Some((g[0], g[0] + 1)),
                        _ => Some((g[0], g[1])),
                    },
                    _ => None,
                }
            };
            let opt_grid = {
                match header.get_style_ints_of_name(at::GRID).as_ints(None) {
                    Some(g) => match g.len() {
                        0 => None,
                        1 => Some((g[0], g[0], g[0] + 1, g[0] + 1)),
                        2 => Some((g[0], g[1], g[0] + 1, g[1] + 1)),
                        3 => Some((g[0], g[1], g[2], g[1] + 1)),
                        _ => Some((g[0], g[1], g[2], g[3])),
                    },
                    _ => None,
                }
            };
            if let Some((gx0, gx1)) = opt_gx {
                if let Some((gy0, gy1)) = opt_gy {
                    Some((gx0, gy0, gx1, gy1))
                } else if let Some((_, gy0, _, gy1)) = opt_grid {
                    Some((gx0, gy0, gx1, gy1))
                } else {
                    Some((gx0, 1, gx1, 2))
                }
            } else if let Some((gy0, gy1)) = opt_gy {
                if let Some((gx0, _, gx1, _)) = opt_grid {
                    Some((gx0, gy0, gx1, gy1))
                } else {
                    Some((1, gy0, 2, gy1))
                }
            } else {
                opt_grid
            }
        } {
            layout.set_grid(
                format!("{}", sx),
                format!("{}", sy),
                format!("{}", ex),
                format!("{}", ey),
            );
        }
        if let Some((x, y)) = {
            match header.get_style_ints_of_name(at::PLACE).as_floats(None) {
                Some(g) => match g.len() {
                    0 => None,
                    1 => Some((g[0], g[0])),
                    _ => Some((g[0], g[1])),
                },
                _ => None,
            }
        } {
            layout.set_place(x, y);
        }
        Ok(layout)
    }

    //mp debug_get_grid
    pub fn debug_get_grid(&self) -> Option<(f64, &str)> {
        if self.debug != "" {
            Some((1., "cyan"))
        } else {
            None
        }
    }

    //mp set_grid
    pub fn set_grid(&mut self, sx: String, sy: String, ex: String, ey: String) {
        self.placement = LayoutPlacement::Grid(sx, sy, ex, ey);
    }

    //mp set_place
    pub fn set_place(&mut self, x: f64, y: f64) {
        self.placement = LayoutPlacement::Place(Point::from_array([x, y]));
    }

    //mp set_layout_box
    /// This method is invoked to update the [LayoutBox] based on the
    /// properties of this [ElementLayout] and given a desired content
    /// geometry.
    ///
    /// The [LayoutBox] handles the actual management of the layout;
    /// this provides a styling wrapper to provide the information for
    /// the [LayoutBox].
    ///
    /// eref is a string that identifies the element for human debug
    ///
    /// When this function returns the [LayoutBox] will have all the
    /// information to provide its desired geometry.
    pub fn set_layout_box(
        &self,
        _eref: &str,
        layout_box: &mut LayoutBox,
        content_desired: Rectangle,
    ) {
        let bbox = {
            if self.bbox.is_none() {
                content_desired
            } else {
                self.bbox
            }
        };
        layout_box.set_content_geometry(bbox, Point::zero(), self.scale, self.rotation);
        layout_box.set_border_width(self.border_width);
        layout_box.set_border_round(self.border_round);
        layout_box.set_margin(&self.margin);
        layout_box.set_padding(&self.pad);
        layout_box.set_anchor_expand(self.anchor.clone(), self.expand.clone());
    }

    //mp set_layout_properties
    /// This method is invoked after the desired geometry for the
    /// element (in terms of size) has been determined, to inform the
    /// parent [Layout] where this is placed or how it is gridded.
    ///
    /// eref is a string that identifies the element for human debug
    ///
    /// The [Layout] can later be interrogated to determine *its*
    /// desired geometry, for hierarchical layout
    pub fn set_layout_properties(&self, eref: &str, layout: &mut Layout, bbox: Rectangle) {
        match &self.placement {
            LayoutPlacement::None => {
                layout.add_placed_element(eref, &Point::zero(), &None, &bbox);
            }
            LayoutPlacement::Grid(sx, sy, ex, ey) => {
                let sx = layout.add_grid_id(true, sx);
                let sy = layout.add_grid_id(false, sy);
                let ex = layout.add_grid_id(true, ex);
                let ey = layout.add_grid_id(false, ey);
                layout.add_grid_element(eref, (sx, sy), (ex, ey), (bbox.width(), bbox.height()));
            }
            LayoutPlacement::Place(pt) => {
                layout.add_placed_element(eref, &pt, &self.ref_pt, &bbox);
            }
        }
    }

    //mp display
    pub fn display(&self, indent_str: &str) {
        println!("{}  ", self.placement);
        if let Some(pt) = self.ref_pt {
            println!("{}  ref_pt:", pt);
        }
    }

    //zz All done
}

//ti IndentedDisplay for ElementLayout
impl<'a> IndentedDisplay<'a, IndentOptions> for ElementLayout {
    fn indent(&self, ind: &mut Indenter<'_, IndentOptions>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(ind, "Layout\n")?;
        let mut sub = ind.sub();
        if let Some(pt) = self.ref_pt {
            write!(&mut sub, "ref_pt: {}\n", pt)?;
        }
        if !self.bbox.is_none() {
            write!(&mut sub, "bbox:    {}\n", self.bbox)?;
        }
        write!(&mut sub, "anchor  : {}\n", self.anchor)?;
        write!(&mut sub, "expand  : {}\n", self.expand)?;
        write!(&mut sub, "scale   : {}\n", self.scale)?;
        write!(&mut sub, "rotation: {}\n", self.rotation)?;
        write!(&mut sub, "border wid: {}\n", self.border_width)?;
        write!(&mut sub, "border rnd: {}\n", self.border_round)?;
        write!(&mut sub, "border color: {:?}\n", self.border_color)?;
        write!(&mut sub, "bg color: {:?}\n", self.bg)?;
        write!(&mut sub, "pad: {:?}\n", self.pad)?;
        write!(&mut sub, "margin: {:?}\n", self.margin)?;
        Ok(())
    }
}
