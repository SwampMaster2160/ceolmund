use std::collections::HashMap;

use crate::{world::{tile::{tile::Tile, tile_stack::TileStack}, chunk::chunk_pool_offset::ChunkPoolOffset}, render::texture::Texture, io::{namespace::Namespace, file_reader::FileReader}};
use strum::IntoEnumIterator;

use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

/// An item that can exist in a player's inventory.
#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(ItemVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum Item {
	None,
	Hammer,
	Shovel,
	Axe,
	SandboxDestroyWand,
	Tile(Tile),
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
			Self::Tile(tile) => tile.get_texture(),
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
			Self::Tile(tile) => {
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
			//_ => false,
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

	/// Save
	pub fn serialize(&self, data: &mut Vec<u8>) {
		// Push id
		data.push(ItemVariant::from(self) as u8);
		
		match self {
			Self::Tile(tile) => tile.serialize(data),
			_ => {}
		}
	}

	/*/// Create a item from disk data.
	pub fn deserialize(data: &[u8], namespace: &Namespace, version: u32) -> Option<(Self, usize)> {
		let item_id = *data.get(0)? as usize;
		let item_variant = *namespace.items.get(item_id)?;
		Some(match item_variant {
			ItemVariant::Axe => (Self::Axe, 1),
			ItemVariant::Hammer => (Self::Hammer, 1),
			ItemVariant::None => (Self::None, 1),
			ItemVariant::SandboxDestroyWand => (Self::SandboxDestroyWand, 1),
			ItemVariant::Shovel => (Self::Shovel, 1),
			ItemVariant::Tile => {
				let (tile, data_read_size) = Tile::deserialize(data.get(1..)?, namespace, version)?;
				(Self::Tile(tile), 1 + data_read_size)
			},
		})
	}*/

	/// Create a item from disk data.
	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32) -> Option<Self> {
		let variant = *namespace.items.get(file.read_u8()? as usize)?;

		Some(match variant {
			ItemVariant::Axe => Self::Axe,
			ItemVariant::Hammer => Self::Hammer,
			ItemVariant::None => Self::None,
			ItemVariant::SandboxDestroyWand => Self::SandboxDestroyWand,
			ItemVariant::Shovel => Self::Shovel,
			ItemVariant::Tile => Self::Tile(Tile::deserialize(file, namespace, version)?),
		})
	}
}

impl ItemVariant {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::Axe => "axe",
			Self::Tile => "tile",
			Self::Hammer => "hammer",
			Self::None => "none",
			Self::SandboxDestroyWand => "sandbox_destroy_wand",
			Self::Shovel => "shovel",
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