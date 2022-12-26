use crate::{world::tile::tile::Tile, render::texture::Texture};

#[repr(u8)]
pub enum Item {
	None,
	Shovel,
	Axe,
	SandboxDestroyer,
	BaseTilePlacer(Tile),
}

impl Item {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::None => Texture::NoTexture,
			Self::Shovel => Texture::NoTexture,
			Self::Axe => Texture::NoTexture,
			Self::SandboxDestroyer => Texture::NoTexture,
			Self::BaseTilePlacer(tile) => tile.get_texture(),
		}
	}
}