use glow::HasContext;
pub struct Window<WindowContext, WindowHandle> {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub ctx: Option<WindowContext>,
    pub window_handle: Option<WindowHandle>,
}

pub trait WindowTrait<WindowHandle> {
    fn create_display(&mut self);
    fn load_with(&mut self, window: &WindowHandle, s: &str) -> *const std::ffi::c_void;
    fn event_loop(&mut self) {}
}

impl WindowTrait<sdl2::video::Window> for Window<sdl2::Sdl, sdl2::video::Window> {
    fn create_display(&mut self) {
        let ctx = sdl2::init().unwrap();

        let video_subsystem = ctx.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(4, 1);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_flags().debug().set();

        let window = video_subsystem
            .window(&self.title, self.width, self.height)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = unsafe { glow::Context::from_loader_function(|s| self.load_with(&window, s)) };

        println!("{:?}", gl.version());

        window.gl_make_current(&gl_context).unwrap();

        window.subsystem().gl_set_swap_interval(1).unwrap();

        self.ctx = Some(ctx);
        self.window_handle = Some(window);
    }

    fn event_loop(&mut self) {
        let mut event_pump = self
            .ctx
            .as_ref()
            .expect("Context not initialized before event loop")
            .event_pump()
            .unwrap();

        'event_loop: loop {
            for event in event_pump.poll_iter() {
                self.window_handle
                    .as_ref()
                    .expect("REASON")
                    .gl_swap_window();
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'event_loop;
                }
            }
        }
    }

    fn load_with(&mut self, window: &sdl2::video::Window, s: &str) -> *const std::ffi::c_void {
        window.subsystem().gl_get_proc_address(s) as _
    }
}
