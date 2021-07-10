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

@file    mod.rs
@brief   Diagram Markup Reader module
 */

/*a to do

MLError from parse error - need MLError to be generic on the Reader, not just the Position
Make ParseError be of hml::ReaderError<R:Reader> then it will include the span

 */

//a Imports
mod error;
mod error_list;
mod name_ids;
mod ml_event;
mod ml_reader;
mod diagram_ml;
pub use name_ids::{NameIds, KnownName};
pub use error::{MLError, MLResult};
pub use error_list::MLErrorList;
pub use ml_reader::MLReader;
pub use ml_event::MLReadElement;

pub use diagram_ml::DiagramML;

