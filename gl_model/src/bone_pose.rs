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
use super::bone::{Bone, BoneSet};

//a Pose
pub struct Pose<'a> {
    bone             : &'a Bone,
    // relative to bone rest
    transformation   : Transformation,
    btp              : Mat4,
    // ptb              : Mat4,
    animated_btm     : Mat4,
    animated_mtm     : Mat4,
}
impl <'a> Pose <'a> {
    pub fn new(bone:&'a Bone) -> Self {
        let transformation = bone.borrow_transformation().clone();
        // self.ptb = [0.; 16]
        let btp = [0.; 16];
        let animated_btm  = [0.; 16];
        let animated_mtm = [0.; 16];
        Self { bone, transformation, btp, animated_btm, animated_mtm }
    }
    pub fn transformation_reset(&mut self) {
        self.transformation = *self.bone.borrow_transformation();
    }
    pub fn set_transformation(&mut self, transform:Transformation) {
        self.transformation = transform;
        self.btp = self.transformation.mat4();
    }
    pub fn derive_animation(&mut self, is_root:bool, parent_animated_btm:&Mat4) -> &Mat4 {
        if is_root {
            self.animated_btm = self.btp;
        } else {
            self.animated_btm = matrix::multiply4(parent_animated_btm, &self.btp);
        }
        self.animated_mtm = matrix::multiply4(&self.animated_btm, &self.bone.mtb);
        &self.animated_btm
    }
}
    /*
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"Pose {self.bone.matrix_index}")
        hier.push()
        hier.add(f"{self.transformation}")
        hier.add(f"parent-to-bone: {self.ptb}")
        hier.add(f"bone-to-parent: {self.btp}")
        hier.add(f"bone-to-mesh  : {self.animated_btm}")
        hier.add(f"mesh-to-mesh  : {self.animated_mtm}")
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

pub struct BonePoseSet<'a> {
    bones        : &'a BoneSet,
    poses        : Vec<Pose<'a>>,
    data         : Vec<Mat4>,
    last_updated : usize,
}
impl <'a> BonePoseSet<'a> {
    pub fn new(bones:&'a BoneSet) -> Self {
        let mut poses = Vec::new();
        for b in bones.bones.borrow_elements().iter() {
            poses.push( Pose::new(&b.data) );
        }
        let mut data = Vec::new();
        for _ in 0..bones.max_index {
            data.push([0.;16]);
        }
        let last_updated = 0;
        Self { bones, poses, data, last_updated }
    }
    pub fn derive_animation(&mut self) {
        let mut mat_depth = 0;
        for (_, recipe) in &self.bones.roots {
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeIterOp::Push(n,_) => {
                        if mat_depth == 0 {
                            self.data[mat_depth]   = *self.poses[*n].derive_animation(true, &self.data[mat_depth]);
                        } else {
                            self.data[mat_depth+1] = *self.poses[*n].derive_animation(false, &self.data[mat_depth]);
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
    pub fn update(&mut self, tick:usize) {
        if tick != self.last_updated {
            self.last_updated = tick;
            self.derive_animation();
            let bones = self.bones.bones.borrow_elements();
            for i in 0..self.poses.len() {
                let matrix_index = bones[i].data.matrix_index;
                self.data[matrix_index] = self.poses[i].animated_mtm;
            }
        }
    }
}
/*
        pass
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"BonePoseSet {self.bones.roots} {self.max_index} {self.last_updated} {self.data}")
        hier.push()
        self.bones.hier_debug(hier)
        for pose in self.poses:
            pose.hier_debug(hier)
            pass
        hier.pop()
        return hier
    #f All done
    pass
 */

        /*
#c AnimatedBonePose
class AnimatedBonePose:
    def __init__(self, poses:List[BonePose]) -> None:
        self.poses = poses
        self.animatable = Bezier2(Transformation())
        self.animatable.set_target( t1=1.,
                                    c0=Transformation( quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    c1=Transformation( quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    tgt=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    callback=self.animation_callback )
        pass
    def interpolate_to_time(self, t:float) -> None:
        z = self.animatable.interpolate_to_time(t)
        # print(t, z)
        self.poses[1].transformation_reset()
        self.poses[1].transform(z)
        pass
    def animation_callback(self, t:float) -> None:
        t_sec = math.floor(t)
        t_int = int(t_sec)
        tgt = 1.0
        if (t_int&1): tgt=-1.
        self.animatable.set_target( t1=t_sec+1.,
                                    c0=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), 0.3)),
                                    c1=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(0.,1.,0.), 0.5)),
                                    tgt=Transformation(quaternion=Glm.quat.setAxisAngle(Glm.quat.create(), Glm.vec3.fromValues(1.,0.,0.), tgt*0.3)),
                                    callback=self.animation_callback )
        pass
    pass

*/
