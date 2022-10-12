use crate::shaders::ShaderData;
use gfx_maths::{Mat4, Vec3};
use glium::backend::Facade;
use glow::{Context, HasContext, NativeProgram};

#[derive(Debug)]
pub struct BufferData {
    pub vbo: glow::NativeBuffer,
    pub vao: glow::NativeVertexArray,
    pub ibo: glow::NativeBuffer,
}

#[derive(Debug)]
pub struct TestSize {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct TestingEvent {
    pub window_resize: [f32; 2],
    pub window_draw_resize: [f32; 2],
}

impl TestingEvent {
    pub fn new(size_x: i32, size_y: i32, draw_size_x: i32, draw_size_y: i32) -> Self {
        Self {
            window_resize: [size_x as f32, size_y as f32],
            window_draw_resize: [draw_size_x as f32, draw_size_y as f32],
        }
    }
}

#[derive(Debug)]
pub struct MVP {
    pub model: Vec3,
    pub view: Mat4,
    pub projection: Mat4,
}

impl MVP {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            projection: Mat4::orthographic_opengl(
                0.0,
                screen_width as f32,
                screen_height as f32,
                0.0,
                -1.0,
                1.0,
            ),
            view: Mat4::identity(),
            model: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait GliumObjectTrait: OpenGLObjectTrait {
    fn attach_glium(&mut self, gl: &Context, frame: &mut glium::Frame, facade: &dyn Facade);
}

pub trait OpenGLObjectTrait {
    fn attach(&mut self, gl: &Context);
    fn render(&mut self, gl: &Context);
    fn detach(&mut self, gl: &Context);

    fn get_movement_model(movement_x: f32, movement_y: f32, movement_z: f32) -> Mat4
    where
        Self: Sized,
    {
        Mat4::translate(Vec3::new(movement_x, movement_y, movement_z))
    }

    fn move_model(&mut self, movement_x: f32, movement_y: f32, movement_z: f32);
    fn window_resize(&mut self, draw_size: [f32; 2], size: [f32; 2]);

    unsafe fn setup_shaders(gl: &Context, program: &NativeProgram, source: String)
    where
        Self: Sized,
    {
        let shaders = ShaderData::new(source);

        let shader_sources = [
            (glow::VERTEX_SHADER, shaders.vertex_shader.source),
            (glow::FRAGMENT_SHADER, shaders.fragment_shader.source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl.create_shader(*shader_type).unwrap();

            gl.shader_source(shader, shader_source);

            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!(
                    "Shader compilation failed: {}",
                    gl.get_shader_info_log(shader)
                );
            }

            gl.attach_shader(*program, shader);

            shaders.push(shader);
        }

        gl.link_program(*program);

        if !gl.get_program_link_status(*program) {
            panic!("{}", gl.get_program_info_log(*program));
        }

        for shader in shaders {
            gl.detach_shader(*program, shader);
            gl.delete_shader(shader);
        }
    }

    unsafe fn setup_buffers(
        gl: &Context,
        vertices: &[f32],
        indices: &[u32],
        vao_size: i32,
        vao_stride: i32,
    ) -> BufferData
    where
        Self: Sized,
    {
        let (vbo, vao, ibo) = {
            let triangle_vertices_u8: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );

            let indices = indices;

            let indices_u8 = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );

            // We construct a buffer and upload the data
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

            // We now construct a vertex array to describe the format of the input buffer
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, vao_size, glow::FLOAT, false, vao_stride, 0);

            let ibo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_u8, glow::STATIC_DRAW);

            (vbo, vao, ibo)
        };
        BufferData { vbo, vao, ibo }
    }

    fn set_model(&mut self, movement_x: f32, movement_y: f32, movement_z: f32);
}
