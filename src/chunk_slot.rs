use tokio::task::JoinHandle;

use crate::chunk::Chunk;

pub enum ChunkSlot {
	Chunk(Chunk),
	Getting(JoinHandle<Chunk>),
	Freeing(JoinHandle<()>)
}