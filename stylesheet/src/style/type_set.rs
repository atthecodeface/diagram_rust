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
use std::collections::HashMap;

use crate::StyleTypeValue;

//a TypeSet
//tp TypeSetValue
#[derive(Debug)]
pub struct TypeSetValue {
    type_value: StyleTypeValue,
    inheritable: bool,
}

impl TypeSetValue {
    //ap type_value
    /// Get a reference to the [StyleTypeValue]
    pub fn type_value(&self) -> &StyleTypeValue {
        &self.type_value
    }
    //ap inheritable
    /// Return true if the [TypeSetValue] is inheritable by a child
    /// from its parent
    pub fn inheritable(&self) -> bool {
        self.inheritable
    }
}

//ti std::fmt::Display for TypeSetValue
impl std::fmt::Display for TypeSetValue {
    //mp fmt - format for display
    /// Display the style id
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.inheritable, self.type_value.type_name())
    }
}

//tp TypeSet
/// A [TypeSet] is a collection of [StyleTypeValue] as types, each with a given name
///
/// The model of the stylesheet library is to define a stylesheet-wide (in a sense, *global*)
/// [TypeSet] that contains all the possible style names that stylable
/// elements may have; the set of styles that a particular element
/// type may then have is a subset of the [TypeSet].
///
/// This is used as the complete allowable set of stylable properties
/// and their types for a stylesheet
#[derive(Debug, Default)]
pub struct TypeSet {
    set: HashMap<String, TypeSetValue>,
}

//ti std::fmt::Display for TypeSet
impl std::fmt::Display for TypeSet {
    //mp fmt - format for display
    /// Display the style id
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, value) in self.set.iter() {
            writeln!(f, "   {} => {}", name, value)?;
        }
        Ok(())
    }

    //zz All done
}

//ti TypeSet
impl TypeSet {
    //cp add_type
    /// Constructor to add a new named type to the set, and indicated
    /// whether it is inherited from a parent
    pub fn add_type(mut self, s: &str, type_value: StyleTypeValue, inheritable: bool) -> Self {
        self.set.insert(
            s.to_string(),
            TypeSetValue {
                type_value,
                inheritable,
            },
        );
        self
    }

    //cp borrow_type
    /// Borrow a type from the set, if it is there, and whether it is inheritable
    pub fn borrow_type(&self, s: &str) -> Option<(&StyleTypeValue, bool)> {
        match self.set.get(s) {
            Some(value) => Some((value.type_value(), value.inheritable())),
            _ => None,
        }
    }

    //zz All done
}
