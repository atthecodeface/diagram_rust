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

@file    transform.rs
@brief   Transformation class
 */

//a Imports
use super::Point;
use geo_nd::Vector;

//a Constants
// const DEBUG_TRANSFORM     : bool = true;

//a Transform type
//tp Transform
/// A Transfom is a transformation applied to something - for example, applied to content to present it in its parent coordinates.
///
/// The transformation is translate(rotate(scale(pt)))
///
#[derive(Debug, Clone)]
pub struct Transform {
    /// Translation - applied last
    pub translation: Point,
    /// Rotation around the origin
    pub rotation: f64,
    /// Scale factor
    pub scale: f64,
}

//ti Transform
impl Transform {
    //fp new
    /// Create a new identity transform
    pub fn new() -> Self {
        Self {
            translation: Point::zero(),
            rotation: 0.,
            scale: 1.,
        }
    }

    //fp of_trs
    /// Create a transform from a translation, rotation and scale
    pub fn of_trs(translation: Point, rotation: f64, scale: f64) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    //mp of_translation
    /// Create a transform from a translation
    pub fn of_translation(translation: Point) -> Self {
        Self::of_trs(translation, 0., 1.)
    }

    //fp of_matrix
    /// Set to be whatever a 3x3 matrix indicates
    pub fn of_matrix(matrix: &Vec<f64>) -> Self {
        assert!(matrix[8] == 1. && matrix[7] == 0. && matrix[6] == 0.);
        let dx = matrix[2];
        let dy = matrix[5];
        let skew = matrix[0] * matrix[1] + matrix[4] * matrix[3];
        assert!(skew > -1.0E-6 && skew < 1.0E-6);
        let sc2 = matrix[0] * matrix[4] - matrix[1] * matrix[3];
        assert!(sc2 >= -1.0E-9);
        let sc = {
            if sc2 < 0. {
                0.
            } else {
                sc2.sqrt()
            }
        };
        let angle = matrix[3].atan2(matrix[4]).to_degrees();

        Self::of_trs(Point::from_array([dx, dy]), angle, sc)
    }

    //mp is_identity
    /// Return true if this is an identity transform
    pub fn is_identity(&self) -> bool {
        self.rotation == 0. && self.scale == 1. && self.translation.is_zero()
    }

    //mp to_matrix
    /// Returns a 3x3 matrix that can be applied to points (x,y,1) or vectors (dx,dy,0)
    pub fn to_matrix(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(9);
        for _ in 0..9 {
            result.push(0.);
        }
        let sc = self.scale;
        let s = self.rotation.to_radians().sin();
        let c = self.rotation.to_radians().cos();
        let dx = self.translation[0];
        let dy = self.translation[1];
        // the result of three matrices
        // scale      sc  0  0;  0 sc  0;  0  0  1
        // rotate      c -s  0;  s  c  0;  0  0  1
        // translate   1  0 dx;  0  1 dy;  0  0  1
        // i.e.
        // R.S    =   c*sc -s*sc  0;  s*sc  c*sc  0;  0  0  1
        // T.R.S  =   c*sc -s*sc  dx;  s*sc  c*sc  dy;  0  0  1
        result[0] = sc * c;
        result[1] = -sc * s;
        result[2] = dx;
        result[3] = sc * s;
        result[4] = sc * c;
        result[5] = dy;
        result[8] = 1.;
        result
    }

    //mp apply
    /// Apply this transform to another transform, returning a new
    /// transform
    // The result will be a scaling of both multipled together, and a
    // rotation of both added together, plus a translation
    //
    // Note that matrix(other) = CS -SS DX; SS CS DY; 0 0 1
    // Note that matrix(self)  = cs -ss dx; ss cs dy; 0 0 1
    // Combine we get _ _ cs.DX-ss.DY+dx ; _ _ ss.DX+cs.DY+dy; 0 0 1
    // i.e. the resultant translation is:
    // self.rotate_scale(other.translate)+self.translate
    pub fn apply(&self, other: &Self) -> Self {
        let mut dxy = other.translation.clone();
        dxy.rotate_around(&Point::zero(), self.rotation, 0, 1);
        dxy = dxy * self.scale + self.translation;
        Self::of_trs(
            dxy,
            self.rotation + other.rotation,
            self.scale * other.scale,
        )
    }

    //zz All done
}

//ip std::fmt::Display for Transform
impl std::fmt::Display for Transform {
    //mp fmt - format a `Transform` for display
    /// Display the `TokenError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.translation.is_zero() && self.rotation == 0. && self.scale == 1. {
            write!(f, "<identity>")
        } else if self.rotation == 0. && self.scale == 1. {
            write!(f, "<+{}>", self.translation)
        } else {
            if !self.translation.is_zero() {
                write!(f, "<+{}>", self.translation)?
            };
            if self.rotation != 0. {
                write!(f, "<rot({})>", self.rotation)?
            };
            if self.scale != 1. {
                write!(f, "<*{}>", self.scale)?
            };
            Ok(())
        }
    }
}

//mt Test for Transform
#[cfg(test)]
mod tests {
    use super::*;
    fn approx_eq(a: f64, b: f64) -> bool {
        let diff = a - b;
        diff > -1.0E-6 && diff < 1.0E-6
    }
    fn check_transform(t: &Transform, dx: f64, dy: f64, r: f64, sc: f64) {
        assert!(
            approx_eq(t.translation[0], dx),
            "Transform {} dx of {} {} {} {}",
            t,
            dx,
            dy,
            r,
            sc
        );
        assert!(
            approx_eq(t.translation[1], dy),
            "Transform {} dy of {} {} {} {}",
            t,
            dx,
            dy,
            r,
            sc
        );
        assert!(
            approx_eq(t.rotation, r),
            "Transform {} r of {} {} {} {}",
            t,
            dx,
            dy,
            r,
            sc
        );
        assert!(
            approx_eq(t.scale, sc),
            "Transform {} sc of {} {} {} {}",
            t,
            dx,
            dy,
            r,
            sc
        );
    }
    fn check_matrix(m: &Vec<f64>, e: &Vec<f64>) {
        let okay = m
            .iter()
            .zip(e.iter())
            .fold(true, |acc, (m, e)| (acc && approx_eq(*m, *e)));
        assert!(okay, "Matrix {:?} expected {:?}", m, e);
    }
    #[test]
    fn test_0() {
        check_transform(&Transform::new(), 0., 0., 0., 1.);
        check_transform(&Transform::of_translation(Point::zero()), 0., 0., 0., 1.);
        check_transform(
            &Transform::of_translation(Point::from_array([1., 2.])),
            1.,
            2.,
            0.,
            1.,
        );
        check_transform(
            &Transform::of_trs(Point::from_array([3., -2.]), 7., 6.),
            3.,
            -2.,
            7.,
            6.,
        );
    }
    #[test]
    fn test_1() {
        let m = Transform::of_trs(Point::zero(), 0., 1.).to_matrix();
        assert_eq!(m, vec![1., 0., 0., 0., 1., 0., 0., 0., 1.]);
        let m = Transform::of_trs(Point::zero(), 0., 7.).to_matrix();
        assert_eq!(m, vec![7., 0., 0., 0., 7., 0., 0., 0., 1.]);
        let m = Transform::of_trs(Point::from_array([4., 5.]), 0., 7.).to_matrix();
        check_matrix(&m, &vec![7., 0., 4., 0., 7., 5., 0., 0., 1.]);
        let m = Transform::of_trs(Point::from_array([4., 5.]), 90., 7.).to_matrix();
        check_matrix(&m, &vec![0., -7., 4., 7., 0., 5., 0., 0., 1.]);
        let m = Transform::of_trs(Point::from_array([4., 5.]), 180., 7.).to_matrix();
        check_matrix(&m, &vec![-7., 0., 4., 0., -7., 5., 0., 0., 1.]);
        let m = Transform::of_trs(Point::from_array([4., 5.]), 270., 7.).to_matrix();
        check_matrix(&m, &vec![0., 7., 4., -7., 0., 5., 0., 0., 1.]);
    }
    #[test]
    fn test_2() {
        // Note matrix of 0. always produces a transform of 0.0., 0., 0.
        for (x, y) in vec![
            (0., 0.),
            (0., 1.),
            (1., 0.),
            (1., 1.),
            (-1., 0.),
            (-1., -1.),
        ] {
            for r in vec![0., 45., 90., 135.] {
                for s in vec![1., 5., 0.1] {
                    // cannot use 0.
                    let t = Transform::of_trs(Point::from_array([x, y]), r, s);
                    let m = t.to_matrix();
                    let t1 = Transform::of_matrix(&m);
                    check_transform(&t1, x, y, r, s);
                }
            }
        }
    }
}
