use crate::*;

#[derive(Default, Debug)]
pub struct UI<STATE> {
	app_input: input::InputManager,
	app_state: STATE,
	
	elements: Arena<element::Element>,

	#[feature("opengl")]
	raw_ui: opengl::ui::UIOpenGL,
}

impl<STATE> UI<STATE> {
	pub fn new(state: STATE) -> Self {
		Self {
			app_input: input::InputManager::default(),
			app_state: state,

			elements: vec![],

			raw_ui: if cfg!(feature = "opengl") {
				opengl::ui::UIOpenGL::default()
			} else {
				panic!("No backend!")
			},
		}
	}
	pub fn new_with<F: FnOnce(&mut Self)>(state: STATE, callback: F) -> Self { let mut res = Self::new(state); callback(&mut res); res }
	pub fn with<F: FnOnce(&mut Self)>(&mut self, callback: F) { callback(self); }

	pub fn element(&mut self, element: element::Element) { self.elements.push(ArenaNode::new(element)); }
	pub fn component<COMPONENT: element::Component>(&mut self, component: COMPONENT) { COMPONENT::add(component, self); }

	pub fn run(mut self) -> Result<(), &'static str> {
		self.start()?;
		if cfg!(feature = "opengl") {
			self.raw_ui.run(&mut self.app_input)
		} else {
			panic!("No backend!")
		}

		Ok(())
	}

	pub(crate) fn start(mut self) -> Result<(), &'static str> {
	}

	pub(crate) fn frame(mut self) -> Result<(), &'static str> {
		todo!()
	}
}