use super::item::Item;

macro_rules! recipe {
	( $in:expr, $out:expr ) => {
		{
			const A: &[(Item, u16)] = $in.as_slice();
			const B: &[(Item, u16)] = $out.as_slice();
			(A, B)
		}
	};
}

#[derive(Copy, Clone)]
pub enum CraftingRecipes {
	QUICK,
}

impl CraftingRecipes {
	pub const fn get_recipes(self) -> &'static [(&'static [(Item, u16)], &'static [(Item, u16)])] {
		match self {
			Self::QUICK => [
				recipe!([(Item::SharpendFlint, 1), (Item::OakStick, 1)], [(Item::FlintAxe, 1)]),
				recipe!([(Item::SharpendFlint, 5), (Item::OakStick, 1)], [(Item::FlintShovel, 1)]),
				recipe!([(Item::SharpendFlint, 1), (Item::PineStick, 1)], [(Item::FlintAxe, 1)]),
				recipe!([(Item::SharpendFlint, 5), (Item::PineStick, 1)], [(Item::FlintShovel, 1)]),
			].as_slice(),
		}
	}
}