use std::{path::PathBuf};

use crate::io::{io::IO, formatted_file_reader::FormattedFileReader, namespace::Namespace};

/// A struct containing the name and path of all the worlds in the users world folder.
#[derive(Clone)]
pub struct LoadWorldData {
	pub worlds: Vec<(String, PathBuf)>,
}

impl LoadWorldData {
	/// Get all the worlds.
	pub fn new(io: &IO) -> Self {
		let mut worlds = Vec::new();
		for item in io.worlds_path.read_dir().unwrap() {
			if let Ok(item) = item {
				let path = item.path();
				let mut overview_path = path.clone();
				overview_path.push("overview.wld");
				let mut namespaces_filepath = path.clone();
				namespaces_filepath.push("namespaces");
				if let Some((overview, is_version_0)) = FormattedFileReader::read_from_file(&overview_path) {
					/*if overview.version <= 0 {
						if let Some(name_pos) = overview.body.get(0..4) {
							let name_pos: [u8; 4] = name_pos.try_into().unwrap();
							let name_pos = u32::from_le_bytes(name_pos);
							if let Some(name) = overview.get_string(name_pos) {
								worlds.push((name, path));
							}
						}
					}*/
					let (body_index, version) = if is_version_0 {
						(0, 0)
					}
					else {
						//let namespace_hash = overview.body.get(0..8)?.try_into().ok()?;
						let namespace_hash = if let Some(namespace_hash) = overview.body.get(0..8) {
							let namespace_hash = if let Some(namespace_hash) = namespace_hash.try_into().ok() {
								namespace_hash
							}
							else {
								continue;
							};
							namespace_hash
						}
						else {
							continue;
						};
						let namespace_hash = u64::from_le_bytes(namespace_hash);
						let namespace = if let Some(namespace) = Namespace::load(namespace_hash, namespaces_filepath) {
							namespace
						}
						else {
							continue;
						};
						(8, namespace.version)
					};
					if let Some(name_pos) = overview.body.get(body_index..body_index + 4) {
						let name_pos: [u8; 4] = name_pos.try_into().unwrap();
						let name_pos = u32::from_le_bytes(name_pos);
						if let Some(name) = overview.get_string(name_pos) {
							worlds.push((name, path));
						}
					}
				}
			}
		}
		Self {
			worlds,
		}
	}
}