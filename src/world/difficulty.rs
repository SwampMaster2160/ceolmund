use strum::IntoEnumIterator;
use std::collections::HashMap;

use strum_macros::{EnumIter, EnumCount};

#[derive(EnumCount, EnumIter, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
	Sandbox,
	Easy,
	Medium,
	Hard,
}

impl Difficulty {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::Sandbox => "sandbox",
			Self::Easy => "easy",
			Self::Medium => "medium",
			Self::Hard => "hard",
		}
	}

	pub fn get_name_map() -> HashMap<String, Self> {
		let mut out = HashMap::new();
		for difficulty in Self::iter() {
			out.insert(difficulty.get_name_id().to_string(), difficulty);
		}
		out
	}
}