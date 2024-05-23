use super::*;

use glium::{
	backend::Facade,
	VertexBuffer,
};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Geometry {
	pub position: PixelSize,
	pub size: PixelSize,

	pub draw_order: isize,
}

cfg_if::cfg_if! {
	if #[cfg(feature = "glium")] {
		#[derive(Clone, Copy, PartialEq, Default, Debug)]
		pub struct GliumVertex {
			pub position: [f32; 2],
			pub color: [f32; 4],
		}
		glium::implement_vertex!(GliumVertex, position, color);
		
		impl GliumVertex {
			pub fn screenspace(&self, screen_size: PixelSize) -> Self {
				let mut position = Vec2::from(self.position) / screen_size.as_() * 2.0 - 1.0;
				position.y *= -1.0;
				Self {
					position: position.into_array(),
					color: self.color,
				}
			}
		}
		
		impl Geometry {
			pub fn quad(&self) -> [GliumVertex; 6] {
				[
					GliumVertex { position: self.position.as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
					GliumVertex { position: (self.position + (self.size.x, 0)).as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
					GliumVertex { position: (self.position + (0, self.size.y)).as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
		
					GliumVertex { position: (self.position + (self.size.x, 0)).as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
					GliumVertex { position: (self.position + (0, self.size.y)).as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
					GliumVertex { position: (self.position + self.size).as_().into_array(), color: [1.0, 1.0, 1.0, 1.0] },
				]
			}
		
			pub fn create_vertex_buffer<F>(display: &F, screen_size: PixelSize, geometries: &Vec<Geometry>) -> VertexBuffer<GliumVertex>
			where F: ?Sized + Facade{
				let vertices: Vec<GliumVertex> = geometries
					.iter()
					.flat_map(|geometry| {
						geometry.quad()
							.iter()
							.map(|vertex| vertex.screenspace(screen_size))
							.collect::<Vec<GliumVertex>>()
					})
					.collect();
				VertexBuffer::new(display, &vertices).unwrap()
			}
		}
	}
}