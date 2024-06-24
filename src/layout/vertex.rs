use crate::*;
use style::types::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
	position:			Vec2<Physical>,
	color:				Color,
	background_id:		Option<u32>,
	background_uvs:		[Abstract;2],
}
impl Vertex {
	pub fn new(position: Vec2<Physical>, color: Color, background_id: Option<u32>, background_uvs: [Abstract; 2]) -> Self {
		Self { position, color, background_id, background_uvs }
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GliumVertex {
	position:			[Abstract;2],
	color:				[Abstract;4],
	background_id:		[u32;2],
	background_uvs:		[Abstract;2],
}
glium::implement_vertex!(GliumVertex, position, color, background_id, background_uvs);
impl GliumVertex {
	pub fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
		let mut position = Vec2::from(self.position) / screen_size.as_() * 2.0 - 1.0;
		position.y *= -1.0;
		Self {
			position: position.into_array(),
			color: self.color,
			background_id: self.background_id,
			background_uvs: self.background_uvs,
		}
	}
}
impl From<Vertex> for GliumVertex {
	fn from(value: Vertex) -> Self {
		Self {
			position: value.position.as_().into_array(),
			color: value.color.into_array(),
			background_id: [
				value.background_id.is_some() as u32,
				value.background_id.unwrap_or(0),
			],
			background_uvs: value.background_uvs,
		}
	}
}