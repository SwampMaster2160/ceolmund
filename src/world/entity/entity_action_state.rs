use std::collections::HashMap;

use strum::{IntoEnumIterator};
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

#[derive(Eq, PartialEq, Clone)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(EntityActionStateVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum EntityActionState {
	Idle,
	Walking(u8),
}

impl EntityActionState {
	/// Save
	pub fn save(&self, data: &mut Vec<u8>) {
		// Push id
		data.push(EntityActionStateVariant::from(self) as u8);
		
		match self {
			Self::Idle => {}
			Self::Walking(amount) => data.push(*amount),
		}
	}
}

impl EntityActionStateVariant {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::Idle => "idle",
			Self::Walking => "walking",
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