pub mod graphics;
pub mod texture;
pub mod k4a;
pub mod app;
pub mod streams;


use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};



pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let app = app::App::new();
    let mut state = graphics::State::new(&window, app);

    let _ = 
    event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                state.update(event, control_flow);
            },
            _ => {}
        }
    });    
}
