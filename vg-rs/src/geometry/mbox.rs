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

@file    mbox.rs
@brief   Part of SVG library
 */
//a Imports
use crate::Margin;

//a MBox
//tp MBox
#[derive(Debug, Clone, Copy, Default, PartialEq)]
/// [MBox] describes margins (or padding) for a box; it is a pair of
/// Margin
pub struct MBox {
    /// X margin
    pub x: Margin,
    /// Y margin
    pub y: Margin,
}

//ti Display for MBox
impl std::fmt::Display for MBox {
    //mp fmt - format for a human
    /// Display the MBox
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[x {}:y {}]", self.x, self.y)
    }

    //zz All done
}

//ti MBox
impl MBox {
    //mp none
    /// Create a none mbox - where both ranges are none
    pub fn none() -> Self {
        Self {
            x: Margin::none(),
            y: Margin::none(),
        }
    }

    //mp is_none
    /// Return `true` if the margin is completely None
    pub fn is_none(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    //cp new
    /// Make a rectangle using the coordinates supplied, ensuring that
    /// the rectangle is correctly defined
    pub fn new(x: Margin, y: Margin) -> Self {
        Self { x, y }
    }

    //mp totals
    /// Get the total margin in X and Y
    pub fn totals(&self) -> (f64, f64) {
        (self.x.total(), self.y.total())
    }

    //zz All done
}

//ip From<f64> for MBox
impl From<f64> for MBox {
    #[inline]
    fn from(m: f64) -> Self {
        let m: Margin = m.into();
        (m, m).into()
    }
}

//ip From<(f64, f64, f64, f64)> for MBox
impl From<(f64, f64, f64, f64)> for MBox {
    #[inline]
    fn from((lx, by, rx, ty): (f64, f64, f64, f64)) -> Self {
        let x = (lx, rx).into();
        let y = (by, ty).into();
        Self::new(x, y)
    }
}

//ip From<(Margin, Margin)> for MBox
impl From<(Margin, Margin)> for MBox {
    #[inline]
    fn from((x, y): (Margin, Margin)) -> Self {
        Self::new(x, y)
    }
}

//ip From<Margin> for MBox
impl From<Margin> for MBox {
    #[inline]
    fn from(m: Margin) -> Self {
        Self::new(m, m)
    }
}

//a Add/Sub ops
//ip std::ops::AddAssign<f64> for MBox
impl std::ops::AddAssign<f64> for MBox {
    #[inline]
    fn add_assign(&mut self, delta: f64) {
        self.x += delta;
        self.y += delta;
    }
}

//ip std::ops::SubAssign<f64> for MBox
impl std::ops::SubAssign<f64> for MBox {
    #[inline]
    fn sub_assign(&mut self, delta: f64) {
        self.x -= delta;
        self.y -= delta;
    }
}

//ip std::ops::AddAssign<MBox> for MBox
impl std::ops::AddAssign<MBox> for MBox {
    #[inline]
    fn add_assign(&mut self, delta: MBox) {
        self.x += delta.x;
        self.y += delta.y;
    }
}

//ip std::ops::SubAssign<MBox> for MBox
impl std::ops::SubAssign<MBox> for MBox {
    #[inline]
    fn sub_assign(&mut self, delta: MBox) {
        self.x -= delta.x;
        self.y -= delta.y;
    }
}

//ip std::ops::AddAssign<(f64, f64)> for MBox
impl std::ops::AddAssign<(f64, f64)> for MBox {
    #[inline]
    fn add_assign(&mut self, (dx, dy): (f64, f64)) {
        self.x += dx;
        self.y += dy;
    }
}

//ip std::ops::SubAssign<(f64, f64)> for MBox
impl std::ops::SubAssign<(f64, f64)> for MBox {
    #[inline]
    fn sub_assign(&mut self, (dx, dy): (f64, f64)) {
        self.x -= dx;
        self.y -= dy;
    }
}

//ip std::ops::AddAssign<Margin> for MBox
impl std::ops::AddAssign<Margin> for MBox {
    #[inline]
    fn add_assign(&mut self, delta: Margin) {
        self.x += delta;
        self.y += delta;
    }
}

//ip std::ops::SubAssign<Margin> for MBox
impl std::ops::SubAssign<Margin> for MBox {
    #[inline]
    fn sub_assign(&mut self, delta: Margin) {
        self.x -= delta;
        self.y -= delta;
    }
}

//ip std::ops::Add<f64> for MBox
impl std::ops::Add<f64> for MBox {
    type Output = Self;
    #[inline]
    fn add(mut self, delta: f64) -> Self {
        self += delta;
        self
    }
}

//ip std::ops::Sub<f64> for MBox
impl std::ops::Sub<f64> for MBox {
    type Output = Self;
    #[inline]
    fn sub(mut self, delta: f64) -> Self {
        self -= delta;
        self
    }
}

//ip std::ops::Add<MBox> for MBox
impl std::ops::Add<MBox> for MBox {
    type Output = Self;
    #[inline]
    fn add(mut self, delta: MBox) -> Self {
        self += delta;
        self
    }
}

//ip std::ops::Sub<MBox> for MBox
impl std::ops::Sub<MBox> for MBox {
    type Output = Self;
    #[inline]
    fn sub(mut self, delta: MBox) -> Self {
        self -= delta;
        self
    }
}

//ip std::ops::Add<Margin> for MBox
impl std::ops::Add<Margin> for MBox {
    type Output = Self;
    #[inline]
    fn add(mut self, delta: Margin) -> Self {
        self += delta;
        self
    }
}

//ip std::ops::Sub<Margin> for MBox
impl std::ops::Sub<Margin> for MBox {
    type Output = Self;
    #[inline]
    fn sub(mut self, delta: Margin) -> Self {
        self -= delta;
        self
    }
}

//ip std::ops::Add<(f64, f64)> for MBox
impl std::ops::Add<(f64, f64)> for MBox {
    type Output = Self;
    #[inline]
    fn add(mut self, delta: (f64, f64)) -> Self {
        self += delta;
        self
    }
}

//ip std::ops::Sub<(f64, f64)> for MBox
impl std::ops::Sub<(f64, f64)> for MBox {
    type Output = Self;
    #[inline]
    fn sub(mut self, delta: (f64, f64)) -> Self {
        self -= delta;
        self
    }
}

//a Mul/Div ops
//ip std::ops::MulAssign<f64> for MBox
impl std::ops::MulAssign<f64> for MBox {
    #[inline]
    fn mul_assign(&mut self, scale: f64) {
        self.x *= scale;
        self.y *= scale;
    }
}

//ip std::ops::DivAssign<f64> for MBox
impl std::ops::DivAssign<f64> for MBox {
    #[inline]
    fn div_assign(&mut self, scale: f64) {
        self.x /= scale;
        self.y /= scale;
    }
}

//ip std::ops::MulAssign<(f64, f64)> for MBox
impl std::ops::MulAssign<(f64, f64)> for MBox {
    #[inline]
    fn mul_assign(&mut self, (sx, sy): (f64, f64)) {
        self.x *= sx;
        self.y *= sy;
    }
}

//ip std::ops::DivAssign<(f64, f64)> for MBox
impl std::ops::DivAssign<(f64, f64)> for MBox {
    #[inline]
    fn div_assign(&mut self, (sx, sy): (f64, f64)) {
        self.x /= sx;
        self.y /= sy;
    }
}

//ip std::ops::Mul<f64> for MBox
impl std::ops::Mul<f64> for MBox {
    type Output = Self;
    #[inline]
    fn mul(mut self, scale: f64) -> Self {
        self *= scale;
        self
    }
}

//ip std::ops::Div<f64> for MBox
impl std::ops::Div<f64> for MBox {
    type Output = Self;
    #[inline]
    fn div(mut self, scale: f64) -> Self {
        self /= scale;
        self
    }
}

//ip std::ops::Mul<(f64, f64)> for MBox
impl std::ops::Mul<(f64, f64)> for MBox {
    type Output = Self;
    #[inline]
    fn mul(mut self, scale: (f64, f64)) -> Self {
        self *= scale;
        self
    }
}

//ip std::ops::Div<(f64, f64)> for MBox
impl std::ops::Div<(f64, f64)> for MBox {
    type Output = Self;
    #[inline]
    fn div(mut self, scale: (f64, f64)) -> Self {
        self /= scale;
        self
    }
}

//a Test
#[cfg(test)]
mod tests_margin {
    use super::*;
    #[track_caller]
    pub fn margin_eq(pt: &Margin, x: f64, y: f64) {
        assert!(
            (pt[0] - x).abs() < 1E-8,
            "mismatch in x {:?} {} {}",
            pt,
            x,
            y
        );
        assert!(
            (pt[1] - y).abs() < 1E-8,
            "mismatch in y {:?} {} {}",
            pt,
            x,
            y
        );
    }
    #[test]
    fn test_zero() {
        let x = MBox::none();
        assert!(x.is_none());
        assert_eq!(x.totals(), (0., 0.));
        let m: MBox = (0.).into();
        assert!(m.is_none());
    }
    #[test]
    fn test_0() {
        let m: MBox = (1.).into();
        assert!(!m.is_none());
        assert_eq!(m.totals(), (2., 2.));
        let m: MBox = (1., 2., 3., 4.).into();
        assert!(!m.is_none());
        assert_eq!(m.totals(), (4., 6.));
    }
    #[test]
    fn test_1() {
        let x: Margin = (1.).into();
        let y: Margin = (2.).into();
        let mut m: MBox = x.into();
        assert!(!m.is_none());
        assert_eq!(m.totals(), (2., 2.));
        m = (x, y).into();
        assert!(!m.is_none());
        assert_eq!(m.totals(), (2., 4.));
    }
    #[test]
    fn test_add_sub_ops_0() {
        let m: MBox = (2., 1., 5., 7.).into();
        let dx: MBox = (1., 0., 2., 0.).into();
        let dy: MBox = (0., 1., 0., 3.).into();
        let dm: Margin = (3., 2.).into();

        margin_eq(&m.x, 2., 5.);
        margin_eq(&m.y, 1., 7.);

        margin_eq(&(m + 3.).x, 5., 8.);
        margin_eq(&(m + 3.).y, 4., 10.);

        margin_eq(&(m + dx).x, 3., 7.);
        margin_eq(&(m + dx).y, 1., 7.);
        margin_eq(&(m + dy).x, 2., 5.);
        margin_eq(&(m + dy).y, 2., 10.);

        margin_eq(&(m + (2., 4.)).x, 4., 7.);
        margin_eq(&(m + (2., 4.)).y, 5., 11.);

        margin_eq(&(m + dm).x, 5., 7.);
        margin_eq(&(m + dm).y, 4., 9.);

        margin_eq(&(m - 3.).x, -1., 2.);
        margin_eq(&(m - 3.).y, -2., 4.);

        margin_eq(&(m - dx).x, 1., 3.);
        margin_eq(&(m - dx).y, 1., 7.);
        margin_eq(&(m - dy).x, 2., 5.);
        margin_eq(&(m - dy).y, 0., 4.);

        margin_eq(&(m - (2., 4.)).x, 0., 3.);
        margin_eq(&(m - (2., 4.)).y, -3., 3.);

        margin_eq(&(m - dm).x, -1., 3.);
        margin_eq(&(m - dm).y, -2., 5.);
    }
    #[test]
    fn test_mul_div_ops_0() {
        let m: MBox = (2., 1., 5., 7.).into();

        margin_eq(&m.x, 2., 5.);
        margin_eq(&m.y, 1., 7.);

        margin_eq(&(m * 2.).x, 4., 10.);
        margin_eq(&(m * 2.).y, 2., 14.);

        margin_eq(&(m * (4., 3.)).x, 8., 20.);
        margin_eq(&(m * (4., 3.)).y, 3., 21.);

        margin_eq(&(m / 2.).x, 1., 2.5);
        margin_eq(&(m / 2.).y, 0.5, 3.5);

        margin_eq(&((m * 3.) / (2., 3.)).x, 3., 7.5);
        margin_eq(&((m * 3.) / (2., 3.)).y, 1., 7.);
    }
}
