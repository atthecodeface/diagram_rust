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

@file    traits.rs
@brief   Traits for the grid layout
 */

//a Imports

//a Traits
//tt NodeId
/// Types must implement this trait if they are to be used as a node ID in the grid
///
/// This must be copy; it is stored in the resolver for the nodes and links. It is *not* a node!
///
/// Default is required *not* because it is used, but because
/// Resolver<N:NodeId> can only be default if N is; N::default is not
/// used
pub trait NodeId:
    Sized
    + PartialEq
    + Eq
    + std::hash::Hash
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + std::default::Default
{
}

//ip NodeId for usize
impl NodeId for usize {}
