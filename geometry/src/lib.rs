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

//a Imports and exports
mod point;
mod bezier;
mod rectangle;
mod polygon;
mod transform;

pub use self::transform::Transform;
pub use self::point::{Point, Range};
pub use self::bezier::Bezier;
pub use self::rectangle::{Rectangle, Float4};
pub use self::polygon::Polygon;

