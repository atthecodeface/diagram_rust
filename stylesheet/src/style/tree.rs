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
/// iterates as
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
    tree  : &'a Tree<'b, V>,
    start : bool,
    stack : Vec<(usize, usize)>,
}

//ip TreeIter
impl <'a, 'b, V> TreeIter<'a, 'b, V> {
    //fp new
    /// Create a new iterator
    pub fn new(tree:&'a Tree<'b, V>) -> Self {
        Self { tree,
               start:true,
               stack:Vec::new(),
        }
    }
}

//ip Iterator for TreeIter
impl <'a, 'b, V> Iterator for TreeIter<'a, 'b, V> {
    type Item = TreeIterOp<&'a TreeNode<'b, V>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => {
                if self.start {
                    self.stack.push((0, 0));
                    self.start = false;
                    Some(TreeIterOp::Push(&self.tree.nodes[0]))
                } else {
                    None
                }
            },
            Some((node_index, child_index)) => {
                if child_index >= self.tree.nodes[node_index].children.len() {
                    if child_index == 0 {
                        Some(TreeIterOp::NoChildren)
                    } else {
                        Some(TreeIterOp::Pop)
                    }
                } else {
                    let child_node_index = self.tree.nodes[node_index].children[child_index];
                    self.stack.push((node_index, child_index+1));
                    self.stack.push((child_node_index, 0));
                    if child_index == 0 {
                        Some(TreeIterOp::Push(&self.tree.nodes[child_node_index]))
                    } else {
                        Some(TreeIterOp::Sibling(&self.tree.nodes[child_node_index]))
                    }
                }
            }
        }
    }
}

//a TreeNode and Tree
//tp TreeNode
/// A tree node is a node in the tree with a mutable reference to the
/// node element, and an array of children
///
/// V is the type of the element refered by the tree
///
/// 'a is the lifetime of the tree
///
/// A TreeNode is never cloned.
pub struct TreeNode<'a, V> {
    node     : &'a mut V,
    parent   : Option<usize>,
    depth    : usize,
    children : Vec<usize>,
}
impl <'a, V> TreeNode<'a, V> {
    pub fn new(node:&'a mut V, parent:Option<usize>, depth:usize) -> Self {
        Self { node, parent, depth, children:Vec::new() }
    }
    pub fn add_child(&mut self, child:usize) {
        self.children.push(child);
    }
    //mp borrow_node
    pub fn borrow(&self) -> &V {
        self.node
    }
    //mp borrow_mut
    pub fn borrow_mut(&mut self) -> &mut V {
        self.node
    }
    //zz All done
}

//tp Tree
pub struct Tree<'a, V> {
    nodes : Vec<TreeNode<'a, V>>,
    stack : Vec<usize>,
}

//ip Tree
impl <'a, V> Tree<'a, V> {
    //fp new
    pub fn new(node:&'a mut V) -> Self {
        let mut result = Self {
            nodes : Vec::new(),
            stack : Vec::new(),
        };
        result.push_node(node);
        result
    }
    
    //fi int_add_node
    /// Add a node to the tree at the current stack depth and return
    /// its node index and stack index
    fn int_add_node(&mut self, node:&'a mut V) -> (usize, usize) {
        let node_index = self.nodes.len();
        let stack_depth = self.stack.len();
        let parent = { if stack_depth == 0 {None} else {Some(self.stack[stack_depth-1])} };
        let tree_node = TreeNode::new(node, parent, stack_depth);
        if let Some(parent) = parent {
            self.nodes[parent].add_child(node_index);
        }
        self.nodes.push(tree_node);
        (node_index, stack_depth)
    }
    
    //fi push_node
    /// Invoked by new and to open a container element that may have children
    fn push_node(&mut self, node:&'a mut V) {
        let (node_index, _depth) = self.int_add_node(node);
        self.stack.push(node_index);
    }
    
    //fi pop_node
    /// Invoked to close the container
    fn pop_node(&mut self) {
        assert!(self.stack.len() > 0);
        self.stack.pop();
    }

    //fi open_container
    pub fn open_container(&mut self, node:&'a mut V) {
        self.push_node(node);
    }

    //fi add_node
    /// Add a leaf node that will not have children
    /// This means the stack depth will not change
    pub fn add_node(&mut self, node:&'a mut V) {
        self.int_add_node(node);
    }

    //fi close_container
    pub fn close_container(&mut self) {
        self.pop_node();
    }

    //fi iter_tree
    pub fn iter_tree(&self) -> TreeIter<V> {
        TreeIter::new(self)
    }
    //fi borrow
    pub fn borrow(&self, node_index:usize) -> &V {
        self.nodes[node_index].borrow()
    }
    
    //fi borrow_mut
    pub fn borrow_mut(&mut self, node_index:usize) -> &mut V {
        self.nodes[node_index].borrow_mut()
    }
    
    //zz All done
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
        node0_0.add_name_value("x", "1").unwrap();
        node0_0.add_name_value("y", "0").unwrap();
        let mut node0_1 = StylableNode::new("pt", &d_pt);
        node0_1.add_name_value("x", "2").unwrap();
        node0_1.add_name_value("y", "10").unwrap();
        let mut group0 = StylableNode::new("g", &d_g);

        {
            let mut tree = Tree::new(&mut group0);
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
