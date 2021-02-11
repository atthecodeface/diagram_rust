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

@file    types.rs
@brief   Types derived from other libraries used throughout the diagram
 */

//a Imports
use std::cell::RefCell;
use std::rc::Rc;
use stylesheet;
use super::font;
pub type StyleValue      = stylesheet::BaseValue;
pub type StyleDescriptor = stylesheet::Descriptor<StyleValue>;
pub type RrcStyleDescriptor = stylesheet::RrcDescriptor<StyleValue>;
pub type StyleSet        = stylesheet::NamedTypeSet<StyleValue>;
pub type ValueError      = stylesheet::ValueError;
pub type RrcFont = Rc<RefCell<font::Font>>;

