use crate::*;

#[derive(Default, Debug)]
pub struct InputManager {
	/* Mouse Movement */
	pub mouse:				Vec2<Physical>,
	pub old_mouse:			Vec2<Physical>,
	pub mouse_delta:		Vec2<Physical>,
	pub mouse_on_screen:	bool,
	
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

	pub fn register_key_event(&mut self, event: winit::event::KeyEvent) {
		println!("{:?}", event.logical_key);
	}

	// bug somewhere in here
	pub fn set_states(&self, element: &mut element::Element) -> bool {
		if self.mouse_on_screen {
			let self_hovered = Rect::from((element.position, (element.size - Vec2::one()).into())).contains_point(self.mouse);
			let mut child_hovered = false;

			if self_hovered {
				for child in &mut element.children {
					child_hovered |= self.set_states(child);
					if child_hovered { break; }
				}
			}
			
			if !child_hovered {
				element.is_hovered = self_hovered;
			} else {
				element.is_hovered = false;
			}
		}
		element.is_hovered
	}

	pub fn reset_states(&self, element: &mut element::Element) {
		for child in &mut element.children {
			self.reset_states(child);
		}
		element.is_hovered = false;
		element.is_focused = false;
	}
}