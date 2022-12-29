use std::path::PathBuf;

use crate::{render::vertex::Vertex, io::{game_key::GameKey, io::IO, formatted_file_writer::FormattedFileWriter, formatted_file_reader::FormattedFileReader, namespace::Namespace}, world::{direction::Direction4, chunk::chunk_pool::ChunkPool, tile::tile::Tile, item::item::Item}, gui::{gui::GUI, gui_menu::GUIMenu, gui_menu_variant::GUIMenuVariant}};
use super::{entity_action_state::EntityActionState, entity_type::EntityType};

/// A world object that is can move from tile to tile.
pub struct Entity {
	pub pos: [i64; 2],
	pub facing: Direction4,
	pub action_state: EntityActionState,
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
		let pos_in_front = self.get_pos_in_front();
		if input.get_game_key_starting_now(GameKey::MenuOpenClose) {
			gui.menus.push(GUIMenu::new(GUIMenuVariant::Paused));
		}
		let action_state = self.action_state.clone();
		if action_state == EntityActionState::Idle {
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

			// Interact with world
			let mut chunks_offset = chunks.get_offset(pos_in_front);
			if input.get_game_key_starting_now(GameKey::Interact) {
				let item_stack = &mut inventory[*selected_item as usize];
				Item::use_stack_mut_self(item_stack, &mut chunks_offset);
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
		inventory[1] = (Item::Tile(Tile::Grass), 1);
		inventory[2] = (Item::Tile(Tile::Gravel), 1);
		inventory[3] = (Item::Tile(Tile::Sand), 1);
		inventory[4] = (Item::Tile(Tile::BlackSand), 1);
		inventory[5] = (Item::Tile(Tile::Rocks), 1);
		inventory[6] = (Item::Tile(Tile::OakTree), 1);
		inventory[7] = (Item::Tile(Tile::PineTree), 1);
		inventory[8] = (Item::Tile(Tile::Flowers), 1);
		inventory[9] = (Item::Tile(Tile::FlowersRedYellow), 1);
		inventory[10] = (Item::Tile(Tile::Water), 1);
		inventory[11] = (Item::Tile(Tile::Path), 1);
		inventory[12] = (Item::Axe, 1);
		inventory[13] = (Item::Shovel, 1);
		Entity {
			pos: [0, 0],
			action_state: EntityActionState::Idle,
			facing: Direction4::South,
			entity_type: EntityType::Player { inventory, selected_item: 0 },
		}
	}

	/// Save player to file
	pub fn save_player(&self, player_filepath: &PathBuf, namespace_hash: u64) -> Option<()> {
		// Open file
		let mut file = FormattedFileWriter::new(0);
		// Push namespace hash
		file.body.extend(namespace_hash.to_le_bytes());
		// Get entity data
		self.save(&mut file.body);
		// Write
		file.write(player_filepath)?;
		Some(())
	}

	// Load player from file
	pub fn load_player(player_filepath: &PathBuf, namespaces_filepath: &PathBuf) -> Option<Self> {
		// Open file
		let file = FormattedFileReader::read_from_file(player_filepath)?;
		if file.version > 0 {
			return None;
		}
		// Get namespace
		let namespace_hash =  file.body.get(0..8)?.try_into().ok()?;
		let namespace_hash = u64::from_le_bytes(namespace_hash);
		let namespace = Namespace::load(namespace_hash, namespaces_filepath.clone())?;
		// Load entity
		Some(Self::load(file.body.get(8..)?, &namespace, file.version)?.0)
	}

	/// Save an entity
	pub fn save(&self, data: &mut Vec<u8>) {
		// Push pos
		data.extend(self.pos[0].to_le_bytes());
		data.extend(self.pos[1].to_le_bytes());
		// Push facing
		data.push(self.facing as u8);
		// Push action state
		self.action_state.save(data);
		// Push type
		self.entity_type.save(data);
	}

	/// Load an entity
	pub fn load(data: &[u8], namespace: &Namespace, version: u32) -> Option<(Self, usize)> {
		// Get pos
		let pos_x = data.get(0..8)?.try_into().ok()?;
		let pos_y = data.get(8..16)?.try_into().ok()?;
		let pos = [i64::from_le_bytes(pos_x), i64::from_le_bytes(pos_y)];
		// Get facing
		let facing = namespace.direction_4s[*data.get(16)? as usize];
		// Get action state
		let (action_state, advanced_amount) = EntityActionState::load(data.get(17..)?, namespace, version)?;
		let mut data_read_size = 17 + advanced_amount;
		let data = data.get(17 + advanced_amount..)?;
		// Get entity type
		let (entity_type, advanced_amount) = EntityType::load(data, namespace, version)?;
		data_read_size += advanced_amount;

		Some((Self {
			pos,
			facing,
			action_state,
			entity_type,
		}, data_read_size))
	}
}