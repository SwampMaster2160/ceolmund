use noise::{Perlin, NoiseFn, Fbm};

use crate::{render::{vertex::Vertex, texture::Texture}, world::{entity::{entity::Entity, entity_action_state::EntityActionState, entity_type::EntityType}, direction::Direction4, item::item::Item}, io::{namespace::Namespace, file_reader::FileReader, file_writer::FileWriter}, error::Error};

use super::tile::{Tile, TileVariant};

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
		if self.tiles.len() == 0 {
			vertices.extend(Texture::Pit.render_basic(pos, [0, 0]));
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

	/// Create an empty tile stack.
	pub fn new() -> Self {
		Self {
			tiles: Vec::new(),
			needs_redrawing: true,
			extra_vertices: Vec::new(),
		}
	}

	/// Generates tiles onto an empty tile stack.
	pub fn generate(&mut self, pos: [i64; 2], world_seed: u32) {
		let height = Fbm::<Perlin>::new(world_seed).get([pos[0] as f64 / 64., pos[1] as f64 / 64.]);
		let sand_type = Fbm::<Perlin>::new(world_seed + 3).get([pos[0] as f64 / 64., pos[1] as f64 / 64.]);
		self.tiles = match height {
			_ if height < -0.1 => vec![Tile::Sand, Tile::Water],
			_ if height < 0.1 => match sand_type {
				_ if sand_type > 0.4 => vec![Tile::BlackSand],
				_ if sand_type < -0.4 => vec![Tile::Gravel],
				_ => vec![Tile::Sand],
			},
			_ if height > 0.9 => vec![Tile::Grass, Tile::PineTree],
			_ => vec![Tile::Grass],
		};
		let decoration_type = Fbm::<Perlin>::new(world_seed + 2).get([pos[0] as f64 * 4., pos[1] as f64 * 4.]);
		if Fbm::<Perlin>::new(world_seed + 1).get([pos[0] as f64, pos[1] as f64]) > 0.3 {
			if let Some(top_tile) = self.tiles.last() {
				match top_tile {
					Tile::Grass => {
						self.tiles.push( match decoration_type {
							_ if decoration_type < -0.3 => Tile::Item(Box::new(Item::Rock)),
							_ if decoration_type < -0.25 => Tile::Item(Box::new(Item::OakStick)),
							_ if decoration_type < -0.2 => Tile::PineTree,
							_ if decoration_type < 0. => Tile::OakTree,
							_ if decoration_type < 0.2 => Tile::Flowers,
							_ if decoration_type < 0.25 => Tile::Item(Box::new(Item::PineStick)),
							_ if decoration_type < 0.3 => Tile::Item(Box::new(Item::FlintRock)),
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
	}

	/// Called when an entity trys to move to this tile stack. If so, the entity start walking.
	pub fn entity_try_move_to(&mut self, entity: &mut Entity, direction: Direction4) {
		let mut walk = false;
		if let Some(top_tile) = self.tiles.last_mut() {
			walk = top_tile.entity_try_move_to(entity);
		}
		if self.tiles.len() == 0 {
			walk = true;
		}
		if walk {
			entity.action_state = EntityActionState::Walking(direction, 0);
		}
	}

	/// When an entity steps on the tile stack.
	pub fn entity_move_to(&mut self, entity: &mut Entity) {
		let inventory = match entity.entity_type {
			EntityType::Player { ref mut inventory, .. } => inventory,
			//_ => return,
		};
		for x in (0..self.tiles.len()).rev() {
			let tile = &mut self.tiles[x];
			let (stack_item, stack_amount) = match tile {
				Tile::DroppedItemStack(stack_item, stack_amount) => (stack_item, stack_amount),
				_ => continue,
			};
			let leftover = inventory.add_items((*stack_item.clone(), *stack_amount)).1;
			*stack_amount = leftover;
			if leftover == 0 {
				self.tiles.remove(x);
			}
		}
	}

	pub fn drop_item_onto(&mut self, to_drop_onto: (Item, u16)) {
		let item_to_add = to_drop_onto.0;
		let mut amount_left_to_add = to_drop_onto.1;
		// Return if the item is none.
		if item_to_add.is_none() {
			return;
		}
		// Add to exising dropped item stacks.
		for tile in &mut self.tiles {
			// Skip tiles that are not item stacks.
			let (stack_item, stack_amount) = match tile {
				Tile::DroppedItemStack(stack_item, stack_amount) => (stack_item, stack_amount),
				_ => continue,
			};
			// Skip stacks that are not the same as the item to add
			if **stack_item != item_to_add {
				continue;
			}
			// Calculate how many items to add to the stack and add them to the stack and remove them from the to add amount.
			let amount_to_add_to_stack = (u16::MAX - *stack_amount).min(amount_left_to_add);
			*stack_amount += amount_to_add_to_stack;
			amount_left_to_add -= amount_to_add_to_stack;
			// Return if there is nothing left to add
			if amount_left_to_add == 0 {
				return;
			}
		}
		// Add remaining items as a new stack.
		self.tiles.push(Tile::DroppedItemStack(Box::new(item_to_add), amount_left_to_add));
	}

	pub fn serialize(&self, file: &mut FileWriter) {
		for tile in &self.tiles {
			tile.serialize(file);
		}
		file.push_u8(TileVariant::None as u8);
	}

	pub fn load_v0(&mut self, tile_lengths: &[u8], tile_datas: &[u8], tile_lengths_index: &mut usize, tile_datas_index: &mut usize, namespace: &Namespace, version: u32) -> Option<()> {
		loop {
			let length = *tile_lengths.get(*tile_lengths_index)? as usize;
			*tile_lengths_index += 1;
			if length == 0 {
				break;
			}
			let tile_data = tile_datas.get(*tile_datas_index..*tile_datas_index + length)?;
			let (tile, length) = Tile::deserialize_v0(tile_data, namespace, version)?;
			self.tiles.push(tile);
			*tile_datas_index += length;
		}
		Some(())
	}

	pub fn deserialize(&mut self, file: &mut FileReader, namespace: &Namespace, version: u32) -> Result<(), Error> {
		loop {
			let tile_id = *file.data.get(file.read_index).ok_or(Error::OutOfBoundsFileRead)?;
			let variant = *namespace.tiles.get(tile_id as usize).ok_or(Error::IDOutOfNamespaceBounds)?;
			if variant == TileVariant::None {
				file.read_index += 1;
				break;
			}
			self.tiles.push(Tile::deserialize(file, namespace, version)?);
		}
		Ok(())
	}
}