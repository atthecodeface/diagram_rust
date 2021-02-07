use super::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x0 : f64,
    pub x1 : f64,
    pub y0 : f64,
    pub y1 : f64,
}

impl Rectangle {
    //fp zeros
    /// Create an empty rectangle at 0,0
    pub fn zeros() -> Self {
        Self { x0:0., x1:0., y0:0., y1:0.}
    }

    //fp new
    /// Make a rectangle
    pub fn new(x0:f64, y0:f64, x1:f64, y1:f64) -> Self {
        let (x0,x1) = {if x0<x1 {(x0,x1)} else {(x1,x0)}};
        let (y0,y1) = {if y0<y1 {(y0,y1)} else {(y1,y0)}};
        Self {x0, x1, y0, y1}
    }

    //mp is_zero
    pub fn is_zero(&self) -> bool {
        self.x0==0. && self.x1==1. && self.y0==0. && self.y1==1.
    }

    //mp as_points
    pub fn as_points(&self, close:bool, mut v:Vec<Point>) -> Vec<Point> {
        v.push(Point::new(self.x0,self.y0));
        v.push(Point::new(self.x1,self.y0));
        v.push(Point::new(self.x1,self.y1));
        v.push(Point::new(self.x0,self.y1));
        v.push(Point::new(self.x0,self.y0));
        if close { v.push(Point::new(self.x0,self.y0)); }
        v
    }

    //mp expand
    /// exand in-place by expansion scaled by 'scale'
    pub fn expand(mut self, expansion:&Self, scale:f64) -> Self {
        self.x0 -= scale * self.x0;
        self.y0 -= scale * self.y0;
        self.x1 += scale * self.x1;
        self.y1 += scale * self.y1;
        self
    }

    //mp shrink
    /// shrink in-place by expansion scaled by 'scale'
    pub fn shrink(mut self, expansion:&Self, scale:f64) -> Self {
        self.expand(expansion, -scale)
    }

    //mp union
    /// union this with another; if either is_zero then just the other
    pub fn union(mut self, other:&Self) -> Self {
        if other.is_zero() {
            ();
        } else if self.is_zero() {
            self.x0 = other.x0;
            self.y0 = other.y0;
            self.x1 = other.x1;
            self.y1 = other.y1;
        } else {
            self.x0 = if other.x0<self.x0 {other.x0} else {self.x0};
            self.y0 = if other.y0<self.y0 {other.y0} else {self.y0};
            self.x1 = if other.x1>self.x1 {other.x1} else {self.x1};
            self.y1 = if other.y1>self.y1 {other.y1} else {self.y1};
        }
        self
    }

    //mp intersect
    /// intersect this with another; if either is_zero then this will be zero
    pub fn intersect(mut self, other:&Self) -> Self {
        self.x0 = if other.x0>self.x0 {other.x0} else {self.x0};
        self.y0 = if other.y0>self.y0 {other.y0} else {self.y0};
        self.x1 = if other.x1<self.x1 {other.x1} else {self.x1};
        self.y1 = if other.y1<self.y1 {other.y1} else {self.y1};
        if self.x0>=self.x1 || self.y0>=self.y1 {
            self.x0 = 0.;
            self.y0 = 0.;
            self.x1 = 0.;
            self.y1 = 0.;
        }
        self
    }

    //mp translate
    /// translate in-place by scale*pt
    pub fn translate(mut self, pt:&Point, scale:f64) -> Self {
        self.x0 += scale*pt.x;
        self.x1 += scale*pt.x;
        self.y0 += scale*pt.y;
        self.y1 += scale*pt.y;
        self
    }
}
/*
  (*f rotate_around *)
  let rotate_around r v a =
    let sinr = sin (deg_to_rad a) in
    let cosr = cos (deg_to_rad a) in
    let rotate x y =
      ( ((x-.v.(0)) *. cosr -. (y-.v.(1)) *. sinr),
        ((x-.v.(0)) *. sinr +. (y-.v.(1)) *. cosr) )
    in      
    let x0,y0 = rotate r.(0) r.(1) in
    let x1,y1 = rotate r.(0) r.(3) in
    let x2,y2 = rotate r.(2) r.(1) in
    let x3,y3 = rotate r.(2) r.(3) in
    let lx = v.(0) +. (min (min x0 x1) (min x2 x3)) in
    let rx = v.(0) +. (max (max x0 x1) (max x2 x3)) in
    let ly = v.(1) +. (min (min y0 y1) (min y2 y3)) in
    let ry = v.(1) +. (max (max y0 y1) (max y2 y3)) in
    make lx ly rx ry

  (*f wh_of_largest_area_within w h a
    Find the width and height of the largest rectangle
    that fits within w h at angle a
    If a<0 then we can mirror vertically and use -a, hence abs(a)
    Then modulo 180

    If a>=90 then we can consider a-90 and swap w and h.
    Then, if w>h we can consider 90-a and swap w and h.

    Hence only consider a<90 (hence tan(a)>=0) and w<=h

    Note that the area of a RH triangle with angle a and
    adjacent length l is 1/2.l.l.tan(a) = l^2.t/2
    Assume the largest rectangle leaves rectangular spaces
    above and below; with coordinates of (xw,0), (w,(y'-y)h),
    (w-xw,y'h), (0,yh)
    This assumes largest rectangle is limited by width w, and y'<1.
    We know that tan(a) = xw/yh; i.e.
     yh=xw/t.
    And tan(a)=(y'-y)h/w(1-x); i.e.
     wt(1-x) = y'h-yh
     y'h     = wt(1-x) + xw/t = wt(1+x/(t^2)-x)

    Then the 'wasted space' is then two triangles of size xw : yh and
    two triangles of size w(1-x) : (y'-y)h, and the rectangle of size w : (1-y')h.

    The total is the sum of:
      xw.yh = x^2.w^2/t
      w(1-x).(y'-y)h = w^2.(1-x)^2.t = w^2.t.(1+x^2-2x) = w^2.t + x^2.w^2.t -2x.w^2.t
      wh-wy'h = wh - w^2.t(1+x/(t^2)-x) = wh -w^2.t -x.w^2/t + x.w^2.t

    Sum = x^2.w^2/t + w^2.t + x^2.w^2.t -2x.w^2.t + wh -w^2.t -x.w^2/t + x.w^2.t
        = x^2.w^2/t + x^2.w^2.t -x.w^2.t -x.w^2/t + wh

    This has a minimum (wasted) area when its derivative is 0 (since it has +ve x^2)

    dA/dx = 2x.w^2/t + 2x.w^2.t -w^2.t -w^2/t
          = (2x-1).w^2.(1/t+t)

    i.e. x=0.5; i.e. the correct x is independent of w, h, a.

    But, y' must be <=1. So, we want an x closest to 0.5 where y'<=1
    Now y' when x=0.5 is:
     y' = wt(1+0.5/(t^2)-0.5)/h
        = w/2h * (t + 1/t)
        <=1 if
    t+1/t <= 2h/w
    (t^2+1)/t <= 2h/w
    But (tan^1+1)/tan = sec^2/tan = 1/(sin.cos) = 2/(sin 2a), hence
    2/sin(2a) <= 2h/w
    sin(2a)   >= w/h
    So we only have to worry if sin(2a) < w/h

    Now y'=1 occurs when w/h.t(1+x/(t^2)-x) = 1
    i.e. 1+x/(t^2)-x  = h/(wt)
    i.e. x(1/(t^2)-1) = h/(wt) - 1
    i.e. x(1-(t^2))/t^2 = h/(wt) - 1
    i.e. x              = (h/(wt) - 1) * t^2 / (1-(t^2))
    i.e. x              = (ht/w - t^2) / (1-(t^2))
    Now when the a=45 we have t=1 hence 1-t^2 is 0;
      if w==h then we have a diamond solution (i.e. x-0.5 works)
      if w<=h then sin(2*a) = 1 >= w/h then x=0.5 works
    If w>h then, as at the top, we should have used 90-a and swapped w/h
    
   *)
  let wh_of_largest_area_within w h a =
    let a = abs_float a in
    let a = mod_float a 180. in
    let (w,h,a,flip) = if a>=90. then (h,w,a-.90.,true) else (w,h,a,false) in
    let (w,h,a,flip) = if w>h then (h,w,90.-.a,not flip) else (w,h,a,flip) in
    let sin2a = sin (2. *. a) in
    let t     = tan a in
    let y  x   = x *. w /. h /. t in
    let y' x y = w /. h *. t *. (1. -. x) +. y in
    let x =
      if (t > 1E10) then 0.5 (* cover a=45 *)
      else if (sin2a < w /. h) then (
        (h *. t /. w -. t *. t) /. (1. -. t *. t)
      ) else (
        0.5
      )
    in
    let y  = y x in
    let y' = y' x y in
    let yh = h *. y in
    let y'myh = h *. (y' -. y) in
    let wx = w *. x in
    let wmwx = w *. (1. -. x) in
    let (width,height) =
      if (t>1E-10) then
        ( sqrt (wx*.wx +. yh*.yh) , sqrt (wmwx*.wmwx +. y'myh*.y'myh) )
      else
        (w,h)
    in
    if flip then (height,width) else (width,height)        
    
  (*f get_wh *)
  let get_wh r = (r.(2)-.r.(0), r.(3)-.r.(1))

  (*f get_c *)
  let get_c r =
    ( (r.(0)+.r.(2))/.2.,
      (r.(1)+.r.(3))/.2. )

  (*f get_dim *)
  let get_dim r = function
    | 0 -> [|r.(0); r.(2)|]
    | _ -> [|r.(1); r.(3)|]

  (*f get_width *)
  let get_width r = r.(2) -. r.(0)

  (*f get_height *)
  let get_height r = r.(3) -. r.(1)

  (*f get_cwh *)
  let get_cwh r =
    ( (r.(0)+.r.(2))/.2.,
      (r.(1)+.r.(3))/.2.,
      r.(2)-.r.(0),
      r.(3)-.r.(1))

  (*f of_cwh *)
  let of_cwh (x,y,w,h) =
    make  (x -. w/.2.) (y -. h/.2.) (x +. w/.2.) (y +. h/.2.)

  (*f str *)
  let str r = Printf.sprintf "(%g,%g,%g,%g)" r.(0) r.(1) r.(2) r.(3)

  (*f All done *)
end

 */
