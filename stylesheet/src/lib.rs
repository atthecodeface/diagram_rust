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

@file    stylesheet/lib.rs
@brief   Stylesheet library
 */

//a Documentation
//! This library provides mechanisms to support a hierarchy of
//! 'stylable' nodes in conjunction with stylesheets.
//!
//! The system requires a definition of what may be styled, which
//! consists of style names and types.

//a Imports
extern crate regex;
extern crate num;
#[macro_use]
extern crate lazy_static;

mod style;
mod tree_rules;

pub use style::{TypeValue, ValueError, BaseValue};
pub use style::{NamedTypeSet};
pub use style::{StylableNode};
pub use style::{Descriptor};

pub use tree_rules::{BitMask, BitMaskU32, BitMaskU64};
pub use tree_rules::{RuleResult, RuleFn, Action, RuleSet};
pub use tree_rules::{Tree, TreeIterOp};
pub use tree_rules::{TreeApplicator32, TreeApplicator64};

