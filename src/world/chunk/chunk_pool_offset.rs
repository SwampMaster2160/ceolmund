use crate::world::tile::tile_stack::TileStack;

use super::chunk_pool::ChunkPool;

pub struct ChunkPoolOffset<'a> {
	pub chunk_pool: &'a mut ChunkPool,
	pub offset: [i64; 2],
}

impl ChunkPoolOffset<'_> {
	/// Get the tile stack at the world pos wrapped in Some if the chunk it is in is loaded, else get None.
	pub fn get_tile_stack_at_mut(&mut self, pos: [i64; 2]) -> Option<&mut TileStack> {
		let pos = [pos[0] + self.offset[0], pos[1] + self.offset[1]];
		self.chunk_pool.get_tile_stack_at_mut(pos)
	}

	/// Get the tile stack at 0, 0 wrapped in Some if the chunk it is in is loaded, else get None.
	pub fn get_origin_tile_stack_mut(&mut self) -> Option<&mut TileStack> {
		self.chunk_pool.get_tile_stack_at_mut(self.offset)
	}
}