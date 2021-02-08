use super::Point;
use super::Rectangle;
use super::Polygon;

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

