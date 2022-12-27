use std::collections::HashMap;

use crate::{render::texture::Texture, world::item::item::Item};

use strum::{IntoEnumIterator};
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(EntityVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
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

impl EntityVariant {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::Player => "player",
		}
	}

	pub fn get_name_map() -> HashMap<String, Self> {
		let mut out = HashMap::new();
		for tile in Self::iter() {
			out.insert(tile.get_name_id().to_string(), tile);
		}
		out
	}
}