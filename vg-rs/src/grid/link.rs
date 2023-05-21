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
use super::NodeId;

//a Link
//tp Link
/// A link between two grid points, specified by NodeId
///
/// This has a minimum size, and an optional growth factor; if the
/// growth is None then the link is of a fixed size. The larger the
/// growth value the weaker (more elastic) the link
#[derive(Debug)]
pub struct Link<N: NodeId> {
    /// Start NodeID (left-hand end)
    #[allow(dead_code)]
    start: N,
    /// End NodeID (right-hand end)
    #[allow(dead_code)]
    end: N,
    /// Minimum size
    min_size: f64,
    /// Growth factor; the larger, the easier it is to grow; None
    /// means it is fixed size
    growth: Option<f64>,
}

//ip Link
impl<N: NodeId> Link<N> {
    //fp new
    /// Create a new link with unknown growth
    pub fn new(start: N, end: N, min_size: f64) -> Self {
        Self {
            start,
            end,
            min_size,
            growth: None,
        }
    }

    //ap min_size
    pub fn min_size(&self) -> f64 {
        self.min_size
    }

    //ap growth
    pub fn growth(&self) -> Option<f64> {
        self.growth
    }

    //fp union
    /// Make the link at least the size of the larger link
    pub fn union(&mut self, min_size: f64) {
        if min_size > self.min_size {
            self.min_size = min_size;
        }
    }

    //fp set_growth
    /// Take the growth from the new one if it has a growth
    pub fn set_growth(&mut self, growth: f64) {
        self.growth = Some(growth);
    }

    //zz All done
}
