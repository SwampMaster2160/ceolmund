use std::collections::HashMap;

use crate::{render::texture::Texture, world::{item::item::Item, tile::tile::Tile, difficulty::Difficulty}, io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}};

use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(EntityVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum EntityType {
	Player { inventory: Box<[(Item, u8); 50]>, selected_item: u8, respawn_pos: [i64; 2] },
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
			Self::Player { inventory, selected_item, respawn_pos } => {
				// Push inventory
				for (item, stack_amount) in inventory.iter() {
					item.serialize(file);
					file.push_u8(*stack_amount);
				}
				// Push selected item
				file.push_u8(*selected_item);
				// Push respawn pos
				file.push_i64(respawn_pos[0]);
				file.push_i64(respawn_pos[1]);
			},
		}
	}

	/// Load an entity
	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32, difficulty: Difficulty) -> Option<Self> {
		// Get variant
		let variant = *namespace.entities.get(file.read_u8()? as usize)?;

		Some(match variant {
			EntityVariant::Player => {
				let mut inventory = Box::new([(); 50].map(|_| (Item::None, 0)));
				for x in 0..50 {
					let item = Item::deserialize(file, namespace, version)?;
					let amount = file.read_u8()?;
					inventory[x] = (item, amount);
				}
				let selected_item = file.read_u8()?;
				// Temp
				if difficulty == Difficulty::Sandbox {
					inventory[0] = (Item::SandboxDestroyWand, 1);
					inventory[1] = (Item::Tile(Tile::Grass), 1);
					inventory[2] = (Item::Tile(Tile::Gravel), 1);
					inventory[3] = (Item::Tile(Tile::Sand), 1);
					inventory[4] = (Item::Tile(Tile::BlackSand), 1);
					inventory[5] = (Item::Tile(Tile::Rocks), 1);
					inventory[6] = (Item::Tile(Tile::OakTree), 1);
					inventory[7] = (Item::Tile(Tile::PineTree), 1);
					inventory[8] = (Item::Tile(Tile::Flowers), 1);
					inventory[9] = (Item::Tile(Tile::FlowersRedYellow), 1);
					inventory[10] = (Item::Tile(Tile::Water), 1);
					inventory[11] = (Item::Tile(Tile::Path), 1);
					inventory[12] = (Item::Axe, 1);
					inventory[13] = (Item::Shovel, 1);
				}
				// Respawn pos
				let respawn_pos = if version > 0 {
					let pos_x = file.read_i64()?;
					let pos_y = file.read_i64()?;
					[pos_x, pos_y]
				}
				else {
					[0, 0]
				};
				
				Self::Player { inventory, selected_item, respawn_pos }
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