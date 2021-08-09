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
use super::{GridData, GridDimension};

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
    pub start : isize,
    /// end is the index of the right-hand edge of the cell in the
    /// grid dimension
    pub end   : isize,
    /// size is the desired size, or actual size post-expansion
    pub size  : f64,
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
        write!(f, "{}->{}:{}", self.start, self.end, self.size)
    }

    //zz All done
}

//tp GridCellData
/// This structure holds the positions and sizes of one dimension of
/// all the elements in a grid
#[derive(Debug)]
pub struct GridCellData {
    pub data : Vec<GridCellDataEntry>,
    pub start : isize,
    pub end   : isize
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

