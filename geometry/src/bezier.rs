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

@file    bezier.rs
@brief   Part of geometry library
 */

//a Imports
use super::vector::{Vector, VectorCoord};
use super::bezier_line::BezierLineIter;
use super::bezier_point::BezierPointIter;

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
pub struct Bezier<V:VectorCoord, const D:usize> {
    /// Number of valid control points (2-4)
    num : usize,
    /// Control points - endpoints are always 0 and 1
    pts : [Vector<V,D>; 4],
}

//ti Display for Bezier
impl <V:VectorCoord, const D:usize> std::fmt::Display for Bezier<V,D> {

    //mp fmt - format a `Bezier` for display
    /// Display the `Bezier' as sets of points
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.num {
            2  => write!(f, "[{}<->{}]", self.pts[0], self.pts[1]),
            3  => write!(f, "[{}<-:{}:->{}]", self.pts[0], self.pts[2], self.pts[1]),
            _  => write!(f, "[{}<-:{}:{}:->{}]", self.pts[0], self.pts[2], self.pts[3], self.pts[1]),
        }
    }

    //zz All done
}

//ip Bezier
impl <V:VectorCoord, const D:usize> Bezier<V,D> {
    //mp borrow_pt
    /// Get the start or end point of the Bezier - index 0 gives the
    /// start point, index 1 the end point
    pub fn borrow_pt(&self, index:usize) -> &Vector<V,D> {
        &self.pts[index]
    }

    //mp endpoints
    /// Deconstruct and get the endpoints
    pub fn endpoints(self) -> (Vector<V,D>, Vector<V,D>) {
        (self.pts[0], self.pts[1])
    }

    //mp get_distance
    /// Get the distance between the start and end points
    pub fn get_distance(&self) -> V {
        self.pts[0].distance_to(self.pts[1])
    }

    //fp line
    /// Create a new Bezier that is a line between two points
    pub fn line(p0:&Vector<V,D>, p1:&Vector<V,D>) -> Self {
        Self { num:2, pts:[p0.clone(), p1.clone(), V::origin(), V::origin()] }
    }

    //fp quadratic
    /// Create a new Quadratic Bezier that is a line between two points
    /// with one absolute control points
    pub fn quadratic(p0:&Vector<V,D>, c:&Vector<V,D>, p1:&Vector<V,D>) -> Self {
        Self { num:3, pts:[p0.clone(), p1.clone(), c.clone(), V::origin()] }
    }

    //fp cubic
    /// Create a new Cubic Bezier that is a line between two points
    /// with two absolute control points
    pub fn cubic(p0:&Vector<V,D>, c0:&Vector<V,D>, c1:&Vector<V,D>, p1:&Vector<V,D>) -> Self {
        Self { num:4, pts:[p0.clone(), p1.clone(), c0.clone(), c1.clone()] }
    }

    //mp is_line
    /// Returns true if the Bezier is a straight line
    pub fn is_line(&self) -> bool { self.num == 2 }

    //mp is_quadratic
    /// Returns true if the Bezier is a quadratic
    pub fn is_quadratic(&self) -> bool { self.num == 3 }

    //mp is_cubic
    /// Returns true if the Bezier is a cubic
    pub fn is_cubic(&self) -> bool { self.num == 4 }

    //cp scale
    /// Consume the Bezier and return a new Bezier scaled separately in X and Y by two scaling parameters
    pub fn scale(mut self, s:V) -> Self {
        for i in 0..self.pts.len() {
            self.pts[i] = self.pts[i].scale(s);
        }
        self
    }

    //mp vector_of
    /// Returns a vector of a combination of the vectors of the bezier
    #[inline]
    pub fn vector_of(&self, sc:&[V], reduce:V) -> Vector<V,D> {
        let mut r = self.pts[0].clone().scale(sc[0]);
        for i in 1..sc.len() {
            r = r.add(*self.pts[i], sc[i]);
        }
        r.reduce(reduce)
    }

    //mp point_at
    /// Returns the point at parameter 't' along the Bezier
    pub fn point_at(&self, t:V) -> Vector<V,D> {
        let omt = 1. - t;
        match self.num {
            2 => {
                self.vector_of(&[omt, t], 1.)
            },
            3 => {
                let p0_sc = omt*omt;
                let c_sc  = omt*t*2.;
                let p1_sc = t*t;
                self.vector_of(&[p0_sc, p1_sc, c_sc], 1.)
            },
            _  => {
                let p0_sc = omt*omt*omt;
                let c0_sc = omt*omt*t*3.;
                let c1_sc = omt*t*t*3.;
                let p1_sc = t*t*t*1.;
                self.vector_of(&[p0_sc, p1_sc, c0_sc, c1_sc], 1.)
            },
        }
    }

    //mp tangent_at
    /// Returns the tangent vector at parameter 't' along the Bezier
    ///
    /// This is not necessarily a unit vector
    pub fn tangent_at(&self, t:V) -> Vector<V,D> {
        match self.num {
            2 => {
                self.vector_of(&[1., -1.], 1.)
            },
            3 => {
                let p0_sc = t - 1.;    // d/dt (1-t)^2
                let c_sc  = 1. - 2.*t; // d/dt 2t(1-t)
                let p1_sc = t;         // d/dt t^2
                self.vector_of(&[p0_sc, p1_sc, c_sc], 1.)
            },
            Self::Cubic(p0,c0,c1,p1) => {
                let p0_sc = 2.*t   - t*t - 1. ; // d/dt (1-t)^3
                let c0_sc = 3.*t*t - 4.*t + 1.; // d/dt 3t(1-t)^2
                let c1_sc = 2.*t   - 3.*t*t   ; // d/dt 3t^2(1-t)
                let p1_sc = t*t               ; // d/dt t^3
                self.vector_of(&[p0_sc, p1_sc, c0_sc, c1_sc], 1.)
            },
        }
    }

    //mp bisect
    /// Returns two Bezier's that split the curve at parameter t=0.5
    ///
    /// For quadratics the midpoint is 1/4(p0 + 2*c + p1)
    pub fn bisect(&self) -> (Self, Self) {
        let zero = V::zero();
        let one  = V::one();
        let two  = V::from(2);
        let three  = V::from(3);
        match self.num {
            2 => {
                let pm = self.vector_of(&[one,one],two);
                (Self::line(&self.pts[0], &pm), Self::line(&pm, &self.pts[1]))
            },
            3 => {
                let c0 = self.vector_of(&[one,zero,one],two);
                let c1 = self.vector_of(&[zero,one,one],two);
                let pm = c0.clone().add(c1.one).reduce(two);
                (Self::quadratic(&self.pts[0], &c0, &pm), Self::quadratic(&pm, &c1, &self.pts[1]))
            },
            _ => {
                let pm  = self.vector_of(&[one,three,three,one],V::from(8));
                let c00 = self.vector_of(&[one,zero,one],two);
                let c01 = self.vector_of(&[one,two,zero,one],V::from(4));
                let c10 = self.vector_of(&[one,zero,two,one],V::from(4));
                let c11 = self.vector_of(&[zero,one,zero,one],two);
                (Self::cubic(&self.pts[0],&c00,&c01,&pm), Self::cubic(&pm,&c10,&c11,&self.pts[1]))
            },
        }
    }

    //mp bezier_between
    /// Returns the Bezier between two parameters 0 <= t0 < t1 <= 1
    pub fn bezier_between(&self, t0:f64, t1:f64) -> Self {
        let p0 = &self.pts[0];
        let p1 = &self.pts[1];
        match self.num {
            2 => {
                let u0 = 1. - t0;
                let u1 = 1. - t1;
                let r0 = p0.clone().scale(u0).add(&p1,t0);
                let r1 = p0.clone().scale(u1).add(&p1,t1);
                Self::line(r0, r1)
            },
            3 => {
                let c = &self.pts[2];
                let u0 = 1. - t0;
                let u1 = 1. - t1;
                let rp0 = p0.clone().scale(u0*u0).add(&c, 2.*u0*t0).add(&p1,t0*t0);
                let rp1 = p0.clone().scale(u1*u1).add(&c, 2.*u1*t1).add(&p1,t1*t1);
                let rc0 = p0.clone().scale(u0*u1).add(&c, u0*t1+u1*t0).add(&p1,t1*t0);
                Self::quadratic(rp0, rc0, rp1)
            },
            _ => {
                let c0 = &self.pts[2];
                let c1 = &self.pts[3];
                // simply: c0 = p0 + tangent(0)/3
                // and if we scale the curve to t1-t0 in size, tangents scale the same
                let rp0 = self.point_at(t0);
                let rt0 = self.tangent_at(t0);
                let rt1 = self.tangent_at(t1);
                let rp1 = self.point_at(t1);
                let t1_m_t0 = t1 - t0;
                let rc0 = rp0.clone().add(&rt0,t1_m_t0/3.);
                let rc1 = rp1.clone().add(&rt1,-t1_m_t0/3.);
                Self::cubic(rp0, rc0, rc1, rp1)
            },
        }
    }

    //mp as_lines
    /// Iterate over line segments that are 'straight' enough
    pub fn as_lines(&self, straightness:V) -> BezierLineIter<V,D> {
        BezierLineIter::new(self, straightness)
    }

    //mp as_points
    /// Iterate over points that make 'straight' enough lines
    pub fn as_points(&self, straightness:V) -> BezierPointIter<V,D> {
        BezierPointIter::new(BezierLineIter::new(self, straightness))
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod test_bezier {
    use super::*;
    type Point = Vector<f64, 2>;
    //fi vec_eq
    pub fn vec_eq(v0:&Vector<f64,2>, v1:&Vector<f64,2>) {
        let d = v0.distance_to(v1);
        assert!(d<1E-8, "mismatch in {} {}",v0, v1);
    }
    //fi pt_eq
    pub fn pt_eq(v:&Point, x:f64, y:f64) {
        assert!((v.c[0]-x).abs()<1E-8, "mismatch in x {} {} {}",v,x,y);
        assert!((v.c[1]-y).abs()<1E-8, "mismatch in y {} {} {}",v,x,y);
    }
    //fi approx_eq
    pub fn approx_eq(a:f64, b:f64, tolerance:f64, msg:&str) {
        assert!((a-b).abs()<tolerance, "{} {} {}",msg,a,b);
    }
    //fi bezier_eq
    /*
    pub fn bezier_eq(bez:&Bezier<f64,2>, v:Vec<(f64,f64)>) {
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
*/
    //fi bezier_straight_as
    fn bezier_straight_as( bezier:&Bezier<f64,2>, straightness:f64 ) {
        for i in 0..30 {
            let s = (1.4_f64).powf(i as f64 - 15.);
            println!("{} {} {}",s,straightness, bezier.is_straight(s));
            assert_eq!( straightness < s, bezier.is_straight(s), "Bezier {} .is_straight({}) failed for {}",bezier, s, straightness);
        }
    }
    //fi does_bisect
    fn does_bisect(bezier:&Bezier<f64,2>) {
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
    fn does_split(bezier:&Bezier<f64,2>, t0:f64, t1:f64) {
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
        let p0 = Vector::origin();
        let p1 = Vector::new(&[10.,0.]);
        let p2 = Vector::new(&[10.,1.]);
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
        let p0 = Vector::origin();
        let p1 = Vector::new(&[10.,0.]);
        let p2 = Vector::new(&[10.,1.]);
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
        let p0 = Vector::origin();
        let p1 = Vector::new(&[10.,0.]);
        let p2 = Vector::new(&[6.,1.]);
        let p3 = Vector::new(&[20.,5.]);
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

        let x = Bezier::arc(90.,1.,&Vector::origin(),0.);
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
        let p0 = Vector::origin();
        let p1 = Vector::new(&[10.,0.]);
        let p2 = Vector::new(&[10.,1.]);
        let p3 = Vector::new(&[20.,0.]);
        let p4 = Vector::new(&[20.,1.]);
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
    /*
    #[test]
    fn test_arc() {
        let sqrt2 = 2.0_f64.sqrt();
        let r_sqrt2 = 1.0 / sqrt2;
        let magic = 0.5522847498307935;
        let magic2 = magic * r_sqrt2;
        let x = Bezier::arc(90.,1.,&Vector<V,D>::new(0.,0.),0.);
        bezier_eq(&x, vec![(1.,0.), (1.,magic), (magic,1.), (0.,1.)]);
        let x = Bezier::arc(90.,1.,&Vector<V,D>::new(0.,0.),-90.);
        bezier_eq(&x, vec![(0.,-1.), (magic,-1.), (1.,-magic), (1.,0.)]);
        let x = Bezier::of_round_corner(&Vector<V,D>::new(1.,1.), &Vector<V,D>::new(0.,3.), &Vector<V,D>::new(0.5,0.), 1.);
        bezier_eq(&x, vec![(1.,0.), (1.,magic), (magic,1.), (0.,1.)]);
        let x = Bezier::of_round_corner(&Vector<V,D>::new(sqrt2,0.), &Vector<V,D>::new(1.,1.), &Vector<V,D>::new(1.,-1.), 1.);
        bezier_eq(&x, vec![(r_sqrt2, -r_sqrt2), (r_sqrt2+magic2 , -r_sqrt2+magic2), (r_sqrt2+magic2, r_sqrt2-magic2), (r_sqrt2, r_sqrt2)]);
        pt_eq(x.borrow_pt(0), r_sqrt2, -r_sqrt2);
        pt_eq(x.borrow_pt(1), r_sqrt2, r_sqrt2);
        let x = Bezier::of_round_corner(&Vector<V,D>::new(1.,1.), &Vector<V,D>::new(0.,3.), &Vector<V,D>::new(0.5,0.), 0.5);
        println!("{:?}",x);
        // assert_eq!(true,false);
    }
*/
    //fi All done
}
