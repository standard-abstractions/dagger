pub mod style;
pub mod vertex;

use crate::*;
use layout::*;

#[derive(Clone, Default, Debug)]
pub struct Element {
	pub layout:		Layout,

	pub style:		style::StyleSheet,
	pub is_hovered:	bool,
	pub is_focused:	bool,

	pub size:		Vec2<Physical>,
	pub reserve:	Vec2<Physical>,
	pub position:	Vec2<Physical>,

	pub children:	Vec<Element>,
}

impl Element {
	pub fn gather_vertices_quad(&self) -> Vec<vertex::Vertex> {
		let mut res = vec![];
		res.extend(self.vertices_quad());
		for child in &self.children {
			res.extend(child.gather_vertices_quad());
		}
		res
	}
}