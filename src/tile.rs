use crate::{vertex::Vertex, texture::Texture, tile_movement_type::TileMovementType, entity::Entity};

#[derive(Clone, Debug)]
pub enum Tile {
	Grass,
	Water,
}

impl Tile {
	pub fn get_texture(&self) -> Texture {
		match self {
			Self::Grass => Texture::Grass,
			Self::Water => Texture::Water,
		}
	}

	pub fn render(&self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		match self {
			_ => vertices_in_out.extend(self.get_texture().render(pos, [0, 0])),
		}
	}

	pub fn get_tile_movement_type(&self) -> TileMovementType {
		match self {
			Tile::Grass => TileMovementType::Clear,
			Tile::Water => TileMovementType::Wall,
		}
	}

	pub fn try_move_to(&mut self, entity: &mut Entity) -> bool {
		match self.get_tile_movement_type() {
			TileMovementType::Clear => true,
			TileMovementType::Wall => false,
		}
	}
}