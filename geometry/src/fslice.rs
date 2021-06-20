//a Imports
use super::{Float, Vector, SqMatrix, Vector3D, Geometry3D};
use super::{vector, matrix};

//a FSlice
//tp FSlice
#[derive(Clone, Copy, Debug)]
pub struct FSlice<F:Float, const D:usize> { data: [F;D] }

//ip Add for FSlice
impl <F:Float, const D:usize> std::ops::Add for FSlice<F, D> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut data = [F::zero();D];
        for i in 0..D {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
}

//ip AddAssign for FSlice
impl <F:Float, const D:usize> std::ops::AddAssign for FSlice<F, D> {
    fn add_assign(&mut self, other: Self) { for i in 0..D {self.data[i] += other.data[i];} }
}

//ip Sub for FSlice
impl <F:Float, const D:usize> std::ops::Sub for FSlice<F, D> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut data = [F::zero();D];
        for i in 0..D {
            data[i] = self.data[i] - other.data[i];
        }
        Self { data }
    }
}

//ip SubAssign for FSlice
impl <F:Float, const D:usize> std::ops::SubAssign for FSlice<F, D> {
    fn sub_assign(&mut self, other: Self) { for i in 0..D {self.data[i] -= other.data[i];} }
}

//ip Mul<FSlice> for FSlice
impl <F:Float, const D:usize> std::ops::Mul<Self> for FSlice<F, D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = self.data;
        let data = vector::comp_mult(data, &rhs.data);
        Self { data }
    }
}

//ip MulAssign for FSlice
impl <F:Float, const D:usize> std::ops::MulAssign for FSlice<F, D> {
    fn mul_assign(&mut self, other: Self) { for i in 0..D {self.data[i] *= other.data[i];} }
}

//ip Mul<F> for FSlice
impl <F:Float, const D:usize> std::ops::Mul<F> for FSlice<F, D> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        let data = self.data;
        let data = vector::scale(data, rhs);
        Self { data }
    }
}

//ip MulAssign<F> for FSlice
impl <F:Float, const D:usize> std::ops::MulAssign<F> for FSlice<F, D> {
    fn mul_assign(&mut self, other: F) { for i in 0..D {self.data[i] *= other;} }
}

//ip Div<F> for FSlice
impl <F:Float, const D:usize> std::ops::Div<F> for FSlice<F, D> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        let data = self.data;
        let data = vector::reduce(data, rhs);
        Self { data }
    }
}

//ip DivAssign<F> for FSlice
impl <F:Float, const D:usize> std::ops::DivAssign<F> for FSlice<F, D> {
    fn div_assign(&mut self, other: F) { for i in 0..D {self.data[i] /= other;} }
}

//ip Vector<F,D> for FSlice
impl <F:Float, const D:usize> std::convert::AsRef<[F;D]> for FSlice<F, D> {
    fn as_ref(&self) -> &[F;D] {&self.data}
}
impl <F:Float, const D:usize> Vector<F, D> for FSlice<F, D> {
    fn from_array(data:[F;D]) -> Self { Self { data  } }
    fn zero() -> Self {
        Self { data:vector::zero() }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
    fn mix(&self, other:&Self, t:F) -> Self {
        Self { data:vector::mix(&self.data, &other.data, t) }
    }
    fn reduce_sum(&self) -> F {
        let mut r = F::zero();
        for d in self.data { r = r + d }
        r
    }
    fn dot(&self, other:&Self) -> F {
        vector::dot(&self.data, &other.data)
    }
}

//a FSlice2
//tp FSlice2
#[derive(Clone, Copy, Debug)]
pub struct FSlice2<F:Float, const D:usize, const D2:usize> { data: [F;D2] }

//ip Add for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::ops::Add for FSlice2<F, D, D2> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut data = [F::zero();D2];
        for i in 0..D {
            data[i] = self.data[i] + other.data[i];
        }
        Self { data }
    }
}

//ip Sub for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::ops::Sub for FSlice2<F, D, D2> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut data = [F::zero();D2];
        for i in 0..D {
            data[i] = self.data[i] - other.data[i];
        }
        Self { data }
    }
}

//ip Mul<FSlice2> for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::ops::Mul<Self> for FSlice2<F, D, D2> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data = self.data;
        let data = vector::comp_mult(data, &rhs.data);
        Self { data }
    }
}

//ip Mul<F> for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::ops::Mul<F> for FSlice2<F, D, D2> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        let data = self.data;
        let data = vector::scale(data, rhs);
        Self { data }
    }
}

//ip Div<F> for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::ops::Div<F> for FSlice2<F, D, D2> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        let data = self.data;
        let data = vector::reduce(data, rhs);
        Self { data }
    }
}

//ip SqMatrix<F,D,D2> for FSlice2
impl <F:Float, const D:usize, const D2:usize> std::convert::AsRef<[F;D2]> for FSlice2<F, D, D2> {
    fn as_ref(&self) -> &[F;D2] {&self.data}
}
impl <F:Float, const D:usize, const D2:usize> SqMatrix<FSlice<F,D>, F, D, D2> for FSlice2<F, D, D2> {
    fn from_array(data:[F;D2]) -> Self { Self { data  } }
    fn zero() -> Self {
        Self { data:vector::zero() }
    }
    fn identity() -> Self {
        Self { data:vector::zero() }
    }
    fn is_zero(&self) -> bool {
        vector::is_zero(&self.data)
    }
    fn set_zero(&mut self) {
        vector::set_zero(&mut self.data)
    }
}

//a Vector3D and Geometry3D for FSlice/FSlice2
//ip Vector3D for f32
impl Vector3D<f32> for f32 {
    type Vec2 = FSlice<f32,2>;
    type Vec3 = FSlice<f32,3>;
    type Vec4 = FSlice<f32,4>;
}

//ip Geometry3D for f32
impl Geometry3D<f32> for f32 {
    type Vec3 = FSlice<f32,3>;
    type Vec4 = FSlice<f32,4>;
    type Mat3 = FSlice2<f32,3,9>;
    type Mat4 = FSlice2<f32,4,16>;
    fn transform3(m:&Self::Mat3, v:Self::Vec3) -> Self::Vec3 {
        Self::Vec3::from_array(matrix::multiply::<f32,9,3,3,3,3,1> (m.as_ref(), v.as_ref()))
    }
}

