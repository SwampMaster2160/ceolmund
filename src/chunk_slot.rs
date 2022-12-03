use std::future::Future;

use crate::chunk::Chunk;

pub enum ChunkSlot {
	Chunk(Chunk),
	//Getting(Future<Chunk>),
}