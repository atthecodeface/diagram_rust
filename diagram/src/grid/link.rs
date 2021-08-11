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
use super::{NodeId};

//a Link
//tp Link
/// A link between two grid points
#[derive(Debug)]
pub struct Link<N:NodeId> {
    /// Start client id
    pub start    : N,
    pub end      : N,
    pub min_size : f64,
    pub growth   : Option<f64>,
}

//ip Link
impl <N:NodeId> Link<N> {
    //fp new
    /// Create a new link with unknown growth
    pub fn new(start:N, end:N, min_size:f64) -> Self {
        Self {
            start, end, min_size,
            growth   : None,
        }
    }

    //fp union
    /// Make the link at least the size of the larger link
    pub fn union(&mut self, min_size:f64) {
        if min_size > self.min_size {
            self.min_size = min_size;
        }
    }

    //fp set_growth
    /// Take the growth from the new one if it has a growth
    pub fn set_growth(&mut self, growth:f64) {
        self.growth = Some(growth);
    }

    //zz All done
}

