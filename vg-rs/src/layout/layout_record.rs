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

@file    layout_record.rs
@brief   Recording the layout of placed items and grids
 */

//a Imports
use std::collections::HashMap;

use crate::layout::Layout;

//a LayoutRecord
//tp LayoutRecord
/// A type used to preserve the layout for, e.g., display as a grid
#[derive(Debug, Default)]
pub struct LayoutRecord {
    grid_positions: Option<(HashMap<String, f64>, HashMap<String, f64>)>,
}

//ip LayoutRecord
impl LayoutRecord {
    //ap grid_positions
    /// Borrow the grid positions
    pub fn grid_positions(&self) -> Option<&(HashMap<String, f64>, HashMap<String, f64>)> {
        self.grid_positions.as_ref()
    }

    //mp capture_grid
    /// Capture the grid positions from a layout
    pub fn capture_grid(&mut self, layout: &Layout) -> Result<(), String> {
        self.grid_positions = Some(layout.get_grid_positions()?);
        Ok(())
    }

    //zz All done
}
