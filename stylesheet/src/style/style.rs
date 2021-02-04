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

@file    style.rs
@brief   Stylable values
 */

//a Imports
use std::collections::HashMap;
use super::value::{StylableValue, StylableType};

//tp StyleTypeInstance
/// A `StyleTypeInstance` is used for everything that may belong to a style; it
/// has an ID, and a type (such as rgb or int etc)
pub struct StyleTypeInstance {// from types    
    name  : String,
    stype : StylableType,
}

impl StyleTypeInstance {
    pub fn new(name:String, stype:StylableType) -> Self {
        Self { name, stype }
    }
    pub fn get_type(&self) -> StylableType {
        self.stype
    }
}

//ti std::fmt::Display for StyleTypeInstance
impl std::fmt::Display for StyleTypeInstance {
    //mp fmt - format for display
    /// Display the style id
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.stype)
    }

    //zz All done
}

//tp Style
/// A `Style` is a collection of `StyleTypeInstance`s, such as for a line the style might be color and width
/// Each element in the style can be optional - if it is optional, then a default value is used?
pub struct Style {
    styles : Vec<(StyleTypeInstance, (StylableValue, bool))>
}

impl Style { // from style.ml
    pub fn create() -> Self {
        Self { styles:Vec::new() }
    }
    pub fn add_styling(&mut self, sid:StyleTypeInstance, value:StylableValue, opt:bool) -> () {
        self.styles.push( (sid,(value,opt)) );
    }

/*    pub fn str() -> () {
        let str_svo acc svo =
            let (sid,(svalue,opt)) = svo in
            Printf.sprintf "%s%s:%s:%b\n" acc (Style_id.str sid) (Value.str svalue) opt
            in
            List.fold_left str_svo "style:\n" t.styles
    }
*/

    pub fn get_value(&self, sid:&StyleTypeInstance) -> Option<StylableValue> {
        None
    }
    pub fn get_opt(&self, sid:&StyleTypeInstance) -> Option<bool> {
        None
    }
    //zz All done
}

//a StyleTypeSet errors
//tp StyleTypeInstanceError
/// `StyleTypeInstanceError` represents an error from the style sheet
#[derive(Debug)]
pub enum StyleTypeInstanceError<'a> {
    /// Failure to find an ID
    FailedToFindId(&'a String),
    /// Duplicate ID
    DuplicateId,
}

//ip std::fmt::Display for StyleTypeInstanceError
impl <'a> std::fmt::Display for StyleTypeInstanceError<'a> {
    //mp fmt - format a `StyleTypeInstanceError` for display
    /// Display the `StyleTypeInstanceError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FailedToFindId(s) => write!(f, "failed to find id {}", s),
            Self::DuplicateId       => write!(f, "duplicate id"),
        }
    }
}

//tp StyleTypeSet
/// A `StyleTypeSet` is a collection of StyleTypeInstance's with unique names
pub struct StyleTypeSet { // from types - was t_style_ids
    set : HashMap<String , StyleTypeInstance>,
}

// Style and style change callback
// pub type StyleChangeCallback = FnMut Vec<(StyleTypeInstance, StylableValue)> -> ()

//tp StyleTypeSet
// immutable, but contains a hash table of Style_id_hash.t -> Style_id.t *)
impl StyleTypeSet {
    pub fn new() -> Self {
        Self { set:HashMap::new() }
    }

    pub fn find_id<'a>(&'a self, id:&String) -> Option<&'a StyleTypeInstance> {
        self.set.get(id)
    }

    pub fn find_id_err<'a>(&'a self, id:&'a String) -> Result<&'a StyleTypeInstance, StyleTypeInstanceError<'a>> {
        self.set.get(id).ok_or_else(|| StyleTypeInstanceError::FailedToFindId(id))
    }

    pub fn add_id_err<'a>(&'a mut self, id:String, s:StyleTypeInstance) -> Result<(), StyleTypeInstanceError<'a>>{
        match self.set.insert(id, s) {
            None => Ok(()),
            Some(_) =>  Err(StyleTypeInstanceError::DuplicateId),
        }
    }

    pub fn add_id(&mut self, id:String, s:StyleTypeInstance) -> () {
        self.set.insert(id, s);
    }
}
/*
  let build_id_value_list nvs t = 
    let rec add_id_value acc (name,x) = 
      let hash = Style_id.hash_of_string name in
      let opt_sid = find_opt_id hash t in
      match opt_sid with
        None -> raise (Unknown_id name)
      | Some sid -> (sid,x)::acc
    in
    List.fold_left add_id_value [] nvs
end
 */

