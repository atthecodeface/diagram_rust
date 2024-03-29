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

@file    svg_error.rs
@brief   Errors when generating SVG output
 */

//a Imports
use thiserror::Error;

//a Error
//tp Error
/// Errors used in the Vector Graphics library
#[derive(Error, Debug)]
pub enum Error {
    /// An invalid matrix transformation
    #[error("Invalid transformation matrix, {reason}")]
    InvalidTransformationMatrix {
        /// The underlying reason for the error
        reason: String,
    },
}
