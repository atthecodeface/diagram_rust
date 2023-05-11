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

//a GridData
//tp GridData
/// Used in external interfaces
#[derive(Debug)]
pub struct GridData {
    pub start: isize,
    pub end: isize,
    pub size: f64,
}

//ip GridData
impl GridData {
    //fp new
    /// Create a new [GridData] element
    pub fn new(start: isize, end: isize, size: f64) -> Self {
        Self { start, end, size }
    }
}

//ip Display for GridData
impl std::fmt::Display for GridData {
    //mp fmt - format a GridData
    /// Display the `GridData' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}->{}:{}]", self.start, self.end, self.size)
    }

    //zz All done
}
