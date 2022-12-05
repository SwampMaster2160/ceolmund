use std::ops::Range;

use crate::{render::vertex::Vertex, world::tile::tile_stack::TileStack};

pub struct Chunk {
	pub tile_stacks: [Box<[TileStack; 64]>; 64],
	pub basic_vertices: Vec<Vertex>,
	pub extra_vertices: Vec<Vertex>,
}

impl Chunk {
	pub fn render(&mut self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>, render_range: &[Range<i64>; 2]) {
		let world_x = pos[0] * 64;
		let world_y = pos[1] * 64;

		let render_start_x = (render_range[0].start - world_x).clamp(0, 64) as usize;
		let render_start_y = (render_range[1].start - world_y).clamp(0, 64) as usize;
		let render_end_x = (render_range[0].end - world_x).clamp(0, 64) as usize;
		let render_end_y = (render_range[1].end - world_y).clamp(0, 64) as usize;

		let mut extra_vertices: Vec<Vertex> = Vec::new();
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

	pub fn tick(&mut self, _pos: &[i64; 2]) {
		
	}

	pub fn new() -> Self {
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

	pub fn generate(&mut self, pos: [i64; 2], seed: u32) {
		let tile_x_start = pos[0] * 64;
		let tile_y_start = pos[1] * 64;
		for x in 0..64 {
			for y in 0..64 {
				self.tile_stacks[y][x].generate([tile_x_start + x as i64, tile_y_start + y as i64], seed);
			}
		}
	}

	pub async fn get(pos: [i64; 2], seed: u32) -> Self {
		let mut out = Self::new();
		out.generate(pos, seed);
		out
	}

	pub async fn free(self, _pos: [i64; 2]) {
		
	}
}