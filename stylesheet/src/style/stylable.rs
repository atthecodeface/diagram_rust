use std::cell::RefCell;
use std::rc::Rc;
use super::value::StyleValue;
use super::style::{StyleTypeInstance};

//tp StylableDescriptor
/// A `StylableDescriptor` is used to describe the values that a particular node type may have in a hierarchy of nodes.
pub struct StylableDescriptor {
    /// `states` has one entry for each class of state, and each entry is a vector of <name>:<value>
    /// An example of one state class would be for a GUI 'button', with the options being 'enabled', 'disabled', and 'active'
    pub state_classes : Vec<(String,  Vec<(String,isize)>)>,
    /// Vec of all stylenames the stylable cares about; this is normally known at compile time
    pub styles : Vec<(String, StyleValue /* as type and default value */, bool /* inheritable? */)>,
}

//tp StylableNode
/// A `StylableNode` is an element that is part of a hierarchy of elements, which
/// are styled by a stylesheet
pub struct StylableNode<'a>{
    /// The `parent` of a node is the parent in the hierarchy; this is
    /// required to provide inheritance by a child of style values
    /// from its parent
    parent                : Option<RrcStylableNode<'a>>,
    /// The `children` of a node are those which have the node as a
    /// parent; this is used to propagate the stylesheet through the
    /// hierarchy.
    children              : Vec<RrcStylableNode<'a>>,
    /// The descriptor provides the description of the styles required by the node
    descriptor            : &'a StylableDescriptor,
    /// id_name is a string that (should be) is unique in the hierarchy for the element,
    /// and which can be used to specify style values; it may be used in rules.
    id_name               : Option<String>,
    /// type_name is the type of the element, such as 'line' or 'circle'; it may be used in rules.
    type_name             : String,
    /// classes is an array of class names that the element belongs to, the styles of all 
    /// of which may be used to specify style values; it may be used in rules.
    classes               : Vec<String>,
    /// `extra_sids` provides values for a stylesheet that do *not*
    /// belong to the node, but may be inherited by children of the
    /// node
    extra_sids            : Vec<(String, StyleValue)>,
    /// `values` contains the nodes values for each of the styles in the descriptor; it is in 1-to-1 correspondence with descriptor.styles
    values                : Vec<StyleValue>,
    /// state is a vector the same length as the descriptor.state_classes
    /// possibly the state is animatable state - i.e. 'is this thing covered by the mouse'
    /// this has a 1-to-1 correspondence with descriptor.state_classes
    state                 : Vec<isize>,
    // style_change_callback : t_style_change_callback,
}

pub type NameValues = Vec<(String,String)>;
pub type RrcStylableNode<'a> = Rc<RefCell<StylableNode<'a>>>;
impl <'a> StylableNode<'a> {
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::{StylableNode, StylableDescriptor};
    ///  let d = StylableDescriptor { state_classes:Vec::new(), styles:Vec::new(), };
    ///  let name_values = Vec::new();
    ///  let root = StylableNode::new(None, &d, name_values);
    ///  let name_values = Vec::new();
    ///  let child_1 = StylableNode::new(Some(root.clone()),     &d, name_values);
    ///  let name_values = Vec::new();
    ///  let child_2 = StylableNode::new(Some(root.clone()),     &d, name_values);
    ///  let name_values = Vec::new();
    ///  let child_11 = StylableNode::new(Some(child_1.clone()), &d, name_values);
    ///
    /// ```
    ///
    pub fn new <'b>(parent:Option<RrcStylableNode<'b>>, descriptor:&'b StylableDescriptor, mut name_values:NameValues) -> RrcStylableNode<'b> {
        let mut extra_sids = Vec::new();
        let mut classes    = Vec::new();
        let mut id_name    = None;
        while name_values.len()>0 {
            let (name,value) = name_values.pop().unwrap();
            if name=="id" {
                id_name = Some(value);
            } else if name=="class" {
                for s in value.split_whitespace() {
                    classes.push(s.to_string());
                }
            } // else if it is in the desc_built then add it to extra_sids
        }
        Rc::new(RefCell::new(StylableNode {
            parent,
            children : Vec::new(),
            descriptor,
            extra_sids,
            values:      Vec::new(),
            state:       Vec::new(),
            id_name:     id_name,
            type_name : "".to_string(),
            classes,
        }))
    }
}
/*
  // let desc_built = build_desc desc sheet in
  let id_name = 
    if (List.mem_assoc "id" name_values) then (List.assoc "id" name_values) else "no_id"
  in
  let classes = 
    let class_str = if (List.mem_assoc "class" name_values) then (List.assoc "class" name_values) else "" in
    let class_list = String.split_on_char ' ' class_str in
    List.filter (fun x->(x<>"")) class_list
  in
  let count_extra_styles acc nv =
    let (name,_) = nv in
    match style_id_of_name name sheet with
      None -> acc
    | Some sid -> (
      match Styleable_desc_built.find_sid_index sid desc_built with
        None -> (acc+1)
      | Some sid_index -> acc
    )
  in
  let num_extra_styles = List.fold_left count_extra_styles 0 name_values in
  let num_base_styles = (Array.length desc_built.sids) in
  let num_styles = (num_base_styles+num_extra_styles) in
  let t = {
      desc_built;
      num_base_styles;
      num_styles;
      children;
      style_change_callback;
      id_name;
      parent = None;
      type_name;
      classes;
      extra_sids = Array.make num_extra_styles Style_id.dummy;
      state      = Array.make (List.length desc.state_descriptor) 0;
      values     = Array.init num_styles (fun i -> Value_ref.create ());
    }
  in
  add_styleable t sheet;
  let add_extra_style acc nv =
    let (name,_) = nv in
    match style_id_of_name name sheet with
      None -> acc
    | Some sid -> (
      match Styleable_desc_built.find_sid_index sid t.desc_built with
        Some sid_index -> acc
      | None -> (t.extra_sids.(acc) <- sid; acc+1)
    )
  in
  ignore (List.fold_left add_extra_style 0 name_values);
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
