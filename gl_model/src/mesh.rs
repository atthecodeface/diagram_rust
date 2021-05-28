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

@file    mesh.rs
@brief   Part of OpenGL library
 */

//a Notes
//
//

//a Imports
use crate::shader::ShaderClass;
use crate::primitive::Primitive;
use super::shader;

pub struct Mesh<'a>
{
    name       : String,
    primitives : Vec<Primitive<'a>>,
    // shader_vaos : Vec<Vec<usize>>,
}

impl <'a> Mesh <'a> {
    pub fn new(name: &str) -> Self {
        let name = String::from(name);
        let primitives  = Vec::new();
        // let shader_vaos = Vec::new();
        Self { name, primitives }
    }
    pub fn add_primitive(&mut self, primitive:Primitive<'a>) {
        self.primitives.push(primitive);
    }
    pub fn add_shader_drawables<'z, S:ShaderClass>(&self, shader:&S, instantiable:&'z mut shader::Instantiable) {
        for p in &self.primitives {
            p.add_shader_drawables(shader, instantiable);
        }
    }
}
