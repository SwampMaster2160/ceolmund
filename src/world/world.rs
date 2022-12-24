use std::{fs::{create_dir, File}, path::PathBuf, io::Write};

use crate::{render::{vertex::Vertex, render::world_pos_to_render_pos}, io::{io::IO, formatted_file_writer::FormattedFileWriter, formatted_file_reader::FormattedFileReader}, gui::gui::GUI, validate_filename};

use super::{direction::Direction4, chunk::chunk_pool::ChunkPool, entity::{entity::Entity, entity_action_state::EntityActionState, entity_type::EntityType}};

/// Contains everthing visable that isn't the GUI.
pub struct World {
	player: Entity,
	chunk_pool: ChunkPool,
	seed: u32,
	pub is_freeing: bool, // If true, the world is saving all chunks and preparing to be deleted from RAM.
	pub is_freed: bool, // If true, the world is saved to disk and can be deleted from RAM.
	pub name: String,
	pub filepath: PathBuf, // Path to the world folder
	pub chunks_filepath: PathBuf,
	pub namespaces_filepath: PathBuf,
	pub overview_filepath: PathBuf,
}

impl World {
	/// Create a world from a name and seed.
	pub fn new(seed: u32, name: String, io: &IO) -> Option<Self> {
		// Convert the world name to a world folder filepath, converting character that are not filename safe to underscores. Then create the folder.
		let dirname: String = validate_filename(name.clone());
		let mut filepath = io.worlds_path.clone();
		filepath.push(dirname);
		create_dir(&filepath).ok()?;
		// Create overview path
		let mut overview_filepath = filepath.clone();
		overview_filepath.push("overview.wld".to_string());
		// Create dummy world object
		let dummy_world = Self {
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
			filepath: filepath.clone(),
			chunks_filepath: filepath.clone(),
			overview_filepath,
			namespaces_filepath: filepath.clone(),
		};
		dummy_world.save_overview();
		Self::load(filepath, io)
	}

	/// Load a world given the path to it's world folder.
	pub fn load(filepath: PathBuf, io: &IO) -> Option<Self> {
		// Get the path of the overview file for the world
		let mut overview_filepath = filepath.clone();
		overview_filepath.push("overview.wld".to_string());
		// Get and create paths of the chunks and namespaces folders for the world
		let mut chunks_filepath = filepath.clone();
		chunks_filepath.push("chunks".to_string());
		create_dir(&chunks_filepath).ok();
		let mut namespaces_filepath = filepath.clone();
		namespaces_filepath.push("namespaces".to_string());
		create_dir(&namespaces_filepath).ok();
		if !chunks_filepath.exists() || !namespaces_filepath.exists() {
			return None;
		}
		// Save namespace
		let mut namespace_filepath = namespaces_filepath.clone();
		namespace_filepath.push(format!("{:16x}.nsp", io.saving_namespace_hash));
		if !namespace_filepath.exists() {
			let mut file = File::create(namespace_filepath).ok()?;
			file.write(&io.saving_namespace).ok()?;
		}
		// Read overview
		let overview = FormattedFileReader::read_from_file(&overview_filepath)?;
		// Get world name
		let name_pos = overview.body.get(0..4)?;
		let name_pos: [u8; 4] = name_pos.try_into().ok()?;
		let name_pos = u32::from_le_bytes(name_pos);
		let name = overview.get_string(name_pos)?;
		// Get world seed
		let seed = overview.body.get(4..8)?;
		let seed: [u8; 4] = seed.try_into().ok()?;
		let seed = u32::from_le_bytes(seed);
		// Create world object
		Some(Self { 
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
			chunks_filepath,
			overview_filepath,
			namespaces_filepath,
		})
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
		self.chunk_pool.tick_always(&self.player, player_visable_width, &io.async_runtime, self.seed, self.is_freeing, &mut self.is_freed, &self.chunks_filepath, &self.namespaces_filepath, io.saving_namespace_hash);
	}

	pub fn save_overview(&self) {
		// Create file
		let mut file = FormattedFileWriter::new(0);
		// Push world name
		let name_pos = file.push_string(&self.name).unwrap().to_le_bytes();
		file.body.extend(name_pos);
		// Push seed
		let seed = self.seed.to_le_bytes();
		file.body.extend(seed);
		// Write file
		file.write(&self.overview_filepath);
	}
}