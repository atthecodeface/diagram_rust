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

@file    layout.rs
@brief   Layout of placed items and grids
 */

//a Imports
use super::Placements;
use crate::{GridData, GridPlacement};
use geometry::{Point, Rectangle, Transform};

//a Constants
const DEBUG_LAYOUT: bool = 1 == 0;

//a Layout
//tp Layout
#[derive(Debug)]
pub struct Layout {
    pub grid_placements: (GridPlacement, GridPlacement),
    /// 0. to 1. for each dimension to expand layout to fill its parent
    pub grid_expand: (f64, f64),
    pub direct_placements: (Placements, Placements),
    pub desired_grid: Rectangle,
    pub desired_placement: Rectangle,
    pub desired_geometry: Rectangle,
    content_to_actual: Transform,
}

//ip Layout
impl Layout {
    pub fn new() -> Self {
        let grid_placements = (GridPlacement::new(), GridPlacement::new());
        let direct_placements = (Placements::new(), Placements::new());
        Self {
            grid_placements,
            direct_placements,
            grid_expand: (0., 0.),
            desired_placement: Rectangle::none(),
            desired_grid: Rectangle::none(),
            desired_geometry: Rectangle::none(),
            content_to_actual: Transform::new(),
        }
    }

    //mp add_grid_element
    /// Add a grid element given two references for each start and
    /// end, and a minimum size between them
    pub fn add_grid_element(
        &mut self,
        start: (isize, isize),
        end: (isize, isize),
        size: (f64, f64),
    ) {
        self.grid_placements.0.add_cell(start.0, end.0, size.0);
        self.grid_placements.1.add_cell(start.1, end.1, size.1);
    }

    //mp add_placed_element
    pub fn add_placed_element(&mut self, pt: &Point, ref_pt: &Option<Point>, bbox: &Rectangle) {
        self.direct_placements
            .0
            .add_element(pt[0], ref_pt.map(|pt| pt[0]), bbox.x0, bbox.x1);
        self.direct_placements
            .1
            .add_element(pt[1], ref_pt.map(|pt| pt[1]), bbox.y0, bbox.y1);
    }

    //mp add_min_cell_data
    pub fn add_min_cell_data(&mut self, x: &Vec<GridData>, y: &Vec<GridData>) {
        for cd in x {
            self.grid_placements.0.add_cell(cd.start, cd.end, cd.size);
        }
        for cd in y {
            self.grid_placements.1.add_cell(cd.start, cd.end, cd.size);
        }
    }

    //mp add_grow_cell_data
    pub fn add_grow_cell_data(&mut self, x: &Vec<GridData>, y: &Vec<GridData>) {
        self.grid_placements.0.add_growth_data(x);
        self.grid_placements.1.add_growth_data(y);
    }

    //mp get_desired_geometry
    /// With all elements placed the layout will have a desired geometry
    ///
    /// Any placements provide a true bbox
    /// A grid has a desired width and height, centred on 0,0
    pub fn get_desired_geometry(&mut self) -> Rectangle {
        self.grid_placements.0.calculate_positions(0., 0., 0.);
        self.grid_placements.1.calculate_positions(0., 0., 0.);

        let grid_width = self.grid_placements.0.get_size();
        let grid_height = self.grid_placements.1.get_size();

        let place_x_pt = self.direct_placements.0.get_desired_geometry();
        let place_y_pt = self.direct_placements.1.get_desired_geometry();

        self.desired_grid = {
            if grid_width == 0. || grid_height == 0. {
                Rectangle::none()
            } else {
                Rectangle::new(
                    grid_width * -0.5,
                    grid_height * -0.5,
                    grid_width * 0.5,
                    grid_height * 0.5,
                )
            }
        };
        self.desired_placement = {
            if place_x_pt.is_none() || place_y_pt.is_none() {
                Rectangle::none()
            } else {
                Rectangle::none().to_ranges(place_x_pt, place_y_pt)
            }
        };
        self.desired_geometry = {
            if self.desired_placement.is_none() {
                self.desired_grid.clone()
            } else {
                self.desired_placement.clone().union(&self.desired_grid)
            }
        };
        if DEBUG_LAYOUT {
            println!(
                "Layout has desired geometries of grid:{}, place:{}, union {}",
                self.desired_grid, self.desired_placement, self.desired_geometry
            );
        }
        self.desired_geometry.clone()
    }

    //mp layout
    /// All the placement data must have been provided, and a layout of the box can be performed.
    ///
    /// For any grid within the layout this requires a possibly expansion, plus a translation
    pub fn layout(&mut self, within: &Rectangle) {
        // expand_default:(f64,f64), expand:Vec<(isize,f64)>, cell_data:&'a Vec<GridCellData>) -> Self {
        if DEBUG_LAYOUT {
            println!(
                "Laying out Layout {} : {} : {} within rectangle {}",
                self.desired_geometry, self.desired_placement, self.desired_grid, within
            );
        }
        let (ac, aw, ah) = within.get_cwh();
        let (dc, _dw, _dh) = self.desired_geometry.get_cwh();
        if DEBUG_LAYOUT {
            println!("Why not centre on ac {}?", ac);
        }
        self.grid_placements
            .0
            .calculate_positions(aw, 0., self.grid_expand.0);
        self.grid_placements
            .1
            .calculate_positions(ah, 0., self.grid_expand.1);
        self.content_to_actual = Transform::of_translation(ac - dc);
    }

    //mp get_layout_transform
    pub fn get_layout_transform(&self) -> Transform {
        self.content_to_actual.clone()
    }

    //mp get_grid_rectangle
    pub fn get_grid_rectangle(&self, start: (isize, isize), end: (isize, isize)) -> Rectangle {
        let (x0, x1) = self.grid_placements.0.get_span(start.0, end.0);
        let (y0, y1) = self.grid_placements.1.get_span(start.1, end.1);
        Rectangle::new(x0, y0, x1, y1)
    }

    //mp get_place_rectangle
    pub fn get_placed_rectangle(&self, _pt: &Point, _ref_pt: &Option<Point>) -> Rectangle {
        Rectangle::new(0., 0., 10., 10.)
    }

    //mp get_grid_positions
    /// Used to record the layout so it may, for example, be drawn
    ///
    pub fn get_grid_positions(&self) -> (Vec<(isize, f64)>, Vec<(isize, f64)>) {
        let mut result = (Vec::new(), Vec::new());
        for (p, s) in self.grid_placements.0.iter_positions() {
            result.0.push((p, s));
        }
        for (p, s) in self.grid_placements.1.iter_positions() {
            result.1.push((p, s));
        }
        result
    }

    //mp display
    // Display with an indent of indent_str plus two spaces
    pub fn display(&self, indent_str: &str) {
        println!("{}  Layout NOT DONE", indent_str);
    }
    //zz All done
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
