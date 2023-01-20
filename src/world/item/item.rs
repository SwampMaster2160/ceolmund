use std::collections::HashMap;

use crate::{world::{tile::{tile::Tile, tile_stack::TileStack}, chunk::chunk_pool_offset::ChunkPoolOffset}, render::texture::Texture, io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}};
use strum::IntoEnumIterator;

use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};

use super::item_drop::ItemDrop;

/// An item that can exist in a player's inventory.
#[derive(Clone, EnumDiscriminants, PartialEq, Eq)]
#[strum_discriminants(name(ItemVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum Item {
	None,
	Hammer,
	Shovel,
	Axe,
	SandboxDestroyWand,
	Tile(Tile),
	Rock,
	FlintRock,
	PineStick,
	OakStick,
}

impl Item {
	/// Get the texture use for the item.
	pub const fn get_texture(&self) -> Texture {
		match self {
			Self::None => Texture::NoTexture,
			Self::Shovel => Texture::Shovel,
			Self::Axe => Texture::Axe,
			Self::SandboxDestroyWand => Texture::SandboxDestroyWand,
			Self::Hammer => Texture::Hammer,
			Self::Tile(tile) => tile.get_texture(),
			Self::Rock => Texture::Rock,
			Self::FlintRock => Texture::FlintRock,
			Self::PineStick => Texture::PineStick,
			Self::OakStick => Texture::OakStick,
		}
	}

	pub fn is_none(&self) -> bool {
		ItemVariant::from(self) == ItemVariant::None
	}

	/// The item is used, returns weather the item should be consumed and the drops to be added to the player inventory.
	pub fn use_stack_mut_self(self_stack: &mut (Self, u8), chunk_pool_used_on: &mut ChunkPoolOffset) -> (bool, Vec<ItemDrop>) {
		let (item, _count) = self_stack;
		match item {
			// Tools and nothing
			Self::SandboxDestroyWand | Self::Axe | Self::Hammer | Self::Shovel | Self::None => {
				// Get the tile stack
				let tile_stack = match chunk_pool_used_on.get_origin_tile_stack_mut() {
					Some(tile_stack) => tile_stack,
					None => return (false, Vec::new()),
				};
				// Check if we can break the tile with the tool we have
				if !item.can_break(tile_stack) {
					return (false, Vec::new());
				}
				// Break tile
				let tile = match tile_stack.tiles.pop() {
					Some(tile) => tile,
					None => return (false, Vec::new()),
				};
				tile_stack.needs_redrawing = true;
				(false, tile.get_drops())
			}
			// Place a tile
			Self::Tile(tile) => {
				let tile_stack = match chunk_pool_used_on.get_origin_tile_stack_mut() {
					Some(tile_stack) => tile_stack,
					None => return (false, Vec::new()),
				};
				if !tile.can_place_on(tile_stack) {
					return (false, Vec::new());
				}
				tile_stack.tiles.push(tile.clone());
				tile_stack.needs_redrawing = true;
				(true, Vec::new())
			}
			Self::Rock | Self::FlintRock | Self::PineStick | Self::OakStick => {
				let tile_stack = match chunk_pool_used_on.get_origin_tile_stack_mut() {
					Some(tile_stack) => tile_stack,
					None => return (false, Vec::new()),
				};
				let to_place = Tile::Item(Box::new(item.clone()));
				if to_place.can_place_on(tile_stack) {
					tile_stack.tiles.push(to_place);
					tile_stack.needs_redrawing = true;
				}
				(true, Vec::new())
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

	pub fn consume_item(&mut self, stack_size: &mut u8) {
		*stack_size = stack_size.saturating_sub(1);
		if *stack_size == 0 {
			*self = Item::None;
		}
	}

	/// Save
	pub fn serialize(&self, file: &mut FileWriter) {
		// Push id
		file.push_u8(ItemVariant::from(self) as u8);
		
		match self {
			Self::Tile(tile) => tile.serialize(file),
			_ => {}
		}
	}

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
			ItemVariant::Rock => Self::Rock,
			ItemVariant::FlintRock => Self::FlintRock,
			ItemVariant::PineStick => Self::PineStick,
			ItemVariant::OakStick => Self::OakStick,
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
			Self::Rock => "rock",
			Self::FlintRock => "flint_rock",
			Self::PineStick => "pine_stick",
			Self::OakStick => "oak_stick",
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