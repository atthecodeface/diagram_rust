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
use crate::{TypeValue, BaseValue, ValueError, Descriptor, NamedTypeSet, StylableNode};


//a Constants for debug
const DEBUG_TREE_ITERATOR : bool = 1 == 0;

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

// never cloned, only moved
//a Stylable Tree
//a SvgElement iterator
//tp StylableTreeIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct StylableTreeIter<'a, 'b, V:TypeValue> {
    tree: &'a StylableTree<'b, 'b, V>,
    root_index : usize,
    stack : Vec<(&'a StylableTreeNode<'b, 'b, V>, usize)>,
}

//ip StylableTreeIter
impl <'a, 'b, V:TypeValue> StylableTreeIter<'a, 'b, V> {
    //fp new
    /// Create a new iterator
    pub fn new(tree:&'a StylableTree<'b, 'b, V>) -> Self {
        Self { tree,
               root_index: 0,
               stack:Vec::new(),
        }
    }
}

//ip Iterator for StylableTreeIter
impl <'a, 'b, V:TypeValue> Iterator for StylableTreeIter<'a, 'b, V> {
    type Item = &'a StylableTreeNode<'b, 'b, V>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            if self.root_index >= self.tree.roots.len() {
                None
            } else {
                let n = self.root_index;
                self.root_index += 1;
                self.stack.push((&self.tree.roots[n], 0));
                Some(&self.tree.roots[n])
            }
        } else {
            let (tn,child_index) = self.stack.pop().unwrap();
            if child_index >= tn.children.len() {
                self.next()
            } else {
                self.stack.push((tn, child_index+1));
                self.stack.push((&tn.children[child_index], 0));
                Some(&tn.children[child_index])
            }
        }
    }
}

//tp StylableTreeNode
pub struct StylableTreeNode<'a, 'b, V:TypeValue> {
    node : &'b StylableNode<'a, V>,
    children : Vec<StylableTreeNode<'a, 'b, V>>,
}
impl <'a, 'b, V:TypeValue> StylableTreeNode<'a, 'b, V> {
    pub fn new(node:&'b StylableNode<'a, V>) -> Self {
        Self { node, children:Vec::new() }
    }
    pub fn add_child(&mut self, child:Self) {
        self.children.push(child);
    }
}

//tp StylableTree
pub struct StylableTree<'a, 'b, V:TypeValue> {
    stylesheet : &'b Stylesheet<'a, V>,
    roots : Vec<StylableTreeNode<'a, 'b, V>>,
    stack : Vec<StylableTreeNode<'a, 'b, V>>,
}

//ip StylableTree
impl <'a, 'b, V:TypeValue> StylableTree<'a, 'b, V> {
    //fp new
    pub fn new(stylesheet:&'b Stylesheet<'a, V>) -> Self {
        Self { stylesheet,
               roots : Vec::new(),
               stack : Vec::new(),
        }
    }
    //fi push_node
    fn push_node(&mut self, node:&'a StylableNode<V>) {
        let tree_node = StylableTreeNode::new(node);
        self.stack.push(tree_node);
    }
    //fi pop_node
    fn pop_node(&mut self) {
        assert!(self.stack.len() > 0);
        let tree_node = self.stack.pop().unwrap(); // no longer mutatable
        self.add_tree_node(tree_node);
    }
    //fi add_tree_node
    fn add_tree_node(&mut self, tree_node:StylableTreeNode<'a, 'b, V>) {
        if self.stack.len() == 0 {
            self.roots.push(tree_node);
        } else {
            let mut parent = self.stack.pop().unwrap();
            parent.add_child(tree_node);
            self.stack.push(parent);
        }
    }
    //fi open_container
    pub fn open_container(&mut self, node:&'a StylableNode<V>) {
        self.push_node(node);
    }
    //fi add_node
    pub fn add_node(&mut self, node:&'a StylableNode<V>) {
        let tree_node = StylableTreeNode::new(node);
        self.add_tree_node(tree_node);
    }
    //fi close_container
    pub fn close_container(&mut self) {
        self.pop_node();
    }
    //fi iter_tree
    pub fn iter_tree(&self) -> StylableTreeIter<V> {
        StylableTreeIter::new(self)
    }
}

//tm Test code
#[cfg(test)]
mod test_stylesheet {
    use super::*;
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
        node0_0.add_name_value("x", "1");
        node0_0.add_name_value("y", "0");
        let mut node0_1 = StylableNode::new("pt", &d_pt);
        node0_1.add_name_value("x", "2");
        node0_1.add_name_value("y", "10");
        let mut group0 = StylableNode::new("g", &d_g);

        {
            let mut tree = StylableTree::new(&stylesheet);
            tree.open_container(&group0);
            tree.add_node(&node0_0);
            tree.add_node(&node0_1);
            tree.close_container();
            for n in tree.iter_tree() {
                println!("{:?} {:?} {:?}",n.node.id_name, n.node.node_type, n.node.values );
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
