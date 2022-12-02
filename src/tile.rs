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

	pub fn render(&self, pos: [i64; 2]) -> Vec<Vertex> {
		match self {
			_ => self.get_texture().to_tris(pos, [0, 0]).to_vec()
		}
	}
}