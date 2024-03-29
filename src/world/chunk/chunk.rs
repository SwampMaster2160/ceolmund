use std::{ops::Range, path::PathBuf};

use crate::{render::vertex::Vertex, world::tile::tile_stack::TileStack, io::{file_writer::FileWriter, file_reader::FileReader, namespace::Namespace}, error::Error};

/// A 64x64 grid of tile stacks
pub struct Chunk {
	pub tile_stacks: [Box<[TileStack; 64]>; 64],
	pub basic_vertices: Vec<Vertex>,
	pub extra_vertices: Vec<Vertex>,
}

impl Chunk {
	pub fn render(&mut self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>, render_range: &[Range<i64>; 2]) {
		// Get the tile pos of the chunk
		let world_x = pos[0] * 64;
		let world_y = pos[1] * 64;
		// Get where to start and stop rendering the chunk
		let render_start_x = (render_range[0].start - world_x).clamp(0, 64) as usize;
		let render_start_y = (render_range[1].start - world_y).clamp(0, 64) as usize;
		let render_end_x = (render_range[0].end - world_x).clamp(0, 64) as usize;
		let render_end_y = (render_range[1].end - world_y).clamp(0, 64) as usize;

		let mut extra_vertices: Vec<Vertex> = Vec::new();
		// For each tile in the render region
		for y in render_start_y..render_end_y {
			for x in render_start_x..render_end_x {
				let tile_stack = &mut self.tile_stacks[y][x];
				if tile_stack.needs_redrawing {
					tile_stack.render(
						[world_x + x as i64, world_y + y as i64],
						(&mut self.basic_vertices[(y * 64 + x) * 48..(y * 64 + x + 1) * 48]).try_into().unwrap()
					);
				}
				extra_vertices.extend(tile_stack.extra_vertices.iter());
			}
		}
		vertices_in_out.extend(self.basic_vertices[render_start_y * 64 * 48..render_end_y * 64 * 48].iter());
		vertices_in_out.extend(extra_vertices.iter());
	}

	/// Do a tick on the chunk
	pub fn tick(&mut self, _pos: &[i64; 2]) {
		
	}

	/// Create a chunk consisting of blank tile stacks.
	pub fn new_blank() -> Self {
		let mut vertices = Vec::new();
		vertices.reserve(48 * 64 * 64);
		for _ in 0..(48 * 64 * 64) {
			vertices.push(Vertex::new_null());
		}
		Self {
			tile_stacks: [(); 64].map(|_| Box::new([(); 64].map(|_| TileStack::new()))),
			basic_vertices: vertices,
			extra_vertices: Vec::new(),
		}
	}

	/// Generate a chunk using the world generator
	pub fn generate(&mut self, pos: [i64; 2], seed: u32) {
		// Get the tile pos of the chunk
		let tile_x_start = pos[0] * 64;
		let tile_y_start = pos[1] * 64;
		// For each tile
		for x in 0..64 {
			for y in 0..64 {
				// Generate tile
				self.tile_stacks[y][x].generate([tile_x_start + x as i64, tile_y_start + y as i64], seed);
			}
		}
	}

	/// Load or generate chunk
	pub async fn get(pos: [i64; 2], chunks_filepath: PathBuf, namespaces_filepath: PathBuf, seed: u32) -> Result<Self, Error> {
		// Create blank chunk
		let mut out = Self::new_blank();
		// Try to load chunk otherwise generate said chunk
		if !out.load(pos, chunks_filepath, namespaces_filepath)? {
			out.generate(pos, seed);
		}
		
		Ok(out)
	}

	/// Load chunk, returning weather it exists or not wrapped in an option that is none when there is an error loading the chunk.
	pub fn load(&mut self, pos: [i64; 2], chunks_filepath: PathBuf, namespaces_filepath: PathBuf) -> Result<bool, Error> {
		// Get filepath for chunk and load
		let mut chunk_filepath = chunks_filepath.clone();
		chunk_filepath.push(format!("{} {}.cnk", pos[0], pos[1]));
		let (mut file, _is_version_0) = match FileReader::read_from_file(&chunk_filepath) {
			Ok(file) => file,
			Err(_) => return Ok(false),
		};
		// Get chunk namespace hash
		let namespace_hash = file.read_u64()?;
		// Get namespace
		let namespace = Namespace::load(namespace_hash, namespaces_filepath.clone())?;
		
		if namespace.version == 0 {
			self.load_v0(&mut file, namespace).ok_or(Error::V0Error)?;
			return Ok(true);
		}

		for tile_stack_row in &mut self.tile_stacks {
			for tile_stack in tile_stack_row.iter_mut() {
				tile_stack.deserialize(&mut file, &namespace, namespace.version)?;
			}
		}
		//
		Ok(true)
	}

	pub fn load_v0(&mut self, file: &mut FileReader, namespace: Namespace) -> Option<()> {
		// Get pointer to tile datas
		let tile_datas_ptr = file.read_u32().ok()? as usize;
		// Get datas
		let tile_lengths = file.data.get(12..tile_datas_ptr)?;
		let tile_datas = file.data.get(tile_datas_ptr..)?;
		// Go over each chunk
		let mut tile_lengths_index = 0usize;
		let mut tile_datas_index = 0usize;
		for tile_stack_row in &mut self.tile_stacks {
			for tile_stack in tile_stack_row.iter_mut() {
				tile_stack.load_v0(tile_lengths, tile_datas, &mut tile_lengths_index, &mut tile_datas_index, &namespace, namespace.version)?;
			}
		}
		//
		Some(())
	}

	/// Save chunk
	pub async fn save(self, pos: [i64; 2], chunks_filepath: PathBuf, namespace_hash: u64) -> Result<(), Error> {
		// Open file
		let mut file = FileWriter::new();
		// Push namespace hash
		file.push_u64(namespace_hash);
		// Get tile datas
		for tile_stack_row in &self.tile_stacks {
			for tile_stack in tile_stack_row.iter() {
				tile_stack.serialize(&mut file);
			}
		}
		// Get filepath for chunk and save
		let mut chunk_filepath = chunks_filepath.clone();
		chunk_filepath.push(format!("{} {}.cnk", pos[0], pos[1]));
		file.write(&chunk_filepath).ok_or(Error::CannotReadToFile)?;
		Ok(())
	}
}