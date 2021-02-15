//a Point
//tp Point
#[derive(Clone, Copy, PartialEq, Debug)]
/// This is a simple point class for two dimensions
pub struct Point {
    /// X coordinate of the point
    pub x:f64,
    /// Y coordinate of the point
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
    //fp new
    /// Create a new point from (x,y)
    pub const fn new(x:f64, y:f64) -> Self { Self {x,y} }

    //fp origin
    /// Create a new point from at (0,0)
    pub const fn origin() -> Self { Self {x:0.,y:0.} }

    //fp is_origin
    /// Return true if the point is the origin
    pub fn is_origin(&self) -> bool { self.x==0. && self.y==0. }

    //cp rotate
    /// Consume the point and return a new point that is the original
    /// rotated around the origin
    pub fn rotate(mut self, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*self.x - s*self.y;
        let y1 = c*self.y + s*self.x;
        self.x = x1;
        self.y = y1;
        self
    }

    //cp scale_xy
    /// Consume the point and return a new point that is the original
    /// scaled in x and y by two different scaling factors
    pub fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        self.x = self.x*sx;
        self.y = self.y*sy;
        self
    }

    //cp add
    /// Consume the point, and return a new point that is the sum of
    /// this point and a borrowed other point
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
        self.x = self.x + other.x*scale;
        self.y = self.y + other.y*scale;
        self
    }

    //mp len2
    /// Return the distance^2 of the point from the origin
    pub fn len2(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    
    //mp len
    /// Return the distance of the point from the origin
    pub fn len(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    //mp dot
    /// Return the dot product of this and another point
    pub fn dot(&self, other:&Point) -> f64 {
        self.x*other.x + self.y*other.y
    }

    //cp rotate_around
    /// Consume the point and return a new point rotated around a
    /// *pivot* point by the specified angle
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*(self.x-pivot.x) + s*(self.y-pivot.y);
        let y1 = c*(self.y-pivot.y) - s*(self.x--pivot.x);
        self.x = x1 + pivot.x;
        self.y = y1 + pivot.y;
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

        assert_eq!( Point::origin().len(), 0. );
        assert_eq!( Point::new(1.,0.).len(), 1. );
        assert_eq!( Point::new(0.,1.).len(), 1. );
        assert_eq!( Point::new(2.,0.).len(), 2. );
        assert_eq!( Point::new(0.,2.).len(), 2. );

        assert_eq!( Point::origin().rotate(30.).len(), 0. );
        assert_eq!( Point::new(1.,0.).rotate(30.).len(), 1. );
        assert_eq!( Point::new(0.,1.).rotate(30.).len(), 1. );
        assert_eq!( Point::new(2.,0.).rotate(30.).len(), 2. );
        assert_eq!( Point::new(0.,2.).rotate(30.).len(), 2. );
    }
    #[test]
    fn test_rotate() {
        pt_eq( &Point::new(1.,0.).rotate(0.),    1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(90.),   0.,  1. );
        pt_eq( &Point::new(1.,0.).rotate(180.), -1.,  0. );
        pt_eq( &Point::new(1.,0.).rotate(270.),  0., -1. );

        pt_eq( &Point::new(0.,1.).rotate(0.),    0.,  1. );
        pt_eq( &Point::new(0.,1.).rotate(90.),  -1.,  0. );
        pt_eq( &Point::new(0.,1.).rotate(180.),  0., -1. );
        pt_eq( &Point::new(0.,1.).rotate(270.),  1.,  0. );
    }
    /*
    pub fn add(mut self, other:&Self, scale:f64) -> Self {
    pub fn dot(self, other:&Point) -> f64 {
    pub fn rotate_around(mut self, pivot:&Point, degrees:f64) -> Self {
     */
}

//a Range
//tp Range
#[derive(Clone, Copy, PartialEq, Debug)]
/// This is a simple 'range' class for two dimensions
///
/// min < max for a valid range; min >= max indicates an empty range
pub struct Range {
    /// Minimum coordinate of the range
    pub min:f64,
    /// Maximum coordinate of the range
    pub max:f64
}

//ti Display for Range
impl std::fmt::Display for Range {

    //mp fmt - format a `CharError` for display
    /// Display the `Range' as (min to max)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} to {})", self.min, self.max)
    }

    //zz All done
}

//ti Range
impl Range {
    //fp new
    /// Create a new point from (min,max)
    pub const fn new(min:f64, max:f64) -> Self { Self {min,max} }

    //fp none
    /// Create a new empty range (0,0)
    pub const fn none() -> Self { Self {min:0.,max:0.} }

    //fp is_none
    /// Return true if the range is empty
    pub fn is_none(&self) -> bool { self.min >= self.max }

    //cp scale
    /// Consume the range and return a new range that is the original
    /// scaled by a factor for both coordinates
    pub fn scale(mut self, scale:f64) -> Self {
        self.min = self.min*scale;
        self.max = self.max*scale;
        self
    }

    //cp add
    /// Consume the range, and return a new range that is translated
    /// by a single value
    pub fn add(mut self, translate:f64) -> Self {
        self.min = self.min + translate;
        self.max = self.max + translate;
        self
    }

    //mp size
    /// Return the size of the range
    pub fn size(&self) -> f64 {
        if self.is_none() {0.} else {self.max - self.min}
    }

    //cp union
    /// Consume the range, and find the union min and max of this with
    /// another, returning the new region
    pub fn union(mut self, other:&Range) -> Self {
        if other.is_none() {
            self
        } else if self.is_none() {
            self.min = other.min;
            self.max = other.max;
            self
        } else {
            if other.min < self.min {self.min=other.min;}
            if other.max > self.max {self.max=other.max;}
            self
        }
    }

    //cp intersect
    /// Consume the range, and find the intersection min and max of this with
    /// another, returning the new region
    pub fn intersect(mut self, other:&Range) -> Self {
        if other.is_none() {
            self
        } else if self.is_none() {
            self.min = other.min;
            self.max = other.max;
            self
        } else {
            if other.min > self.min {self.min=other.min;}
            if other.max < self.max {self.max=other.max;}
            self
        }
    }

    //mp fit_within_region
    /// Treating the point as a range, place it within an outer range (if possible)
    /// using 'anchor' as a value from -1 to 1, where -1 is place this at the minimum
    /// of the outer range, 1 is place this at the maximum of the outer range
    ///
    /// First off, the 'anchor' point (for example at 0.5 in the range -1 to 1) indicates
    /// that the point that is at 'anchor' relative to the 'self' remains at 'anchor' relative
    /// to 'outer' (i.e. will be at 0.5 in the range -1 to 1 for the outer).
    ///
    /// This means that anchor of 0 means map center of self to center of outer
    ///  anchor of 1 means map right hand of self to right hand of outer
    ///  anchor of -1 means map left hand of self to left hand of outer
    ///  anchor of 0.5 means map 3/4 of the way along self to 3/4 the way along outer
    ///
    /// Then the slack between the translated region can be expanded by 'expand'.
    ///
    /// The result is then a translation and a range.
    ///
    /// As an example, fitting (-4,4) to an outer of (4 25), (centers are 0 and 14.5)
    ///   self_size = 8; outer_size=21; slack=13
    /// anchor -1 means outer of (4,N): expand 0 means (4,12); expand 1 means (4,25)
    /// anchor  1 means outer of (N,25): expand 0 means (17,25); expand 1 means (4,25)
    /// anchor  0 means outer of (14.5-x,14.5-x): expand 0 means (10.5,18.5); expand 1 means (4,25)
    ///
    /// current anchor = inner_center + anchor * inner_size
    /// new anchor     = outer_center + anchor * outer_size
    /// translation = new anchor - current anchor
    ///
    /// new left edge unexpanded = inner left + translation
    /// new right edge unexpanded = inner right + translation
    ///
    /// new left slack  = (inner left + translation) - outer left
    /// new right slack = outer right - (inner right + translation)
    /// slack to use = expand * new slack
    ///
    /// new left edge = new left edge unexpanded - expand * new left slack
    /// new right edge = new right edge unexpanded + expand * new right slack
    ///
    /// Or:
    /// translation = outer_center - inner_center + anchor * (outer_size-inner_size)
    /// new left edge = inner_left + translation + expand * (outer_left - inner left - translation)
    /// new right edge = inner_right + translation + expand * (outer_right - inner_right - translation)
    pub fn fit_within_dimension(mut self, outer:&Range, anchor:f64,  expand:f64) -> (f64,Self) {
        let inner_center_2 = self.max  + self.min;
        let outer_center_2 = outer.max + outer.min;
        let inner_size   = self.max-self.min;
        let outer_size   = outer.max-outer.min;
        let translation = (outer_center_2 - inner_center_2)/2. + anchor * (outer_size-inner_size)/2.;
        let new_left_edge  = self.min + translation + expand * (outer.min - self.min - translation);
        let new_right_edge = self.max + translation + expand * (outer.max - self.max - translation);
        // println!("{} {} {} {} {} {} {} {} {} {} {}", self, outer, anchor, expand, inner_center_2/2., outer_center_2/2., inner_size, outer_size, translation, new_left_edge, new_right_edge);
        self.min = new_left_edge;
        self.max = new_right_edge;
        (translation,self)
    }

    //zz All done
}

//mt Test for Range
#[cfg(test)]
mod test_range {
    use super::*;
    pub fn rng_eq(rng:&Range, min:f64, max:f64) {
        assert!((rng.min-min).abs()<1E-8, "mismatch in x {:?} {} {}",rng,min,max);
        assert!((rng.max-max).abs()<1E-8, "mismatch in x {:?} {} {}",rng,min,max);
    }
    #[test]
    fn test_simple() {
        assert!( Range::none().is_none() );
        rng_eq( &Range::new(1.,2.), 1., 2. );
        assert!( Range::new(0.1,0.).is_none() );
        assert!( !Range::new(0.,0.1).is_none() );
        rng_eq( &Range::new(1.,2.).scale(3.), 3., 6. );
        rng_eq( &Range::new(1.,2.).scale(3.), 3., 6. );

        assert_eq!( Range::none().size(), 0. );
        assert_eq!( Range::new(1.,0.).size(), 0. );
        assert_eq!( Range::new(0.,1.).size(), 1. );
        assert_eq!( Range::new(2.,0.).size(), 0. );
        assert_eq!( Range::new(0.,2.).size(), 2. );

    }
    #[test]
    fn test_union() {
        rng_eq( &Range::new(0.,4.).union(&Range::new(0.,4.)),  0., 4. );
        rng_eq( &Range::new(0.,4.).union(&Range::new(0.,5.)),  0., 5. );
        rng_eq( &Range::new(0.,4.).union(&Range::new(2.,5.)),  0., 5. );
        rng_eq( &Range::new(0.,4.).union(&Range::new(2.,3.)),  0., 4. );
        rng_eq( &Range::new(0.,4.).union(&Range::new(-1.,3.)), -1., 4. );
        rng_eq( &Range::new(0.,4.).union(&Range::new(-1.,5.)), -1., 5. );
    }
    #[test]
    fn test_intersect() {
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(0.,4.)), 0., 4. );
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(0.,5.)), 0., 4. );
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(2.,5.)), 2., 4. );
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(2.,3.)), 2., 3. );
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(-1.,3.)), 0., 3. );
        rng_eq( &Range::new(0.,4.).intersect(&Range::new(-1.,5.)), 0., 4. );
    }
    #[test]
    fn test_fit() {
        // As an example, fitting (-4,4) to an outer of (4 25), (centers are 0 and 14.5)
        //   self_size = 8; outer_size=21; slack=13
        // anchor -1 means outer of (4,N): translate +8, expand 0 means (4,12); expand 1 means (4,25)
        // anchor  1 means outer of (N,25): translate +21, expand 0 means (17,25); expand 1 means (4,25)
        // anchor  0 means outer of (14.5-x,14.5-x): expand 0 means (10.5,18.5); expand 1 means (4,25)
        //   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 0, result of (10.5,18.5)
        //   with expand of 0 (new size 8, half unused slack=6.5) and anchor of -1, result of (4,12)
        //   with expand of 0 (new size 8, half unused slack=6.5) and anchor of 1, result of (17,25)
        //   with expand of 1 (new size 21, half unused slack=0) and anchor of _, result of (4,25)
        //   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of -1, result of (4,18.5)
        //   with expand of 0.5 (new size 14.5, half unused slack=3.25) and anchor of 0, result of (7.25,21.75)
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.), -1., 0.).1,    4.,  12. );
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  1., 0.).1,   17.,  25. );
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  0., 0.).1,   10.5, 18.5 );
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.), -1., 1.).1,    4.,  25. );
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  1., 1.).1,    4.,  25. );
        rng_eq( &Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  0., 1.).1,    4.,  25. );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.), -1., 0.).0,  8. );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  1., 0.).0, 21. );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  0., 0.).0, 14.5 );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.), -1., 1.).0,  8. );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  1., 1.).0, 21. );
        assert_eq!( Range::new(-4.,4.).fit_within_dimension(&Range::new(4.,25.),  0., 1.).0, 14.5 );
    }
}
