use crate::*;

pub type Color = vek::Rgba<Abstract>;

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum CornerType { Square, Polygon(u32), Circle }

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum Alignment { Start, Center, End }

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Slice4<T>(Vec4<T>);
impl<T:> Slice4<T> {
	pub fn new(north: T, east: T, south: T, west: T) -> Self { Self(Vec4::new(north, east, south, west)) }
	pub fn broadcast(value: T) -> Self where T: Clone { Self::new(value.clone(), value.clone(), value.clone(), value.clone()) }

	pub fn north(&self) -> &T { &self.0.x }
	pub fn east(&self) -> &T { &self.0.y }
	pub fn south(&self) -> &T { &self.0.z }
	pub fn west(&self) -> &T { &self.0.w }
	pub fn north_east(&self) -> &T { &self.0.x }
	pub fn south_east(&self) -> &T { &self.0.y }
	pub fn south_west(&self) -> &T { &self.0.z }
	pub fn north_west(&self) -> &T { &self.0.w }
	pub fn north_south(&self) -> Vec2<&T> { Vec2::new(&self.0.x, &self.0.z) }
	pub fn south_north(&self) -> Vec2<&T> { Vec2::new(&self.0.z, &self.0.x) }
	pub fn east_west(&self) -> Vec2<&T> { Vec2::new(&self.0.y, &self.0.w) }
	pub fn west_east(&self) -> Vec2<&T> { Vec2::new(&self.0.w, &self.0.y) }
}
impl<T> From<Vec4<T>> for Slice4<T> { fn from(value: Vec4<T>) -> Self { Self(value) } }
impl<T> std::ops::Deref for Slice4<T> { type Target = Vec4<T>; fn deref(&self) -> &Self::Target { &self.0 } }
impl<T> std::ops::DerefMut for Slice4<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum DP {
	Distance(Physical),
	Percent(Abstract),
}
impl DP {
	pub fn calculate(values: &Vec<Self>, context: Physical) -> Physical {
		let mut result = 0;
		for value in values {
			match value {
				Self::Distance(distance) => result += distance,
				Self::Percent(percent) => result += (percent * (context as Abstract / 100.0)) as Physical,
			}
		}
		result
	}
}
pub type SizeDP = Vec<DP>;

#[derive(Clone, Copy, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
pub enum DPRA {
	Distance(Physical),
	Percent(Abstract),
	Remaining(Abstract),
	Auto,
}
impl DPRA {
	pub fn is_auto(values: &Vec<Self>) -> bool { values.iter().any(|v| match v { DPRA::Auto => true, _ => false}) }
	pub fn calculate(values: &Vec<Self>, context: (Physical, Physical, i32)) -> Physical {
		let mut result = 0;
		for value in values {
			match value {
				Self::Distance(distance) => result += distance,
				Self::Percent(percent) => result += (percent * (context.0 as Abstract / 100.0)) as Physical,
				Self::Remaining(remaining) => result += (remaining * (context.1 / context.2) as Abstract) as Physical,
				Self::Auto => result += 0,
			}
		}
		result
	}
}
pub type SizeDPRA = Vec<DPRA>;