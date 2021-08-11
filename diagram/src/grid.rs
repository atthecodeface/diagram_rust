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

@file    grid.rs
@brief   Grid placement, layout, growth, resolution etc
 */

//a Imports
mod traits;
mod link;
mod node;
mod equation_set;
mod lup_decomposition;
mod grid_data;
mod grid_cell_data;
mod resolver;
mod grid_placement;

pub use traits::NodeId;
pub use link::Link;
pub use node::Node;
pub use lup_decomposition::LUPDecomposition;
pub use equation_set::EquationSet;
pub use resolver::Resolver;
pub use grid_placement::{GridPlacement};
// pub use grid_dimension::{GridDimension, GridDimensionIter};
pub use grid_cell_data::{GridCellDataEntry};
pub use grid_data::{GridData};
