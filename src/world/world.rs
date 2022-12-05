use tokio::runtime::Runtime;

use crate::{world_pos_to_render_pos, render::vertex::Vertex, io::input::Input};

use super::{direction::Direction4, chunk::chunk_pool::ChunkPool, entity::{entity::Entity, entity_action_state::EntityActionState, entity_type::EntityType}};

/// Contains everthing visable that isn't the GUI.
pub struct World {
	player: Entity,
	chunk_pool: ChunkPool,
	seed: u32,
}

impl World {
	pub fn new() -> Self {
		Self { 
			player: Entity {
				pos: [0, 0],
				action_state: EntityActionState::Idle,
    			facing: Direction4::South,
				entity_type: EntityType::Player,
			},
			chunk_pool: ChunkPool::new(),
			seed: 420,
		}
	}

	/// Render the world getting a vector of tris and the center pos of the camera.
	/// The player will be in the center of the screen.
	pub fn render(&mut self, player_visable_width: u64) -> (Vec<Vertex>, [f32; 2]) {
		let mut vertices = Vec::new();
		self.chunk_pool.render(&self.player, player_visable_width, &mut vertices);
		self.player.render(&mut vertices);

		let player = &self.player;
		(vertices, world_pos_to_render_pos(player.pos, player.get_subtile_pos()))
	}

	pub fn tick(&mut self, input: &Input, async_runtime: &Runtime, player_visable_width: u64) {
		self.chunk_pool.tick(&self.player, player_visable_width, async_runtime, self.seed);
		self.player.player_tick(&mut self.chunk_pool, input);
		self.player.tick(&mut self.chunk_pool);
	}
}