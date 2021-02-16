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
use std::cell::RefCell;
use std::rc::Rc;
use crate::{TypeValue, BaseValue, ValueError, Descriptor, NamedTypeSet};
use super::stylable::{StylableNode, StylableNodeAction, StylableNodeRule};
use crate::{BitMask, BitMaskU32};
use crate::{RuleResult, RuleFn, Action, RuleSet};
use crate::{Tree, TreeIterOp};
use crate::{TreeApplicator32};

//a Constants for debug
const DEBUG_STYLESHEETTREE_ITERATOR : bool = 1 == 0;

//a Stylesheet
//tp Stylesheet
/// The Stylesheet
pub struct Stylesheet <'a, V:TypeValue> {
    descriptor : &'a NamedTypeSet<V>,
//    rules : 
}

impl <'a, V:TypeValue> Stylesheet<'a, V> {
    pub fn new(descriptor:&'a NamedTypeSet<V>) -> Self {
        Self { descriptor
        }
    }
}

//tm Test code
#[cfg(test)]
mod test_stylesheet {
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
            for c in self.children.iter_mut() {
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
        let stylesheet = Stylesheet::new(&style_set);
        let mut node0_0 = StylableNode::new("pt", &d_pt);
        node0_0.add_name_value("id", "pt0");
        node0_0.add_name_value("x", "1");
        node0_0.add_name_value("y", "0");
        let mut node0_1 = StylableNode::new("pt", &d_pt);
        node0_1.add_name_value("id", "pt1");
        node0_1.add_name_value("x", "2");
        node0_1.add_name_value("y", "10");
        let mut group0 = StylableNode::new("g", &d_g);
        group0.add_name_value("id", "group");
        let mut group0 = Element::new(group0);
        group0.add_child(Element::new(node0_0));
        group0.add_child(Element::new(node0_1));

        let mut rules = RuleSet::new();
        let act0_nv = vec![("x",BaseValue::int(Some(7))),];
        let act_0 = rules.add_action(StylableNodeAction::new(&act0_nv));
        let rule_0 = rules.add_rule(None, StylableNodeRule::new().has_id("pt1"), Some(act_0));

        {
            let mut tree = group0.create_tree();
            let mut applicator = TreeApplicator32::new(&rules);
            let mut iter = tree.it_create();
            loop {
                match tree.it_next(&mut iter) {
                    Some (n) => applicator.handle_tree_op(n.map(|(_,x)| tree.borrow_mut(x))),
                    None => {break;},
                }
            }
            for top in tree.iter_tree() {
                top.as_option().map(|(depth,n)| println!("{} {:?}",depth, n.borrow()));
            }
        }
        assert!(false);
    }
}
/*
(*f create *)
let create () = {
    entity_list = [];
    roots = [];
    ids = Style_ids.create ();
    default_style = Style.create [];
    rules = [];
    built_descs = [];
  }
 */
