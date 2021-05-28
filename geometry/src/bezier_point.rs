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

@file    bezier_point.rs
@brief   Part of geometry library
 */

//a Imports
use super::vector::{Vector, VectorCoord};
use super::bezier_line::BezierLineIter;
use super::bezier::Bezier;

//a BezierPointIter
//tp BezierPointIter
/// An iterator of points that form a single Bezier curve where the
/// steps between points would be lines that are 'straight enough'
///
/// This iterator returns the points that BezierLineIter uses, in the
/// same order (pa, pb, ...)
pub struct BezierPointIter<V:VectorCoord, const D:usize> {
    /// A line iterator that returns the next line segment required;
    /// usually the first point of this segment that this iterator
    /// provides is returned as the next point.
    ///
    /// When this returns none, the end-point of the previous
    /// iteration needs to be returned as the last point.
    lines : BezierLineIter<V,D>,
    /// The last point to be returned - if this is valid then the line
    /// iterator has finished, and just the last point on the Bezier
    /// needs to be returned.
    last_point : Option<Vector<V,D>>,
}

//ip BezierPointIter
impl <V:VectorCoord, const D:usize> BezierPointIter<V,D> {
    //fp new
    /// Create a new point iterator from a line iterator
    pub fn new(lines:BezierLineIter<V,D>) -> Self {
        Self { lines, last_point:None }
    }

    //zz All done
}

//ii BezierPointIter
impl <V:VectorCoord, const D:usize> Iterator for BezierPointIter<V, D> {
    /// Iterator returns Point's
    type Item = Vector<V,D>;

    /// Return the first point of any line segment provided by the
    /// line iterator, but record the endpoint of that segment first;
    /// if the line iterator has finished then return any recorded
    /// endpoint, deleting it first.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some( (p0, p1) ) = self.lines.next() {
            self.last_point = Some(p1);
            Some(p0)
        } else {
            let p = self.last_point;
            self.last_point = None;
            p
        }
    }

    //zz All done
}
