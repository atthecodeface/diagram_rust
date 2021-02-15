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

@file    placement.rs
@brief   Placed items
 */

//a Imports
use crate::{Range};

//a Types
#[derive(Debug)]
pub struct Placements {
    elements : Vec<Range>
}

impl Placements {
    pub fn new() -> Self {
        Self { elements:Vec::new(),
        }
    }
    pub fn add_element(&mut self, placement:f64, ref_value:Option<f64>, min:f64, max:f64) {
        let ref_value = ref_value.unwrap_or(0.);
        // actual bounds are such that 'ref_value' is at 'placement'
        let min = min + placement - ref_value;
        let max = max + placement - ref_value;
        self.elements.push(Range::new(min,max));
    }
    pub fn get_desired_geometry(&self) -> Range {
        match self.elements.len() {
            0 => Range::none(),
            _ => {
                let mut range = self.elements[0].clone();
                for e in &self.elements {
                    range = range.union(e);
                }
                range
            },
        }
    }
}
