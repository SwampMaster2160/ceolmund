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
			Self::QUICK => [/*recipe!([(Item::SandboxDestroyWand, 2)], [(Item::SandboxDestroyWand, 1)])*/].as_slice(),
		}
	}
}