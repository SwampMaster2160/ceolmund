use std::{fs::create_dir, path::PathBuf};

use crate::{world_pos_to_render_pos, render::vertex::Vertex, io::io::IO, gui::gui::GUI};

use super::{direction::Direction4, chunk::chunk_pool::ChunkPool, entity::{entity::Entity, entity_action_state::EntityActionState, entity_type::EntityType}};

/// Contains everthing visable that isn't the GUI.
pub struct World {
	player: Entity,
	chunk_pool: ChunkPool,
	seed: u32,
	pub is_freeing: bool,
	pub is_freed: bool,
	pub name: String,
	pub filepath: PathBuf,
	pub chunks_filepath: PathBuf,
}

impl World {
	pub fn new(seed: u32, name: String, io: &IO) -> Option<Self> {
		let dirname: String = name.chars().map(|chr| match chr {
			'/' | '\\' | '<' | '>' | ':' | '\'' | '|' | '?' | '*' | '.' | '~' | '#' | '%' | '&' | '+' | '-' | '{' | '}' | '@' | '"' | '!' | '`' | '=' => '_',
			_ => chr,
		}).collect();
		let mut filepath = io.worlds_path.clone();
		filepath.push(dirname);
		create_dir(&filepath).ok()?;
		let mut chunks_filepath = filepath.clone();
		chunks_filepath.push("chunks".to_string());
		create_dir(&chunks_filepath).ok()?;
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
			filepath,
			chunks_filepath
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

	/// Tick called when the game is not paused.
	pub fn tick(&mut self, io: &IO, player_visable_width: u64, gui: &mut GUI) {
		self.chunk_pool.tick(&self.player, player_visable_width, &io.async_runtime, self.seed);
		self.player.player_tick(&mut self.chunk_pool, io, gui);
		self.player.tick(&mut self.chunk_pool);
	}

	/// Tick always called.
	pub fn tick_always(&mut self, io: &IO, player_visable_width: u64, _gui: &mut GUI) {
		self.chunk_pool.tick_always(&self.player, player_visable_width, &io.async_runtime, self.seed, self.is_freeing, &mut self.is_freed);
	}
}