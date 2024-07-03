use winit::{
	event::{
		Event,
		WindowEvent,
	},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

use crate::*;

#[derive(Default, Debug)]
pub struct RawUI {

}

impl RawUI {
	pub fn run<State: Default>(self, ui: &mut ui::UI<State>) -> DaggerResult<()> {
		let event_loop = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Wait);
		
		let window = WindowBuilder::new().build(&event_loop).unwrap();

		ui.state_manager.start();
		ui.input_manager.start();
		
		event_loop.run(move |event, window_target| {
			match event {
				Event::AboutToWait => {
					ui.state_manager.before_frame();
					ui.input_manager.before_frame();
					
					ui.state_manager.after_frame();
					ui.input_manager.after_frame();
				},
				Event::LoopExiting => {
					ui.state_manager.end();
					ui.input_manager.end();
				},
				Event::WindowEvent { event, .. } => {
					match event {
						WindowEvent::CloseRequested => window_target.exit(),
						WindowEvent::CursorMoved { position, .. } => ui.input_manager.register_mouse_position(position),
						WindowEvent::KeyboardInput { event, .. } => ui.input_manager.register_key_event(event),
						WindowEvent::RedrawRequested => {

						},
						_ => {},
					}
				},
				_ => {},
			}
		}).map_err(|err| format!("{:?}", err))
	}
}