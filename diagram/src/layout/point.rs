//tp Point
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x:f64,
    pub y:f64
}

//ti Display for Point
impl std::fmt::Display for Point {

    //mp fmt - format a `CharError` for display
    /// Display the `Point' as (x,y)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }

    //zz All done
}

//ti Point
impl Point {
    pub const fn new(x:f64, y:f64) -> Self { Self {x,y} }
    pub const fn origin() -> Self { Self {x:0.,y:0.} }
    pub fn is_origin(&self) -> bool { self.x==0. && self.y==0. }
    pub fn rotate(mut self, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*self.x + s*self.y;
        let y1 = c*self.y - s*self.x;
        self.x = x1;
        self.y = y1;
        self
    }
    pub fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        self.x = self.x*sx;
        self.y = self.y*sy;
        self
    }
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
        self.x = self.x + other.x*scale;
        self.y = self.y + other.y*scale;
        self
    }
    pub fn len2(self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    pub fn len(self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn dot(self, other:&Point) -> f64 {
        self.x*other.x + self.y*other.y
    }
    //mp rotate_around
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*(self.x-pivot.x) + s*(self.y-pivot.y);
        let y1 = c*(self.y-pivot.y) - s*(self.x--pivot.x);
        self.x = x1 + pivot.x;
        self.y = y1 + pivot.y;
        self
    }
    //mp union
    /// Treat this and other as a range, and find the min and max
    pub fn union(mut self, other:&Point) -> Self {
        if other.x<self.x {self.x=other.x;}
        if other.y>self.y {self.y=other.y;}
        self
    }

    //mp intersect
    /// Treat this and other as a range, and find the intersection
    pub fn intersect(mut self, other:&Point) -> Self {
        if other.x>self.x {self.x=other.x;}
        if other.y<self.y {self.y=other.y;}
        self
    }

    //mp fit_within_region
    /// Treating the point as a range, place it within an outer range (if possible)
    /// using 'anchor' as a value from -1 to 1, where -1 is place this at the minimum
    /// of the outer range, 1 is place this at the maximum of the outer range
    ///
    /// expand is 0 to not grow the size of the region, or 1 to make it expand to the whole of outer
    ///
    /// As an example, fitting (-4,4) to an outer of (4 25), (centers are 0 and 14.5)
    ///   self_size = 8; outer_size=21; slack=13
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 0, result of (10.5,18.5)
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of -1, result of (4,12)
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 1, result of (17,25)
    ///   with expand of 1 (new size 21, half unused slack=0) and anchor of _, result of (4,25)
    ///   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of -1, result of (4,18.5)
    ///   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of 0, result of (7.25,21.75)
    ///
    /// used slack = expand*slack; unused slack=(1-expand)*slack
    /// from this it is clear the new size = size+slack*expand, new center is 14.5+anchor*half_unused_slack
    /// new center = outer_center + anchor*(1-expand)*slack/2
    /// new left is new center - new_size/2
    pub fn fit_within_dimension(mut self, outer:&Point, anchor:f64,  expand:f64) -> Self {
        let self_size    = self.y-self.x;
        let outer_size   = outer.y-outer.x;
        let outer_center = (outer.y+outer.x)/2.;
        let slack        = outer_size - self_size;
        let new_size     = self_size + slack*expand;
        let new_center   = outer_center + anchor*(1.-expand)*slack/2.;
        self.x += new_center - new_size/2.0;
        self.y += new_center + new_size/2.0;
        self
    }

    //zz All done
}

//mt Test for Point
#[cfg(test)]
mod test_point {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    #[test]
    fn test_simple() {
        pt_eq( &Point::origin(), 0., 0. );
        pt_eq( &Point::new(1.,2.), 1., 2. );
        assert!( Point::origin().is_origin() );
        assert!( !Point::new(0.1,0.).is_origin() );
        assert!( !Point::new(0.,0.1).is_origin() );
        pt_eq( &Point::new(1.,2.).scale_xy(3.,4.), 3., 8. );
        pt_eq( &Point::new(1.,2.).scale_xy(3.,4.), 3., 8. );
        assert_eq!( Point::origin().len2(), 0. );
        assert_eq!( Point::new(1.,0.).len2(), 1. );
        assert_eq!( Point::new(0.,1.).len2(), 1. );
        assert_eq!( Point::new(2.,0.).len2(), 4. );
        assert_eq!( Point::new(0.,2.).len2(), 4. );

        assert_eq!( Point::origin().rotate(30.).len2(), 0. );
        assert_eq!( Point::new(1.,0.).rotate(30.).len2(), 1. );
        assert_eq!( Point::new(0.,1.).rotate(30.).len2(), 1. );
        assert_eq!( Point::new(2.,0.).rotate(30.).len2(), 4. );
        assert_eq!( Point::new(0.,2.).rotate(30.).len2(), 4. );
    }
    #[test]
    fn test_rotate() {
        pt_eq( &Point::new(1.,0.).rotate(0.),    1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(90.),   0.,  1. );
        pt_eq( &Point::new(1.,0.).rotate(180.), -1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(270.),  0., -1. );
    }
    /*
    pub fn rotate(mut self, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*self.x + s*self.y;
        let y1 = c*self.y - s*self.x;
        self.x = x1;
        self.y = y1;
        self
    }
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
        self.x = self.x + other.x*scale;
        self.y = self.y + other.y*scale;
        self
    }
    pub fn len2(self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    pub fn len(self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn dot(self, other:&Point) -> f64 {
        self.x*other.x + self.y*other.y
    }
    //mp rotate_around
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*(self.x-pivot.x) + s*(self.y-pivot.y);
        let y1 = c*(self.y-pivot.y) - s*(self.x--pivot.x);
        self.x = x1 + pivot.x;
        self.y = y1 + pivot.y;
        self
    }
    //mp union
    /// Treat this and other as a range, and find the min and max
    pub fn union(mut self, other:&Point) -> Self {
        if other.x<self.x {self.x=other.x;}
        if other.y>self.y {self.y=other.y;}
        self
    }

    //mp intersect
    /// Treat this and other as a range, and find the intersection
    pub fn intersect(mut self, other:&Point) -> Self {
        if other.x>self.x {self.x=other.x;}
        if other.y<self.y {self.y=other.y;}
        self
    }

    //mp fit_within_region
    /// Treating the point as a range, place it within an outer range (if possible)
    /// using 'anchor' as a value from -1 to 1, where -1 is place this at the minimum
    /// of the outer range, 1 is place this at the maximum of the outer range
    ///
    /// expand is 0 to not grow the size of the region, or 1 to make it expand to the whole of outer
    ///
    /// As an example, fitting (-4,4) to an outer of (4 25), (centers are 0 and 14.5)
    ///   self_size = 8; outer_size=21; slack=13
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 0, result of (10.5,18.5)
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of -1, result of (4,12)
    ///   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 1, result of (17,25)
    ///   with expand of 1 (new size 21, half unused slack=0) and anchor of _, result of (4,25)
    ///   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of -1, result of (4,18.5)
    ///   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of 0, result of (7.25,21.75)
    ///
    /// used slack = expand*slack; unused slack=(1-expand)*slack
    /// from this it is clear the new size = size+slack*expand, new center is 14.5+anchor*half_unused_slack
    /// new center = outer_center + anchor*(1-expand)*slack/2
    /// new left is new center - new_size/2
    pub fn fit_within_dimension(mut self, outer:&Point, anchor:f64,  expand:f64) -> Self {
        let self_size    = self.y-self.x;
        let outer_size   = outer.y-outer.x;
        let outer_center = (outer.y+outer.x)/2.;
        let slack        = outer_size - self_size;
        let new_size     = self_size + slack*expand;
        let new_center   = outer_center + anchor*(1.-expand)*slack/2.;
        self.x += new_center - new_size/2.0;
        self.y += new_center + new_size/2.0;
        self
    }
    }
*/
}
