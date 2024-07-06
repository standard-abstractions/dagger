use crate::*;
use layout::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct StyleSheet {
	pub normal:		Style,
	pub hovered:	Option<Style>,
	pub focused:	Option<Style>,
}

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Style {
	/* Panel Design */
	pub color:			Color,
	pub image:			Option<usize>,					// TODO: Image type
	pub corner_size:	Slice4<Vec2<size::Simple>>,
	pub corner_type:	Slice4<CornerType>,
	
	/* Border Design */
	pub edge_border_size:		Slice4<size::Simple>,
	pub edge_border_color:		Slice4<Color>,
	pub corner_border_size:		Slice4<size::Simple>,
	pub corner_border_color:	Slice4<Color>,
}

// TODO: move these?
pub type Color = vek::Rgba<Abstract>;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub enum CornerType { #[default] Square, Polygon(u32), Circle }