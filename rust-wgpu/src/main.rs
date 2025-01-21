pub mod texture;
pub mod k4a;
pub mod app;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};



pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = app::App::new(&window);

    let _ = 
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app.window().id() => {
                app.update(event, control_flow);
            },
            _ => {}
        }
    });    
}
