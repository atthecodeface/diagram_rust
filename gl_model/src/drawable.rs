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

@file    drawable.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use super::types::*;
use geometry::matrix;
use super::transformation::Transformation;
use super::bone::BoneSet;
use super::bone_pose::BonePoseSet;

//a Drawable
#[derive(Clone, Copy, Debug)]
pub enum Drawable {
    /// DrawElements
    Elements {
        /// Graphics primitive type that the element consists of, such as TRIANGLES
        gl_type     : gl::types::GLenum,
        /// Such as gl::UNSIGNED_BYTE
        index_type  : gl::types::GLenum,
        /// Number of indices to use (not number of graphics primitives - so for TRIANGLES this is at least 3)
        count       : i32,
        /// Offset in to indices buffer
        byte_offset  : usize,
    }
}

impl Drawable {
    pub fn new_elements( gl_type : gl::types::GLenum, index_type : gl::types::GLenum, count:i32, byte_offset:usize ) -> Self {
        Self::Elements { gl_type, index_type, count, byte_offset }
    }
    pub fn gl_draw(&self) {
        match self {
            Self::Elements{gl_type, count, index_type, byte_offset} => unsafe {
                gl::DrawElements( *gl_type,
                                  *count,
                                  *index_type,
                                  std::mem::transmute(*byte_offset) );

            }
        }
    }
}

//a Instance
//tp Instance
/// A drawable::Instance contains the instance data for an instance of a drawable::Instantiable
///
/// It requires a base transformation, an array of BonePose (which matches the Instantiable's BoneSet array), and an array of Mat4 for each bone in the BonePose array.
pub struct Instance<'a> {
    /// Reference to the Instantiable
    instantiable : &'a Instantiable,
    /// The transformation to apply to this model instance
    pub transformation : Transformation,
    /// Matrix for the transformation (must be updated after updating Transformation),
    pub trans_mat : Mat4,
    /// The sets of BonePose corresponding to the BoneSet array in the Instantiable
    pub bone_poses   : Vec<BonePoseSet<'a>>,
    /// Transformation matrices for the bones
    pub bone_matrices   : Vec<Mat4>,
}

//a Instantiable
//tp Instantiable
/// A drawable::Instantiable is a type that is related to a set of Mesh data, which can be instanced for different drawable::Instance's
///
/// It requires a related set of Mesh data that it does not refer to:
/// in object construction this Mesh data is likely to be the
/// structures containing vertex information and so on resident on a
/// CPU; in rendering the Mesh data is likely to be graphical objects
/// (such as OpenGL VAOs) that may reside (at least in part) on the
/// GPU.
///
/// The Instantiable data must be kept available to its related Instance.
///
/// The content of the Instantiable includes an array of BoneSets and
/// mesh transformation matrices, with appropriate index values. These
/// index values are into the related set of Mesh data.
struct X {set:BoneSet, bone_matrix_index:usize}
pub struct MeshIndexData {pub mesh_matrix_index:usize, pub bone_matrices:(usize, usize)}
pub struct Instantiable {
    /// The sets of bones, each of which will have a pose, and a corresponding first bone matrix
    bones   : Vec<X>,
    /// Transformation matrices for the meshes
    pub mesh_matrices   : Vec<Mat4>,
    /// An array indexed by the associated mesh data index value, and for each such index the content
    /// is an index in to this structure's mesh_matrices, and the range of bone matrices required by that associated mesh data.
    /// If the associated mesh data requires no bones then the tuple will be (0,0)
    mesh_data : Vec<MeshIndexData>,
    /// Number of bone matrices required for all the bone sets in this structure
    pub num_bone_matrices : usize,
}

//ip Instantiable
impl Instantiable {
    pub fn new() -> Self {
        let bones = Vec::new();
        let mesh_matrices = Vec::new();
        let mesh_data = Vec::new();
        let num_bone_matrices = 0;
        Self { bones, mesh_matrices, mesh_data, num_bone_matrices }
    }
    //mp add_mesh
    /// Add a mesh with an optional parent mesh_data index (and hence parent transformation) and bone_matrices
    pub fn add_mesh(&mut self, parent:Option<usize>, transformation:&Option<Mat4>, bone_matrices:(usize,usize)) -> usize {
        let mesh_matrix_index = {
            if let Some(parent) = parent {
                let parent = self.mesh_data[parent].mesh_matrix_index;
                if let Some(transformation) = transformation {
                    let n = self.mesh_matrices.len();
                    // let t = transformation.mat4();
                    let m = matrix::multiply::<f32,4,4,4>(&self.mesh_matrices[parent], transformation);
                    self.mesh_matrices.push(m);
                    n
                } else {
                    parent
                }
            } else if let Some(transformation) = transformation { // parent is none
                let n = self.mesh_matrices.len();
                // let t = transformation.mat4();
                let t = transformation.clone();
                self.mesh_matrices.push(t);
                n
            } else { // both are none - requires an identity matrix
                let n = self.mesh_matrices.len();
                self.mesh_matrices.push(matrix::identity::<f32,4>());
                n
            }
        };
        let n = self.mesh_data.len();
        self.mesh_data.push( MeshIndexData {mesh_matrix_index, bone_matrices} );
        n
    }
    //mp add_bone_set
    /// Add a bone set; clones it, and generates a number of bone matrices and updates appropriately, returning the range of bone matrices that the set corresponds to
    pub fn add_bone_set(&mut self, bone_set:&BoneSet) -> (usize, usize) {
        (0,0)
    }
    //mp borrow_mesh_data
    pub fn borrow_mesh_data<'a> (&self) -> &'a
    //mp instantiate
    /// Create an `Instance` from this instantiable - must be used with accompanying mesh data in the appropriate form for the client
    pub fn instantiate<'a>(&'a self) -> Instance<'a> {
        let transformation = Transformation::new();
        let trans_mat = [0.;16];
        let mut bone_poses = Vec::new();
        let mut bone_matrices = Vec::with_capacity(self.num_bone_matrices);
        for _ in 0..self.num_bone_matrices {
            bone_matrices.push([0.;16]);
        }
        Instance { instantiable:self, transformation, trans_mat, bone_poses, bone_matrices }
    }
}


//tp ShaderDrawableSet
/// An array of primitives that can be drawn by a single shader
///
/// Prior to invoking gl_draw() on this the shader program must be configured and uniforms set up
///
/// gl_draw will bind vaos and draw elements
///
/// Maybe have an index in to a mesh transformation [Mat4] and an index range for [Mat4] for BonesMatrices for each drawable
pub struct ShaderDrawableSet {
    vaos      : Vec<Vao>,
    drawables : Vec<(Vao,Vec<Drawable>)>,
}
impl ShaderDrawableSet {
    pub fn new() -> Self {
        let vaos      = Vec::new();
        let drawables = Vec::new();
        Self { vaos, drawables }
    }
    pub fn add_drawables(&mut self, vao:Vao, drawables:&[Drawable]) {
        if !self.vaos.contains(&vao) { self.vaos.push(vao); }
        let drawables = Vec::from(drawables);
        self.drawables.push( (vao, drawables ) );
    }
    pub fn gl_draw(&self) {
        let mut first = true;
        let mut last_vao = 0;
        for (vao, drawables) in &self.drawables {
            if first || (last_vao != *vao) {
                unsafe { gl::BindVertexArray(*vao) };
                first = false;
                last_vao = *vao;
            }
            // self.material.gl_program_configure(program)
            for d in drawables {
                d.gl_draw();
            }
        }
    }
}
//ip Drop for ShaderDrawableSet
impl Drop for ShaderDrawableSet {
    //fp drop
    /// If any OpenGL VAOs have been created for this then delete them
    fn drop(&mut self) {
        for vao in &self.vaos {
            unsafe { gl::DeleteVertexArrays(1, vao ); }
        }
        self.vaos.clear();
    }
}

