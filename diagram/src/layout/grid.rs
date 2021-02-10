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

//a Internal types
//ti CellPosition
type CellPosition = (isize,f64);

//tp CellData
/// (start row * number of rows (>=1) * height in pixels) *)
#[derive(Clone, Copy, Debug)]
pub struct CellData {
    start : isize,
    end   : isize,
    size  : f64,
}

//ip CellData
impl CellData {

    //fp new
    pub fn new(start:isize, span:usize, size:f64) -> Self {
        let end = start+(span as isize);
        Self {start, end, size}
    }

    //fp find_first_last_indices
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

    //fp generate_cell_positions
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
            println!("{}:{} {}",sd_index, next_col, min_size);
            assert!(min_size>0.);
            i = sd_index;
            while i<n {
                if sorted_cell_data[i].start >= next_col {break;}
                println!("reduce {} by {}",sorted_cell_data[i], min_size);
                sorted_cell_data[i].size -= min_size;
                i += 1;
            }
            current_col = next_col;
            posn       += min_size;
            result.push( (current_col, posn) );
        }
        result
    }

    //fp find_position
    pub fn find_position(positions:&Vec<CellPosition>, index:usize, col:isize) -> (usize, f64) {
        let mut i = index;
        loop {
            if i >= positions.len() { break; }
            if positions[i].0 == col { return (i, positions[i].1); }
            i += 1;
        }
        (0, 0.)
    }

    //zz All done
}

//it Display for CellData
impl std::fmt::Display for CellData {

    //mp fmt - format a CellData
    /// Display the `CellData' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}->{}:{}]", self.start, self.end, self.size)
    }

    //zz All done
}

//mt Test for CellData
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let mut cd = Vec::new();
        cd.push( CellData::new(0,4,4.) );
        cd.push( CellData::new(4,2,2.) );
        assert_eq!((0,6),CellData::find_first_last_indices(&cd));
        let cp = CellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), CellData::find_position(&cp, 0, 0),"Column 0 starts at 0., and is at index 0");
        assert_eq!((1,4.), CellData::find_position(&cp, 0, 4),"Column 4 starts at 4., and is at index 1");
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
    cell_positions : Vec<CellPosition>,
    start_index    : isize,
    last_index     : isize,
    expansion      : Vec<f64>,
}

//ip GridPlacement
impl GridPlacement {
    //fp new
    pub fn new() -> Self {
        let cell_positions = Vec::new();
        let expansion      = Vec::new();
        Self { cell_positions,
               start_index:0,
               last_index:0,
               expansion,
        }
    }

    //mp set_cell_data
    pub fn set_cell_data(&mut self, cell_data:&Vec<CellData>) -> () {
        self.cell_positions           = CellData::generate_cell_positions(cell_data);
        let (start_index, last_index) = CellData::find_first_last_indices(cell_data);
        self.start_index = start_index;
        self.last_index = last_index;
        println!("Given cell data {:?}", cell_data);
    }

    //mp set_expansion
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

    //mp get_span
    /// Find the span of a start/number of grid positions
    pub fn get_span(&self, start:isize, span:usize) -> (f64,f64) {
        let end = start + (span as isize);
        let (index, start_posn) = CellData::find_position(&self.cell_positions, 0,     start);
        let (_,     end_posn)   = CellData::find_position(&self.cell_positions, index, end);
        println!("get spans {} {} {} {} {:?}", start, span, start_posn, end_posn, self);
        (start_posn, end_posn)
    }

    //mp get_size
    /// Get the size of the whole placement
    /// This is the position of the end grid element
    pub fn get_size(&self) -> f64 {
        match self.cell_positions.len() {
            0 => 0.,
            n => self.cell_positions[n-1].1
        }
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
