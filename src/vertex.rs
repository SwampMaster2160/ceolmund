#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: [f32; 2],
	pub texture_position: [f32; 2],
	pub color: [f32; 4],
}

impl Vertex {
	pub fn new_null() -> Self {
		Self {
			position: [0., 0.], texture_position: [0., 0.], color: [0., 0., 0., 0.]
		}
	}
}

glium::implement_vertex!(Vertex, position, texture_position, color);