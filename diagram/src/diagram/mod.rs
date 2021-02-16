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
@brief   Diagram module
 */

//a Imports
mod font;
mod text;
mod descriptor;
mod element;
mod diagram;
mod svg;
mod elements;

pub mod types;
pub use self::types::{ValueError, StyleSheet, StyleRule, StyleAction};
pub use self::descriptor::DiagramDescriptor;
pub use self::element::{Element, ElementScope, ElementError, ElementContent, ElementHeader, DiagramElementContent, ElementLayout};
pub use self::elements::{Shape, Group, Text, Use};
pub use self::diagram::{Diagram, DiagramContents};
pub use self::svg::{Svg, SvgElement, GenerateSvg, GenerateSvgElement, SvgError};
