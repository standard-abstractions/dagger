
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct InputManager {

}

impl InputManager {
	pub fn register_winit_keyboard_event(&mut self, event: winit::event::KeyEvent) {
		match event.state {
			winit::event::ElementState::Pressed => println!("Keypress: {:?}", event.logical_key),
			winit::event::ElementState::Released => println!("Key Release: {:?}", event.logical_key),
		}
	}
}