use crate::*;

#[derive(Default, Debug)]
pub struct StateManager<State> {
	state: State,
}

impl<State> StateManager<State> {
	pub fn start(&mut self) {

	}

	pub fn before_frame(&mut self) {
		
	}

	pub fn after_frame(&mut self) {
		
	}

	pub fn end(&mut self) {

	}
}