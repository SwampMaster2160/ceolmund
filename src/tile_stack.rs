use rand::{thread_rng, Rng};

use crate::{tile::Tile, vertex::Vertex};

#[derive(Clone)]
pub struct TileStack {
	pub tiles: Vec<Tile>,
	pub extra_vertices: Vec<Vertex>,
	pub needs_redrawing: bool,
}

impl TileStack {
	pub fn render(&mut self, pos: [i64; 2], basic_vertices: &mut [Vertex; 48]) {
		let mut vertices = Vec::new();
		for tile in self.tiles.iter_mut() {
			tile.render(pos, &mut vertices);
		}
		for x in 0..vertices.len().min(48) {
			basic_vertices[x] = vertices[x];
		}
		if vertices.len() < 48 {
			for x in vertices.len()..48 {
				basic_vertices[x] = Vertex::new_null();
			}
		}
		self.extra_vertices = Vec::new();
		for x in vertices.len().min(48)..vertices.len() {
			self.extra_vertices[x] = vertices[x];
		}
		self.needs_redrawing = false;
	}

	pub fn new() -> Self {
		let mut rng = thread_rng();
		let tile = match rng.gen_bool(0.5) {
			true => Tile::Grass,
			false => Tile::Water,
		};
		Self {
			tiles: vec![tile],
			needs_redrawing: true,
			extra_vertices: Vec::new(),
		}
	}
}