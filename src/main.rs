pub mod input;
pub mod state;
pub mod ui;

pub use vek::{Vec2, Vec3, Vec4, Clamp};

pub type DaggerResult<T> = Result<T, String>;

pub type Abstract = f32;
pub type Physical = i32;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct AppState {
	
}

fn main() {
	let ui = ui::UI::<AppState>::new(|ui| {
		
	});
	ui.run().expect("Failed to run UI!");
}