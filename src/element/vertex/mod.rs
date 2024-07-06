use crate::*;
use super::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
	pub position:			Vec2<Physical>,
	pub color:				style::Color,
}
impl Vertex {
	pub fn new(position: Vec2<Physical>, color: style::Color) -> Self {
		Self { position, color }
	}
}

impl Element {
	pub fn vertices_quad(&self) -> [Vertex;6] {
		let color = if self.is_hovered {
			self.style.hovered.unwrap_or(self.style.normal).color
		} else if self.is_focused {
			self.style.focused.unwrap_or(self.style.normal).color
		} else {
			self.style.normal.color
		};

		[
			Vertex::new(self.position, color),
			Vertex::new(self.position + (0, self.size.y), color),
			Vertex::new(self.position + (self.size.x, 0), color),

			Vertex::new(self.position + (self.size.x, 0), color),
			Vertex::new(self.position + (0, self.size.y), color),
			Vertex::new(self.position + self.size, color),
		]
	}

	pub fn vertices_full(&self) -> Vec<Vertex> {
		todo!()
	}
}