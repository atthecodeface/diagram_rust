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

@file    node.rs
@brief   A node in the grid layout
 */

//a Imports
use super::NodeId;

//a Node
//ti Node
/// A node in the grid row - in essence a border between cells
///
/// This can be linked to other cells through multiple content handled by the `Link` references,
/// which refer to the other end of the link
#[derive(Debug)]
pub struct Node<N: NodeId> {
    /// The index into our vec of ids
    pub index: usize,
    /// The forced position of the node
    pub forced_position: Option<f64>,
    /// The placed position of the node
    ///
    /// A node that is forced will have a placed_position set to its
    /// forced_position A node that is not forced but is placed
    /// because it is a root or leaf of the network with no dependency
    /// on forced nodes will have a placed_position but not a
    /// forced_position
    pub placed_position: Option<f64>,
    /// The derived position of the node
    ///
    /// A node that is forced/placed will have a position set to its
    /// forced/placed position.
    ///
    /// The position may be otherwise derived - from simply following
    /// links or from the spring model
    ///
    pub position: Option<f64>,
    /// The `node_id`s of the 'start's of links for which this node is the end
    pub link_starts: Vec<N>,
    /// The `node_id`s of the 'end's of links for which this node is the start
    pub link_ends: Vec<N>,
}

//ip Node
impl<N: NodeId> Node<N> {
    //fp new
    /// Create a new node with a given client id
    fn new(index: usize) -> Self {
        Self {
            index,
            forced_position: None,
            placed_position: None,
            position: None,
            link_starts: Vec::new(),
            link_ends: Vec::new(),
        }
    }

    //fp new_with_link_from
    /// Create a new node with a given client index that is linked from
    /// another startpoint id
    pub fn new_with_link_from(index: usize, s: N) -> Self {
        let mut n = Self::new(index);
        n.add_startpoint(s);
        n
    }

    //fp new_with_link_to
    /// Create a new node with a given client index that links to
    /// another startpoint id
    pub fn new_with_link_to(index: usize, e: N) -> Self {
        let mut n = Self::new(index);
        n.add_endpoint(e);
        n
    }

    //mp add_startpoint
    /// Add another node_id to the list of nodes that link to this one
    ///
    /// The node_id *must not* already be in the list
    pub fn add_startpoint(&mut self, s: N) {
        self.link_starts.push(s);
    }

    //mp add_endpoint
    /// Add another node_id to the list of nodes that this links to
    ///
    /// The node_id *must not* already be in the list
    pub fn add_endpoint(&mut self, e: N) {
        self.link_ends.push(e);
    }

    //fp forced_position
    /// Set the forced position
    pub fn forced_position(&mut self, forced_position: Option<f64>) {
        self.forced_position = forced_position;
    }

    //fp placed_position
    /// Set the placed position
    pub fn placed_position(&mut self, placed_position: Option<f64>) {
        self.placed_position = placed_position;
    }

    //fp reset_position
    /// Reset the derived position of the [Node]
    pub fn reset_position(&mut self) {
        self.position = None;
    }

    //fp set_min_position
    /// Set the minimum position of the [Node]. This is the
    /// furthest-to-the-right (i.e. max value) of the two positions
    /// (if there are two)
    pub fn set_min_position(&mut self, p: f64) {
        if self.is_placed() {
            self.position = Some(self.get_position());
        } else if let Some(pos) = self.position {
            if p > pos {
                self.position = Some(p);
            }
        } else {
            self.position = Some(p);
        }
    }

    //fp set_max_position
    /// Set the maximum position of the [Node]. This is the
    /// furthest-to-the-left (i.e. min value) of the two positions
    /// (if there are two)
    pub fn set_max_position(&mut self, p: f64) {
        if self.is_placed() {
            self.position = Some(self.get_position());
        } else if let Some(pos) = self.position {
            if p < pos {
                self.position = Some(p);
            }
        } else {
            self.position = Some(p);
        }
    }

    //fp has_position
    /// Return true if the node has a position
    pub fn has_position(&self) -> bool {
        self.forced_position.is_some() || self.placed_position.is_some() || self.position.is_some()
    }

    //fp is_placed
    /// Return true if the node is placed or forced
    pub fn is_placed(&self) -> bool {
        self.forced_position.is_some() || self.placed_position.is_some()
    }

    //fp get_position
    /// Get the position of the [Node]
    pub fn get_position(&self) -> f64 {
        if let Some(p) = self.forced_position {
            p
        } else if let Some(p) = self.placed_position {
            p
        } else {
            assert!(self.position.is_some());
            self.position.unwrap()
        }
    }

    //zz All done
}
