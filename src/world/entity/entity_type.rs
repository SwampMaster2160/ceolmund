use std::collections::HashMap;

use crate::{render::texture::Texture, world::{item::item::Item, tile::tile::Tile}, io::namespace::Namespace};

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

	/// Save
	pub fn serialize(&self, data: &mut Vec<u8>) {
		// Push id
		data.push(EntityVariant::from(self) as u8);
		
		match self {
			Self::Player { inventory, selected_item } => {
				for (item, stack_amount) in inventory.iter() {
					item.serialize(data);
					data.push(*stack_amount);
				}
				data.push(*selected_item);
			},
		}
	}

	/// Load an entity
	pub fn deserialize(data: &[u8], namespace: &Namespace, version: u32) -> Option<(Self, usize)> {
		// Get variant
		let variant = *namespace.entities.get(*data.get(0)? as usize)?;
		let mut data_read_size_out = 1;

		Some((match variant {
			EntityVariant::Player => {
				let mut inventory = Box::new([(); 50].map(|_| (Item::None, 0)));
				for x in 0..50 {
					let (item, data_read_size) = Item::deserialize(data.get(data_read_size_out..)?, namespace, version)?;
					data_read_size_out += data_read_size;
					let amount = *data.get(data_read_size_out)?;
					data_read_size_out += 1;
					inventory[x] = (item, amount);
				}
				let selected_item = *data.get(data_read_size_out)?;
				data_read_size_out += 1;
				// Temp
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
				
				Self::Player { inventory, selected_item }
			}
		}, data_read_size_out))
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