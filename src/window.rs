use glfw::Context;
use glium::Surface;
use glow::HasContext;

use std::sync::mpsc::channel;

use crate::object::{OpenGLObjectTrait, TestingEvent};

pub struct Window<WindowContext, WindowHandle> {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub ctx: Option<Box<WindowContext>>,
    pub internal_handle: Option<*mut WindowHandle>,
    pub gl: Option<Box<glow::Context>>,
    // sdl2 specific
    #[cfg(feature = "sdl2")]
    gl_context: Option<Box<sdl2::video::GLContext>>,
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
            #[cfg(feature = "sdl2")]
            gl_context: None,
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

        if self.ctx.is_none() {
            panic!("ctx is none");
        }
        let gl = self.gl.as_ref().unwrap();
        let glfw = self.ctx.as_mut().unwrap();
        let raw_handle = self.internal_handle.unwrap();
        let window = unsafe { &mut *raw_handle };

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

        struct Backend {
            gl_window: *mut glfw::Window,
        }
        unsafe impl<'a> glium::backend::Backend for Backend {
            fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
                let window = unsafe { &mut *self.gl_window };

                Ok(window.swap_buffers())
            }

            unsafe fn get_proc_address(&self, symbol: &str) -> *const std::os::raw::c_void {
                let window = &mut *self.gl_window;

                window.get_proc_address(symbol)
            }

            fn get_framebuffer_dimensions(&self) -> (u32, u32) {
                let window = unsafe { &mut *self.gl_window };
                let x = window.get_framebuffer_size();
                // .get_framebuffer_size();
                (x.0 as u32, x.1 as u32)
            }

            fn is_current(&self) -> bool {
                let window = unsafe { &mut *self.gl_window };

                window.is_current()
            }

            unsafe fn make_current(&self) {
                let window = &mut *self.gl_window;

                window.make_current()
            }
        }

        let gl_window = raw_handle;

        // target.borrow_mut().clear_color(0.0, 1.0, 0.0, 1.0);
        // target.borrow_mut().clear_depth(1.0);

        // self.frame = target.clone();

        // self.render(objects);

        let mut frame_count = 0;
        let time = std::time::Instant::now();

        while !window.should_close() || true {
            // FIXME - hacky. probably a bad idea to create the context inside
            // the loop but the fps was still around 60ish
            println!(
                "frames per second {}",
                frame_count as f32 / time.elapsed().as_secs_f32()
            );
            let glium_context = unsafe {
                // The first parameter is our backend.
                //
                // The second parameter tells glium whether or not it should regularly call `is_current`
                // on the backend to make sure that the OpenGL context is still the current one.
                //
                // It is recommended to pass `true`, but you can pass `false` if you are sure that no
                // other OpenGL context will be made current in this thread.
                let backend = Backend {
                    gl_window: gl_window,
                };
                glium::backend::Context::new(backend, true, Default::default()).unwrap()
            };

            if window.should_close() {
                println!("frame is non");
                window.set_should_close(true);

                break;
            } else {
                glfw.poll_events();

                let mut test_event = None;
                for (_, event) in glfw::flush_messages(&receiver) {
                    if let glfw::WindowEvent::Size(x, y) = event {
                        test_event = Some(TestingEvent::new(
                            x,
                            y,
                            window.get_framebuffer_size().0 as i32,
                            window.get_framebuffer_size().1 as i32,
                        ));

                        // if let glfw::WindowEvent::Close = event {
                        //     window.set_should_close(true);
                        //     Takeable::take(&mut self.frame.borrow_mut())
                        //         .finish()
                        //         .unwrap();
                        // }
                    }
                }

                let mut target = glium::Frame::new(
                    glium_context.clone(),
                    glium_context.get_framebuffer_dimensions(),
                );

                unsafe {
                    gl.clear_color(0.1, 0.2, 0.3, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    target.clear_color(0.0, 1.0, 0.0, 1.0);
                    target.clear_depth(1.0);
                }
                for elem in objects.into_iter() {
                    elem.attach(gl);

                    if test_event.is_some() {
                        let sizes = test_event.as_ref().unwrap();
                        elem.window_resize(
                            [sizes.window_draw_resize[0], sizes.window_draw_resize[1]],
                            [sizes.window_resize[0], sizes.window_resize[1]],
                        )
                    }
                    elem.render(gl);
                }

                let (x, y) = window.get_framebuffer_size();
                unsafe {
                    gl.viewport(0, 0, x, y);
                }
                // window.swap_buffers();
                frame_count += 1;

                target.finish().unwrap_or_else(|e| {
                    println!("Error: {}", e);
                });
            }
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
        let raw_handle = Box::into_raw(Box::new(window));
        self.internal_handle = Some(raw_handle);
        self.gl = Some(Box::new(gl));
    }

    fn load_with(&mut self, s: &str) -> *const std::ffi::c_void {
        let window = unsafe { &mut *self.internal_handle.unwrap() };
        window.get_proc_address(s) as *const std::ffi::c_void
    }
}

#[cfg(feature = "sdl2")]
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
        self.gl_context = Some(Box::new(gl_context));

        self.ctx = Some(Box::new(ctx));
        self.internal_handle = Some(Box::into_raw(Box::new(window)));
        self.gl = Some(Box::new(gl));
        // self.render(objects);
    }

    fn load_with(&mut self, s: &str) -> *const std::ffi::c_void {
        let window = unsafe { &*self.internal_handle.unwrap() };

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
            let raw_handle = self.internal_handle.unwrap();
            let window = { &*raw_handle };

            struct Backend {
                gl_window: *mut sdl2::video::Window,
            }
            unsafe impl<'a> glium::backend::Backend for Backend {
                fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
                    let window = unsafe { &*self.gl_window };

                    Ok(window.gl_swap_window())
                }

                unsafe fn get_proc_address(&self, symbol: &str) -> *const std::os::raw::c_void {
                    let window = &*self.gl_window;

                    window.subsystem().gl_get_proc_address(symbol) as _
                }

                fn get_framebuffer_dimensions(&self) -> (u32, u32) {
                    let window = unsafe { &*self.gl_window };
                    window.drawable_size()
                }

                fn is_current(&self) -> bool {
                    true
                }

                unsafe fn make_current(&self) {}
            }

            let gl_window = raw_handle;

            let glium_context = {
                // The first parameter is our backend.
                //
                // The second parameter tells glium whether or not it should regularly call `is_current`
                // on the backend to make sure that the OpenGL context is still the current one.
                //
                // It is recommended to pass `true`, but you can pass `false` if you are sure that no
                // other OpenGL context will be made current in this thread.
                let backend = Backend {
                    gl_window: gl_window,
                };
                glium::backend::Context::new(backend, true, Default::default()).unwrap()
            };

            window
                .gl_make_current(&self.gl_context.as_ref().unwrap())
                .unwrap();

            let mut event_pump = ctx.event_pump().unwrap();

            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.viewport(
                0,
                0,
                window.drawable_size().0 as i32,
                window.drawable_size().1 as i32,
            );

            'render: loop {
                let mut test_event = None;
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
                                test_event = Some(TestingEvent::new(
                                    x,
                                    y,
                                    window.drawable_size().0 as i32,
                                    window.drawable_size().1 as i32,
                                ));
                            }
                        }

                        if let sdl2::event::Event::Quit { .. } = event {
                            break 'render;
                        }
                    }
                }

                let mut target = glium::Frame::new(
                    glium_context.clone(),
                    glium_context.get_framebuffer_dimensions(),
                );

                gl.clear(glow::COLOR_BUFFER_BIT);

                target.clear_color(0.0, 1.0, 0.0, 1.0);
                target.clear_depth(1.0);

                for elem in objects.into_iter() {
                    elem.attach(gl);

                    if test_event.is_some() {
                        let sizes = test_event.as_ref().unwrap();
                        elem.window_resize(
                            [sizes.window_draw_resize[0], sizes.window_draw_resize[1]],
                            [sizes.window_resize[0], sizes.window_resize[1]],
                        )
                    }

                    elem.render(gl);
                }

                // window.gl_swap_window();
                target.finish().unwrap();
            }
            for elem in objects.into_iter() {
                elem.detach(gl);
            }
        }
    }
}
