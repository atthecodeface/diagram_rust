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
@brief   Grid layout
 */

//a Imports
use crate::grid::NodeId;

//a GridData
//tp GridData
/// This enumeration allows adjusting the specification of a grid
/// dimension, mapping one or more nodes to an attribute (such as the
/// expected size of the separation of two nodes)
#[derive(Debug)]
pub enum GridData<N: NodeId> {
    /// Specify the width between two nodes
    Width(N, N, f64),
    /// Specify the growth (elasticity) of the link between two nodes
    Growth(N, N, f64),
    /// Specify the placement of a node
    Place(N, f64),
}

//ip GridData
impl<N: NodeId> GridData<N> {
    //fp new_width
    /// Create a new [GridData] element
    pub fn new_width(start: N, end: N, size: f64) -> Self {
        Self::Width(start, end, size)
    }

    //fp new_growth
    /// Create a new [GridData] element
    pub fn new_growth(start: N, end: N, size: f64) -> Self {
        Self::Growth(start, end, size)
    }

    //fp new_place
    /// Create a new [GridData] element
    pub fn new_place(start: N, size: f64) -> Self {
        Self::Place(start, size)
    }
}

//ip Display for GridData
impl<N: NodeId> std::fmt::Display for GridData<N> {
    //mp fmt - format a GridData
    /// Display the `GridData' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Width(s, e, w) => write!(f, "[{}->{}:{}]", s, e, w),
            Self::Growth(s, e, g) => write!(f, "[{}->{}:+{}]", s, e, g),
            Self::Place(s, p) => write!(f, "[{}@{}]", s, p),
        }
    }

    //zz All done
}
