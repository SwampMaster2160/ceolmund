use super::item::Item;

#[derive(Clone)]
/// A grouping that may describe an item. Eg. oak and pine sticks are sticks.
pub enum ItemCategory {
	Single(Item),
	Stick,
}

impl ItemCategory {
	/// Weather or not an item falls into a category.
	pub fn has_item(&self, item: &Item) -> bool {
		match self {
			Self::Single(single_item) => *item == *single_item,
			Self::Stick => *item == Item::OakStick || *item == Item::PineStick,
		}
	}
}