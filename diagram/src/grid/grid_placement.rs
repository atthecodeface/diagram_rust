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
use super::{GridCellDataEntry, GridData, NodeId, Resolver};
use geometry::Range;

//a Global constants for debug
const DEBUG_GRID_PLACEMENT: bool = 1 == 0;

//a Public GridPlacement type
//tp GridPlacement
/// This contains a vector of the placement of each element within a grid dimension
/// The cell_positions contains an order vector of <dimension index,posn>, where the dimension indices increase
/// through the vector
/// Structure for a grid - a list of start, span, and height of each cell *)
#[derive(Debug)]
pub struct GridPlacement {
    refs: Vec<(isize, String)>,
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
        let refs = Vec::new();
        let cell_data = Vec::new();
        let resolver = Resolver::none();
        let growth_data = Vec::new();
        Self {
            refs,
            cell_data,
            resolver,
            growth_data,
            desired_range: Range::none(),
            size: 0.,
        }
    }

    //mi find_ref_of_isize
    fn find_ref_of_isize(&self, x: isize) -> Option<usize> {
        let xs = format!("{}", x);
        for (n, s) in self.refs.iter().enumerate() {
            if s.1 == xs.as_str() {
                return Some(n);
            }
        }
        None
    }

    //mi ref_of_isize
    fn ref_of_isize(&mut self, x: isize) -> usize {
        if let Some(n) = self.find_ref_of_isize(x) {
            n
        } else {
            let xs = format!("{}", x);
            self.refs.push((x, xs));
            self.refs.len() - 1
        }
    }

    //mp add_cell
    pub fn add_cell(&mut self, start: isize, end: isize, size: f64) {
        let start = self.ref_of_isize(start);
        let end = self.ref_of_isize(end);
        assert!(end != start);
        let size = if size < 0. { 0. } else { size };
        self.cell_data
            .push(GridCellDataEntry::new(start, end, size));
    }

    //mp add_growth_data
    /// Used to add growth of cell data
    pub fn add_growth_data(&mut self, growth_data: &Vec<GridData>) {
        for gd in growth_data {
            let start = self.ref_of_isize(gd.start);
            let end = self.ref_of_isize(gd.end);
            let growth = gd.size;
            self.growth_data.push((start, end, growth));
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
            self.resolver.set_growth_data(*start, *end, *growth);
        }
        // should do placements
        // Should only place roots if the placements don't lead to a resolution
        self.resolver.place_roots(0.);
        self.resolver.assign_min_positions();
        self.desired_range = self.resolver.find_bounds();
        self.size = self.desired_range.size();
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
        self.resolver.place_roots(center - final_size * 0.5);
        self.resolver.place_leaves(center + final_size * 0.5);
        self.resolver.minimize_energy();
        self.size = self.resolver.find_bounds().size();
    }

    //mp get_span
    /// Find the span of a start/number of grid positions
    pub fn get_span(&self, start: isize, end: isize) -> (f64, f64) {
        let start = self.find_ref_of_isize(start).unwrap();
        let end = self.find_ref_of_isize(end).unwrap();
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

    //mp get_positions
    /// Get the positions of all the references
    pub fn get_positions(&self) -> Vec<(isize, f64)> {
        let mut result = Vec::new();
        for (n, (r, _)) in self.refs.iter().enumerate() {
            let pos = self.resolver.get_node_position(n);
            if DEBUG_GRID_PLACEMENT {
                println!("{} : {} : {}", n, r, pos);
            }
            result.push((*r, pos));
        }
        result
    }

    // Display with an indent of indent_str plus six spaces
    pub fn display(&self, indent_str: &str) {
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
    fn check_positions(cp: &GridPlacement, e: &Vec<(isize, f64)>) {
        let err = cp.get_positions().iter().zip(e.iter()).fold(
            None,
            |acc, ((cp_c, cp_s), (e_c, e_s))| {
                acc.or({
                    if *cp_c == *e_c && (*cp_s - *e_s).abs() < 1E-8 {
                        None
                    } else {
                        Some((*cp_c, *cp_s, *e_c, *e_s))
                    }
                })
            },
        );
        assert_eq!(err, None, "Expected positions {:?} got grid {:?}", e, cp);
    }
    //ft test_0
    // #[test]
    fn test_0() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 4, 4.);
        gp.add_cell(4, 6, 2.);
        gp.calculate_positions(0., 0., 0.);
        assert_eq!(gp.get_size(), 6.);
        check_positions(&gp, &vec![(0, -3.), (4, 1.), (6, 3.), (-999, 999.)]);
        assert_eq!(gp.get_span(0, 4), (-3., 1.));
        assert_eq!(gp.get_span(4, 6), (1., 3.));
    }
    //ft test_1
    // #[test]
    fn test_1() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 4, 4.);
        gp.add_cell(4, 6, 2.);
        gp.add_growth_data(&vec![GridData::new(2, 4, 1.)]);

        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size() + 2., 0., 1.);
        assert_eq!(gp.get_size(), 8.);
        check_positions(
            &gp,
            &vec![(0, -4.), (2, -2.), (4, 2.), (6, 4.), (-999, 999.)],
        );
        assert_eq!(gp.get_span(0, 4), (-4., 2.));
        assert_eq!(gp.get_span(4, 6), (2., 4.));
    }
    //ft test_2
    // #[test]
    fn test_2() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 10, 10.);
        gp.add_growth_data(&vec![
            GridData::new(0, 2, 1.),
            GridData::new(2, 8, 0.),
            GridData::new(8, 10, 1.),
        ]);

        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size() + 4., 7., 1.);
        assert_eq!(gp.get_size(), 14.);
        check_positions(
            &gp,
            &vec![(0, 0.), (2, 4.), (8, 10.), (10, 14.), (-999, 999.)],
        );
        assert_eq!(gp.get_span(0, 2), (0., 4.));
        assert_eq!(gp.get_span(2, 8), (4., 10.));
        assert_eq!(gp.get_span(8, 10), (10., 14.));
    }
}
