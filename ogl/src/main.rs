extern crate gl;
extern crate sdl2;
extern crate gl_model;

pub mod render_gl;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();


    // Create background data
    let vertex_data = [-0.5f32, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    let index_data = [0u8,1,2];

    // Create indices and vertex data
    let indices  = gl_model::buffer::Indices::new(&index_data, 0, 0);

    let data = gl_model::buffer::Data::new(&vertex_data, 0, 0);
    let view = gl_model::buffer::View::new(&data, 3, gl::FLOAT, 0, 0); // 3 floats per, stride==0 => they are packed

    // Create set of data (indices, vertex data) to by subset into by the meshes and their primitives
    let vertices = gl_model::primitive::Vertices::new(&indices, &view);

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let mut triangle = gl_model::primitive::Primitive::new("triangle", &vertices);
    triangle.add_element(gl_model::drawable::Drawable::new_elements(gl::TRIANGLES, gl::UNSIGNED_BYTE, 3, 0)); // TRIANGLES of 3 indices from byte 0

    // Combine primitives into a mesh
    let mut triangle_mesh = gl_model::mesh::Mesh::new("triangle");
    triangle_mesh.add_primitive(triangle);

    let mut obj = gl_model::object::Object::new();
    obj.add_node(None, Some(&triangle_mesh), None);

    let instantiable = obj.create_instantiable();
    let shader_instantiable = obj.bind_shader(&instantiable, &shader_program);
    let instance = instantiable.instantiate();
    // Create the OpenGL buffers and data etc for the mesh for the shader
    // let sds = triangle_mesh.add_shader(&shader_program);

    // Can now drop all data except shader_program and sds

    // set up shared state for window

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // main loop

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // draw triangle
        shader_program.set_used();
    shader_instantiable.gl_draw(&instance);
        // sds.gl_draw();
        window.gl_swap_window();
    }
}
