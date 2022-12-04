use std::{collections::HashMap, task::{Context, Poll}};

use futures::FutureExt;
use tokio::runtime::Runtime;
use noop_waker::noop_waker;

use crate::{chunk_slot::ChunkSlot, chunk::Chunk, vertex::Vertex, tile_stack::TileStack, entity::Entity};

pub struct ChunkPool {
	chunks: HashMap<[i64; 2], ChunkSlot>,
}

impl ChunkPool {
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
		}
	}

	pub fn render(&mut self, vertices_in_out: &mut Vec<Vertex>) {
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.render(*pos, vertices_in_out);
			}
		}
	}

	pub fn tick(&mut self, player: &Entity, player_visable_width: u64, async_runtime: &Runtime) {
		let waker = noop_waker();
		let mut cx = Context::from_waker(&waker);

		// Get the bounds of what should be generated.
		let chunk_y_to_load_start = player.pos[1].div_euclid(64) - 1;
		let chunk_y_to_load_end = chunk_y_to_load_start + 2;
		let chunk_x_to_load_start = (player.pos[0] - player_visable_width as i64 / 2).div_euclid(64) - 1;
		let chunk_x_to_load_end = (player.pos[0] + player_visable_width as i64 / 2).div_euclid(64) + 1;

		// Start generating chunks if in bounds and not loaded.
		for y in chunk_y_to_load_start..=chunk_y_to_load_end {
			for x in chunk_x_to_load_start..=chunk_x_to_load_end {
				let pos = [x, y];
				if !self.chunks.contains_key(&pos) {
					self.chunks.insert(pos, ChunkSlot::Getting(async_runtime.spawn(Chunk::get(pos))));
				}
			}
		}

		// Run over all loaded chunks.
		let mut to_free: Vec<[i64; 2]> = Vec::new();
		let mut to_remove: Vec<[i64; 2]> = Vec::new();
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			match chunk_slot {
				ChunkSlot::Chunk(chunk) => {
					if (chunk_x_to_load_start..=chunk_x_to_load_end).contains(&pos[0]) && (chunk_y_to_load_start..=chunk_y_to_load_end).contains(&pos[1]) {
						chunk.tick(pos);
					}
					else {
						to_free.push(*pos);
					}
				},
				ChunkSlot::Getting(chunk_getting) => {
					if let Poll::Ready(chunk) = chunk_getting.poll_unpin(&mut cx) {
						*chunk_slot = ChunkSlot::Chunk(chunk.unwrap());
						println!("Generated chunk {:?}", pos);
					}
				}
				ChunkSlot::Freeing(chunk_freeing) => {
					if let Poll::Ready(_) = chunk_freeing.poll_unpin(&mut cx) {
						to_remove.push(*pos);
						println!("Freed chunk {:?}", pos);
					}
				}
			}
		}
		for pos in to_free.iter() {
			if let ChunkSlot::Chunk(chunk) = self.chunks.remove(pos).unwrap() {
				self.chunks.insert(*pos, ChunkSlot::Freeing(async_runtime.spawn(chunk.free(*pos))));
			}
		}
		for pos in to_remove.iter() {
			self.chunks.remove(pos);
		}
	}

	pub fn get_tile_stack_at(&mut self, pos: [i64; 2]) -> Option<&mut TileStack> {
		if let ChunkSlot::Chunk(chunk) = self.chunks.get_mut(&[pos[0].div_euclid(64), pos[1].div_euclid(64)])? {
			return Some(&mut chunk.tile_stacks[pos[1].rem_euclid(64) as usize][pos[0].rem_euclid(64) as usize])
		}
		None
	}
}