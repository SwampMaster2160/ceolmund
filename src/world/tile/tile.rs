use crate::{render::{vertex::Vertex, texture::Texture}, world::entity::entity::Entity};

use super::tile_movement_type::TileMovementType;

#[derive(Clone, Debug)]
pub enum Tile {
	Grass,
	Water,
	Sand,
	PineTree,
	OakTree,
	Flowers,
	FlowersRedYellow,
	Rocks,
	Gravel,
	BlackSand,
}

impl Tile {
	pub fn get_texture(&self) -> Texture {
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
		}
	}

	pub fn render(&self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		match self {
			_ => vertices_in_out.extend(self.get_texture().render_basic(pos, [0, 0])),
		}
	}

	pub fn get_tile_movement_type(&self) -> TileMovementType {
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
		}
	}

	/// Called when an entity trys to move to this tile and returns weather or not the entity can move to this tile.
	pub fn entity_try_move_to(&mut self, _entity: &mut Entity) -> bool {
		match self.get_tile_movement_type() {
			TileMovementType::Clear => true,
			TileMovementType::Wall => false,
		}
	}
}