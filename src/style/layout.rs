use crate::*;
use super::types::*;

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum LayoutSelf {
	None,
	Flow {
		alignment:	Option<Alignment>,
	},
	Positioned {
		position:			Vec2<Physical>,
		relative_to_parent:	bool,
	},
}
impl LayoutSelf {
	pub fn in_flow(&self) -> bool {
		match self {
			Self::Flow { .. } => true,
			_ => false,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum LayoutChildren {
	None,
	Stacked {
		alignment:	Alignment,
		gap:		Physical,
		column:		bool,
	},
}