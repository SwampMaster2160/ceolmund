use crate::{vertex::Vertex, texture::Texture};

#[derive(Clone)]
pub enum Tile {
	Grass,
	Water,
}

impl Tile {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::Grass => Texture::Grass,
			Self::Water => Texture::Water,
		}
	}

	pub fn render(&self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		match self {
			_ => vertices_in_out.extend(self.get_texture().to_tris(pos, [0, 0])),
		}
	}
}