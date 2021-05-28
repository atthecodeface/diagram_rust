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

@file    lib.rs
@brief   Geometry library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
/*!

# Geometry library

This library provides for N-dimensional geometrical objects,
particularly *Vector*s, *Matrix*, *Quaternion* operations.

The underlying type is \[Num; N\], so the data may be shared simply
with other libraries, including OpenGL.

The library mirrors the operation of 'glm' in some sense.

The desire for the library is that it does not undergo much
development; it provides a stable and simple basis for operations that
are common mathematical operations, with no aim for it to grow into a
larger linear algebra library.

The library operates on arrays of elements that support the
[`Num`](Num) trait, which requires basic arithmetic operations, copy,
clone, debug and display; some functions further require the
[`Float`](Float) trait, which also requires operations such as sqrt,
sin/cos, etc.

The library does not expose any types: all of the operations it
supports are provided through functions.

## Caveat

For many operations the functions depend on const generics, and these
can require the use of *nightly* at present. The usage of them is not
complicated, and should not be unstable, but it is still early days
for const generics, so there you go.

## Basic operation

```
extern crate geometry;
use geometry::vector;
let y = [0., 1.];
let x = [1., 0.];
assert_eq!( vector::dot(&x, &y), 0., "Dot product of X and Y axis vectors is zero");
let xy = vector::add(x,&y,2.);
assert_eq!( xy, [1., 2.], "x + 2*y");
assert_eq!( vector::len_sq(&xy), (5.), "|x + 2*y|^2 = 5");
assert_eq!( vector::len(&xy), (5.0f64).sqrt(), "|x + 2*y| = sqrt(5)");
```

!*/

//a Crates
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
extern crate num_traits;

//a Public trait
//tp Num
/// Trait required for matrix or vector elements
pub trait Num : std::ops::Neg<Output=Self> + num_traits::Num +
    Clone + Copy + PartialEq + std::fmt::Display + std::fmt::Debug {}

//tp Float
/// Trait required for matrix or vector elements such that also need operations such as sqrt, sin/cos, etc
pub trait Float : Num + num_traits::Float {}

//ti Num for f32/f64/i32/i64/isize
impl Num for f32 {}
impl Num for f64 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for isize {}

//ti Float for f32/f64
impl Float for f32 {}
impl Float for f64 {}

//a Imports and exports
mod vector_op;
mod quaternion_op;
mod matrixr_op;
mod matrix_op;

/// Vector functions module
///
/// This module provides numerous N-dimensional vector operations operating on [Num; N] (or [Float; N]).
pub mod vector   {
    pub use super::vector_op::* ;
}

/// Quaternion module
pub mod quat     {
    pub use super::quaternion_op::* ;
}

/// Matrix library
pub mod matrix   {
    pub use super::matrixr_op::* ;
    pub use super::matrix_op::* ;
}

//a Generic types as per GLSL
pub type Vec2 = [f32;2];
pub type Vec3 = [f32;3];
pub type Vec4 = [f32;4];
pub type DVec2 = [f64;2];
pub type DVec3 = [f64;3];
pub type DVec4 = [f64;4];
pub type IVec2 = [i32;2];
pub type IVec3 = [i32;3];
pub type IVec4 = [i32;4];
pub type Mat2 = [f32;4];
pub type Mat3 = [f32;9];
pub type Mat4 = [f32;16];
pub type DMat2 = [f64;4];
pub type DMat3 = [f64;9];
pub type DMat4 = [f64;16];

