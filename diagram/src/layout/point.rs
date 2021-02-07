#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x:f64,
    pub y:f64
}
impl Point {
    pub fn new(x:f64, y:f64) -> Self { Self {x,y} }
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
    pub fn add(mut self, other:Self) -> Self {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
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

    //zz All done
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
