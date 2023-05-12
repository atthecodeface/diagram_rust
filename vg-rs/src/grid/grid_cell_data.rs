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

//a Internal types
//tp GridCellDataEntry
/// This holds the desired placement of actual data with overlapping GridData in an array (the GridCellData
/// structure)
#[derive(Debug, Clone)]
pub struct GridCellDataEntry {
    /// start is the index of the left-hand edge of the cell in the
    /// grid dimension
    pub start: usize,
    /// end is the index of the right-hand edge of the cell in the
    /// grid dimension
    pub end: usize,
    /// size is the desired size, or actual size post-expansion
    pub size: f64,
}

//ii GridCellDataEntry
impl GridCellDataEntry {
    //fp new
    pub fn new(start: usize, end: usize, size: f64) -> Self {
        Self { start, end, size }
    }
}

//it Display for GridCellDataEntry
impl std::fmt::Display for GridCellDataEntry {
    //mp fmt - format a GridCellDataEntry
    /// Display the `GridCellDataEntry' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}->{}:{}", self.start, self.end, self.size)
    }

    //zz All done
}