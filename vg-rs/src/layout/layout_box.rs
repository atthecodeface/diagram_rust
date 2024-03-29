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

@file    layout_box.rs
@brief   Layout of placed items and grids
 */

//a Imports
use crate::{BBox, Point, Polygon, Range, Transform};
use geo_nd::Vector;
use indent_display::{IndentedDisplay, IndentedOptions, Indenter};

//a Constants
const DEBUG_LAYOUT_BOX: bool = 1 == 0;

//a LayoutBox
//tp LayoutBox
/// A box that lis used to layout a content container with a
/// transformation for that content; this is itself a container
/// (i.e. it has a margin, border, and padding)
#[derive(Debug)]
pub struct LayoutBox {
    /// This indicates how much to expand the content within its laid-out space (0-1 each in x and y)
    expansion: Point,
    /// This indicates how much where to anchor the content within its laid-out space; if the expansion is 1 then this is irrelevant. -1 indicates to the minimum X/Y, +1 indicates to the maximum X/Y
    anchor: Point,
    /// The margin may be specified for each of the four sides - it reduces the laid-out space, with the border within the margin
    margin: Option<[f64; 4]>,
    /// The border is a fixed width all round, and may be 0. for no border; the border is within the laid-out margin, around the padding around the content
    border_width: f64,
    /// The border may be rounded
    border_round: f64,
    /// The padding may be specified for each of the four sides - it reduces the laid-out space for the content within the border
    padding: Option<[f64; 4]>,
    /// The content may be rotated within its laid-out (post-padding) space; it will still be rectangular, so it will be the largest rectangle permitted at the rotation provided by the laid-out rectangle
    content_rotation: f64,
    /// The content may be scaled its space, by a uniform amount in X and Y
    content_scale: f64,
    /// The content reference is a fractional point within the content rectangle; this is required for 'placement'
    content_ref: Option<Point>,
    /// This rectangle specifies in content coordinates the desired rectangle for the content
    content_desired: Option<BBox>,
    /// This rectangle is derived from the content_desired rectangle by scaling, rotation, padding, adding border, and adding margin
    outer_desired: Option<BBox>,
    /// The outer rectangle is provided by the layout - it is the actual laid-out outer rectangle, from which the inner laid-out regions are derived
    outer: Option<BBox>,
    /// The border_shape is an optional Polygon (rectangle) that may be drawn with a fill, or a stroke width of `border_width`, to provide the required border
    border_shape: Option<Polygon>,
    /// The inner rectangle is the region into which the rotated, scale content fits
    inner: Option<BBox>,
    /// The content rectangle is the content-coordinate space rectangle for the laid-out content
    content: Option<BBox>,
    /// The content transform maps from the content coordinate system to the layout coordinate system
    content_to_layout: Option<Transform>,
}

//ip Default for LayoutBox
impl Default for LayoutBox {
    fn default() -> Self {
        Self {
            expansion: Point::zero(),
            anchor: Point::zero(),
            margin: None,
            border_width: 0.,
            border_round: 0.,
            padding: None,
            content_desired: None,
            content_scale: 1.,
            content_ref: None,
            content_rotation: 0.,
            outer_desired: None,
            outer: None,
            border_shape: None,
            inner: None,
            content: None,
            content_to_layout: None,
        }
    }
}

//ip LayoutBox
impl LayoutBox {
    //fp set_content_geometry
    /// Sets the content's desired geometry
    pub fn set_content_geometry(&mut self, rect: BBox, ref_pt: Point, scale: f64, rotation: f64) {
        self.content_ref = Some(ref_pt);
        self.content_scale = scale;
        self.content_rotation = rotation;
        // self.content_translation = ;
        self.content_desired = Some(rect);
    }

    //fp set_border_width
    /// Set the border width
    pub fn set_border_width(&mut self, border_width: f64) {
        self.border_width = border_width;
    }

    //fp set_border_round
    /// Set the border rounding
    pub fn set_border_round(&mut self, border_round: f64) {
        self.border_round = border_round;
    }

    //fp set_margin
    /// Set the margin
    pub fn set_margin(&mut self, value: &Option<(f64, f64, f64, f64)>) {
        if let Some((x0, y0, x1, y1)) = value.as_ref() {
            self.margin = Some([*x0, *y0, *x1, *y1]);
        } else {
            self.margin = None;
        }
    }

    //fp set_padding
    /// Set the padding
    pub fn set_padding(&mut self, value: &Option<(f64, f64, f64, f64)>) {
        if let Some((x0, y0, x1, y1)) = value.as_ref() {
            self.padding = Some([*x0, *y0, *x1, *y1]);
        } else {
            self.padding = None;
        }
    }

    //fp set_anchor_expand
    /// Set the anchor point and expansion
    pub fn set_anchor_expand(&mut self, anchor: Point, expansion: Point) {
        self.anchor = anchor;
        self.expansion = expansion;
    }

    //ap content_transform
    /// Get the content transformation
    pub fn content_transform(&self) -> Option<&Transform> {
        self.content_to_layout.as_ref()
    }

    //fp desired_bbox
    /// Get the desired [BBox] for the layout from all of its content,
    /// with associated placement; include the container expansion
    pub fn desired_bbox(&self) -> BBox {
        let mut rect = {
            match &self.content_desired {
                None => BBox::none(),
                Some(bbox) => {
                    bbox.new_rotated_around(
                        self.content_ref.as_ref().unwrap(),
                        self.content_rotation,
                    ) * self.content_scale
                }
            }
        };
        rect = self.padding.map_or(rect, |r| rect.expand(&r, 1.));
        rect = rect.enlarge(self.border_width);
        rect = self.margin.map_or(rect, |r| rect.expand(&r, 1.));
        rect
    }

    //fp wh_of_largest_area_within
    /// Finds the largest rectangle rotated to `angle` that can fit
    /// within a given width and height
    ///
    /// If angle<0 then the solution is mirrored along the horizontal; the same
    /// rectangle size works therefore.
    ///
    /// If angle>=90 then we can consider a-90 and swap width and height
    /// Then, if width>height we can consider 90-angle and swap width and height.
    ///
    /// Hence only consider angle<90 (hence tan(a)>=0) and width<=height
    ///
    /// Note that the area of a RH triangle with `angle` and
    /// adjacent length l is 1/2.l.l.tan(a) = l^2.t/2
    /// Assume the largest rectangle leaves rectangular spaces
    /// above and below; with coordinates of (xw,0), (w,(y'-y)h),
    /// (w-xw,y'h), (0,yh)
    /// This assumes largest rectangle is limited by width w, and y'<1.
    /// We know that tan(a) = xw/yh; i.e.
    /// yh=xw/t.
    /// And tan(a)=(y'-y)h/w(1-x); i.e.
    /// wt(1-x) = y'h-yh
    /// y'h     = wt(1-x) + xw/t = wt(1+x/(t^2)-x)
    ///
    /// Then the 'wasted space' is then two triangles of size xw : yh and
    /// two triangles of size w(1-x) : (y'-y)h, and the rectangle of size w : (1-y')h.
    ///
    /// The total is the sum of:
    /// xw.yh = x^2.w^2/t
    /// w(1-x).(y'-y)h = w^2.(1-x)^2.t = w^2.t.(1+x^2-2x) = w^2.t + x^2.w^2.t -2x.w^2.t
    /// wh-wy'h = wh - w^2.t(1+x/(t^2)-x) = wh -w^2.t -x.w^2/t + x.w^2.t
    ///
    /// Sum = x^2.w^2/t + w^2.t + x^2.w^2.t -2x.w^2.t + wh -w^2.t -x.w^2/t + x.w^2.t
    /// = x^2.w^2/t + x^2.w^2.t -x.w^2.t -x.w^2/t + wh
    ///
    /// This has a minimum (wasted) area when its derivative is 0 (since it has +ve x^2)
    ///
    /// dA/dx = 2x.w^2/t + 2x.w^2.t -w^2.t -w^2/t
    /// = (2x-1).w^2.(1/t+t)
    ///
    /// i.e. x=0.5; i.e. the correct x is independent of w, h, a.
    ///
    /// But, y' must be <=1. So, we want an x closest to 0.5 where y'<=1
    /// Now y' when x=0.5 is:
    /// y' = wt(1+0.5/(t^2)-0.5)/h
    ///    = w/2h * (t + 1/t)
    /// <=1 if
    /// t+1/t <= 2h/w
    /// (t^2+1)/t <= 2h/w7
    /// But (tan^1+1)/tan = sec^2/tan = 1/(sin.cos) = 2/(sin 2a), hence
    /// 2/sin(2a) <= 2h/w
    /// sin(2a)   >= w/h
    /// So we only have to worry if sin(2a) < w/h
    ///
    /// Now y'=1 occurs when w/h.t(1+x/(t^2)-x) = 1
    /// i.e. 1+x/(t^2)-x  = h/(wt)
    /// i.e. x(1/(t^2)-1) = h/(wt) - 1
    /// i.e. x(1-(t^2))/t^2 = h/(wt) - 1
    /// i.e. x              = (h/(wt) - 1) * t^2 / (1-(t^2))
    /// i.e. x              = (ht/w - t^2) / (1-(t^2))
    /// Now when `angle`=45 we have t=1 hence 1-t^2 is 0;
    /// if w==h then we have a diamond solution (i.e. x-0.5 works)
    /// if w<=h then sin(2*a) = 1 >= w/h then x=0.5 works
    /// If w>h then, as at the top, we should have used 90-a and swapped w/h
    fn find_wh_of_largest_area_within(width: f64, height: f64, angle: f64) -> (f64, f64) {
        let angle = {
            if angle > 0. {
                angle
            } else {
                -angle
            }
        };
        let angle = {
            if angle < 180. {
                angle
            } else {
                angle - 180.
            }
        };
        let (width, height, angle, flip) = {
            if angle >= 90. {
                (width, height, angle, false)
            } else {
                (height, width, angle - 90., true)
            }
        };
        let (width, height, angle, flip) = {
            if width < height {
                (width, height, angle, flip)
            } else {
                (height, width, 90. - angle, !flip)
            }
        };

        if DEBUG_LAYOUT_BOX {
            println!("{} {} {} {}", angle, width, height, flip);
        }

        let sin2a = (2. * angle).to_radians().sin();
        let tana = angle.to_radians().tan();
        let x = {
            if angle > 89.999 {
                // tana will be very large
                0.5
            } else if sin2a < width / height {
                (height * tana / width - tana * tana) / (1. - tana * tana)
            } else {
                0.5
            }
        };
        let y = x * width / height / tana;
        let y_p = width / height * tana * (1. - x) + y;
        let yh = height * y;
        let y_p_myh = height * (y_p - y);
        let wx = width * x;
        let wmwx = width * (1. - x);
        let (width, height) = {
            if tana < 1E-10 {
                (width, height)
            } else {
                (
                    (wx * wx + yh * yh).sqrt(),
                    (wmwx * wmwx + y_p_myh * y_p_myh).sqrt(),
                )
            }
        };
        if flip {
            (height, width)
        } else {
            (width, height)
        }
    }

    //mp inner_within_outer
    /// Sets the inner rectangle based on an outer rectangle, allowing for border
    ///
    /// This also creates any border shape required later
    fn inner_within_outer(&mut self, rectangle: BBox) {
        if DEBUG_LAYOUT_BOX {
            println!(
                "Create inner within outer rectangle {} {} {}",
                rectangle, self.border_width, self.border_round
            );
        }
        let mut inner = rectangle;
        self.outer = Some(rectangle);
        inner = self.margin.map_or(inner, |r| inner.shrink(&r, 1.));
        let (c, w, h) = inner.reduce(self.border_width * 0.5).get_cwh();
        let mut polygon = Polygon::new_rect(w, h).translate(&c);
        polygon.set_rounding(self.border_round);
        self.border_shape = Some(polygon);
        inner = inner.reduce(self.border_width);
        inner = self.padding.map_or(inner, |r| inner.shrink(&r, 1.));
        self.inner = Some(inner);
    }

    //ap get_border_shape
    /// Get the [Polygon] of the border if it has one
    pub fn get_border_shape(&self) -> Option<&Polygon> {
        self.border_shape.as_ref()
    }

    //mp content_within_inner
    ///
    fn content_within_inner(&mut self) {
        if DEBUG_LAYOUT_BOX {
            println!("{:?} {:?}", self.inner, self.content_desired);
        }
        let (ic, iw, ih) = self.inner.unwrap().get_cwh();

        // If scale is 1. and rotation is 0. then we should be able to use a translation of ic-dc
        // so that is what we should get...
        let (aw, ah) = Self::find_wh_of_largest_area_within(iw, ih, self.content_rotation);
        // self.content_desired can be 'fit_within_region' of the width/height
        let cd = self.content_desired.unwrap();

        // Find the inner-scale coordinates for rectangle of content after scaling prior to rotation around centre of inner
        let di_x_range = cd.x * self.content_scale;
        let a_x_range = Range::new(ic[0] - aw / 2., ic[0] + aw / 2.);
        let (x_translation, ci_x_range) =
            di_x_range.fit_within_dimension(&a_x_range, self.anchor[0], self.expansion[0]);

        let di_y_range = cd.y * self.content_scale;
        let a_y_range = Range::new(ic[1] - ah / 2., ic[1] + ah / 2.);
        let (y_translation, ci_y_range) =
            di_y_range.fit_within_dimension(&a_y_range, self.anchor[1], self.expansion[1]);

        // ci_*_range is in inner coordinates centred on 'zero => inner center'
        // assuming content will be 'centred' on its desired centre (should perhaps use reference points?)
        // then find the inner coordinates of this desired centre
        // the transform maps this inner coordinates desired centre to the inner centre
        // then when the content is drawn centred on this desired centre it will appear centres on inner centre
        if DEBUG_LAYOUT_BOX {
            println!(
                "Getting content within inner {} {} : {} {} : {} {}",
                di_x_range, di_y_range, a_x_range, a_y_range, ci_x_range, ci_y_range
            );
        }
        let bbox = BBox::of_ranges(ci_x_range, ci_y_range);
        self.content =
            Some((bbox - Point::from_array([x_translation, y_translation])) / self.content_scale);

        // content_to_layout transform is scale, rotate, and then translate from 0,0 to ic
        let transform = Transform::of_trs(
            Point::from_array([x_translation, y_translation]), // This helped .rotate(self.content_rotation),
            self.content_rotation,
            self.content_scale,
        );
        let dc = cd.center();
        let t2 = Transform::of_translation(-dc);
        let transform = transform.apply_to_transform(&t2);
        let t2 = Transform::of_translation(dc);
        let transform = t2.apply_to_transform(&transform);
        // if cd.get_center().len() > 0.001 {
        //     println!("Transform of {} for {:?}", transform, cd);
        // }
        self.content_to_layout = Some(transform)
    }

    //mp layout_within_rectangle
    /// Layout the LayoutBox within a specified rectangle within layout coordinate space, generating any border required and the inner geometry,
    /// and the content transformation
    pub fn layout_within_rectangle(&mut self, rectangle: BBox) {
        self.inner_within_outer(rectangle);
        self.content_within_inner();
    }

    //mp get_content_rectangle
    /// Get the content rectangle
    ///
    /// Must only be invoked after layout_within_rectangle has been called
    pub fn get_content_rectangle(&self) -> BBox {
        self.content.unwrap()
    }

    //mp display
    /// Display with an indent of indent_str plus two spaces
    pub fn display(&self, indent_str: &str) {
        println!("{}  Layout box", indent_str);
        println!("{}    Margin {:?}", indent_str, self.margin);
        println!(
            "{}    Border w:{} r:{}",
            indent_str, self.border_width, self.border_round
        );
        println!("{}    Padding {:?}", indent_str, self.margin);
        println!("{}    Outer {:?}", indent_str, self.outer);
        println!("{}    Inner {:?}", indent_str, self.inner);
        println!("{}    Content {:?}", indent_str, self.content);
    }

    //zz All done
}

//ti IndentedDisplay for ElementLayout
impl<'a, O: IndentedOptions<'a>> IndentedDisplay<'a, O> for LayoutBox {
    fn indent(&self, ind: &mut Indenter<'a, O>) -> std::fmt::Result {
        use std::fmt::Write;
        writeln!(ind, "Layout box")?;
        let mut sub = ind.sub();
        writeln!(&mut sub, "anchor  : {}", self.anchor)?;
        writeln!(&mut sub, "expand  : {}", self.expansion)?;
        writeln!(&mut sub, "pad: {:?}", self.padding)?;
        writeln!(&mut sub, "border wid: {}", self.border_width)?;
        writeln!(&mut sub, "border rnd: {}", self.border_round)?;
        // writeln!(&mut sub, "border color: {:?}", self.border_color)?;
        writeln!(&mut sub, "margin: {:?}", self.margin)?;
        writeln!(&mut sub, "content_scale   : {}", self.content_scale)?;
        writeln!(&mut sub, "content_rotation: {}", self.content_rotation)?;
        if let Some(pt) = &self.content_ref {
            writeln!(&mut sub, "content_ref: {}", pt)?;
        }
        if let Some(r) = &self.content_desired {
            writeln!(&mut sub, "content desired: {}", r)?;
        }
        if let Some(r) = &self.outer_desired {
            writeln!(&mut sub, "outer desired: {}", r)?;
        }
        if let Some(r) = &self.outer {
            writeln!(&mut sub, "outer actual: {}", r)?;
        }
        if let Some(r) = &self.inner {
            writeln!(&mut sub, "inner actual: {}", r)?;
        }
        if let Some(r) = &self.content {
            writeln!(&mut sub, "content actual: {}", r)?;
        }
        if let Some(t) = &self.content_to_layout {
            writeln!(&mut sub, "content to layout: {}", t)?;
        }
        Ok(())
    }
}

//mt Test for LayoutBox
#[cfg(test)]
mod test_layoutbox {
    // use super::*;
}
