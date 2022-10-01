use crate::object::BufferData;
use glow::HasContext;
use glow::NativeProgram;

use crate::object::OpenGLObject;

#[derive(Debug)]
pub(crate) struct Rectangle {
    height: u32,
    width: u32,
    program: Option<Box<NativeProgram>>,
    buffers: Option<BufferData>,
    source: String,
}

impl Rectangle {
    pub(crate) fn new(height: u32, width: u32, source: &str) -> Self {
        Self {
            height: height,
            width: width,
            program: None,
            buffers: None,
            source: source.to_string(),
        }
    }
}

impl OpenGLObject for Rectangle {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            Self::setup_shaders(gl, &program, self.source.clone());
            gl.use_program(Some(program));

            let vertices = [
                -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32,
            ];
            self.buffers = Some(Self::setup_buffers(
                gl,
                &vertices,
                &vec![0u32, 1, 2, 2, 3, 0],
                2,
                8,
            ));
        }
    }

    fn render(&mut self, gl: &glow::Context) {
        unsafe {
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
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
