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

@file    resolver.rs
@brief   Grid layout resolver using springs
 */

//a To do

//a Imports
use super::{EquationSet, Link, Node, NodeId};
use geometry::Range;
use std::collections::{HashMap, HashSet};

//a Global constants for debug
const DEBUG_CELL_DATA: bool = 1 == 0;

//a Resolver
//tp Resolver
/// This provides a means to resolve the requirements for a grid
/// dimension
///
/// It is built from the requisite link data; this should define a set
/// of nodes with links, which form a DAG (possibly with more than one
/// 'root' node).
///
/// From the DAG the *minimum* positions of each node can be deduced,
/// by traversing from roots (which have position 0) to new nodes,
/// where each step to a new node requires that *all* the predecessor
/// link-node to that node have already been traversed (and hence
/// assigned positions).
#[derive(Debug)]
pub struct Resolver<N: NodeId> {
    /// All the node ids used
    node_ids: Vec<N>,
    /// Node structures indexed by node id
    nodes: HashMap<N, Node<N>>,
    /// Links squashed
    links: HashMap<(N, N), Link<N>>,
    /// The node ids that are never the 'end' of a link
    roots: Vec<N>,
    /// The node ids that are never the 'start' of a link
    leaves: Vec<N>,
    /// Order in which nodes can be resolved
    node_resolution_order: Vec<N>,
    /// Unresoved nodes
    unresolved_nodes: HashSet<N>,
}

//ip Resolver
impl<N: NodeId> Resolver<N> {
    //fp none
    /// Create a new resolver with no data
    pub fn none() -> Self {
        let node_ids = Vec::new();
        let nodes = HashMap::new();
        let links = HashMap::new();
        let roots = Vec::new();
        let leaves = Vec::new();
        let node_resolution_order = Vec::new();
        Self {
            node_ids,
            nodes,
            links,
            roots,
            leaves,
            node_resolution_order,
            unresolved_nodes,
        }
    }

    //fp new
    /// Create a new resolver from the provided cell data
    pub fn new(link_data: &mut dyn Iterator<Item = (N, N, f64)>) -> Self {
        let mut links: HashMap<(N, N), Link<N>> = HashMap::new();
        let mut roots = HashSet::new();
        let mut leaves = HashSet::new();
        for (start, end, size) in link_data {
            roots.insert(start);
            leaves.insert(end);
            let key = (start, end);
            if let Some(link) = links.get_mut(&key) {
                link.union(size);
            } else {
                links.insert(key, Link::new(start, end, size));
            }
        }
        let mut nodes: HashMap<N, Node<N>> = HashMap::new();
        let mut unresolved_nodes = HashSet::new();
        let mut node_ids = Vec::new();
        for (s, e) in links.keys() {
            if let Some(node) = nodes.get_mut(s) {
                node.add_endpoint(*e);
            } else {
                let index = node_ids.len();
                nodes.insert(*s, Node::new_with_link_to(index, *e));
                node_ids.push(*s);
                unresolved_nodes.insert(*s);
            }
            if let Some(node) = nodes.get_mut(e) {
                node.add_startpoint(*s);
            } else {
                let index = node_ids.len();
                node_ids.push(*e);
                nodes.insert(*e, Node::new_with_link_from(index, *s));
                unresolved_nodes.insert(*e);
            }
            leaves.remove(s);
            roots.remove(e);
        }
        let roots = roots.into_iter().collect();
        let leaves = leaves.into_iter().collect();
        let node_resolution_order = Vec::new();
        let mut s = Self {
            node_ids,
            nodes,
            links,
            roots,
            leaves,
            node_resolution_order,
            unresolved_nodes,
        };
        s.place_roots(0.);
        s
    }

    //mp set_growth_data
    pub fn set_growth_data(&mut self, start: N, end: N, growth: f64) -> Result<(), String> {
        let reachable = self.reachable_nodes(start);
        let predecessor = self.predecessor_nodes(end);
        let intersection: HashSet<N> = reachable.intersection(&predecessor).map(|x| *x).collect();
        for node_id in intersection.iter() {
            for e in &self.nodes[node_id].link_ends {
                if intersection.contains(&e) {
                    self.links
                        .get_mut(&(*node_id, *e))
                        .unwrap()
                        .set_growth(growth);
                }
            }
        }
        Ok(())
    }

    //mp force_node
    pub fn force_node(&mut self, node: N, position: Option<f64>) {
        self.nodes.get_mut(&node).unwrap().forced_position(position);
    }

    //mp place_node
    pub fn place_node(&mut self, node: N, position: Option<f64>) {
        self.nodes.get_mut(&node).unwrap().placed_position(position);
    }

    //mp clear_node_placements
    pub fn clear_node_placements(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            node.placed_position(None);
        }
        self.create_node_resolution_order();
    }

    //mp assign_min_positions
    /// Assign the minimum positions for each node
    pub fn assign_min_positions(&mut self) {
        self.create_node_resolution_order();
        for (_, n) in self.nodes.iter_mut() {
            n.reset_position();
        }
        for n in &self.node_resolution_order {
            let node = self.nodes.get_mut(n).unwrap();
            let p = node.get_position();
            let mut end_updates = Vec::new();
            let mut start_updates = Vec::new();
            for e in &node.link_starts {
                let link = self.links.get(&(*e, *n)).unwrap();
                start_updates.push((*e, p - link.min_size));
            }
            for e in &node.link_ends {
                let link = self.links.get(&(*n, *e)).unwrap();
                end_updates.push((*e, p + link.min_size));
            }
            drop(node);
            for (start, max_pos) in start_updates {
                self.nodes
                    .get_mut(&start)
                    .unwrap()
                    .set_max_position(max_pos);
            }
            for (end, min_pos) in end_updates {
                self.nodes.get_mut(&end).unwrap().set_min_position(min_pos);
            }
        }
    }

    //fi reachable_nodes
    /// Find the set of all `node_id`s reachable from a node_id
    fn reachable_nodes(&self, node_id: N) -> HashSet<N> {
        // println!("reachable_nodes of {} self {:?}",node_id, self);
        let mut to_do = Vec::new();
        let mut result = HashSet::new();
        result.insert(node_id);
        to_do.push(node_id);
        while let Some(node_id) = to_do.pop() {
            println!("node_id {}", node_id);
            for e in &self.nodes[&node_id].link_ends {
                if result.insert(*e) {
                    // was not in the set before
                    to_do.push(*e);
                }
            }
        }
        result
    }

    //fi predecessor_nodes
    /// Find the set of all `node_id`s that can reach from a node_id
    fn predecessor_nodes(&self, node_id: N) -> HashSet<N> {
        let mut to_do = Vec::new();
        let mut result = HashSet::new();
        result.insert(node_id);
        to_do.push(node_id);
        while let Some(node_id) = to_do.pop() {
            for e in &self.nodes[&node_id].link_starts {
                if result.insert(*e) {
                    // was not in the set before
                    to_do.push(*e);
                }
            }
        }
        result
    }

    //fi create_node_resolution_order
    /// Create a node_resolution_order from a set of placed nodes
    ///
    /// Create a count of unresolved links for every node
    ///
    /// A placed node is explicitly resolved;
    ///
    /// Returns the count of unresolved nodes
    pub fn create_node_resolution_order(&mut self) -> usize {
        if DEBUG_CELL_DATA {
            println!("Create node resolution order");
        }
        let mut node_stack: Vec<N> = Vec::new();
        let mut unresolved_link_starts = HashMap::new();
        let mut unresolved_link_ends = HashMap::new();
        for (node_id, node) in self.nodes.iter() {
            if node.is_placed() {
                if DEBUG_CELL_DATA {
                    println!("Placed {} {:?}", node_id, node);
                }
                node_stack.push(*node_id);
            }
            unresolved_link_starts.insert(*node_id, node.link_starts.len());
            unresolved_link_ends.insert(*node_id, node.link_ends.len());
        }

        let mut resolved_nodes = HashSet::new();
        let mut node_resolution_order = Vec::new();
        while let Some(node_id) = node_stack.pop() {
            if resolved_nodes.insert(node_id) {
                let node = self.nodes.get(&node_id).unwrap();
                if DEBUG_CELL_DATA {
                    println!("add {} {:?}", node_id, node);
                }
                node_resolution_order.push(node_id);
                for e in &node.link_ends {
                    let count = unresolved_link_starts.get_mut(e).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        node_stack.push(*e);
                    }
                }
                for e in &node.link_starts {
                    let count = unresolved_link_ends.get_mut(e).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        node_stack.push(*e);
                    }
                }
            }
        }
        self.unresolved_nodes.clear();
        for (node_id, node) in self.nodes.iter() {
            if !resolved_nodes.contains(node_id) {
                self.unresolved_nodes.insert(*node_id);
                println!("Unresolved node in resolution order {} {:?}", node_id, node);
            }
        }
        self.node_resolution_order = node_resolution_order;
        self.unresolved_nodes.len()
    }

    //mp create_energy_matrix
    /// Create the [EquationSet] for the energies of the springs
    ///
    /// For all links that have an elasticity (growth), add them in
    ///
    /// All roots are fixed to the left-hand edge
    ///
    /// All leaves are fixed to the right-hand edge
    ///
    /// If any node is unconstrained (which should not happen!) then
    /// force it to its min position
    pub fn create_energy_matrix(&self) -> EquationSet {
        let num_nodes = self.node_ids.len();
        let mut eqns = EquationSet::new(num_nodes);
        for (se, link) in self.links.iter() {
            if let Some(growth) = link.growth {
                let (s, e) = se;
                let s = self.nodes[s].index;
                let e = self.nodes[e].index;
                let length = link.min_size;
                eqns.add_growth_link(s, e, length, growth);
            }
        }
        for (_, node) in self.nodes.iter() {
            if node.is_placed() {
                println!("Placed {} at {}", node.index, node.get_position());
                eqns.force_value(node.index, node.get_position());
            }
        }
        for i in 0..num_nodes {
            if eqns.row_is_zero(i) {
                let node_id = self.node_ids[i];
                println!(
                    "Row {} = {} is zero, force to {}",
                    i,
                    node_id,
                    self.nodes[&node_id].get_position()
                );
                eqns.force_value(i, self.nodes[&node_id].get_position());
            }
        }
        eqns
    }

    //mp minimize_energy
    /// Calculate the positions that minimize the energy
    pub fn minimize_energy(&mut self) -> Result<(), String> {
        if self.node_ids.is_empty() {
            return Ok(());
        }
        let mut eqns = self.create_energy_matrix();
        eqns.solve()?;
        for (i, position) in eqns.results() {
            let node_id = self.node_ids[i];
            self.nodes
                .get_mut(&node_id)
                .unwrap()
                .set_min_position(*position);
        }
        Ok(())
    }

    //mp find_bounds
    /// Find the min and max of all nodes with positions
    pub fn find_bounds(&self) -> Range {
        let mut min_max = None;
        for (_, node) in self.nodes.iter() {
            if node.has_position() {
                let p = node.get_position();
                if let Some((min, max)) = min_max {
                    if p < min {
                        min_max = Some((p, max));
                    } else if p > max {
                        min_max = Some((min, p));
                    }
                } else {
                    min_max = Some((p, p));
                }
            }
        }
        if let Some((min, max)) = min_max {
            Range::new(min, max)
        } else {
            Range::none()
        }
    }

    //mp get_node_index
    #[cfg(test)]
    pub fn get_node_index(&self, node: N) -> usize {
        self.nodes[&node].index
    }

    //mp has_node
    pub fn has_node(&self, node: &N) -> bool {
        self.nodes.contains_key(node)
    }

    //mp get_node_position
    pub fn get_node_position(&self, node: N) -> f64 {
        self.nodes[&node].get_position()
    }

    //mp get_edge_nodes
    pub fn get_edge_nodes(&self, epsilon: f64) -> Vec<(N, bool)> {
        let bounds = self.find_bounds();
        let mut result = Vec::new();
        if !bounds.is_none() {
            let epsilon = epsilon.min(bounds.size() / 2.0);
            for (node_id, node) in self.nodes.iter() {
                if node.has_position() {
                    let p = node.get_position();
                    if p <= bounds.min + epsilon {
                        result.push((*node_id, false));
                    } else if p >= bounds.max - epsilon {
                        result.push((*node_id, true));
                    }
                }
            }
        }
        println!("Got edge nodes {:?}", result);
        result
    }

    //mp place_edge_nodes
    pub fn place_edge_nodes(
        &mut self,
        edge_nodes: Vec<(N, bool)>,
        min: Option<f64>,
        max: Option<f64>,
    ) {
        for (node_id, is_max) in edge_nodes {
            if is_max {
                self.place_node(node_id, max);
            } else {
                self.place_node(node_id, min);
            }
        }
    }

    //mp borrow_roots
    pub fn borrow_roots(&self) -> &Vec<N> {
        &self.roots
    }

    //mp borrow_resolution_order
    #[allow(dead_code)]
    pub fn borrow_resolution_order(&self) -> &Vec<N> {
        &self.node_resolution_order
    }

    //mp place_roots_to_resolve
    pub fn place_roots_to_resolve(&mut self, pos: f64) -> bool {
        while !self.unresolved_nodes.is_empty() {
            let mut placed = false;
            for node_id in &self.roots {
                if self.unresolved_nodes.contains(node_id) {
                    placed = true;
                    self.nodes
                        .get_mut(node_id)
                        .unwrap()
                        .placed_position(Some(pos));
                    break;
                }
            }
            if !placed {
                break;
            }
            self.create_node_resolution_order();
        }
        self.unresolved_nodes.is_empty()
    }

    //mp place_leaves_to_resolve
    pub fn place_leaves_to_resolve(&mut self, pos: f64) -> bool {
        while !self.unresolved_nodes.is_empty() {
            let mut placed = false;
            for node_id in &self.leaves {
                if self.unresolved_nodes.contains(node_id) {
                    placed = true;
                    self.nodes
                        .get_mut(node_id)
                        .unwrap()
                        .placed_position(Some(pos));
                    break;
                }
            }
            if !placed {
                break;
            }
            self.create_node_resolution_order();
        }
        self.unresolved_nodes.is_empty()
    }

    //zz All done
}

//a Test
//mt Test for GridPlacement
#[cfg(test)]
mod test_resolver {
    use super::*;
    //ft test_0
    #[test]
    fn test_0() {
        let data = vec![(0, 100, 10.), (100, 50, 10.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        assert_eq!(res.borrow_roots(), &vec![0]);
        assert_eq!(res.borrow_resolution_order(), &vec![0, 100, 50]);
        res.assign_min_positions();
        assert_eq!(res.get_node_position(0), 0.);
        assert_eq!(res.get_node_position(100), 10.);
        assert_eq!(res.get_node_position(50), 20.);
        res.create_energy_matrix();
    }
    //ft test_1
    #[test]
    fn test_1() {
        let data = vec![(0, 100, 10.), (100, 50, 10.), (100, 250, 5.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        assert_eq!(res.borrow_roots(), &vec![0]);
        res.place_roots_to_resolve(0.);
        // assert_eq!(res.borrow_resolution_order(), &vec![0,100,50,250]);
        res.assign_min_positions();
        assert_eq!(res.get_node_position(0), 0.);
        assert_eq!(res.get_node_position(100), 10.);
        assert_eq!(res.get_node_position(250), 15.);
        assert_eq!(res.get_node_position(50), 20.);
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(20.));
        res.create_energy_matrix();
    }
    //fi approx_eq
    fn approx_eq(x: f64, e: f64) {
        assert!((x - e).abs() < 0.001, "{} should be approx {}", x, e);
    }

    //ft test_2
    #[test]
    fn test_2() {
        let data = vec![(0, 1, 10.), (1, 2, 10.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        assert_eq!(res.borrow_roots(), &vec![0]);
        assert_eq!(res.borrow_resolution_order(), &vec![0, 1, 2]);
        res.set_growth_data(0, 1, 1.).unwrap();
        res.set_growth_data(1, 2, 0.00001).unwrap();
        res.assign_min_positions();
        assert_eq!(res.get_node_position(0), 0.);
        assert_eq!(res.get_node_position(1), 10.);
        assert_eq!(res.get_node_position(2), 20.);
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(30.));
        let mut eqns = res.create_energy_matrix();
        println!("energy {}", eqns);
        eqns.invert().unwrap();
        println!("inverted energy {}", eqns);
        res.assign_min_positions();
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(30.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 20.);
        approx_eq(res.get_node_position(2), 30.);
    }
    //ft test_3
    #[test]
    fn test_3() {
        let data = vec![(0, 1, 10.), (1, 2, 10.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        assert_eq!(res.borrow_roots(), &vec![0]);
        assert_eq!(res.borrow_resolution_order(), &vec![0, 1, 2]);
        res.set_growth_data(0, 2, 1.).unwrap();
        res.assign_min_positions();
        assert_eq!(res.get_node_position(0), 0.);
        assert_eq!(res.get_node_position(1), 10.);
        assert_eq!(res.get_node_position(2), 20.);
        let eqns = res.create_energy_matrix();
        println!("energy {}", eqns);
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(30.));
        let mut eqns = res.create_energy_matrix();
        println!("energy {}", eqns);
        eqns.invert().unwrap();
        println!("inverted energy {}", eqns);

        println!("Clear node placements");
        res.clear_node_placements();
        println!("Place roots");
        res.place_roots_to_resolve(0.);
        println!("Assign min positions");
        res.assign_min_positions();
        println!("Place edge nodes streteched");
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(30.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 15.);
        approx_eq(res.get_node_position(2), 30.);
    }
    //ft test_4
    #[test]
    fn test_4() {
        // Even growth of the extra 10.
        let data = vec![(0, 1, 10.), (1, 2, 10.), (1, 2, 10.), (2, 3, 10.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        res.set_growth_data(0, 3, 1.).unwrap();
        res.assign_min_positions();
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(40.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 13.333333);
        approx_eq(res.get_node_position(2), 26.666666);
        approx_eq(res.get_node_position(3), 40.);
    }
    //ft test_5
    #[test]
    fn test_5() {
        // Dbl growth of the extra 10. in 1->2
        let data = vec![(0, 1, 10.), (1, 2, 10.), (1, 2, 10.), (2, 3, 10.)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        res.set_growth_data(0, 3, 1.).unwrap();
        res.set_growth_data(1, 2, 2.).unwrap();
        res.assign_min_positions();
        res.place_edge_nodes(res.get_edge_nodes(1.0E-3), Some(0.), Some(40.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 12.5);
        approx_eq(res.get_node_position(2), 27.5);
        approx_eq(res.get_node_position(3), 40.);
    }
    //ft test_6
    #[test]
    fn test_6() {
        // Dbl growth of the extra 40. in 1->2
        let data = vec![
            (0, 1, 0.0001),
            (1, 2, 0.0001),
            (1, 2, 0.0001),
            (2, 3, 0.0001),
        ];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        res.set_growth_data(0, 3, 1.).unwrap();
        res.set_growth_data(1, 2, 2.).unwrap();
        println!("Assign min positions");
        res.assign_min_positions();
        println!("Place edge nodes");
        res.place_edge_nodes(res.get_edge_nodes(1.0E-7), Some(0.), Some(40.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 10.);
        approx_eq(res.get_node_position(2), 30.);
        approx_eq(res.get_node_position(3), 40.);
    }
    //ft test_7
    #[test]
    fn test_7() {
        // Dbl growth of the extra 40. in 1->2, no effect having 0->3 connection
        let data = vec![
            (0, 1, 0.0001),
            (1, 2, 0.0001),
            (2, 3, 0.0001),
            (0, 2, 0.0001),
        ];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.place_roots_to_resolve(0.);
        res.set_growth_data(0, 1, 1.).unwrap();
        res.set_growth_data(1, 2, 2.).unwrap();
        res.set_growth_data(2, 3, 1.).unwrap();
        res.assign_min_positions();
        res.place_edge_nodes(res.get_edge_nodes(1.0E-7), Some(0.), Some(40.));
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 0.);
        approx_eq(res.get_node_position(1), 10.);
        approx_eq(res.get_node_position(2), 30.);
        approx_eq(res.get_node_position(3), 40.);
    }
    //ft test_8
    #[test]
    fn test_8() {
        // Test some placement
        let data = vec![(0, 1, 10.0), (1, 2, 10.0), (2, 3, 10.0), (0, 2, 0.0001)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        assert_eq!(res.create_node_resolution_order(), 4);
        res.set_growth_data(0, 1, 1.).unwrap();
        res.set_growth_data(1, 2, 2.).unwrap();
        res.set_growth_data(2, 3, 1.).unwrap();
        res.place_leaves_to_resolve(40.);
        assert_eq!(res.create_node_resolution_order(), 0);
        res.assign_min_positions();
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 10.);
        approx_eq(res.get_node_position(1), 20.);
        approx_eq(res.get_node_position(2), 30.);
        approx_eq(res.get_node_position(3), 40.);
    }
    //ft test_9
    #[test]
    fn test_9() {
        // Test placement of a middle value
        // Note that this requires the middle value to not be in a loop - so node 2 would not work
        let data = vec![(0, 1, 10.0), (1, 2, 10.0), (2, 3, 10.0), (1, 3, 0.0001)];
        let mut res = Resolver::new(&mut data.iter().map(|x| (x.0, x.1, x.2)));
        res.clear_node_placements();
        res.place_node(1, Some(20.));
        res.set_growth_data(0, 1, 1.).unwrap();
        res.set_growth_data(1, 2, 2.).unwrap();
        res.set_growth_data(2, 3, 1.).unwrap();
        assert_eq!(res.create_node_resolution_order(), 0);
        res.minimize_energy().unwrap();
        approx_eq(res.get_node_position(0), 10.);
        approx_eq(res.get_node_position(1), 20.);
        approx_eq(res.get_node_position(2), 30.);
        approx_eq(res.get_node_position(3), 40.);
    }
}
