use std::collections::HashMap;

use strum::{IntoEnumIterator};
use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

use crate::{io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}, world::direction::Direction4};

#[derive(Eq, PartialEq, Clone)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(EntityActionStateVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum EntityActionState {
	Idle,
	Walking(Direction4, u8),
}

impl EntityActionState {
	/// Save
	pub fn serialize(&self, file: &mut FileWriter) {
		// Push id
		file.push_u8(EntityActionStateVariant::from(self) as u8);
		
		match self {
			Self::Idle => {}
			Self::Walking(moving_direction, amount) => {
				file.push_u8(*moving_direction as u8);
				file.push_u8(*amount);
			},
		}
	}

	/// Load
	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32, facing: Direction4) -> Option<Self> {
		let variant = *namespace.entity_action_states.get(file.read_u8()? as usize)?;
		Some(match variant {
			EntityActionStateVariant::Idle => Self::Idle,
			EntityActionStateVariant::Walking => {
				let moving_direction = if version < 2 {
					facing
				}
				else {
					*namespace.direction_4s.get(file.read_u8()? as usize)?
				};
				Self::Walking(moving_direction, file.read_u8()?)
			},
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