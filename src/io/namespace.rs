use crate::{world::entity::entity_action_state::EntityActionStateVariant, io::io::SERIALIZATION_VERSION};
use crate::world::entity::entity_type::EntityVariant;
use crate::world::item::item::ItemVariant;
use std::path::PathBuf;

use crate::world::{tile::tile::TileVariant, direction::Direction4};

use super::{formatted_file_reader::FormattedFileReader, namespace_name::NamespaceName};

/// A namespace loaded from disk
pub struct Namespace {
	pub version: u32,
	pub tiles: Vec<TileVariant>,
	pub items: Vec<ItemVariant>,
	pub entities: Vec<EntityVariant>,
	pub direction_4s: Vec<Direction4>,
	pub entity_action_states: Vec<EntityActionStateVariant>,
}

impl Namespace {
	/// Load a namespace from a hash and a namespace folder path.
	pub fn load(hash: u64, namespaces_filepath: PathBuf) -> Option<Self> {
		// Get the path of the namespace
		let mut namespace_filepath = namespaces_filepath.clone();
		namespace_filepath.push(format!("{:16x}.nsp", hash));
		let (file, is_version_0) = FormattedFileReader::read_from_file(&namespace_filepath)?;
		// Error if the namespace is a future version.
		/*if file.version > SERIALIZATION_VERSION {
			return None;
		}*/
		// Get version
		let (version, mut body_index) = if is_version_0 {
			(0, 0)
		}
		else {
			let version: [u8; 4] = file.body.get(0..4)?.try_into().ok()?;
			let version = u32::from_le_bytes(version);
			(version, 4)
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
		let mut tiles = Vec::new();
		let mut items = Vec::new();
		let mut entities = Vec::new();
		let mut direction_4s = Vec::new();
		let mut entity_action_states = Vec::new();
		// For each namespace
		loop {
			// Get the name of the namespace and break the loop if we are at the end of the namespaces.
			let string_ptr: [u8; 4] = file.body.get(body_index..body_index + 4)?.try_into().ok()?;
			let string_ptr = u32::from_le_bytes(string_ptr);
			if string_ptr == 0xFFFFFFFF {
				break;
			}
			let namespace_name = file.get_string(string_ptr)?;
			let namespace_name = NamespaceName::from_name(&namespace_name)?;
			// Point to the next string pointer
			body_index += 4;
			// For each name
			loop {
				// Get the name and break if we are at the end of the namespace.
				let name: [u8; 4] = file.body.get(body_index..body_index + 4)?.try_into().ok()?;
				let string_ptr = u32::from_le_bytes(name);
				if string_ptr == 0xFFFFFFFF {
					body_index += 4;
					break;
				}
				let name = file.get_string(string_ptr)?;
				// Point to the next string pointer
				body_index += 4;
				// Convert
				match namespace_name {
					NamespaceName::Tile => tiles.push(*tile_name_map.get(&name)?),
					NamespaceName::Item => items.push(*item_name_map.get(&name)?),
					NamespaceName::Entity => entities.push(*entity_name_map.get(&name)?),
					NamespaceName::Direction4 => direction_4s.push(*direction_4_name_map.get(&name)?),
					NamespaceName::EntityActionStates => entity_action_states.push(*entity_action_state_name_map.get(&name)?),
				}
			}
		}
		println!("{:?}", tiles);

		Some(Self {
			version,
			tiles,
			entities,
			direction_4s,
			items,
			entity_action_states,
		})
	}
}