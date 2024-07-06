use crate::*;

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Simple {
	pub pixels:		Physical,
	pub percent:	Abstract,
}
impl Simple {
	pub fn new(pixels: Physical, percent: Abstract) -> Self { Self { pixels, percent } }
	pub fn pixels(pixels: Physical) -> Self { Self { pixels, percent: 0.0 } }
	pub fn percent(percent: Abstract) -> Self { Self { pixels: 0, percent } }

	pub fn calculate(&self, parent_size: Physical) -> Physical {
		(self.percent * (parent_size as Abstract / 100.0)).ceil() as Physical + self.pixels
	}
	
	pub fn calculate_vec2(size: Vec2<Simple>, parent_size: Vec2<Physical>) -> Vec2<Physical> {
		Vec2::new(
			size.x.calculate(parent_size.x),
			size.y.calculate(parent_size.y),
		)
	}
}

#[derive(Clone, Copy, PartialEq, Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct Complex {
	pub pixels:		Physical,
	pub percent:	Abstract,
	pub growth:		Abstract,
}
impl Complex {
	pub fn new(pixels: Physical, percent: Abstract, growth: Abstract) -> Self { Self { pixels, percent, growth } }
	pub fn pixels(pixels: Physical) -> Self { Self { pixels, percent: 0.0, growth: 0.0 } }
	pub fn percent(percent: Abstract) -> Self { Self { pixels: 0, percent, growth: 0.0 } }
	pub fn growth(growth: Abstract) -> Self { Self { pixels: 0, percent: 0.0, growth } }

	pub fn calculate(&self, parent_size: Physical, share_size: Abstract) -> Physical {
		(share_size as Abstract * self.growth).ceil() as Physical + 
		(self.percent * (parent_size as Abstract / 100.0)).ceil() as Physical +
		self.pixels
	}
	
	pub fn calculate_vec2(size: Vec2<Complex>, parent_size: Vec2<Physical>, share_size: Vec2<Abstract>) -> Vec2<Physical> {
		Vec2::new(
			size.x.calculate(parent_size.x, share_size.x),
			size.y.calculate(parent_size.y, share_size.y),
		)
	}
} 