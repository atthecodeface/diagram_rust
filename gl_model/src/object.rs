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
use super::types::*;
use crate::shader::ShaderClass;
use crate::primitive::Primitive;
use super::drawable::{ShaderDrawableSet};
use super::bone::BoneSet;
use super::mesh::Mesh;
use super::transformation::Transformation;
use super::hierarchy::{Hierarchy, Recipe};

pub struct ObjectNode<'a>
{
    transformation : Option<Transformation>,
    bones          : Option<&'a BoneSet>,
    mesh           : Option<&'a Mesh<'a>>,
}

impl <'a> ObjectNode <'a> {
    pub fn new() -> Self {
        let transformation = None;
        let bones = None;
        let mesh = None;
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
    /*
    pub fn add_meshes_of_recipe(&self, &mut meshes:Vec<usize>, &mut drawable:drawable::Instantiable, mut iter:NodeIter<ObjectNode>) {
        let mut parent = None;
        let mut transformation = None;
        let mut mesh_stack = Vec::new();
        for op in iter {
            match op {
                Push((node, has_children)) => {
                    if self.node
                },
                _ => { // Pop
                },
            }
        }
    }
    pub fn create_instantiable(&mut self) -> drawable::Instantiable {
        let drawable = drawable::Instantiable::new();
        let mut meshes = Vec::new();
        for r in self.roots() {
            self.add_meshes_of_recipe(&mut meshes, self.nodes.iter_from_root(r));
        }
        self.meshes = meshes;
    }
*/
}

