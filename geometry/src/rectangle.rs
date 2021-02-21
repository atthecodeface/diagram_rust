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

@file    rectangle.rs
@brief   Geometry library
 */

//a Imports
use super::{Point, Range};

//t Rectangle
//tp Float4
/// Four floating-point numbers, with no restrictions - this is
/// similar to a Rectangle, except the constrainnt of x1>x0 is not
/// required
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float4 {
    /// One X coordinate
    pub x0 : f64,
    /// Second X coordinate
    pub x1 : f64,
    /// One Y coordinate
    pub y0 : f64,
    /// Second Y coordinate
    pub y1 : f64,
}

//ti Display for Float4
impl std::fmt::Display for Float4 {

    //mp fmt - format for a human
    /// Display the Float4
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[({},{}):({},{})]", self.x0, self.y0, self.x1, self.y1)
    }

    //zz All done
}

//ti Float4
impl Float4 {
    //fp none
    /// Create an empty float4 at 0,0
    pub const fn none() -> Self {
        Self { x0:0., x1:0., y0:0., y1:0.}
    }

    //fp new
    /// Make a float4
    pub fn new(x0:f64, y0:f64, x1:f64, y1:f64) -> Self {
        Self {x0, x1, y0, y1}
    }

    //zz All done
}

//tp Rectangle
#[derive(Clone, Copy, Debug, PartialEq)]
/// `Rectangle` describes a region bounded by (x0,y0) and (x1,y1) It
/// requires x0 <= x1 and y0 <= y1, and if either are equal then the
/// region is deemed to be *none*
pub struct Rectangle {
    /// smaller x coordinate of region
    pub x0 : f64,
    /// larger x coordinate of region
    pub x1 : f64,
    /// smaller y coordinate of region
    pub y0 : f64,
    /// larger y coordinate of region
    pub y1 : f64,
}

//ti Display for Rectangle
impl std::fmt::Display for Rectangle {

    //mp fmt - format for a human
    /// Display the Rectangle
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[({},{}):({},{})]", self.x0, self.y0, self.x1, self.y1)
    }

    //zz All done
}

//ti Rectangle
impl Rectangle {
    //fp none
    /// Create an empty rectangle at 0,0
    pub const fn none() -> Self {
        Self { x0:0., x1:0., y0:0., y1:0.}
    }

    //mp is_none
    /// Return `true` if the rectangle describes a 'none' region
    pub fn is_none(&self) -> bool {
        self.x0 >= self.x1 || self.y0 >= self.y1
    }

    //fp new
    /// Make a rectangle using the coordinates supplied, ensuring that
    /// the rectangle is correctly defined
    pub fn new(x0:f64, y0:f64, x1:f64, y1:f64) -> Self {
        let (x0,x1) = {if x0<x1 {(x0,x1)} else {(x1,x0)}};
        let (y0,y1) = {if y0<y1 {(y0,y1)} else {(y1,y0)}};
        Self {x0, x1, y0, y1}
    }

    //cp to_ranges
    /// Set the rectangle to be the ranges supplied
    pub fn to_ranges(mut self, x:Range, y:Range) -> Self {
        self.x0 = x.min;
        self.x1 = x.max;
        self.y0 = y.min;
        self.y1 = y.max;
        self
    }

    //fp bbox_of_points
    /// Make a new rectangle that is the bounding box of a vec of points
    pub fn bbox_of_points(pts:&Vec<Point>) -> Self {
        match pts.len() {
            0 => Self::none(),
            1 => Self::none(),
            _ => {
                let mut min_x = pts[0].x;
                let mut min_y = pts[0].y;
                let mut max_x = min_x;
                let mut max_y = min_y;
                for p in pts {
                    if p.x < min_x { min_x = p.x; }
                    if p.y < min_y { min_y = p.y; }
                    if p.x > max_x { max_x = p.x; }
                    if p.y > max_y { max_y = p.y; }
                }
                Self::new(min_x, min_y, max_x, max_y)
            },
        }
    }

    //fp of_cwh
    /// Generate a rectangle from a centre `Point` and a width/height.
    pub fn of_cwh(centre:Point, width:f64, height:f64) -> Self {
        Self::new( centre.x-width/2.,
                  centre.y-height/2.,
                  centre.x+width/2.,
                  centre.y+height/2. )
    }

    //mp pt_within
    /// Consume a point, and return a new point that whose X
    /// coordinate indicate the fraction of this rectangles' width the
    /// original point is along its width, and similarly for the Y
    /// coordinate
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(0.,0., 10.,5.);
    /// assert_eq!( r.pt_within(Point::origin()).x, 0. );
    /// assert_eq!( r.pt_within(Point::origin()).y, 0. );
    /// assert_eq!( r.pt_within(Point::new(10.,5.)).x, 1. );
    /// assert_eq!( r.pt_within(Point::new(10.,5.)).y, 1. );
    /// assert_eq!( r.pt_within(Point::new(5.,5.)).x, 0.5 );
    /// assert_eq!( r.pt_within(Point::new(5.,5.)).y, 1. );
    /// ```
    pub fn pt_within(&self, mut pt:Point) -> Point {
        if self.is_none() {
            pt
        } else {
            pt = pt.add( &Point::new(self.x0,self.y0), -1.);
            pt.scale_xy( 1./(self.x1-self.x0),  1./(self.y1-self.y0) )
        }
    }

    //mp as_points
    /// Create a vector of four points that are the
    /// anticlockwise-ordered corners of the rectangle starting at the
    /// minumum (x,y)
    pub fn as_points(&self, close:bool, mut v:Vec<Point>) -> Vec<Point> {
        v.push(Point::new(self.x0,self.y0));
        v.push(Point::new(self.x1,self.y0));
        v.push(Point::new(self.x1,self.y1));
        v.push(Point::new(self.x0,self.y1));
        v.push(Point::new(self.x0,self.y0));
        if close { v.push(Point::new(self.x0,self.y0)); }
        v
    }

    //mp get_wh
    /// Return a point consisting of the width and height of the rectangle
    pub fn get_wh(&self) -> Point {
        Point::new(self.x1-self.x0, self.y1-self.y0)
    }

    //mp get_center
    /// Return a point indicating the centre of the rectangle
    pub fn get_center(&self) -> Point {
        Point::new((self.x1+self.x0)/2., (self.y1+self.y0)/2.)
    }

    //mp xrange
    /// Return a point to be used as the region that covers the X
    /// dimension of the rectangle - that is `x0` to `x1`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(0.,0., 10.,5.);
    /// assert_eq!( r.xrange().x, 0. );
    /// assert_eq!( r.xrange().y, 10. );
    /// ```
    pub fn xrange(&self) -> Point {
        Point::new(self.x0, self.x1)
    }

    //mp yrange
    /// Return a point to be used as the region that covers the Y
    /// dimension of the rectangle - that is `y0` to `y1`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(0.,0., 10.,5.);
    /// assert_eq!( r.yrange().x, 0. );
    /// assert_eq!( r.yrange().y, 5. );
    /// ```
    pub fn yrange(&self) -> Point {
        Point::new(self.y0, self.y1)
    }

    //mp width
    /// Return the width of the rectangle (`x1` - `x0`)
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(15.,12., 29., 30.);
    /// assert_eq!( r.width(), 14. );
    /// # assert_eq!( r.height(), 18. );
    /// ```
    pub fn width(&self) -> f64 {self.x1-self.x0}

    //mp height
    /// Return the height of the rectangle (`y1` - `y0`)
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(15.,12., 29., 30.);
    /// # assert_eq!( r.width(), 14. );
    /// assert_eq!( r.height(), 18. );
    /// ```
    pub fn height(&self) -> f64 {self.y1-self.y0}

    //mp get_cwh
    /// Get the centre, width and height of the rectangle 
    pub fn get_cwh(&self) -> (Point, f64, f64) {
        (self.get_center(), self.width(), self.height())
    }

    //cp scale
    /// Consume the rectangle and return a new rectangle whose
    /// coordinates are scaled by a value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(15.,12., 29., 30.)
    ///         .scale(2.);
    /// assert_eq!( r.x0, 30. );
    /// assert_eq!( r.x1, 58. );
    /// assert_eq!( r.y0, 24. );
    /// assert_eq!( r.y1, 60. );
    /// ```
    pub fn scale(mut self, value:f64) -> Self {
        self.x0 *= value;
        self.y0 *= value;
        self.x1 *= value;
        self.y1 *= value;
        self
    }

    //cp enlarge
    /// Consume the rectangle and return a new rectangle enlarge by a
    /// fixed value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(15.,12., 29., 30.)
    ///         .enlarge(1.);
    /// assert_eq!( r.x0, 14. );
    /// assert_eq!( r.x1, 30. );
    /// assert_eq!( r.y0, 11. );
    /// assert_eq!( r.y1, 31. );
    /// ```
    pub fn enlarge(mut self, value:f64) -> Self {
        self.x0 -= value;
        self.y0 -= value;
        self.x1 += value;
        self.y1 += value;
        self
    }

    //cp reduce
    /// Shrink the rectangle, keeping the same center, by a fixed value
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate diagram;
    /// # use diagram::{Point, Rectangle};
    /// let r = Rectangle::new(15.,12., 29., 30.)
    ///         .reduce(1.);
    /// assert_eq!( r.x0, 16. );
    /// assert_eq!( r.x1, 28. );
    /// assert_eq!( r.y0, 13. );
    /// assert_eq!( r.y1, 29. );
    /// ```
    pub fn reduce(mut self, value:f64) -> Self {
        self.x0 += value;
        self.y0 += value;
        self.x1 -= value;
        self.y1 -= value;
        self
    }

    //mp expand
    /// exand in-place by expansion scaled by 'scale'
    pub fn expand(mut self, other:&Float4, scale:f64) -> Self {
        self.x0 -= scale * other.x0;
        self.y0 -= scale * other.y0;
        self.x1 += scale * other.x1;
        self.y1 += scale * other.y1;
        self
    }

    //mp shrink
    /// shrink in-place by expansion scaled by 'scale'
    pub fn shrink(self, other:&Float4, scale:f64) -> Self {
        self.expand(other, -scale)
    }

    //mp union
    /// union this with another; if either is_zero then just the other
    pub fn union(mut self, other:&Self) -> Self {
        if other.is_none() {
            ();
        } else if self.is_none() {
            self.x0 = other.x0;
            self.y0 = other.y0;
            self.x1 = other.x1;
            self.y1 = other.y1;
        } else {
            self.x0 = if other.x0<self.x0 {other.x0} else {self.x0};
            self.y0 = if other.y0<self.y0 {other.y0} else {self.y0};
            self.x1 = if other.x1>self.x1 {other.x1} else {self.x1};
            self.y1 = if other.y1>self.y1 {other.y1} else {self.y1};
        }
        self
    }

    //mp intersect
    /// intersect this with another; if either is_zero then this will be zero
    pub fn intersect(mut self, other:&Self) -> Self {
        self.x0 = if other.x0>self.x0 {other.x0} else {self.x0};
        self.y0 = if other.y0>self.y0 {other.y0} else {self.y0};
        self.x1 = if other.x1<self.x1 {other.x1} else {self.x1};
        self.y1 = if other.y1<self.y1 {other.y1} else {self.y1};
        if self.x0>=self.x1 || self.y0>=self.y1 {
            self.x0 = 0.;
            self.y0 = 0.;
            self.x1 = 0.;
            self.y1 = 0.;
        }
        self
    }

    //mp translate
    /// translate in-place by scale*pt
    pub fn translate(mut self, pt:&Point, scale:f64) -> Self {
        self.x0 += scale*pt.x;
        self.x1 += scale*pt.x;
        self.y0 += scale*pt.y;
        self.y1 += scale*pt.y;
        self
    }

    //mp new_rotated_around
    /// Rotate the rectangle around a point by an angle,
    /// generating a new rectangle that is the bounding box of that rotated rectangle
    pub fn new_rotated_around(&self, pt:&Point, degrees:f64) -> Self{
        let p0 = Point::new(self.x0,self.y0).rotate_around(pt, degrees);
        let p1 = Point::new(self.x0,self.y1).rotate_around(pt, degrees);
        let p2 = Point::new(self.x1,self.y1).rotate_around(pt, degrees);
        let p3 = Point::new(self.x1,self.y0).rotate_around(pt, degrees);
        let x0 = if p0.x<p1.x {p0.x} else {p1.x};
        let x0 = if x0<p2.x {x0} else {p2.x};
        let x0 = if x0<p3.x {x0} else {p3.x};
        let y0 = if p0.y<p1.y {p0.y} else {p1.y};
        let y0 = if y0<p2.y {y0} else {p2.y};
        let y0 = if y0<p3.y {y0} else {p3.y};
        let x1 = if p0.x>p1.x {p0.x} else {p1.x};
        let x1 = if x1>p2.x {x1} else {p2.x};
        let x1 = if x1>p3.x {x1} else {p3.x};
        let y1 = if p0.y>p1.y {p0.y} else {p1.y};
        let y1 = if y1>p2.y {y1} else {p2.y};
        let y1 = if y1>p3.y {y1} else {p3.y};
        Self {x0, x1, y0, y1}
    }

    //mp fit_within_region
    /// Using two anchor values (x and y) between -1 and 1, and expansion values (between 0 and 1),
    /// fit this region within an outer region
    ///
    /// See Range::fit_within_region for more details on each dimension
    pub fn fit_within_dimension(self, outer:&Rectangle, anchor:&Point,  expand:&Point) -> (Point,Self) {
        let (dx,xs) = Range::new(self.x0,self.x1).fit_within_dimension( &Range::new(outer.x0,outer.x1), anchor.x, expand.x);
        let (dy,ys) = Range::new(self.y0,self.y1).fit_within_dimension( &Range::new(outer.y0,outer.y1), anchor.y, expand.y);
        (Point::new(dx,dy), self.to_ranges(xs, ys))
    }
    
    //zz All done
}

//a Test
#[cfg(test)]
mod tests_polygon {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    #[test]
    fn test_zero() {
        let x = Rectangle::none();
        assert!(x.is_none());
        pt_eq(&x.get_center(),0.,0.);
        pt_eq(&x.get_wh(),0.,0.);
        assert_eq!(x.width(),0.);
        assert_eq!(x.height(),0.);
        pt_eq(&x.xrange(),0.,0.);
        pt_eq(&x.yrange(),0.,0.);
    }
    #[test]
    fn test_0() {
        let x = Rectangle::new(-3.,1., 5.,7.);
        pt_eq(&x.get_center(),1.,4.);
        assert_eq!(x.width(),8.);
        assert_eq!(x.height(),6.);
        pt_eq(&x.get_wh(),8.,6.);
        pt_eq(&x.xrange(),-3.,5.);
        pt_eq(&x.yrange(),1.,7.);
        pt_eq(&x.get_cwh().0,1.,4.);
        assert_eq!(x.get_cwh().1,8.);
        assert_eq!(x.get_cwh().2,6.);
    }
    #[test]
    fn test_ops_0() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Rectangle::new(4.,0., 6.,3.);
        let z = Rectangle::new(5.,1., 7.,4.);
        let x_and_y = x.clone().intersect(&y);
        let x_or_y  = x.clone().union(&y);
        let x_and_z = x.clone().intersect(&z);
        let x_or_z  = x.clone().union(&z);
        println!("x_and_y:{}",x_and_y);
        println!("x_or_y:{}",x_or_y);
        println!("x_and_z:{}",x_and_z);
        println!("x_or_z:{}",x_or_z);
        pt_eq(&x_and_y.xrange(),4.,5.);
        pt_eq(&x_and_y.yrange(),1.,3.);
        pt_eq(&x_or_y.xrange(),2.,6.);
        pt_eq(&x_or_y.yrange(),0.,7.);

        assert!(x_and_z.is_none());
        pt_eq(&x_and_z.xrange(),0.,0.);
        pt_eq(&x_and_z.yrange(),0.,0.);
        pt_eq(&x_or_z.xrange(),2.,7.);
        pt_eq(&x_or_z.yrange(),1.,7.);
    }
    #[test]
    fn test_ops_1() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Float4::new(0.1, 0.2, 0.3, 0.5);
        let x_p_y  = x.clone().expand(&y,1.);
        let x_p_2y = x.clone().expand(&y,2.);
        println!("x_p_y:{}",x_p_y);
        println!("x_p_2y:{}",x_p_2y);
        pt_eq(&x_p_y.xrange(),1.9, 5.3);
        pt_eq(&x_p_y.yrange(),0.8, 7.5);
        pt_eq(&x_p_2y.xrange(),1.8, 5.6);
        pt_eq(&x_p_2y.yrange(),0.6, 8.);
    }
    #[test]
    fn test_ops_2() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Float4::new(0.1, 0.2, 0.3, 0.5);
        let x_m_y  = x.clone().shrink(&y,1.);
        let x_m_2y = x.clone().shrink(&y,2.);
        println!("x_m_y:{}",x_m_y);
        println!("x_m_2y:{}",x_m_2y);
        pt_eq(&x_m_y.xrange(),2.1, 4.7);
        pt_eq(&x_m_y.yrange(),1.2, 6.5);
        pt_eq(&x_m_2y.xrange(),2.2, 4.4);
        pt_eq(&x_m_2y.yrange(),1.4, 6.);
    }
}
