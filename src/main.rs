use crate::window::Window;
use crate::window::WindowTrait;

mod object;
mod primitives;
pub mod shaders;
mod window;

fn main() {
    println!("Hello, world!");

    let width = 800u32;
    let height = 600u32;

    let title = "GL Window".to_string();

    // #[cfg(feature = "use_glfw")]
    // let mut handle = Window::<glfw::Glfw, glfw::Window>::new(width, height, title);

    // #[cfg(not(feature = "use_default"))]
    let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(width, height, title);

    handle.create_display();
    // handle.event_loop(render, render, render);
}
