use tokio::task::JoinHandle;

use super::chunk::Chunk;

pub enum ChunkSlot {
	Chunk(Chunk),
	Getting(JoinHandle<Chunk>),
	Freeing(JoinHandle<Option<()>>)
}