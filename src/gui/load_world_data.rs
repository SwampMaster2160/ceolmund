use std::{path::PathBuf};

use crate::io::{io::IO, file_reader::FileReader, namespace::Namespace};

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
				if let Some((overview, is_version_0)) = FileReader::read_from_file(&overview_path) {
					//println!("{:?}", overview_path);
					//println!("C: {:?}", overview_path);
					let (body_index, _version) = if is_version_0 {
						(0, 0)
					}
					else {
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
						//println!("{}", namespace_hash);
						let namespace = if let Some(namespace) = Namespace::load(namespace_hash, namespaces_filepath) {
							namespace
						}
						else {
							continue;
						};
						//println!("{}", namespace_hash);
						(8, namespace.version)
					};
					//println!("C: {:?}", overview_path);
					if is_version_0 {
						//println!("A: {:?}", overview_path);
						if let Some(name_pos) = overview.body.get(body_index..body_index + 4) {
							let name_pos: [u8; 4] = name_pos.try_into().unwrap();
							let name_pos = u32::from_le_bytes(name_pos);
							if let Some(name) = overview.get_string_v0(name_pos) {
								worlds.push((name, path));
							}
						}
					}
					else {
						//println!("B: {:?}", overview_path);
						if let Some((name, _data_read_size)) = overview.get_string(body_index) {
							worlds.push((name, path));
							//body_index += data_read_size;
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