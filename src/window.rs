use super::Element;

use winit::{
	event_loop::EventLoop,
	window::{
		Window as WinitWindow,
		WindowBuilder as WinitWindowBuilder,
	},
};

use glium::{
	backend::glutin::SimpleWindowBuilder,
	index::{
		NoIndices,
		PrimitiveType,
	},
	Display, Program,
};

#[derive(Debug)]
pub struct Window {
	pub winit_window: WinitWindow,
	pub glium_display: Display<glutin::surface::WindowSurface>,
	pub opengl_indices: NoIndices,
	pub opengl_program: Program,

	pub elements: Vec<Element>,
}

impl Window {
	pub fn from_builder_and_loop(window_builder: WinitWindowBuilder, event_loop: &EventLoop<()>) -> Self {
		let (winit_window, glium_display) = SimpleWindowBuilder::new()
			.set_window_builder(window_builder)
			.build(event_loop);
		
		let indices = NoIndices(PrimitiveType::TrianglesList);
		let program = Program::from_source(&glium_display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

		Self {
			winit_window,
			glium_display,

			opengl_indices: indices,
			opengl_program: program,

			elements: vec![],
		}
	}
}

const VERTEX_SHADER: &str = r#"
	#version 140

	in vec2 position;
	in vec4 color;
	out vec4 vertex_color;
	
	void main() {
		vertex_color = color;
		gl_Position = vec4(position, 0.0, 1.0);
	}
"#;

const FRAGMENT_SHADER: &str = r#"
	#version 140

	in vec4 vertex_color;
	out vec4 color;

	void main() {
		color = vertex_color;
	}
"#;