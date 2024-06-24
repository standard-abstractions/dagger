use crate::*;
use style::*;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Element {
	pub styles: Stylesheet,
}

impl Element {
	pub fn current_style(&self) -> &attributes::StyleAttributes {
		&self.styles.normal
	}
}