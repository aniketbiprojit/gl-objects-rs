use crate::primitives::rectangle::Rectangle;
use crate::primitives::triangle;
use crate::window::Window;
use crate::window::WindowTrait;

mod imgui_ctx;
mod object;
mod primitives;
pub mod shaders;
mod window;

fn main() {
    println!("Hello, world!");

    let width = 800_u32;
    let height = 600_u32;

    let title = "Window".to_string();

    // #[cfg(feature = "use_glfw")]
    let handle =
        Window::<glfw::Glfw, glfw::Window>::new(width, height, format!("GLFW {}", title.clone()));

    let objects: &mut Vec<&mut dyn object::OpenGLObjectTrait> = &mut vec![];
    let sdl = true;

    // #[cfg(not(feature = "use_default"))]
    let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(
        width,
        height,
        format!("SDL {}", title.clone()),
    );

    let rectangle1 = &mut Rectangle::new(200, 200, "resources/rectangle2.shader");

    let triangle2 = &mut triangle::Triangle::new(
        [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32],
        "resources/rectangle1.shader",
    );
    objects.push(rectangle1);
    objects.push(triangle2);

    handle.create_display();

    let mut imgui_ctx = None;
    if !sdl {
        imgui_ctx = Some(imgui_ctx::ImguiCtx::new(|s| -> *const std::ffi::c_void {
            handle.load_with(s)
        }));
    }

    if imgui_ctx.is_some() {
        objects.push(imgui_ctx.as_mut().unwrap());
    }

    handle.render(objects)
}
