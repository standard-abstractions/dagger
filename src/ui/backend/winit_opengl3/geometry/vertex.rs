use crate::*;
use element::vertex::Vertex as ElementVertex;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GliumVertex {
	position:			[Abstract;2],
	color:				[Abstract;4],
}
glium::implement_vertex!(GliumVertex, position, color);
impl GliumVertex {
	pub fn to_screen_space(&self, screen_size: Vec2<Physical>) -> Self {
		let mut position = Vec2::from(self.position) / screen_size.as_() * 2.0 - 1.0;
		position.y *= -1.0;
		Self {
			position: position.into_array(),
			color: self.color,
		}
	}
}
impl From<ElementVertex> for GliumVertex {
	fn from(value: ElementVertex) -> Self {
		Self {
			position: value.position.as_().into_array(),
			color: value.color.into_array(),
		}
	}
}