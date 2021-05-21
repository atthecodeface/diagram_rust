use types::*;
use geometry::{vector, quat, matrix};

#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    translation   : Vec3,
    scale         : Vec3,
    rotation      : Quat,
}
impl Transformation {
    pub fn new() -> Self {
        let translation = vector::origin();
        let scale       = [1.;3];
        let rotation    = quat::new();
        Self {translation, scale, rotation}
    }
    pub fn set_scale(mut self, scale:Vec3) -> Self {
        self.scale = scale;
        self
    }
    pub fn set_translation(mut self, translation:Vec3) -> Self {
        self.translation = translation;
        self
    }
    pub fn set_rotation(mut self, rotation:Quat) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn copy(&mut self, other:&Self) {
        self.translation = other.translation.clone();
        self.scale = other.scale.clone();
        self.rotation = other.rotation.clone();
    }
    pub fn combine(&mut self, base:&Self, other:&Self) {
        self.rotation    = quat::multiply( &base.rotation, &other.rotation );
        self.translation = base.translation.clone();
        self.translation = vector::add( self.translation, &other.translation, 1. );
        for i in 0..3 {
            self.scale[i] = base.scale[i] * other.scale[i];
        }
    }
    pub fn translate(&mut self, translation:&Vec3, scale:f32) {
        self.translation = vector::add( self.translation, translation, scale );
    }
    pub fn rotate(&mut self, axis:&Vec3, angle:f32) {
        let q = quat::of_axis_angle(axis, angle);
        self.rotation = quat::multiply( &q, &self.rotation);
        // Glm.quat.multiply(self.translation, q, self.translation)
        // # self.translation = q * self.translation # type: ignore
    }
    pub fn mat4(&self) -> Mat4 {
        let mut m = matrix::from_quat4(self.rotation);
        for i in 0..3 {
            m[4*i+0] *= self.scale[i];
            m[4*i+1] *= self.scale[i];
            m[4*i+2] *= self.scale[i];
        }
        m[12] += self.translation[0];
        m[13] += self.translation[1];
        m[14] += self.translation[2];
        m
    }
    pub fn mat4_inverse(&self) -> Mat4 {
        let r = quat::conjugate(&self.rotation);
        let mut m = matrix::from_quat4(r);
        for i in 0..3 {
            m[i+0] /= self.scale[i];
            m[i+4] /= self.scale[i];
            m[i+8] /= self.scale[i];
        }
        m[12] -= self.translation[0];
        m[13] -= self.translation[1];
        m[14] -= self.translation[2];
        m
    }
    pub fn from_mat4(&mut self, m:Mat4) {
        self.translation = [m[12], m[13], m[14]];
        let mut rotation = [0.;9];
        for i in 0..3 {
            let v = [m[4*i+0],m[4*i+1],m[4*i+2]];
            let l = vector::len(&v);
            self.scale[i] = l;
            rotation[3*i+0] = v[0] / l;
            rotation[3*i+1] = v[1] / l;
            rotation[3*i+2] = v[2] / l;
        }
        self.rotation = quat::of_rotation(&rotation);
    }
    pub fn mat4_after(&self, pre_mat:&Mat4) -> Mat4 {
        let m = self.mat4();
        matrix::multiply::<f32,4,4,4>(pre_mat, &m)
    }
    pub fn lerp(&mut self, t:f32, in0:&Self, in1:&Self) {
        let tn = 1.0 - t;
        for i in 0..3 {
            self.translation[i] = t*in0.translation[i] + tn*in1.translation[i];
            self.scale[i]       = t*in0.scale[i]       + tn*in1.scale[i];
        }
        self.rotation = quat::nlerp(t, &in0.rotation, &in1.rotation);
    }
    pub fn distance(&self, other:&Self) -> f32 {
        let td = vector::distance_to(&self.translation, &other.translation);
        let sd = vector::distance_to(&self.scale,       &other.scale);
        let qd = quat::distance_to(  &self.rotation,    &other.rotation);
        return td+sd+qd;
    }
    //zz All done
}

//ip Distance for Transformation
impl std::fmt::Display for Transformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}:{:?}:{:?}",self.translation,self.rotation,self.scale)
    }
}

