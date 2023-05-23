use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
};
fn main() {
    let window_builder = winit::window::WindowBuilder::new().with_decorations(false);
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let window_target = EventLoopBuilder::new().build();
    let template_builder = ConfigTemplateBuilder::new().with_transparency(true);
    let (mut window, gl_config) = display_builder
        .build(&window_target, template_builder, |configs| {
            configs.into_iter().next().unwrap()
        })
        .unwrap();

    window_target.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            _ => {}
        },
        _ => {}
    });
}
