use tokio::task::JoinHandle;

use super::chunk::Chunk;

pub enum ChunkSlot {
	Chunk(Chunk),
	Getting(JoinHandle<Option<Chunk>>),
	Freeing(JoinHandle<Option<()>>)
}