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

@file    named_type_set.rs
@brief   A set of name -> (stype:StyleTypeValue, inheritable:bool)  instances
 */

//a Imports
use crate::StyleTypeValue;
use std::collections::HashMap;

//tp NamedTypeSet
/// This is a set of name => (StyleTypeValue, inhertiable:bool) pairs
///
/// This is used as the complete allowable set of stylable properties
/// and their types for a stylesheet
#[derive(Debug, Default)]
pub struct NamedTypeSet {
    set: HashMap<String, (StyleTypeValue, bool)>,
}

//ti std::fmt::Display for NamedTypeSet
impl std::fmt::Display for NamedTypeSet {
    //mp fmt - format for display
    /// Display the style id
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, (value, inheritable)) in self.set.iter() {
            write!(f, "   {} => {}:{}\n", name, inheritable, value.as_type())?;
        }
        Ok(())
    }

    //zz All done
}

//ti NamedTypeSet
impl NamedTypeSet {
    //cp add_type
    /// Constructor to add a new named type to the set, and indicated
    /// whether it is inherited from a parent
    pub fn add_type(mut self, s: &str, value: StyleTypeValue, inheritable: bool) -> Self {
        self.set.insert(s.to_string(), (value, inheritable));
        self
    }

    //cp borrow_type
    /// Borrow a type from the set, if it is there, and whether it is inheritable
    pub fn borrow_type(&self, s: &str) -> Option<(&StyleTypeValue, bool)> {
        match self.set.get(s) {
            Some((value, inheritable)) => Some((value, *inheritable)),
            _ => None,
        }
    }

    //zz All done
}
