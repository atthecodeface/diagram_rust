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

@file    element.rs
@brief   Diagram elements
 */

//a Constants
const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use geo_nd::Vector;
use geometry::{Rectangle, Point};
use stylesheet::TypeValue;    // For the trait, to get access to 'from_string'
use stylesheet::{StylableNode, Tree};
use crate::constants::attributes as at;
use crate::constants::elements   as el;
use crate::DiagramDescriptor;
use crate::{Layout, LayoutBox};
pub use super::elements::{Group, Shape, Path, Text, Use};
use super::types::*;
use super::DiagramElementContent;
use super::{ElementLayout, LayoutPlacement};
use super::ElementHeader;

//a ElementError
//tp ElementError
pub enum ElementError {
    UnknownId(String,String),
    Error(String,String),
}

//ii ElementError
impl ElementError {
    //fp unknown_id
    pub fn unknown_id(hdr:&ElementHeader, name:&str) -> Self {
        Self::UnknownId(hdr.borrow_id().to_string(), name.to_string())
    }
    //fp of_string
    pub fn of_string(hdr:&ElementHeader, s:&str) -> Self {
        Self::Error(hdr.borrow_id().to_string(), s.to_string())
    }
    //mi of_result
    pub fn of_result<V,E:std::fmt::Display>(hdr:&ElementHeader, result:Result<V,E>) -> Result<V,ElementError> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(ElementError::Error(hdr.borrow_id().to_string(), e.to_string()))
        }
    }

    //zz All done
}

//ip Display for ElementError
impl std::fmt::Display for ElementError {
    //mp fmt - format error for display
    /// Display the error
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ElementError::UnknownId(id,s) => write!(f, "Element id '{}': Unknown id reference '{}'", id, s),
            ElementError::Error(id,s) => write!(f, "Element id '{}': {}", id, s),
        }
    }

    //zz All done
}

