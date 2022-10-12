use sdl2::video::Window;
use std::{ops::Deref, rc::Rc};

use glium::backend::Facade;

pub struct Sdl2Backend {
    pub gl_window: *mut sdl2::video::Window,
}

impl Sdl2Backend {
    fn subsystem(&self) -> &sdl2::VideoSubsystem {
        let ptr = self.gl_window;
        let window = unsafe { &*ptr };
        window.subsystem()
    }

    fn window(&self) -> &Window {
        let ptr = self.gl_window;
        let window = unsafe { &*ptr };
        window
    }

    fn window_mut(&self) -> &mut Window {
        let ptr = self.gl_window;
        let window = unsafe { &mut *ptr };
        window
    }
}

unsafe impl<'a> glium::backend::Backend for Sdl2Backend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        let window = self.window();

        Ok(window.gl_swap_window())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::os::raw::c_void {
        self.subsystem().gl_get_proc_address(symbol) as _
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let window = self.window();
        window.drawable_size()
    }

    fn is_current(&self) -> bool {
        true
    }

    unsafe fn make_current(&self) {
        // skip looping causes troubles.
    }
}

/// Facade implementation for an SDL2 window.
#[derive(Clone)]
pub struct Sdl2Facade {
    // contains everything related to the current context and its state
    context: Rc<glium::backend::Context>,
    backend: Rc<Sdl2Backend>,
}

impl Facade for Sdl2Facade {
    fn get_context(&self) -> &Rc<glium::backend::Context> {
        &self.context
    }
}

impl Deref for Sdl2Facade {
    type Target = glium::backend::Context;

    fn deref(&self) -> &glium::backend::Context {
        &self.context
    }
}

impl Sdl2Facade {
    pub fn window(&self) -> &Window {
        self.backend.window()
    }

    pub fn window_mut(&mut self) -> &mut Window {
        self.backend.window_mut()
    }

    /// Start drawing on the backbuffer.
    ///
    /// This function returns a `Frame`, which can be used to draw on it.
    /// When the `Frame` is destroyed, the buffers are swapped.
    ///
    /// Note that destroying a `Frame` is immediate, even if vsync is enabled.
    pub fn draw(&self) -> glium::Frame {
        glium::Frame::new(
            self.context.clone(),
            glium::backend::Backend::get_framebuffer_dimensions(&self.backend),
        )
    }
}
