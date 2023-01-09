use std::{collections::HashMap, task::{Context, Poll}, path::PathBuf};

use futures::FutureExt;
use tokio::runtime::Runtime;
use noop_waker::noop_waker;

use crate::{render::vertex::Vertex, world::{direction::Direction4, tile::tile_stack::TileStack, entity::{entity::Entity, entity_action_state::EntityActionState}}};

use super::{chunk_slot::ChunkSlot, chunk::Chunk, chunk_pool_offset::ChunkPoolOffset};

/// A struct that holds all the chunks weather loaded, loading or freeing.
pub struct ChunkPool {
	chunks: HashMap<[i64; 2], ChunkSlot>,
}

impl ChunkPool {
	pub fn new() -> Self {
		Self {
			chunks: HashMap::new(),
		}
	}

	/// Rendef all tiles in the visable area.
	pub fn render(&mut self, player: &Entity, player_visable_width: u64, vertices_in_out: &mut Vec<Vertex>) {
		// Get visable area
		let mut render_start_x = player.pos[0] - player_visable_width as i64 / 2;
		let mut render_end_x = player.pos[0] + player_visable_width as i64 / 2 + 1;
		let mut render_start_y = player.pos[1] - 8;
		let mut render_end_y = player.pos[1] + 9;
		// If the player is walking then extend the visable area by one in that direction
		match player.action_state {
			EntityActionState::Walking(direction, ..) => match direction {
				Direction4::North => render_start_y -= 1,
				Direction4::East => render_end_x += 1,
				Direction4::South => render_end_y += 1,
				Direction4::West => render_start_x -= 1,
			}
			_ => {},
		}

		let render_range = [render_start_x..render_end_x, render_start_y..render_end_y];
		// Render chunks that are not loading or freeing
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.render(*pos, vertices_in_out, &render_range);
			}
		}
	}

	/// Tick the chunks
	pub fn tick(&mut self, _player: Option<&Entity>, _player_visable_width: u64, _async_runtime: &Runtime, _seed: u32) {

	}

	/// Tick that should always be called even if the game is paused.
	pub fn tick_always(&mut self, player: Option<&Entity>, player_visable_width: u64, async_runtime: &Runtime, seed: u32, is_freeing: bool, is_freed: &mut bool, chunks_filepath: &PathBuf, namespaces_filepath: &PathBuf, namespace_hash: u64) {
		// Dummy thread context (used and discarded, wakers are discarded).
		let waker = noop_waker();
		let mut cx = Context::from_waker(&waker);

		// Get the bounds of what should be loaded.
		let mut chunk_y_to_load_start = 0;
		let mut chunk_y_to_load_end = 0;
		let mut chunk_x_to_load_start = 0;
		let mut chunk_x_to_load_end = 0;

		if let Some(player) = player {
			chunk_y_to_load_start = player.pos[1].div_euclid(64) - 1;
			chunk_y_to_load_end = chunk_y_to_load_start + 2;
			chunk_x_to_load_start = (player.pos[0] - player_visable_width as i64 / 2).div_euclid(64) - 1;
			chunk_x_to_load_end = (player.pos[0] + player_visable_width as i64 / 2).div_euclid(64) + 1;
		}

		// Start generating chunks if in bounds and not loaded.
		if !is_freeing {
			for y in chunk_y_to_load_start..=chunk_y_to_load_end {
				for x in chunk_x_to_load_start..=chunk_x_to_load_end {
					let pos = [x, y];
					if !self.chunks.contains_key(&pos) {
						self.chunks.insert(pos, ChunkSlot::Getting(async_runtime.spawn(Chunk::get(pos, chunks_filepath.clone(), namespaces_filepath.clone(), seed))));
					}
				}
			}
		}

		// Run over all loaded chunks.
		let mut to_free: Vec<[i64; 2]> = Vec::new();
		let mut to_remove: Vec<[i64; 2]> = Vec::new();
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			match chunk_slot {
				// Free loaded chunks if out of load bounds.
				ChunkSlot::Chunk(chunk) => {
					if (chunk_x_to_load_start..=chunk_x_to_load_end).contains(&pos[0]) && (chunk_y_to_load_start..=chunk_y_to_load_end).contains(&pos[1])
					&& !is_freeing {
						chunk.tick(pos);
					}
					else {
						to_free.push(*pos);
					}
				}
				// Unwrap a loading chunk if it has been loaded and add to loaded chunks.
				ChunkSlot::Getting(chunk_getting) => {
					if let Poll::Ready(chunk) = chunk_getting.poll_unpin(&mut cx) {
						*chunk_slot = ChunkSlot::Chunk(chunk.unwrap().unwrap());
					}
				}
				// If a chunk is finished freeing then finally delete it.
				ChunkSlot::Freeing(chunk_freeing) => {
					if let Poll::Ready(_) = chunk_freeing.poll_unpin(&mut cx) {
						to_remove.push(*pos);
					}
				}
			}
		}
		for pos in to_free.iter() {
			if let ChunkSlot::Chunk(chunk) = self.chunks.remove(pos).unwrap() {
				self.chunks.insert(*pos, ChunkSlot::Freeing(async_runtime.spawn(chunk.save(*pos, chunks_filepath.clone(), namespace_hash))));
			}
		}
		for pos in to_remove.iter() {
			self.chunks.remove(pos);
		}
		if is_freeing && self.chunks.len() == 0 {
			*is_freed = true;
		}
	}

	/// Get the tile stack at the world pos wrapped in Some if the chunk it is in is loaded, else get None.
	pub fn get_tile_stack_at_mut(&mut self, pos: [i64; 2]) -> Option<&mut TileStack> {
		let chunk_slot = self.chunks.get_mut(&[pos[0].div_euclid(64), pos[1].div_euclid(64)])?;
		let chunk = chunk_slot.get_loaded_mut()?;
		Some(&mut chunk.tile_stacks[pos[1].rem_euclid(64) as usize][pos[0].rem_euclid(64) as usize])
	}

	/// Get an offset version of the chunk.
	pub fn get_offset(&mut self, offset: [i64; 2]) -> ChunkPoolOffset {
		ChunkPoolOffset {
			chunk_pool: self,
			offset,
		}
	}
}