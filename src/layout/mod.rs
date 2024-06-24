pub mod geometry;
pub mod vertex;

use indextree::{Arena, NodeId};

use crate::*;
use style::{
	layout::*,
	types::*,
};

pub struct SizeContext {
	parent_size: 		Vec2<Physical>,
	remaining_space:	Vec2<Physical>,
	remaining_children:	Vec2<i32>,
}

pub fn calculate_element_sizes(elements: &Arena<element::Element>, root_id: &NodeId, screen_size: Vec2<Physical>) -> Arena<Vec2<Physical>> {
	let mut sizes = Arena::with_capacity(elements.capacity());
	impl_calculate_element_sizes(&mut sizes, elements, root_id, SizeContext {
		parent_size: screen_size,
		remaining_space: screen_size,
		remaining_children: Vec2::one(),
	});
	sizes
}

pub fn impl_calculate_element_sizes<'a>(sizes: &'a mut Arena<Vec2<Physical>>, elements: &Arena<element::Element>, element_id: &NodeId, context: SizeContext) -> Vec2<Physical> {
	let element_node = elements.get(*element_id).unwrap();
	let current_style = element_node.get().current_style();

	let offset_minimum_width = DPRA::calculate(&current_style.minimum_size.x, (context.parent_size.x, context.remaining_space.x, context.remaining_children.x));
	let offset_minimum_height = DPRA::calculate(&current_style.minimum_size.y, (context.parent_size.y, context.remaining_space.y, context.remaining_children.y));
	let offset_maximum_width = DPRA::calculate(&current_style.maximum_size.x, (context.parent_size.x, context.remaining_space.x, context.remaining_children.x));
	let offset_maximum_height = DPRA::calculate(&current_style.maximum_size.y, (context.parent_size.y, context.remaining_space.y, context.remaining_children.y));
	let offset_ideal_width = DPRA::calculate(&current_style.size.x, (context.parent_size.x, context.remaining_space.x, context.remaining_children.x));
	let offset_ideal_height = DPRA::calculate(&current_style.size.y, (context.parent_size.y, context.remaining_space.y, context.remaining_children.y));
	
	let horizontal_padding = current_style.padding.west_east().map(|v| DP::calculate(v, context.parent_size.x)).sum();
	let vertical_padding = current_style.padding.north_south().map(|v| DP::calculate(v, context.parent_size.y)).sum();
	let padding = Vec2::new(horizontal_padding, vertical_padding);

	let element_children: Vec<NodeId> = element_id.children(&elements).collect();
	let children_count = element_children.len();
	let element_size = sizes.new_node(Vec2::zero());
	
	let auto_size = match current_style.layout_children {
		LayoutChildren::None => Vec2::zero(),
		LayoutChildren::Stacked { gap, column, .. } => {
			let mut total_size: Vec2<Physical> = Vec2::zero();

			let parent_size = Vec2::new(offset_ideal_width, offset_ideal_height) - padding;
			for (child_index, child_id) in element_children.iter().enumerate() {
				let remaining_children = if column {
					Vec2::new(1, (children_count - child_index) as Physical)
				} else {
					Vec2::new((children_count - child_index) as Physical, 1)
				};

				let remaining_space = parent_size - if column {
					(0, ((remaining_children.y - 1) * gap) + total_size.y)
				} else {
					(((remaining_children.x - 1) * gap) + total_size.x, 0)
				};

				let child_size = impl_calculate_element_sizes(sizes, elements, child_id, SizeContext { parent_size, remaining_space, remaining_children });

				if elements.get(*child_id).unwrap().get().current_style().layout_self.in_flow() {
					total_size += child_size;
					if child_index < children_count - 1 {
						total_size += gap;
					}
				}
			}

			total_size
		}
	};

	let minimum_width = offset_minimum_width + (auto_size.x * DPRA::is_auto(&current_style.minimum_size.x) as Physical);
	let minimum_height = offset_minimum_height + (auto_size.y * DPRA::is_auto(&current_style.minimum_size.y) as Physical);
	let maximum_width = offset_maximum_width + (auto_size.x * DPRA::is_auto(&current_style.maximum_size.x) as Physical);
	let maximum_height = offset_maximum_height + (auto_size.y * DPRA::is_auto(&current_style.maximum_size.y) as Physical);
	let ideal_width = offset_ideal_width + (auto_size.x * DPRA::is_auto(&current_style.size.x) as Physical);
	let ideal_height = offset_ideal_height + (auto_size.y * DPRA::is_auto(&current_style.size.y) as Physical);

	let size: Vec2<Physical> = Vec2::clamp(
		Vec2::new(ideal_width, ideal_height),
		Vec2::new(minimum_width, minimum_height),
		Vec2::new(maximum_width, maximum_height),
	);

	*sizes.get_mut(element_size).unwrap().get_mut() = size;
	size
}

pub fn calculate_element_positions(elements: &Arena<element::Element>, root_id: &NodeId, sizes: &Arena<Vec2<Physical>>, size_root_id: &NodeId, current_position: Vec2<Physical>) -> Arena<Vec2<Physical>>