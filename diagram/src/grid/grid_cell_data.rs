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

@file    grid.rs
@brief   Grid layout
 */

//a Imports
use std::collections::{HashSet, HashMap};

use super::{GridDimension};

//a Global constants for debug
const DEBUG_CELL_DATA      : bool = 1 == 0;

//a Internal types
//ti GridCellDataEntry
/// This holds the desired placement of actual data with overlapping GridData in an array (the GridCellData
/// structure)
#[derive(Debug, Clone)]
pub struct GridCellDataEntry {
    /// start is the index of the left-hand edge of the cell in the
    /// grid dimension
    pub start : usize,
    /// end is the index of the right-hand edge of the cell in the
    /// grid dimension
    pub end   : usize,
    /// size is the desired size, or actual size post-expansion
    pub size  : f64,
}

//ii GridCellDataEntry
impl GridCellDataEntry {

    //fp new
    pub fn new(start:usize, end:usize, size:f64) -> Self {
        Self {start, end, size}
    }
}

//it Display for GridCellDataEntry
impl std::fmt::Display for GridCellDataEntry {

    //mp fmt - format a GridCellDataEntry
    /// Display the `GridCellDataEntry' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}->{}:{}", self.start, self.end, self.size)
    }

    //zz All done
}

//a Resolver
struct Link {
    start    : usize,
    end      : usize,
    min_size : f64,
    growth   : Option<f64>,
}

impl Link {
    fn new(cde:&GridCellDataEntry) -> Self {
        Self {
            start    : cde.start,
            end      : cde.end,
            min_size : cde.size,
            growth   : None,
        }
    }
    fn union(&mut self, cde:&GridCellDataEntry) {
        if cde.size > self.min_size {
            self.min_size = cde.size;
        }
    }
}

//ti Node
/// A node in the grid row - in essence a border between cells
///
/// This can be linked to other cells through multiple content
/// (GridCellDataEntry); this is handled by the [Link] references,
/// which refer to the other end of the link
struct Node {
    /// The node id from the client structure
    id : usize,
    /// The position of the node
    position  : Option<f64>,
    /// The 'start's of links for which this node is the end
    link_starts : Vec<usize>,
    /// The 'end's of links for which this node is the start
    link_ends : Vec<usize>,
}
impl Node {
    fn new(id:usize) -> Self {
        Self { id,
               position : None,
               link_starts : Vec::new(),
               link_ends   : Vec::new(),
        }
    }
    fn new_with_link_from(id:usize, s:usize) -> Self {
        let mut n = Self::new(id);
        n.add_startpoint(s);
        n
    }
    fn new_with_link_to(id:usize, e:usize) -> Self {
        let mut n = Self::new(id);
        n.add_endpoint(e);
        n
    }
    fn add_startpoint(&mut self, s:usize) {
        self.link_starts.push(s);
    }
    fn add_endpoint(&mut self, e:usize) {
        self.link_ends.push(e);
    }
}

//tp Resolver
struct Resolver {
    /// Node structures indexed by node id
    nodes  : HashMap<usize, Node>,
    /// Links squashed
    links  : HashMap<(usize, usize), Link>,
    /// The node ids that are never the 'end' of a link
    roots  : Vec<usize>,
    /// Order in which nodes can be resolved
    node_resolution_order : Vec<usize>,
}

impl Resolver {
    //fp new
    fn new(data : &Vec<GridCellDataEntry>) -> Self {
        let mut links : HashMap<(usize, usize), Link> = HashMap::new();
        let mut roots = HashSet::new();
        for d in data.iter() {
            roots.insert(d.start);
            let key = (d.start, d.end);
            if let Some(mut link) = links.get_mut(&key) {
                link.union(d);
            } else {
                links.insert(key, Link::new(d));
            }
        }
        let mut nodes : HashMap<usize, Node>= HashMap::new();
        for (s,e) in links.keys() {
            if let Some(mut node) = nodes.get_mut(&s) {
                node.add_endpoint(*e);
            } else {
                nodes.insert(*s, Node::new_with_link_to(*s, *e));
            }
            if let Some(mut node) = nodes.get_mut(&e) {
                node.add_startpoint(*s);
            } else {
                nodes.insert(*e, Node::new_with_link_from(*e, *s));
            }
            roots.remove(e);
        }
        let roots = roots.into_iter().collect();
        let node_resolution_order = Self::create_node_resolution_order(&nodes);
        Self { nodes, links, roots, node_resolution_order }
    }
    
    //fi create_node_resolution_order
    fn create_node_resolution_order(nodes:&HashMap<usize, Node>) -> Vec<usize> {
        let mut node_resolution_order  = Vec::new();
        let mut unresolved_link_counts = HashMap::new();
        let mut node_stack             = Vec::new();
        for (node_id, node) in nodes.iter() {
            let num = node.link_starts.len();
            unresolved_link_counts.insert(*node_id, num);
            if num == 0 {
                node_stack.push(*node_id);
            }
        }
        while let Some(node_id) = node_stack.pop() {
            // unresolved_link_counts.get(node_id) must be 0
            node_resolution_order.push(node_id);
            for e in &nodes.get(&node_id).unwrap().link_ends {
                let count = unresolved_link_counts.get_mut(e).unwrap();
                *count -= 1;
                if *count == 0 {
                    node_stack.push(*e);
                }
            }
        }
        node_resolution_order
    }

}

//a GridCellData
//tp GridCellData
/// This structure holds the positions and sizes of one dimension of
/// all the elements in a grid
#[derive(Debug)]
pub struct GridCellData {
    /// The declared start/end link ids and min size between them
    ///
    /// There may be more than one entry for the same start/end link
    /// id. The max of the min sizes is what is required
    data   : Vec<GridCellDataEntry>,
}

//ip GridCellData
impl GridCellData {

    //fp new
    pub fn new() -> Self {
        let data = Vec::new();
        Self { data }
    }

    //fp add
    pub fn add(&mut self, start:usize, end:usize, size:f64) {
        let size = if size < 0. {0.} else {size};
        self.data.push(GridCellDataEntry::new(start, end, size));
    }

    //fp create_grid_dimension
    /// Destructively create a grid dimension
    pub fn create_grid_dimension(&mut self) -> GridDimension {
        // self.create_link_resolution_order();
        // let mut gd = GridDimension::new(self.start, self.end);
        let mut gd = GridDimension::new(0,1);
        
        if DEBUG_CELL_DATA { println!("Generate cell positions of cell data {:?}", self.data); }

        if self.data.len() == 0 {return gd;}

        if DEBUG_CELL_DATA { println!("Sorted cell data {:?}", self.data); }
/*
        let mut index = 0;
        let mut column  = clone.start;
        loop {
            if DEBUG_CELL_DATA { println!("start loop: column {} at index {}",column,index); }
            if let Some((next_index, next_col, size)) = clone.remove_next_region(index, column) {
                gd.add(column,next_col,size);
                index  = next_index;
                column = next_col;
            } else {
                break;
            }
        }
        if clone.end > column { gd.add(column, clone.end, 0.); }
*/
        gd.calculate_positions(0.,0.);
        if DEBUG_CELL_DATA { println!("Generated cell positions {:?}\n for cell data {:?}", gd, self.data); }
        gd
    }

    //zz All done
}

//ip Display for GridCellData
impl std::fmt::Display for GridCellData {
    //mp fmt - format for display
    /// Display
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for d in &self.data {
            write!(f, "{}, ", d)?;
        }
        Ok(())
    }

    //zz All done
}

