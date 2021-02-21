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
                let v0     = corners[i+1].clone().add(&corners[i ], -1.);
                let v1     = corners[i+1].clone().add(&corners[i_2], -1.);
                corner_beziers.push(Bezier::round(&corners[i_1], &v0, &v1, rounding));
            }
            let mut edge_beziers = Vec::new();
            for i in 0..n {
                let i_1 = (i+1) % n;
                let p0 = corner_beziers[i  ].get_pt(1);
                let p1 = corner_beziers[i_1].get_pt(0);
                edge_beziers.push(Bezier::line(p0, p1));
            }
            for (e,c) in edge_beziers.iter().zip(corner_beziers.iter()) {
                v.push(*c);
                v.push(*e);
            }
        }
        Self { elements:v }
    }


    //fp get_pt
    /// Get the start or the end point
    pub fn get_pt(&self, index:usize) -> Point {
        let n = self.elements.len();
        if n == 0 {
            Point::origin()
        } else if index == 0 {
            self.elements[0].get_pt(0).clone()
        } else {
            self.elements[n-1].get_pt(1).clone()
        }
    }

    //fp add_bezier
    /// Add a Bezier at the end of the path
    pub fn add_bezier(&mut self, b:Bezier) {
        self.elements.push(b);
    }

    //fp iter_beziers
    /// Iterate through all the Beziers
    pub fn iter_beziers(&self) -> impl Iterator<Item = &Bezier> {
        self.elements.iter()
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod test_bezier {
    use super::*;
    pub fn pt_eq(pt:&Point, x:f64, y:f64) {
        assert!((pt.x-x).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
        assert!((pt.y-y).abs()<1E-8, "mismatch in x {:?} {} {}",pt,x,y);
    }
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
        let x = Bezier::round(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 1.);
        bezier_eq(&x, vec![(1.,0.), (1.,magic), (magic,1.), (0.,1.)]);
        let x = Bezier::round(&Point::new(sqrt2,0.), &Point::new(1.,1.), &Point::new(1.,-1.), 1.);
        bezier_eq(&x, vec![(r_sqrt2, -r_sqrt2), (r_sqrt2+magic2 , -r_sqrt2+magic2), (r_sqrt2+magic2, r_sqrt2-magic2), (r_sqrt2, r_sqrt2)]);
        pt_eq(x.get_pt(0), r_sqrt2, -r_sqrt2);
        pt_eq(x.get_pt(1), r_sqrt2, r_sqrt2);
        let x = Bezier::round(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 0.5);
        println!("{:?}",x);
        // assert_eq!(true,false);
    }
}
