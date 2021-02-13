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
use crate::{TypeValue, NamedTypeSet, ValueError};

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
            match self.style_set.get_type(name) {
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

    //mp build_style_array
    pub fn build_style_array(&self) -> Vec<V> {
        let mut result = Vec::new();
        for (_, v, _) in &self.styles {
            result.push(v.new_value());
        }
        result
    }

    //mp clone_style_array
    pub fn clone_style_array(&self, values:&Vec<V>) -> Vec<V> {
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

//tp StylableNode
/// A `StylableNode` is an element that is part of a hierarchy of elements, which
/// are styled by a stylesheet
/// ```
///  extern crate stylesheet;
///  use stylesheet::{TypeValue, BaseValue, NamedTypeSet, StylableNode, Descriptor};
///  let nts = NamedTypeSet::<BaseValue>::new()
///       .add_type("width",  BaseValue::int(None), true)
///       .add_type("height", BaseValue::int(None), true);
///  let mut d = Descriptor::<BaseValue>::new(&nts);
///  d.add_styles(vec!["width", "height"]);
///  let root = StylableNode::new("graph", &d); //, vec![("width","3"), ("height","1")]);
///  let child_1 = StylableNode::new("line", &d);// , vec![]);
///  let child_2 = StylableNode::new("text", &d);// , vec![]);
///  let child_11 = StylableNode::new("line", &d);// , vec![]);
///
/// ```
#[derive(Debug)]
pub struct StylableNode<'a, V:TypeValue>{
    /// The `parent` of a node is the parent in the hierarchy; this is
    /// required to provide inheritance by a child of style values
    /// from its parent
    // parent                : Option<RrcStylableNode<'a, V>>,
    /// The `children` of a node are those which have the node as a
    /// parent; this is used to propagate the stylesheet through the
    /// hierarchy.
    // children              : Vec<RrcStylableNode<'a, V>>,
    /// The descriptor provides the description of the styles required by the node
    // descriptor            : RrcDescriptor<V>,
    descriptor            : &'a Descriptor<'a, V>,
    /// id_name is a string that (should be) is unique in the hierarchy for the element,
    /// and which can be used to specify style values; it may be used in rules.
    id_name               : Option<String>,
    /// node_type is the type of the element, such as 'line' or 'circle'; it may be used in rules.
    node_type             : String,
    /// classes is an array of class names that the element belongs to, the styles of all 
    /// of which may be used to specify style values; it may be used in rules.
    classes               : Vec<String>,
    /// `extra_sids` provides values for a stylesheet that do *not*
    /// belong to the node, but may be inherited by children of the
    /// node
    extra_sids            : Vec<(String, V)>,
    /// `values` contains the nodes values for each of the styles in the descriptor; it is in 1-to-1 correspondence with descriptor.styles + extra_sids
    /// `values` is supposed to be a set of ValueRefs
    values                : Vec<V>,
    /// state is a vector the same length as the descriptor.state_classes
    /// possibly the state is animatable state - i.e. 'is this thing covered by the mouse'
    /// this has a 1-to-1 correspondence with descriptor.state_classes
    state                 : Vec<isize>,
    // style_change_callback : t_style_change_callback,
}

// pub type NameValues<'a> = Vec<(&'a str, &'a str)>;
pub type RrcStylableNode<'a, V> = Rc<RefCell<StylableNode<'a, V>>>;
impl <'a, V:TypeValue> StylableNode<'a, V> {
    //fp new
    /// Create a new stylable node with a given node descriptor and a set of name/value pairs that set the values to be non-default
    /// any name_values that are not specific to the node descriptor, but that are permitted by the stylesheet, are added as 'extra_value's
    /// to the node
    ///
    /// The name of 'id' is special; it defines the (document-unique) id of the node
    /// The name of 'class' is special; it provides a list of whitespace-separated class names that the node belongs to
    pub fn new(node_type:&str, descriptor:&'a Descriptor<V>) -> Self {
        // parent:Option<RrcStylableNode<'b, V>>,
        // let parent_clone = match parent { None => None, Some(ref p)=> Some(p.clone()) };
        // parent_clone.map(|p| p.borrow_mut().children.push(node.clone()));
        // parent,
        // children : Vec::new(),
        // let descriptor = descriptor.clone();
        let extra_sids = Vec::new();
        let classes    = Vec::new();
        let values     = descriptor.build_style_array();
        let id_name    = None;
        Self {
            descriptor,
            extra_sids,
            values,
            id_name,
            state:       Vec::new(),
            node_type:   node_type.to_string(),
            classes,
        }
    }

    //fp clone
    pub fn clone(&self, id_name:&str) -> Self {
        let extra_sids = self.extra_sids.iter().map(|(s,v)| (s.clone(),v.clone())).collect();
        let classes    = self.classes.iter().map(|s| s.clone()).collect();
        let values     = self.descriptor.clone_style_array(&self.values);
        let id_name    = Some(id_name.to_string());
        let node = Self {
            descriptor: self.descriptor,
            extra_sids,
            values,
            id_name,
            state:       Vec::new(),
            node_type:   self.node_type.clone(),
            classes,
        };
        node
    }

    //mp borrow_id
    pub fn borrow_id(&self) -> Option<&str> {
        match &self.id_name {
            None => None,
            Some(s) => Some(s)
        }
    }
    
    //mp add_name_value
    pub fn add_name_value(&mut self, name:&str, value:&str) -> Result<(),ValueError> {
        if name=="id" {
            self.id_name = Some(value.to_string());
            Ok(())
        } else if name=="class" {
            for s in value.split_whitespace() {
                self.classes.push(s.to_string());
            }
            Ok(())
        } else if let Some(n) = self.descriptor.find_style_index(name) {
            self.values[n].from_string(value)?;
            Ok(())
        } else if let Some((v, _inheritable)) = self.descriptor.style_set.get_type(name) {
            let mut v = v.new_value();
            v.from_string(value)?;
            self.extra_sids.push((name.to_string(),v));
            Ok(())
        } else {
            Err(ValueError::bad_value(&format!("name '{}' has no known value type in style set",name)))
        }
    }

    //mp find_style_index -- was find_sid_index(_exn)
    pub fn find_style_index(&self, s:&str) -> Option<usize> {
        // println!("Find style index {} {}",s,self.values.len());
        match self.descriptor.find_style_index(s) {
            Some(n) => Some(n),
            None => {
                let mut n = self.descriptor.styles.len();
                for (sn, _) in &self.extra_sids {
                    if sn==s { return Some(n); }
                    n += 1
                }
                None
            },
        }
    }

    //mp override_values
    pub fn override_values(&mut self, other:&Self) {
        for c in &other.classes {
            let mut found = false;
            for s in &self.classes {
                if s == c { found = true; }
            }
            if !found {
                self.classes.push(c.clone());
            }
        }
        for (name, value) in &other.extra_sids {
            if let Some(n) = self.find_style_index(name) {
                self.values[n] = value.clone();
            } else {
                self.extra_sids.push( (name.clone(), value.clone()) );
            }
        }
    }

    //mp has_id
    pub fn has_id(&self, s:&str) -> bool {
        match &self.id_name { Some(id) => s == id, None => false }
    }

    //mp has_class
    pub fn has_classs(&self, s:&str) -> bool {
        for c in &self.classes { if c==s {return true;} }
        false
    }

    /*
    //fp delete_children
    /// ```
    ///  extern crate stylesheet;
    ///  use std::rc::Rc;
    ///  use stylesheet::{TypeValue, BaseValue, NamedTypeSet, StylableNode, Descriptor};
    ///  let nts = NamedTypeSet::<BaseValue>::new()
    ///       .add_type("width",  BaseValue::int(None), true)
    ///       .add_type("height", BaseValue::int(None), true);
    ///  let d = Descriptor::<BaseValue>::new();
    ///  d.borrow_mut().add_styles(&nts, vec!["width", "height"]);
    ///  let root    = StylableNode::new(None,               "graph", &d, vec![]);
    ///  let child_1 = StylableNode::new(Some(root.clone()), "line",  &d, vec![]);
    ///  assert_eq!(2, Rc::strong_count(&child_1));
    ///  assert_eq!(2, Rc::strong_count(&root));
    ///  root.borrow_mut().delete_children();
    ///  assert_eq!(1, Rc::strong_count(&root));    // only root
    ///  assert_eq!(1, Rc::strong_count(&child_1)); // only child_1
    ///
    /// ```
    
    pub fn delete_children(&mut self) -> () {
        while self.children.len()>0 {
            let c = self.children.pop().unwrap();
            c.borrow_mut().parent = None;
            c.borrow_mut().delete_children();
        }
    }
     */

    //mp get_style_value_of_name
    pub fn get_style_value_of_name(&self, s:&str) -> Option<&V> {
        match self.find_style_index(s) {
            None => None,
            Some(n) => {
                let nv = self.values.len();
                if n < nv {
                    Some(&self.values[n])
                } else {
                    Some(&self.extra_sids[n-nv].1)
                }
            },
        }
    }

    /*
    //mp get_value_ref
    pub fn get_value_ref(&self, sheet, s:&str) ->  {
        let sid = style_id_of_name_exn s sheet in
            let sindex = find_sid_index_exn sid t in
            t.values.(sindex)
    }

    (*f get_value *)
        let get_value t sheet (s:string) =
        Value_ref.get_value (get_value_ref t sheet s)
*/

    //zz All done
}
/*
  in
  add_styleable t sheet;
  let set_default_value nv =
    let (name,value) = nv in
    match style_id_of_name name sheet with
      None -> ()
    | Some sid -> (
      match find_sid_index sid t with
        None -> ()
      | Some sid_index -> (
        let stype = Style_id.get_type sid in
        (*Printf.printf "Set default value of %s.%s.%s to be %s\n" t.id_name t.type_name name value;*)
        Value_ref.set_default_from_string t.values.(sid_index) stype value
      )
    )
  in
  List.iter set_default_value name_values;
  let set_inheritance n vr =
    let sid = get_nth_sid n t in
    let di = (is_default_inherit sid sheet) in
    Value_ref.set_default_inherit vr di
  in
  Array.iteri set_inheritance t.values;
  t
}
        
*/
