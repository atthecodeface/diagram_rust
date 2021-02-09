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

//a Types
/// (start row * number of rows (>=1) * height in pixels) *)
#[derive(Clone, Copy, Debug)]
pub struct CellData {
    start : isize,
    end   : isize,
    size  : f64,
}
type CellPosition = (isize,f64);
impl CellData {
    pub fn new(start:isize, span:usize, size:f64) -> Self {
        let end = start+(span as isize);
        Self {start, end, size}
    }

    pub fn find_first_last_indices(cell_data:&Vec<CellData>) -> (isize,isize) {
        if cell_data.len() == 0 {
            (0,0)
        } else {
            let mut first = cell_data[0].start;
            let mut last  = cell_data[0].end;
            for Self {start:s, end:e, ..} in cell_data {
                if first > *s {first = *s;}
                if last  < *e {last = *e; }
            }
            (first, last)
        }
    }
    pub fn generate_cell_positions(cell_data:&Vec<CellData>) -> Vec<CellPosition> {
        let n = cell_data.len();
        let mut result = Vec::with_capacity(n);
        if n==0 {return result;}
        
        let mut sorted_indices = Vec::with_capacity(n);
        for i in 0..n {
            sorted_indices.push(i);
        }
        sorted_indices.sort_by(|a,b| cell_data[*a].start.partial_cmp(&cell_data[*b].start).unwrap());
        let mut sorted_cell_data : Vec<CellData> = sorted_indices.iter().map(|n| cell_data[*n]).collect();

        let first_col = sorted_cell_data[0].start;
        let mut sd_index = 0;
        let mut current_col  = first_col;
        let mut posn = 0.;
        result.push( (current_col, posn) );
        loop {
            while sd_index<n {
                if sorted_cell_data[sd_index].size != 0. {break;}
                sd_index += 1;
            }
            if sd_index == n { break; }

            // The current space is being added
            // in (current_col,position)
            //
            // Find the next column and the size up to that column required
            let mut min_size = 0.;
            let mut next_col  = 99999999;
            let mut i = sd_index;
            while i<n {
                if sorted_cell_data[i].start > next_col {break;}
                if sorted_cell_data[i].size == 0.      {continue;}
                let Self {start, end, size} = sorted_cell_data[i];
                if (start > current_col) && (end < next_col) {
                    min_size = 0.;
                    next_col = start;
                } else if (start <= current_col) && (end < next_col) {
                    min_size = size;
                    next_col = end;
                } else if (start <= current_col) && (end == next_col) && (size>min_size){
                    min_size = size;
                } else if (start >= next_col) {
                    break;
                }
                i += 1;
            }
            assert!(min_size>0.);
            i = sd_index;
            while i<n {
                if sorted_cell_data[i].start > next_col {break;}
                sorted_cell_data[i].size -= min_size;
                i += 1;
            }
            current_col = next_col;
            posn       += min_size;
            result.push( (current_col, posn) );
        }
        result
    }
}

//tp GridPlacement
/// This contains a vector of the placement of each element within a grid dimension
/// The cell_positions contains an order vector of <dimension index,posn>, where the dimension indices increase
/// through the vector
/// Structure for a grid - a list of start, span, and height of each cell *)
pub struct GridPlacement {
    cell_positions : Vec<CellPosition>,
    start_index    : isize,
    last_index     : isize,
    expansion      : Vec<f64>,
}

impl GridPlacement {
    pub fn new() -> Self {
        let cell_positions = Vec::new();
        let expansion      = Vec::new();
        Self { cell_positions,
               start_index:0,
               last_index:0,
               expansion,
        }
    }

    pub fn set_cell_data(&mut self, cell_data:&Vec<CellData>) -> () {
        self.cell_positions           = CellData::generate_cell_positions(cell_data);
        let (start_index, last_index) = CellData::find_first_last_indices(cell_data);
        self.start_index = start_index;
        self.last_index = last_index;
    }

    pub fn set_expansion(&mut self, expand_default:f64, expand:Vec<(isize,f64)>) -> () {
        let n = self.last_index - self.start_index;
        let mut expansion = Vec::with_capacity(n as usize);
        expansion.extend((0..n).map(|_| expand_default));
        for (index, amount) in expand {
            let i = index - self.start_index;
            if i >= 0 && i < n { expansion[i as usize] = amount; }
        }
        let expand_total = expansion.iter().fold(0., |sum,x| sum+x);
        if expand_total > 0. {
            for e in &mut expansion { *e = *e / expand_total; }
        }
        self.expansion = expansion;
    }

    //mp get_last_index
    /// The last dimension index is in the end of the vector of (dimension indices, position)
    pub fn get_last_index(&self) -> isize {
        let n = self.cell_positions.len();
        self.cell_positions[n-1].0
    }

    //mp get_span_size
    /// The size of a dimension is the position of the last element in its dimension
    pub fn get_span_size(&self) -> f64 {
        let n = self.cell_positions.len();
        self.cell_positions[n-1].1
    }

    //mp get_position
    /// Get the position for dimension index N
    pub fn get_position(&self, index:isize) -> f64 {
        for (i,x) in &self.cell_positions {
            if *i >= index { return *x; }
        }
        0.
    }

    /*f str *)
    let str t = 
      let str_cell_data = String.concat "\n" (List.map (fun (s,n,sz) -> Printf.sprintf "start %d num %d size %f" s n sz) t.cell_data) in
      let str_positions = String.concat "\n" (List.map (fun (s,p) -> Printf.sprintf "start %d position %f" s p) t.cell_positions) in
      "Grid cell data:\n" ^ str_cell_data ^ "\nRev positions:\n" ^ str_positions
     */

    //zz All done
}

