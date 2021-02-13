//a Imports
use super::Point;

//t Rectangle
//tp Rectangle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x0 : f64,
    pub x1 : f64,
    pub y0 : f64,
    pub y1 : f64,
}

//ti Display for Rectangle
impl std::fmt::Display for Rectangle {

    //mp fmt - format a `CharError` for display
    /// Display the `Point' as (x,y)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[({},{}):({},{})]", self.x0, self.y0, self.x1, self.y1)
    }

    //zz All done
}

//ti Rectangle
impl Rectangle {
    //fp none
    /// Create an empty rectangle at 0,0
    pub const fn none() -> Self {
        Self { x0:0., x1:0., y0:0., y1:0.}
    }

    //fp new
    /// Make a rectangle
    pub fn new(x0:f64, y0:f64, x1:f64, y1:f64) -> Self {
        let (x0,x1) = {if x0<x1 {(x0,x1)} else {(x1,x0)}};
        let (y0,y1) = {if y0<y1 {(y0,y1)} else {(y1,y0)}};
        Self {x0, x1, y0, y1}
    }

    //fp bbox_of_points
    /// Make a new rectangle that is the bbox of a vec of points
    pub fn bbox_of_points(pts:&Vec<Point>) -> Self {
        match pts.len() {
            0 => Self::none(),
            _ => {
                let mut r = Self::new(pts[0].x, pts[0].y, pts[0].x, pts[0].y);
                for p in pts {
                    r = r.union( &Self{x0:p.x, y0:p.y, x1:p.x, y1:p.y} );
                }
                r
            },
        }
    }

    //fp of_cwh
    pub fn of_cwh(centre:Point, width:f64, height:f64) -> Self {
        Self::new( centre.x-width/2.,
                  centre.y-height/2.,
                  centre.x+width/2.,
                  centre.y+height/2. )
    }

    //mp pt_within
    pub fn pt_within(&self, mut pt:Point) -> Point {
        pt = pt.add( &Point::new(self.x0,self.y0), -1.);
        pt.scale_xy( 1./(self.x1-self.x0),  1./(self.y1-self.y0) )
    }

    //mp is_none
    pub fn is_none(&self) -> bool {
        self.x0==0. && self.x1==0. && self.y0==0. && self.y1==0.
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

    //mp get_wh
    pub fn get_wh(&self) -> Point {
        Point::new(self.x1-self.x0, self.y1-self.y0)
    }

    //mp get_center
    pub fn get_center(&self) -> Point {
        Point::new((self.x1+self.x0)/2., (self.y1+self.y0)/2.)
    }

    //mp xrange
    pub fn xrange(&self) -> Point {
        Point::new(self.x0, self.x1)
    }

    //mp yrange
    pub fn yrange(&self) -> Point {
        Point::new(self.y0, self.y1)
    }

    //mp width
    pub fn width(&self) -> f64 {self.x1-self.x0}

    //mp height
    pub fn height(&self) -> f64 {self.y1-self.y0}

    //mp get_cwh
    pub fn get_cwh(&self) -> (Point, f64, f64) {
        (self.get_center(), self.width(), self.height())
    }

    //mp scale
    /// Scale by a fixed value
    pub fn scale(mut self, value:f64) -> Self {
        self.x0 *= value;
        self.y0 *= value;
        self.x1 *= value;
        self.y1 *= value;
        self
    }

    //mp enlarge
    /// enlarge by a fixed value
    pub fn enlarge(mut self, value:f64) -> Self {
        self.x0 -= value;
        self.y0 -= value;
        self.x1 += value;
        self.y1 += value;
        self
    }

    //mp reduce
    /// reduce by a fixed value
    pub fn reduce(mut self, value:f64) -> Self {
        self.x0 += value;
        self.y0 += value;
        self.x1 -= value;
        self.y1 -= value;
        self
    }

    //mp expand
    /// exand in-place by expansion scaled by 'scale'
    pub fn expand(mut self, other:&Self, scale:f64) -> Self {
        self.x0 -= scale * other.x0;
        self.y0 -= scale * other.y0;
        self.x1 += scale * other.x1;
        self.y1 += scale * other.y1;
        self
    }

    //mp shrink
    /// shrink in-place by expansion scaled by 'scale'
    // note that self is not mut as this does not modify it - but it consumes it,
    // and returns that from expand
    pub fn shrink(self, other:&Self, scale:f64) -> Self {
        self.expand(other, -scale)
    }

    //mp union
    /// union this with another; if either is_zero then just the other
    pub fn union(mut self, other:&Self) -> Self {
        if other.is_none() {
            ();
        } else if self.is_none() {
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

    //mp new_rotated_around
    /// Rotate the rectangle around a point by an angle,
    /// generating a new rectangle that is the bounding box of that rotated rectangle
    pub fn new_rotated_around(&self, pt:&Point, degrees:f64) -> Self{
        let p0 = Point::new(self.x0,self.y0).rotate_around(pt, degrees);
        let p1 = Point::new(self.x0,self.y1).rotate_around(pt, degrees);
        let p2 = Point::new(self.x1,self.y1).rotate_around(pt, degrees);
        let p3 = Point::new(self.x1,self.y0).rotate_around(pt, degrees);
        let x0 = if p0.x<p1.x {p0.x} else {p1.x};
        let x0 = if x0<p2.x {x0} else {p2.x};
        let x0 = if x0<p3.x {x0} else {p3.x};
        let y0 = if p0.y<p1.y {p0.y} else {p1.y};
        let y0 = if y0<p2.y {y0} else {p2.y};
        let y0 = if y0<p3.y {y0} else {p3.y};
        let x1 = if p0.x>p1.x {p0.x} else {p1.x};
        let x1 = if x1>p2.x {x1} else {p2.x};
        let x1 = if x1>p3.x {x1} else {p3.x};
        let y1 = if p0.y>p1.y {p0.y} else {p1.y};
        let y1 = if y1>p2.y {y1} else {p2.y};
        let y1 = if y1>p3.y {y1} else {p3.y};
        Self {x0, x1, y0, y1}
    }

    //mp fit_within_region
    /// Using two anchor values (x and y) between -1 and 1, and expansion values (between 0 and 1),
    /// fit this region within an outer region
    ///
    /// See Point::fit_within_region for more details on each dimension
    pub fn fit_within_dimension(mut self, outer:&Rectangle, anchor:&Point,  expand:&Point) -> Self {
        let xs = Point::new(self.x0,self.x1).fit_within_dimension( &Point::new(outer.x0,outer.x1), anchor.x, expand.x);
        let ys = Point::new(self.y0,self.y1).fit_within_dimension( &Point::new(outer.y0,outer.y1), anchor.y, expand.y);
        self.x0 = xs.x;
        self.x1 = xs.y;
        self.y0 = ys.x;
        self.y1 = ys.y;
        self
    }
    
    //zz All done
}

//a Test
#[cfg(test)]
mod tests_polygon {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    #[test]
    fn test_zero() {
        let x = Rectangle::none();
        assert!(x.is_zero());
        pt_eq(&x.get_center(),0.,0.);
        pt_eq(&x.get_wh(),0.,0.);
        assert_eq!(x.width(),0.);
        assert_eq!(x.height(),0.);
        pt_eq(&x.xrange(),0.,0.);
        pt_eq(&x.yrange(),0.,0.);
    }
    #[test]
    fn test_0() {
        let x = Rectangle::new(-3.,1., 5.,7.);
        pt_eq(&x.get_center(),1.,4.);
        assert_eq!(x.width(),8.);
        assert_eq!(x.height(),6.);
        pt_eq(&x.get_wh(),8.,6.);
        pt_eq(&x.xrange(),-3.,5.);
        pt_eq(&x.yrange(),1.,7.);
        pt_eq(&x.get_cwh().0,1.,4.);
        assert_eq!(x.get_cwh().1,8.);
        assert_eq!(x.get_cwh().2,6.);
    }
    #[test]
    fn test_ops_0() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Rectangle::new(4.,0., 6.,3.);
        let z = Rectangle::new(5.,1., 7.,4.);
        let x_and_y = x.clone().intersect(&y);
        let x_or_y  = x.clone().union(&y);
        let x_and_z = x.clone().intersect(&z);
        let x_or_z  = x.clone().union(&z);
        println!("x_and_y:{}",x_and_y);
        println!("x_or_y:{}",x_or_y);
        println!("x_and_z:{}",x_and_z);
        println!("x_or_z:{}",x_or_z);
        pt_eq(&x_and_y.xrange(),4.,5.);
        pt_eq(&x_and_y.yrange(),1.,3.);
        pt_eq(&x_or_y.xrange(),2.,6.);
        pt_eq(&x_or_y.yrange(),0.,7.);

        assert!(x_and_z.is_zero());
        pt_eq(&x_and_z.xrange(),0.,0.);
        pt_eq(&x_and_z.yrange(),0.,0.);
        pt_eq(&x_or_z.xrange(),2.,7.);
        pt_eq(&x_or_z.yrange(),1.,7.);
    }
    #[test]
    fn test_ops_1() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Rectangle::new(0.1, 0.2, 0.3, 0.5);
        let x_p_y  = x.clone().expand(&y,1.);
        let x_p_2y = x.clone().expand(&y,2.);
        println!("x_p_y:{}",x_p_y);
        println!("x_p_2y:{}",x_p_2y);
        pt_eq(&x_p_y.xrange(),1.9, 5.3);
        pt_eq(&x_p_y.yrange(),0.8, 7.5);
        pt_eq(&x_p_2y.xrange(),1.8, 5.6);
        pt_eq(&x_p_2y.yrange(),0.6, 8.);
    }
    #[test]
    fn test_ops_2() {
        let x = Rectangle::new(2.,1., 5.,7.);
        let y = Rectangle::new(0.1, 0.2, 0.3, 0.5);
        let x_m_y  = x.clone().shrink(&y,1.);
        let x_m_2y = x.clone().shrink(&y,2.);
        println!("x_m_y:{}",x_m_y);
        println!("x_m_2y:{}",x_m_2y);
        pt_eq(&x_m_y.xrange(),2.1, 4.7);
        pt_eq(&x_m_y.yrange(),1.2, 6.5);
        pt_eq(&x_m_2y.xrange(),2.2, 4.4);
        pt_eq(&x_m_2y.yrange(),1.4, 6.);
    }
}
