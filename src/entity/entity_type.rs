use crate::texture::Texture;

pub enum EntityType {
	Player
}

impl EntityType {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::Player => Texture::Player,
		}
	}
}