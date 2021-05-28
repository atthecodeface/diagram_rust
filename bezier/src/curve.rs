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
use geometry::vector;
use geometry::Float;
use crate::BezierLineIter;
use crate::BezierPointIter;

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
pub struct Bezier<V:Float, const D:usize> {
    /// Number of valid control points (2-4)
    num : usize,
    /// Control points - endpoints are always 0 and 1
    pts : [[V;D];4],
}

//ti Display for Bezier
impl <V:Float, const D:usize> std::fmt::Display for Bezier<V,D> {

    //mp fmt - format a `Bezier` for display
    /// Display the `Bezier' as sets of points
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[" )?;
        vector::fmt(f,&self.pts[0])?;
        write!(f, "<-" )?;
        if self.num > 2 {vector::fmt(f,&self.pts[2])?;}
        if self.num > 3 {
            write!(f, ":" )?;
            vector::fmt(f,&self.pts[3])?;
        }
        write!(f, "->" )?;
        vector::fmt(f,&self.pts[1])
    }

    //zz All done
}

//ip Bezier
impl <V:Float, const D:usize> Bezier<V,D> {
    //mp borrow_pt
    /// Get the start or end point of the Bezier - index 0 gives the
    /// start point, index 1 the end point
    pub fn borrow_pt(&self, index:usize) -> &[V;D] {
        &self.pts[index]
    }

    //mp endpoints
    /// Deconstruct and get the endpoints
    pub fn endpoints(self) -> ([V;D], [V;D]) {
        (self.pts[0], self.pts[1])
    }

    //mp get_distance
    /// Get the distance between the start and end points
    pub fn get_distance(&self) -> V {
        vector::distance(&self.pts[0], &self.pts[1])
    }

    //fp line
    /// Create a new Bezier that is a line between two points
    pub fn line(p0:&[V;D], p1:&[V;D]) -> Self {
        Self { num:2, pts:[p0.clone(), p1.clone(), vector::zero(), vector::zero()] }
    }

    //fp quadratic
    /// Create a new Quadratic Bezier that is a line between two points
    /// with one absolute control points
    pub fn quadratic(p0:&[V;D], c:&[V;D], p1:&[V;D]) -> Self {
        Self { num:3, pts:[p0.clone(), p1.clone(), c.clone(), vector::zero()] }
    }

    //fp cubic
    /// Create a new Cubic Bezier that is a line between two points
    /// with two absolute control points
    pub fn cubic(p0:&[V;D], c0:&[V;D], c1:&[V;D], p1:&[V;D]) -> Self {
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

    //mp scale
    /// Consume the Bezier and return a new Bezier scaled separately in X and Y by two scaling parameters
    pub fn scale(&mut self, s:V)  {
        for i in 0..self.pts.len() {
            self.pts[i] = vector::scale(self.pts[i], s);
        }
    }

    //mp rotate_around
    /// Rotate the Bezier and return a new Bezier rotated around a
    /// *pivot* point anticlockwise by the specified angle
    pub fn rotate_around(&mut self, pivot:&[V;D], angle:V, c0:usize, c1:usize) {
        for p in self.pts.iter_mut() {
            *p = vector::rotate_around(*p, pivot, angle, c0, c1);
        }
    }

    //mp vector_of
    /// Returns a vector of a combination of the vectors of the bezier
    #[inline]
    pub fn vector_of(&self, sc:&[V], reduce:V) -> [V;D] {
        let mut r = vector::scale(self.pts[0],sc[0]);
        for i in 1..sc.len() {
            r = vector::add(r, &self.pts[i], sc[i]);
        }
        vector::reduce(r, reduce)
    }

    //mp point_at
    /// Returns the point at parameter 't' along the Bezier
    pub fn point_at(&self, t:V) -> [V;D] {
        let one = V::one();
        let two = V::from(2).unwrap();
        let three = V::from(3).unwrap();
        let omt = one - t;
        match self.num {
            2 => {
                self.vector_of(&[omt, t], one)
            },
            3 => {
                let p0_sc =       omt*omt;
                let c_sc  = two * omt*t;
                let p1_sc =       t*t;
                self.vector_of(&[p0_sc, p1_sc, c_sc], one)
            },
            _  => {
                let p0_sc =         omt*omt*omt;
                let c0_sc = three * omt*omt*t;
                let c1_sc = three * omt*t*t;
                let p1_sc =         t*t*t;
                self.vector_of(&[p0_sc, p1_sc, c0_sc, c1_sc], one)
            },
        }
    }

    //mp tangent_at
    /// Returns the tangent vector at parameter 't' along the Bezier
    ///
    /// This is not necessarily a unit vector
    pub fn tangent_at(&self, t:V) -> [V;D] {
        let one = V::one();
        let two = V::from(2).unwrap();
        let three = V::from(3).unwrap();
        let four = V::from(4).unwrap();
        match self.num {
            2 => {
                self.vector_of(&[-one, one], one)
            },
            3 => {
                let p0_sc = t - one;    // d/dt (1-t)^2
                let c_sc  = one - two*t; // d/dt 2t(1-t)
                let p1_sc = t;         // d/dt t^2
                self.vector_of(&[p0_sc, p1_sc, c_sc], one)
            },
            _ => {
                let p0_sc = two*t     - t*t - one ; // d/dt (1-t)^3
                let c0_sc = three*t*t - four*t + one; // d/dt 3t(1-t)^2
                let c1_sc = two*t     - three*t*t   ; // d/dt 3t^2(1-t)
                let p1_sc = t*t               ; // d/dt t^3
                self.vector_of(&[p0_sc, p1_sc, c0_sc, c1_sc], one)
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
        let two  = V::from(2).unwrap();
        let three  = V::from(3).unwrap();
        match self.num {
            2 => {
                let pm = self.vector_of(&[one,one],two);
                (Self::line(&self.pts[0], &pm), Self::line(&pm, &self.pts[1]))
            },
            3 => {
                let c0 = self.vector_of(&[one,zero,one],two);
                let c1 = self.vector_of(&[zero,one,one],two);
                let pm = vector::reduce(vector::add(c0,&c1,one),two);
                (Self::quadratic(&self.pts[0], &c0, &pm), Self::quadratic(&pm, &c1, &self.pts[1]))
            },
            _ => {
                let pm  = self.vector_of(&[one,one,three,three],V::from(8).unwrap());
                let c00 = self.vector_of(&[one,zero,one],two);
                let c01 = self.vector_of(&[one,zero,two,one],V::from(4).unwrap());
                let c10 = self.vector_of(&[zero,one,one,two],V::from(4).unwrap());
                let c11 = self.vector_of(&[zero,one,zero,one],two);
                (Self::cubic(&self.pts[0],&c00,&c01,&pm), Self::cubic(&pm,&c10,&c11,&self.pts[1]))
            },
        }
    }

    //mp bezier_between
    /// Returns the Bezier between two parameters 0 <= t0 < t1 <= 1
    pub fn bezier_between(&self, t0:V, t1:V) -> Self {
        let two = V::from(2).unwrap();
        let p0 = &self.pts[0];
        let p1 = &self.pts[1];
        match self.num {
            2 => {
                let u0 = V::one() - t0;
                let u1 = V::one() - t1;
                let r0 = vector::add(vector::scale(self.pts[0],u0),&p1,t0);
                let r1 = vector::add(vector::scale(self.pts[0],u1),&p1,t1);
                Self::line(&r0, &r1)
            },
            3 => {
                let c = &self.pts[2];
                let u0 = V::one() - t0;
                let u1 = V::one() - t1;
                let rp0 = vector::add(vector::add(vector::scale(p0.clone(),u0*u0),&c, two*u0*t0),&p1,t0*t0);
                let rp1 = vector::add(vector::add(vector::scale(p0.clone(),u1*u1),&c, two*u1*t1),&p1,t1*t1);
                let rc0 = vector::add(vector::add(vector::scale(p0.clone(),u0*u1),&c, u0*t1 + u1*t0),&p1,t1*t0);
                Self::quadratic(&rp0, &rc0, &rp1)
            },
            _ => {
                // simply: c0 = p0 + tangent(0)
                // and if we scale the curve to t1-t0 in size, tangents scale the same
                let rp0 = self.point_at(t0);
                let rt0 = self.tangent_at(t0);
                let rt1 = self.tangent_at(t1);
                let rp1 = self.point_at(t1);
                let t1_m_t0 = t1 - t0;
                let rc0 = vector::add(rp0.clone(),&rt0,t1_m_t0);
                let rc1 = vector::add(rp1.clone(),&rt1,-t1_m_t0);
                Self::cubic(&rp0, &rc0, &rc1, &rp1)
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

    //mp is_straight
    /// Returns true if the Bezier is straighter than a 'straightness' measure
    ///
    /// A linear bezier is always straight.
    ///
    /// A straightness measure for a quadratic bezier (one control
    /// point) can be thought of as the ratio between the area of the
    /// triangle formed by the two endpoints and the control point
    /// (three points must form a triangle on a plane) in relation to
    /// the distance between the endpoints (the curve will be entirely
    /// within the triangle.
    ///
    /// A straightness measure for a cubic bezier (two control points)
    /// can be though of similarly, except that the curve now must fit
    /// within a volume given by the two control points and the
    /// endpoints; hence the straightness is measured in some way by
    /// the volume in relation to the distance between the endpoints,
    /// but also should be no straighter than the area of any one
    /// control point in relation to the disnance between the
    /// endpoints (the Bezier may be a planar curve that is quite
    /// unstraight but with a volume of zero).
    ///
    /// Hence the straightness here is defined as the sum of (the
    /// ratio between (the distance of each control point from the
    /// straight line between the two endpoints) and (the distance
    /// between the two endpoints))
    ///
    /// `straightness` is thus independent of the length of the Bezier
    pub fn is_straight(&self, straightness:V) -> bool {
        fn straightness_of_control<V:Float, const D:usize>(p:&[V;D], lp2:V, c:&[V;D]) -> (V,V) {
            let lc2 = vector::length_sq(c);
            if lc2 < V::epsilon() {
                (V::zero(),lp2)
            } else if lp2 < V::epsilon() {
                (lc2,V::one())
            } else {
                let cdp = vector::dot(c, p);
                let c_s = V::sqrt(lp2*lc2 - cdp*cdp);
                (c_s,lp2)
            }
        }
        let one  = V::one();
        match self.num {
            2 => true,
            3 => {
                let p = vector::sub(self.pts[1], &self.pts[0], one);
                let lp2 = vector::length_sq(&p);
                let c = vector::sub(self.pts[2], &self.pts[0], one);
                let (c_s, sc) = straightness_of_control(&p, lp2, &c);
                c_s <= straightness * sc
            },
            _ => {
                let p = vector::sub(self.pts[1], &self.pts[0], one);
                let lp2 = vector::length_sq(&p);
                let c0 = vector::sub(self.pts[2], &self.pts[0], one);
                let (c0_s, sc0) = straightness_of_control(&p, lp2, &c0);
                let c1 = vector::sub(self.pts[3], &self.pts[0], one);
                let (c1_s, sc1) = straightness_of_control(&p, lp2, &c1);
                (c0_s + c1_s) <= straightness * V::max(sc0, sc1)
            },
        }
    }

    //mp length
    /// Calculates the length given a straightness
    ///
    /// `straightness` is independent of the length of the Bezier
    pub fn length(&self, straightness:V) -> V {
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
    fn t_of_distance_rec(&self, straightness:V, distance:V, t_start:V, t_scale:V, acc_length:V) -> (Option<V>, V) {
        let zero = V::zero();
        if distance <= acc_length {
            (Some(t_start), zero)
        } else if self.is_straight(straightness) {
            let d     = self.get_distance();
            if distance > acc_length+d {
                (None, acc_length+d)
            } else if d < V::epsilon() {
                (Some(t_start + t_scale), acc_length+d)
            } else {
                let rel_d = distance - acc_length;
                (Some(t_start + t_scale * rel_d / d), acc_length+d)
            }
        } else {
            let t_subscale = t_scale / V::from(2).unwrap();
            let (b0, b1) = self.bisect();
            match b0.t_of_distance_rec(straightness, distance, t_start, t_subscale, acc_length) {
                (None, length) => {
                    b1.t_of_distance_rec( straightness, distance, t_start + t_subscale, t_subscale, length )
                }
                r => r
            }
        }
    }
    pub fn t_of_distance(&self, straightness:V, distance:V) -> (V, bool) {
        let zero = V::zero();
        let one  = V::one();
        if distance < zero {
            (zero,false)
        } else {
            match self.t_of_distance_rec(straightness, distance, zero, one, zero) {
                (None, _)    => (one, false),
                (Some(t), _) => (t, true),
            }
        }
    }

    //fp arc
    /// Create a Cubic Bezier that approximates closely a circular arc
    ///
    /// The arc has a center C, a radius R, and is of an angle (should be <= PI/2).
    ///
    /// The arc sweeps through points a distance R from C, in a circle
    /// using a pair of the planar unit vectors in the vector space for the
    /// points.
    ///
    /// The arc will be between an angle A1 and A2, where A2-A1 == angle, and A1==rotate
    ///
    pub fn arc(angle:V, radius:V, center:&[V;D], unit:&[V;D], normal:&[V;D], rotate:V) -> Self {
        let one   = V::one();
        let two   = V::from(2).unwrap();
        let three = V::from(3).unwrap();
        let four  = V::from(4).unwrap();
        let half_angle = angle / two;
        let s = half_angle.sin();
        let lambda = radius * four / three * (one/s - one);

        let d0a = rotate;
        let (d0s,d0c) = d0a.sin_cos();
        let d1a = rotate+angle;
        let (d1s,d1c) = d1a.sin_cos();

        let p0 = vector::add( vector::add(center.clone(), unit, d0c*radius), normal, d0s*radius );
        let p1 = vector::add( vector::add(center.clone(), unit, d1c*radius), normal, d1s*radius );

        let c0 = vector::add( vector::add(p0.clone(), unit, -d0s*lambda), normal,  d0c*lambda );
        let c1 = vector::add( vector::add(p1.clone(), unit,  d1s*lambda), normal, -d1c*lambda );

        Self::cubic(&p0, &c0, &c1, &p1)
    }

    //fp of_round_corner
    /// Create a Cubic Bezier that is a circular arc focused on the corner point,
    /// with v0 and v1 are vectors IN to the point (P)
    ///
    /// As it is a circular arc we have a kite P, P+k.v0, C, P+k.v1, where
    ///
    /// |P+k.v0 - C| = |P+k.v1 - C| = r; |P-C| = d (i.e. side lengths are r, r, k, k)
    ///
    /// with two corners being right-angles. (and d is the length of
    /// the kite diagonal opposite these right-angles).
    ///
    /// The kite is formed from two d, r, k right-angled triangles; it
    /// has two other angles, alpha and 2*theta, (alpha = angle
    /// between v0 and v1). Hence alpha = 180 - 2*theta, theta = 90-(alpha/2)
    ///
    /// Hence d^2 = r^2 + k^2; r/d = cos(theta), k/d=sin(theta)
    ///
    /// We know cos(alpha) = v0.v1 (assuming unit vectors).
    ///
    /// cos(alpha) = cos(180-2*theta) = -cos(2*theta) = 1 - 2cos^2(theta)
    ///
    /// cos^2(theta) = (1 - cos(alpha)) / 2 = r^2/d^2
    ///
    /// sin^2(theta) = (1 + cos(alpha)) / 2
    ///
    /// => d^2 = 2*r^2  / (1 - cos(alpha))
    ///
    /// Hence also k^2, and hence d and k.
    ///
    /// Then we require an arc given the angle of the arc is 2*theta, which requires a lambda of
    /// 4/3 * r * (1/sin(theta)-1) = 4/3 * r * (d/k - 1)
    ///
    /// Note though that d^2/k^2 = 1/sin^2(theta) = 2/(1+cos(alpha))
    ///
    /// hence d/k = sqrt(2/(1+cos(alpha)))
    ///
    /// hence lambda = 4/3 * r * (sqrt(2/(1+cos(alpha))) - 1)
    pub fn of_round_corner(corner:&[V;D], v0:&[V;D], v1:&[V;D], radius:V) -> Self {
        let nearly_one = V::from(99_999).unwrap() / V::from(100_000).unwrap();
        let one   = V::one();
        let two   = V::from(2).unwrap();
        let three = V::from(3).unwrap();
        let four  = V::from(4).unwrap();
        let v0    = vector::normalize(v0.clone());
        let v1    = vector::normalize(v1.clone());
        let cos_alpha = vector::dot(&v0, &v1);
        if cos_alpha >= nearly_one {
            // v0 and v1 point in the same direction
            let p0 = vector::add(corner.clone(), &v0, -radius);
            let p1 = vector::add(corner.clone(), &v1, -radius);
            Self::quadratic(&p0, corner, &p1)
        } else if cos_alpha <= -nearly_one {
            // basically 180 degress apart
            let p0 = vector::add(corner.clone(), &v0, -radius);
            let p1 = vector::add(corner.clone(), &v1, -radius);
            Self::quadratic(&p0, corner, &p1)
        } else {
            let r2 = radius * radius;
            let d2 = two * r2 / (one - cos_alpha);
            let k2 = d2 - r2;
            let d = d2.sqrt();
            let k = k2.sqrt();
            // let v0_plus_v1_u = vector::normalize(vector::add(v0.clone(), &v1, one));
            // let center = vector::add(corner.clone(), &v0_plus_v1_u, d);
            // let lambda = four/three * radius * ((two / (one + cos_alpha)).sqrt() - one);

            let lambda = four/three * radius * (d/k - one);
            let p0 = vector::add(corner.clone(), &v0, -k);
            let p1 = vector::add(corner.clone(), &v1, -k);
            let c0 = vector::add(p0.clone(), &v0, lambda);
            let c1 = vector::add(p1.clone(), &v1, lambda);
            Self::cubic(&p0, &c0, &c1, &p1)
        }
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod test_bezier {
    use super::*;
    type Point = [f64;2];
    //fi vec_eq
    pub fn vec_eq(v0:&[f64;2], v1:&[f64;2]) {
        let d = vector::distance(v0, v1);
        assert!(d<1E-8, "mismatch in {:?} {:?}",v0, v1);
    }
    //fi pt_eq
    pub fn pt_eq(v:&Point, x:f64, y:f64) {
        assert!((v[0]-x).abs()<1E-8, "mismatch in x {:?} {:?} {:?}",v,x,y);
        assert!((v[1]-y).abs()<1E-8, "mismatch in y {:?} {:?} {:?}",v,x,y);
    }
    //fi approx_eq
    pub fn approx_eq(a:f64, b:f64, tolerance:f64, msg:&str) {
        assert!((a-b).abs()<tolerance, "{} {:?} {:?}",msg,a,b);
    }
    //fi bezier_eq
    pub fn bezier_eq(bez:&Bezier<f64,2>, v:Vec<[f64;2]>) {
        assert_eq!(bez.num, 4, "bezier_eq works only for cubics");
        vec_eq(&bez.pts[0], &v[0]);
        vec_eq(&bez.pts[2], &v[1]);
        vec_eq(&bez.pts[3], &v[2]);
        vec_eq(&bez.pts[1], &v[3]);
    }

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
            println!("t {} : {:?} : {:?}",t,p0,p1);
            pt_eq(&b0.point_at(t), p0[0], p0[1]);
            pt_eq(&b1.point_at(t), p1[0], p1[1]);
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
            println!("t {} : {:?} : {:?}",t,p,pb);
            approx_eq(p[0], pb[0], 1E-6, &format!("Bezier split x {} {} {} : {} : {}", t, t0, t1, bezier, b));
            approx_eq(p[1], pb[1], 1E-6, &format!("Bezier split y {} {} {} : {} : {}", t, t0, t1, bezier, b));
        }
    }
    //fi test_line
    #[test]
    fn test_line() {
        let p0 = vector::zero();
        let p1 = [10.,0.];
        let p2 = [10.,1.];
        let b01 = Bezier::line(&p0, &p1);
        let b02 = Bezier::line(&p0, &p2);

        pt_eq( &b01.point_at(0.), p0[0], p0[1] );
        pt_eq( &b01.point_at(0.5), (p0[0]+p1[0])/2., (p0[1]+p1[1])/2. );
        pt_eq( &b01.point_at(1.), p1[0], p1[1] );
        pt_eq( &b02.point_at(0.), p0[0], p0[1] );
        pt_eq( &b02.point_at(0.5), (p0[0]+p2[0])/2., (p0[1]+p2[1])/2. );
        pt_eq( &b02.point_at(1.), p2[0], p2[1] );

        pt_eq( &b01.bisect().0.point_at(0.), p0[0], p0[1] );
        pt_eq( &b01.bisect().0.point_at(1.), (p0[0]+p1[0])/2., (p0[1]+p1[1])/2. );
        pt_eq( &b01.bisect().1.point_at(0.), (p0[0]+p1[0])/2., (p0[1]+p1[1])/2. );
        pt_eq( &b01.bisect().1.point_at(1.), p1[0], p1[1] );


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

        pt_eq( &b01.tangent_at(0.),  p1[0]-p0[0], p1[1]-p0[1] );
        pt_eq( &b01.tangent_at(0.5), p1[0]-p0[0], p1[1]-p0[1] );
        pt_eq( &b01.tangent_at(1.0), p1[0]-p0[0], p1[1]-p0[1] );
        pt_eq( &b02.tangent_at(0.),  p2[0]-p0[0], p2[1]-p0[1] );
        pt_eq( &b02.tangent_at(0.5), p2[0]-p0[0], p2[1]-p0[1] );
        pt_eq( &b02.tangent_at(1.0), p2[0]-p0[0], p2[1]-p0[1] );

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
        let p0 = vector::zero();
        let p1 = [10.,0.];
        let p2 = [10.,1.];
        let b = Bezier::quadratic(&p0, &p1, &p2);

        pt_eq( &b.point_at(0.), p0[0], p0[1] );
        pt_eq( &b.point_at(0.5), (p0[0]+p2[0])/4.+p1[0]/2., (p0[1]+p2[1])/4.+p1[1]/2. );
        pt_eq( &b.point_at(1.), p2[0], p2[1] );

        does_bisect(&b);

        does_split(&b, 0., 1.);
        does_split(&b, 0.1, 0.3);
        does_split(&b, 0.3, 0.7);
        does_split(&b, 0.7, 1.0);

        pt_eq( &b.tangent_at(0.),  1.*(p1[0]-p0[0]), 1.*(p1[1]-p0[1]) );
        // pt_eq( &b.tangent_at(0.5), p1[0]-p0[0], p1[1]-p0[1] );
        pt_eq( &b.tangent_at(1.0), 1.*(p2[0]-p1[0]), 1.*(p2[1]-p1[1]) );

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
        let p0 = vector::zero();
        let p1 = [10.,0.];
        let p2 = [6.,1.];
        let p3 = [20.,5.];
        let b = Bezier::cubic(&p0, &p1, &p2, &p3);

        pt_eq( &b.point_at(0.), p0[0], p0[1] );
        pt_eq( &b.point_at(1.), p3[0], p3[1] );

        pt_eq( &b.tangent_at(0.),  p1[0]-p0[0], p1[1]-p0[1] );
        pt_eq( &b.tangent_at(1.0), p3[0]-p2[0], p3[1]-p2[1] );

        does_bisect(&b);

        does_split(&b, 0., 1.);
        does_split(&b, 0.1, 0.3);
        does_split(&b, 0.3, 0.7);
        does_split(&b, 0.7, 1.0);

        let x = Bezier::arc((90.0f64).to_radians(),1.,&vector::zero(),&[1.,0.],&[0.,1.],0.);
        println!("{}",x);
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
        let p0 = vector::zero();
        let p1 = [10.,0.];
        let p2 = [10.,1.];
        let p3 = [20.,0.];
        let p4 = [20.,1.];
        let sp0 = vector::scale(p0,10.);
        let sp1 = vector::comp_mult(p1,&[10.,10.]);
        let sp2 = vector::comp_mult(p2,&[10.,10.]);
        let sp3 = vector::comp_mult(p3,&[10.,10.]);
        let sp4 = vector::comp_mult(p4,&[10.,10.]);

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

        let x = Bezier::arc((90.0f64).to_radians(), 1., &[0.,0.], &[1.,0.], &[0.,1.], 0.);
        println!("arc((90.).to_radians(), 1., &[0.,0.], &[1.,0.], &[0.,1.], 0.) : {}",x);
        bezier_eq(&x, vec![[1.,0.], [1.,magic], [magic,1.], [0.,1.]]);

        let x = Bezier::arc((90.0f64).to_radians(), 1., &[0.,0.], &[1.,0.], &[0.,1.], (-90.0f64).to_radians());
        println!("arc((90.).to_radians(), 1., &[0.,0.], &[1.,0.], &[0.,1.], (-90.).to_radians()) : {}",x);
        bezier_eq(&x, vec![[0.,-1.], [magic,-1.], [1.,-magic], [1.,0.]]);

        let x = Bezier::of_round_corner(&[1.,1.], &[0.,3.], &[0.5,0.], 1.);
        println!("of_round_corner(&[1.,1.], &[0.,3.], &[0.5,0.], 1.) : {}",x);
        bezier_eq(&x, vec![[1.,0.], [1.,magic], [magic,1.], [0.,1.]]);

        let x = Bezier::of_round_corner(&[sqrt2,0.], &[1.,1.], &[1.,-1.], 1.);
        println!("of_round_corner(&[sqrt2,0.], &[1.,1.], &[1.,-1.], 1.) : {}",x);
        bezier_eq(&x, vec![[r_sqrt2, -r_sqrt2],
                           [r_sqrt2+magic2 , -r_sqrt2+magic2],
                           [r_sqrt2+magic2, r_sqrt2-magic2],
                           [r_sqrt2, r_sqrt2]]);

        pt_eq(x.borrow_pt(0), r_sqrt2, -r_sqrt2);
        pt_eq(x.borrow_pt(1), r_sqrt2, r_sqrt2);
        let x = Bezier::of_round_corner(&[1.,1.], &[0.,3.], &[0.5,0.], 0.5);
        println!("{:?}",x);
        // assert_eq!(true,false);
    }
    //fi All done
}
