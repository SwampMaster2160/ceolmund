use std::{ops::Range, path::PathBuf};

use crate::{render::vertex::Vertex, world::tile::tile_stack::TileStack, io::{formatted_file_writer::FormattedFileWriter, formatted_file_reader::FormattedFileReader, namespace::Namespace}};

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
	pub async fn get(pos: [i64; 2], chunks_filepath: PathBuf, namespaces_filepath: PathBuf, seed: u32) -> Option<Self> {
		// Create blank chunk
		let mut out = Self::new_blank();
		// Try to load chunk otherwise generate said chunk
		if !out.load(pos, chunks_filepath, namespaces_filepath)? {
			out.generate(pos, seed);
		}
		
		Some(out)
	}

	/// Load chunk, returning weather it exists or not wrapped in an option that is none when there is an error loading the chunk.
	pub fn load(&mut self, pos: [i64; 2], chunks_filepath: PathBuf, namespaces_filepath: PathBuf) -> Option<bool> {
		// Get filepath for chunk and load
		let mut chunk_filepath = chunks_filepath.clone();
		chunk_filepath.push(format!("{} {}.cnk", pos[0], pos[1]));
		let (file, _is_version_0) = match FormattedFileReader::read_from_file(&chunk_filepath) {
			Some(file) => file,
			None => return Some(false),
		};
		/*if file.version > SERIALIZATION_VERSION {
			return None;
		}*/
		// Get chunk namespace hash
		let namespace_hash: [u8; 8] = file.body.get(0..8)?.try_into().ok()?;
		let namespace_hash = u64::from_le_bytes(namespace_hash);
		// Get namespace
		let namespace = Namespace::load(namespace_hash, namespaces_filepath.clone())?;
		// Get pointer to tile datas
		let tile_datas_ptr: [u8; 4] = file.body.get(8..12)?.try_into().ok()?;
		let tile_datas_ptr = u32::from_le_bytes(tile_datas_ptr) as usize;
		// Get datas
		let tile_lengths = file.body.get(12..tile_datas_ptr)?;
		let tile_datas = file.body.get(tile_datas_ptr..)?;
		// Go over each chunk
		let mut tile_lengths_index = 0usize;
		let mut tile_datas_index = 0usize;
		for tile_stack_row in &mut self.tile_stacks {
			for tile_stack in tile_stack_row.iter_mut() {
				tile_stack.load(tile_lengths, tile_datas, &mut tile_lengths_index, &mut tile_datas_index, &namespace, namespace.version)?;
			}
		}
		//
		Some(true)
	}

	/// Save chunk
	pub async fn save(self, pos: [i64; 2], chunks_filepath: PathBuf, namespace_hash: u64) -> Option<()> {
		// Open file
		let mut file = FormattedFileWriter::new(/*SERIALIZATION_VERSION*/);
		// Push namespace hash
		file.body.extend(namespace_hash.to_le_bytes());
		// Create file arrays
		let mut tile_lengths: Vec<u8> = Vec::new();
		let mut tile_datas: Vec<u8> = Vec::new();
		// Get tile datas
		for tile_stack_row in &self.tile_stacks {
			for tile_stack in tile_stack_row.iter() {
				tile_stack.serialize(&mut tile_lengths, &mut tile_datas);
			}
		}
		// Push tile datas pointer
		let tile_datas_ptr: u32 = (12 + tile_lengths.len()).try_into().unwrap();
		file.body.extend(tile_datas_ptr.to_le_bytes());
		// Push tile lengths and datas
		file.body.extend(tile_lengths);
		file.body.extend(tile_datas);
		// Get filepath for chunk and save
		let mut chunk_filepath = chunks_filepath.clone();
		chunk_filepath.push(format!("{} {}.cnk", pos[0], pos[1]));
		file.write(&chunk_filepath)?;
		Some(())
	}
}