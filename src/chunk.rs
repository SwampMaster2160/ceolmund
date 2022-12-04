use std::{thread::{sleep_ms, sleep}, time::Duration};

use rand::{thread_rng, Rng};

use crate::{tile_stack::TileStack, vertex::Vertex};

pub struct Chunk {
	pub chunk_stacks: [Box<[TileStack; 64]>; 64],
	pub basic_vertices: Vec<Vertex>,
	pub extra_vertices: Vec<Vertex>,
}

impl Chunk {
	pub fn render(&mut self, pos: [i64; 2], vertices_in_out: &mut Vec<Vertex>) {
		let world_x = pos[0] * 64;
		let world_y = pos[1] * 64;
		for y in 0..64 {
			for x in 0..64 {
				let tile_stack = &mut self.chunk_stacks[y][x];
				if tile_stack.needs_redrawing {
					tile_stack.render(
						[world_x + x as i64, world_y + y as i64],
						(&mut self.basic_vertices[(y * 64 + x) * 48..(y * 64 + x + 1) * 48]).try_into().unwrap()
					);
				}
				vertices_in_out.extend(tile_stack.extra_vertices.iter());
			}
		}
		vertices_in_out.extend(self.basic_vertices.iter());
	}

	pub fn tick(&mut self, pos: &[i64; 2]) {
		let mut rng = thread_rng();
		let x: usize = rng.gen_range(0..64);
		let y: usize = rng.gen_range(0..64);
		let stack = &mut self.chunk_stacks[y][x];
		//*stack = TileStack::new();
	}

	pub fn new() -> Self {
		let mut vertices = Vec::new();
		vertices.reserve(48 * 64 * 64);
		for _ in 0..(48 * 64 * 64) {
			vertices.push(Vertex::new_null());
			/*for _ in 0..1000 {
				print!("");
			}*/
		}
		Self {
			chunk_stacks: [(); 64].map(|_| Box::new([(); 64].map(|_| TileStack::new()))),
			basic_vertices: vertices,
			extra_vertices: Vec::new(),
		}
	}

	pub async fn get() -> Self {
		Self::new()
	}
}