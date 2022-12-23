use std::path::PathBuf;

use crate::world::{tile::tile::TileVariant};

use super::{formatted_file_reader::FormattedFileReader, namespace_name::NamespaceName};

/// A namespace loaded from disk
pub struct Namespace {
	pub tiles: Vec<TileVariant>,
}

impl Namespace {
	/// Load a namespace from a hash and a namespace folder path.
	pub fn load(hash: u64, namespaces_filepath: PathBuf) -> Option<Self> {
		// Get the path of the namespace
		let mut namespace_filepath = namespaces_filepath.clone();
		namespace_filepath.push(format!("{:16x}.nsp", hash));
		let file = FormattedFileReader::read_from_file(&namespace_filepath)?;
		// Error if the namespace is a future version.
		if file.version > 0 {
			return None;
		}
		// Data to extract
		let tile_name_map = TileVariant::get_name_map();
		let mut tiles = Vec::new();
		// The index to where we are indexing to in the body vec.
		let mut body_index = 0;
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
					break;
				}
				let name = file.get_string(string_ptr)?;
				// Point to the next string pointer
				body_index += 4;
				// Convert
				match namespace_name {
					NamespaceName::Tile => tiles.push(*tile_name_map.get(&name)?),
				}
			}
		}

		Some(Self {
			tiles,
		})
	}
}