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

@file    stylable.rs
@brief   A stylable node and its descriptor
 */

//a Imports
use std::cell::RefCell;
use std::rc::Rc;
use crate::{TypeValue, NamedTypeSet};

//tp Descriptor
/// A `Descriptor` is used to describe the values that a particular node type may have in a hierarchy of nodes.
#[derive(Debug)]
pub struct Descriptor<'a, V:TypeValue> {
    pub style_set: &'a NamedTypeSet<V>,
    /// `states` has one entry for each class of state, and each entry is a vector of <name>:<value>
    /// An example of one state class would be for a GUI 'button', with the options being 'enabled', 'disabled', and 'active'
    pub state_classes : Vec<(String,  Vec<(String,isize)>)>,
    /// Vec of all stylenames the stylable cares about; this is normally known at compile time
    pub styles : Vec<(String, V /* as type and default value */, bool /* is inheritable by default? */)>,
}

/* styles was this:
styles:
// HashMap name:str => (value, bool)
fn add_styling(name, value, bool)
fn get_default_value === fn get_value(name) => value
fn is_default_inherit === fn get_opt(name) => bool

styled_ids: - used in a stylesheet as its 'ids'
style_set: HashMap name:str => StyleType
fn style_id_of_name(name) -> Option<(name,StyleType)>
fn style_id_of_name_exn(name) -> Result<(name,StyleType)>

Stylesheet:
    ids                   : style_set
    default_style         : map of (name,StyleType) => (value, default of inheritable)
    mutable rules         : t_style_rule list;
    mutable built_descs   : Vec<Descriptors>

plus
    mutable entity_list   : Vec<RrcStylableNode>
    mutable roots         : Vec<RrcStylableNode>
  }
pub fn add_styleable(&mut self, s:StylableNode) -> () { self.entity_list.push(s) }
pub fn add_style_default(&mut self, ntvi:(name,type,value,default_inheritable)) -> () {
 self.ids.add(name,type);
 self.default_style.add((name,type), (value,default_inheritable)
}
pub fn add_style_defaults(&mut, ...) adds vec

*f build_desc *)
let build_desc desc t =
  if (not (List.mem_assoc desc t.built_descs)) then
    (t.built_descs <- (desc,Styleable_desc_built.create desc t.ids)::t.built_descs);
  List.assoc desc t.built_descs

*/

//ti Descriptor
pub type RrcDescriptor<'a, V> = Rc<RefCell<Descriptor<'a, V>>>;
impl <'a, V:TypeValue> Descriptor<'a, V> {
    //fp new
    pub fn new(style_set:&'a NamedTypeSet<V>) -> Self {
        Self { style_set, state_classes:Vec::new(), styles:Vec::new() }
    }

    //fp new_rrc
    /*
    pub fn new_rrc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { state_classes:Vec::new(), styles:Vec::new() }))
    }
     */

    //cp add_style
    pub fn add_style(&mut self, name:&str ) -> () {
        let (value, inheritable) = {
            match self.style_set.borrow_type(name) {
                None     => {
                    panic!("Failed to add style {} as it is not in NamedTypeSet  {}\n",name,self.style_set);
                },
                Some(vi) => vi,
            }
        };
        self.styles.push( (name.to_string(), value.as_type(), inheritable) );
    }

    //mp add_styles
    pub fn add_styles(&mut self, names:Vec<&str> ) -> () {
        for name in names {
            self.add_style(name);
        }
    }

    //mp build_style_value_array
    pub fn build_style_value_array(&self) -> Vec<(bool, V)> {
        let mut result = Vec::new();
        for (_, v, _) in &self.styles {
            result.push((false, v.new_value()));
        }
        result
    }

    //mp clone_style_value_array
    pub fn clone_style_value_array(&self, values:&Vec<(bool, V)>) -> Vec<(bool, V)> {
        let mut result = Vec::new();
        for v in values.iter() {
            result.push(v.clone());
        }
        result
    }

    //mp find_style_index -- was find_sid_index(_exn)
    pub fn find_style_index(&self, s:&str) -> Option<usize> {
        let mut n=0;
        for (sn, _, _) in &self.styles {
            if sn==s { return Some(n); }
            n += 1
        }
        None
    }

    //zz All done
}

