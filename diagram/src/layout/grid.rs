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

//a Global constants for debug
const DEBUG_CELL_DATA : bool = true;
const DEBUG_GRID_PLACEMENT : bool = true;

//a GridData
//tp GridData
/// Used in external interfaces
#[derive(Debug)]
pub struct GridData {
    start:isize,
    end:isize,
    size:f64,
}
impl GridData {
    pub fn new(start:isize, end:isize, size:f64) -> Self {
        Self { start, end, size }
    }
}

//ip Display for GridData
impl std::fmt::Display for GridData {

    //mp fmt - format a GridData
    /// Display the `GridData' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}->{}:{}]", self.start, self.end, self.size)
    }

    //zz All done
}

//a Internal types
//ti GridCellDataEntry
/// This holds the desired placement of actual data with overlapping GridData in an array (the GridCellData
/// structure)
#[derive(Debug, Clone)]
struct GridCellDataEntry {
    /// start is the index of the left-hand edge of the cell in the
    /// grid dimension
    start : isize,
    /// end is the index of the right-hand edge of the cell in the
    /// grid dimension
    end   : isize,
    /// size is the desired size, or actual size post-expansion
    size  : f64,
}

//ii GridCellDataEntry
impl GridCellDataEntry {

    //fp new
    pub fn new(start:isize, end:isize, size:f64) -> Self {
        Self {start, end, size}
    }
}

//it Display for GridCellDataEntry
impl std::fmt::Display for GridCellDataEntry {

    //mp fmt - format a GridCellDataEntry
    /// Display the `GridCellDataEntry' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}->{}:{}]", self.start, self.end, self.size)
    }

    //zz All done
}

//tp GridCellData
/// This structure holds the positions and sizes of one dimension of
/// all the elements in a grid
#[derive(Debug)]
pub struct GridCellData {
    data : Vec<GridCellDataEntry>,
    start : isize,
    end   : isize
}

//ip GridCellData
impl GridCellData {

    //fp new
    pub fn new() -> Self {
        Self { data:Vec::new(), start:0, end:0, }
    }

    //fi clone
    /// Clone so that a sorted data version may be used non-destructively
    fn clone(&self) -> Self {
        let mut clone = Self { data:Vec::new(), start:self.start, end:self.end };
        for cd in &self.data {
            clone.data.push(cd.clone());
        }
        clone
    }

    //fp add
    pub fn add(&mut self, start:isize, end:isize, size:f64) {
        let (start,end) = { if start < end { (start,end) } else {(end,start)} };
        let (start,end) = { if start != end { (start,end) } else {(start,end+1)} };
        if start < self.start { self.start = start; }
        if end   > self.end   { self.end   = end; }
        let size = if size < 0. {0.} else {size};
        if self.data.len()==0 { self.start = start; self.end = end; }
        self.data.push(GridCellDataEntry::new(start, end, size));
    }

    //fp add_data
    pub fn add_data(&mut self, grid_data:&GridData) {
        self.add(grid_data.start, grid_data.end, grid_data.size);
    }

    //fi sort_cell_data
    /// Sort the GridCellDataEntry so that they are in increasing order of
    /// 'start'
    fn sort_cell_data(&mut self) {
        self.data.sort_by(|a,b| a.start.partial_cmp(&b.start).unwrap());
    }

    //fi find_next_column_and_size
    /// Find the next column and the size given cell data
    ///
    fn find_next_column_and_size (&self, current_col:isize, mut index:usize) -> (isize, f64) {
        let mut min_size = 0.;
        let mut next_col = self.end+1;
        while index < self.data.len() {
            if DEBUG_CELL_DATA { println!("{}->{} min size {} : checking sd {} {:?}", current_col, next_col, min_size, index, self.data[index]); }
            if self.data[index].start > next_col {break;}
            if self.data[index].size <= 0. {index+=1; continue;}
            let GridCellDataEntry {start, end, size, ..} = self.data[index];
            if start <= current_col {
                // If this grid cell MUST ovelap with the current
                // concept...
                if end < next_col {
                    // and if this is the shortest segment including
                    // current_col so far then it is a better
                    // candidate. Split in to:
                    // current,end,size; end,next_col,min_size-size
                    min_size = size;
                    next_col = end;
                } else if end == next_col {
                    // else if it matches the shortest segment
                    // including current_col so far then it is a
                    // better candidate if it has a larger size
                    // current,(end==next_col),size
                    if size > min_size { min_size = size; }
                    // next_col already == end
                }
                // else this region extends beyond the current
                // current,next_col,min_size; next_col,end,size-min_size
            } else if end <= next_col {
                // If this grid cell starts in the middle of the
                // region but this ends before the region finishes
                if min_size <= size {
                    // if the region is smaller than what this requires
                    // then we can split into:
                    // current,start,0. ; start,end,size ; end,next_col,0.
                    next_col = start;
                    min_size = 0.;
                } else {
                    // if the region is larger than what this requires
                    // then we can split into:
                    // current,start,min_size-size ; start,end,size ; end,next_col,0.
                    next_col = start;
                    min_size = min_size - size;
                }
            } else {
                // if this grid cell starts in the middle of the region
                // but ends after the region then
                // current,next_col,min_size ; next_col,end,size-min_size
                // will satisfy this region
                // since the data is sorted, no other grid cells will overlap
                // with the region, so stop
                break;
            }
            index += 1;
        }
        (next_col, min_size)
    }

    //mi remove_next_region
    /// Find the next region starting at 'column' required by the cell data, knowing that
    /// up to 'index' all the data has non-positive size
    ///
    /// Return a new index (skipping any more initial non-positive
    /// size data) and a next_column and size for the next region,
    /// having removed that size from all elements overlapping that
    /// region
    fn remove_next_region(&mut self, mut index:usize, column:isize) -> Option<(usize, isize, f64)> {
        while index < self.data.len() {
            if self.data[index].size > 0. {break;}
            index += 1;
        }
        if index == self.data.len() {
            None
        } else {
            if DEBUG_CELL_DATA { println!("moved past completed indices to sd index {}",index); }
            let (next_col, min_size) = self.find_next_column_and_size(column, index);
            if DEBUG_CELL_DATA { println!("{}->{} will have size {} [ {} ]",column, next_col, min_size, index); }
        
            // min_size can be zero if we have no cell requirements between (e.g.) cells 1 and 2
            if min_size > 0. {
                for i in index..self.data.len() {
                    if self.data[i].start >= next_col {break;}
                    if DEBUG_CELL_DATA { println!("reduce {} by {}",self.data[i], min_size); }
                    self.data[i].size -= min_size; // This may reduce the size below zero
                }
            }
            Some((index, next_col, min_size))
        }
    }

    //fp create_grid_dimension
    /// Destructively create a grid dimension
    pub fn create_grid_dimension(&self) -> GridDimension {
        let mut gd = GridDimension::new(self.start, self.end);
        
        if DEBUG_CELL_DATA { println!("Generate cell positions of cell data {:?}", self.data); }

        if self.data.len() == 0 {return gd;}

        let mut clone = self.clone();
        clone.sort_cell_data();

        if DEBUG_CELL_DATA { println!("Sorted cell data {:?}", self.data); }

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
        gd.calculate_positions(0.,0.);
        if DEBUG_CELL_DATA { println!("Generated cell positions {:?}\n for cell data {:?}", gd, self.data); }
        gd
    }

    //zz All done
}

//tp GridDimensionEntry
/// This holds the data for the actual
/// dimension of the grid (in the GridDimension structure),
/// with size, growth and position.
/// When created only start, end and size are valid
/// With grow provided and an amount for the dimension to expand by
/// the expansion and position can be determined
#[derive(Debug)]
pub struct GridDimensionEntry {
    /// start is the index of the left-hand edge of the cell in the
    /// grid dimension
    start : isize,
    /// end is the index of the right-hand edge of the cell in the
    /// grid dimension
    end   : isize,
    /// size is the desired size, or actual size post-expansion
    size  : f64,
    /// grow is a positive float growth factor
    /// it describes how much this region would like to grow;
    /// for a total growth of X across the whole dimension,
    /// this has a desired growth of size/X*grow
    grow : f64, // defaults to 0.
    /// position - where the left-edge is finally placed
    position : f64, // defaults to 0.
}

//ip GridDimensionEntry
impl GridDimensionEntry {

    //fp new
    pub fn new(start:isize, end:isize, size:f64) -> Self {
        Self {start, end, size, grow:0., position:0.}
    }
}

//tp GridDimensionIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct GridDimensionIter<'a> {
    gd : &'a GridDimension,
    index: usize,
    n : usize,
}

//ip GridDimensionIter
impl <'a> GridDimensionIter<'a> {
    //fp new
    /// Create a new iterator
    pub fn new(e:&'a GridDimension) -> Self {
        Self { gd : e,
               index : 0,
               n : e.data.len(),
        }
    }
}

//ip Iterator for GridDimensionIter
impl <'a> Iterator for GridDimensionIter<'a> {
    type Item = (isize, f64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.n {
            None
        } else if self.index == self.n {
            let i = self.index - 1;
            self.index += 1;
            Some((self.gd.data[i].end, self.gd.data[i].position+self.gd.data[i].size))
        } else {
            let i = self.index;
            self.index += 1;
            Some((self.gd.data[i].start, self.gd.data[i].position))
        }
    }
}

//tp GridDimension
/// This structure holds the non-overlapping positions and sizes of one dimension of
/// a grid
#[derive(Debug)]
pub struct GridDimension {
    data  : Vec<GridDimensionEntry>,
    start : isize,
    end   : isize,
    min_pos : f64,
    max_pos : f64,
}

//ii GridDimension
impl GridDimension {
    //fp new
    pub fn new(start:isize, end:isize) -> Self {
        Self { start, end, data:Vec::new(), min_pos:0., max_pos:0. }
    }

    //fp add
    pub fn add(&mut self, start:isize, end:isize, size:f64) {
        assert!((self.data.len() == 0) || (self.end == start), "GridDimension added with a gap between old and new!");
        if start < self.start { self.start = start; }
        if end > self.end { self.end = end; }
        if self.data.len()==0 { self.start = start; self.end = end; }
        self.data.push( GridDimensionEntry::new(start, end, size) );
    }

    //fp add_data
    pub fn add_data(&mut self, grid_data:&GridData) {
        self.add(grid_data.start, grid_data.end, grid_data.size);
    }

    //mp calculate_positions
    pub fn calculate_positions(&mut self, base:f64, _expansion:f64) {
        let mut pos = base;
        self.min_pos = pos;
        for gde in self.data.iter_mut() {
            gde.position = pos;
            pos += gde.size;
        }
        self.max_pos = pos;
    }

    //mp get_size
    /// Get the size of the whole placement
    pub fn get_size(&self) -> f64 {
        self.max_pos - self.min_pos
    }
    
    //fp find_position
    /// Find the (precalculated) position of 'column' using the clue that it starts within or beyond 'index' data entry
    pub fn find_position(&self, index:usize, column:isize) -> (usize, f64) {
        if self.data.len() == 0 { return (0, 0.); } 
        let mut i = index;
        if i >= self.data.len() { i = 0; }
        while i<self.data.len() {
            if column < self.data[i].end { break; }
            i += 1;
        }
        if i >= self.data.len() {
            (i, self.data[i-1].position + self.data[i-1].size)
        } else {
            (i, self.data[i].position)
        }
    }

    //mp iter_positions
    //
    pub fn iter_positions<'z>(&'z self) -> GridDimensionIter<'z> {
        GridDimensionIter::new(self)
    }

    //zz All done
}

//mt Test for GridDimension
#[cfg(test)]
mod tests {
    use super::*;
    fn check_position(cp:&GridDimension, index:usize, column:isize, posn:f64) {
        assert_eq!(posn, cp.find_position(index, column).1, "Column {} with index {} should be at {}", column, index, posn );
    }
    #[test]
    fn test_0() {
        let mut cd = GridCellData::new();
        cd.add( 0, 4, 4.);
        cd.add( 4, 6, 2.);
        assert_eq!(0, cd.start);
        assert_eq!(6, cd.end);
        let cp = cd.create_grid_dimension();
        check_position(&cp, 0, -1, 0.);
        check_position(&cp, 0, 0, 0.);
        check_position(&cp, 0, 1, 0.);
        check_position(&cp, 0, 2, 0.);
        check_position(&cp, 0, 3, 0.);
        check_position(&cp, 0, 4, 4.);
        check_position(&cp, 0, 5, 4.);
        check_position(&cp, 0, 6, 6.);
        check_position(&cp, 0, 7, 6.);
        assert_eq!(6., cp.get_size());
    }
    #[test]
    fn test_simple_gap() {
        let mut cd = GridCellData::new();
        cd.add( 0, 1, 1.);
        cd.add( 2, 3, 1.);
        assert_eq!(0, cd.start);
        assert_eq!(3, cd.end);
        let cp = cd.create_grid_dimension();
        check_position(&cp, 0, -1, 0.);
        check_position(&cp, 0, 0, 0.);
        check_position(&cp, 0, 1, 1.);
        check_position(&cp, 0, 2, 1.);
        check_position(&cp, 0, 3, 2.);
        check_position(&cp, 0, 4, 2.);
        assert_eq!(2., cp.get_size());
    }
    #[test]
    fn test_1() {
        let mut cd = GridCellData::new();
        cd.add( 1, 2, 10.);
        cd.add( 1, 2, 10.);
        cd.add( 1, 2, 20.);
        assert_eq!(1, cd.start);
        assert_eq!(2, cd.end);
        let cp = cd.create_grid_dimension();
        check_position(&cp, 0, -1, 0.);
        check_position(&cp, 0, 0, 0.);
        check_position(&cp, 0, 1, 0.);
        check_position(&cp, 0, 2, 20.);
        check_position(&cp, 0, 3, 20.);
        assert_eq!(20., cp.get_size());
    }
    #[test]
    fn test_2() {
        let mut cd = GridCellData::new();
        cd.add( 60, 90, 10.);
        cd.add( 80,110, 10.);
        cd.add(100,110, 20.);
        assert_eq!(60, cd.start);
        assert_eq!(110, cd.end);
        let cp = cd.create_grid_dimension();
        check_position(&cp, 0, 50, 0.);
        check_position(&cp, 0, 60, 0.);
        check_position(&cp, 0, 70, 0.);
        check_position(&cp, 0, 80, 0.);
        check_position(&cp, 0, 90, 10.);
        check_position(&cp, 0,100, 10.);
        check_position(&cp, 0,110, 30.);
        check_position(&cp, 0,120, 30.);
        check_position(&cp, 0,130, 30.);
        assert_eq!(30., cp.get_size());
    }
    #[test]
    fn test_3() {
        let mut cd = GridCellData::new();
        cd.add( 10, 20, 20.);
        cd.add(-10, 20, 20.);
        cd.add(-30, 0,  10.);
        assert_eq!(-30, cd.start);
        assert_eq!( 20, cd.end);
        let cp = cd.create_grid_dimension();
        check_position(&cp, 0,-40, 0.);
        check_position(&cp, 0,-30, 0.);
        check_position(&cp, 0,-20, 00.);
        check_position(&cp, 0,-10, 00.);
        check_position(&cp, 0,  0, 10.);
        check_position(&cp, 0, 10, 10.);
        check_position(&cp, 0, 20, 30.);
        check_position(&cp, 0, 30, 30.);
        check_position(&cp, 0, 40, 30.);
        assert_eq!(30., cp.get_size());
    } 
}

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

    //mp recalculate
    pub fn recalculate(&mut self) {
        self.grid_dimension = self.cell_data.create_grid_dimension();
        self.grid_dimension.calculate_positions(0., 0.);
    }

    //mp expand_and_centre
    /// 
    /// Given an actual size, centered on a value, expand the grid as required, and translate so that it is centered on the value.
    pub fn expand_and_centre(&mut self, _size:f64, center:f64) {
        println!("********************************************************************************");
        println!("recenter by {}",center);
        let total_size = self.get_size();
        if total_size <= 0. { return ; }
        self.grid_dimension.calculate_positions(-total_size/2., 0.);
        /*
        let mut sizes = self.generate_cell_sizes();
        let extra_size = size - total_size; // share this according to expansion
        for (n, s) in sizes.iter_mut().enumerate() {
            *s = *s + extra_size * 1.; // self.expansion[n];
        }
        let mut pos = center - size / 2.;
        let mut index = self.start_index;
        let mut i = 0;
        for j in 0..self.cell_positions.len() {
            let (cell_index, _) = self.cell_positions[j];
            while index < cell_index {
                pos += sizes[i];
                index += 1;
                i += 1;
            }
            self.cell_positions[j] = (cell_index, pos);
        }
         */
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

    //zz All done
}
