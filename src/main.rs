use core::slice;

use crate::window::Window;
use crate::window::WindowTrait;
use glow::HasContext;
use object::OpenGLObject;

mod object;
mod primitives;
pub mod shaders;
mod window;

fn main() {
    println!("Hello, world!");

    let width = 800u32;
    let height = 600u32;

    let title = "GL Window".to_string();

    // #[cfg(feature = "use_glfw")]
    // let mut handle = Window::<glfw::Glfw, glfw::Window>::new(width, height, title);

    // #[cfg(not(feature = "use_default"))]
    let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(width, height, title);

    handle.create_display(setup_shaders, setup_buffers);
    // handle.event_loop(render, render, render);
}

const VERTEX_SHADER_SOURCE: &str = r#"#version 330
  in vec2 in_position;
  out vec2 position;
  void main() {
    position = in_position;
    gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
  }"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"#version 330
  precision mediump float;
  in vec2 position;
  out vec4 color;
  uniform float blue;
  void main() {
    color = vec4(position, 1.0, 1.0);
  }"#;

fn setup_shaders(gl: &glow::Context) -> glow::NativeProgram {
    let program = unsafe {
        let program = gl.create_program().expect("Cannot create program");

        let shader_sources = [
            (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE),
            (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        program
    };
    program
}

fn setup_buffers(
    gl: &glow::Context,
) -> (
    glow::NativeBuffer,
    glow::NativeVertexArray,
    glow::NativeBuffer,
) {
    let (vbo, vao, ibo) = unsafe {
        let triangle_vertices = vec![0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32];
        let triangle_vertices_u8: &[u8] = slice::from_raw_parts(
            triangle_vertices.as_ptr() as *const u8,
            triangle_vertices.len() * core::mem::size_of::<f32>(),
        );

        let triangle_indices = vec![0u32, 1, 2, 0];

        let triangle_indices_u8 = slice::from_raw_parts(
            triangle_indices.as_ptr() as *const u8,
            triangle_indices.len() * core::mem::size_of::<u32>(),
        );

        // We construct a buffer and upload the data
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

        // We now construct a vertex array to describe the format of the input buffer
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);

        let ibo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            triangle_indices_u8,
            glow::STATIC_DRAW,
        );

        (vbo, vao, ibo)
    };
    (vbo, vao, ibo)
}
