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

@file    quaternion_op.rs
@brief   Part of geometry library
 */

//a Imports
use num_traits::{Float};
use crate::VectorCoord;
use crate::vector_op as vector;
use crate::matrix_op as matrix;
use crate::matrixr_op as matrixr;

//fp new
/// Create a new quaternion
pub fn new<V:VectorCoord>() -> [V; 4] {
    [V::zero(), V::zero(), V::zero(), V::one(), ]
}

//fp as_rijk
/// Return the breakdown of a quaternion
pub fn as_rijk<V:VectorCoord>(v:&[V;4]) -> (V, V, V, V) {
    (v[3], v[0], v[1], v[2])
}

//fp of_rijk
/// Create a quaternion from its components
pub fn of_rijk<V:VectorCoord>(r:V, i:V, j:V, k:V) -> [V;4] {
    [i, j, k, r]
}

//fp identity
/// Create an identity quaternion
pub fn identity<V:VectorCoord>() -> [V;4] {
    [V::zero(), V::zero(), V::zero(), V::one()]
}

//mp invert
/// Get the quaternion inverse
pub fn invert<V:VectorCoord+Float>(a:&[V;4]) -> [V;4] {
    let l = vector::len2(a);
    let r_l = {
        if l < V::epsilon() { V::zero() } else {V::one()/l}
    };
    [ -a[0]*r_l,
      -a[1]*r_l,
       -a[2]*r_l,
       a[3]*r_l ]
}

//fp conjugate
/// Find the conjugate of a quaternion
pub fn conjugate<V:VectorCoord>(a:&[V;4]) -> [V;4] {
    [ -a[0], -a[1], -a[2], a[3] ]
}

//mp normalize
/// Find the conjugate of a quaternion
pub fn normalize<V:VectorCoord+Float>(a:&mut [V;4], epsilon:V) {
    vector::normalize(a, epsilon)
}

//fp rotate_x
/// Find a rotation about the X-axis
pub fn rotate_x<V:VectorCoord+Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[3] * s;
    let j = a[1] * c + a[2] * s;
    let k = a[2] * c - a[1] * s;
    let r = a[3] * c - a[0] * s;
    [ i, j, k, r ]
}

//fp rotate_y
/// Find a rotation about the Y-axis
pub fn rotate_y<V:VectorCoord+Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c - a[2] * s;
    let j = a[1] * c + a[3] * s;
    let k = a[2] * c + a[0] * s;
    let r = a[3] * c - a[1] * s;
    [ i, j, k, r ]
}

//fp rotate_z
/// Find a rotation about the Z-axis
pub fn rotate_z<V:VectorCoord+Float>(a:&[V;4], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
    let i = a[0] * c + a[1] * s;
    let j = a[1] * c - a[0] * s;
    let k = a[2] * c + a[3] * s;
    let r = a[3] * c - a[2] * s;
    [ i, j, k, r ]
}

//fp distance_to2
/// Get a measure of the 'distance' between two quaternions
pub fn distance_to2<V:VectorCoord+Float>(a:&[V;4], b:&[V;4]) -> V {
    let qi = invert(a);
    let mut qn = multiply(&qi, b);
    if qn[3] < V::zero() {
        qn[3] = qn[3] + V::one();
    } else {
        qn[3] = qn[3] - V::one();
    }
    vector::len2(&qn)
}

//fp distance_to
/// Get a measure of the 'distance' between two quaternions
pub fn distance_to<V:VectorCoord+Float>(a:&[V;4], b:&[V;4]) -> V {
    distance_to2(a,b).sqrt()
}

//fp multiply
/// Multiply two quaternions together
pub fn multiply<V:VectorCoord>(a:&[V;4], b:&[V;4]) -> [V;4] {
    let i = a[0]*b[3] + a[3]*b[0] + a[1]*b[2] - a[2]*b[1];
    let j = a[1]*b[3] + a[3]*b[1] + a[2]*b[0] - a[0]*b[2];
    let k = a[2]*b[3] + a[3]*b[2] + a[0]*b[1] - a[1]*b[0];
    let r = a[3]*b[3] - a[0]*b[0] - a[1]*b[1] - a[2]*b[2];
    [ i, j, k, r ]
}

//fp of_axis_angle
/// Find the quaternion for a rotation of an angle around an axis
pub fn of_axis_angle<V:VectorCoord+Float>(axis:&[V;3], angle:V) -> [V;4] {
    let (s,c) = V::sin_cos(angle / V::from(2).unwrap());
        let i = s * axis[0];
        let j = s * axis[1];
        let k = s * axis[2];
        let r = c;
    [ i, j, k, r ]
}

//fp get_axis_angle
/// Get the axis of a quaternion, and the angle of rotation it corresponds to
pub fn get_axis_angle<V:VectorCoord+Float>(q:&[V;4]) -> ([V;3], V) {
    let angle = V::acos(q[3]);
    let mut axis = [q[0], q[1], q[2]];
    vector::normalize(&mut axis, V::epsilon());
    (axis, angle)
}

//fp to_euler
pub fn to_euler<V:VectorCoord+Float>(q:&[V;4]) -> (V,V,V) {
    let i=q[0];
    let j=q[1];
    let k=q[2];
    let r=q[3];
    let test = i*j + r*k;
    let two = V::from(2).unwrap();
    let almost_half = V::from(4_999_999).unwrap() / V::from(10_000_000).unwrap();
    let halfpi = V::zero().acos();
    let (heading, attitude, bank) = {
        if test > almost_half {
            (two*V::atan2(i,r), halfpi, V::zero())
        } else if test < -almost_half {
            (-two*V::atan2(i,r), -halfpi, V::zero())
        } else {
            let i2 = i*i;
            let j2 = j*j;
            let k2 = k*k;
            (V::atan2(two*j*r - two*i*k , V::one() - two*j2 - two*k2),
             V::asin(two*test),
             V::atan2(two*i*r - two*j*k , V::one() - two*i2 - two*k2)
            )
        }
    };
    (bank, heading, attitude)
}

pub fn nlerp<V:VectorCoord+Float>(t:V, in0:&[V;4], in1:&[V;4]) -> [V;4] {
    let mut r = [V::zero(); 4];
    let tn = V::one() - t;
    for i in 0..4 {
        r[i] = t * in0[i] + tn * in1[i];
    }
    normalize(&mut r, V::epsilon());
    r
}

//
//    Note that R . axis = axis
//    RI = (R - I*999/1000)
//    Then RI . axis = axis/1000
//    Then det(RI) is going to be 1/1000 * at most 2 * at most 2
//    And if na is perp to axis, then RI.na = R.na - 999/1000.na, which is perp to axis
//    Then |R.na| < 2|na|
//    If RI' . RI = I, then consider v' = RI' . v for some v=(a*axis + b*na0 + c*na1)
//    (a'*axis + b'*na0 + c'*na1) = RI' . (a*axis + b*na0 + c*na1)
//    Then RI . (a'*axis + b'*na0 + c'*na1) = (a*axis + b*na0 + c*na1)
//    Then a'*RI.axis + b'*RI.na0 + c'*RI.na1 = a*axis + b*na0 + c*na1
//    Then a'/1000*axis + b'*(R.na0-0.999.na0) + c'*(R.na1-0.999.na1) = a*axis + b*na0 + c*na1
//    Then a = a'/1000, and
//    -0.999b' + b'cos(angle) + c'sin(angle) = b, etc
//    If we set |v| to be 1, then |v'| must be det(RI') = 1/det(RI) > 100
//    If angle is not close to zero, then a' / b' >> 1
//    This can be repeated:
//    v' = normalize(RI' . v)
//    v'' = normalize(RI' . v')
//    v''' = normalize(RI' . v'') etc
//    This gets closer and closer to the axis
pub fn axis_of_rotation<V:VectorCoord+Float>(rotation:&[V;9]) -> [V;3] {
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
            vector::normalize(&mut v, V::epsilon());
        }
        if vector::distance_to2(&v, &last_v) < almost_zero { return v; }
    }
    [V::zero(); 3]
}

pub fn of_rotation<V:VectorCoord+Float>(rotation:&[V;9]) -> [V;4] {
    let axis = axis_of_rotation(rotation);

    // Find a decent vector not parallel to the axis
    let mut w = [V::one(), V::zero(), V::zero()];
    if V::abs(axis[0]) > (V::from(9).unwrap() / V::from(10).unwrap()) { w[0] = V::zero(); w[1] = V::one(); }

    // Find three vectors (axis, na0, na1) that are all mutually perpendicular
    let mut na0 = vector::cross_product3(&w, &axis);
    vector::normalize(&mut na0, V::epsilon());
    let mut na1 = vector::cross_product3(&axis, &na0);
    vector::normalize(&mut na1, V::epsilon());

    // Rotate na0, na1 around the axis of rotation by angle A - i.e. apply 'rotation'
    let na0_r = matrixr::transform_vec::<V,3,3>( &rotation, &na0 );
    let na1_r = matrixr::transform_vec::<V,3,3>( &rotation, &na1 );

    //  Get angle of rotation
    let cos_angle =  vector::inner_product(&na0, &na0_r);
    let sin_angle = -vector::inner_product(&na0, &na1_r);
    let angle     = V::atan2(sin_angle, cos_angle);

    of_axis_angle(&axis, angle)
}

