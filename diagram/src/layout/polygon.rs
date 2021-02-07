//tp Polygon
/// A polygon here defines an n-gon, from which one can generate a bezier path
///
/// It may have rounded corners
///
/// Nominally it is a regular n-gon, but it may have an eccentricity
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point { x:f64, y:f64 }
impl Point {
    fn new(x:f64, y:f64) -> Self { Self {x,y} }
    fn rotate(mut self, degrees:f64) -> Self {
        let c = degrees.to_radians().cos();
        let s = degrees.to_radians().sin();
        let x1 = c*self.x + s*self.y;
        let y1 = c*self.y - s*self.x;
        self.x = x1;
        self.y = y1;
        self
    }
    fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        self.x = self.x*sx;
        self.y = self.y*sy;
        self
    }
    fn add(mut self, other:Self) -> Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self
    }
    fn len2(self) -> f64 {
        self.x*self.x + self.y*self.y
    }
    fn len(self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    fn dot(self, other:&Point) -> f64 {
        self.x*other.x + self.y*other.y
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Bezier {
    Linear(Point, Point),
    Quadratic(Point, Point, Point),
    Cubic(Point, Point, Point, Point),
}
impl Bezier {
    pub fn line(p0:&Point, p1:&Point) -> Self {
        Self::Linear(p0.clone(), p1.clone())
    }
    pub fn scale_xy(mut self, sx:f64, sy:f64) -> Self {
        match &self {
            Self::Linear(ref p0, ref p1) => {
                p0.scale_xy(sx,sy);
                p1.scale_xy(sx,sy);
            },
            Self::Quadratic(ref p0, ref c, ref p1) => {
                p0.scale_xy(sx,sy);
                c.scale_xy(sx,sy);
                p1.scale_xy(sx,sy);
            },
            Self::Cubic(ref p0, ref c0, ref c1, ref p1) => {
                p0.scale_xy(sx,sy);
                c0.scale_xy(sx,sy);
                c1.scale_xy(sx,sy);
                p1.scale_xy(sx,sy);
            },
            _ => { panic!("Argh");},
        }
        self
    }
    pub fn rotate(mut self, degrees:f64) -> Self {
        match &self {
            Self::Linear(ref p0, ref p1) => {
                p0.rotate(degrees);
                p1.rotate(degrees);
            },
            Self::Quadratic(ref p0, ref c, ref p1) => {
                p0.rotate(degrees);
                c.rotate(degrees);
                p1.rotate(degrees);
            },
            Self::Cubic(ref p0, ref c0, ref c1, ref p1) => {
                p0.rotate(degrees);
                c0.rotate(degrees);
                c1.rotate(degrees);
                p1.rotate(degrees);
            },
            _ => { panic!("Argh");},
        }
        self
    }
    /// v0 and v1 are vectors in to the point
    pub fn round(corner:&Point, v0:&Point, v1:&Point, radius:f64) -> Self {
        let rl0 = 1.0/v0.len();
        let rl1 = 1.0/v1.len();
        let v0u = Point::new(v0.x*rl0, v0.y*rl0);
        let v1u = Point::new(v1.x*rl1, v1.y*rl1);
        let n0u = Point::new(-v0u.y,v0u.x); // unit normal
        let n1u = Point::new(-v1u.y,v1u.x); // unit normal
        let k = radius / (n1u.dot(&v0u));
        println!("k:{}",k);
        let center = Point::new( corner.x-k*(v0u.x+v1u.x), corner.y-k*(v0u.y+v1u.y) );
        let normal_diff = Point::new(n0u.x-n1u.x, n0u.y-n1u.y);
        let vector_sum  = Point::new(v0u.x+v1u.x, v0u.y+v1u.y);
        let l2 = (vector_sum.x*vector_sum.x + vector_sum.y*vector_sum.y);
        let l = l2.sqrt();
        let lambda = 4.0*radius/(3.*l2) * (2.*l + (normal_diff.x*vector_sum.x + normal_diff.y*vector_sum.y));
        println!("{:?} {:?} {:?} {:?} {:?}",center,v0,normal_diff,vector_sum, lambda);
        let p0 = Point::new(center.x-radius*n0u.x, center.y-radius*n0u.y);
        let p1 = Point::new(center.x+radius*n1u.x, center.y+radius*n1u.y);
        let c0 = Point::new(p0.x + lambda * v0u.x, p0.y + lambda * v0u.y);
        let c1 = Point::new(p1.x + lambda * v1u.x, p1.y + lambda * v1u.y);
        Self::Cubic(p0,c0,c1,p1)
    }
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
        let x = Bezier::arc(90.,1.,&Point::new(0.,0.),0.);
        bezier_eq(&x, vec![(1.,0.), (1.,0.5522847498307935), (0.5522847498307935,1.), (0.,1.)]);
        let x = Bezier::arc(90.,1.,&Point::new(0.,0.),-90.);
        bezier_eq(&x, vec![(0.,-1.), (0.5522847498307935,-1.), (1.,-0.5522847498307935), (1.,0.)]);
        let x = Bezier::round(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 1.);
        println!("{:?}",x);
        let x = Bezier::round(&Point::new(1.414,0.), &Point::new(1.,1.), &Point::new(1.,-1.), 1.);
        println!("{:?}",x);
        let x = Bezier::round(&Point::new(1.,1.), &Point::new(0.,3.), &Point::new(0.5,0.), 0.5);
        println!("{:?}",x);
    }
}
pub struct Polygon {
    vertices : usize,
    size          : f64,     // height
    stellate_size : f64,     // if not 0., then double the points and make a star
    eccentricity: f64, // width/height; i.e. width = size*eccentricity
    rotation : f64,  // rotation in degrees (after eccentricity)
    rounding : f64,  // 0 for no rounding of corners
}

impl Polygon {
    pub fn new_rect(w:f64, h:f64) -> Self {
        Self{ vertices:4, rotation:0., rounding:0., size:h, eccentricity:w/h, stellate_size:0. }
    }
    pub fn new_polygon(vertices:usize, size:f64, rotation:f64, rounding:f64) -> Self {
        Self{ vertices, rotation, rounding, size, eccentricity:1., stellate_size:0. }
    }
    pub fn new_star(vertices:usize, size:f64, in_out:f64, rotation:f64, rounding:f64) -> Self {
        Self{ vertices, rotation, rounding, size, eccentricity:1., stellate_size:size*in_out }
    }
    pub fn new_circle(r:f64) -> Self {
        Self{ vertices:0, rotation:0., rounding:0., size:r, eccentricity:1., stellate_size:0. }
    }
    pub fn new_ellipse(rx:f64, ry:f64, rotation:f64) -> Self {
        Self{ vertices:0, rotation, rounding:0., size:ry, eccentricity:rx/ry, stellate_size:0. }
    }
    pub fn as_paths(&self, mut v:Vec<Bezier>) -> Vec<Bezier> {
        match self.vertices {
            0 => self.elliptical_paths(v),
            1 => v,
            _ => self.polygon_paths(v),
        }
    }
    fn elliptical_paths(&self, mut v:Vec<Bezier>) -> Vec<Bezier> {
        let origin = Point::new(0.,0.);
        v.push( Bezier::arc(90.,self.size,&origin,  0.).scale_xy(self.eccentricity,1.));
        v.push( Bezier::arc(90.,self.size,&origin, 90.).scale_xy(self.eccentricity,1.));
        v.push( Bezier::arc(90.,self.size,&origin,180.).scale_xy(self.eccentricity,1.));
        v.push( Bezier::arc(90.,self.size,&origin,270.).scale_xy(self.eccentricity,1.));
        v
    }
    fn polygon_paths(&self, mut v:Vec<Bezier>) -> Vec<Bezier> {
        assert!(self.vertices>1);
        let mut corners = Vec::new();
        let delta_angle = 360./(self.vertices as f64);
        for i in 0..self.vertices {
            let p = Point::new(self.size,0.)
                .rotate(delta_angle*(0.5+(i as f64)))
                .scale_xy(self.eccentricity,1.)
                .rotate(self.rotation);
            corners.push(p);
            if self.stellate_size!=0. {
                let p = Point::new(self.stellate_size,0.)
                    .rotate(delta_angle*(0.75+(i as f64)))
                    .scale_xy(self.eccentricity,1.)
                    .rotate(self.rotation);
                corners.push(p);
            }
        }
        corners.push(corners[0].clone());
        if self.rounding==0. {
            for i in 0..self.vertices {
                v.push(Bezier::line(&corners[i], &corners[i+1]));
            }
        } else {
            let mut midpoints = Vec::new();
            for i in 0..self.vertices {
                let p = corners[i].clone().add(corners[i+1]).scale_xy(0.5,0.5);
                midpoints.push(p);
            }
            // Want paths Linear(midpoint[0] -> x), CircleRadius(x,y,self.rounding,angle=delta_angle, )
        }
        v
    }
}

//a Test
#[cfg(test)]
mod tests_polygon {
    use super::*;
    use super::test_bezier::{bezier_eq};
    #[test]
    fn test_circle() {
        let x = Polygon::new_circle(1.0);
        let mut v = Vec::new();
        let mut v = x.as_paths(v);
        bezier_eq(&v[0], vec![(1.,0.),  (1.,0.5522847498307935),   (0.5522847498307935,1.), (0.,1.)]);
        bezier_eq(&v[1], vec![(0., 1.), (-0.5522847498307935, 1.), (-1.,0.5522847498307935), (-1.,0.)]);
        bezier_eq(&v[2], vec![(-1.,0.), (-1.,-0.5522847498307935),   (-0.5522847498307935,-1.), (0.,-1.)]);
        bezier_eq(&v[3], vec![(0.,-1.), (0.5522847498307935,-1.),  (1.,-0.5522847498307935), (1.,0.)]);
    }
}
