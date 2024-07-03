use winit::event::KeyEvent;

use crate::*;

#[derive(Default, Debug)]
pub struct InputManager {
	/* Mouse Movement */
	pub mouse:			Vec2<Physical>,
	pub old_mouse:		Vec2<Physical>,
	pub mouse_delta:	Vec2<Physical>,
	
	/* Mouse Buttons */

	
	/* Keys */
}

impl InputManager {
	pub fn start(&mut self) {

	}

	pub fn before_frame(&mut self) {

	}

	pub fn after_frame(&mut self) {
		self.mouse_delta = Vec2::zero();
	}

	pub fn end(&mut self) {

	}

	pub fn register_mouse_position(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
		self.old_mouse = self.mouse;
		self.mouse = Vec2::new(position.x, position.y).as_();
		self.mouse_delta = self.old_mouse - self.mouse;
	}

	pub fn register_key_event(&mut self, event: KeyEvent) {
		println!("{:?}", event.logical_key);
	}
}