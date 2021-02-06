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
@brief   A set of name -> (stype:TypeValue, inheritable:bool)  instances
 */

//a Imports
use std::collections::HashMap;
use crate::TypeValue;

//tp NamedTypeSet
pub struct NamedTypeSet<V:TypeValue> {
    set:HashMap<String,(V,bool)>
}

//ti std::fmt::Display for NamedTypeSet
impl <V:TypeValue> std::fmt::Display for NamedTypeSet< V> {
    //mp fmt - format for display
    /// Display the style id
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name,(value,inheritable)) in self.set.iter() {
            write!(f, "{} => {}:{}", name, inheritable, value.as_type())?;
        }
        Ok(())
    }
    
    //zz All done
}

//ti NamedTypeSet
impl < V:TypeValue> NamedTypeSet< V> {
    pub fn add_type(&mut self, s:&str, value:V, inheritable:bool) -> () {
        self.set.insert(s.to_string(), (value, inheritable));
    }
    
    pub fn get_type(&self, s:&str) -> Option<(V, bool)> {
        None
    }

    //zz All done
}

