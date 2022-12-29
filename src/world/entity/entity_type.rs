use std::collections::HashMap;

use crate::{render::texture::Texture, world::item::item::Item, io::namespace::Namespace};

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
	pub fn save(&self, data: &mut Vec<u8>) {
		// Push id
		data.push(EntityVariant::from(self) as u8);
		
		match self {
			Self::Player { inventory, selected_item } => {
				for (item, stack_amount) in inventory.iter() {
					item.save(data);
					data.push(*stack_amount);
				}
				data.push(*selected_item);
			},
		}
	}

	/// Load an entity
	pub fn load(data: &[u8], namespace: &Namespace, _version: u32) -> Option<(Self, usize)> {
		// Get variant
		let variant = *namespace.entities.get(*data.get(0)? as usize)?;
		let mut data_advanced_amount = 1;

		Some((match variant {
			EntityVariant::Player => {
				let mut inventory = Box::new([(); 50].map(|_| (Item::None, 0)));
				for x in 0..50 {
					let (item, advanced) = Item::load(data.get(data_advanced_amount..)?, namespace)?;
					data_advanced_amount += advanced;
					let amount = *data.get(data_advanced_amount)?;
					data_advanced_amount += 1;
					inventory[x] = (item, amount);
				}
				let selected_item = *data.get(data_advanced_amount)?;
				data_advanced_amount += 1;
				Self::Player { inventory, selected_item }
			}
		}, data_advanced_amount))
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