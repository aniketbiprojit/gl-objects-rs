use std::borrow::Cow;

use glium::Frame;
use glium::*;
use glow::HasContext;
use rusttype::{point, Font, PositionedGlyph, Scale};

use crate::object::OpenGLObjectTrait;

fn layout_paragraph<'a>(
    font: &Font<'a>,
    scale: Scale,
    width: u32,
    text: &str,
) -> Vec<PositionedGlyph<'a>> {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {}
                _ => {}
            }
            continue;
        }
        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

pub struct Text<'text_lifetime> {
    pub target: Option<Frame>,
    pub font: Font<'text_lifetime>,
    source: String,
}

impl<'a> Text<'a> {
    pub fn new(target: Option<Frame>, font: Font<'a>, source: &str) -> Text<'a> {
        Text {
            target,
            font,
            source: source.to_string(),
        }
    }
}

impl OpenGLObjectTrait for Text<'_> {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            gl.use_program(Some(program));

            Self::setup_shaders(gl, &program, self.source.to_string());
            let (cache_width, cache_height) = ((512.0 * 2.0) as u32, (512.0 * 2.0) as u32);
        }
    }

    fn render(&mut self, _gl: &glow::Context) {}

    fn detach(&mut self, _gl: &glow::Context) {}

    fn move_model(&mut self, _movement_x: f32, _movement_y: f32, _movement_z: f32) {}

    fn window_resize(&mut self, _draw_size: [f32; 2], _size: [f32; 2]) {}

    fn set_model(&mut self, _movement_x: f32, _movement_y: f32, _movement_z: f32) {}

    unsafe fn setup_shaders(gl: &glow::Context, program: &glow::NativeProgram, source: String)
    where
        Self: Sized,
    {
        let shaders = crate::shaders::ShaderData::new(source);

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
}
