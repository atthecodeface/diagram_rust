use super::point::Point;
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Bezier {
    Linear(Point, Point),
    Quadratic(Point, Point, Point),
    Cubic(Point, Point, Point, Point),
}

//ti Display for Bezier
impl std::fmt::Display for Bezier {

    //mp fmt - format a `CharError` for display
    /// Display the `Point' as (x,y)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Linear(p0,p1)      => write!(f, "[{}->{}]", p0, p1),
            Self::Quadratic(p0,c,p1) => write!(f, "[{}-:{}:->{}]", p0, c, p1),
            Self::Cubic(p0,c0,c1,p1) => write!(f, "[{}-:{}:{}:>{}]", p0, c0, c1, p1),
        }
    }

    //zz All done
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
