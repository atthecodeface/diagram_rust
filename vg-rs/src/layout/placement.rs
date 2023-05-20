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
use crate::Range;

//a Types
//tp Placements
/// A set of placements for a single dimension, where items when
/// placed have a range that they fit within
#[derive(Debug, Default)]
pub struct Placements {
    elements: Vec<Range>,
}

//ip Placements
impl Placements {
    //fp mp add_element
    /// Add an element to a 1D placemment
    pub fn add_element(
        &mut self,
        _eref: &str,
        placement: f64,
        ref_value: Option<f64>,
        min: f64,
        max: f64,
    ) {
        let ref_value = ref_value.unwrap_or(0.);
        // actual bounds are such that 'ref_value' is at 'placement'
        let min = min + placement - ref_value;
        let max = max + placement - ref_value;
        self.elements.push(Range::new(min, max));
    }

    //mp get_desired_geometry
    /// Get the union of all the content
    pub fn get_desired_geometry(&self) -> Range {
        match self.elements.len() {
            0 => Range::none(),
            _ => {
                let mut range = self.elements[0];
                for e in &self.elements {
                    range = range.union(e);
                }
                range
            }
        }
    }

    //zz All done
}
