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

@file    stylesheet.rs
@brief   A stylesheet with rules, using a descriptor, and a tree that can be used for styling
 */

//a Imports
use std::collections::HashMap;
use crate::{TypeValue, ValueError, NamedTypeSet};
use crate::{RuleSet};
use crate::{Tree};
use crate::{TreeApplicator32, TreeApplicator64, TreeApplicatorX};
use super::stylable::{StylableNode, StylableNodeAction, StylableNodeRule};

//a Constants for debug
const DEBUG_STYLESHEETTREE : bool = 1 == 0;

//a Stylesheet
//tp Stylesheet
/// The Stylesheet
pub struct Stylesheet <'a, V:TypeValue> {
    style_set  : &'a NamedTypeSet<V>,
    rules      : RuleSet<StylableNode<'a, V>, StylableNodeAction<V>, StylableNodeRule>,
    style_of_id : HashMap<String, usize>,
}

impl <'a, V:TypeValue> Stylesheet<'a, V> {
    pub fn new(style_set:&'a NamedTypeSet<V>) -> Self {
        Self { style_set,
               rules : RuleSet::new(),
               style_of_id : HashMap::new(),
        }
    }
    //mp add_action
    /// Add an action to the set
    pub fn add_action(&mut self, id:Option<&str>, action:StylableNodeAction<V>) -> usize {
        if DEBUG_STYLESHEETTREE { println!("Adding action {:?}", action); }
        let index = self.rules.add_action(action);
        if let Some(s) = id {
            self.style_of_id.insert(s.to_string(), index);
        }
        if DEBUG_STYLESHEETTREE { println!("Added {} with id {:?}", index, id); }
        index
    }

    //mp add_action_from_name_values
    /// Add an action to the set from a vec of string pairs
    pub fn add_action_from_name_values(&mut self, name_values:&Vec<(String, String)>) -> Result<usize, ValueError> {
        let mut id = None;
        let mut styling = Vec::new();
        for (name, value) in name_values {
            if name == "id" {
                id = Some(value.as_str());
            } else if let Some((value_type, _)) = self.style_set.borrow_type(name) {
                let mut v = value_type.new_value();
                v.from_string(value)?;
                styling.push( (name.to_string(), v) );
            } else {
                return Err(ValueError::bad_value( &format!("unknown style name {} with value {}", name, value) ));
            }
        }
        Ok(self.add_action(id, StylableNodeAction::new(styling)))
    }

    //mp get_action_index
    pub fn get_action_index(&self, s:&str) -> Option<&usize> {
        self.style_of_id.get(s)
    }
    
    //mp add_rule
    /// Add a rule to the set, and return its handle; it may be a
    /// child of a parent rule given by a handle, or a toplevel rule.
    ///
    /// Note that the rule's children will all have a greater index
    /// than the rule. This invariant is required
    pub fn add_rule(&mut self, parent:Option<usize>, rule:StylableNodeRule, action:Option<usize>) -> usize {
        self.rules.add_rule(parent, rule, action)
    }

    //mp apply_rule
    pub fn apply_rules_to_tree(&self, tree:&mut Tree<StylableNode<'a,V>> ) {
        let num_rules = self.rules.num_rules();
        let mut iter = tree.it_create();

        if num_rules <= 32 {
            let mut applicator = TreeApplicator32::new(&self.rules);
            loop { match tree.it_next(&mut iter) {
                Some (n) => applicator.handle_tree_op(n.map(|(_,x)| tree.borrow_mut(x))),
                None => {break;},
            }}
        } else if num_rules <= 64 {
            let mut applicator = TreeApplicator64::new(&self.rules);
            loop { match tree.it_next(&mut iter) {
                Some (n) => applicator.handle_tree_op(n.map(|(_,x)| tree.borrow_mut(x))),
                None => {break;},
            }}
        } else {
            let mut applicator = TreeApplicatorX::new(&self.rules);
            loop { match tree.it_next(&mut iter) {
                Some (n) => applicator.handle_tree_op(n.map(|(_,x)| tree.borrow_mut(x))),
                None => {break;},
            }}
        }
    }
}

//tm Test code
#[cfg(test)]
mod test_stylesheet {
    use crate::{BaseValue, Descriptor, NamedTypeSet};
    use super::*;
    struct Element <'a> {
        pub stylable : StylableNode<'a, BaseValue>,
        pub children : Vec<Element<'a>>,
    }
    impl <'a> Element <'a>{
        pub fn new(stylable:StylableNode<'a, BaseValue>) -> Self {
            Self { stylable, children:Vec::new() }
        }
        pub fn add_child(&mut self, child:Element<'a>) {
            self.children.push(child);
        }
        pub fn add_to_tree<'b>(&'b mut self, mut tree:Tree<'b, StylableNode<'a,BaseValue>>) -> Tree<'b, StylableNode<'a,BaseValue>>{
            if self.children.len()>0 {
                tree.open_container(&mut self.stylable);
                for c in self.children.iter_mut() {
                    tree = c.add_to_tree(tree);
                }
            } else {
                tree.add_node(&mut self.stylable);
            }
            tree
        }
        pub fn create_tree<'b>(&'b mut self) -> Tree<'b, StylableNode<'a,BaseValue>> {
            let Self {stylable, children, ..} = self;
            let mut tree = Tree::new(stylable);
            for c in children.iter_mut() {
                tree = c.add_to_tree(tree);
            }
            tree.close_container();
            tree
        }
    }
    #[test]
    fn test_simple() {
        let style_set = NamedTypeSet::<BaseValue>::new()
            .add_type("x", BaseValue::int(None), false)
            .add_type("y", BaseValue::int(None), false)
            ;
        let mut d_pt = Descriptor::new(&style_set);
        d_pt.add_style("x");
        d_pt.add_style("y");
        let d_g  = Descriptor::new(&style_set);
        
        let mut stylesheet = Stylesheet::new(&style_set);
        let act0_nv = vec![("x".to_string(),BaseValue::int(Some(7))),];
        let act_0  = stylesheet.add_action(None, StylableNodeAction::new(act0_nv));
        let act_1  = stylesheet.add_action_from_name_values(&(vec![("x","3"),
                                                                 ("id","action_1"),
                                                                 ("y","-99"),
                                                                 ].iter().map(|(a,b)| (a.to_string(), b.to_string())).collect())).unwrap();
        let _rule_0 = stylesheet.add_rule(None, StylableNodeRule::new().has_id("pt1"), Some(act_0));
        let _rule_1 = stylesheet.add_rule(None, StylableNodeRule::new().has_id("pt0"), Some(act_1));
        
        let mut node0_0 = StylableNode::new("pt", &d_pt);
        node0_0.add_name_value("id", "pt0").unwrap();
        node0_0.add_name_value("x", "1").unwrap();
        node0_0.add_name_value("y", "0").unwrap();
        let mut node0_1 = StylableNode::new("pt", &d_pt);
        node0_1.add_name_value("id", "pt1").unwrap();
        node0_1.add_name_value("x", "2").unwrap();
        node0_1.add_name_value("y", "10").unwrap();
        let mut group0 = StylableNode::new("g", &d_g);
        group0.add_name_value("id", "group").unwrap();
        let mut group0 = Element::new(group0);
        group0.add_child(Element::new(node0_0));
        group0.add_child(Element::new(node0_1));

        {
            let mut tree = group0.create_tree();
            stylesheet.apply_rules_to_tree(&mut tree);
            for top in tree.iter_tree() {
                top.as_option().map(|(depth,n)| println!("{} {:?}",depth, n.borrow()));
            }
        }
        // FIXME make this a useful test
        // assert!(false);
    }
}
