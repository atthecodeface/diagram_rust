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
@brief   Diagram library
 */

//a Documentation
//! The diagram library provides the support for creating styled
//! diagrams, usually using a markup language to describe a diagram
//! and its styling, and generating an SVG output.
//!
//! The diagrams use a boxed layout model, similar to web pages - and
//! the styling is similar to cascading stylesheets.
//!
//! Layout types
//!
//! A grid layout uses a specification for each cell that participates
//! in the layout, using a pair of cell start/end indications for the
//! cell. The X and Y are laid out independently. All the cells within
//! the grid are defined, and then styled (given style values from the
//! stylesheet). Then the cells contents are interrogated to determine
//! their *desired size*, to which scaling, rotation, padding, border
//! and margin are added. The grid layout uses the cell start/end
//! indications and the desired size to determine the demands of the
//! cell on the X and Y dimensions of the grid layout. This produces a
//! basic layout for each cell grid X and Y value mapping to a desired
//! grid X and Y value; and the grid therefore has a desired size.
//!
//! The grid layout will eventually be asked to be laid out in a real
//! geometry for the diagram.  At this point the real geometry may be
//! larger than the desired, in which case the grid may be expanded if
//! required by the styles.
//!
//! To permit the styling of the layout the grid may also be provided
//! with minimum sizes for cell ranges, in the styling of the `layout`
//! element. These are lists of <int> (<float> <int>)*; the ints
//! should be in increasing order, and they specify the cell
//! boundaries; the floats are the minimum size between its two
//! neighboring float boundaries.
//!
//! An example layout could be just two elements:
//!
//! ```
//!  #layout ##shape id=a grid=1,1 ##shape id=b grid=2,1
//! ```
//!
//! This specifies two shapes, one at grid cell (1,1,2,2) (there is a
//! default span of one row and one column), and the second at grid
//! cell (2,1,3,2). The grid therefore has in the X dimension cell
//! boundaries at 1, 2 and 3; in Y it just has 1 and 2 (i.e. a single row).
//!
//! These two shapes will be laid out, therefore, in a single row,
//! using the sizes required by the shapes. The row will be tall
//! enough for the taller of the two shapes.
//!
//! If the shapes are of different size, but the desire is to have the
//! cells be the same width of 50 (provided the shapes are smaller
//! than that) then one can provide the minimum sizes:
//!
//! ```
//!  #layout minx=1,50.,2,50.,3 ##shape id=a grid=1,1 ##shape id=b grid=2,1
//! ```
//!
//! Now the minimum width (X dimension) between cell 1 and cell 2 will
//! be 50. units, and the same is required between cells 2 and 3.

//a Crates
extern crate xml;
extern crate hmlm;
extern crate stylesheet;

//a Imports and exports
mod layout;
mod diagram;
mod diagram_ml;
pub use layout::{Polygon, Rectangle, Point, Bezier};
pub use layout::{Transform, Layout, LayoutBox, LayoutRecord, GridCellData};

pub use diagram::{Diagram, DiagramContents, DiagramDescriptor, Element, ElementError, Svg, GenerateSvg};
pub use diagram_ml::{DiagramML};
