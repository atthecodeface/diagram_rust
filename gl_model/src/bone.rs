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

@file    bone.rs
@brief   Bone and bone hierarchy
 */

//a Imports
use geometry::{matrix};
use super::hierarchy;
use super::types::*;
use super::transformation::Transformation;

//a Bone
/// Each bone has a transformation with respect to its parent that is
/// a translation (its origin relative to its parent origin), scale
/// (in each direction, although a common scale for each coordinates
/// is best), and an orientation of its contents provided by a
/// quaternion (as a rotation).
///
/// A point in this bone's space is then translate(rotate(scale(pt)))
/// in its parent's space. The bone's children start with this
/// transformation too.
///
/// From this the bone has a local bone-to-parent transform matrix
/// and it has a local parent-to-bone transform matrix
///
/// At rest (where a mesh is skinned) there are two rest matrix variants
/// Hence bone_relative = ptb * parent_relative
///
/// The skinned mesh has points that are parent relative, so
/// animated_parent_relative(t) = btp(t) * ptb * parent_relative(skinned)
///
/// For a chain of bones Root -> A -> B -> C:
///  bone_relative = C.ptb * B.ptb * A.ptb * mesh
///  root = A.btp * B.btp * C.btp * C_bone_relative
///  animated(t) = A.btp(t) * B.btp(t) * C.btp(t) * C.ptb * B.ptb * A.ptb * mesh
pub struct Bone {
    /// rest transform - translation, scale, rotation
    pub transformation : Transformation,
    /// The parent-to-bone mapping Matrix at rest; updated when the
    /// transformation is changed
    pub(crate) ptb            : Mat4,
    /// The mesh-to-bone mapping Matrix at rest, derived from the
    /// hierarchy root; updated when any transformation is changed in
    /// the hierarchy at this bone or above
    pub(crate) mtb            : Mat4,
    ///  Index into matrix array to put this bones animated mtm
    pub matrix_index   : usize,
}

//ip Bone
impl Bone {
    //fp new
    /// Create a new bone with a given rest
    pub fn new(transformation:Transformation, matrix_index:usize) -> Self {
        let ptb = [0.; 16];
        let mtb = [0.; 16];
        Self { transformation, matrix_index, ptb, mtb }
    }
    pub fn borrow_transformation<'a> (&'a self) -> &'a Transformation {
        &self.transformation
    }
    pub fn set_transformation(mut self, transformation:Transformation) -> Self {
        self.transformation = transformation;
        self
    }
    pub fn derive_matrices(&mut self, is_root:bool, parent_mtb:&Mat4) -> &Mat4{
        self.ptb = self.transformation.mat4_inverse();
        if is_root {
            self.mtb = self.ptb;
        } else {
            self.mtb = matrix::multiply::<f32,4,4,4>(&self.ptb, parent_mtb);
        }
        &self.mtb
    }
}

/*
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"Bone {self.matrix_index}")
        hier.push()
        hier.add(f"{self.transformation}")
        if hasattr(self, "ptb"): hier.add(f"parent-to-bone: {self.ptb}")
        if hasattr(self, "mtb"): hier.add(f"mesh-to-bone  : {self.mtb}")
        for c in self.children:
            c.hier_debug(hier)
            pass
        hier.pop()
        return hier
    #f __str__
    def __str__(self) -> str:
        return str(self.hier_debug(Hierarchy()))
    #f All done
    pass
 */

//a BoneSet
/// A set of related bones, with one or more roots
///
/// This corresponds to a skeleton (or a number thereof), with each
/// bone appearing once in each skeleton. The bones form a hierarchy.
pub struct BoneSet {
    /// The bones that make up the set, with the hierarchical relationships
    pub bones   : hierarchy::Hierarchy<Bone>,
    /// The roots of the bones and hierarchical recipes for traversal
    pub roots   : Vec<(usize, hierarchy::Recipe)>,
    /// An array of matrices long enough for the one per level of traversal
    pub temp_mat4s: Vec<Mat4>,
    /// Max bone index
    pub max_index : usize,
}
impl BoneSet {
    pub fn new() -> Self {
        let bones  = hierarchy::Hierarchy::new();
        let roots  = Vec::new();
        let temp_mat4s = Vec::new();
        Self { bones, roots, temp_mat4s, max_index:0 }
    }

    pub fn add_bone(&mut self, transformation:Transformation, matrix_index:usize) -> usize {
        self.roots.clear();
        let bone = Bone::new(transformation, matrix_index);
        self.bones.add_node(bone)
    }

    pub fn relate(&mut self, parent:usize, child:usize) {
        self.bones.relate( parent, child);
    }

    pub fn find_max_index(&mut self) {
        let mut max_index = 0;
        for b in self.bones.borrow_elements() {
            if b.data.matrix_index >= max_index {
                max_index = b.data.matrix_index + 1
            }
        }
        self.max_index = max_index;
    }
    pub fn resolve(&mut self) {
        if self.roots.len() == 0 {
            self.bones.find_roots();
            for r in self.bones.borrow_roots() {
                self.roots.push( (*r, hierarchy::Recipe::of_ops(self.bones.enum_from_root(*r))) );
            }
            let mut max_depth = 0;
            for (_, recipe) in &self.roots {
                max_depth = if recipe.depth() > max_depth { recipe.depth() } else { max_depth };
            }
            self.temp_mat4s = Vec::new();
            for i in 0..max_depth {
                self.temp_mat4s.push([0.;16]);
            }
            self.find_max_index();
        }
    }
    pub fn rewrite_indices(&mut self) {
        self.resolve();
        if self.max_index < self.bones.len()  {
            let mut bone_count = 0;
            let (_, bones) = self.bones.borrow_mut();
            for (_,recipe) in &self.roots {
                for op in recipe.borrow_ops() {
                    match op {
                        hierarchy::NodeIterOp::Push(n,_) => {
                            bones[*n].data.matrix_index = bone_count;
                            bone_count += 1;
                        },
                        _ => {},
                    }
                }
            }
            self.max_index = bone_count;
        }
    }
    pub fn derive_matrices(&mut self) {
        assert!(self.roots.len() != 0, "Resolve MUST have been invoked prior to derive_matrices");
        let (_, bones) = self.bones.borrow_mut();
        let mut mat_depth = 0;
        for (_, recipe) in &self.roots {
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeIterOp::Push(n,_) => {
                        if mat_depth == 0 {
                            self.temp_mat4s[mat_depth]   = *bones[*n].data.derive_matrices(true, &self.temp_mat4s[mat_depth]);
                        } else {
                            self.temp_mat4s[mat_depth+1] = *bones[*n].data.derive_matrices(false, &self.temp_mat4s[mat_depth]);
                        }
                        mat_depth += 1;
                    },
                    _ => {
                        mat_depth -= 1;
                    }
                }
            }
        }
    }
    pub fn iter_roots<'z> (&'z self) -> impl Iterator<Item=usize> + '_ {
        self.roots.iter().map(|(n,_)| *n)
    }
}
    /*
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"BoneSet {self.roots}")
        hier.push()
        for b in self.iter_roots():
            b.hier_debug(hier)
            pass
        hier.pop()
        return hier
    #f All done
    pass
*/
