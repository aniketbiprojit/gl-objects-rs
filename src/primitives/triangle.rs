use crate::object::BufferData;
use glow::HasContext;
use glow::NativeProgram;

use crate::object::OpenGLObjectTrait;

#[derive(Debug)]
pub struct Triangle {
    positions: [f32; 6],
    program: Option<Box<NativeProgram>>,
    buffers: Option<BufferData>,
    source: String,
}

impl Triangle {
    pub fn new(positions: [f32; 6], source: &str) -> Self {
        Self {
            positions,
            program: None,
            buffers: None,
            source: source.to_string(),
        }
    }
}

impl OpenGLObjectTrait for Triangle {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            Self::setup_shaders(gl, &program, self.source.clone());
            gl.use_program(Some(program));
            self.buffers = Some(Self::setup_buffers(
                gl,
                &self.positions,
                &vec![0u32, 1, 2],
                2,
                8,
            ));
        }
    }

    fn render(&mut self, gl: &glow::Context) {
        unsafe {
            gl.draw_elements(glow::TRIANGLES, 4, glow::UNSIGNED_INT, 0);
        }
    }

    fn detach(&mut self, gl: &glow::Context) {
        if self.program.is_some() {
            unsafe {
                gl.delete_program(**self.program.as_ref().unwrap());
            }
        };
    }

    fn move_model(&mut self, _movement_x: f32, _movement_y: f32, _movement_z: f32) {}

    fn window_resize(&mut self, _draw_size: [f32; 2], _size: [f32; 2]) {}

    fn set_model(&mut self, _movement_x: f32, _movement_y: f32, _movement_z: f32) {}
}
