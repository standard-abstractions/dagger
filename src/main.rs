pub mod window;

use dagger_layout::{
	geometry::Geometry, SizeCalculationContext, Element, Node, attributes, types::*, layout::LayoutChildren,
};

use glium::Surface;
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
		let image = image::load(std::io::Cursor::new(&include_bytes!("../wewritelogo.png")), image::ImageFormat::Png).unwrap().to_rgba8();
		let image_dimensions = image.dimensions();
		let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
		let texture = glium::Texture2d::new(&self.windows[0].glium_display, image).unwrap();
	
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
						let window_size = Vec2::new(window.winit_window.inner_size().width, window.winit_window.inner_size().height).as_();

						let geometries = window.elements.as_ref().unwrap().calculate_geometries(SizeCalculationContext::begin_calculation(window_size));
						let vertex_buffer = Geometry::create_vb_simple_quads(&window.glium_display, window_size, &geometries.to_vec());

						let mut parameters = glium::DrawParameters::default();
						parameters.blend = glium::Blend::alpha_blending();

						let mut frame = window.glium_display.draw();
						frame.clear_color(1.0, 0.0, 1.0, 1.0);
						frame.draw(&vertex_buffer, &window.opengl_indices, &window.opengl_program, &glium::uniform! { tex: &texture }, &parameters).expect("Failed to draw frame!");
						frame.finish().expect("Failed to draw frame!");
					},
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

pub fn main() {
	let mut ui = UI::default();
	ui.windows[0].elements = Some(Node::new(Element::new().with_normal(
		attributes::Attributes::default()
			.panel_width(DistancePercentRemainingAuto::Remaining(1.0))
			.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
			.panel_background(Background::Color(Color::black()))
			.layout_children(LayoutChildren::column().with_gap(1))
	)).with_children(vec![
		Node::new(Element::new().with_normal(
			attributes::Attributes::default()
				.panel_width(DistancePercentRemainingAuto::Remaining(1.0))
				.panel_height(DistancePercentRemainingAuto::Pixels(32))
				.panel_padding([DistancePercent::Pixels(4);4])
				.panel_background(Background::Color(Color::white()))
				.layout_children(LayoutChildren::row().with_gap(4))
		)).with_children(vec![
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Pixels(64))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Color(Color::rgb(Vec3::broadcast(128))))
			)),
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Pixels(42))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Color(Color::rgb(Vec3::broadcast(128))))
			)),
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Pixels(17))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Color(Color::rgb(Vec3::broadcast(128))))
			)),
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Pixels(121))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Color(Color::rgb(Vec3::broadcast(128))))
			)),
		]),
		Node::new(Element::new().with_normal(
			attributes::Attributes::default()
				.panel_width(DistancePercentRemainingAuto::Remaining(1.0))
				.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
				.layout_children(LayoutChildren::row().with_gap(1))
		)).with_children(vec![
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Pixels(256))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Color(Color::white()))
			)).with_children(vec![
				
			]),
			Node::new(Element::new().with_normal(
				attributes::Attributes::default()
					.panel_width(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_height(DistancePercentRemainingAuto::Remaining(1.0))
					.panel_background(Background::Image(1))
			)).with_children(vec![
				
			]),
		]),
	]));
	ui.run().expect("Failed to run UI!");
}