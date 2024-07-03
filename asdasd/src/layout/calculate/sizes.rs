use crate::*;
use arena::*;
use style::layout;

pub fn calculate_sizes(
	elements:		&Arena<element::Element>,
	screen_size:	Vec2<Physical>,
) -> Vec<(Vec2<Physical>, Vec2<Physical>)> {
	let mut sizes = Vec::with_capacity(elements.len());
	impl_calculate_sizes(elements, &mut sizes, 0, screen_size, screen_size, Vec2::one());
	sizes
}

fn impl_calculate_sizes(
	elements:			&Arena<element::Element>,
	sizes:				&mut Vec<(Vec2<Physical>, Vec2<Physical>)>,
	node_id:			usize,
	parent_size:		Vec2<Physical>,
	remaining_space:	Vec2<Physical>,
	remaining_children:	Vec2<i32>,
) {
	let node_element = &elements[node_id];
	let element = &node_element.data;

	let style = element.current_style();

	let minimum_width_offset = DPRA::calculate(&style.minimum_size.x, (parent_size.x, remaining_space.x, remaining_children.x));
	let minimum_height_offset = DPRA::calculate(&style.minimum_size.y, (parent_size.y, remaining_space.y, remaining_children.y));
	let maximum_width_offset = DPRA::calculate(&style.maximum_size.x, (parent_size.x, remaining_space.x, remaining_children.x));
	let maximum_height_offset = DPRA::calculate(&style.maximum_size.y, (parent_size.y, remaining_space.y, remaining_children.y));
	let ideal_width_offset = DPRA::calculate(&style.size.x, (parent_size.x, remaining_space.x, remaining_children.x));
	let ideal_height_offset = DPRA::calculate(&style.size.y, (parent_size.y, remaining_space.y, remaining_children.y));

	let padding = Vec2::new(
		style.padding.west_east().map(|pad| DP::calculate(pad, parent_size.x)).sum(),
		style.padding.north_south().map(|pad| DP::calculate(pad, parent_size.y)).sum(),
	);
	let margin = Vec2::new(
		style.margin.west_east().map(|pad| DP::calculate(pad, parent_size.x)).sum(),
		style.margin.north_south().map(|pad| DP::calculate(pad, parent_size.y)).sum(),
	);

	let element_size_index = sizes.len();
	sizes.push((Vec2::zero(), Vec2::zero()));

	let children_count = node_element.children.len();
	let children_offset = match style.layout_children {
		layout::LayoutChildren::None => Vec2::zero(),
		layout::LayoutChildren::Stacked { gap, column, .. } => {
			let mut total_space_reserved = Vec2::zero();

			let child_parent_size = Vec2::new(ideal_width_offset, ideal_height_offset) - padding;
			for (child_index, &child_id) in node_element.children.iter().enumerate() {
				let child_remaining_children = if column {
					Vec2::new(1, (children_count - child_index) as Physical)
				} else {
					Vec2::new((children_count - child_index) as Physical, 1)
				};

				let child_remaining_space = parent_size - if column {
					(0, ((remaining_children.y - 1) * gap) + total_space_reserved.y)
				} else {
					(((remaining_children.x - 1) * gap) + total_space_reserved.x, 0)
				};

				impl_calculate_sizes(elements, sizes, child_id, child_parent_size, child_remaining_space, child_remaining_children);
				
				if elements[child_id].current_style().layout_self.in_flow() {
					total_space_reserved += sizes[child_id].0;
					if child_index < children_count - 1 {
						total_space_reserved += gap;
					}
				}
			}

			total_space_reserved
		},
	};

	let minimum_width = (DPRA::is_auto(&style.minimum_size.x) as Physical * children_offset.x) + minimum_width_offset;
	let minimum_height = (DPRA::is_auto(&style.minimum_size.y) as Physical * children_offset.y) + minimum_height_offset;
	let maximum_width = (DPRA::is_auto(&style.maximum_size.x) as Physical * children_offset.x) + maximum_width_offset;
	let maximum_height = (DPRA::is_auto(&style.maximum_size.y) as Physical * children_offset.y) + maximum_height_offset;
	let ideal_width = (DPRA::is_auto(&style.size.x) as Physical * children_offset.x) + ideal_width_offset;
	let ideal_height = (DPRA::is_auto(&style.size.y) as Physical * children_offset.y) + ideal_height_offset;

	let element_size = Vec2::clamp(
		Vec2::new(ideal_width, ideal_height),
		Vec2::new(minimum_width, minimum_height),
		Vec2::new(maximum_width, maximum_height),
	);

	sizes[element_size_index] = (element_size + margin, element_size);
}
