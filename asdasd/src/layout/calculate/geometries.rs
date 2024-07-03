use crate::*;
use arena::*;
use calculate::*;
use layout::*;

pub fn calculate_geometries(
	elements:		&Arena<element::Element>,
	screen_size:	Vec2<Physical>
) -> Vec<geometry::Geometry> {
	let sizes = sizes::calculate_sizes(elements, screen_size);
	let positions = positions::calculate_positions(elements, &sizes);

	let mut geometries = Vec::with_capacity(elements.len());
	impl_calculate_geometries(&mut geometries, elements, &sizes, &positions, 0);
	geometries
}

fn impl_calculate_geometries(
	geometries:	&mut Vec<geometry::Geometry>,
	elements:	&Arena<element::Element>,
	sizes:		&Vec<(Vec2<Physical>, Vec2<Physical>)>,
	positions:	&Vec<Vec2<Physical>>,
	node_id:	usize,
) {
	let node_element = &elements[node_id];	
	let element = &node_element.data;

	let size = sizes[node_id].1;
	let position = positions[node_id];
	let style = element.current_style();

	let geometry_index = geometries.len();

	for &child_id in &node_element.children {
		impl_calculate_geometries(geometries, elements, sizes, positions, child_id);
	}

	geometries.insert(geometry_index, geometry::Geometry {
		position,
		size,
		color: style.color,
		background_id: style.background_id,
		corner_size: Slice4::new(
			Vec2::new(DP::calculate(&style.corner_size.north().x, size.x), DP::calculate(&style.corner_size.north().y, size.y)),
			Vec2::new(DP::calculate(&style.corner_size.east().x, size.x), DP::calculate(&style.corner_size.east().y, size.y)),
			Vec2::new(DP::calculate(&style.corner_size.south().x, size.x), DP::calculate(&style.corner_size.south().y, size.y)),
			Vec2::new(DP::calculate(&style.corner_size.west().x, size.x), DP::calculate(&style.corner_size.west().y, size.y)),
		),
		corner_type: style.corner_type,
		edge_border_thickness: Slice4::new(
			DP::calculate(&style.edge_border_thickness.north(), size.y),
			DP::calculate(&style.edge_border_thickness.east(), size.x),
			DP::calculate(&style.edge_border_thickness.south(), size.y),
			DP::calculate(&style.edge_border_thickness.west(), size.x),
		),
		edge_border_color: style.edge_border_color,
		corner_border_thickness: Slice4::new(
			DP::calculate(&style.corner_border_thickness.north(), size.y),
			DP::calculate(&style.corner_border_thickness.east(), size.x),
			DP::calculate(&style.corner_border_thickness.south(), size.y),
			DP::calculate(&style.corner_border_thickness.west(), size.x),
		),
		corner_border_color: style.corner_border_color,
	});
}
