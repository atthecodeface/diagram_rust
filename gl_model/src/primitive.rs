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

@file    primitive.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use super::drawable::Drawable;
use super::shader;
use buffer::Data   as BufferData;
use buffer::View   as BufferView;
use crate::shader::ShaderClass;

//a Vertices
//tp Vertices
/// A primitive Vertices - independent of the material (e.g. uniforms)
/// required, this is a set of vertex attributes and a vertex index buffer
///
/// For a particular Shader the Vertices will have an OpenGL VAO (vertex
/// attribute object) associated with it; this VAO depends on the
/// attribute numbering of the shader.
///
/// A subset of this view is used for drawing a primitive - so this
/// may be the complete set of vertices, for example, of a robot.
pub struct Vertices<'a> {
    indices    : &'a BufferData<'a>,
    position   : &'a BufferView<'a>,
    normal     : Option<&'a BufferView<'a>>,
    tex_coords : Option<&'a BufferView<'a>>,
    joints     : Option<&'a BufferView<'a>>,
    weights    : Option<&'a BufferView<'a>>,
    tangent    : Option<&'a BufferView<'a>>,
    color      : Option<&'a BufferView<'a>>,
}

//ip Vertices
impl <'a>  Vertices<'a> {
    pub fn new(indices: &'a BufferData<'a>, position:&'a BufferView<'a>) -> Self {
        Self { indices, position,
               normal: None,
               tex_coords : None,
               joints : None,
               weights : None,
               tangent : None,
               color : None,
        }
    }

    //mp gl_create
    /// Create the underlying buffers
    pub fn gl_create(&self) {
        unsafe {
            // stops the indices messing up other VAO
            gl::BindVertexArray(0);
            self.indices.gl_create_indices();
            self.position.gl_create();
            self.normal.map(|x| x.gl_create());
            self.tex_coords.map(|x| x.gl_create());
            self.joints.map(|x| x.gl_create());
            self.weights.map(|x| x.gl_create());
            self.tangent.map(|x| x.gl_create());
            self.color.map(|x| x.gl_create());
        }
    }

    //mp gl_bind_to_shader
    /// Create the VAO, if that has not already been done
    pub fn gl_bind_to_shader <S:ShaderClass>(&self, shader:&S) -> gl::types::GLuint {
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
            self.indices.gl_bind_indices();
            shader.gl_bind_attr_view("vPosition", Some(self.position));
            shader.gl_bind_attr_view("vNormal",    self.normal);
            shader.gl_bind_attr_view("vTexCoords", self.tex_coords);
            shader.gl_bind_attr_view("vJoints",    self.joints);
            shader.gl_bind_attr_view("vWeights",   self.weights);
            shader.gl_bind_attr_view("vTangent",   self.tangent);
            shader.gl_bind_attr_view("vColor",     self.color);
            gl::BindVertexArray(0);
        }
        gl_vao
    }
/*    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        self.indices.hier_debug(hier)
        for (san,an) in self.attribute_mapping.items():
            if hasattr(self, an):
                mbv = getattr(self,an)
                if mbv is not None: mbv.hier_debug(hier, an)
                pass
            pass
        return hier
    #f All done
    pass
     */
}

//a Primitive
//tp Primitive
/// A primitive consisting of a material and a subset of primitive::Vertices using a particular range of indices
///
/// This might be, for example, the arm of a robot.
pub struct Primitive<'a> {
    /// Name - not required, but provided sometimes in GLTF - can be useful for debug
    name : String,
    /// Material to be used in drawing
    // material : Material,
    /// Primitive vertices
    vertices        : &'a Vertices<'a>,
    elements        : Vec<Drawable>,
}

//ip Primitive
impl <'a> Primitive<'a> {
    //fp new
    /// Create a new Primitive from a Vertices
    pub fn new(name:&str, vertices:&'a Vertices<'a> ) -> Self {
        let name = String::from(name);
        Self { name, vertices, elements:Vec::new() }
    }
    pub fn add_element(&mut self, e:Drawable) {
        self.elements.push(e);
    }
    pub fn add_shader_drawables<'z, S:ShaderClass>(&self, shader:&S, instantiable:&'z mut shader::Instantiable) {
        self.vertices.gl_create();
        let vao = self.vertices.gl_bind_to_shader(shader);
        instantiable.add_drawables(vao, &self.elements);
    }
}

/*
    #f gl_bind_program
    def gl_bind_program(self, shader_class:ShaderClass) -> None:
        self.view.gl_bind_program(shader_class)
        pass
    #f gl_draw
        pass
    #f hier_debug
    def hier_debug(self, hier:Hierarchy) -> Hierarchy:
        hier.add(f"Primitive '{self.name}' {self.gl_type} {self.indices_gl_type} {self.indices_count} {self.indices_offset}")
        hier.push()
        self.view.hier_debug(hier)
        hier.pop()
        return hier
    #f All done
    pass

     */

