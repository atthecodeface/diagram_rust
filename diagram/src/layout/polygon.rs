use super::Point;
use super::Rectangle;
use super::Bezier;

//tp Polygon
/// A polygon here defines an n-gon, from which one can generate a bezier path
///
/// It may have rounded corners
///
/// Nominally it is a regular n-gon, but it may have an eccentricity
///
#[derive(Debug, PartialEq)]
pub struct Polygon {
    center   : Point,
    vertices : usize,
    size          : f64,     // height
    stellate_size : f64,     // if not 0., then double the points and make a star
    eccentricity: f64, // width/height; i.e. width = size*eccentricity
    rotation : f64,  // rotation in degrees (after eccentricity)
    rounding : f64,  // 0 for no rounding of corners
}

//ip std::fmt::Display for Polygon
impl std::fmt::Display for Polygon {
    //mp fmt - format a `Polygon` for display
    /// Display the `Polygon` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.vertices == 0 {
            if self.eccentricity == 1. {
                write!(f, "Circle<{}, r={}>", self.center, self.size)
            } else {
                write!(f, "Ellipse<{}, a={}, b={}, rot={}>", self.center, self.size*self.eccentricity, self.size, self.rotation)
            }
        } else {
            write!(f, "Poly<{}, n={}, s={}, e={}, rot={}, stel={}, rnd={}>", self.center, self.vertices, self.size, self.eccentricity, self.rotation, self.stellate_size, self.rounding)
        }
    }
}

//ti Polygon
impl Polygon {
    //fp new
    pub fn new(vertices:usize, stellate_size:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices:vertices, rotation:0., rounding:0., size:0., eccentricity:1., stellate_size }
    }

    //fp new_rect
    pub fn new_rect(w:f64, h:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices:4, rotation:0., rounding:0., size:h/(2.0_f64.sqrt()), eccentricity:w/h, stellate_size:0. }
    }

    //fp new_polygon
    pub fn new_polygon(vertices:usize, size:f64, rotation:f64, rounding:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices, rotation, rounding, size, eccentricity:1., stellate_size:0. }
    }

    //fp new_star
    pub fn new_star(vertices:usize, size:f64, in_out:f64, rotation:f64, rounding:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices, rotation, rounding, size, eccentricity:1., stellate_size:size*in_out }
    }

    //fp new_circle
    pub fn new_circle(r:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices:0, rotation:0., rounding:0., size:r, eccentricity:1., stellate_size:0. }
    }

    //fp new_ellipse
    pub fn new_ellipse(rx:f64, ry:f64, rotation:f64) -> Self {
        Self{ center:Point::new(0.,0.), vertices:0, rotation, rounding:0., size:ry, eccentricity:rx/ry, stellate_size:0. }
    }

    //cp set_vertices
    pub fn set_vertices(&mut self, vertices:usize) {
        self.vertices = vertices;
    }

    //cp set_size
    pub fn set_size(&mut self, size:f64, eccentricity:f64) {
        self.size = size;
        self.eccentricity = eccentricity;
    }

    //cp set_rounding
    pub fn set_rounding(&mut self, rounding:f64) {
        self.rounding = rounding;
    }

    //cp set_stellate_size
    pub fn set_stellate_size(&mut self, stellate_size:f64) {
        self.stellate_size = stellate_size;
    }

    //cp translate
    pub fn translate(mut self, pt:&Point) -> Self {
        self.center = self.center.add(pt, 1.);
        self
    }

    //mp clone
    pub fn clone(&self) -> Self {
        Self { center:   self.center.clone(),
               vertices: self.vertices,
               size          : self.size,
               stellate_size : self.stellate_size,
               eccentricity: self.eccentricity,
               rotation : self.rotation,
               rounding : self.rounding,
        }
    }
    
    //mp as_paths
    pub fn as_paths(&self, v:Vec<Bezier>) -> Vec<Bezier> {
        match self.vertices {
            0 => self.elliptical_paths(v),
            1 => v,
            _ => self.polygon_paths(v),
        }
    }

    //mp get_bbox
    pub fn get_bbox(&self) -> Rectangle {
        match self.vertices {
            0 => Rectangle::new(-self.size*self.eccentricity, -self.size, self.size*self.eccentricity, self.size),
            1 => Rectangle::new(self.center.x, self.center.y, self.center.x, self.center.y),
            _ => Rectangle::bbox_of_points(&self.get_points()),
        }
    }

    //mp elliptical_paths
    fn elliptical_paths(&self, mut v:Vec<Bezier>) -> Vec<Bezier> {
        let origin = Point::new(0.,0.);
        v.push( Bezier::arc(90.,self.size,&origin,  0.).scale_xy(self.eccentricity,1.).rotate(self.rotation));
        v.push( Bezier::arc(90.,self.size,&origin, 90.).scale_xy(self.eccentricity,1.).rotate(self.rotation));
        v.push( Bezier::arc(90.,self.size,&origin,180.).scale_xy(self.eccentricity,1.).rotate(self.rotation));
        v.push( Bezier::arc(90.,self.size,&origin,270.).scale_xy(self.eccentricity,1.).rotate(self.rotation));
        v
    }

    //mp get_points
    fn get_points(&self) -> Vec<Point> {
        assert!(self.vertices>1);
        let mut corners = Vec::new();
        let delta_angle = 360./(self.vertices as f64);
        for i in 0..self.vertices {
            let p = Point::new(self.size,0.)
                .rotate(delta_angle*(0.5-(i as f64)))
                .scale_xy(self.eccentricity,1.)
                .rotate(self.rotation)
                .add(&self.center, 1.);
            corners.push(p);
            if self.stellate_size != 0. {
                let p = Point::new(self.stellate_size,0.)
                    .rotate(delta_angle*(0.0-(i as f64)))
                    .scale_xy(self.eccentricity,1.)
                    .rotate(self.rotation)
                    .add(&self.center, 1.);
                corners.push(p);
            }
        }
        // println!("{:?} {}",corners, self.stellate_size);
        corners
    }

    //mp polygon_paths
    fn polygon_paths(&self, mut v:Vec<Bezier>) -> Vec<Bezier> {
        let mut corners = self.get_points();
        let n = corners.len();
        if self.rounding == 0. {
            corners.push(corners[0].clone());
            for i in 0..n {
                v.push(Bezier::line(&corners[i], &corners[i+1]));
            }
        } else {
            corners.push(corners[0].clone());
            corners.push(corners[1].clone());
            let mut corner_beziers = Vec::new();
            for i in 0..n {
                let v0     = corners[i+1].clone().add(&corners[i+0], -1.);
                let v1     = corners[i+1].clone().add(&corners[i+2], -1.);
                corner_beziers.push(Bezier::round(&corners[i+1], &v0, &v1, self.rounding));
            }
            let mut edge_beziers = Vec::new();
            for i in 0..n {
                let p0 = corner_beziers[i].get_pt(1);
                let p1 = corner_beziers[(i+1)%n].get_pt(0);
                edge_beziers.push(Bezier::line(p0, p1));
            }
            for (e,c) in edge_beziers.iter().zip(corner_beziers.iter()) {
                v.push(*c);
                v.push(*e);
            }
            // for b in &v {
            //     println!("{}",b);
            // }
        }
        v
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
    fn test_circle() {
        let x = Polygon::new_circle(1.0);
        let v = Vec::new();
        let v = x.as_paths(v);
        bezier_eq(&v[0], vec![(1.,0.),  (1.,0.5522847498307935),   (0.5522847498307935,1.), (0.,1.)]);
        bezier_eq(&v[1], vec![(0., 1.), (-0.5522847498307935, 1.), (-1.,0.5522847498307935), (-1.,0.)]);
        bezier_eq(&v[2], vec![(-1.,0.), (-1.,-0.5522847498307935),   (-0.5522847498307935,-1.), (0.,-1.)]);
        bezier_eq(&v[3], vec![(0.,-1.), (0.5522847498307935,-1.),  (1.,-0.5522847498307935), (1.,0.)]);
    }
}
