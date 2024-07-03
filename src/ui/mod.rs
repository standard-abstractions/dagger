pub mod backend;

use crate::*;

#[derive(Default, Debug)]
pub struct UI<State: Default> {
	input_manager:	input::InputManager,
	state_manager:	state::StateManager<State>,

	elements:		Vec<()>,
}

impl<State: Default> UI<State> {
	pub fn new<F>(builder: F) -> Self
	where F: FnOnce(&mut Self) {
		let mut res = Self::default();
		builder(&mut res);
		res
	}

	pub fn run(mut self) -> DaggerResult<()> {
		let backend = backend::Backend::default();
		backend.run(&mut self)?;
		Ok(())
	}
}