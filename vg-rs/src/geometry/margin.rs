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

@file    margin.rs
@brief   Part of SVG library
 */

//a Margin
//tp Margin
/// This is a simple one-dimension 'margin' class
///
/// min <= max for a valid range; min > max indicates an empty range
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Margin {
    /// Left or bottom margin value
    lb: f64,
    /// Right or top margin value
    rt: f64,
}

//ip From<f64> for Margin
impl From<f64> for Margin {
    #[inline]
    fn from(m: f64) -> Self {
        Self::new(m, m)
    }
}

//ip From<(f64, f64)> for Margin
impl From<(f64, f64)> for Margin {
    #[inline]
    fn from((lb, rt): (f64, f64)) -> Self {
        Self::new(lb, rt)
    }
}

//ip Margin
impl Margin {
    //cp new
    /// Construct a new margin given two values
    ///
    /// It is preferable to use (a,b).into()
    #[inline]
    pub fn new(lb: f64, rt: f64) -> Self {
        Self { lb, rt }
    }

    //cp none
    /// Construct a new 'null' margin (i.e. zeroes)
    #[inline]
    pub fn none() -> Self {
        Self { lb: 0., rt: 0. }
    }

    //ap is_none
    /// Return true if the margin is none (i.e. zeroes)
    #[inline]
    pub fn is_none(&self) -> bool {
        self.lb == 0. && self.rt == 0.
    }

    //ap lx
    /// Get the left margin value (if it is construed as an X margin)
    #[inline]
    pub fn lx(&self) -> f64 {
        self.lb
    }

    //ap by
    /// Get the bottom margin value (if it is construed as a Y margin)
    #[inline]
    pub fn by(&self) -> f64 {
        self.lb
    }

    //ap rx
    /// Get the right margin value (if it is construed as an X margin)
    #[inline]
    pub fn rx(&self) -> f64 {
        self.rt
    }

    //ap ty
    /// Get the top margin value (if it is construed as a Y margin)
    #[inline]
    pub fn ty(&self) -> f64 {
        self.rt
    }

    //ap total
    /// Get the total margin value
    #[inline]
    pub fn total(&self) -> f64 {
        self.lb + self.rt
    }

    //zz All done
}

//ti Display for Margin
impl std::fmt::Display for Margin {
    /// Display the [Range] as (min to max)
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.lb == self.rt {
            write!(f, "<{}>", self.lb)
        } else {
            write!(f, "<{}:{}>", self.lb, self.rt)
        }
    }
}

//tp Default for Margin
impl std::default::Default for Margin {
    fn default() -> Self {
        Self::none()
    }
}

//ip std::ops::AddAssign<f64> for Margin
impl std::ops::AddAssign<f64> for Margin {
    #[inline]
    fn add_assign(&mut self, delta: f64) {
        self.lb += delta;
        self.rt += delta;
    }
}

//ip std::ops::SubAssign<f64> for Margin
impl std::ops::SubAssign<f64> for Margin {
    #[inline]
    fn sub_assign(&mut self, delta: f64) {
        self.lb -= delta;
        self.rt -= delta;
    }
}

//ip std::ops::MulAssign<f64> for Margin
impl std::ops::MulAssign<f64> for Margin {
    #[inline]
    fn mul_assign(&mut self, scale: f64) {
        self.rt *= scale;
        self.lb *= scale;
    }
}

//ip std::ops::DivAssign<f64> for Margin
impl std::ops::DivAssign<f64> for Margin {
    #[inline]
    fn div_assign(&mut self, scale: f64) {
        self.rt /= scale;
        self.lb /= scale;
    }
}

//ip std::ops::AddAssign<Margin> for Margin
impl std::ops::AddAssign<Margin> for Margin {
    #[inline]
    fn add_assign(&mut self, delta: Margin) {
        self.lb += delta.lb;
        self.rt += delta.rt;
    }
}

//ip std::ops::SubAssign<Margin> for Margin
impl std::ops::SubAssign<Margin> for Margin {
    #[inline]
    fn sub_assign(&mut self, delta: Margin) {
        self.lb -= delta.lb;
        self.rt -= delta.rt;
    }
}

//ip std::ops::Add<f64> for Margin
impl std::ops::Add<f64> for Margin {
    type Output = Self;
    #[inline]
    fn add(mut self, delta: f64) -> Self {
        self += delta;
        self
    }
}

//ip std::ops::Sub<f64> for Margin
impl std::ops::Sub<f64> for Margin {
    type Output = Self;
    #[inline]
    fn sub(mut self, delta: f64) -> Self {
        self -= delta;
        self
    }
}

//ip std::ops::Mul<f64> for Margin
impl std::ops::Mul<f64> for Margin {
    type Output = Self;
    #[inline]
    fn mul(mut self, scale: f64) -> Self {
        self *= scale;
        self
    }
}

//ip std::ops::Div<f64> for Margin
impl std::ops::Div<f64> for Margin {
    type Output = Self;
    #[inline]
    fn div(mut self, scale: f64) -> Self {
        self /= scale;
        self
    }
}

//ip std::ops::Index<usize> for Margin
impl std::ops::Index<usize> for Margin {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 2);
        if index == 0 {
            &self.lb
        } else {
            &self.rt
        }
    }
}

//a Tests
//mt Test for Margin
#[cfg(test)]
mod test_margin {
    use super::*;
    pub fn mgn_eq(rng: &Margin, lb: f64, rt: f64) {
        assert!(
            (rng.lb - lb).abs() < 1E-8,
            "mismatch in lb {:?} {} {}",
            rng,
            lb,
            rt
        );
        assert!(
            (rng.rt - rt).abs() < 1E-8,
            "mismatch in rt {:?} {} {}",
            rng,
            lb,
            rt
        );
    }
    #[test]
    fn test_simple() {
        assert!(Margin::none().is_none());
        mgn_eq(&Margin::new(1., 2.), 1., 2.);
        assert!(!Margin::new(0.1, 0.).is_none());
        assert!(!Margin::new(0., 0.1).is_none());
        mgn_eq(&(Margin::new(1., 2.) * 3.), 3., 6.);
        mgn_eq(&(Margin::new(3., 6.) / 3.), 1., 2.);

        assert_eq!(Margin::none().total(), 0.);
        assert_eq!(Margin::new(1., 0.).total(), 1.);
        assert_eq!(Margin::new(0., 1.).total(), 1.);
        assert_eq!(Margin::new(2., 0.).total(), 2.);
        assert_eq!(Margin::new(0., 2.).total(), 2.);
        assert_eq!((Margin::new(0., 2.) + 1.).total(), 4.);
        assert_eq!((Margin::new(0., 2.) - 1.).total(), 0.);
        assert_eq!(((Margin::new(0., 2.) + 1.) * 3.).total(), 12.);
        assert_eq!(((Margin::new(7., -8.) + 3.) / 5.).total(), 1.);
    }
}
