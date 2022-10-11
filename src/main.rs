use crate::primitives::rectangle::Rectangle;
use crate::primitives::text::Text;
use crate::primitives::triangle;
use crate::window::Window;
use crate::window::WindowTrait;
use rusttype::Font;

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

    #[cfg(not(feature = "sdl2"))]
    let mut handle =
        Window::<glfw::Glfw, glfw::Window>::new(width, height, format!("GLFW {}", title.clone()));

    let objects: &mut Vec<&mut dyn object::OpenGLObjectTrait> = &mut vec![];
    let sdl = false; // is_sdl();

    #[cfg(feature = "sdl2")]
    let mut handle = Window::<sdl2::Sdl, sdl2::video::Window>::new(
        width,
        height,
        format!("SDL {}", title.clone()),
    );

    let font_data = include_bytes!("../fonts/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let rectangle1 = &mut Rectangle::new(200, 200, "resources/shader_with_matrix.shader");

    let triangle2 = &mut triangle::Triangle::new(
        [0.5f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32],
        "resources/shader_with_uniform.shader",
    );

    let text = &mut Text::new(None, font, "resources/text_shader.shader");
    println!("{}", rectangle1.is_in_bounding_box(20, 10));

    objects.push(rectangle1);
    objects.push(triangle2);
    objects.push(text);

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
