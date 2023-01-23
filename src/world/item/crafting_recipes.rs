use crate::world::item::item_category::ItemCategory;
use super::item::Item;

macro_rules! recipe {
	( $in:expr, $out:expr ) => {
		{
			const A: &[(ItemCategory, u16)] = $in.as_slice();
			const B: &[(Item, u16)] = $out.as_slice();
			(A, B)
		}
	};
}

#[derive(Copy, Clone)]
pub enum CraftingRecipes {
	Quick,
}

impl CraftingRecipes {
	pub const fn get_recipes(self) -> &'static [(&'static [(ItemCategory, u16)], &'static [(Item, u16)])] {
		match self {
			Self::Quick => [
				recipe!([(ItemCategory::Single(Item::SharpendFlint), 1), (ItemCategory::Stick, 1)], [(Item::FlintAxe, 1)]),
				recipe!([(ItemCategory::Single(Item::SharpendFlint), 5), (ItemCategory::Stick, 1)], [(Item::FlintShovel, 1)]),
			].as_slice(),
		}
	}
}