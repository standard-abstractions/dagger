pub mod position;
pub mod size;

use crate::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Layout {
	/* Panel Layout */
	pub size:			Vec2<size::Complex>,
	pub minimum_size:	Vec2<size::Simple>,
	pub maximum_size:	Vec2<size::Simple>,
	pub padding:		Slice4<size::Simple>,
	pub margin:			Slice4<size::Simple>,
	
	/* Positioning */
	pub position_self:		position::PositionSelf,
	pub position_children:	position::PositionChildren,
}

pub fn calculate_geometries(
	element:			&mut element::Element,
	screen_size:		Vec2<Physical>,
) {
	impl_calculate_geometries(element, screen_size, screen_size.as_(), Vec2::zero());
}

fn impl_calculate_geometries(
	element:			&mut element::Element,
	parent_size:		Vec2<Physical>,
	share_size:			Vec2<Abstract>,
	position:			Vec2<Physical>,
) {
	let minimum_offset = size::Simple::calculate_vec2(element.layout.minimum_size, parent_size);
	let maximum_offset = size::Simple::calculate_vec2(element.layout.maximum_size, parent_size);
	let ideal_offset = Vec2::clamp(
		size::Complex::calculate_vec2(element.layout.size, parent_size, share_size),
		minimum_offset,
		maximum_offset,
	);

	let padding = Vec2::new(
		element.layout.padding.west().calculate(parent_size.x) + element.layout.padding.east().calculate(parent_size.x),
		element.layout.padding.north().calculate(parent_size.y) + element.layout.padding.south().calculate(parent_size.y),
	);
	let margin = Vec2::new(
		element.layout.margin.west().calculate(parent_size.x) + element.layout.margin.east().calculate(parent_size.x),
		element.layout.margin.north().calculate(parent_size.y) + element.layout.margin.south().calculate(parent_size.y),
	);

	let mut next_position = position + Vec2::new(
		element.layout.padding.west().calculate(parent_size.x),
		element.layout.padding.north().calculate(parent_size.y),
	);
	
	let children_count = element.children.len();
	let children_offset = match element.layout.position_children {
		position::PositionChildren::None => todo!(),
		position::PositionChildren::Stacked { gap, column, .. } => {
			let mut total_space_reserved: Vec2<Physical> = Vec2::zero();

			let growth_rates: Vec2<Abstract> = element.children.iter().map(|c| c.layout.size.map(|n| n.growth)).sum();
			for (index, child) in element.children.iter_mut().enumerate() {
				let share_size = parent_size.as_::<Abstract>() / growth_rates;

				impl_calculate_geometries(child, ideal_offset - padding, share_size, next_position);

				if child.layout.position_self.is_in_flow() {
					total_space_reserved += child.size;
				}

				if column {
					next_position.y += child.size.y;
					next_position.y += gap;
				} else {
					next_position.x += child.size.x;
					next_position.x += gap;
				}
			}

			total_space_reserved
		},
	};

	element.size = Vec2::clamp(ideal_offset, minimum_offset, maximum_offset);
	element.reserve = element.size + margin;
	element.position = position;
}