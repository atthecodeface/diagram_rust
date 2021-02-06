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

@file    value.rs
@brief   The basics of a style value/type
 */

//a Imports

//a Value error
//tp ValueError
#[derive(std::fmt::Debug)]
pub enum ValueError {
    BadValue(String),
}

//ti ValueError
impl ValueError {
    pub fn bad_value(s:&str) -> Self {
        Self::BadValue(s.to_string())
    }
}

//ti Display for ValueError
//ip std::fmt::Display for ValueError
impl std::fmt::Display for ValueError {
    //mp fmt - format a ValueError for display
    /// Display the ValueError
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BadValue(s) => write!(f, "Bad value '{}'", s)
        }
    }
}


//a StyleValue trait
//tp StyleValue
/// The `StyleValue` trait is used in descriptors of stylesheets to define the
/// styles that are expected within the stylesheet. They are expected to have
/// a concept of 'no value', which is used also as the *type* of the value
///
/// The 'new_value' method creates a new value from what should be a
/// 'no value' of a specific type; the 'as_type' method operates in
/// the other direction, creating a new value from an actual (possibly
/// unset, or 'no value') value.
///
pub trait StyleValue : std::fmt::Display+std::fmt::Debug+Clone+PartialEq {
    fn new_value(&self) -> Self;
    fn as_type(&self) -> Self;
    //mp from_string
    /// Set the value from a string
    fn from_string<'a>(&'a mut self, s:&str) -> Result<&'a mut Self,ValueError>;
    
}

