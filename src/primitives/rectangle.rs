use crate::object::BufferData;
use crate::object::OpenGLObjectTrait;
use crate::object::TestingEvent;
use gfx_maths::Mat4;
use glow::HasContext;
use glow::NativeProgram;

#[derive(Debug)]
pub struct MVP {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
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
            model: Mat4::identity(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Rectangle {
    width: u32,
    height: u32,
    program: Option<Box<NativeProgram>>,
    buffers: Option<BufferData>,
    source: String,
    matrix: MVP,
}

impl Rectangle {
    pub(crate) fn new(width: u32, height: u32, source: &str) -> Self {
        Self {
            height: height,
            width: width,
            program: None,
            buffers: None,
            source: source.to_string(),
            matrix: MVP::new(800, 600),
        }
    }
}

impl OpenGLObjectTrait for Rectangle {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            Self::setup_shaders(gl, &program, self.source.clone());
            gl.use_program(Some(program));

            let vertices = [
                0f32,
                0f32,
                0f32,
                self.width as f32,
                self.width as f32,
                self.height as f32,
                self.height as f32,
                0f32,
            ];

            self.buffers = Some(Self::setup_buffers(
                gl,
                &vertices,
                &vec![0u32, 1, 2, 2, 3, 0],
                2,
                8,
            ));

            let matrix = self.matrix.projection * self.matrix.view * self.matrix.model;

            let proj_matrix = gl.get_uniform_location(program, "u_proj_matrix");

            gl.uniform_matrix_4_f32_slice(proj_matrix.as_ref(), false, &matrix.values);
        }
    }

    fn render(&mut self, gl: &glow::Context, event: &TestingEvent) {
        if let TestingEvent::WindowResize(x, y) = event {
            self.matrix.projection =
                Mat4::orthographic_opengl(0.0, *x as f32, *y as f32, 0.0, -1.0, 1.0);
        }

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
