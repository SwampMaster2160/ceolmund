use std::collections::HashMap;

use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};
use strum::{IntoEnumIterator};

use crate::{render::{vertex::Vertex, texture::Texture}, world::{entity::entity::Entity, item::{item::Item, item_drop::ItemDrop}}, io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}};

use super::{tile_movement_type::TileMovementType, tile_stack::TileStack};

#[derive(Clone, EnumDiscriminants, PartialEq, Eq)]
#[strum_discriminants(name(TileVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum Tile {
	None,
	Grass,
	Water,
	Path,
	PineTree,
	OakTree,
	Flowers,
	Rocks,
	FlowersRedYellow,
	Gravel,
	BlackSand,
	Sand,
	DroppedItemStack(Box<Item>, u16),
	Item(Box<Item>),
}

/// A tile in the world
impl Tile {
	/// The texture that is used to draw the tile
	pub const fn get_texture(&self) -> Texture {
		match self {
			Self::None => Texture::NoTexture,
			Self::Grass => Texture::Grass,
			Self::Water => Texture::Water,
			Self::Sand => Texture::Sand,
			Self::PineTree => Texture::PineTree,
			Self::OakTree => Texture::OakTree,
			Self::Flowers => Texture::Flowers,
			Self::FlowersRedYellow => Texture::FlowersRedYellow,
			Self::Rocks => Texture::Rocks,
			Self::Gravel => Texture::Gravel,
			Self::BlackSand => Texture::BlackSand,
			Self::Path => Texture::Path,
			Self::DroppedItemStack(..) => Texture::DroppedItems,
			Self::Item(item) => item.get_texture(),
		}
	}

	/// Renders the tile to a tri list.
	pub fn render(&self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		match self {
			_ => vertices_in_out.extend(self.get_texture().render_basic(pos, [0, 0])),
		}
	}

	/// Get when an entity can move.
	pub const fn get_tile_movement_type(&self) -> TileMovementType {
		match self {
			Self::None => TileMovementType::Wall,
			Self::Grass => TileMovementType::Clear,
			Self::Water => TileMovementType::Wall,
			Self::Sand => TileMovementType::Clear,
			Self::PineTree => TileMovementType::Wall,
			Self::OakTree => TileMovementType::Wall,
			Self::Flowers => TileMovementType::Clear,
			Self::FlowersRedYellow => TileMovementType::Clear,
			Self::Rocks => TileMovementType::Wall,
			Self::BlackSand => TileMovementType::Clear,
			Self::Gravel => TileMovementType::Clear,
			Self::Path => TileMovementType::Clear,
			Self::DroppedItemStack(..) => TileMovementType::Clear,
			Self::Item(..) => TileMovementType::Clear,
		}
	}

	/// Called when an entity trys to move to this tile and returns weather or not the entity can move to this tile.
	pub fn entity_try_move_to(&mut self, _entity: &mut Entity) -> bool {
		let out = match self.get_tile_movement_type() {
			TileMovementType::Clear => true,
			TileMovementType::Wall => false,
		};
		out
	}

	/// Get data for the tile to save to disk.
	pub fn serialize(&self, file: &mut FileWriter) {
		file.push_u8(TileVariant::from(self) as u8);
		match self {
			Self::DroppedItemStack(item, amount) => {
				item.serialize(file);
				file.push_u16(*amount);
			}
			Self::Item(item) => item.serialize(file),
			_ => {},
		}
	}

	/// Create a tile form disk data.
	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32) -> Option<Self> {
		//let tile_id = *data.get(0)? as usize;
		let variant = *namespace.tiles.get(file.read_u8()? as usize)?;
		Some(match variant {
			TileVariant::None => panic!("None tile should not exist."),//Self::None,
			TileVariant::Grass => Self::Grass,
			TileVariant::Water => Self::Water,
			TileVariant::Sand => Self::Sand,
			TileVariant::PineTree => Self::PineTree,
			TileVariant::OakTree => Self::OakTree,
			TileVariant::Flowers => Self::Flowers,
			TileVariant::FlowersRedYellow => Self::FlowersRedYellow,
			TileVariant::Rocks => Self::Rocks,
			TileVariant::Gravel => Self::Gravel,
			TileVariant::BlackSand => Self::BlackSand,
			TileVariant::Path => Self::Path,
			TileVariant::DroppedItemStack => {
				let item = Item::deserialize(file, namespace, version)?;
				let amount = file.read_u16()?;
				Self::DroppedItemStack(Box::new(item), amount)
			}
			TileVariant::Item => {
				let item = Item::deserialize(file, namespace, version)?;
				Self::Item(Box::new(item))
			}
		})
	}

	/// Create a tile form disk data.
	pub fn deserialize_v0(data: &[u8], namespace: &Namespace, _version: u32) -> Option<(Self, usize)> {
		let tile_id = *data.get(0)? as usize;
		let tile_variant = *namespace.tiles.get(tile_id)?;
		Some((match tile_variant {
			TileVariant::None => panic!("None tile should not exist."),//Self::None,
			TileVariant::Grass => Self::Grass,
			TileVariant::Water => Self::Water,
			TileVariant::Sand => Self::Sand,
			TileVariant::PineTree => Self::PineTree,
			TileVariant::OakTree => Self::OakTree,
			TileVariant::Flowers => Self::Flowers,
			TileVariant::FlowersRedYellow => Self::FlowersRedYellow,
			TileVariant::Rocks => Self::Rocks,
			TileVariant::Gravel => Self::Gravel,
			TileVariant::BlackSand => Self::BlackSand,
			TileVariant::Path => Self::Path,
			_ => panic!("Tile should not exist in this version."),
		}, 1))
	}

	/// Can an axe be used on the tile?
	pub fn is_choppable(&self) -> bool {
		match self {
			Self::OakTree | Self::PineTree => true,
			_ => false,
		}
	}

	/// Can water be placed on top?
	pub fn is_floodable(&self) -> bool {
		match self {
			Self::Sand | Self::BlackSand | Self::Gravel | Self::Rocks => true,
			_ => false,
		}
	}

	/// Can stuff grow on top? (flowers, trees, ect.)
	pub fn is_fertile(&self) -> bool {
		match self {
			Self::Grass => true,
			_ => false,
		}
	}

	/// Can the tile be broken with the fist?
	pub fn is_pluckable(&self) -> bool {
		match self {
			Self::Flowers | Self::FlowersRedYellow | Self::Item(..) => true,
			_ => false,
		}
	}

	/// Is a tile like grass or gravel that is flat.
	pub fn is_plain(&self) -> bool {
		match self {
			Self::Grass | Self::Gravel | Self::BlackSand | Self::Sand => true,
			_ => false,
		}
	}

	/// Can the tile be placed on a tile stack?
	pub fn can_place_on(&self, tile_stack: &TileStack) -> bool {
		match self {
			Self::None => panic!("None tile should not exist."),
			Self::Grass | Self::Gravel | Self::Sand | Self::BlackSand => tile_stack.tiles.is_empty(),
			Self::Water => match tile_stack.tiles.last() {
				Some(top_tile) => top_tile.is_floodable(),
				None => false,
			}
			Self::OakTree | Self::PineTree | Self::Flowers | Self::FlowersRedYellow => match tile_stack.tiles.last() {
				Some(top_tile) => top_tile.is_fertile(),
				None => false,
			}
			Self::Rocks => match tile_stack.tiles.last() {
				Some(top_tile) => match top_tile {
					Self::OakTree | Self::PineTree | Self::Flowers | Self::FlowersRedYellow | Self::Path | Self::Rocks | Self::DroppedItemStack(..) => false,
					_ => true,
				},
				None => false,
			}
			Self::Path => match tile_stack.tiles.last() {
				Some(top_tile) => match top_tile {
					Self::OakTree | Self::PineTree | Self::Flowers | Self::FlowersRedYellow | Self::Water | Self::Path | Self::Rocks => false,
					_ => true,
				},
				None => false,
			}
			Self::DroppedItemStack(..) => true,
			Self::Item(..) => {
				let top_tile = match tile_stack.tiles.last() {
					Some(top_tile) => top_tile,
					None => return false,
				};
				top_tile.is_plain()
			}
		}
	}

	/// Get a vector of items and amounts that are dropped when the tile is broken.
	pub fn get_drops(&self) -> Vec<ItemDrop> {
		match self {
			Self::Item(item) => vec![ItemDrop::Single(*item.clone())],
			Self::OakTree => vec![
				ItemDrop::RangedRandomAmount { item: Item::Acorn, amount_range: 1..=4 },
				ItemDrop::RangedRandomAmount { item: Item::OakStick, amount_range: 0..=5 },
			],
			Self::PineTree => vec![
				ItemDrop::RangedRandomAmount { item: Item::PineCone, amount_range: 1..=4 },
				ItemDrop::RangedRandomAmount { item: Item::PineStick, amount_range: 0..=5 },
			],
			_ => vec![ItemDrop::Single(Item::Tile(self.clone()))],
		}
	}
}

impl TileVariant {
	pub const fn get_name_id(self) -> &'static str {
		match self {
			Self::None => "none",
			Self::Grass => "grass",
			Self::Water => "water",
			Self::Sand => "sand",
			Self::PineTree => "pine_tree",
			Self::OakTree => "oak_tree",
			Self::Flowers => "flowers",
			Self::FlowersRedYellow => "red_yellow_flowers",
			Self::Rocks => "rocks",
			Self::Gravel => "gravel",
			Self::BlackSand => "black_sand",
			Self::Path => "path",
			Self::DroppedItemStack => "dropped_item_stack",
			Self::Item => "item",
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