use crate::primitives::rectangle::Rectangle;
use crate::primitives::triangle;
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

    let title = "Window".to_string();

    // #[cfg(feature = "use_glfw")]
    let mut handle =
        Window::<glfw::Glfw, glfw::Window>::new(width, height, format!("GLFW {}", title.clone()));

    // #[cfg(not(feature = "use_default"))]
    // let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(
    //     width,
    //     height,
    //     format!("SDL {}", title.clone()),
    // );

    let rectangle1 = &mut Rectangle::new(2, 3, "resources/rectangle2.shader");

    let triangle2 = &mut triangle::Triangle::new(
        [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32],
        "resources/rectangle1.shader",
    );

    handle.create_display(&mut vec![rectangle1, triangle2]);
}
