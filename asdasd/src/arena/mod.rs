#[derive(Clone, PartialEq, Debug)]
pub struct ArenaNode<T> {
	pub data: T,
	pub children: Vec<usize>,
}
impl<T> ArenaNode<T> {
	pub fn new(data: T) -> Self { Self { data, children: vec![] } }
	pub fn add(&mut self, id: usize) { self.children.push(id); }
}
impl<T> std::ops::Deref for ArenaNode<T> { type Target = T; fn deref(&self) -> &Self::Target { &self.data } }
impl<T> std::ops::DerefMut for ArenaNode<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data } }
pub type Arena<T> = Vec<ArenaNode<T>>;