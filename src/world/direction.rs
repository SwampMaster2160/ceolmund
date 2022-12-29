use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumCount};

#[derive(PartialEq, Eq, Clone, Copy, EnumCount, EnumIter, Debug)]
/// North/East/South/West direction.
pub enum Direction4 {
	North,
	East,
	South,
	West,
}

impl Direction4 {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::North => "north",
			Self::East => "east",
			Self::South => "south",
			Self::West => "west",
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