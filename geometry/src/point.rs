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

@file    point.rs
@brief   Part of geometry library
 */

//a Imports

//a Point
//tp Point
#[derive(Clone, Copy, PartialEq, Debug)]
/// This is a simple point class for two dimensions
pub struct Point {
    /// Coordinates
    pub x : f64,
    /// Coordinates
    pub y : f64,
}

//ti Display for Point
impl std::fmt::Display for Point {

    //mp fmt - format a `CharError` for display
    /// Display the `Point' as (x,y)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }

    //zz All done
}

//ti Point
impl Point {
    //fp new
    /// Create a new point from (x,y)
    pub const fn new(x:f64, y:f64) -> Self { Self {x,y} }

    //fp origin
    /// Create a new point from at (0,0)
    pub const fn origin() -> Self { Self {x:0.,y:0.} }

    //fp is_origin
    /// Return true if the point is the origin
    pub fn is_origin(&self) -> bool { self.x==0. && self.y==0. }

    //cp rotate
    /// Consume the point and return a new point that is the original
    /// rotated around the origin
    pub fn rotate(mut self, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*self.x - s*self.y;
        let y1 = c*self.y + s*self.x;
        self.x = x1;
        self.y = y1;
        self
    }

    //cp scale
    /// Consume the point and return a new point that is the original
    /// scaled in x and y by a single scaling factors
    pub fn scale(mut self, s:f64) -> Self {
        self.x = self.x*s;
        self.y = self.y*s;
        self
    }

    //cp scale_xy
    /// Consume the point and return a new point that is the original
    /// scaled in x and y by two different scaling factors
    pub fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        self.x = self.x*sx;
        self.y = self.y*sy;
        self
    }

    //cp add
    /// Consume the point, and return a new point that is the sum of
    /// this point and a borrowed other point
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
        self.x = self.x + other.x*scale;
        self.y = self.y + other.y*scale;
        self
    }

    //mp len2
    /// Return the distance^2 of the point from the origin
    pub fn len2(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    //mp len
    /// Return the distance of the point from the origin
    pub fn len(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    //mp distance_to
    /// Return the distance between this and another point
    pub fn distance(&self, other:&Self) -> f64 {
        let dx = self.x-other.x;
        let dy = self.y-other.y;
        (dx*dx + dy*dy).sqrt()
    }

    //mp dot
    /// Return the dot product of this and another point
    pub fn dot(&self, other:&Point) -> f64 {
        self.x*other.x + self.y*other.y
    }

    //cp rotate_around
    /// Consume the point and return a new point rotated around a
    /// *pivot* point by the specified angle
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*(self.x-pivot.x) + s*(self.y-pivot.y);
        let y1 = c*(self.y-pivot.y) - s*(self.x--pivot.x);
        self.x = x1 + pivot.x;
        self.y = y1 + pivot.y;
        self
    }

    //zz All done
}

//mt Test for Point
#[cfg(test)]
mod test_point {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    #[test]
    fn test_simple() {
        pt_eq( &Point::origin(), 0., 0. );
        pt_eq( &Point::new(1.,2.), 1., 2. );
        assert!( Point::origin().is_origin() );
        assert!( !Point::new(0.1,0.).is_origin() );
        assert!( !Point::new(0.,0.1).is_origin() );
        pt_eq( &Point::new(1.,2.).scale_xy(3.,4.), 3., 8. );
        pt_eq( &Point::new(1.,2.).scale_xy(3.,4.), 3., 8. );

        assert_eq!( Point::origin().len2(), 0. );
        assert_eq!( Point::new(1.,0.).len2(), 1. );
        assert_eq!( Point::new(0.,1.).len2(), 1. );
        assert_eq!( Point::new(2.,0.).len2(), 4. );
        assert_eq!( Point::new(0.,2.).len2(), 4. );

        assert_eq!( Point::origin().rotate(30.).len2(), 0. );
        assert_eq!( Point::new(1.,0.).rotate(30.).len2(), 1. );
        assert_eq!( Point::new(0.,1.).rotate(30.).len2(), 1. );
        assert_eq!( Point::new(2.,0.).rotate(30.).len2(), 4. );
        assert_eq!( Point::new(0.,2.).rotate(30.).len2(), 4. );

        assert_eq!( Point::origin().len(), 0. );
        assert_eq!( Point::new(1.,0.).len(), 1. );
        assert_eq!( Point::new(0.,1.).len(), 1. );
        assert_eq!( Point::new(2.,0.).len(), 2. );
        assert_eq!( Point::new(0.,2.).len(), 2. );

        assert_eq!( Point::origin().rotate(30.).len(), 0. );
        assert_eq!( Point::new(1.,0.).rotate(30.).len(), 1. );
        assert_eq!( Point::new(0.,1.).rotate(30.).len(), 1. );
        assert_eq!( Point::new(2.,0.).rotate(30.).len(), 2. );
        assert_eq!( Point::new(0.,2.).rotate(30.).len(), 2. );
    }
    #[test]
    fn test_rotate() {
        pt_eq( &Point::new(1.,0.).rotate(0.),    1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(90.),   0.,  1. );
        pt_eq( &Point::new(1.,0.).rotate(180.), -1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(270.),  0., -1. );

        pt_eq( &Point::new(0.,1.).rotate(0.),    0.,  1. );
        pt_eq( &Point::new(0.,1.).rotate(90.),  -1.,  0. );
        pt_eq( &Point::new(0.,1.).rotate(180.),  0., -1. );
        pt_eq( &Point::new(0.,1.).rotate(270.),  1.,  0. );
    }
    /*
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
    pub fn dot(self, other:&Point) -> f64 {
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
     */
}

