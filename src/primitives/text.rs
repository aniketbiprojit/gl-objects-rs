use glium::{Rect, Surface};
use std::borrow::Cow;

use glium::{backend::Facade, implement_vertex, program, uniform, Frame};
use glow::HasContext;
use rusttype::{gpu_cache::Cache, point, vector, Font, PositionedGlyph, Scale};

use crate::object::{GliumObjectTrait, OpenGLObjectTrait};

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

pub struct Text {
    pub target: Option<Frame>,
    pub font: Font<'static>,
    source: String,
}

impl Text {
    pub fn new(target: Option<Frame>, font: Font<'static>, source: &str) -> Text {
        Text {
            target,
            font,
            source: source.to_string(),
        }
    }
}

impl OpenGLObjectTrait for Text {
    fn attach(&mut self, gl: &glow::Context) {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");
            Self::setup_shaders(gl, &program, self.source.to_string());

            gl.use_program(Some(program));

            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }
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

impl GliumObjectTrait for Text {
    fn attach_glium(&mut self, frame: &mut glium::Frame, display: &dyn Facade) {
        let target = frame;
        let scale = 20.0;
        let (cache_width, cache_height) = ((512.0 * scale) as u32, (512.0 * scale) as u32);
        let mut cache: rusttype::gpu_cache::Cache<'static> = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();

        let program = program!(
        display,
        140 => {
                vertex: "
                    #version 140
                    in vec2 position;
                    in vec2 tex_coords;
                    in vec4 colour;
                    out vec2 v_tex_coords;
                    out vec4 v_colour;
                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                        v_tex_coords = tex_coords;
                        v_colour = colour;
                    }
                ",

                fragment: "
                    #version 140
                    uniform sampler2D tex;
                    in vec2 v_tex_coords;
                    in vec4 v_colour;
                    out vec4 f_colour;
                    void main() {
                        f_colour = v_colour * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
                    }
                "
        })
        .unwrap();
        let cache_tex = (glium::texture::Texture2d::with_format(
            display,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8,
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
        ))
        .unwrap();
        let text: String = "A".into();

        let width = 800;
        let glyphs = layout_paragraph(&self.font, Scale::uniform(24.0 * scale), width, &text);
        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }
        cache
            .cache_queued(|rect, data| {
                cache_tex.main_level().write(
                    glium::Rect {
                        left: rect.min.x,
                        bottom: rect.min.y,
                        width: rect.width(),
                        height: rect.height(),
                    },
                    glium::texture::RawImage2d {
                        data: Cow::Borrowed(data),
                        width: rect.width(),
                        height: rect.height(),
                        format: glium::texture::ClientFormat::U8,
                    },
                );
            })
            .unwrap();

        let uniforms = uniform! {
            tex: cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        let vertex_buffer = {
            #[derive(Copy, Clone)]
            struct Vertex {
                position: [f32; 2],
                tex_coords: [f32; 2],
                colour: [f32; 4],
            }

            implement_vertex!(Vertex, position, tex_coords, colour);
            let colour = [0.0, 0.0, 0.0, 1.0];
            let (screen_width, screen_height) = {
                // let (w, h) = display.get_framebuffer_dimensions();
                // (w as f32, h as f32)
                (800, 600)
            };
            let origin = point(0.0, 0.0);

            let vertices: Vec<Vertex> = glyphs
                .iter()
                .filter_map(|g| cache.rect_for(0, g).ok().flatten())
                .flat_map(|(uv_rect, screen_rect)| {
                    let min = origin
                        + (vector(
                            screen_rect.min.x as f32 / screen_width as f32 - 0.5,
                            1.0 - screen_rect.min.y as f32 / screen_height as f32 - 0.5,
                        )) * 2.0;
                    let max = origin
                        + (vector(
                            screen_rect.max.x as f32 / screen_width as f32 - 0.5,
                            1.0 - screen_rect.max.y as f32 / screen_height as f32 - 0.5,
                        )) * 2.0;
                    let gl_rect = Rect {
                        left: min.x as u32,
                        bottom: min.y as u32,
                        width: (max.x - min.x) as u32,
                        height: (max.y - min.y) as u32,
                    };
                    vec![
                        Vertex {
                            position: [gl_rect.left as f32, max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                            colour,
                        },
                        Vertex {
                            position: [gl_rect.left as f32, min.y],
                            tex_coords: [uv_rect.min.x, uv_rect.min.y],
                            colour,
                        },
                        Vertex {
                            position: [max.x, min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                            colour,
                        },
                        Vertex {
                            position: [max.x, min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                            colour,
                        },
                        Vertex {
                            position: [max.x, max.y],
                            tex_coords: [uv_rect.max.x, uv_rect.max.y],
                            colour,
                        },
                        Vertex {
                            position: [gl_rect.left as f32, max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                            colour,
                        },
                    ]
                })
                .collect();

            glium::VertexBuffer::new(display, &vertices).unwrap()
        };

        target
            .draw(
                &vertex_buffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniforms,
                &glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
