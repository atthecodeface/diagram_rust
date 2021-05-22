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

@file    object.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use geometry::matrix;
use crate::shader::ShaderClass;
use super::drawable;
use super::bone::BoneSet;
use super::mesh::Mesh;
use super::transformation::Transformation;
use super::hierarchy::{Hierarchy, Recipe, NodeIterOp, NodeIter};
use super::shader;

pub struct ObjectNode<'a>
{
    transformation : Option<Transformation>,
    bones          : Option<&'a BoneSet>,
    mesh           : Option<&'a Mesh<'a>>,
}

impl <'a> ObjectNode <'a> {
    pub fn new( transformation:Option<Transformation>,
                mesh:Option<&'a Mesh<'a>>,
                bones:Option<&'a BoneSet> ) -> Self {
        Self { transformation, bones, mesh }
    }
}

//a Object
/// A hierarchy of ObjectNode's
///
/// This can be flattened in to an Instantiable
pub struct Object<'a> {
    /// The meshes etc that make up the object
    pub nodes   : Hierarchy<ObjectNode<'a>>,
    /// The roots of the bones and hierarchical recipes for traversal
    pub roots   : Vec<(usize, Recipe)>,
    /// Meshes - indices in to nodes.nodes array of the meshes in the order of instance
    pub meshes : Vec<usize>
}

impl <'a> Object<'a> {
    pub fn new() -> Self {
        let nodes = Hierarchy::new();
        let roots = Vec::new();
        let meshes = Vec::new();
        Self { nodes, roots, meshes }
    }
    pub fn add_node(&mut self,
                    transformation:Option<Transformation>,
                    mesh:Option<&'a Mesh<'a>>,
                    bones:Option<&'a BoneSet> ) -> usize {
        let node = ObjectNode::new(transformation, mesh, bones);
        self.nodes.add_node(node)
    }

    pub fn relate(&mut self, parent:usize, child:usize) {
        self.nodes.relate( parent, child);
    }

    pub fn add_meshes_of_node_iter(&self, meshes:&mut Vec<usize>, drawable:&mut drawable::Instantiable, mut iter:NodeIter<ObjectNode>) {
        let mut parent = None;
        let mut transformation = None;
        let mut bone_matrices = (0,0);
        let mut mesh_stack = Vec::new();
        for op in iter {
            match op {
                NodeIterOp::Push((n,obj_node), has_children) => {
                    mesh_stack.push((parent, transformation, bone_matrices));
                    if let Some(bone_set) = obj_node.bones {
                        bone_matrices = drawable.add_bone_set(bone_set);
                    }
                    if let Some(obj_transformation) = &obj_node.transformation {
                        if transformation.is_none() {
                            transformation = Some(obj_transformation.mat4());
                        } else {
                            transformation = Some(matrix::multiply::<f32,4,4,4>(&transformation.unwrap(), &obj_transformation.mat4()));
                        }
                    }
                    if let Some(mesh) = obj_node.mesh {
                        let index = drawable.add_mesh(&parent, &transformation, &bone_matrices);
                        assert_eq!(index, meshes.len());
                        meshes.push(n);
                        parent = Some(index);
                        transformation = None;
                    }
                },
                NodeIterOp::Pop(_,_) => {
                    let ptb = mesh_stack.pop().unwrap();
                    parent = ptb.0;
                    transformation = ptb.1;
                    bone_matrices = ptb.2;
                },
            }
        }
    }

    pub fn create_instantiable(&mut self) -> drawable::Instantiable {
        self.nodes.find_roots();
        let mut drawable = drawable::Instantiable::new();
        let mut meshes = Vec::new();
        for r in self.nodes.borrow_roots() {
            self.add_meshes_of_node_iter(&mut meshes, &mut drawable, self.nodes.iter_from_root(*r));
        }
        self.meshes = meshes;
        drawable
    }
    pub fn bind_shader<'b, S:ShaderClass>(&self, drawable:&'b drawable::Instantiable, shader:&S) -> shader::Instantiable<'b> {
        let mut s = shader::Instantiable::new(drawable);
        for i in 0..self.meshes.len() {
            let obj_node = self.nodes.borrow_node(self.meshes[i]);
            assert!(obj_node.mesh.is_some(), "Mesh at node must be Some() for it to have been added to the self.meshes array by add_meshes_of_node_iter");
            let mesh = obj_node.mesh.unwrap();
            mesh.add_shader_drawables(shader, &mut s);
        }
        s
    }
}

