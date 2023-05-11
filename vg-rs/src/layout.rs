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

@file    layout.rs
@brief   Layout for Vector Graphics Library
 */

//a Documentation
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
/*!
# Vector Graphics Library

!*/

//a Imports and exports
mod layout;
mod layout_box;
mod layout_record;
mod placement;

pub use layout::Layout;
pub use layout_box::LayoutBox;
pub use layout_record::LayoutRecord;
pub use placement::Placements;
