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

//ip RuleResult
impl RuleResult {
    pub fn new(depth: usize, matched: bool, sideways: bool, max_depth: usize) -> Self {
        let downwards = (max_depth == 0) || (depth < max_depth);
        match (matched, downwards, sideways) {
            (true, true, true) => Self::MatchPropagateAgain,
            (true, true, false) => Self::MatchPropagateChildren,
            (true, false, true) => Self::MatchEndAgain,
            (true, _, _) => Self::MatchEndChildren,
            (_, true, _) => Self::MismatchPropagate,
            _ => Self::MismatchEnd,
        }
    }
}

//ip std::fmt::Display for RuleResult
impl std::fmt::Display for RuleResult {
    //mp fmt
    /// Display the rule
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::MismatchEnd => write!(f, "mismatch"),
            Self::MismatchPropagate => write!(f, "mismatch but apply rule to children"),
            Self::MatchEndChildren => write!(f, "*match*"),
            Self::MatchPropagateChildren => write!(f, "*match* and also apply rule to children"),
            Self::MatchEndAgain => write!(f, "*match* and apply child rules to this node"),
            Self::MatchPropagateAgain => write!(
                f,
                "*match* and apply child rules only to this node and this rule to children"
            ),
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
    fn apply(&self, depth: usize, value: &T) -> RuleResult;
}

//tp trait Action<T>
/// This trait must be satisfied by a struct that a RuleSet can use as
/// an action
///
/// It requires a single function that applies the rule to a tree
/// depth and struct value, and it returns the result of applying the
/// rule to that value
pub trait Action<T> {
    /// Apply the action to the value, at a given depth in the tree
    fn apply(&self, _rule: usize, depth: usize, _value: &mut T) {
        println!("Application of action not defined {}", depth);
    }
}

//tp Rule
/// A rule is RuleFn and an optional action indicator; it is part of a
/// RuleSet, which is a Vec of Rule's; it has children, indicated by
/// indices in to the RuleSet
///
/// This type is private to this module
struct Rule<T, F: RuleFn<T>> {
    match_fn: F,
    parent: Option<usize>,
    action: Option<usize>,
    child_rules: Vec<usize>,
    phantom: std::marker::PhantomData<T>,
}

//ip Rule
impl<T, F: RuleFn<T>> Rule<T, F> {
    //fp new
    /// Create a new rule given a RuleFn to match against and an
    /// optional action to take if the rule is matched
    fn new(parent: Option<usize>, match_fn: F, action: Option<usize>) -> Self {
        Self {
            parent,
            match_fn,
            action,
            child_rules: Vec::new(),
            phantom: std::marker::PhantomData,
        }
    }

    //fp add_child
    /// Add a child rule to this rule; a child rule is enabled if this
    /// rule matches, and that child rule will be applied to either
    /// children (if the rule optional action to take if the rule is
    /// matched with a '..Children' result) or this node (if the rule
    /// is matched with a '..Again' result).
    fn add_child(&mut self, rule: usize) {
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
/// A is a type that has the Action trait, which actions must be
/// instances of. The Action trait methods will be invoked when the
/// actions need to be taken due to the rules.
///
/// F is a type that has the RuleFn trait to allow it to be applied to
/// a value of type T and get a rule match result.
///
/// Note that the rule's children will all have a greater rule index
/// than the parent rule. This invariant is required
pub struct RuleSet<T, A: Action<T>, F: RuleFn<T>> {
    rules: Vec<Rule<T, F>>,
    actions: Vec<A>,
}

//ip RuleSet
impl<T, A: Action<T>, F: RuleFn<T>> RuleSet<T, A, F> {
    //fp new
    /// Create an empty rule set
    #[inline]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            actions: Vec::new(),
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
    pub fn add_action(&mut self, action: A) -> usize {
        let action_num = self.actions.len();
        self.actions.push(action);
        action_num
    }

    //mp add_rule
    /// Add a rule to the set, and return its handle; it may be a
    /// child of a parent rule given by a handle, or a toplevel rule.
    ///
    /// Note that the rule's children will all have a greater index
    /// than the rule. This invariant is required
    pub fn add_rule(&mut self, parent: Option<usize>, match_fn: F, action: Option<usize>) -> usize {
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
    pub fn is_toplevel(&self, rule: usize) -> bool {
        self.rules[rule].parent.is_none()
    }

    //mp apply
    #[inline]
    pub fn apply(&self, rule: usize, depth: usize, node: &T) -> RuleResult {
        self.rules[rule].match_fn.apply(depth, node)
    }

    //mp fire
    #[inline]
    pub fn fire(&self, rule: usize, depth: usize, node: &mut T) {
        if let Some(action) = self.rules[rule].action {
            self.actions[action].apply(rule, depth, node);
        }
    }

    //mp iter_children
    #[inline]
    pub fn iter_children(&self, rule: usize) -> std::slice::Iter<usize> {
        self.rules[rule].iter_children()
    }

    //zz All done
}

//a Test
//tm Test code
#[cfg(test)]
mod test_types {
    use super::*;
    pub struct UsizeRule {
        min_value: usize,
        max_value: usize,
    }
    impl UsizeRule {
        pub fn new(min_value: usize, max_value: usize) -> Self {
            Self {
                min_value,
                max_value,
            }
        }
    }
    impl Action<usize> for usize {
        fn apply(&self, rule: usize, depth: usize, value: &mut usize) {
            println!(
                "Apply to {} because of rule {} at depth {}",
                value, rule, depth
            );
        }
    }
    impl RuleFn<usize> for UsizeRule {
        fn apply(&self, _depth: usize, value: &usize) -> RuleResult {
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
    use super::test_types::*;
    use super::*;
    #[test]
    fn test_simple() {
        let mut rules = RuleSet::new();
        rules.add_action(0);
        rules.add_rule(None, UsizeRule::new(0, 10), None);
    }
}
