use crate::*;
use super::vertex::*;
use style::types::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geometry {
	position:			Vec2<Physical>,
	size:				Vec2<Physical>,
	color:				Color,
	background_id:	Option<u32>,

	corner_size:	Slice4<Physical>,
	corner_type:	Slice4<CornerType>,

	edge_border_thickness:		Slice4<Physical>,
	edge_border_color:			Slice4<Color>,
	corner_border_thickness:	Slice4<Physical>,
	corner_border_color:		Slice4<Color>,
}
impl Geometry {
	pub fn simple_quad_vertices(&self) -> [Vertex;6] {
		[
			Vertex::new(self.position, self.color, self.background_id, [0.0, 0.0]),
			Vertex::new(self.position + (0, self.size.y), self.color, self.background_id, [0.0, 1.0]),
			Vertex::new(self.position + self.size, self.color, self.background_id, [1.0, 1.0]),

			Vertex::new(self.position, self.color, self.background_id, [0.0, 0.0]),
			Vertex::new(self.position + (self.size.x, 0), self.color, self.background_id, [1.0, 0.0]),
			Vertex::new(self.position + self.size, self.color, self.background_id, [1.0, 1.0]),
		]
	}

	pub fn vertices(&self) -> Vec<Vertex> {
		todo!()
	}
}

pub fn create_simple_quad_vertex_buffer<F>(display: &F, geometries: &Vec<Geometry>, screen_size: Vec2<Physical>) -> glium::VertexBuffer<GliumVertex>
where F: ?Sized + glium::backend::Facade {
	let mut vertices: Vec<GliumVertex> = Vec::with_capacity(geometries.len() * 6);
	for geometry in geometries {
		for vertex in geometry.simple_quad_vertices().iter() {
			vertices.push(GliumVertex::from(*vertex).to_screen_space(screen_size));
		}
	}
	glium::VertexBuffer::new(display, &vertices).unwrap()
}

pub fn create_vertex_buffer<F>(display: &F, geometries: &Vec<Geometry>, screen_size: Vec2<Physical>) -> glium::VertexBuffer<GliumVertex>
where F: ?Sized + glium::backend::Facade {
	let mut vertices: Vec<GliumVertex> = Vec::with_capacity(geometries.len() * 6);
	for geometry in geometries {
		let geometry_vertices = geometry.vertices();
		vertices.reserve(geometry_vertices.len());
		for vertex in geometry_vertices.iter() {
			vertices.push(GliumVertex::from(*vertex).to_screen_space(screen_size));
		}
	}
	glium::VertexBuffer::new(display, &vertices).unwrap()
}