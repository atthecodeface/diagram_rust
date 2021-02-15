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
const DEBUG_CELL_DATA      : bool = 1 == 0;
const DEBUG_GRID_PLACEMENT : bool = 1 == 0;

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
            let GridCellDataEntry {start, end, size, ..} = self.data[index];
            if size <= 0. {index+=1; continue;}
            if start >= next_col {
                break;
            } else if start <= current_col {
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
                if min_size <= size {
                    next_col = start;
                    min_size = 0.;
                } else {
                    next_col = start;
                    min_size = min_size - size;
                }
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
/// With growth provided and an amount for the dimension to expand by
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
    /// growth is a positive float growth factor
    /// it describes how much this region would like to grow;
    /// for a total growth of X across the whole dimension,
    /// this has a desired growth of size/X*grow
    growth : f64, // defaults to 0.
    /// position - where the left-edge is finally placed
    position : f64, // defaults to 0.
}

//ip GridDimensionEntry
impl GridDimensionEntry {

    //fp new
    pub fn new(start:isize, end:isize, size:f64) -> Self {
        Self {start, end, size, growth:0., position:0.}
    }
}

//tp GridDimensionIter - produces (isize,f64)
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
            Some((self.gd.data[i].end, self.gd.max_pos))
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
///
/// `data` is maintained such that the entries abut each other.
///
#[derive(Debug)]
pub struct GridDimension {
    /// Data that abut, and in increasing column order
    data  : Vec<GridDimensionEntry>,
    /// Starting column of the dimension
    /// This will always be data[0].start
    start : isize,
    /// Ending column of the dimension
    /// This will always be data[-1].end
    end   : isize,
    /// min_pos is calculated once positions are generated
    /// this will always be data[0].pos
    min_pos : f64,
    /// max_pos is calculated once positions are generated
    /// this will always be data[-1].pos+data[-1].size
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

    //mp get_size
    /// Get the size of the whole placement
    pub fn get_size(&self) -> f64 {
        self.max_pos - self.min_pos
    }
    
    //mi find_column
    /// Find a column within the data - if it is beyond the ends then
    /// return None, but otherwise find one of the data cells that it
    /// is within (if it is the boundary of cells) or *the* cell it is
    /// in (if it is not a boundary)
    pub(self) fn find_column(&self, column:isize) -> Option<usize> {
        if column >= self.start && column <= self.end {
            for (i,d) in self.data.iter().enumerate() {
                if column >= d.start && column <= d.end { return Some(i); }
            }
        }
        None
    }
    
    //mi ensure_column_exists
    /// Ensure that a column number exists in the cell data, and
    /// return the index of the cell data and true if it is the start
    /// of that data, false if it is the end
    fn ensure_column_exists(&mut self, column:isize) -> (usize, bool) {
        if self.data.len() == 0 {
            self.add( column, column+1, 0. );
        }
        if column > self.end {
            self.add(self.end, column, 0.);
        }
        if column < self.start { // note that since self.data.len()>0, self.end is already valid
            self.data.insert(0, GridDimensionEntry::new(column, self.start, 0.) );
            self.start = column;
        }
        // Now when we find the index it *must* be something
        // as column is between start and end
        let index = self.find_column(column).unwrap();
        let GridDimensionEntry {start, end, size, ..} = self.data[index];
        if column == start {
            (index, true)
        } else if column == end {
            (index, false)
        } else {
            let ncols        = end - start;
            let size_per_col = size / (ncols as f64);
            let size_0 = (column - start) as f64 * size_per_col;
            self.data.insert(index, GridDimensionEntry::new(start, column, size_0) );
            self.data[index+1].start = column;
            self.data[index+1].size  = size - size_0;
            (index, false)
        }
    }
    
    //mp set_growth_data
    /// Set the growth data for the region start<>end to be
    /// growth
    ///
    /// This involves ensuring that gde.start and gde.end exist
    fn set_growth_data(&mut self, start:isize, end:isize, growth:f64) {
        // The order of the next two lines is important ensure the
        // left exists before the right, as then l_index will still be
        // valid after both lines.
        let (l_index, l_ie) = self.ensure_column_exists(start);
        let (r_index, r_ie) = self.ensure_column_exists(end);
        let l_index = { if l_ie  {l_index} else {l_index+1} };
        let r_index = { if r_ie  {r_index} else {r_index+1} };
        assert_eq!( self.data[l_index].start, start );
        assert_eq!( self.data[r_index-1].end  , end );
        for i in l_index..r_index {
            self.data[i].growth = growth;
        }
    }

    //mp total_relative_growth
    pub fn total_relative_growth(&self) -> f64 {
        self.data.iter().fold(0., |acc, gde| acc+gde.size*gde.growth)
    }

    //mp calculate_positions
    pub fn calculate_positions(&mut self, base:f64, expansion:f64) {
        let mut pos = base;
        self.min_pos = pos;
        for gde in self.data.iter_mut() {
            gde.position = pos;
            pos += gde.size * (1. + gde.growth * expansion);
        }
        self.max_pos = pos;
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
            (i, self.max_pos)
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
mod test_grid_dimension {
    use super::*;
    //fi check_position
    fn check_position(cp:&GridDimension, index:usize, column:isize, posn:f64) {
        assert_eq!(posn, cp.find_position(index, column).1, "Column {} with index {} should be at {}", column, index, posn );
    }
    //fi check_positions
    fn check_positions(cp:&GridDimension, e:&Vec<(isize,f64)>) {
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
        check_positions(&cp, &vec![(0,0.), (4,4.), (6,6.), (-999,999.)]);
    }
    //ft test_simple_gap
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
        check_positions(&cp, &vec![(0,0.), (1,1.), (2,1.), (3,2.), (-999,999.)]);
    }
    //ft test_1
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
        check_positions(&cp, &vec![(1,0.), (2,20.), (-999,999.)]);
    }
    //ft test_2
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
        check_positions(&cp, &vec![(60,0.), (80,0.), (90,10.), (100,10.), (110,30.), (-999,999.)]);
    }
    //ft test_3
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
        check_positions(&cp, &vec![(-30,0.), (-10,0.), (0,10.), (10,10.), (20,30.), (-999,999.)]);
    } 
    //ft test_find_col
    #[test]
    fn test_find_col() {
        let mut cd = GridCellData::new();
        cd.add( 0, 4, 4.);
        cd.add( 4, 6, 2.);
        let cp = cd.create_grid_dimension();
        assert_eq!( cp.find_column(-1), None );
        assert_eq!( cp.find_column(0) , Some(0) );
        assert_eq!( cp.find_column(1) , Some(0) );
        assert_eq!( cp.find_column(3) , Some(0) );
        assert_eq!( cp.find_column(4) , Some(0) );
        assert_eq!( cp.find_column(5) , Some(1) );
        assert_eq!( cp.find_column(6) , Some(1) );
        assert_eq!( cp.find_column(7) , None );
    }
    //ft test_ensure_exists_1
    #[test]
    fn test_ensure_exists_1() {
        let mut cd = GridCellData::new();
        cd.add( 0, 4, 4.);
        cd.add( 4, 6, 2.);
        let mut cp = cd.create_grid_dimension();
        assert_eq!( cp.ensure_column_exists(0), (0, true) );
        assert_eq!( cp.ensure_column_exists(4), (0, false) ); // could return 1,true but does not
        assert_eq!( cp.ensure_column_exists(6), (1, false) );
        cp.calculate_positions(0.,0.);
        assert_eq!(6., cp.get_size());
        check_positions(&cp, &vec![(0,0.), (4,4.), (6,6.), (-999,999.)]);
    }
    //ft test_ensure_exists_2
    #[test]
    fn test_ensure_exists_2() {
        let mut cd = GridCellData::new();
        cd.add( 0, 4, 4.);
        cd.add( 4, 6, 2.);
        let mut cp = cd.create_grid_dimension();
        assert_eq!( cp.ensure_column_exists(-1), (0, true) );
        assert_eq!( cp.ensure_column_exists(4),  (1, false) ); // could return 2,true but does not
        assert_eq!( cp.ensure_column_exists(6),  (2, false) );
        cp.calculate_positions(0.,0.);
        assert_eq!(6., cp.get_size());
        check_positions(&cp, &vec![(-1,0.), (0,0.), (4,4.), (6,6.), (-999,999.)]);

        assert_eq!( cp.ensure_column_exists(-1), (0, true) );
        assert_eq!( cp.ensure_column_exists(2),  (1, false) );
        assert_eq!( cp.ensure_column_exists(4),  (2, false) );
        assert_eq!( cp.ensure_column_exists(5),  (3, false) );
        assert_eq!( cp.ensure_column_exists(6),  (4, false) );
        cp.calculate_positions(0.,0.);
        assert_eq!(6., cp.get_size());
        check_positions(&cp, &vec![(-1,0.), (0,0.), (2,2.), (4,4.), (5,5.), (6,6.), (-999,999.)]);

        assert_eq!( cp.ensure_column_exists(7),  (5, false) );
        assert_eq!( cp.ensure_column_exists(6),  (4, false) );
        assert_eq!( cp.ensure_column_exists(5),  (3, false) );
        assert_eq!( cp.ensure_column_exists(4),  (2, false) );
        assert_eq!( cp.ensure_column_exists(2),  (1, false) );
        assert_eq!( cp.ensure_column_exists(-1), (0, true) );
        cp.calculate_positions(0.,0.);
        assert_eq!(6., cp.get_size());
        check_positions(&cp, &vec![(-1,0.), (0,0.), (2,2.), (4,4.), (5,5.), (6,6.), (7,6.), (-999,999.)]);
    }

    //ft test_set_growth_data_1
    #[test]
    fn test_set_growth_data_1() {
        let mut cd = GridCellData::new();
        cd.add( 0, 4, 4.);
        cd.add( 4, 6, 2.);
        let mut cp = cd.create_grid_dimension();
        cp.set_growth_data( 1, 2, 1. );
        cp.calculate_positions(0.,0.);
        check_positions(&cp, &vec![(0,0.), (1,1.), (2,2.), (4,4.), (6,6.), (-999,999.)]);
        assert_eq!( cp.total_relative_growth(), 1. ); // since the only growth is between 1 and 2 and that has size 1.
            
        cp.set_growth_data( 1, 3, 1. );
        cp.calculate_positions(0.,0.);
        check_positions(&cp, &vec![(0,0.), (1,1.), (2,2.), (3,3.), (4,4.), (6,6.), (-999,999.)]);
        assert_eq!( cp.total_relative_growth(), 2. ); // since the only growth is between 1 and 3 and that has size 2.

        cp.calculate_positions(0.,1./cp.total_relative_growth());
        check_positions(&cp, &vec![(0,0.), (1,1.), (2,2.5), (3,4.), (4,5.), (6,7.), (-999,999.)]);
    }

    //ft test_set_growth_data_21
    #[test]
    fn test_set_growth_data_2() {
        let mut cd = GridCellData::new();
        cd.add( 0, 10, 10.);
        let mut cp = cd.create_grid_dimension();
        cp.set_growth_data( 0, 2, 1. );
        cp.set_growth_data( 8, 10, 1. );

        cp.calculate_positions(0.,0.);
        check_positions(&cp, &vec![(0,0.), (2,2.), (8,8.), (10,10.), (-999,999.)]);
        assert_eq!( cp.total_relative_growth(), 4. );
            
        cp.calculate_positions(0.,4./cp.total_relative_growth());
        check_positions(&cp, &vec![(0,0.), (2,4.), (8,10.), (10,14.), (-999,999.)]);
    }

    //zz All done
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
