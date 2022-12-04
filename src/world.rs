use tokio::runtime::Runtime;

use crate::{vertex::Vertex, entity::{Entity, entity_action_state::EntityActionState, entity_type::EntityType}, direction::Direction4, world_pos_to_render_pos, input::Input, chunk_pool::ChunkPool};

pub struct World {
	player: Entity,
	chunk_pool: ChunkPool,
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
		}
	}

	/// Render the world getting a vector of tris and the center pos of the camera.
	pub fn render(&mut self) -> (Vec<Vertex>, [f32; 2]) {
		let mut vertices = Vec::new();
		self.chunk_pool.render(&mut vertices);
		self.player.render(&mut vertices);

		let player = &self.player;
		(vertices, world_pos_to_render_pos(player.pos, player.get_subtile_pos()))
	}

	pub fn tick(&mut self, input: &Input, async_runtime: &Runtime, player_visable_width: u64) {
		self.chunk_pool.tick(&self.player, player_visable_width, async_runtime);
		self.player.player_tick(&mut self.chunk_pool, input);
		self.player.tick(&mut self.chunk_pool);
	}
}