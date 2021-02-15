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
use crate::{TypeValue, ValueError, Descriptor};

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
    pub(crate) id_name               : Option<String>,
    /// node_type is the type of the element, such as 'line' or 'circle'; it may be used in rules.
    pub(crate) node_type             : String,
    /// classes is an array of class names that the element belongs to, the styles of all 
    /// of which may be used to specify style values; it may be used in rules.
    pub(crate) classes               : Vec<String>,
    /// `extra_sids` provides values for a stylesheet that do *not*
    /// belong to the node, but may be inherited by children of the
    /// node
    pub(crate) extra_sids            : Vec<(String, V)>,
    /// `values` contains the nodes values for each of the styles in
    /// the descriptor; it contains bool and value, the bool
    /// indicating whether it is set by the node or not
    pub(crate) values                : Vec<(bool, V)>,
    /// state is a vector the same length as the descriptor.state_classes
    /// possibly the state is animatable state - i.e. 'is this thing covered by the mouse'
    /// this has a 1-to-1 correspondence with descriptor.state_classes
    pub(crate) state                 : Vec<isize>,
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
        let values     = descriptor.build_style_value_array();
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
        let values     = self.descriptor.clone_style_value_array(&self.values);
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
            self.values[n].1.from_string(value)?;
            self.values[n].0 = true;
            Ok(())
        } else if let Some((v, _inheritable)) = self.descriptor.style_set.borrow_type(name) {
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
                self.values[n].1 = value.clone();
                self.values[n].0 = true;
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
                    Some(&self.values[n].1)
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
