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

//a Constants for debug
const DEBUG_TREE_ITERATOR : bool = 1 == 0;

//a TreeIter iterator
//tp TreeIterOp
/// This operation is used to iterate over a tree
///
/// A tree of the form
///
/// ```text
///      A
///     / \ 
///    B   C 
///   / \   \
///   F  G   H
///  /      / \
///  J     K   L
///
/// itetates as
///  Push(A)
///  Push(B)
///  Push(F)
///  Push(J)
///  NoChildren
///  Pop
///  Sibling(G)
///  NoChildren
///  Pop
///  Sibling(C)
///  Push(H)
///  Push(K)
///  NoChildren
///  Sibling(L)
///  NoChildren
///  Pop
///  Pop
///  Pop
///  Pop
pub enum TreeIterOp<V> {
    Push(V),
    Sibling(V),
    NoChildren,
    Pop
}
//tp TreeIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct TreeIter<'a, 'b, V> {
    tree: &'a Tree<'b, V>,
    root_index : usize,
    stack : Vec<(&'a TreeNode<'b, V>, usize)>,
}

//ip TreeIter
impl <'a, 'b, V> TreeIter<'a, 'b, V> {
    //fp new
    /// Create a new iterator
    pub fn new(tree:&'a Tree<'b, V>) -> Self {
        Self { tree,
               root_index: 0,
               stack:Vec::new(),
        }
    }
}

//ip Iterator for TreeIter
impl <'a, 'b, V> Iterator for TreeIter<'a, 'b, V> {
    type Item = TreeIterOp<&'a TreeNode<'b, V>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            if self.root_index > self.tree.roots.len() {
                None
            }
            else if self.root_index == self.tree.roots.len() {
                self.root_index += 1;
                Some(TreeIterOp::Pop)
            } else {
                let n = self.root_index;
                self.root_index += 1;
                self.stack.push((&self.tree.roots[n], 0));
                if n == 0 {
                    Some(TreeIterOp::Push(&self.tree.roots[n]))
                } else {
                    Some(TreeIterOp::Sibling(&self.tree.roots[n]))
                }
            }
        } else {
            let (tn,child_index) = self.stack.pop().unwrap();
            if child_index >= tn.children.len() {
                if child_index == 0 {
                    Some(TreeIterOp::NoChildren)
                } else {
                    Some(TreeIterOp::Pop)
                }
            } else {
                self.stack.push((tn, child_index+1));
                self.stack.push((&tn.children[child_index], 0));
                if child_index == 0 {
                    Some(TreeIterOp::Push(&tn.children[child_index]))
                } else {
                    Some(TreeIterOp::Sibling(&tn.children[child_index]))
                }
            }
        }
    }
}

// never cloned, only moved

//a TreeNode and Tree
//tp TreeNode
/// A tree node is a node in the tree with a mutable reference to the
/// node element, and an array of children
///
/// V is the type of the element refered by the tree
///
/// 'a is the lifetime of the tree
pub struct TreeNode<'a, V> {
    node     : &'a mut V,
    children : Vec<TreeNode<'a, V>>,
}
impl <'a, V> TreeNode<'a, V> {
    pub fn new(node:&'a mut V) -> Self {
        Self { node, children:Vec::new() }
    }
    pub fn add_child(&mut self, child:Self) {
        self.children.push(child);
    }
    //mp borrow_node
    pub fn borrow_node(&self) -> &V {
        self.node
    }
}

//tp Tree
pub struct Tree<'a, V> {
    roots : Vec<TreeNode<'a, V>>,
    stack : Vec<TreeNode<'a, V>>,
}

//ip Tree
impl <'a, V> Tree<'a, V> {
    //fp new
    pub fn new() -> Self {
        Self { roots : Vec::new(),
               stack : Vec::new(),
        }
    }
    //fi push_node
    fn push_node(&mut self, node:&'a mut V) {
        let tree_node = TreeNode::new(node);
        self.stack.push(tree_node);
    }
    //fi pop_node
    fn pop_node(&mut self) {
        assert!(self.stack.len() > 0);
        let tree_node = self.stack.pop().unwrap(); // the tree node is no longer mutatable
        self.add_tree_node(tree_node);
    }
    //fi add_tree_node
    fn add_tree_node(&mut self, tree_node:TreeNode<'a, V>) {
        if self.stack.len() == 0 {
            self.roots.push(tree_node);
        } else {
            let mut parent = self.stack.pop().unwrap();
            parent.add_child(tree_node);
            self.stack.push(parent);
        }
    }
    //fi open_container
    pub fn open_container(&mut self, node:&'a mut V) {
        self.push_node(node);
    }
    //fi add_node
    pub fn add_node(&mut self, node:&'a mut V) {
        let tree_node = TreeNode::new(node);
        self.add_tree_node(tree_node);
    }
    //fi close_container
    pub fn close_container(&mut self) {
        self.pop_node();
    }
    //fi iter_tree
    pub fn iter_tree(&self) -> TreeIter<V> {
        TreeIter::new(self)
    }
}

//tm Test code
#[cfg(test)]
mod test_tree {
    use super::*;
    use crate::{BaseValue, Descriptor, NamedTypeSet, StylableNode};

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
        let mut node0_0 = StylableNode::new("pt", &d_pt);
        node0_0.add_name_value("x", "1");
        node0_0.add_name_value("y", "0");
        let mut node0_1 = StylableNode::new("pt", &d_pt);
        node0_1.add_name_value("x", "2");
        node0_1.add_name_value("y", "10");
        let mut group0 = StylableNode::new("g", &d_g);

        {
            let mut tree = Tree::new();
            tree.open_container(&mut group0);
            tree.add_node(&mut node0_0);
            tree.add_node(&mut node0_1);
            tree.close_container();
            
            for n in tree.iter_tree() {
                // println!("{:?} {:?} {:?}",n.node.id_name, n.node.node_type, n.node.values );
                // n.node.id_name = Some("banana".to_string());
            }
        }
        assert!(false);
    }
}
