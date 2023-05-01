use std::collections::HashMap;

use crate::{render::texture::Texture, world::{item::inventory::Inventory, difficulty::Difficulty}, io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}, error::Error};

use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(EntityVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum EntityType {
	Player { inventory: Inventory<50>, selected_item: u8, respawn_pos: [i64; 2], is_swaping_item: bool },
}

impl EntityType {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::Player { .. } => Texture::Player,
		}
	}

	/// Save
	pub fn serialize(&self, file: &mut FileWriter) {
		// Push id
		file.push_u8(EntityVariant::from(self) as u8);
		
		match self {
			Self::Player { inventory, selected_item, respawn_pos, is_swaping_item: _ } => {
				// Push inventory
				inventory.serialize(file);
				// Push selected item
				file.push_u8(*selected_item);
				// Push respawn pos
				file.push_world_pos(*respawn_pos);
			},
		}
	}

	/// Load an entity
	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32, _difficulty: Difficulty) -> Result<Self, Error> {
		// Get variant
		let variant = *namespace.entities.get(file.read_u8()? as usize).ok_or(Error::IDOutOfNamespaceBounds)?;

		Ok(match variant {
			EntityVariant::Player => {
				let inventory = Inventory::deserialize(file, namespace, version)?;
				let selected_item = file.read_u8()?;
				// Respawn pos
				let respawn_pos = match version {
					0 => [0, 0],
					_ => file.read_world_pos()?,
				};
				
				Self::Player { inventory, selected_item, respawn_pos, is_swaping_item: false }
			}
		})
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

	pub fn max_health(self) -> u32 {
		match self {
			Self::Player { .. } => 100,
		}
	}
}