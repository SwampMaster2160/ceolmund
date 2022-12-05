use noise::{Perlin, NoiseFn, Fbm};

use crate::{render::vertex::Vertex, world::entity::{entity::Entity, entity_action_state::EntityActionState}};

use super::tile::Tile;

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

	pub fn generate(&mut self, pos: [i64; 2], world_seed: u32) {
		let height = Fbm::<Perlin>::new(world_seed).get([pos[0] as f64 / 64., pos[1] as f64 / 64.]);
		self.tiles = match height {
			_ if height < -0.1 => vec![Tile::Sand, Tile::Water],
			_ if height < 0.1 => vec![Tile::Sand],
			_ if height > 0.9 => vec![Tile::Grass, Tile::PineTree],
			_ => vec![Tile::Grass],
		};
		let decoration_type = Fbm::<Perlin>::new(world_seed + 2).get([pos[0] as f64 * 4., pos[1] as f64 * 4.]);
		if Fbm::<Perlin>::new(world_seed + 1).get([pos[0] as f64, pos[1] as f64]) > 0.3 {
			match self.tiles[self.tiles.len() - 1] {
				Tile::Grass => {
					self.tiles.push( match decoration_type {
						_ if decoration_type < -0.25 => Tile::PineTree,
						_ if decoration_type < 0. => Tile::OakTree,
						_ if decoration_type < 0.25 => Tile::Flowers,
						_ => Tile::FlowersRedYellow,
					});
				}
				Tile::Water => {
					if height > -0.3 {
						self.tiles.push(Tile::Rocks);
					}
					else {
						self.tiles.insert(self.tiles.len() - 1, Tile::Rocks)
					}
				}
				_ => {}
			}
		}
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