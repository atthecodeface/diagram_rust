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

@file    vector_op.rs
@brief   Part of geometry library
 */

//a VectorOp trait
pub trait VectorOp {
    type Value;
    fn zero<const D:usize> () -> [Self::Value; D];
    fn zero2() -> [Self::Value; 2];
    fn fmt(f: &mut std::fmt::Formatter, v : &[Self::Value]) -> std::fmt::Result;
}

//a Num and Float traits
//tp Num
/// Trait required for matrix or vector elements
pub trait Num : std::ops::Neg<Output=Self> + num_traits::Num +
    Clone + Copy + PartialEq + std::fmt::Display + std::fmt::Debug {}

//tp Float
/// Trait required for matrix or vector elements such that also need operations such as sqrt, sin/cos, etc
pub trait Float : Num + num_traits::Float {}

//ti Num for f32/f64/i32/i64/isize
impl Num for f32 {}
impl Num for f64 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for isize {}

//ti Float for f32/f64
impl Float for f32 {}
impl Float for f64 {}

//a Vector and SqMatrix
//tt Vector
pub trait Vector<F:Float, const D:usize> : Copy
    + std::convert::AsRef<[F;D]>
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Mul<F> +
    + std::ops::Div<F> {
        fn from_array(data:[F;D]) -> Self;
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
        fn set_zero(&mut self);
        fn reduce_sum(&self) -> F;
        fn mix(&self, other:&Self, t:F) -> Self;
        fn dot(&self, other:&Self) -> F;
        fn length_sq(&self) -> F { self.dot(self) }
        fn length(&self)    -> F { self.length_sq().sqrt() }
        fn distance_sq(&self, other:&Self) -> F { (*self - *other).length_sq() }
        fn distance(&self, other:&Self) -> F { self.distance_sq(other).sqrt() }
        // clamp
        // normalize
        // rotate_around
    }

//tt SqMatrix
pub trait SqMatrix<F:Float, const D:usize, const D2:usize> : Copy
    + std::convert::AsRef<[F;D2]>
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Mul<F> +
    + std::ops::Div<F> {
        fn from_array(data:[F;D2]) -> Self;
        fn identity() -> Self;
        fn zero() -> Self;
        fn is_zero(&self) -> bool;
        fn set_zero(&mut self);
        // absmax
        // transpose
        // fn determinant(&self) -> F;
        // fn inverse(&self) -> Self;
    }

//a Vector3D, Geometry3D
//tt Vector3D
pub trait Vector3D<Scalar:Float> {
    type Vec2 : Vector<Scalar, 2>;
    type Vec3 : Vector<Scalar, 3>;
    type Vec4 : Vector<Scalar, 4>;
}

//tt Geometry3D
pub trait Geometry3D<Scalar:Float> {
    type Vec3 : Vector<Scalar, 3>;
    type Vec4 : Vector<Scalar, 4>;
    type Mat3 : SqMatrix<Scalar, 3, 9>;
    type Mat4 : SqMatrix<Scalar, 4, 16>;
    // Might need to move transform3 to be inside Mat3 in which case Vec3 has to be part of Mat3 too
    fn transform3(m:&Self::Mat3, v:Self::Vec3) -> Self::Vec3;
    // fn perspective4
    // fn translate4
    // fn from_quat3
    // fn from_quat4
    // fn of_transform3/4?
    // cross_product3
    // axis_of_rotation3/4
    // clamp
}

