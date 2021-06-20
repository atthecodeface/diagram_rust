use super::{Vector, Vector3D};

//a F32x4Vec4
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec4 (core_simd::f32x4);

//ip Add for F32x4Vec4
impl std::ops::Add<F32x4Vec4> for F32x4Vec4 { type Output = Self; fn add(self, other: Self) -> Self { Self(self.0 + other.0) } }
impl std::ops::Sub<F32x4Vec4> for F32x4Vec4 { type Output = Self; fn sub(self, other: Self) -> Self { Self(self.0 - other.0) } }
impl std::ops::Mul<F32x4Vec4> for F32x4Vec4 { type Output = Self; fn mul(self, other: Self) -> Self { Self(self.0 * other.0) } }
impl std::ops::Mul<f32>       for F32x4Vec4 { type Output = Self; fn mul(self, other: f32)  -> Self { Self(self.0 * other) } }
impl std::ops::Div<f32>       for F32x4Vec4 { type Output = Self; fn div(self, other: f32)  -> Self { Self(self.0 / other) } }
impl std::ops::AddAssign<F32x4Vec4> for F32x4Vec4 { fn add_assign(&mut self, other: Self) { self.0 += other.0; } }
impl std::ops::SubAssign<F32x4Vec4> for F32x4Vec4 { fn sub_assign(&mut self, other: Self) { self.0 -= other.0; } }
impl std::ops::MulAssign<F32x4Vec4> for F32x4Vec4 { fn mul_assign(&mut self, other: Self) { self.0 *= other.0; } }
impl std::ops::MulAssign<f32>       for F32x4Vec4 { fn mul_assign(&mut self, other: f32)  { self.0 *= other; } }
impl std::ops::DivAssign<f32>       for F32x4Vec4 { fn div_assign(&mut self, other: f32)  { self.0 /= other; } }
impl std::convert::AsRef<[f32;4]> for F32x4Vec4 { fn as_ref(&self) -> &[f32;4] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;4]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec4 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;4]>(&self.0) } } }
impl std::convert::AsMut<[f32;4]> for F32x4Vec4 { fn as_mut(&mut self) -> &mut [f32;4] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;4]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec4 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;4]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec4 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec4 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec4 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }

impl Vector<f32, 4> for F32x4Vec4 {
    fn from_array(data:[f32;4]) -> Self { Self(core_simd::f32x4::from_array(data)) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}


//a F32x4Vec3
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec3 (core_simd::f32x4);

//ip Add for F32x4Vec3
impl std::ops::Add<F32x4Vec3> for F32x4Vec3 { type Output = Self; fn add(self, other: Self) -> Self { Self(self.0 + other.0) } }
impl std::ops::Sub<F32x4Vec3> for F32x4Vec3 { type Output = Self; fn sub(self, other: Self) -> Self { Self(self.0 - other.0) } }
impl std::ops::Mul<F32x4Vec3> for F32x4Vec3 { type Output = Self; fn mul(self, other: Self) -> Self { Self(self.0 * other.0) } }
impl std::ops::Mul<f32>       for F32x4Vec3 { type Output = Self; fn mul(self, other: f32)  -> Self { Self(self.0 * other) } }
impl std::ops::Div<f32>       for F32x4Vec3 { type Output = Self; fn div(self, other: f32)  -> Self { Self(self.0 / other) } }
impl std::ops::AddAssign<F32x4Vec3> for F32x4Vec3 { fn add_assign(&mut self, other: Self) { self.0 += other.0; } }
impl std::ops::SubAssign<F32x4Vec3> for F32x4Vec3 { fn sub_assign(&mut self, other: Self) { self.0 -= other.0; } }
impl std::ops::MulAssign<F32x4Vec3> for F32x4Vec3 { fn mul_assign(&mut self, other: Self) { self.0 *= other.0; } }
impl std::ops::MulAssign<f32>       for F32x4Vec3 { fn mul_assign(&mut self, other: f32)  { self.0 *= other; } }
impl std::ops::DivAssign<f32>       for F32x4Vec3 { fn div_assign(&mut self, other: f32)  { self.0 /= other; } }
impl std::convert::AsRef<[f32;3]> for F32x4Vec3 { fn as_ref(&self) -> &[f32;3] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;3]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec3 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;3]>(&self.0) } } }
impl std::convert::AsMut<[f32;3]> for F32x4Vec3 { fn as_mut(&mut self) -> &mut [f32;3] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;3]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec3 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;3]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec3 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec3 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec3 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }

impl Vector<f32, 3> for F32x4Vec3 {
    fn from_array(data:[f32;3]) -> Self { Self(core_simd::f32x4::from_array([data[0],data[1],data[2],0.])) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

//a F32x4Vec2
#[derive(Clone, Copy, Debug)]
pub struct F32x4Vec2 (core_simd::f32x4);

//ip Add for F32x4Vec2
impl std::ops::Add<F32x4Vec2> for F32x4Vec2 { type Output = Self; fn add(self, other: Self) -> Self { Self(self.0 + other.0) } }
impl std::ops::Sub<F32x4Vec2> for F32x4Vec2 { type Output = Self; fn sub(self, other: Self) -> Self { Self(self.0 - other.0) } }
impl std::ops::Mul<F32x4Vec2> for F32x4Vec2 { type Output = Self; fn mul(self, other: Self) -> Self { Self(self.0 * other.0) } }
impl std::ops::Mul<f32>       for F32x4Vec2 { type Output = Self; fn mul(self, other: f32)  -> Self { Self(self.0 * other) } }
impl std::ops::Div<f32>       for F32x4Vec2 { type Output = Self; fn div(self, other: f32)  -> Self { Self(self.0 / other) } }
impl std::ops::AddAssign<F32x4Vec2> for F32x4Vec2 { fn add_assign(&mut self, other: Self) { self.0 += other.0; } }
impl std::ops::SubAssign<F32x4Vec2> for F32x4Vec2 { fn sub_assign(&mut self, other: Self) { self.0 -= other.0; } }
impl std::ops::MulAssign<F32x4Vec2> for F32x4Vec2 { fn mul_assign(&mut self, other: Self) { self.0 *= other.0; } }
impl std::ops::MulAssign<f32>       for F32x4Vec2 { fn mul_assign(&mut self, other: f32)  { self.0 *= other; } }
impl std::ops::DivAssign<f32>       for F32x4Vec2 { fn div_assign(&mut self, other: f32)  { self.0 /= other; } }
impl std::convert::AsRef<[f32;2]> for F32x4Vec2 { fn as_ref(&self) -> &[f32;2] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;2]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x4Vec2 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x4, &[f32;2]>(&self.0) } } }
impl std::convert::AsMut<[f32;2]> for F32x4Vec2 { fn as_mut(&mut self) -> &mut [f32;2] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;2]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x4Vec2 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x4, &mut [f32;2]>(&mut self.0) } } }
impl std::default::Default for F32x4Vec2 { fn default() -> Self { Self ( core_simd::f32x4::default() ) } }
impl std::ops::Index<usize> for F32x4Vec2 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x4Vec2 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }

impl Vector<f32, 2> for F32x4Vec2 {
    fn from_array(data:[f32;2]) -> Self { Self(core_simd::f32x4::from_array([data[0],data[1],0.,0.])) }
    fn zero() -> Self { Self(core_simd::f32x4::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x4::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x4::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

//a F32x2Vec2
#[derive(Clone, Copy, Debug)]
pub struct F32x2Vec2 (core_simd::f32x2);

//ip Add for F32x2Vec2
impl std::ops::Add<F32x2Vec2> for F32x2Vec2 { type Output = Self; fn add(self, other: Self) -> Self { Self(self.0 + other.0) } }
impl std::ops::Sub<F32x2Vec2> for F32x2Vec2 { type Output = Self; fn sub(self, other: Self) -> Self { Self(self.0 - other.0) } }
impl std::ops::Mul<F32x2Vec2> for F32x2Vec2 { type Output = Self; fn mul(self, other: Self) -> Self { Self(self.0 * other.0) } }
impl std::ops::Mul<f32>       for F32x2Vec2 { type Output = Self; fn mul(self, other: f32)  -> Self { Self(self.0 * other) } }
impl std::ops::Div<f32>       for F32x2Vec2 { type Output = Self; fn div(self, other: f32)  -> Self { Self(self.0 / other) } }
impl std::ops::AddAssign<F32x2Vec2> for F32x2Vec2 { fn add_assign(&mut self, other: Self) { self.0 += other.0; } }
impl std::ops::SubAssign<F32x2Vec2> for F32x2Vec2 { fn sub_assign(&mut self, other: Self) { self.0 -= other.0; } }
impl std::ops::MulAssign<F32x2Vec2> for F32x2Vec2 { fn mul_assign(&mut self, other: Self) { self.0 *= other.0; } }
impl std::ops::MulAssign<f32>       for F32x2Vec2 { fn mul_assign(&mut self, other: f32)  { self.0 *= other; } }
impl std::ops::DivAssign<f32>       for F32x2Vec2 { fn div_assign(&mut self, other: f32)  { self.0 /= other; } }
impl std::convert::AsRef<[f32;2]> for F32x2Vec2 { fn as_ref(&self) -> &[f32;2] {unsafe {std::mem::transmute::<&core_simd::f32x2, &[f32;2]>(&self.0) } } }
impl std::convert::AsRef<[f32]> for F32x2Vec2 { fn as_ref(&self) -> &[f32] {unsafe {std::mem::transmute::<&core_simd::f32x2, &[f32;2]>(&self.0) } } }
impl std::convert::AsMut<[f32;2]> for F32x2Vec2 { fn as_mut(&mut self) -> &mut [f32;2] {unsafe {std::mem::transmute::<&mut core_simd::f32x2, &mut [f32;2]>(&mut self.0) } } }
impl std::convert::AsMut<[f32]> for F32x2Vec2 { fn as_mut(&mut self) -> &mut [f32] {unsafe {std::mem::transmute::<&mut core_simd::f32x2, &mut [f32;2]>(&mut self.0) } } }
impl std::default::Default for F32x2Vec2 { fn default() -> Self { Self ( core_simd::f32x2::default() ) } }
impl std::ops::Index<usize> for F32x2Vec2 { type Output = f32; fn index(&self, index: usize) -> &f32 { let x:&[f32]=self.as_ref(); &x[index] } }
impl std::ops::IndexMut<usize> for F32x2Vec2 { fn index_mut(&mut self, index: usize) -> &mut f32 { let x:&mut [f32]=self.as_mut(); &mut x[index] } }

impl Vector<f32, 2> for F32x2Vec2 {
    fn from_array(data:[f32;2]) -> Self { Self(core_simd::f32x2::from_array(data)) }
    fn zero() -> Self { Self(core_simd::f32x2::splat(0.)) }
    fn is_zero(&self) -> bool { self.0.lanes_eq(core_simd::f32x2::splat(0.)).all() }
    fn set_zero(&mut self)  { self.0 =  core_simd::f32x2::splat(0.) }
    fn mix(&self, other:&Self, t:f32) -> Self { Self(self.0*(1.0-t) + other.0*t) }
    fn reduce_sum(&self) -> f32 {self.0.horizontal_sum()}
    fn dot(&self, other:&Self) -> f32 {(self.0 * other.0).horizontal_sum()}
}

//a SimdVecF32A16, SimdVecF32A8
//tp SimdVecF32A16 - empty struct that provides a wrapper for the associated types
pub struct SimdVecF32A16 {}

impl Vector3D<f32> for SimdVecF32A16 {
    type Vec2   = F32x4Vec2;
    type Vec3   = F32x4Vec3;
    type Vec4   = F32x4Vec4;
}

//tp SimdVecF32A8 - empty struct that provides a wrapper for the associated types
pub struct SimdVecF32A8 {}
impl Vector3D<f32> for SimdVecF32A8 {
    type Vec2   = F32x2Vec2;
    type Vec3   = F32x4Vec3;
    type Vec4   = F32x4Vec4;
}

/*
#[cfg(test)]
fn test() {
    let x = SimdVecF32A16{};
    let y = <SimdVecF32A16 as Vector3D>::Vec3::zero();
}
 */
