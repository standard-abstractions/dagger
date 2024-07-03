use glium::{
	backend::Facade,
	texture::ResidentTexture,
	Surface
};
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

use crate::*;
use opengl::*;

#[derive(Debug)]
pub struct UIOpenGL {
	pub event_loop: EventLoop<()>,
	pub window: Option<window::Window>,
}

pub fn gather_images<F>(facade: &F) -> Vec<ResidentTexture>
where F: Facade {
	let mut textures = vec![];
	for image_path in IMAGE_PATHS {
		let image = image::io::Reader::open(image_path).unwrap().decode().unwrap().to_rgba8();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);

		let texture = glium::texture::Texture2d::new(facade, image).unwrap().resident().unwrap();
		textures.push(texture);
	}
	textures
}

const IMAGE_PATHS: &[&str] = &[
	"wewritelogo.png",
	"l.png",
];

impl UIOpenGL {
	pub fn new() -> Self {
		Self {
			event_loop: EventLoop::new().expect("Failed to create event loop!"),
			window: None,
		}
	}

	pub fn add_window_from_builder(&mut self, window_builder: WinitWindowBuilder) {
		let window = window::Window::from_builder_and_loop(window_builder, &self.event_loop);

		self.window = Some(window);
	}

	pub fn run(mut self, input: &mut input::InputManager) -> Result<(), EventLoopError> {
		// Load inital images
		let glium_display = &self.window.as_ref().unwrap().glium_display;
		let textures = gather_images(glium_display);
		let mut uniform_buffer = glium::uniforms::UniformBuffer::<uniform::TextureBuffer>::empty_unsized(glium_display, textures.len() * 8).unwrap();
		for (i, element) in uniform_buffer.map().textures.iter_mut().enumerate() {
			*element = glium::texture::TextureHandle::new(&textures[i], &Default::default());
		}

		self.event_loop.run(move |event, window_target| {
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => {
						window_target.exit();
					},
					WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
						input.register_winit_keyboard_event(event);
						self.window.as_ref().unwrap().winit_window.request_redraw();
					},
					WindowEvent::RedrawRequested => {
						let window = self.window.as_ref().unwrap();
						let window_size = Vec2::new(window.winit_window.inner_size().width, window.winit_window.inner_size().height).as_();

						let geometries = layout::calculate::calculate_geometries(&window.elements.as_ref().unwrap(), window_size);
						let vertex_buffer = layout::geometry::create_simple_quad_vertex_buffer(&window.glium_display, &geometries.iter().cloned().collect::<Vec<layout::geometry::Geometry>>().iter().cloned().collect(), window_size);

						let mut parameters = glium::DrawParameters::default();
						parameters.blend = glium::Blend::alpha_blending();

						let mut frame = window.glium_display.draw();
						frame.clear_color(1.0, 0.0, 1.0, 1.0);
						frame.draw(&vertex_buffer, &window.opengl_indices, &window.opengl_program, &glium::uniform! { textures_buffer: &uniform_buffer }, &parameters).expect("Failed to draw UI!");
						frame.finish().expect("Failed to draw frame!");
					},
					_ => {},
				},
				_ => {},
			}
		})
	}
}

impl Default for UIOpenGL {
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