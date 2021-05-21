///  A ModelInstance is an instance of a ModelClass
///  It has a bone pose hierarchy and a model transformation.
///  It should have any texture and color overrides too.
///
///  The model transform places it appropriately in world space
///  Each model object instance inside the model has a transformation relative to that
///  In addition, the model object instances may have a bone pose hierarchy
pub struct Instance<'a> {
    bones           : Vec<&'a Bones>,
    bone_set_poses  : Vec<BonePoseSet<'a>>,
    /// Transformation for the mesh, the mesh, and an index to bone_set_poses for its bone matrices, if required
    meshes          : Vec<(TransMat, Mesh<'a>, usize)>,
}
impl Instance {
    pub fn new(class?) -> Self {
        self.bone_set_poses = []
        self.meshes = []
        self.transformation = Transformation();
        bone_set_dict = {}
        for (trans_mat,model) in model_class.iter_objects():
            if not model.has_mesh(): continue
            mesh_instance = model.get_mesh()
            bone_set_index = -1
            if model.has_bones():
                bone_set = model.get_bones() # get bone set
                if bone_set not in bone_set_dict:
                    bone_set_dict[bone_set] = len(self.bone_set_poses)
                    pose = BonePoseSet(bone_set)
                    self.bone_set_poses.append(pose)
                    pass
                bone_set_index = bone_set_dict[bone_set]
                pass
            self.meshes.append( (trans_mat, mesh_instance, bone_set_index) )
            pass
        pass
    #f gl_create
    def gl_create(self) -> None:
        for (t,m,b) in self.meshes:
            m.gl_create()
            pass
        pass
    #f gl_bind_program
    def gl_bind_program(self, shader_class:ShaderClass) -> None:
        for (t,m,b) in self.meshes:
            m.gl_bind_program(shader_class)
            pass
        pass
    #f gl_draw
    def gl_draw(self, program:ShaderProgram, tick:int) -> None:
        mat = self.transformation.mat4()
        GL.glUniformMatrix4fv(program.uniforms["uModelMatrix"], 1, False, mat)
        for bone_set_pose in self.bone_set_poses:
            bone_set_pose.update(tick)
            pass
        for (t,m,b) in self.meshes:
            if b>=0:
                bma = self.bone_set_poses[b]
                program.set_uniform_if("uBonesMatrices",
                                      lambda u:GL.glUniformMatrix4fv(u, bma.max_index, False, bma.data))
                program.set_uniform_if("uBonesScale",
                                       lambda u: GL.glUniform1f(u, 1.0) )
                pass
            else:
                program.set_uniform_if("uBonesScale",
                                       lambda u: GL.glUniform1f(u, 0.0) )
                pass
            # Provide mesh matrix and material uniforms
            program.set_uniform_if("uMeshMatrix",
                                   lambda u: GL.glUniformMatrix4fv(u, 1, False, t.mat4()) )
            m.gl_draw(program)
            pass
        pass
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"ModelInstance with {len(self.bone_set_poses)} poses")
        hier.push()
        for i in range(len(self.bone_set_poses)):
            hier.add(f"Pose/Matrix {i}")
            self.bone_set_poses[i].hier_debug(hier)
            pass
        for (t,m,b) in self.meshes:
            hier.add(f"Mesh transform {t} pose/matrix {b}")
            m.hier_debug(hier)
            pass
        hier.pop()
        return hier
    #f __str__
    def __str__(self) -> str:
        return str(self.hier_debug(Hierarchy()))
    #f All done
    pass
