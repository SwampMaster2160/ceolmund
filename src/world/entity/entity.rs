use crate::{render::vertex::Vertex, io::{game_key::GameKey, io::IO}, world::{direction::Direction4, chunk::chunk_pool::ChunkPool, tile::tile::Tile, item::item::Item}, gui::{gui::GUI, gui_menu::GUIMenu, gui_menu_variant::GUIMenuVariant}};
use crate::world::tile::tile::TileVariant;
use super::{entity_action_state::EntityActionState, entity_type::EntityType};

/// A world object that is can move from tile to tile.
pub struct Entity {
	pub pos: [i64; 2],
	pub action_state: EntityActionState,
	pub facing: Direction4,
	pub entity_type: EntityType,
}

impl Entity {
	/// Get how many pixels an entity is offset from a tile (when walking).
	pub fn get_subtile_pos(&self) -> [i8; 2] {
		match self.action_state {
			EntityActionState::Idle => [0, 0],
			EntityActionState::Walking(walked_amount) => match self.facing {
				Direction4::North => [0, -(walked_amount as i8)],
				Direction4::East => [(walked_amount as i8), 0],
				Direction4::South => [0, (walked_amount as i8)],
				Direction4::West => [-(walked_amount as i8), 0],
			}
		}
	}

	/// A tick for the player that reads the io input.
	pub fn player_tick(&mut self, chunks: &mut ChunkPool, input: &IO, gui: &mut GUI) {
		if input.get_game_key_starting_now(GameKey::MenuOpenClose) {
			gui.menus.push(GUIMenu::new(GUIMenuVariant::Paused));
		}
		if self.action_state == EntityActionState::Idle {
			// Walk
			let mut try_move = true;
			if input.get_game_key(GameKey::WalkNorth) {
				self.facing = Direction4::North;
			}
			else if input.get_game_key(GameKey::WalkEast) {
				self.facing = Direction4::East;
			}
			else if input.get_game_key(GameKey::WalkSouth) {
				self.facing = Direction4::South;
			}
			else if input.get_game_key(GameKey::WalkWest) {
				self.facing = Direction4::West;
			}
			else {
				try_move = false;
			}
			if try_move {
				if let Some(tile_stack) = chunks.get_tile_stack_at_mut(self.get_pos_in_front()) {
					tile_stack.entity_try_move_to(self);
				}
			}
			// Change item
			let (inventory, selected_item) = match &mut self.entity_type {
				EntityType::Player { inventory, selected_item } => (inventory, selected_item),
			};
			let mut selected_item_x = (*selected_item % 10) as i8;
			let mut selected_item_y = (*selected_item / 10) as i8;
			if input.get_game_key_starting_now(GameKey::InventoryUp) {
				selected_item_y -= 1;
			}
			if input.get_game_key_starting_now(GameKey::InventoryDown) {
				selected_item_y += 1;
			}
			if input.get_game_key_starting_now(GameKey::InventoryLeft) {
				selected_item_x -= 1;
			}
			if input.get_game_key_starting_now(GameKey::InventoryRight) {
				selected_item_x += 1;
			}
			*selected_item = selected_item_x.rem_euclid(10) as u8 + selected_item_y.rem_euclid(5) as u8 * 10;

			if let Some(tile_stack_in_front) = chunks.get_tile_stack_at_mut(self.get_pos_in_front()) {
				let tiles = &mut tile_stack_in_front.tiles;
				if input.get_game_key_starting_now(GameKey::DeleteTile) {
					tiles.pop();
					tile_stack_in_front.needs_redrawing = true;
				}
				if tiles.len() == 0 {
					if input.get_game_key_starting_now(GameKey::Build1) {
						tiles.push(Tile::Grass);
						tile_stack_in_front.needs_redrawing = true;
						return;
					}
					if input.get_game_key_starting_now(GameKey::Build2) {
						tiles.push(Tile::Sand);
						tile_stack_in_front.needs_redrawing = true;
					}
					if input.get_game_key_starting_now(GameKey::Build3) {
						tiles.push(Tile::Gravel);
						tile_stack_in_front.needs_redrawing = true;
					}
					if input.get_game_key_starting_now(GameKey::Build4) {
						tiles.push(Tile::BlackSand);
						tile_stack_in_front.needs_redrawing = true;
					}
				}
				if input.get_game_key_starting_now(GameKey::Build5) && tiles.len() == 1 {
					tiles.push(Tile::Water);
					tile_stack_in_front.needs_redrawing = true;
				}
				if let Some(top_tile) = tiles.last() {
					let tile_variant: TileVariant = top_tile.into();
					if tile_variant == TileVariant::Grass {
						if input.get_game_key_starting_now(GameKey::Build6) {
							tiles.push(Tile::PineTree);
							tile_stack_in_front.needs_redrawing = true;
						}
						if input.get_game_key_starting_now(GameKey::Build7) {
							tiles.push(Tile::OakTree);
							tile_stack_in_front.needs_redrawing = true;
						}
						if input.get_game_key_starting_now(GameKey::Build8) {
							tiles.push(Tile::Flowers);
							tile_stack_in_front.needs_redrawing = true;
						}
						if input.get_game_key_starting_now(GameKey::Build9) {
							tiles.push(Tile::FlowersRedYellow);
							tile_stack_in_front.needs_redrawing = true;
						}
					}
					if tile_variant == TileVariant::Grass || tile_variant == TileVariant::Water {
						if input.get_game_key_starting_now(GameKey::Build0) {
							tiles.push(Tile::Rocks);
							tile_stack_in_front.needs_redrawing = true;
						}
					}
					if tile_variant == TileVariant::Grass || tile_variant == TileVariant::BlackSand ||
					tile_variant == TileVariant::Gravel || tile_variant == TileVariant::Sand {
						if input.get_game_key_starting_now(GameKey::Build1) {
							tiles.push(Tile::Path);
							tile_stack_in_front.needs_redrawing = true;
						}
					}
				}
			}
		}
	}

	/// A tick for all entities.
	pub fn tick(&mut self, _chunks: &mut ChunkPool) {
		match &mut self.action_state {
			EntityActionState::Idle => {},
			EntityActionState::Walking(amount) => {
				*amount += 1;
				if *amount > 15 {
					self.action_state = EntityActionState::Idle;
					self.pos = self.get_pos_in_front();
				}
			}
		}
	}

	/// Get the pos of the tile directly in front of the entity.
	pub fn get_pos_in_front(&self) -> [i64; 2] {
		let pos = self.pos;
		match self.facing {
			Direction4::North => [pos[0], pos[1] - 1],
			Direction4::East => [pos[0] + 1, pos[1]],
			Direction4::South => [pos[0], pos[1] + 1],
			Direction4::West => [pos[0] - 1, pos[1]],
		}
	}

	/// Get a vertex of tris for the entity.
	pub fn render(&self, vertices_in_out: &mut Vec<Vertex>) {
		let texture = self.entity_type.get_texture();
		vertices_in_out.extend(texture.render_entity(self.pos, self.get_subtile_pos(), self.facing, match self.action_state {
			EntityActionState::Walking(amount) => amount / 8 + 1,
			EntityActionState::Idle => 0,
		}));
	}

	/// Create a neew player at 0, 0
	pub fn new_player() -> Self {
		let mut inventory = Box::new([(); 50].map(|_| (Item::None, 0)));
		inventory[0] = (Item::SandboxDestroyWand, 1);
		inventory[1] = (Item::BaseTilePlacer(Tile::Grass), 1);
		inventory[2] = (Item::BaseTilePlacer(Tile::Gravel), 1);
		inventory[3] = (Item::BaseTilePlacer(Tile::Sand), 1);
		inventory[4] = (Item::BaseTilePlacer(Tile::BlackSand), 1);
		Entity {
			pos: [0, 0],
			action_state: EntityActionState::Idle,
			facing: Direction4::South,
			entity_type: EntityType::Player { inventory, selected_item: 0 },
		}
	}
}