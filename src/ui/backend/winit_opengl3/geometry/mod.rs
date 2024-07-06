pub mod vertex;

use crate::*;
use element::vertex::Vertex as ElementVertex;

pub fn create_vertex_buffer<F>(display: &F, element_vertices: &Vec<ElementVertex>, screen_size: Vec2<Physical>) -> glium::VertexBuffer<vertex::GliumVertex>
where F: ?Sized + glium::backend::Facade {
	let mut vertices: Vec<vertex::GliumVertex> = Vec::with_capacity(element_vertices.len() * 6);
	for vertex in element_vertices {
		vertices.push(vertex::GliumVertex::from(*vertex).to_screen_space(screen_size));
	}
	glium::VertexBuffer::new(display, &vertices).unwrap()
}