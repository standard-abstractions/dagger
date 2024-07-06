pub mod element;
pub mod layout;
pub mod input;
pub mod ui;

pub use vek::{Vec2, Vec3, Vec4, Rect, Clamp};

pub type DaggerResult<T> = Result<T, String>;

pub type Abstract = f32;
pub type Physical = i32;

// TODO: remove
#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Slice4<T>(Vec4<T>);
impl<T:> Slice4<T> {
	pub fn new(north: T, east: T, south: T, west: T) -> Self { Self(Vec4::new(north, east, south, west)) }
	pub fn broadcast(value: T) -> Self where T: Clone { Self::new(value.clone(), value.clone(), value.clone(), value.clone()) }

	pub fn north(&self) -> &T { &self.0.x }
	pub fn east(&self) -> &T { &self.0.y }
	pub fn south(&self) -> &T { &self.0.z }
	pub fn west(&self) -> &T { &self.0.w }
	pub fn north_east(&self) -> &T { &self.0.x }
	pub fn south_east(&self) -> &T { &self.0.y }
	pub fn south_west(&self) -> &T { &self.0.z }
	pub fn north_west(&self) -> &T { &self.0.w }
}
impl<T> From<Vec4<T>> for Slice4<T> { fn from(value: Vec4<T>) -> Self { Self(value) } }
impl<T: Clone> From<&Vec4<T>> for Slice4<T> { fn from(value: &Vec4<T>) -> Self { Self(value.clone()) } }
impl<T> std::ops::Deref for Slice4<T> { type Target = Vec4<T>; fn deref(&self) -> &Self::Target { &self.0 } }
impl<T> std::ops::DerefMut for Slice4<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

fn main() {
	let mut bg = element::Element::default();
	bg.layout.size = Vec2::broadcast(layout::size::Complex::percent(100.0));
	bg.layout.maximum_size = Vec2::broadcast(layout::size::Simple::pixels(Physical::MAX));
	bg.style.normal.color = element::style::Color::black();
	bg.layout.position_children = layout::position::PositionChildren::Stacked { alignment: layout::position::Alignment::Start, gap: 1, column: true };

	let mut nav = element::Element::default();
	nav.layout.size = Vec2::new(layout::size::Complex::percent(100.0), layout::size::Complex::pixels(32));
	nav.layout.maximum_size = Vec2::broadcast(layout::size::Simple::pixels(Physical::MAX));
	nav.style.normal.color = element::style::Color::white();
	nav.style.hovered = Some(element::style::Style {
		color: element::style::Color::cyan(),
		..Default::default()
	});
	bg.children.push(nav);

	let mut not_nav = element::Element::default();
	not_nav.layout.size = Vec2::new(layout::size::Complex::percent(100.0), layout::size::Complex::growth(1.0));
	not_nav.layout.maximum_size = Vec2::broadcast(layout::size::Simple::pixels(Physical::MAX));
	not_nav.layout.position_children = layout::position::PositionChildren::Stacked { alignment: layout::position::Alignment::Start, gap: 1, column: false };
	
	let mut side = element::Element::default();
	side.layout.size = Vec2::new(layout::size::Complex::pixels(256), layout::size::Complex::growth(1.0));
	side.layout.maximum_size = Vec2::broadcast(layout::size::Simple::pixels(Physical::MAX));
	side.style.normal.color = element::style::Color::white();
	side.style.hovered = Some(element::style::Style {
		color: element::style::Color::cyan(),
		..Default::default()
	});
	not_nav.children.push(side);
	
	let mut content = element::Element::default();
	content.layout.size = Vec2::new(layout::size::Complex::growth(1.0), layout::size::Complex::growth(1.0));
	content.layout.maximum_size = Vec2::broadcast(layout::size::Simple::pixels(Physical::MAX));
	content.style.normal.color = element::style::Color::white();
	content.style.hovered = Some(element::style::Style {
		color: element::style::Color::cyan(),
		..Default::default()
	});
	not_nav.children.push(content);

	bg.children.push(not_nav);

	let ui = ui::UI::<()>::new(|ui| {
		ui.root(bg);
	});

	ui.run().expect("Failed to run UI!");
}