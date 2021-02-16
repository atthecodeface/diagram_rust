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

@file    active_mask.rs
@brief   Bit mask and vectors thereof
 */

//a Imports
use super::bitmask::{BitMask, BitMaskU32};
use super::rules::{RuleResult, RuleFn, Action, RuleSet};
use super::tree::{Tree, TreeIterOp, TreeNode};

//a Global constants for debug
const DEBUG_RULE_TREE      : bool = 1 == 0;

//a TreeApplicator
//tp TreeApplicator
/// This provides an applicator that applies a RuleSet to a tree
///
/// The Tree must have node element type V, and its lifetime is 'a
///
/// The RuleSet must use the type F, which has the trait RuleFn<V>,
/// which means it has methods that apply itself to a value V and
/// return a RuleResult
///
/// The TreeApplicator maintains a set of 'active rules' as it
/// traverses the tree; there are many mechanisms that could be used
/// to manage such a set, and the mechanism is abstracted out through
/// 'M'. M must therefore be an BitMask that can cope with the
/// number of rules in the RuleSet. Some implementations of BitMask
/// are limited in the number of rules they support (for example a u32
/// bitmask has support for 32 rules), trading off speed versus
/// capability Another implementation could use an BitMask that
/// supports an arbitrary number of rules using, for example, an array
/// of bitmasks.
pub struct TreeApplicator<'a, V, A, F, M>
where V:std::fmt::Debug, A:Action<V>, F: RuleFn<V>, M:BitMask {
    /// The rules that this tree can handle
    rules        : &'a RuleSet<V, A, F>,
    num_rules    : usize,
    active_stack : Vec<M>,
}

//ip TreeApplicator
impl <'a, V, A, F, M> TreeApplicator<'a, V, A, F, M>
where V:std::fmt::Debug, A:Action<V>, F: RuleFn<V>, M:BitMask {
    //fp new
    /// Create a new TreeApplicator consuming a given RuleSet
    pub fn new(rules:&'a RuleSet<V, A, F>) -> Self {
        let num_rules = rules.num_rules();
        let mut active_stack = Vec::new();
        let mut active_mask = M::new(num_rules);
        for i in 0..num_rules {
            if rules.is_toplevel(i) { active_mask.set(i); }
        }
        active_stack.push(active_mask);
        Self { rules, num_rules, active_stack }
    }

    //mi try_rules
    /// Try applying the active rules in the set to a node
    ///
    /// 
    fn try_rules(&mut self, mut active_mask:M, node:&TreeNode<V>) {
        if DEBUG_RULE_TREE {println!("Try mask {:?} to node content {:?}", active_mask, node.borrow());}
        let depth = self.active_stack.len();
        let mut result_mask = active_mask.clone(self.num_rules);
        for i in 0..self.num_rules {
            if active_mask.is_set(i) {
                let action = {
                    let result = self.rules.apply(i, depth, node.borrow());
                    if DEBUG_RULE_TREE {println!("Apply rule {} yields result {}", i, result);}
                    match result {
                        // No match, and dont propagate so clear mask bit
                        RuleResult::MismatchEnd => {
                            result_mask.clear(i);
                            false
                        },
                        // No match, but propagate so leave mask bit
                        RuleResult::MismatchPropagate => {
                            // no action
                            false
                        },
                        // Match, and propagate just child rules to child nodes
                        RuleResult::MatchEndChildren => {
                            for j in self.rules.iter_children(i) {
                                result_mask.set(*j);
                            }
                            result_mask.clear(i);
                            true
                        },
                        // Match, and dont propagate this and child rules to child nodes
                        RuleResult::MatchPropagateChildren => {
                            for j in self.rules.iter_children(i) {
                                result_mask.set(*j);
                            }
                            true
                        },
                        // Match, and propagate just child rules to this and child nodes
                        RuleResult::MatchEndAgain => {
                            for j in self.rules.iter_children(i) {
                                active_mask.set(*j); // so that it child rules will apply to this node
                            }
                            result_mask.clear(i);
                            true
                        },
                        // Match, and propagate this and child rules to this and child nodes
                        RuleResult::MatchPropagateAgain => {
                            for j in self.rules.iter_children(i) {
                                active_mask.set(*j); // so that it child rules will apply to this node
                            }
                            true
                        },
                    }
                };
                if action {
                    self.rules.fire(i, depth, node.borrow());
                }
            }
        }
        if DEBUG_RULE_TREE {println!("Depth after matching now {} with mask {:?}", self.active_stack.len()+1, result_mask);}
        self.active_stack.push(result_mask);
    }

    // mp handle_tree_op
    /// This handles a tree operation returned by a Tree::iter_tree()
    ///
    /// The stack is maintained as:
    /// 
    ///   .. gp parent last_node
    ///
    /// A Push operation leads to:
    ///
    ///   .. ggp gp parent <this node>
    ///
    /// A sibling operation leads to
    ///
    ///   .. gp parent <this node>
    ///
    /// A  pop operation leads to
    ///
    ///   .. parent last_node
    ///
    /// And no children is a nop (in a sense it is push and pop)
    /// 
    ///   .. gp parent last_node
    ///
    pub fn handle_tree_op(&mut self, top:TreeIterOp<&TreeNode<V>>) {
        match top {
            TreeIterOp::Push(node)    => {
                let n = self.active_stack.len();
                let am = self.active_stack[n-1].clone(self.num_rules);
                self.try_rules(am, node) // will push updated am
            }
            TreeIterOp::Sibling(node) => {
                self.active_stack.pop().unwrap();
                let n = self.active_stack.len();
                let am = self.active_stack[n-1].clone(self.num_rules);
                self.try_rules(am, node) // will push updated am
            },
            TreeIterOp::Pop        => {
                self.active_stack.pop();
                if DEBUG_RULE_TREE {println!("Pop to depth {}", self.active_stack.len());}
            }
            TreeIterOp::NoChildren => {
            }
        };
        
    }
}


//tp TreeApplicator32
/// A instance of the TreeApplicator that works for up to 32 rules
pub type TreeApplicator32<'a, T, A, F> = TreeApplicator<'a, T, A, F, BitMaskU32>;

//tm Test code
#[cfg(test)]
mod test_ruleset {
    use super::*;
    #[derive(Debug)]
    struct UsizeNode {
        pub value : usize,
        pub liked : bool,
    }
    impl UsizeNode {
        fn new(value:usize) -> Self {
            Self { value, liked:false }
        }
    }
    struct ActionUsize <'a> {
        callback : Box<dyn Fn(&UsizeNode)->() + 'a>,
    }
    impl <'a> ActionUsize<'a> {
        pub fn new(callback: impl Fn(&UsizeNode)->() + 'a) -> Self {
            Self { callback:Box::new(callback) }
        }
    }
    impl Action<UsizeNode> for ActionUsize<'_> {
        fn apply(&self, rule:usize, depth:usize, value:&UsizeNode)  {
            (self.callback)(value);
        }
    }
    #[derive(Default)]
    pub struct UsizeRule {
        min_value : usize,
        max_value : usize,
        mask : usize,
        value : usize,
    }
    impl UsizeRule {
        pub fn new() -> Self {
            let result : Self = std::default::Default::default();
            result
        }
        pub fn range(mut self, min_value:usize, max_value:usize) -> Self {
            self.min_value = min_value;
            self.max_value = max_value;
            self
        }
        pub fn mask_value(mut self, mask:usize, value:usize) -> Self {
            self.mask =  mask;
            self.value = value;
            self
        }
    }
    impl RuleFn<UsizeNode> for UsizeRule {
        fn apply(&self, _depth:usize, value:&UsizeNode) -> RuleResult {
            if self.min_value < self.max_value {
                if value.value >= self.min_value && value.value < self.max_value {
                    RuleResult::MatchPropagateChildren
                } else {
                    RuleResult::MismatchPropagate
                }
            } else {
                if (value.value & self.mask) == self.value {
                    RuleResult::MatchPropagateChildren
                } else {
                    RuleResult::MismatchPropagate
                }
            }
        }
    }
    impl RuleFn<usize> for UsizeRule {
        fn apply(&self, _depth:usize, value:&usize) -> RuleResult {
            if self.min_value < self.max_value {
                if *value >= self.min_value && *value < self.max_value {
                    RuleResult::MatchPropagateChildren
                } else {
                    RuleResult::MismatchPropagate
                }
            } else {
                if (*value & self.mask) == self.value {
                    RuleResult::MatchPropagateChildren
                } else {
                    RuleResult::MismatchPropagate
                }
            }
        }
    }
    #[test]
    fn test_apply() {
        let mut rules = RuleSet::new();
        let act_0 = rules.add_action(ActionUsize::new(|s| {println!("like this {} {}",s.value, s.liked);}));
        let act_1 = rules.add_action(ActionUsize::new(|s| {println!("Really like this - odd inside ancestor with range 0 to 3 - {} {}",s.value, s.liked);}));
        let rule_0 = rules.add_rule(None, UsizeRule::new().range(0,4),      Some(act_0));
        rules.add_rule(Some(rule_0), UsizeRule::new().mask_value(1,1), Some(act_1));
        {
            let mut root    = UsizeNode::new(16);
            let mut group0  = UsizeNode::new(0); // liked
            let mut node0_0 = UsizeNode::new(1); // liked and really liked
            let mut node0_1 = UsizeNode::new(2); // liked
            let mut node0_2 = UsizeNode::new(4);
            let mut node0_3 = UsizeNode::new(8);
            let mut group1  = UsizeNode::new(4);
            let mut node1_0 = UsizeNode::new(1); // liked
            let mut node1_1 = UsizeNode::new(2); // liked
            let mut node1_2 = UsizeNode::new(4);
            let mut node1_3 = UsizeNode::new(8);
            let mut tree = Tree::new(&mut root);
            tree.open_container(&mut group0);
            tree.add_node(&mut node0_0);
            tree.add_node(&mut node0_1);
            tree.add_node(&mut node0_2);
            tree.add_node(&mut node0_3);
            tree.close_container();
            tree.open_container(&mut group1);
            tree.add_node(&mut node1_0);
            tree.add_node(&mut node1_1);
            tree.add_node(&mut node1_2);
            tree.add_node(&mut node1_3);
            tree.close_container();
            tree.close_container(); // closes root

            let mut applicator = TreeApplicator32::new(&rules);
            for n in tree.iter_tree() {
                applicator.handle_tree_op(n);
            }
        }
        assert!(false);
    }
}

