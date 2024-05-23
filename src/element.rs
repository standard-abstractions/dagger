use dagger_layout::Layout;
use dagger_macros::Layout;

#[derive(Clone, PartialEq, Debug, Layout)]
pub struct Element {
	#[layout] pub layout: Layout,
	#[children] pub children: Vec<Element>,
}