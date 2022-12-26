use crate::{world::tile::tile::Tile, render::texture::Texture};

#[repr(u8)]
pub enum Item {
	None,
	Hammer,
	Shovel,
	Axe,
	SandboxDestroyWand,
	BaseTilePlacer(Tile),
}

impl Item {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::None => Texture::NoTexture,
			Self::Shovel => Texture::Shovel,
			Self::Axe => Texture::Axe,
			Self::SandboxDestroyWand => Texture::SandboxDestroyWand,
			Self::Hammer => Texture::Hammer,
			Self::BaseTilePlacer(tile) => tile.get_texture(),
		}
	}
}