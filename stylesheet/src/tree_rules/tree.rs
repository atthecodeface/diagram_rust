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
// const DEBUG_TREE_ITERATOR : bool = 1 == 0;

//a TreeIterOp
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
    Pop,
}

impl<V> TreeIterOp<V> {
    pub fn map<U, F: FnOnce(V) -> U>(self, f: F) -> TreeIterOp<U> {
        match self {
            TreeIterOp::Push(x) => TreeIterOp::Push(f(x)),
            TreeIterOp::Sibling(x) => TreeIterOp::Sibling(f(x)),
            TreeIterOp::NoChildren => TreeIterOp::NoChildren,
            TreeIterOp::Pop => TreeIterOp::Pop,
        }
    }
    pub fn as_option(self) -> Option<V> {
        match self {
            TreeIterOp::Push(x) => Some(x),
            TreeIterOp::Sibling(x) => Some(x),
            TreeIterOp::NoChildren => None,
            TreeIterOp::Pop => None,
        }
    }
}
//a TreeIndexIter non-iterator
//tp TreeIndexIter
/// An iterator structure to permit iteration over a tree
#[derive(Debug)]
pub struct TreeIndexIter {
    start: bool,
    stack: Vec<(usize, usize)>,
}

//ip Default for TreeIterIter
impl Default for TreeIndexIter {
    fn default() -> Self {
        Self {
            start: true,
            stack: Vec::new(),
        }
    }
}

//ip TreeIndexIter
impl TreeIndexIter {
    //fp new
    /// Create a new iterator
    pub fn new() -> Self {
        Self::default()
    }

    //mp next
    fn next<V>(&mut self, tree: &Tree<V>) -> Option<TreeIterOp<(usize, usize)>> {
        match self.stack.pop() {
            None => {
                if self.start {
                    self.stack.push((0, 0));
                    self.start = false;
                    Some(TreeIterOp::Push((0, 0)))
                } else {
                    None
                }
            }
            Some((node_index, child_index)) => {
                if child_index >= tree.nodes[node_index].children.len() {
                    if child_index == 0 {
                        Some(TreeIterOp::NoChildren)
                    } else {
                        Some(TreeIterOp::Pop)
                    }
                } else {
                    let child_node_index = tree.nodes[node_index].children[child_index];
                    let depth = tree.nodes[node_index].depth;
                    self.stack.push((node_index, child_index + 1));
                    self.stack.push((child_node_index, 0));
                    if child_index == 0 {
                        Some(TreeIterOp::Push((depth, child_node_index)))
                    } else {
                        Some(TreeIterOp::Sibling((depth, child_node_index)))
                    }
                }
            }
        }
    }

    //zz All done
}

//a TreeIter iterator
//tp TreeIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct TreeIter<'a, 'b, V> {
    tree: &'a Tree<'b, V>,
    iter: TreeIndexIter,
}

//ip TreeIter
impl<'a, 'b, V> TreeIter<'a, 'b, V> {
    //fp new
    /// Create a new iterator
    pub fn new(tree: &'a Tree<'b, V>) -> Self {
        Self {
            tree,
            iter: TreeIndexIter::new(),
        }
    }
}

//ip Iterator for TreeIter
impl<'a, 'b, V> Iterator for TreeIter<'a, 'b, V> {
    //tp Item
    type Item = TreeIterOp<(usize, &'a TreeNode<'b, V>)>;

    //mp next
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next(self.tree)
            .map(|top| top.map(|(d, i)| (d, &self.tree.nodes[i])))
    }

    //zz All done
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
    node: &'a mut V,
    _parent: Option<usize>,
    depth: usize,
    children: Vec<usize>,
}
impl<'a, V> TreeNode<'a, V> {
    pub fn new(node: &'a mut V, parent: Option<usize>, depth: usize) -> Self {
        Self {
            node,
            _parent: parent,
            depth,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: usize) {
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
    nodes: Vec<TreeNode<'a, V>>,
    stack: Vec<usize>,
}

//ip Tree
impl<'a, V> Tree<'a, V> {
    //fp new
    pub fn new(node: &'a mut V) -> Self {
        let mut result = Self {
            nodes: Vec::new(),
            stack: Vec::new(),
        };
        result.push_node(node);
        result
    }

    //fi int_add_node
    /// Add a node to the tree at the current stack depth and return
    /// its node index and stack index
    fn int_add_node(&mut self, node: &'a mut V) -> (usize, usize) {
        let node_index = self.nodes.len();
        let stack_depth = self.stack.len();
        let parent = {
            if stack_depth == 0 {
                None
            } else {
                Some(self.stack[stack_depth - 1])
            }
        };
        let tree_node = TreeNode::new(node, parent, stack_depth);
        if let Some(parent) = parent {
            self.nodes[parent].add_child(node_index);
        }
        self.nodes.push(tree_node);
        (node_index, stack_depth)
    }

    //fi push_node
    /// Invoked by new and to open a container element that may have children
    fn push_node(&mut self, node: &'a mut V) {
        let (node_index, _depth) = self.int_add_node(node);
        self.stack.push(node_index);
    }

    //fi pop_node
    /// Invoked to close the container
    fn pop_node(&mut self) {
        assert!(!self.stack.is_empty());
        self.stack.pop();
    }

    //mp open_container
    pub fn open_container(&mut self, node: &'a mut V) {
        self.push_node(node);
    }

    //mp add_node
    /// Add a leaf node that will not have children
    /// This means the stack depth will not change
    pub fn add_node(&mut self, node: &'a mut V) {
        self.int_add_node(node);
    }

    //mp close_container
    pub fn close_container(&mut self) {
        self.pop_node();
    }

    //mp iter_tree
    pub fn iter_tree(&self) -> TreeIter<V> {
        TreeIter::new(self)
    }

    //mp it_create
    /// Create an iterator over the tree
    ///
    /// It is a logic error to mutate the tree after this is called,
    /// before the iterator is finished
    #[inline]
    #[must_use]
    pub fn it_create(&self) -> TreeIndexIter {
        TreeIndexIter::new()
    }

    //mp it_next
    /// Return None if iterator complete, or
    /// Some(TreeOp<(depth,node_index)>) if returning a node or tree
    /// operation
    pub fn it_next(&self, iter: &mut TreeIndexIter) -> Option<TreeIterOp<(usize, usize)>> {
        iter.next(self)
    }

    //mp it_next_borrow_mut
    /// Return None if iterator complete, or Some(&mut V)
    pub fn it_next_borrow_mut(&mut self, iter: &mut TreeIndexIter) -> Option<(usize, &mut V)> {
        if let Some(r) = iter.next(self) {
            if let Some((depth, index)) = r.as_option() {
                Some((depth, self.borrow_mut(index)))
            } else {
                self.it_next_borrow_mut(iter)
            }
        } else {
            None
        }
    }

    //mp borrow
    pub fn borrow(&self, node_index: usize) -> &V {
        self.nodes[node_index].borrow()
    }

    //mp borrow_mut
    pub fn borrow_mut(&mut self, node_index: usize) -> &mut V {
        self.nodes[node_index].borrow_mut()
    }

    //zz All done
}
