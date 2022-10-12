use std::{ops::Deref, rc::Rc};

use glfw::Context;
use glium::backend::Facade;

pub struct GlfwBackend {
    pub gl_window: *mut glfw::Window,
}

impl GlfwBackend {
    fn window(&self) -> &mut glfw::Window {
        unsafe { &mut *self.gl_window }
    }
}

unsafe impl<'a> glium::backend::Backend for GlfwBackend {
    fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
        self.window().swap_buffers();
        Ok(())
    }

    unsafe fn get_proc_address(&self, symbol: &str) -> *const std::os::raw::c_void {
        self.window().get_proc_address(symbol)
    }

    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        let sizes = self.window().get_framebuffer_size();
        (sizes.0 as u32, sizes.1 as u32)
    }

    fn is_current(&self) -> bool {
        self.window().is_current()
    }

    unsafe fn make_current(&self) {
        self.window().make_current();
    }
}

/// Facade implementation for an GLFW window.
#[derive(Clone)]
pub struct GlfwFacade {
    // contains everything related to the current context and its state
    pub context: Rc<glium::backend::Context>,
    pub backend: Rc<GlfwBackend>,
}

impl Facade for GlfwFacade {
    fn get_context(&self) -> &Rc<glium::backend::Context> {
        &self.context
    }
}

impl Deref for GlfwFacade {
    type Target = glium::backend::Context;

    fn deref(&self) -> &glium::backend::Context {
        &self.context
    }
}

impl GlfwFacade {
    pub fn window(&self) -> &glfw::Window {
        self.backend.window()
    }

    pub fn window_mut(&mut self) -> &mut glfw::Window {
        self.backend.window()
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
