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

@file    shader.rs
@brief   Part of OpenGL support library
 */

//a Documentation

/*!

This library provides types for shader-specific objects.

An object instance that may be drawn by a shader is called a ShaderDrawable. This is an instance of a ShaderInstantiable. A ShaderInstantiable is a shader-specific derivation of an ObjectInstantiable.

An ObjectInstantiableData consists of an array of BoneSets, an array
of [Mat4] for the object's Meshes, and an array of Mesh that the
object consists of. The ObjectInstantiableData is derived from an
Object - which in turn is Hierarchy of ObjectNodes, each of the which
may have a BoneSet, a transformation (relative to its parent) and a
mesh. The ObjectInstantiableData is, in effect, a flattened set of
ObjectNode hierarchies. In flattening the nodes BoneSets are cloned in
to a linear array of BoneSets; each node's mesh has a relative-to-root
transformation matrix generated and placed in a linear array, and the
index in to

!*/

//a Imports
use crate::buffer;
use super::types::*;
use super::object;
use super::drawable;

//a ShaderClass
pub trait ShaderClass {
    fn get_attr(&self, key:&str) -> Option<gl::types::GLuint>;
    fn get_uniform(&self, key:&str) -> Option<gl::types::GLuint>;
    fn gl_bind_attr_view(&self, key:&str, view:Option<&buffer::View>) {
        if let Some(attr) = self.get_attr(key) {
            match view {
                None       => {unsafe{gl::DisableVertexAttribArray(attr);}},
                Some(view) => {view.gl_bind_attribute(attr); },
            }
        }
    }
}

//tp Instantiable
/// An array of primitives that can be drawn by a single shader, if provided with an array of matrices for bone poses and a base model matrix
///
/// Prior to invoking gl_draw() on this the shader program must be configured and uniforms set up for ModelMatrix
///
/// gl_draw will bind vaos, bind uMeshMatrix, bind uBonesMatrices, and draw elements
pub struct Instantiable<'a> {
    /// Arrays of BoneSets and arrays of mesh matrices
    instantiable : &'a drawable::Instantiable,
    /// VAOs of the meshes used in all the Drawables
    vaos      : Vec<Vao>,
    /// Array of (VAOs, drawables), the index of which ties in with instantiable.mesh_data, so that
    /// this in effect becomes an array of Vao, Drawable, mesh_matrix index, bone set matrix range
    drawables : Vec<(Vao,usize,Vec<drawable::Drawable>)>,
}
impl <'a> Instantiable<'a> {
    pub fn new(instantiable:&'a drawable::Instantiable) -> Self {
        let vaos      = Vec::new();
        let drawables = Vec::new();
        Self { instantiable, vaos, drawables }
    }
    pub fn add_drawables(&mut self, vao:Vao, drawables:&[drawable::Drawable]) {
        if !self.vaos.contains(&vao) { self.vaos.push(vao); }
        let drawables = Vec::from(drawables);
        self.drawables.push( (vao, 0, drawables ) );
    }
    pub fn gl_draw(&self, instance:&drawable::Instance) {
        let mut first = true;
        let mut last_vao = 0;
        let mut last_mesh_mat_index = 0;
        let mut last_mesh_bone_matrices = (0,0);
        for i in 0..self.drawables.len() {
            let (vao, mesh_index, drawables) = &self.drawables[i];
            if first || (last_vao != *vao) {
                unsafe { gl::BindVertexArray(*vao) };
                last_vao = *vao;
            }
            let mesh_data = self.instantiable.borrow_mesh_data(mesh_index);
            if first || (mesh_data.mesh_matrix_index != last_mesh_mat_index) {
                // set mesh matrix uniform if shader needs it
                last_mesh_mat_index = mesh_data.mesh_matrix_index;
            }
            if first || (mesh_data.bone_matrices != last_mesh_bone_matrices) {
                // set bone matrices / bone scale uniform if shader needs it
                last_mesh_bone_matrices = mesh_data.bone_matrices;
            }
            // self.material.gl_program_configure(program)
            for d in drawables {
                d.gl_draw();
            }
            first = false;
        }
    }
}
//ip Drop for Instantiable<'a>
impl <'a> Drop for Instantiable<'a> {
    //fp drop
    /// If any OpenGL VAOs have been created for this then delete them
    fn drop(&mut self) {
        for vao in &self.vaos {
            unsafe { gl::DeleteVertexArrays(1, vao ); }
        }
        self.vaos.clear();
    }
}

