pub mod element;
pub mod window;

use dagger_layout::{
	geometry::Geometry, Layout, SizeCalculationContext
};
use element::*;

use vek::*;

use winit::{
	dpi::PhysicalSize,
	error::EventLoopError,
	event::{
		Event,
		WindowEvent
	},
	event_loop::EventLoop,
	window::WindowBuilder as WinitWindowBuilder,
};

use glium::Surface;

#[derive(Debug)]
pub struct UI {
	pub event_loop: EventLoop<()>,
	pub windows: Vec<window::Window>,
}

impl UI {
	pub fn new() -> Self {
		Self {
			event_loop: EventLoop::new().expect("Failed to create event loop!"),
			windows: vec![],
		}
	}

	pub fn add_window_from_builder(&mut self, window_builder: WinitWindowBuilder) {
		let window = window::Window::from_builder_and_loop(window_builder, &self.event_loop);

		self.windows.push(window);
	}

	pub fn run(mut self) -> Result<(), EventLoopError> {		
		self.event_loop.run(move |event, window_target| {
			match event {
				Event::WindowEvent { window_id, event } => match event {
					WindowEvent::CloseRequested => {
						self.windows.retain(|p| p.winit_window.id() != window_id);
						if self.windows.len() == 0 {
							window_target.exit();
						}
					},
					WindowEvent::RedrawRequested => {
						let window = self.windows.iter_mut().find(|p| p.winit_window.id() == window_id).unwrap();

						let geometries = Layout::calculate(&window.elements[0], SizeCalculationContext {
							parent_size: Vec2::new(window.winit_window.inner_size().width, window.winit_window.inner_size().height).as_(),
							remaining_space: Vec2::new(window.winit_window.inner_size().width, window.winit_window.inner_size().height).as_(),
							remaining_children: Vec2::one(),
						}, Vec2::zero());
						let vertex_buffer = Geometry::create_vertex_buffer(&window.glium_display, Vec2::new(window.winit_window.inner_size().width, window.winit_window.inner_size().height).as_(), &geometries);

						let mut frame = window.glium_display.draw();
						frame.clear_color(1.0, 0.0, 1.0, 1.0);
						frame.draw(&vertex_buffer, &window.opengl_indices, &window.opengl_program, &glium::uniform!(), &Default::default()).expect("Failed to draw frame!");
						frame.finish().expect("Failed to draw frame!");
					}
					_ => {},
				},
				_ => {},
			}
		})
	}
}

impl Default for UI {
	fn default() -> Self {
		let mut res = Self::new();
		res.add_window_from_builder(
			WinitWindowBuilder::new()
				.with_title("Dagger Window")
				.with_inner_size(PhysicalSize::new(1156, 768))
		);
		res
	}
}