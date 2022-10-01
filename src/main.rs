use crate::window::Window;
use crate::window::WindowTrait;

mod window;

fn main() {
    println!("Hello, world!");

    let width = 800u32;
    let height = 600u32;

    let title = "GL Window".to_string();

    let mut handle: Window<sdl2::Sdl, sdl2::video::Window> = Window {
        width,
        height,
        title: format!("SDL2 - {}", title),
        ctx: None,
        window_handle: None,
    };

    handle.create_display();
    handle.event_loop();
}
