use std::{collections::HashMap, task::{Context, Waker, Poll}, thread::JoinHandle};

use futures::{FutureExt, executor::block_on, future::{poll_fn, Pending}, poll, Future};
use glium::{buffer::Content, glutin::ContextBuilder};
use tokio::{runtime::Runtime, task::JoinError};
use noop_waker::noop_waker;

use crate::{chunk_slot::ChunkSlot, chunk::Chunk, vertex::Vertex, tile_stack::TileStack};

pub struct ChunkPool {
	chunks: HashMap<[i64; 2], ChunkSlot>,
}

impl ChunkPool {
	pub fn new() -> Self {
		let mut out = Self {
			chunks: HashMap::new(),
		};
		let rt = Runtime::new().unwrap();
		//let mut chunk = GetChunk::new();
		//let a = block_on(chunk);
		//let waker = noop_waker();
		//let mut cx = Context::from_waker(&waker);

		//println!("{}", a.poll_unpin(&mut cx).is_ready());
		/*loop {
			match a.poll_unpin(&mut cx) {
				Poll::Ready(chunk) => {
					out.chunks.insert([0, 0], ChunkSlot::Chunk(chunk.unwrap()));
					break;
				},
				Poll::Pending => {
					println!("a");
				},
			}
			//println!("{}", a.poll_unpin(&mut cx).is_ready());
		}*/

		//poll_fn(f)
		//chunk.poll_unpin(cx);
		//out.chunks.insert([0, 0], ChunkSlot::Chunk(block_on(chunk)));
		/*out.chunks.insert([0, 0], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([-1, 0], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([0, -1], ChunkSlot::Chunk(Chunk::new()));
		out.chunks.insert([-1, -1], ChunkSlot::Chunk(Chunk::new()));*/
		/*let chunk_getting = rt.spawn(
			Chunk::get()
		);*/
		out.chunks.insert([0, 0], ChunkSlot::Getting(rt.spawn(Chunk::get())));
		out.chunks.insert([-1, 0], ChunkSlot::Getting(rt.spawn(Chunk::get())));
		out.chunks.insert([0, -1], ChunkSlot::Getting(rt.spawn(Chunk::get())));
		out.chunks.insert([-1, -1], ChunkSlot::Getting(rt.spawn(Chunk::get())));
		out
	}

	pub fn render(&mut self, vertices_in_out: &mut Vec<Vertex>) {
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.render(*pos, vertices_in_out);
			}
		}
	}

	pub fn tick(&mut self) {
		let waker = noop_waker();
		let mut cx = Context::from_waker(&waker);
		let rt = Runtime::new().unwrap();
		for (pos, chunk_slot) in self.chunks.iter_mut() {
			/*if let ChunkSlot::Chunk(chunk) = chunk_slot {
				chunk.tick(pos);
			}*/
			match chunk_slot {
				ChunkSlot::Chunk(chunk) => chunk.tick(pos),
				ChunkSlot::Getting(chunk_getting) => {
					if let Poll::Ready(chunk_or_err) = chunk_getting.poll_unpin(&mut cx) {
						match chunk_or_err {
							Err(error) => {
								println!("{:?}", error);
								*chunk_slot = ChunkSlot::Getting(rt.spawn(Chunk::get()));
							},
							Ok(chunk) => *chunk_slot = ChunkSlot::Chunk(chunk),
						}
						//*chunk_slot = ChunkSlot::Chunk(chunk.unwrap());
					}
					else {
						println!("a");
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