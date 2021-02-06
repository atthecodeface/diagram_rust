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
use crate::TypeValue;
use crate::NamedTypeSet;

//tp Descriptor
/// A `Descriptor` is used to describe the values that a particular node type may have in a hierarchy of nodes.
pub struct Descriptor<V:TypeValue> {
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
impl <V:TypeValue> Descriptor<V> {
    //fp new
    pub fn new() -> Self {
        Self { state_classes:Vec::new(), styles:Vec::new() }
    }

    //cp add_style
    pub fn add_style(mut self, nts:&NamedTypeSet<V>, name:&str ) -> Self {
        println!("Get style {} from {}",name,nts);
        let (value, inheritable) = nts.get_type(name).unwrap();
        self.styles.push( (name.to_string(), value.as_type(), inheritable) );
        self
    }

    //mp build_style_array
    pub fn build_style_array(&self) -> Vec<V> {
        let mut result = Vec::new();
        for (_, v, _) in &self.styles {
            result.push(v.new_value());
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
///  let d = Descriptor::<BaseValue>::new().add_style(&nts, "width").add_style(&nts, "height");
///  let root = StylableNode::new(None, "graph", &d, vec![("width","3"), ("height","1")]);
///  let child_1 = StylableNode::new(Some(root.clone()),     "line", &d, vec![]);
///  let child_2 = StylableNode::new(Some(root.clone()),     "text", &d, vec![]);
///  let child_11 = StylableNode::new(Some(child_1.clone()), "line", &d, vec![]);
///
/// ```
pub struct StylableNode<'a, V:TypeValue>{
    /// The `parent` of a node is the parent in the hierarchy; this is
    /// required to provide inheritance by a child of style values
    /// from its parent
    parent                : Option<RrcStylableNode<'a, V>>,
    /// The `children` of a node are those which have the node as a
    /// parent; this is used to propagate the stylesheet through the
    /// hierarchy.
    children              : Vec<RrcStylableNode<'a, V>>,
    /// The descriptor provides the description of the styles required by the node
    descriptor            : &'a Descriptor<V>,
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

pub type NameValues<'a> = Vec<(&'a str, &'a str)>;
pub type RrcStylableNode<'a, V> = Rc<RefCell<StylableNode<'a, V>>>;
impl <'a, V:TypeValue> StylableNode<'a, V> {
    //fp new
    /// Create a new stylable node with a given node descriptor and a set of name/value pairs that set the values to be non-default
    /// any name_values that are not specific to the node descriptor, but that are permitted by the stylesheet, are added as 'extra_value's
    /// to the node
    ///
    /// The name of 'id' is special; it defines the (document-unique) id of the node
    /// The name of 'class' is special; it provides a list of whitespace-separated class names that the node belongs to
    pub fn new <'b>(parent:Option<RrcStylableNode<'b, V>>, node_type:&str, descriptor:&'b Descriptor<V>, name_values:NameValues) -> RrcStylableNode<'b, V> {
        let extra_sids = Vec::new();
        let mut classes    = Vec::new();
        let mut values     = descriptor.build_style_array();
        let mut id_name    = None;
        for (name, value) in name_values {
            if name=="id" {
                id_name = Some(value.to_string());
            } else if name=="class" {
                for s in value.split_whitespace() {
                    classes.push(s.to_string());
                }
            } else {
                match descriptor.find_style_index(name) {
                    Some(n) => {
                        values[n].from_string(value).unwrap();
                    },
                    None => {
                        // let v = stylesheet.get_style_value(name);
                        // let v = v.new_value().from_string(value).unwrap();
                        // extra_sids.push(name);
                        // values.push(v)
                    }
                }
            }
        }
        let parent_clone = match parent { None => None, Some(ref p)=> Some(p.clone()) };
        let node = Rc::new(RefCell::new(StylableNode {
            parent,
            children : Vec::new(),
            descriptor,
            extra_sids,
            values,
            state:       Vec::new(),
            id_name:     id_name,
            node_type:   node_type.to_string(),
            classes,
        }));
        parent_clone.map(|p| p.borrow_mut().children.push(node.clone()));
        node
    }

    //fp delete_children
    /// ```
    ///  extern crate stylesheet;
    ///  use std::rc::Rc;
    ///  use stylesheet::{TypeValue, BaseValue, NamedTypeSet, StylableNode, Descriptor};
    ///  let nts = NamedTypeSet::<BaseValue>::new()
    ///       .add_type("width",  BaseValue::int(None), true)
    ///       .add_type("height", BaseValue::int(None), true);
    ///  let d = Descriptor::<BaseValue>::new().add_style(&nts, "width").add_style(&nts, "height");
    ///  let root = StylableNode::new(None,                  "graph",  &d, vec![]);
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

    //mp find_style_index -- was find_sid_index(_exn)
    pub fn find_style_index(&self, s:&str) -> Option<usize> {
        match self.descriptor.find_style_index(s) {
            Some(n) => Some(n),
            None => {
                let mut n=self.descriptor.styles.len();
                for (sn, _) in &self.extra_sids {
                    if sn==s { return Some(n); }
                    n += 1
                }
                None
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
