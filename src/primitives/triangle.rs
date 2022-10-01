use crate::object::BufferData;
use glow::HasContext;
use glow::NativeProgram;

use crate::object::OpenGLObject;

#[derive(Debug)]
pub(crate) struct Triangle {
    positions: [f32; 6],
    program: Option<Box<NativeProgram>>,
    buffers: Option<BufferData>,
}

impl Triangle {
    pub(crate) fn new(positions: [f32; 6]) -> Self {
        Self {
            positions,
            program: None,
            buffers: None,
        }
    }
}

impl OpenGLObject for Triangle {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            Self::setup_shaders(gl, &program, "resources/rectangle.shader".to_string());
            gl.use_program(Some(program));
            self.buffers = Some(Self::setup_buffers(
                gl,
                &self.positions,
                &vec![0u32, 1, 2, 0],
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
}
