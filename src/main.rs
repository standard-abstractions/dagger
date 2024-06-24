pub mod element;
pub mod layout;
pub mod style;
pub mod ui;
pub mod window;

pub use vek::{Vec2, Vec3, Vec4, Clamp};

use crate::style::types::DPRA;

pub type Abstract = f32;
pub type Physical = i32;

fn main() {
	let mut elements = indextree::Arena::new();
	let mut root_element = element::Element::default();
	root_element.styles.normal = root_element.styles.normal.with_size(Vec2::new(vec![DPRA::Remaining(1.0)], vec![DPRA::Remaining(1.0)]));
	let root_id = elements.new_node(root_element);

	let s = layout::calculate_element_sizes(&elements, &root_id, Vec2::new(1156, 768));
    println!("{:?}", s);
}