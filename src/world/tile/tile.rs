use std::collections::HashMap;

use strum_macros::{EnumDiscriminants, EnumCount, EnumIter};
use strum::{EnumCount, IntoEnumIterator};

use crate::{render::{vertex::Vertex, texture::Texture}, world::entity::entity::Entity, io::namespace::Namespace};

use super::tile_movement_type::TileMovementType;

#[derive(Clone, EnumDiscriminants)]
#[strum_discriminants(name(TileVariant), derive(EnumCount, EnumIter))]
#[repr(u8)]
pub enum Tile {
	Grass = 0,
	Water = 1,
	Sand = 10,
	PineTree = 3,
	OakTree = 4,
	Flowers = 5,
	FlowersRedYellow = 7,
	Rocks = 6,
	Gravel = 8,
	BlackSand = 9,
	Path = 2,
}

impl Tile {
	pub const fn get_texture(&self) -> Texture {
		match self {
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
		}
	}

	pub fn render(&self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		match self {
			_ => vertices_in_out.extend(self.get_texture().render_basic(pos, [0, 0])),
		}
	}

	pub const fn get_tile_movement_type(&self) -> TileMovementType {
		match self {
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

	pub fn save(&self) -> Vec<u8> {
		let mut out = Vec::new();
		out.push(TileVariant::from(self) as u8);
		out
	}

	pub fn load(data: &[u8], namespace: &Namespace) -> Option<Self> {
		let tile_id = *data.get(0)? as usize;
		let tile_variant = *namespace.tiles.get(tile_id)?;
		Some(match tile_variant {
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
		})
	}
}

impl TileVariant {
	pub fn get_variant_array() -> [Self; Self::COUNT] {
		let mut out = [None; Self::COUNT];
		for variant in Self::iter() {
			out[variant as usize] = Some(variant);
		}
		out.map(|variant| variant.unwrap())
	}

	pub const fn get_name_id(self) -> &'static str {
		match self {
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