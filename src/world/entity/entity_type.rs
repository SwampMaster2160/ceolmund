use crate::{render::texture::Texture, world::item::item::Item};

pub enum EntityType {
	Player { inventory: Box<[(Item, u8); 50]>, selected_item: u8 },
}

impl EntityType {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::Player { .. } => Texture::Player,
		}
	}
}