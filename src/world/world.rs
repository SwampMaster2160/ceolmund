use std::fs::create_dir;

use tokio::runtime::Runtime;

use crate::{world_pos_to_render_pos, render::vertex::Vertex, io::input::Input, gui::gui::GUI};

use super::{direction::Direction4, chunk::chunk_pool::ChunkPool, entity::{entity::Entity, entity_action_state::EntityActionState, entity_type::EntityType}};

/// Contains everthing visable that isn't the GUI.
pub struct World {
	player: Entity,
	chunk_pool: ChunkPool,
	seed: u32,
	pub is_freeing: bool,
	pub is_freed: bool,
	name: String,
	filename: String,
}

impl World {
	pub fn new(seed: u32, name: String) -> Option<Self> {
		let filename: String = name.chars().map(|chr| match chr {
			'/' | '\\' | '<' | '>' | ':' | '\'' | '|' | '?' | '*' | '.' | '~' | '#' | '%' | '&' | '+' | '-' | '{' | '}' | '@' | '"' | '!' | '`' | '=' => '_',
			_ => chr,
		}).collect();
		create_dir(&filename).ok()?;
		let out = Self { 
			player: Entity {
				pos: [0, 0],
				action_state: EntityActionState::Idle,
    			facing: Direction4::South,
				entity_type: EntityType::Player,
			},
			chunk_pool: ChunkPool::new(),
			seed,
			is_freeing: false,
			is_freed: false,
			name,
			filename,
		};
		Some(out)
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

	pub fn tick(&mut self, input: &Input, async_runtime: &Runtime, player_visable_width: u64, gui: &mut GUI) {
		self.chunk_pool.tick(&self.player, player_visable_width, async_runtime, self.seed);
		self.player.player_tick(&mut self.chunk_pool, input, gui);
		self.player.tick(&mut self.chunk_pool);
	}

	pub fn tick_always(&mut self, _input: &Input, async_runtime: &Runtime, player_visable_width: u64, _gui: &mut GUI) {
		self.chunk_pool.tick_always(&self.player, player_visable_width, async_runtime, self.seed, self.is_freeing, &mut self.is_freed);
	}
}