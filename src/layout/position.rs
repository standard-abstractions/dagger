use crate::*;

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum PositionSelf {
	None,
	Flow {
		alignment:	Option<Alignment>,
	},
	Positioned {
		position:			Vec2<Physical>,
		relative_to_parent:	bool,
	},
}
impl PositionSelf {
	pub fn is_in_flow(&self) -> bool {
		match self {
			Self::Flow { .. } => true,
			_ => false,
		}
	}
}
impl Default for PositionSelf {
	fn default() -> Self {
		Self::Flow { alignment: Default::default() }
	}
}

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum PositionChildren {
	None,
	Stacked {
		alignment:	Alignment,
		gap:		Physical,
		column:		bool,
	},
}
impl Default for PositionChildren {
	fn default() -> Self {
		Self::Stacked { alignment: Default::default(), gap: 0, column: false }
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub enum Alignment { #[default] Start, Center, End }