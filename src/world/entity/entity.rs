use std::path::PathBuf;

use crate::{render::vertex::Vertex, io::{game_key::GameKey, io::IO, file_writer::FileWriter, file_reader::FileReader, namespace::Namespace}, world::{direction::Direction4, chunk::chunk_pool::ChunkPool, tile::tile::Tile, item::{item::Item, inventory::Inventory}, difficulty::Difficulty}, gui::{gui::GUI, gui_menu::GUIMenu, gui_menu_variant::GUIMenuVariant}};
use super::{entity_action_state::EntityActionState, entity_type::{EntityType, EntityVariant}};

/// A world object that is can move from tile to tile.
pub struct Entity {
	pub pos: [i64; 2],
	pub facing: Direction4,
	pub action_state: EntityActionState,
	pub entity_type: EntityType,
	pub health: u32,
}

impl Entity {
	/// Get how many pixels an entity is offset from a tile (when walking).
	pub fn get_subtile_pos(&self) -> [i8; 2] {
		match self.action_state {
			EntityActionState::Idle => [0, 0],
			EntityActionState::Walking(walking_direction, walked_amount) => match walking_direction {
				Direction4::North => [0, -(walked_amount as i8)],
				Direction4::East => [(walked_amount as i8), 0],
				Direction4::South => [0, (walked_amount as i8)],
				Direction4::West => [-(walked_amount as i8), 0],
			}
		}
	}

	/// A tick for the player that reads the io input.
	pub fn player_tick(&mut self, chunks: &mut ChunkPool, input: &IO, gui: &mut GUI, difficulty: Difficulty) {
		let pos_in_front = self.get_pos_in_front();
		if input.get_game_key_starting_now(GameKey::MenuOpenClose) {
			gui.menus.push(GUIMenu::new(GUIMenuVariant::Paused));
		}
		let action_state = self.action_state.clone();
		if action_state == EntityActionState::Idle {
			// Change item
			let (inventory, selected_item) = match &mut self.entity_type {
				EntityType::Player { inventory, selected_item, .. } => (inventory, selected_item),
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

			// Interact with world
			let mut chunks_offset = chunks.get_offset(pos_in_front);
			if input.get_game_key_starting_now(GameKey::Interact) || (input.get_game_key(GameKey::Turbo) && input.get_game_key(GameKey::Interact)) {
				// Get the item stack selected.
				let item_stack = &mut inventory.items[*selected_item as usize];
				// Use the item and get back drops.
				let (consume_item, drops) = Item::use_stack_mut_self(item_stack, &mut chunks_offset);
				// Consume item and get drops if not in sandbox mode.
				if difficulty != Difficulty::Sandbox {
					if consume_item {
						item_stack.0.consume_item(&mut item_stack.1);
					}
					for drop in drops {
						// Randomize drop.
						let drop_rolled = drop.roll();
						// Add to player inventory.
						let to_drop_on_floor = inventory.add_items(drop_rolled);
						// Drop items that cannot be added to the players inventory on the floor at the tile the player is standing on.
						match chunks_offset.get_tile_stack_at_mut(self.pos) {
							Some(tile_stack) => tile_stack.drop_item_onto(to_drop_on_floor),
							None => {},
						}
					}
				}
			}
			// Walk
			let mut try_move = !input.get_game_key(GameKey::ChangeDirectionInplace);
			let mut direction = self.facing;
			if input.get_game_key(GameKey::WalkNorth) {
				direction = Direction4::North;
			}
			else if input.get_game_key(GameKey::WalkEast) {
				direction = Direction4::East;
			}
			else if input.get_game_key(GameKey::WalkSouth) {
				direction = Direction4::South;
			}
			else if input.get_game_key(GameKey::WalkWest) {
				direction = Direction4::West;
			}
			else {
				try_move = false;
			}
			if !input.get_game_key(GameKey::MoveWithoutChangingDirection) {
				self.facing = direction;
			}
			if try_move {
				if let Some(tile_stack) = chunks.get_tile_stack_at_mut(self.get_pos_in_direction(direction)) {
					tile_stack.entity_try_move_to(self, direction);
				}
			}
		}
	}

	/// A tick for all entities.
	pub fn tick(&mut self, chunks: &mut ChunkPool) {
		match &mut self.action_state {
			EntityActionState::Idle => {},
			EntityActionState::Walking(direction, amount) => {
				if *amount < 16 {
					*amount += 1;
				}
				if *amount > 15 {
					let direction = *direction;
					let pos = self.get_pos_in_direction(direction);
					if let Some(tile) = chunks.get_tile_stack_at_mut(self.pos) {
						self.pos = pos;
						self.action_state = EntityActionState::Idle;
						tile.entity_move_to(self);
					}
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

	/// Get the pos of the tile the entity is moving to.
	pub fn get_pos_in_direction(&self, direction: Direction4) -> [i64; 2] {
		let pos = self.pos;
		match direction {
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
			EntityActionState::Walking(_walking_direction, amount) => amount / 8 + 1,
			EntityActionState::Idle => 0,
		}));
	}

	/// Create a neew player at 0, 0
	pub fn new_player(difficulty: Difficulty) -> Self {
		let mut inventory = Inventory::new();
		if difficulty == Difficulty::Sandbox {
			inventory.items[0] = (Item::SandboxDestroyWand, 1);
			inventory.items[1] = (Item::Tile(Tile::Grass), 1);
			inventory.items[2] = (Item::Tile(Tile::Gravel), 1);
			inventory.items[3] = (Item::Tile(Tile::Sand), 1);
			inventory.items[4] = (Item::Tile(Tile::BlackSand), 1);
			inventory.items[5] = (Item::Tile(Tile::Rocks), 1);
			inventory.items[6] = (Item::Tile(Tile::OakTree), 1);
			inventory.items[7] = (Item::Tile(Tile::PineTree), 1);
			inventory.items[8] = (Item::Tile(Tile::Flowers), 1);
			inventory.items[9] = (Item::Tile(Tile::FlowersRedYellow), 1);
			inventory.items[10] = (Item::Tile(Tile::Water), 1);
			inventory.items[11] = (Item::Tile(Tile::Path), 1);
			inventory.items[12] = (Item::Axe, 1);
			inventory.items[13] = (Item::Shovel, 1);
		}
		Entity {
			pos: [0, 0],
			action_state: EntityActionState::Idle,
			facing: Direction4::South,
			entity_type: EntityType::Player { inventory, selected_item: 0, respawn_pos: [0, 0] },
			health: 100,
		}
	}

	/// Save player to file
	pub fn save_player(&self, player_filepath: &PathBuf, namespace_hash: u64) -> Option<()> {
		// Open file
		let mut file = FileWriter::new();
		// Push namespace hash
		file.data.extend(namespace_hash.to_le_bytes());
		// Get entity data
		self.serialize(&mut file);
		// Write
		file.write(player_filepath)?;
		Some(())
	}

	// Load player from file
	pub fn load_player(player_filepath: &PathBuf, namespaces_filepath: &PathBuf, difficulty: Difficulty) -> Option<Self> {
		// Open file
		let (mut file, _is_version_0) = FileReader::read_from_file(player_filepath)?;
		// Get namespace
		let namespace_hash = file.read_u64()?;
		let namespace = Namespace::load(namespace_hash, namespaces_filepath.clone())?;
		// Load entity
		Some(Self::deserialize(&mut file, &namespace, namespace.version, difficulty)?)
	}

	/// Save an entity
	pub fn serialize(&self, file: &mut FileWriter) {
		// Push pos
		file.push_world_pos(self.pos);
		// Push facing
		file.push_u8(self.facing as u8);
		// Push action state
		self.action_state.serialize(file);
		// Push type
		self.entity_type.serialize(file);
		// Push health
		file.push_u32(self.health);
	}

	pub fn deserialize(file: &mut FileReader, namespace: &Namespace, version: u32, difficulty: Difficulty) -> Option<Self> {
		// Get pos
		let pos = file.read_world_pos()?;
		// Get facing
		let facing = *namespace.direction_4s.get(file.read_u8()? as usize)?;
		// Get action state
		let action_state = EntityActionState::deserialize(file, namespace, version, facing)?;
		// Get entity type
		let entity_type = EntityType::deserialize(file, namespace, version, difficulty)?;
		// Get health
		let health = match version {
			0 => EntityVariant::Player.max_health(),
			_ => file.read_u32()?,
		};

		Some(Self {
			pos,
			facing,
			action_state,
			entity_type,
			health,
		})
	}
}