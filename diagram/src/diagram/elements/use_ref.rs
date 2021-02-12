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

@file    group.rs
@brief   Diagram group element
 */

//a Imports
use super::super::{DiagramElementContent, ElementHeader};
use super::super::types::*;

//a Use element
//tp Use - an Element that is a reference to a group or other element
#[derive(Debug)]
pub struct Use {
    // has Transform - to put it somewhere!
    id_ref  : String,
}

//ti DiagramElementContent for Use
impl DiagramElementContent for Use {
    //fp new
    /// Create a new element of the given name
    fn new(_header:&ElementHeader, _name:&str) -> Result<Self,ValueError> {
        Ok(Self { id_ref:"".to_string() })
    }
    //fp get_descriptor
    fn get_descriptor(nts:&StyleSet, _name:&str) -> RrcStyleDescriptor {
        ElementHeader::get_descriptor(nts)
    }
    //zz All done
}

//ti Use
impl Use {
}

