use imgui_opengl_renderer::Renderer;

use crate::object::OpenGLObjectTrait;

pub struct ImguiCtx {
    imgui_ctx: imgui::Context,
    renderer: Renderer,
}

impl ImguiCtx {
    pub fn new<F>(mut load_fn: F) -> Self
    where
        F: FnMut(&'static str) -> *const ::std::os::raw::c_void,
    {
        let mut imgui_ctx = imgui::Context::create();
        imgui_ctx
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        imgui_ctx.fonts().build_rgba32_texture();
        let renderer =
            imgui_opengl_renderer::Renderer::new(&mut imgui_ctx, |s| -> *const std::ffi::c_void {
                load_fn(s)
            });

        Self {
            imgui_ctx,
            renderer,
        }
    }
}

impl OpenGLObjectTrait for ImguiCtx {
    fn attach(&mut self, _gl: &glow::Context) {
        let io = self.imgui_ctx.io_mut();
        let (win_w, win_h) = (800, 600);
        let (draw_w, draw_h) = (1600, 1200);

        io.display_size = [win_w as f32, win_h as f32];
        io.display_framebuffer_scale = [
            (draw_w as f32) / (win_w as f32),
            (draw_h as f32) / (win_h as f32),
        ];
    }

    fn render(&mut self, _gl: &glow::Context, _event: &crate::object::TestingEvent) {
        let ui = self.imgui_ctx.frame();
        ui.text(format!("{:?}", "Some Data"));
        ui.text("More Data");
        self.renderer.render(ui);
    }

    fn detach(&mut self, _gl: &glow::Context) {
        drop(&self.imgui_ctx);
    }
}
