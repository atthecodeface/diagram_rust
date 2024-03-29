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

/*!
This library provides a gridded placement system for a single dimension

It utilizes the concept of grid Nodes (with a user-defined identifier
type), which are placed in the grid with relations between them
defined. This is handled with the [GridPlacement]; a node identifier
can be any type that supports the NodeId type.

The [GridPlacement] is first created (with default()), and cell data
is added. This cell data specifies the placement of grid node IDs, or
the gap between two, or the elasticity of the gaps.

Once fully specified a [GridPlacement] can be calculated, which
resolves the placement given a centre, maximum size, and expansion
value; then the size of any span, or position of any node, can be
found

!*/

//a Imports
mod equation_set;
mod grid_cell_data;
mod grid_data;
mod grid_placement;
mod link;
mod lup_decomposition;
mod node;
mod resolver;
mod traits;

pub use equation_set::EquationSet;
pub use grid_cell_data::GridCellDataEntry;
pub use grid_data::GridData;
pub use grid_placement::GridPlacement;
pub use link::Link;
pub use lup_decomposition::LUPDecomposition;
pub use node::Node;
pub use resolver::Resolver;
pub use traits::NodeId;
