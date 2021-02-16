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
use super::tree::{Tree, TreeIterOp, TreeNode};

//a Global constants for debug
// const DEBUG_      : bool = 1 == 0;

//a Rules
//tp RuleResult
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuleResult {
    /// Indicate that the rule does not match and it should not propagate in to children
    MismatchEnd,
    /// Indicate that the rule does not match but it should propagate in to children
    MismatchPropagate,
    /// Indicate that the rule matched and it should not propagate in to children
    /// A match indicates that the rule action fires and subrules are enabled for children
    MatchEndChildren,
    /// Indicate that the rule matched and it should propagate in to children
    /// A match indicates that the rule action fires and subrules are enabled for children
    MatchPropagateChildren,
    /// Indicate that the rule matched and it should not propagate in to children
    /// A match indicates that the rule action fires and subrules are run on for the node
    MatchEndAgain,
    /// Indicate that the rule matched and it should propagate in to children
    /// A match indicates that the rule action fires and subrules are run on for the node
    MatchPropagateAgain,
}

//ip std::fmt::Display for RuleResult
impl std::fmt::Display for RuleResult {
    //mp fmt
    /// Display the rule
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::MismatchEnd        => write!(f, "mismatch"),
            Self::MismatchPropagate  => write!(f, "mismatch but apply rule to children"),
            Self::MatchEndChildren        => write!(f, "*match*"),
            Self::MatchPropagateChildren  => write!(f, "*match* and also apply rule to children"),
            Self::MatchEndAgain           => write!(f, "*match* and apply child rules to this node"),
            Self::MatchPropagateAgain     => write!(f, "*match* and apply child rules only to this node and this rule to children"),
        }
    }
}


//tp trait RuleFn<T>
/// This trait must be satisfied by a struct that a RuleSet can be made for
///
/// It requires a single function that applies the rule to a tree
/// depth and struct value, and it returns the result of applying the
/// rule to that value
pub trait RuleFn<T> {
    /// Apply the rule to the value, at a given depth in the tree
    fn apply(&self, depth:usize, value:&T) -> RuleResult;
}

//tp Rule
/// A rule is RuleFn and an optional action indicator; it is part of a
/// RuleSet, which is a Vec of Rule's; it has children, indicated by
/// indices in to the RuleSet
///
/// This type is private to this module
struct Rule<T, F:RuleFn<T>> {
    match_fn: F,
    parent : Option<usize>,
    action : Option<usize>,
    child_rules : Vec<usize>,
    phantom : std::marker::PhantomData::<T>,
}

//ip Rule
impl <T, F:RuleFn<T>> Rule<T, F> {
    //fp new
    /// Create a new rule given a RuleFn to match against and an
    /// optional action to take if the rule is matched
    fn new(parent:Option<usize>, match_fn:F, action:Option<usize>) -> Self {
        Self { parent, match_fn, action, child_rules:Vec::new(), phantom:std::marker::PhantomData }
    }

    //fp add_child
    /// Add a child rule to this rule; a child rule is enabled if this
    /// rule matches, and that child rule will be applied to either
    /// children (if the rule optional action to take if the rule is
    /// matched with a '..Children' result) or this node (if the rule
    /// is matched with a '..Again' result).
    fn add_child(&mut self, rule:usize) {
        self.child_rules.push(rule);
    }

    //mp iter_children
    #[inline]
    pub fn iter_children(&self) -> std::slice::Iter<usize> {
        self.child_rules.iter()
    }
    
    //zz All done
}

//tp RuleSet
/// A set of rules that can be applied
///
/// T is the type of the value which rules are tested against
///
/// F is a type that has the RuleFn trait to allow it to be applied to
/// a value of type T and get a rule match result.
///
/// Note that the rule's children will all have a greater rule index
/// than the parent rule. This invariant is required
pub struct RuleSet<T, F:RuleFn<T>> {
    rules :   Vec<Rule<T,F>>,
    actions : Vec<Rule<T,F>>,
}

//ip RuleSet
impl <T,F:RuleFn<T>> RuleSet <T,F> {
    //fp new
    /// Create an empty rule set
    #[inline]
    pub fn new() -> Self {
        Self { rules : Vec::new(),
               actions : Vec::new()
        }
    }

    //mp num_rules
    /// Find the number of rules in the set
    #[inline]
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }
    
    //mp add_action
    /// Add an action to the set
    pub fn add_action(&mut self) -> usize {
        let action_num = self.actions.len();
        action_num
    }
    
    //mp add_rule
    /// Add a rule to the set, and return its handle; it may be a
    /// child of a parent rule given by a handle, or a toplevel rule.
    ///
    /// Note that the rule's children will all have a greater index
    /// than the rule. This invariant is required
    pub fn add_rule(&mut self, parent:Option<usize>, match_fn:F, action:Option<usize>) -> usize {
        let rule = Rule::new(parent, match_fn, action);
        let rule_num = self.rules.len();
        if let Some(parent) = parent {
            assert!(parent < rule_num);
            self.rules[parent].add_child(rule_num);
        }
        self.rules.push(rule);
        rule_num
    }
    
    //mp is_toplevel
    /// Returns true if the rule has no parents - so it should be
    /// tested from the start to all root nodes
    pub fn is_toplevel(&self, rule:usize) -> bool {
        self.rules[rule].parent.is_none()
    }

    //mp apply
    #[inline]
    pub fn apply(&self, rule:usize, depth:usize, node:&T) -> RuleResult {
        self.rules[rule].match_fn.apply(depth, node)
    }
    
    //mp iter_children
    #[inline]
    pub fn iter_children(&self, rule:usize) -> std::slice::Iter<usize> {
        self.rules[rule].iter_children()
    }
    
    //zz All done
}

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
pub struct TreeApplicator<'a, V, F, M>
where V:std::fmt::Debug, F: RuleFn<V>, M:BitMask {
    /// The rules that this tree can handle
    rules        : &'a RuleSet<V, F>,
    num_rules    : usize,
    active_stack : Vec<M>,
}

//tp TreeApplicator32
/// A instance of the TreeApplicator that works for up to 32 rules
pub type TreeApplicator32<'a, T,F> = TreeApplicator<'a, T,F,BitMaskU32>;

//ip TreeApplicator
impl <'a, V, F, M> TreeApplicator<'a, V, F, M>
where V:std::fmt::Debug, F: RuleFn<V>, M:BitMask {
    //fp new
    /// Create a new TreeApplicator consuming a given RuleSet
    pub fn new(rules:&'a RuleSet<V, F>) -> Self {
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
        println!("Try mask {:?} to node content {:?}", active_mask, node.borrow_node());
        let depth = self.active_stack.len();
        let mut result_mask = active_mask.clone(self.num_rules);
        for i in 0..self.num_rules {
            if active_mask.is_set(i) {
                let action = {
                    let result = self.rules.apply(i, depth, node.borrow_node());
                    println!("Apply rule {} yields result {}", i, result);
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
                if action { println!("Should do action"); }
            }
        }
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
                let am = self.active_stack.pop().unwrap();
                self.try_rules(am, node) // will push updated am
            },
            TreeIterOp::Pop        => {
                self.active_stack.pop();
            }
            TreeIterOp::NoChildren => {
            }
        };
        
    }
}


//tm Test code
#[cfg(test)]
mod test_types {
    use super::*;
    pub struct UsizeRule {
        min_value : usize,
        max_value : usize,
    }
    impl UsizeRule {
        pub fn new(min_value:usize, max_value:usize) -> Self { Self { min_value, max_value } }
    }
    impl RuleFn<usize> for UsizeRule {
        fn apply(&self, _depth:usize, value:&usize) -> RuleResult {
            if *value >= self.min_value && *value < self.max_value {
                RuleResult::MatchPropagateChildren
            } else {
                RuleResult::MismatchPropagate
            }
        }
    }
}
#[cfg(test)]
mod test_ruleset {
    use super::*;
    use super::test_types::*;
    #[test]
    fn test_simple() {
        let mut rules = RuleSet::new();
        rules.add_rule(None, UsizeRule::new(0,10), None);
    }
    #[test]
    fn test_apply() {
        let mut rules = RuleSet::new();
        rules.add_rule(None, UsizeRule::new(0,10), None);
        {
            let mut tree = Tree::new();
            let mut group0 = 0;
            let mut node0_0 = 1;
            let mut node0_1 = 2;
            tree.open_container(&mut group0);
            tree.add_node(&mut node0_0);
            tree.add_node(&mut node0_1);
            tree.close_container();

            let mut applicator = TreeApplicator32::new(&rules);
            for n in tree.iter_tree() {
                applicator.handle_tree_op(n);
            }
        }
        assert!(false);
    }
}

