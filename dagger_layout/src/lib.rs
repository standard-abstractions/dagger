pub mod geometry;
pub mod scheme;

use geometry::*;
use scheme::*;

use itertools::Itertools;
use vek::*;

type PixelSize = Vec2<isize>;
type FloatSize = Vec2<f32>;

pub trait CalculateLayout: Sized + Clone {
	fn get_layout(&self) -> &Layout;
	fn get_children(&self) -> &Vec<Self>;
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Layout {
	pub scheme: Scheme,

	pub resolve_priority: isize,
	pub draw_priority: isize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SizeCalculationContext {
	pub parent_size: FloatSize,
	pub remaining_space: FloatSize,
	pub remaining_children: Vec2<usize>,
}

#[derive(Clone, PartialEq, Debug)]
struct LayoutTree {
	pub layout: Layout,
	pub children: Vec<LayoutTree>,
}

impl Layout {
	pub fn calculate<T: CalculateLayout>(element: &T, context: SizeCalculationContext, position: FloatSize) -> Vec<Geometry> {
		let tree = Self::create_tree(element);
		let sizes = Self::calculate_sizes_from_tree(&tree, context);
		let mut geometries = Self::calculate_positions_from_tree(&tree, &sizes, 0, position);
		geometries.sort_by_key(|g| g.draw_order);
		geometries
	}

	fn create_tree<T: CalculateLayout>(element: &T) -> LayoutTree {
		LayoutTree {
			layout: *element.get_layout(),
			children: element.get_children().iter().map(|e| Self::create_tree(e)).collect(),
		}
	}

	fn calculate_sizes_from_tree(tree: &LayoutTree, context: SizeCalculationContext) -> Vec<PixelSize> {
		match tree.layout.scheme {
			Scheme::None => vec![],
			Scheme::Stacked { fit_to_content, ideal_size, optional_minimum_size, optional_maximum_size, horizontal } => {
				let mut res = Vec::with_capacity(tree.children.len() + 1);

				// Calculate initial size
				let mut size = ideal_size.calculate(context);
				if let Some(minimum_size) = optional_minimum_size {
					size = Vec2::partial_max(size, minimum_size.calculate(context));
				}
				if let Some(maximum_size) = optional_maximum_size {
					size = Vec2::partial_min(size, maximum_size.calculate(context));
				}
				
				let fit_to_content_factor = fit_to_content.as_::<usize>().as_();

				// Calculate children size
				let children_count = tree.children.len();
				let mut total_children_size = FloatSize::zero();

				let mut children_ids_sorted_by_resolve_priority: Vec<usize> = (0..children_count).collect();
				children_ids_sorted_by_resolve_priority.sort_by_key(|&i| tree.children[i].layout.resolve_priority);

				for (index, &child_id) in children_ids_sorted_by_resolve_priority.iter().enumerate() {
					let child = &tree.children[child_id];
					let res_current_child_index = res.len();

					let child_context = SizeCalculationContext {
						parent_size: size * fit_to_content_factor,
						remaining_space: if horizontal {
							size - (total_children_size.x, 0.0)
						} else {
							size - (0.0, total_children_size.y)
						},
						remaining_children: if horizontal {
							Vec2::new(children_count - index, 1)
						} else {
							Vec2::new(1, children_count - index)
						},
					};

					res.extend(Self::calculate_sizes_from_tree(child, child_context));

					if let Scheme::Stacked { .. } = child.layout.scheme {
						total_children_size += res[res_current_child_index].as_();
					}
				}

				size += total_children_size * fit_to_content_factor;

				res.insert(0, size.as_());
				res
			},
		}
	}

	fn calculate_positions_from_tree(tree: &LayoutTree, sizes: &Vec<PixelSize>, current_index: usize, position: FloatSize) -> Vec<Geometry> {
		match tree.layout.scheme {
			Scheme::None => vec![],
			Scheme::Stacked { horizontal, .. } => {
				let mut res = vec![];
		
				let mut next_child_position = FloatSize::zero();
				for child in tree.children.iter() {
					let res_current_child_index = res.len();

					res.extend(Self::calculate_positions_from_tree(
						child,
						&sizes,
						current_index + res_current_child_index + 1,
						position + next_child_position,
					));

					if horizontal {
						next_child_position += (sizes[res_current_child_index + current_index + 1].as_::<f32>().x, 0.0)
					} else {
						next_child_position += (0.0, sizes[res_current_child_index + current_index + 1].as_::<f32>().y as f32)
					}
				}

				res.insert(0, Geometry {
					position: position.as_(),
					size: sizes[current_index],
					draw_order: tree.layout.draw_priority,
				});

				res
			},
		}
	}
}
