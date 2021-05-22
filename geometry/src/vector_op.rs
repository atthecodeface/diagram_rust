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

//a Imports
use num_traits::{Num, Float};
use std::fmt::Display;
use super::matrix_op as matrix;
use super::matrixr_op as matrixr;

//a VectorCoord
//tp VectorCoord
/// Trait required for a vector coordinate - satisfied by f32 and f64
pub trait VectorCoord : std::ops::Neg<Output=Self>+Num+Clone+Copy+PartialEq+Display {
}

//fp origin
/// Create a zero vector of the correct size
pub fn origin<V:VectorCoord,const D:usize> () -> [V; D] { [V::zero();D] }

//mp set_zero
/// Set the vector to be zero
pub fn set_zero<V:VectorCoord,const D:usize> (v:&mut [V;D]) {
    for c in v.iter_mut() { c.set_zero(); }
}

//cp zero
/// Set the vector to be zero
pub fn zero<V:VectorCoord,const D:usize> (mut v:[V;D]) -> [V;D] {
    for c in &mut v { c.set_zero(); }
    v
}

//fp is_zero
/// Return true if the point is the origin
pub fn is_zero<V:VectorCoord,const D:usize> (v:&[V;D]) -> bool {
    for c in v { if !c.is_zero() {return false;}}
    true
}

//cp scale
/// Consume the vector and return a new vector that is the original
/// scaled in each coordinate a single scaling factor
pub fn scale<V:VectorCoord,const D:usize> (mut v:[V;D], s:V) -> [V;D] {
    for c in &mut v { *c = (*c) * s; }
    v
}

//cp scale_coords
/// Consume the vector and return a new vector that is the original
/// scaled in each coordinate a different scale factor
pub fn scale_coords<V:VectorCoord,const D:usize> (mut v:[V;D], s:&[V;D]) -> [V;D] {
    for i in 0..D { v[i] = v[i] * s[i]; }
    v
}

//cp reduce
/// Consume the vector and return a new vector that is the original
/// reduces in scale in each coordinate by a single scaling factor
pub fn reduce<V:VectorCoord,const D:usize> (mut v:[V;D], s:V) -> [V;D] {
    for c in &mut v { *c = (*c) / s; }
    v
}

//cp add
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn add<V:VectorCoord,const D:usize> (mut v:[V;D], other:&[V;D], scale:V) -> [V;D] {
    for i in 0..D {
        v[i] = v[i] + other[i] * scale;
    }
    v
}

//cp sub
/// Consume the vector, and return a new vector that is the sum of
/// this and a borrowed other vector scaled
pub fn sub<V:VectorCoord,const D:usize> (mut v:[V;D], other:&[V;D], scale:V) -> [V;D] {
    for i in 0..D {
        v[i] = v[i] - other[i] * scale;
    }
    v
}

//mp len2
/// Return the length^2 of the vector
pub fn len2<V:VectorCoord> (v:&[V]) -> V {
    let mut r = V::zero();
    for c in v.iter() { r = r + (*c) * (*c) }
    r
}

//mp len
/// Return the length of the vector
pub fn len<V:VectorCoord+Float> (v:&[V]) -> V {
    len2(v).sqrt()
}

//mp distance_to2
/// Return the distance square between two vectors
pub fn distance_to2<V:VectorCoord,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    let mut r = V::zero();
    for i in 0..D {
        let d = v[i] - other[i];
        r = r + d * d;
    }
    r
}

//mp distance_to
/// Return the distance between two vectors
pub fn distance_to<V:VectorCoord+Float,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    distance_to2(v,other).sqrt()
}

//mp inner_product
/// Return the inner product (dot product) of this and another vector
pub fn inner_product<V:VectorCoord,const D:usize> (v:&[V;D], other:&[V;D]) -> V {
    let mut r = V::zero();
    for i in 0..D {
        r = r + v[i]*other[i];
    }
    r
}

//mp normalize
/// Create a new vector with zeros
pub fn normalize<V:VectorCoord+Float,const D:usize> (v:&mut [V;D], eps:V) {
    let l = len(v);
    if l < eps {
        set_zero(v);
    } else {
        *v = reduce(*v, l);
    }
}

//cp rotate_around
/// Consume the vector and return a new vector rotated around a
/// *pivot* point by the specified angle
pub fn rotate_around<V:VectorCoord+Float,const D:usize> (mut v:[V;D], pivot:&[V;D], degrees:V, c0:usize, c1:usize) -> [V;D] {
    let (s,c) = degrees.to_radians().sin_cos();
    let dx = v[c0] - pivot[c0];
    let dy = v[c1] - pivot[c1];
    let x1 = c*dx - s*dy;
    let y1 = c*dy + s*dx;
    v[c0] = x1 + pivot[c0];
    v[c1] = y1 + pivot[c1];
    v
}

//mp cross_product3
/// Return the inner product (dot product) of this and another vector
pub fn cross_product3<V:VectorCoord> (x:&[V;3], y:&[V;3]) -> [V;3] {
    let c0 = x[1] * y[2] - x[2] * y[1];
    let c1 = x[2] * y[0] - x[0] * y[2];
    let c2 = x[0] * y[1] - x[1] * y[0];
    [c0, c1, c2]
}

//mp fmt - format a `Vector` for display
/// Display the `Point' as (x,y)
pub fn fmt<V:VectorCoord + Display>(f: &mut std::fmt::Formatter, v : &[V]) -> std::fmt::Result {
    for i in 0..v.len() {
        if i==0 {
            write!(f, "({}", v[i])?;
        } else {
            write!(f, ",{}", v[i])?;
        }
    }
    write!(f, ")")
}

/*
    #f transformMat3
    @staticmethod
    def transformMat4(a:Vec4,x:Vec4,M:Mat4) -> Vec4:
        c0=M[0]*x[0] + M[4]*x[1] + M[8]*x[2]  + M[12]*x[3];
        c1=M[1]*x[0] + M[5]*x[1] + M[9]*x[2]  + M[13]*x[3];
        c2=M[2]*x[0] + M[6]*x[1] + M[10]*x[2] + M[14]*x[3];
        c3=M[3]*x[0] + M[7]*x[1] + M[11]*x[2] + M[15]*x[3];
        a[0]=c0; a[1]=c1; a[2]=c2; a[3]=c3;
        return a
 */

//fp axis_of_rotation3
/// Find the axis of rotation of a Matrix3
///    Note that R . axis = axis
///    RI = (R - I*999/1000)
///    Then RI . axis = axis/1000
///    Then det(RI) is going to be 1/1000 * at most 2 * at most 2
///    And if na is perp to axis, then RI.na = R.na - 999/1000.na, which is perp to axis
///    Then |R.na| < 2|na|
///    If RI' . RI = I, then consider v' = RI' . v for some v=(a*axis + b*na0 + c*na1)
///    (a'*axis + b'*na0 + c'*na1) = RI' . (a*axis + b*na0 + c*na1)
///    Then RI . (a'*axis + b'*na0 + c'*na1) = (a*axis + b*na0 + c*na1)
///    Then a'*RI.axis + b'*RI.na0 + c'*RI.na1 = a*axis + b*na0 + c*na1
///    Then a'/1000*axis + b'*(R.na0-0.999.na0) + c'*(R.na1-0.999.na1) = a*axis + b*na0 + c*na1
///    Then a = a'/1000, and
///    -0.999b' + b'cos(angle) + c'sin(angle) = b, etc
///    If we set |v| to be 1, then |v'| must be det(RI') = 1/det(RI) > 100
///    If angle is not close to zero, then a' / b' >> 1
///    This can be repeated:
///    v' = normalize(RI' . v)
///    v'' = normalize(RI' . v')
///    v''' = normalize(RI' . v'') etc
///    This gets closer and closer to the axis
pub fn axis_of_rotation3<V:VectorCoord+Float>(rotation:&[V;9]) -> [V;3] {
    let mut rot_min_id = rotation.clone();
    let almost_one  = V::from(99999).unwrap() / V::from(100000).unwrap();
    let almost_zero = V::one() / V::from(100000).unwrap();
    rot_min_id[0] = rot_min_id[8] - almost_one;
    rot_min_id[4] = rot_min_id[8] - almost_one;
    rot_min_id[8] = rot_min_id[8] - almost_one;
    let rot_min_id_i = matrix::inverse3(&rot_min_id);
    for j in 0..3 {
        let mut v = [V::zero(); 3];
        v[j] = V::one();
        let mut last_v = [V::zero(); 3];
        for _ in 0..10 {
            last_v = v;
            v = matrixr::transform_vec::<V,3,3>( &rot_min_id_i, &v );
            normalize(&mut v, V::epsilon());
        }
        if distance_to2(&v, &last_v) < almost_zero { return v; }
    }
    [V::zero(); 3]
}

//ti VectorCoord for f32/f64
impl VectorCoord for f32 {}
impl VectorCoord for f64 {}

