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

//tp Placement
/// This contains a vector of the placement of each element within a grid dimension
/// The cell_positions contains an order vector of <dimension index,posn>, where the dimension indices increase
/// through the vector
/// Structure for a grid - a list of start, span, and height of each cell *)
pub struct Placement <'a> {
    cell_data      : &'a Vec<CellData>,
    cell_positions : Vec<CellPosition>,
    start_index    : isize,
    last_index     : isize,
    expansion      : Vec<f64>,
}

impl <'a> Placement <'a> {
    pub fn make(expand_default:f64, expand:Vec<(isize,f64)>, cell_data:&'a Vec<CellData>) -> Self {
        let cell_positions            = CellData::generate_cell_positions(cell_data);
        let (start_index, last_index) = CellData::find_first_last_indices(cell_data);
        let n = last_index-start_index;
        assert!(n>=0);
        let mut expansion = Vec::with_capacity(n as usize);
        expansion.extend((0..n).map(|_| expand_default));
        for (index, amount) in expand {
            let i = index - start_index;
            if i >= 0 && i < n { expansion[i as usize] = amount; }
        }
        let expand_total = expansion.iter().fold(0., |sum,x| sum+x);
        if expand_total > 0. {
            for e in &mut expansion { *e = *e / expand_total; }
        }
        Self { cell_data,
               cell_positions,
               start_index,
               last_index,
               expansion
        }
    }

    //mp get_last_index
    /// The last dimension index is in the end of the vector of (dimension indices, position)
    pub fn get_last_index(&self) -> isize {
        let n = self.cell_positions.len();
        self.cell_positions[n-1].0
    }

    //mp get_size
    /// The size of a dimension is the position of the last element in its dimension
    pub fn get_size(&self) -> f64 {
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

//tp GridLayout
pub struct GridLayout {
    cell_data : (Vec<CellData>, Vec<CellData>),
}
impl GridLayout {
    pub fn new() -> Self {
        Self { cell_data:(Vec::new(), Vec::new()) }
    }
    pub fn add_element(&mut self, start:(isize,isize), span:(usize,usize), size:(f64,f64)) {
        self.cell_data.0.push(CellData::new(start.0, span.0, size.0));
        self.cell_data.1.push(CellData::new(start.1, span.1, size.1));
    }
}

//tp Layout
pub struct Layout {
    start_index : isize,
    last_index  : isize,
    positions   : Vec<isize>,
}

/*

impl Layout {
    pub fn get_bbox(&self, start:isize, number:usize) -> (f64,f64) {
        let i0 = start - self.start_index;
        let i1 = i0+number;
        let i0 = { if i0 < 0) {0} else {i0} };
    let i1 = min i1 (tl.last_index-tl.start_index) in (* i1 <= n *)
    if (i1<=i0) then (0.,0.) else (tl.positions.(i0), tl.positions.(i1))

    
  (*f resize_and_place : t -> center float -> size float -> t_layout *)
  let resize_and_place (t:t_placement) c (size:float) =
    let n = t.last_index - t.start_index in
    let positions = Array.init (n+1) (fun i-> Placement.get_position t (i+t.start_index)) in
    let slack     = size -. positions.(n) in
    let rec set_position extra i =
      positions.(i) <- positions.(i)+.extra; (* do this for i==n *)
      if (i<n) then (
        let extra=extra +. slack *. t.expansion.(i) in
        set_position extra (i+1)
      )
    in
    set_position (c -. size /. 2.) 0;
    {start_index=t.start_index; last_index=t.last_index; positions;}

  (*f str *)
  let str t =
    if t.last_index<=t.start_index then
      "Grid layout <none>"
    else if t.last_index<=t.start_index+1 then
      Printf.sprintf "Grid layout of 1 at %g" t.positions.(0)
    else (
      let str_positions = String.concat "," (Array.fold_left (fun acc p->acc@[Printf.sprintf "%g" p]) [] t.positions) in
      Printf.sprintf "Grid layout %d..%d [%s]" t.start_index t.last_index str_positions
    )

  (*f All done *)
end
}

(*a Functions for positions for a grid *)
(*f sort_by_start_index - sort the data by starting index *)
let sort_by_start_index cell_data =
  List.sort (fun (s0,_,_) (s1,_,_) -> compare s0 s1) cell_data

(*f find_min_size - find the shortest height in cell_data starting
    at the specified row;
    
    If there are any cells that start at a row
    after first_row but all cells starting at first_row span beyond
    those, then first_row can be zero height.

    If there are no such cells then find the cells that have the
    minimum span; then of these we need the largest of their sizes, in
    order to fit that cell in. This will be the min height for first_row
    then.

 *)
let find_min_size cell_data first_row =
  let acc_if_smaller acc (s0,h0,size) =
    let (min_height, current_next) = acc in
    (* Printf.printf "acc_if_smaller %d %f %d %d %d %f\n" first_row min_height current_next s0 h0 size; *)
    if ((s0 > first_row) && (s0 < current_next)) then
      (0., s0)
    else if (s0==first_row) && (s0+h0<current_next) then
      (size, s0+h0)
    else if (s0==first_row) && (s0+h0==current_next) && (size>min_height) then
      (size, current_next)
    else
      acc
  in
  List.fold_left acc_if_smaller (0.,max_int) cell_data

(*f remove_rows - remove the span of rows 'first_row' through
    'next_row' given that they have the specified size

    Any cell that starts at first_row can have next_row-first_row rows
    removed from its span: if a cell does not start at first_row then
    it will not overlap with the range; if it does, then remove size
    from its height and changes its start to begin at next_row (since
    the span first_row to next_row had size height).

 *)
let remove_rows sd first_row next_row row_size =
  let n = next_row - first_row in
  let remove_row acc row =
    let (s0,h0,size) = row in
    if (s0>first_row) then (row::acc)
    else if (h0<=n) then acc
    else if (size<=row_size) then (next_row, h0-n, 0.)::acc
    else (next_row, h0-n, size-.row_size)::acc
  in
  List.fold_left remove_row [] sd

(*f find_next_row_position - find the minimum height and next given
    the current row, then set the row positions and remove the span
    height from the cell data, and move on 

 *)
let rec find_next_row_position acc sd first_row current_posn =
  if (List.length sd)==0 then acc else (
    let (size, next_row) = find_min_size sd first_row in
    let posn = current_posn +. size in
    let sd = remove_rows sd first_row next_row size in
    let acc = (next_row, posn)::acc in
    find_next_row_position acc sd next_row posn
  )

(*f find_row_positions cell_data - find the minimal starting positions for
    each row
 *)
let find_row_positions cell_data =
  match cell_data with
  | [] -> []
  | _ -> (
    let sd = sort_by_start_index cell_data in
    let (first_row,_,_) = List.hd sd in
    find_next_row_position [(first_row, 0.)] sd first_row 0.
  )

(*f find_first_last_index *)
let find_first_last_index (f,l) (s,n,_) =
  let f = min s f in
  let l = max l (s+n) in
  (f,l)
  

(*a Top level *)
let make_placement = Placement.make
let get_placement_size = Placement.get_size

let make_layout = Layout.resize_and_place
let get_layout_bbox = Layout.get_bbox
                        
                  
*/
