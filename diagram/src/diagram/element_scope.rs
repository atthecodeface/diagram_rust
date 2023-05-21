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
// const DEBUG_ELEMENT_HEADER : bool = 1 == 0;

//a Imports
use super::Element;
use super::ElementError;
use super::ElementHeader;

//a ElementScope<'a> - 'a is the lifetime of the definition elements
//tp ElementScope
#[derive(Debug)]
pub struct ElementScope<'a, 'b> {
    pub id_prefix: String,
    definitions: &'b Vec<Element<'a>>,
    pub depth: usize,
}

//ip ElementScope
impl<'a, 'b> ElementScope<'a, 'b> {
    //fp new
    pub fn new(id_prefix: &str, definitions: &'b Vec<Element<'a>>) -> Self {
        let id_prefix = id_prefix.to_string();
        Self {
            id_prefix,
            definitions,
            depth: 0,
        }
    }
    //mp new_subscope
    pub fn new_subscope<'c>(
        &'c self,
        header: &ElementHeader<'a>,
        name: &str,
        depth: usize,
    ) -> Result<(ElementScope<'a, 'c>, &'c Element<'a>), ElementError> {
        if depth > 50 {
            Err(ElementError::of_string(
                header,
                &format!("Maximum scope depth of {} reached - recursive Use?", depth),
            ))
        } else {
            let n = self.definitions.len();
            let mut index = None;
            for i in 0..n {
                if self.definitions[i].has_id(name) {
                    index = Some(i);
                }
            }
            if let Some(index) = index {
                let mut id_prefix = self.id_prefix.clone();
                id_prefix.push_str(header.borrow_id());
                id_prefix.push('.');
                id_prefix.push_str(name);
                let definitions = self.definitions;
                let element = &self.definitions[index];
                Ok((
                    Self {
                        id_prefix,
                        definitions,
                        depth,
                    },
                    element,
                ))
            } else {
                Err(ElementError::unknown_id(header, name))
            }
        }
    }
}
