use std::collections::HashMap;

use crate::{vertex::Vertex, entity::{Entity, entity_action_state::EntityActionState}, direction::Direction4, world_pos_to_render_pos, input::Input, game_key::GameKey, chunk_slot::ChunkSlot, chunk::Chunk};

pub struct World {
	player: Entity,
	chunks: HashMap<[i64; 2], ChunkSlot>,
}

impl World {
	pub fn new() -> Self {
		let mut out = Self { 
			player: Entity {
				pos: [0, 0],
				action_state: EntityActionState::Idle,
    			facing: Direction4::South,
			},
			chunks: HashMap::new(),
		};
		out.chunks.insert([0, 0], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([-1, 0], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([0, -1], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([-1, -1], ChunkSlot::Chunk(Chunk::new()));
		out
	}

	/// Render the world getting a vector of tris and the center pos of the camera.
	pub fn render(&mut self) -> (Vec<Vertex>, [f32; 2]) {
		let mut vertices = Vec::new();
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.render(*pos, &mut vertices);
			}
		}

		let player = &self.player;
		(vertices, world_pos_to_render_pos(player.pos, player.get_subtile_pos()))
	}

	pub fn tick(&mut self, input: &Input) {
		if input.get_game_key(GameKey::WalkNorth) {
			self.player.pos[1] -= 1;
		}
		if input.get_game_key(GameKey::WalkEast) {
			self.player.pos[0] += 1;
		}
		if input.get_game_key(GameKey::WalkSouth) {
			self.player.pos[1] += 1;
		}
		if input.get_game_key(GameKey::WalkWest) {
			self.player.pos[0] -= 1;
		}
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.tick(pos);
			}
		}
	}
}