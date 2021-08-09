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
use super::{GridDimension, GridDimensionIter, GridData, GridCellData};

//a Global constants for debug
const DEBUG_GRID_PLACEMENT : bool = 1 == 0;

//a Public GridPlacement type
//tp GridPlacement
/// This contains a vector of the placement of each element within a grid dimension
/// The cell_positions contains an order vector of <dimension index,posn>, where the dimension indices increase
/// through the vector
/// Structure for a grid - a list of start, span, and height of each cell *)
#[derive(Debug)]
pub struct GridPlacement {
    cell_data      : GridCellData,
    grid_dimension : GridDimension,
    growth_data    : GridDimension,
}

//ip GridPlacement
impl GridPlacement {
    //fp new
    pub fn new() -> Self {
        let cell_data      = GridCellData::new();
        let grid_dimension = GridDimension::new(0,0);
        let growth_data    = GridDimension::new(0,0);
        Self { cell_data,
               grid_dimension,
               growth_data,
        }
    }

    //mp add_cell
    pub fn add_cell(&mut self, start:isize, end:isize, size:f64 ) {
        self.cell_data.add( start, end, size );
    }

    //mp add_cell_data
    pub fn add_cell_data(&mut self, cell_data:&GridData) {
        self.cell_data.add_data( cell_data );
    }

    //mp add_growth_data
    pub fn add_growth_data(&mut self, growth_data:&Vec<GridData>) {
        for gd in growth_data{
            self.growth_data.add_data(gd);
        }
    }

    //mp calculate_positions
    /// calculates the positions of the elements in the grid given a center and expansion
    ///
    /// For a desired geometry this should be invoked with 0. for both arguments
    ///
    /// Given an actual size, centered on a value, expand the grid as required, and translate so that it is centered on the value.
    pub fn calculate_positions(&mut self, size:f64, center:f64, expansion:f64) {
        if DEBUG_GRID_PLACEMENT { println!("Must move creation of grid_dimension out to a new function"); }
        self.grid_dimension = self.cell_data.create_grid_dimension();
        for gde in &self.growth_data.data {
            self.grid_dimension.set_growth_data(gde.start, gde.end, gde.size);
        }
        let total_relative_growth = self.grid_dimension.total_relative_growth();
        if DEBUG_GRID_PLACEMENT { println!("total relative growth of {}", total_relative_growth); }

        // Calculate the basic positions assuming no expansion
        self.grid_dimension.calculate_positions(0., 0.);

        // Total size is now valid, so find that out
        let total_size = self.get_size();
        if DEBUG_GRID_PLACEMENT { println!("total size without growth of {}", total_size); }
        if total_size <= 0. { return ; }

        // In an invocation of self.grid_dimension.calculate_positions(_, X)
        // the total amount of growth that will be added is
        //  Sum( gde.size * gde.growth * X ) == X * Sum_GD(size * growth)
        //
        // which must equal (size - total_size) * expansion
        //
        // Hence X = (size - total_size) * expansion / Sum_GD(size*growth)
        //
        let expansion_size  = (size - total_size) * expansion; // May want to bound this by 0. minimum
        let expanded_size   = total_size + expansion_size;
        let expansion_factor = { if total_relative_growth < 1E-6 { 0.} else {expansion_size / total_relative_growth}};
        if DEBUG_GRID_PLACEMENT { println!("sizes s {} t {} e {} es {} ES {} E {}", size, total_size, expansion, expansion_size, expanded_size, expansion_factor); }
        self.grid_dimension.calculate_positions(center-expanded_size/2., expansion_factor);
    }

    //mp get_span
    /// Find the span of a start/number of grid positions
    pub fn get_span(&self, start:isize, end:isize) -> (f64,f64) {
        let (index, start_posn) = self.grid_dimension.find_position(0,     start);
        let (_,     end_posn)   = self.grid_dimension.find_position(index, end);
        if DEBUG_GRID_PLACEMENT { println!("Got span for {} {} to be {} {}", start, end, start_posn, end_posn); }
        (start_posn, end_posn)
    }

    //mp get_size
    /// Get the size of the whole placement
    pub fn get_size(&self) -> f64 {
        self.grid_dimension.get_size()
    }

    //mp iter_positions
    //
    pub fn iter_positions<'z>(&'z self) -> GridDimensionIter<'z> {
        if DEBUG_GRID_PLACEMENT { println!("Iter positions {:?}", self.grid_dimension); }
        self.grid_dimension.iter_positions()
    }

    // Display with an indent of indent_str plus six spaces
    pub fn display(&self, indent_str:&str) {
        println!("{}      {}", indent_str, self.cell_data);
        println!("{}      {}", indent_str, self.grid_dimension);
        println!("{}      {}", indent_str, self.growth_data);
    }
    //zz All done
}

//mt Test for GridPlacement
#[cfg(test)]
mod test_grid_placement {
    use super::*;
    //fi check_positions
    fn check_positions(cp:&GridPlacement, e:&Vec<(isize,f64)>) {
        let err = cp.iter_positions()
            .zip(e.iter())
            .fold(None, | acc,( (cp_c,cp_s), (e_c,e_s))
                  | acc.or( {
                      if cp_c == *e_c && (cp_s-*e_s).abs()<1E-8 {None} else {Some((cp_c,cp_s,*e_c,*e_s))}
                  } ));
        assert_eq!(err, None, "Expected positions {:?} got grid {:?}",e,cp);
    }
    //ft test_0
    #[test]
    fn test_0() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 4, 4.);
        gp.add_cell_data(&GridData::new(4, 6, 2.));
        gp.calculate_positions(0.,0.,0.);
        assert_eq!(gp.get_size(), 6.);
        check_positions(&gp, &vec![(0,-3.), (4,1.), (6,3.), (-999,999.)]);
        assert_eq!(gp.get_span(0, 4), (-3., 1.));
        assert_eq!(gp.get_span(4, 6), (1., 3.));
    }
    //ft test_1
    #[test]
    fn test_1() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 4, 4.);
        gp.add_cell_data(&GridData::new(4, 6, 2.));
        gp.add_growth_data(&vec![ GridData::new(2,4,1.),
        ]);
        
        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size()+2., 0., 1.);
        assert_eq!(gp.get_size(), 8.);
        check_positions(&gp, &vec![(0,-4.), (2,-2.), (4,2.), (6,4.), (-999,999.)]);
        assert_eq!(gp.get_span(0, 4), (-4., 2.));
        assert_eq!(gp.get_span(4, 6), (2., 4.));
    }
    //ft test_2
    #[test]
    fn test_2() {
        let mut gp = GridPlacement::new();
        gp.add_cell(0, 10, 10.);
        gp.add_growth_data(&vec![ GridData::new(0,2,1.),
                                  GridData::new(2,8,0.),
                                  GridData::new(8,10,1.),
        ]);
        
        gp.calculate_positions(0., 0., 0.); // so we can invoke gp.get_size()
        gp.calculate_positions(gp.get_size()+4., 7., 1.);
        assert_eq!(gp.get_size(), 14.);
        check_positions(&gp, &vec![(0,0.), (2,4.), (8,10.), (10,14.), (-999,999.)]);
        assert_eq!(gp.get_span(0, 2),  (0., 4.));
        assert_eq!(gp.get_span(2, 8),  (4., 10.));
        assert_eq!(gp.get_span(8, 10), (10., 14.));
    }
}
