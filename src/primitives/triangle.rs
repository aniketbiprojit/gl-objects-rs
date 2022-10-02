use crate::object::BufferData;
use crate::object::TestingEvent;
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

    fn render(&mut self, gl: &glow::Context, _event: &TestingEvent) {
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
