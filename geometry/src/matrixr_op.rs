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
@brief   Rectangular matrix operations - part of geometry library
 */

//a Imports
use crate::{Num, Float};
use crate::vector_op as vector;

//fp zero
/// Crate a new matrix which is all zeros
pub fn zero<V:Num,const R:usize,const C:usize> () -> [V; R*C] { [V::zero();R*C] }

//mp set_zero
/// Set the matrix to have all elements of zero
pub fn set_zero<V:Num,const R:usize,const C:usize> (m:&mut [V;R*C]) {
    vector::set_zero(m)
}

//fp is_zero
/// Return true if the matrix is all zeros
pub fn is_zero<V:Num,const R:usize,const C:usize> (m:&[V;R*C]) -> bool {
    vector::is_zero(m)
}

//cp scale
/// Consume the vector and return a new vector that is the original
/// scaled in each coordinate a single scaling factor
pub fn scale<V:Num,const R:usize,const C:usize> (m:[V;R*C], s:V) -> [V;R*C] {
    vector::scale(m, s)
}

//cp reduce
/// Consume the vector and return a new vector that is the original
/// reduces in scale in each coordinate by a single scaling factor
pub fn reduce<V:Num,const R:usize,const C:usize> (m:[V;R*C], s:V) -> [V;R*C] {
    vector::reduce(m, s)
}

//cp add
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn add<V:Num,const R:usize,const C:usize> (m:[V;R*C], other:&[V;R*C], scale:V) -> [V;R*C] {
    vector::add(m, other, scale)
}

//cp absmax
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn absmax<V:Float,const R:usize,const C:usize> (m:&[V;R*C]) -> V {
    m.iter().fold(V::zero(), |acc, c| V::max(acc,V::abs(*c)))
}

//cp normalize
/// Create a new vector with zeros
pub fn normalize<V:Float,const R:usize,const C:usize> (mut m:[V;R*C], eps:V) -> [V;R*C] {
    let l = absmax::<V,R,C>(&m);
    if l < eps {
        set_zero::<V,R,C>(&mut m); m
    } else {
        reduce::<V,R,C>(m, l)
    }
}

//mp multiply_old
#[allow(dead_code)]
/// Multiply two matrices
fn multiply_old<V:Float,const R:usize,const X:usize,const C:usize> (a:&[V;R*X], b:&[V;X*C]) -> [V;R*C] {
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
#[warn(dead_code)]

//mp multiply
/// Multiply two matrices
pub fn multiply<V:Float,const RX:usize, const XC:usize, const RC:usize, const R:usize, const X:usize, const C:usize> (a:&[V;RX], b:&[V;XC]) -> [V;RC] {
    assert_eq!(RX, R*X);
    assert_eq!(RC, R*C);
    assert_eq!(XC, X*C);
    let mut m = [V::zero();RC];
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
pub fn transform_vec<V:Float,const RD:usize,const R:usize,const D:usize> (m:&[V;RD], v:&[V;D]) -> [V;R] {
    multiply::<V,RD,D,R,R,D,1> (m, v)
}
