pub mod geometry;
pub mod texture_uniform;

use winit::{
	event::{
		Event,
		WindowEvent,
	},
	event_loop::{ControlFlow, EventLoop},
};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};

use crate::*;

const VERTEX_SHADER: &str = include_str!("./shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/fragment.glsl");

#[derive(Default, Debug)]
pub struct RawUI;
impl RawUI {
	pub fn run<State: Default>(self, ui: &mut ui::UI<State>) -> DaggerResult<()> {
		let event_loop = EventLoop::new().expect("Failed to create EventLoop!");
		
		// TODO: window settings
		let (window, display) = SimpleWindowBuilder::new().with_title("Dagger Window").build(&event_loop);
		
		let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).expect("Failed to compile OpenGL program!");

		ui.input_manager.start();
		
		event_loop.run(move |event, window_target| {
			match event {
				Event::AboutToWait => {
					ui.input_manager.before_frame();
					// TODO: check if all images are gathered
					// TODO: gather images if needed
					
					let screen_size = Vec2::new(window.inner_size().width, window.inner_size().height).as_();
					layout::calculate_geometries(&mut ui.root, screen_size);
					ui.input_manager.set_states(&mut ui.root);
					
					window.request_redraw();
					
					ui.input_manager.after_frame();
				},
				Event::LoopExiting => {
					ui.input_manager.end();
				},
				Event::WindowEvent { event, .. } => {
					match event {
						WindowEvent::CloseRequested => window_target.exit(),
						WindowEvent::CursorMoved { position, .. } => {
							ui.input_manager.mouse_on_screen = true;
							ui.input_manager.register_mouse_position(position);
						},
						WindowEvent::CursorEntered { .. } => {
							ui.input_manager.mouse_on_screen = true;
						},
						WindowEvent::CursorLeft { .. } => {
							ui.input_manager.mouse_on_screen = false;
							ui.input_manager.reset_states(&mut ui.root);
						},
						WindowEvent::KeyboardInput { event, .. } => ui.input_manager.register_key_event(event),
						WindowEvent::RedrawRequested => {
							let screen_size = Vec2::new(window.inner_size().width, window.inner_size().height).as_();

							let element_vertices = ui.root.gather_vertices_quad();
							let vertex_buffer = geometry::create_vertex_buffer(&display, &element_vertices, screen_size);
							
							let mut parameters = glium::DrawParameters::default();
							parameters.blend = glium::Blend::alpha_blending();

							let mut frame = display.draw();
							frame.clear_color(1.0, 0.0, 1.0, 1.0);
							frame.draw(&vertex_buffer, &indices, &program, &glium::uniform! { }, &parameters).expect("Failed to draw UI!");
							frame.finish().expect("Failed to draw frame!");
						},
						_ => {},
					}
				},
				_ => {},
			}
		}).map_err(|err| format!("{:?}", err))
	}
}