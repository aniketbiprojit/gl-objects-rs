use gl_object::window::WindowTrait;

fn main() {
    let mut handle = gl_object::window::Window::<sdl2::Sdl, sdl2::video::Window>::new(
        800,
        600,
        format!("SDL {}", "Window".to_string()),
    );

    handle.create_display();
    let rectangle = &mut gl_object::primitives::rectangle::Rectangle::new(
        200,
        200,
        "resources/shader_with_matrix.shader",
    );

    handle.render(&mut vec![rectangle])
}
