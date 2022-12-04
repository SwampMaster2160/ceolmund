use std::{collections::HashMap, task::{Context, Poll}};

use futures::FutureExt;
use tokio::runtime::Runtime;
use noop_waker::noop_waker;

use crate::{chunk_slot::ChunkSlot, chunk::Chunk, vertex::Vertex, tile_stack::TileStack};

pub struct ChunkPool {
	chunks: HashMap<[i64; 2], ChunkSlot>,
}

impl ChunkPool {
	pub fn new(async_runtime: &Runtime) -> Self {
		let mut out = Self {
			chunks: HashMap::new(),
		};
		out.chunks.insert([0, 0], ChunkSlot::Getting(async_runtime.spawn(Chunk::get())));
		out.chunks.insert([-1, 0], ChunkSlot::Getting(async_runtime.spawn(Chunk::get())));
		out.chunks.insert([0, -1], ChunkSlot::Getting(async_runtime.spawn(Chunk::get())));
		out.chunks.insert([-1, -1], ChunkSlot::Getting(async_runtime.spawn(Chunk::get())));
		out
	}

	pub fn render(&mut self, vertices_in_out: &mut Vec<Vertex>) {
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.render(*pos, vertices_in_out);
			}
		}
	}

	pub fn tick(&mut self, async_runtime: &Runtime) {
		let waker = noop_waker();
		let mut cx = Context::from_waker(&waker);
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			match chunk_slot {
				ChunkSlot::Chunk(chunk) => chunk.tick(pos),
				ChunkSlot::Getting(chunk_getting) => {
					if let Poll::Ready(chunk) = chunk_getting.poll_unpin(&mut cx) {
						*chunk_slot = ChunkSlot::Chunk(chunk.unwrap());
					}
				}
			}
		}
	}

	pub fn get_tile_stack_at(&mut self, pos: [i64; 2]) -> Option<&mut TileStack> {
		if let ChunkSlot::Chunk(chunk) = self.chunks.get_mut(&[pos[0].div_euclid(64), pos[1].div_euclid(64)])? {
			return Some(&mut chunk.chunk_stacks[pos[1].rem_euclid(64) as usize][pos[0].rem_euclid(64) as usize])
		}
		None
	}
}