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

This library provides for two-dimensional geometrical objects,
particularly *Point*s, *Bezier* curves, and *Polygon*s, but also
*Rectangle*s and some other useful items.

The library was originally written for the *diagram* utility, and so
some of its features are focused in that direction.

!*/

//a Crates
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
extern crate num_traits;

//a Imports and exports
mod vector_op;
mod quaternion_op;
mod matrixr_op;
mod matrix_op;
// mod vector;
mod point;
// mod range;
// mod bezier;
// mod bezier_line;
// mod bezier_point;
// mod path;
// mod rectangle;
// mod polygon;
mod transform;

pub use self::transform::Transform;
// pub use self::range::{Range};
pub use self::point::{Point};
pub use vector_op::{VectorCoord};
/// Vector library
pub mod vector   { pub use super::vector_op::* ; }
/// Quaternion library
pub mod quat     { pub use super::quaternion_op::* ; }
/// Matrix library
pub mod matrix   { pub use super::matrixr_op::* ;
                   pub use super::matrix_op::* ;}
// pub use self::bezier::Bezier;
// pub use self::path::{BezierPath};
// pub use self::rectangle::{Rectangle, Float4};
// pub use self::polygon::Polygon;

