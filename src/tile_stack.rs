use rand::{thread_rng, Rng};

use crate::{tile::Tile, vertex::Vertex, entity::{Entity, entity_action_state::EntityActionState}};

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
		Self {
			tiles: Vec::new(),
			needs_redrawing: true,
			extra_vertices: Vec::new(),
		}
	}

	pub fn generate(&mut self, pos: [i64; 2]) {
		let mut rng = thread_rng();
		self.tiles = vec![match rng.gen_bool(0.9) {
			true => Tile::Grass,
			false => Tile::Water,
		}];
	}

	pub fn try_move_to(&mut self, entity: &mut Entity) {
		if let Some(top_tile) = self.tiles.last_mut() {
			let can_move = top_tile.try_move_to(entity);
			if can_move {
				entity.action_state = EntityActionState::Walking(0);
			}
		}
	}
}