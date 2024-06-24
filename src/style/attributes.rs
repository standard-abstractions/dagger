use crate::*;
use super::{
	layout::*,
	types::*,
};

#[derive(Clone, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct StyleAttributes {
	pub size:				Vec2<SizeDPRA>,
	pub minimum_size:		Vec2<SizeDPRA>,
	pub maximum_size:		Vec2<SizeDPRA>,
	pub padding:			Slice4<SizeDP>,
	pub color:				Color,
	pub background_id:		Option<u32>,

	pub corner_size:	Slice4<Vec2<SizeDP>>,
	pub corner_type:	Slice4<CornerType>,

	pub edge_border_thickness:		Slice4<SizeDP>,
	pub edge_border_color:			Slice4<Color>,
	pub corner_border_thickness:	Slice4<SizeDP>,
	pub corner_border_color:		Slice4<Color>,

	pub layout_self:		LayoutSelf,
	pub layout_children:	LayoutChildren,
}
impl StyleAttributes {
	pub fn with_size(mut self, size: Vec2<SizeDPRA>) -> Self { self.size = size; self }
	pub fn with_width(mut self, width: SizeDPRA) -> Self { self.size.x = width; self }
	pub fn with_height(mut self, height: SizeDPRA) -> Self { self.size.y = height; self }
	pub fn with_minimum_size(mut self, minimum_size: Vec2<SizeDPRA>) -> Self { self.minimum_size = minimum_size; self }
	pub fn with_minimum_width(mut self, minimum_width: SizeDPRA) -> Self { self.minimum_size.x = minimum_width; self }
	pub fn with_minimum_height(mut self, minimum_height: SizeDPRA) -> Self { self.minimum_size.y = minimum_height; self }
	pub fn with_maximum_size(mut self, maximum_size: Vec2<SizeDPRA>) -> Self { self.maximum_size = maximum_size; self }
	pub fn with_maximum_width(mut self, maximum_width: SizeDPRA) -> Self { self.maximum_size.x = maximum_width; self }
	pub fn with_maximum_height(mut self, maximum_height: SizeDPRA) -> Self { self.maximum_size.y = maximum_height; self }
	pub fn with_padding(mut self, padding: Slice4<SizeDP>) -> Self { self.padding = padding; self }
	pub fn with_color(mut self, color: Color) -> Self { self.color = color; self }
	pub fn with_background_id(mut self, background_id: Option<u32>) -> Self { self.background_id = background_id; self }
	pub fn with_corner_size(mut self, corner_size: Slice4<Vec2<SizeDP>>) -> Self { self.corner_size = corner_size; self }
	pub fn with_corner_type(mut self, corner_type: Slice4<CornerType>) -> Self { self.corner_type = corner_type; self }
	pub fn with_edge_border_thickness(mut self, edge_border_thickness: Slice4<SizeDP>) -> Self { self.edge_border_thickness = edge_border_thickness; self }
	pub fn with_edge_border_color(mut self, edge_border_color: Slice4<Color>) -> Self { self.edge_border_color = edge_border_color; self }
	pub fn with_corner_border_thickness(mut self, corner_border_thickness: Slice4<SizeDP>) -> Self { self.corner_border_thickness = corner_border_thickness; self }
	pub fn with_corner_border_color(mut self, corner_border_color: Slice4<Color>) -> Self { self.corner_border_color = corner_border_color; self }
}
impl Default for StyleAttributes {
	fn default() -> Self {
		Self {
			size:				Vec2::new(vec![DPRA::Auto], vec![DPRA::Auto]),
			minimum_size:		Vec2::new(vec![], vec![]),
			maximum_size:		Vec2::new(vec![DPRA::Distance(Physical::MAX)], vec![DPRA::Distance(Physical::MAX)]),
			padding:			Slice4::broadcast(vec![]),
			color:				Color::zero(),
			background_id:		None,

			corner_size:	Slice4::broadcast(Vec2::new(vec![], vec![])),
			corner_type:	Slice4::broadcast(CornerType::Square),

			edge_border_thickness:		Slice4::broadcast(vec![DP::Distance(0)]),
			edge_border_color:			Slice4::broadcast(Color::zero()),
			corner_border_thickness:	Slice4::broadcast(vec![DP::Distance(0)]),
			corner_border_color:		Slice4::broadcast(Color::zero()),

			layout_self:		LayoutSelf::Flow { alignment: None },
			layout_children:	LayoutChildren::Stacked { alignment: Alignment::Start, gap: 0, column: false },
		}
	}
}