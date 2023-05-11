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
@brief   Vector Graphics Library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
/*!
# Vector Graphics Library

This library provides a vector graphics library, supporting
2-dimensional graphics with fonts etc, and graphs

!*/

//a Modules
mod colors;
mod error;
mod geometry;
pub mod grid;
pub mod layout;
mod shapes;

/// The [Point] type is a 2D point of f64's
pub type Point = geo_nd::FArray<f64, 2>;

/// The [Bezier] type is a Bezier curve of [Point]s
pub type Bezier = bezier_nd::Bezier<f64, Point, 2>;

pub use colors::{Color, ColorDatabase, Rgba, COLOR_DB_SVG};
pub use error::Error;
pub use geometry::{BBox, Container, ContainerBorder, Edge, MBox, Margin, Range, Transform};
pub use shapes::{BezierPath, Polygon};
