pub mod arena;
pub mod element;
pub mod input;
pub mod layout;
pub mod style;
pub mod ui;

pub mod opengl;

pub use vek::{Vec2, Vec3, Vec4, Clamp};

use arena::*;
use style::types::*;

pub type Abstract = f32;
pub type Physical = i32;

fn main() {
	let mut content = element::Element::default();
	content.styles.normal = content.styles.normal.with_size(Vec2::new(vec![DPRA::Distance(32)], vec![DPRA::Percent(50.0)]));
	content.styles.normal = content.styles.normal.with_color(Color::white());

	let mut ui = ui::UI::new_with((), move |ui| {
		ui.element(content);
	});
	ui.run().expect("Failed to run UI!");
}