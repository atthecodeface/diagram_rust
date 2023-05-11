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
use crate::grid::{GridCellDataEntry, GridData, Resolver};
use crate::Range;

//a Global constants for debug
const DEBUG_GRID_PLACEMENT: bool = 1 == 1;

//a Public GridPlacement type
//tp GridPlacement
/// This contains a vector of the placement of each element within a grid dimension
/// The cell_positions contains an order vector of <dimension index,posn>, where the dimension indices increase
/// through the vector
/// Structure for a grid - a list of start, span, and height of each cell *)
#[derive(Debug)]
pub struct GridPlacement {
    cell_data: Vec<GridCellDataEntry>,
    resolver: Resolver<usize>,
    growth_data: Vec<(usize, usize, f64)>,
    desired_range: Range,
    size: f64,
}

//ip GridPlacement
impl GridPlacement {
    //fp new
    pub fn new() -> Self {
        let cell_data = Vec::new();
        let resolver = Resolver::none();
        let growth_data = Vec::new();
        Self {
            cell_data,
            resolver,
            growth_data,
            desired_range: Range::none(),
            size: 0.,
        }
    }

    //mp add_cell
    pub fn add_cell(&mut self, eref: &str, start: usize, end: usize, size: f64) {
        if DEBUG_GRID_PLACEMENT {
            println!("Add cell {} {} {} {}", eref, start, end, size);
        }
        assert!(end != start);
        let size = if size < 0. { 0. } else { size };
        self.cell_data
            .push(GridCellDataEntry::new(start, end, size));
    }

    //mp add_cell_data
    /// Used to add growth of cell data
    pub fn add_cell_data(&mut self, growth_data: &Vec<GridData>) {
        for gd in growth_data {
            match gd {
                GridData::Width(start, end, size) => {
                    self.cell_data
                        .push(GridCellDataEntry::new(*start, *end, *size));
                }
                GridData::Growth(start, end, size) => {
                    self.growth_data.push((*start, *end, *size));
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    //mp get_desired_geometry
    /// calculates the positions of the elements in the grid given a center and expansion
    ///
    /// For a desired geometry this should be invoked with 0. for both arguments
    ///
    /// Given an actual size, centered on a value, expand the grid as required, and translate so that it is centered on the value.
    pub fn get_desired_geometry(&mut self) -> Range {
        self.resolver = Resolver::new(&mut self.cell_data.iter().map(|x| (x.start, x.end, x.size)));
        for (start, end, growth) in &self.growth_data {
            if self.resolver.has_node(start) && self.resolver.has_node(end) {
                match self.resolver.set_growth_data(*start, *end, *growth) {
                    Err(x) => {
                        eprintln!(
                            "Warning: Could not set growth data {} {} {}: {}",
                            *start, *end, *growth, x
                        );
                    }
                    _ => (),
                }
            }
        }
        self.resolver.place_roots_to_resolve(0.);
        self.resolver.assign_min_positions();
        self.desired_range = self.resolver.find_bounds();
        self.size = self.desired_range.size();
        // Centre on the origin
        self.desired_range -= self.size * 0.5;
        self.desired_range
    }

    //mp calculate_positions
    /// calculates the positions of the elements in the grid given a max size permitted, center and expansion
    ///
    /// The desired size will have been calculated before
    pub fn calculate_positions(&mut self, size: f64, center: f64, expansion: f64) {
        let extra_space = size - self.size;
        let expanded_space = expansion * extra_space;
        let final_size = self.size + expanded_space;
        self.resolver.clear_node_placements();
        let min = center - final_size * 0.5;
        let max = center + final_size * 0.5;
        self.resolver.place_roots_to_resolve(min);
        self.resolver.assign_min_positions();
        self.resolver
            .place_edge_nodes(self.resolver.get_edge_nodes(1.0E-7), Some(min), Some(max));
        match self.resolver.minimize_energy() {
            Err(x) => {
                eprintln!("Warning: failed to resolve cleanly: {}", x);
            }
            _ => (),
        }
        self.size = self.resolver.find_bounds().size();
    }

    //mp get_span
    /// Find the span of a start/number of grid positions
    pub fn get_span(&self, start: usize, end: usize) -> (f64, f64) {
        if DEBUG_GRID_PLACEMENT {
            println!("Get span {} {}", start, end);
        }
        assert!(end != start);
        let start_posn = self.resolver.get_node_position(start);
        let end_posn = self.resolver.get_node_position(end);
        if DEBUG_GRID_PLACEMENT {
            println!(
                "Got span for {} {} to be {} {}",
                start, end, start_posn, end_posn
            );
        }
        (start_posn, end_posn)
    }

    //mp get_size
    /// Get the size of the whole placement
    pub fn get_size(&self) -> f64 {
        self.size
    }

    //mp get_position
    /// Get the position of all the references
    pub fn get_position(&self, n: usize) -> Option<f64> {
        if self.resolver.has_node(&n) {
            let pos = self.resolver.get_node_position(n);
            if DEBUG_GRID_PLACEMENT {
                println!("get_pos - {} : {}", n, pos);
            }
            Some(pos)
        } else {
            None
        }
    }

    // Display with an indent of indent_str plus six spaces
    pub fn display(&self, _indent_str: &str) {
        // println!("{}      {}", indent_str, self.cell_data);
        // println!("{}      {}", indent_str, self.grid_dimension);
        // println!("{}      {}", indent_str, self.growth_data);
    }

    //zz All done
}

//mt Test for GridPlacement
#[cfg(test)]
mod test_grid_placement {
    use super::*;
    //fi check_positions
    fn check_positions(cp: &GridPlacement, exp: &Vec<(usize, f64)>) {
        for (r, e) in exp {
            let p = cp.get_position(*r);
            assert!(p.is_some(), "Expected ref {} to have a position", r);
            let p = p.unwrap();
            assert!(
                (p - e).abs() < 1E-8,
                "For {} Expected position {} got grid {}",
                r,
                e,
                p
            );
        }
    }
    //ft test_0
    // #[test]
    // This test is old and does not work with current system
    #[allow(dead_code)]
    fn test_0() {
        let mut gp = GridPlacement::new();
        gp.add_cell("", 0, 4, 4.);
        gp.add_cell("", 4, 6, 2.);
        gp.calculate_positions(0., 0., 0.);
        assert_eq!(gp.get_size(), 6.);
        check_positions(&gp, &vec![(0, -3.), (4, 1.), (6, 3.)]);
        assert_eq!(gp.get_span(0, 4), (-3., 1.));
        assert_eq!(gp.get_span(4, 6), (1., 3.));
    }
    //ft test_1
    // #[test]
    // This test is old and does not work with current system
    #[allow(dead_code)]
    fn test_1() {
        let mut gp = GridPlacement::new();
        gp.add_cell("", 0, 4, 4.);
        gp.add_cell("", 4, 6, 2.);
        gp.add_cell_data(&vec![GridData::new_growth(2, 4, 1.)]);

        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size() + 2., 0., 1.);
        assert_eq!(gp.get_size(), 8.);
        check_positions(&gp, &vec![(0, -4.), (2, -2.), (4, 2.), (6, 4.)]);
        assert_eq!(gp.get_span(0, 4), (-4., 2.));
        assert_eq!(gp.get_span(4, 6), (2., 4.));
    }
    //ft test_2
    // #[test]
    // This test is old and does not work with current system
    #[allow(dead_code)]
    fn test_2() {
        let mut gp = GridPlacement::new();
        gp.add_cell("", 0, 10, 10.);
        gp.add_cell_data(&vec![
            GridData::new_growth(0, 2, 1.),
            GridData::new_growth(2, 8, 0.),
            GridData::new_growth(8, 10, 1.),
        ]);

        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size() + 4., 7., 1.);
        assert_eq!(gp.get_size(), 14.);
        check_positions(&gp, &vec![(0, 0.), (2, 4.), (8, 10.), (10, 14.)]);
        assert_eq!(gp.get_span(0, 2), (0., 4.));
        assert_eq!(gp.get_span(2, 8), (4., 10.));
        assert_eq!(gp.get_span(8, 10), (10., 14.));
    }
}
