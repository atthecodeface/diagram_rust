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
mod descriptor;
mod diagram;
mod element;
mod element_content;
mod element_error;
mod element_header;
mod element_layout;
mod element_scope;
mod elements;
mod font;
mod svg;
mod text;
mod traits;

pub mod types;
pub use traits::DiagramElementContent;

pub use self::descriptor::DiagramDescriptor;
pub use self::diagram::{Diagram, DiagramContents};
pub use self::types::{IndentOptions, StyleRule, StyleSheet, ValueError}; // , StyleAction};
pub use element::Element;
pub use element_content::ElementContent;
pub use element_error::ElementError;
pub use element_header::ElementHeader;
pub use element_layout::{ElementLayout, LayoutPlacement};
pub use element_scope::ElementScope;
pub use elements::{Group, Path, Shape, Text, Use};
pub use svg::{GenerateSvg, GenerateSvgElement, Svg, SvgElement, SvgError};
