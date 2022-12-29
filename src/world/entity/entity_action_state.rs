use std::collections::HashMap;

use strum::{IntoEnumIterator};
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

use crate::io::namespace::Namespace;

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
	pub fn serialize(&self, data: &mut Vec<u8>) {
		// Push id
		data.push(EntityActionStateVariant::from(self) as u8);
		
		match self {
			Self::Idle => {}
			Self::Walking(amount) => data.push(*amount),
		}
	}

	/// Load
	pub fn deserialize(data: &[u8], namespace: &Namespace, _version: u32) -> Option<(Self, usize)> {
		// Get id
		let id = *data.get(0)?;
		let variant = *namespace.entity_action_states.get(id as usize)?;
		Some(match variant {
			EntityActionStateVariant::Idle => (Self::Idle, 1),
			EntityActionStateVariant::Walking => (Self::Walking(*data.get(1)?), 2),
		})
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