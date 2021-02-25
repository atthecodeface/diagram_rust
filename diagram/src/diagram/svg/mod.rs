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

@file    svg/mod.rs
@brief   Generate SVG output
 */

//a Imports
mod svg_error;
mod svg_element_iter;
mod svg_element;
mod svg;

pub use self::svg_error::{SvgError};
pub use self::svg_element_iter::{ElementIter};
pub use self::svg_element::{SvgElement};
pub use self::svg::{Svg, GenerateSvg, GenerateSvgElement};
