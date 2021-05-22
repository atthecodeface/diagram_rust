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

//a Documentation

/*!

Vector library

!*/

//a Imports
use num_traits::{Float};
use super::vector_op as VOp;
use self::VOp::VectorCoord;

//a Vector
//tp Vector
/// This is a simple point class for two dimensions
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<V:VectorCoord, const D:usize> {
    /// Coordinates
    pub c : [V; D],
}

//ti Display for Vector
impl <V:VectorCoord, const D:usize> std::fmt::Display for Vector<V, D> {

    //mp fmt - format a `Vector` for display
    /// Display the `Vector' as (x,y,...)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}", self.c[0] )?;
        for i in 1..D {
            write!(f, ",{}", self.c[i] )?;
        }
        write!(f,")")
    }

    //zz All done
}

//ti Vector
impl <V:VectorCoord, const D:usize> Vector<V, D> {
    //fp zero
    /// Create a new vector with zeros
    pub fn origin() -> Self {
        let c = VOp::origin();
        Self {c}
    }

    //fp is_zero
    /// Return true if the point is the origin
    pub fn is_zero(&self) -> bool {
        VOp::is_zero(&self.c)
    }

    //fp new
    /// Create a new vector
    pub fn new(c:&[V; D]) -> Self {
        let c = (*c).clone();
        Self {c}
    }

    //cp scale
    /// Consume the vector and return a new vector that is the original
    /// scaled in each coordinate a single scaling factors
    pub fn scale(mut self, s:V) -> Self {
        self.c = VOp::scale(self.c, s);
        self
    }

    //cp reduce
    /// Consume the vector and return a new vector that is the original
    /// scaled in each coordinate a single scaling factors
    pub fn reduce(mut self, s:V) -> Self {
        self.c = VOp::reduce(self.c, s);
        self
    }

    //cp add
    /// Consume the vector, and return a new vector that is the sum of
    /// this and a borrowed other vector scaled
    pub fn add(mut self, other:&Self, scale:V) -> Self {
        self.c = VOp::add(self.c, &other.c, scale);
        self
    }

    //mp len2
    /// Return the distance^2 of the point from the origin
    pub fn len2(&self) -> V {
        VOp::len2(&self.c)
    }

    //mp distance_to2
    /// Return the distance between two vectors
    pub fn distance_to2(&self, other:&Self) -> V {
        VOp::distance_to2(&self.c, &other.c)
    }

    //mp inner_product
    /// Return the inner product (dot product) of this and another vector
    pub fn inner_product(&self, other:&Self) -> V {
        VOp::inner_product(&self.c, &other.c)
    }

    //zz All done
}

//ti Vector
impl <V:VectorCoord+Float, const D:usize> Vector<V, D> {
    //mp len
    /// Create a new vector with zeros
    pub fn len(&self) -> V {
        VOp::len2(&self.c)
    }

    //mp normalize
    /// Create a new vector with zeros
    pub fn normalize(&mut self, eps:V) {
        VOp::normalize(&mut self.c, eps)
    }

    //cp rotate_around
    /// Consume the vector and return a new vector rotated around a
    /// *pivot* point by the specified angle
    pub fn rotate_around(mut self, pivot:&Self, degrees:V, c0:usize, c1:usize) -> Self {
        self.c = VOp::rotate_around(self.c, &pivot.c, degrees, c0, c1);
        self
    }

    //mp distance_to
    /// Return the distance between two vectors
    pub fn distance_to(&self, other:&Self) -> V {
        VOp::distance_to(&self.c, &other.c)
    }

}

//mt Test for Vector
#[cfg(test)]
mod test_vector {
    use super::*;
    type Point = Vector<f64, 2>;
    pub fn pt_eq(v:&Point, x:f64, y:f64) {
        assert!((v.c[0]-x).abs()<1E-8, "mismatch in x {} {} {}",v,x,y);
        assert!((v.c[1]-y).abs()<1E-8, "mismatch in y {} {} {}",v,x,y);
    }
    #[test]
    fn test_simple() {
        pt_eq( &Point::origin(), 0., 0. );
        pt_eq( &Point::new(&[1.,2.]), 1., 2. );
        assert!( Point::origin().is_zero() );
        assert!( !Point::new(&[0.1,0.]).is_zero() );
        assert!( !Point::new(&[0.,0.1]).is_zero() );
        pt_eq( &Point::new(&[1.,2.]).scale(3.), 3., 6. );
        pt_eq( &Point::new(&[1.,2.]).scale(4.), 4., 8. );

        assert_eq!( Point::origin().len2(), 0. );
        assert_eq!( Point::new(&[1.,0.]).len2(), 1. );
        assert_eq!( Point::new(&[0.,1.]).len2(), 1. );
        assert_eq!( Point::new(&[2.,0.]).len2(), 4. );
        assert_eq!( Point::new(&[0.,2.]).len2(), 4. );

    }
    #[test]
    fn test_rotate() {
        let origin = Point::origin();
        pt_eq( &Point::new(&[1.,0.]).rotate_around(&origin, 0.,   0, 1),    1.,  0. );
        pt_eq( &Point::new(&[1.,0.]).rotate_around(&origin, 90.,  0, 1),   0.,  1. );
        pt_eq( &Point::new(&[1.,0.]).rotate_around(&origin, 180., 0, 1), -1.,  0. );
        pt_eq( &Point::new(&[1.,0.]).rotate_around(&origin, 270., 0, 1),  0., -1. );

        pt_eq( &Point::new(&[0.,1.]).rotate_around(&origin, 0.,   0, 1),    0.,  1. );
        pt_eq( &Point::new(&[0.,1.]).rotate_around(&origin, 90.,  0, 1),  -1.,  0. );
        pt_eq( &Point::new(&[0.,1.]).rotate_around(&origin, 180., 0, 1),  0., -1. );
        pt_eq( &Point::new(&[0.,1.]).rotate_around(&origin, 270., 0, 1),  1.,  0. );
    }
    /*
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
    pub fn dot(self, other:&Point) -> f64 {
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
     */
}

