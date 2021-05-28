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

//a Constructors
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
/// Update the new matrix with every element e scaled so that max(abs(e)) is 1.
pub fn normalize<V:Float,const R:usize,const C:usize> (mut m:[V;R*C]) -> [V;R*C] {
    let l = absmax::<V,R,C>(&m);
    if l < V::epsilon() {
        set_zero::<V,R,C>(&mut m); m
    } else {
        reduce::<V,R,C>(m, l)
    }
}

//cp transpose
/// Transpose the matrix
///
/// The matrices are row-major, so successive entries of m are adjacent columns
///
/// The output matrix has adjacent entries that are therefore adjacent rows in m
pub fn transpose<V:Float,const R:usize,const C:usize> (m:[V;R*C]) -> [V;R*C] {
    let mut v = zero::<V,R,C>();
    for r in 0..R {
        for c in 0..C {
            v[r+c*R] = m[c+r*C];
        }
    }
    v
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

//a Formatting
//mp fmt - format a `Matrix` for display
/// Format the matrix for display
///
/// This can be used by the Display or Debug traits of a type which
/// contains an array. It is not possible to use this method directly
/// without a `Formatter` instance.
///
/// To display an array `m` as a matrix use `&MatrixType::new(&m)` (see [MatrixType])
///
/// # Examples
///
/// ```
/// use geometry::matrix;
/// struct Mat { c : [f32;6] };
/// impl std::fmt::Display for Mat {
///   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { matrix::fmt::<f32,2>(f, &self.c) }
/// }
/// assert_eq!( format!("{}", &Mat{c:[0., 1., 2., 3., 4., 5.]} ), "[0,1 2,3 4,5]" );
/// ```
pub fn fmt<V:Num,const C:usize>(f: &mut std::fmt::Formatter, v : &[V]) -> std::fmt::Result {
    let mut c=0;
    for i in 0..v.len() {
        if i==0 {
            write!(f, "[{}", v[i])?;
        } else if c==0 {
            write!(f, " {}", v[i])?;
        } else {
            write!(f, ",{}", v[i])?;
        }
        c += 1;
        if c == C { c=0; }
    }
    write!(f, "]")
}

//a MatrixType, used in formatting
/// A structure that supports the Debug and Display traits, borrowing
/// a matrix; this permits a relatively simple format of a matrix
/// through using the [`new`] constructor of the type
///
/// # Example
///
/// ```
/// use geometry::matrix::{identity, MatrixType};
/// assert_eq!( format!("{}", MatrixType::<f32,2,2>::new(&identity::<f32,2>())), "[1,0 0,1]" );
/// ```
///
#[derive(Debug)]
pub struct MatrixType<'a, V:Num,const R:usize,const C:usize> where [(); R*C]:Sized {
    /// Contents of the matrix
    m : &'a [V;R*C],
}

//ip MatrixType
impl <'a, V:Num,const R:usize,const C:usize> MatrixType<'a,V,R,C>  where [(); R*C]:Sized {
    //fp new
    /// Create a new MatrixType by borrowing an array of Num; this may then be formatted using its Display trait
    pub fn new(m:&'a [V;R*C]) -> Self { Self {m} }
}

//ip Display for MatrixType
impl <'a, V:Num,const R:usize,const C:usize> std::fmt::Display for MatrixType<'a,V,R,C>  where [(); R*C]:Sized {
    //
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { fmt::<V,C>(f, self.m) }
}
