use unicode_segmentation::UnicodeSegmentation;

use crate::world::difficulty::Difficulty;
use crate::{world::entity::entity_action_state::EntityActionStateVariant, io::io::SERIALIZATION_VERSION};
use crate::world::entity::entity_type::EntityVariant;
use crate::world::item::item::ItemVariant;
use std::path::PathBuf;

use crate::world::{tile::tile::TileVariant, direction::Direction4};

use super::{file_reader::FileReader, namespace_name::NamespaceName};

/// A namespace loaded from disk
pub struct Namespace {
	pub version: u32,
	pub tiles: Vec<TileVariant>,
	pub items: Vec<ItemVariant>,
	pub entities: Vec<EntityVariant>,
	pub direction_4s: Vec<Direction4>,
	pub entity_action_states: Vec<EntityActionStateVariant>,
	pub difficulties: Vec<Difficulty>,
}

impl Namespace {
	/// Load a namespace from a hash and a namespace folder path.
	pub fn load(hash: u64, namespaces_filepath: PathBuf) -> Option<Self> {
		// Get the path of the namespace
		let mut namespace_filepath = namespaces_filepath.clone();
		namespace_filepath.push(format!("{:0>16x}.nsp", hash));
		let (mut file, is_version_0) = FileReader::read_from_file(&namespace_filepath)?;
		// Get version
		let version = match is_version_0 {
			true => 0,
			false => file.read_u32()?,
		};
		if version > SERIALIZATION_VERSION {
			return None;
		}
		// Data to extract
		let tile_name_map = TileVariant::get_name_map();
		let item_name_map = ItemVariant::get_name_map();
		let entity_name_map = EntityVariant::get_name_map();
		let direction_4_name_map = Direction4::get_name_map();
		let entity_action_state_name_map = EntityActionStateVariant::get_name_map();
		let difficulty_name_map = Difficulty::get_name_map();
		let mut tiles = Vec::new();
		let mut items = Vec::new();
		let mut entities = Vec::new();
		let mut direction_4s = Vec::new();
		let mut entity_action_states = Vec::new();
		let mut difficulties = Vec::new();
		// For each namespace
		if version == 0 {
			let mut body_index = file.read_index;
			loop {
				// Get the name of the namespace and break the loop if we are at the end of the namespaces.
				let string_ptr: [u8; 4] = file.data.get(body_index..body_index + 4)?.try_into().ok()?;
				let string_ptr = u32::from_le_bytes(string_ptr);
				if string_ptr == 0xFFFFFFFF {
					break;
				}
				let namespace_name = file.get_string_v0(string_ptr)?;
				let namespace_name = NamespaceName::from_name(&namespace_name)?;
				// Point to the next string pointer
				body_index += 4;
				// For each name
				loop {
					// Get the name and break if we are at the end of the namespace.
					let name: [u8; 4] = file.data.get(body_index..body_index + 4)?.try_into().ok()?;
					let string_ptr = u32::from_le_bytes(name);
					if string_ptr == 0xFFFFFFFF {
						body_index += 4;
						break;
					}
					let name = file.get_string_v0(string_ptr)?;
					// Point to the next string pointer
					body_index += 4;
					// Convert
					match namespace_name {
						NamespaceName::Tile => tiles.push(*tile_name_map.get(&name)?),
						NamespaceName::Item => items.push(*item_name_map.get(&name)?),
						NamespaceName::Entity => entities.push(*entity_name_map.get(&name)?),
						NamespaceName::Direction4 => direction_4s.push(*direction_4_name_map.get(&name)?),
						NamespaceName::EntityActionStates => entity_action_states.push(*entity_action_state_name_map.get(&name)?),
						NamespaceName::Difficulty => difficulties.push(*difficulty_name_map.get(&name)?),
					}
				}
			}
		}
		else {
			loop {
				// Get the name of the namespace and break the loop if we are at the end of the namespaces.
				let namespace_name = file.read_string()?;
				if namespace_name.graphemes(true).count() == 0 {
					break;
				}
				let namespace_name = NamespaceName::from_name(&namespace_name)?;
				// For each name
				loop {
					// Get the name and break if we are at the end of the namespace.
					let name = file.read_string()?;
					if name.graphemes(true).count() == 0 {
						break;
					}
					// Convert
					match namespace_name {
						NamespaceName::Tile => tiles.push(*tile_name_map.get(&name)?),
						NamespaceName::Item => items.push(*item_name_map.get(&name)?),
						NamespaceName::Entity => entities.push(*entity_name_map.get(&name)?),
						NamespaceName::Direction4 => direction_4s.push(*direction_4_name_map.get(&name)?),
						NamespaceName::EntityActionStates => entity_action_states.push(*entity_action_state_name_map.get(&name)?),
						NamespaceName::Difficulty => difficulties.push(*difficulty_name_map.get(&name)?),
					}
				}
			}
		}

		Some(Self {
			version,
			tiles,
			entities,
			direction_4s,
			items,
			entity_action_states,
			difficulties,
		})
	}
}