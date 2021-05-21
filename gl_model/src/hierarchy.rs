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

@file    hierarchy.rs
@brief   Hiearchy using usize indices to arrays of elements
 */

//a Imports

//a Constants
const DEBUG_ITERATOR : bool = false;

//a Node
//tp Node
pub struct Node<T> {
    /// An optional parent index - if None, this is a root
    parent         : Option<usize>,
    /// Array of child indices
    children       : Vec<usize>,
    /// Data associated with the node
    pub data: T,
}

//ip Node
impl <T> Node<T> {
    //fp new
    /// Create a new bone with a given rest
    pub fn new(data:T, parent:Option<usize>) -> Self {
        let children = Vec::new();
        Self { parent, children, data }
    }
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
    pub fn set_parent(&mut self, parent:Option<usize>) {
        self.parent = parent;
    }
    pub fn add_child(&mut self, child:usize) {
        self.children.push(child);
    }
    pub fn has_children(&self) -> bool {
        self.children != []
    }
}

//a Hierarchy
pub struct Hierarchy<T> {
    elements : Vec<Node<T>>,
    roots    : Vec<usize>,
}
impl <T> Hierarchy<T> {
    pub fn new() -> Self {
        Self { elements:Vec::new(), roots:Vec::new() }
    }
    pub fn len(&self) -> usize { self.elements.len() }
    pub fn add_node(&mut self, data:T ) -> usize {
        let n = self.elements.len();
        self.elements.push(Node::new(data, None));
        n
    }
    pub fn relate(&mut self, parent:usize, child:usize) {
        self.elements[parent].add_child(child);
        self.elements[child].set_parent(Some(parent));
    }
    pub fn find_roots(&mut self) {
        self.roots = Vec::new();
        for (i,e) in self.elements.iter().enumerate() {
            if !e.has_parent() {
                self.roots.push(i);
            }
        }
    }
    pub fn borrow_mut(&mut self) -> (&Vec<usize>, &mut Vec<Node<T>>) {
        (&self.roots, &mut self.elements)
    }
    pub fn borrow_roots(&self) -> &Vec<usize> {
        &self.roots
    }
    pub fn enum_from_root<'z> (&'z self, root:usize) -> NodeEnum<T> {
        NodeEnum::new(&self.elements, root)
    }
    pub fn iter_from_root<'z> (&'z self, root:usize) -> NodeIter<T> {
        NodeIter::new(&self.elements, root)
    }
    pub fn borrow_elements<'z> (&'z self) -> &Vec<Node<T>> {
        &self.elements
    }
}

//a NodeIterOp
//tp NodeIterOp
/// This enumeration provides
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeIterOp<T> {
    /// Pushing in to the hierachy to new node index, and true if node has children
    Push(T, bool),
    /// Popping out to the hierachy to node index
    Pop(T),
}

impl <T> NodeIterOp<T> {
    #[inline]
    pub fn is_pop(&self) -> bool {
        match self {
            Self::Pop(_) => true,
            _ => false
        }
    }
}

//a Recipe
//tp Recipe
pub struct Recipe {
    ops       : Vec<NodeIterOp<usize>>,
    max_depth : usize,
    depth     : usize,
}

impl Recipe {
    pub fn new() -> Self {
        Self { ops:Vec::new(), max_depth:0, depth:0 }
    }
    pub fn add_op(&mut self, op:NodeIterOp<usize>) {
        if op.is_pop() { self.depth -= 1; } else {
            self.depth +=1;
            if self.depth > self.max_depth { self.max_depth = self.depth; }
        }
        self.ops.push(op);
    }
    pub fn depth(&self) -> usize {
        self.max_depth
    }
    pub fn get(self) -> (usize, Vec<NodeIterOp<usize>>) {
        (self.max_depth, self.ops)
    }
    pub fn borrow_ops<'z> (&'z self) -> &'z Vec<NodeIterOp<usize>> {
        &self.ops
    }
    pub fn of_ops<T>(iter:NodeEnum<T>) -> Self {
        let mut r = Self::new();
        for op in iter {
            r.add_op(op);
        }
        r
    }
}

//a NodeEnum
//ti NodeEnumState
/// This enumeration provides
#[derive(Debug, Clone, Copy)]
enum NodeEnumState {
    /// PreNode indicates that the element has not been returned yet
    PreNode(usize),
    PreChildren(usize),
    Child(usize,usize),
    PostChildren(usize),
}

//tp NodeEnum
/// An iterator structure to permit iteration over a hierarchy of nodes
///
/// The iterator yields pairs of (NodeEnumState, usize)
/// For a hierarchy of nodes:
///   A -> B -> C0
///             C1
///        D
///        E  -> F
/// the iterator will provide
///
/// Push(A,true), Push(B,true), Push(C0,false), Pop(C0), Push(C1,false), Pop(C1), Pop(B), Push(D,false), Pop(D), Push(E,true), Push(F,false), Pop(F), Pop(E), Pop(A)
pub struct NodeEnum<'a, T> {
    /// Hierarchy of nodes that is being iterated over
    pub hierarchy : &'a [Node<T>],
    /// Stack of indices in to the hierarchy and whether the node at that point has been handled
    stack     : Vec<NodeEnumState>,
}

//ip NodeEnum
impl <'a, T> NodeEnum<'a, T> {
    //fp new
    /// Create a new hierarchy node iterator
    pub fn new(hierarchy:&'a [Node<T>], root:usize) -> Self {
        let mut stack = Vec::new();
        stack.push(NodeEnumState::PreNode(root));
        Self { hierarchy, stack }
    }
}

//ip Iterator for NodeEnum
impl <'a, T> Iterator for NodeEnum<'a, T> {
    type Item = NodeIterOp<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0 {
            None
        } else {
            let se = self.stack.pop().unwrap();
            // Track the state for debugging
            if DEBUG_ITERATOR {
                println!("{:?}", se );
            }
            match se {
                NodeEnumState::PreNode(x) => {
                    self.stack.push(NodeEnumState::PreChildren(x));
                    let has_children = self.hierarchy[x].has_children();
                    Some(NodeIterOp::Push(x, has_children))
                },
                NodeEnumState::PreChildren(x) => { // Push(x) has happened
                    self.stack.push(NodeEnumState::Child(x,0));
                    self.next()
                },
                NodeEnumState::Child(x, n) => { // Children of x prior to n have happened
                    if let Some(c) = self.hierarchy[x].children.get(n) {
                        self.stack.push(NodeEnumState::Child(x,n+1));
                        self.stack.push(NodeEnumState::PreNode(*c));
                    } else { // run out of children
                        self.stack.push(NodeEnumState::PostChildren(x));
                    }
                    self.next()
                },
                NodeEnumState::PostChildren(x) => { // Push(x) and all children ops have happened
                    Some(NodeIterOp::Pop(x))
                },
            }
        }
    }
}

//ip NodeIter
pub struct NodeIter<'a, T> {
    node_enum : NodeEnum<'a, T>,
}
impl <'a, T> NodeIter<'a, T> {
    //fp new
    /// Create a new hierarchy node iterator
    pub fn new(hierarchy:&'a [Node<T>], root:usize) -> Self {
        Self { node_enum : NodeEnum::new(hierarchy, root) }
    }
}

//ip Iterator for NodeIter
impl <'a, T> Iterator for NodeIter<'a, T> {
    type Item = NodeIterOp<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.node_enum.next() {
            Some(NodeIterOp::Push(x,c)) => {
                Some(NodeIterOp::Push(&self.node_enum.hierarchy[x].data, c))
            },
            Some(NodeIterOp::Pop(x)) => {
                Some(NodeIterOp::Pop(&self.node_enum.hierarchy[x].data))
            },
            None => None,
        }
    }
}

//a Test
#[cfg(test)]
mod test_node {
    use super::*;
    //fi basic_hierarchy
    pub fn basic_hierarchy() -> Hierarchy<&'static str> {
        let mut h = Hierarchy::new();
        let a = h.add_node("A");
        let b = h.add_node("B");
        let c0 = h.add_node("C0");
        let c1 = h.add_node("C1");
        let d = h.add_node("D");
        let e = h.add_node("E");
        let f = h.add_node("F");
        h.relate(a,b);
        h.relate(a,d);
        h.relate(a,e);
        h.relate(b,c0);
        h.relate(b,c1);
        h.relate(e,f);
        h.find_roots();
        h
    }
    //fi test_0
    #[test]
    fn test_0() {
        let h = basic_hierarchy();
        assert_eq!(h.borrow_roots(), &[0], "Expect roots to just be A" );
    }
    //fi test_recipe
    #[test]
    fn test_recipe() {
        let h = basic_hierarchy();
        let mut r = Recipe::new();
        for op in h.enum_from_root(0) {
            r.add_op(op);
        }
        let (max_depth, ops) = r.get();
        assert_eq!(max_depth, 3, "Max depth of tree is 3" );
        assert_eq!(ops, vec![ NodeIterOp::Push(0,true),
                              NodeIterOp::Push(1,true),
                              NodeIterOp::Push(2,false),
                              NodeIterOp::Pop(2),
                              NodeIterOp::Push(3,false),
                              NodeIterOp::Pop(3),
                              NodeIterOp::Pop(1),
                              NodeIterOp::Push(4,false),
                              NodeIterOp::Pop(4),
                              NodeIterOp::Push(5,true),
                              NodeIterOp::Push(6,false),
                              NodeIterOp::Pop(6),
                              NodeIterOp::Pop(5),
                              NodeIterOp::Pop(0),        ],
                   "Recipe mismatch" );
    }
}
