use rand::{thread_rng, Rng};

use crate::{tile::Tile, vertex::Vertex};

#[derive(Clone)]
pub struct TileStack {
	tiles: Vec<Tile>,
}

impl TileStack {
	pub fn render(&self, pos: [i64; 2]) -> Vec<Vertex> {
		self.tiles.iter().map(|tile| tile.render(pos)).flatten().collect()
	}

	pub fn new() -> Self {
		let mut rng = thread_rng();
		let tile = match rng.gen_bool(0.5) {
			true => Tile::Grass,
			false => Tile::Water,
		};
		Self {
			tiles: vec![tile],
		}
	}
}