use crate::*;
use arena::*;
use style::layout;

pub fn calculate_positions(
	elements:	&Arena<element::Element>,
	sizes:		&Vec<(Vec2<Physical>, Vec2<Physical>)>,
) -> Vec<Vec2<Physical>> {
	let mut positions = Vec::with_capacity(elements.len());
	impl_calculate_positions(elements, sizes, &mut positions, 0, Vec2::zero());
	positions
}

fn impl_calculate_positions(
	elements:			&Arena<element::Element>,
	sizes:				&Vec<(Vec2<Physical>, Vec2<Physical>)>,
	positions:			&mut Vec<Vec2<Physical>>,
	node_id:			usize,
	current_position:	Vec2<Physical>,
) {
	positions.push(current_position);

	let node_element = &elements[node_id];
	let element = &node_element.data;
	let element_size = sizes[node_id].1;

	let style = element.current_style();

	match style.layout_children {
		layout::LayoutChildren::None => {},
		layout::LayoutChildren::Stacked { alignment, gap, column } => {
			let mut next_position = Vec2::new(
				DP::calculate(style.padding.west(), element_size.x),
				DP::calculate(style.padding.north(), element_size.y),
			);

			let children_count = node_element.children.len();
			for (child_index, &child_id) in node_element.children.iter().enumerate() {
				impl_calculate_positions(elements, sizes, positions, child_id, current_position + next_position);

				if column {
					next_position += (0, sizes[child_id].0.y);
					if child_index < children_count {
						next_position.y += gap;
					}
				} else {
					next_position += (sizes[child_id].0.x, 0);
					if child_index < children_count {
						next_position.x += gap;
					}
				}
			}
		},
	}
}