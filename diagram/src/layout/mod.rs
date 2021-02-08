mod point;
mod bezier;
mod rectangle;
mod polygon;
mod layout;
mod grid;

pub use self::point::Point;
pub use self::bezier::Bezier;
pub use self::rectangle::Rectangle;
pub use self::polygon::Polygon;
pub use self::layout::LayoutBox;
pub use self::grid::GridLayout;

/*
(** Copyright (C) 2018,  Gavin J Stark.  All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * @file     diaglib.ml
 * @brief    Diagrams for SVG initially
 *
 *)

open Types
open Primitives

(*a Types for layout *)
(*t t_layout_properties *)
type t_layout_properties = {
        place             : t_vector option; (* If placed this is where reference point is placed *)
        width             : t_vector option; (* If None then no min/max width; else min/max width *)
        height            : t_vector option; (* If None then no min/max height; else min/max height *)
        expand            : t_vector; (* Amount of slack to take up in each direction, 0 to 1 *)
        anchor            : t_vector; (* If does not fill its outer bbox, then where it is anchored to (-1 top/left, 1 bottom/right) *)
        grid              : t_int4 option;   (* grid elements to cover w=1, h=1 are a single cell *)
        z_index           : float;
        padding           : t_rect;
        border            : t_rect;
        margin            : t_rect;
        border_fill       : Primitives.Color.t;
        border_color      : Primitives.Color.t;
        border_round      : float option; 
        magnets_per_side  : int; 
        rotation          : float option; 
        scale             : t_vector option; 
}

(*f make_layout_hdr stylesheet styleable - get actual data from the provided properties *)
let make_layout_hdr stylesheet styleable =
  let grid_value_of_n n ints =
    match n with | 0 -> [| 0;0;0;0; |]
                 | 1 -> [| ints.(0); ints.(0); 1; 1 |]
                 | _ -> [| ints.(0); ints.(1); 1; 1 |]
  in
  let wh_value_of_n n floats =
    match n with | 0 -> [|0. ; 1.E20 |]
                 | _ -> [|floats.(0) ; 1.E20 |]
  in
  let pbm_value_of_n n floats =
    match n with | 0 -> Primitives.Rectangle.zeros
                 | 1 -> [| floats.(0); floats.(0); floats.(0); floats.(0) |]
                 | _ -> [| floats.(0); floats.(1); floats.(0); floats.(1) |]
  in
  let open Properties in
  let magnets_per_side = get_property_int    stylesheet styleable            Attr_names.magnets_per_side in
  let place            = get_property_vector_option stylesheet styleable     Attr_names.place in
  let width            = get_property_vector_option ~value_of_n:wh_value_of_n stylesheet styleable     Attr_names.width in
  let height           = get_property_vector_option ~value_of_n:wh_value_of_n stylesheet styleable     Attr_names.height in
  let expand           = get_property_vector stylesheet styleable            Attr_names.expand in
  let anchor           = get_property_vector stylesheet styleable            Attr_names.anchor in
  let grid             = get_property_int4_option  ~value_of_n:grid_value_of_n stylesheet styleable     Attr_names.grid in
  let z_index          = get_property_float  stylesheet styleable            Attr_names.z_index in
  let rotation         = get_property_float_option  stylesheet styleable            Attr_names.rotation in
  let scale            = get_property_vector_option  stylesheet styleable           Attr_names.scale in
  let padding          = get_property_rect  ~value_of_n:pbm_value_of_n stylesheet styleable            Attr_names.padding in
  let border           = get_property_rect  ~value_of_n:pbm_value_of_n stylesheet styleable            Attr_names.border in
  let margin           = get_property_rect  ~value_of_n:pbm_value_of_n stylesheet styleable            Attr_names.margin in
  let border_fill      = get_property_color  stylesheet styleable            Attr_names.border_fill in
  let border_color     = get_property_color  stylesheet styleable            Attr_names.border_color in
  let border_round     = get_property_float_option  stylesheet styleable     Attr_names.border_round in
(* content_transform and content_inv_transform *)
{
  place; width; height; anchor; expand; grid; z_index;
  rotation; scale;
  padding; border; margin; border_fill; border_color; border_round; magnets_per_side;
}

(*f props_min_max *)
let props_min_max mm_opt m =
  match mm_opt with 
  | None -> m
  | Some mm ->
    let m = if (m<mm.(0)) then mm.(0) else m in
    let m = if (m>mm.(1)) then mm.(1) else m in
    m

(*f is_placed *)
let is_placed t = 
  match t.grid with | None -> true | _ -> false

(*a Types *)
(*t t - structure of a desired layout
  props is the layout properties used - these are not recalculated for placement
  grids is a pair of GridDimensions that are the table layout for X and Y
  des_geom is the desired geometry for the layout
    This includes border, padding and margin.
      This is at least the width and height of the table layout.
        The table layout is assumed to have a reference point of 0,0 and bounds +-(w,h)/2
      This is at least the width and height of the non-grid items.
        A non-grid item may have a placement for where is reference point should go
        It will have a reference point at the centre of the bounding box of all placed items.
        If there are no placed items then the bounding box is (0,0,0,0)
      The resulting geometry has a bbox of (px0,py0,px1,py1) and a reference point at the centre of that.
    After border, padding and margin (which affect bounding box and reference point) the width/height is bounded with min/max width/height. If this changes a dimension then the reference point moves by half that amount.
    
 *)
type t = {
    props  : t_layout_properties;
    grids  : Grid.t_placement array;
    cont_geom : t_ref_bbox; (* prior to margin expansion etc *)
    des_geom : t_ref_bbox;
  }

(*t t_transform *)
type t_transform = {
translate : t_vector option;
geom      : t_ref_bbox; (* bounding box to be displayed in - do border/fill of this *)
content_geom   : t_ref_bbox; (* bounding box for content to be displayed in *)
    grids  : Grid.t_layout array;
  }

(*a Toplevel Layout module *)
(*f build_layout_data : (gcl * grl * bbox) -> (t_layout_properties * t_desired_geometry) -> (gcl * grl * bbox)
 *)
let build_layout_data acc (cp,des_geom) =
  let (gcl, grl, place_bbox) = acc in
  match cp.grid with
  | None -> (
  (* if it has a placement  then not 0,0 ...*)
    let placement = Primitives.Vector.zeros in
    let child_bbox = Primitives.Rectangle.translate (Desired_geometry.get_drext des_geom) placement in
    let place_bbox = Primitives.Rectangle.union place_bbox child_bbox in
    (gcl, grl, place_bbox)
  )
  | Some gsw -> ( (* gsw is array cs, rs, cw, rw *)
    let (w,h) = Desired_geometry.get_wh des_geom in
    let gcl = (gsw.(0),gsw.(2),w)::gcl in
    let grl = (gsw.(1),gsw.(3),h)::grl in
    (gcl, grl, place_bbox)
  )

(*f create : t_layout_properties -> (t_layout_properties * t_desired_geometry) list -> t

  Create the appropriate layout for the content (not this element) and determine the min_bbox given properties of this
  and the properties and min_bbox of contents

  Start off with no desired placed geometry
  If a child has a grid then add its dimensions to the grid_x and grid_y
  Else if it is placed then its (ref,bbox) contributes as (bbox rel to ref + place)

  Calculate the required width/height for the grid (if any)

  Create final bbox that is centred on placed bbox center; (0,0) if none
  Final geometry is reference point place centre, bbox final bbox
 *)
let create props element_geom children_props_geom =
  let expand_default = props.expand in
  let expand = [] in
  let (gcdx, gcdy, place_bbox) = List.fold_left build_layout_data ([],[],Primitives.Rectangle.zeros) children_props_geom in
  let grids  = Array.mapi (fun i g -> Grid.make_placement expand_default.(i) expand g) [|gcdx; gcdy|] in
  let grid_sizes = Array.map Grid.get_placement_size  grids  in
  let (pw, ph)   = Primitives.Rectangle.get_wh place_bbox in
  let (ew, eh)   = Desired_geometry.get_wh element_geom in
  let cont_w = max (max grid_sizes.(0) pw) ew in
  let cont_h = max (max grid_sizes.(1) ph) eh in
  let cont_wh = match props.rotation with
    | None   -> [| cont_w; cont_h |]
    | Some f -> (
      let r = Primitives.Rectangle.make 0. 0. cont_w cont_h  in
      let r = Primitives.Rectangle.rotate_around r Primitives.Vector.zeros f in
      let (w,h) = Primitives.Rectangle.get_wh r in
      [|w; h|]
    )
  in
  let cont_w = props_min_max props.width  cont_wh.(0) in
  let cont_h = props_min_max props.height cont_wh.(1) in
  let cont_bbox = Primitives.Rectangle.of_cwh (0., 0., cont_w, cont_h) in
  let cont_geom = Desired_geometry.make Primitives.Vector.zeros cont_bbox in
  let des_geom = Desired_geometry.(expand (expand (expand cont_geom props.padding) props.border) props.margin) in
  {props; grids; des_geom; cont_geom}

(*f get_z_index - get the z_index *)
let get_z_index t = t.props.z_index

(*f get_desired_geometry - get the minimum bbox required by the content given grid and placemet (previously created as t) *)
let get_desired_geometry t = t.des_geom

(*f magnets_of_path mps path

 if mps is <=2 then this is just the path coordinates

 if mps is 3 then we want (side 0-1 at 0.0/1.; side 0-1 at 0.5/1.; side 1-2 at 0./1....)

 *)
let magnets_of_path mps path =
  let n = (Array.length path)-1 in
  let mps = max 2 mps in
  let total_pts = (mps-1) * n in
  let fmps = float (mps - 1) in
  let f =
    if mps<=2 then (
      function i -> path.(i)
    ) else (
      function i -> (           
        let side        = i / (mps-1) in
        let ofs_in_side = i mod (mps-1) in
        let x0 = path.(side).(0) in
        let y0 = path.(side).(1) in
        let x1 = path.(side+1).(0) in
        let y1 = path.(side+1).(1) in
        let dx = (x1 -. x0) *. (float ofs_in_side /. fmps) in
        let dy = (y1 -. y0) *. (float ofs_in_side /. fmps) in
        [| x0 +. dx ;
           y0 +. dy ; |]
      )
    )
  in
  Ev_vectors (total_pts, (Array.init total_pts f))

(*f layout_with_geometry
  shrink content bbox by padding/border/margin
  anchor/expand desired geometry into outer geometry and generate geometry in parent coordinates

  The returned content_geom should clearly be inside geom
 *)
let layout_with_geometry t geom =
  let border_geom   = Desired_geometry.(shrink geom        t.props.margin) in
  let padded_geom   = Desired_geometry.(shrink border_geom t.props.border) in
  let internal_geom = Desired_geometry.(shrink padded_geom t.props.padding) in
  let (translate, content_geom) = match t.props.rotation with
    | None -> (None, internal_geom)
    | Some f -> (
      (* let cref = Desired_geometry.get_ref internal_geom in ... cref is not required *)
      let (icx,icy,iw,ih) = Desired_geometry.get_cwh internal_geom in
      let (_,_,cw,ch) = Desired_geometry.get_cwh t.cont_geom in
      let (width,height) = Primitives.Rectangle.wh_of_largest_area_within iw ih f in
      let cref = Primitives.Vector.make icx icy in
      (* Printf.printf "Rot %g,%g %g gives %g,%g\n" cw ch f width height; *)
      let bbox =
        if ((width>=cw) && (height>=ch)) then
          Primitives.Rectangle.of_cwh (0.,0.,width,height)
        else
          Primitives.Rectangle.of_cwh (0.,0.,cw,ch)
      in
      (* (Primitives.Rectangle.translate ~scale:(-1.) bbox cref) *)
      let translate = Some cref in
      let content_geom = Desired_geometry.make Primitives.Vector.zeros bbox in
      (translate, content_geom)
    )
  in
  (* fitted_content_geom is mapping the desired content geometry in to the width/height of our actual content_geom
    the incoming reference point is irrelevant? *)
  let fitted_content_geom  = Desired_geometry.fit_within content_geom t.cont_geom t.props.anchor t.props.expand in
  let (ccx,ccy,cw,ch) = Desired_geometry.get_cwh fitted_content_geom in
  let gx = Grid.make_layout t.grids.(0) ccx cw in
  let gy = Grid.make_layout t.grids.(1) ccy ch in
  let grids = [|gx; gy|] in
  let transform = {translate; geom=geom; content_geom=fitted_content_geom; grids;} in (* geom is used for the border generation - should include updated grid *)
    let magnets =
      let bbox = Desired_geometry.get_bbox geom in
      let path = Primitives.Rectangle.as_vectors ~close:true bbox in
      magnets_of_path t.props.magnets_per_side path
    in
    let layout_pl = [Attr_names.outer_bbox,Ev_rect    (Desired_geometry.get_bbox geom);
                     Attr_names.border_bbox,Ev_rect   (Desired_geometry.get_bbox border_geom);
                     Attr_names.padded_bbox,Ev_rect   (Desired_geometry.get_bbox padded_geom);
                     Attr_names.content_bbox,Ev_rect  (Desired_geometry.get_bbox content_geom);
                     Attr_names.magnets, magnets;
                      ] in
    (transform, content_geom, layout_pl)

(*f get_geom_element *)
let get_geom_element t tr cp des_geom = 
  match cp.grid with
  | None -> (
    tr.content_geom (* as we cannot place things yet *)
  )
  | Some gsw -> ( (* gsw is (cs,rs,cw,rw) *)
    let (x0,x1) = Grid.get_layout_bbox tr.grids.(0) gsw.(0) gsw.(2) in
    let (y0,y1) = Grid.get_layout_bbox tr.grids.(1) gsw.(1) gsw.(3) in
    let bbox = Primitives.Rectangle.make x0 y0 x1 y1 in
    let (cx,cy) = Primitives.Rectangle.get_c bbox in
    (* Printf.printf "get_geom_element %g,%g : %g,%g,%g,%g\n" cx cy x0 y0 x1 y1; *)
    Desired_geometry.make [|cx;cy|] bbox
  )

(*f acc_translate *)
let acc_translate tr acc =
  match tr.translate with
  | None -> acc
  | Some v -> (Printf.sprintf "translate(%g,%g)" v.(0) v.(1))::acc

(*f acc_rotation *)
let acc_rotation t acc =
  match t.props.rotation with
  | None -> acc
  | Some f -> (Printf.sprintf "rotate(%g)" f)::acc

(*f acc_scale *)
let acc_scale t acc =
  match t.props.scale with
  | None -> acc
  | Some v -> (Printf.sprintf "scale(%g,%g)" v.(0) v.(1))::acc

(*f add_transform_tag *)
let add_transform_tag t tr tags =
  let transform = [] in
  let transform = acc_rotation  t  transform in
  let transform = acc_scale     t  transform in
  let transform = acc_translate tr transform in
  match transform with
  | [] -> tags
  | _ -> (Svg.attribute_string "transform" (String.concat " " transform)) :: tags

(*f path_ele *)
let path_ele t coords = String.concat " " (t::(List.map (Printf.sprintf "%g") coords))

(*f svg_border_path_coords *)
let svg_border_path_coords t (tr:t_transform) =
  let bbox = Rectangle.(shrink ~scale:(0.5) (shrink (Desired_geometry.get_bbox tr.geom) t.props.margin) t.props.border)  in
  let x0 = bbox.(0) in
  let y0 = bbox.(1) in
  let x1 = bbox.(2) in
  let y1 = bbox.(3) in
  let path_string = 
    match t.props.border_round with
    | None -> Printf.sprintf "M%g %g L%g %g L%g %g L%g %g Z" x0 y0 x1 y0 x1 y1 x0 y1
    | Some r ->
     String.concat " " [
     path_ele "M" [(x0+.r); y0];
     path_ele "L" [(x1-.r); y0];
     path_ele "Q" [x1; y0; x1; (y0+.r)];
     path_ele "L" [x1; (y1-.r)];
     path_ele "Q" [x1; y1; (x1-.r); y1];
     path_ele "L" [(x0+.r); y1];
     path_ele "Q" [x0; y1; x0; (y1-.r)];
     path_ele "L" [x0; (y0+.r)];
     path_ele "Q" [x0; y0; (x0+.r); y0];
    "Z"]
  in
  Svg.attribute_string "d" path_string

let svg_prepend_fill t tr s =
  let coords = svg_border_path_coords t tr in
  let bw = t.props.border.(0) in (* Border stroke width is just taken from first border element *)
  if (Color.is_none t.props.border_fill) then s else 
    let stroke = Svg.attribute_string "stroke" "none" in
    let stroke_width = Svg.attribute_string "stroke-width" (Printf.sprintf "%g" bw) in
    let fill   = Color.svg_attr "fill" t.props.border_fill in
    let path = Svg.tag "path" [stroke; fill; stroke_width; coords] [] [] in
    path :: s

let svg_append_border t tr s =
  let coords = svg_border_path_coords t tr in
  let bw = t.props.border.(0) in (* Border stroke width is just taken from first border element *)
  if (Color.is_none t.props.border_color) then s else 
    let stroke = Color.svg_attr "stroke" t.props.border_color in
    let stroke_width = Svg.attribute_string "stroke-width" (Printf.sprintf "%g" bw) in
    let fill   = Svg.attribute_string "fill" "none" in
    let path = Svg.tag "path" [stroke; fill; stroke_width; coords] [] [] in
    s @ [path]

let render_svg t tr id z_index make_element_svg svg_contents =
  let id_attrs, svg_contents =
    if t.props.z_index = z_index then (
      ([Svg.attribute_string "id" id], (make_element_svg z_index) @ svg_contents)
    ) else (
      ([],svg_contents)
    )
  in
  let svg_contents = 
    match svg_contents with 
    | [] -> []
    | s -> [ Svg.tag "g" (add_transform_tag t tr id_attrs) s []]
  in
  if (t.props.z_index = z_index) then (
    let svg_contents = svg_prepend_fill  t tr svg_contents in
    let svg_contents = svg_append_border t tr svg_contents in
    [Svg.tag "g" [] svg_contents []]
  ) else (
    svg_contents
  )
 */
