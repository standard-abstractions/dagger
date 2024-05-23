use super::*;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Size {
	pub pixels: PixelSize,
	pub parent_percentage: FloatSize,
	pub remaining_share: FloatSize,
}

impl Size {
	pub fn calculate(&self, context: SizeCalculationContext) -> FloatSize {
		((context.remaining_space / context.remaining_children.as_()) * self.remaining_share) +
		((self.parent_percentage / 100.0) * context.parent_size) +
		self.pixels.as_()
	}
}