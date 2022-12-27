use crate::{world::{tile::{tile::Tile, tile_stack::TileStack}, chunk::chunk_pool_offset::ChunkPoolOffset}, render::texture::Texture};

/// An item that can exist in a player's inventory.
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
	/// Get the texture use for the item.
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

	/// The item is used, if false is returned then the item will try to execute using other functions.
	pub fn use_stack_mut_self(self_stack: &mut (Self, u8), chunk_pool_used_on: &mut ChunkPoolOffset) -> bool {
		let (item, _count) = self_stack;
		match item {
			Self::SandboxDestroyWand | Self::Axe | Self::Hammer | Self::Shovel | Self::None => {
				let tile_stack = match chunk_pool_used_on.get_origin_tile_stack_mut() {
					Some(tile_stack) => tile_stack,
					None => return false,
				};
				if !item.can_break(tile_stack) {
					return false;
				}
				tile_stack.tiles.pop();
				tile_stack.needs_redrawing = true;
				true
			}
			Self::BaseTilePlacer(tile) => {
				let tile_stack = match chunk_pool_used_on.get_origin_tile_stack_mut() {
					Some(tile_stack) => tile_stack,
					None => return false,
				};
				if !tile.can_place_on(tile_stack) {
					return false;
				}
				tile_stack.tiles.push(tile.clone());
				tile_stack.needs_redrawing = true;
				true
			}
			_ => false,
		}
	}

	pub fn can_break(&self, tile_stack: &TileStack) -> bool {
		match self {
			Self::SandboxDestroyWand => true,
			Self::Shovel => tile_stack.tiles.len() == 1,
			Self::Axe => match tile_stack.tiles.last() {
				Some(top_tile) => top_tile.is_choppable(),
				None => false,
			}
			Self::None =>  match tile_stack.tiles.last() {
				Some(top_tile) => top_tile.is_pluckable(),
				None => false,
			}
			_ => false,
		}
	}
}