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

@file    bexier.rs
@brief   Part of geometry library
 */

//a Imports
use super::point::Point;

//a BezierLineIter
//tp BezierLineIter
/// An iterator of straight lines that form a single Bezier curve
///
/// An iteration will provide (Pa, Pb) pairs of points, with
/// the next iteration providing (Pb, Pc), then (Pc, Pd), etc;
/// sharing the end/start points.
pub struct BezierLineIter {
    /// Maximum curviness of the line segments returned
    straightness: f64,
    /// A stack of future beziers to examine
    /// The top of the stack is p0->p1; below that is p1->p2, etc
    /// These beziers will need to be split to achieve the maximum
    /// curviness
    stack : Vec<Bezier>
}

//pi BezierLineIter
impl BezierLineIter {
    //fp new
    /// Create a new Bezier line iterator for a given Bezier and
    /// straightness
    ///
    /// This clones the Bezier.
    pub fn new(bezier:&Bezier, straightness:f64) -> Self {
        let mut stack = Vec::new();
        stack.push(bezier.clone());
        Self { straightness, stack }
    }
    
    //zz All done
}

//ip Iterator for BezierLineIter
impl Iterator for BezierLineIter {
    /// Item is a pair of points that make a straight line
    type Item = (Point,Point);
    /// next - return None or Some(pa,pb)
    ///
    /// It pops the first Bezier from the stack: this is (pa,px); if
    /// this is straight enough then return it, else split it in two
    /// (pa,pm), (pm,px) and push them in reverse order, then recurse.
    ///
    /// This forces the segment returned (eventually!) to be (pa,pb)
    /// and to leave the top of the stack starting with pb.
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            None => None,
            Some(b) => {
                if b.is_straight(self.straightness) {
                    Some(b.endpoints())
                } else {
                    let (b0, b1) = b.bisect();
                    self.stack.push(b1);
                    self.stack.push(b0);
                    self.next()
                }
            }
        }
    }
    
    //zz All done
}

//a BezierPointIter
//tp BezierPointIter
/// An iterator of points that form a single Bezier curve where the
/// steps between points would be lines that are 'straight enough'
///
/// This iterator returns the points that BezierLineIter uses, in the
/// same order (pa, pb, ...)
pub struct BezierPointIter {
    /// A line iterator that returns the next line segment required;
    /// usually the first point of this segment that this iterator
    /// provides is returned as the next point.
    ///
    /// When this returns none, the end-point of the previous
    /// iteration needs to be returned as the last point.
    lines : BezierLineIter,
    /// The last point to be returned - if this is valid then the line
    /// iterator has finished, and just the last point on the Bezier
    /// needs to be returned.
    last_point : Option<Point>,
}

//ip BezierPointIter
impl BezierPointIter {
    //fp new
    /// Create a new point iterator from a line iterator
    pub fn new(lines:BezierLineIter) -> Self {
        Self { lines, last_point:None }
    }
    
    //zz All done
}

//ii BezierPointIter
impl Iterator for BezierPointIter {
    /// Iterator returns Point's
    type Item = Point;

    /// Return the first point of any line segment provided by the
    /// line iterator, but record the endpoint of that segment first;
    /// if the line iterator has finished then return any recorded
    /// endpoint, deleting it first.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some( (p0, p1) ) = self.lines.next() {
            self.last_point = Some(p1);
            Some(p0)
        } else {
            let p = self.last_point;
            self.last_point = None;
            p
        }
    }
    
    //zz All done
}

//a Bezier
//tp Bezier
/// This library supports Bezier curves of up to order 3 - i.e. up to
/// Cubic; these have two control poits.
///
/// Note: in this section we use u=1-t
///
/// A linear Bezier has two points, p0 and p1, and provides points
/// along the line as:
///    p(t,u=1-t) = u*p0 + t*p1
///
/// A linear Bezier may be split at t into (p0, u*p0+t*p1); (u*p0+t*p1, p1).
///
/// A quadratic Bezier has three points, p0, c and p1, and provides
/// points along the curve as:
///
///    p(t,u=1-t) = u^2.p0 + 2.t.u.c + t^2.p1
///
/// or, viewing it is a linear Bezier between two linear beziers:
///
///    p(t) = u(u.p0 + t.c) + t(u.c + t.p1)
///
/// To split a quadratic bezier at t is simple: the split point is p(t),
/// and the two control points (cl, cr) are:
///
///   cl(t) = u.p0 + t.c ; cr = u.c + t.p1
///
/// Hence the Quadratic Bezier between t0 and t1 can be calculated
/// by splitting to get the right-hand Bezier of t0->1, and splitting
/// this to get the left-hand Bezier at (t1-t0)/u0 = (t2,u2)
///
///    Note t2 = (t1-t0)/u0; u2=1-t2 = (u0+t0-t1)/u0 = (1-t1)/u0 = u1/u0
///
///    cl(t0) = u0.p0 + t0.c
///    cr(t0) = u0.c  + t1.p1
///     p(t0) = u0.cl(t0)  + t0.cr(t0)
/// 
///    Bezier t0->1 : p(t0), cr(t0), p1
///
///  c(t0,t1)  = u2.p(t0)  + t2.cr(t0)
///            = u2.u0.cl(t0) + u2.t0.cr(t0) + t2.cr(t0)
///            = u2.u0.cl(t0) + (u2.t0+t2).cr(t0)
///  But u2.u0    = u1
///  And u2.t0+t2 = u1/u0.t0+(t1-t0)/u0
///               = (t0.u1+t1-t0)/u0
///               = (t0 - t1.t0 + t1 - t0) / u0
///               = (t1 - t1.t0) / u0
///               = t1(1-t0) / (1-t0)
///               = t1
///  Hence
///  c(t0,t1)  = u1.cl(t0) + t1.cr(t0)
///            = u0.u1.p0 + u1.t0.c + u0.t1.c + t0.t1.p1
///            = u0.u1.p0 + (u1.t0+u0.t1).c + t0.t1.p1
///  And the points are:
///      p(t0) = u0.u0.p0 + 2(u0.t0).c + t0.t0.p1
///      p(t1) = u1.u1.p0 + 2(u1.t1).c + t1.t1.p1
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Bezier {
    /// Linear is a straight line between two points
    Linear(Point, Point),
    /// Quadratic is a Bezier whose point p at parameter t is:
    /// (1-t)^2.p0 + 2t.(1-t).c + t^2.p1
    Quadratic(Point, Point, Point),
    /// Cubic is a Bezier whose point p at parameter t is:
    /// (1-t)^3.p0 + 3.t.(1-t)^2.c0 + 3.t^2.(1-t).c1 + t^3.p1
    Cubic(Point, Point, Point, Point),
}

//ti Display for Bezier
impl std::fmt::Display for Bezier {

    //mp fmt - format a `CharError` for display
    /// Display the `Point' as (x,y)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Linear(p0,p1)      => write!(f, "[{}<->{}]", p0, p1),
            Self::Quadratic(p0,c,p1) => write!(f, "[{}<-:{}:->{}]", p0, c, p1),
            Self::Cubic(p0,c0,c1,p1) => write!(f, "[{}<-:{}:{}:->{}]", p0, c0, c1, p1),
        }
    }

    //zz All done
}

//ip Bezier
impl Bezier {
    //mp borrow_pt
    /// Get the start or end point of the Bezier - index 0 gives the
    /// start point, index 1 the end point
    pub fn borrow_pt(&self, index:usize) -> &Point {
        match self {
            Self::Linear(p0,p1) =>      { if index==0 {p0} else {p1} },
            Self::Quadratic(p0,_,p1) => { if index==0 {p0} else {p1} },
            Self::Cubic(p0,_,_,p1) =>   { if index==0 {p0} else {p1} },
        }
    }

    //mp endpoints
    /// Deconstruct and get the endpoints
    pub fn endpoints(self) -> (Point,Point) {
        match self {
            Self::Linear(p0,p1) =>      { (p0,p1) },
            Self::Quadratic(p0,_,p1) => { (p0,p1) },
            Self::Cubic(p0,_,_,p1) =>   { (p0,p1) },
        }
    }

    //mp get_distance
    /// Get the distance between the start and end points
    pub fn get_distance(&self) -> f64 {
        match self {
            Self::Linear(p0,p1)      => { p0.distance(p1) },
            Self::Quadratic(p0,_,p1) => { p0.distance(p1) },
            Self::Cubic(p0,_,_,p1)   => { p0.distance(p1) },
        }
    }

    //fp line
    /// Create a new Bezier that is a line between two points
    pub fn line(p0:&Point, p1:&Point) -> Self {
        Self::Linear(p0.clone(), p1.clone())
    }

    //fp quadratic
    /// Create a new Quadratic Bezier that is a line between two points
    /// with one absolute control points
    pub fn quadratic(p0:&Point, c:&Point, p1:&Point) -> Self {
        Self::Quadratic(p0.clone(), c.clone(), p1.clone())
    }

    //fp cubic
    /// Create a new Cubic Bezier that is a line between two points
    /// with two absolute control points
    pub fn cubic(p0:&Point, c0:&Point, c1:&Point, p1:&Point) -> Self {
        Self::Cubic(p0.clone(), c0.clone(), c1.clone(), p1.clone())
    }

    //mp is_line
    /// Returns true if the Bezier is a straight line
    pub fn is_line(&self) -> bool {
        match self { Self::Linear(_,_) => true, _ => false }
    }

    //mp is_quadratic
    /// Returns true if the Bezier is a quadratic
    pub fn is_quadratic(&self) -> bool {
        match self { Self::Quadratic(_,_,_) => true, _ => false }
    }

    //mp is_cubic
    /// Returns true if the Bezier is a cubic
    pub fn is_cubic(&self) -> bool {
        match self { Self::Cubic(_,_,_,_) => true, _ => false }
    }

    //mp bisect
    /// Returns two Bezier's that split the curve at parameter t=0.5
    ///
    /// For quadratics the midpoint is 1/4(p0 + 2*c + p1)
    pub fn bisect(&self) -> (Self, Self) {
        match self {
            Self::Linear(p0,p1) => {
                let pm = p0.clone().add(p1,1.).scale_xy(0.5,0.5);
                (Self::line(p0, &pm), Self::line(&pm, p1))
            },
            Self::Quadratic(p0,c,p1) => {
                let pm = c.clone().scale_xy(0.5,0.5).add(p1,0.25).add(p0,0.25);
                let c0 = c.clone().scale_xy(0.5,0.5).add(p0,0.5);
                let c1 = c.clone().scale_xy(0.5,0.5).add(p1,0.5);
                (Self::quadratic(p0, &c0, &pm), Self::quadratic(&pm, &c1, p1))
            },
            Self::Cubic(p0,c0,c1,p1) => {
                let pm = c0.clone().scale_xy(0.375,0.375).add(p0,0.125).add(c1,0.375).add(p1,0.125);
                let c00 = c0.clone().scale_xy(0.5,0.5).add(p0,0.5);
                let c01 = c0.clone().scale_xy(0.5,0.5).add(p0,0.25).add(c1,0.25);
                let c10 = c1.clone().scale_xy(0.5,0.5).add(p1,0.25).add(c0,0.25);
                let c11 = c1.clone().scale_xy(0.5,0.5).add(p1,0.5);
                (Self::cubic(p0,&c00,&c01,&pm), Self::cubic(&pm,&c10,&c11,p1))
            },
        }
    }
        
    //mp bezier_between
    /// Returns the Bezier between two parameters 0 <= t0 < t1 <= 1
    pub fn bezier_between(&self, t0:f64, t1:f64) -> Self {
        match self {
            Self::Linear(p0,p1) => {
                let u0 = 1. - t0;
                let u1 = 1. - t1;
                let r0 = p0.clone().scale(u0).add(&p1,t0);
                let r1 = p0.clone().scale(u1).add(&p1,t1);
                Self::Linear(r0, r1)
            },
            Self::Quadratic(p0,c,p1) => {
                let u0 = 1. - t0;
                let u1 = 1. - t1;
                let rp0 = p0.clone().scale(u0*u0).add(&c, 2.*u0*t0).add(&p1,t0*t0);
                let rp1 = p0.clone().scale(u1*u1).add(&c, 2.*u1*t1).add(&p1,t1*t1);
                let rc0 = p0.clone().scale(u0*u1).add(&c, u0*t1+u1*t0).add(&p1,t1*t0);
                Self::Quadratic(rp0, rc0, rp1)
            },
            Self::Cubic(p0,c0,c1,p1) => {
                // simply: c0 = p0 + tangent(0)/3
                // and if we scale the curve to t1-t0 in size, tangents scale the same
                let rp0 = self.point_at(t0);
                let rt0 = self.tangent_at(t0);
                let rt1 = self.tangent_at(t1);
                let rp1 = self.point_at(t1);
                let t1_m_t0 = t1 - t0;
                let rc0 = rp0.clone().add(&rt0,t1_m_t0/3.);
                let rc1 = rp1.clone().add(&rt1,-t1_m_t0/3.);
                Self::Cubic(rp0, rc0, rc1, rp1)
            },
        }
    }
        
    //mp point_at
    /// Returns the point at parameter 't' along the Bezier
    pub fn point_at(&self, t:f64) -> Point {
        let omt = 1. - t;
        match self {
            Self::Linear(p0,p1) => {
                p0.clone().scale(omt).add(p1,t)
            },
            Self::Quadratic(p0,c,p1) => {
                let p0_sc = omt*omt;
                let c_sc  = omt*t*2.;
                let p1_sc = t*t;
                c.clone().scale(c_sc).add(p0,p0_sc).add(p1,p1_sc)
            },
            Self::Cubic(p0,c0,c1,p1) => {
                let p0_sc = omt*omt*omt;
                let c0_sc = omt*omt*t*3.;
                let c1_sc = omt*t*t*3.;
                let p1_sc = t*t*t*1.;
                c0.clone().scale(c0_sc).add(p0,p0_sc).add(c1,c1_sc).add(p1,p1_sc)
            },
        }
    }
        
    //mp tangent_at
    /// Returns the tangent vector at parameter 't' along the Bezier
    /// 
    /// This is not necessarilly a unit vector
    pub fn tangent_at(&self, t:f64) -> Point {
        match self {
            Self::Linear(p0,p1) => {
                p1.clone().add(p0,-1.)
            },
            Self::Quadratic(p0,c,p1) => {
                let p0_sc = 2.*t - 2.; // d/dt (1-t)^2 
                let c_sc  = 2. - 4.*t; // d/dt 2t(1-t)
                let p1_sc = 2.*t     ; // d/dt t^2
                c.clone().scale(c_sc).add(p0,p0_sc).add(p1,p1_sc)
            },
            Self::Cubic(p0,c0,c1,p1) => {
                let p0_sc = 6.*t - 3.*t*t - 3. ; // d/dt (1-t)^3 
                let c0_sc = 9.*t*t - 12.*t + 3.; // d/dt 3t(1-t)^2
                let c1_sc = 6.*t - 9.*t*t      ; // d/dt 3t^2(1-t)
                let p1_sc = 3.*t*t             ; // d/dt t^3
                c0.clone().scale(c0_sc).add(p0,p0_sc).add(c1,c1_sc).add(p1,p1_sc)
            },
        }
    }
        
    //cp scale_xy
    /// Consume the Bezier and return a new Bezier scaled separately in X and Y by two scaling parameters
    pub fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        match &mut self {
            Self::Linear(ref mut p0, ref mut p1) => {
                *p0 = p0.scale_xy(sx,sy);
                *p1 = p1.scale_xy(sx,sy);
            },
            Self::Quadratic(ref mut p0, ref mut c, ref mut p1) => {
                *p0 = p0.scale_xy(sx,sy);
                *c  = c.scale_xy(sx,sy);
                *p1 = p1.scale_xy(sx,sy);
            },
            Self::Cubic(ref mut p0, ref mut c0, ref mut c1, ref mut p1) => {
                *p0 = p0.scale_xy(sx,sy);
                *c0 = c0.scale_xy(sx,sy);
                *c1 = c1.scale_xy(sx,sy);
                *p1 = p1.scale_xy(sx,sy);
            },
        }
        self
    }

    //cp rotate
    /// Consume the Bezier and return a new Bezier rotated
    /// anticlockwise around the origin by the angle in degrees
    pub fn rotate(mut self, degrees:f64) -> Self {
        match &mut self {
            Self::Linear(ref mut p0, ref mut p1) => {
                *p0 = p0.rotate(degrees);
                *p1 = p1.rotate(degrees);
            },
            Self::Quadratic(ref mut p0, ref mut c, ref mut p1) => {
                *p0 = p0.rotate(degrees);
                *c =  c.rotate(degrees);
                *p1 = p1.rotate(degrees);
            },
            Self::Cubic(ref mut p0, ref mut c0, ref mut c1, ref mut p1) => {
                *p0 = p0.rotate(degrees);
                *c0 = c0.rotate(degrees);
                *c1 = c1.rotate(degrees);
                *p1 = p1.rotate(degrees);
            },
        }
        self
    }

    //fp of_round_corner
    /// Create a Cubic Bezier that is a circular arc focused on the corner point,
    /// with v0 and v1 are vectors in to the point
    pub fn of_round_corner(corner:&Point, v0:&Point, v1:&Point, radius:f64) -> Self {
        // println!("corner {} vec0 {} vec1 {} radius {}",corner,v0,v1,radius);
        let reverse = v0.x*v1.y - v1.x*v0.y > 0.;
        let rl0 = 1.0/v0.len();
        let rl1 = 1.0/v1.len();
        let v0u = Point::new(v0.x*rl0, v0.y*rl0);
        let v1u = Point::new(v1.x*rl1, v1.y*rl1);
        let (v0u, v1u) = {if reverse { (v1u,v0u) } else { (v0u,v1u) } };
        drop(v0); drop (v1); // we only use the units, so this is defensive
        let n0u = Point::new(-v0u.y,v0u.x); // unit normal
        let n1u = Point::new(-v1u.y,v1u.x); // unit normal
        let k = radius / (n1u.dot(&v0u));
        // println!("k:{}",k);
        let center = Point::new( corner.x-k*(v0u.x+v1u.x), corner.y-k*(v0u.y+v1u.y) );
        let normal_diff = Point::new(n0u.x-n1u.x, n0u.y-n1u.y);
        let vector_sum  = Point::new(v0u.x+v1u.x, v0u.y+v1u.y);
        let l2 = vector_sum.x*vector_sum.x + vector_sum.y*vector_sum.y;
        let l = l2.sqrt();
        let lambda = 4.0*radius/(3.*l2) * (2.*l + (normal_diff.x*vector_sum.x + normal_diff.y*vector_sum.y));
        // println!("{:?} {:?} {:?} {:?} {:?}",center,v0,normal_diff,vector_sum, lambda);
        let p0 = Point::new(center.x-radius*n0u.x, center.y-radius*n0u.y);
        let p1 = Point::new(center.x+radius*n1u.x, center.y+radius*n1u.y);
        let c0 = Point::new(p0.x + lambda * v0u.x, p0.y + lambda * v0u.y);
        let c1 = Point::new(p1.x + lambda * v1u.x, p1.y + lambda * v1u.y);
        let bezier = { if reverse { Self::Cubic(p1,c1,c0,p0) } else { Self::Cubic(p0,c0,c1,p1) } };
        // println!("Bezier {}",bezier);
        bezier
    }

    //fp arc
    /// Create a Cubic Bezier that approximates closely a circular arc
    /// given a centre point and a radius, of a certain angle, rotated
    /// around the origin by the rotate parameter
    pub fn arc(angle:f64, radius:f64, center:&Point, rotate:f64) -> Self {
        let half_angle = (0.5*angle).to_radians();
        let s = half_angle.sin();
        let lambda = radius * 4./3. * (1./s-1.);

        let d0a = rotate.to_radians();
        let d0s = d0a.sin();
        let d0c = d0a.cos();
        let d1a = (rotate+angle).to_radians();
        let d1s = d1a.sin();
        let d1c = d1a.cos();

        let p0 = Point::new(center.x+d0c*radius,           center.y+d0s*radius);
        let c0 = Point::new(center.x+d0c*radius-lambda*d0s,center.y+d0s*radius+lambda*d0c);
        let p1 = Point::new(center.x+d1c*radius,           center.y+d1s*radius);
        let c1 = Point::new(center.x+d1c*radius+lambda*d1s,center.y+d1s*radius-lambda*d1c);
        Self::Cubic(p0,c0,c1,p1)
    }

    //mp is_straight
    /// Returns true if the Bezier is straighter than a 'straightness' measure
    ///
    /// `straightness` is independent of the length of the Bezier
    pub fn is_straight(&self, straightness:f64) -> bool {
        match self {
            Self::Cubic(p0,c0,c1,p1) => {
                let pn = Point::new(p0.y - p1.y, p1.x - p0.x);
                let a0 = c0.clone().add(p0,-1.).dot(&pn).abs();
                let a1 = c1.clone().add(p1,-1.).dot(&pn).abs();
                (a0 + a1) < straightness * pn.len2()
            },
            Self::Quadratic(p0,c,p1) => {
                let pn = Point::new(p0.y - p1.y, p1.x - p0.x);
                let a0 = c.clone().add(p0,-1.).dot(&pn).abs();
                a0 < straightness * pn.len2()
            },
            _ => true,
        }
    }
        
    //mp length
    /// Calculates the length given a straightness
    ///
    /// `straightness` is independent of the length of the Bezier
    pub fn length(&self, straightness:f64) -> f64 {
        if self.is_straight(straightness) {
            self.get_distance()
        } else {
            let (b0, b1) = self.bisect();
            b0.length(straightness) + b1.length(straightness)
        }
    }
        
    //mp t_of_distance
    /// Calculates the parameter 't' at a certain distance along the Bezier given a straightness
    ///
    /// `straightness` is independent of the length of the Bezier
    ///
    /// Returns t,true if the distance is along the Bezier
    /// Returns 0.,false if the distance is before the start of the Bezier
    /// Returns 1.,false if the distance is beyond the end of the Bezier
    pub fn t_of_distance(&self, straightness:f64, distance:f64) -> (f64, bool) {
        if distance < 0. {
            (0.,false)
        } else {
            match self.t_of_distance_rec(straightness, distance, 0., 1., 0.) {
                (None, _)    => (1., false),
                (Some(t), _) => (t, true),
            }
        }
    }
    fn t_of_distance_rec(&self, straightness:f64, distance:f64, t_start:f64, t_scale:f64, acc_length:f64) -> (Option<f64>, f64) {
        // println!("t_of_distance_rec {} {} {} {} {}",straightness, distance, t_start, t_scale, acc_length);
        if distance <= acc_length {
            (Some(t_start), 0.)
        } else if self.is_straight(straightness) {
            let d     = self.get_distance();
            if distance > acc_length+d {
                (None, acc_length+d)
            } else if d < 1E-8 {
                (Some(t_start + t_scale), acc_length+d)
            } else {
                let rel_d = distance - acc_length;
                (Some(t_start + t_scale * rel_d / d), acc_length+d)
            }
        } else {
            let t_subscale = t_scale / 2.;
            let (b0, b1) = self.bisect();
            match b0.t_of_distance_rec(straightness, distance, t_start, t_subscale, acc_length) {
                (None, length) => {
                    b1.t_of_distance_rec( straightness, distance, t_start + t_subscale, t_subscale, length )
                }
                r => r
            }
        }
    }    
        
    //mp as_lines
    /// Iterate over line segments that are 'straight' enough
    pub fn as_lines(&self, straightness:f64) -> BezierLineIter {
        BezierLineIter::new(self, straightness)
    }
        
    //mp as_points
    /// Iterate over points that make 'straight' enough lines
    pub fn as_points(&self, straightness:f64) -> BezierPointIter {
        BezierPointIter::new(BezierLineIter::new(self, straightness))
    }
        
    //zz All done
}

//a Test
#[cfg(test)]
mod test_bezier {
    use super::*;
    //fi pt_eq
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    //fi approx_eq
    pub fn approx_eq(a:f64, b:f64, tolerance:f64, msg:&str) {
        assert!((a-b).abs()<tolerance, "{} {} {}",msg,a,b);
    }
    //fi bezier_eq
    pub fn bezier_eq(bez:&Bezier, v:Vec<(f64,f64)>) {
        match bez {
            Bezier::Cubic(p0,c0,c1,p1) => {
                pt_eq(p0, v[0].0, v[0].1);
                pt_eq(c0, v[1].0, v[1].1);
                pt_eq(c1, v[2].0, v[2].1);
                pt_eq(p1, v[3].0, v[3].1);
            }
            _ => {},
        }
    }
    //fi bezier_straight_as
    fn bezier_straight_as( bezier:&Bezier, straightness:f64 ) {
        for i in 0..30 {
            let s = (1.4_f64).powf(i as f64 - 15.);
            println!("{} {} {}",s,straightness, bezier.is_straight(s));
            assert_eq!( straightness < s, bezier.is_straight(s), "Bezier {} .is_straight({}) failed for {}",bezier, s, straightness);
        }
    }
    //fi does_bisect
    fn does_bisect(bezier:&Bezier) {
        let (b0,b1) = bezier.bisect();
        println!("Test bisection of {} into {}, {}",bezier, b0, b1);
        for i in 0..21 {
            let t = (i as f64) / 20.0;
            let p0 = bezier.point_at(t * 0.5);
            let p1 = bezier.point_at(t * 0.5 + 0.5);
            println!("t {} : {} : {}",t,p0,p1);
            pt_eq(&b0.point_at(t), p0.x, p0.y);
            pt_eq(&b1.point_at(t), p1.x, p1.y);
        }
    }
    //fi does_split
    fn does_split(bezier:&Bezier, t0:f64, t1:f64) {
        let b = bezier.bezier_between(t0, t1);
        for i in 0..21 {
            let bt = (i as f64) / 20.0;
            let t  = t0 + (t1 - t0) * bt;
            let p  = bezier.point_at(t);
            let pb = b.point_at(bt);
            println!("t {} : {} : {}",t,p,pb);
            approx_eq(p.x, pb.x, 1E-6, &format!("Bezier split x {} {} {} : {} : {}", t, t0, t1, bezier, b));
            approx_eq(p.y, pb.y, 1E-6, &format!("Bezier split y {} {} {} : {} : {}", t, t0, t1, bezier, b));
        }
    }
    //fi test_line
    #[test]
    fn test_line() {
        let p0 = Point::origin();
        let p1 = Point::new(10.,0.);
        let p2 = Point::new(10.,1.);
        let b01 = Bezier::line(&p0, &p1);
        let b02 = Bezier::line(&p0, &p2);

        pt_eq( &b01.point_at(0.), p0.x, p0.y );
        pt_eq( &b01.point_at(0.5), (p0.x+p1.x)/2., (p0.y+p1.y)/2. );
        pt_eq( &b01.point_at(1.), p1.x, p1.y );
        pt_eq( &b02.point_at(0.), p0.x, p0.y );
        pt_eq( &b02.point_at(0.5), (p0.x+p2.x)/2., (p0.y+p2.y)/2. );
        pt_eq( &b02.point_at(1.), p2.x, p2.y );

        pt_eq( &b01.bisect().0.point_at(0.), p0.x, p0.y );
        pt_eq( &b01.bisect().0.point_at(1.), (p0.x+p1.x)/2., (p0.y+p1.y)/2. );
        pt_eq( &b01.bisect().1.point_at(0.), (p0.x+p1.x)/2., (p0.y+p1.y)/2. );
        pt_eq( &b01.bisect().1.point_at(1.), p1.x, p1.y );


        does_split(&b01, 0., 1.);
        does_split(&b01, 0.1, 0.3);
        does_split(&b01, 0.3, 0.7);
        does_split(&b01, 0.7, 1.0);

        does_split(&b02, 0., 1.);
        does_split(&b02, 0.1, 0.3);
        does_split(&b02, 0.3, 0.7);
        does_split(&b02, 0.7, 1.0);

        does_bisect(&b01);
        does_bisect(&b02);

        pt_eq( &b01.tangent_at(0.),  p1.x-p0.x, p1.y-p0.y );
        pt_eq( &b01.tangent_at(0.5), p1.x-p0.x, p1.y-p0.y );
        pt_eq( &b01.tangent_at(1.0), p1.x-p0.x, p1.y-p0.y );
        pt_eq( &b02.tangent_at(0.),  p2.x-p0.x, p2.y-p0.y );
        pt_eq( &b02.tangent_at(0.5), p2.x-p0.x, p2.y-p0.y );
        pt_eq( &b02.tangent_at(1.0), p2.x-p0.x, p2.y-p0.y );

        let mut v = Vec::new();
        v.clear();
        for (a,_b) in b01.as_lines(0.1) {
            v.push(a);
        }
        assert_eq!(v.len(), 1, "We know that at any straightness there must be 1 line segments" );
    }
    //fi test_quadratic
    #[test]
    fn test_quadratic() {
        let p0 = Point::origin();
        let p1 = Point::new(10.,0.);
        let p2 = Point::new(10.,1.);
        let b = Bezier::quadratic(&p0, &p1, &p2);

        pt_eq( &b.point_at(0.), p0.x, p0.y );
        pt_eq( &b.point_at(0.5), (p0.x+p2.x)/4.+p1.x/2., (p0.y+p2.y)/4.+p1.y/2. );
        pt_eq( &b.point_at(1.), p2.x, p2.y );

        does_bisect(&b);

        does_split(&b, 0., 1.);
        does_split(&b, 0.1, 0.3);
        does_split(&b, 0.3, 0.7);
        does_split(&b, 0.7, 1.0);

        pt_eq( &b.tangent_at(0.),  2.*(p1.x-p0.x), 2.*(p1.y-p0.y) );
        // pt_eq( &b.tangent_at(0.5), p1.x-p0.x, p1.y-p0.y );
        pt_eq( &b.tangent_at(1.0), 2.*(p2.x-p1.x), 2.*(p2.y-p1.y) );

        let mut v = Vec::new();
        v.clear();
        for (a,_b) in b.as_lines(0.1) {
            v.push(a);
        }
        assert_eq!(v.len(), 1, "We know that at straightness 0.1 there must be 1 line segments" );

        let mut v = Vec::new();
        v.clear();
        for (a,_b) in b.as_lines(0.01) {
            v.push(a);
        }
        assert_eq!(v.len(), 52, "We know that at straightness 0.01  there must be 52 line segments" );
    }
    //fi test_cubic
    #[test]
    fn test_cubic() {
        let p0 = Point::origin();
        let p1 = Point::new(10.,0.);
        let p2 = Point::new(6.,1.);
        let p3 = Point::new(20.,5.);
        let b = Bezier::cubic(&p0, &p1, &p2, &p3);

        pt_eq( &b.point_at(0.), p0.x, p0.y );
        pt_eq( &b.point_at(1.), p3.x, p3.y );

        pt_eq( &b.tangent_at(0.),  3.*(p1.x-p0.x), 3.*(p1.y-p0.y) );
        // pt_eq( &b.tangent_at(0.5), p1.x-p0.x, p1.y-p0.y );
        pt_eq( &b.tangent_at(1.0), 3.*(p3.x-p2.x), 3.*(p3.y-p2.y) );

        does_bisect(&b);

        does_split(&b, 0., 1.);
        does_split(&b, 0.1, 0.3);
        does_split(&b, 0.3, 0.7);
        does_split(&b, 0.7, 1.0);
        
        let x = Bezier::arc(90.,1.,&Point::new(0.,0.),0.);
        use std::f64::consts::PI;
        approx_eq( 0.5, x.length(0.001) / PI, 0.001, "Length of 90-degree arc of circle radius 1 should be PI/2");

        approx_eq( 0.5,   x.t_of_distance(0.001, PI/4.).0, 0.001, "t of half-way round 90-degree arc of circle radius 1");
        approx_eq( 0.245, x.t_of_distance(0.001, PI/8.).0, 0.001, "t of quarter-way round 90-degree arc of circle radius 1");
        approx_eq( 0.755, x.t_of_distance(0.001, PI*3./8.).0, 0.001, "t of three-quarters-way round 90-degree arc of circle radius 1");
        
        let mut v = Vec::new();
        v.clear();
        for (a,_b) in b.as_lines(0.1) {
            v.push(a);
        }
        assert_eq!(v.len(), 3, "We know that at straightness 0.1 there should be 3 line segments" );

        v.clear();
        for (a,_b) in b.as_lines(0.01) {
            v.push(a);
        }
        assert_eq!(v.len(), 24, "We know that at straightness 0.01 there should be 24 line segments" );
    }
    //fi test_straight
    #[test]
    fn test_straight() {
        let p0 = Point::origin();
        let p1 = Point::new(10.,0.);
        let p2 = Point::new(10.,1.);
        let p3 = Point::new(20.,0.);
        let p4 = Point::new(20.,1.);
        let sp0 = p0.clone().scale_xy(10.,10.);
        let sp1 = p1.clone().scale_xy(10.,10.);
        let sp2 = p2.clone().scale_xy(10.,10.);
        let sp3 = p3.clone().scale_xy(10.,10.);
        let sp4 = p4.clone().scale_xy(10.,10.);
        
        bezier_straight_as( &Bezier::line(&p0, &p1), 1E-10 );
        bezier_straight_as( &Bezier::line(&p0, &p2), 1E-10 );
        bezier_straight_as( &Bezier::line(&p0, &p3), 1E-10 );
        bezier_straight_as( &Bezier::line(&p0, &p4), 1E-10 );
        bezier_straight_as( &Bezier::line(&sp0, &sp1), 1E-10 );
        bezier_straight_as( &Bezier::line(&sp0, &sp2), 1E-10 );
        bezier_straight_as( &Bezier::line(&sp0, &sp3), 1E-10 );
        bezier_straight_as( &Bezier::line(&sp0, &sp4), 1E-10 );

        bezier_straight_as( &Bezier::quadratic(&p0, &p1, &p3),  1E-10 );
        bezier_straight_as( &Bezier::quadratic(&sp0, &sp1, &sp3), 1E-10 );

        bezier_straight_as( &Bezier::quadratic(&p0, &p2, &p3), 0.05 );
        bezier_straight_as( &Bezier::quadratic(&sp0, &sp2, &sp3), 0.05 );

        bezier_straight_as( &Bezier::quadratic(&p0, &p1, &p4),  0.03 );
        bezier_straight_as( &Bezier::quadratic(&sp0, &sp1, &sp4), 0.03 );

        bezier_straight_as( &Bezier::cubic(&p0, &p1, &p2, &p3), 0.05 );
        bezier_straight_as( &Bezier::cubic(&sp0, &sp1, &sp2, &sp3), 0.05 );

        bezier_straight_as( &Bezier::cubic(&p0, &p1, &p2, &p4), 0.065 );
        bezier_straight_as( &Bezier::cubic(&sp0, &sp1, &sp2, &sp4), 0.065 );
    }
    //fi test_arc
    #[test]
    fn test_arc() {
        let sqrt2 = 2.0_f64.sqrt();
        let r_sqrt2 = 1.0 / sqrt2;
        let magic = 0.5522847498307935;
        let magic2 = magic * r_sqrt2;
        let x = Bezier::arc(90.,1.,&Point::new(0.,0.),0.);
        bezier_eq(&x, vec![(1.,0.), (1.,magic), (magic,1.), (0.,1.)]);
        let x = Bezier::arc(90.,1.,&Point::new(0.,0.),-90.);
        bezier_eq(&x, vec![(0.,-1.), (magic,-1.), (1.,-magic), (1.,0.)]);
        let x = Bezier::of_round_corner(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 1.);
        bezier_eq(&x, vec![(1.,0.), (1.,magic), (magic,1.), (0.,1.)]);
        let x = Bezier::of_round_corner(&Point::new(sqrt2,0.), &Point::new(1.,1.), &Point::new(1.,-1.), 1.);
        bezier_eq(&x, vec![(r_sqrt2, -r_sqrt2), (r_sqrt2+magic2 , -r_sqrt2+magic2), (r_sqrt2+magic2, r_sqrt2-magic2), (r_sqrt2, r_sqrt2)]);
        pt_eq(x.borrow_pt(0), r_sqrt2, -r_sqrt2);
        pt_eq(x.borrow_pt(1), r_sqrt2, r_sqrt2);
        let x = Bezier::of_round_corner(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 0.5);
        println!("{:?}",x);
        // assert_eq!(true,false);
    }
    //fi All done
}
