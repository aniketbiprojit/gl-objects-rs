use glfw::Context;
use glow::HasContext;
use std::sync::mpsc::channel;

use crate::object::{OpenGLObjectTrait, TestingEvent};

pub struct Window<WindowContext, WindowHandle> {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub ctx: Option<Box<WindowContext>>,
    pub internal_handle: Option<Box<WindowHandle>>,
    pub gl: Option<Box<glow::Context>>,
}

pub trait WindowTrait<WindowContext, WindowHandle> {
    fn new(width: u32, height: u32, title: String) -> Window<WindowContext, WindowHandle> {
        Window {
            width,
            height,
            title: format!("{}", title),
            ctx: None,
            internal_handle: None,
            gl: None,
        }
    }
    fn create_display<'a>(&mut self);
    fn render<'a>(&mut self, objects: &mut Vec<&'a mut dyn OpenGLObjectTrait>);
    fn load_with(&mut self, s: &str) -> *const std::ffi::c_void;
}

impl WindowTrait<glfw::Glfw, glfw::Window> for Window<glfw::Glfw, glfw::Window> {
    fn render<'a>(&mut self, objects: &mut Vec<&'a mut dyn OpenGLObjectTrait>) {
        if self.gl.is_none() {
            panic!("gl is none");
        }
        if self.internal_handle.is_none() {
            panic!("internal_handle is none");
        }
        if self.ctx.is_none() {
            panic!("ctx is none");
        }
        let gl = self.gl.as_ref().unwrap();
        let glfw = self.ctx.as_mut().unwrap();
        let window = self.internal_handle.as_mut().unwrap();
        let (sender, receiver): (
            std::sync::mpsc::Sender<(f64, glfw::WindowEvent)>,
            std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
        ) = channel();

        unsafe {
            glfw::ffi::glfwSetWindowUserPointer(
                window.window_ptr(),
                std::mem::transmute(Box::new(sender)),
            );
        }

        while !window.should_close() {
            glfw.poll_events();
            let mut test_event = TestingEvent::Ignore;
            for (_, event) in glfw::flush_messages(&receiver) {
                if let glfw::WindowEvent::Size(x, y) = event {
                    test_event = TestingEvent::WindowResize(x, y);
                }
            }

            unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
            }
            for elem in objects.into_iter() {
                elem.attach(gl);
                elem.render(gl, &test_event);
            }

            let (x, y) = window.get_framebuffer_size();
            unsafe {
                gl.viewport(0, 0, x, y);
            }
            window.swap_buffers();
        }
        for elem in objects.into_iter() {
            elem.detach(gl);
        }
    }

    fn create_display<'a>(&mut self) {
        let mut glfw: glfw::Glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, ..) = glfw
            .create_window(
                self.width,
                self.height,
                &self.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_size_polling(true);
        window.make_current();

        window.set_framebuffer_size_polling(true);

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                window.get_proc_address(s) as *const std::ffi::c_void
            })
        };

        println!("GLFW: {:?}", gl.version());
        glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

        self.ctx = Some(Box::new(glfw));
        self.internal_handle = Some(Box::new(window));
        self.gl = Some(Box::new(gl));

        // self.render(objects);
    }

    fn load_with(&mut self, s: &str) -> *const std::ffi::c_void {
        self.internal_handle.as_mut().unwrap().get_proc_address(s) as *const std::ffi::c_void
    }
}

impl WindowTrait<sdl2::Sdl, sdl2::video::Window> for Window<sdl2::Sdl, sdl2::video::Window> {
    fn create_display<'a>(&mut self) {
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
        let gl = unsafe {
            glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
        };

        println!("SDL {:?}", gl.version());

        window.gl_make_current(&gl_context).unwrap();

        window.subsystem().gl_set_swap_interval(1).unwrap();

        self.ctx = Some(Box::new(ctx));
        self.internal_handle = Some(Box::new(window));
        self.gl = Some(Box::new(gl));
        // self.render(objects);
    }

    fn load_with(&mut self, s: &str) -> *const std::ffi::c_void {
        let window = self.internal_handle.as_ref().unwrap();

        window.subsystem().gl_get_proc_address(s) as _
    }

    // calling externally on SDL2 fails.
    fn render(&mut self, objects: &mut Vec<&mut dyn OpenGLObjectTrait>) {
        if self.gl.is_none() {
            panic!("gl is none");
        }
        if self.internal_handle.is_none() {
            panic!("internal_handle is none");
        }
        if self.ctx.is_none() {
            panic!("ctx is none");
        }
        unsafe {
            let gl = self.gl.as_ref().unwrap();
            let ctx = self.ctx.as_ref().unwrap();
            let window = self.internal_handle.as_ref().unwrap();
            let gl_context = window.gl_create_context().unwrap();

            window.gl_make_current(&gl_context).unwrap();

            let mut event_pump = ctx.event_pump().unwrap();

            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.viewport(
                0,
                0,
                window.drawable_size().0 as i32,
                window.drawable_size().1 as i32,
            );

            'render: loop {
                let mut test_event = TestingEvent::Ignore;
                {
                    for event in event_pump.poll_iter() {
                        if let sdl2::event::Event::Window { win_event, .. } = event {
                            if let sdl2::event::WindowEvent::Resized(x, y) = win_event {
                                gl.viewport(
                                    0,
                                    0,
                                    window.drawable_size().0 as i32,
                                    window.drawable_size().1 as i32,
                                );
                                test_event = TestingEvent::WindowResize(x, y);
                            }
                        }

                        if let sdl2::event::Event::Quit { .. } = event {
                            break 'render;
                        }
                    }
                }

                gl.clear(glow::COLOR_BUFFER_BIT);

                for elem in objects.into_iter() {
                    elem.attach(gl);
                    elem.render(gl, &test_event);
                }

                window.gl_swap_window();
            }
            for elem in objects.into_iter() {
                elem.detach(gl);
            }
        }
    }
}
