pub mod stacked;

use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Scheme {
	/* No Layout Scheme - Do not display */
	#[default]
	None,
	
	/* Positioned Layout Scheme */
	// Positioned {

	// },
	
	/* Stacked Layout Scheme */
	Stacked {
		fit_to_content: Vec2<bool>,
		ideal_size: stacked::Size,
		optional_minimum_size: Option<stacked::Size>,
		optional_maximum_size: Option<stacked::Size>,
		horizontal: bool,
	},
}