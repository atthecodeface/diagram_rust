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
//ti GridCellPosition
type GridCellPosition = (isize,f64);

//tp GridCellData
/// (start row * number of rows (>=1) * height in pixels) *)
#[derive(Clone, Copy, Debug)]
pub struct GridCellData {
    start : isize,
    end   : isize,
    size  : f64,
}

//ip GridCellData
const DEBUG_CELL_DATA : bool = false;
impl GridCellData {

    //fp new
    pub fn new(start:isize, end:isize, size:f64) -> Self {
        let (start,end) = if start < end { (start,end) } else {(end,start)};
        let (start,end) = if start != end { (start,end) } else {(start,end+1)};
        Self {start, end, size}
    }

    //fp find_first_last_indices
    pub fn find_first_last_indices(cell_data:&Vec<GridCellData>) -> (isize,isize) {
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
    pub fn generate_cell_positions(cell_data:&Vec<GridCellData>) -> Vec<GridCellPosition> {
        if DEBUG_CELL_DATA { println!("Generate cell positions of cell data {:?}", cell_data); }
        let n = cell_data.len();
        let mut result = Vec::with_capacity(n);
        if n==0 {return result;}
        
        let mut sorted_indices = Vec::with_capacity(n);
        for i in 0..n {
            sorted_indices.push(i);
        }
        sorted_indices.sort_by(|a,b| cell_data[*a].start.partial_cmp(&cell_data[*b].start).unwrap());
        let mut sorted_cell_data : Vec<GridCellData> = sorted_indices.iter().map(|n| cell_data[*n]).collect();

        if DEBUG_CELL_DATA { println!("Sorted cell data {:?}", sorted_cell_data); }

        let first_col = sorted_cell_data[0].start;
        let mut sd_index = 0;
        let mut current_col  = first_col;
        let mut posn = 0.;
        result.push( (current_col, posn) );
        loop {
            // (1,1,10.), (1,1,20.), (1,1,20.)
            if DEBUG_CELL_DATA { println!("start loop: column {} at posn {} sd index {}",current_col,posn,sd_index); }
            while sd_index<n {
                if sorted_cell_data[sd_index].size > 0. {break;}
                sd_index += 1;
            }
            if sd_index == n { break; }

            if DEBUG_CELL_DATA { println!("moved past completed indices to sd index {}",sd_index); }
            // The current space is being added
            // in (current_col,position)
            //
            // Find the next column and the size up to that column required
            let mut min_size = 0.;
            let mut next_col  = 99999999;
            let mut i = sd_index;
            while i<n {

                if DEBUG_CELL_DATA { println!("{}->{} min size {} : checking sd {} {:?}", current_col, next_col, min_size, i, sorted_cell_data[i]); }
                if sorted_cell_data[i].start > next_col {break;}
                if sorted_cell_data[i].size <= 0.      {i+=1; continue;}
                let Self {start, end, size} = sorted_cell_data[i];
                if start <= current_col { // MUST ovelap
                    if end < next_col { // this is the shortest segment starting at current_col so far
                        min_size = size;
                        next_col = end;
                    } else if end == next_col {
                        if size > min_size { min_size = size; }
                        // next_col already == end
                    }
                } else if end <= next_col { // may overlap
                    if min_size > size {
                        next_col = start;
                        min_size = min_size - size;
                    } else {
                        next_col = start;
                        min_size = 0.;
                    }
                } else {
                    break;
                }
                i += 1;
            }
            if DEBUG_CELL_DATA { println!("{}->{} will have size {} [ {} ]",current_col, next_col, min_size, sd_index); }
            // min_size can be zero if we have no cell requirements between (e.g.) cells 1 and 2
            if min_size > 0. {
                i = sd_index;
                while i<n {
                    if sorted_cell_data[i].start >= next_col {break;}
                    if DEBUG_CELL_DATA { println!("reduce {} by {}",sorted_cell_data[i], min_size); }
                    sorted_cell_data[i].size -= min_size; // This may reduce the size below zero
                    i += 1;
                }
            }
            current_col = next_col;
            posn       += min_size;
            result.push( (current_col, posn) );
        }
        let mut last_end = sorted_cell_data[0].end;
        for Self{ end,  .. } in &sorted_cell_data {
            if *end > last_end {last_end = *end; }
        }
        if last_end > current_col {
            result.push( (last_end,posn) );
        }
        if DEBUG_CELL_DATA { println!("Generated cell positions {:?}\n for cell data {:?}", result, cell_data); }
        result
    }

    //fp find_position
    pub fn find_position(positions:&Vec<GridCellPosition>, index:usize, col:isize) -> (usize, f64) {
        if positions.len() == 0 { return (0, 0.); } 
        let mut i = index;
        if i >= positions.len() {
            i = 0;
        }
        let mut posn = positions[i].1;
        loop {
            if i >= positions.len() { break; }
            if positions[i].0 > col { return (i, posn); }
            posn = positions[i].1;
            if positions[i].0 == col { return (i, posn); }
            i += 1;
        }
        (i, posn)
    }

    //zz All done
}

//it Display for GridCellData
impl std::fmt::Display for GridCellData {

    //mp fmt - format a GridCellData
    /// Display the `GridCellData' as (min->max:size)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}->{}:{}]", self.start, self.end, self.size)
    }

    //zz All done
}

//mt Test for GridCellData
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let mut cd = Vec::new();
        cd.push( GridCellData::new(0,4,4.) );
        cd.push( GridCellData::new(4,6,2.) );
        assert_eq!((0,6),GridCellData::find_first_last_indices(&cd));
        let cp = GridCellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), GridCellData::find_position(&cp, 0, 0),"Column 0 starts at 0., and is at index 0");
        assert_eq!((1,4.), GridCellData::find_position(&cp, 0, 4),"Column 4 starts at 4., and is at index 1");
        assert_eq!((2,6.), GridCellData::find_position(&cp, 0, 6),"Column 6 starts at 6., and is at index 2");
    }
    #[test]
    fn test_simple_gap() {
        let mut cd = Vec::new();
        cd.push( GridCellData::new(0,1,1.) );
        cd.push( GridCellData::new(2,3,1.) );
        assert_eq!((0,3),GridCellData::find_first_last_indices(&cd));
        let cp = GridCellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), GridCellData::find_position(&cp, 0, 0),"Column 0 starts at 0., and is at index 0");
        assert_eq!((1,1.), GridCellData::find_position(&cp, 0, 1),"Column 1 starts at 1., and is at index 1");
        assert_eq!((2,1.), GridCellData::find_position(&cp, 0, 2),"Column 2 starts at 1., and is at index 2");
        assert_eq!((3,2.), GridCellData::find_position(&cp, 0, 3),"Column 3 starts at 2., and is at index 3");
    }
    #[test]
    fn test_1() {
        let mut cd = Vec::new();
        cd.push( GridCellData::new(1,2,10.) );
        cd.push( GridCellData::new(1,2,20.) );
        cd.push( GridCellData::new(1,2,20.) );
        assert_eq!((1,2),GridCellData::find_first_last_indices(&cd));
        let cp = GridCellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), GridCellData::find_position(&cp, 0, 0),"Column 0 starts at 0., and is at index 0");
    }        
    #[test]
    fn test_2() {
        let mut cd = Vec::new();
        cd.push( GridCellData::new(60,90,10.) );
        cd.push( GridCellData::new(80,110,20.) );
        cd.push( GridCellData::new(100,110,20.) );
        assert_eq!((60,110),GridCellData::find_first_last_indices(&cd));
        let cp = GridCellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), GridCellData::find_position(&cp, 0, 60), "Column 0 starts at 0., and is at index 0");
        assert_eq!((1,0.), GridCellData::find_position(&cp, 0, 80), "Column 0 starts at 0., and is at index 0");
        assert_eq!((2,10.), GridCellData::find_position(&cp, 0,100),"Column 0 starts at 0., and is at index 0");
        assert_eq!((3,30.), GridCellData::find_position(&cp, 0,110),"Column 0 starts at 0., and is at index 0");
    }        
    #[test]
    fn test_3() {
        let mut cd = Vec::new();
        cd.push( GridCellData::new( 10,20,20.) );
        cd.push( GridCellData::new(-10,20,20.) );
        cd.push( GridCellData::new(-30, 0,10.) );
        assert_eq!((-30,20),GridCellData::find_first_last_indices(&cd));
        let cp = GridCellData::generate_cell_positions(&cd);
        assert_eq!((0,0.), GridCellData::find_position(&cp, 0, -30), "Column 0 starts at 0., and is at index 0");
        assert_eq!((1,0.), GridCellData::find_position(&cp, 0, -10), "Column 0 starts at 0., and is at index 0");
        assert_eq!((2,10.), GridCellData::find_position(&cp, 0, 10),"Column 0 starts at 0., and is at index 0");
        assert_eq!((3,30.), GridCellData::find_position(&cp, 0, 20),"Column 0 starts at 0., and is at index 0");
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
    cell_positions : Vec<GridCellPosition>,
    start_index    : isize,
    last_index     : isize,
    expansion      : Vec<f64>,
}

//ip GridPlacement
const DEBUG_GRID_PLACEMENT : bool = false;
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
    pub fn set_cell_data(&mut self, cell_data:&Vec<GridCellData>) -> () {
        self.cell_positions           = GridCellData::generate_cell_positions(cell_data);
        let (start_index, last_index) = GridCellData::find_first_last_indices(cell_data);
        self.start_index = start_index;
        self.last_index = last_index;
        if DEBUG_GRID_PLACEMENT { println!("Given cell data {:?}", cell_data); }
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

    //mp expand_and_centre
    /// Given an actual size, centered on a value, expand the grid as required, and translate so that it is centered on the value.
    pub fn expand_and_centre(&mut self, size:f64, center:f64) {
        let total_size = self.get_size();
        if total_size <= 0. { return ; }
        let mut sizes = self.generate_cell_sizes();
        let extra_size = size - total_size; // share this according to expansion
        for (n, s) in sizes.iter_mut().enumerate() {
            *s = *s + extra_size * self.expansion[n];
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
    }

    //mi generate_cell_sizes
    fn generate_cell_sizes(&mut self) -> Vec<f64> {
        let n = self.last_index - self.start_index;
        let mut sizes = Vec::with_capacity(n as usize);
        for _ in 0..n {
            sizes.push(0.);
        }
        for i in 0..(self.cell_positions.len()-1) {
            let (cell_index, pos) = self.cell_positions[i];
            let (_, next_pos) = self.cell_positions[i+1];
            sizes[ (cell_index-self.start_index) as usize ] = next_pos - pos;
        }
        sizes
    }
    
    //mp get_span
    /// Find the span of a start/number of grid positions
    pub fn get_span(&self, start:isize, end:isize) -> (f64,f64) {
        let (index, start_posn) = GridCellData::find_position(&self.cell_positions, 0,     start);
        let (_,     end_posn)   = GridCellData::find_position(&self.cell_positions, index, end);
        if DEBUG_GRID_PLACEMENT { println!("Got span for {} {} to be {} {}", start, end, start_posn, end_posn); }
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

    //mp iter_positions
    pub fn iter_positions(&self) -> impl Iterator<Item = (&isize,&f64)> {
        if DEBUG_GRID_PLACEMENT { println!("Iter positions {:?}", self.cell_positions); }
        self.cell_positions.iter().map(|(p,s)| (p,s))
    }

    /*f str *)
    let str t = 
      let str_cell_data = String.concat "\n" (List.map (fun (s,n,sz) -> Printf.sprintf "start %d num %d size %f" s n sz) t.cell_data) in
      let str_positions = String.concat "\n" (List.map (fun (s,p) -> Printf.sprintf "start %d position %f" s p) t.cell_positions) in
      "Grid cell data:\n" ^ str_cell_data ^ "\nRev positions:\n" ^ str_positions
     */

    //zz All done
}