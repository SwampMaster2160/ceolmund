use tokio::task::JoinHandle;

use super::chunk::Chunk;

/// A chunk slot, can be a chunk, a generating chunk or a freeing chunk.
pub enum ChunkSlot {
	Chunk(Chunk),
	Getting(JoinHandle<Option<Chunk>>),
	Freeing(JoinHandle<Option<()>>)
}

impl ChunkSlot {
	pub fn get_loaded_mut(&mut self) -> Option<&mut Chunk> {
		match self {
			Self::Chunk(chunk) => Some(chunk),
			_ => None,
		}
	}
}