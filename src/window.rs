use dagger_layout::{Element, Node};

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

	pub elements: Option<Node<Element>>,
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

			elements: None,
		}
	}
}

const VERTEX_SHADER: &str = r#"
	#version 140

	in vec2 position;
	in vec2 tex_coords;
	in int tex_id;
	in vec4 color;

	out vec2 vertex_tex_coords;
	flat out int vertex_tex_id;
	out vec4 vertex_color;

	void main() {
		vertex_tex_coords = tex_coords;
		vertex_tex_id = tex_id;
		vertex_color = color;
		gl_Position = vec4(position, 0.0, 1.0);
	}
"#;

const FRAGMENT_SHADER: &str = r#"
	#version 140

	in vec2 vertex_tex_coords;
	flat in int vertex_tex_id;
	in vec4 vertex_color;

	out vec4 color;

	uniform sampler2D tex;

	void main() {
		if (vertex_tex_id == 0) {
			color = vertex_color;
		} else {
			color = texture(tex, vertex_tex_coords);
		}
	}
"#;