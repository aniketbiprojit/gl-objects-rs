use crate::primitives::rectangle::Rectangle;
use crate::window::Window;
use window::WindowTrait;

pub mod backend;
pub mod imgui_ctx;
pub mod object;
pub mod primitives;
pub mod shaders;
mod window;

pub fn glfw_example() {
    let mut handle =
        Window::<glfw::Glfw, glfw::Window>::new(800, 600, format!("GLFW {}", "Window".to_string()));

    handle.create_display();
    let rectangle = &mut Rectangle::new(200, 200, "resources/shader_with_matrix.shader");

    handle.render(&mut vec![rectangle], &mut vec![]);
}

#[cfg(feature = "sdl2")]
pub fn sdl2_example() {
    let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(
        800,
        600,
        format!("SDL {}", "Window".to_string()),
    );

    handle.create_display();
    let rectangle = &mut Rectangle::new(200, 200, "resources/shader_with_matrix.shader");

    handle.render(&mut vec![rectangle], &mut vec![])
}
