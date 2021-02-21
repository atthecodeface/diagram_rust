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

@file    path.rs
@brief   Part of geometry library
 */

//a Imports
use super::Point;
use super::Bezier;

//a Types
//tp BezierPath
/// A path is a set of Beziers that start and end at the same points
#[derive(Debug)]
pub struct BezierPath {
    elements : Vec<Bezier>,
}

//ip BezierPath
impl BezierPath {
    //fp new
    /// Create a new BezierPath
    pub fn new() -> Self {
        Self { elements : Vec::new() }
    }

    //fp of_ellipse
    /// Create a set of paths that make an ellipse
    pub fn of_ellipse(origin:&Point, radius:f64, eccentricity:f64, degrees:f64) -> Self {
        let mut v = Vec::new();
        v.push( Bezier::arc(90.,radius,origin,  0.).scale_xy(eccentricity,1.).rotate(degrees));
        v.push( Bezier::arc(90.,radius,origin, 90.).scale_xy(eccentricity,1.).rotate(degrees));
        v.push( Bezier::arc(90.,radius,origin,180.).scale_xy(eccentricity,1.).rotate(degrees));
        v.push( Bezier::arc(90.,radius,origin,270.).scale_xy(eccentricity,1.).rotate(degrees));
        Self { elements:v }
    }

    //fp of_points
    /// Generate a set of Beziers that join the corners
    pub fn of_points(corners:&Vec<Point>, rounding:f64) -> Self {
        let mut v = Vec::new();
        let n = corners.len();
        if rounding == 0. {
            for i in 0..n {
                let i_1 = (i+1) % n;
                v.push(Bezier::line(&corners[i], &corners[i_1]));
            }
        } else {
            let mut corner_beziers = Vec::new();
            for i in 0..n {
                let i_1 = (i+1) % n;
                let i_2 = (i+2) % n;
                let v0     = corners[i_1].clone().add(&corners[i ], -1.);
                let v1     = corners[i_1].clone().add(&corners[i_2], -1.);
                corner_beziers.push(Bezier::of_round_corner(&corners[i_1], &v0, &v1, rounding));
            }
            let mut edge_beziers = Vec::new();
            for i in 0..n {
                let i_1 = (i+1) % n;
                let p0 = corner_beziers[i  ].borrow_pt(1);
                let p1 = corner_beziers[i_1].borrow_pt(0);
                edge_beziers.push(Bezier::line(p0, p1));
            }
            for (e,c) in edge_beziers.iter().zip(corner_beziers.iter()) {
                v.push(*c);
                v.push(*e);
            }
        }
        Self { elements:v }
    }

    //mp round
    /// Run through the path; for every adjacent pair of Beziers that
    /// are *line*s add an intermediate Bezier that is a rounded
    /// corner of the correct radius.
    ///
    /// If the path is closed, thenn treat the first Bezier is
    /// adjacent to the last Bezier
    pub fn round(&mut self, rounding:f64, closed:bool) {
        let mut n = self.elements.len();
        if n < 2 || rounding == 0. { return; }
        let mut i = n-1;
        if !closed { i -= 1; }
        loop {
            let i_1 = (i+1) % n;
            if self.elements[i].is_line() && self.elements[i_1].is_line() {
                let corner = self.elements[i].borrow_pt(1); // same as i_1.borrow_pt(0);
                let v0 = self.elements[i  ].tangent_at(1.);
                let v1 = self.elements[i_1].tangent_at(0.).scale_xy(-1.,-1.);
                println!("{} {} {}",corner, v0, v1);
                let bezier = Bezier::of_round_corner(&corner, &v0, &v1, rounding);
                let np00 = self.elements[i].borrow_pt(0).clone();
                let np01 = bezier.borrow_pt(0).clone();
                let np10 = bezier.borrow_pt(1).clone();
                let np11 = self.elements[i_1].borrow_pt(1).clone();
                self.elements[i]   = Bezier::line(&np00, &np01);
                self.elements[i_1] = Bezier::line(&np10, &np11);
                self.elements.insert(i+1, bezier);
                n += 1; // Not really required but it keeps n == self.elements.len()
            }
            if i == 0 {break;}
            i -= 1;
        }
    }

    //mp get_pt
    /// Get the start or the end point
    pub fn get_pt(&self, index:usize) -> Point {
        let n = self.elements.len();
        if n == 0 {
            Point::origin()
        } else if index == 0 {
            self.elements[0].borrow_pt(0).clone()
        } else {
            self.elements[n-1].borrow_pt(1).clone()
        }
    }

    //mp add_bezier
    /// Add a Bezier at the end of the path
    pub fn add_bezier(&mut self, b:Bezier) {
        self.elements.push(b);
    }

    //mp iter_beziers
    /// Iterate through all the Beziers
    pub fn iter_beziers(&self) -> impl Iterator<Item = &Bezier> {
        self.elements.iter()
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod test_path {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
    pub fn bezier_eq(bez:&Bezier, v:Vec<(f64,f64)>) {
        match bez {
            Bezier::Linear(p0,p1) => {
                pt_eq(p0, v[0].0, v[0].1);
                pt_eq(p1, v[1].0, v[1].1);
            }
            Bezier::Cubic(p0,c0,c1,p1) => {
                pt_eq(p0, v[0].0, v[0].1);
                pt_eq(c0, v[1].0, v[1].1);
                pt_eq(c1, v[2].0, v[2].1);
                pt_eq(p1, v[3].0, v[3].1);
            }
            _ => {},
        }
    }
    #[test]
    fn test_round_open() {
        let p0 = Point::origin();
        let p1 = Point::new(1.,0.);
        let p2 = Point::new(1.,1.);
        let p3 = Point::new(0.,1.);
        let mut bp = BezierPath::new();
        bp.add_bezier( Bezier::line( &p0, &p1 ) );
        bp.add_bezier( Bezier::line( &p1, &p2 ) );
        bp.add_bezier( Bezier::line( &p2, &p3 ) );
        bp.add_bezier( Bezier::line( &p3, &p0 ) );

        bp.round(0.1,false);
        for b in bp.iter_beziers() {
            println!("Bezier {}", b);
        }
        bezier_eq(&bp.elements[0], vec![(0.,0.), (0.9,0.0)]);
        bezier_eq(&bp.elements[2], vec![(1.,0.1), (1.,0.9)]);
        bezier_eq(&bp.elements[4], vec![(0.9,1.0), (0.1,1.)]);
        bezier_eq(&bp.elements[6], vec![(0.,0.9), (0.,0.)]);
        assert_eq!(bp.elements.len(), 7, "Path should be 7 elements");
    }
    #[test]
    fn test_round_closed() {
        let p0 = Point::origin();
        let p1 = Point::new(1.,0.);
        let p2 = Point::new(1.,1.);
        let p3 = Point::new(0.,1.);
        let mut bp = BezierPath::new();
        bp.add_bezier( Bezier::line( &p0, &p1 ) );
        bp.add_bezier( Bezier::line( &p1, &p2 ) );
        bp.add_bezier( Bezier::line( &p2, &p3 ) );
        bp.add_bezier( Bezier::line( &p3, &p0 ) );

        bp.round(0.1,true);
        for b in bp.iter_beziers() {
            println!("Bezier {}", b);
        }
        bezier_eq(&bp.elements[0], vec![(0.,0.), (0.9,0.0)]);
        bezier_eq(&bp.elements[2], vec![(1.,0.1), (1.,0.9)]);
        bezier_eq(&bp.elements[4], vec![(0.9,1.0), (0.1,1.)]);
        bezier_eq(&bp.elements[6], vec![(0.,0.9), (0.,0.)]);
        assert_eq!(bp.elements.len(), 8, "Path should be 8 elements");
    }
}
