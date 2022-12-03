pub mod entity_action_state;
pub mod entity_type;

use crate::{direction::Direction4, chunk_pool::ChunkPool, input::Input, game_key::GameKey, tile_stack::TileStack, vertex::Vertex, texture::Texture};

use self::{entity_action_state::EntityActionState, entity_type::EntityType};

pub struct Entity {
	pub pos: [i64; 2],
	pub action_state: EntityActionState,
	pub facing: Direction4,
	pub entity_type: EntityType,
}

impl Entity {
	pub fn get_subtile_pos(&self) -> [i8; 2] {
		match self.action_state {
			EntityActionState::Idle => [0, 0],
			EntityActionState::Walking(walked_amount) => match self.facing {
				Direction4::North => [0, -(walked_amount as i8)],
				Direction4::East => [(walked_amount as i8), 0],
				Direction4::South => [0, (walked_amount as i8)],
				Direction4::West => [-(walked_amount as i8), 0],
			}
		}
	}

	pub fn player_tick(&mut self, chunks: &mut ChunkPool, input: &Input) {
		if self.action_state == EntityActionState::Idle {
			let mut try_move = true;
			if input.get_game_key(GameKey::WalkNorth) {
				self.facing = Direction4::North;
			}
			else if input.get_game_key(GameKey::WalkEast) {
				self.facing = Direction4::East;
			}
			else if input.get_game_key(GameKey::WalkSouth) {
				self.facing = Direction4::South;
			}
			else if input.get_game_key(GameKey::WalkWest) {
				self.facing = Direction4::West;
			}
			else {
				try_move = false;
			}
			if try_move {
				if let Some(tile_stack) = chunks.get_tile_stack_at(self.get_pos_in_front()) {
					let pos_in_front = self.get_pos_in_front();
					tile_stack.try_move_to(self);
				}
			}
		}
	}

	pub fn tick(&mut self, chunks: &mut ChunkPool) {
		match &mut self.action_state {
			EntityActionState::Idle => {},
			EntityActionState::Walking(amount) => {
				*amount += 1;
				if *amount > 15 {
					self.action_state = EntityActionState::Idle;
					self.pos = self.get_pos_in_front();
				}
			}
		}
	}

	pub fn get_pos_in_front(&self) -> [i64; 2] {
		let pos = self.pos;
		match self.facing {
			Direction4::North => [pos[0], pos[1] - 1],
			Direction4::East => [pos[0] + 1, pos[1]],
			Direction4::South => [pos[0], pos[1] + 1],
			Direction4::West => [pos[0] - 1, pos[1]],
		}
	}

	pub fn render(&self, vertices_in_out: &mut Vec<Vertex>) {
		vertices_in_out.extend(Texture::BlueThing.to_tris(self.pos, self.get_subtile_pos()));
	}
}