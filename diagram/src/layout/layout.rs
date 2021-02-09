use super::Point;
use super::Rectangle;
use super::Polygon;
use super::grid::{CellData, GridPlacement};

#[derive(Debug, PartialEq)]
pub struct LayoutBox {
    /// This indicates how much to expand the content within its laid-out space (0-1 each in x and y)
    expansion : Point,
    /// This indicates how much where to anchor the content within its laid-out space; if the expansion is 1 then this is irrelevant. -1 indicates to the minimum X/Y, +1 indicates to the maximum X/Y
    anchor    : Point,
    /// The margin may be specified for each of the four sides - it reduces the laid-out space, with the border within the margin
    margin: Option<Rectangle>,
    /// The border is a fixed width all round, and may be 0. for no border; the border is within the laid-out margin, around the padding around the content
    border_width: f64,
    /// The padding may be specified for each of the four sides - it reduces the laid-out space for the content within the border
    padding: Option<Rectangle>,
    /// The content may be rotated within its laid-out (post-padding) space; it will still be rectangular, so it will be the largest rectangle permitted at the rotation provided by the laid-out rectangle
    content_rotation : f64,
    /// The content may be scaled its space, by a uniform amount in X and Y
    content_scale    : f64,
    /// The content reference is a fractional point within the content rectangle; this is probably not required
    content_ref      : Option<Point>,
    /// This rectangle specifies in content coordinates the desired rectangle for the content
    content_desired: Option<Rectangle>,
    /// This rectangle is derived from the content_desired rectangle by scaling, rotation, padding, adding border, and adding margin
    outer_desired: Option<Rectangle>,
    /// The outer rectangle is provided by the layout - it is the actual laid-out outer rectangle, from which the inner laid-out regions are derived
    outer : Option<Rectangle>,
    /// The border_shape is an optional Polygon (rectangle) that may be drawn with a fill, or a stroke width of `border_width`, to provide the required border
    border_shape : Option<Polygon>,
    /// The inner rectangle is the region into which the rotated, scale content fits
    inner : Option<Rectangle>,
    /// The content rectangle is the content-coordinate space rectangle for the laid-out content
    content : Option<Rectangle>,
}
impl LayoutBox {
    pub fn new() -> Self {
        Self { expansion : Point::new(0.,0.),
               anchor    : Point::new(0.,0.),
               margin    : None,
               border_width    : 0.,
               padding   : None,
               content_desired : None,
               content_scale    : 1.,
               content_ref      : None,
               content_rotation : 0.,
               outer_desired : None,
               outer   : None,
               border_shape : None,
               inner   : None,
               content   : None,
        }
    }
    pub fn desired_geometry(&mut self, rect:Rectangle, ref_pt:Point, scale:f64, rotation:f64) -> () {
        self.content_ref      = Some(rect.pt_within(ref_pt));
        self.content_scale    = scale;
        self.content_rotation = rotation;
        self.content_desired  = Some(rect);
    }

    //fp wh_of_largest_area_within
    /// Finds the largest rectangle rotated to `angle` that can fit
    /// within a given width and height
    ///
    /// If angle<0 then the solution is mirrored along the horizontal; the same
    /// rectangle size works therefore.
    /// 
    /// If angle>=90 then we can consider a-90 and swap width and height
    /// Then, if width>height we can consider 90-angle and swap width and height.
    /// 
    /// Hence only consider angle<90 (hence tan(a)>=0) and width<=height
    /// 
    /// Note that the area of a RH triangle with `angle` and
    /// adjacent length l is 1/2.l.l.tan(a) = l^2.t/2
    /// Assume the largest rectangle leaves rectangular spaces
    /// above and below; with coordinates of (xw,0), (w,(y'-y)h),
    /// (w-xw,y'h), (0,yh)
    /// This assumes largest rectangle is limited by width w, and y'<1.
    /// We know that tan(a) = xw/yh; i.e.
    /// yh=xw/t.
    /// And tan(a)=(y'-y)h/w(1-x); i.e.
    /// wt(1-x) = y'h-yh
    /// y'h     = wt(1-x) + xw/t = wt(1+x/(t^2)-x)
    /// 
    /// Then the 'wasted space' is then two triangles of size xw : yh and
    /// two triangles of size w(1-x) : (y'-y)h, and the rectangle of size w : (1-y')h.
    /// 
    /// The total is the sum of:
    /// xw.yh = x^2.w^2/t
    /// w(1-x).(y'-y)h = w^2.(1-x)^2.t = w^2.t.(1+x^2-2x) = w^2.t + x^2.w^2.t -2x.w^2.t
    /// wh-wy'h = wh - w^2.t(1+x/(t^2)-x) = wh -w^2.t -x.w^2/t + x.w^2.t
    /// 
    /// Sum = x^2.w^2/t + w^2.t + x^2.w^2.t -2x.w^2.t + wh -w^2.t -x.w^2/t + x.w^2.t
    /// = x^2.w^2/t + x^2.w^2.t -x.w^2.t -x.w^2/t + wh
    /// 
    /// This has a minimum (wasted) area when its derivative is 0 (since it has +ve x^2)
    /// 
    /// dA/dx = 2x.w^2/t + 2x.w^2.t -w^2.t -w^2/t
    /// = (2x-1).w^2.(1/t+t)
    /// 
    /// i.e. x=0.5; i.e. the correct x is independent of w, h, a.
    /// 
    /// But, y' must be <=1. So, we want an x closest to 0.5 where y'<=1
    /// Now y' when x=0.5 is:
    /// y' = wt(1+0.5/(t^2)-0.5)/h
    ///    = w/2h * (t + 1/t)
    /// <=1 if
    /// t+1/t <= 2h/w
    /// (t^2+1)/t <= 2h/w7
    /// But (tan^1+1)/tan = sec^2/tan = 1/(sin.cos) = 2/(sin 2a), hence
    /// 2/sin(2a) <= 2h/w
    /// sin(2a)   >= w/h
    /// So we only have to worry if sin(2a) < w/h
    /// 
    /// Now y'=1 occurs when w/h.t(1+x/(t^2)-x) = 1
    /// i.e. 1+x/(t^2)-x  = h/(wt)
    /// i.e. x(1/(t^2)-1) = h/(wt) - 1
    /// i.e. x(1-(t^2))/t^2 = h/(wt) - 1
    /// i.e. x              = (h/(wt) - 1) * t^2 / (1-(t^2))
    /// i.e. x              = (ht/w - t^2) / (1-(t^2))
    /// Now when `angle`=45 we have t=1 hence 1-t^2 is 0;
    /// if w==h then we have a diamond solution (i.e. x-0.5 works)
    /// if w<=h then sin(2*a) = 1 >= w/h then x=0.5 works
    /// If w>h then, as at the top, we should have used 90-a and swapped w/h
    fn find_wh_of_largest_area_within(width:f64, height:f64, angle:f64) ->(f64,f64) {
        let angle = {if angle>0.   {angle} else {-angle}};
        let angle = {if angle<180. {angle} else {angle-180.}};
        let (width, height, angle, flip) = {if angle<90.    {(width, height, angle, false)} else {(height, width, angle-90., true)}};
        let (width, height, angle, flip) = {if width<height {(width, height, angle,  flip)} else {(height, width, 90.-angle, !flip)}};
        let sin2a = (2. * angle).to_radians().sin();
        let tana = angle.to_radians().tan();
        let x = {
            if tana > 1E10 {
                0.5
            } else if sin2a<width/height {
                (height * tana / width - tana * tana) / (1. - tana * tana)
            } else {
                0.5
            }
        };
        let y       = x * width / height / tana;
        let y_p     = width / height * tana * (1. - x) + y;
        let yh      = height * y;
        let y_p_myh = height * (y_p - y);
        let wx      = width * x;
        let wmwx    = width * (1. - x);
        let (width,height) = {
            if tana<1E-10 {
                (width, height)
            } else {
                ( (wx*wx + yh*yh).sqrt(), (wmwx * wmwx + y_p_myh * y_p_myh).sqrt() )
            }
        };
        if flip {(height,width)} else {(width,height)}
    }

    fn inner_within_outer(&mut self, rectangle:Rectangle) -> () {
        let mut inner = rectangle.clone();
        self.outer = Some(rectangle);
        inner = self.margin.map_or(inner, |r| inner.shrink(&r, 1.));
        if self.border_width > 0. {
            let (c,w,h) = inner.clone().reduce(self.border_width*0.5).get_cwh();
            self.border_shape = Some(Polygon::new_rect(w,h).translate(&c));
        }
        inner = inner.reduce(self.border_width);
        inner = self.padding.map_or(inner, |r| inner.shrink(&r, 1.));
        self.inner = Some(inner);
    }
    
    fn content_within_inner(&mut self) -> () {
        let inner_cwh  = self.inner.unwrap().get_cwh();
        let content_wh = Self::find_wh_of_largest_area_within(inner_cwh.1, inner_cwh.2, self.content_rotation);
        self.content = Some(Rectangle::of_cwh(inner_cwh.0, content_wh.0, content_wh.1).scale(1.0/self.content_scale));
    }

    pub fn layout_within_rectangle(mut self, rectangle:Rectangle) -> Self {
        self.inner_within_outer(rectangle);
        self.content_within_inner();
        self
    }
}

//tp Layout
pub struct Layout {
    cell_data  : (Vec<CellData>, Vec<CellData>),
    placements : (GridPlacement, GridPlacement),
}
impl Layout {
    pub fn new() -> Self {
        let placements = ( GridPlacement::new(), GridPlacement::new() );
        Self { cell_data:(Vec::new(), Vec::new()),
               placements:placements,
        }
    }
    pub fn add_element(&mut self, start:(isize,isize), span:(usize,usize), size:(f64,f64)) {
        self.cell_data.0.push(CellData::new(start.0, span.0, size.0));
        self.cell_data.1.push(CellData::new(start.1, span.1, size.1));
    }
    pub fn layout(&mut self) {// expand_default:(f64,f64), expand:Vec<(isize,f64)>, cell_data:&'a Vec<CellData>) -> Self {
        self.placements.0.set_cell_data( &self.cell_data.0 );
        self.placements.1.set_cell_data( &self.cell_data.0 );
        self.placements.0.set_expansion( 0., vec![] );
        self.placements.1.set_expansion( 0., vec![] );
    }
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
