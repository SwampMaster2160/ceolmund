use std::ops::RangeInclusive;

use rand::{thread_rng, Rng};

use crate::world::item::item::Item;

/// Info about a item drop for a tile or whatever, is an item with a ranged/constant amount with a roll function to get a random amount to drop.
pub enum ItemDrop {
	Single (Item), // Always get a single item.
	ConstantAmount { item: Item, amount: u16 }, // Always get the amount of items.
	RangedRandomAmount { item: Item, amount_range: RangeInclusive<u16> }, // Get a random amount in the amount range.
}

impl ItemDrop {
	/// Get a tuple containing the item and a random/constant amount in the ItemDrop range.
	pub fn roll(&self) -> (Item, u16) {
		match self {
			Self::Single(item) => (item.clone(), 1),
			Self::ConstantAmount { item, amount } => (item.clone(), *amount),
			Self::RangedRandomAmount { item, amount_range } => {
				let mut rng = thread_rng();
				(item.clone(), rng.gen_range(amount_range.clone()))
			}
		}
	}
}