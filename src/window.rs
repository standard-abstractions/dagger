<<<<<<< Updated upstream
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

const VERTEX_SHADER: &str = include_str!("../shaders/vertex_shader.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment_shader.glsl");

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
=======
>>>>>>> Stashed changes
