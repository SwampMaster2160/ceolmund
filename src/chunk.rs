use crate::{tile_stack::TileStack, vertex::Vertex, tile};

pub struct Chunk {
	chunk_stacks: [Box<[TileStack; 64]>; 64],
}

impl Chunk {
	pub fn render(&self, pos: [i64; 2]) -> Vec<Vertex> {
		let world_x = pos[0] * 64;
		let world_y = pos[1] * 64;
		let mut out = Vec::new();
		for y in 0..64 {
			for x in 0..64 {
				out.extend(self.chunk_stacks[x][y].render([world_x + x as i64, world_y + y as i64]));
			}
		}
		out
	}

	pub fn new() -> Self {
		Self {
			chunk_stacks: [(); 64].map(|_| Box::new([(); 64].map(|_| TileStack::new())))
		}
	}
}