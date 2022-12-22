use crate::world::{tile::tile::TileVariant, world::World};

use super::{formatted_file_reader::FormattedFileReader};

pub struct Namespace {
	pub tiles: Vec<TileVariant>,
}

impl Namespace {
	pub fn load(hash: u64, world: &World) -> Option<Self> {
		let mut namespace_filepath = world.namespaces_filepath.clone();
		namespace_filepath.push(format!("{:16x}.nsp", hash));
		let file = FormattedFileReader::read_from_file(&namespace_filepath)?;
		if file.version > 0 {
			return None;
		}
		let mut meta_namespace: Vec<(String, Vec<String>)> = Vec::new();
		let mut x = 0;
		loop {
			let mut namespace = Vec::new();
			let namespace_name: [u8; 4] = file.body.get(x..x + 4)?.try_into().ok()?;
			let string_ptr = u32::from_le_bytes(namespace_name);
			if string_ptr == 0 {
				break;
			}
			let namespace_name = file.get_string(string_ptr)?;
			x += 4;
			loop {
				let name: [u8; 4] = file.body.get(x..x + 4)?.try_into().ok()?;
				let string_ptr = u32::from_le_bytes(name);
				if string_ptr == 0 {
					break;
				}
				let name = file.get_string(string_ptr)?;
				x += 4;
				namespace.push(name);
			}
			meta_namespace.push((namespace_name, namespace));
		}

		let tile_name_map = TileVariant::get_name_map();

		let mut tiles = Vec::new();

		for (namespace_name, namespace) in meta_namespace {
			match namespace_name.as_str() {
				"tile" => {
					for name in namespace {
						let tile = tile_name_map.get(&name)?;
						tiles.push(*tile);
					}
				}
				_ => return None,
			}
		}

		Some(Self {
			tiles,
		})
	}
}