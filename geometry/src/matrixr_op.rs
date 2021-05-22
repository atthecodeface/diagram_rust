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

@file    matrixr_op.rs
@brief   Part of geometry library
 */

//a Imports
use num_traits::{Float};
use crate::{vector_op, VectorCoord};

//fp zero
/// Crate a new matrix which is all zeros
pub fn zero<V:VectorCoord,const R:usize,const C:usize> () -> [V; R*C] { [V::zero();R*C] }

//mp set_zero
/// Set the matrix to have all elements of zero
pub fn set_zero<V:VectorCoord,const R:usize,const C:usize> (m:&mut [V;R*C]) {
    vector_op::set_zero(m)
}

//fp is_zero
/// Return true if the matrix is all zeros
pub fn is_zero<V:VectorCoord,const R:usize,const C:usize> (m:&[V;R*C]) -> bool {
    vector_op::is_zero(m)
}

//cp scale
/// Consume the vector and return a new vector that is the original
/// scaled in each coordinate a single scaling factor
pub fn scale<V:VectorCoord,const R:usize,const C:usize> (m:[V;R*C], s:V) -> [V;R*C] {
    vector_op::scale(m, s)
}

//cp reduce
/// Consume the vector and return a new vector that is the original
/// reduces in scale in each coordinate by a single scaling factor
pub fn reduce<V:VectorCoord,const R:usize,const C:usize> (m:[V;R*C], s:V) -> [V;R*C] {
    vector_op::reduce(m, s)
}

//cp add
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn add<V:VectorCoord,const R:usize,const C:usize> (m:[V;R*C], other:&[V;R*C], scale:V) -> [V;R*C] {
    vector_op::add(m, other, scale)
}

//cp absmax
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn absmax<V:VectorCoord+Float,const R:usize,const C:usize> (m:&[V;R*C]) -> V {
    m.iter().fold(V::zero(), |acc, c| V::max(acc,V::abs(*c)))
}

//cp normalize
/// Create a new vector with zeros
pub fn normalize<V:VectorCoord+Float,const R:usize,const C:usize> (mut m:[V;R*C], eps:V) -> [V;R*C] {
    let l = absmax::<V,R,C>(&m);
    if l < eps {
        set_zero::<V,R,C>(&mut m); m
    } else {
        reduce::<V,R,C>(m, l)
    }
}

//mp multiply
/// Multiply two matrices
pub fn multiply<V:VectorCoord+Float,const R:usize,const X:usize,const C:usize> (a:&[V;R*X], b:&[V;X*C]) -> [V;R*C] {
    let mut m = [V::zero();R*C];
    for r in 0..R {
        for c in 0..C {
            let mut v = V::zero();
            for x in 0..X {
                v = v + a[r*X+x]*b[x*C+c];
            }
            m[r*C+c] = v;
        }
    }
    m
}

//mp transform_vec
/// Transform a vector
pub fn transform_vec<V:VectorCoord+Float,const R:usize,const D:usize> (m:&[V;R*D], v:&[V;D*1]) -> [V;R*1] {
    multiply::<V,R,D,1> (m, v)
}
